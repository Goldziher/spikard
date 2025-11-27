//! PHP handler infrastructure.
//!
//! This module provides the global Tokio runtime for PHP async operations
//! and handler types for implementing the spikard-http Handler trait.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use ext_php_rs::convert::IntoZval;
use ext_php_rs::types::ZendCallable;
use once_cell::sync::Lazy;
use spikard_http::{CONTENT_TYPE_PROBLEM_JSON, Handler, HandlerResult, ProblemDetails, RequestData};
use std::pin::Pin;
use std::sync::{Arc, OnceLock};
use tokio::runtime::{Builder, Runtime};

/// Global Tokio runtime for async operations.
///
/// Initialized once and reused for all async operations throughout the lifetime
/// of the PHP process. Uses single-threaded runtime to avoid Send/Sync requirements
/// for PHP Zval types.
pub static GLOBAL_RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    Builder::new_current_thread().enable_all().build().unwrap_or_else(|e| {
        eprintln!("Failed to initialise global Tokio runtime: {}", e);
        panic!("Cannot continue without Tokio runtime");
    })
});

/// Inner state of a PHP handler.
#[allow(dead_code)]
pub struct PhpHandlerInner {
    pub handler_name: String,
    pub method: String,
    pub path: String,
}

/// Wrapper around a PHP callable that implements the Handler trait.
///
/// Note: PHP handlers run synchronously on the PHP thread. The Handler trait
/// implementation wraps the synchronous call in an async wrapper for compatibility
/// with spikard-http's async infrastructure.
#[derive(Clone)]
#[allow(dead_code)]
pub struct PhpHandler {
    pub inner: Arc<PhpHandlerInner>,
    /// Index into a handler registry - used to retrieve the actual PHP callable
    /// at invocation time (since ZendCallable is not Send/Sync).
    pub handler_index: usize,
}

/// Registry for PHP callables referenced by Handler index.
///
/// We store Zval instead of ZendCallable directly because ZendCallable has
/// a lifetime parameter that prevents it from being stored in a static.
/// We reconstruct the ZendCallable from the Zval when invoking.
///
/// NOTE: This is thread_local because Zval is not Send/Sync (contains raw pointers
/// to PHP's internal structures which are single-threaded).
thread_local! {
    static PHP_HANDLER_REGISTRY: std::cell::RefCell<Vec<ext_php_rs::types::Zval>> = std::cell::RefCell::new(Vec::new());
}

impl PhpHandler {
    /// Register a PHP callable and return its index for later invocation.
    ///
    /// # Parameters
    /// * `callable_zval` - The Zval containing the callable (not wrapped in ZendCallable yet)
    ///
    /// This allows us to clone the Zval before it gets wrapped in ZendCallable with a lifetime.
    pub fn register_from_zval(callable_zval: &ext_php_rs::types::Zval, handler_name: String, method: String, path: String) -> Result<Self, String> {
        // Verify it's actually callable before storing
        if !callable_zval.is_callable() {
            return Err(format!("Handler '{}' is not callable", handler_name));
        }

        let idx = PHP_HANDLER_REGISTRY.with(|registry| {
            let mut registry = registry.borrow_mut();
            let idx = registry.len();

            // Clone the Zval for storage
            let zval_copy = callable_zval.shallow_clone();
            registry.push(zval_copy);
            idx
        });

        Ok(Self {
            inner: Arc::new(PhpHandlerInner {
                handler_name,
                method,
                path,
            }),
            handler_index: idx,
        })
    }
}

impl Handler for PhpHandler {
    fn call(
        &self,
        _req: Request<Body>,
        request_data: RequestData,
    ) -> Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send + '_>> {
        // All PHP invocation is performed synchronously before returning the future
        // to avoid capturing non-Send PHP types across .await boundaries.
        let result = invoke_php_handler(self.handler_index, &self.inner.handler_name, &request_data);
        Box::pin(async move { result })
    }
}

/// Invoke the PHP callable registered at index and return a HandlerResult.
fn invoke_php_handler(handler_index: usize, handler_name: &str, request_data: &RequestData) -> HandlerResult {
    // Build PhpRequest from RequestData and convert to Zval
    let php_request = crate::php::request::PhpRequest::from_request_data(request_data);
    let request_zval = php_request.into_zval(false).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to convert request for PHP handler: {:?}", e),
        )
    })?;

    // Invoke PHP callable synchronously from thread-local registry
    let response_zval = PHP_HANDLER_REGISTRY.with(|registry| -> Result<ext_php_rs::types::Zval, (StatusCode, String)> {
        let registry = registry.borrow();
        let callable_zval = registry.get(handler_index).ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("PHP handler not found: index {}", handler_index),
            )
        })?;

        // Reconstruct ZendCallable from stored Zval
        let callable = ZendCallable::new(callable_zval).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to reconstruct PHP callable: {:?}", e),
            )
        })?;

        callable.try_call(vec![&request_zval]).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("PHP handler '{handler_name}' failed: {:?}", e),
            )
        })
    })?;

    // Interpret PHP response into HandlerResult
    crate::php::server::interpret_php_response(&response_zval, handler_name)
}

/// Build a structured ProblemDetails response with application/problem+json.
fn internal_problem(status: StatusCode, detail: impl Into<String>) -> HandlerResult {
    let problem = ProblemDetails::new(
        ProblemDetails::TYPE_INTERNAL_SERVER_ERROR,
        "Internal Server Error",
        status,
    )
    .with_detail(detail);

    let body = serde_json::to_vec(&problem).unwrap_or_else(|_| b"{}".to_vec());

    axum::http::Response::builder()
        .status(status)
        .header("content-type", CONTENT_TYPE_PROBLEM_JSON)
        .body(Body::from(body))
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to build error response: {}", e),
            )
        })
}

//! PHP handler infrastructure.
//!
//! This module provides the global Tokio runtime for PHP async operations
//! and handler types for implementing the spikard-http Handler trait.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use ext_php_rs::convert::IntoZval;
use ext_php_rs::types::ZendCallable;
use once_cell::sync::{Lazy, OnceLock};
use spikard_http::{CONTENT_TYPE_PROBLEM_JSON, Handler, HandlerResult, ProblemDetails, RequestData};
use std::pin::Pin;
use std::sync::Arc;
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
static PHP_HANDLER_REGISTRY: OnceLock<parking_lot::Mutex<Vec<ZendCallable>>> = OnceLock::new();

impl PhpHandler {
    /// Register a PHP callable and return its index for later invocation.
    pub fn register(callable: ZendCallable, handler_name: String, method: String, path: String) -> Self {
        let registry = PHP_HANDLER_REGISTRY.get_or_init(|| parking_lot::Mutex::new(Vec::new()));
        let mut guard = registry.lock();
        let idx = guard.len();
        guard.push(callable);

        Self {
            inner: Arc::new(PhpHandlerInner {
                handler_name,
                method,
                path,
            }),
            handler_index: idx,
        }
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
    let registry = PHP_HANDLER_REGISTRY.get().ok_or_else(|| {
        internal_problem(
            StatusCode::INTERNAL_SERVER_ERROR,
            "PHP handler registry not initialised",
        )
    })?;

    // Build PhpRequest from RequestData and convert to Zval
    let php_request = crate::php::request::PhpRequest::from_request_data(request_data);
    let request_zval = php_request.into_zval(false).map_err(|e| {
        internal_problem(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to convert request for PHP handler: {:?}", e),
        )
    })?;

    // Invoke PHP callable synchronously.
    let response_zval = {
        let mut guard = registry.lock();
        let callable = guard.get_mut(handler_index).ok_or_else(|| {
            internal_problem(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("PHP handler not found: index {}", handler_index),
            )
        })?;

        callable.try_call(vec![&request_zval]).map_err(|e| {
            internal_problem(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("PHP handler '{handler_name}' failed: {:?}", e),
            )
        })?
    };

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

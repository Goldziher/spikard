//! PHP handler infrastructure.
//!
//! This module provides the global Tokio runtime for PHP async operations
//! and handler types for implementing the spikard-http Handler trait.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use ext_php_rs::convert::IntoZval;
use ext_php_rs::prelude::*;
use ext_php_rs::types::ZendCallable;
use spikard_bindings_shared::ErrorResponseBuilder;
use spikard_core::errors::StructuredError;
use spikard_http::{Handler, HandlerResult, RequestData};
use std::panic::AssertUnwindSafe;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::OnceLock;
use tokio::runtime::{Builder, Runtime};

/// Global Tokio runtime for async operations.
///
/// Initialized once and reused for all async operations throughout the lifetime
/// of the PHP process. Uses single-threaded runtime to avoid Send/Sync requirements
/// for PHP Zval types.
static GLOBAL_RUNTIME: OnceLock<Result<Runtime, StructuredError>> = OnceLock::new();

pub fn get_runtime() -> PhpResult<&'static Runtime> {
    let runtime_result = GLOBAL_RUNTIME.get_or_init(|| {
        Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| StructuredError::simple("runtime_init_failed".to_string(), e.to_string()))
    });

    runtime_result
        .as_ref()
        .map_err(|err| PhpException::default(serde_json::to_string(err).unwrap_or_else(|_| err.code.clone())))
}

/// Inner state of a PHP handler.
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
pub struct PhpHandler {
    pub inner: Arc<PhpHandlerInner>,
    /// Index into a handler registry - used to retrieve the actual PHP callable
    /// at invocation time (since ZendCallable is not Send/Sync).
    pub handler_index: usize,
}

// NOTE: This is thread_local because Zval is not Send/Sync (contains raw pointers
thread_local! {
    static PHP_HANDLER_REGISTRY: std::cell::RefCell<Vec<ext_php_rs::types::Zval>> = const {
        std::cell::RefCell::new(Vec::new())
    };
}

impl PhpHandler {
    /// Register a PHP callable and return its index for later invocation.
    ///
    /// # Parameters
    /// * `callable_zval` - The Zval containing the callable (not wrapped in ZendCallable yet)
    ///
    /// This allows us to clone the Zval before it gets wrapped in ZendCallable with a lifetime.
    pub fn register_from_zval(
        callable_zval: &ext_php_rs::types::Zval,
        handler_name: String,
        method: String,
        path: String,
    ) -> Result<Self, String> {
        if !callable_zval.is_callable() {
            return Err(format!("Handler '{}' is not callable", handler_name));
        }

        let idx = PHP_HANDLER_REGISTRY.with(|registry| -> Result<usize, String> {
            let mut registry = registry.borrow_mut();
            let idx = registry.len();

            if idx > 10_000 {
                return Err("Handler registry is full; refusing to register more handlers".to_string());
            }

            let zval_copy = callable_zval.shallow_clone();
            registry.push(zval_copy);
            Ok(idx)
        })?;

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
        let result = invoke_php_handler(self.handler_index, &self.inner.handler_name, request_data);
        Box::pin(async move { result })
    }
}

/// Invoke the PHP callable registered at index and return a HandlerResult.
fn invoke_php_handler(handler_index: usize, handler_name: &str, request_data: RequestData) -> HandlerResult {
    let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
        let php_request = crate::php::request::PhpRequest::from_request_data_owned(request_data);
        let request_zval = php_request.into_zval(false).map_err(|e| {
            ErrorResponseBuilder::structured_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "request_conversion_failed",
                format!("Failed to convert request for PHP handler: {:?}", e),
            )
        })?;

        let response_zval =
            PHP_HANDLER_REGISTRY.with(|registry| -> Result<ext_php_rs::types::Zval, (StatusCode, String)> {
                let registry = registry.borrow();
                let Some(callable_zval) = registry.get(handler_index) else {
                    return Err(ErrorResponseBuilder::structured_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "handler_not_found",
                        format!("PHP handler not found: index {}", handler_index),
                    ));
                };

                let callable = ZendCallable::new(callable_zval).map_err(|e| {
                    ErrorResponseBuilder::structured_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "callable_reconstruct_failed",
                        format!("Failed to reconstruct PHP callable: {:?}", e),
                    )
                })?;

                callable.try_call(vec![&request_zval]).map_err(|e| {
                    ErrorResponseBuilder::structured_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "handler_failed",
                        format!("PHP handler '{handler_name}' failed: {:?}", e),
                    )
                })
            })?;

        crate::php::server::interpret_php_response(&response_zval, handler_name)
    }));

    match result {
        Ok(inner) => inner,
        Err(_) => Err(ErrorResponseBuilder::structured_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "panic",
            "Unexpected panic while executing PHP handler",
        )),
    }
}

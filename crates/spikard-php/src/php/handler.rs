//! PHP handler infrastructure.
//!
//! This module provides the global Tokio runtime for PHP async operations
//! and handler types for implementing the spikard-http Handler trait.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use once_cell::sync::Lazy;
use spikard_http::{Handler, HandlerResult, RequestData};
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

/// Placeholder implementation - actual handler invocation happens via PhpServer
/// which has access to the PHP callables stored in a thread-local registry.
impl Handler for PhpHandler {
    fn call(
        &self,
        _req: Request<Body>,
        _request_data: RequestData,
    ) -> Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send + '_>> {
        // This is a placeholder - in practice, PHP handlers are invoked
        // synchronously via the test client or server which has access to
        // the actual PHP callables. The Handler trait implementation here
        // exists to satisfy the type system but the actual invocation
        // happens through PhpServer::build_router() which creates closures
        // that capture the PHP callables.
        Box::pin(async move {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "PHP handlers must be invoked through PhpServer".to_string(),
            ))
        })
    }
}

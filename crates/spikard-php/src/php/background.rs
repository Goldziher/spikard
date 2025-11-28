//! Background task execution for PHP bindings.
//!
//! Provides fire-and-forget background task execution using Tokio's blocking threadpool.
//! Tasks run outside the HTTP request lifecycle and don't block responses.

use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;
use once_cell::sync::Lazy;
use spikard_http::{BackgroundHandle, BackgroundJobError, BackgroundJobMetadata};
use std::sync::RwLock;

static BACKGROUND_HANDLE: Lazy<RwLock<Option<BackgroundHandle>>> =
    Lazy::new(|| RwLock::new(None));

/// Install the background handle at server startup
pub fn install_handle(handle: BackgroundHandle) {
    if let Ok(mut guard) = BACKGROUND_HANDLE.write() {
        *guard = Some(handle);
    }
}

/// Clear the background handle at server shutdown
pub fn clear_handle() {
    if let Ok(mut guard) = BACKGROUND_HANDLE.write() {
        *guard = None;
    }
}

/// Run a PHP callable in the background
///
/// Spawns a blocking task on the Tokio threadpool to execute the PHP callable.
/// The task runs outside the request lifecycle and doesn't block the HTTP server.
///
/// # Arguments
/// * `callable` - PHP callable (closure, function name, array ['class', 'method'])
/// * `args` - Optional array of arguments to pass to callable
///
/// # Errors
/// * Returns error if background runtime not initialized
/// * Returns error if task queue is full
/// * Returns error if callable is not actually callable
///
/// # Example
/// ```php
/// spikard_background_run(function() {
///     sleep(5);
///     error_log("Background task complete");
/// });
/// ```
#[php_function]
#[php(name = "spikard_background_run")]
pub fn spikard_background_run(_callable: &Zval, _args: Option<&Zval>) -> PhpResult<()> {
    // TODO: Background tasks not yet supported in PHP due to threading constraints.
    // PHP is single-threaded and Zvals cannot be sent across thread boundaries.
    // We need a different queue mechanism for PHP background tasks.
    Err(PhpException::default(
        "Background tasks are not yet supported in PHP bindings. \
         PHP is single-threaded and cannot safely execute callables in background threads."
            .to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::mpsc;

    #[test]
    fn test_handle_lifecycle() {
        let (tx, _rx) = mpsc::channel(10);
        let handle = BackgroundHandle::new(tx, Arc::new(Default::default()));

        install_handle(handle.clone());

        let retrieved = BACKGROUND_HANDLE.read().unwrap().clone();
        assert!(retrieved.is_some());

        clear_handle();
        assert!(BACKGROUND_HANDLE.read().unwrap().is_none());
    }
}

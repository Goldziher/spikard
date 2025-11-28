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
pub fn spikard_background_run(callable: &Zval, args: Option<&Zval>) -> PhpResult<()> {
    // Validate callable
    if !callable.is_callable() {
        return Err(PhpException::default(
            "First argument to spikard_background_run must be callable".to_string(),
        ));
    }

    // Get background handle
    let handle = BACKGROUND_HANDLE
        .read()
        .map_err(|_| PhpException::default("Background handle lock poisoned".to_string()))?
        .clone()
        .ok_or_else(|| {
            PhpException::default(
                "Background runtime not initialized. Server must be running to spawn background tasks.".to_string(),
            )
        })?;

    // Clone for 'static lifetime
    let callable_owned = callable.shallow_clone();
    let args_owned = args.map(|a| a.shallow_clone());

    // Spawn blocking task
    handle
        .spawn_with_metadata(
            async move {
                tokio::task::spawn_blocking(move || -> Result<(), BackgroundJobError> {
                    // Reconstruct ZendCallable in blocking context
                    let call_result = if let Some(args_zval) = args_owned {
                        // Extract args array
                        let args_array = args_zval
                            .array()
                            .ok_or_else(|| BackgroundJobError::from("Arguments must be an array"))?;

                        let args_vec: Vec<&dyn ext_php_rs::convert::IntoZvalDyn> =
                            args_array.values().map(|v| v as &dyn ext_php_rs::convert::IntoZvalDyn).collect();

                        ZendCallable::new(&callable_owned)
                            .map_err(|e| BackgroundJobError::from(format!("Failed to create callable: {:?}", e)))?
                            .try_call(args_vec)
                    } else {
                        ZendCallable::new(&callable_owned)
                            .map_err(|e| BackgroundJobError::from(format!("Failed to create callable: {:?}", e)))?
                            .try_call(vec![])
                    };

                    call_result.map(|_| ()).map_err(|e| {
                        BackgroundJobError::from(format!("Background task failed: {:?}", e))
                    })
                })
                .await
                .map_err(|e| BackgroundJobError::from(format!("Task join error: {}", e)))?
            },
            BackgroundJobMetadata::default(),
        )
        .map_err(|err| PhpException::default(err.to_string()))
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

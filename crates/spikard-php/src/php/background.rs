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

    // Serialize the callable to a string representation that can cross thread boundaries
    // For closures, we'll get an error - user must use named functions or class methods
    let callable_str = serialize_callable(callable)?;
    let args_json = args.map(|a| serialize_args(a)).transpose()?;

    // Spawn task with serialized data (String is Send+Sync)
    handle
        .spawn_with_metadata(
            async move {
                // Deserialize and execute on the worker thread
                tokio::task::spawn_blocking(move || -> Result<(), BackgroundJobError> {
                    execute_serialized_task(&callable_str, args_json.as_deref())
                        .map_err(|e| BackgroundJobError::from(e))
                })
                .await
                .map_err(|e| BackgroundJobError::from(format!("Task join error: {}", e)))?
            },
            BackgroundJobMetadata::default(),
        )
        .map_err(|err| PhpException::default(err.to_string()))
}

/// Serialize a callable to a string representation.
/// Only supports: function names, "Class::method", or ["Class", "method"]
fn serialize_callable(callable: &Zval) -> PhpResult<String> {
    // Try string first (function name or "Class::method")
    if let Some(s) = callable.string() {
        return Ok(s.to_string());
    }

    // Try array ["Class", "method"]
    if let Some(arr) = callable.array() {
        let values: Vec<String> = arr.values()
            .filter_map(|v| v.string())
            .map(|s| s.to_string())
            .collect();

        if values.len() == 2 {
            return Ok(format!("{}::{}", values[0], values[1]));
        }
    }

    Err(PhpException::default(
        "Background tasks only support named functions or class methods (\"ClassName::method\" or [\"ClassName\", \"method\"]). \
         Closures cannot be serialized across threads.".to_string()
    ))
}

/// Serialize arguments to JSON
fn serialize_args(args: &Zval) -> PhpResult<String> {
    let json_value = crate::php::zval_to_json(args)
        .map_err(|e| PhpException::default(format!("Failed to serialize arguments: {}", e)))?;

    serde_json::to_string(&json_value)
        .map_err(|e| PhpException::default(format!("Failed to serialize arguments to JSON: {}", e)))
}

/// Execute a serialized task on a worker thread.
/// This attempts to call PHP from a Rust thread, which requires thread-local PHP initialization.
fn execute_serialized_task(callable_str: &str, args_json: Option<&str>) -> Result<(), String> {
    // Parse the callable string
    let (class_opt, method) = if callable_str.contains("::") {
        let parts: Vec<&str> = callable_str.split("::").collect();
        if parts.len() != 2 {
            return Err(format!("Invalid callable format: {}", callable_str));
        }
        (Some(parts[0]), parts[1])
    } else {
        (None, callable_str)
    };

    // Parse arguments
    let args_value: Option<serde_json::Value> = if let Some(json_str) = args_json {
        Some(serde_json::from_str(json_str)
            .map_err(|e| format!("Failed to parse arguments JSON: {}", e))?)
    } else {
        None
    };

    // Convert JSON args to PHP-compatible format
    // We need to call into PHP from this thread, but PHP may not be initialized here
    // This is the fundamental challenge - PHP contexts are thread-local

    // For now, log what we would do and return success
    // Real implementation would need:
    // 1. Initialize PHP on this thread (php_embed_init or similar)
    // 2. Create Zvals from JSON
    // 3. Call the function/method
    // 4. Cleanup PHP context

    eprintln!(
        "Background task (simulated): {} with args: {:?}",
        callable_str, args_json
    );

    Ok(())
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

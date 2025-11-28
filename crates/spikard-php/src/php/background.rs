//! Background task execution for PHP bindings.
//!
//! Uses a message queue pattern to execute tasks on the main thread after responses complete.
//! This avoids PHP threading issues while still providing non-blocking background execution.

use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;
use once_cell::sync::Lazy;
use spikard_http::BackgroundHandle;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::sync::Mutex;

/// Task stored in the queue for later execution
#[derive(Debug)]
struct QueuedTask {
    callable: Zval,
    args: Option<Zval>,
}

thread_local! {
    /// Task queue for background execution (thread-local because Zvals are not Send)
    /// Tasks are queued here and executed asynchronously on the main thread
    static TASK_QUEUE: RefCell<VecDeque<QueuedTask>> = RefCell::new(VecDeque::new());
}

static BACKGROUND_HANDLE: Lazy<Mutex<Option<BackgroundHandle>>> = Lazy::new(|| Mutex::new(None));

/// Install the background handle at server startup
pub fn install_handle(handle: BackgroundHandle) {
    if let Ok(mut guard) = BACKGROUND_HANDLE.lock() {
        *guard = Some(handle.clone());
    }

    // Note: The task runner loop is integrated into the server's main runtime
    // via process_pending_tasks() which is called periodically
}

/// Clear the background handle at server shutdown
pub fn clear_handle() {
    if let Ok(mut guard) = BACKGROUND_HANDLE.lock() {
        *guard = None;
    }
}

/// Process pending background tasks from the queue.
///
/// This should be called periodically by the server runtime to execute
/// queued background tasks. Processes one task per call, returning true
/// if a task was executed or false if the queue was empty.
///
/// # Returns
/// * `true` if a task was processed
/// * `false` if the queue was empty
pub fn process_pending_tasks() -> bool {
    // Check if background tasks are enabled
    let is_enabled = BACKGROUND_HANDLE
        .lock()
        .ok()
        .and_then(|guard| guard.as_ref().cloned())
        .is_some();

    if !is_enabled {
        return false;
    }

    // Process one task from the queue
    let task = TASK_QUEUE.with(|queue| queue.borrow_mut().pop_front());

    if let Some(task) = task {
        // Execute the task on this thread (no Send required!)
        if let Err(e) = execute_queued_task(task) {
            eprintln!("Background task failed: {}", e);
        }
        true
    } else {
        false
    }
}

/// Run a PHP callable in the background
///
/// Queues a task for execution after the HTTP response completes.
/// Tasks execute on the main thread, avoiding PHP threading issues.
///
/// # Arguments
/// * `callable` - PHP callable (closure, function name, array ['class', 'method'])
/// * `args` - Array of arguments to pass to callable (or null for no args)
///
/// # Errors
/// * Returns error if callable is not actually callable
///
/// # Example
/// ```php
/// // Without arguments
/// spikard_background_run(function() {
///     error_log("Background task complete");
/// }, null);
///
/// // With arguments
/// spikard_background_run(function($x, $y) {
///     error_log("Sum: " . ($x + $y));
/// }, [1, 2]);
/// ```
#[php_function]
#[php(name = "spikard_background_run")]
pub fn spikard_background_run(callable: &Zval, args: &Zval) -> PhpResult<()> {
    // Validate callable
    if !callable.is_callable() {
        return Err(PhpException::default(
            "First argument to spikard_background_run must be callable".to_string(),
        ));
    }

    // Clone the Zvals for queueing (shallow clone is cheap)
    let callable_owned = callable.shallow_clone();
    let args_owned = if args.is_null() {
        None
    } else {
        Some(args.shallow_clone())
    };

    // Queue the task
    let task = QueuedTask {
        callable: callable_owned,
        args: args_owned,
    };

    TASK_QUEUE.with(|queue| queue.borrow_mut().push_back(task));

    Ok(())
}

/// Execute a queued task on the main thread
fn execute_queued_task(task: QueuedTask) -> Result<(), String> {
    // Build arguments for try_call
    let args: Vec<&dyn ext_php_rs::convert::IntoZvalDyn> = if let Some(args_zval) = &task.args {
        // Extract args array if provided
        if let Some(arr) = args_zval.array() {
            arr.values()
                .map(|v| v as &dyn ext_php_rs::convert::IntoZvalDyn)
                .collect()
        } else {
            // Single argument, not an array
            vec![args_zval as &dyn ext_php_rs::convert::IntoZvalDyn]
        }
    } else {
        vec![]
    };

    // Create callable and invoke
    let callable = ext_php_rs::types::ZendCallable::new(&task.callable)
        .map_err(|e| format!("Failed to create callable: {:?}", e))?;

    callable
        .try_call(args)
        .map_err(|e| format!("Task execution failed: {:?}", e))?;

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

        let retrieved = BACKGROUND_HANDLE.lock().unwrap().clone();
        assert!(retrieved.is_some());

        clear_handle();
        assert!(BACKGROUND_HANDLE.lock().unwrap().is_none());
    }
}

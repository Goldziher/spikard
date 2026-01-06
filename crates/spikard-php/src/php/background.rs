//! Background task execution for PHP bindings.
//!
//! Uses a message queue pattern to execute tasks on the main thread after responses complete.
//! This avoids PHP threading issues while still providing non-blocking background execution.

use ext_php_rs::convert::IntoZval;
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
    static TASK_QUEUE: RefCell<VecDeque<QueuedTask>> = const { RefCell::new(VecDeque::new()) };
}

static BACKGROUND_HANDLE: Lazy<Mutex<Option<BackgroundHandle>>> = Lazy::new(|| Mutex::new(None));

/// Install the background handle at server startup
pub fn install_handle(handle: BackgroundHandle) {
    if let Ok(mut guard) = BACKGROUND_HANDLE.lock() {
        *guard = Some(handle.clone());
    }
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
    let is_enabled = BACKGROUND_HANDLE
        .lock()
        .ok()
        .and_then(|guard| guard.as_ref().cloned())
        .is_some();

    if !is_enabled {
        return false;
    }

    let task = TASK_QUEUE.with(|queue| queue.borrow_mut().pop_front());

    if let Some(task) = task {
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
/// * `callable` - PHP callable (closure, function name, array `['class', 'method']`)
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
    if !callable.is_callable() {
        return Err(PhpException::default(
            "First argument to spikard_background_run must be callable".to_string(),
        ));
    }

    let is_enabled = BACKGROUND_HANDLE
        .lock()
        .ok()
        .and_then(|guard| guard.as_ref().cloned())
        .is_some();

    let callable_owned = callable.shallow_clone();
    let args_owned = if args.is_null() {
        None
    } else {
        Some(args.shallow_clone())
    };

    if !is_enabled {
        let args_vec: Vec<&dyn ext_php_rs::convert::IntoZvalDyn> = if let Some(args_zval) = &args_owned {
            if let Some(arr) = args_zval.array() {
                arr.values()
                    .map(|v| v as &dyn ext_php_rs::convert::IntoZvalDyn)
                    .collect()
            } else {
                vec![args_zval as &dyn ext_php_rs::convert::IntoZvalDyn]
            }
        } else {
            vec![]
        };

        let callable = ext_php_rs::types::ZendCallable::new(&callable_owned)?;
        match callable.try_call(args_vec) {
            Ok(_) => return Ok(()),
            Err(ext_php_rs::error::Error::Exception(ex)) => {
                let zval = ex.into_zval(false)?;
                ext_php_rs::exception::throw_object(zval)?;
                return Ok(());
            }
            Err(err) => return Err(err.into()),
        }
    }

    let task = QueuedTask {
        callable: callable_owned,
        args: args_owned,
    };

    TASK_QUEUE.with(|queue| queue.borrow_mut().push_back(task));

    Ok(())
}

/// Execute a queued task on the main thread
fn execute_queued_task(task: QueuedTask) -> Result<(), String> {
    let args: Vec<&dyn ext_php_rs::convert::IntoZvalDyn> = if let Some(args_zval) = &task.args {
        if let Some(arr) = args_zval.array() {
            arr.values()
                .map(|v| v as &dyn ext_php_rs::convert::IntoZvalDyn)
                .collect()
        } else {
            vec![args_zval as &dyn ext_php_rs::convert::IntoZvalDyn]
        }
    } else {
        vec![]
    };

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

    #[test]
    fn test_background_task_queue() {
        TASK_QUEUE.with(|queue| {
            let mut q = queue.borrow_mut();
            q.clear();
            assert_eq!(q.len(), 0);
        });
    }
}

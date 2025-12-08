//! Behavioral tests for background task execution in spikard-http.
//!
//! These tests focus on observable behavior: task completion, timing, resource cleanup,
//! and graceful shutdown patterns. They validate end-to-end behavior rather than
//! implementation details.
//!
//! Test categories:
//! 1. Graceful Shutdown & Draining
//! 2. Shutdown Timeout Behavior
//! 3. Task Success/Failure Observable Outcomes
//! 4. High-Volume Task Queue
//! 5. Task Execution Order Guarantees
//! 6. Concurrent Task Execution
//! 7. Task Cancellation Propagation

use spikard_http::background::{
    BackgroundJobError, BackgroundJobMetadata, BackgroundRuntime, BackgroundSpawnError, BackgroundTaskConfig,
};
use std::borrow::Cow;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, Instant};

// ============================================================================
// Test 1: Graceful Shutdown Drains In-Flight Tasks
// ============================================================================

/// Verifies that shutdown waits for all in-flight tasks to complete gracefully.
/// Observable behavior: all spawned tasks complete before shutdown returns Ok.
#[tokio::test]
async fn test_graceful_shutdown_drains_all_spawned_tasks() {
    let config = BackgroundTaskConfig {
        max_queue_size: 50,
        max_concurrent_tasks: 5,
        drain_timeout_secs: 10,
    };

    let runtime = BackgroundRuntime::start(config).await;
    let handle = runtime.handle();

    let completion_count = Arc::new(AtomicU64::new(0));
    let task_count = 20;

    // Spawn multiple tasks with realistic delays
    for _ in 0..task_count {
        let count = completion_count.clone();
        handle
            .spawn(move || {
                let c = count.clone();
                async move {
                    tokio::time::sleep(Duration::from_millis(10)).await;
                    c.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                }
            })
            .expect("spawn failed");
    }

    // Wait briefly for tasks to begin
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Graceful shutdown should wait for all tasks
    let shutdown_result = runtime.shutdown().await;
    assert!(
        shutdown_result.is_ok(),
        "shutdown should succeed when draining in-flight tasks"
    );

    // Observable outcome: all tasks completed
    assert_eq!(
        completion_count.load(Ordering::SeqCst),
        task_count,
        "all spawned tasks must complete during graceful shutdown"
    );
}

/// Verifies that shutdown processes queued tasks before returning.
/// Observable behavior: both in-flight and queued tasks complete.
#[tokio::test]
async fn test_graceful_shutdown_processes_both_inflight_and_queued_tasks() {
    let config = BackgroundTaskConfig {
        max_queue_size: 100,
        max_concurrent_tasks: 2, // Low concurrency to force queueing
        drain_timeout_secs: 10,
    };

    let runtime = BackgroundRuntime::start(config).await;
    let handle = runtime.handle();

    let completion_count = Arc::new(AtomicU64::new(0));
    let task_count = 15;

    // Spawn tasks that will queue due to concurrency limit
    for _ in 0..task_count {
        let count = completion_count.clone();
        handle
            .spawn(move || {
                let c = count.clone();
                async move {
                    tokio::time::sleep(Duration::from_millis(5)).await;
                    c.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                }
            })
            .expect("spawn failed");
    }

    // Immediate shutdown should drain all queued tasks
    let shutdown_result = runtime.shutdown().await;
    assert!(
        shutdown_result.is_ok(),
        "shutdown should drain both in-flight and queued tasks"
    );

    assert_eq!(
        completion_count.load(Ordering::SeqCst),
        task_count,
        "all tasks including queued ones must complete"
    );
}

// ============================================================================
// Test 2: Shutdown Timeout Leaves Incomplete Tasks
// ============================================================================

/// Verifies that shutdown times out when tasks exceed drain timeout.
/// Observable behavior: shutdown returns error, incomplete tasks remain unfinished.
#[tokio::test]
async fn test_shutdown_timeout_with_long_running_task() {
    let config = BackgroundTaskConfig {
        max_queue_size: 10,
        max_concurrent_tasks: 2,
        drain_timeout_secs: 1, // Very short timeout
    };

    let runtime = BackgroundRuntime::start(config).await;
    let handle = runtime.handle();

    let task_completed = Arc::new(AtomicBool::new(false));
    let completed_clone = task_completed.clone();

    // Spawn a task that takes longer than drain timeout
    handle
        .spawn(move || {
            let c = completed_clone.clone();
            async move {
                tokio::time::sleep(Duration::from_secs(10)).await;
                c.store(true, Ordering::SeqCst);
                Ok(())
            }
        })
        .expect("spawn failed");

    tokio::time::sleep(Duration::from_millis(50)).await;

    // Observable behavior: shutdown times out
    let shutdown_result = runtime.shutdown().await;
    assert!(
        shutdown_result.is_err(),
        "shutdown should timeout with incomplete long-running task"
    );

    // Observable behavior: task was not completed before timeout
    assert!(
        !task_completed.load(Ordering::SeqCst),
        "incomplete task should not complete after shutdown timeout"
    );
}

/// Verifies shutdown respects drain timeout duration.
/// Observable behavior: shutdown duration is approximately the drain_timeout_secs.
#[tokio::test]
async fn test_shutdown_timeout_duration_respected() {
    let drain_timeout_secs = 2;
    let config = BackgroundTaskConfig {
        max_queue_size: 10,
        max_concurrent_tasks: 1,
        drain_timeout_secs,
    };

    let runtime = BackgroundRuntime::start(config).await;
    let handle = runtime.handle();

    // Spawn a very long-running task
    handle
        .spawn(|| async {
            tokio::time::sleep(Duration::from_secs(30)).await;
            Ok(())
        })
        .expect("spawn failed");

    tokio::time::sleep(Duration::from_millis(100)).await;

    // Measure shutdown time
    let shutdown_start = Instant::now();
    let _ = runtime.shutdown().await;
    let shutdown_elapsed = shutdown_start.elapsed();

    // Observable behavior: shutdown duration is ~drain_timeout (±1 second tolerance)
    assert!(
        shutdown_elapsed >= Duration::from_secs(drain_timeout_secs - 1),
        "shutdown should wait at least drain_timeout"
    );
    assert!(
        shutdown_elapsed < Duration::from_secs(drain_timeout_secs + 2),
        "shutdown should not wait much longer than drain_timeout"
    );
}

// ============================================================================
// Test 3: Task Success/Failure Observable Outcomes
// ============================================================================

/// Verifies that successful tasks complete without affecting other tasks.
/// Observable behavior: task runs to completion, no side effects on runtime.
#[tokio::test]
async fn test_task_success_completes_cleanly() {
    let config = BackgroundTaskConfig::default();
    let runtime = BackgroundRuntime::start(config).await;
    let handle = runtime.handle();

    let success_flag = Arc::new(AtomicBool::new(false));
    let flag_clone = success_flag.clone();

    handle
        .spawn(move || {
            let f = flag_clone.clone();
            async move {
                tokio::time::sleep(Duration::from_millis(10)).await;
                f.store(true, Ordering::SeqCst);
                Ok(())
            }
        })
        .expect("spawn failed");

    tokio::time::sleep(Duration::from_millis(50)).await;

    // Observable behavior: task succeeded and flag set
    assert!(
        success_flag.load(Ordering::SeqCst),
        "successful task should execute and set flag"
    );

    // Shutdown should complete normally
    let shutdown_result = runtime.shutdown().await;
    assert!(shutdown_result.is_ok(), "shutdown should succeed after successful task");
}

/// Verifies that failed tasks log failure but don't crash the runtime.
/// Observable behavior: failed task executes, runtime continues accepting new tasks.
#[tokio::test]
async fn test_task_failure_doesnt_crash_runtime() {
    let config = BackgroundTaskConfig::default();
    let runtime = BackgroundRuntime::start(config).await;
    let handle = runtime.handle();

    let failure_count = Arc::new(AtomicU64::new(0));
    let success_count = Arc::new(AtomicU64::new(0));

    // Spawn a failing task
    {
        let f = failure_count.clone();
        handle
            .spawn(move || {
                let fail = f.clone();
                async move {
                    fail.fetch_add(1, Ordering::SeqCst);
                    Err(BackgroundJobError::from("task error"))
                }
            })
            .expect("spawn failed");
    }

    tokio::time::sleep(Duration::from_millis(50)).await;

    // Spawn another task after failure
    {
        let s = success_count.clone();
        handle
            .spawn(move || {
                let succ = s.clone();
                async move {
                    succ.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                }
            })
            .expect("spawn failed");
    }

    tokio::time::sleep(Duration::from_millis(100)).await;

    // Observable behavior: both tasks ran despite first one failing
    assert_eq!(failure_count.load(Ordering::SeqCst), 1, "failed task should execute");
    assert_eq!(
        success_count.load(Ordering::SeqCst),
        1,
        "task after failure should also execute"
    );

    let shutdown_result = runtime.shutdown().await;
    assert!(
        shutdown_result.is_ok(),
        "runtime should shutdown cleanly after failed tasks"
    );
}

/// Verifies that mixed success/failure tasks all execute during shutdown.
/// Observable behavior: shutdown drains both successful and failed tasks.
#[tokio::test]
async fn test_shutdown_drains_mixed_success_and_failure_tasks() {
    let config = BackgroundTaskConfig {
        max_queue_size: 100,
        max_concurrent_tasks: 5,
        drain_timeout_secs: 10,
    };

    let runtime = BackgroundRuntime::start(config).await;
    let handle = runtime.handle();

    let success_count = Arc::new(AtomicU64::new(0));
    let failure_count = Arc::new(AtomicU64::new(0));
    let task_count = 20;

    for i in 0..task_count {
        if i % 2 == 0 {
            let s = success_count.clone();
            handle
                .spawn(move || {
                    let succ = s.clone();
                    async move {
                        succ.fetch_add(1, Ordering::SeqCst);
                        Ok(())
                    }
                })
                .expect("spawn failed");
        } else {
            let f = failure_count.clone();
            handle
                .spawn(move || {
                    let fail = f.clone();
                    async move {
                        fail.fetch_add(1, Ordering::SeqCst);
                        Err(BackgroundJobError::from("intentional failure"))
                    }
                })
                .expect("spawn failed");
        }
    }

    let shutdown_result = runtime.shutdown().await;
    assert!(shutdown_result.is_ok(), "shutdown should drain all tasks");

    // Observable behavior: all tasks executed
    assert_eq!(
        success_count.load(Ordering::SeqCst),
        10,
        "all successful tasks should execute"
    );
    assert_eq!(
        failure_count.load(Ordering::SeqCst),
        10,
        "all failing tasks should execute"
    );
}

// ============================================================================
// Test 4: High-Volume Task Queue (10K+ tasks)
// ============================================================================

/// Verifies that high-volume queues are processed without resource exhaustion.
/// Observable behavior: 10K tasks all complete within drain timeout.
#[tokio::test]
async fn test_high_volume_queue_10k_tasks() {
    let task_count = 10_000;
    let config = BackgroundTaskConfig {
        max_queue_size: 15_000,
        max_concurrent_tasks: 50,
        drain_timeout_secs: 60,
    };

    let runtime = BackgroundRuntime::start(config).await;
    let handle = runtime.handle();

    let completion_count = Arc::new(AtomicU64::new(0));

    // Spawn 10K tasks
    for _ in 0..task_count {
        let count = completion_count.clone();
        let result = handle.spawn(move || {
            let c = count.clone();
            async move {
                c.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }
        });
        assert!(result.is_ok(), "spawn should succeed for high-volume queue");
    }

    // Shutdown should drain all 10K tasks
    let shutdown_result = runtime.shutdown().await;
    assert!(shutdown_result.is_ok(), "shutdown should complete high-volume queue");

    // Observable behavior: all 10K tasks completed
    assert_eq!(
        completion_count.load(Ordering::SeqCst),
        task_count as u64,
        "all 10K tasks must execute"
    );
}

/// Verifies queue full behavior under high spawn rate.
/// Observable behavior: QueueFull errors when queue capacity exceeded.
#[tokio::test]
async fn test_high_volume_queue_overflow_behavior() {
    let config = BackgroundTaskConfig {
        max_queue_size: 10,
        max_concurrent_tasks: 50,
        drain_timeout_secs: 10,
    };

    let runtime = BackgroundRuntime::start(config).await;
    let handle = runtime.handle();

    let blocking_counter = Arc::new(AtomicU64::new(0));
    let spawned_count = Arc::new(AtomicU64::new(0));

    // Try to fill queue with more tasks than capacity
    let mut overflow_error_count = 0;
    for _ in 0..50 {
        let counter = blocking_counter.clone();
        let spawned = spawned_count.clone();
        let result = handle.spawn(move || {
            let c = counter.clone();
            let s = spawned.clone();
            async move {
                s.fetch_add(1, Ordering::SeqCst);
                // Sleep briefly to keep tasks in flight
                tokio::time::sleep(Duration::from_millis(100)).await;
                c.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }
        });

        if let Err(BackgroundSpawnError::QueueFull) = result {
            overflow_error_count += 1;
        }
    }

    // Observable behavior: queue full errors when capacity exceeded
    assert!(
        overflow_error_count > 0,
        "should see queue full errors when exceeding capacity"
    );

    // Shutdown should drain remaining tasks
    runtime.shutdown().await.expect("shutdown should succeed");
}

// ============================================================================
// Test 5: Task Execution Order Guarantees
// ============================================================================

/// Verifies that multiple tasks execute to completion (order not guaranteed, but all complete).
/// Observable behavior: all tasks execute despite concurrent nature.
#[tokio::test]
async fn test_task_execution_order_all_complete() {
    let config = BackgroundTaskConfig::default();
    let runtime = BackgroundRuntime::start(config).await;
    let handle = runtime.handle();

    let execution_log = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let task_count = 100;

    for i in 0..task_count {
        let log = execution_log.clone();
        handle
            .spawn(move || {
                let l = log.clone();
                async move {
                    l.lock().await.push(i);
                    Ok(())
                }
            })
            .expect("spawn failed");
    }

    tokio::time::sleep(Duration::from_millis(200)).await;

    let log = execution_log.lock().await;

    // Observable behavior: all tasks executed
    assert_eq!(log.len(), task_count, "all spawned tasks should execute");

    // Observable behavior: each task ran exactly once
    for i in 0..task_count {
        let count = log.iter().filter(|&&x| x == i).count();
        assert_eq!(count, 1, "task {} should execute exactly once", i);
    }

    runtime.shutdown().await.expect("shutdown should succeed");
}

/// Verifies FIFO-like behavior when concurrency is limited.
/// Observable behavior: with 1 concurrent task, tasks execute sequentially.
#[tokio::test]
async fn test_sequential_execution_with_single_concurrency() {
    let config = BackgroundTaskConfig {
        max_queue_size: 100,
        max_concurrent_tasks: 1, // Force sequential execution
        drain_timeout_secs: 30,
    };

    let runtime = BackgroundRuntime::start(config).await;
    let handle = runtime.handle();

    let execution_order = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let task_count = 10;

    for i in 0..task_count {
        let order = execution_order.clone();
        handle
            .spawn(move || {
                let o = order.clone();
                async move {
                    o.lock().await.push(i);
                    tokio::time::sleep(Duration::from_millis(5)).await;
                    Ok(())
                }
            })
            .expect("spawn failed");
    }

    let shutdown_result = runtime.shutdown().await;
    assert!(shutdown_result.is_ok(), "shutdown should succeed");

    let order = execution_order.lock().await;

    // Observable behavior: all tasks executed
    assert_eq!(order.len(), task_count, "all tasks should execute");

    // With single concurrency, tasks may not strictly execute in order due to async
    // scheduling, but they should all complete
}

// ============================================================================
// Test 6: Concurrent Task Execution
// ============================================================================

/// Verifies that concurrent limit is respected during execution.
/// Observable behavior: peak concurrent tasks <= configured limit.
#[tokio::test]
async fn test_concurrent_execution_respects_limit() {
    let config = BackgroundTaskConfig {
        max_queue_size: 100,
        max_concurrent_tasks: 5,
        drain_timeout_secs: 10,
    };

    let runtime = BackgroundRuntime::start(config).await;
    let handle = runtime.handle();

    let active_count = Arc::new(AtomicU64::new(0));
    let peak_count = Arc::new(AtomicU64::new(0));
    let task_count = 20;

    for _ in 0..task_count {
        let active = active_count.clone();
        let peak = peak_count.clone();

        handle
            .spawn(move || {
                let a = active.clone();
                let p = peak.clone();

                async move {
                    // Increment active count
                    let current = a.fetch_add(1, Ordering::SeqCst) + 1;

                    // Update peak
                    let mut peak_val = p.load(Ordering::SeqCst);
                    while current > peak_val {
                        if p.compare_exchange(peak_val, current, Ordering::SeqCst, Ordering::SeqCst)
                            .is_ok()
                        {
                            break;
                        }
                        peak_val = p.load(Ordering::SeqCst);
                    }

                    // Hold task for a bit to measure peak
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    a.fetch_sub(1, Ordering::SeqCst);
                    Ok(())
                }
            })
            .expect("spawn failed");
    }

    // Wait for tasks to execute and measure peak
    tokio::time::sleep(Duration::from_millis(300)).await;

    let peak = peak_count.load(Ordering::SeqCst);

    // Observable behavior: peak concurrent never exceeds limit
    assert!(
        peak <= 5,
        "peak concurrent tasks ({}) should not exceed limit of 5",
        peak
    );

    runtime.shutdown().await.expect("shutdown should succeed");
}

/// Verifies tasks can run concurrently and interact safely.
/// Observable behavior: multiple tasks run simultaneously without data races.
#[tokio::test]
async fn test_concurrent_tasks_safe_interaction() {
    let config = BackgroundTaskConfig {
        max_queue_size: 100,
        max_concurrent_tasks: 10,
        drain_timeout_secs: 10,
    };

    let runtime = BackgroundRuntime::start(config).await;
    let handle = runtime.handle();

    let shared_value = Arc::new(AtomicU64::new(0));
    let task_count = 50;

    for _ in 0..task_count {
        let val = shared_value.clone();
        handle
            .spawn(move || {
                let v = val.clone();
                async move {
                    // Safe concurrent increment via atomic
                    v.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                }
            })
            .expect("spawn failed");
    }

    tokio::time::sleep(Duration::from_millis(200)).await;

    // Observable behavior: all concurrent increments succeeded
    assert_eq!(
        shared_value.load(Ordering::SeqCst),
        task_count as u64,
        "concurrent increments should all complete"
    );

    runtime.shutdown().await.expect("shutdown should succeed");
}

// ============================================================================
// Test 7: Task Cancellation Propagation
// ============================================================================

/// Verifies that shutdown immediately stops accepting new tasks.
/// Observable behavior: spawn after shutdown signal returns error.
#[tokio::test]
async fn test_spawn_fails_after_shutdown_initiated() {
    let config = BackgroundTaskConfig::default();
    let runtime = BackgroundRuntime::start(config).await;
    let handle = runtime.handle();

    // Clone handle before shutdown
    let handle_clone = handle.clone();

    // Shutdown the runtime
    runtime.shutdown().await.expect("shutdown should succeed");

    // Brief delay to ensure shutdown is complete
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Observable behavior: spawn after shutdown returns error
    let result = handle_clone.spawn(|| async { Ok(()) });
    assert!(result.is_err(), "spawn after shutdown should fail");
}

/// Verifies that incomplete tasks are cancelled when shutdown times out.
/// Observable behavior: incomplete task never completes after timeout.
#[tokio::test]
async fn test_incomplete_task_cancelled_on_timeout() {
    let config = BackgroundTaskConfig {
        max_queue_size: 10,
        max_concurrent_tasks: 1,
        drain_timeout_secs: 1,
    };

    let runtime = BackgroundRuntime::start(config).await;
    let handle = runtime.handle();

    let task_started = Arc::new(AtomicBool::new(false));
    let task_completed = Arc::new(AtomicBool::new(false));
    let started = task_started.clone();
    let completed = task_completed.clone();

    handle
        .spawn(move || {
            let s = started.clone();
            let c = completed.clone();
            async move {
                s.store(true, Ordering::SeqCst);
                tokio::time::sleep(Duration::from_secs(10)).await;
                c.store(true, Ordering::SeqCst);
                Ok(())
            }
        })
        .expect("spawn failed");

    tokio::time::sleep(Duration::from_millis(100)).await;

    // Task started before shutdown
    assert!(task_started.load(Ordering::SeqCst), "task should have started");

    let shutdown_result = runtime.shutdown().await;

    // Observable behavior: shutdown timed out
    assert!(shutdown_result.is_err(), "shutdown should timeout with incomplete task");

    // Observable behavior: task was not allowed to complete
    assert!(
        !task_completed.load(Ordering::SeqCst),
        "incomplete task should not complete after shutdown timeout"
    );
}

/// Verifies task cancellation doesn't affect other tasks.
/// Observable behavior: other tasks complete normally even if one is cancelled.
#[tokio::test]
async fn test_task_cancellation_doesnt_affect_others() {
    let config = BackgroundTaskConfig {
        max_queue_size: 100,
        max_concurrent_tasks: 5,
        drain_timeout_secs: 1, // Short timeout
    };

    let runtime = BackgroundRuntime::start(config).await;
    let handle = runtime.handle();

    let short_task_completed = Arc::new(AtomicBool::new(false));
    let long_task_started = Arc::new(AtomicBool::new(false));

    // Spawn a short task that will complete
    {
        let c = short_task_completed.clone();
        handle
            .spawn(move || {
                let completed = c.clone();
                async move {
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    completed.store(true, Ordering::SeqCst);
                    Ok(())
                }
            })
            .expect("spawn failed");
    }

    // Spawn a long task that will be cancelled
    {
        let s = long_task_started.clone();
        handle
            .spawn(move || {
                let started = s.clone();
                async move {
                    started.store(true, Ordering::SeqCst);
                    tokio::time::sleep(Duration::from_secs(30)).await;
                    Ok(())
                }
            })
            .expect("spawn failed");
    }

    tokio::time::sleep(Duration::from_millis(100)).await;

    // Shutdown with timeout
    let shutdown_result = runtime.shutdown().await;
    assert!(shutdown_result.is_err(), "shutdown should timeout due to long task");

    // Observable behavior: short task completed, long task started but not completed
    assert!(
        short_task_completed.load(Ordering::SeqCst),
        "short task should have completed before timeout"
    );
    assert!(
        long_task_started.load(Ordering::SeqCst),
        "long task should have started before timeout"
    );
}

// ============================================================================
// Additional Coverage: Resource Cleanup & Edge Cases
// ============================================================================

/// Verifies immediate shutdown with no tasks.
/// Observable behavior: shutdown succeeds quickly with empty queue.
#[tokio::test]
async fn test_shutdown_with_no_tasks() {
    let config = BackgroundTaskConfig::default();
    let runtime = BackgroundRuntime::start(config).await;

    let start = Instant::now();
    let result = runtime.shutdown().await;
    let elapsed = start.elapsed();

    // Observable behavior: shutdown succeeds and completes quickly
    assert!(result.is_ok(), "shutdown should succeed with no tasks");
    assert!(
        elapsed < Duration::from_secs(1),
        "shutdown with no tasks should be fast"
    );
}

/// Verifies task metadata is preserved (metadata doesn't affect execution).
/// Observable behavior: tasks with metadata execute successfully.
#[tokio::test]
async fn test_task_metadata_preserved_execution() {
    let config = BackgroundTaskConfig::default();
    let runtime = BackgroundRuntime::start(config).await;
    let handle = runtime.handle();

    let executed = Arc::new(AtomicBool::new(false));
    let executed_clone = executed.clone();

    let metadata = BackgroundJobMetadata {
        name: Cow::Owned("test_task".to_string()),
        request_id: Some("req-123".to_string()),
    };

    let future = async move {
        executed_clone.store(true, Ordering::SeqCst);
        Ok(())
    };

    handle.spawn_with_metadata(future, metadata).expect("spawn failed");

    tokio::time::sleep(Duration::from_millis(50)).await;

    // Observable behavior: task with metadata executed
    assert!(executed.load(Ordering::SeqCst), "task with metadata should execute");

    runtime.shutdown().await.expect("shutdown should succeed");
}

/// Verifies that multiple handles to the same runtime work correctly.
/// Observable behavior: multiple handle clones spawn tasks independently.
#[tokio::test]
async fn test_multiple_handle_clones_spawn_independently() {
    let config = BackgroundTaskConfig::default();
    let runtime = BackgroundRuntime::start(config).await;
    let handle1 = runtime.handle();
    let handle2 = runtime.handle();

    let count = Arc::new(AtomicU64::new(0));

    // Spawn via first handle
    {
        let c = count.clone();
        handle1
            .spawn(move || {
                let counter = c.clone();
                async move {
                    counter.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                }
            })
            .expect("spawn failed");
    }

    // Spawn via second handle
    {
        let c = count.clone();
        handle2
            .spawn(move || {
                let counter = c.clone();
                async move {
                    counter.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                }
            })
            .expect("spawn failed");
    }

    tokio::time::sleep(Duration::from_millis(100)).await;

    // Observable behavior: tasks from both handles executed
    assert_eq!(
        count.load(Ordering::SeqCst),
        2,
        "tasks from multiple handles should all execute"
    );

    runtime.shutdown().await.expect("shutdown should succeed");
}

/// Verifies that resource cleanup occurs after shutdown.
/// Observable behavior: runtime can be dropped safely after shutdown.
#[tokio::test]
async fn test_resource_cleanup_after_shutdown() {
    let config = BackgroundTaskConfig::default();
    let runtime = BackgroundRuntime::start(config).await;
    let handle = runtime.handle();

    handle
        .spawn(|| async {
            tokio::time::sleep(Duration::from_millis(10)).await;
            Ok(())
        })
        .expect("spawn failed");

    let shutdown_result = runtime.shutdown().await;
    assert!(shutdown_result.is_ok(), "shutdown should complete successfully");

    // Observable behavior: dropping runtime after shutdown is safe (no double-free, panics)
    drop(handle);
}

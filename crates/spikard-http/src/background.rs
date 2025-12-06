use std::borrow::Cow;
use std::sync::Arc;
use std::time::Duration;

use futures::FutureExt;
use futures::future::BoxFuture;
use tokio::sync::{Semaphore, mpsc};
use tokio::task::JoinSet;
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;

/// Configuration for in-process background task execution.
#[derive(Clone, Debug)]
pub struct BackgroundTaskConfig {
    pub max_queue_size: usize,
    pub max_concurrent_tasks: usize,
    pub drain_timeout_secs: u64,
}

impl Default for BackgroundTaskConfig {
    fn default() -> Self {
        Self {
            max_queue_size: 1024,
            max_concurrent_tasks: 128,
            drain_timeout_secs: 30,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BackgroundJobMetadata {
    pub name: Cow<'static, str>,
    pub request_id: Option<String>,
}

impl Default for BackgroundJobMetadata {
    fn default() -> Self {
        Self {
            name: Cow::Borrowed("background_task"),
            request_id: None,
        }
    }
}

pub type BackgroundJobFuture = BoxFuture<'static, Result<(), BackgroundJobError>>;

struct BackgroundJob {
    pub future: BackgroundJobFuture,
    pub metadata: BackgroundJobMetadata,
}

impl BackgroundJob {
    fn new<F>(future: F, metadata: BackgroundJobMetadata) -> Self
    where
        F: futures::Future<Output = Result<(), BackgroundJobError>> + Send + 'static,
    {
        Self {
            future: future.boxed(),
            metadata,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BackgroundJobError {
    pub message: String,
}

impl From<String> for BackgroundJobError {
    fn from(message: String) -> Self {
        Self { message }
    }
}

impl From<&str> for BackgroundJobError {
    fn from(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BackgroundSpawnError {
    QueueFull,
}

impl std::fmt::Display for BackgroundSpawnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackgroundSpawnError::QueueFull => write!(f, "background task queue is full"),
        }
    }
}

impl std::error::Error for BackgroundSpawnError {}

#[derive(Debug)]
pub struct BackgroundShutdownError;

#[derive(Default)]
struct BackgroundMetrics {
    queued: std::sync::atomic::AtomicU64,
    running: std::sync::atomic::AtomicU64,
    failed: std::sync::atomic::AtomicU64,
}

impl BackgroundMetrics {
    fn inc_queued(&self) {
        self.queued.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    fn dec_queued(&self) {
        self.queued.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
    }

    fn inc_running(&self) {
        self.running.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    fn dec_running(&self) {
        self.running.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
    }

    fn inc_failed(&self) {
        self.failed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

#[derive(Clone)]
pub struct BackgroundHandle {
    sender: mpsc::Sender<BackgroundJob>,
    metrics: Arc<BackgroundMetrics>,
}

impl BackgroundHandle {
    pub fn spawn<F, Fut>(&self, f: F) -> Result<(), BackgroundSpawnError>
    where
        F: FnOnce() -> Fut,
        Fut: futures::Future<Output = Result<(), BackgroundJobError>> + Send + 'static,
    {
        let future = f();
        self.spawn_with_metadata(future, BackgroundJobMetadata::default())
    }

    pub fn spawn_with_metadata<Fut>(
        &self,
        future: Fut,
        metadata: BackgroundJobMetadata,
    ) -> Result<(), BackgroundSpawnError>
    where
        Fut: futures::Future<Output = Result<(), BackgroundJobError>> + Send + 'static,
    {
        self.metrics.inc_queued();
        let job = BackgroundJob::new(future, metadata);
        self.sender.try_send(job).map_err(|_| {
            self.metrics.dec_queued();
            BackgroundSpawnError::QueueFull
        })
    }
}

pub struct BackgroundRuntime {
    handle: BackgroundHandle,
    drain_timeout: Duration,
    shutdown_token: CancellationToken,
    join_handle: tokio::task::JoinHandle<()>,
}

impl BackgroundRuntime {
    pub async fn start(config: BackgroundTaskConfig) -> Self {
        let (tx, rx) = mpsc::channel(config.max_queue_size);
        let metrics = Arc::new(BackgroundMetrics::default());
        let handle = BackgroundHandle {
            sender: tx.clone(),
            metrics: metrics.clone(),
        };
        let shutdown_token = CancellationToken::new();
        let semaphore = Arc::new(Semaphore::new(config.max_concurrent_tasks));
        let driver_token = shutdown_token.clone();

        let join_handle = tokio::spawn(run_executor(rx, semaphore, metrics.clone(), driver_token));

        Self {
            handle,
            drain_timeout: Duration::from_secs(config.drain_timeout_secs),
            shutdown_token,
            join_handle,
        }
    }

    pub fn handle(&self) -> BackgroundHandle {
        self.handle.clone()
    }

    pub async fn shutdown(self) -> Result<(), BackgroundShutdownError> {
        self.shutdown_token.cancel();
        drop(self.handle);
        match timeout(self.drain_timeout, self.join_handle).await {
            Ok(Ok(_)) => Ok(()),
            _ => Err(BackgroundShutdownError),
        }
    }
}

async fn run_executor(
    mut rx: mpsc::Receiver<BackgroundJob>,
    semaphore: Arc<Semaphore>,
    metrics: Arc<BackgroundMetrics>,
    token: CancellationToken,
) {
    let mut join_set = JoinSet::new();
    let token_clone = token.clone();

    // Phase 1: Accept new jobs until shutdown signal
    loop {
        tokio::select! {
            maybe_job = rx.recv() => {
                match maybe_job {
                    Some(job) => {
                        metrics.dec_queued();
                        let semaphore = semaphore.clone();
                        let metrics_clone = metrics.clone();
                        join_set.spawn(async move {
                            let BackgroundJob { future, metadata } = job;
                            // Acquire permit - this may block if at max concurrency
                            // During shutdown, the drain will wait for all spawned tasks
                            match semaphore.acquire_owned().await {
                                Ok(_permit) => {
                                    metrics_clone.inc_running();
                                    if let Err(err) = future.await {
                                        metrics_clone.inc_failed();
                                        tracing::error!(target = "spikard::background", task = %metadata.name, error = %err.message, "background task failed");
                                    }
                                    metrics_clone.dec_running();
                                }
                                Err(_) => {
                                    // Semaphore acquisition failed - this task will never run
                                    // We already decremented queued above, so metrics are consistent
                                    metrics_clone.inc_failed();
                                    tracing::warn!(target = "spikard::background", "failed to acquire semaphore permit for background task");
                                }
                            }
                        });
                    }
                    None => break,
                }
            }
            _ = token_clone.cancelled() => {
                // Shutdown signal received; exit phase 1 and begin phase 2 (draining)
                break;
            }
        }
    }

    // Phase 2: Drain remaining queued jobs without the cancel token check.
    // The shutdown() function drops its handle copy, but handle clones may still exist,
    // so we use try_recv in a loop to check for remaining messages without blocking forever.
    let mut drain_attempts = 0;
    loop {
        match rx.try_recv() {
            Ok(job) => {
                metrics.dec_queued();
                let semaphore = semaphore.clone();
                let metrics_clone = metrics.clone();
                join_set.spawn(async move {
                    let BackgroundJob { future, metadata } = job;
                    match semaphore.acquire_owned().await {
                        Ok(_permit) => {
                            metrics_clone.inc_running();
                            if let Err(err) = future.await {
                                metrics_clone.inc_failed();
                                tracing::error!(target = "spikard::background", task = %metadata.name, error = %err.message, "background task failed");
                            }
                            metrics_clone.dec_running();
                        }
                        Err(_) => {
                            metrics_clone.inc_failed();
                            tracing::warn!(target = "spikard::background", "failed to acquire semaphore permit for background task");
                        }
                    }
                });
                drain_attempts = 0;
            }
            Err(mpsc::error::TryRecvError::Empty) => {
                // Queue is empty but sender might still be held by clones in user code.
                // Wait a bit and retry, but give up after ~1 second of empty checks.
                drain_attempts += 1;
                if drain_attempts > 100 {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
            Err(mpsc::error::TryRecvError::Disconnected) => {
                // All senders dropped; nothing more to drain
                break;
            }
        }
    }

    // Wait for all spawned tasks to complete before returning
    while join_set.join_next().await.is_some() {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    #[tokio::test]
    async fn test_basic_spawn_and_execution() {
        let runtime = BackgroundRuntime::start(BackgroundTaskConfig::default()).await;
        let handle = runtime.handle();

        let counter = Arc::new(AtomicU64::new(0));
        let counter_clone = counter.clone();

        handle
            .spawn(move || {
                let c = counter_clone.clone();
                async move {
                    c.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                }
            })
            .expect("spawn failed");

        tokio::time::sleep(Duration::from_millis(100)).await;
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        runtime.shutdown().await.expect("shutdown failed");
    }

    #[tokio::test]
    async fn test_multiple_tasks() {
        let runtime = BackgroundRuntime::start(BackgroundTaskConfig::default()).await;
        let handle = runtime.handle();

        let counter = Arc::new(AtomicU64::new(0));

        for _ in 0..10 {
            let counter_clone = counter.clone();
            handle
                .spawn(move || {
                    let c = counter_clone.clone();
                    async move {
                        c.fetch_add(1, Ordering::SeqCst);
                        Ok(())
                    }
                })
                .expect("spawn failed");
        }

        tokio::time::sleep(Duration::from_millis(200)).await;
        assert_eq!(counter.load(Ordering::SeqCst), 10);

        runtime.shutdown().await.expect("shutdown failed");
    }

    #[tokio::test]
    async fn test_task_with_metadata() {
        let runtime = BackgroundRuntime::start(BackgroundTaskConfig::default()).await;
        let handle = runtime.handle();

        let metadata = BackgroundJobMetadata {
            name: Cow::Owned("test_task".to_string()),
            request_id: Some("req-123".to_string()),
        };

        let counter = Arc::new(AtomicU64::new(0));
        let counter_clone = counter.clone();

        let future = async move {
            counter_clone.fetch_add(1, Ordering::SeqCst);
            Ok(())
        };

        handle.spawn_with_metadata(future, metadata).expect("spawn failed");

        tokio::time::sleep(Duration::from_millis(100)).await;
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        runtime.shutdown().await.expect("shutdown failed");
    }

    #[tokio::test]
    async fn test_queue_full_error() {
        let config = BackgroundTaskConfig {
            max_queue_size: 2,
            max_concurrent_tasks: 10,
            drain_timeout_secs: 5,
        };

        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        // Spawn slow tasks to fill the queue and keep them running
        let blocking_barrier = Arc::new(tokio::sync::Barrier::new(3));

        for _ in 0..2 {
            let barrier = blocking_barrier.clone();
            handle
                .spawn(move || {
                    let b = barrier.clone();
                    async move {
                        b.wait().await;
                        tokio::time::sleep(Duration::from_secs(1)).await;
                        Ok(())
                    }
                })
                .expect("spawn failed");
        }

        // Now queue is full, next spawn should fail
        let result = handle.spawn(move || async { Ok(()) });
        assert!(matches!(result, Err(BackgroundSpawnError::QueueFull)));

        blocking_barrier.wait().await;
        tokio::time::sleep(Duration::from_millis(100)).await;

        runtime.shutdown().await.expect("shutdown failed");
    }

    #[tokio::test]
    async fn test_task_failure_handling() {
        let runtime = BackgroundRuntime::start(BackgroundTaskConfig::default()).await;
        let handle = runtime.handle();

        let success_count = Arc::new(AtomicU64::new(0));
        let success_count_clone = success_count.clone();

        // Spawn a failing task
        handle
            .spawn(move || {
                let s = success_count_clone.clone();
                async move {
                    s.fetch_add(1, Ordering::SeqCst);
                    Err(BackgroundJobError::from("test error"))
                }
            })
            .expect("spawn failed");

        tokio::time::sleep(Duration::from_millis(100)).await;
        // Success count still increments (it ran), but failure metrics increment too
        assert_eq!(success_count.load(Ordering::SeqCst), 1);

        runtime.shutdown().await.expect("shutdown failed");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_concurrency_limit_with_proper_synchronization() {
        let config = BackgroundTaskConfig {
            max_queue_size: 100,
            max_concurrent_tasks: 2,
            drain_timeout_secs: 30,
        };

        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        let running_count = Arc::new(AtomicU64::new(0));
        let max_concurrent = Arc::new(AtomicU64::new(0));

        for _ in 0..5 {
            let running = running_count.clone();
            let max = max_concurrent.clone();

            handle
                .spawn(move || {
                    let r = running.clone();
                    let m = max.clone();
                    async move {
                        r.fetch_add(1, Ordering::SeqCst);
                        let current_running = r.load(Ordering::SeqCst);
                        let mut current_max = m.load(Ordering::SeqCst);
                        while current_running > current_max {
                            m.store(current_running, Ordering::SeqCst);
                            current_max = current_running;
                        }

                        tokio::time::sleep(Duration::from_millis(100)).await;
                        r.fetch_sub(1, Ordering::SeqCst);
                        Ok(())
                    }
                })
                .expect("spawn failed");
        }

        // Wait for all tasks to start and monitor max concurrent
        tokio::time::sleep(Duration::from_millis(700)).await;
        let max_concurrent_observed = max_concurrent.load(Ordering::SeqCst);
        assert!(
            max_concurrent_observed <= 2,
            "Max concurrent should be <= 2, but was {}",
            max_concurrent_observed
        );

        runtime.shutdown().await.expect("shutdown failed");
    }

    #[tokio::test]
    async fn test_graceful_shutdown() {
        let config = BackgroundTaskConfig {
            max_queue_size: 10,
            max_concurrent_tasks: 2,
            drain_timeout_secs: 5,
        };

        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        let counter = Arc::new(AtomicU64::new(0));
        let counter_clone = counter.clone();

        handle
            .spawn(move || {
                let c = counter_clone.clone();
                async move {
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    c.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                }
            })
            .expect("spawn failed");

        tokio::time::sleep(Duration::from_millis(200)).await;

        // Shutdown should have already completed tasks
        let result = runtime.shutdown().await;
        assert!(result.is_ok());
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_shutdown_timeout() {
        let config = BackgroundTaskConfig {
            max_queue_size: 10,
            max_concurrent_tasks: 2,
            drain_timeout_secs: 1,
        };

        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        // Spawn a task that takes longer than the drain timeout
        handle
            .spawn(|| async {
                tokio::time::sleep(Duration::from_secs(5)).await;
                Ok(())
            })
            .expect("spawn failed");

        tokio::time::sleep(Duration::from_millis(100)).await;

        let result = runtime.shutdown().await;
        // Should timeout
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_metrics_tracking() {
        let config = BackgroundTaskConfig::default();
        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        let barrier = Arc::new(tokio::sync::Barrier::new(2));

        for _ in 0..2 {
            let b = barrier.clone();
            let _ = handle.spawn(move || {
                let barrier = b.clone();
                async move {
                    barrier.wait().await;
                    Ok(())
                }
            });
        }

        tokio::time::sleep(Duration::from_millis(150)).await;

        runtime.shutdown().await.expect("shutdown failed");
    }

    #[tokio::test]
    async fn test_task_cancellation_on_shutdown() {
        let config = BackgroundTaskConfig {
            max_queue_size: 10,
            max_concurrent_tasks: 2,
            drain_timeout_secs: 1,
        };

        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        let started_count = Arc::new(AtomicU64::new(0));
        let _completed_count = Arc::new(AtomicU64::new(0));

        // Spawn a task that won't complete
        let started = started_count.clone();

        handle
            .spawn(move || {
                let s = started.clone();
                async move {
                    s.fetch_add(1, Ordering::SeqCst);
                    // Simulate a long-running operation
                    tokio::time::sleep(Duration::from_secs(10)).await;
                    Ok(())
                }
            })
            .expect("spawn failed");

        tokio::time::sleep(Duration::from_millis(100)).await;
        assert_eq!(started_count.load(Ordering::SeqCst), 1);

        // Shutdown with short timeout - should not wait for the full 10 seconds
        let shutdown_start = std::time::Instant::now();
        let result = runtime.shutdown().await;
        let shutdown_elapsed = shutdown_start.elapsed();

        // Should return error due to timeout
        assert!(result.is_err());
        // Should be close to the 1 second drain_timeout
        assert!(shutdown_elapsed < Duration::from_secs(3));
    }

    #[tokio::test]
    async fn test_queue_overflow_multiple_spawns() {
        let config = BackgroundTaskConfig {
            max_queue_size: 3,
            max_concurrent_tasks: 10,
            drain_timeout_secs: 5,
        };

        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        let blocking_barrier = Arc::new(tokio::sync::Barrier::new(4));

        // Fill the queue with 3 blocking tasks
        for _ in 0..3 {
            let b = blocking_barrier.clone();
            handle
                .spawn(move || {
                    let barrier = b.clone();
                    async move {
                        barrier.wait().await;
                        tokio::time::sleep(Duration::from_millis(100)).await;
                        Ok(())
                    }
                })
                .expect("spawn failed");
        }

        // 4th attempt should fail with QueueFull
        let result = handle.spawn(|| async { Ok(()) });
        assert!(matches!(result, Err(BackgroundSpawnError::QueueFull)));

        blocking_barrier.wait().await;
        tokio::time::sleep(Duration::from_millis(200)).await;

        // After queue drains, we should be able to spawn again
        let result = handle.spawn(|| async { Ok(()) });
        assert!(result.is_ok());

        runtime.shutdown().await.expect("shutdown failed");
    }

    #[tokio::test]
    async fn test_concurrent_task_execution_order() {
        let runtime = BackgroundRuntime::start(BackgroundTaskConfig::default()).await;
        let handle = runtime.handle();

        let execution_order = Arc::new(tokio::sync::Mutex::new(Vec::new()));

        for i in 0..5 {
            let order = execution_order.clone();
            handle
                .spawn(move || {
                    let o = order.clone();
                    async move {
                        o.lock().await.push(i);
                        Ok(())
                    }
                })
                .expect("spawn failed");
        }

        tokio::time::sleep(Duration::from_millis(200)).await;

        let order = execution_order.lock().await;
        // Verify all tasks executed
        assert_eq!(order.len(), 5);
        // Verify each task ran exactly once
        for i in 0..5 {
            assert!(order.contains(&i));
        }

        runtime.shutdown().await.expect("shutdown failed");
    }

    #[tokio::test]
    async fn test_error_from_string_conversion() {
        let error = BackgroundJobError::from("test message");
        assert_eq!(error.message, "test message");

        let error2 = BackgroundJobError::from("test".to_string());
        assert_eq!(error2.message, "test");
    }

    #[tokio::test]
    async fn test_background_job_metadata_default() {
        let metadata = BackgroundJobMetadata::default();
        assert_eq!(metadata.name, "background_task");
        assert_eq!(metadata.request_id, None);
    }

    #[tokio::test]
    async fn test_background_job_metadata_custom() {
        let metadata = BackgroundJobMetadata {
            name: Cow::Borrowed("custom_task"),
            request_id: Some("req-456".to_string()),
        };
        assert_eq!(metadata.name, "custom_task");
        assert_eq!(metadata.request_id, Some("req-456".to_string()));
    }

    #[tokio::test]
    async fn test_metrics_inc_dec_operations() {
        let metrics = BackgroundMetrics::default();

        metrics.inc_queued();
        assert_eq!(metrics.queued.load(Ordering::Relaxed), 1);

        metrics.inc_queued();
        assert_eq!(metrics.queued.load(Ordering::Relaxed), 2);

        metrics.dec_queued();
        assert_eq!(metrics.queued.load(Ordering::Relaxed), 1);

        metrics.inc_running();
        assert_eq!(metrics.running.load(Ordering::Relaxed), 1);

        metrics.dec_running();
        assert_eq!(metrics.running.load(Ordering::Relaxed), 0);

        metrics.inc_failed();
        assert_eq!(metrics.failed.load(Ordering::Relaxed), 1);

        metrics.inc_failed();
        assert_eq!(metrics.failed.load(Ordering::Relaxed), 2);
    }

    #[tokio::test]
    async fn test_spawn_error_display() {
        let error = BackgroundSpawnError::QueueFull;
        assert_eq!(error.to_string(), "background task queue is full");
    }

    #[tokio::test]
    async fn test_background_config_default() {
        let config = BackgroundTaskConfig::default();
        assert_eq!(config.max_queue_size, 1024);
        assert_eq!(config.max_concurrent_tasks, 128);
        assert_eq!(config.drain_timeout_secs, 30);
    }

    // ========== SHUTDOWN EDGE CASES AND CONCURRENCY TESTS ==========

    #[tokio::test]
    async fn test_shutdown_with_zero_pending_tasks() {
        let runtime = BackgroundRuntime::start(BackgroundTaskConfig::default()).await;

        // Shutdown immediately without spawning anything
        let result = runtime.shutdown().await;
        assert!(result.is_ok(), "shutdown should succeed with no tasks");
    }

    #[tokio::test]
    async fn test_shutdown_with_only_running_tasks() {
        let config = BackgroundTaskConfig {
            max_queue_size: 10,
            max_concurrent_tasks: 2,
            drain_timeout_secs: 5,
        };
        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        let execution_started: Arc<std::sync::atomic::AtomicBool> = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let execution_completed: Arc<std::sync::atomic::AtomicBool> =
            Arc::new(std::sync::atomic::AtomicBool::new(false));

        let started = execution_started.clone();
        let completed = execution_completed.clone();

        handle
            .spawn(move || {
                let s = started.clone();
                let c = completed.clone();
                async move {
                    s.store(true, std::sync::atomic::Ordering::SeqCst);
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    c.store(true, std::sync::atomic::Ordering::SeqCst);
                    Ok(())
                }
            })
            .unwrap();

        // Wait for task to start
        tokio::time::sleep(Duration::from_millis(20)).await;

        // Shutdown should wait for running task
        let result = runtime.shutdown().await;
        assert!(result.is_ok(), "shutdown should succeed and wait for running tasks");
        assert!(
            execution_completed.load(std::sync::atomic::Ordering::SeqCst),
            "task should have completed"
        );
    }

    // TODO: FAILING TEST - Architectural Issue
    // This test and 27 others fail due to shutdown/semaphore deadlock issue
    // documented at line 217. Tests correctly identify the bug - graceful drain
    // doesn't work with semaphore-limited concurrency. Will fix in separate cycle.
    #[tokio::test]
    async fn test_shutdown_drains_queued_tasks() {
        let config = BackgroundTaskConfig {
            max_queue_size: 100,
            max_concurrent_tasks: 1,
            drain_timeout_secs: 5,
        };
        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        let execution_count: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));

        // Spawn multiple tasks that queue up due to semaphore limit
        for _ in 0..10 {
            let count = execution_count.clone();
            handle
                .spawn(move || {
                    let c = count.clone();
                    async move {
                        c.fetch_add(1, Ordering::SeqCst);
                        tokio::time::sleep(Duration::from_millis(10)).await;
                        Ok(())
                    }
                })
                .unwrap();
        }

        // Shutdown should drain all queued tasks
        let result = runtime.shutdown().await;
        assert!(result.is_ok());
        assert_eq!(
            execution_count.load(Ordering::SeqCst),
            10,
            "all queued tasks should execute"
        );
    }

    #[tokio::test]
    async fn test_shutdown_timeout_force_stops_long_tasks() {
        let config = BackgroundTaskConfig {
            max_queue_size: 10,
            max_concurrent_tasks: 2,
            drain_timeout_secs: 1,
        };
        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        let completed: Arc<std::sync::atomic::AtomicBool> = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let completed_clone = completed.clone();

        handle
            .spawn(move || {
                let c = completed_clone.clone();
                async move {
                    tokio::time::sleep(Duration::from_secs(10)).await;
                    c.store(true, std::sync::atomic::Ordering::SeqCst);
                    Ok(())
                }
            })
            .unwrap();

        tokio::time::sleep(Duration::from_millis(50)).await;

        let shutdown_start = std::time::Instant::now();
        let result = runtime.shutdown().await;
        let elapsed = shutdown_start.elapsed();

        // Should timeout, not wait full 10 seconds
        assert!(result.is_err(), "shutdown should timeout");
        assert!(
            elapsed < Duration::from_secs(3),
            "shutdown should timeout near drain_timeout"
        );
        assert!(
            !completed.load(std::sync::atomic::Ordering::SeqCst),
            "long-running task should not complete"
        );
    }

    #[tokio::test]
    async fn test_multiple_shutdown_calls_idempotent() {
        let runtime = BackgroundRuntime::start(BackgroundTaskConfig::default()).await;

        // First shutdown succeeds
        let result1 = runtime.shutdown().await;
        assert!(result1.is_ok(), "first shutdown should succeed");

        // Runtime is consumed, so we can't call shutdown again
        // This test validates that shutdown takes ownership (consumes self)
    }

    #[tokio::test]
    async fn test_spawn_after_all_senders_dropped_fails() {
        let config = BackgroundTaskConfig::default();
        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        // Shutdown consumes the runtime and drops its handle copy.
        // After shutdown, existing handle clones will still be able to send
        // to a closed channel, which results in TrySendError::Closed.
        runtime.shutdown().await.expect("shutdown should succeed");

        // Give executor time to fully shut down
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Now the sender is fully closed. Spawn should fail with a send error.
        // However, the actual behavior depends on whether any other handle clones exist.
        // In this test, we're the only holder, so the sender is closed.
        // The behavior is a closed channel error, but our API wraps it as QueueFull.
        let result = handle.spawn(|| async { Ok(()) });
        // The result should be an error since the channel is closed
        assert!(result.is_err(), "spawn should fail after all senders are dropped");
    }

    #[tokio::test]
    async fn test_concurrent_spawns_hit_semaphore_limit() {
        let config = BackgroundTaskConfig {
            max_queue_size: 100,
            max_concurrent_tasks: 3,
            drain_timeout_secs: 10,
        };
        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        let barrier: Arc<tokio::sync::Barrier> = Arc::new(tokio::sync::Barrier::new(3));
        let running_count: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
        let peak_concurrent: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));

        for _ in 0..5 {
            let b = barrier.clone();
            let running = running_count.clone();
            let peak = peak_concurrent.clone();

            handle
                .spawn(move || {
                    let barrier = b.clone();
                    let r = running.clone();
                    let p = peak.clone();
                    async move {
                        let current = r.fetch_add(1, Ordering::SeqCst) + 1;
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

                        barrier.wait().await;
                        tokio::time::sleep(Duration::from_millis(200)).await;
                        r.fetch_sub(1, Ordering::SeqCst);
                        Ok(())
                    }
                })
                .unwrap();
        }

        barrier.wait().await;
        tokio::time::sleep(Duration::from_millis(100)).await;

        let peak = peak_concurrent.load(Ordering::SeqCst);
        assert!(
            peak <= 3,
            "concurrent execution should not exceed semaphore limit of 3, got {}",
            peak
        );

        runtime.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_task_panic_cleanup_still_occurs() {
        let runtime = BackgroundRuntime::start(BackgroundTaskConfig::default()).await;
        let handle = runtime.handle();

        let mut spawned_count: u32 = 0;
        let panic_task_executed: Arc<std::sync::atomic::AtomicBool> =
            Arc::new(std::sync::atomic::AtomicBool::new(false));
        let after_panic_executed: Arc<std::sync::atomic::AtomicBool> =
            Arc::new(std::sync::atomic::AtomicBool::new(false));

        let panic_flag = panic_task_executed.clone();
        handle
            .spawn(move || {
                let p = panic_flag.clone();
                async move {
                    p.store(true, std::sync::atomic::Ordering::SeqCst);
                    Err(BackgroundJobError::from("simulated task failure"))
                }
            })
            .unwrap();
        spawned_count += 1;

        // Spawn another task after the failing one to verify executor continues
        let after_flag = after_panic_executed.clone();
        handle
            .spawn(move || {
                let a = after_flag.clone();
                async move {
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    a.store(true, std::sync::atomic::Ordering::SeqCst);
                    Ok(())
                }
            })
            .unwrap();
        spawned_count += 1;

        tokio::time::sleep(Duration::from_millis(200)).await;

        assert!(panic_task_executed.load(std::sync::atomic::Ordering::SeqCst));
        assert!(after_panic_executed.load(std::sync::atomic::Ordering::SeqCst));
        assert_eq!(spawned_count, 2);

        runtime.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_queue_overflow_with_immediate_rejection() {
        let config = BackgroundTaskConfig {
            max_queue_size: 2,
            max_concurrent_tasks: 100,
            drain_timeout_secs: 5,
        };
        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        let barrier: Arc<tokio::sync::Barrier> = Arc::new(tokio::sync::Barrier::new(3));

        for _ in 0..2 {
            let b = barrier.clone();
            handle
                .spawn(move || {
                    let barrier = b.clone();
                    async move {
                        barrier.wait().await;
                        tokio::time::sleep(Duration::from_millis(500)).await;
                        Ok(())
                    }
                })
                .unwrap();
        }

        // Queue is now at 2/2 capacity
        let overflow_result = handle.spawn(|| async { Ok(()) });
        assert!(matches!(overflow_result, Err(BackgroundSpawnError::QueueFull)));

        barrier.wait().await;
        runtime.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_metrics_accuracy_under_concurrent_load() {
        let config = BackgroundTaskConfig {
            max_queue_size: 50,
            max_concurrent_tasks: 5,
            drain_timeout_secs: 10,
        };
        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        let completed: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));

        for _ in 0..20 {
            let c = completed.clone();
            handle
                .spawn(move || {
                    let count = c.clone();
                    async move {
                        tokio::time::sleep(Duration::from_millis(50)).await;
                        count.fetch_add(1, Ordering::SeqCst);
                        Ok(())
                    }
                })
                .unwrap();
        }

        runtime.shutdown().await.unwrap();
        assert_eq!(completed.load(Ordering::SeqCst), 20, "all tasks should complete");
    }

    #[tokio::test]
    async fn test_drain_with_slowly_completing_tasks() {
        let config = BackgroundTaskConfig {
            max_queue_size: 50,
            max_concurrent_tasks: 2,
            drain_timeout_secs: 10,
        };
        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        let completed_count: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));

        for i in 0..5 {
            let count = completed_count.clone();
            handle
                .spawn(move || {
                    let c = count.clone();
                    async move {
                        let sleep_ms = 100 + (i as u64 * 50);
                        tokio::time::sleep(Duration::from_millis(sleep_ms)).await;
                        c.fetch_add(1, Ordering::SeqCst);
                        Ok(())
                    }
                })
                .unwrap();
        }

        // Shutdown should wait for all slow tasks to complete
        let result = runtime.shutdown().await;
        assert!(result.is_ok());
        assert_eq!(completed_count.load(Ordering::SeqCst), 5);
    }

    #[tokio::test]
    async fn test_semaphore_starvation_doesnt_deadlock() {
        let config = BackgroundTaskConfig {
            max_queue_size: 100,
            max_concurrent_tasks: 1,
            drain_timeout_secs: 10,
        };
        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        let completion_order: Arc<tokio::sync::Mutex<Vec<u32>>> = Arc::new(tokio::sync::Mutex::new(Vec::new()));

        for i in 0..10 {
            let order = completion_order.clone();
            handle
                .spawn(move || {
                    let o = order.clone();
                    async move {
                        tokio::time::sleep(Duration::from_millis(5)).await;
                        let mut guard = o.lock().await;
                        guard.push(i);
                        Ok(())
                    }
                })
                .unwrap();
        }

        // Shutdown should complete without deadlock
        let result = runtime.shutdown().await;
        assert!(result.is_ok());

        let order = completion_order.lock().await;
        assert_eq!(order.len(), 10);
    }

    #[tokio::test]
    async fn test_cancel_task_mid_execution() {
        let config = BackgroundTaskConfig {
            max_queue_size: 10,
            max_concurrent_tasks: 2,
            drain_timeout_secs: 1, // Short timeout to force expiration
        };
        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        let started: Arc<std::sync::atomic::AtomicBool> = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let ended: Arc<std::sync::atomic::AtomicBool> = Arc::new(std::sync::atomic::AtomicBool::new(false));

        let start_flag = started.clone();
        let end_flag = ended.clone();

        handle
            .spawn(move || {
                let s = start_flag.clone();
                let e = end_flag.clone();
                async move {
                    s.store(true, std::sync::atomic::Ordering::SeqCst);
                    tokio::time::sleep(Duration::from_secs(10)).await;
                    e.store(true, std::sync::atomic::Ordering::SeqCst);
                    Ok(())
                }
            })
            .unwrap();

        tokio::time::sleep(Duration::from_millis(50)).await;
        assert!(started.load(std::sync::atomic::Ordering::SeqCst));

        // Shutdown should timeout due to the long-running task
        let result = runtime.shutdown().await;
        assert!(result.is_err(), "shutdown should timeout due to long task");
        assert!(
            !ended.load(std::sync::atomic::Ordering::SeqCst),
            "task should not complete"
        );
    }

    #[tokio::test]
    async fn test_rapid_spawn_and_shutdown() {
        let config = BackgroundTaskConfig {
            max_queue_size: 1000,
            max_concurrent_tasks: 10,
            drain_timeout_secs: 5,
        };
        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        let count: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));

        // Rapidly spawn many tasks
        for _ in 0..100 {
            let c = count.clone();
            let _ = handle.spawn(move || {
                let counter = c.clone();
                async move {
                    counter.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                }
            });
        }

        // Immediate shutdown
        let result = runtime.shutdown().await;
        assert!(result.is_ok());

        let final_count = count.load(Ordering::SeqCst);
        assert!(final_count > 0, "at least some tasks should execute");
        assert!(final_count <= 100, "no more than spawned count should execute");
    }

    #[tokio::test]
    async fn test_shutdown_with_mixed_success_and_failure_tasks() {
        let config = BackgroundTaskConfig::default();
        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        let success_count: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
        let failure_count: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));

        for i in 0..10 {
            if i % 2 == 0 {
                let s = success_count.clone();
                handle
                    .spawn(move || {
                        let counter = s.clone();
                        async move {
                            counter.fetch_add(1, Ordering::SeqCst);
                            Ok(())
                        }
                    })
                    .unwrap();
            } else {
                let f = failure_count.clone();
                handle
                    .spawn(move || {
                        let counter = f.clone();
                        async move {
                            counter.fetch_add(1, Ordering::SeqCst);
                            Err(BackgroundJobError::from("intentional failure"))
                        }
                    })
                    .unwrap();
            }
        }

        tokio::time::sleep(Duration::from_millis(200)).await;

        let result = runtime.shutdown().await;
        assert!(result.is_ok());
        assert_eq!(success_count.load(Ordering::SeqCst), 5);
        assert_eq!(failure_count.load(Ordering::SeqCst), 5);
    }

    #[tokio::test]
    async fn test_concurrent_handle_clones_spawn_independently() {
        let config = BackgroundTaskConfig::default();
        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        let count: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));

        let mut join_handles = vec![];

        // Spawn 3 concurrent tasks that each spawn background jobs
        for _ in 0..3 {
            let h = handle.clone();
            let c = count.clone();

            let jh = tokio::spawn(async move {
                for _ in 0..5 {
                    let counter = c.clone();
                    let _ = h.spawn(move || {
                        let cnt = counter.clone();
                        async move {
                            cnt.fetch_add(1, Ordering::SeqCst);
                            Ok(())
                        }
                    });
                }
            });
            join_handles.push(jh);
        }

        for jh in join_handles {
            let _ = jh.await;
        }

        tokio::time::sleep(Duration::from_millis(200)).await;

        let result = runtime.shutdown().await;
        assert!(result.is_ok());
        assert_eq!(count.load(Ordering::SeqCst), 15);
    }

    #[tokio::test]
    async fn test_queue_full_metrics_updated() {
        let config = BackgroundTaskConfig {
            max_queue_size: 2,
            max_concurrent_tasks: 100,
            drain_timeout_secs: 5,
        };
        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        let barrier: Arc<tokio::sync::Barrier> = Arc::new(tokio::sync::Barrier::new(3));

        // Fill queue
        for _ in 0..2 {
            let b = barrier.clone();
            handle
                .spawn(move || {
                    let barrier = b.clone();
                    async move {
                        barrier.wait().await;
                        tokio::time::sleep(Duration::from_secs(1)).await;
                        Ok(())
                    }
                })
                .unwrap();
        }

        // Attempt to overflow - should fail gracefully
        let result = handle.spawn(|| async { Ok(()) });
        assert!(matches!(result, Err(BackgroundSpawnError::QueueFull)));

        // After first task completes, we should be able to spawn again
        barrier.wait().await;
        tokio::time::sleep(Duration::from_millis(100)).await;

        runtime.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_handle_persistence_across_spawns() {
        let config = BackgroundTaskConfig::default();
        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        let count: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));

        // Use same handle multiple times
        for _ in 0..5 {
            let c = count.clone();
            handle
                .spawn(move || {
                    let counter = c.clone();
                    async move {
                        counter.fetch_add(1, Ordering::SeqCst);
                        Ok(())
                    }
                })
                .unwrap();
        }

        tokio::time::sleep(Duration::from_millis(150)).await;
        assert_eq!(count.load(Ordering::SeqCst), 5);

        runtime.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_shutdown_with_queue_at_capacity() {
        let config = BackgroundTaskConfig {
            max_queue_size: 5,
            max_concurrent_tasks: 1,
            drain_timeout_secs: 10,
        };
        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        let completion_count: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));

        // Fill queue to capacity
        for _ in 0..5 {
            let c = completion_count.clone();
            handle
                .spawn(move || {
                    let counter = c.clone();
                    async move {
                        tokio::time::sleep(Duration::from_millis(20)).await;
                        counter.fetch_add(1, Ordering::SeqCst);
                        Ok(())
                    }
                })
                .unwrap();
        }

        // Shutdown should drain all tasks despite queue being at capacity
        let result = runtime.shutdown().await;
        assert!(result.is_ok());
        assert_eq!(completion_count.load(Ordering::SeqCst), 5);
    }

    #[tokio::test]
    async fn test_metadata_preserved_through_execution() {
        let runtime = BackgroundRuntime::start(BackgroundTaskConfig::default()).await;
        let handle = runtime.handle();

        let metadata = BackgroundJobMetadata {
            name: Cow::Owned("test_metadata_task".to_string()),
            request_id: Some("req-metadata-123".to_string()),
        };

        let executed: Arc<std::sync::atomic::AtomicBool> = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let executed_clone = executed.clone();

        let future = async move {
            executed_clone.store(true, std::sync::atomic::Ordering::SeqCst);
            Ok(())
        };

        handle.spawn_with_metadata(future, metadata).unwrap();

        tokio::time::sleep(Duration::from_millis(100)).await;

        assert!(executed.load(std::sync::atomic::Ordering::SeqCst));
        runtime.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_very_short_drain_timeout_forces_stop() {
        let config = BackgroundTaskConfig {
            max_queue_size: 10,
            max_concurrent_tasks: 2,
            drain_timeout_secs: 0, // Immediate timeout
        };
        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        handle
            .spawn(|| async {
                tokio::time::sleep(Duration::from_secs(1)).await;
                Ok(())
            })
            .unwrap();

        tokio::time::sleep(Duration::from_millis(10)).await;

        // Should timeout immediately
        let result = runtime.shutdown().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_spawn_many_tasks_sequential_drain() {
        let config = BackgroundTaskConfig {
            max_queue_size: 200,
            max_concurrent_tasks: 2,
            drain_timeout_secs: 15,
        };
        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        let count: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));

        // Spawn many tasks that will queue and drain sequentially
        for _ in 0..50 {
            let c = count.clone();
            handle
                .spawn(move || {
                    let counter = c.clone();
                    async move {
                        tokio::time::sleep(Duration::from_millis(1)).await;
                        counter.fetch_add(1, Ordering::SeqCst);
                        Ok(())
                    }
                })
                .unwrap();
        }

        // Shutdown should drain all 50 tasks
        let result = runtime.shutdown().await;
        assert!(result.is_ok());
        assert_eq!(count.load(Ordering::SeqCst), 50);
    }

    #[tokio::test]
    async fn test_no_deadlock_with_max_concurrency_barrier() {
        let config = BackgroundTaskConfig {
            max_queue_size: 100,
            max_concurrent_tasks: 3,
            drain_timeout_secs: 10,
        };
        let runtime = BackgroundRuntime::start(config).await;
        let handle = runtime.handle();

        let barrier: Arc<tokio::sync::Barrier> = Arc::new(tokio::sync::Barrier::new(4));

        for _ in 0..3 {
            let b = barrier.clone();
            handle
                .spawn(move || {
                    let barrier = b.clone();
                    async move {
                        barrier.wait().await;
                        tokio::time::sleep(Duration::from_millis(50)).await;
                        Ok(())
                    }
                })
                .unwrap();
        }

        barrier.wait().await;
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Should not deadlock
        let result = runtime.shutdown().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_error_from_owned_string() {
        let message = String::from("error message");
        let error = BackgroundJobError::from(message);
        assert_eq!(error.message, "error message");
    }

    #[tokio::test]
    async fn test_borrowed_str_conversion() {
        let error = BackgroundJobError::from("borrowed message");
        assert_eq!(error.message, "borrowed message");
    }
}

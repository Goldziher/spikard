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

    loop {
        tokio::select! {
            _ = token.cancelled() => {
                break;
            }
            maybe_job = rx.recv() => {
                match maybe_job {
                    Some(job) => {
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
                                    tracing::warn!(target = "spikard::background", "failed to acquire semaphore permit for background task");
                                }
                            }
                        });
                    }
                    None => break,
                }
            }
        }
    }

    while join_set.join_next().await.is_some() {}
}

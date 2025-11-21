//! Resource monitoring for benchmark processes

use crate::types::{ResourceMetrics, bytes_to_mb};
use sysinfo::{Pid, System};
use tokio::time::{Duration, interval};

/// Resource sample at a point in time
#[derive(Debug, Clone)]
pub struct ResourceSample {
    pub memory_bytes: u64,
    pub cpu_percent: f64,
}

/// Monitor that tracks resource usage of a process
pub struct ResourceMonitor {
    pid: Pid,
    samples: Vec<ResourceSample>,
    system: System,
}

impl ResourceMonitor {
    /// Create a new monitor for the given process ID
    pub fn new(pid: u32) -> Self {
        Self {
            pid: Pid::from_u32(pid),
            samples: Vec::new(),
            system: System::new(),
        }
    }

    /// Take a single sample of resource usage
    pub fn sample(&mut self) -> Option<ResourceSample> {
        self.system.refresh_all();

        let process = self.system.process(self.pid)?;

        let sample = ResourceSample {
            memory_bytes: process.memory(),
            cpu_percent: process.cpu_usage() as f64,
        };

        self.samples.push(sample.clone());
        Some(sample)
    }

    /// Start monitoring in the background
    /// Returns a handle that stops monitoring when dropped
    pub fn start_monitoring(mut self, interval_ms: u64) -> MonitorHandle {
        let (tx, mut rx) = tokio::sync::oneshot::channel::<()>();

        let handle = tokio::spawn(async move {
            let mut ticker = interval(Duration::from_millis(interval_ms));

            loop {
                tokio::select! {
                    _ = ticker.tick() => {
                        if self.sample().is_none() {
                            break;
                        }
                    }
                    _ = &mut rx => {
                        break;
                    }
                }
            }

            self
        });

        MonitorHandle {
            handle,
            shutdown: Some(tx),
        }
    }

    /// Calculate aggregate metrics from samples
    pub fn calculate_metrics(&self) -> ResourceMetrics {
        if self.samples.is_empty() {
            return ResourceMetrics {
                avg_memory_mb: 0.0,
                peak_memory_mb: 0.0,
                p50_memory_mb: 0.0,
                p95_memory_mb: 0.0,
                p99_memory_mb: 0.0,
                avg_cpu_percent: 0.0,
                peak_cpu_percent: 0.0,
            };
        }

        let mut memory_sorted: Vec<u64> = self.samples.iter().map(|s| s.memory_bytes).collect();
        memory_sorted.sort_unstable();

        let memory_percentile = |p: f64| -> u64 {
            let index = ((memory_sorted.len() as f64) * p / 100.0) as usize;
            memory_sorted[index.min(memory_sorted.len() - 1)]
        };

        let avg_memory = self.samples.iter().map(|s| s.memory_bytes).sum::<u64>() as f64 / self.samples.len() as f64;
        let peak_memory = *memory_sorted.last().unwrap();

        let avg_cpu = self.samples.iter().map(|s| s.cpu_percent).sum::<f64>() / self.samples.len() as f64;
        let peak_cpu = self.samples.iter().map(|s| s.cpu_percent).fold(0.0, f64::max);

        ResourceMetrics {
            avg_memory_mb: bytes_to_mb(avg_memory as u64),
            peak_memory_mb: bytes_to_mb(peak_memory),
            p50_memory_mb: bytes_to_mb(memory_percentile(50.0)),
            p95_memory_mb: bytes_to_mb(memory_percentile(95.0)),
            p99_memory_mb: bytes_to_mb(memory_percentile(99.0)),
            avg_cpu_percent: avg_cpu,
            peak_cpu_percent: peak_cpu,
        }
    }

    /// Calculate CPU percentile from samples
    pub fn cpu_percentile(&self, p: f64) -> f64 {
        if self.samples.is_empty() {
            return 0.0;
        }

        let mut cpu_sorted: Vec<f64> = self.samples.iter().map(|s| s.cpu_percent).collect();
        cpu_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let index = ((cpu_sorted.len() as f64) * p / 100.0) as usize;
        cpu_sorted[index.min(cpu_sorted.len() - 1)]
    }

    /// Get all samples
    pub fn samples(&self) -> &[ResourceSample] {
        &self.samples
    }
}

/// Handle for a monitoring task
pub struct MonitorHandle {
    handle: tokio::task::JoinHandle<ResourceMonitor>,
    shutdown: Option<tokio::sync::oneshot::Sender<()>>,
}

impl MonitorHandle {
    /// Stop monitoring and get the final metrics
    pub async fn stop(self) -> ResourceMonitor {
        use std::mem::ManuallyDrop;
        let mut me = ManuallyDrop::new(self);

        let handle = unsafe { std::ptr::read(&me.handle) };
        let shutdown = me.shutdown.take();

        if let Some(tx) = shutdown {
            let _ = tx.send(());
        }

        handle.await.expect("Monitor task panicked")
    }
}

impl Drop for MonitorHandle {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown.take() {
            let _ = tx.send(());
        }
    }
}

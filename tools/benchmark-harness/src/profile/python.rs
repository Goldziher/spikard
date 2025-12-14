//! Python profiler integration
//!
//! Supports multiple profiling approaches:
//! - Application instrumentation for GC and timing metrics
//! - Optional speedscope output produced by in-process profilers

use crate::error::Result;
use serde::Deserialize;
use std::path::PathBuf;
use std::process::Child;

/// Python profiler handle
pub struct PythonProfiler {
    process: Option<Child>,
    output_path: String,
    pid: u32,
}

/// Metrics collected from Python application instrumentation
#[derive(Debug, Deserialize)]
struct PythonMetricsFile {
    gc_collections: Option<u64>,
    gc_time_ms: Option<f64>,
    handler_time_ms: Option<f64>,
    serialization_time_ms: Option<f64>,
    ffi_overhead_ms: Option<f64>,
}

/// Start Python profiler for the given PID
pub fn start_profiler(pid: u32, output_path: Option<PathBuf>) -> Result<PythonProfiler> {
    let desired_output_path = output_path
        .as_ref()
        .and_then(|p| p.to_str().map(|s| s.to_string()))
        .or_else(|| std::env::var("SPIKARD_PYSPY_OUTPUT").ok());
    let output_path = desired_output_path.unwrap_or_else(|| format!("/tmp/python-profile-{}.speedscope.json", pid));
    if let Some(parent) = PathBuf::from(&output_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    Ok(PythonProfiler {
        process: None,
        output_path,
        pid,
    })
}

impl PythonProfiler {
    /// Get the output path for profiler data
    pub fn output_path(&self) -> Option<&str> {
        if self.output_path.is_empty() {
            None
        } else {
            Some(&self.output_path)
        }
    }

    /// Stop the profiler and collect final metrics
    pub fn stop(mut self) -> ProfilingData {
        let metrics_path = std::env::var("SPIKARD_METRICS_FILE")
            .ok()
            .unwrap_or_else(|| format!("/tmp/python-metrics-{}.json", self.pid));

        #[cfg(unix)]
        {
            unsafe {
                libc::kill(self.pid as i32, libc::SIGUSR1);
            }
            let start = std::time::Instant::now();
            while start.elapsed() < std::time::Duration::from_secs(2) {
                if std::fs::metadata(&metrics_path).is_ok() {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }

        let mut profiler_exit_status = None;
        if let Some(mut process) = self.process.take() {
            #[cfg(unix)]
            {
                unsafe {
                    libc::kill(process.id() as i32, libc::SIGINT);
                }
            }

            #[cfg(not(unix))]
            {
                let _ = process.kill();
            }

            use std::time::{Duration, Instant};
            let start = Instant::now();
            let timeout = Duration::from_secs(10);

            loop {
                match process.try_wait() {
                    Ok(Some(status)) => {
                        profiler_exit_status = Some(status);
                        break;
                    }
                    Ok(None) => {
                        if start.elapsed() > timeout {
                            eprintln!("  ⚠ Profiler did not exit within timeout, terminating");
                            #[cfg(unix)]
                            unsafe {
                                libc::kill(process.id() as i32, libc::SIGTERM);
                            }
                            let _ = process.kill();
                            break;
                        }
                        std::thread::sleep(Duration::from_millis(100));
                    }
                    Err(_) => break,
                }
            }
        }

        let app_metrics = self.load_metrics_file(&metrics_path);

        if app_metrics.is_none() {
            eprintln!("  ⚠ Python metrics file not found or unreadable at {}", metrics_path);
        }

        let flamegraph_path = self.output_path().and_then(|p| {
            let start = std::time::Instant::now();
            while start.elapsed() < std::time::Duration::from_secs(2) {
                if std::fs::metadata(p).is_ok() {
                    return Some(p.to_string());
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            eprintln!("  ⚠ Python profile output not found at {}", p);
            if let Some(status) = profiler_exit_status {
                eprintln!("  → profiler exit status: {}", status);
            }
            None
        });

        ProfilingData {
            flamegraph_path,
            gc_collections: app_metrics.as_ref().and_then(|m| m.gc_collections),
            gc_time_ms: app_metrics.as_ref().and_then(|m| m.gc_time_ms),
            handler_time_ms: app_metrics.as_ref().and_then(|m| m.handler_time_ms),
            serialization_time_ms: app_metrics.as_ref().and_then(|m| m.serialization_time_ms),
            ffi_overhead_ms: app_metrics.as_ref().and_then(|m| m.ffi_overhead_ms),
            gil_wait_time_ms: None,
            gil_contention_percent: None,
        }
    }

    /// Load metrics from application instrumentation file
    fn load_metrics_file(&self, path: &str) -> Option<PythonMetricsFile> {
        match std::fs::read_to_string(path) {
            Ok(content) => match serde_json::from_str::<PythonMetricsFile>(&content) {
                Ok(metrics) => {
                    println!("  ✓ Loaded application metrics from {}", path);
                    Some(metrics)
                }
                Err(e) => {
                    eprintln!("  ⚠ Failed to parse metrics file: {}", e);
                    None
                }
            },
            Err(_) => None,
        }
    }
}

/// Collected profiling data
pub struct ProfilingData {
    pub flamegraph_path: Option<String>,
    pub gc_collections: Option<u64>,
    pub gc_time_ms: Option<f64>,
    pub handler_time_ms: Option<f64>,
    pub serialization_time_ms: Option<f64>,
    pub ffi_overhead_ms: Option<f64>,
    pub gil_wait_time_ms: Option<f64>,
    pub gil_contention_percent: Option<f64>,
}

impl Drop for PythonProfiler {
    fn drop(&mut self) {
        if let Some(ref mut process) = self.process {
            let _ = process.kill();
        }
    }
}

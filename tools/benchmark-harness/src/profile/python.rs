//! Python profiler integration
//!
//! Supports multiple profiling approaches:
//! - Application instrumentation for GC and timing metrics
//! - Optional speedscope output produced by in-app instrumentation (e.g. `pyinstrument`)

use crate::error::Result;
use serde::Deserialize;
use std::path::PathBuf;
use std::time::Duration;

/// Python profiler handle
pub struct PythonProfiler {
    pid: u32,
    output_path: Option<String>,
}

/// Metrics collected from Python application instrumentation
#[derive(Debug, Deserialize)]
pub struct PythonAppMetrics {
    pub gc_collections: Option<u64>,
    pub gc_time_ms: Option<f64>,
    pub handler_time_ms: Option<f64>,
    pub serialization_time_ms: Option<f64>,
    pub ffi_overhead_ms: Option<f64>,
}

pub fn collect_app_metrics(pid: u32) -> Option<PythonAppMetrics> {
    let metrics_path = std::env::var("SPIKARD_METRICS_FILE")
        .ok()
        .unwrap_or_else(|| format!("/tmp/python-metrics-{}.json", pid));

    #[cfg(unix)]
    {
        unsafe {
            if libc::kill(-(pid as i32), libc::SIGUSR1) != 0 {
                let err = std::io::Error::last_os_error();
                if err.raw_os_error() == Some(libc::ESRCH) {
                    libc::kill(pid as i32, libc::SIGUSR1);
                }
            }
        }
        let start = std::time::Instant::now();
        while start.elapsed() < Duration::from_secs(2) {
            if std::fs::metadata(&metrics_path).is_ok() {
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    match std::fs::read_to_string(&metrics_path) {
        Ok(content) => match serde_json::from_str::<PythonAppMetrics>(&content) {
            Ok(metrics) => {
                println!("  ✓ Loaded application metrics from {}", metrics_path);
                Some(metrics)
            }
            Err(e) => {
                eprintln!("  ⚠ Failed to parse metrics file: {}", e);
                None
            }
        },
        Err(_) => {
            eprintln!("  ⚠ Python metrics file not found or unreadable at {}", metrics_path);
            None
        }
    }
}

pub fn wait_for_profile_output(path: &str) -> Option<String> {
    let start = std::time::Instant::now();
    while start.elapsed() < Duration::from_secs(5) {
        if std::fs::metadata(path).is_ok() {
            return Some(path.to_string());
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    eprintln!("  ⚠ Python profile output not found at {}", path);
    None
}

/// Start Python profiler for the given PID
pub fn start_profiler(pid: u32, output_path: Option<PathBuf>) -> Result<PythonProfiler> {
    let output_path = output_path
        .as_ref()
        .and_then(|p| p.to_str().map(|s| s.to_string()))
        .or_else(|| std::env::var("SPIKARD_PYTHON_PROFILE_OUTPUT").ok());

    if let Some(ref path) = output_path
        && let Some(parent) = PathBuf::from(path).parent()
    {
        let _ = std::fs::create_dir_all(parent);
    }

    Ok(PythonProfiler { pid, output_path })
}

impl PythonProfiler {
    /// Get the output path for profiler data
    pub fn output_path(&self) -> Option<&str> {
        self.output_path.as_deref()
    }

    /// Stop the profiler and collect final metrics
    pub fn stop(self) -> ProfilingData {
        let app_metrics = collect_app_metrics(self.pid);

        let flamegraph_path = self.output_path().and_then(wait_for_profile_output);

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

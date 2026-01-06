//! Rust profiler integration
//!
//! Supports multiple profiling approaches:
//! - perf for CPU profiling on Linux
//! - Instruments for CPU profiling on macOS
//! - Heap allocation tracking

use crate::error::Result;
use serde::Deserialize;
use std::process::Child;

/// Rust profiler handle
pub struct RustProfiler {
    process: Option<Child>,
    output_path: String,
    pid: u32,
}

/// Metrics collected from Rust application instrumentation
#[derive(Debug, Deserialize)]
struct RustMetricsFile {
    heap_allocated_mb: Option<f64>,
}

/// Start Rust profiler for the given PID
pub fn start_profiler(pid: u32) -> Result<RustProfiler> {
    #[cfg(target_os = "linux")]
    {
        if which::which("perf").is_ok() {
            eprintln!("  ℹ Rust profiling via perf");
            eprintln!("  → CPU profiling available");
            eprintln!("  → Run: perf record -p {pid} -g");
        } else {
            eprintln!("  ⚠ perf not found (install: apt-get install linux-tools-common)");
        }
    }

    #[cfg(target_os = "macos")]
    {
        eprintln!("  ℹ Rust profiling via Instruments");
        eprintln!("  → CPU profiling available");
        eprintln!("  → Run: xcrun xctrace record --template 'Time Profiler' --attach {pid}");
    }

    eprintln!("  → Heap metrics will be collected if instrumented");

    Ok(RustProfiler {
        process: None,
        output_path: String::new(),
        pid,
    })
}

impl RustProfiler {
    /// Get the output path for profiler data
    #[must_use]
    pub fn output_path(&self) -> Option<&str> {
        if self.output_path.is_empty() {
            None
        } else {
            Some(&self.output_path)
        }
    }

    /// Stop the profiler and collect final metrics
    #[must_use]
    pub fn stop(mut self) -> ProfilingData {
        if let Some(mut process) = self.process.take() {
            let _ = process.kill();
            let _ = process.wait();
        }

        let metrics_path = format!("/tmp/rust-metrics-{}.json", self.pid);
        let app_metrics = self.load_metrics_file(&metrics_path);

        ProfilingData {
            flamegraph_path: self.output_path().map(std::string::ToString::to_string),
            heap_allocated_mb: app_metrics.as_ref().and_then(|m| m.heap_allocated_mb),
        }
    }

    /// Load metrics from application instrumentation file
    fn load_metrics_file(&self, path: &str) -> Option<RustMetricsFile> {
        match std::fs::read_to_string(path) {
            Ok(content) => match serde_json::from_str::<RustMetricsFile>(&content) {
                Ok(metrics) => {
                    println!("  ✓ Loaded Rust application metrics from {path}");
                    Some(metrics)
                }
                Err(e) => {
                    eprintln!("  ⚠ Failed to parse Rust metrics file: {e}");
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
    pub heap_allocated_mb: Option<f64>,
}

impl Drop for RustProfiler {
    fn drop(&mut self) {
        if let Some(ref mut process) = self.process {
            let _ = process.kill();
        }
    }
}

//! Python profiler integration
//!
//! Supports multiple profiling approaches:
//! - py-spy for sampling profiler with flamegraphs
//! - Application instrumentation for GC and timing metrics
//! - Combined metrics from both sources

use crate::error::Result;
use serde::Deserialize;
use std::process::{Child, Command};

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
pub fn start_profiler(pid: u32) -> Result<PythonProfiler> {
    let desired_output_path = std::env::var("SPIKARD_PYSPY_OUTPUT").ok();
    if which::which("py-spy").is_ok() {
        let output_path = desired_output_path.unwrap_or_else(|| format!("/tmp/py-spy-{}.json", pid));

        match Command::new("py-spy")
            .arg("record")
            .arg("--pid")
            .arg(pid.to_string())
            .arg("--output")
            .arg(&output_path)
            .arg("--format")
            .arg("speedscope")
            .arg("--rate")
            .arg("100")
            .arg("--nonblocking")
            .spawn()
        {
            Ok(process) => {
                println!("  ✓ py-spy profiler started (output: {})", output_path);
                return Ok(PythonProfiler {
                    process: Some(process),
                    output_path,
                    pid,
                });
            }
            Err(e) => {
                eprintln!("  ⚠ Failed to start py-spy: {}", e);
            }
        }
    } else {
        eprintln!("  ⚠ py-spy not found (install: pip install py-spy)");
        eprintln!("  → Profiling will be limited to application metrics");
    }

    Ok(PythonProfiler {
        process: None,
        output_path: String::new(),
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
        if let Some(mut process) = self.process.take() {
            #[cfg(unix)]
            {
                unsafe {
                    libc::kill(process.id() as i32, libc::SIGTERM);
                }
            }

            #[cfg(not(unix))]
            {
                let _ = process.kill();
            }

            use std::time::{Duration, Instant};
            let start = Instant::now();
            let timeout = Duration::from_secs(3);

            loop {
                match process.try_wait() {
                    Ok(Some(_)) => break,
                    Ok(None) => {
                        if start.elapsed() > timeout {
                            eprintln!("  ⚠ Profiler did not exit within timeout, killing");
                            let _ = process.kill();
                            break;
                        }
                        std::thread::sleep(Duration::from_millis(100));
                    }
                    Err(_) => break,
                }
            }
        }

        let metrics_path = format!("/tmp/python-metrics-{}.json", self.pid);
        let app_metrics = self.load_metrics_file(&metrics_path);

        ProfilingData {
            flamegraph_path: self.output_path().map(|s| s.to_string()),
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

//! Python profiler integration
//!
//! Supports multiple profiling approaches:
//! - Application instrumentation for GC and timing metrics
//! - Optional speedscope output produced by `py-spy`

use crate::error::Result;
use serde::Deserialize;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::Duration;

/// Python profiler handle
pub struct PythonProfiler {
    process: Option<Child>,
    output_path: String,
    pid: u32,
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
            libc::kill(pid as i32, libc::SIGUSR1);
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
    let desired_output_path = output_path
        .as_ref()
        .and_then(|p| p.to_str().map(|s| s.to_string()))
        .or_else(|| std::env::var("SPIKARD_PYTHON_PROFILE_OUTPUT").ok())
        .or_else(|| std::env::var("SPIKARD_PYSPY_OUTPUT").ok());
    let output_path = desired_output_path.unwrap_or_else(|| format!("/tmp/python-profile-{}.speedscope.json", pid));
    if let Some(parent) = PathBuf::from(&output_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let process = Command::new("py-spy")
        .args([
            "record",
            "--pid",
            &pid.to_string(),
            "--format",
            "speedscope",
            "--output",
            &output_path,
            "--rate",
            "100",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()?;

    Ok(PythonProfiler {
        process: Some(process),
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
                        if !status.success()
                            && let Some(mut stderr) = process.stderr.take()
                        {
                            use std::io::Read;
                            let mut buf = String::new();
                            if stderr.read_to_string(&mut buf).is_ok() {
                                let text = buf.trim();
                                if !text.is_empty() {
                                    eprintln!("  ⚠ py-spy stderr:\n{}", text);
                                }
                            }
                        }
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

        let app_metrics = collect_app_metrics(self.pid);

        let flamegraph_path = self.output_path().and_then(wait_for_profile_output);
        if flamegraph_path.is_none()
            && let Some(status) = profiler_exit_status
        {
            eprintln!("  → profiler exit status: {}", status);
        }

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

impl Drop for PythonProfiler {
    fn drop(&mut self) {
        if let Some(ref mut process) = self.process {
            let _ = process.kill();
        }
    }
}

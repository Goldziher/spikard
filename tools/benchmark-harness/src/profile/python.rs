//! Python profiler integration
//!
//! Supports multiple profiling approaches:
//! - Application instrumentation for GC and timing metrics
//! - Sampling via `py-spy` (speedscope JSON)

use crate::error::Error;
use crate::error::Result;
use serde::Deserialize;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::Duration;

/// Python profiler handle
pub struct PythonProfiler {
    pid: u32,
    output_path: Option<PathBuf>,
    child: Option<Child>,
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
        while start.elapsed() < Duration::from_secs(10) {
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
    while start.elapsed() < Duration::from_secs(30) {
        if std::fs::metadata(path).is_ok() {
            return Some(path.to_string());
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    eprintln!("  ⚠ Python profile output not found at {}", path);
    None
}

/// Start Python profiler for the given PID
pub fn start_profiler(pid: u32, output_path: Option<PathBuf>, duration_secs: u64) -> Result<PythonProfiler> {
    let output_path = output_path.or_else(|| std::env::var("SPIKARD_PYTHON_PROFILE_OUTPUT").ok().map(PathBuf::from));

    let child = if let Some(ref path) = output_path {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let record_secs = duration_secs.min(5).max(1);
        let mut cmd = Command::new("py-spy");
        cmd.args([
            "record",
            "--pid",
            &pid.to_string(),
            "--format",
            "speedscope",
            "--output",
            &path.display().to_string(),
            "--duration",
            &record_secs.to_string(),
            "--rate",
            "100",
        ]);
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
        Some(cmd.spawn().map_err(|e| {
            Error::BenchmarkFailed(format!("Failed to start py-spy profiler for pid {}: {}", pid, e))
        })?)
    } else {
        None
    };

    Ok(PythonProfiler {
        pid,
        output_path,
        child,
    })
}

impl PythonProfiler {
    /// Get the output path for profiler data
    pub fn output_path(&self) -> Option<&PathBuf> {
        self.output_path.as_ref()
    }

    /// Stop the profiler and collect final metrics
    pub fn stop(self) -> ProfilingData {
        let PythonProfiler {
            output_path, child, ..
        } = self;

        if let Some(mut child) = child {
            let _ = child.wait();
        }

        let flamegraph_path = output_path
            .as_ref()
            .and_then(|p| p.to_str())
            .and_then(wait_for_profile_output);

        ProfilingData {
            flamegraph_path,
            gc_collections: None,
            gc_time_ms: None,
            handler_time_ms: None,
            serialization_time_ms: None,
            ffi_overhead_ms: None,
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

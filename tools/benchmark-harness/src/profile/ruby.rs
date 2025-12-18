//! Ruby profiler integration
//!
//! Supports multiple profiling approaches:
//! - rbspy sampling for CPU profiling with flamegraphs
//! - GC metrics collection
//! - Heap metrics collection

use crate::error::{Error, Result};
use serde::Deserialize;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::Duration;

/// Ruby profiler handle
pub struct RubyProfiler {
    output_path: Option<PathBuf>,
    stderr_path: Option<PathBuf>,
    child: Option<Child>,
    pid: u32,
}

/// Metrics collected from Ruby application instrumentation
#[derive(Debug, Deserialize)]
struct RubyMetricsFile {
    gc_count: Option<u64>,
    gc_time_ms: Option<f64>,
    heap_allocated_pages: Option<u64>,
    heap_live_slots: Option<u64>,
}

/// Start Ruby profiler for the given PID
pub fn start_profiler(pid: u32, output_path: Option<PathBuf>, duration_secs: u64) -> Result<RubyProfiler> {
    let (child, stderr_path) = if let Some(ref path) = output_path {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let record_secs = duration_secs.clamp(1, 5);
        let stderr_path = path.with_extension("speedscope.stderr.log");
        let stderr_file = std::fs::File::create(&stderr_path)?;

        let mut cmd = Command::new("rbspy");
        cmd.args([
            "record",
            "--pid",
            &pid.to_string(),
            "--format",
            "speedscope",
            "--file",
            &path.display().to_string(),
            "--duration",
            &record_secs.to_string(),
            "--rate",
            "100",
            "--nonblocking",
            "--silent",
        ]);
        cmd.stdout(Stdio::null()).stderr(Stdio::from(stderr_file));
        let child = cmd
            .spawn()
            .map_err(|e| Error::BenchmarkFailed(format!("Failed to start rbspy profiler for pid {}: {}", pid, e)))?;
        (Some(child), Some(stderr_path))
    } else {
        (None, None)
    };

    Ok(RubyProfiler {
        output_path,
        stderr_path,
        child,
        pid,
    })
}

impl RubyProfiler {
    /// Get the output path for profiler data
    pub fn output_path(&self) -> Option<&PathBuf> {
        self.output_path.as_ref()
    }

    /// Stop the profiler and collect final metrics
    pub fn stop(mut self) -> ProfilingData {
        if let Some(mut child) = self.child.take() {
            let _ = child.wait();
        }

        let metrics_path = std::env::var("SPIKARD_RUBY_METRICS_FILE")
            .ok()
            .unwrap_or_else(|| format!("/tmp/ruby-metrics-{}.json", self.pid));
        let app_metrics = self.load_metrics_file(&metrics_path);

        let flamegraph_path = self
            .output_path
            .as_ref()
            .and_then(|p| p.to_str())
            .and_then(wait_for_profile_output);

        if flamegraph_path.is_none()
            && let (Some(output_path), Some(stderr_path)) = (self.output_path.as_ref(), self.stderr_path.as_ref())
            && stderr_path.exists()
        {
            eprintln!(
                "  ⚠ rbspy did not produce profile output at {}; see {}",
                output_path.display(),
                stderr_path.display()
            );
        }

        ProfilingData {
            flamegraph_path,
            gc_count: app_metrics.as_ref().and_then(|m| m.gc_count),
            gc_time_ms: app_metrics.as_ref().and_then(|m| m.gc_time_ms),
            heap_allocated_pages: app_metrics.as_ref().and_then(|m| m.heap_allocated_pages),
            heap_live_slots: app_metrics.as_ref().and_then(|m| m.heap_live_slots),
        }
    }

    /// Load metrics from application instrumentation file
    fn load_metrics_file(&self, path: &str) -> Option<RubyMetricsFile> {
        match std::fs::read_to_string(path) {
            Ok(content) => match serde_json::from_str::<RubyMetricsFile>(&content) {
                Ok(metrics) => {
                    println!("  ✓ Loaded Ruby application metrics from {}", path);
                    Some(metrics)
                }
                Err(e) => {
                    eprintln!("  ⚠ Failed to parse Ruby metrics file: {}", e);
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
    pub gc_count: Option<u64>,
    pub gc_time_ms: Option<f64>,
    pub heap_allocated_pages: Option<u64>,
    pub heap_live_slots: Option<u64>,
}

pub fn wait_for_profile_output(path: &str) -> Option<String> {
    let start = std::time::Instant::now();
    while start.elapsed() < Duration::from_secs(30) {
        if std::fs::metadata(path).is_ok() {
            return Some(path.to_string());
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    None
}

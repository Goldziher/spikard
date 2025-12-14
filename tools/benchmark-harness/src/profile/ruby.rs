//! Ruby profiler integration
//!
//! Supports multiple profiling approaches:
//! - stackprof for CPU profiling with flamegraphs
//! - GC metrics collection
//! - Heap metrics collection

use crate::error::Result;
use serde::Deserialize;
use std::process::Child;

/// Ruby profiler handle
pub struct RubyProfiler {
    process: Option<Child>,
    output_path: String,
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
pub fn start_profiler(pid: u32) -> Result<RubyProfiler> {
    eprintln!("  ℹ Ruby profiling via application instrumentation");
    eprintln!("  → GC and heap metrics will be collected");
    eprintln!("  → For CPU profiling, add: require 'stackprof'");

    Ok(RubyProfiler {
        process: None,
        output_path: String::new(),
        pid,
    })
}

impl RubyProfiler {
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
            let _ = process.kill();
            let _ = process.wait();
        }

        let metrics_path = format!("/tmp/ruby-metrics-{}.json", self.pid);
        let app_metrics = self.load_metrics_file(&metrics_path);

        ProfilingData {
            flamegraph_path: self.output_path().map(|s| s.to_string()),
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

impl Drop for RubyProfiler {
    fn drop(&mut self) {
        if let Some(ref mut process) = self.process {
            let _ = process.kill();
        }
    }
}

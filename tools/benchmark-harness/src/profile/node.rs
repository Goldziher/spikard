//! Node.js profiler integration
//!
//! Supports multiple profiling approaches:
//! - Node --prof for CPU profiling with flamegraphs
//! - V8 heap metrics collection
//! - Event loop lag monitoring
//! - GC metrics collection

use crate::error::Result;
use serde::Deserialize;
use std::process::Child;

/// Node profiler handle
pub struct NodeProfiler {
    process: Option<Child>,
    output_path: String,
    pid: u32,
}

/// Metrics collected from Node application instrumentation
#[derive(Debug, Deserialize)]
struct NodeMetricsFile {
    v8_heap_used_mb: Option<f64>,
    v8_heap_total_mb: Option<f64>,
    event_loop_lag_ms: Option<f64>,
    gc_time_ms: Option<f64>,
}

/// Start Node profiler for the given PID
pub fn start_profiler(pid: u32) -> Result<NodeProfiler> {
    eprintln!("  ℹ Node profiling via application instrumentation");
    eprintln!("  → V8 heap and event loop metrics will be collected");
    eprintln!("  → For CPU profiling, restart with: node --prof app.js");

    Ok(NodeProfiler {
        process: None,
        output_path: String::new(),
        pid,
    })
}

impl NodeProfiler {
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

        let metrics_path = std::env::var("SPIKARD_NODE_METRICS_FILE")
            .ok()
            .unwrap_or_else(|| format!("/tmp/node-metrics-{}.json", self.pid));
        let app_metrics = self.load_metrics_file(&metrics_path);

        ProfilingData {
            flamegraph_path: self.output_path().map(std::string::ToString::to_string),
            v8_heap_used_mb: app_metrics.as_ref().and_then(|m| m.v8_heap_used_mb),
            v8_heap_total_mb: app_metrics.as_ref().and_then(|m| m.v8_heap_total_mb),
            event_loop_lag_ms: app_metrics.as_ref().and_then(|m| m.event_loop_lag_ms),
            gc_time_ms: app_metrics.as_ref().and_then(|m| m.gc_time_ms),
        }
    }

    /// Load metrics from application instrumentation file
    fn load_metrics_file(&self, path: &str) -> Option<NodeMetricsFile> {
        match std::fs::read_to_string(path) {
            Ok(content) => match serde_json::from_str::<NodeMetricsFile>(&content) {
                Ok(metrics) => {
                    println!("  ✓ Loaded Node application metrics from {path}");
                    Some(metrics)
                }
                Err(e) => {
                    eprintln!("  ⚠ Failed to parse Node metrics file: {e}");
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
    pub v8_heap_used_mb: Option<f64>,
    pub v8_heap_total_mb: Option<f64>,
    pub event_loop_lag_ms: Option<f64>,
    pub gc_time_ms: Option<f64>,
}

impl Drop for NodeProfiler {
    fn drop(&mut self) {
        if let Some(ref mut process) = self.process {
            let _ = process.kill();
        }
    }
}

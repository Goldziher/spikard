//! WASM (Deno) profiling integration
//!
//! Deno does not currently provide a stable, portable CPU profiler output format that we can
//! enable without external tooling. For CI benchmarks we capture:
//! - Application-level memory metrics via `Deno.memoryUsage()` written on shutdown.
//! - Optional V8 `v8.log` output when `--v8-flags=--prof` is enabled (best-effort).

use crate::error::Result;
use serde::Deserialize;
use std::path::Path;
use std::time::{Duration, Instant};

/// Metrics emitted by the Deno benchmark app.
#[derive(Debug, Deserialize)]
pub struct WasmMetricsFile {
    pub rss_mb: Option<f64>,
    pub heap_total_mb: Option<f64>,
    pub heap_used_mb: Option<f64>,
    pub external_mb: Option<f64>,
}

pub fn wait_for_metrics_output(path: &str) -> Option<WasmMetricsFile> {
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(5) {
        if let Ok(content) = std::fs::read_to_string(path) {
            match serde_json::from_str::<WasmMetricsFile>(&content) {
                Ok(metrics) => return Some(metrics),
                Err(e) => {
                    eprintln!("  ⚠ Failed to parse WASM metrics file {}: {}", path, e);
                    return None;
                }
            }
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    None
}

/// Copy `v8.log` (if produced) into a stable location under the results directory.
pub fn collect_v8_log(app_dir: &Path, output_path: &Path) -> Result<Option<String>> {
    let source = app_dir.join("v8.log");
    if !source.exists() {
        return Ok(None);
    }

    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let _ = std::fs::copy(&source, output_path);
    Ok(Some(output_path.display().to_string()))
}

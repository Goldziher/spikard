//! WASM profiling integration
//!
//! For `WASIp3` components running under wasmtime, we measure process-level RSS
//! via the `sysinfo` crate (same approach as native Rust profiling).
//! Deno-specific metrics are no longer applicable.

use crate::error::Result;
use std::path::Path;

/// Metrics collected from a WASM component process.
#[derive(Debug)]
pub struct WasmProcessMetrics {
    pub rss_mb: Option<f64>,
}

/// Collect process-level memory metrics for a WASM runtime process.
///
/// Uses `/proc/{pid}/status` on Linux or equivalent OS APIs.
#[must_use]
#[allow(clippy::missing_const_for_fn)] // reads /proc on linux
pub fn collect_process_metrics(pid: u32) -> WasmProcessMetrics {
    #[cfg(target_os = "linux")]
    {
        // Read RSS from /proc on Linux
        if let Ok(status) = std::fs::read_to_string(format!("/proc/{pid}/status")) {
            for line in status.lines() {
                if let Some(value) = line.strip_prefix("VmRSS:") {
                    let kb: f64 = value.trim().trim_end_matches(" kB").trim().parse().unwrap_or(0.0);
                    return WasmProcessMetrics {
                        rss_mb: Some(kb / 1024.0),
                    };
                }
            }
        }
    }

    #[cfg(not(target_os = "linux"))]
    let _ = pid; // unused on non-Linux platforms

    // Fallback: no metrics available
    WasmProcessMetrics { rss_mb: None }
}

/// Copy wasmtime profiling output if available.
pub fn collect_wasmtime_profile(app_dir: &Path, output_path: &Path) -> Result<Option<String>> {
    let source = app_dir.join("wasmtime-profile.json");
    if !source.exists() {
        return Ok(None);
    }

    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let _ = std::fs::copy(&source, output_path);
    Ok(Some(output_path.display().to_string()))
}

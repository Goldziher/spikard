//! PHP profiler integration
//!
//! Profile output is produced by application-level instrumentation (e.g. `excimer`)
//! and written to a path configured via `SPIKARD_PHP_PROFILE_OUTPUT`.

use crate::error::Result;
use std::path::PathBuf;
use std::time::Duration;

/// PHP profiler handle (app-managed output)
pub struct PhpProfiler {
    output_path: Option<String>,
}

#[must_use]
pub fn wait_for_profile_output(path: &str) -> Option<String> {
    let start = std::time::Instant::now();
    while start.elapsed() < Duration::from_secs(30) {
        if std::fs::metadata(path).is_ok() {
            return Some(path.to_string());
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    eprintln!("  ⚠ PHP profile output not found at {path}");
    None
}

/// Start PHP profiler for the given PID.
///
/// The benchmark app is expected to write profiling output when the process exits.
pub fn start_profiler(_pid: u32, output_path: Option<&PathBuf>) -> Result<PhpProfiler> {
    let output_path = output_path
        .as_ref()
        .and_then(|p| p.to_str().map(std::string::ToString::to_string))
        .or_else(|| std::env::var("SPIKARD_PHP_PROFILE_OUTPUT").ok());

    if let Some(ref path) = output_path
        && let Some(parent) = PathBuf::from(path).parent()
    {
        let _ = std::fs::create_dir_all(parent);
    }

    Ok(PhpProfiler { output_path })
}

impl PhpProfiler {
    #[must_use]
    pub fn output_path(&self) -> Option<&str> {
        self.output_path.as_deref()
    }

    pub fn stop(self) -> ProfilingData {
        let flamegraph_path = self.output_path().and_then(wait_for_profile_output);
        ProfilingData { flamegraph_path }
    }
}

pub struct ProfilingData {
    pub flamegraph_path: Option<String>,
}

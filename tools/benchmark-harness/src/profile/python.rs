//! Python profiler integration (py-spy)

use crate::error::Result;
use std::process::{Child, Command};

/// Python profiler handle
pub struct PythonProfiler {
    _process: Child,
}

/// Start py-spy profiler for the given PID
pub fn start_profiler(pid: u32) -> Result<PythonProfiler> {
    // Check if py-spy is installed
    if which::which("py-spy").is_err() {
        eprintln!("⚠ py-spy not found, profiling disabled");
        eprintln!("  Install: pip install py-spy");
        // Return dummy profiler for now
        // TODO: make this a proper error or graceful fallback
    }

    // Start py-spy in record mode
    // py-spy record --pid <PID> --output profile.svg --format speedscope
    let process = Command::new("py-spy")
        .arg("record")
        .arg("--pid")
        .arg(pid.to_string())
        .arg("--output")
        .arg("/tmp/py-spy-profile.svg")
        .arg("--format")
        .arg("speedscope")
        .spawn()?;

    Ok(PythonProfiler { _process: process })
}

impl Drop for PythonProfiler {
    fn drop(&mut self) {
        // py-spy will stop when process exits
        let _ = self._process.kill();
    }
}

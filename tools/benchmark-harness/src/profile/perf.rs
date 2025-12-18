//! Linux perf-based profiler
//!
//! Captures native stacks via `perf record`, then renders an SVG flamegraph via `inferno`.

#[cfg(target_os = "linux")]
use crate::error::{Error, Result};
#[cfg(target_os = "linux")]
use inferno::collapse::perf::Folder;
#[cfg(target_os = "linux")]
use inferno::flamegraph::{self, Options};
#[cfg(target_os = "linux")]
use std::io::Cursor;
#[cfg(target_os = "linux")]
use std::path::PathBuf;
#[cfg(target_os = "linux")]
use std::process::{Child, Command, Stdio};

#[cfg(target_os = "linux")]
pub struct PerfProfiler {
    svg_path: Option<PathBuf>,
    perf_data_path: Option<PathBuf>,
    child: Option<Child>,
}

#[cfg(target_os = "linux")]
pub fn start_profiler(pid: u32, svg_path: Option<PathBuf>, duration_secs: u64) -> Result<PerfProfiler> {
    let (child, perf_data_path) = if let Some(ref svg_path) = svg_path {
        if let Some(parent) = svg_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let perf_data_path = svg_path.with_extension("perf.data");
        let record_secs = duration_secs.clamp(1, 5);

        let mut cmd = Command::new("perf");
        cmd.args([
            "record",
            "-F",
            "99",
            "-g",
            "--call-graph",
            "dwarf",
            "-p",
            &pid.to_string(),
            "-o",
            &perf_data_path.display().to_string(),
            "--",
            "sleep",
            &record_secs.to_string(),
        ]);
        cmd.stdout(Stdio::null()).stderr(Stdio::null());

        let child = cmd.spawn().map_err(|e| {
            Error::BenchmarkFailed(format!("Failed to start perf profiler for pid {}: {}", pid, e))
        })?;
        (Some(child), Some(perf_data_path))
    } else {
        (None, None)
    };

    Ok(PerfProfiler {
        svg_path,
        perf_data_path,
        child,
    })
}

#[cfg(target_os = "linux")]
impl PerfProfiler {
    pub fn stop(mut self) -> Option<String> {
        if let Some(mut child) = self.child.take() {
            let _ = child.wait();
        }

        let (Some(svg_path), Some(perf_data_path)) = (self.svg_path.as_ref(), self.perf_data_path.as_ref()) else {
            return None;
        };

        let script_output = Command::new("perf")
            .args(["script", "-i", &perf_data_path.display().to_string()])
            .output()
            .ok()?;

        if !script_output.status.success() {
            return None;
        }

        let mut folded = Vec::<u8>::new();
        if Folder::new()
            .collapse(Cursor::new(script_output.stdout), &mut folded)
            .is_err()
        {
            return None;
        }

        let mut opts = Options::default();
        opts.count_name = "samples".to_string();
        opts.direction = flamegraph::Direction::Inverted;

        let out_file = std::fs::File::create(svg_path).ok()?;
        if flamegraph::from_reader(&mut opts, Cursor::new(folded), out_file).is_err() {
            return None;
        }

        Some(svg_path.display().to_string())
    }
}

//! Python profiler integration
//!
//! Supports multiple profiling approaches:
//! - py-spy for sampling profiler with flamegraphs
//! - Application instrumentation for GC and timing metrics
//! - Combined metrics from both sources

use crate::error::Result;
use serde::Deserialize;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

/// Python profiler handle
pub struct PythonProfiler {
    process: Option<Child>,
    output_path: String,
    pid: u32,
    stderr_log_path: Option<String>,
}

/// Metrics collected from Python application instrumentation
#[derive(Debug, Deserialize)]
struct PythonMetricsFile {
    gc_collections: Option<u64>,
    gc_time_ms: Option<f64>,
    handler_time_ms: Option<f64>,
    serialization_time_ms: Option<f64>,
    ffi_overhead_ms: Option<f64>,
}

/// Start Python profiler for the given PID
pub fn start_profiler(pid: u32, output_path: Option<PathBuf>) -> Result<PythonProfiler> {
    let desired_output_path = output_path
        .as_ref()
        .and_then(|p| p.to_str().map(|s| s.to_string()))
        .or_else(|| std::env::var("SPIKARD_PYSPY_OUTPUT").ok());
    if which::which("py-spy").is_ok() {
        let output_path = desired_output_path.unwrap_or_else(|| format!("/tmp/py-spy-{}.json", pid));
        if let Some(parent) = PathBuf::from(&output_path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let stdout_log_path = format!("{}.stdout.log", output_path);
        let stderr_log_path = format!("{}.stderr.log", output_path);

        match Command::new("py-spy")
            .arg("record")
            .arg("--pid")
            .arg(pid.to_string())
            .arg("--output")
            .arg(&output_path)
            .arg("--format")
            .arg("speedscope")
            .arg("--rate")
            .arg("100")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(mut process) => {
                if let Some(stdout) = process.stdout.take() {
                    let path = stdout_log_path.clone();
                    std::thread::spawn(move || tee_process_output(stdout, &path));
                }
                if let Some(stderr) = process.stderr.take() {
                    let path = stderr_log_path.clone();
                    std::thread::spawn(move || tee_process_output(stderr, &path));
                }

                std::thread::sleep(std::time::Duration::from_millis(250));
                if let Ok(Some(status)) = process.try_wait() {
                    eprintln!(
                        "  ⚠ py-spy exited early with status: {} (stderr: {})",
                        status, stderr_log_path
                    );
                    print_log_tail("py-spy stderr", &stderr_log_path);
                    return Ok(PythonProfiler {
                        process: None,
                        output_path: String::new(),
                        pid,
                        stderr_log_path: Some(stderr_log_path),
                    });
                }

                println!("  ✓ py-spy profiler started (output: {})", output_path);
                println!("    ↳ py-spy logs: {}, {}", stdout_log_path, stderr_log_path);
                return Ok(PythonProfiler {
                    process: Some(process),
                    output_path,
                    pid,
                    stderr_log_path: Some(stderr_log_path),
                });
            }
            Err(e) => {
                eprintln!("  ⚠ Failed to start py-spy: {}", e);
            }
        }
    } else {
        eprintln!("  ⚠ py-spy not found (install: pip install py-spy)");
        eprintln!("  → Profiling will be limited to application metrics");
    }

    Ok(PythonProfiler {
        process: None,
        output_path: String::new(),
        pid,
        stderr_log_path: None,
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
        let metrics_path = std::env::var("SPIKARD_METRICS_FILE")
            .ok()
            .unwrap_or_else(|| format!("/tmp/python-metrics-{}.json", self.pid));

        #[cfg(unix)]
        {
            unsafe {
                libc::kill(self.pid as i32, libc::SIGUSR1);
            }
            let start = std::time::Instant::now();
            while start.elapsed() < std::time::Duration::from_secs(2) {
                if std::fs::metadata(&metrics_path).is_ok() {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }

        let mut py_spy_exit_status = None;
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
                        py_spy_exit_status = Some(status);
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

        let app_metrics = self.load_metrics_file(&metrics_path);

        if app_metrics.is_none() {
            eprintln!("  ⚠ Python metrics file not found or unreadable at {}", metrics_path);
        }

        let flamegraph_path = self.output_path().and_then(|p| {
            if std::fs::metadata(p).is_ok() {
                Some(p.to_string())
            } else {
                eprintln!("  ⚠ py-spy output not found at {}", p);
                if let Some(status) = py_spy_exit_status {
                    eprintln!("  → py-spy exit status: {}", status);
                }
                if let Some(path) = self.stderr_log_path.as_deref() {
                    print_log_tail("py-spy stderr", path);
                }
                None
            }
        });

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

    /// Load metrics from application instrumentation file
    fn load_metrics_file(&self, path: &str) -> Option<PythonMetricsFile> {
        match std::fs::read_to_string(path) {
            Ok(content) => match serde_json::from_str::<PythonMetricsFile>(&content) {
                Ok(metrics) => {
                    println!("  ✓ Loaded application metrics from {}", path);
                    Some(metrics)
                }
                Err(e) => {
                    eprintln!("  ⚠ Failed to parse metrics file: {}", e);
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

fn tee_process_output(mut stream: impl std::io::Read, path: &str) {
    let mut buf = [0u8; 8192];
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)
        .ok();

    loop {
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                if let Some(ref mut file) = file {
                    use std::io::Write;
                    let _ = file.write_all(&buf[..n]);
                }
            }
            Err(_) => break,
        }
    }
}

fn print_log_tail(label: &str, path: &str) {
    let Ok(data) = std::fs::read(path) else {
        return;
    };
    let tail = if data.len() > 4096 {
        &data[data.len() - 4096..]
    } else {
        &data
    };
    let Ok(text) = String::from_utf8(tail.to_vec()) else {
        return;
    };
    let text = text.trim();
    if !text.is_empty() {
        eprintln!("  → {} (tail):\n{}", label, text);
    }
}

//! Benchmark result schemas
//!
//! This module defines the complete data model for benchmark results, supporting:
//! - Profile mode: Deep analysis of Spikard implementations
//! - Compare mode: Framework comparisons
//! - CI integration: Structured JSON for analytics

pub mod compare;
pub mod profile;
pub mod workload;

use serde::{Deserialize, Serialize};

/// Top-level benchmark result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum BenchmarkResult {
    Profile(profile::ProfileResult),
    Compare(compare::CompareResult),
}

/// System metadata for reproducibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub timestamp: String,
    pub git_commit: Option<String>,
    pub git_branch: Option<String>,
    pub git_dirty: bool,
    pub host: HostInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostInfo {
    pub os: String,
    pub arch: String,
    pub cpu_model: String,
    pub cpu_cores: usize,
    pub cpu_threads: usize,
    pub memory_gb: f64,
    pub hostname: String,
}

/// Benchmark configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub duration_secs: u64,
    pub concurrency: usize,
    pub warmup_secs: u64,
    pub load_tool: String,
}

/// Framework information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkInfo {
    pub name: String,
    pub version: String,
    pub language: String,
    pub runtime: String,
    pub app_dir: String,
    pub variant: Option<String>,
}

/// Throughput metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Throughput {
    pub requests_per_sec: f64,
    pub bytes_per_sec: f64,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub success_rate: f64,
}

/// Latency distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Latency {
    pub mean_ms: f64,
    pub median_ms: f64,
    pub p90_ms: f64,
    pub p95_ms: f64,
    pub p99_ms: f64,
    pub p999_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
    pub stddev_ms: f64,
}

/// Resource utilization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resources {
    pub cpu: CpuMetrics,
    pub memory: MemoryMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuMetrics {
    pub avg_percent: f64,
    pub peak_percent: f64,
    pub p95_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    pub avg_mb: f64,
    pub peak_mb: f64,
    pub p95_mb: f64,
}

/// Language-specific profiling data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "language", rename_all = "snake_case")]
pub enum ProfilingData {
    Python(PythonProfilingData),
    Node(NodeProfilingData),
    Ruby(RubyProfilingData),
    Php(PhpProfilingData),
    Rust(RustProfilingData),
    Wasm(WasmProfilingData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonProfilingData {
    pub gil_wait_time_ms: Option<f64>,
    pub gil_contention_percent: Option<f64>,
    pub ffi_overhead_ms: Option<f64>,
    pub handler_time_ms: Option<f64>,
    pub serialization_time_ms: Option<f64>,
    pub gc_collections: Option<u64>,
    pub gc_time_ms: Option<f64>,
    pub flamegraph_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeProfilingData {
    pub v8_heap_used_mb: Option<f64>,
    pub v8_heap_total_mb: Option<f64>,
    pub event_loop_lag_ms: Option<f64>,
    pub gc_time_ms: Option<f64>,
    pub flamegraph_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubyProfilingData {
    pub gc_count: Option<u64>,
    pub gc_time_ms: Option<f64>,
    pub heap_allocated_pages: Option<u64>,
    pub heap_live_slots: Option<u64>,
    pub flamegraph_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhpProfilingData {
    pub flamegraph_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustProfilingData {
    pub heap_allocated_mb: Option<f64>,
    pub flamegraph_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmProfilingData {
    pub rss_mb: Option<f64>,
    pub heap_total_mb: Option<f64>,
    pub heap_used_mb: Option<f64>,
    pub external_mb: Option<f64>,
    pub v8_log_path: Option<String>,
    pub flamegraph_path: Option<String>,
}

/// Statistical comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalSignificance {
    pub p_value: f64,
    pub significant: bool,
    pub confidence_level: f64,
}

/// Helper to collect system metadata
impl Metadata {
    pub fn collect() -> Self {
        Self {
            timestamp: chrono::Utc::now().to_rfc3339(),
            git_commit: Self::git_commit(),
            git_branch: Self::git_branch(),
            git_dirty: Self::git_dirty(),
            host: HostInfo::collect(),
        }
    }

    fn git_commit() -> Option<String> {
        std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
    }

    fn git_branch() -> Option<String> {
        std::process::Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
    }

    fn git_dirty() -> bool {
        std::process::Command::new("git")
            .args(["status", "--porcelain"])
            .output()
            .ok()
            .map(|o| !o.stdout.is_empty())
            .unwrap_or(false)
    }
}

impl HostInfo {
    pub fn collect() -> Self {
        let cpu_model = Self::cpu_model();
        let cpu_info = Self::cpu_info();

        Self {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            cpu_model,
            cpu_cores: cpu_info.0,
            cpu_threads: cpu_info.1,
            memory_gb: Self::memory_gb(),
            hostname: Self::hostname(),
        }
    }

    fn cpu_model() -> String {
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("sysctl")
                .args(["-n", "machdep.cpu.brand_string"])
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "Unknown CPU".to_string())
        }
        #[cfg(target_os = "linux")]
        {
            std::fs::read_to_string("/proc/cpuinfo")
                .ok()
                .and_then(|content| {
                    content
                        .lines()
                        .find(|line| line.starts_with("model name"))
                        .and_then(|line| line.split(':').nth(1))
                        .map(|s| s.trim().to_string())
                })
                .unwrap_or_else(|| "Unknown CPU".to_string())
        }
        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        {
            "Unknown CPU".to_string()
        }
    }

    fn cpu_info() -> (usize, usize) {
        let logical = num_cpus::get();
        let physical = num_cpus::get_physical();
        (physical, logical)
    }

    fn memory_gb() -> f64 {
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("sysctl")
                .args(["-n", "hw.memsize"])
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .and_then(|s| s.trim().parse::<u64>().ok())
                .map(|bytes| bytes as f64 / 1024.0 / 1024.0 / 1024.0)
                .unwrap_or(0.0)
        }
        #[cfg(target_os = "linux")]
        {
            std::fs::read_to_string("/proc/meminfo")
                .ok()
                .and_then(|content| {
                    content
                        .lines()
                        .find(|line| line.starts_with("MemTotal"))
                        .and_then(|line| line.split_whitespace().nth(1))
                        .and_then(|s| s.parse::<u64>().ok())
                })
                .map(|kb| kb as f64 / 1024.0 / 1024.0)
                .unwrap_or(0.0)
        }
        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        {
            0.0
        }
    }

    fn hostname() -> String {
        hostname::get()
            .ok()
            .and_then(|h| h.into_string().ok())
            .unwrap_or_else(|| "unknown".to_string())
    }
}

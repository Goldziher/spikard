//! Profile mode result schema

use super::{
    workload::*, Configuration, FrameworkInfo, Latency, Metadata, ProfilingData, Resources, Throughput,
};
use serde::{Deserialize, Serialize};

/// Complete profile mode result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileResult {
    pub metadata: Metadata,
    pub framework: FrameworkInfo,
    pub configuration: Configuration,
    pub suites: Vec<SuiteResult>,
    pub summary: ProfileSummary,
    pub comparison: Option<BaselineComparison>,
}

/// Results for a workload suite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteResult {
    pub name: String,
    pub description: String,
    pub workloads: Vec<WorkloadResult>,
}

/// Individual workload result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadResult {
    pub name: String,
    pub description: String,
    pub category: String,
    pub payload_size_bytes: Option<u64>,
    pub endpoint: Endpoint,
    pub results: WorkloadMetrics,
}

/// Metrics for a single workload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadMetrics {
    pub throughput: Throughput,
    pub latency: Latency,
    pub resources: Resources,
    pub profiling: Option<ProfilingData>,
}

/// Summary across all workloads
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileSummary {
    pub total_workloads: usize,
    pub total_requests: u64,
    pub overall_success_rate: f64,
    pub avg_requests_per_sec: f64,
    pub total_duration_secs: u64,
    pub category_breakdown: Vec<CategorySummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorySummary {
    pub category: String,
    pub workload_count: usize,
    pub avg_requests_per_sec: f64,
    pub avg_latency_ms: f64,
}

/// Comparison with Rust baseline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineComparison {
    pub baseline_framework: String, // "spikard-rust"
    pub workload_comparisons: Vec<WorkloadComparison>,
    pub overall_ratio: f64, // Baseline RPS / This RPS
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadComparison {
    pub workload_name: String,
    pub baseline_requests_per_sec: f64,
    pub this_requests_per_sec: f64,
    pub ratio: f64, // Baseline / This
}

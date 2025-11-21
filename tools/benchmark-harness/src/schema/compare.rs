//! Compare mode result schema

use super::{Configuration, FrameworkInfo, Latency, Metadata, Resources, Throughput, workload::*};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete compare mode result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompareResult {
    pub metadata: Metadata,
    pub frameworks: Vec<FrameworkInfo>,
    pub configuration: Configuration,
    pub suites: Vec<CompareSuiteResult>,
    pub summary: CompareSummary,
}

/// Suite results with multiple frameworks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompareSuiteResult {
    pub name: String,
    pub description: String,
    pub workloads: Vec<CompareWorkloadResult>,
}

/// Workload results across frameworks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompareWorkloadResult {
    pub name: String,
    pub description: String,
    pub category: String,
    pub payload_size_bytes: Option<u64>,
    pub endpoint: Endpoint,
    pub results: Vec<FrameworkResult>,
    pub comparison: WorkloadComparisonAnalysis,
}

/// Results for one framework in a workload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkResult {
    pub framework: String,
    pub throughput: Throughput,
    pub latency: Latency,
    pub resources: Resources,
}

/// Statistical analysis of workload comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadComparisonAnalysis {
    pub winner: String,                           // Framework name
    pub performance_ratios: HashMap<String, f64>, // "framework_a_vs_framework_b" -> ratio
    pub statistical_significance: Option<super::StatisticalSignificance>,
}

/// Overall comparison summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompareSummary {
    pub overall_winner: String,
    pub avg_performance_gain: f64,
    pub workloads_won: HashMap<String, usize>,     // Framework -> count
    pub category_winners: HashMap<String, String>, // Category -> winner
}

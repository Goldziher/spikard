//! Compare mode result schema

use super::{Configuration, FrameworkInfo, Latency, Metadata, Resources, Throughput, workload::*};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Statistical test result from hypothesis testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalTest {
    /// Name of the statistical test performed
    pub test_name: String,

    /// Metric being tested (e.g., "requests_per_sec", "latency_p50_ms")
    pub metric: String,

    /// Test statistic value (e.g., t-statistic for Welch's t-test)
    pub statistic: f64,

    /// Two-tailed p-value indicating probability of observing this difference by chance
    pub p_value: f64,

    /// Whether the result is statistically significant at the configured threshold
    pub is_significant: bool,

    /// 95% confidence interval for the mean difference (baseline - comparison)
    pub confidence_interval: (f64, f64),
}

/// Effect size measurement using Cohen's d
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectSize {
    /// Metric being measured
    pub metric: String,

    /// Cohen's d value (positive = baseline better, negative = comparison better)
    pub cohens_d: f64,

    /// Magnitude classification: "small", "medium", "large", "very_large"
    pub magnitude: String,
}

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

    /// Statistical tests comparing to baseline (optional, only for non-baseline frameworks)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistical_tests: Option<Vec<StatisticalTest>>,

    /// Effect sizes comparing to baseline (optional, only for non-baseline frameworks)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_sizes: Option<Vec<EffectSize>>,

    /// Overall verdict: "baseline", "significantly_better", "significantly_worse", "similar"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verdict: Option<String>,
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

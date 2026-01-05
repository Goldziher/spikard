//! Framework comparison and regression detection
//!
//! This module provides functions to compare benchmark results across different
//! frameworks, calculate relative performance metrics, and detect regressions.

use crate::types::BenchmarkResult;
use serde::{Deserialize, Serialize};

/// Comparison report showing performance relative to a baseline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonReport {
    /// Baseline framework name
    pub baseline_framework: String,

    /// Baseline workload name
    pub baseline_workload: String,

    /// Baseline benchmark result
    pub baseline: BenchmarkResult,

    /// Comparisons against baseline
    pub comparisons: Vec<FrameworkComparison>,
}

/// Comparison of a single framework against the baseline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkComparison {
    /// Framework being compared
    pub framework: String,

    /// Benchmark result for this framework
    pub result: BenchmarkResult,

    /// Relative performance metrics
    pub relative: RelativeMetrics,

    /// Performance summary (Better/Worse/Similar)
    pub summary: PerformanceSummary,
}

/// Relative performance metrics (ratio compared to baseline)
///
/// Values > 1.0 indicate better performance than baseline
/// Values < 1.0 indicate worse performance than baseline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelativeMetrics {
    pub throughput_ratio: f64,
    pub throughput_percent_diff: f64,

    pub latency_p50_ratio: f64,
    pub latency_p50_percent_diff: f64,

    pub latency_p95_ratio: f64,
    pub latency_p95_percent_diff: f64,

    pub latency_p99_ratio: f64,
    pub latency_p99_percent_diff: f64,

    pub memory_avg_ratio: f64,
    pub memory_avg_percent_diff: f64,

    pub memory_peak_ratio: f64,
    pub memory_peak_percent_diff: f64,

    pub cpu_avg_ratio: f64,
    pub cpu_avg_percent_diff: f64,

    pub success_rate_ratio: f64,
    pub success_rate_percent_diff: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub startup_time_ratio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startup_time_percent_diff: Option<f64>,
}

/// Performance summary classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PerformanceSummary {
    /// Significantly better than baseline
    MuchBetter,
    /// Better than baseline
    Better,
    /// Similar to baseline
    Similar,
    /// Worse than baseline
    Worse,
    /// Significantly worse than baseline
    MuchWorse,
}

/// Regression warning for a specific metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionWarning {
    /// Framework with regression
    pub framework: String,

    /// Metric that regressed
    pub metric: String,

    /// Percentage change (negative for regression)
    pub percent_change: f64,

    /// Threshold that was exceeded
    pub threshold_pct: f64,

    /// Severity of regression
    pub severity: RegressionSeverity,
}

/// Severity of performance regression
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegressionSeverity {
    /// Minor regression (within 2x threshold)
    Minor,
    /// Moderate regression (2-3x threshold)
    Moderate,
    /// Severe regression (>3x threshold)
    Severe,
}

/// Compare frameworks against a baseline
///
/// # Arguments
///
/// * `baseline` - The baseline benchmark result to compare against
/// * `comparisons` - Slice of benchmark results to compare
///
/// # Returns
///
/// Comparison report with relative metrics for each framework
#[must_use]
pub fn compare_frameworks(baseline: &BenchmarkResult, comparisons: &[BenchmarkResult]) -> ComparisonReport {
    let framework_comparisons: Vec<FrameworkComparison> = comparisons
        .iter()
        .map(|result| {
            let relative = calculate_relative_metrics(baseline, result);
            let summary = classify_performance(&relative);

            FrameworkComparison {
                framework: result.framework.clone(),
                result: result.clone(),
                relative,
                summary,
            }
        })
        .collect();

    ComparisonReport {
        baseline_framework: baseline.framework.clone(),
        baseline_workload: baseline.workload.clone(),
        baseline: baseline.clone(),
        comparisons: framework_comparisons,
    }
}

/// Calculate relative performance metrics
fn calculate_relative_metrics(baseline: &BenchmarkResult, comparison: &BenchmarkResult) -> RelativeMetrics {
    let throughput_ratio = safe_divide(
        comparison.throughput.requests_per_sec,
        baseline.throughput.requests_per_sec,
    );
    let throughput_percent_diff = (throughput_ratio - 1.0) * 100.0;

    let latency_p50_ratio = safe_divide(baseline.latency.p50_ms, comparison.latency.p50_ms);
    let latency_p50_percent_diff = (latency_p50_ratio - 1.0) * 100.0;

    let latency_p95_ratio = safe_divide(baseline.latency.p95_ms, comparison.latency.p95_ms);
    let latency_p95_percent_diff = (latency_p95_ratio - 1.0) * 100.0;

    let latency_p99_ratio = safe_divide(baseline.latency.p99_ms, comparison.latency.p99_ms);
    let latency_p99_percent_diff = (latency_p99_ratio - 1.0) * 100.0;

    let memory_avg_ratio = safe_divide(baseline.resources.avg_memory_mb, comparison.resources.avg_memory_mb);
    let memory_avg_percent_diff = (memory_avg_ratio - 1.0) * 100.0;

    let memory_peak_ratio = safe_divide(baseline.resources.peak_memory_mb, comparison.resources.peak_memory_mb);
    let memory_peak_percent_diff = (memory_peak_ratio - 1.0) * 100.0;

    let cpu_avg_ratio = safe_divide(baseline.resources.avg_cpu_percent, comparison.resources.avg_cpu_percent);
    let cpu_avg_percent_diff = (cpu_avg_ratio - 1.0) * 100.0;

    let success_rate_ratio = safe_divide(comparison.throughput.success_rate, baseline.throughput.success_rate);
    let success_rate_percent_diff = (success_rate_ratio - 1.0) * 100.0;

    let (startup_time_ratio, startup_time_percent_diff) =
        if let (Some(baseline_startup), Some(comparison_startup)) = (&baseline.startup, &comparison.startup) {
            let ratio = safe_divide(baseline_startup.total_startup_ms, comparison_startup.total_startup_ms);
            let percent_diff = (ratio - 1.0) * 100.0;
            (Some(ratio), Some(percent_diff))
        } else {
            (None, None)
        };

    RelativeMetrics {
        throughput_ratio,
        throughput_percent_diff,
        latency_p50_ratio,
        latency_p50_percent_diff,
        latency_p95_ratio,
        latency_p95_percent_diff,
        latency_p99_ratio,
        latency_p99_percent_diff,
        memory_avg_ratio,
        memory_avg_percent_diff,
        memory_peak_ratio,
        memory_peak_percent_diff,
        cpu_avg_ratio,
        cpu_avg_percent_diff,
        success_rate_ratio,
        success_rate_percent_diff,
        startup_time_ratio,
        startup_time_percent_diff,
    }
}

/// Classify overall performance based on relative metrics
fn classify_performance(relative: &RelativeMetrics) -> PerformanceSummary {
    let throughput_score = (relative.throughput_ratio - 1.0) * 100.0 * 0.4;
    let latency_score = (relative.latency_p99_ratio - 1.0) * 100.0 * 0.3;
    let memory_score = (relative.memory_peak_ratio - 1.0) * 100.0 * 0.2;
    let cpu_score = (relative.cpu_avg_ratio - 1.0) * 100.0 * 0.1;

    let total_score = throughput_score + latency_score + memory_score + cpu_score;

    match total_score {
        s if s > 20.0 => PerformanceSummary::MuchBetter,
        s if s > 5.0 => PerformanceSummary::Better,
        s if s >= -5.0 => PerformanceSummary::Similar,
        s if s >= -20.0 => PerformanceSummary::Worse,
        _ => PerformanceSummary::MuchWorse,
    }
}

/// Detect performance regressions in comparison results
///
/// # Arguments
///
/// * `comparison` - Comparison report to analyze
/// * `threshold_pct` - Threshold percentage for regression (e.g., 5.0 for 5%)
///
/// # Returns
///
/// Vector of regression warnings for metrics that degraded beyond threshold
#[must_use]
pub fn detect_regressions(comparison: &ComparisonReport, threshold_pct: f64) -> Vec<RegressionWarning> {
    let mut warnings = Vec::new();

    for comp in &comparison.comparisons {
        let rel = &comp.relative;

        if rel.throughput_percent_diff < -threshold_pct {
            warnings.push(RegressionWarning {
                framework: comp.framework.clone(),
                metric: "throughput".to_string(),
                percent_change: rel.throughput_percent_diff,
                threshold_pct,
                severity: classify_severity(rel.throughput_percent_diff.abs(), threshold_pct),
            });
        }

        if rel.latency_p99_percent_diff < -threshold_pct {
            warnings.push(RegressionWarning {
                framework: comp.framework.clone(),
                metric: "latency_p99".to_string(),
                percent_change: rel.latency_p99_percent_diff,
                threshold_pct,
                severity: classify_severity(rel.latency_p99_percent_diff.abs(), threshold_pct),
            });
        }

        if rel.memory_peak_percent_diff < -threshold_pct {
            warnings.push(RegressionWarning {
                framework: comp.framework.clone(),
                metric: "memory_peak".to_string(),
                percent_change: rel.memory_peak_percent_diff,
                threshold_pct,
                severity: classify_severity(rel.memory_peak_percent_diff.abs(), threshold_pct),
            });
        }

        if rel.cpu_avg_percent_diff < -threshold_pct {
            warnings.push(RegressionWarning {
                framework: comp.framework.clone(),
                metric: "cpu_avg".to_string(),
                percent_change: rel.cpu_avg_percent_diff,
                threshold_pct,
                severity: classify_severity(rel.cpu_avg_percent_diff.abs(), threshold_pct),
            });
        }

        if rel.success_rate_percent_diff < -threshold_pct {
            warnings.push(RegressionWarning {
                framework: comp.framework.clone(),
                metric: "success_rate".to_string(),
                percent_change: rel.success_rate_percent_diff,
                threshold_pct,
                severity: classify_severity(rel.success_rate_percent_diff.abs(), threshold_pct),
            });
        }

        if let Some(startup_percent_diff) = rel.startup_time_percent_diff.filter(|&d| d < -threshold_pct) {
            warnings.push(RegressionWarning {
                framework: comp.framework.clone(),
                metric: "startup_time".to_string(),
                percent_change: startup_percent_diff,
                threshold_pct,
                severity: classify_severity(startup_percent_diff.abs(), threshold_pct),
            });
        }
    }

    warnings
}

/// Classify regression severity based on how much threshold was exceeded
fn classify_severity(percent_change: f64, threshold: f64) -> RegressionSeverity {
    let ratio = percent_change / threshold;
    match ratio {
        r if r >= 3.0 => RegressionSeverity::Severe,
        r if r >= 2.0 => RegressionSeverity::Moderate,
        _ => RegressionSeverity::Minor,
    }
}

/// Generate markdown report for comparison
///
/// # Arguments
///
/// * `comparison` - Comparison report to format
///
/// # Returns
///
/// Markdown-formatted report string
#[must_use]
pub fn generate_markdown_report(comparison: &ComparisonReport) -> String {
    let mut report = String::new();

    report.push_str("# Benchmark Comparison Report\n\n");
    report.push_str(&format!(
        "**Baseline:** {} ({})\n\n",
        comparison.baseline_framework, comparison.baseline_workload
    ));

    report.push_str("## Summary\n\n");
    report.push_str("| Framework | Throughput | Latency p99 | Memory Peak | CPU Avg | Overall |\n");
    report.push_str("|-----------|------------|-------------|-------------|---------|----------|\n");

    for comp in &comparison.comparisons {
        let rel = &comp.relative;
        report.push_str(&format!(
            "| {} | {:+.1}% | {:+.1}% | {:+.1}% | {:+.1}% | {} |\n",
            comp.framework,
            rel.throughput_percent_diff,
            rel.latency_p99_percent_diff,
            rel.memory_peak_percent_diff,
            rel.cpu_avg_percent_diff,
            format_summary(comp.summary)
        ));
    }

    report.push_str("\n## Detailed Metrics\n\n");

    for comp in &comparison.comparisons {
        report.push_str(&format!("### {}\n\n", comp.framework));

        report.push_str("**Throughput:**\n");
        report.push_str(&format!(
            "- Requests/sec: {:.2} ({:+.1}%)\n",
            comp.result.throughput.requests_per_sec, comp.relative.throughput_percent_diff
        ));

        report.push_str("\n**Latency:**\n");
        report.push_str(&format!(
            "- p50: {:.2}ms ({:+.1}%)\n",
            comp.result.latency.p50_ms, comp.relative.latency_p50_percent_diff
        ));
        report.push_str(&format!(
            "- p95: {:.2}ms ({:+.1}%)\n",
            comp.result.latency.p95_ms, comp.relative.latency_p95_percent_diff
        ));
        report.push_str(&format!(
            "- p99: {:.2}ms ({:+.1}%)\n",
            comp.result.latency.p99_ms, comp.relative.latency_p99_percent_diff
        ));

        report.push_str("\n**Resources:**\n");
        report.push_str(&format!(
            "- Avg Memory: {:.2}MB ({:+.1}%)\n",
            comp.result.resources.avg_memory_mb, comp.relative.memory_avg_percent_diff
        ));
        report.push_str(&format!(
            "- Peak Memory: {:.2}MB ({:+.1}%)\n",
            comp.result.resources.peak_memory_mb, comp.relative.memory_peak_percent_diff
        ));
        report.push_str(&format!(
            "- Avg CPU: {:.1}% ({:+.1}%)\n",
            comp.result.resources.avg_cpu_percent, comp.relative.cpu_avg_percent_diff
        ));

        if let (Some(startup), Some(diff)) = (&comp.result.startup, comp.relative.startup_time_percent_diff) {
            report.push_str("\n**Startup:**\n");
            report.push_str(&format!("- Total: {:.2}ms ({:+.1}%)\n", startup.total_startup_ms, diff));
        }

        report.push('\n');
    }

    report
}

/// Generate JSON report for comparison
///
/// # Arguments
///
/// * `comparison` - Comparison report to format
///
/// # Returns
///
/// JSON value representing the comparison
#[must_use]
pub fn generate_json_report(comparison: &ComparisonReport) -> serde_json::Value {
    serde_json::to_value(comparison).unwrap_or(serde_json::Value::Null)
}

/// Format performance summary as emoji + text
fn format_summary(summary: PerformanceSummary) -> String {
    match summary {
        PerformanceSummary::MuchBetter => "Much Better".to_string(),
        PerformanceSummary::Better => "Better".to_string(),
        PerformanceSummary::Similar => "Similar".to_string(),
        PerformanceSummary::Worse => "Worse".to_string(),
        PerformanceSummary::MuchWorse => "Much Worse".to_string(),
    }
}

/// Safe division that returns 1.0 for division by zero
fn safe_divide(numerator: f64, denominator: f64) -> f64 {
    if denominator == 0.0 {
        1.0
    } else {
        numerator / denominator
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{LatencyMetrics, ResourceMetrics, ThroughputMetrics};
    use chrono::Utc;

    fn create_test_result(framework: &str, rps: f64, latency_p99: f64) -> BenchmarkResult {
        BenchmarkResult {
            framework: framework.to_string(),
            workload: "test".to_string(),
            variant: None,
            timestamp: Utc::now(),
            duration_secs: 60,
            concurrency: 100,
            startup: None,
            throughput: ThroughputMetrics {
                total_requests: (rps * 60.0) as u64,
                requests_per_sec: rps,
                bytes_per_sec: rps * 1000.0,
                failed_requests: 0,
                success_rate: 1.0,
            },
            latency: LatencyMetrics {
                mean_ms: latency_p99 * 0.5,
                p50_ms: latency_p99 * 0.4,
                p90_ms: latency_p99 * 0.8,
                p95_ms: latency_p99 * 0.9,
                p99_ms: latency_p99,
                p999_ms: latency_p99 * 1.2,
                max_ms: latency_p99 * 1.5,
                min_ms: latency_p99 * 0.2,
                stddev_ms: latency_p99 * 0.1,
            },
            resources: ResourceMetrics {
                avg_memory_mb: 100.0,
                peak_memory_mb: 150.0,
                p50_memory_mb: 95.0,
                p95_memory_mb: 140.0,
                p99_memory_mb: 145.0,
                avg_cpu_percent: 50.0,
                peak_cpu_percent: 80.0,
            },
            route_types: vec![],
            error_metrics: None,
            serialization: None,
            patterns: vec![],
            success: true,
            error: None,
        }
    }

    #[test]
    fn test_compare_frameworks() {
        let baseline = create_test_result("baseline", 1000.0, 10.0);
        let better = create_test_result("better", 1200.0, 8.0);
        let worse = create_test_result("worse", 800.0, 15.0);

        let report = compare_frameworks(&baseline, &[better, worse]);

        assert_eq!(report.comparisons.len(), 2);

        let better_comp = &report.comparisons[0];
        assert!(better_comp.relative.throughput_ratio > 1.0);
        assert!(better_comp.relative.latency_p99_ratio > 1.0);

        let worse_comp = &report.comparisons[1];
        assert!(worse_comp.relative.throughput_ratio < 1.0);
        assert!(worse_comp.relative.latency_p99_ratio < 1.0);
    }

    #[test]
    fn test_detect_regressions() {
        let baseline = create_test_result("baseline", 1000.0, 10.0);
        let regressed = create_test_result("regressed", 800.0, 15.0);

        let report = compare_frameworks(&baseline, &[regressed]);
        let warnings = detect_regressions(&report, 5.0);

        assert!(!warnings.is_empty());
        assert!(warnings.iter().any(|w| w.metric == "throughput"));
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10.0, 2.0), 5.0);
        assert_eq!(safe_divide(10.0, 0.0), 1.0);
    }

    #[test]
    fn test_classify_performance() {
        let much_better = RelativeMetrics {
            throughput_ratio: 1.3,
            throughput_percent_diff: 30.0,
            latency_p50_ratio: 1.2,
            latency_p50_percent_diff: 20.0,
            latency_p95_ratio: 1.2,
            latency_p95_percent_diff: 20.0,
            latency_p99_ratio: 1.2,
            latency_p99_percent_diff: 20.0,
            memory_avg_ratio: 1.1,
            memory_avg_percent_diff: 10.0,
            memory_peak_ratio: 1.1,
            memory_peak_percent_diff: 10.0,
            cpu_avg_ratio: 1.1,
            cpu_avg_percent_diff: 10.0,
            success_rate_ratio: 1.0,
            success_rate_percent_diff: 0.0,
            startup_time_ratio: None,
            startup_time_percent_diff: None,
        };

        assert_eq!(classify_performance(&much_better), PerformanceSummary::MuchBetter);
    }
}

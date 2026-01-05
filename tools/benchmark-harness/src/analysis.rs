//! Statistical analysis of benchmark results
//!
//! This module provides functions to aggregate multiple benchmark runs,
//! calculate statistics, detect outliers, and compute confidence intervals.

use crate::Result;
use crate::types::{BenchmarkResult, ErrorMetrics, SerializationMetrics, StartupMetrics};
use serde::{Deserialize, Serialize};

/// Aggregated benchmark result with statistical measures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedResult {
    /// Framework name
    pub framework: String,

    /// Workload name
    pub workload: String,

    /// Number of runs aggregated
    pub num_runs: usize,

    /// Aggregated startup metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startup: Option<AggregatedStartupMetrics>,

    /// Aggregated throughput metrics
    pub throughput: AggregatedThroughputMetrics,

    /// Aggregated latency metrics
    pub latency: AggregatedLatencyMetrics,

    /// Aggregated resource metrics
    pub resources: AggregatedResourceMetrics,

    /// Aggregated error metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_metrics: Option<AggregatedErrorMetrics>,

    /// Aggregated serialization metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serialization: Option<AggregatedSerializationMetrics>,

    /// Indices of runs identified as outliers
    pub outlier_runs: Vec<usize>,
}

/// Statistical summary for a single metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricStats {
    pub mean: f64,
    pub median: f64,
    pub stddev: f64,
    pub min: f64,
    pub max: f64,
    /// 95% confidence interval for the mean: (lower, upper)
    pub ci_95: (f64, f64),
    /// Coefficient of variation (stddev/mean)
    pub cv: f64,
}

/// Aggregated startup metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedStartupMetrics {
    pub process_spawn_ms: MetricStats,
    pub time_to_first_response_ms: MetricStats,
    pub initialization_memory_mb: MetricStats,
    pub total_startup_ms: MetricStats,
}

/// Aggregated throughput metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedThroughputMetrics {
    pub total_requests: MetricStats,
    pub requests_per_sec: MetricStats,
    pub bytes_per_sec: MetricStats,
    pub failed_requests: MetricStats,
    pub success_rate: MetricStats,
}

/// Aggregated latency metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedLatencyMetrics {
    pub mean_ms: MetricStats,
    pub p50_ms: MetricStats,
    pub p90_ms: MetricStats,
    pub p95_ms: MetricStats,
    pub p99_ms: MetricStats,
    pub p999_ms: MetricStats,
    pub max_ms: MetricStats,
    pub min_ms: MetricStats,
    pub stddev_ms: MetricStats,
}

/// Aggregated resource metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedResourceMetrics {
    pub avg_memory_mb: MetricStats,
    pub peak_memory_mb: MetricStats,
    pub p50_memory_mb: MetricStats,
    pub p95_memory_mb: MetricStats,
    pub p99_memory_mb: MetricStats,
    pub avg_cpu_percent: MetricStats,
    pub peak_cpu_percent: MetricStats,
}

/// Aggregated error metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedErrorMetrics {
    pub validation_error_p99_ms: MetricStats,
    pub not_found_p99_ms: MetricStats,
    pub server_error_p99_ms: MetricStats,
    pub error_throughput_rps: MetricStats,
    pub error_memory_impact_mb: MetricStats,
    pub total_errors: MetricStats,
    pub error_rate: MetricStats,
}

/// Aggregated serialization metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedSerializationMetrics {
    pub json_parse_overhead_ms: MetricStats,
    pub json_serialize_overhead_ms: MetricStats,
    pub validation_overhead_ms: MetricStats,
    pub total_overhead_pct: MetricStats,
    pub sample_count: MetricStats,
}

/// Aggregate multiple benchmark runs into a single result with statistics
///
/// # Arguments
///
/// * `runs` - Slice of benchmark results to aggregate
///
/// # Returns
///
/// Aggregated result with mean, median, stddev, and confidence intervals
///
/// # Errors
///
/// Returns error if runs slice is empty or if frameworks/workloads don't match
pub fn aggregate_runs(runs: &[BenchmarkResult]) -> Result<AggregatedResult> {
    if runs.is_empty() {
        return Err(crate::Error::InvalidInput("Cannot aggregate empty runs".to_string()));
    }

    let framework = &runs[0].framework;
    let workload = &runs[0].workload;
    for run in runs {
        if run.framework != *framework || run.workload != *workload {
            return Err(crate::Error::InvalidInput(format!(
                "All runs must be for same framework/workload. Expected {}/{}, got {}/{}",
                framework, workload, run.framework, run.workload
            )));
        }
    }

    let rps_values: Vec<f64> = runs.iter().map(|r| r.throughput.requests_per_sec).collect();
    let outlier_runs = detect_outliers(&rps_values);

    let startup = if runs.iter().any(|r| r.startup.is_some()) {
        Some(aggregate_startup_metrics(runs))
    } else {
        None
    };

    let throughput = aggregate_throughput_metrics(runs);
    let latency = aggregate_latency_metrics(runs);
    let resources = aggregate_resource_metrics(runs);

    let error_metrics = if runs.iter().any(|r| r.error_metrics.is_some()) {
        Some(aggregate_error_metrics(runs))
    } else {
        None
    };

    let serialization = if runs.iter().any(|r| r.serialization.is_some()) {
        Some(aggregate_serialization_metrics(runs))
    } else {
        None
    };

    Ok(AggregatedResult {
        framework: framework.clone(),
        workload: workload.clone(),
        num_runs: runs.len(),
        startup,
        throughput,
        latency,
        resources,
        error_metrics,
        serialization,
        outlier_runs,
    })
}

/// Calculate statistics for a set of values
pub fn calculate_stats(values: &[f64]) -> MetricStats {
    let mean = mean(values);
    let median = median(values);
    let stddev = stddev(values);
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let ci_95 = calculate_confidence_interval(values, 0.95);
    let cv = coefficient_of_variation(values);

    MetricStats {
        mean,
        median,
        stddev,
        min,
        max,
        ci_95,
        cv,
    }
}

fn aggregate_startup_metrics(runs: &[BenchmarkResult]) -> AggregatedStartupMetrics {
    let values: Vec<&StartupMetrics> = runs.iter().filter_map(|r| r.startup.as_ref()).collect();

    if values.is_empty() {
        let zero_stats = MetricStats {
            mean: 0.0,
            median: 0.0,
            stddev: 0.0,
            min: 0.0,
            max: 0.0,
            ci_95: (0.0, 0.0),
            cv: 0.0,
        };
        return AggregatedStartupMetrics {
            process_spawn_ms: zero_stats.clone(),
            time_to_first_response_ms: zero_stats.clone(),
            initialization_memory_mb: zero_stats.clone(),
            total_startup_ms: zero_stats,
        };
    }

    let process_spawn_ms: Vec<f64> = values.iter().map(|v| v.process_spawn_ms).collect();
    let time_to_first_response_ms: Vec<f64> = values.iter().map(|v| v.time_to_first_response_ms).collect();
    let initialization_memory_mb: Vec<f64> = values.iter().map(|v| v.initialization_memory_mb).collect();
    let total_startup_ms: Vec<f64> = values.iter().map(|v| v.total_startup_ms).collect();

    AggregatedStartupMetrics {
        process_spawn_ms: calculate_stats(&process_spawn_ms),
        time_to_first_response_ms: calculate_stats(&time_to_first_response_ms),
        initialization_memory_mb: calculate_stats(&initialization_memory_mb),
        total_startup_ms: calculate_stats(&total_startup_ms),
    }
}

fn aggregate_throughput_metrics(runs: &[BenchmarkResult]) -> AggregatedThroughputMetrics {
    let total_requests: Vec<f64> = runs.iter().map(|r| r.throughput.total_requests as f64).collect();
    let requests_per_sec: Vec<f64> = runs.iter().map(|r| r.throughput.requests_per_sec).collect();
    let bytes_per_sec: Vec<f64> = runs.iter().map(|r| r.throughput.bytes_per_sec).collect();
    let failed_requests: Vec<f64> = runs.iter().map(|r| r.throughput.failed_requests as f64).collect();
    let success_rate: Vec<f64> = runs.iter().map(|r| r.throughput.success_rate).collect();

    AggregatedThroughputMetrics {
        total_requests: calculate_stats(&total_requests),
        requests_per_sec: calculate_stats(&requests_per_sec),
        bytes_per_sec: calculate_stats(&bytes_per_sec),
        failed_requests: calculate_stats(&failed_requests),
        success_rate: calculate_stats(&success_rate),
    }
}

fn aggregate_latency_metrics(runs: &[BenchmarkResult]) -> AggregatedLatencyMetrics {
    let mean_ms: Vec<f64> = runs.iter().map(|r| r.latency.mean_ms).collect();
    let p50_ms: Vec<f64> = runs.iter().map(|r| r.latency.p50_ms).collect();
    let p90_ms: Vec<f64> = runs.iter().map(|r| r.latency.p90_ms).collect();
    let p95_ms: Vec<f64> = runs.iter().map(|r| r.latency.p95_ms).collect();
    let p99_ms: Vec<f64> = runs.iter().map(|r| r.latency.p99_ms).collect();
    let p99_9_ms: Vec<f64> = runs.iter().map(|r| r.latency.p999_ms).collect();
    let max_ms: Vec<f64> = runs.iter().map(|r| r.latency.max_ms).collect();
    let min_ms: Vec<f64> = runs.iter().map(|r| r.latency.min_ms).collect();
    let stddev_ms: Vec<f64> = runs.iter().map(|r| r.latency.stddev_ms).collect();

    AggregatedLatencyMetrics {
        mean_ms: calculate_stats(&mean_ms),
        p50_ms: calculate_stats(&p50_ms),
        p90_ms: calculate_stats(&p90_ms),
        p95_ms: calculate_stats(&p95_ms),
        p99_ms: calculate_stats(&p99_ms),
        p999_ms: calculate_stats(&p99_9_ms),
        max_ms: calculate_stats(&max_ms),
        min_ms: calculate_stats(&min_ms),
        stddev_ms: calculate_stats(&stddev_ms),
    }
}

fn aggregate_resource_metrics(runs: &[BenchmarkResult]) -> AggregatedResourceMetrics {
    let avg_memory_mb: Vec<f64> = runs.iter().map(|r| r.resources.avg_memory_mb).collect();
    let peak_memory_mb: Vec<f64> = runs.iter().map(|r| r.resources.peak_memory_mb).collect();
    let p50_memory_mb: Vec<f64> = runs.iter().map(|r| r.resources.p50_memory_mb).collect();
    let p95_memory_mb: Vec<f64> = runs.iter().map(|r| r.resources.p95_memory_mb).collect();
    let p99_memory_mb: Vec<f64> = runs.iter().map(|r| r.resources.p99_memory_mb).collect();
    let avg_cpu_percent: Vec<f64> = runs.iter().map(|r| r.resources.avg_cpu_percent).collect();
    let peak_cpu_percent: Vec<f64> = runs.iter().map(|r| r.resources.peak_cpu_percent).collect();

    AggregatedResourceMetrics {
        avg_memory_mb: calculate_stats(&avg_memory_mb),
        peak_memory_mb: calculate_stats(&peak_memory_mb),
        p50_memory_mb: calculate_stats(&p50_memory_mb),
        p95_memory_mb: calculate_stats(&p95_memory_mb),
        p99_memory_mb: calculate_stats(&p99_memory_mb),
        avg_cpu_percent: calculate_stats(&avg_cpu_percent),
        peak_cpu_percent: calculate_stats(&peak_cpu_percent),
    }
}

fn aggregate_error_metrics(runs: &[BenchmarkResult]) -> AggregatedErrorMetrics {
    let values: Vec<&ErrorMetrics> = runs.iter().filter_map(|r| r.error_metrics.as_ref()).collect();

    if values.is_empty() {
        let zero_stats = MetricStats {
            mean: 0.0,
            median: 0.0,
            stddev: 0.0,
            min: 0.0,
            max: 0.0,
            ci_95: (0.0, 0.0),
            cv: 0.0,
        };
        return AggregatedErrorMetrics {
            validation_error_p99_ms: zero_stats.clone(),
            not_found_p99_ms: zero_stats.clone(),
            server_error_p99_ms: zero_stats.clone(),
            error_throughput_rps: zero_stats.clone(),
            error_memory_impact_mb: zero_stats.clone(),
            total_errors: zero_stats.clone(),
            error_rate: zero_stats,
        };
    }

    let validation_error_p99_ms: Vec<f64> = values.iter().map(|v| v.validation_error_p99_ms).collect();
    let not_found_p99_ms: Vec<f64> = values.iter().map(|v| v.not_found_p99_ms).collect();
    let server_error_p99_ms: Vec<f64> = values.iter().map(|v| v.server_error_p99_ms).collect();
    let error_throughput_rps: Vec<f64> = values.iter().map(|v| v.error_throughput_rps).collect();
    let error_memory_impact_mb: Vec<f64> = values.iter().map(|v| v.error_memory_impact_mb).collect();
    let total_errors: Vec<f64> = values.iter().map(|v| v.total_errors as f64).collect();
    let error_rate: Vec<f64> = values.iter().map(|v| v.error_rate).collect();

    AggregatedErrorMetrics {
        validation_error_p99_ms: calculate_stats(&validation_error_p99_ms),
        not_found_p99_ms: calculate_stats(&not_found_p99_ms),
        server_error_p99_ms: calculate_stats(&server_error_p99_ms),
        error_throughput_rps: calculate_stats(&error_throughput_rps),
        error_memory_impact_mb: calculate_stats(&error_memory_impact_mb),
        total_errors: calculate_stats(&total_errors),
        error_rate: calculate_stats(&error_rate),
    }
}

fn aggregate_serialization_metrics(runs: &[BenchmarkResult]) -> AggregatedSerializationMetrics {
    let values: Vec<&SerializationMetrics> = runs.iter().filter_map(|r| r.serialization.as_ref()).collect();

    if values.is_empty() {
        let zero_stats = MetricStats {
            mean: 0.0,
            median: 0.0,
            stddev: 0.0,
            min: 0.0,
            max: 0.0,
            ci_95: (0.0, 0.0),
            cv: 0.0,
        };
        return AggregatedSerializationMetrics {
            json_parse_overhead_ms: zero_stats.clone(),
            json_serialize_overhead_ms: zero_stats.clone(),
            validation_overhead_ms: zero_stats.clone(),
            total_overhead_pct: zero_stats.clone(),
            sample_count: zero_stats,
        };
    }

    let json_parse_overhead_ms: Vec<f64> = values.iter().map(|v| v.json_parse_overhead_ms).collect();
    let json_serialize_overhead_ms: Vec<f64> = values.iter().map(|v| v.json_serialize_overhead_ms).collect();
    let validation_overhead_ms: Vec<f64> = values.iter().map(|v| v.validation_overhead_ms).collect();
    let total_overhead_pct: Vec<f64> = values.iter().map(|v| v.total_overhead_pct).collect();
    let sample_count: Vec<f64> = values.iter().map(|v| v.sample_count as f64).collect();

    AggregatedSerializationMetrics {
        json_parse_overhead_ms: calculate_stats(&json_parse_overhead_ms),
        json_serialize_overhead_ms: calculate_stats(&json_serialize_overhead_ms),
        validation_overhead_ms: calculate_stats(&validation_overhead_ms),
        total_overhead_pct: calculate_stats(&total_overhead_pct),
        sample_count: calculate_stats(&sample_count),
    }
}

/// Detect outliers using the Interquartile Range (IQR) method
///
/// Returns indices of values that fall outside [Q1 - 1.5*IQR, Q3 + 1.5*IQR]
///
/// # Arguments
///
/// * `values` - Slice of values to analyze
///
/// # Returns
///
/// Vector of indices identifying outlier values
#[must_use]
pub fn detect_outliers(values: &[f64]) -> Vec<usize> {
    if values.len() < 4 {
        return Vec::new();
    }

    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let q1 = percentile(&sorted, 0.25);
    let q3 = percentile(&sorted, 0.75);
    let iqr = q3 - q1;

    let lower_bound = 1.5f64.mul_add(-iqr, q1);
    let upper_bound = 1.5f64.mul_add(iqr, q3);

    values
        .iter()
        .enumerate()
        .filter(|&(_, &v)| v < lower_bound || v > upper_bound)
        .map(|(i, _)| i)
        .collect()
}

/// Calculate confidence interval for the mean
///
/// Uses t-distribution for small samples (n < 30) and normal approximation for larger samples
///
/// # Arguments
///
/// * `values` - Slice of values
/// * `confidence` - Confidence level (e.g., 0.95 for 95% CI)
///
/// # Returns
///
/// Tuple of (`lower_bound`, `upper_bound`)
#[must_use]
pub fn calculate_confidence_interval(values: &[f64], confidence: f64) -> (f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0);
    }

    if values.len() == 1 {
        return (values[0], values[0]);
    }

    let mean_val = mean(values);
    let std = stddev(values);
    let n = values.len() as f64;

    let se = std / n.sqrt();

    let t_critical = if values.len() < 30 {
        match values.len() {
            2 => 12.706,
            3 => 4.303,
            4 => 3.182,
            5 => 2.776,
            6 => 2.571,
            7..=10 => 2.447,
            11..=20 => 2.228,
            _ => 2.093,
        }
    } else {
        match confidence {
            x if x >= 0.99 => 2.576,
            x if x >= 0.95 => 1.96,
            x if x >= 0.90 => 1.645,
            _ => 1.96,
        }
    };

    let margin = t_critical * se;
    (mean_val - margin, mean_val + margin)
}

/// Calculate coefficient of variation (stddev/mean)
///
/// Measures relative variability of a dataset
///
/// # Arguments
///
/// * `values` - Slice of values
///
/// # Returns
///
/// Coefficient of variation (0.0 if mean is zero)
#[must_use]
pub fn coefficient_of_variation(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }

    let mean_val = mean(values);
    if mean_val == 0.0 {
        return 0.0;
    }

    let std = stddev(values);
    std / mean_val
}

fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}

fn median(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }

    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let mid = sorted.len() / 2;
    if sorted.len().is_multiple_of(2) {
        f64::midpoint(sorted[mid - 1], sorted[mid])
    } else {
        sorted[mid]
    }
}

fn stddev(values: &[f64]) -> f64 {
    if values.len() < 2 {
        return 0.0;
    }

    let mean_val = mean(values);
    let variance = values
        .iter()
        .map(|&x| {
            let diff = x - mean_val;
            diff * diff
        })
        .sum::<f64>()
        / (values.len() - 1) as f64;

    variance.sqrt()
}

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn percentile(sorted_values: &[f64], p: f64) -> f64 {
    if sorted_values.is_empty() {
        return 0.0;
    }

    if sorted_values.len() == 1 {
        return sorted_values[0];
    }

    let index = p * (sorted_values.len() - 1) as f64;
    let lower = index.floor() as usize;
    let upper = index.ceil() as usize;

    if lower == upper {
        sorted_values[lower]
    } else {
        let weight = index - lower as f64;
        sorted_values[lower].mul_add(1.0 - weight, sorted_values[upper] * weight)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        assert_eq!(mean(&[1.0, 2.0, 3.0, 4.0, 5.0]), 3.0);
        assert_eq!(mean(&[]), 0.0);
        assert_eq!(mean(&[5.0]), 5.0);
    }

    #[test]
    fn test_median() {
        assert_eq!(median(&[1.0, 2.0, 3.0, 4.0, 5.0]), 3.0);
        assert_eq!(median(&[1.0, 2.0, 3.0, 4.0]), 2.5);
        assert_eq!(median(&[5.0]), 5.0);
        assert_eq!(median(&[]), 0.0);
    }

    #[test]
    fn test_stddev() {
        let values = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let std = stddev(&values);
        assert!((std - 2.138).abs() < 0.01);
    }

    #[test]
    fn test_detect_outliers() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(detect_outliers(&values), Vec::<usize>::new());

        let values = vec![1.0, 2.0, 3.0, 4.0, 100.0];
        let outliers = detect_outliers(&values);
        assert!(outliers.contains(&4));
    }

    #[test]
    fn test_coefficient_of_variation() {
        let values = vec![2.0, 4.0, 6.0, 8.0];
        let cv = coefficient_of_variation(&values);
        assert!(cv > 0.0);

        assert_eq!(coefficient_of_variation(&[0.0, 0.0]), 0.0);
    }

    #[test]
    fn test_confidence_interval() {
        let values = vec![10.0, 12.0, 14.0, 16.0, 18.0];
        let (lower, upper) = calculate_confidence_interval(&values, 0.95);
        assert!(lower < 14.0);
        assert!(upper > 14.0);
        assert!(upper > lower);
    }
}

//! Statistical analysis for framework comparisons
//!
//! Provides rigorous statistical testing using Welch's t-test and effect size
//! calculations (Cohen's d) to determine if performance differences between
//! frameworks are statistically significant and practically meaningful.
//!
//! # Statistical Methods
//!
//! - **Welch's t-test**: Robust to unequal variances and sample sizes, compares
//!   means of two samples and calculates p-value for statistical significance.
//! - **Cohen's d**: Standardized effect size measuring the magnitude of difference
//!   between two groups in units of standard deviations.
//!
//! # References
//!
//! - Welch, B. L. (1947). "The generalization of 'Student's' problem when several
//!   different population variances are involved". Biometrika. 34 (1–2): 28–35.
//! - Cohen, J. (1988). Statistical Power Analysis for the Behavioral Sciences.

use crate::schema::{
    compare::{EffectSize, StatisticalTest},
    profile::ProfileResult,
};
use serde::{Deserialize, Serialize};
use statrs::distribution::{ContinuousCDF, StudentsT};

/// Complete statistical comparison analysis between two frameworks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonAnalysis {
    /// Framework being compared to baseline
    pub framework: String,

    /// Statistical hypothesis tests for key metrics
    pub statistical_tests: Vec<StatisticalTest>,

    /// Effect sizes for key metrics
    pub effect_sizes: Vec<EffectSize>,

    /// Overall verdict: "`significantly_better`", "`significantly_worse`", "similar"
    pub overall_verdict: String,
}

/// Statistical comparison analyzer
///
/// Performs rigorous statistical testing to determine if performance differences
/// between frameworks are statistically significant and practically meaningful.
#[derive(Debug)]
pub struct CompareAnalyzer {
    /// Statistical significance threshold (alpha level), typically 0.05
    significance_threshold: f64,
}

impl CompareAnalyzer {
    /// Create a new analyzer with the given significance threshold
    ///
    /// # Arguments
    ///
    /// * `significance_threshold` - Alpha level for hypothesis testing (e.g., 0.05 for 95% confidence)
    ///
    /// # Example
    ///
    /// ```
    /// use benchmark_harness::compare::CompareAnalyzer;
    ///
    /// let analyzer = CompareAnalyzer::new(0.05); // 95% confidence level
    /// ```
    #[must_use]
    pub const fn new(significance_threshold: f64) -> Self {
        Self { significance_threshold }
    }

    /// Compare two frameworks using per-workload ratio-based comparison
    ///
    /// For each workload that both frameworks ran, computes the RPS ratio
    /// (comparison / baseline). The geometric mean of these ratios gives the
    /// overall performance comparison. Per-workload breakdowns are reported
    /// as individual effect sizes.
    ///
    /// If multiple independent runs of the SAME workload exist (i.e. repeated
    /// iterations), those replicate samples ARE valid for a Welch's t-test.
    /// When only a single run per workload exists, the t-test is not applicable
    /// and the comparison relies on ratio-based analysis instead.
    ///
    /// # Arguments
    ///
    /// * `baseline` - Baseline framework profile result
    /// * `comparison` - Framework being compared to baseline
    ///
    /// # Returns
    ///
    /// Complete statistical analysis with per-workload comparisons and overall verdict
    #[must_use]
    pub fn compare_frameworks(&self, baseline: &ProfileResult, comparison: &ProfileResult) -> ComparisonAnalysis {
        let mut statistical_tests = Vec::new();
        let mut effect_sizes = Vec::new();

        // Build a map of workload name -> metrics for each framework
        let baseline_workloads = Self::build_workload_map(baseline);
        let comparison_workloads = Self::build_workload_map(comparison);

        // Collect per-workload RPS ratios for matching workloads
        let mut rps_ratios: Vec<f64> = Vec::new();
        let mut latency_p50_ratios: Vec<f64> = Vec::new();
        let mut latency_p95_ratios: Vec<f64> = Vec::new();
        let mut latency_p99_ratios: Vec<f64> = Vec::new();

        for (workload_name, baseline_metrics) in &baseline_workloads {
            if let Some(comparison_metrics) = comparison_workloads.get(workload_name.as_str()) {
                // Per-workload RPS comparison (ratio > 1.0 means comparison is faster)
                let baseline_rps = baseline_metrics.0;
                let comparison_rps = comparison_metrics.0;

                if baseline_rps > 0.0 {
                    let ratio = comparison_rps / baseline_rps;
                    rps_ratios.push(ratio);

                    // Report per-workload ratio as an effect size entry
                    effect_sizes.push(EffectSize {
                        metric: format!("rps_ratio:{workload_name}"),
                        cohens_d: ratio, // Using cohens_d field to store the ratio
                        magnitude: format!("{:+.1}%", (ratio - 1.0) * 100.0),
                    });
                }

                // Per-workload latency comparisons (ratio < 1.0 means comparison is faster)
                let (b_p50, b_p95, b_p99) = baseline_metrics.1;
                let (c_p50, c_p95, c_p99) = comparison_metrics.1;

                if b_p50 > 0.0 {
                    latency_p50_ratios.push(c_p50 / b_p50);
                }
                if b_p95 > 0.0 {
                    latency_p95_ratios.push(c_p95 / b_p95);
                }
                if b_p99 > 0.0 {
                    latency_p99_ratios.push(c_p99 / b_p99);
                }
            }
        }

        // Also check if we have multiple independent runs of the same workload
        // (replicate iterations). Only then is a t-test statistically valid.
        let baseline_replicates = Self::extract_replicate_samples(baseline);
        let comparison_replicates = Self::extract_replicate_samples(comparison);

        for (workload_name, baseline_samples) in &baseline_replicates {
            if let Some(comparison_samples) = comparison_replicates.get(workload_name.as_str()) {
                // Only run t-test if we have >= 2 replicate runs per workload
                if baseline_samples.len() >= 2 && comparison_samples.len() >= 2 {
                    statistical_tests.push(self.welch_t_test(
                        baseline_samples,
                        comparison_samples,
                        &format!("rps:{workload_name}"),
                    ));
                }
            }
        }

        // Compute geometric mean of RPS ratios as the overall comparison metric.
        // Geometric mean is appropriate for ratios because it is symmetric:
        // geo_mean(a/b) = 1 / geo_mean(b/a).
        let geo_mean_rps = if rps_ratios.is_empty() {
            1.0
        } else {
            let log_sum: f64 = rps_ratios.iter().map(|r| r.ln()).sum();
            (log_sum / rps_ratios.len() as f64).exp()
        };

        // Report overall geometric mean ratios as statistical test entries
        if !rps_ratios.is_empty() {
            statistical_tests.push(StatisticalTest {
                test_name: "geometric_mean_ratio".to_string(),
                metric: "requests_per_sec".to_string(),
                statistic: geo_mean_rps,
                p_value: f64::NAN, // Not applicable for ratio-based comparison
                is_significant: (geo_mean_rps - 1.0).abs() > 0.05, // >5% difference
                confidence_interval: (
                    *rps_ratios
                        .iter()
                        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                        .unwrap_or(&1.0),
                    *rps_ratios
                        .iter()
                        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                        .unwrap_or(&1.0),
                ),
            });
        }

        if !latency_p50_ratios.is_empty() {
            let geo_mean = Self::geometric_mean(&latency_p50_ratios);
            statistical_tests.push(StatisticalTest {
                test_name: "geometric_mean_ratio".to_string(),
                metric: "latency_p50_ms".to_string(),
                statistic: geo_mean,
                p_value: f64::NAN,
                is_significant: (geo_mean - 1.0).abs() > 0.05,
                confidence_interval: (0.0, 0.0),
            });
        }

        if !latency_p95_ratios.is_empty() {
            let geo_mean = Self::geometric_mean(&latency_p95_ratios);
            statistical_tests.push(StatisticalTest {
                test_name: "geometric_mean_ratio".to_string(),
                metric: "latency_p95_ms".to_string(),
                statistic: geo_mean,
                p_value: f64::NAN,
                is_significant: (geo_mean - 1.0).abs() > 0.05,
                confidence_interval: (0.0, 0.0),
            });
        }

        if !latency_p99_ratios.is_empty() {
            let geo_mean = Self::geometric_mean(&latency_p99_ratios);
            statistical_tests.push(StatisticalTest {
                test_name: "geometric_mean_ratio".to_string(),
                metric: "latency_p99_ms".to_string(),
                statistic: geo_mean,
                p_value: f64::NAN,
                is_significant: (geo_mean - 1.0).abs() > 0.05,
                confidence_interval: (0.0, 0.0),
            });
        }

        // Determine overall verdict based on geometric mean RPS ratio.
        // Convention: baseline is sample1 (the reference), comparison is sample2.
        // geo_mean_rps > 1.0 means comparison is FASTER than baseline.
        // geo_mean_rps < 1.0 means comparison is SLOWER than baseline.
        let overall_verdict = if rps_ratios.is_empty() {
            "insufficient_data".to_string()
        } else if geo_mean_rps > 1.05 {
            // Comparison framework is >5% faster than baseline
            "significantly_better".to_string()
        } else if geo_mean_rps < 0.95 {
            // Comparison framework is >5% slower than baseline
            "significantly_worse".to_string()
        } else {
            "similar".to_string()
        };

        ComparisonAnalysis {
            framework: comparison.framework.name.clone(),
            statistical_tests,
            effect_sizes,
            overall_verdict,
        }
    }

    /// Perform Welch's t-test for two independent samples of the SAME measurement.
    ///
    /// **IMPORTANT**: Both samples must be replicate observations of the same workload.
    /// Mixing values from different workloads (e.g., JSON body RPS vs query params RPS)
    /// violates the independence assumption and produces invalid results.
    ///
    /// Welch's t-test is a robust alternative to Student's t-test that does not
    /// assume equal variances. It uses the Welch-Satterthwaite equation for
    /// degrees of freedom calculation.
    ///
    /// # Convention (IMPORTANT -- do not change without updating verdict logic)
    ///
    /// * `sample1` is the **baseline** framework's replicate measurements.
    /// * `sample2` is the **comparison** framework's replicate measurements.
    /// * `t_stat = (mean1 - mean2) / se`, so:
    ///   - `t_stat > 0` means the **baseline** has a higher mean.
    ///   - `t_stat < 0` means the **comparison** has a higher mean.
    /// * For RPS metrics, `t_stat < 0` means comparison is faster (better).
    /// * For latency metrics, `t_stat < 0` means comparison is slower (worse).
    ///
    /// # Algorithm
    ///
    /// 1. Calculate sample means and variances
    /// 2. Compute Welch-Satterthwaite degrees of freedom
    /// 3. Calculate t-statistic: `(mean1 - mean2) / se`
    /// 4. Compute two-tailed p-value from Student's t-distribution
    /// 5. Calculate 95% CI using the t-distribution critical value (not z=1.96)
    ///
    /// # Arguments
    ///
    /// * `sample1` - Baseline replicate samples (must be >= 2 for valid test)
    /// * `sample2` - Comparison replicate samples (must be >= 2 for valid test)
    /// * `metric_name` - Name of metric being tested
    ///
    /// # Returns
    ///
    /// Statistical test result with t-statistic, p-value, and confidence interval.
    /// Returns a non-significant placeholder if either sample has < 2 observations.
    #[must_use] 
    pub fn welch_t_test(&self, sample1: &[f64], sample2: &[f64], metric_name: &str) -> StatisticalTest {
        if sample1.is_empty() || sample2.is_empty() {
            return StatisticalTest {
                test_name: "welch_t_test".to_string(),
                metric: metric_name.to_string(),
                statistic: 0.0,
                p_value: 1.0,
                is_significant: false,
                confidence_interval: (0.0, 0.0),
            };
        }

        if sample1.len() == 1 && sample2.len() == 1 {
            return StatisticalTest {
                test_name: "welch_t_test".to_string(),
                metric: metric_name.to_string(),
                statistic: 0.0,
                p_value: 1.0,
                is_significant: false,
                confidence_interval: (0.0, 0.0),
            };
        }

        let mean1 = sample1.iter().sum::<f64>() / sample1.len() as f64;
        let mean2 = sample2.iter().sum::<f64>() / sample2.len() as f64;

        let var1 = if sample1.len() > 1 {
            sample1.iter().map(|&x| (x - mean1).powi(2)).sum::<f64>() / (sample1.len() - 1) as f64
        } else {
            0.0
        };

        let var2 = if sample2.len() > 1 {
            sample2.iter().map(|&x| (x - mean2).powi(2)).sum::<f64>() / (sample2.len() - 1) as f64
        } else {
            0.0
        };

        let se1 = var1 / sample1.len() as f64;
        let se2 = var2 / sample2.len() as f64;
        let se_diff = (se1 + se2).sqrt();

        if se_diff == 0.0 {
            let is_different = (mean1 - mean2).abs() > f64::EPSILON;
            return StatisticalTest {
                test_name: "welch_t_test".to_string(),
                metric: metric_name.to_string(),
                statistic: if is_different { f64::INFINITY } else { 0.0 },
                p_value: if is_different { 0.0 } else { 1.0 },
                is_significant: is_different,
                confidence_interval: (mean1 - mean2, mean1 - mean2),
            };
        }

        let t_stat = (mean1 - mean2) / se_diff;

        let df = if var1 > 0.0 && var2 > 0.0 {
            let numerator = (se1 + se2).powi(2);
            let denominator = (se1.powi(2) / (sample1.len() - 1) as f64) + (se2.powi(2) / (sample2.len() - 1) as f64);
            numerator / denominator
        } else {
            (sample1.len() + sample2.len() - 2) as f64
        };

        let t_dist = StudentsT::new(0.0, 1.0, df).unwrap_or_else(|_| StudentsT::new(0.0, 1.0, 1.0).unwrap());
        let p_value = 2.0 * (1.0 - t_dist.cdf(t_stat.abs()));

        // Use the t-distribution inverse CDF for the 95% CI critical value
        // at the Welch-Satterthwaite degrees of freedom, NOT the z=1.96 approximation.
        let t_critical = t_dist.inverse_cdf(0.975); // two-tailed 95% CI
        let margin_of_error = t_critical * se_diff;
        let mean_diff = mean1 - mean2;
        let ci_lower = mean_diff - margin_of_error;
        let ci_upper = mean_diff + margin_of_error;

        StatisticalTest {
            test_name: "welch_t_test".to_string(),
            metric: metric_name.to_string(),
            statistic: t_stat,
            p_value,
            is_significant: p_value < self.significance_threshold,
            confidence_interval: (ci_lower, ci_upper),
        }
    }

    /// Calculate Cohen's d effect size
    ///
    /// Cohen's d measures the standardized difference between two means,
    /// providing a scale-independent measure of effect magnitude.
    ///
    /// # Formula
    ///
    /// d = (mean1 - mean2) / `pooled_standard_deviation`
    ///
    /// where `pooled_sd` = sqrt(((n1-1)*sd1² + (n2-1)*sd2²) / (n1 + n2 - 2))
    ///
    /// # Arguments
    ///
    /// * `sample1` - Baseline sample data
    /// * `sample2` - Comparison sample data
    /// * `metric_name` - Name of metric being measured
    ///
    /// # Returns
    ///
    /// Effect size with Cohen's d value and magnitude classification
    #[must_use] 
    pub fn cohens_d(&self, sample1: &[f64], sample2: &[f64], metric_name: &str) -> EffectSize {
        if sample1.is_empty() || sample2.is_empty() {
            return EffectSize {
                metric: metric_name.to_string(),
                cohens_d: 0.0,
                magnitude: "none".to_string(),
            };
        }

        let mean1 = sample1.iter().sum::<f64>() / sample1.len() as f64;
        let mean2 = sample2.iter().sum::<f64>() / sample2.len() as f64;

        let sd1 = if sample1.len() > 1 {
            let var = sample1.iter().map(|&x| (x - mean1).powi(2)).sum::<f64>() / (sample1.len() - 1) as f64;
            var.sqrt()
        } else {
            0.0
        };

        let sd2 = if sample2.len() > 1 {
            let var = sample2.iter().map(|&x| (x - mean2).powi(2)).sum::<f64>() / (sample2.len() - 1) as f64;
            var.sqrt()
        } else {
            0.0
        };

        let n1 = sample1.len() as f64;
        let n2 = sample2.len() as f64;

        let pooled_sd = if n1 + n2 > 2.0 {
            let numerator = (n1 - 1.0).mul_add(sd1.powi(2), (n2 - 1.0) * sd2.powi(2));
            let denominator = n1 + n2 - 2.0;
            (numerator / denominator).sqrt()
        } else {
            f64::midpoint(sd1, sd2)
        };

        let d = if pooled_sd > 0.0 {
            (mean1 - mean2) / pooled_sd
        } else {
            0.0
        };

        let magnitude = Self::classify_effect_size(d.abs());

        EffectSize {
            metric: metric_name.to_string(),
            cohens_d: d,
            magnitude,
        }
    }

    /// Classify effect size magnitude based on Cohen's guidelines (Cohen, 1988)
    ///
    /// # Guidelines
    ///
    /// - |d| < 0.2: Negligible effect (below Cohen's "small" threshold)
    /// - 0.2 <= |d| < 0.5: Small effect
    /// - 0.5 <= |d| < 0.8: Medium effect
    /// - |d| >= 0.8: Large effect
    fn classify_effect_size(d_abs: f64) -> String {
        if d_abs < 0.2 {
            "negligible".to_string()
        } else if d_abs < 0.5 {
            "small".to_string()
        } else if d_abs < 0.8 {
            "medium".to_string()
        } else {
            "large".to_string()
        }
    }

    /// Build a map of workload name -> (rps, (p50, p95, p99)) for per-workload comparison
    fn build_workload_map(profile: &ProfileResult) -> std::collections::HashMap<String, (f64, (f64, f64, f64))> {
        let mut map = std::collections::HashMap::new();
        for suite in &profile.suites {
            for workload in &suite.workloads {
                let rps = workload.results.throughput.requests_per_sec;
                let p50 = workload.results.latency.median_ms;
                let p95 = workload.results.latency.p95_ms;
                let p99 = workload.results.latency.p99_ms;
                map.insert(workload.name.clone(), (rps, (p50, p95, p99)));
            }
        }
        map
    }

    /// Extract replicate samples grouped by workload name.
    ///
    /// If a workload name appears more than once (i.e. multiple iterations were run),
    /// we have valid replicate samples suitable for a t-test.
    fn extract_replicate_samples(profile: &ProfileResult) -> std::collections::HashMap<String, Vec<f64>> {
        let mut map: std::collections::HashMap<String, Vec<f64>> = std::collections::HashMap::new();
        for suite in &profile.suites {
            for workload in &suite.workloads {
                map.entry(workload.name.clone())
                    .or_default()
                    .push(workload.results.throughput.requests_per_sec);
            }
        }
        map
    }

    /// Compute geometric mean of a slice of positive values
    fn geometric_mean(values: &[f64]) -> f64 {
        if values.is_empty() {
            return 1.0;
        }
        let log_sum: f64 = values.iter().map(|r| r.ln()).sum();
        (log_sum / values.len() as f64).exp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welch_t_test_known_values() {
        let analyzer = CompareAnalyzer::new(0.05);
        let sample1 = vec![10.0, 12.0, 14.0, 16.0, 18.0];
        let sample2 = vec![20.0, 22.0, 24.0, 26.0, 28.0];

        let result = analyzer.welch_t_test(&sample1, &sample2, "test_metric");

        assert_eq!(result.test_name, "welch_t_test");
        assert_eq!(result.metric, "test_metric");
        assert!(
            result.statistic < -4.0 && result.statistic > -6.0,
            "t-statistic should be around -5.0, got {}",
            result.statistic
        );
        assert!(
            result.p_value < 0.01,
            "p-value should be < 0.01, got {}",
            result.p_value
        );
        assert!(result.is_significant, "Result should be significant");

        assert!(result.confidence_interval.0 < 0.0 && result.confidence_interval.1 < 0.0);
    }

    #[test]
    fn test_welch_t_test_identical_samples() {
        let analyzer = CompareAnalyzer::new(0.05);
        let sample1 = vec![10.0, 10.0, 10.0, 10.0, 10.0];
        let sample2 = vec![10.0, 10.0, 10.0, 10.0, 10.0];

        let result = analyzer.welch_t_test(&sample1, &sample2, "test_metric");

        assert!(result.statistic.abs() < 1e-10);
        assert!((result.p_value - 1.0).abs() < 1e-10);
        assert!(!result.is_significant);
    }

    #[test]
    fn test_welch_t_test_high_variance() {
        let analyzer = CompareAnalyzer::new(0.05);

        let sample1 = vec![10.0, 10.1, 9.9, 10.2, 9.8];

        let sample2 = vec![5.0, 15.0, 8.0, 12.0, 10.0];

        let result = analyzer.welch_t_test(&sample1, &sample2, "test_metric");

        assert!(!result.p_value.is_nan());
        assert!(!result.statistic.is_nan());
    }

    #[test]
    fn test_welch_t_test_empty_samples() {
        let analyzer = CompareAnalyzer::new(0.05);
        let sample1: Vec<f64> = vec![];
        let sample2 = vec![10.0, 20.0, 30.0];

        let result = analyzer.welch_t_test(&sample1, &sample2, "test_metric");

        assert!(result.statistic.abs() < 1e-10);
        assert!((result.p_value - 1.0).abs() < 1e-10);
        assert!(!result.is_significant);
    }

    #[test]
    fn test_cohens_d_classification_negligible() {
        let analyzer = CompareAnalyzer::new(0.05);

        let sample1 = vec![10.0, 10.0, 10.0, 10.0, 10.0];
        let sample2 = vec![10.1, 10.1, 10.1, 10.1, 10.1];

        let result = analyzer.cohens_d(&sample1, &sample2, "test_metric");

        assert!(
            result.cohens_d.abs() < 0.2,
            "Effect size should be negligible, got d={}",
            result.cohens_d
        );
        assert_eq!(result.magnitude, "negligible");
    }

    #[test]
    fn test_cohens_d_classification_small() {
        let analyzer = CompareAnalyzer::new(0.05);

        let sample1 = vec![10.5, 11.0, 11.5, 12.0, 12.5];
        let sample2 = vec![10.0, 10.5, 11.0, 11.5, 12.0];

        let result = analyzer.cohens_d(&sample1, &sample2, "test_metric");

        assert!(
            result.cohens_d.abs() >= 0.2,
            "Effect size should be at least small, got d={}",
            result.cohens_d
        );
        assert!(result.magnitude == "small" || result.magnitude == "medium");
    }

    #[test]
    fn test_cohens_d_classification_medium() {
        let analyzer = CompareAnalyzer::new(0.05);

        let sample1 = vec![15.0, 16.0, 17.0, 18.0, 19.0];
        let sample2 = vec![10.0, 11.0, 12.0, 13.0, 14.0];

        let result = analyzer.cohens_d(&sample1, &sample2, "test_metric");

        assert!(result.cohens_d.abs() >= 0.5);
        assert!(result.magnitude == "medium" || result.magnitude == "large");
    }

    #[test]
    fn test_cohens_d_classification_large() {
        let analyzer = CompareAnalyzer::new(0.05);

        let sample1 = vec![20.0, 22.0, 24.0, 26.0, 28.0];
        let sample2 = vec![10.0, 12.0, 14.0, 16.0, 18.0];

        let result = analyzer.cohens_d(&sample1, &sample2, "test_metric");

        assert!(result.cohens_d.abs() >= 0.8);
        assert_eq!(result.magnitude, "large");
    }

    #[test]
    fn test_cohens_d_identical_samples() {
        let analyzer = CompareAnalyzer::new(0.05);
        let sample1 = vec![10.0, 10.0, 10.0, 10.0, 10.0];
        let sample2 = vec![10.0, 10.0, 10.0, 10.0, 10.0];

        let result = analyzer.cohens_d(&sample1, &sample2, "test_metric");

        assert!(result.cohens_d.abs() < 1e-10);
        assert_eq!(result.magnitude, "negligible");
    }

    #[test]
    fn test_cohens_d_negative_values() {
        let analyzer = CompareAnalyzer::new(0.05);
        // Convention: sample1 = baseline, sample2 = comparison.
        // When baseline mean < comparison mean, Cohen's d is negative,
        // meaning the comparison framework has higher values.
        let sample1 = vec![10.0, 11.0, 12.0, 13.0, 14.0];
        let sample2 = vec![20.0, 21.0, 22.0, 23.0, 24.0];

        let result = analyzer.cohens_d(&sample1, &sample2, "test_metric");

        assert!(
            result.cohens_d < 0.0,
            "Cohen's d should be negative when baseline (sample1) mean is lower than comparison (sample2) mean"
        );
    }

    #[test]
    fn test_effect_size_classification_boundaries() {
        // Cohen's d classification (Cohen, 1988):
        // < 0.2: negligible, 0.2-0.5: small, 0.5-0.8: medium, >= 0.8: large
        assert_eq!(CompareAnalyzer::classify_effect_size(0.0), "negligible");
        assert_eq!(CompareAnalyzer::classify_effect_size(0.1), "negligible");
        assert_eq!(CompareAnalyzer::classify_effect_size(0.19), "negligible");
        assert_eq!(CompareAnalyzer::classify_effect_size(0.2), "small");
        assert_eq!(CompareAnalyzer::classify_effect_size(0.4), "small");
        assert_eq!(CompareAnalyzer::classify_effect_size(0.49), "small");
        assert_eq!(CompareAnalyzer::classify_effect_size(0.5), "medium");
        assert_eq!(CompareAnalyzer::classify_effect_size(0.7), "medium");
        assert_eq!(CompareAnalyzer::classify_effect_size(0.79), "medium");
        assert_eq!(CompareAnalyzer::classify_effect_size(0.8), "large");
        assert_eq!(CompareAnalyzer::classify_effect_size(1.0), "large");
        assert_eq!(CompareAnalyzer::classify_effect_size(2.0), "large");
    }

    #[test]
    fn test_geometric_mean() {
        // geometric mean of [2, 8] = sqrt(16) = 4
        let result = CompareAnalyzer::geometric_mean(&[2.0, 8.0]);
        assert!((result - 4.0).abs() < 1e-10, "Expected 4.0, got {result}");

        // geometric mean of empty = 1.0
        assert!((CompareAnalyzer::geometric_mean(&[]) - 1.0).abs() < 1e-10);

        // geometric mean of [1.0] = 1.0
        assert!((CompareAnalyzer::geometric_mean(&[1.0]) - 1.0).abs() < 1e-10);
    }
}

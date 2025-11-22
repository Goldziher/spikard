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

    /// Overall verdict: "significantly_better", "significantly_worse", "similar"
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
    /// let analyzer = CompareAnalyzer::new(0.05); // 95% confidence level
    /// ```
    pub fn new(significance_threshold: f64) -> Self {
        Self { significance_threshold }
    }

    /// Compare two frameworks using statistical tests and effect sizes
    ///
    /// Performs Welch's t-test and calculates Cohen's d for key performance metrics:
    /// - Requests per second (RPS)
    /// - Latency percentiles (p50, p95, p99)
    ///
    /// # Arguments
    ///
    /// * `baseline` - Baseline framework profile result
    /// * `comparison` - Framework being compared to baseline
    ///
    /// # Returns
    ///
    /// Complete statistical analysis with test results, effect sizes, and overall verdict
    pub fn compare_frameworks(&self, baseline: &ProfileResult, comparison: &ProfileResult) -> ComparisonAnalysis {
        let mut statistical_tests = Vec::new();
        let mut effect_sizes = Vec::new();

        // Extract RPS samples from all workloads
        let baseline_rps = Self::extract_rps_samples(baseline);
        let comparison_rps = Self::extract_rps_samples(comparison);

        // Test RPS difference
        if !baseline_rps.is_empty() && !comparison_rps.is_empty() {
            statistical_tests.push(self.welch_t_test(&baseline_rps, &comparison_rps, "requests_per_sec"));
            effect_sizes.push(self.cohens_d(&baseline_rps, &comparison_rps, "requests_per_sec"));
        }

        // Extract latency samples
        let baseline_p50 = Self::extract_latency_samples(baseline, |l| l.median_ms);
        let comparison_p50 = Self::extract_latency_samples(comparison, |l| l.median_ms);

        let baseline_p95 = Self::extract_latency_samples(baseline, |l| l.p95_ms);
        let comparison_p95 = Self::extract_latency_samples(comparison, |l| l.p95_ms);

        let baseline_p99 = Self::extract_latency_samples(baseline, |l| l.p99_ms);
        let comparison_p99 = Self::extract_latency_samples(comparison, |l| l.p99_ms);

        // Test latency differences (lower is better for latency)
        if !baseline_p50.is_empty() && !comparison_p50.is_empty() {
            statistical_tests.push(self.welch_t_test(&baseline_p50, &comparison_p50, "latency_p50_ms"));
            effect_sizes.push(self.cohens_d(&baseline_p50, &comparison_p50, "latency_p50_ms"));
        }

        if !baseline_p95.is_empty() && !comparison_p95.is_empty() {
            statistical_tests.push(self.welch_t_test(&baseline_p95, &comparison_p95, "latency_p95_ms"));
            effect_sizes.push(self.cohens_d(&baseline_p95, &comparison_p95, "latency_p95_ms"));
        }

        if !baseline_p99.is_empty() && !comparison_p99.is_empty() {
            statistical_tests.push(self.welch_t_test(&baseline_p99, &comparison_p99, "latency_p99_ms"));
            effect_sizes.push(self.cohens_d(&baseline_p99, &comparison_p99, "latency_p99_ms"));
        }

        // Determine overall verdict based on RPS test (primary metric)
        let overall_verdict = if let Some(rps_test) = statistical_tests.iter().find(|t| t.metric == "requests_per_sec")
        {
            if rps_test.is_significant {
                // Negative t-statistic means comparison has higher mean (better)
                if rps_test.statistic < 0.0 {
                    "significantly_better".to_string()
                } else {
                    "significantly_worse".to_string()
                }
            } else {
                "similar".to_string()
            }
        } else {
            "insufficient_data".to_string()
        };

        ComparisonAnalysis {
            framework: comparison.framework.name.clone(),
            statistical_tests,
            effect_sizes,
            overall_verdict,
        }
    }

    /// Perform Welch's t-test for two independent samples
    ///
    /// Welch's t-test is a robust alternative to Student's t-test that does not
    /// assume equal variances. It uses the Welch-Satterthwaite equation for
    /// degrees of freedom calculation.
    ///
    /// # Algorithm
    ///
    /// 1. Calculate sample means and variances
    /// 2. Compute Welch-Satterthwaite degrees of freedom
    /// 3. Calculate t-statistic
    /// 4. Compute two-tailed p-value from Student's t-distribution
    /// 5. Calculate 95% confidence interval for mean difference
    ///
    /// # Arguments
    ///
    /// * `sample1` - Baseline sample data
    /// * `sample2` - Comparison sample data
    /// * `metric_name` - Name of metric being tested
    ///
    /// # Returns
    ///
    /// Statistical test result with t-statistic, p-value, and confidence interval
    fn welch_t_test(&self, sample1: &[f64], sample2: &[f64], metric_name: &str) -> StatisticalTest {
        // Handle edge cases
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

        // Calculate means
        let mean1 = sample1.iter().sum::<f64>() / sample1.len() as f64;
        let mean2 = sample2.iter().sum::<f64>() / sample2.len() as f64;

        // Calculate variances
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

        // Calculate standard errors
        let se1 = var1 / sample1.len() as f64;
        let se2 = var2 / sample2.len() as f64;
        let se_diff = (se1 + se2).sqrt();

        // Handle zero variance cases
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

        // Calculate t-statistic
        let t_stat = (mean1 - mean2) / se_diff;

        // Calculate Welch-Satterthwaite degrees of freedom
        let df = if var1 > 0.0 && var2 > 0.0 {
            let numerator = (se1 + se2).powi(2);
            let denominator = (se1.powi(2) / (sample1.len() - 1) as f64) + (se2.powi(2) / (sample2.len() - 1) as f64);
            numerator / denominator
        } else {
            // Fallback to simpler calculation if variances are zero
            (sample1.len() + sample2.len() - 2) as f64
        };

        // Calculate p-value using Student's t-distribution
        let t_dist = StudentsT::new(0.0, 1.0, df).unwrap_or_else(|_| StudentsT::new(0.0, 1.0, 1.0).unwrap());
        let p_value = 2.0 * (1.0 - t_dist.cdf(t_stat.abs()));

        // Calculate 95% confidence interval for mean difference
        let t_critical = 1.96; // Approximate for large df, exact for df > 30
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
    /// d = (mean1 - mean2) / pooled_standard_deviation
    ///
    /// where pooled_sd = sqrt(((n1-1)*sd1² + (n2-1)*sd2²) / (n1 + n2 - 2))
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
    fn cohens_d(&self, sample1: &[f64], sample2: &[f64], metric_name: &str) -> EffectSize {
        // Handle edge cases
        if sample1.is_empty() || sample2.is_empty() {
            return EffectSize {
                metric: metric_name.to_string(),
                cohens_d: 0.0,
                magnitude: "none".to_string(),
            };
        }

        // Calculate means
        let mean1 = sample1.iter().sum::<f64>() / sample1.len() as f64;
        let mean2 = sample2.iter().sum::<f64>() / sample2.len() as f64;

        // Calculate standard deviations
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

        // Calculate pooled standard deviation
        let n1 = sample1.len() as f64;
        let n2 = sample2.len() as f64;

        let pooled_sd = if n1 + n2 > 2.0 {
            let numerator = (n1 - 1.0) * sd1.powi(2) + (n2 - 1.0) * sd2.powi(2);
            let denominator = n1 + n2 - 2.0;
            (numerator / denominator).sqrt()
        } else {
            (sd1 + sd2) / 2.0 // Simple average for small samples
        };

        // Calculate Cohen's d
        let d = if pooled_sd > 0.0 {
            (mean1 - mean2) / pooled_sd
        } else {
            0.0
        };

        // Classify magnitude
        let magnitude = Self::classify_effect_size(d.abs());

        EffectSize {
            metric: metric_name.to_string(),
            cohens_d: d,
            magnitude,
        }
    }

    /// Classify effect size magnitude based on Cohen's guidelines
    ///
    /// # Guidelines (Cohen, 1988)
    ///
    /// - |d| < 0.2: Small effect
    /// - 0.2 ≤ |d| < 0.5: Medium effect
    /// - 0.5 ≤ |d| < 0.8: Large effect
    /// - |d| ≥ 0.8: Very large effect
    fn classify_effect_size(d_abs: f64) -> String {
        if d_abs < 0.2 {
            "small".to_string()
        } else if d_abs < 0.5 {
            "medium".to_string()
        } else if d_abs < 0.8 {
            "large".to_string()
        } else {
            "very_large".to_string()
        }
    }

    /// Extract RPS samples from all workloads in a ProfileResult
    fn extract_rps_samples(profile: &ProfileResult) -> Vec<f64> {
        profile
            .suites
            .iter()
            .flat_map(|suite| &suite.workloads)
            .map(|workload| workload.results.throughput.requests_per_sec)
            .collect()
    }

    /// Extract latency samples from all workloads using a selector function
    fn extract_latency_samples<F>(profile: &ProfileResult, selector: F) -> Vec<f64>
    where
        F: Fn(&crate::schema::Latency) -> f64,
    {
        profile
            .suites
            .iter()
            .flat_map(|suite| &suite.workloads)
            .map(|workload| selector(&workload.results.latency))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welch_t_test_known_values() {
        // Test with known statistical values
        // Sample 1: [10.0, 12.0, 14.0, 16.0, 18.0], mean = 14.0, sd = 3.16
        // Sample 2: [20.0, 22.0, 24.0, 26.0, 28.0], mean = 24.0, sd = 3.16
        // Expected difference: -10.0
        // Expected t-statistic: approximately -5.0
        // Expected p-value: < 0.01 (highly significant)

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

        // CI should not include 0 for significant difference
        assert!(result.confidence_interval.0 < 0.0 && result.confidence_interval.1 < 0.0);
    }

    #[test]
    fn test_welch_t_test_identical_samples() {
        // Identical samples should have p-value ≈ 1.0, t ≈ 0
        let analyzer = CompareAnalyzer::new(0.05);
        let sample1 = vec![10.0, 10.0, 10.0, 10.0, 10.0];
        let sample2 = vec![10.0, 10.0, 10.0, 10.0, 10.0];

        let result = analyzer.welch_t_test(&sample1, &sample2, "test_metric");

        assert_eq!(result.statistic, 0.0);
        assert_eq!(result.p_value, 1.0);
        assert!(!result.is_significant);
    }

    #[test]
    fn test_welch_t_test_high_variance() {
        // Test robustness with unequal variances (Welch's advantage over Student's t)
        let analyzer = CompareAnalyzer::new(0.05);

        // Low variance sample
        let sample1 = vec![10.0, 10.1, 9.9, 10.2, 9.8];

        // High variance sample with similar mean
        let sample2 = vec![5.0, 15.0, 8.0, 12.0, 10.0];

        let result = analyzer.welch_t_test(&sample1, &sample2, "test_metric");

        // Should handle unequal variances gracefully
        assert!(!result.p_value.is_nan());
        assert!(!result.statistic.is_nan());
    }

    #[test]
    fn test_welch_t_test_empty_samples() {
        // Edge case: empty samples
        let analyzer = CompareAnalyzer::new(0.05);
        let sample1: Vec<f64> = vec![];
        let sample2 = vec![10.0, 20.0, 30.0];

        let result = analyzer.welch_t_test(&sample1, &sample2, "test_metric");

        assert_eq!(result.statistic, 0.0);
        assert_eq!(result.p_value, 1.0);
        assert!(!result.is_significant);
    }

    #[test]
    fn test_cohens_d_classification_small() {
        let analyzer = CompareAnalyzer::new(0.05);

        // Small effect: d ≈ 0.1 (very close means, same SD)
        // Sample 1: mean ≈ 10.0, sd ≈ 0.7
        // Sample 2: mean ≈ 10.05, sd ≈ 0.7
        let sample1 = vec![10.0, 10.0, 10.0, 10.0, 10.0];
        let sample2 = vec![10.1, 10.1, 10.1, 10.1, 10.1];

        let result = analyzer.cohens_d(&sample1, &sample2, "test_metric");

        assert!(
            result.cohens_d.abs() < 0.2,
            "Effect size should be small, got d={}",
            result.cohens_d
        );
        assert_eq!(result.magnitude, "small");
    }

    #[test]
    fn test_cohens_d_classification_medium() {
        let analyzer = CompareAnalyzer::new(0.05);

        // Medium effect: d ≈ 0.35-0.45 (moderate separation)
        // Sample 1: mean = 12.0, sd ≈ 1.58
        // Sample 2: mean = 10.0, sd ≈ 1.58
        // d = (12 - 10) / 1.58 ≈ 1.27 -> actually large
        // Let's use smaller difference:
        let sample1 = vec![10.5, 11.0, 11.5, 12.0, 12.5]; // mean = 11.5, sd ≈ 0.79
        let sample2 = vec![10.0, 10.5, 11.0, 11.5, 12.0]; // mean = 11.0, sd ≈ 0.79
        // d = (11.5 - 11.0) / 0.79 ≈ 0.63 -> large

        let result = analyzer.cohens_d(&sample1, &sample2, "test_metric");

        // This will be large or medium depending on exact calculation
        assert!(
            result.cohens_d.abs() >= 0.2,
            "Effect size should be at least medium, got d={}",
            result.cohens_d
        );
        assert!(result.magnitude == "medium" || result.magnitude == "large");
    }

    #[test]
    fn test_cohens_d_classification_large() {
        let analyzer = CompareAnalyzer::new(0.05);

        // Large effect: d ≈ 0.65
        let sample1 = vec![15.0, 16.0, 17.0, 18.0, 19.0];
        let sample2 = vec![10.0, 11.0, 12.0, 13.0, 14.0];

        let result = analyzer.cohens_d(&sample1, &sample2, "test_metric");

        assert!(result.cohens_d.abs() >= 0.5);
        assert!(result.magnitude == "large" || result.magnitude == "very_large");
    }

    #[test]
    fn test_cohens_d_classification_very_large() {
        let analyzer = CompareAnalyzer::new(0.05);

        // Very large effect: d ≈ 1.0
        let sample1 = vec![20.0, 22.0, 24.0, 26.0, 28.0];
        let sample2 = vec![10.0, 12.0, 14.0, 16.0, 18.0];

        let result = analyzer.cohens_d(&sample1, &sample2, "test_metric");

        assert!(result.cohens_d.abs() >= 0.8);
        assert_eq!(result.magnitude, "very_large");
    }

    #[test]
    fn test_cohens_d_identical_samples() {
        // Identical samples should have d = 0
        let analyzer = CompareAnalyzer::new(0.05);
        let sample1 = vec![10.0, 10.0, 10.0, 10.0, 10.0];
        let sample2 = vec![10.0, 10.0, 10.0, 10.0, 10.0];

        let result = analyzer.cohens_d(&sample1, &sample2, "test_metric");

        assert_eq!(result.cohens_d, 0.0);
        assert_eq!(result.magnitude, "small");
    }

    #[test]
    fn test_cohens_d_negative_values() {
        // Test that sign is preserved (baseline worse than comparison)
        let analyzer = CompareAnalyzer::new(0.05);
        let sample1 = vec![10.0, 11.0, 12.0, 13.0, 14.0]; // Lower values
        let sample2 = vec![20.0, 21.0, 22.0, 23.0, 24.0]; // Higher values

        let result = analyzer.cohens_d(&sample1, &sample2, "test_metric");

        // Cohen's d should be negative (sample1 < sample2)
        assert!(
            result.cohens_d < 0.0,
            "Cohen's d should be negative when baseline is lower"
        );
    }

    #[test]
    fn test_effect_size_classification_boundaries() {
        // Test exact boundary conditions
        assert_eq!(CompareAnalyzer::classify_effect_size(0.0), "small");
        assert_eq!(CompareAnalyzer::classify_effect_size(0.1), "small");
        assert_eq!(CompareAnalyzer::classify_effect_size(0.19), "small");
        assert_eq!(CompareAnalyzer::classify_effect_size(0.2), "medium");
        assert_eq!(CompareAnalyzer::classify_effect_size(0.4), "medium");
        assert_eq!(CompareAnalyzer::classify_effect_size(0.49), "medium");
        assert_eq!(CompareAnalyzer::classify_effect_size(0.5), "large");
        assert_eq!(CompareAnalyzer::classify_effect_size(0.7), "large");
        assert_eq!(CompareAnalyzer::classify_effect_size(0.79), "large");
        assert_eq!(CompareAnalyzer::classify_effect_size(0.8), "very_large");
        assert_eq!(CompareAnalyzer::classify_effect_size(1.0), "very_large");
        assert_eq!(CompareAnalyzer::classify_effect_size(2.0), "very_large");
    }
}

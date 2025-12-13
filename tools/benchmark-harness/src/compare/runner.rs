//! Compare mode runner - orchestrates multi-framework benchmarking

use crate::{
    compare::analyzer::CompareAnalyzer,
    error::{Error, Result},
    profile::{ProfileRunner, ProfileRunnerConfig},
    schema::{
        Configuration, FrameworkInfo, Metadata,
        compare::{CompareResult, CompareSummary},
        profile::ProfileResult,
        workload::WorkloadSuite,
    },
};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Configuration for compare mode execution
#[derive(Debug, Clone)]
pub struct CompareConfig {
    /// List of frameworks to compare (minimum 2 required)
    pub frameworks: Vec<String>,

    /// Workload suite to run for each framework
    pub workload_suite: String,

    /// Base port for server (will increment for each framework to avoid conflicts)
    pub port: u16,

    /// Number of warmup requests before actual benchmark
    pub warmup_requests: usize,

    /// Output directory for results
    pub output_dir: PathBuf,

    /// Statistical significance threshold (p-value)
    pub significance_threshold: f64,

    /// Duration in seconds for each workload
    pub duration_secs: u64,

    /// Concurrency level for load testing
    pub concurrency: usize,
}

impl Default for CompareConfig {
    fn default() -> Self {
        Self {
            frameworks: Vec::new(),
            workload_suite: "all".to_string(),
            port: 8100,
            warmup_requests: 100,
            output_dir: PathBuf::from("benchmark-results"),
            significance_threshold: 0.05,
            duration_secs: 30,
            concurrency: 100,
        }
    }
}

/// Compare mode orchestrator
#[derive(Debug)]
pub struct CompareRunner {
    config: CompareConfig,
    #[allow(dead_code)]
    suite: WorkloadSuite,
}

impl CompareRunner {
    /// Create a new CompareRunner with validation
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Less than 2 frameworks specified
    /// - Workload suite not found
    /// - Invalid configuration parameters
    pub fn new(config: CompareConfig) -> Result<Self> {
        if config.frameworks.len() < 2 {
            return Err(Error::InvalidInput(
                "Compare mode requires at least 2 frameworks".to_string(),
            ));
        }

        let suite = WorkloadSuite::by_name(&config.workload_suite)
            .ok_or_else(|| Error::WorkloadNotFound(config.workload_suite.clone()))?;

        if let Some(parent) = config.output_dir.parent()
            && !parent.exists()
        {
            std::fs::create_dir_all(parent)
                .map_err(|e| Error::InvalidInput(format!("Cannot create output directory: {}", e)))?;
        }

        Ok(Self { config, suite })
    }

    /// Run the comparison across all configured frameworks
    ///
    /// Executes frameworks sequentially to avoid resource contention,
    /// collecting ProfileResult for each framework.
    ///
    /// # Errors
    ///
    /// Returns error if any framework execution fails.
    ///
    /// # Returns
    ///
    /// Returns tuple of (CompareResult, profile_results) for markdown generation
    pub async fn run(self) -> Result<(CompareResult, Vec<(String, ProfileResult)>)> {
        let total = self.config.frameworks.len();

        println!("\n🔬 Compare Mode - Starting comparison of {} frameworks", total);
        println!("Workload suite: {}", self.config.workload_suite);
        println!("Duration: {}s per workload", self.config.duration_secs);
        println!("Concurrency: {}", self.config.concurrency);
        println!();

        println!("🔍 Validating {} frameworks...", total);
        for framework in &self.config.frameworks {
            self.detect_app_dir(framework)?;
        }
        println!("✓ All frameworks found\n");

        let mut profile_results = Vec::new();

        for (idx, framework) in self.config.frameworks.iter().enumerate() {
            let num = idx + 1;
            println!("{}", "━".repeat(60));
            println!("📊 Framework {}/{}: {}", num, total, framework);
            println!("{}", "━".repeat(60));

            match self.run_single_framework(framework, idx).await {
                Ok(result) => {
                    println!("✓ {} completed", framework);
                    println!("  Average RPS: {:.2}", result.summary.avg_requests_per_sec);
                    println!("  Total requests: {}", result.summary.total_requests);
                    println!("  Success rate: {:.2}%", result.summary.overall_success_rate * 100.0);
                    println!();

                    profile_results.push((framework.clone(), result));
                }
                Err(e) => {
                    eprintln!("✗ {} failed: {}", framework, e);
                    return Err(Error::FrameworkExecutionFailed {
                        framework: framework.clone(),
                        source: Box::new(e),
                    });
                }
            }
        }

        println!("🧮 Running statistical analysis...");
        let compare_result = self.create_basic_result(profile_results.clone())?;
        println!("✓ Analysis complete\n");

        println!("\n{}", "═".repeat(60));
        println!("✓ Comparison complete!");
        println!("Frameworks compared: {}", compare_result.frameworks.len());
        println!("{}", "═".repeat(60));

        Ok((compare_result, profile_results))
    }

    /// Run a single framework using ProfileRunner
    ///
    /// # Arguments
    ///
    /// * `framework` - Framework name (e.g., "spikard-python", "fastapi")
    /// * `index` - Framework index for port allocation
    ///
    /// # Errors
    ///
    /// Returns error if ProfileRunner execution fails
    async fn run_single_framework(&self, framework: &str, index: usize) -> Result<ProfileResult> {
        let app_dir = self.detect_app_dir(framework)?;

        let port = self.config.port + (index as u16 * 10);

        println!("App directory: {}", app_dir.display());
        println!("Port: {}", port);

        let profile_config = ProfileRunnerConfig {
            framework: framework.to_string(),
            app_dir,
            suite_name: self.config.workload_suite.clone(),
            duration_secs: self.config.duration_secs,
            concurrency: self.config.concurrency,
            warmup_secs: (self.config.warmup_requests / self.config.concurrency).max(5) as u64,
            profiler: None,
            baseline_path: None,
            variant: None,
        };

        let runner = ProfileRunner::new(profile_config)?;
        runner.run().await
    }

    /// Detect app directory for a given framework
    ///
    /// Looks for framework-specific directories in standard locations
    fn detect_app_dir(&self, framework: &str) -> Result<PathBuf> {
        let workspace_root = std::env::current_dir()
            .map_err(|e| Error::InvalidInput(format!("Cannot determine workspace root: {}", e)))?;

        let apps_dir = workspace_root
            .join("tools")
            .join("benchmark-harness")
            .join("apps")
            .join(framework);
        if apps_dir.exists() {
            return Ok(apps_dir);
        }

        let benchmark_dir = workspace_root.join("benchmarks").join(framework);
        if benchmark_dir.exists() {
            return Ok(benchmark_dir);
        }

        if framework.starts_with("spikard-") {
            let language = framework.strip_prefix("spikard-").unwrap_or("python");
            let e2e_dir = workspace_root.join("e2e").join(language);
            if e2e_dir.exists() {
                return Ok(e2e_dir);
            }
        }

        let examples_dir = workspace_root.join("examples").join(framework);
        if examples_dir.exists() {
            return Ok(examples_dir);
        }

        Err(Error::FrameworkNotFound(framework.to_string()))
    }

    /// Create CompareResult from ProfileResults with statistical analysis
    ///
    /// Phase 2 implementation: Performs statistical tests and effect size calculations
    fn create_basic_result(&self, profile_results: Vec<(String, ProfileResult)>) -> Result<CompareResult> {
        let metadata = if let Some((_, first_result)) = profile_results.first() {
            first_result.metadata.clone()
        } else {
            Metadata::collect()
        };

        let frameworks: Vec<FrameworkInfo> = profile_results
            .iter()
            .map(|(_, result)| result.framework.clone())
            .collect();

        let configuration = if let Some((_, first_result)) = profile_results.first() {
            first_result.configuration.clone()
        } else {
            Configuration {
                duration_secs: self.config.duration_secs,
                concurrency: self.config.concurrency,
                warmup_secs: 10,
                load_tool: "oha".to_string(),
            }
        };

        let analyzer = CompareAnalyzer::new(self.config.significance_threshold);

        let baseline = &profile_results[0];

        let mut statistical_comparisons = Vec::new();

        for (idx, (framework, result)) in profile_results.iter().enumerate() {
            if idx == 0 {
                continue;
            }

            let analysis = analyzer.compare_frameworks(&baseline.1, result);
            statistical_comparisons.push((framework.clone(), analysis));
        }

        let suites = Vec::new();

        let overall_winner = if let Some((fw, _analysis)) = statistical_comparisons
            .iter()
            .find(|(_, a)| a.overall_verdict == "significantly_better")
        {
            fw.clone()
        } else {
            profile_results
                .iter()
                .max_by(|(_, a), (_, b)| {
                    a.summary
                        .avg_requests_per_sec
                        .partial_cmp(&b.summary.avg_requests_per_sec)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|(name, _)| name.clone())
                .unwrap_or_else(|| baseline.0.clone())
        };

        let avg_rps_values: Vec<f64> = profile_results
            .iter()
            .map(|(_, r)| r.summary.avg_requests_per_sec)
            .collect();

        let min_rps = avg_rps_values
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .copied()
            .unwrap_or(1.0);

        let max_rps = avg_rps_values
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .copied()
            .unwrap_or(1.0);

        let avg_performance_gain = if min_rps > 0.0 { (max_rps / min_rps) - 1.0 } else { 0.0 };

        println!("\n📊 Statistical Analysis Summary");
        println!("Baseline: {}", baseline.0);
        for (framework, analysis) in &statistical_comparisons {
            println!("\n{} vs {}: {}", framework, baseline.0, analysis.overall_verdict);

            if let Some(rps_test) = analysis
                .statistical_tests
                .iter()
                .find(|t| t.metric == "requests_per_sec")
            {
                println!(
                    "  RPS: t={:.2}, p={:.4}, significant={}",
                    rps_test.statistic, rps_test.p_value, rps_test.is_significant
                );
            }

            if let Some(rps_effect) = analysis.effect_sizes.iter().find(|e| e.metric == "requests_per_sec") {
                println!("  Effect size: d={:.2} ({})", rps_effect.cohens_d, rps_effect.magnitude);
            }
        }

        let summary = CompareSummary {
            overall_winner,
            avg_performance_gain,
            workloads_won: std::collections::HashMap::new(),
            category_winners: std::collections::HashMap::new(),
        };

        Ok(CompareResult {
            metadata,
            frameworks,
            configuration,
            suites,
            summary,
        })
    }

    /// Generate markdown comparison report
    ///
    /// Creates a comprehensive markdown report with statistical analysis tables
    pub fn generate_markdown_report(
        result: &CompareResult,
        profile_results: &[(String, ProfileResult)],
        significance_threshold: f64,
        workload_suite: &str,
    ) -> Result<String> {
        let mut report = String::new();

        report.push_str("# Framework Comparison Report\n\n");
        report.push_str(&format!("**Baseline:** {}\n", result.frameworks[0].name));
        report.push_str(&format!("**Date:** {}\n", result.metadata.timestamp));
        report.push_str(&format!("**Suite:** {}\n\n", workload_suite));

        report.push_str("## Summary\n\n");
        report.push_str("| Framework | Runtime | Verdict | Overall |\n");
        report.push_str("|-----------|---------|---------|----------|\n");

        let profile_map: std::collections::HashMap<&str, &ProfileResult> = profile_results
            .iter()
            .map(|(name, result)| (name.as_str(), result))
            .collect();

        for (idx, fw_info) in result.frameworks.iter().enumerate() {
            let verdict = if idx == 0 {
                "baseline"
            } else if let Some(profile_result) = profile_map.get(fw_info.name.as_str()) {
                let baseline_rps = profile_results[0].1.summary.avg_requests_per_sec;
                let this_rps = profile_result.summary.avg_requests_per_sec;

                if this_rps > baseline_rps * 1.1 {
                    "significantly better"
                } else if this_rps < baseline_rps * 0.9 {
                    "significantly worse"
                } else {
                    "similar"
                }
            } else {
                "unknown"
            };

            let emoji = match verdict {
                "baseline" => "📊",
                "significantly better" => "🏆",
                "significantly worse" => "📉",
                "similar" => "≈",
                _ => "?",
            };

            report.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                fw_info.name, fw_info.runtime, verdict, emoji
            ));
        }

        report.push('\n');
        report.push_str(&format!("**Overall Winner:** {}\n\n", result.summary.overall_winner));

        report.push_str("## Performance Metrics\n\n");
        report.push_str("| Framework | Avg RPS | Avg Latency (ms) | Success Rate | Workloads |\n");
        report.push_str("|-----------|---------|------------------|--------------|----------|\n");

        for (name, profile_result) in profile_results {
            let avg_latency = if profile_result.summary.category_breakdown.is_empty() {
                0.0
            } else {
                profile_result
                    .summary
                    .category_breakdown
                    .iter()
                    .map(|c| c.avg_latency_ms)
                    .sum::<f64>()
                    / profile_result.summary.category_breakdown.len() as f64
            };

            report.push_str(&format!(
                "| {} | {:.2} | {:.2} | {:.2}% | {} |\n",
                name,
                profile_result.summary.avg_requests_per_sec,
                avg_latency,
                profile_result.summary.overall_success_rate * 100.0,
                profile_result.summary.total_workloads
            ));
        }

        report.push('\n');

        if profile_results.len() > 1 {
            report.push_str("## Statistical Analysis\n\n");

            let analyzer = CompareAnalyzer::new(significance_threshold);
            let baseline = &profile_results[0];

            for (idx, (fw_name, fw_result)) in profile_results.iter().enumerate() {
                if idx == 0 {
                    continue;
                }

                report.push_str(&format!("### {} vs {}\n\n", fw_name, baseline.0));

                let analysis = analyzer.compare_frameworks(&baseline.1, fw_result);

                report.push_str("| Metric | t-statistic | p-value | Significant | Effect Size |\n");
                report.push_str("|--------|-------------|---------|-------------|-------------|\n");

                for test in &analysis.statistical_tests {
                    let sig_marker = if test.is_significant { "✓" } else { "✗" };

                    let effect_size = analysis
                        .effect_sizes
                        .iter()
                        .find(|es| es.metric == test.metric)
                        .map(|es| format!("{:.2} ({})", es.cohens_d, es.magnitude))
                        .unwrap_or_else(|| "N/A".to_string());

                    report.push_str(&format!(
                        "| {} | {:.2} | {:.4} | {} | {} |\n",
                        test.metric, test.statistic, test.p_value, sig_marker, effect_size
                    ));
                }

                report.push('\n');
            }

            report.push_str("---\n");
            report.push_str("**Legend:** ✓ = statistically significant (p < ");
            report.push_str(&format!("{:.2}", significance_threshold));
            report.push_str("), ✗ = not significant\n");
        }

        Ok(report)
    }

    /// Save markdown report to file
    ///
    /// # Errors
    ///
    /// Returns error if file cannot be written
    pub fn save_markdown_report(
        result: &CompareResult,
        profile_results: &[(String, ProfileResult)],
        output_dir: &Path,
        significance_threshold: f64,
        workload_suite: &str,
    ) -> Result<PathBuf> {
        let report_content =
            Self::generate_markdown_report(result, profile_results, significance_threshold, workload_suite)?;
        let report_path = output_dir.join("compare_report.md");

        let mut file = File::create(&report_path)?;
        file.write_all(report_content.as_bytes())?;

        Ok(report_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_config_validation_min_frameworks() {
        let config = CompareConfig {
            frameworks: vec!["framework1".to_string()],
            ..Default::default()
        };

        let result = CompareRunner::new(config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("at least 2 frameworks"));
    }

    #[test]
    fn test_compare_config_validation_unknown_suite() {
        let config = CompareConfig {
            frameworks: vec!["framework1".to_string(), "framework2".to_string()],
            workload_suite: "non-existent-suite".to_string(),
            ..Default::default()
        };

        let result = CompareRunner::new(config);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("not found") || error_msg.contains("Unknown workload suite"));
    }

    #[test]
    fn test_compare_config_validation_valid() {
        let config = CompareConfig {
            frameworks: vec!["framework1".to_string(), "framework2".to_string()],
            workload_suite: "all".to_string(),
            ..Default::default()
        };

        let result = CompareRunner::new(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_port_allocation() {
        let config = CompareConfig {
            frameworks: vec!["fw1".to_string(), "fw2".to_string(), "fw3".to_string()],
            port: 8100,
            ..Default::default()
        };

        let runner = CompareRunner::new(config).expect("valid config");

        assert_eq!(runner.config.port, 8100);
        assert_eq!(runner.config.port + 10, 8110);
        assert_eq!(runner.config.port + 20, 8120);
    }
}

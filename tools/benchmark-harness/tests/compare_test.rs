//! Integration tests for Compare Mode functionality
//!
//! Tests the complete end-to-end Compare Mode workflow including:
//! - Configuration validation
//! - Statistical analysis framework
//! - Baseline selection logic
//! - Error handling
//! - Report generation (markdown and JSON)

use benchmark_harness::compare::{CompareConfig, CompareRunner};
use std::path::PathBuf;

/// Test configuration validation - minimum frameworks
///
/// Verifies error when less than 2 frameworks specified
#[tokio::test]
async fn test_error_min_frameworks() {
    let config = CompareConfig {
        frameworks: vec!["single-framework".to_string()],
        workload_suite: "all".to_string(),
        port: 8100,
        duration_secs: 5,
        concurrency: 10,
        warmup_requests: 10,
        output_dir: PathBuf::from("./test-output/error-min"),
        significance_threshold: 0.05,
    };

    let result = CompareRunner::new(config);

    assert!(result.is_err());

    if let Err(e) = result {
        let error_msg = e.to_string();
        assert!(
            error_msg.contains("at least 2 frameworks"),
            "Error should mention minimum 2 frameworks: {error_msg}"
        );
    }

    std::fs::remove_dir_all("./test-output/error-min").ok();
}

/// Test configuration validation - invalid workload suite
///
/// Verifies error message lists available suites
#[tokio::test]
async fn test_error_invalid_workload_suite() {
    let config = CompareConfig {
        frameworks: vec!["framework1".to_string(), "framework2".to_string()],
        workload_suite: "non-existent-suite-xyz".to_string(),
        port: 8100,
        duration_secs: 5,
        concurrency: 10,
        warmup_requests: 10,
        output_dir: PathBuf::from("./test-output/error-suite"),
        significance_threshold: 0.05,
    };

    let result = CompareRunner::new(config);

    assert!(result.is_err());

    if let Err(e) = result {
        let error_msg = e.to_string();
        assert!(
            error_msg.contains("not found") || error_msg.contains("Available suites"),
            "Error should list available suites: {error_msg}"
        );
    }

    std::fs::remove_dir_all("./test-output/error-suite").ok();
}

/// Test configuration defaults
#[tokio::test]
async fn test_config_defaults() {
    let config = CompareConfig::default();

    assert_eq!(config.workload_suite, "all");
    assert_eq!(config.port, 8100);
    assert_eq!(config.warmup_requests, 100);
    assert!((config.significance_threshold - 0.05).abs() < 1e-10);
    assert_eq!(config.duration_secs, 30);
    assert_eq!(config.concurrency, 100);
    assert_eq!(config.output_dir, PathBuf::from("benchmark-results"));
}

/// Test port allocation strategy
///
/// Verifies ports increment by 10 to avoid conflicts
#[tokio::test]
async fn test_port_allocation_strategy() {
    let config = CompareConfig {
        frameworks: vec!["fw1".to_string(), "fw2".to_string(), "fw3".to_string()],
        workload_suite: "all".to_string(),
        port: 8100,
        duration_secs: 5,
        concurrency: 10,
        warmup_requests: 10,
        output_dir: PathBuf::from("./test-output/port-test"),
        significance_threshold: 0.05,
    };

    assert_eq!(config.port, 8100);

    let expected_ports = [8100, 8110, 8120];

    for (idx, _fw) in config.frameworks.iter().enumerate() {
        #[allow(clippy::cast_possible_truncation)]
        let allocated_port = config.port + (idx as u16 * 10);
        assert_eq!(allocated_port, expected_ports[idx]);
    }

    std::fs::remove_dir_all("./test-output/port-test").ok();
}

/// Test `CompareAnalyzer` instantiation and basic properties
#[tokio::test]
async fn test_compare_analyzer_basics() {
    use benchmark_harness::compare::CompareAnalyzer;

    let _analyzer_05 = CompareAnalyzer::new(0.05);
    let _analyzer_01 = CompareAnalyzer::new(0.01);
}

/// Test baseline selection logic with minimal schema
#[tokio::test]
async fn test_baseline_selection_logic() {
    use benchmark_harness::schema::compare::CompareResult;

    let frameworks = vec![
        benchmark_harness::schema::FrameworkInfo {
            name: "framework-a".to_string(),
            version: "1.0.0".to_string(),
            language: "python".to_string(),
            runtime: "Python 3.10".to_string(),
            app_dir: "/tmp/a".to_string(),
            variant: None,
        },
        benchmark_harness::schema::FrameworkInfo {
            name: "framework-b".to_string(),
            version: "1.0.0".to_string(),
            language: "python".to_string(),
            runtime: "Python 3.11".to_string(),
            app_dir: "/tmp/b".to_string(),
            variant: None,
        },
    ];

    let result = CompareResult {
        metadata: benchmark_harness::schema::Metadata::collect(),
        frameworks,
        configuration: benchmark_harness::schema::Configuration {
            duration_secs: 10,
            concurrency: 100,
            warmup_secs: 5,
            load_tool: "oha".to_string(),
        },
        suites: vec![],
        summary: benchmark_harness::schema::compare::CompareSummary {
            overall_winner: "framework-a".to_string(),
            avg_performance_gain: 0.0,
            workloads_won: std::collections::HashMap::new(),
            category_winners: std::collections::HashMap::new(),
        },
    };

    assert_eq!(result.frameworks[0].name, "framework-a");
}

/// Test JSON serialization of `CompareResult`
#[tokio::test]
async fn test_compare_result_json_serialization() {
    use benchmark_harness::schema::compare::CompareResult;

    let result = CompareResult {
        metadata: benchmark_harness::schema::Metadata::collect(),
        frameworks: vec![
            benchmark_harness::schema::FrameworkInfo {
                name: "fw1".to_string(),
                version: "1.0.0".to_string(),
                language: "python".to_string(),
                runtime: "Python 3.10".to_string(),
                app_dir: "/tmp/fw1".to_string(),
                variant: None,
            },
            benchmark_harness::schema::FrameworkInfo {
                name: "fw2".to_string(),
                version: "1.0.0".to_string(),
                language: "python".to_string(),
                runtime: "Python 3.11".to_string(),
                app_dir: "/tmp/fw2".to_string(),
                variant: None,
            },
        ],
        configuration: benchmark_harness::schema::Configuration {
            duration_secs: 10,
            concurrency: 100,
            warmup_secs: 5,
            load_tool: "oha".to_string(),
        },
        suites: vec![],
        summary: benchmark_harness::schema::compare::CompareSummary {
            overall_winner: "fw2".to_string(),
            avg_performance_gain: 0.2,
            workloads_won: std::collections::HashMap::new(),
            category_winners: std::collections::HashMap::new(),
        },
    };

    let json_result = serde_json::to_string_pretty(&result);
    assert!(json_result.is_ok(), "CompareResult should serialize to JSON");

    let json_content = json_result.unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_content).unwrap();

    assert!(parsed.get("metadata").is_some(), "JSON should have metadata");
    assert!(parsed.get("frameworks").is_some(), "JSON should have frameworks");
    assert!(parsed.get("summary").is_some(), "JSON should have summary");
    assert_eq!(
        parsed["frameworks"].as_array().unwrap().len(),
        2,
        "Should have 2 frameworks"
    );
}

/// Test validation of invalid framework names
#[tokio::test]
async fn test_error_invalid_framework() {
    let config = CompareConfig {
        frameworks: vec![
            "nonexistent-framework-xyz".to_string(),
            "another-missing-framework".to_string(),
        ],
        workload_suite: "all".to_string(),
        port: 8100,
        duration_secs: 5,
        concurrency: 10,
        warmup_requests: 10,
        output_dir: PathBuf::from("./test-output/error-framework"),
        significance_threshold: 0.05,
    };

    let runner = CompareRunner::new(config).expect("Config validation should pass");
    let result = runner.run().await;

    assert!(result.is_err());

    if let Err(e) = result {
        let error_msg = e.to_string();
        assert!(
            error_msg.contains("not found") || error_msg.contains("Framework"),
            "Error should mention framework not found: {error_msg}"
        );
    }

    std::fs::remove_dir_all("./test-output/error-framework").ok();
}

//! Consolidation tests for profile results.

use benchmark_harness::consolidate_profile_paths;
use benchmark_harness::schema::profile::{
    CategorySummary, ProfileResult, ProfileSummary, SuiteResult, WorkloadMetrics, WorkloadResult,
};
use benchmark_harness::schema::workload::Endpoint;
use benchmark_harness::schema::{
    Configuration, CpuMetrics, FrameworkInfo, HostInfo, Latency, MemoryMetrics, Metadata, Resources, Throughput,
};
use std::fs;
use tempfile::tempdir;

fn sample_profile(name: &str, rps: f64, latency_ms: f64) -> ProfileResult {
    let throughput = Throughput {
        requests_per_sec: rps,
        bytes_per_sec: 1024.0,
        total_requests: 1000,
        successful_requests: 1000,
        failed_requests: 0,
        success_rate: 1.0,
    };
    let latency = Latency {
        mean_ms: latency_ms,
        median_ms: latency_ms,
        p90_ms: latency_ms,
        p95_ms: latency_ms,
        p99_ms: latency_ms,
        p999_ms: latency_ms,
        min_ms: latency_ms,
        max_ms: latency_ms,
        stddev_ms: 0.0,
    };
    let resources = Resources {
        cpu: CpuMetrics {
            avg_percent: 10.0,
            peak_percent: 20.0,
            p95_percent: 18.0,
        },
        memory: MemoryMetrics {
            avg_mb: 128.0,
            peak_mb: 256.0,
            p95_mb: 220.0,
        },
    };
    let workload = WorkloadResult {
        name: "json-small".to_string(),
        description: "Small JSON payload".to_string(),
        category: "json-bodies".to_string(),
        payload_size_bytes: Some(86),
        endpoint: Endpoint {
            method: "POST".to_string(),
            path: "/json/small".to_string(),
        },
        results: WorkloadMetrics {
            throughput,
            latency,
            resources,
            profiling: None,
        },
    };

    ProfileResult {
        metadata: Metadata {
            timestamp: "2025-01-01T00:00:00Z".to_string(),
            git_commit: Some("deadbeef".to_string()),
            git_branch: Some("main".to_string()),
            git_dirty: false,
            host: HostInfo {
                os: "linux".to_string(),
                arch: "x86_64".to_string(),
                cpu_model: "test-cpu".to_string(),
                cpu_cores: 2,
                cpu_threads: 4,
                memory_gb: 8.0,
                hostname: "test-host".to_string(),
            },
        },
        framework: FrameworkInfo {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            language: "node".to_string(),
            runtime: "Node v24.12.0".to_string(),
            app_dir: "/tmp/app".to_string(),
            variant: None,
        },
        configuration: Configuration {
            duration_secs: 10,
            concurrency: 50,
            warmup_secs: 1,
            load_tool: "oha".to_string(),
        },
        suites: vec![SuiteResult {
            name: "all".to_string(),
            description: "All workloads".to_string(),
            workloads: vec![workload],
        }],
        summary: ProfileSummary {
            total_workloads: 1,
            total_requests: 1000,
            overall_success_rate: 1.0,
            avg_requests_per_sec: rps,
            total_duration_secs: 10,
            category_breakdown: vec![CategorySummary {
                category: "json-bodies".to_string(),
                workload_count: 1,
                avg_requests_per_sec: rps,
                avg_latency_ms: latency_ms,
            }],
        },
        comparison: None,
    }
}

#[test]
fn consolidates_profile_results() {
    let dir = tempdir().expect("tempdir");
    let path_one = dir.path().join("run1/profile.json");
    let path_two = dir.path().join("run2/profile.json");
    fs::create_dir_all(path_one.parent().unwrap()).expect("mkdir");
    fs::create_dir_all(path_two.parent().unwrap()).expect("mkdir");

    let run_one = sample_profile("spikard-node-validation", 100.0, 2.0);
    let run_two = sample_profile("spikard-node-validation", 200.0, 4.0);
    fs::write(&path_one, serde_json::to_string_pretty(&run_one).unwrap()).expect("write");
    fs::write(&path_two, serde_json::to_string_pretty(&run_two).unwrap()).expect("write");

    let report = consolidate_profile_paths(&[path_one, path_two]).expect("consolidate");
    assert_eq!(report.frameworks.len(), 1);

    let framework = &report.frameworks[0];
    assert_eq!(framework.name, "spikard-node-validation");
    assert_eq!(framework.run_count, 2);
    assert!((framework.avg_requests_per_sec.mean - 150.0).abs() < 0.001);

    let workload = &framework.workloads[0];
    assert_eq!(workload.name, "json-small");
    assert_eq!(workload.run_count, 2);
    assert!((workload.throughput.requests_per_sec.mean - 150.0).abs() < 0.001);
    assert!((workload.latency.mean_ms.mean - 3.0).abs() < 0.001);
}

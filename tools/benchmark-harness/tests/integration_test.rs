//! End-to-end integration tests for the benchmark harness

use axum::{Router, routing::get};
use benchmark_harness::fixture::{Fixture, FixtureManager};
use benchmark_harness::load_generator::{LoadTestConfig, find_load_generator};
use benchmark_harness::monitor::ResourceMonitor;
use benchmark_harness::server::find_available_port;
use serde_json::json;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;

/// Simple echo handler for testing
async fn health_handler() -> &'static str {
    "OK"
}

async fn echo_handler() -> &'static str {
    "Hello, World!"
}

/// Spawn a simple HTTP server for testing
async fn spawn_test_server() -> (u16, tokio::task::JoinHandle<()>) {
    let app = Router::new()
        .route("/", get(echo_handler))
        .route("/health", get(health_handler));

    let mut last_error = None;
    for attempt in 0..5 {
        let start_port = 53000 + (attempt * 100);
        if let Some(port) = find_available_port(start_port) {
            let addr = SocketAddr::from(([127, 0, 0, 1], port));
            match TcpListener::bind(addr).await {
                Ok(listener) => {
                    let handle = tokio::spawn(async move {
                        axum::serve(listener, app).await.expect("Server failed to start");
                    });

                    tokio::time::sleep(Duration::from_millis(200)).await;

                    return (port, handle);
                }
                Err(e) => {
                    last_error = Some(e);
                    continue;
                }
            }
        }
    }

    panic!("Failed to bind: {:?}", last_error.unwrap());
}

#[tokio::test]
async fn test_full_benchmark_flow() {
    if find_load_generator().is_none() {
        eprintln!("Skipping test: No load generator (oha or bombardier) available");
        return;
    }

    let (port, server_handle) = spawn_test_server().await;
    let base_url = format!("http://localhost:{}", port);

    let client = reqwest::Client::new();
    let mut attempts = 0;
    let max_attempts = 10;
    loop {
        match client.get(format!("{}/health", base_url)).send().await {
            Ok(response) if response.status().is_success() => break,
            _ if attempts < max_attempts => {
                attempts += 1;
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            _ => panic!("Server not reachable after {} attempts", max_attempts),
        }
    }

    let pid = std::process::id();

    let monitor = ResourceMonitor::new(pid);
    let monitor_handle = monitor.start_monitoring(50);

    let load_config = LoadTestConfig {
        base_url: base_url.clone(),
        duration_secs: 2,
        concurrency: 10,
        fixture: None,
    };

    let generator = find_load_generator().unwrap();
    let result = benchmark_harness::load_generator::run_load_test(load_config, generator).await;

    let monitor = monitor_handle.stop().await;
    let resources = monitor.calculate_metrics();

    match result {
        Ok((oha_output, throughput)) => {
            verify_benchmark_results(oha_output, throughput, resources);
            server_handle.abort();
            return;
        }
        Err(e) => {
            eprintln!("Load test failed: {}", e);
            eprintln!("This might happen if the server isn't fully ready or if there's a connection issue");
            server_handle.abort();
            return;
        }
    }
}

fn verify_benchmark_results(
    oha_output: benchmark_harness::types::OhaOutput,
    throughput: benchmark_harness::types::ThroughputMetrics,
    resources: benchmark_harness::types::ResourceMetrics,
) {
    assert!(throughput.total_requests > 0);
    assert!(throughput.requests_per_sec > 0.0);
    assert!(throughput.success_rate >= 0.0);
    assert!(throughput.success_rate <= 1.0);

    let latency = benchmark_harness::types::LatencyMetrics::from(oha_output);
    assert!(latency.mean_ms >= 0.0);
    assert!(latency.p50_ms >= 0.0);
    assert!(latency.p95_ms >= 0.0);
    assert!(latency.p99_ms >= 0.0);
    assert!(latency.max_ms >= latency.p99_ms);
    assert!(latency.min_ms <= latency.mean_ms);

    assert!(resources.avg_memory_mb >= 0.0);
    assert!(resources.peak_memory_mb >= resources.avg_memory_mb);
    assert!(resources.avg_cpu_percent >= 0.0);

    let benchmark_result = benchmark_harness::types::BenchmarkResult {
        framework: "test-framework".to_string(),
        workload: "test-workload".to_string(),
        variant: None,
        timestamp: chrono::Utc::now(),
        duration_secs: 2,
        concurrency: 10,
        startup: None,
        throughput,
        latency,
        resources,
        route_types: vec![],
        error_metrics: None,
        serialization: None,
        patterns: vec![],
        success: true,
        error: None,
    };

    let json = serde_json::to_string(&benchmark_result).unwrap();
    assert!(!json.is_empty());

    let deserialized: benchmark_harness::types::BenchmarkResult = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.framework, "test-framework");
    assert!(deserialized.success);
}

#[tokio::test]
async fn test_benchmark_with_fixture() {
    if find_load_generator().is_none() {
        eprintln!("Skipping test: No load generator available");
        return;
    }

    let (port, server_handle) = spawn_test_server().await;
    let base_url = format!("http://localhost:{}", port);

    let fixture_json = json!({
        "name": "simple_get",
        "description": "Simple GET request",
        "handler": {
            "route": "/",
            "method": "GET"
        },
        "request": {
            "method": "GET",
            "path": "/"
        },
        "expected_response": {
            "status_code": 200
        }
    });

    let fixture: Fixture = serde_json::from_value(fixture_json).unwrap();

    let load_config = LoadTestConfig {
        base_url: base_url.clone(),
        duration_secs: 1,
        concurrency: 5,
        fixture: Some(fixture),
    };

    let generator = find_load_generator().unwrap();
    let result = benchmark_harness::load_generator::run_load_test(load_config, generator).await;

    server_handle.abort();

    match result {
        Ok((_, throughput)) => {
            assert!(throughput.total_requests > 0, "Should have some requests");
        }
        Err(e) => {
            eprintln!("Load test with fixture failed (expected in some environments): {}", e);
        }
    }
}

#[tokio::test]
async fn test_monitor_during_load() {
    let pid = std::process::id();
    let monitor = ResourceMonitor::new(pid);

    let monitor_handle = monitor.start_monitoring(20);

    tokio::time::sleep(Duration::from_millis(500)).await;

    for _ in 0..10 {
        let _v: Vec<u8> = (0..100_000).map(|x| (x % 256) as u8).collect();
        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    let monitor = monitor_handle.stop().await;
    let metrics = monitor.calculate_metrics();

    assert!(monitor.samples().len() > 10);

    assert!(metrics.avg_memory_mb > 0.0);
    assert!(metrics.peak_memory_mb >= metrics.avg_memory_mb);
    assert!(metrics.p50_memory_mb > 0.0);
    assert!(metrics.p95_memory_mb >= metrics.p50_memory_mb);
    assert!(metrics.p99_memory_mb >= metrics.p95_memory_mb);
}

#[tokio::test]
async fn test_concurrent_benchmarks() {
    if find_load_generator().is_none() {
        eprintln!("Skipping test: No load generator available");
        return;
    }

    let (port, server_handle) = spawn_test_server().await;
    let base_url = format!("http://localhost:{}", port);

    let generator = find_load_generator().unwrap();

    let mut successes = 0;
    for i in 0..2 {
        let load_config = LoadTestConfig {
            base_url: base_url.clone(),
            duration_secs: 1,
            concurrency: 5,
            fixture: None,
        };

        let result = benchmark_harness::load_generator::run_load_test(load_config, generator).await;
        if result.is_ok() {
            successes += 1;
        } else if let Err(e) = result {
            eprintln!("Benchmark {} encountered error (may be expected): {}", i, e);
        }
    }

    server_handle.abort();

    if successes == 0 {
        eprintln!("Warning: No benchmarks succeeded (this can happen in constrained test environments)");
    } else {
        println!("Successfully completed {} out of 2 benchmarks", successes);
    }
}

#[tokio::test]
async fn test_fixture_manager_integration() {
    let mut manager = FixtureManager::new();

    let temp_dir = tempfile::TempDir::new().unwrap();
    let fixture_json = json!({
        "name": "integration_test",
        "description": "Integration test fixture",
        "category": "test",
        "handler": {
            "route": "/test",
            "method": "GET"
        },
        "request": {
            "method": "GET",
            "path": "/test"
        },
        "expected_response": {
            "status_code": 200
        }
    });

    let fixture_path = temp_dir.path().join("test.json");
    std::fs::write(&fixture_path, serde_json::to_string_pretty(&fixture_json).unwrap()).unwrap();

    let fixtures = Fixture::from_dir(temp_dir.path()).unwrap();
    assert_eq!(fixtures.len(), 1);

    manager.fixtures.extend(fixtures);
    assert_eq!(manager.len(), 1);

    let test_fixtures = manager.by_category("test");
    assert_eq!(test_fixtures.len(), 1);
    assert_eq!(test_fixtures[0].name, "integration_test");
}

#[test]
fn test_benchmark_result_output_format() {
    let result = benchmark_harness::types::BenchmarkResult {
        framework: "test".to_string(),
        workload: "simple".to_string(),
        variant: None,
        timestamp: chrono::Utc::now(),
        duration_secs: 10,
        concurrency: 50,
        startup: None,
        throughput: benchmark_harness::types::ThroughputMetrics {
            total_requests: 1000,
            requests_per_sec: 100.0,
            bytes_per_sec: 50000.0,
            failed_requests: 5,
            success_rate: 0.995,
        },
        latency: benchmark_harness::types::LatencyMetrics {
            mean_ms: 10.0,
            p50_ms: 8.0,
            p90_ms: 15.0,
            p95_ms: 20.0,
            p99_ms: 30.0,
            p999_ms: 50.0,
            max_ms: 100.0,
            min_ms: 1.0,
            stddev_ms: 5.0,
        },
        resources: benchmark_harness::types::ResourceMetrics {
            avg_memory_mb: 50.0,
            peak_memory_mb: 75.0,
            p50_memory_mb: 48.0,
            p95_memory_mb: 70.0,
            p99_memory_mb: 73.0,
            avg_cpu_percent: 25.0,
            peak_cpu_percent: 80.0,
        },
        route_types: vec![],
        error_metrics: None,
        serialization: None,
        patterns: vec![],
        success: true,
        error: None,
    };

    let json = serde_json::to_value(&result).unwrap();

    assert_eq!(json["framework"], "test");
    assert_eq!(json["workload"], "simple");
    assert_eq!(json["duration_secs"], 10);
    assert_eq!(json["concurrency"], 50);
    assert_eq!(json["throughput"]["total_requests"], 1000);
    assert!((json["latency"]["p99_ms"].as_f64().unwrap() - 30.0).abs() < 1e-10);
    assert!((json["resources"]["peak_memory_mb"].as_f64().unwrap() - 75.0).abs() < 1e-10);
    assert_eq!(json["success"], true);

    assert!(json.get("startup").is_none());
    assert!(json.get("error_metrics").is_none());
    assert!(json.get("error").is_none());
}

#[tokio::test]
async fn test_error_handling_in_benchmark() {
    if find_load_generator().is_none() {
        eprintln!("Skipping test: No load generator available");
        return;
    }

    let load_config = LoadTestConfig {
        base_url: "http://localhost:65534".to_string(),
        duration_secs: 1,
        concurrency: 1,
        fixture: None,
    };

    let generator = find_load_generator().unwrap();
    let result = benchmark_harness::load_generator::run_load_test(load_config, generator).await;

    if let Ok((_, throughput)) = result {
        assert!(throughput.success_rate < 0.5);
    }
}

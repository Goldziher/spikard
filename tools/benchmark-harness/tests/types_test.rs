//! Unit tests for benchmark types

use benchmark_harness::types::*;
use chrono::Utc;
use serde_json::json;

#[test]
fn test_route_type_display() {
    assert_eq!(RouteType::GetSimple.to_string(), "GET (simple)");
    assert_eq!(RouteType::GetPathParams.to_string(), "GET (path params)");
    assert_eq!(RouteType::GetQueryParams.to_string(), "GET (query params)");
    assert_eq!(RouteType::GetBoth.to_string(), "GET (path + query)");
    assert_eq!(RouteType::PostJsonSimple.to_string(), "POST (simple JSON)");
    assert_eq!(RouteType::PostJsonNested.to_string(), "POST (nested JSON)");
    assert_eq!(RouteType::PostJsonLarge.to_string(), "POST (large payload)");
    assert_eq!(RouteType::PostValidated.to_string(), "POST (validated)");
    assert_eq!(RouteType::PostMultipart.to_string(), "POST (multipart)");
    assert_eq!(RouteType::PutJson.to_string(), "PUT (JSON)");
    assert_eq!(RouteType::PatchJson.to_string(), "PATCH (JSON)");
    assert_eq!(RouteType::Delete.to_string(), "DELETE");
    assert_eq!(RouteType::Other.to_string(), "Other");
}

#[test]
fn test_route_type_serialization() {
    let route_type = RouteType::GetSimple;
    let json = serde_json::to_string(&route_type).unwrap();
    assert_eq!(json, "\"get_simple\"");

    let deserialized: RouteType = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, RouteType::GetSimple);
}

#[test]
fn test_latency_metrics_from_oha_output() {
    let oha_output = OhaOutput {
        summary: OhaSummary {
            success_rate: Some(1.0),
            total: Some(10.0),    
            slowest: Some(0.5),   
            fastest: Some(0.001), 
            average: Some(0.05),  
            requests_per_sec: Some(100.0),
            total_data: Some(10000.0),
            size_per_request: Some(100.0),
            size_per_sec: Some(10000.0),
        },
        latency_percentiles: LatencyPercentiles {
            p10: Some(0.01),
            p25: Some(0.02),
            p50: Some(0.03),
            p75: Some(0.04),
            p90: Some(0.08),
            p95: Some(0.1),
            p99: Some(0.2),
            p99_9: Some(0.4),
            p99_99: Some(0.45),
        },
    };

    let latency = LatencyMetrics::from(oha_output);

    assert_eq!(latency.mean_ms, 50.0);
    assert_eq!(latency.p50_ms, 30.0);
    assert_eq!(latency.p90_ms, 80.0);
    assert_eq!(latency.p95_ms, 100.0);
    assert_eq!(latency.p99_ms, 200.0);
    assert_eq!(latency.p999_ms, 400.0);
    assert_eq!(latency.max_ms, 500.0);
    assert_eq!(latency.min_ms, 1.0);
}

#[test]
fn test_latency_metrics_from_oha_output_with_none_values() {
    let oha_output = OhaOutput {
        summary: OhaSummary {
            success_rate: Some(0.5),
            total: Some(5.0),
            slowest: None,
            fastest: None,
            average: None,
            requests_per_sec: Some(50.0),
            total_data: Some(5000.0),
            size_per_request: None,
            size_per_sec: Some(5000.0),
        },
        latency_percentiles: LatencyPercentiles {
            p10: None,
            p25: None,
            p50: None,
            p75: None,
            p90: None,
            p95: None,
            p99: None,
            p99_9: None,
            p99_99: None,
        },
    };

    let latency = LatencyMetrics::from(oha_output);

    assert_eq!(latency.mean_ms, 0.0);
    assert_eq!(latency.p50_ms, 0.0);
    assert_eq!(latency.p90_ms, 0.0);
    assert_eq!(latency.p95_ms, 0.0);
    assert_eq!(latency.p99_ms, 0.0);
    assert_eq!(latency.p999_ms, 0.0);
    assert_eq!(latency.max_ms, 0.0);
    assert_eq!(latency.min_ms, 0.0);
}

#[test]
fn test_benchmark_result_serialization_full() {
    let result = BenchmarkResult {
        framework: "test-framework".to_string(),
        workload: "test-workload".to_string(),
        variant: None,
        timestamp: Utc::now(),
        duration_secs: 30,
        concurrency: 100,
        startup: Some(StartupMetrics {
            process_spawn_ms: 10.0,
            time_to_first_response_ms: 50.0,
            initialization_memory_mb: 25.0,
            total_startup_ms: 60.0,
        }),
        throughput: ThroughputMetrics {
            total_requests: 3000,
            requests_per_sec: 100.0,
            bytes_per_sec: 10000.0,
            failed_requests: 5,
            success_rate: 0.998,
        },
        latency: LatencyMetrics {
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
        resources: ResourceMetrics {
            avg_memory_mb: 50.0,
            peak_memory_mb: 75.0,
            p50_memory_mb: 48.0,
            p95_memory_mb: 70.0,
            p99_memory_mb: 73.0,
            avg_cpu_percent: 25.0,
            peak_cpu_percent: 80.0,
        },
        route_types: vec![RouteTypeMetrics {
            route_type: RouteType::GetSimple,
            sample_count: 1000,
            throughput_rps: 100.0,
            latency: LatencyMetrics {
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
            success_rate: 1.0,
            avg_memory_delta_mb: 0.5,
        }],
        error_metrics: Some(ErrorMetrics {
            validation_error_p99_ms: 5.0,
            not_found_p99_ms: 3.0,
            server_error_p99_ms: 10.0,
            error_throughput_rps: 10.0,
            error_memory_impact_mb: 0.1,
            total_errors: 5,
            error_rate: 0.002,
        }),
        serialization: Some(SerializationMetrics {
            json_parse_overhead_ms: 0.5,
            json_serialize_overhead_ms: 0.3,
            validation_overhead_ms: 0.2,
            total_overhead_pct: 10.0,
            sample_count: 1000,
        }),
        patterns: vec![],
        success: true,
        error: None,
    };

    let json = serde_json::to_string(&result).unwrap();
    assert!(!json.is_empty());

    let deserialized: BenchmarkResult = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.framework, result.framework);
    assert_eq!(deserialized.workload, result.workload);
    assert_eq!(deserialized.duration_secs, result.duration_secs);
    assert_eq!(deserialized.concurrency, result.concurrency);
    assert!(deserialized.success);
}

#[test]
fn test_benchmark_result_serialization_minimal() {
    let result = BenchmarkResult {
        framework: "test-framework".to_string(),
        workload: "test-workload".to_string(),
        variant: None,
        timestamp: Utc::now(),
        duration_secs: 30,
        concurrency: 100,
        startup: None,
        throughput: ThroughputMetrics {
            total_requests: 0,
            requests_per_sec: 0.0,
            bytes_per_sec: 0.0,
            failed_requests: 0,
            success_rate: 0.0,
        },
        latency: LatencyMetrics {
            mean_ms: 0.0,
            p50_ms: 0.0,
            p90_ms: 0.0,
            p95_ms: 0.0,
            p99_ms: 0.0,
            p999_ms: 0.0,
            max_ms: 0.0,
            min_ms: 0.0,
            stddev_ms: 0.0,
        },
        resources: ResourceMetrics {
            avg_memory_mb: 0.0,
            peak_memory_mb: 0.0,
            p50_memory_mb: 0.0,
            p95_memory_mb: 0.0,
            p99_memory_mb: 0.0,
            avg_cpu_percent: 0.0,
            peak_cpu_percent: 0.0,
        },
        route_types: vec![],
        error_metrics: None,
        serialization: None,
        patterns: vec![],
        success: false,
        error: Some("Test error".to_string()),
    };

    let json = serde_json::to_string(&result).unwrap();
    assert!(!json.is_empty());

    let deserialized: BenchmarkResult = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.framework, result.framework);
    assert!(!deserialized.success);
    assert_eq!(deserialized.error, Some("Test error".to_string()));
}

#[test]
fn test_duration_to_ms() {
    let duration = std::time::Duration::from_secs(1);
    assert_eq!(duration_to_ms(duration), 1000.0);

    let duration = std::time::Duration::from_millis(500);
    assert_eq!(duration_to_ms(duration), 500.0);

    let duration = std::time::Duration::from_micros(1500);
    assert_eq!(duration_to_ms(duration), 1.5);
}

#[test]
fn test_bytes_to_mb() {
    assert_eq!(bytes_to_mb(1024 * 1024), 1.0);
    assert_eq!(bytes_to_mb(1024 * 1024 * 10), 10.0);
    assert_eq!(bytes_to_mb(1024 * 512), 0.5);
    assert_eq!(bytes_to_mb(0), 0.0);
}

#[test]
fn test_oha_output_deserialization() {
    let json = json!({
        "summary": {
            "successRate": 0.99,
            "total": 10.0,
            "slowest": 0.5,
            "fastest": 0.001,
            "average": 0.05,
            "requestsPerSec": 100.0,
            "totalData": 10000.0,
            "sizePerRequest": 100.0,
            "sizePerSec": 10000.0
        },
        "latencyPercentiles": {
            "p10": 0.01,
            "p25": 0.02,
            "p50": 0.03,
            "p75": 0.04,
            "p90": 0.08,
            "p95": 0.1,
            "p99": 0.2,
            "p99.9": 0.4,
            "p99.99": 0.45
        }
    });

    let oha_output: OhaOutput = serde_json::from_value(json).unwrap();
    assert_eq!(oha_output.summary.success_rate, Some(0.99));
    assert_eq!(oha_output.summary.requests_per_sec, Some(100.0));
    assert_eq!(oha_output.latency_percentiles.p50, Some(0.03));
    assert_eq!(oha_output.latency_percentiles.p99, Some(0.2));
}

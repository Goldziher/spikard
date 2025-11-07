//! Benchmark runner - orchestrates the full benchmark process

use crate::error::{Error, Result};
use crate::fixture::Fixture;
use crate::load_generator::{LoadGeneratorType, LoadTestConfig, run_load_test};
use crate::monitor::ResourceMonitor;
use crate::server::{ServerConfig, find_available_port, start_server};
use crate::types::{BenchmarkResult, LatencyMetrics, RouteType, RouteTypeMetrics, StartupMetrics, ThroughputMetrics};
use chrono::Utc;
use std::path::PathBuf;
use std::time::Instant;
use tokio::time::{Duration, sleep};

/// Runner configuration
pub struct RunnerConfig {
    pub framework: String,
    pub app_dir: PathBuf,
    pub workload_name: String,
    pub duration_secs: u64,
    pub concurrency: usize,
    pub warmup_secs: u64,
}

/// Benchmark runner
pub struct BenchmarkRunner {
    config: RunnerConfig,
    load_generator: LoadGeneratorType,
}

impl BenchmarkRunner {
    /// Create a new benchmark runner
    pub fn new(config: RunnerConfig) -> Result<Self> {
        let load_generator = crate::load_generator::find_load_generator()
            .ok_or_else(|| Error::LoadGeneratorNotFound("oha or bombardier".to_string()))?;

        Ok(Self { config, load_generator })
    }

    /// Run a single benchmark
    pub async fn run(&self, fixture: Option<&Fixture>) -> Result<BenchmarkResult> {
        println!("Starting benchmark for {}", self.config.framework);

        // Find available port
        let port =
            find_available_port(8000).ok_or_else(|| Error::ServerStartFailed("No available ports".to_string()))?;

        println!("Starting server on port {}", port);

        // Start timing for startup metrics
        let spawn_start = Instant::now();

        // Start server
        let server_config = ServerConfig {
            framework: self.config.framework.clone(),
            port,
            app_dir: self.config.app_dir.clone(),
        };

        let server = start_server(server_config).await?;
        let pid = server.pid();
        let base_url = server.base_url.clone();

        // Measure startup time (spawn + health check)
        let total_startup_ms = spawn_start.elapsed().as_secs_f64() * 1000.0;

        println!("Server started with PID {} (startup: {:.2}ms)", pid, total_startup_ms);

        // Start monitoring immediately to capture initialization memory
        let init_monitor = ResourceMonitor::new(pid);
        let init_monitor_handle = init_monitor.start_monitoring(50); // Sample every 50ms

        // Give monitor a moment to capture initial samples
        sleep(Duration::from_millis(200)).await;

        // Stop initial monitoring to get baseline memory
        let init_monitor = init_monitor_handle.stop().await;
        let init_samples = init_monitor.samples();

        // Calculate initialization memory (average of first few samples)
        let initialization_memory_mb = if !init_samples.is_empty() {
            let sample_count = init_samples.len().min(5); // Use first 5 samples or all if fewer
            let total_memory: u64 = init_samples.iter().take(sample_count).map(|s| s.memory_bytes).sum();
            crate::types::bytes_to_mb(total_memory / sample_count as u64)
        } else {
            0.0
        };

        // Create startup metrics
        // Note: process_spawn_ms is approximated as a small fraction of total startup
        // In the future, we could instrument start_server to return separate timings
        let startup_metrics = StartupMetrics {
            process_spawn_ms: total_startup_ms * 0.1, // Approximate: 10% for spawn, 90% for health check
            time_to_first_response_ms: total_startup_ms * 0.9,
            initialization_memory_mb,
            total_startup_ms,
        };

        println!("Initialization memory: {:.2} MB", initialization_memory_mb);

        // Warm-up phase
        if self.config.warmup_secs > 0 {
            println!("Warming up for {} seconds...", self.config.warmup_secs);
            let warmup_config = LoadTestConfig {
                base_url: base_url.clone(),
                duration_secs: self.config.warmup_secs,
                concurrency: 10, // Low concurrency for warmup
                fixture: fixture.cloned(),
            };

            let _ = run_load_test(warmup_config, self.load_generator).await;
            sleep(Duration::from_secs(2)).await;
        }

        println!(
            "Running benchmark for {} seconds with concurrency {}...",
            self.config.duration_secs, self.config.concurrency
        );

        // Start monitoring
        let monitor = ResourceMonitor::new(pid);
        let monitor_handle = monitor.start_monitoring(100); // Sample every 100ms

        // Run load test
        let load_config = LoadTestConfig {
            base_url: base_url.clone(),
            duration_secs: self.config.duration_secs,
            concurrency: self.config.concurrency,
            fixture: fixture.cloned(),
        };

        let timestamp = Utc::now();

        let load_result = run_load_test(load_config, self.load_generator).await;

        // Stop monitoring
        let monitor = monitor_handle.stop().await;
        let resources = monitor.calculate_metrics();

        // Stop server
        server.kill()?;

        println!("Benchmark completed");

        // Build result
        match load_result {
            Ok((oha_output, throughput)) => {
                let latency = LatencyMetrics::from(oha_output);

                // Create route type metrics if we have fixture information
                let route_types = if let Some(fixture) = fixture {
                    let route_type = classify_route_type(fixture);
                    vec![RouteTypeMetrics {
                        route_type,
                        sample_count: throughput.total_requests,
                        throughput_rps: throughput.requests_per_sec,
                        latency: latency.clone(),
                        success_rate: throughput.success_rate,
                        avg_memory_delta_mb: 0.0, // TODO: Calculate memory delta per request
                                                   // This requires comparing memory samples before/during load test
                                                   // Could be: (peak_memory - initialization_memory) / total_requests
                    }]
                } else {
                    vec![]
                };

                Ok(BenchmarkResult {
                    framework: self.config.framework.clone(),
                    workload: self.config.workload_name.clone(),
                    timestamp,
                    duration_secs: self.config.duration_secs,
                    concurrency: self.config.concurrency,
                    startup: Some(startup_metrics),
                    throughput,
                    latency,
                    resources,
                    route_types,
                    // TODO: Error Metrics Collection
                    // To collect error metrics, we need to:
                    // 1. Run separate benchmark scenarios with error-inducing requests
                    // 2. Test validation errors (400) - send invalid payloads
                    // 3. Test not found errors (404) - send requests to non-existent routes
                    // 4. Test server errors (500) - trigger error conditions in handlers
                    // 5. Measure p99 latency for each error type
                    // 6. Track memory impact during error handling
                    // 7. Calculate error rate and throughput
                    // This should be done in a separate error_metrics benchmarking phase
                    error_metrics: None,
                    // TODO: Serialization Metrics Collection
                    // To collect serialization metrics, we need to:
                    // 1. Instrument the application with timing hooks OR
                    // 2. Use profiling tools to measure time spent in:
                    //    - JSON parsing (serde_json::from_str or msgspec.decode)
                    //    - JSON serialization (serde_json::to_string or msgspec.encode)
                    //    - Schema validation (pydantic validation, etc.)
                    // 3. Calculate average overhead per request
                    // 4. Express as percentage of total latency
                    // This likely requires framework-specific instrumentation
                    // or integration with tracing/profiling tools
                    serialization: None,
                    patterns: vec![], // Deprecated
                    success: true,
                    error: None,
                })
            }
            Err(e) => Ok(BenchmarkResult {
                framework: self.config.framework.clone(),
                workload: self.config.workload_name.clone(),
                timestamp,
                duration_secs: self.config.duration_secs,
                concurrency: self.config.concurrency,
                startup: Some(startup_metrics),
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
                resources,
                route_types: vec![],
                error_metrics: None,
                serialization: None,
                patterns: vec![],
                success: false,
                error: Some(e.to_string()),
            }),
        }
    }
}

/// Classify a fixture into a RouteType based on its characteristics
fn classify_route_type(fixture: &Fixture) -> RouteType {
    let method = fixture.handler.method.to_uppercase();
    let has_path_params = !fixture.handler.parameters.path.is_empty();
    let has_query_params = !fixture.handler.parameters.query.is_empty() || !fixture.request.query_params.is_empty();
    let body_size_bytes = fixture.request.body.as_ref().map(|b| b.to_string().len()).unwrap_or(0);

    match method.as_str() {
        "GET" => {
            match (has_path_params, has_query_params) {
                (true, true) => RouteType::GetBoth,
                (true, false) => RouteType::GetPathParams,
                (false, true) => RouteType::GetQueryParams,
                (false, false) => RouteType::GetSimple,
            }
        }
        "POST" => {
            // Check for multipart based on content-type header
            let is_multipart = fixture.request.headers.get("content-type")
                .or_else(|| fixture.request.headers.get("Content-Type"))
                .map(|ct| ct.contains("multipart"))
                .unwrap_or(false);

            if is_multipart {
                return RouteType::PostMultipart;
            }

            // Classify based on body characteristics
            if body_size_bytes > 10 * 1024 {
                // Large payload (>10KB)
                RouteType::PostJsonLarge
            } else if let Some(body) = &fixture.request.body {
                // Check for validation constraints
                // This is heuristic - we look for common validation keywords in the fixture
                let body_str = body.to_string();
                let has_validation_keywords = body_str.contains("validation")
                    || fixture.name.to_lowercase().contains("validation")
                    || fixture.name.to_lowercase().contains("validate");

                if has_validation_keywords {
                    RouteType::PostValidated
                } else if is_nested_json(body, 0) {
                    // Nested structure (3+ levels deep)
                    RouteType::PostJsonNested
                } else {
                    // Simple flat JSON
                    RouteType::PostJsonSimple
                }
            } else {
                RouteType::PostJsonSimple
            }
        }
        "PUT" => RouteType::PutJson,
        "PATCH" => RouteType::PatchJson,
        "DELETE" => RouteType::Delete,
        _ => RouteType::Other,
    }
}

/// Helper to detect nested JSON structures (3+ levels deep)
fn is_nested_json(value: &serde_json::Value, depth: usize) -> bool {
    if depth >= 3 {
        return true;
    }

    match value {
        serde_json::Value::Object(map) => {
            for v in map.values() {
                if is_nested_json(v, depth + 1) {
                    return true;
                }
            }
            false
        }
        serde_json::Value::Array(arr) => {
            arr.iter().any(|v| is_nested_json(v, depth + 1))
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixture::{ExpectedResponse, Handler, Parameters, Request};
    use std::collections::HashMap;

    fn create_test_fixture(method: &str, path_params: bool, query_params: bool, body: Option<serde_json::Value>) -> Fixture {
        let mut path = HashMap::new();
        if path_params {
            path.insert("id".to_string(), serde_json::json!("123"));
        }

        let mut query = HashMap::new();
        if query_params {
            query.insert("q".to_string(), serde_json::json!("test"));
        }

        Fixture {
            name: "test".to_string(),
            description: "test fixture".to_string(),
            category: None,
            handler: Handler {
                route: "/test".to_string(),
                method: method.to_string(),
                parameters: Parameters {
                    path,
                    query: query.clone(),
                    header: HashMap::new(),
                    cookie: HashMap::new(),
                },
            },
            request: Request {
                method: method.to_string(),
                path: "/test".to_string(),
                query_params: HashMap::new(),
                headers: HashMap::new(),
                cookies: HashMap::new(),
                body,
                body_raw: None,
            },
            expected_response: ExpectedResponse {
                status_code: 200,
                body: None,
                headers: HashMap::new(),
            },
        }
    }

    #[test]
    fn test_classify_get_simple() {
        let fixture = create_test_fixture("GET", false, false, None);
        assert_eq!(classify_route_type(&fixture), RouteType::GetSimple);
    }

    #[test]
    fn test_classify_get_path_params() {
        let fixture = create_test_fixture("GET", true, false, None);
        assert_eq!(classify_route_type(&fixture), RouteType::GetPathParams);
    }

    #[test]
    fn test_classify_get_query_params() {
        let fixture = create_test_fixture("GET", false, true, None);
        assert_eq!(classify_route_type(&fixture), RouteType::GetQueryParams);
    }

    #[test]
    fn test_classify_get_both() {
        let fixture = create_test_fixture("GET", true, true, None);
        assert_eq!(classify_route_type(&fixture), RouteType::GetBoth);
    }

    #[test]
    fn test_classify_post_simple() {
        let body = serde_json::json!({"name": "test"});
        let fixture = create_test_fixture("POST", false, false, Some(body));
        assert_eq!(classify_route_type(&fixture), RouteType::PostJsonSimple);
    }

    #[test]
    fn test_classify_post_nested() {
        let body = serde_json::json!({
            "level1": {
                "level2": {
                    "level3": {
                        "value": "deep"
                    }
                }
            }
        });
        let fixture = create_test_fixture("POST", false, false, Some(body));
        assert_eq!(classify_route_type(&fixture), RouteType::PostJsonNested);
    }

    #[test]
    fn test_classify_put() {
        let body = serde_json::json!({"name": "test"});
        let fixture = create_test_fixture("PUT", false, false, Some(body));
        assert_eq!(classify_route_type(&fixture), RouteType::PutJson);
    }

    #[test]
    fn test_classify_delete() {
        let fixture = create_test_fixture("DELETE", false, false, None);
        assert_eq!(classify_route_type(&fixture), RouteType::Delete);
    }

    #[test]
    fn test_is_nested_json() {
        // Simple flat object - not nested
        let flat = serde_json::json!({"a": 1, "b": 2});
        assert!(!is_nested_json(&flat, 0));

        // 2 levels - not nested enough
        let two_levels = serde_json::json!({"a": {"b": 1}});
        assert!(!is_nested_json(&two_levels, 0));

        // 3 levels - nested
        let three_levels = serde_json::json!({"a": {"b": {"c": 1}}});
        assert!(is_nested_json(&three_levels, 0));

        // Array with nested objects
        let nested_array = serde_json::json!([{"a": {"b": {"c": 1}}}]);
        assert!(is_nested_json(&nested_array, 0));
    }
}

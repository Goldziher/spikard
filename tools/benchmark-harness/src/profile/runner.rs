//! Profile mode runner

use crate::{
    error::{Error, Result},
    fixture::Fixture,
    load_generator::{self, LoadGeneratorType, LoadTestConfig},
    monitor::ResourceMonitor,
    schema::{
        profile::*, workload::WorkloadSuite, Configuration, FrameworkInfo, Latency, Metadata, ProfilingData,
        PythonProfilingData, Resources, Throughput,
    },
    server::{ServerConfig, ServerHandle, start_server},
};
use std::path::PathBuf;

/// Profile mode configuration
pub struct ProfileRunnerConfig {
    pub framework: String,
    pub app_dir: PathBuf,
    pub suite_name: String,
    pub duration_secs: u64,
    pub concurrency: usize,
    pub warmup_secs: u64,
    pub profiler: Option<String>, // "python", "node", "ruby", "perf"
    pub baseline_path: Option<PathBuf>,
    pub variant: Option<String>,
}

/// Profile mode runner
pub struct ProfileRunner {
    config: ProfileRunnerConfig,
    suite: WorkloadSuite,
}

impl ProfileRunner {
    pub fn new(config: ProfileRunnerConfig) -> Result<Self> {
        // Load workload suite
        let suite = WorkloadSuite::by_name(&config.suite_name)
            .ok_or_else(|| Error::InvalidInput(format!("Unknown suite: {}", config.suite_name)))?;

        Ok(Self { config, suite })
    }

    /// Run the profile benchmark
    pub async fn run(self) -> Result<ProfileResult> {
        println!("🔬 Profile Mode");
        println!("Framework: {}", self.config.framework);
        println!("Suite: {} ({} workloads)", self.suite.name, self.suite.workloads.len());
        println!();

        // Collect metadata
        let metadata = Metadata::collect();

        // Framework info
        let framework_info = FrameworkInfo {
            name: self.config.framework.clone(),
            version: self.detect_framework_version(),
            language: self.detect_language(),
            runtime: self.detect_runtime(),
            app_dir: self.config.app_dir.display().to_string(),
            variant: self.config.variant.clone(),
        };

        // Configuration
        let configuration = Configuration {
            duration_secs: self.config.duration_secs,
            concurrency: self.config.concurrency,
            warmup_secs: self.config.warmup_secs,
            load_tool: "oha".to_string(),
        };

        // Start server
        println!("🚀 Starting {} server...", self.config.framework);
        let port = self.find_available_port();
        let server_config = ServerConfig {
            framework: self.config.framework.clone(),
            port,
            app_dir: self.config.app_dir.clone(),
            variant: self.config.variant.clone(),
        };

        let server = start_server(server_config).await?;
        println!("✓ Server healthy on port {}", server.port);
        println!();

        // Run workloads
        let mut suite_results = Vec::new();
        let suite_result = self.run_suite(&server).await?;
        suite_results.push(suite_result);

        // Calculate summary
        let summary = self.calculate_summary(&suite_results);

        // Load baseline comparison if provided
        let comparison = if let Some(baseline_path) = &self.config.baseline_path {
            Some(self.load_baseline_comparison(baseline_path, &suite_results)?)
        } else {
            None
        };

        // Kill server
        server.kill()?;

        Ok(ProfileResult {
            metadata,
            framework: framework_info,
            configuration,
            suites: suite_results,
            summary,
            comparison,
        })
    }

    async fn run_suite(&self, server: &ServerHandle) -> Result<SuiteResult> {
        println!("📊 Running suite: {}", self.suite.name);
        println!();

        let mut workload_results = Vec::new();

        for (i, workload_def) in self.suite.workloads.iter().enumerate() {
            println!("[{}/{}] Testing: {}", i + 1, self.suite.workloads.len(), workload_def.name);

            let result = self.run_workload(workload_def, server).await?;
            workload_results.push(result);

            println!("  ✓ Complete");
            println!();
        }

        Ok(SuiteResult {
            name: self.suite.name.clone(),
            description: self.suite.description.clone(),
            workloads: workload_results,
        })
    }

    async fn run_workload(
        &self,
        workload_def: &crate::schema::workload::WorkloadDef,
        server: &ServerHandle,
    ) -> Result<WorkloadResult> {
        // Create fixture from workload definition
        let fixture = self.create_fixture_from_workload(workload_def)?;

        // Start profiler if configured
        let _profiler_handle = if let Some(ref profiler_type) = self.config.profiler {
            match profiler_type.as_str() {
                "python" => Some(crate::profile::python::start_profiler(server.pid())?),
                _ => None,
            }
        } else {
            None
        };

        // Start resource monitor
        let monitor = ResourceMonitor::new(server.pid());
        let monitor_handle = monitor.start_monitoring(100); // Sample every 100ms

        // Warmup
        if self.config.warmup_secs > 0 {
            self.run_load_test(&server.base_url, &fixture, self.config.warmup_secs)
                .await?;
        }

        // Actual benchmark
        let (oha_output, throughput) = self
            .run_load_test(&server.base_url, &fixture, self.config.duration_secs)
            .await?;

        // Stop monitor and collect samples
        let monitor_with_samples = monitor_handle.stop().await;
        let resource_metrics = monitor_with_samples.calculate_metrics();
        let cpu_p95 = monitor_with_samples.cpu_percentile(95.0);

        // Stop profiler and collect data
        let profiling = _profiler_handle.map(|_| ProfilingData::Python(PythonProfilingData {
                gil_wait_time_ms: None,
                gil_contention_percent: None,
                ffi_overhead_ms: None,
                handler_time_ms: None,
                serialization_time_ms: None,
                gc_collections: None,
                gc_time_ms: None,
                flamegraph_path: None,
            }));

        // Convert to schema types
        let throughput_schema = Throughput {
            requests_per_sec: throughput.requests_per_sec,
            bytes_per_sec: throughput.bytes_per_sec,
            total_requests: throughput.total_requests,
            successful_requests: throughput.total_requests - throughput.failed_requests,
            failed_requests: throughput.failed_requests,
            success_rate: throughput.success_rate,
        };

        // Extract latency from oha_output
        let latency_metrics: crate::types::LatencyMetrics = oha_output.into();
        let latency = Latency {
            mean_ms: latency_metrics.mean_ms,
            median_ms: latency_metrics.p50_ms,
            p90_ms: latency_metrics.p90_ms,
            p95_ms: latency_metrics.p95_ms,
            p99_ms: latency_metrics.p99_ms,
            p999_ms: latency_metrics.p999_ms,
            min_ms: latency_metrics.min_ms,
            max_ms: latency_metrics.max_ms,
            stddev_ms: latency_metrics.stddev_ms,
        };

        let resources = Resources {
            cpu: crate::schema::CpuMetrics {
                avg_percent: resource_metrics.avg_cpu_percent,
                peak_percent: resource_metrics.peak_cpu_percent,
                p95_percent: cpu_p95,
            },
            memory: crate::schema::MemoryMetrics {
                avg_mb: resource_metrics.avg_memory_mb,
                peak_mb: resource_metrics.peak_memory_mb,
                p95_mb: resource_metrics.p95_memory_mb,
            },
        };

        let metrics = WorkloadMetrics {
            throughput: throughput_schema,
            latency,
            resources,
            profiling,
        };

        Ok(WorkloadResult {
            name: workload_def.name.clone(),
            description: workload_def.description.clone(),
            category: workload_def.category.clone(),
            payload_size_bytes: workload_def.payload_size_bytes,
            endpoint: workload_def.endpoint.clone(),
            results: metrics,
        })
    }

    async fn run_load_test(
        &self,
        base_url: &str,
        fixture: &Fixture,
        duration_secs: u64,
    ) -> Result<(crate::types::OhaOutput, crate::types::ThroughputMetrics)> {
        let config = LoadTestConfig {
            base_url: base_url.to_string(),
            duration_secs,
            concurrency: self.config.concurrency,
            fixture: Some(fixture.clone()),
        };

        load_generator::run_load_test(config, LoadGeneratorType::Oha).await
    }

    fn create_fixture_from_workload(
        &self,
        workload_def: &crate::schema::workload::WorkloadDef,
    ) -> Result<Fixture> {
        use crate::fixture::{Handler, Request, ExpectedResponse, Parameters};

        let mut headers = std::collections::HashMap::new();
        if let Some(ref content_type) = workload_def.content_type {
            headers.insert("Content-Type".to_string(), content_type.clone());
        }

        // Load body from testing_data fixtures
        let body = if let Some(ref body_file) = workload_def.body_file {
            Some(self.load_body_from_fixtures(body_file)?)
        } else {
            None
        };

        let request = Request {
            method: workload_def.endpoint.method.clone(),
            path: workload_def.endpoint.path.clone(),
            query_params: std::collections::HashMap::new(),
            headers,
            body,
            body_raw: None,
            cookies: std::collections::HashMap::new(),
        };

        let handler = Handler {
            route: workload_def.endpoint.path.clone(),
            method: workload_def.endpoint.method.clone(),
            parameters: Parameters::default(),
        };

        let expected_response = ExpectedResponse {
            status_code: 200,
            body: None,
            headers: std::collections::HashMap::new(),
        };

        Ok(Fixture {
            name: workload_def.name.clone(),
            description: workload_def.description.clone(),
            category: Some(workload_def.category.clone()),
            handler,
            request,
            expected_response,
        })
    }

    /// Resolve fixture path relative to workspace root
    fn resolve_fixture_path(&self, fixture_file: &str) -> PathBuf {
        // Assume fixture_file is relative to testing_data
        let workspace_root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        workspace_root.join("testing_data").join(fixture_file)
    }

    /// Load body data from testing_data fixtures
    fn load_body_from_fixtures(&self, body_file: &str) -> Result<serde_json::Value> {
        // Map body file names to actual fixture files in testing_data
        let fixture_path = match body_file {
            "json-small.json" => "json_bodies/01_simple_object_success.json",
            "json-medium.json" => "json_bodies/04_nested_object_success.json",
            "json-large.json" => "json_bodies/25_deeply_nested_objects.json",
            "json-very-large.json" => "json_bodies/05_array_of_objects.json",
            _ => {
                // Try direct path in testing_data
                body_file
            }
        };

        let full_path = self.resolve_fixture_path(fixture_path);

        // Load fixture and extract body
        match Fixture::from_file(&full_path) {
            Ok(fixture) => {
                fixture.request.body.ok_or_else(|| {
                    Error::InvalidInput(format!("Fixture {} has no body", fixture_path))
                })
            }
            Err(_) => {
                // Fallback to generating synthetic data
                self.generate_synthetic_body(body_file)
            }
        }
    }

    /// Generate synthetic body data as fallback
    fn generate_synthetic_body(&self, body_file: &str) -> Result<serde_json::Value> {
        let json_str = match body_file {
            "json-small.json" => r#"{"id":12345,"name":"test_item","active":true,"count":42}"#,
            "json-medium.json" => r#"{"id":12345,"name":"test_item","description":"Medium payload","price":99.99,"tags":["tag1","tag2","tag3"]}"#,
            "json-large.json" => r#"{"id":12345,"name":"large_item","description":"Large payload with nested data","metadata":{"version":1,"status":"active"},"attributes":{"color":"blue","size":"large"}}"#,
            "json-very-large.json" => {
                // Generate array of 50 items
                let items: Vec<String> = (0..50)
                    .map(|i| format!(r#"{{"id":{},"name":"item_{}","value":{}}}"#, i, i, i * 100))
                    .collect();
                return Ok(serde_json::json!({
                    "items": items.iter().map(|s| serde_json::from_str::<serde_json::Value>(s).unwrap()).collect::<Vec<_>>(),
                    "count": 50
                }));
            }
            _ => return Err(Error::InvalidInput(format!("Unknown body file: {}", body_file))),
        };

        serde_json::from_str(json_str).map_err(Error::Json)
    }

    fn calculate_summary(&self, suite_results: &[SuiteResult]) -> ProfileSummary {
        let total_workloads: usize = suite_results.iter().map(|s| s.workloads.len()).sum();

        let total_requests: u64 = suite_results
            .iter()
            .flat_map(|s| &s.workloads)
            .map(|w| w.results.throughput.total_requests)
            .sum();

        let successful_requests: u64 = suite_results
            .iter()
            .flat_map(|s| &s.workloads)
            .map(|w| w.results.throughput.successful_requests)
            .sum();

        let overall_success_rate = if total_requests > 0 {
            successful_requests as f64 / total_requests as f64
        } else {
            0.0
        };

        let avg_requests_per_sec = if total_workloads > 0 {
            suite_results
                .iter()
                .flat_map(|s| &s.workloads)
                .map(|w| w.results.throughput.requests_per_sec)
                .sum::<f64>()
                / total_workloads as f64
        } else {
            0.0
        };

        let total_duration_secs = self.config.duration_secs * total_workloads as u64;

        // Category breakdown
        let mut category_map: std::collections::HashMap<String, Vec<&WorkloadResult>> = std::collections::HashMap::new();
        for suite in suite_results {
            for workload in &suite.workloads {
                category_map.entry(workload.category.clone()).or_default().push(workload);
            }
        }

        let category_breakdown: Vec<CategorySummary> = category_map
            .into_iter()
            .map(|(category, workloads)| {
                let avg_rps = workloads.iter().map(|w| w.results.throughput.requests_per_sec).sum::<f64>()
                    / workloads.len() as f64;
                let avg_latency = workloads.iter().map(|w| w.results.latency.mean_ms).sum::<f64>() / workloads.len() as f64;

                CategorySummary {
                    category,
                    workload_count: workloads.len(),
                    avg_requests_per_sec: avg_rps,
                    avg_latency_ms: avg_latency,
                }
            })
            .collect();

        ProfileSummary {
            total_workloads,
            total_requests,
            overall_success_rate,
            avg_requests_per_sec,
            total_duration_secs,
            category_breakdown,
        }
    }

    fn load_baseline_comparison(
        &self,
        baseline_path: &PathBuf,
        suite_results: &[SuiteResult],
    ) -> Result<BaselineComparison> {
        // Load baseline ProfileResult from JSON file
        let baseline_json = std::fs::read_to_string(baseline_path)
            .map_err(|e| Error::InvalidInput(format!("Failed to read baseline file: {}", e)))?;

        let baseline: crate::schema::profile::ProfileResult = serde_json::from_str(&baseline_json)
            .map_err(|e| Error::InvalidInput(format!("Failed to parse baseline JSON: {}", e)))?;

        // Build a map of workload name -> baseline metrics
        let mut baseline_map: std::collections::HashMap<String, &WorkloadResult> = std::collections::HashMap::new();
        for suite in &baseline.suites {
            for workload in &suite.workloads {
                baseline_map.insert(workload.name.clone(), workload);
            }
        }

        // Compare each current workload with baseline
        let mut workload_comparisons = Vec::new();
        let mut total_current_rps = 0.0;
        let mut total_baseline_rps = 0.0;

        for suite in suite_results {
            for current_workload in &suite.workloads {
                if let Some(baseline_workload) = baseline_map.get(&current_workload.name) {
                    let current_rps = current_workload.results.throughput.requests_per_sec;
                    let baseline_rps = baseline_workload.results.throughput.requests_per_sec;

                    let throughput_ratio = if baseline_rps > 0.0 {
                        current_rps / baseline_rps
                    } else {
                        1.0
                    };

                    workload_comparisons.push(WorkloadComparison {
                        workload_name: current_workload.name.clone(),
                        baseline_requests_per_sec: baseline_rps,
                        this_requests_per_sec: current_rps,
                        ratio: throughput_ratio,
                    });

                    total_current_rps += current_rps;
                    total_baseline_rps += baseline_rps;
                }
            }
        }

        let overall_ratio = if total_baseline_rps > 0.0 {
            total_current_rps / total_baseline_rps
        } else {
            1.0
        };

        Ok(BaselineComparison {
            baseline_framework: baseline.framework.name.clone(),
            workload_comparisons,
            overall_ratio,
        })
    }

    /// Find an available port for the server
    fn find_available_port(&self) -> u16 {
        // Try to bind to a port starting from 8100
        for port in 8100..8200 {
            if std::net::TcpListener::bind(("127.0.0.1", port)).is_ok() {
                return port;
            }
        }
        // Fallback to 8100 if no port found
        8100
    }

    fn detect_framework_version(&self) -> String {
        // Try to detect version from Cargo.toml for Rust apps
        if self.detect_language() == "rust" {
            let cargo_toml = self.config.app_dir.join("Cargo.toml");
            if let Ok(contents) = std::fs::read_to_string(cargo_toml) {
                // Simple parsing for version line
                for line in contents.lines() {
                    if line.trim().starts_with("version") {
                        if let Some(version) = line.split('=').nth(1) {
                            return version.trim().trim_matches('"').to_string();
                        }
                    }
                }
            }
        }
        // For now, return a default version for other languages
        "0.1.0".to_string()
    }

    fn detect_language(&self) -> String {
        if self.config.framework.contains("python") {
            "python".to_string()
        } else if self.config.framework.contains("node") {
            "node".to_string()
        } else if self.config.framework.contains("ruby") {
            "ruby".to_string()
        } else if self.config.framework.contains("rust") {
            "rust".to_string()
        } else {
            "unknown".to_string()
        }
    }

    fn detect_runtime(&self) -> String {
        match self.detect_language().as_str() {
            "python" => {
                // Try to detect actual Python version
                if let Ok(output) = std::process::Command::new("python3")
                    .arg("--version")
                    .output()
                {
                    if let Ok(version) = String::from_utf8(output.stdout) {
                        return version.trim().to_string();
                    }
                }
                "Python 3.x".to_string()
            }
            "node" => {
                // Try to detect actual Node version
                if let Ok(output) = std::process::Command::new("node")
                    .arg("--version")
                    .output()
                {
                    if let Ok(version) = String::from_utf8(output.stdout) {
                        return format!("Node {}", version.trim());
                    }
                }
                "Node.js".to_string()
            }
            "ruby" => {
                // Try to detect actual Ruby version
                if let Ok(output) = std::process::Command::new("ruby")
                    .arg("--version")
                    .output()
                {
                    if let Ok(version) = String::from_utf8(output.stdout) {
                        // Ruby version output is like "ruby 3.3.0 (2023-12-25 revision...)"
                        if let Some(version_part) = version.split_whitespace().nth(1) {
                            return format!("Ruby {}", version_part);
                        }
                    }
                }
                "Ruby".to_string()
            }
            "rust" => {
                // Try to detect actual rustc version
                if let Ok(output) = std::process::Command::new("rustc")
                    .arg("--version")
                    .output()
                {
                    if let Ok(version) = String::from_utf8(output.stdout) {
                        return version.trim().to_string();
                    }
                }
                "rustc".to_string()
            }
            _ => "unknown".to_string(),
        }
    }
}

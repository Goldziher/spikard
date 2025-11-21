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
            version: "0.1.0".to_string(), // TODO: detect from app
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
        let server_config = ServerConfig {
            framework: self.config.framework.clone(),
            port: 8100, // TODO: find available port
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
        let (_oha_output, throughput) = self
            .run_load_test(&server.base_url, &fixture, self.config.duration_secs)
            .await?;

        // Stop monitor and collect samples
        let monitor_with_samples = monitor_handle.stop().await;
        let resource_metrics = monitor_with_samples.calculate_metrics();

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

        // For latency, we'll use the resource metrics' placeholder values
        // TODO: extract from oha_output
        let latency = Latency {
            mean_ms: 0.0,
            median_ms: 0.0,
            p90_ms: 0.0,
            p95_ms: 0.0,
            p99_ms: 0.0,
            p999_ms: 0.0,
            min_ms: 0.0,
            max_ms: 0.0,
            stddev_ms: 0.0,
        };

        let resources = Resources {
            cpu: crate::schema::CpuMetrics {
                avg_percent: resource_metrics.avg_cpu_percent,
                peak_percent: resource_metrics.peak_cpu_percent,
                p95_percent: resource_metrics.avg_cpu_percent, // TODO: calculate p95 properly
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

        let body = if let Some(ref body_file) = workload_def.body_file {
            Some(self.load_body_data(body_file)?)
        } else {
            None
        };

        let request = Request {
            method: workload_def.endpoint.method.clone(),
            path: workload_def.endpoint.path.clone(),
            query_params: std::collections::HashMap::new(), // TODO: extract from path
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

    fn load_body_data(&self, body_file: &str) -> Result<serde_json::Value> {
        // TODO: load from fixtures directory
        // For now, return hardcoded test data
        let json_str = match body_file {
            "json-small.json" => r#"{"id":12345,"name":"test_item","active":true,"count":42,"tags":["tag1","tag2","tag3"]}"#,
            "json-medium.json" => r#"{
                "id": 12345,
                "name": "test_item_medium",
                "description": "This is a medium-sized JSON payload for benchmarking purposes.",
                "active": true,
                "count": 42,
                "price": 99.99,
                "currency": "USD",
                "category": "electronics",
                "tags": ["tag1", "tag2", "tag3", "tag4", "tag5"],
                "attributes": {
                    "color": "black",
                    "storage": "256GB",
                    "ram": "8GB"
                }
            }"#,
            "json-large.json" => r#"{
                "id": 12345,
                "name": "test_item_large",
                "description": "This is a large JSON payload for benchmarking purposes with extensive nested data.",
                "active": true,
                "count": 42,
                "price": 199.99,
                "currency": "USD",
                "category": "electronics",
                "subcategory": "laptops",
                "tags": ["premium", "business", "ultrabook", "touchscreen", "backlit-keyboard"],
                "metadata": {
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-15T12:30:45Z",
                    "version": 3,
                    "author": "system",
                    "status": "published"
                },
                "attributes": {
                    "brand": "TechBrand",
                    "model": "ProBook X1",
                    "color": "space gray",
                    "storage": "1TB SSD",
                    "ram": "32GB DDR5",
                    "cpu": "Intel Core i9-13900H",
                    "gpu": "NVIDIA RTX 4070",
                    "display": "15.6 inch 4K OLED",
                    "battery": "99Wh",
                    "weight": "1.8kg"
                },
                "specifications": [
                    {"key": "ports", "value": "4x USB-C, 2x USB-A, HDMI 2.1, SD card"},
                    {"key": "wireless", "value": "WiFi 6E, Bluetooth 5.3"},
                    {"key": "audio", "value": "Quad speakers, Dolby Atmos"},
                    {"key": "camera", "value": "1080p IR webcam with privacy shutter"},
                    {"key": "keyboard", "value": "Backlit with fingerprint sensor"}
                ],
                "reviews": [
                    {"rating": 5, "comment": "Excellent performance"},
                    {"rating": 4, "comment": "Great build quality"},
                    {"rating": 5, "comment": "Best laptop I've owned"}
                ]
            }"#,
            "json-very-large.json" => {
                // Generate a very large JSON (>100KB) with repeated data
                let mut items = Vec::new();
                for i in 0..100 {
                    items.push(format!(r#"{{"id":{},"name":"item_{}","value":{},"metadata":{{"timestamp":"2024-01-01T00:00:00Z","status":"active","tags":["tag1","tag2","tag3","tag4","tag5"]}}}}"#, i, i, i * 100));
                }
                &format!(r#"{{"items":[{}],"count":100,"total_value":495000}}"#, items.join(","))
            },
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
        _baseline_path: &PathBuf,
        _suite_results: &[SuiteResult],
    ) -> Result<BaselineComparison> {
        // TODO: implement baseline loading and comparison
        Ok(BaselineComparison {
            baseline_framework: "spikard-rust".to_string(),
            workload_comparisons: vec![],
            overall_ratio: 1.0,
        })
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
        // TODO: detect actual runtime version
        match self.detect_language().as_str() {
            "python" => "CPython 3.12".to_string(),
            "node" => "Node 20.10.0".to_string(),
            "ruby" => "Ruby 3.3.0".to_string(),
            "rust" => "rustc 1.75.0".to_string(),
            _ => "unknown".to_string(),
        }
    }
}

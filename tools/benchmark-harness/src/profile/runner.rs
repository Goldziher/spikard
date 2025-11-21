//! Profile mode runner

use crate::{
    error::{Error, Result},
    load_generator::{LoadGeneratorType, LoadTestConfig},
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
            .ok_or_else(|| Error::InvalidWorkload(format!("Unknown suite: {}", config.suite_name)))?;

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
        let suite_result = self.run_suite(&server, &metadata).await?;
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

    async fn run_suite(&self, server: &ServerHandle, _metadata: &Metadata) -> Result<SuiteResult> {
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
        let url = format!("{}{}", server.base_url, workload_def.endpoint.path);

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
        let mut monitor = ResourceMonitor::new(server.pid());
        monitor.start();

        // Warmup
        if self.config.warmup_secs > 0 {
            self.run_load_test(&url, &workload_def.endpoint.method, workload_def, self.config.warmup_secs)
                .await?;
        }

        // Actual benchmark
        let load_result = self
            .run_load_test(
                &url,
                &workload_def.endpoint.method,
                workload_def,
                self.config.duration_secs,
            )
            .await?;

        // Stop monitor
        monitor.stop().await;
        let samples = monitor.samples();

        // Stop profiler and collect data
        let profiling = if let Some(_profiler_handle) = _profiler_handle {
            // TODO: collect profiling data
            Some(ProfilingData::Python(PythonProfilingData {
                gil_wait_time_ms: None,
                gil_contention_percent: None,
                ffi_overhead_ms: None,
                handler_time_ms: None,
                serialization_time_ms: None,
                gc_collections: None,
                gc_time_ms: None,
                flamegraph_path: None,
            }))
        } else {
            None
        };

        // Convert load test result to schema types
        let throughput = Throughput {
            requests_per_sec: load_result.throughput.requests_per_sec,
            bytes_per_sec: load_result.throughput.bytes_per_sec,
            total_requests: load_result.throughput.total_requests,
            successful_requests: load_result.throughput.successful_requests,
            failed_requests: load_result.throughput.failed_requests,
            success_rate: load_result.throughput.success_rate,
        };

        let latency = Latency {
            mean_ms: load_result.latency.mean_ms,
            median_ms: load_result.latency.p50_ms,
            p90_ms: load_result.latency.p90_ms,
            p95_ms: load_result.latency.p95_ms,
            p99_ms: load_result.latency.p99_ms,
            p999_ms: load_result.latency.p999_ms,
            min_ms: load_result.latency.min_ms,
            max_ms: load_result.latency.max_ms,
            stddev_ms: load_result.latency.stddev_ms,
        };

        let resources = self.calculate_resources(&samples);

        let metrics = WorkloadMetrics {
            throughput,
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
        url: &str,
        method: &str,
        workload_def: &crate::schema::workload::WorkloadDef,
        duration_secs: u64,
    ) -> Result<crate::types::BenchmarkResult> {
        let mut config = LoadTestConfig {
            url: url.to_string(),
            method: method.to_string(),
            headers: vec![],
            body: None,
            duration_secs,
            concurrency: self.config.concurrency,
            generator: LoadGeneratorType::Oha,
        };

        // Add body if needed
        if let Some(ref body_file) = workload_def.body_file {
            let body_data = self.load_body_data(body_file)?;
            config.body = Some(body_data);
        }

        // Add content-type header
        if let Some(ref content_type) = workload_def.content_type {
            config.headers.push(("Content-Type".to_string(), content_type.clone()));
        }

        let generator = crate::load_generator::create_generator(config)?;
        generator.run().await
    }

    fn load_body_data(&self, body_file: &str) -> Result<String> {
        // TODO: load from fixtures directory
        // For now, return hardcoded test data
        match body_file {
            "json-small.json" => Ok(r#"{"id":12345,"name":"test_item","active":true,"count":42,"tags":["tag1","tag2","tag3"]}"#.to_string()),
            "urlencoded-simple.txt" => Ok("name=John+Doe&email=john%40example.com&age=30&city=New+York".to_string()),
            "urlencoded-complex.txt" => Ok("name=John+Doe&email=john%40example.com&age=30&city=New+York&country=USA&phone=%2B1234567890&address=123+Main+St&zip=10001&interests=tech&interests=sports&interests=music&company=TestCorp&position=Engineer&department=Engineering&salary=100000&start_date=2020-01-15&active=true&verified=true&newsletter=true&terms=true".to_string()),
            _ => Err(Error::InvalidWorkload(format!("Unknown body file: {}", body_file))),
        }
    }

    fn calculate_resources(&self, samples: &[crate::monitor::ResourceSample]) -> Resources {
        if samples.is_empty() {
            return Resources {
                cpu: crate::schema::CpuMetrics {
                    avg_percent: 0.0,
                    peak_percent: 0.0,
                    p95_percent: 0.0,
                },
                memory: crate::schema::MemoryMetrics {
                    avg_mb: 0.0,
                    peak_mb: 0.0,
                    p95_mb: 0.0,
                },
            };
        }

        let avg_cpu = samples.iter().map(|s| s.cpu_percent).sum::<f64>() / samples.len() as f64;
        let peak_cpu = samples.iter().map(|s| s.cpu_percent).fold(0.0f64, f64::max);

        let avg_mem = samples.iter().map(|s| s.memory_mb).sum::<f64>() / samples.len() as f64;
        let peak_mem = samples.iter().map(|s| s.memory_mb).fold(0.0f64, f64::max);

        // Calculate p95
        let mut cpu_sorted: Vec<f64> = samples.iter().map(|s| s.cpu_percent).collect();
        cpu_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let p95_cpu = cpu_sorted[(cpu_sorted.len() as f64 * 0.95) as usize];

        let mut mem_sorted: Vec<f64> = samples.iter().map(|s| s.memory_mb).collect();
        mem_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let p95_mem = mem_sorted[(mem_sorted.len() as f64 * 0.95) as usize];

        Resources {
            cpu: crate::schema::CpuMetrics {
                avg_percent: avg_cpu,
                peak_percent: peak_cpu,
                p95_percent: p95_cpu,
            },
            memory: crate::schema::MemoryMetrics {
                avg_mb: avg_mem,
                peak_mb: peak_mem,
                p95_mb: p95_mem,
            },
        }
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

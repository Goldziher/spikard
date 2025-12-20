//! Profile mode runner

use crate::{
    error::{Error, Result},
    fixture::Fixture,
    load_generator::{self, LoadGeneratorType, LoadTestConfig},
    monitor::ResourceMonitor,
    schema::{
        Configuration, FrameworkInfo, Latency, Metadata, NodeProfilingData, PhpProfilingData, ProfilingData,
        PythonProfilingData, Resources, RubyProfilingData, RustProfilingData, Throughput, WasmProfilingData,
        profile::*, workload::WorkloadSuite,
    },
    server::{ServerConfig, ServerHandle, start_server},
};
use std::path::PathBuf;
use sysinfo::{Pid, System};
fn find_descendant_pid_by_name(root_pid: u32, needle: &str, max_depth: usize) -> Option<u32> {
    let mut system = System::new();
    system.refresh_all();

    let root = Pid::from_u32(root_pid);

    let mut children: std::collections::HashMap<Pid, Vec<Pid>> = std::collections::HashMap::new();
    for (&pid, proc_) in system.processes() {
        if let Some(parent) = proc_.parent() {
            children.entry(parent).or_default().push(pid);
        }
    }

    let needle = needle.to_lowercase();
    let mut queue: std::collections::VecDeque<(Pid, usize)> = std::collections::VecDeque::new();
    queue.push_back((root, 0));

    while let Some((pid, depth)) = queue.pop_front() {
        if depth > max_depth {
            continue;
        }

        if let Some(proc_) = system.process(pid) {
            let name = proc_.name().to_string_lossy().to_lowercase();
            if name.contains(&needle) {
                return Some(pid.as_u32());
            }
        }

        if let Some(kids) = children.get(&pid) {
            for &child in kids {
                queue.push_back((child, depth + 1));
            }
        }
    }

    None
}

/// Unified profiler handle for different languages
enum ProfilerHandle {
    Python(crate::profile::python::PythonProfiler),
    PythonInApp(PathBuf),
    #[cfg(target_os = "linux")]
    PerfPython(crate::profile::perf::PerfProfiler),
    Node(crate::profile::node::NodeProfiler),
    Ruby(crate::profile::ruby::RubyProfiler),
    Php(crate::profile::php::PhpProfiler),
    Rust(crate::profile::rust::RustProfiler),
    RustInApp(PathBuf),
}

/// Profile mode configuration
pub struct ProfileRunnerConfig {
    pub framework: String,
    pub app_dir: PathBuf,
    pub suite_name: String,
    pub duration_secs: u64,
    pub concurrency: usize,
    pub warmup_secs: u64,
    pub profiler: Option<String>,
    pub baseline_path: Option<PathBuf>,
    pub variant: Option<String>,
    pub output_dir: Option<PathBuf>,
}

/// Profile mode runner
pub struct ProfileRunner {
    config: ProfileRunnerConfig,
    suite: WorkloadSuite,
}

impl ProfileRunner {
    pub fn new(config: ProfileRunnerConfig) -> Result<Self> {
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

        let metadata = Metadata::collect();

        let framework_info = FrameworkInfo {
            name: self.config.framework.clone(),
            version: self.detect_framework_version(),
            language: self.detect_language(),
            runtime: self.detect_runtime(),
            app_dir: self.config.app_dir.display().to_string(),
            variant: self.config.variant.clone(),
        };

        let configuration = Configuration {
            duration_secs: self.config.duration_secs,
            concurrency: self.config.concurrency,
            warmup_secs: self.config.warmup_secs,
            load_tool: "oha".to_string(),
        };

        println!("🚀 Starting {} server...", self.config.framework);
        let port = self.find_available_port();

        let suite_php_profiler = self.config.profiler.as_deref() == Some("php") && framework_info.language == "php";
        let suite_node_profiler = self.config.profiler.as_deref() == Some("node") && framework_info.language == "node";
        let suite_wasm_profiler = self.config.profiler.as_deref() == Some("wasm") && framework_info.language == "wasm";

        let php_profile_output = suite_php_profiler
            .then(|| std::env::var("SPIKARD_PHP_PROFILE_OUTPUT").ok().map(PathBuf::from))
            .flatten();
        let wasm_metrics_output = suite_wasm_profiler
            .then(|| std::env::var("SPIKARD_WASM_METRICS_FILE").ok().map(PathBuf::from))
            .flatten();
        let wasm_v8_log_output = suite_wasm_profiler
            .then(|| std::env::var("SPIKARD_WASM_V8_LOG_OUTPUT").ok().map(PathBuf::from))
            .flatten();

        let mut server_env = Vec::new();
        if let Some(ref output_path) = php_profile_output {
            server_env.push((
                "SPIKARD_PHP_PROFILE_OUTPUT".to_string(),
                output_path.display().to_string(),
            ));
        }
        if let Some(ref output_path) = wasm_metrics_output {
            server_env.push((
                "SPIKARD_WASM_METRICS_FILE".to_string(),
                output_path.display().to_string(),
            ));
        }

        let node_cpu_profile_dir = suite_node_profiler.then(|| {
            self.config
                .output_dir
                .as_ref()
                .map(|dir| dir.join("profiles"))
                .unwrap_or_else(|| PathBuf::from("profiles"))
        });
        if let Some(ref dir) = node_cpu_profile_dir {
            let _ = std::fs::create_dir_all(dir);
        }

        if suite_wasm_profiler {
            let _ = std::fs::remove_file(self.config.app_dir.join("v8.log"));
        }

        let node_start_cmd_override = node_cpu_profile_dir.as_ref().map(|dir| {
            let tsx_cli = self.config.app_dir.join("../../../../node_modules/tsx/dist/cli.mjs");
            format!(
                "node --cpu-prof --cpu-prof-dir {} {} server.ts {{port}}",
                dir.display(),
                tsx_cli.display()
            )
        });

        let server_config = ServerConfig {
            framework: Some(self.config.framework.clone()),
            port,
            app_dir: self.config.app_dir.clone(),
            variant: self.config.variant.clone(),
            env: server_env,
            start_cmd_override: if suite_wasm_profiler {
                Some(
                    "deno run --allow-net --allow-read --allow-env --allow-write --v8-flags=--prof,--logfile=v8.log server.ts {port}"
                        .to_string(),
                )
            } else {
                node_start_cmd_override
            },
        };

        let server = start_server(server_config).await?;
        println!("✓ Server healthy on port {}", server.port);
        println!();

        let python_pid = (framework_info.language == "python").then(|| self.python_target_pid(&server));

        let mut suite_results = Vec::new();
        let suite_result = self.run_suite(&server, suite_php_profiler).await?;

        suite_results.push(suite_result);

        let summary = self.calculate_summary(&suite_results);

        let comparison = if let Some(baseline_path) = &self.config.baseline_path {
            Some(self.load_baseline_comparison(baseline_path, &suite_results)?)
        } else {
            None
        };

        self.try_flush_python_profiling(&server).await;
        let suite_app_metrics = python_pid.and_then(crate::profile::python::collect_app_metrics);
        server.kill()?;

        if let Some(metrics) = suite_app_metrics.as_ref() {
            for suite in &mut suite_results {
                for workload in &mut suite.workloads {
                    let Some(ProfilingData::Python(data)) = workload.results.profiling.as_mut() else {
                        continue;
                    };
                    if data.ffi_overhead_ms.is_none() {
                        data.ffi_overhead_ms = metrics.ffi_overhead_ms;
                    }
                    if data.handler_time_ms.is_none() {
                        data.handler_time_ms = metrics.handler_time_ms;
                    }
                    if data.serialization_time_ms.is_none() {
                        data.serialization_time_ms = metrics.serialization_time_ms;
                    }
                    if data.gc_collections.is_none() {
                        data.gc_collections = metrics.gc_collections;
                    }
                    if data.gc_time_ms.is_none() {
                        data.gc_time_ms = metrics.gc_time_ms;
                    }
                }
            }
        }
        if suite_php_profiler {
            let flamegraph_path = php_profile_output
                .as_ref()
                .and_then(|p| p.to_str())
                .and_then(crate::profile::php::wait_for_profile_output);

            for suite in &mut suite_results {
                for workload in &mut suite.workloads {
                    workload.results.profiling = Some(ProfilingData::Php(PhpProfilingData {
                        flamegraph_path: flamegraph_path.clone(),
                    }));
                }
            }
        }
        if suite_node_profiler {
            let profile_path = node_cpu_profile_dir
                .as_ref()
                .and_then(|dir| find_latest_file_with_extension(dir, "cpuprofile"))
                .map(|p| p.display().to_string());

            for suite in &mut suite_results {
                for workload in &mut suite.workloads {
                    let profiling = workload.results.profiling.take();
                    let (v8_heap_used_mb, v8_heap_total_mb, event_loop_lag_ms, gc_time_ms) = match profiling {
                        Some(ProfilingData::Node(data)) => (
                            data.v8_heap_used_mb,
                            data.v8_heap_total_mb,
                            data.event_loop_lag_ms,
                            data.gc_time_ms,
                        ),
                        _ => (None, None, None, None),
                    };
                    workload.results.profiling = Some(ProfilingData::Node(NodeProfilingData {
                        v8_heap_used_mb,
                        v8_heap_total_mb,
                        event_loop_lag_ms,
                        gc_time_ms,
                        flamegraph_path: profile_path.clone(),
                    }));
                }
            }
        }
        if suite_wasm_profiler {
            let metrics = wasm_metrics_output
                .as_ref()
                .and_then(|p| p.to_str())
                .and_then(crate::profile::wasm::wait_for_metrics_output);
            let v8_log_path = wasm_v8_log_output
                .as_ref()
                .and_then(|p| crate::profile::wasm::collect_v8_log(&self.config.app_dir, p).ok())
                .flatten();

            for suite in &mut suite_results {
                for workload in &mut suite.workloads {
                    workload.results.profiling = Some(ProfilingData::Wasm(WasmProfilingData {
                        rss_mb: metrics.as_ref().and_then(|m| m.rss_mb),
                        heap_total_mb: metrics.as_ref().and_then(|m| m.heap_total_mb),
                        heap_used_mb: metrics.as_ref().and_then(|m| m.heap_used_mb),
                        external_mb: metrics.as_ref().and_then(|m| m.external_mb),
                        v8_log_path: v8_log_path.clone(),
                    }));
                }
            }
        }

        Ok(ProfileResult {
            metadata,
            framework: framework_info,
            configuration,
            suites: suite_results,
            summary,
            comparison,
        })
    }

    async fn try_flush_python_profiling(&self, server: &ServerHandle) {
        if self.detect_language() != "python" {
            return;
        }

        let Ok(client) = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
        else {
            return;
        };

        let url = format!("{}/__benchmark__/flush-profile", server.base_url);
        for attempt in 1..=5 {
            if let Ok(resp) = client.get(&url).send().await
                && resp.status().is_success()
            {
                return;
            }
            if attempt < 5 {
                tokio::time::sleep(std::time::Duration::from_millis(250 * attempt)).await;
            }
        }
    }

    async fn run_suite(&self, server: &ServerHandle, suite_php_profiler: bool) -> Result<SuiteResult> {
        println!("📊 Running suite: {}", self.suite.name);
        println!();

        let mut workload_results = Vec::new();

        for (i, workload_def) in self.suite.workloads.iter().enumerate() {
            println!(
                "[{}/{}] Testing: {}",
                i + 1,
                self.suite.workloads.len(),
                workload_def.name
            );

            let result = self.run_workload(workload_def, server, suite_php_profiler).await?;
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

    fn python_target_pid(&self, server: &ServerHandle) -> u32 {
        // The server is frequently started via a wrapper (e.g. `uv`), so we find the actual
        // Python child process when possible.
        find_descendant_pid_by_name(server.pid(), "python", 10).unwrap_or(server.pid())
    }

    async fn try_profile_start(&self, server: &ServerHandle, output_path: &std::path::Path) -> bool {
        let Ok(client) = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
        else {
            return false;
        };

        let Ok(mut url) = reqwest::Url::parse(&format!("{}/__benchmark__/profile/start", server.base_url)) else {
            return false;
        };
        url.query_pairs_mut()
            .append_pair("output", &output_path.display().to_string());

        let Ok(resp) = client.get(url).send().await else {
            return false;
        };
        if !resp.status().is_success() {
            return false;
        }

        match resp.json::<serde_json::Value>().await {
            Ok(body) => body.get("ok").and_then(serde_json::Value::as_bool).unwrap_or(true),
            Err(_) => true,
        }
    }

    async fn try_profile_stop(&self, server: &ServerHandle) -> bool {
        let Ok(client) = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
        else {
            return false;
        };

        let url = format!("{}/__benchmark__/profile/stop", server.base_url);
        let Ok(resp) = client.get(url).send().await else {
            return false;
        };
        if !resp.status().is_success() {
            return false;
        }

        match resp.json::<serde_json::Value>().await {
            Ok(body) => body.get("ok").and_then(serde_json::Value::as_bool).unwrap_or(true),
            Err(_) => true,
        }
    }

    async fn run_workload(
        &self,
        workload_def: &crate::schema::workload::WorkloadDef,
        server: &ServerHandle,
        suite_php_profiler: bool,
    ) -> Result<WorkloadResult> {
        let fixture = self.create_fixture_from_workload(workload_def)?;

        let monitor = ResourceMonitor::new(server.pid());
        let monitor_handle = monitor.start_monitoring(100);

        if self.config.warmup_secs > 0 {
            self.run_load_test(&server.base_url, &fixture, self.config.warmup_secs)
                .await?;
        }

        let suite_node_profiler = self.config.profiler.as_deref() == Some("node") && self.detect_language() == "node";
        let output_dir = self.config.output_dir.clone().unwrap_or_else(|| PathBuf::from("."));

        let profiler_handle = if let Some(ref profiler_type) = self.config.profiler {
            match profiler_type.as_str() {
                "python" => {
                    let output_path = absolutize_path(
                        output_dir
                            .join("profiles")
                            .join(&self.config.framework)
                            .join(format!("{}.speedscope.json", workload_def.name)),
                    );
                    let _ = std::fs::create_dir_all(output_path.parent().unwrap_or(&output_dir));

                    if self.detect_language() == "python" && self.try_profile_start(server, &output_path).await {
                        Some(ProfilerHandle::PythonInApp(output_path))
                    } else {
                        let python_pid = self.python_target_pid(server);
                        Some(ProfilerHandle::Python(crate::profile::python::start_profiler(
                            python_pid,
                            Some(output_path),
                            self.config.duration_secs,
                        )?))
                    }
                }
                #[cfg(target_os = "linux")]
                "perf" => {
                    if self.detect_language() != "python" {
                        eprintln!("  ⚠ perf profiler currently only supported for python targets");
                        None
                    } else {
                        let output_path = absolutize_path(
                            output_dir
                                .join("profiles")
                                .join(&self.config.framework)
                                .join(format!("{}.perf.svg", workload_def.name)),
                        );
                        let _ = std::fs::create_dir_all(output_path.parent().unwrap_or(&output_dir));
                        let python_pid = self.python_target_pid(server);
                        Some(ProfilerHandle::PerfPython(crate::profile::perf::start_profiler(
                            python_pid,
                            Some(output_path),
                            self.config.duration_secs,
                        )?))
                    }
                }
                "php" => {
                    if suite_php_profiler {
                        None
                    } else {
                        let output_path = output_dir.join(format!("php-{}.speedscope.json", workload_def.name));
                        Some(ProfilerHandle::Php(crate::profile::php::start_profiler(
                            server.pid(),
                            Some(output_path),
                        )?))
                    }
                }
                "node" => {
                    if suite_node_profiler {
                        None
                    } else {
                        Some(ProfilerHandle::Node(crate::profile::node::start_profiler(
                            server.pid(),
                        )?))
                    }
                }
                "ruby" => {
                    let output_path = absolutize_path(
                        output_dir
                            .join("profiles")
                            .join(&self.config.framework)
                            .join(format!("{}.speedscope.json", workload_def.name)),
                    );
                    Some(ProfilerHandle::Ruby(crate::profile::ruby::start_profiler(
                        server.pid(),
                        Some(output_path),
                        self.config.duration_secs,
                    )?))
                }
                "rust" => {
                    let output_path = absolutize_path(
                        output_dir
                            .join("profiles")
                            .join(&self.config.framework)
                            .join(format!("{}.svg", workload_def.name)),
                    );
                    let _ = std::fs::create_dir_all(output_path.parent().unwrap_or(&output_dir));
                    if self.try_profile_start(server, &output_path).await {
                        Some(ProfilerHandle::RustInApp(output_path))
                    } else {
                        Some(ProfilerHandle::Rust(crate::profile::rust::start_profiler(
                            server.pid(),
                        )?))
                    }
                }
                "wasm" => None,
                _ => {
                    eprintln!("  ⚠ Unknown profiler type: {}", profiler_type);
                    None
                }
            }
        } else {
            None
        };

        let (oha_output, throughput) = self
            .run_load_test(&server.base_url, &fixture, self.config.duration_secs)
            .await?;

        if matches!(profiler_handle, Some(ProfilerHandle::RustInApp(_))) {
            let _ = self.try_profile_stop(server).await;
        }
        if matches!(profiler_handle, Some(ProfilerHandle::PythonInApp(_))) {
            let _ = self.try_profile_stop(server).await;
        }

        let monitor_with_samples = monitor_handle.stop().await;
        let resource_metrics = monitor_with_samples.calculate_metrics();
        let cpu_p95 = monitor_with_samples.cpu_percentile(95.0);

        let profiling = profiler_handle.map(|profiler| match profiler {
            ProfilerHandle::Python(p) => {
                let data = p.stop();
                ProfilingData::Python(PythonProfilingData {
                    gil_wait_time_ms: data.gil_wait_time_ms,
                    gil_contention_percent: data.gil_contention_percent,
                    ffi_overhead_ms: data.ffi_overhead_ms,
                    handler_time_ms: data.handler_time_ms,
                    serialization_time_ms: data.serialization_time_ms,
                    gc_collections: data.gc_collections,
                    gc_time_ms: data.gc_time_ms,
                    flamegraph_path: data.flamegraph_path,
                })
            }
            ProfilerHandle::PythonInApp(path) => {
                let flamegraph_path = path.to_str().and_then(crate::profile::python::wait_for_profile_output);
                ProfilingData::Python(PythonProfilingData {
                    gil_wait_time_ms: None,
                    gil_contention_percent: None,
                    ffi_overhead_ms: None,
                    handler_time_ms: None,
                    serialization_time_ms: None,
                    gc_collections: None,
                    gc_time_ms: None,
                    flamegraph_path,
                })
            }
            #[cfg(target_os = "linux")]
            ProfilerHandle::PerfPython(p) => {
                let flamegraph_path = p.stop();
                ProfilingData::Python(PythonProfilingData {
                    gil_wait_time_ms: None,
                    gil_contention_percent: None,
                    ffi_overhead_ms: None,
                    handler_time_ms: None,
                    serialization_time_ms: None,
                    gc_collections: None,
                    gc_time_ms: None,
                    flamegraph_path,
                })
            }
            ProfilerHandle::Php(p) => {
                let data = p.stop();
                ProfilingData::Php(PhpProfilingData {
                    flamegraph_path: data.flamegraph_path,
                })
            }
            ProfilerHandle::Node(n) => {
                let data = n.stop();
                ProfilingData::Node(NodeProfilingData {
                    v8_heap_used_mb: data.v8_heap_used_mb,
                    v8_heap_total_mb: data.v8_heap_total_mb,
                    event_loop_lag_ms: data.event_loop_lag_ms,
                    gc_time_ms: data.gc_time_ms,
                    flamegraph_path: data.flamegraph_path,
                })
            }
            ProfilerHandle::Ruby(r) => {
                let data = r.stop();
                ProfilingData::Ruby(RubyProfilingData {
                    gc_count: data.gc_count,
                    gc_time_ms: data.gc_time_ms,
                    heap_allocated_pages: data.heap_allocated_pages,
                    heap_live_slots: data.heap_live_slots,
                    flamegraph_path: data.flamegraph_path,
                })
            }
            ProfilerHandle::Rust(r) => {
                let data = r.stop();
                ProfilingData::Rust(RustProfilingData {
                    heap_allocated_mb: data.heap_allocated_mb,
                    flamegraph_path: data.flamegraph_path,
                })
            }
            ProfilerHandle::RustInApp(path) => {
                let flamegraph_path = path.to_str().and_then(crate::profile::python::wait_for_profile_output);
                ProfilingData::Rust(RustProfilingData {
                    heap_allocated_mb: None,
                    flamegraph_path,
                })
            }
        });

        let throughput_schema = Throughput {
            requests_per_sec: throughput.requests_per_sec,
            bytes_per_sec: throughput.bytes_per_sec,
            total_requests: throughput.total_requests,
            successful_requests: throughput.total_requests - throughput.failed_requests,
            failed_requests: throughput.failed_requests,
            success_rate: throughput.success_rate,
        };

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

    fn create_fixture_from_workload(&self, workload_def: &crate::schema::workload::WorkloadDef) -> Result<Fixture> {
        use crate::fixture::{ExpectedResponse, Handler, Parameters, Request};

        let mut headers = std::collections::HashMap::new();
        if let Some(ref content_type) = workload_def.content_type {
            headers.insert("Content-Type".to_string(), content_type.clone());
        }

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
            files: Vec::new(),
            data: std::collections::HashMap::new(),
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
        let workspace_root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        workspace_root.join("testing_data").join(fixture_file)
    }

    /// Load body data from testing_data fixtures
    fn load_body_from_fixtures(&self, body_file: &str) -> Result<serde_json::Value> {
        let fixture_path = match body_file {
            "json-small.json" => "json_bodies/01_simple_object_success.json",
            "json-medium.json" => "json_bodies/04_nested_object_success.json",
            "json-large.json" => "json_bodies/25_deeply_nested_objects.json",
            "json-very-large.json" => "json_bodies/05_array_of_objects.json",
            _ => body_file,
        };

        let full_path = self.resolve_fixture_path(fixture_path);

        match Fixture::from_file(&full_path) {
            Ok(fixture) => fixture
                .request
                .body
                .ok_or_else(|| Error::InvalidInput(format!("Fixture {} has no body", fixture_path))),
            Err(_) => self.generate_synthetic_body(body_file),
        }
    }

    /// Generate synthetic body data as fallback
    fn generate_synthetic_body(&self, body_file: &str) -> Result<serde_json::Value> {
        if body_file.ends_with(".txt") {
            let urlencoded_str = match body_file {
                "urlencoded-simple.txt" => "name=John+Doe&email=john%40example.com&age=30&subscribe=true",
                "urlencoded-complex.txt" => {
                    "username=testuser&password=secret123&email=test%40example.com&first_name=John&last_name=Doe&age=30&country=US&state=CA&city=San+Francisco&zip=94102&phone=%2B1-555-1234&company=Acme+Corp&job_title=Engineer&subscribe=true&newsletter=weekly&terms_accepted=true&privacy_accepted=true&marketing_consent=false&two_factor_enabled=true"
                }
                _ => return Err(Error::InvalidInput(format!("Unknown body file: {}", body_file))),
            };
            return Ok(serde_json::Value::String(urlencoded_str.to_string()));
        }

        if body_file.ends_with(".bin") {
            let (file_count, total_bytes) = match body_file {
                "multipart-small.bin" => (1, 1024),
                "multipart-medium.bin" => (2, 10240),
                "multipart-large.bin" => (5, 102400),
                _ => return Err(Error::InvalidInput(format!("Unknown body file: {}", body_file))),
            };
            return Ok(serde_json::json!({
                "files_received": file_count,
                "total_bytes": total_bytes
            }));
        }

        let json_str = match body_file {
            "json-small.json" => r#"{"id":12345,"name":"test_item","active":true,"count":42}"#,
            "json-medium.json" => {
                r#"{"id":12345,"name":"test_item","description":"Medium payload","price":99.99,"tags":["tag1","tag2","tag3"]}"#
            }
            "json-large.json" => {
                r#"{"id":12345,"name":"large_item","description":"Large payload with nested data","metadata":{"version":1,"status":"active"},"attributes":{"color":"blue","size":"large"}}"#
            }
            "json-very-large.json" => {
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

        let mut category_map: std::collections::HashMap<String, Vec<&WorkloadResult>> =
            std::collections::HashMap::new();
        for suite in suite_results {
            for workload in &suite.workloads {
                category_map
                    .entry(workload.category.clone())
                    .or_default()
                    .push(workload);
            }
        }

        let category_breakdown: Vec<CategorySummary> = category_map
            .into_iter()
            .map(|(category, workloads)| {
                let avg_rps = workloads
                    .iter()
                    .map(|w| w.results.throughput.requests_per_sec)
                    .sum::<f64>()
                    / workloads.len() as f64;
                let avg_latency =
                    workloads.iter().map(|w| w.results.latency.mean_ms).sum::<f64>() / workloads.len() as f64;

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
        let baseline_json = std::fs::read_to_string(baseline_path)
            .map_err(|e| Error::InvalidInput(format!("Failed to read baseline file: {}", e)))?;

        let baseline: crate::schema::profile::ProfileResult = serde_json::from_str(&baseline_json)
            .map_err(|e| Error::InvalidInput(format!("Failed to parse baseline JSON: {}", e)))?;

        let mut baseline_map: std::collections::HashMap<String, &WorkloadResult> = std::collections::HashMap::new();
        for suite in &baseline.suites {
            for workload in &suite.workloads {
                baseline_map.insert(workload.name.clone(), workload);
            }
        }

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
        for port in 8100..8200 {
            if std::net::TcpListener::bind(("127.0.0.1", port)).is_ok() {
                return port;
            }
        }
        8100
    }

    fn detect_framework_version(&self) -> String {
        if self.detect_language() == "rust" {
            let cargo_toml = self.config.app_dir.join("Cargo.toml");
            if let Ok(contents) = std::fs::read_to_string(cargo_toml) {
                for line in contents.lines() {
                    if line.trim().starts_with("version")
                        && let Some(version) = line.split('=').nth(1)
                    {
                        return version.trim().trim_matches('"').to_string();
                    }
                }
            }
        }
        "0.1.0".to_string()
    }

    fn detect_language(&self) -> String {
        if self.config.framework.contains("wasm") {
            "wasm".to_string()
        } else if self.config.app_dir.join("server.py").exists() {
            "python".to_string()
        } else if self.config.app_dir.join("server.ts").exists() {
            "node".to_string()
        } else if self.config.app_dir.join("server.rb").exists() {
            "ruby".to_string()
        } else if self.config.app_dir.join("server.php").exists() {
            "php".to_string()
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
                if let Ok(output) = std::process::Command::new("python3").arg("--version").output()
                    && let Ok(version) = String::from_utf8(output.stdout)
                {
                    return version.trim().to_string();
                }
                "Python 3.x".to_string()
            }
            "node" => {
                if let Ok(output) = std::process::Command::new("node").arg("--version").output()
                    && let Ok(version) = String::from_utf8(output.stdout)
                {
                    return format!("Node {}", version.trim());
                }
                "Node.js".to_string()
            }
            "wasm" => {
                if let Ok(output) = std::process::Command::new("deno").arg("--version").output()
                    && let Ok(version) = String::from_utf8(output.stdout)
                {
                    return version.lines().next().unwrap_or("Deno").trim().to_string();
                }
                "Deno".to_string()
            }
            "ruby" => {
                if let Ok(output) = std::process::Command::new("ruby").arg("--version").output()
                    && let Ok(version) = String::from_utf8(output.stdout)
                    && let Some(version_part) = version.split_whitespace().nth(1)
                {
                    return format!("Ruby {}", version_part);
                }
                "Ruby".to_string()
            }
            "rust" => {
                if let Ok(output) = std::process::Command::new("rustc").arg("--version").output()
                    && let Ok(version) = String::from_utf8(output.stdout)
                {
                    return version.trim().to_string();
                }
                "rustc".to_string()
            }
            _ => "unknown".to_string(),
        }
    }
}

fn find_latest_file_with_extension(dir: &PathBuf, ext: &str) -> Option<PathBuf> {
    let Ok(read_dir) = std::fs::read_dir(dir) else {
        return None;
    };

    read_dir
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some(ext) {
                let modified = entry.metadata().ok()?.modified().ok()?;
                Some((modified, path))
            } else {
                None
            }
        })
        .max_by_key(|(modified, _)| *modified)
        .map(|(_, path)| path)
}

fn absolutize_path(path: PathBuf) -> PathBuf {
    if path.is_absolute() {
        return path;
    }

    match std::env::current_dir() {
        Ok(cwd) => cwd.join(path),
        Err(_) => path,
    }
}

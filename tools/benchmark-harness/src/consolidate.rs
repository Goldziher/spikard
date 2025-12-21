//! Consolidate profile results across multiple runs.

use crate::analysis::{MetricStats, calculate_stats};
use crate::schema::profile::{ProfileResult, WorkloadResult};
use crate::schema::{FrameworkInfo, workload::Endpoint};
use crate::{Error, Result};
use chrono::Utc;
use glob::glob;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidatedProfileReport {
    pub metadata: ConsolidationMetadata,
    pub frameworks: Vec<FrameworkConsolidation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationMetadata {
    pub generated_at: String,
    pub input_count: usize,
    pub inputs: Vec<ConsolidationInput>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationInput {
    pub path: String,
    pub framework: String,
    pub timestamp: String,
    pub duration_secs: u64,
    pub concurrency: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkConsolidation {
    pub name: String,
    pub language: String,
    pub runtime: String,
    pub variant: Option<String>,
    pub run_count: usize,
    pub avg_requests_per_sec: MetricStats,
    pub avg_latency_ms: MetricStats,
    pub categories: Vec<CategoryConsolidation>,
    pub workloads: Vec<WorkloadConsolidation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryConsolidation {
    pub category: String,
    pub workload_count: usize,
    pub run_count: usize,
    pub avg_requests_per_sec: MetricStats,
    pub avg_latency_ms: MetricStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadConsolidation {
    pub name: String,
    pub description: String,
    pub category: String,
    pub endpoint: Endpoint,
    pub payload_size_bytes: Option<u64>,
    pub run_count: usize,
    pub throughput: ThroughputStats,
    pub latency: LatencyStats,
    pub resources: ResourceStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroughputStats {
    pub requests_per_sec: MetricStats,
    pub bytes_per_sec: MetricStats,
    pub total_requests: MetricStats,
    pub success_rate: MetricStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyStats {
    pub mean_ms: MetricStats,
    pub median_ms: MetricStats,
    pub p90_ms: MetricStats,
    pub p95_ms: MetricStats,
    pub p99_ms: MetricStats,
    pub p999_ms: MetricStats,
    pub min_ms: MetricStats,
    pub max_ms: MetricStats,
    pub stddev_ms: MetricStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceStats {
    pub cpu_avg_percent: MetricStats,
    pub cpu_peak_percent: MetricStats,
    pub cpu_p95_percent: MetricStats,
    pub memory_avg_mb: MetricStats,
    pub memory_peak_mb: MetricStats,
    pub memory_p95_mb: MetricStats,
}

#[derive(Default)]
struct CategoryAccumulator {
    avg_rps: Vec<f64>,
    avg_latency: Vec<f64>,
    workload_count: Option<usize>,
}

struct WorkloadAccumulator {
    description: String,
    category: String,
    endpoint: Endpoint,
    payload_size_bytes: Option<u64>,
    throughput_rps: Vec<f64>,
    throughput_bytes_per_sec: Vec<f64>,
    throughput_total_requests: Vec<f64>,
    throughput_success_rate: Vec<f64>,
    latency_mean_ms: Vec<f64>,
    latency_median_ms: Vec<f64>,
    latency_p90_ms: Vec<f64>,
    latency_p95_ms: Vec<f64>,
    latency_p99_ms: Vec<f64>,
    latency_p999_ms: Vec<f64>,
    latency_min_ms: Vec<f64>,
    latency_max_ms: Vec<f64>,
    latency_stddev_ms: Vec<f64>,
    cpu_avg_percent: Vec<f64>,
    cpu_peak_percent: Vec<f64>,
    cpu_p95_percent: Vec<f64>,
    memory_avg_mb: Vec<f64>,
    memory_peak_mb: Vec<f64>,
    memory_p95_mb: Vec<f64>,
}

impl WorkloadAccumulator {
    fn new(workload: &WorkloadResult) -> Self {
        Self {
            description: workload.description.clone(),
            category: workload.category.clone(),
            endpoint: workload.endpoint.clone(),
            payload_size_bytes: workload.payload_size_bytes,
            throughput_rps: Vec::new(),
            throughput_bytes_per_sec: Vec::new(),
            throughput_total_requests: Vec::new(),
            throughput_success_rate: Vec::new(),
            latency_mean_ms: Vec::new(),
            latency_median_ms: Vec::new(),
            latency_p90_ms: Vec::new(),
            latency_p95_ms: Vec::new(),
            latency_p99_ms: Vec::new(),
            latency_p999_ms: Vec::new(),
            latency_min_ms: Vec::new(),
            latency_max_ms: Vec::new(),
            latency_stddev_ms: Vec::new(),
            cpu_avg_percent: Vec::new(),
            cpu_peak_percent: Vec::new(),
            cpu_p95_percent: Vec::new(),
            memory_avg_mb: Vec::new(),
            memory_peak_mb: Vec::new(),
            memory_p95_mb: Vec::new(),
        }
    }

    fn push_metrics(&mut self, workload: &WorkloadResult) {
        let throughput = &workload.results.throughput;
        let latency = &workload.results.latency;
        let resources = &workload.results.resources;

        self.throughput_rps.push(throughput.requests_per_sec);
        self.throughput_bytes_per_sec.push(throughput.bytes_per_sec);
        self.throughput_total_requests.push(throughput.total_requests as f64);
        self.throughput_success_rate.push(throughput.success_rate);

        self.latency_mean_ms.push(latency.mean_ms);
        self.latency_median_ms.push(latency.median_ms);
        self.latency_p90_ms.push(latency.p90_ms);
        self.latency_p95_ms.push(latency.p95_ms);
        self.latency_p99_ms.push(latency.p99_ms);
        self.latency_p999_ms.push(latency.p999_ms);
        self.latency_min_ms.push(latency.min_ms);
        self.latency_max_ms.push(latency.max_ms);
        self.latency_stddev_ms.push(latency.stddev_ms);

        self.cpu_avg_percent.push(resources.cpu.avg_percent);
        self.cpu_peak_percent.push(resources.cpu.peak_percent);
        self.cpu_p95_percent.push(resources.cpu.p95_percent);
        self.memory_avg_mb.push(resources.memory.avg_mb);
        self.memory_peak_mb.push(resources.memory.peak_mb);
        self.memory_p95_mb.push(resources.memory.p95_mb);
    }
}

struct FrameworkAccumulator {
    info: FrameworkInfo,
    avg_rps: Vec<f64>,
    avg_latency: Vec<f64>,
    categories: BTreeMap<String, CategoryAccumulator>,
    workloads: BTreeMap<String, WorkloadAccumulator>,
    warnings: BTreeSet<String>,
}

impl FrameworkAccumulator {
    fn new(info: FrameworkInfo) -> Self {
        Self {
            info,
            avg_rps: Vec::new(),
            avg_latency: Vec::new(),
            categories: BTreeMap::new(),
            workloads: BTreeMap::new(),
            warnings: BTreeSet::new(),
        }
    }
}

pub fn consolidate_profile_dir(input_dir: &Path, pattern: &str) -> Result<ConsolidatedProfileReport> {
    let pattern = normalize_pattern(input_dir, pattern);
    let mut paths = Vec::new();
    for path in (glob(&pattern).map_err(|e| Error::InvalidInput(format!("Invalid glob pattern: {}", e)))?).flatten() {
        paths.push(path);
    }
    consolidate_profile_paths(&paths)
}

pub fn consolidate_profile_paths(paths: &[PathBuf]) -> Result<ConsolidatedProfileReport> {
    if paths.is_empty() {
        return Err(Error::InvalidInput("No profile.json files found".to_string()));
    }

    let mut inputs = Vec::new();
    let mut warnings = Vec::new();
    let mut frameworks: BTreeMap<String, FrameworkAccumulator> = BTreeMap::new();

    for path in paths {
        let content = match std::fs::read_to_string(path) {
            Ok(value) => value,
            Err(err) => {
                warnings.push(format!("Failed to read {}: {}", path.display(), err));
                continue;
            }
        };

        let profile: ProfileResult = match serde_json::from_str(&content) {
            Ok(value) => value,
            Err(err) => {
                warnings.push(format!("Failed to parse {}: {}", path.display(), err));
                continue;
            }
        };

        inputs.push(ConsolidationInput {
            path: path.display().to_string(),
            framework: profile.framework.name.clone(),
            timestamp: profile.metadata.timestamp.clone(),
            duration_secs: profile.configuration.duration_secs,
            concurrency: profile.configuration.concurrency,
        });

        let key = framework_key(&profile.framework);
        let acc = frameworks
            .entry(key.clone())
            .or_insert_with(|| FrameworkAccumulator::new(profile.framework.clone()));

        if acc.info.runtime != profile.framework.runtime {
            acc.warnings.insert(format!(
                "Framework {} runtime changed ({} -> {})",
                acc.info.name, acc.info.runtime, profile.framework.runtime
            ));
        }
        if acc.info.language != profile.framework.language {
            acc.warnings.insert(format!(
                "Framework {} language changed ({} -> {})",
                acc.info.name, acc.info.language, profile.framework.language
            ));
        }

        acc.avg_rps.push(profile.summary.avg_requests_per_sec);
        acc.avg_latency.push(average_workload_latency(&profile));

        for category in &profile.summary.category_breakdown {
            let entry = acc.categories.entry(category.category.clone()).or_default();
            entry.avg_rps.push(category.avg_requests_per_sec);
            entry.avg_latency.push(category.avg_latency_ms);
            if let Some(existing) = entry.workload_count {
                if existing != category.workload_count {
                    acc.warnings.insert(format!(
                        "Framework {} category {} workload count changed ({} -> {})",
                        acc.info.name, category.category, existing, category.workload_count
                    ));
                }
            } else {
                entry.workload_count = Some(category.workload_count);
            }
        }

        for workload in profile_workloads(&profile) {
            let entry = acc
                .workloads
                .entry(workload.name.clone())
                .or_insert_with(|| WorkloadAccumulator::new(workload));

            if entry.category != workload.category {
                acc.warnings.insert(format!(
                    "Workload {} category changed ({} -> {})",
                    workload.name, entry.category, workload.category
                ));
            }

            if entry.payload_size_bytes != workload.payload_size_bytes {
                acc.warnings.insert(format!(
                    "Workload {} payload size changed ({:?} -> {:?})",
                    workload.name, entry.payload_size_bytes, workload.payload_size_bytes
                ));
            }

            if entry.endpoint.path != workload.endpoint.path || entry.endpoint.method != workload.endpoint.method {
                acc.warnings.insert(format!(
                    "Workload {} endpoint changed ({} {} -> {} {})",
                    workload.name,
                    entry.endpoint.method,
                    entry.endpoint.path,
                    workload.endpoint.method,
                    workload.endpoint.path
                ));
            }

            entry.push_metrics(workload);
        }
    }

    if frameworks.is_empty() {
        return Err(Error::InvalidInput("No valid profile results found".to_string()));
    }

    for acc in frameworks.values() {
        warnings.extend(acc.warnings.iter().cloned());
    }

    let frameworks = frameworks.into_values().map(build_framework_consolidation).collect();

    Ok(ConsolidatedProfileReport {
        metadata: ConsolidationMetadata {
            generated_at: Utc::now().to_rfc3339(),
            input_count: inputs.len(),
            inputs,
            warnings,
        },
        frameworks,
    })
}

fn build_framework_consolidation(acc: FrameworkAccumulator) -> FrameworkConsolidation {
    let categories = acc
        .categories
        .into_iter()
        .map(|(category, data)| CategoryConsolidation {
            category,
            workload_count: data.workload_count.unwrap_or_default(),
            run_count: data.avg_rps.len(),
            avg_requests_per_sec: calculate_stats(&data.avg_rps),
            avg_latency_ms: calculate_stats(&data.avg_latency),
        })
        .collect();

    let workloads = acc
        .workloads
        .into_iter()
        .map(|(name, data)| WorkloadConsolidation {
            name,
            description: data.description,
            category: data.category,
            endpoint: data.endpoint,
            payload_size_bytes: data.payload_size_bytes,
            run_count: data.throughput_rps.len(),
            throughput: ThroughputStats {
                requests_per_sec: calculate_stats(&data.throughput_rps),
                bytes_per_sec: calculate_stats(&data.throughput_bytes_per_sec),
                total_requests: calculate_stats(&data.throughput_total_requests),
                success_rate: calculate_stats(&data.throughput_success_rate),
            },
            latency: LatencyStats {
                mean_ms: calculate_stats(&data.latency_mean_ms),
                median_ms: calculate_stats(&data.latency_median_ms),
                p90_ms: calculate_stats(&data.latency_p90_ms),
                p95_ms: calculate_stats(&data.latency_p95_ms),
                p99_ms: calculate_stats(&data.latency_p99_ms),
                p999_ms: calculate_stats(&data.latency_p999_ms),
                min_ms: calculate_stats(&data.latency_min_ms),
                max_ms: calculate_stats(&data.latency_max_ms),
                stddev_ms: calculate_stats(&data.latency_stddev_ms),
            },
            resources: ResourceStats {
                cpu_avg_percent: calculate_stats(&data.cpu_avg_percent),
                cpu_peak_percent: calculate_stats(&data.cpu_peak_percent),
                cpu_p95_percent: calculate_stats(&data.cpu_p95_percent),
                memory_avg_mb: calculate_stats(&data.memory_avg_mb),
                memory_peak_mb: calculate_stats(&data.memory_peak_mb),
                memory_p95_mb: calculate_stats(&data.memory_p95_mb),
            },
        })
        .collect();

    FrameworkConsolidation {
        name: acc.info.name,
        language: acc.info.language,
        runtime: acc.info.runtime,
        variant: acc.info.variant,
        run_count: acc.avg_rps.len(),
        avg_requests_per_sec: calculate_stats(&acc.avg_rps),
        avg_latency_ms: calculate_stats(&acc.avg_latency),
        categories,
        workloads,
    }
}

fn normalize_pattern(input_dir: &Path, pattern: &str) -> String {
    if pattern.contains(std::path::MAIN_SEPARATOR) || pattern.contains('/') {
        pattern.to_string()
    } else {
        format!("{}/{}", input_dir.display(), pattern)
    }
}

fn average_workload_latency(profile: &ProfileResult) -> f64 {
    let mut total = 0.0;
    let mut count = 0usize;
    for workload in profile_workloads(profile) {
        total += workload.results.latency.mean_ms;
        count += 1;
    }
    if count > 0 { total / count as f64 } else { 0.0 }
}

fn profile_workloads(profile: &ProfileResult) -> Vec<&WorkloadResult> {
    profile.suites.iter().flat_map(|suite| suite.workloads.iter()).collect()
}

fn framework_key(info: &FrameworkInfo) -> String {
    if let Some(variant) = &info.variant {
        format!("{}::{}", info.name, variant)
    } else {
        info.name.clone()
    }
}

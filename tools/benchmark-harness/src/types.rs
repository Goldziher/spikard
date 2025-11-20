//! Benchmark result types and metrics

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Complete benchmark result for a workload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Framework name (e.g., "spikard-python", "fastapi")
    pub framework: String,

    /// Workload name (e.g., "simple", "comprehensive")
    pub workload: String,

    /// Variant name (e.g., "sync", "async") - optional for backwards compatibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,

    /// Timestamp when benchmark started
    pub timestamp: DateTime<Utc>,

    /// Duration of the benchmark
    pub duration_secs: u64,

    /// Concurrency level (number of concurrent connections)
    pub concurrency: usize,

    /// Startup/initialization metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startup: Option<StartupMetrics>,

    /// Throughput metrics
    pub throughput: ThroughputMetrics,

    /// Latency percentiles
    pub latency: LatencyMetrics,

    /// Resource usage
    pub resources: ResourceMetrics,

    /// Per-route-type breakdown (optional)
    #[serde(default)]
    pub route_types: Vec<RouteTypeMetrics>,

    /// Error handling metrics (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_metrics: Option<ErrorMetrics>,

    /// Serialization overhead metrics (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serialization: Option<SerializationMetrics>,

    /// Per-pattern breakdown (deprecated - use route_types instead)
    #[serde(default)]
    pub patterns: Vec<PatternMetrics>,

    /// Whether the benchmark completed successfully
    pub success: bool,

    /// Error message if failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Throughput metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroughputMetrics {
    /// Total number of requests completed
    pub total_requests: u64,

    /// Requests per second
    pub requests_per_sec: f64,

    /// Bytes per second (response bodies)
    pub bytes_per_sec: f64,

    /// Number of failed requests
    pub failed_requests: u64,

    /// Success rate (0.0-1.0)
    pub success_rate: f64,
}

/// Latency metrics in milliseconds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyMetrics {
    /// Mean latency
    pub mean_ms: f64,

    /// 50th percentile (median)
    pub p50_ms: f64,

    /// 90th percentile
    pub p90_ms: f64,

    /// 95th percentile
    pub p95_ms: f64,

    /// 99th percentile
    pub p99_ms: f64,

    /// 99.9th percentile
    pub p999_ms: f64,

    /// Maximum latency observed
    pub max_ms: f64,

    /// Minimum latency observed
    pub min_ms: f64,

    /// Standard deviation
    pub stddev_ms: f64,
}

/// Resource usage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    /// Average memory usage in MB
    pub avg_memory_mb: f64,

    /// Peak memory usage in MB
    pub peak_memory_mb: f64,

    /// 50th percentile memory in MB
    pub p50_memory_mb: f64,

    /// 95th percentile memory in MB
    pub p95_memory_mb: f64,

    /// 99th percentile memory in MB
    pub p99_memory_mb: f64,

    /// Average CPU usage percentage (0-100)
    pub avg_cpu_percent: f64,

    /// Peak CPU usage percentage (0-100)
    pub peak_cpu_percent: f64,
}

/// Metrics for a specific request pattern (deprecated - use RouteTypeMetrics)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternMetrics {
    /// Pattern name (e.g., "simple_query", "json_body")
    pub pattern: String,

    /// Number of requests for this pattern
    pub count: u64,

    /// Average latency for this pattern
    pub avg_latency_ms: f64,

    /// p99 latency for this pattern
    pub p99_latency_ms: f64,

    /// Success rate for this pattern
    pub success_rate: f64,
}

/// Startup and initialization metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupMetrics {
    /// Time to spawn the process (ms)
    pub process_spawn_ms: f64,

    /// Time from spawn to first successful health check (ms)
    pub time_to_first_response_ms: f64,

    /// Memory footprint after initialization (MB)
    pub initialization_memory_mb: f64,

    /// Total startup time (spawn + health check) (ms)
    pub total_startup_ms: f64,
}

/// Route type classification for benchmarking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteType {
    /// Simple GET with no parameters
    GetSimple,

    /// GET with path parameters (e.g., /users/{id})
    GetPathParams,

    /// GET with query parameters (e.g., /search?q=foo)
    GetQueryParams,

    /// GET with both path and query parameters
    GetBoth,

    /// POST with simple JSON body (flat structure)
    PostJsonSimple,

    /// POST with nested JSON objects (3+ levels deep)
    PostJsonNested,

    /// POST with large payload (>10KB)
    PostJsonLarge,

    /// POST with validation constraints (min_length, gt, lt, etc.)
    PostValidated,

    /// POST with multipart/form-data
    PostMultipart,

    /// PUT request with JSON body
    PutJson,

    /// PATCH request with JSON body
    PatchJson,

    /// DELETE request
    Delete,

    /// Custom/uncategorized route type
    Other,
}

impl std::fmt::Display for RouteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RouteType::GetSimple => write!(f, "GET (simple)"),
            RouteType::GetPathParams => write!(f, "GET (path params)"),
            RouteType::GetQueryParams => write!(f, "GET (query params)"),
            RouteType::GetBoth => write!(f, "GET (path + query)"),
            RouteType::PostJsonSimple => write!(f, "POST (simple JSON)"),
            RouteType::PostJsonNested => write!(f, "POST (nested JSON)"),
            RouteType::PostJsonLarge => write!(f, "POST (large payload)"),
            RouteType::PostValidated => write!(f, "POST (validated)"),
            RouteType::PostMultipart => write!(f, "POST (multipart)"),
            RouteType::PutJson => write!(f, "PUT (JSON)"),
            RouteType::PatchJson => write!(f, "PATCH (JSON)"),
            RouteType::Delete => write!(f, "DELETE"),
            RouteType::Other => write!(f, "Other"),
        }
    }
}

/// Metrics for a specific route type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteTypeMetrics {
    /// Route type classification
    pub route_type: RouteType,

    /// Number of requests for this route type
    pub sample_count: u64,

    /// Requests per second for this route type
    pub throughput_rps: f64,

    /// Full latency breakdown for this route type
    pub latency: LatencyMetrics,

    /// Success rate for this route type (0.0-1.0)
    pub success_rate: f64,

    /// Average memory delta per request (MB)
    /// Measures memory impact of processing this route type
    pub avg_memory_delta_mb: f64,
}

/// Error handling performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMetrics {
    /// p99 latency for validation errors (400 responses) in ms
    pub validation_error_p99_ms: f64,

    /// p99 latency for not found errors (404 responses) in ms
    pub not_found_p99_ms: f64,

    /// p99 latency for server errors (500 responses) in ms
    pub server_error_p99_ms: f64,

    /// Throughput when serving error responses (req/s)
    pub error_throughput_rps: f64,

    /// Memory impact of error handling (MB)
    pub error_memory_impact_mb: f64,

    /// Total error count
    pub total_errors: u64,

    /// Error rate (errors / total requests)
    pub error_rate: f64,
}

/// Serialization and validation overhead metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializationMetrics {
    /// Average time parsing request JSON (ms)
    pub json_parse_overhead_ms: f64,

    /// Average time serializing response JSON (ms)
    pub json_serialize_overhead_ms: f64,

    /// Average time validating request schemas (ms)
    pub validation_overhead_ms: f64,

    /// Total serialization overhead as % of total latency
    pub total_overhead_pct: f64,

    /// Number of samples used for calculation
    pub sample_count: u64,
}

/// Result of a streaming benchmark (WebSocket/SSE).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingBenchmarkResult {
    pub framework: String,
    pub protocol: StreamingProtocol,
    pub channel: String,
    pub duration_secs: u64,
    pub connections: usize,
    pub timestamp: DateTime<Utc>,
    pub resources: ResourceMetrics,
    pub metrics: StreamingMetrics,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<StreamingTranscript>,
}

/// Streaming workload metrics.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StreamingMetrics {
    pub connections_established: usize,
    pub messages_sent: u64,
    pub responses_received: u64,
    pub events_received: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency: Option<StreamingLatencyMetrics>,
    pub errors: u64,
}

/// Minimal latency summary for streaming workloads.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingLatencyMetrics {
    pub average_ms: f64,
    pub max_ms: f64,
    pub samples: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StreamingTranscript {
    pub sent: Vec<serde_json::Value>,
    pub received: Vec<serde_json::Value>,
}

/// Supported streaming protocols.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StreamingProtocol {
    #[serde(alias = "ws", alias = "websocket")]
    WebSocket,
    #[serde(alias = "sse", alias = "server-sent-events")]
    Sse,
}

impl std::fmt::Display for StreamingProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StreamingProtocol::WebSocket => write!(f, "websocket"),
            StreamingProtocol::Sse => write!(f, "sse"),
        }
    }
}

/// Raw output from oha load generator
#[derive(Debug, Clone, Deserialize)]
pub struct OhaOutput {
    pub summary: OhaSummary,

    #[serde(rename = "latencyPercentiles")]
    pub latency_percentiles: LatencyPercentiles,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OhaSummary {
    #[serde(rename = "successRate", default)]
    pub success_rate: Option<f64>,

    #[serde(default)]
    pub total: Option<f64>,

    pub slowest: Option<f64>,

    pub fastest: Option<f64>,

    pub average: Option<f64>,

    #[serde(rename = "requestsPerSec", default)]
    pub requests_per_sec: Option<f64>,

    #[serde(rename = "totalData", default)]
    pub total_data: Option<f64>,

    #[serde(rename = "sizePerRequest")]
    pub size_per_request: Option<f64>,

    #[serde(rename = "sizePerSec", default)]
    pub size_per_sec: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LatencyPercentiles {
    pub p10: Option<f64>,
    pub p25: Option<f64>,
    pub p50: Option<f64>,
    pub p75: Option<f64>,
    pub p90: Option<f64>,
    pub p95: Option<f64>,
    pub p99: Option<f64>,
    #[serde(rename = "p99.9")]
    pub p99_9: Option<f64>,
    #[serde(rename = "p99.99")]
    pub p99_99: Option<f64>,
}

impl From<OhaOutput> for LatencyMetrics {
    fn from(oha: OhaOutput) -> Self {
        let s = &oha.summary;
        let p = &oha.latency_percentiles;

        let to_ms = |opt: Option<f64>| opt.map(|secs| secs * 1000.0).unwrap_or(0.0);

        Self {
            mean_ms: to_ms(s.average),
            p50_ms: to_ms(p.p50),
            p90_ms: to_ms(p.p90),
            p95_ms: to_ms(p.p95),
            p99_ms: to_ms(p.p99),
            p999_ms: to_ms(p.p99_9),
            max_ms: to_ms(s.slowest),
            min_ms: to_ms(s.fastest),
            stddev_ms: 0.0,
        }
    }
}

/// Helper to convert Duration to milliseconds
pub fn duration_to_ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

/// Helper to convert bytes to megabytes
pub fn bytes_to_mb(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0)
}

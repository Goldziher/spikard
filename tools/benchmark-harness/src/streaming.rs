//! Streaming (WebSocket/SSE) benchmark runner.
//!
//! Provides throughput/latency measurements for long-lived transports that are
//! not covered by the HTTP load generators used for the core benchmarks.

use crate::error::{Error, Result};
use crate::monitor::ResourceMonitor;
use crate::server::{ServerConfig, find_available_port, start_server};
use crate::types::{
    ResourceMetrics, StreamingBenchmarkResult, StreamingLatencyMetrics, StreamingMetrics, StreamingProtocol,
    StreamingTranscript,
};
use chrono::Utc;
use futures_util::{SinkExt, StreamExt, pin_mut};
use serde::Deserialize;
use serde_json;
use std::path::{Path, PathBuf};
use tokio::task::JoinSet;
use tokio::time::{Duration, Instant, sleep_until, timeout};
use tokio_tungstenite::{connect_async, tungstenite::Message};

/// Configuration for the streaming benchmark runner.
pub struct StreamingRunnerConfig {
    pub framework: String,
    pub app_dir: PathBuf,
    pub duration_secs: u64,
    pub connections: usize,
    pub warmup_secs: u64,
    pub variant: Option<String>,
}

/// Runner handling WebSocket/SSE benchmarks.
pub struct StreamingBenchmarkRunner {
    config: StreamingRunnerConfig,
}

impl StreamingBenchmarkRunner {
    pub fn new(config: StreamingRunnerConfig) -> Self {
        Self { config }
    }

    /// Execute the benchmark using the provided fixture.
    pub async fn run(&self, fixture: &StreamingFixture) -> Result<StreamingBenchmarkResult> {
        let port = find_available_port(9000).ok_or_else(|| Error::ServerStartFailed("No available ports".into()))?;

        let server_config = ServerConfig {
            framework: Some(self.config.framework.clone()),
            port,
            app_dir: self.config.app_dir.clone(),
            variant: self.config.variant.clone(),
            env: Vec::new(),
            start_cmd_override: None,
        };

        let server = start_server(server_config).await?;
        let pid = server.pid();
        let base_http_url = server.base_url.clone();
        let timestamp = Utc::now();

        if self.config.warmup_secs > 0 {
            let warmup_duration = Duration::from_secs(self.config.warmup_secs);
            let _ = run_streaming_load(
                fixture,
                &base_http_url,
                self.config.connections.clamp(1, 5),
                warmup_duration,
            )
            .await;
        }

        let monitor = ResourceMonitor::new(pid);
        let monitor_handle = monitor.start_monitoring(100);

        let stats = run_streaming_load(
            fixture,
            &base_http_url,
            self.config.connections,
            Duration::from_secs(self.config.duration_secs),
        )
        .await;

        let resource_metrics = monitor_handle.stop().await.calculate_metrics();
        server.kill()?;

        match stats {
            Ok(task_stats) => {
                let latency = if task_stats.latency_count > 0 {
                    Some(StreamingLatencyMetrics {
                        average_ms: task_stats.latency_total_ms / task_stats.latency_count as f64,
                        max_ms: task_stats.latency_max_ms,
                        samples: task_stats.latency_count,
                    })
                } else {
                    None
                };

                let metrics = StreamingMetrics {
                    connections_established: task_stats.connections_established,
                    messages_sent: task_stats.messages_sent,
                    responses_received: task_stats.responses_received,
                    events_received: task_stats.events_received,
                    latency,
                    errors: task_stats.errors,
                };

                Ok(StreamingBenchmarkResult {
                    framework: self.config.framework.clone(),
                    protocol: fixture.protocol,
                    channel: fixture.channel.clone(),
                    duration_secs: self.config.duration_secs,
                    connections: self.config.connections,
                    timestamp,
                    resources: resource_metrics,
                    metrics,
                    success: task_stats.errors == 0 && task_stats.connections_established > 0,
                    error: if task_stats.errors == 0 {
                        None
                    } else {
                        Some(format!("{} streaming errors", task_stats.errors))
                    },
                    transcript: task_stats.transcript,
                })
            }
            Err(err) => {
                let zero_resources = ResourceMetrics {
                    avg_memory_mb: 0.0,
                    peak_memory_mb: 0.0,
                    p50_memory_mb: 0.0,
                    p95_memory_mb: 0.0,
                    p99_memory_mb: 0.0,
                    avg_cpu_percent: 0.0,
                    peak_cpu_percent: 0.0,
                };

                Ok(StreamingBenchmarkResult {
                    framework: self.config.framework.clone(),
                    protocol: fixture.protocol,
                    channel: fixture.channel.clone(),
                    duration_secs: self.config.duration_secs,
                    connections: self.config.connections,
                    timestamp,
                    resources: zero_resources,
                    metrics: StreamingMetrics::default(),
                    success: false,
                    error: Some(err.to_string()),
                    transcript: None,
                })
            }
        }
    }
}

/// Streaming fixture derived from AsyncAPI testing_data artifacts.
#[derive(Debug, Clone, Deserialize)]
pub struct StreamingFixture {
    pub name: String,
    pub channel: String,
    pub description: Option<String>,
    pub protocol: StreamingProtocol,
    #[serde(default)]
    pub examples: Vec<serde_json::Value>,
}

impl StreamingFixture {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let data = std::fs::read_to_string(&path)?;
        let fixture: Self = serde_json::from_str(&data).map_err(|e| Error::InvalidFixture {
            path: path.as_ref().to_path_buf(),
            reason: format!("Failed to parse streaming fixture: {}", e),
        })?;
        Ok(fixture)
    }

    /// Example JSON payload for WebSocket benchmarks.
    pub fn example_payload(&self) -> String {
        self.examples
            .first()
            .and_then(|val| serde_json::to_string(val).ok())
            .unwrap_or_else(|| "{}".to_string())
    }
}

/// Aggregate stats returned by worker tasks.
#[derive(Default)]
struct StreamingTaskStats {
    connections_established: usize,
    messages_sent: u64,
    responses_received: u64,
    events_received: u64,
    latency_total_ms: f64,
    latency_max_ms: f64,
    latency_count: u64,
    errors: u64,
    transcript: Option<StreamingTranscript>,
}

async fn run_streaming_load(
    fixture: &StreamingFixture,
    base_http_url: &str,
    connections: usize,
    duration: Duration,
) -> Result<StreamingTaskStats> {
    match fixture.protocol {
        StreamingProtocol::WebSocket => run_websocket_load(fixture, base_http_url, connections, duration).await,
        StreamingProtocol::Sse => run_sse_load(fixture, base_http_url, connections, duration).await,
    }
}

async fn run_websocket_load(
    fixture: &StreamingFixture,
    base_http_url: &str,
    connections: usize,
    duration: Duration,
) -> Result<StreamingTaskStats> {
    let mut join_set = JoinSet::new();
    let payload = fixture.example_payload();
    let ws_base = http_to_ws(base_http_url);
    let url = format!("{}{}", ws_base, fixture.channel);
    let deadline = Instant::now() + duration;

    for idx in 0..connections {
        let uri = url.clone();
        let payload = payload.clone();
        let capture = idx == 0;
        join_set.spawn(async move { websocket_worker(uri, payload, deadline, capture).await });
    }

    aggregate_stats(join_set).await
}

async fn run_sse_load(
    fixture: &StreamingFixture,
    base_http_url: &str,
    connections: usize,
    duration: Duration,
) -> Result<StreamingTaskStats> {
    let mut join_set = JoinSet::new();
    let url = format!("{}{}", base_http_url, fixture.channel);
    let deadline = Instant::now() + duration;

    for idx in 0..connections {
        let uri = url.clone();
        let capture = idx == 0;
        join_set.spawn(async move { sse_worker(uri, deadline, capture).await });
    }

    aggregate_stats(join_set).await
}

async fn aggregate_stats(mut join_set: JoinSet<StreamingTaskStats>) -> Result<StreamingTaskStats> {
    let mut aggregate = StreamingTaskStats::default();

    while let Some(res) = join_set.join_next().await {
        match res {
            Ok(stats) => {
                aggregate.connections_established += stats.connections_established;
                aggregate.messages_sent += stats.messages_sent;
                aggregate.responses_received += stats.responses_received;
                aggregate.events_received += stats.events_received;
                aggregate.latency_total_ms += stats.latency_total_ms;
                aggregate.latency_max_ms = aggregate.latency_max_ms.max(stats.latency_max_ms);
                aggregate.latency_count += stats.latency_count;
                aggregate.errors += stats.errors;
                if aggregate.transcript.is_none() {
                    aggregate.transcript = stats.transcript;
                }
            }
            Err(e) => {
                aggregate.errors += 1;
                eprintln!("Streaming worker failed: {}", e);
            }
        }
    }

    Ok(aggregate)
}

async fn websocket_worker(uri: String, payload: String, deadline: Instant, capture: bool) -> StreamingTaskStats {
    let mut stats = StreamingTaskStats::default();
    let mut transcript = capture.then(StreamingTranscript::default);

    match connect_async(&uri).await {
        Ok((mut ws_stream, _)) => {
            stats.connections_established = 1;

            while Instant::now() < deadline {
                if ws_stream.send(Message::Text(payload.clone().into())).await.is_err() {
                    stats.errors += 1;
                    break;
                }
                stats.messages_sent += 1;
                let send_start = Instant::now();
                if let Some(record) = transcript.as_mut()
                    && let Ok(value) = serde_json::from_str::<serde_json::Value>(&payload)
                {
                    record.sent.push(value);
                }

                match timeout(Duration::from_secs(2), ws_stream.next()).await {
                    Ok(Some(Ok(Message::Text(text)))) => {
                        stats.responses_received += 1;
                        let elapsed = send_start.elapsed().as_secs_f64() * 1000.0;
                        stats.latency_total_ms += elapsed;
                        stats.latency_count += 1;
                        stats.latency_max_ms = stats.latency_max_ms.max(elapsed);
                        if let Some(record) = transcript.as_mut()
                            && let Ok(value) = serde_json::from_str::<serde_json::Value>(&text)
                        {
                            record.received.push(value);
                        }
                    }
                    Ok(Some(Ok(Message::Binary(_)))) => {
                        stats.responses_received += 1;
                    }
                    Ok(Some(Ok(Message::Frame(_)))) => {}
                    Ok(Some(Ok(Message::Ping(_)))) | Ok(Some(Ok(Message::Pong(_)))) => {}
                    Ok(Some(Ok(Message::Close(_)))) => break,
                    Ok(Some(Err(_))) => {
                        stats.errors += 1;
                        break;
                    }
                    Ok(None) => break,
                    Err(_) => {}
                }
            }

            let _ = ws_stream.close(None).await;
        }
        Err(e) => {
            stats.errors += 1;
            eprintln!("WebSocket connection failed: {}", e);
        }
    }

    stats.transcript = transcript;
    stats
}

async fn sse_worker(uri: String, deadline: Instant, capture: bool) -> StreamingTaskStats {
    let mut stats = StreamingTaskStats::default();
    let client = reqwest::Client::new();
    let mut transcript = capture.then(StreamingTranscript::default);

    match client.get(&uri).header("accept", "text/event-stream").send().await {
        Ok(response) => {
            stats.connections_established = 1;
            let stream = response.bytes_stream();
            pin_mut!(stream);
            let mut buffer = Vec::new();

            loop {
                tokio::select! {
                    chunk = stream.next() => {
                        match chunk {
                            Some(Ok(bytes)) => {
                                buffer.extend_from_slice(&bytes);
                                stats.events_received += drain_sse_events(&mut buffer, transcript.as_mut());
                            }
                            Some(Err(_)) => {
                                stats.errors += 1;
                                break;
                            }
                            None => break,
                        }
                    }
                    _ = sleep_until(deadline) => break,
                }
            }
        }
        Err(e) => {
            stats.errors += 1;
            eprintln!("SSE request failed: {}", e);
        }
    }

    stats.transcript = transcript;
    stats
}

fn drain_sse_events(buffer: &mut Vec<u8>, mut transcript: Option<&mut StreamingTranscript>) -> u64 {
    let mut count = 0;
    while let Some(idx) = buffer.windows(2).position(|w| w == b"\n\n") {
        let frame = buffer[..idx].to_vec();
        buffer.drain(..idx + 2);
        count += 1;
        if let Some(record) = transcript.as_mut()
            && let Ok(text) = std::str::from_utf8(&frame)
            && let Some(data_line) = text.lines().find(|line| line.starts_with("data:"))
        {
            let payload = data_line.trim_start_matches("data:").trim();
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(payload) {
                record.received.push(value);
            }
        }
    }
    count
}

fn http_to_ws(base: &str) -> String {
    if let Some(rest) = base.strip_prefix("https://") {
        format!("wss://{}", rest)
    } else if let Some(rest) = base.strip_prefix("http://") {
        format!("ws://{}", rest)
    } else {
        base.to_string()
    }
}

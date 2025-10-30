//! Configuration data structures

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Root configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Configuration version
    pub version: String,

    /// Service name
    pub name: String,

    /// Service description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Runtime configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<RuntimeConfig>,

    /// HTTP routes and middleware
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<HttpConfig>,

    /// gRPC services
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc: Option<GrpcConfig>,

    /// Queue consumers and producers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queues: Option<QueuesConfig>,

    /// CloudEvents subscriptions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudevents: Option<CloudEventsConfig>,

    /// Reusable schemas
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<IndexMap<String, Value>>,

    /// OpenAPI metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openapi: Option<OpenApiConfig>,
}

/// Runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub workers: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
}

/// HTTP configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    pub routes: Vec<HttpRoute>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub middleware: Option<Vec<MiddlewareConfig>>,
}

/// HTTP route definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRoute {
    pub path: String,
    pub method: String,
    pub handler: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<RouteParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<RequestConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<ResponseConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ErrorResponse>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub middleware: Option<Vec<MiddlewareConfig>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing: Option<TracingConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<MetricsConfig>,
}

/// Route parameters (path, query, headers, cookies)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<IndexMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<IndexMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<IndexMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<IndexMap<String, Value>>,
}

/// Request configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    pub body: Value,
}

/// Response configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseConfig {
    pub status: u16,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<IndexMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Value>,
}

/// Error response specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub body: Value,
}

/// Middleware configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareConfig {
    #[serde(rename = "type")]
    pub middleware_type: String,

    #[serde(flatten)]
    pub config: IndexMap<String, Value>,
}

/// Tracing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfig {
    pub enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<f64>,
}

/// Metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    pub enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<IndexMap<String, String>>,
}

/// gRPC configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcConfig {
    pub services: Vec<GrpcService>,
}

/// gRPC service definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcService {
    pub name: String,
    pub proto: String,
    pub handlers: IndexMap<String, String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interceptors: Option<Vec<MiddlewareConfig>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming: Option<IndexMap<String, StreamingConfig>>,
}

/// Streaming configuration for gRPC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingConfig {
    #[serde(rename = "type")]
    pub stream_type: String,
    pub handler: String,
}

/// Queues configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuesConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumers: Option<Vec<QueueConsumer>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub producers: Option<Vec<QueueProducer>>,
}

/// Queue consumer definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConsumer {
    pub name: String,

    #[serde(rename = "type")]
    pub queue_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,

    pub handler: String,

    #[serde(flatten)]
    pub config: IndexMap<String, Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_schema: Option<Value>,
}

/// Queue producer definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueProducer {
    pub name: String,

    #[serde(rename = "type")]
    pub queue_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,

    #[serde(flatten)]
    pub config: IndexMap<String, Value>,
}

/// CloudEvents configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudEventsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<CloudEventSubscription>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub publishers: Option<Vec<CloudEventPublisher>>,
}

/// CloudEvent subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudEventSubscription {
    #[serde(rename = "type")]
    pub event_type: String,

    pub version: String,
    pub handler: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_schema: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<IndexMap<String, String>>,
}

/// CloudEvent publisher
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudEventPublisher {
    pub name: String,

    #[serde(rename = "type")]
    pub publisher_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<CloudEventType>>,
}

/// CloudEvent type definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudEventType {
    #[serde(rename = "type")]
    pub event_type: String,

    pub version: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Value>,
}

/// OpenAPI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiConfig {
    pub info: OpenApiInfo,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<OpenApiServer>>,
}

/// OpenAPI info section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiInfo {
    pub title: String,
    pub version: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<IndexMap<String, String>>,
}

/// OpenAPI server definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiServer {
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

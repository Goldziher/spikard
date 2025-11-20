//! Spikard HTTP Server
//!
//! Pure Rust HTTP server with language-agnostic handler trait.
//! Language bindings (Python, Node, WASM) implement the Handler trait.

pub mod auth;
pub mod background;
pub mod bindings;
pub mod body_metadata;
pub mod cors;
pub mod debug;
pub mod handler_response;
pub mod handler_trait;
pub mod lifecycle;
pub mod middleware;
pub mod openapi;
pub mod parameters;
pub mod problem;
pub mod query_parser;
pub mod response;
pub mod router;
pub mod schema_registry;
pub mod server;
pub mod sse;
pub mod testing;
pub mod type_hints;
pub mod validation;
pub mod websocket;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod handler_trait_tests;

pub use auth::{Claims, api_key_auth_middleware, jwt_auth_middleware};
pub use background::{
    BackgroundHandle, BackgroundJobError, BackgroundJobMetadata, BackgroundRuntime, BackgroundSpawnError,
    BackgroundTaskConfig,
};
pub use body_metadata::ResponseBodySize;
pub use handler_response::HandlerResponse;
pub use handler_trait::{Handler, HandlerResult, RequestData, ValidatedParams};
pub use lifecycle::{HookResult, LifecycleHook, LifecycleHooks, LifecycleHooksBuilder, request_hook, response_hook};
pub use openapi::{ContactInfo, LicenseInfo, OpenApiConfig, SecuritySchemeInfo, ServerInfo};
pub use parameters::ParameterValidator;
pub use problem::{CONTENT_TYPE_PROBLEM_JSON, ProblemDetails};
pub use response::Response;
pub use router::{Route, RouteHandler, Router};
pub use schema_registry::SchemaRegistry;
pub use server::Server;
pub use spikard_core::{CompressionConfig, CorsConfig, Method, RateLimitConfig, RouteMetadata};
pub use sse::{SseEvent, SseEventProducer, SseState, sse_handler};
pub use testing::{ResponseSnapshot, SnapshotError, snapshot_response};
pub use validation::SchemaValidator;
pub use websocket::{WebSocketHandler, WebSocketState, websocket_handler};

/// JWT authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    /// Secret key for JWT verification
    pub secret: String,
    /// Required algorithm (HS256, HS384, HS512, RS256, etc.)
    #[serde(default = "default_jwt_algorithm")]
    pub algorithm: String,
    /// Required audience claim
    pub audience: Option<Vec<String>>,
    /// Required issuer claim
    pub issuer: Option<String>,
    /// Leeway for expiration checks (seconds)
    #[serde(default)]
    pub leeway: u64,
}

fn default_jwt_algorithm() -> String {
    "HS256".to_string()
}

/// API Key authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyConfig {
    /// Valid API keys
    pub keys: Vec<String>,
    /// Header name to check (e.g., "X-API-Key")
    #[serde(default = "default_api_key_header")]
    pub header_name: String,
}

fn default_api_key_header() -> String {
    "X-API-Key".to_string()
}

/// Static file serving configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticFilesConfig {
    /// Directory path to serve
    pub directory: String,
    /// URL path prefix (e.g., "/static")
    pub route_prefix: String,
    /// Fallback to index.html for directories
    #[serde(default = "default_true")]
    pub index_file: bool,
    /// Cache-Control header value
    pub cache_control: Option<String>,
}

/// Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Host to bind to
    pub host: String,
    /// Port to bind to
    pub port: u16,
    /// Number of worker threads (unused with tokio)
    pub workers: usize,

    /// Enable request ID generation and propagation
    pub enable_request_id: bool,
    /// Maximum request body size in bytes (None = unlimited, not recommended)
    pub max_body_size: Option<usize>,
    /// Request timeout in seconds (None = no timeout)
    pub request_timeout: Option<u64>,
    /// Enable compression middleware
    pub compression: Option<CompressionConfig>,
    /// Enable rate limiting
    pub rate_limit: Option<RateLimitConfig>,
    /// JWT authentication configuration
    pub jwt_auth: Option<JwtConfig>,
    /// API Key authentication configuration
    pub api_key_auth: Option<ApiKeyConfig>,
    /// Static file serving configuration
    pub static_files: Vec<StaticFilesConfig>,
    /// Enable graceful shutdown on SIGTERM/SIGINT
    pub graceful_shutdown: bool,
    /// Graceful shutdown timeout (seconds)
    pub shutdown_timeout: u64,
    /// OpenAPI documentation configuration
    pub openapi: Option<crate::openapi::OpenApiConfig>,
    /// Lifecycle hooks for request/response processing
    pub lifecycle_hooks: Option<std::sync::Arc<LifecycleHooks>>,
    /// Background task executor configuration
    pub background_tasks: BackgroundTaskConfig,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8000,
            workers: 1,
            enable_request_id: true,
            max_body_size: Some(10 * 1024 * 1024), 
            request_timeout: Some(30),             
            compression: Some(CompressionConfig::default()),
            rate_limit: None, 
            jwt_auth: None,
            api_key_auth: None,
            static_files: Vec::new(),
            graceful_shutdown: true,
            shutdown_timeout: 30,
            openapi: None,
            lifecycle_hooks: None, 
            background_tasks: BackgroundTaskConfig::default(),
        }
    }
}

const fn default_true() -> bool {
    true
}

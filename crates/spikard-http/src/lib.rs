//! Spikard HTTP Server
//!
//! Pure Rust HTTP server with language-agnostic handler trait.
//! Language bindings (Python, Node, WASM) implement the Handler trait.

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod auth;
pub mod cors;
pub mod debug;
pub mod handler_trait;
pub mod middleware;
pub mod parameters;
pub mod problem;
pub mod query_parser;
pub mod response;
pub mod router;
pub mod schema_registry;
pub mod server;
pub mod type_hints;
pub mod validation;

#[cfg(test)]
mod handler_trait_tests;

pub use auth::{Claims, api_key_auth_middleware, jwt_auth_middleware};
pub use handler_trait::{Handler, HandlerResult, RequestData, ValidatedParams};
pub use parameters::ParameterValidator;
pub use problem::{CONTENT_TYPE_PROBLEM_JSON, ProblemDetails};
pub use response::Response;
pub use router::{Route, RouteHandler, Router};
pub use schema_registry::SchemaRegistry;
pub use server::Server;
pub use validation::SchemaValidator;

/// HTTP method
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
    Trace,
}

impl Method {
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Patch => "PATCH",
            Method::Delete => "DELETE",
            Method::Head => "HEAD",
            Method::Options => "OPTIONS",
            Method::Trace => "TRACE",
        }
    }
}

impl std::str::FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            "PATCH" => Ok(Method::Patch),
            "DELETE" => Ok(Method::Delete),
            "HEAD" => Ok(Method::Head),
            "OPTIONS" => Ok(Method::Options),
            "TRACE" => Ok(Method::Trace),
            _ => Err(format!("Unknown HTTP method: {}", s)),
        }
    }
}

/// CORS configuration for a route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    #[serde(default)]
    pub allowed_headers: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expose_headers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_credentials: Option<bool>,
}

/// Route metadata extracted from Python
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteMetadata {
    pub method: String,
    pub path: String,
    pub handler_name: String,
    pub request_schema: Option<Value>,
    pub response_schema: Option<Value>,
    pub parameter_schema: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_params: Option<Value>, // File parameter schema for validation
    pub is_async: bool,
    pub cors: Option<CorsConfig>,
}

/// Compression configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    /// Enable gzip compression
    #[serde(default = "default_true")]
    pub gzip: bool,
    /// Enable brotli compression
    #[serde(default = "default_true")]
    pub brotli: bool,
    /// Minimum response size to compress (bytes)
    #[serde(default = "default_compression_min_size")]
    pub min_size: usize,
    /// Compression quality (0-11 for brotli, 0-9 for gzip)
    #[serde(default = "default_compression_quality")]
    pub quality: u32,
}

fn default_true() -> bool {
    true
}

fn default_compression_min_size() -> usize {
    1024 // 1KB
}

fn default_compression_quality() -> u32 {
    6 // Default quality
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            gzip: true,
            brotli: true,
            min_size: 1024,
            quality: 6,
        }
    }
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Requests per second
    pub per_second: u64,
    /// Burst allowance
    pub burst: u32,
    /// Use IP-based rate limiting
    #[serde(default = "default_true")]
    pub ip_based: bool,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            per_second: 100,
            burst: 200,
            ip_based: true,
        }
    }
}

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

    // Middleware configurations
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
    /// Enable OpenAPI/Swagger documentation
    pub enable_openapi: bool,
    /// OpenAPI title
    pub openapi_title: Option<String>,
    /// OpenAPI version
    pub openapi_version: Option<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8000,
            workers: 1,
            enable_request_id: true,
            max_body_size: Some(10 * 1024 * 1024), // 10MB default
            request_timeout: Some(30),             // 30 seconds default
            compression: Some(CompressionConfig::default()),
            rate_limit: None, // Disabled by default
            jwt_auth: None,
            api_key_auth: None,
            static_files: Vec::new(),
            graceful_shutdown: true,
            shutdown_timeout: 30,
            enable_openapi: false,
            openapi_title: None,
            openapi_version: None,
        }
    }
}

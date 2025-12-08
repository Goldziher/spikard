use serde::{Deserialize, Serialize};
use serde_json::Value;

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

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
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

/// Route metadata extracted from bindings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteMetadata {
    pub method: String,
    pub path: String,
    pub handler_name: String,
    pub request_schema: Option<Value>,
    pub response_schema: Option<Value>,
    pub parameter_schema: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_params: Option<Value>,
    pub is_async: bool,
    pub cors: Option<CorsConfig>,
    /// Name of the body parameter (defaults to "body" if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_param_name: Option<String>,
    /// List of dependency keys this handler requires (for DI)
    #[cfg(feature = "di")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler_dependencies: Option<Vec<String>>,
}

/// Compression configuration shared across runtimes
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

const fn default_true() -> bool {
    true
}

const fn default_compression_min_size() -> usize {
    1024
}

const fn default_compression_quality() -> u32 {
    6
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            gzip: true,
            brotli: true,
            min_size: default_compression_min_size(),
            quality: default_compression_quality(),
        }
    }
}

/// Rate limiting configuration shared across runtimes
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

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
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Patch => "PATCH",
            Self::Delete => "DELETE",
            Self::Head => "HEAD",
            Self::Options => "OPTIONS",
            Self::Trace => "TRACE",
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
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "PATCH" => Ok(Self::Patch),
            "DELETE" => Ok(Self::Delete),
            "HEAD" => Ok(Self::Head),
            "OPTIONS" => Ok(Self::Options),
            "TRACE" => Ok(Self::Trace),
            _ => Err(format!("Unknown HTTP method: {s}")),
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
    #[serde(default)]
    pub is_async: bool,
    pub cors: Option<CorsConfig>,
    /// Name of the body parameter (defaults to "body" if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_param_name: Option<String>,
    /// List of dependency keys this handler requires (for DI)
    #[cfg(feature = "di")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler_dependencies: Option<Vec<String>>,
    /// JSON-RPC method metadata (if this route is exposed as a JSON-RPC method)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonrpc_method: Option<Value>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_method_as_str_get() {
        assert_eq!(Method::Get.as_str(), "GET");
    }

    #[test]
    fn test_method_as_str_post() {
        assert_eq!(Method::Post.as_str(), "POST");
    }

    #[test]
    fn test_method_as_str_put() {
        assert_eq!(Method::Put.as_str(), "PUT");
    }

    #[test]
    fn test_method_as_str_patch() {
        assert_eq!(Method::Patch.as_str(), "PATCH");
    }

    #[test]
    fn test_method_as_str_delete() {
        assert_eq!(Method::Delete.as_str(), "DELETE");
    }

    #[test]
    fn test_method_as_str_head() {
        assert_eq!(Method::Head.as_str(), "HEAD");
    }

    #[test]
    fn test_method_as_str_options() {
        assert_eq!(Method::Options.as_str(), "OPTIONS");
    }

    #[test]
    fn test_method_as_str_trace() {
        assert_eq!(Method::Trace.as_str(), "TRACE");
    }

    #[test]
    fn test_method_display_get() {
        assert_eq!(Method::Get.to_string(), "GET");
    }

    #[test]
    fn test_method_display_post() {
        assert_eq!(Method::Post.to_string(), "POST");
    }

    #[test]
    fn test_method_display_put() {
        assert_eq!(Method::Put.to_string(), "PUT");
    }

    #[test]
    fn test_method_display_patch() {
        assert_eq!(Method::Patch.to_string(), "PATCH");
    }

    #[test]
    fn test_method_display_delete() {
        assert_eq!(Method::Delete.to_string(), "DELETE");
    }

    #[test]
    fn test_method_display_head() {
        assert_eq!(Method::Head.to_string(), "HEAD");
    }

    #[test]
    fn test_method_display_options() {
        assert_eq!(Method::Options.to_string(), "OPTIONS");
    }

    #[test]
    fn test_method_display_trace() {
        assert_eq!(Method::Trace.to_string(), "TRACE");
    }

    #[test]
    fn test_from_str_get() {
        assert_eq!(Method::from_str("GET"), Ok(Method::Get));
    }

    #[test]
    fn test_from_str_post() {
        assert_eq!(Method::from_str("POST"), Ok(Method::Post));
    }

    #[test]
    fn test_from_str_put() {
        assert_eq!(Method::from_str("PUT"), Ok(Method::Put));
    }

    #[test]
    fn test_from_str_patch() {
        assert_eq!(Method::from_str("PATCH"), Ok(Method::Patch));
    }

    #[test]
    fn test_from_str_delete() {
        assert_eq!(Method::from_str("DELETE"), Ok(Method::Delete));
    }

    #[test]
    fn test_from_str_head() {
        assert_eq!(Method::from_str("HEAD"), Ok(Method::Head));
    }

    #[test]
    fn test_from_str_options() {
        assert_eq!(Method::from_str("OPTIONS"), Ok(Method::Options));
    }

    #[test]
    fn test_from_str_trace() {
        assert_eq!(Method::from_str("TRACE"), Ok(Method::Trace));
    }

    #[test]
    fn test_from_str_lowercase() {
        assert_eq!(Method::from_str("get"), Ok(Method::Get));
    }

    #[test]
    fn test_from_str_mixed_case() {
        assert_eq!(Method::from_str("PoSt"), Ok(Method::Post));
    }

    #[test]
    fn test_from_str_invalid_method() {
        let result = Method::from_str("INVALID");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unknown HTTP method: INVALID");
    }

    #[test]
    fn test_from_str_empty_string() {
        let result = Method::from_str("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unknown HTTP method: ");
    }

    #[test]
    fn test_compression_config_default() {
        let config = CompressionConfig::default();
        assert!(config.gzip);
        assert!(config.brotli);
        assert_eq!(config.min_size, 1024);
        assert_eq!(config.quality, 6);
    }

    #[test]
    fn test_default_true() {
        assert!(default_true());
    }

    #[test]
    fn test_default_compression_min_size() {
        assert_eq!(default_compression_min_size(), 1024);
    }

    #[test]
    fn test_default_compression_quality() {
        assert_eq!(default_compression_quality(), 6);
    }

    #[test]
    fn test_rate_limit_config_default() {
        let config = RateLimitConfig::default();
        assert_eq!(config.per_second, 100);
        assert_eq!(config.burst, 200);
        assert!(config.ip_based);
    }

    #[test]
    fn test_method_equality() {
        assert_eq!(Method::Get, Method::Get);
        assert_ne!(Method::Get, Method::Post);
    }

    #[test]
    fn test_method_clone() {
        let method = Method::Post;
        let cloned = method.clone();
        assert_eq!(method, cloned);
    }

    #[test]
    fn test_compression_config_custom_values() {
        let config = CompressionConfig {
            gzip: false,
            brotli: false,
            min_size: 2048,
            quality: 11,
        };
        assert!(!config.gzip);
        assert!(!config.brotli);
        assert_eq!(config.min_size, 2048);
        assert_eq!(config.quality, 11);
    }

    #[test]
    fn test_rate_limit_config_custom_values() {
        let config = RateLimitConfig {
            per_second: 50,
            burst: 100,
            ip_based: false,
        };
        assert_eq!(config.per_second, 50);
        assert_eq!(config.burst, 100);
        assert!(!config.ip_based);
    }

    #[test]
    fn test_cors_config_construction() {
        let cors = CorsConfig {
            allowed_origins: vec!["http://localhost:3000".to_string()],
            allowed_methods: vec!["GET".to_string(), "POST".to_string()],
            allowed_headers: vec![],
            expose_headers: None,
            max_age: None,
            allow_credentials: None,
        };
        assert_eq!(cors.allowed_origins.len(), 1);
        assert_eq!(cors.allowed_methods.len(), 2);
        assert_eq!(cors.allowed_headers.len(), 0);
    }

    #[test]
    fn test_route_metadata_construction() {
        let metadata = RouteMetadata {
            method: "GET".to_string(),
            path: "/api/users".to_string(),
            handler_name: "get_users".to_string(),
            request_schema: None,
            response_schema: None,
            parameter_schema: None,
            file_params: None,
            is_async: true,
            cors: None,
            body_param_name: None,
            #[cfg(feature = "di")]
            handler_dependencies: None,
            jsonrpc_method: None,
        };
        assert_eq!(metadata.method, "GET");
        assert_eq!(metadata.path, "/api/users");
        assert_eq!(metadata.handler_name, "get_users");
        assert!(metadata.is_async);
    }
}

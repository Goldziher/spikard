//! Configuration extraction trait and implementation for language bindings
//!
//! This module provides a trait-based abstraction for extracting `ServerConfig` and related
//! configuration structs from language-specific objects (Python dicts, JavaScript objects, etc.)
//! without duplicating extraction logic across bindings.
//!
//! The `ConfigSource` trait allows language bindings to implement a unified interface for
//! reading configuration, while `ConfigExtractor` provides the actual extraction logic that
//! works with any `ConfigSource` implementation.

use spikard_http::{
    ApiKeyConfig, CompressionConfig, ContactInfo, JsonRpcConfig, JwtConfig, LicenseInfo, OpenApiConfig,
    RateLimitConfig, SecuritySchemeInfo, ServerConfig, ServerInfo, StaticFilesConfig,
};
use std::collections::HashMap;

/// Trait for reading configuration from language-specific objects
///
/// Bindings implement this trait to provide unified access to configuration values
/// regardless of the language-specific representation (`PyDict`, JavaScript Object, etc.).
pub trait ConfigSource {
    /// Get a boolean value from the source
    fn get_bool(&self, key: &str) -> Option<bool>;

    /// Get a u64 value from the source
    fn get_u64(&self, key: &str) -> Option<u64>;

    /// Get a u16 value from the source
    fn get_u16(&self, key: &str) -> Option<u16>;

    /// Get a string value from the source
    fn get_string(&self, key: &str) -> Option<String>;

    /// Get a vector of strings from the source
    fn get_vec_string(&self, key: &str) -> Option<Vec<String>>;

    /// Get a nested `ConfigSource` for nested objects
    fn get_nested(&self, key: &str) -> Option<Box<dyn ConfigSource + '_>>;

    /// Check if a key exists in the source
    fn has_key(&self, key: &str) -> bool;

    /// Get array length (for collection iteration)
    fn get_array_length(&self, _key: &str) -> Option<usize> {
        None
    }

    /// Get element at index from array
    fn get_array_element(&self, _key: &str, _index: usize) -> Option<Box<dyn ConfigSource + '_>> {
        None
    }

    /// Get u32 value from the source (helper for common case)
    fn get_u32(&self, key: &str) -> Option<u32> {
        self.get_u64(key).and_then(|v| u32::try_from(v).ok())
    }

    /// Get usize value from the source (helper)
    fn get_usize(&self, key: &str) -> Option<usize> {
        self.get_u64(key).and_then(|v| usize::try_from(v).ok())
    }
}

/// Configuration extractor that works with any `ConfigSource`
pub struct ConfigExtractor;

impl ConfigExtractor {
    /// Extract a complete `ServerConfig` from a `ConfigSource`
    ///
    /// # Errors
    ///
    /// Returns an error if required configuration fields are invalid or missing.
    pub fn extract_server_config(source: &dyn ConfigSource) -> Result<ServerConfig, String> {
        let mut config = ServerConfig::default();

        if let Some(host) = source.get_string("host").or_else(|| source.get_string("Host")) {
            config.host = host;
        }

        if let Some(port) = source.get_u16("port").or_else(|| {
            source.get_u32("port").map(|p| {
                #[allow(clippy::cast_possible_truncation)]
                {
                    p as u16
                }
            })
        }) {
            config.port = port;
        }

        if let Some(workers) = source
            .get_usize("workers")
            .or_else(|| source.get_u32("workers").map(|w| w as usize))
        {
            config.workers = workers;
        }

        if let Some(enable_request_id) = source.get_bool("enable_request_id") {
            config.enable_request_id = enable_request_id;
        }

        // `max_body_size = 0` is treated as unlimited.
        if let Some(max_body_size) = source
            .get_usize("max_body_size")
            .or_else(|| source.get_u32("max_body_size").map(|v| v as usize))
        {
            config.max_body_size = if max_body_size == 0 { None } else { Some(max_body_size) };
        }

        if let Some(request_timeout) = source.get_u64("request_timeout") {
            config.request_timeout = Some(request_timeout);
        }

        if let Some(graceful_shutdown) = source.get_bool("graceful_shutdown") {
            config.graceful_shutdown = graceful_shutdown;
        }

        if let Some(shutdown_timeout) = source.get_u64("shutdown_timeout") {
            config.shutdown_timeout = shutdown_timeout;
        }

        config.compression = source
            .get_nested("compression")
            .and_then(|cfg| Self::extract_compression_config(cfg.as_ref()).ok());

        config.rate_limit = source
            .get_nested("rate_limit")
            .and_then(|cfg| Self::extract_rate_limit_config(cfg.as_ref()).ok());

        config.jwt_auth = source
            .get_nested("jwt_auth")
            .and_then(|cfg| Self::extract_jwt_config(cfg.as_ref()).ok());

        config.api_key_auth = source
            .get_nested("api_key_auth")
            .and_then(|cfg| Self::extract_api_key_config(cfg.as_ref()).ok());

        config.static_files = Self::extract_static_files_config(source)?;

        config.openapi = source
            .get_nested("openapi")
            .and_then(|cfg| Self::extract_openapi_config(cfg.as_ref()).ok());

        config.jsonrpc = source
            .get_nested("jsonrpc")
            .and_then(|cfg| Self::extract_jsonrpc_config(cfg.as_ref()).ok());

        if let Some(enable_http_trace) = source.get_bool("enable_http_trace") {
            config.enable_http_trace = enable_http_trace;
        }

        Ok(config)
    }

    /// Extract `CompressionConfig` from a `ConfigSource`
    ///
    /// # Errors
    ///
    /// Returns an error if required configuration fields are invalid.
    pub fn extract_compression_config(source: &dyn ConfigSource) -> Result<CompressionConfig, String> {
        let gzip = source.get_bool("gzip").unwrap_or(true);
        let brotli = source.get_bool("brotli").unwrap_or(true);
        let min_size = source
            .get_usize("min_size")
            .or_else(|| source.get_u32("min_size").map(|s| s as usize))
            .unwrap_or(1024);
        let quality = source.get_u32("quality").unwrap_or(6);

        Ok(CompressionConfig {
            gzip,
            brotli,
            min_size,
            quality,
        })
    }

    /// Extract `RateLimitConfig` from a `ConfigSource`
    ///
    /// # Errors
    ///
    /// Returns an error if required fields `per_second` or `burst` are missing.
    pub fn extract_rate_limit_config(source: &dyn ConfigSource) -> Result<RateLimitConfig, String> {
        let per_second = source.get_u64("per_second").ok_or("Rate limit requires 'per_second'")?;

        let burst = source.get_u32("burst").ok_or("Rate limit requires 'burst' as u32")?;

        let ip_based = source.get_bool("ip_based").unwrap_or(true);

        Ok(RateLimitConfig {
            per_second,
            burst,
            ip_based,
        })
    }

    /// Extract `JwtConfig` from a `ConfigSource`
    ///
    /// # Errors
    ///
    /// Returns an error if the required `secret` field is missing.
    pub fn extract_jwt_config(source: &dyn ConfigSource) -> Result<JwtConfig, String> {
        let secret = source.get_string("secret").ok_or("JWT auth requires 'secret'")?;

        let algorithm = source.get_string("algorithm").unwrap_or_else(|| "HS256".to_string());

        let audience = source.get_vec_string("audience");

        let issuer = source.get_string("issuer");

        let leeway = source.get_u64("leeway").unwrap_or(0);

        Ok(JwtConfig {
            secret,
            algorithm,
            audience,
            issuer,
            leeway,
        })
    }

    /// Extract `ApiKeyConfig` from a `ConfigSource`
    ///
    /// # Errors
    ///
    /// Returns an error if the required `keys` field is missing.
    pub fn extract_api_key_config(source: &dyn ConfigSource) -> Result<ApiKeyConfig, String> {
        let keys = source
            .get_vec_string("keys")
            .ok_or("API Key auth requires 'keys' as Vec<String>)")?;

        let header_name = source
            .get_string("header_name")
            .unwrap_or_else(|| "X-API-Key".to_string());

        Ok(ApiKeyConfig { keys, header_name })
    }

    /// Extract static files configuration list from a `ConfigSource`
    ///
    /// # Errors
    ///
    /// Returns an error if array elements are invalid or missing required fields.
    pub fn extract_static_files_config(source: &dyn ConfigSource) -> Result<Vec<StaticFilesConfig>, String> {
        let length = source.get_array_length("static_files").unwrap_or(0);
        if length == 0 {
            return Ok(Vec::new());
        }

        let mut configs = Vec::new();
        for i in 0..length {
            let sf_source = source
                .get_array_element("static_files", i)
                .ok_or("Failed to get static files array element")?;

            let directory = sf_source
                .get_string("directory")
                .ok_or("Static files requires 'directory'")?;

            let route_prefix = sf_source
                .get_string("route_prefix")
                .ok_or("Static files requires 'route_prefix'")?;

            let index_file = sf_source.get_bool("index_file").unwrap_or(true);

            let cache_control = sf_source.get_string("cache_control");

            configs.push(StaticFilesConfig {
                directory,
                route_prefix,
                index_file,
                cache_control,
            });
        }

        Ok(configs)
    }

    /// Extract `OpenApiConfig` from a `ConfigSource`
    ///
    /// # Errors
    ///
    /// Returns an error if required configuration fields are invalid.
    pub fn extract_openapi_config(source: &dyn ConfigSource) -> Result<OpenApiConfig, String> {
        let enabled = source.get_bool("enabled").unwrap_or(false);
        let title = source.get_string("title").unwrap_or_else(|| "API".to_string());
        let version = source.get_string("version").unwrap_or_else(|| "1.0.0".to_string());
        let description = source.get_string("description");
        let swagger_ui_path = source
            .get_string("swagger_ui_path")
            .unwrap_or_else(|| "/docs".to_string());
        let redoc_path = source.get_string("redoc_path").unwrap_or_else(|| "/redoc".to_string());
        let openapi_json_path = source
            .get_string("openapi_json_path")
            .unwrap_or_else(|| "/openapi.json".to_string());

        let contact = source
            .get_nested("contact")
            .map(|cfg| {
                let name = cfg.get_string("name");
                let email = cfg.get_string("email");
                let url = cfg.get_string("url");
                ContactInfo { name, email, url }
            })
            .filter(|c| c.name.is_some() || c.email.is_some() || c.url.is_some());

        let license = source.get_nested("license").and_then(|cfg| {
            let name = cfg.get_string("name")?;
            let url = cfg.get_string("url");
            Some(LicenseInfo { name, url })
        });

        let servers = Self::extract_servers_config(source)?;

        let security_schemes = Self::extract_security_schemes_config(source);

        Ok(OpenApiConfig {
            enabled,
            title,
            version,
            description,
            swagger_ui_path,
            redoc_path,
            openapi_json_path,
            contact,
            license,
            servers,
            security_schemes,
        })
    }

    /// Extract servers list from `OpenAPI` config
    ///
    /// # Errors
    ///
    /// Returns an error if array elements are invalid or missing.
    fn extract_servers_config(source: &dyn ConfigSource) -> Result<Vec<ServerInfo>, String> {
        let length = source.get_array_length("servers").unwrap_or(0);
        if length == 0 {
            return Ok(Vec::new());
        }

        let mut servers = Vec::new();
        for i in 0..length {
            let server_source = source
                .get_array_element("servers", i)
                .ok_or("Failed to get servers array element")?;

            let url = server_source.get_string("url").ok_or("Server requires 'url'")?;

            let description = server_source.get_string("description");

            servers.push(ServerInfo { url, description });
        }

        Ok(servers)
    }

    /// Extract security schemes from `OpenAPI` config
    fn extract_security_schemes_config(_source: &dyn ConfigSource) -> HashMap<String, SecuritySchemeInfo> {
        // TODO: Implement when bindings support iterating HashMap-like structures
        HashMap::new()
    }

    /// Extract `JsonRpcConfig` from a `ConfigSource`
    ///
    /// # Errors
    ///
    /// Returns an error if required configuration fields are invalid.
    pub fn extract_jsonrpc_config(source: &dyn ConfigSource) -> Result<JsonRpcConfig, String> {
        let enabled = source.get_bool("enabled").unwrap_or(true);
        let endpoint_path = source.get_string("endpoint_path").unwrap_or_else(|| "/rpc".to_string());
        let enable_batch = source.get_bool("enable_batch").unwrap_or(true);
        let max_batch_size = source
            .get_usize("max_batch_size")
            .or_else(|| source.get_u32("max_batch_size").map(|s| s as usize))
            .unwrap_or(100);

        Ok(JsonRpcConfig {
            enabled,
            endpoint_path,
            enable_batch,
            max_batch_size,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    struct MockConfigSource {
        data: HashMap<String, String>,
    }

    impl MockConfigSource {
        fn new() -> Self {
            Self { data: HashMap::new() }
        }

        fn with(mut self, key: &str, value: String) -> Self {
            self.data.insert(key.to_string(), value);
            self
        }
    }

    impl ConfigSource for MockConfigSource {
        fn get_bool(&self, key: &str) -> Option<bool> {
            self.data.get(key).and_then(|v| match v.as_str() {
                "true" => Some(true),
                "false" => Some(false),
                _ => v.parse().ok(),
            })
        }

        fn get_u64(&self, key: &str) -> Option<u64> {
            self.data.get(key).and_then(|v| v.parse().ok())
        }

        fn get_u16(&self, key: &str) -> Option<u16> {
            self.data.get(key).and_then(|v| v.parse().ok())
        }

        fn get_string(&self, key: &str) -> Option<String> {
            self.data.get(key).cloned()
        }

        fn get_vec_string(&self, key: &str) -> Option<Vec<String>> {
            self.data
                .get(key)
                .map(|s| s.split(',').map(|t| t.trim().to_string()).collect())
        }

        fn get_nested(&self, _key: &str) -> Option<Box<dyn ConfigSource + '_>> {
            None
        }

        fn has_key(&self, key: &str) -> bool {
            self.data.contains_key(key)
        }
    }

    #[test]
    fn test_compression_config_extraction() {
        let source = MockConfigSource::new()
            .with("gzip", "true".to_string())
            .with("brotli", "false".to_string())
            .with("min_size", "2048".to_string())
            .with("quality", "9".to_string());

        let config = ConfigExtractor::extract_compression_config(&source).unwrap();
        assert!(config.gzip);
        assert!(!config.brotli);
        assert_eq!(config.min_size, 2048);
        assert_eq!(config.quality, 9);
    }

    #[test]
    fn test_compression_config_defaults() {
        let source = MockConfigSource::new();

        let config = ConfigExtractor::extract_compression_config(&source).unwrap();
        assert!(config.gzip);
        assert!(config.brotli);
        assert_eq!(config.min_size, 1024);
        assert_eq!(config.quality, 6);
    }

    #[test]
    fn test_jwt_config_extraction() {
        let source = MockConfigSource::new()
            .with("secret", "my-secret".to_string())
            .with("algorithm", "HS512".to_string())
            .with("leeway", "30".to_string());

        let config = ConfigExtractor::extract_jwt_config(&source).unwrap();
        assert_eq!(config.secret, "my-secret");
        assert_eq!(config.algorithm, "HS512");
        assert_eq!(config.leeway, 30);
    }

    #[test]
    fn test_jwt_config_missing_secret() {
        let source = MockConfigSource::new();
        let result = ConfigExtractor::extract_jwt_config(&source);
        assert!(result.is_err());
    }

    #[test]
    fn test_api_key_config_extraction() {
        let source = MockConfigSource::new()
            .with("keys", "key1,key2,key3".to_string())
            .with("header_name", "Authorization".to_string());

        let config = ConfigExtractor::extract_api_key_config(&source).unwrap();
        assert_eq!(config.keys, vec!["key1", "key2", "key3"]);
        assert_eq!(config.header_name, "Authorization");
    }

    #[test]
    fn test_api_key_config_defaults() {
        let source = MockConfigSource::new().with("keys", "only-key".to_string());

        let config = ConfigExtractor::extract_api_key_config(&source).unwrap();
        assert_eq!(config.keys, vec!["only-key"]);
        assert_eq!(config.header_name, "X-API-Key");
    }

    #[test]
    fn test_rate_limit_config_extraction() {
        let source = MockConfigSource::new()
            .with("per_second", "100".to_string())
            .with("burst", "50".to_string())
            .with("ip_based", "false".to_string());

        let config = ConfigExtractor::extract_rate_limit_config(&source).unwrap();
        assert_eq!(config.per_second, 100);
        assert_eq!(config.burst, 50);
        assert!(!config.ip_based);
    }

    #[test]
    fn test_rate_limit_config_missing_required() {
        let source = MockConfigSource::new().with("per_second", "100".to_string());

        let result = ConfigExtractor::extract_rate_limit_config(&source);
        assert!(result.is_err());
    }

    #[test]
    fn test_openapi_config_extraction() {
        let source = MockConfigSource::new()
            .with("enabled", "true".to_string())
            .with("title", "Test API".to_string())
            .with("version", "2.0.0".to_string())
            .with("description", "A test API".to_string())
            .with("swagger_ui_path", "/api-docs".to_string())
            .with("redoc_path", "/api-redoc".to_string())
            .with("openapi_json_path", "/api.json".to_string());

        let config = ConfigExtractor::extract_openapi_config(&source).unwrap();
        assert!(config.enabled);
        assert_eq!(config.title, "Test API");
        assert_eq!(config.version, "2.0.0");
        assert_eq!(config.description, Some("A test API".to_string()));
        assert_eq!(config.swagger_ui_path, "/api-docs");
        assert_eq!(config.redoc_path, "/api-redoc");
        assert_eq!(config.openapi_json_path, "/api.json");
    }

    #[test]
    fn test_openapi_config_defaults() {
        let source = MockConfigSource::new();

        let config = ConfigExtractor::extract_openapi_config(&source).unwrap();
        assert!(!config.enabled);
        assert_eq!(config.title, "API");
        assert_eq!(config.version, "1.0.0");
        assert_eq!(config.description, None);
        assert_eq!(config.swagger_ui_path, "/docs");
        assert_eq!(config.redoc_path, "/redoc");
        assert_eq!(config.openapi_json_path, "/openapi.json");
    }

    #[test]
    fn test_static_files_config_empty() {
        let source = MockConfigSource::new();

        let configs = ConfigExtractor::extract_static_files_config(&source).unwrap();
        assert_eq!(configs.len(), 0);
    }

    #[test]
    fn test_server_config_extraction() {
        let source = MockConfigSource::new()
            .with("host", "0.0.0.0".to_string())
            .with("port", "3000".to_string())
            .with("workers", "4".to_string())
            .with("enable_request_id", "false".to_string())
            .with("max_body_size", "5242880".to_string())
            .with("request_timeout", "60".to_string())
            .with("graceful_shutdown", "false".to_string())
            .with("shutdown_timeout", "10".to_string());

        let config = ConfigExtractor::extract_server_config(&source).unwrap();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3000);
        assert_eq!(config.workers, 4);
        assert!(!config.enable_request_id);
        assert_eq!(config.max_body_size, Some(5_242_880));
        assert_eq!(config.request_timeout, Some(60));
        assert!(!config.graceful_shutdown);
        assert_eq!(config.shutdown_timeout, 10);
    }

    #[test]
    fn test_server_config_defaults() {
        let source = MockConfigSource::new();

        let config = ConfigExtractor::extract_server_config(&source).unwrap();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8000);
        assert_eq!(config.workers, 1);
        assert!(!config.enable_request_id);
        assert_eq!(config.max_body_size, Some(10 * 1024 * 1024));
        assert_eq!(config.request_timeout, None);
        assert!(config.graceful_shutdown);
        assert_eq!(config.shutdown_timeout, 30);
    }

    #[test]
    fn test_servers_config_empty() {
        let source = MockConfigSource::new();

        let servers = ConfigExtractor::extract_servers_config(&source).unwrap();
        assert_eq!(servers.len(), 0);
    }

    #[test]
    fn test_security_schemes_config_empty() {
        let source = MockConfigSource::new();

        let schemes = ConfigExtractor::extract_security_schemes_config(&source);
        assert_eq!(schemes.len(), 0);
    }

    struct JsonConfigSource<'a> {
        value: &'a Value,
    }

    impl<'a> JsonConfigSource<'a> {
        fn new(value: &'a Value) -> Self {
            Self { value }
        }
    }

    impl ConfigSource for JsonConfigSource<'_> {
        fn get_bool(&self, key: &str) -> Option<bool> {
            self.value.get(key)?.as_bool()
        }

        fn get_u64(&self, key: &str) -> Option<u64> {
            self.value.get(key)?.as_u64()
        }

        fn get_u16(&self, key: &str) -> Option<u16> {
            u16::try_from(self.get_u64(key)?).ok()
        }

        fn get_string(&self, key: &str) -> Option<String> {
            self.value.get(key)?.as_str().map(str::to_string)
        }

        fn get_vec_string(&self, key: &str) -> Option<Vec<String>> {
            self.value
                .get(key)?
                .as_array()
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(str::to_string)).collect())
        }

        fn get_nested(&self, key: &str) -> Option<Box<dyn ConfigSource + '_>> {
            let nested = self.value.get(key)?;
            nested
                .is_object()
                .then(|| Box::new(JsonConfigSource::new(nested)) as Box<dyn ConfigSource>)
        }

        fn has_key(&self, key: &str) -> bool {
            self.value.get(key).is_some()
        }

        fn get_array_length(&self, key: &str) -> Option<usize> {
            self.value.get(key)?.as_array().map(Vec::len)
        }

        fn get_array_element(&self, key: &str, index: usize) -> Option<Box<dyn ConfigSource + '_>> {
            let arr = self.value.get(key)?.as_array()?;
            let elem = arr.get(index)?;
            elem.is_object()
                .then(|| Box::new(JsonConfigSource::new(elem)) as Box<dyn ConfigSource>)
        }
    }

    #[test]
    fn test_static_files_extraction_supports_arrays() {
        let value = serde_json::json!({
            "static_files": [
                {
                    "directory": "public",
                    "route_prefix": "/assets",
                    "index_file": true,
                    "cache_control": "public, max-age=3600"
                }
            ]
        });
        let source = JsonConfigSource::new(&value);
        let configs = ConfigExtractor::extract_static_files_config(&source).expect("extract");
        assert_eq!(configs.len(), 1);
        assert_eq!(configs[0].directory, "public");
        assert_eq!(configs[0].route_prefix, "/assets");
        assert!(configs[0].index_file);
        assert_eq!(configs[0].cache_control.as_deref(), Some("public, max-age=3600"));
    }

    #[test]
    fn test_static_files_extraction_missing_required_fields_errors() {
        let value = serde_json::json!({
            "static_files": [
                {
                    "route_prefix": "/assets"
                }
            ]
        });
        let source = JsonConfigSource::new(&value);
        let err = ConfigExtractor::extract_static_files_config(&source).expect_err("missing directory should error");
        assert!(
            err.contains("Static files requires 'directory'"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn test_static_files_extraction_array_element_missing_errors() {
        struct BrokenArraySource;

        impl ConfigSource for BrokenArraySource {
            fn get_bool(&self, _key: &str) -> Option<bool> {
                None
            }

            fn get_u64(&self, _key: &str) -> Option<u64> {
                None
            }

            fn get_u16(&self, _key: &str) -> Option<u16> {
                None
            }

            fn get_string(&self, _key: &str) -> Option<String> {
                None
            }

            fn get_vec_string(&self, _key: &str) -> Option<Vec<String>> {
                None
            }

            fn get_nested(&self, _key: &str) -> Option<Box<dyn ConfigSource + '_>> {
                None
            }

            fn has_key(&self, _key: &str) -> bool {
                false
            }

            fn get_array_length(&self, key: &str) -> Option<usize> {
                (key == "static_files").then_some(1)
            }

            fn get_array_element(&self, _key: &str, _index: usize) -> Option<Box<dyn ConfigSource + '_>> {
                None
            }
        }

        let err = ConfigExtractor::extract_static_files_config(&BrokenArraySource)
            .expect_err("missing array element should error");
        assert!(
            err.contains("Failed to get static files array element"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn test_server_config_prefers_host_key_variants() {
        let value = serde_json::json!({
            "Host": "0.0.0.0",
            "port": 9000,
            "workers": 2,
            "enable_request_id": false,
            "graceful_shutdown": true,
            "shutdown_timeout": 1,
            "static_files": []
        });
        let source = JsonConfigSource::new(&value);
        let cfg = ConfigExtractor::extract_server_config(&source).expect("extract");
        assert_eq!(cfg.host, "0.0.0.0");
        assert_eq!(cfg.port, 9000);
        assert_eq!(cfg.workers, 2);
        assert!(!cfg.enable_request_id);
    }
}

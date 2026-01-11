//! Shared test client infrastructure

use std::collections::HashMap;

/// Base configuration for test clients across bindings
pub struct TestClientConfig {
    /// The base URL for the test server
    pub base_url: String,
    /// Request timeout in milliseconds
    pub timeout_ms: u64,
    /// Whether to follow redirects
    pub follow_redirects: bool,
}

impl TestClientConfig {
    /// Create a new test client configuration with custom base URL
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            timeout_ms: 30000,
            follow_redirects: true,
        }
    }

    /// Set the timeout in milliseconds
    #[must_use]
    pub const fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }

    /// Set whether to follow redirects
    #[must_use]
    pub const fn with_follow_redirects(mut self, follow_redirects: bool) -> Self {
        self.follow_redirects = follow_redirects;
        self
    }
}

impl Default for TestClientConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:3000".to_string(),
            timeout_ms: 30000,
            follow_redirects: true,
        }
    }
}

/// Common test response metadata
#[derive(Debug, Clone)]
pub struct TestResponseMetadata {
    /// HTTP status code
    pub status_code: u16,
    /// Response headers
    pub headers: HashMap<String, String>,
    /// Response body size in bytes
    pub body_size: usize,
    /// Response time in milliseconds
    pub response_time_ms: u64,
}

impl TestResponseMetadata {
    /// Create a new test response metadata
    #[must_use]
    pub const fn new(
        status_code: u16,
        headers: HashMap<String, String>,
        body_size: usize,
        response_time_ms: u64,
    ) -> Self {
        Self {
            status_code,
            headers,
            body_size,
            response_time_ms,
        }
    }

    /// Get a header value by name (case-insensitive)
    #[must_use]
    pub fn get_header(&self, name: &str) -> Option<&String> {
        let lower_name = name.to_lowercase();
        self.headers
            .iter()
            .find(|(k, _)| k.to_lowercase() == lower_name)
            .map(|(_, v)| v)
    }

    /// Check if response was successful (2xx status code)
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.status_code >= 200 && self.status_code < 300
    }

    /// Check if response was a client error (4xx status code)
    #[must_use]
    pub const fn is_client_error(&self) -> bool {
        self.status_code >= 400 && self.status_code < 500
    }

    /// Check if response was a server error (5xx status code)
    #[must_use]
    pub const fn is_server_error(&self) -> bool {
        self.status_code >= 500 && self.status_code < 600
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_client_config_default() {
        let config = TestClientConfig::default();
        assert_eq!(config.base_url, "http://localhost:3000");
        assert_eq!(config.timeout_ms, 30000);
        assert!(config.follow_redirects);
    }

    #[test]
    fn test_test_client_config_new() {
        let config = TestClientConfig::new("http://example.com:8080");
        assert_eq!(config.base_url, "http://example.com:8080");
        assert_eq!(config.timeout_ms, 30000);
        assert!(config.follow_redirects);
    }

    #[test]
    fn test_test_client_config_with_timeout() {
        let config = TestClientConfig::default().with_timeout(5000);
        assert_eq!(config.timeout_ms, 5000);
    }

    #[test]
    fn test_test_client_config_with_follow_redirects_false() {
        let config = TestClientConfig::default().with_follow_redirects(false);
        assert!(!config.follow_redirects);
    }

    #[test]
    fn test_test_client_config_chaining() {
        let config = TestClientConfig::new("http://api.example.com")
            .with_timeout(10000)
            .with_follow_redirects(false);

        assert_eq!(config.base_url, "http://api.example.com");
        assert_eq!(config.timeout_ms, 10000);
        assert!(!config.follow_redirects);
    }

    #[test]
    fn test_response_metadata_new() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        let metadata = TestResponseMetadata::new(200, headers.clone(), 256, 100);

        assert_eq!(metadata.status_code, 200);
        assert_eq!(metadata.headers, headers);
        assert_eq!(metadata.body_size, 256);
        assert_eq!(metadata.response_time_ms, 100);
    }

    #[test]
    fn test_response_metadata_clone() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        let metadata1 = TestResponseMetadata::new(201, headers, 512, 200);
        let metadata2 = metadata1.clone();

        assert_eq!(metadata1.status_code, metadata2.status_code);
        assert_eq!(metadata1.body_size, metadata2.body_size);
        assert_eq!(metadata1.response_time_ms, metadata2.response_time_ms);
    }

    #[test]
    fn test_get_header_case_insensitive() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), "Bearer token123".to_string());

        let metadata = TestResponseMetadata::new(200, headers, 100, 50);

        assert_eq!(
            metadata.get_header("Content-Type"),
            Some(&"application/json".to_string())
        );
        assert_eq!(
            metadata.get_header("content-type"),
            Some(&"application/json".to_string())
        );
        assert_eq!(
            metadata.get_header("CONTENT-TYPE"),
            Some(&"application/json".to_string())
        );
        assert_eq!(metadata.get_header("Missing"), None);
    }

    #[test]
    fn test_is_success() {
        let headers = HashMap::new();
        let metadata_200 = TestResponseMetadata::new(200, headers.clone(), 100, 50);
        let metadata_201 = TestResponseMetadata::new(201, headers.clone(), 100, 50);
        let metadata_204 = TestResponseMetadata::new(204, headers.clone(), 0, 50);
        let metadata_299 = TestResponseMetadata::new(299, headers.clone(), 100, 50);
        let metadata_300 = TestResponseMetadata::new(300, headers.clone(), 100, 50);
        let metadata_400 = TestResponseMetadata::new(400, headers, 100, 50);

        assert!(metadata_200.is_success());
        assert!(metadata_201.is_success());
        assert!(metadata_204.is_success());
        assert!(metadata_299.is_success());
        assert!(!metadata_300.is_success());
        assert!(!metadata_400.is_success());
    }

    #[test]
    fn test_is_client_error() {
        let headers = HashMap::new();
        let metadata_399 = TestResponseMetadata::new(399, headers.clone(), 100, 50);
        let metadata_400 = TestResponseMetadata::new(400, headers.clone(), 100, 50);
        let metadata_404 = TestResponseMetadata::new(404, headers.clone(), 100, 50);
        let metadata_499 = TestResponseMetadata::new(499, headers.clone(), 100, 50);
        let metadata_500 = TestResponseMetadata::new(500, headers, 100, 50);

        assert!(!metadata_399.is_client_error());
        assert!(metadata_400.is_client_error());
        assert!(metadata_404.is_client_error());
        assert!(metadata_499.is_client_error());
        assert!(!metadata_500.is_client_error());
    }

    #[test]
    fn test_is_server_error() {
        let headers = HashMap::new();
        let metadata_499 = TestResponseMetadata::new(499, headers.clone(), 100, 50);
        let metadata_500 = TestResponseMetadata::new(500, headers.clone(), 100, 50);
        let metadata_502 = TestResponseMetadata::new(502, headers.clone(), 100, 50);
        let metadata_599 = TestResponseMetadata::new(599, headers.clone(), 100, 50);
        let metadata_600 = TestResponseMetadata::new(600, headers, 100, 50);

        assert!(!metadata_499.is_server_error());
        assert!(metadata_500.is_server_error());
        assert!(metadata_502.is_server_error());
        assert!(metadata_599.is_server_error());
        assert!(!metadata_600.is_server_error());
    }

    #[test]
    fn test_response_metadata_debug() {
        let headers = HashMap::new();
        let metadata = TestResponseMetadata::new(200, headers, 100, 50);
        let debug_str = format!("{metadata:?}");
        assert!(debug_str.contains("200"));
        assert!(debug_str.contains("100"));
        assert!(debug_str.contains("50"));
    }
}

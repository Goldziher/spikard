//! Shared test client infrastructure

/// Base configuration for test clients across bindings
pub struct TestClientConfig {
    /// The base URL for the test server
    pub base_url: String,
    /// Request timeout in milliseconds
    pub timeout_ms: u64,
    /// Whether to follow redirects
    pub follow_redirects: bool,
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
    pub headers: std::collections::HashMap<String, String>,
    /// Response body size in bytes
    pub body_size: usize,
    /// Response time in milliseconds
    pub response_time_ms: u64,
}

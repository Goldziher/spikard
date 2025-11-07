//! HTTP response wrapper for Node.js

use napi::bindgen_prelude::*;
use napi_derive::napi;
use serde_json::Value;

/// HTTP Response wrapper
#[napi]
pub struct TestResponse {
    status: u16,
    headers: serde_json::Map<String, Value>,
    body: Vec<u8>,
}

#[napi]
impl TestResponse {
    /// Create a new response
    #[allow(dead_code)]
    pub(crate) fn new(status: u16, headers: serde_json::Map<String, Value>, body: Vec<u8>) -> Self {
        Self { status, headers, body }
    }

    /// Get the HTTP status code
    #[napi(getter)]
    pub fn status_code(&self) -> u16 {
        self.status
    }

    /// Get response headers as JSON
    #[napi]
    pub fn headers(&self) -> serde_json::Value {
        serde_json::Value::Object(self.headers.clone())
    }

    /// Get response body as text
    #[napi]
    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).to_string()
    }

    /// Parse response body as JSON
    #[napi]
    pub fn json(&self) -> napi::Result<serde_json::Value> {
        serde_json::from_slice(&self.body).map_err(|e| Error::from_reason(format!("Failed to parse JSON: {}", e)))
    }

    /// Get raw response body bytes
    #[napi]
    pub fn bytes(&self) -> Buffer {
        Buffer::from(self.body.clone())
    }
}

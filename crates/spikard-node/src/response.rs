//! HTTP response wrapper for Node.js

use napi::bindgen_prelude::*;
use napi_derive::napi;
use serde_json::Value;
use std::collections::HashMap;

/// HTTP Response wrapper
#[napi]
pub struct TestResponse {
    status: u16,
    headers: serde_json::Map<String, Value>,
    body: Vec<u8>,
}

#[napi]
impl TestResponse {
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

/// Optional configuration for a streaming response.
///
/// This struct is exposed to JavaScript via napi and provides configuration
/// options when creating streaming responses from async iterators.
///
/// NOTE: Marked with #[allow(dead_code)] because the #[napi(object)] macro
/// generates access patterns that aren't visible to the Rust dead code checker,
/// though the struct is actually exposed to and used by JavaScript code.
#[napi(object)]
#[allow(dead_code)]
pub struct StreamingResponseInit {
    /// HTTP status code for the streaming response (default 200).
    pub status_code: Option<u16>,
    /// Headers to attach to the streaming response.
    pub headers: Option<HashMap<String, String>>,
}

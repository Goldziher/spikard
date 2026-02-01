//! Structured output from Node.js handlers
//!
//! This module defines the `HandlerOutput` struct which is returned directly from JavaScript handlers
//! without JSON string serialization. This completes the zero-copy pattern for request/response cycles.

use axum::body::Body;
use axum::http::Response;
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
use serde_json::Value;
use std::collections::HashMap;

/// Structured handler output returned from JavaScript handlers
///
/// This struct is returned directly from handlers without JSON serialization,
/// completing the zero-copy request/response pattern.
#[napi(object)]
pub struct HandlerOutput {
    /// HTTP status code (e.g., 200, 404, 500)
    pub status: u16,
    /// Response headers as a map
    pub headers: Option<HashMap<String, String>>,
    /// Response body as JSON value (used when `raw_body` is not set)
    pub body: Option<Value>,
    /// Pre-serialized response body bytes. When set, this is used directly as
    /// the response body, bypassing `serde_json::to_vec` on the `body` field.
    /// JS handlers can pass `Buffer.from(JSON.stringify(obj))` here to avoid
    /// a napi Value → serde_json::Value → bytes round-trip.
    pub raw_body: Option<Buffer>,
}

impl HandlerOutput {
    /// Convert this handler output into an Axum response
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - JSON serialization of the body fails
    /// - HTTP response construction fails
    pub fn into_response(self) -> Result<Response<Body>, String> {
        let mut response = Response::builder().status(self.status);

        if let Some(headers) = self.headers {
            for (k, v) in headers {
                response = response.header(k, v);
            }
        }

        // Prefer pre-serialized raw_body to skip serde_json round-trip.
        let body = if let Some(raw) = self.raw_body {
            Body::from(Vec::from(raw))
        } else if let Some(body_value) = self.body {
            let bytes = serde_json::to_vec(&body_value).map_err(|e| format!("Failed to serialize body: {}", e))?;
            Body::from(bytes)
        } else {
            Body::empty()
        };

        response
            .body(body)
            .map_err(|e| format!("Failed to build response: {}", e))
    }
}

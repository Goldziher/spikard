//! Structured output from Node.js handlers
//!
//! This module defines the `HandlerOutput` struct which is returned directly from JavaScript handlers
//! without JSON string serialization. This completes the zero-copy pattern for request/response cycles.

use axum::body::Body;
use axum::http::Response;
use napi_derive::napi;
use serde_json::Value;
use std::collections::HashMap;

/// Structured handler output returned from JavaScript handlers
///
/// This struct is returned directly from handlers without JSON serialization,
/// completing the zero-copy request/response pattern.
///
/// PERFORMANCE: `object_to_js = false` skips generating ToNapiValue since
/// HandlerOutput only flows JS→Rust, never Rust→JS. This eliminates unnecessary
/// code generation and potential conversion overhead.
#[napi(object, object_to_js = false)]
pub struct HandlerOutput {
    /// HTTP status code (e.g., 200, 404, 500)
    pub status: u16,
    /// Response headers as a map
    pub headers: Option<HashMap<String, String>>,
    /// Response body as JSON value
    pub body: Option<Value>,
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

        let body = if let Some(body_value) = self.body {
            serde_json::to_vec(&body_value).map_err(|e| format!("Failed to serialize body: {}", e))?
        } else {
            vec![]
        };

        response
            .body(Body::from(body))
            .map_err(|e| format!("Failed to build response: {}", e))
    }
}

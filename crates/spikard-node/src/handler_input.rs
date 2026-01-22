//! Structured input for Node.js handlers
//!
//! This module defines the `HandlerInput` struct which is passed directly to JavaScript handlers
//! without JSON serialization. This eliminates the Value→String→parse→Value cycle, providing
//! a 6x performance improvement.

use napi_derive::napi;
use serde_json::Value;
use spikard_http::RequestData;
use std::collections::HashMap;
use std::sync::Arc;

/// Helper to unwrap Arc without cloning when possible
#[inline]
fn unwrap_arc<T: Clone>(arc: Arc<T>) -> T {
    Arc::try_unwrap(arc).unwrap_or_else(|arc| (*arc).clone())
}

/// Structured handler input passed to JavaScript handlers
///
/// This struct replaces JSON string passing, eliminating serialization overhead.
/// Fields are converted from `RequestData` using a direct `From` impl.
///
/// PERFORMANCE: `object_from_js = false` skips generating FromNapiValue since
/// HandlerInput only flows Rust→JS, never JS→Rust. This eliminates unnecessary
/// code generation and potential conversion overhead.
#[napi(object, object_from_js = false)]
pub struct HandlerInput {
    /// HTTP method (GET, POST, etc.)
    pub method: String,
    /// Request path
    pub path: String,
    /// HTTP headers as a map
    pub headers: HashMap<String, String>,
    /// HTTP cookies as a map
    pub cookies: HashMap<String, String>,
    /// Parsed query parameters
    pub query_params: Value,
    /// Validated parameters (query/path/header/cookie combined)
    pub validated_params: Option<Value>,
    /// Parsed request body
    pub body: Value,
    /// Extracted path parameters
    pub path_params: HashMap<String, String>,
}

impl From<&RequestData> for HandlerInput {
    fn from(data: &RequestData) -> Self {
        Self {
            method: data.method.clone(),
            path: data.path.clone(),
            headers: (*data.headers).clone(),
            cookies: (*data.cookies).clone(),
            query_params: (*data.query_params).clone(),
            validated_params: data.validated_params.as_ref().map(|arc| (**arc).clone()),
            body: (*data.body).clone(),
            path_params: (*data.path_params).clone(),
        }
    }
}

impl From<RequestData> for HandlerInput {
    fn from(data: RequestData) -> Self {
        // PERFORMANCE: Try to unwrap Arcs to avoid cloning when possible
        // When Arc refcount is 1 (typical case): Zero-copy, no allocation
        // When Arc refcount > 1 (rare): Falls back to clone
        Self {
            method: data.method,
            path: data.path,
            headers: unwrap_arc(data.headers),
            cookies: unwrap_arc(data.cookies),
            query_params: unwrap_arc(data.query_params),
            validated_params: data.validated_params.map(unwrap_arc),
            body: unwrap_arc(data.body),
            path_params: unwrap_arc(data.path_params),
        }
    }
}

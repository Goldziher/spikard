//! Structured input for Node.js handlers
//!
//! This module defines the `HandlerInput` struct which is passed directly to JavaScript handlers
//! without JSON serialization. This eliminates the Value→String→parse→Value cycle, providing
//! a 6x performance improvement.

use napi_derive::napi;
use serde_json::Value;
use spikard_http::RequestData;
use std::collections::HashMap;

/// Structured handler input passed to JavaScript handlers
///
/// This struct replaces JSON string passing, eliminating serialization overhead.
/// Fields are converted from `RequestData` using a direct `From` impl.
#[napi(object)]
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
    /// Parsed request body
    pub body: Value,
    /// Extracted path parameters
    pub path_params: HashMap<String, String>,
}

impl From<&RequestData> for HandlerInput {
    fn from(data: &RequestData) -> Self {
        HandlerInput {
            method: data.method.clone(),
            path: data.path.clone(),
            headers: (*data.headers).clone(),
            cookies: (*data.cookies).clone(),
            query_params: data.query_params.clone(),
            body: data.body.clone(),
            path_params: (*data.path_params).clone(),
        }
    }
}

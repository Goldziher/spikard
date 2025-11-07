//! Handler trait for language-agnostic request handling
//!
//! This module defines the core trait that all language bindings must implement.
//! It's completely language-agnostic - no Python, Node, or WASM knowledge.

use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

/// Request data extracted from HTTP request
/// This is the language-agnostic representation passed to handlers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestData {
    pub path_params: HashMap<String, String>,
    pub query_params: Value,
    pub raw_query_params: HashMap<String, Vec<String>>,
    pub body: Value,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub method: String,
    pub path: String,
}

/// Result type for handlers
pub type HandlerResult = Result<Response<Body>, (StatusCode, String)>;

/// Handler trait that all language bindings must implement
///
/// This trait is completely language-agnostic. Each binding (Python, Node, WASM)
/// implements this trait to bridge their runtime to our HTTP server.
pub trait Handler: Send + Sync {
    /// Handle an HTTP request
    ///
    /// Takes the extracted request data and returns a future that resolves to either:
    /// - Ok(Response): A successful HTTP response
    /// - Err((StatusCode, String)): An error with status code and message
    fn call(
        &self,
        request: Request<Body>,
        request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>>;
}

/// Validated parameters from request (path, query, headers, cookies)
#[derive(Debug, Clone)]
pub struct ValidatedParams {
    pub params: HashMap<String, Value>,
}

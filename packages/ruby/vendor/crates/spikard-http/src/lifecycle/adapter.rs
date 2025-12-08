//! Shared utilities for lifecycle hook implementations across language bindings.
//!
//! This module provides common error messages, hook registration patterns, and
//! serialization utilities to eliminate duplication across Python, Node.js,
//! Ruby, and WASM bindings.

use crate::lifecycle::LifecycleHook;
use axum::body::Body;
use axum::http::{Request, Response};
use std::sync::Arc;

/// Standard error message formatters for lifecycle hooks.
/// These are used consistently across all language bindings.
pub mod error {
    use std::fmt::Display;

    /// Format error when a hook invocation fails
    pub fn call_failed(hook_name: &str, reason: impl Display) -> String {
        format!("Hook '{}' call failed: {}", hook_name, reason)
    }

    /// Format error when a task execution fails (tokio/threading)
    pub fn task_error(hook_name: &str, reason: impl Display) -> String {
        format!("Hook '{}' task error: {}", hook_name, reason)
    }

    /// Format error when a promise/future fails
    pub fn promise_failed(hook_name: &str, reason: impl Display) -> String {
        format!("Hook '{}' promise failed: {}", hook_name, reason)
    }

    /// Format error for Python-specific failures
    pub fn python_error(hook_name: &str, reason: impl Display) -> String {
        format!("Hook '{}' Python error: {}", hook_name, reason)
    }

    /// Format error when body reading fails
    pub fn body_read_failed(direction: &str, reason: impl Display) -> String {
        format!("Failed to read {} body: {}", direction, reason)
    }

    /// Format error when body writing fails
    pub fn body_write_failed(reason: impl Display) -> String {
        format!("Failed to write body: {}", reason)
    }

    /// Format error for serialization failures
    pub fn serialize_failed(context: &str, reason: impl Display) -> String {
        format!("Failed to serialize {}: {}", context, reason)
    }

    /// Format error for deserialization failures
    pub fn deserialize_failed(context: &str, reason: impl Display) -> String {
        format!("Failed to deserialize {}: {}", context, reason)
    }

    /// Format error when building HTTP objects fails
    pub fn build_failed(what: &str, reason: impl Display) -> String {
        format!("Failed to build {}: {}", what, reason)
    }
}

/// Utilities for serializing/deserializing request and response bodies
pub mod serial {
    use super::*;

    /// Extract body bytes from an axum Body
    pub async fn extract_body(body: Body) -> Result<bytes::Bytes, String> {
        use axum::body::to_bytes;
        to_bytes(body, usize::MAX)
            .await
            .map_err(|e| error::body_read_failed("request/response", e))
    }

    /// Create a JSON-formatted response body
    pub fn json_response_body(json: &serde_json::Value) -> Result<Body, String> {
        serde_json::to_string(json)
            .map(Body::from)
            .map_err(|e| error::serialize_failed("response JSON", e))
    }

    /// Parse a JSON value from bytes
    pub fn parse_json(bytes: &[u8]) -> Result<serde_json::Value, String> {
        if bytes.is_empty() {
            return Ok(serde_json::Value::Null);
        }
        serde_json::from_slice(bytes)
            .or_else(|_| Ok(serde_json::Value::String(String::from_utf8_lossy(bytes).to_string())))
    }
}

/// Re-export of the HTTP-specific lifecycle hooks type alias
pub use super::LifecycleHooks as HttpLifecycleHooks;

/// Helper for registering hooks with standard naming conventions
pub struct HookRegistry;

impl HookRegistry {
    /// Extract hooks from a configuration and register them with a naming pattern
    /// Used by bindings to standardize hook naming (e.g., "on_request_hook_0")
    pub fn register_from_list<F>(
        hooks: &mut HttpLifecycleHooks,
        hook_list: Vec<Arc<dyn LifecycleHook<Request<Body>, Response<Body>>>>,
        _hook_type: &str,
        register_fn: F,
    ) where
        F: Fn(&mut HttpLifecycleHooks, Arc<dyn LifecycleHook<Request<Body>, Response<Body>>>),
    {
        for hook in hook_list {
            register_fn(hooks, hook);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_messages() {
        let call_err = error::call_failed("test_hook", "test reason");
        assert!(call_err.contains("test_hook"));
        assert!(call_err.contains("test reason"));

        let task_err = error::task_error("task_hook", "spawn failed");
        assert!(task_err.contains("task_hook"));

        let promise_err = error::promise_failed("promise_hook", "rejected");
        assert!(promise_err.contains("promise_hook"));
    }

    #[test]
    fn test_body_error_messages() {
        let read_err = error::body_read_failed("request", "stream closed");
        assert!(read_err.contains("request"));

        let write_err = error::body_write_failed("allocation failed");
        assert!(write_err.contains("allocation"));
    }

    #[test]
    fn test_json_error_messages() {
        let ser_err = error::serialize_failed("request body", "invalid type");
        assert!(ser_err.contains("request body"));

        let deser_err = error::deserialize_failed("response", "malformed");
        assert!(deser_err.contains("response"));
    }
}

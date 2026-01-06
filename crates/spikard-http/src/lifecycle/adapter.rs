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
    use crate::lifecycle::HookResult;
    use axum::body::Body;
    use axum::http::{Request, Response, StatusCode};
    use std::future::Future;
    use std::pin::Pin;

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

    #[tokio::test]
    async fn serial_extract_body_roundtrips_bytes() {
        let body = Body::from("hello");
        let bytes = serial::extract_body(body).await.expect("extract body");
        assert_eq!(&bytes[..], b"hello");
    }

    #[test]
    fn serial_parse_json_handles_empty_valid_and_invalid_json() {
        let empty = serial::parse_json(&[]).expect("parse empty");
        assert_eq!(empty, serde_json::Value::Null);

        let valid = serial::parse_json(br#"{"ok":true}"#).expect("parse json");
        assert_eq!(valid["ok"], true);

        let invalid = serial::parse_json(b"not-json").expect("parse fallback");
        assert_eq!(invalid, serde_json::Value::String("not-json".to_string()));
    }

    #[test]
    fn hook_registry_registers_all_hooks_via_callback() {
        struct NoopHook {
            hook_name: String,
        }

        impl LifecycleHook<Request<Body>, Response<Body>> for NoopHook {
            fn name(&self) -> &str {
                &self.hook_name
            }

            fn execute_request<'a>(
                &self,
                req: Request<Body>,
            ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>, Response<Body>>, String>> + Send + 'a>>
            {
                Box::pin(async move { Ok(HookResult::Continue(req)) })
            }

            fn execute_response<'a>(
                &self,
                resp: Response<Body>,
            ) -> Pin<Box<dyn Future<Output = Result<HookResult<Response<Body>, Response<Body>>, String>> + Send + 'a>>
            {
                Box::pin(async move { Ok(HookResult::Continue(resp)) })
            }
        }

        let mut hooks = HttpLifecycleHooks::new();
        assert!(hooks.is_empty());

        let hook_list: Vec<Arc<dyn LifecycleHook<Request<Body>, Response<Body>>>> = vec![
            Arc::new(NoopHook {
                hook_name: "one".to_string(),
            }),
            Arc::new(NoopHook {
                hook_name: "two".to_string(),
            }),
        ];

        HookRegistry::register_from_list(&mut hooks, hook_list, "on_request", |hooks, hook| {
            hooks.add_on_request(hook);
        });

        let dbg = format!("{:?}", hooks);
        assert!(dbg.contains("on_request_count"));
        assert!(dbg.contains("2"));

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = futures::executor::block_on(hooks.execute_on_request(req)).expect("hook run");
        assert!(matches!(result, HookResult::Continue(_)));

        let resp = Response::builder().status(StatusCode::OK).body(Body::empty()).unwrap();
        let resp = futures::executor::block_on(hooks.execute_on_response(resp)).expect("hook run");
        assert_eq!(resp.status(), StatusCode::OK);
    }
}

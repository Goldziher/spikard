//! Elixir handler wrapper implementing the Handler trait.
//!
//! This module provides the `ElixirHandler` struct that wraps Elixir callback functions
//! (stored as ResourceArc) and implements Spikard's Handler trait for async request processing.
//! Uses Rustler terms for safe Rust-Elixir FFI communication.

#![allow(dead_code)]
#![deny(clippy::unwrap_used)]

use axum::body::Body;
use axum::http::{HeaderName, HeaderValue, Request, StatusCode};
use serde_json::{json, Value as JsonValue};
use spikard_bindings_shared::ErrorResponseBuilder;
use spikard_core::problem::ProblemDetails;
use spikard_http::SchemaValidator;
use spikard_http::{Handler, HandlerResult, RequestData};
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

/// Handler response payload with status, headers, and body data.
#[derive(Debug, Clone)]
pub struct HandlerResponsePayload {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Option<JsonValue>,
    pub raw_body: Option<Vec<u8>>,
}

impl HandlerResponsePayload {
    /// Create a new empty 200 OK response.
    pub fn ok() -> Self {
        Self {
            status: 200,
            headers: HashMap::new(),
            body: None,
            raw_body: None,
        }
    }

    /// Create a new response with JSON body.
    pub fn with_json(status: u16, body: JsonValue) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body: Some(body),
            raw_body: None,
        }
    }

    /// Add a header to the response.
    pub fn with_header(mut self, name: String, value: String) -> Self {
        self.headers.insert(name, value);
        self
    }
}

/// Inner state of an Elixir handler.
///
/// Stores the opaque Elixir callback function reference (as ResourceArc),
/// route metadata, and optional response validator.
pub struct ElixirHandlerInner {
    pub handler_callback: Arc<rustler::LocalPid>,
    pub handler_name: String,
    pub method: String,
    pub path: String,
    pub response_validator: Option<Arc<SchemaValidator>>,
}

/// Wrapper around an Elixir callback that implements the Handler trait.
///
/// The Elixir callback is stored as a LocalPid reference (wrapped in Arc for thread safety).
/// When the handler is called, this invokes the corresponding Elixir function
/// via the NIF (Native Implemented Function) boundary.
#[derive(Clone)]
pub struct ElixirHandler {
    pub inner: Arc<ElixirHandlerInner>,
}

impl ElixirHandler {
    /// Create a new ElixirHandler from a route and Elixir callback PID.
    ///
    /// # Arguments
    ///
    /// * `route` - The Route configuration containing method, path, and validators
    /// * `handler_pid` - The LocalPid of the Elixir handler process/function
    ///
    /// # Returns
    ///
    /// A new ElixirHandler ready to process requests.
    pub fn new(
        route: &spikard_http::Route,
        handler_pid: rustler::LocalPid,
    ) -> Result<Self, String> {
        let method = route.method.as_str().to_string();
        let path = route.path.clone();

        Ok(Self {
            inner: Arc::new(ElixirHandlerInner {
                handler_callback: Arc::new(handler_pid),
                handler_name: route.handler_name.clone(),
                method,
                path,
                response_validator: route.response_validator.clone(),
            }),
        })
    }

    /// Handle a request synchronously.
    ///
    /// This method:
    /// 1. Converts RequestData to an Elixir-friendly format
    /// 2. Calls the Elixir handler function
    /// 3. Interprets the response and validates it if needed
    /// 4. Returns an HTTP response
    pub fn handle(&self, _request_data: RequestData) -> HandlerResult {
        // Convert RequestData to response format.
        // For now, return a basic 200 OK response.
        // The actual Elixir callback invocation will happen in the NIF layer.
        let payload = HandlerResponsePayload::ok();

        // Apply response validation if configured
        if let Some(validator) = &self.inner.response_validator {
            if let Some(body) = &payload.body {
                if let Err(errors) = validator.validate(body) {
                    let problem = ProblemDetails::from_validation_error(&errors);
                    return Err(ErrorResponseBuilder::problem_details_response(&problem));
                }
            }
        }

        // Build the response
        self.payload_to_response(payload)
    }

    /// Convert a HandlerResponsePayload into an axum Response.
    fn payload_to_response(&self, payload: HandlerResponsePayload) -> HandlerResult {
        let HandlerResponsePayload {
            status,
            headers,
            body,
            raw_body,
        } = payload;

        let mut response_builder = axum::http::Response::builder().status(StatusCode::from_u16(status).unwrap_or(StatusCode::OK));
        let mut has_content_type = false;

        for (name, value) in headers.iter() {
            if name.eq_ignore_ascii_case("content-type") {
                has_content_type = true;
            }
            let header_name = HeaderName::from_bytes(name.as_bytes()).map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Invalid header name '{name}': {err}"),
                )
            })?;
            let header_value = HeaderValue::from_str(value).map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Invalid header value for '{name}': {err}"),
                )
            })?;

            response_builder = response_builder.header(header_name, header_value);
        }

        if !has_content_type && body.is_some() {
            response_builder = response_builder.header(
                HeaderName::from_static("content-type"),
                HeaderValue::from_static("application/json"),
            );
        }

        let body_bytes = if let Some(raw) = raw_body {
            raw
        } else if let Some(json_value) = body {
            serde_json::to_vec(&json_value).map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to serialize response body: {err}"),
                )
            })?
        } else {
            Vec::new()
        };

        response_builder.body(Body::from(body_bytes)).map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to build response: {err}"),
            )
        })
    }
}

impl Handler for ElixirHandler {
    fn call(
        &self,
        _req: Request<Body>,
        request_data: RequestData,
    ) -> Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send + '_>> {
        let handler = self.clone();
        Box::pin(async move { handler.handle(request_data) })
    }
}

// Helper functions for Elixir term conversion

/// Convert RequestData to a format suitable for Elixir.
///
/// Returns a map with keys:
/// - `:path_params` - HashMap<String, String>
/// - `:query_params` - serde_json::Value
/// - `:headers` - HashMap<String, String>
/// - `:cookies` - HashMap<String, String>
/// - `:body` - serde_json::Value
/// - `:raw_body` - Option<bytes::Bytes>
/// - `:method` - String
/// - `:path` - String
pub fn request_data_to_elixir_map(request_data: &RequestData) -> JsonValue {
    json!({
        "path_params": request_data.path_params.as_ref().clone(),
        "query_params": request_data.query_params.as_ref().clone(),
        "headers": request_data.headers.as_ref().clone(),
        "cookies": request_data.cookies.as_ref().clone(),
        "body": request_data.body.as_ref().clone(),
        "raw_body": request_data.raw_body.as_ref().map(|b| b.to_vec()),
        "method": request_data.method.clone(),
        "path": request_data.path.clone(),
    })
}

/// Interpret an Elixir handler response.
///
/// Expects a map with optional keys:
/// - `:status` - HTTP status code (default: 200)
/// - `:headers` - HashMap<String, String> (default: {})
/// - `:body` - serde_json::Value or raw bytes (default: null)
pub fn interpret_elixir_response(response_map: &JsonValue) -> Result<HandlerResponsePayload, String> {
    let status = response_map
        .get("status")
        .and_then(|v| v.as_u64())
        .unwrap_or(200) as u16;

    let headers = response_map
        .get("headers")
        .and_then(|v| serde_json::from_value::<HashMap<String, String>>(v.clone()).ok())
        .unwrap_or_default();

    let body = response_map.get("body").cloned();
    let raw_body = response_map
        .get("raw_body")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_u64())
                .map(|v| v as u8)
                .collect::<Vec<u8>>()
        });

    Ok(HandlerResponsePayload {
        status,
        headers,
        body,
        raw_body,
    })
}

/// Convert error details to a structured error response.
fn error_response(status: StatusCode, code: &str, message: impl Into<String>) -> (StatusCode, String) {
    let payload = json!({
        "error": code,
        "code": code,
        "message": message.into(),
        "details": {},
    });
    let body = serde_json::to_string(&payload)
        .unwrap_or_else(|_| format!(r#"{{"error":"{}","code":"{}","details":{{}}}}"#, code, code));
    (status, body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_response_payload_ok() {
        let payload = HandlerResponsePayload::ok();
        assert_eq!(payload.status, 200);
        assert!(payload.headers.is_empty());
        assert!(payload.body.is_none());
        assert!(payload.raw_body.is_none());
    }

    #[test]
    fn test_handler_response_payload_with_json() {
        let body = json!({"message": "Hello"});
        let payload = HandlerResponsePayload::with_json(201, body.clone());
        assert_eq!(payload.status, 201);
        assert_eq!(payload.body, Some(body));
    }

    #[test]
    fn test_handler_response_payload_with_header() {
        let payload = HandlerResponsePayload::ok()
            .with_header("x-custom".to_string(), "value".to_string());
        assert_eq!(payload.headers.get("x-custom"), Some(&"value".to_string()));
    }

    #[test]
    fn test_request_data_to_elixir_map() {
        let request_data = RequestData {
            path_params: Arc::new(HashMap::from([("id".to_string(), "123".to_string())])),
            query_params: Arc::new(json!({"search": "test"})),
            validated_params: None,
            raw_query_params: Arc::new(HashMap::new()),
            body: Arc::new(json!({"name": "Alice"})),
            raw_body: None,
            headers: Arc::new(HashMap::from([("content-type".to_string(), "application/json".to_string())])),
            cookies: Arc::new(HashMap::new()),
            method: "POST".to_string(),
            path: "/test".to_string(),
            dependencies: None,
        };

        let result = request_data_to_elixir_map(&request_data);
        assert!(result.is_object());
        assert_eq!(result["method"], "POST");
        assert_eq!(result["path"], "/test");
    }

    #[test]
    fn test_interpret_elixir_response_defaults() {
        let response = json!({});
        let payload = interpret_elixir_response(&response).unwrap();
        assert_eq!(payload.status, 200);
        assert!(payload.headers.is_empty());
    }

    #[test]
    fn test_interpret_elixir_response_with_status_and_body() {
        let response = json!({
            "status": 201,
            "body": {"id": 42}
        });
        let payload = interpret_elixir_response(&response).unwrap();
        assert_eq!(payload.status, 201);
        assert_eq!(payload.body, Some(json!({"id": 42})));
    }

    #[test]
    fn test_interpret_elixir_response_with_headers() {
        let response = json!({
            "status": 200,
            "headers": {"x-custom": "header-value"}
        });
        let payload = interpret_elixir_response(&response).unwrap();
        assert_eq!(payload.headers.get("x-custom"), Some(&"header-value".to_string()));
    }
}

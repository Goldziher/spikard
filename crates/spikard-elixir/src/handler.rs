//! Elixir handler wrapper implementing the Handler trait.
//!
//! This module provides the `ElixirHandler` struct that wraps Elixir callback functions
//! (stored as ResourceArc) and implements Spikard's Handler trait for async request processing.
//! Uses Rustler terms for safe Rust-Elixir FFI communication.

#![allow(dead_code)]
#![deny(clippy::unwrap_used)]

use axum::body::Body;
use axum::http::{HeaderName, HeaderValue, Request, StatusCode};
use once_cell::sync::Lazy;
use rustler::{Encoder, Env, LocalPid, NifResult, OwnedEnv, Term};
use serde_json::{Value as JsonValue, json};
use spikard_bindings_shared::ErrorResponseBuilder;
use spikard_core::problem::ProblemDetails;
use spikard_http::SchemaValidator;
use spikard_http::{Handler, HandlerResult, RequestData};
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::oneshot;
use tracing::{debug, warn};

use crate::atoms;
use crate::conversion::json_to_elixir;

/// Global map of pending handler requests keyed by unique reference ID.
/// Used to deliver responses from Elixir back to waiting Rust handlers.
static PENDING_REQUESTS: Lazy<Mutex<HashMap<u64, oneshot::Sender<JsonValue>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Counter for generating unique request IDs.
static REQUEST_ID_COUNTER: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));

/// Generate a unique request ID.
fn next_request_id() -> u64 {
    let mut counter = REQUEST_ID_COUNTER.lock().unwrap_or_else(|e| e.into_inner());
    *counter = counter.wrapping_add(1);
    *counter
}

/// Register a pending request and return its ID.
fn register_pending_request(sender: oneshot::Sender<JsonValue>) -> u64 {
    let id = next_request_id();
    let mut pending = PENDING_REQUESTS.lock().unwrap_or_else(|e| e.into_inner());
    pending.insert(id, sender);
    id
}

/// Deliver a response to a pending request by its ID.
/// Returns true if the request was found and response delivered.
pub fn deliver_response(request_id: u64, response: JsonValue) -> bool {
    let sender = {
        let mut pending = PENDING_REQUESTS.lock().unwrap_or_else(|e| e.into_inner());
        pending.remove(&request_id)
    };

    if let Some(tx) = sender {
        tx.send(response).is_ok()
    } else {
        warn!("No pending request found for ID {}", request_id);
        false
    }
}

/// NIF to deliver a handler response from Elixir back to the waiting Rust handler.
///
/// Called by the HandlerRunner GenServer after processing a request.
#[rustler::nif]
pub fn deliver_handler_response<'a>(env: Env<'a>, request_id: u64, response_map: Term<'a>) -> NifResult<Term<'a>> {
    // Convert Elixir term to JSON
    let response_json = crate::conversion::elixir_to_json(env, response_map)?;

    if deliver_response(request_id, response_json) {
        Ok(atoms::ok().encode(env))
    } else {
        Ok((atoms::error(), atoms::not_implemented()).encode(env))
    }
}

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
/// Stores the handler runner PID for callback invocation,
/// route metadata, and optional response validator.
pub struct ElixirHandlerInner {
    /// PID of the HandlerRunner GenServer that manages handler invocations
    pub handler_runner_pid: LocalPid,
    /// Name of the handler function to invoke
    pub handler_name: String,
    /// HTTP method for this route
    pub method: String,
    /// Path pattern for this route
    pub path: String,
    /// Optional response validator
    pub response_validator: Option<Arc<SchemaValidator>>,
    /// Timeout for handler invocation in milliseconds
    pub timeout_ms: u64,
}

/// Wrapper around an Elixir callback that implements the Handler trait.
///
/// When the handler is called, this sends a message to the HandlerRunner GenServer
/// which invokes the corresponding Elixir function and returns the response.
#[derive(Clone)]
pub struct ElixirHandler {
    pub inner: Arc<ElixirHandlerInner>,
}

/// Default timeout for handler invocations (30 seconds).
const DEFAULT_HANDLER_TIMEOUT_MS: u64 = 30_000;

impl ElixirHandler {
    /// Create a new ElixirHandler from a route and handler runner PID.
    ///
    /// # Arguments
    ///
    /// * `route` - The Route configuration containing method, path, and validators
    /// * `handler_runner_pid` - The LocalPid of the HandlerRunner GenServer
    ///
    /// # Returns
    ///
    /// A new ElixirHandler ready to process requests.
    pub fn new(route: &spikard_http::Route, handler_runner_pid: LocalPid) -> Result<Self, String> {
        let method = route.method.as_str().to_string();
        let path = route.path.clone();

        Ok(Self {
            inner: Arc::new(ElixirHandlerInner {
                handler_runner_pid,
                handler_name: route.handler_name.clone(),
                method,
                path,
                response_validator: route.response_validator.clone(),
                timeout_ms: DEFAULT_HANDLER_TIMEOUT_MS,
            }),
        })
    }

    /// Create a new ElixirHandler with custom timeout.
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        if let Some(inner) = Arc::get_mut(&mut self.inner) {
            inner.timeout_ms = timeout_ms;
        }
        self
    }

    /// Handle a request by invoking the Elixir handler via message passing.
    ///
    /// This method:
    /// 1. Converts RequestData to a JSON map
    /// 2. Creates a response channel
    /// 3. Sends a request message to the HandlerRunner
    /// 4. Waits for the response with timeout
    /// 5. Interprets the response and validates it if needed
    /// 6. Returns an HTTP response
    pub async fn handle(&self, request_data: RequestData) -> HandlerResult {
        debug!(
            "[HANDLER] handle() called for {} {}",
            self.inner.method, self.inner.path
        );

        // Convert RequestData to JSON for Elixir
        let request_json = request_data_to_elixir_map(&request_data);

        // Create response channel
        let (tx, rx) = oneshot::channel();
        let request_id = register_pending_request(tx);

        debug!(
            "Sending request {} to handler '{}' for {} {}",
            request_id, self.inner.handler_name, self.inner.method, self.inner.path
        );

        // Send request to HandlerRunner via OwnedEnv
        let handler_name = self.inner.handler_name.clone();
        let handler_runner_pid = self.inner.handler_runner_pid;
        let send_result = send_handler_request(handler_runner_pid, request_id, &handler_name, &request_json);

        if let Err(e) = send_result {
            warn!("Failed to send request to HandlerRunner: {}", e);
            return Err(ErrorResponseBuilder::structured_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "handler_send_error",
                format!("Failed to send request to handler: {}", e),
            ));
        }

        // Wait for response with timeout
        let timeout = Duration::from_millis(self.inner.timeout_ms);
        debug!(
            "[DEBUG] Waiting for response with timeout {:?} for request {}",
            timeout, request_id
        );

        let response_json = match tokio::time::timeout(timeout, rx).await {
            Ok(Ok(json)) => json,
            Ok(Err(_)) => {
                warn!(
                    "Handler response channel closed unexpectedly for request {}",
                    request_id
                );
                return Err(ErrorResponseBuilder::structured_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "handler_channel_error",
                    "Handler response channel closed unexpectedly",
                ));
            }
            Err(_) => {
                warn!("Handler request timed out for request {}", request_id);
                return Err(ErrorResponseBuilder::structured_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "handler_timeout",
                    "Handler request timed out",
                ));
            }
        };

        debug!("Received response for request {}: {:?}", request_id, response_json);

        // Interpret the response
        let payload = match interpret_elixir_response(&response_json) {
            Ok(p) => p,
            Err(e) => {
                warn!("Failed to interpret handler response: {}", e);
                return Err(ErrorResponseBuilder::structured_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "response_interpret_error",
                    format!("Failed to interpret handler response: {}", e),
                ));
            }
        };

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

        debug!(
            "[DEBUG] payload_to_response: status={}, headers_count={}",
            status,
            headers.len()
        );

        let mut response_builder =
            axum::http::Response::builder().status(StatusCode::from_u16(status).unwrap_or(StatusCode::OK));
        let mut has_content_type = false;

        for (name, value) in headers.iter() {
            debug!("[DEBUG] Adding header: {} = {}", name, value);
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
            debug!("[DEBUG] Using raw body, {} bytes", raw.len());
            raw
        } else if let Some(json_value) = body {
            // For plain strings, use the string bytes directly without JSON encoding
            // to avoid double-quoting (e.g., "hello" becoming "\"hello\"")
            if let JsonValue::String(s) = json_value {
                debug!("[DEBUG] Using plain string body: {} bytes", s.len());
                s.into_bytes()
            } else {
                let bytes = serde_json::to_vec(&json_value).map_err(|err| {
                    debug!("[DEBUG] Failed to serialize body: {}", err);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to serialize response body: {err}"),
                    )
                })?;
                debug!("[DEBUG] Serialized JSON body: {} bytes", bytes.len());
                bytes
            }
        } else {
            debug!("[DEBUG] Empty body");
            Vec::new()
        };

        debug!("[DEBUG] Building final response with {} bytes body", body_bytes.len());
        let result = response_builder.body(Body::from(body_bytes)).map_err(|err| {
            debug!("[DEBUG] Failed to build response: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to build response: {err}"),
            )
        });
        debug!("[DEBUG] Response built successfully: {}", result.is_ok());
        result
    }
}

impl Handler for ElixirHandler {
    fn call(
        &self,
        _req: Request<Body>,
        request_data: RequestData,
    ) -> Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send + '_>> {
        debug!("[HANDLER] call() invoked for {} {}", self.inner.method, self.inner.path);
        let handler = self.clone();
        Box::pin(async move {
            debug!("[HANDLER] async block executing");
            handler.handle(request_data).await
        })
    }
}

/// Send a handler request to the Elixir HandlerRunner via OwnedEnv.
///
/// This function creates an OwnedEnv, builds a message tuple, and sends it
/// to the HandlerRunner GenServer. The message format is:
/// `{:"$gen_call", {from_pid, ref}, {:invoke, request_id, handler_name, request_map}}`
///
/// Note: This must be called from a thread NOT managed by the Erlang VM.
fn send_handler_request(
    handler_runner_pid: LocalPid,
    request_id: u64,
    handler_name: &str,
    request_json: &JsonValue,
) -> Result<(), String> {
    // Create an owned environment for building terms
    let owned_env = OwnedEnv::new();

    owned_env.run(|env| {
        // Build the request message
        // Format: {:handle_request, request_id, handler_name, request_map}
        let request_atom =
            rustler::Atom::from_str(env, "handle_request").map_err(|_| "Failed to create request atom".to_string())?;

        let request_id_term = request_id.encode(env);
        let handler_name_term = handler_name.encode(env);

        // Convert JSON to Elixir term
        let request_map_term =
            json_to_elixir(env, request_json).map_err(|e| format!("Failed to convert request to Elixir: {:?}", e))?;

        // Build the message tuple
        let message = (request_atom, request_id_term, handler_name_term, request_map_term).encode(env);

        // Send to the handler runner
        if env.send(&handler_runner_pid, message).is_err() {
            return Err("Failed to send message to handler runner".to_string());
        }

        Ok(())
    })
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
    let status = response_map.get("status").and_then(|v| v.as_u64()).unwrap_or(200) as u16;

    let headers = response_map
        .get("headers")
        .and_then(|v| serde_json::from_value::<HashMap<String, String>>(v.clone()).ok())
        .unwrap_or_default();

    let body = response_map.get("body").cloned();
    let raw_body = response_map.get("raw_body").and_then(|v| v.as_array()).map(|arr| {
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
        let payload = HandlerResponsePayload::ok().with_header("x-custom".to_string(), "value".to_string());
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
            headers: Arc::new(HashMap::from([(
                "content-type".to_string(),
                "application/json".to_string(),
            )])),
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
        let payload = interpret_elixir_response(&response).expect("empty response should use defaults");
        assert_eq!(payload.status, 200);
        assert!(payload.headers.is_empty());
    }

    #[test]
    fn test_interpret_elixir_response_with_status_and_body() {
        let response = json!({
            "status": 201,
            "body": {"id": 42}
        });
        let payload = interpret_elixir_response(&response).expect("valid status/body should parse");
        assert_eq!(payload.status, 201);
        assert_eq!(payload.body, Some(json!({"id": 42})));
    }

    #[test]
    fn test_interpret_elixir_response_with_headers() {
        let response = json!({
            "status": 200,
            "headers": {"x-custom": "header-value"}
        });
        let payload = interpret_elixir_response(&response).expect("valid headers should parse");
        assert_eq!(payload.headers.get("x-custom"), Some(&"header-value".to_string()));
    }
}

//! JSON-RPC HTTP handler for processing JSON-RPC requests over HTTP
//!
//! This module provides the HTTP endpoint handler that accepts JSON-RPC requests
//! and routes them through the JSON-RPC router. It handles single and batch requests,
//! validates content-type headers, and returns properly formatted JSON-RPC responses.
//!
//! # Request Processing
//!
//! The handler:
//! 1. Validates that the Content-Type header is `application/json`
//! 2. Parses the request body as JSON-RPC 2.0 (single or batch)
//! 3. Routes requests through the JsonRpcRouter
//! 4. Returns HTTP 200 with JSON-RPC responses
//! 5. Returns appropriate HTTP error codes for non-JSON-RPC errors
//!
//! # Response Codes
//!
//! - HTTP 200: Valid JSON-RPC request (even if the response contains an error)
//! - HTTP 400: Parse error or invalid request format
//! - HTTP 415: Invalid Content-Type header
//!
//! # Example
//!
//! ```ignore
//! use axum::{
//!     routing::post,
//!     Router,
//! };
//! use std::sync::Arc;
//! use spikard_http::jsonrpc::{JsonRpcRouter, JsonRpcMethodRegistry};
//!
//! let registry = Arc::new(JsonRpcMethodRegistry::new());
//! let router = Arc::new(JsonRpcRouter::new(registry, true, 100));
//! let state = Arc::new(JsonRpcState { router });
//!
//! let app = Router::new()
//!     .route("/rpc", post(handle_jsonrpc))
//!     .with_state(state);
//! ```

use super::router::{JsonRpcRequestOrBatch, JsonRpcRouter};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response as AxumResponse},
};
use std::sync::Arc;

/// State passed to the JSON-RPC HTTP handler
///
/// Contains the shared JSON-RPC router instance that dispatches requests
/// to registered method handlers.
#[derive(Clone)]
pub struct JsonRpcState {
    /// The JSON-RPC router for handling requests
    pub router: Arc<JsonRpcRouter>,
}

/// Main JSON-RPC HTTP handler
///
/// Accepts POST requests with JSON-RPC 2.0 payloads (single or batch).
/// Always returns HTTP 200 for valid JSON-RPC requests, with JSON-RPC error
/// codes in the response body if the method invocation failed.
///
/// # Arguments
///
/// * `state` - The application state containing the JSON-RPC router
/// * `headers` - HTTP request headers (used for Content-Type validation)
/// * `body` - The raw request body as a string
///
/// # Returns
///
/// An Axum response with:
/// - HTTP 200 and JSON-RPC response for valid JSON-RPC requests
/// - HTTP 415 if Content-Type is not application/json
/// - HTTP 400 if the request body cannot be parsed as JSON-RPC
///
/// # Example
///
/// Valid single request:
/// ```text
/// POST /rpc HTTP/1.1
/// Content-Type: application/json
///
/// {"jsonrpc":"2.0","method":"add","params":[1,2],"id":1}
/// ```
///
/// Valid batch request:
/// ```text
/// POST /rpc HTTP/1.1
/// Content-Type: application/json
///
/// [{"jsonrpc":"2.0","method":"add","params":[1,2],"id":1},
///  {"jsonrpc":"2.0","method":"multiply","params":[3,4],"id":2}]
/// ```
pub async fn handle_jsonrpc(State(state): State<Arc<JsonRpcState>>, headers: HeaderMap, body: String) -> AxumResponse {
    // Validate Content-Type header
    if !validate_content_type(&headers) {
        return create_error_response(
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "Content-Type must be application/json",
        );
    }

    // Parse request (single or batch)
    let request = match JsonRpcRouter::parse_request(&body) {
        Ok(req) => req,
        Err(error_response) => {
            let json = serde_json::to_string(&error_response).expect("Error serialization should never fail");
            return create_jsonrpc_response(json);
        }
    };

    // Route request based on type
    let response = match request {
        JsonRpcRequestOrBatch::Single(req) => {
            let response = state.router.route_single(req).await;
            serde_json::to_string(&response).expect("Response serialization should never fail")
        }
        JsonRpcRequestOrBatch::Batch(batch) => match state.router.route_batch(batch).await {
            Ok(responses) => serde_json::to_string(&responses).expect("Batch response serialization should never fail"),
            Err(error_response) => {
                serde_json::to_string(&error_response).expect("Error serialization should never fail")
            }
        },
    };

    create_jsonrpc_response(response)
}

/// Validates that the Content-Type header is application/json
///
/// Performs case-insensitive matching per HTTP specification.
///
/// # Arguments
///
/// * `headers` - The HTTP headers to validate
///
/// # Returns
///
/// `true` if Content-Type is application/json or absent (defaults to JSON),
/// `false` if Content-Type is present but not JSON
fn validate_content_type(headers: &HeaderMap) -> bool {
    match headers.get(header::CONTENT_TYPE) {
        None => true, // Default to JSON if not specified
        Some(value) => {
            if let Ok(ct) = value.to_str() {
                ct.to_lowercase().contains("application/json")
            } else {
                false
            }
        }
    }
}

/// Creates a JSON-RPC response with proper HTTP headers
///
/// Returns HTTP 200 OK with Content-Type: application/json
///
/// # Arguments
///
/// * `json` - The JSON response body as a string
///
/// # Returns
///
/// An Axum response ready to send to the client
fn create_jsonrpc_response(json: String) -> AxumResponse {
    (StatusCode::OK, [(header::CONTENT_TYPE, "application/json")], json).into_response()
}

/// Creates a generic error response for HTTP-level errors
///
/// Returns an appropriate HTTP status code with plain text error message.
/// Used for Content-Type validation failures and other HTTP-level errors.
///
/// # Arguments
///
/// * `status` - The HTTP status code
/// * `message` - The error message as plain text
///
/// # Returns
///
/// An Axum response ready to send to the client
fn create_error_response(status: StatusCode, message: &str) -> AxumResponse {
    (
        status,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        message.to_string(),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jsonrpc::{method_registry::JsonRpcMethodRegistry, router::JsonRpcRouter};
    use serde_json::json;

    /// Helper to create a test state
    fn create_test_state() -> Arc<JsonRpcState> {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        let router = Arc::new(JsonRpcRouter::new(registry, true, 100));
        Arc::new(JsonRpcState { router })
    }

    /// Helper to create headers with JSON content type
    fn create_json_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
        headers
    }

    /// Helper to create headers with wrong content type
    fn create_wrong_content_type_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());
        headers
    }

    /// Helper to create empty headers
    fn create_empty_headers() -> HeaderMap {
        HeaderMap::new()
    }

    #[tokio::test]
    async fn test_handle_jsonrpc_single_request() {
        let state = create_test_state();
        let headers = create_json_headers();
        let body = r#"{"jsonrpc":"2.0","method":"test.method","params":{},"id":1}"#.to_string();

        let response = handle_jsonrpc(State(state), headers, body).await;

        // Should return 200 OK
        assert_eq!(response.status(), StatusCode::OK);

        // Should have JSON content type
        let content_type = response.headers().get(header::CONTENT_TYPE).unwrap().to_str().unwrap();
        assert!(content_type.contains("application/json"));
    }

    #[tokio::test]
    async fn test_handle_jsonrpc_batch_request() {
        let state = create_test_state();
        let headers = create_json_headers();
        let body = r#"[
            {"jsonrpc":"2.0","method":"test.method","params":{},"id":1},
            {"jsonrpc":"2.0","method":"test.method","params":{},"id":2}
        ]"#
        .to_string();

        let response = handle_jsonrpc(State(state), headers, body).await;

        // Should return 200 OK
        assert_eq!(response.status(), StatusCode::OK);

        // Should have JSON content type
        let content_type = response.headers().get(header::CONTENT_TYPE).unwrap().to_str().unwrap();
        assert!(content_type.contains("application/json"));
    }

    #[tokio::test]
    async fn test_invalid_content_type() {
        let state = create_test_state();
        let headers = create_wrong_content_type_headers();
        let body = r#"{"jsonrpc":"2.0","method":"test","id":1}"#.to_string();

        let response = handle_jsonrpc(State(state), headers, body).await;

        // Should return 415 Unsupported Media Type
        assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);

        // Should have plain text content type for error
        let content_type = response.headers().get(header::CONTENT_TYPE).unwrap().to_str().unwrap();
        assert!(content_type.contains("text/plain"));
    }

    #[tokio::test]
    async fn test_missing_content_type_defaults_to_json() {
        let state = create_test_state();
        let headers = create_empty_headers();
        let body = r#"{"jsonrpc":"2.0","method":"test.method","params":{},"id":1}"#.to_string();

        let response = handle_jsonrpc(State(state), headers, body).await;

        // Should return 200 OK (no Content-Type is valid, defaults to JSON)
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_invalid_json_parse_error() {
        let state = create_test_state();
        let headers = create_json_headers();
        let body = r#"{"invalid json"}"#.to_string();

        let response = handle_jsonrpc(State(state), headers, body).await;

        // Should return 200 OK with parse error in JSON-RPC response
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_notification_in_batch() {
        let state = create_test_state();
        let headers = create_json_headers();
        // Batch with both regular requests and a notification
        let body = r#"[
            {"jsonrpc":"2.0","method":"test","params":{},"id":1},
            {"jsonrpc":"2.0","method":"test","params":{}},
            {"jsonrpc":"2.0","method":"test","params":{},"id":2}
        ]"#
        .to_string();

        let response = handle_jsonrpc(State(state), headers, body).await;

        // Should return 200 OK
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn test_validate_content_type_valid() {
        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
        assert!(validate_content_type(&headers));
    }

    #[test]
    fn test_validate_content_type_valid_with_charset() {
        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, "application/json; charset=utf-8".parse().unwrap());
        assert!(validate_content_type(&headers));
    }

    #[test]
    fn test_validate_content_type_invalid() {
        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());
        assert!(!validate_content_type(&headers));
    }

    #[test]
    fn test_validate_content_type_missing() {
        let headers = HeaderMap::new();
        // Missing Content-Type defaults to valid
        assert!(validate_content_type(&headers));
    }

    #[test]
    fn test_create_jsonrpc_response() {
        let json = r#"{"jsonrpc":"2.0","result":42,"id":1}"#.to_string();
        let response = create_jsonrpc_response(json);

        assert_eq!(response.status(), StatusCode::OK);
        let content_type = response.headers().get(header::CONTENT_TYPE).unwrap().to_str().unwrap();
        assert_eq!(content_type, "application/json");
    }

    #[test]
    fn test_create_error_response() {
        let response = create_error_response(StatusCode::UNSUPPORTED_MEDIA_TYPE, "Invalid content type");

        assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
        let content_type = response.headers().get(header::CONTENT_TYPE).unwrap().to_str().unwrap();
        assert!(content_type.contains("text/plain"));
    }

    #[tokio::test]
    async fn test_method_not_found_in_single_request() {
        let state = create_test_state();
        let headers = create_json_headers();
        let body = r#"{"jsonrpc":"2.0","method":"nonexistent.method","params":{},"id":1}"#.to_string();

        let response = handle_jsonrpc(State(state), headers, body).await;

        // Should return 200 OK with error in response
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_batch_disabled() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        let router = Arc::new(JsonRpcRouter::new(registry, false, 100)); // batch disabled
        let state = Arc::new(JsonRpcState { router });
        let headers = create_json_headers();
        let body = r#"[
            {"jsonrpc":"2.0","method":"test","params":{},"id":1}
        ]"#
        .to_string();

        let response = handle_jsonrpc(State(state), headers, body).await;

        // Should return 200 OK with batch error in response
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_batch_size_exceeded() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        let router = Arc::new(JsonRpcRouter::new(registry, true, 2)); // max 2 requests
        let state = Arc::new(JsonRpcState { router });
        let headers = create_json_headers();
        let body = r#"[
            {"jsonrpc":"2.0","method":"test","params":{},"id":1},
            {"jsonrpc":"2.0","method":"test","params":{},"id":2},
            {"jsonrpc":"2.0","method":"test","params":{},"id":3}
        ]"#
        .to_string();

        let response = handle_jsonrpc(State(state), headers, body).await;

        // Should return 200 OK with batch size error in response
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_empty_batch() {
        let state = create_test_state();
        let headers = create_json_headers();
        let body = r#"[]"#.to_string();

        let response = handle_jsonrpc(State(state), headers, body).await;

        // Should return 200 OK with empty batch error in response
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_response_with_params() {
        let state = create_test_state();
        let headers = create_json_headers();
        let params = json!({"key": "value", "number": 42});
        let body = serde_json::to_string(&json!({
            "jsonrpc": "2.0",
            "method": "test.method",
            "params": params,
            "id": 1
        }))
        .unwrap();

        let response = handle_jsonrpc(State(state), headers, body).await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_content_type_case_insensitive() {
        let state = create_test_state();
        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, "Application/JSON".parse().unwrap());
        let body = r#"{"jsonrpc":"2.0","method":"test","id":1}"#.to_string();

        let response = handle_jsonrpc(State(state), headers, body).await;

        // Should work with different case
        assert_eq!(response.status(), StatusCode::OK);
    }
}

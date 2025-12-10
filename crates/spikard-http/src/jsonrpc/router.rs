//! JSON-RPC request router for handling single and batch requests
//!
//! This module provides the core routing logic for JSON-RPC 2.0 requests, including
//! support for batch processing with configurable size limits. The router matches
//! incoming requests to registered method handlers and returns appropriately formatted
//! responses according to the JSON-RPC 2.0 specification.
//!
//! # Features
//!
//! - Single request routing to registered handlers
//! - Batch request processing with size validation
//! - Notification handling (requests without IDs)
//! - Comprehensive error handling for all JSON-RPC error codes
//! - Thread-safe access via Arc<JsonRpcMethodRegistry>
//!
//! # Example
//!
//! ```ignore
//! use spikard_http::jsonrpc::{JsonRpcRouter, JsonRpcMethodRegistry, JsonRpcRequest};
//! use std::sync::Arc;
//! use serde_json::json;
//!
//! let registry = Arc::new(JsonRpcMethodRegistry::new());
//! let router = JsonRpcRouter::new(registry, true, 50);
//!
//! // Route a single request
//! let request = JsonRpcRequest::new("user.getById", Some(json!({"id": "123"})), Some(json!(1)));
//! let response = router.route_single(request).await;
//! ```

use super::method_registry::JsonRpcMethodRegistry;
use super::protocol::*;
use crate::handler_trait::RequestData;
use axum::body::Body;
use axum::http::Request;
use serde_json::Value;
use std::sync::Arc;

/// JSON-RPC router for handling single and batch requests
///
/// Manages request routing to registered method handlers with support for
/// batch processing, notifications, and comprehensive error handling.
pub struct JsonRpcRouter {
    /// Registry of available methods and their handlers
    registry: Arc<JsonRpcMethodRegistry>,
    /// Whether batch requests are enabled
    enable_batch: bool,
    /// Maximum number of requests allowed in a single batch
    max_batch_size: usize,
}

impl JsonRpcRouter {
    /// Creates a new JSON-RPC router
    ///
    /// # Arguments
    ///
    /// * `registry` - The method registry containing registered handlers
    /// * `enable_batch` - Whether to allow batch requests
    /// * `max_batch_size` - Maximum number of requests per batch
    ///
    /// # Example
    ///
    /// ```ignore
    /// use spikard_http::jsonrpc::{JsonRpcRouter, JsonRpcMethodRegistry};
    /// use std::sync::Arc;
    ///
    /// let registry = Arc::new(JsonRpcMethodRegistry::new());
    /// let router = JsonRpcRouter::new(registry, true, 100);
    /// ```
    pub fn new(registry: Arc<JsonRpcMethodRegistry>, enable_batch: bool, max_batch_size: usize) -> Self {
        Self {
            registry,
            enable_batch,
            max_batch_size,
        }
    }

    /// Routes a single JSON-RPC request to its handler
    ///
    /// Processes a single request by:
    /// 1. Checking if the method exists in the registry
    /// 2. Handling notifications (requests without IDs)
    /// 3. Invoking the handler with the HTTP request context
    /// 4. Converting handler responses to JSON-RPC format
    /// 5. Returning appropriately formatted responses
    ///
    /// For notifications, the server MUST NOT send a response.
    /// The response is still generated but marked as not-to-be-sent by the caller.
    ///
    /// # Arguments
    ///
    /// * `request` - The JSON-RPC request to route
    /// * `http_request` - The HTTP request context (headers, method, etc.)
    /// * `request_data` - Reference to extracted request data (params, body, etc.)
    ///
    /// # Returns
    ///
    /// A `JsonRpcResponseType` containing either a success response with the
    /// handler's result or an error response if the method is not found or
    /// the handler fails
    pub async fn route_single(
        &self,
        request: JsonRpcRequest,
        http_request: Request<Body>,
        request_data: &RequestData,
    ) -> JsonRpcResponseType {
        // Check if method exists in registry
        let handler = match self.registry.get(&request.method) {
            Ok(Some(h)) => h,
            Ok(None) => {
                let id = request.id.unwrap_or(Value::Null);
                return JsonRpcResponseType::Error(JsonRpcErrorResponse::error(
                    error_codes::METHOD_NOT_FOUND,
                    "Method not found",
                    id,
                ));
            }
            Err(e) => {
                let id = request.id.unwrap_or(Value::Null);
                return JsonRpcResponseType::Error(JsonRpcErrorResponse::error(
                    error_codes::INTERNAL_ERROR,
                    format!("Internal error: {}", e),
                    id,
                ));
            }
        };

        // If notification (no id), we still call the handler but the response won't be sent
        // (is_notification is tracked for logging/debugging purposes but the response is still generated)
        let _is_notification = request.is_notification();

        // Invoke the handler with a clone of request_data
        // Note: We only clone here, not in the batch loop, so batch requests avoid O(N) clones
        let handler_result = handler.call(http_request, request_data.clone()).await;

        // Handle handler result and convert to JSON-RPC response
        match handler_result {
            Ok(response) => {
                // Extract response body for JSON-RPC result
                let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                    .await
                    .unwrap_or_default();

                let result = if body_bytes.is_empty() {
                    Value::Null
                } else {
                    // Try to parse as JSON, fall back to string if not valid JSON
                    match serde_json::from_slice::<Value>(&body_bytes) {
                        Ok(json_val) => json_val,
                        Err(_) => Value::String(
                            String::from_utf8(body_bytes.to_vec()).unwrap_or_else(|_| "[binary data]".to_string()),
                        ),
                    }
                };

                let id = request.id.unwrap_or(Value::Null);
                JsonRpcResponseType::Success(JsonRpcResponse::success(result, id))
            }
            Err((_status, error_msg)) => {
                // Handler error: return JSON-RPC internal error
                let id = request.id.unwrap_or(Value::Null);
                let error_data = serde_json::json!({
                    "details": error_msg
                });
                JsonRpcResponseType::Error(JsonRpcErrorResponse::error_with_data(
                    error_codes::INTERNAL_ERROR,
                    "Internal error from handler",
                    error_data,
                    id,
                ))
            }
        }
    }

    /// Routes a batch of JSON-RPC requests
    ///
    /// Processes a batch of requests by:
    /// 1. Checking if batch processing is enabled
    /// 2. Validating batch size doesn't exceed the limit
    /// 3. Ensuring batch is not empty
    /// 4. Routing each request in sequence
    /// 5. Filtering out notification responses
    ///
    /// According to JSON-RPC 2.0 spec, the server SHOULD process all requests
    /// in the batch and return a JSON array of responses. Responses for
    /// notifications (requests without IDs) are not included in the result.
    ///
    /// # Arguments
    ///
    /// * `batch` - A vector of JSON-RPC requests
    /// * `http_request` - The HTTP request context (shared for all batch requests)
    /// * `request_data` - Extracted request data (shared for all batch requests)
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<JsonRpcResponseType>)` - Array of responses for all non-notification requests
    /// * `Err(JsonRpcErrorResponse)` - Single error if batch validation fails
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Batch requests are not enabled
    /// - Batch size exceeds the configured maximum
    /// - Batch is empty
    pub async fn route_batch(
        &self,
        batch: Vec<JsonRpcRequest>,
        http_request: Request<Body>,
        request_data: &RequestData,
    ) -> Result<Vec<JsonRpcResponseType>, JsonRpcErrorResponse> {
        // Check if batch requests are enabled
        if !self.enable_batch {
            return Err(JsonRpcErrorResponse::error(
                error_codes::INVALID_REQUEST,
                "Batch requests not enabled",
                Value::Null,
            ));
        }

        // Check batch size doesn't exceed maximum
        if batch.len() > self.max_batch_size {
            return Err(JsonRpcErrorResponse::error(
                error_codes::INVALID_REQUEST,
                format!("Batch size {} exceeds maximum {}", batch.len(), self.max_batch_size),
                Value::Null,
            ));
        }

        // Check batch is not empty
        if batch.is_empty() {
            return Err(JsonRpcErrorResponse::error(
                error_codes::INVALID_REQUEST,
                "Batch request cannot be empty",
                Value::Null,
            ));
        }

        // Preserve original request context (headers, extensions, URI, method)
        // Extract parts from the original request to preserve authentication and tracing context
        let (base_parts, _body) = http_request.into_parts();

        // Route each request and collect responses
        let mut responses = Vec::with_capacity(batch.len());
        for request in batch {
            // Skip notifications - they should not produce responses
            let is_notification = request.is_notification();

            // Create a new request for this batch item, preserving:
            // - Original URI (for routing context)
            // - Original HTTP method (POST for JSON-RPC)
            // - Original headers (contains Authorization, Cookie, Content-Type, etc.)
            // - Original extensions (contains request IDs, tracing context, auth claims)
            let req_for_handler = Request::from_parts(base_parts.clone(), Body::empty());

            let response = self.route_single(request, req_for_handler, request_data).await;

            // Only include non-notification responses
            if !is_notification {
                responses.push(response);
            }
        }

        Ok(responses)
    }

    /// Parses a JSON string into either a single request or a batch
    ///
    /// Attempts to deserialize the input as a single JSON-RPC request first,
    /// then tries batch parsing if that fails. Returns a parse error if both
    /// attempts fail.
    ///
    /// # Arguments
    ///
    /// * `body` - The raw JSON request body as a string
    ///
    /// # Returns
    ///
    /// * `Ok(JsonRpcRequestOrBatch::Single(req))` - Parsed single request
    /// * `Ok(JsonRpcRequestOrBatch::Batch(requests))` - Parsed batch request
    /// * `Err(JsonRpcErrorResponse)` - Parse error
    ///
    /// # Example
    ///
    /// ```ignore
    /// let single_json = r#"{"jsonrpc":"2.0","method":"test","id":1}"#;
    /// let parsed = JsonRpcRouter::parse_request(single_json);
    /// assert!(parsed.is_ok());
    ///
    /// let batch_json = r#"[{"jsonrpc":"2.0","method":"test","id":1}]"#;
    /// let parsed = JsonRpcRouter::parse_request(batch_json);
    /// assert!(parsed.is_ok());
    /// ```
    pub fn parse_request(body: &str) -> Result<JsonRpcRequestOrBatch, Box<JsonRpcErrorResponse>> {
        // Try to parse as single request first
        if let Ok(request) = serde_json::from_str::<JsonRpcRequest>(body) {
            // Validate method name
            if let Err(validation_error) = super::protocol::validate_method_name(&request.method) {
                let id = request.id.unwrap_or(Value::Null);
                return Err(Box::new(JsonRpcErrorResponse::error(
                    error_codes::INVALID_REQUEST,
                    format!("Invalid method name: {}", validation_error),
                    id,
                )));
            }
            return Ok(JsonRpcRequestOrBatch::Single(request));
        }

        // Try to parse as batch
        if let Ok(batch) = serde_json::from_str::<Vec<JsonRpcRequest>>(body) {
            // Validate all method names in the batch
            for request in &batch {
                if let Err(validation_error) = super::protocol::validate_method_name(&request.method) {
                    let id = request.id.clone().unwrap_or(Value::Null);
                    return Err(Box::new(JsonRpcErrorResponse::error(
                        error_codes::INVALID_REQUEST,
                        format!("Invalid method name: {}", validation_error),
                        id,
                    )));
                }
            }
            return Ok(JsonRpcRequestOrBatch::Batch(batch));
        }

        // Neither single nor batch - parse error
        Err(Box::new(JsonRpcErrorResponse::error(
            error_codes::PARSE_ERROR,
            "Parse error",
            Value::Null,
        )))
    }
}

/// Represents either a single JSON-RPC request or a batch of requests
///
/// Used to distinguish between single and batch requests after parsing,
/// allowing different routing logic for each case.
#[derive(Debug)]
pub enum JsonRpcRequestOrBatch {
    /// A single JSON-RPC request
    Single(JsonRpcRequest),
    /// A batch (array) of JSON-RPC requests
    Batch(Vec<JsonRpcRequest>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handler_trait::{Handler, HandlerResult, RequestData};
    use axum::body::Body;
    use axum::http::Request;
    use serde_json::json;
    use std::collections::HashMap;
    use std::sync::Arc;

    /// Helper function to create minimal RequestData for tests
    fn create_test_request_data() -> RequestData {
        RequestData {
            path_params: Arc::new(HashMap::new()),
            query_params: Value::Object(serde_json::Map::new()),
            raw_query_params: Arc::new(HashMap::new()),
            body: Value::Null,
            raw_body: None,
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            method: "POST".to_string(),
            path: "/rpc".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        }
    }

    /// Helper function to create a test HTTP request
    fn create_test_http_request() -> Request<Body> {
        Request::builder()
            .method("POST")
            .uri("/rpc")
            .body(Body::empty())
            .unwrap()
    }

    /// Mock handler that returns success with JSON
    struct MockSuccessHandler;

    impl Handler for MockSuccessHandler {
        fn call(
            &self,
            _request: Request<Body>,
            _request_data: RequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send + '_>> {
            Box::pin(async {
                use axum::response::Response;
                let response = Response::builder()
                    .status(200)
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"result":"success"}"#))
                    .unwrap();
                Ok(response)
            })
        }
    }

    /// Mock handler that returns an error
    struct MockErrorHandler;

    impl Handler for MockErrorHandler {
        fn call(
            &self,
            _request: Request<Body>,
            _request_data: RequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send + '_>> {
            Box::pin(async {
                Err((
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Handler error".to_string(),
                ))
            })
        }
    }

    #[tokio::test]
    async fn test_route_single_method_not_found() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        let router = JsonRpcRouter::new(registry, true, 100);

        let request = JsonRpcRequest::new("unknown_method", None, Some(json!(1)));
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let response = router.route_single(request, http_request, &request_data).await;

        match response {
            JsonRpcResponseType::Error(err) => {
                assert_eq!(err.error.code, error_codes::METHOD_NOT_FOUND);
                assert_eq!(err.id, json!(1));
            }
            _ => panic!("Expected error response"),
        }
    }

    #[tokio::test]
    async fn test_route_single_notification() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        // Register a method so we get past the not found check
        let handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("notify_method");
        registry.register("notify_method", handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let request = JsonRpcRequest::new("notify_method", None, None);
        assert!(request.is_notification());

        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let response = router.route_single(request, http_request, &request_data).await;

        // Notifications should return success (but won't be sent to client)
        match response {
            JsonRpcResponseType::Success(resp) => {
                assert_eq!(resp.id, Value::Null);
            }
            _ => panic!("Expected success response for notification"),
        }
    }

    #[tokio::test]
    async fn test_route_single_with_handler_success() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        // Register a handler that returns JSON
        let handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("test_method");
        registry.register("test_method", handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let request = JsonRpcRequest::new("test_method", None, Some(json!(1)));
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let response = router.route_single(request, http_request, &request_data).await;

        match response {
            JsonRpcResponseType::Success(resp) => {
                // Handler returns {"result":"success"} which should be extracted
                assert_eq!(resp.result, json!({"result":"success"}));
                assert_eq!(resp.id, json!(1));
            }
            _ => panic!("Expected success response"),
        }
    }

    #[tokio::test]
    async fn test_route_single_with_handler_error() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        // Register a handler that returns an error
        let handler = Arc::new(MockErrorHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("error_method");
        registry.register("error_method", handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let request = JsonRpcRequest::new("error_method", None, Some(json!(1)));
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let response = router.route_single(request, http_request, &request_data).await;

        match response {
            JsonRpcResponseType::Error(err) => {
                assert_eq!(err.error.code, error_codes::INTERNAL_ERROR);
                assert_eq!(err.id, json!(1));
                assert!(err.error.data.is_some());
            }
            _ => panic!("Expected error response"),
        }
    }

    #[tokio::test]
    async fn test_route_batch_disabled() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        let router = JsonRpcRouter::new(registry, false, 100);

        let batch = vec![JsonRpcRequest::new("method", None, Some(json!(1)))];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.error.code, error_codes::INVALID_REQUEST);
    }

    #[tokio::test]
    async fn test_route_batch_empty() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        let router = JsonRpcRouter::new(registry, true, 100);

        let batch = vec![];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.error.code, error_codes::INVALID_REQUEST);
    }

    #[tokio::test]
    async fn test_route_batch_size_exceeded() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        let router = JsonRpcRouter::new(registry, true, 5);

        let batch = (1..=10)
            .map(|i| JsonRpcRequest::new("method", None, Some(json!(i))))
            .collect();
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.error.code, error_codes::INVALID_REQUEST);
    }

    #[tokio::test]
    async fn test_route_batch_filters_notifications() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        // Register a method
        let handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("method");
        registry.register("method", handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("method", None, Some(json!(1))), // Normal request
            JsonRpcRequest::new("method", None, None),           // Notification
            JsonRpcRequest::new("method", None, Some(json!(2))), // Normal request
        ];

        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;
        assert!(result.is_ok());

        let responses = result.unwrap();
        // Should only have 2 responses (notifications filtered out)
        assert_eq!(responses.len(), 2);
    }

    #[test]
    fn test_parse_request_single() {
        let json = r#"{"jsonrpc":"2.0","method":"test","id":1}"#;
        let parsed = JsonRpcRouter::parse_request(json);

        assert!(parsed.is_ok());
        match parsed.unwrap() {
            JsonRpcRequestOrBatch::Single(req) => {
                assert_eq!(req.method, "test");
                assert_eq!(req.id, Some(json!(1)));
            }
            _ => panic!("Expected single request"),
        }
    }

    #[test]
    fn test_parse_request_batch() {
        let json = r#"[{"jsonrpc":"2.0","method":"test","id":1},{"jsonrpc":"2.0","method":"test2","id":2}]"#;
        let parsed = JsonRpcRouter::parse_request(json);

        assert!(parsed.is_ok());
        match parsed.unwrap() {
            JsonRpcRequestOrBatch::Batch(batch) => {
                assert_eq!(batch.len(), 2);
                assert_eq!(batch[0].method, "test");
                assert_eq!(batch[1].method, "test2");
            }
            _ => panic!("Expected batch request"),
        }
    }

    #[test]
    fn test_parse_request_invalid() {
        let json = r#"{"invalid":"json"}"#;
        let parsed = JsonRpcRouter::parse_request(json);

        assert!(parsed.is_err());
        let err = parsed.unwrap_err();
        assert_eq!(err.error.code, error_codes::PARSE_ERROR);
    }

    #[test]
    fn test_parse_request_notification() {
        let json = r#"{"jsonrpc":"2.0","method":"notify"}"#;
        let parsed = JsonRpcRouter::parse_request(json);

        assert!(parsed.is_ok());
        match parsed.unwrap() {
            JsonRpcRequestOrBatch::Single(req) => {
                assert!(req.is_notification());
                assert_eq!(req.method, "notify");
            }
            _ => panic!("Expected single request"),
        }
    }

    #[test]
    fn test_parse_request_with_params() {
        let json = r#"{"jsonrpc":"2.0","method":"subtract","params":{"subtrahend":23,"minuend":42},"id":3}"#;
        let parsed = JsonRpcRouter::parse_request(json);

        assert!(parsed.is_ok());
        match parsed.unwrap() {
            JsonRpcRequestOrBatch::Single(req) => {
                assert_eq!(req.method, "subtract");
                assert!(req.params.is_some());
                let params = req.params.unwrap();
                assert_eq!(params["subtrahend"], 23);
                assert_eq!(params["minuend"], 42);
            }
            _ => panic!("Expected single request"),
        }
    }

    // ============= Method Name Validation Tests in parse_request =============

    #[test]
    fn test_parse_request_invalid_method_name_empty() {
        let json = r#"{"jsonrpc":"2.0","method":"","id":1}"#;
        let parsed = JsonRpcRouter::parse_request(json);

        assert!(parsed.is_err());
        let err = parsed.unwrap_err();
        assert_eq!(err.error.code, error_codes::INVALID_REQUEST);
        assert!(err.error.message.contains("Invalid method name"));
    }

    #[test]
    fn test_parse_request_invalid_method_name_leading_space() {
        let json = r#"{"jsonrpc":"2.0","method":" method","id":1}"#;
        let parsed = JsonRpcRouter::parse_request(json);

        assert!(parsed.is_err());
        let err = parsed.unwrap_err();
        assert_eq!(err.error.code, error_codes::INVALID_REQUEST);
        assert!(err.error.message.contains("Invalid method name"));
    }

    #[test]
    fn test_parse_request_invalid_method_name_trailing_space() {
        let json = r#"{"jsonrpc":"2.0","method":"method ","id":1}"#;
        let parsed = JsonRpcRouter::parse_request(json);

        assert!(parsed.is_err());
        let err = parsed.unwrap_err();
        assert_eq!(err.error.code, error_codes::INVALID_REQUEST);
        assert!(err.error.message.contains("Invalid method name"));
    }

    #[test]
    fn test_parse_request_invalid_method_name_with_space() {
        let json = r#"{"jsonrpc":"2.0","method":"method name","id":1}"#;
        let parsed = JsonRpcRouter::parse_request(json);

        assert!(parsed.is_err());
        let err = parsed.unwrap_err();
        assert_eq!(err.error.code, error_codes::INVALID_REQUEST);
        assert!(err.error.message.contains("Invalid method name"));
    }

    #[test]
    fn test_parse_request_invalid_method_name_special_char() {
        let json = r#"{"jsonrpc":"2.0","method":"method@name","id":1}"#;
        let parsed = JsonRpcRouter::parse_request(json);

        assert!(parsed.is_err());
        let err = parsed.unwrap_err();
        assert_eq!(err.error.code, error_codes::INVALID_REQUEST);
        assert!(err.error.message.contains("Invalid method name"));
    }

    #[test]
    fn test_parse_request_invalid_method_name_too_long() {
        let long_method = "a".repeat(256);
        let json = format!(r#"{{"jsonrpc":"2.0","method":"{}","id":1}}"#, long_method);
        let parsed = JsonRpcRouter::parse_request(&json);

        assert!(parsed.is_err());
        let err = parsed.unwrap_err();
        assert_eq!(err.error.code, error_codes::INVALID_REQUEST);
        assert!(err.error.message.contains("Invalid method name"));
    }

    #[test]
    fn test_parse_request_batch_valid_method_names() {
        let json = r#"[
            {"jsonrpc":"2.0","method":"test.method","id":1},
            {"jsonrpc":"2.0","method":"another_method","id":2}
        ]"#;
        let parsed = JsonRpcRouter::parse_request(json);

        assert!(parsed.is_ok());
        match parsed.unwrap() {
            JsonRpcRequestOrBatch::Batch(batch) => {
                assert_eq!(batch.len(), 2);
                assert_eq!(batch[0].method, "test.method");
                assert_eq!(batch[1].method, "another_method");
            }
            _ => panic!("Expected batch request"),
        }
    }

    #[test]
    fn test_parse_request_batch_invalid_first_method_name() {
        let json = r#"[
            {"jsonrpc":"2.0","method":" invalid","id":1},
            {"jsonrpc":"2.0","method":"valid","id":2}
        ]"#;
        let parsed = JsonRpcRouter::parse_request(json);

        assert!(parsed.is_err());
        let err = parsed.unwrap_err();
        assert_eq!(err.error.code, error_codes::INVALID_REQUEST);
        assert!(err.error.message.contains("Invalid method name"));
    }

    #[test]
    fn test_parse_request_batch_invalid_second_method_name() {
        let json = r#"[
            {"jsonrpc":"2.0","method":"valid","id":1},
            {"jsonrpc":"2.0","method":"invalid#method","id":2}
        ]"#;
        let parsed = JsonRpcRouter::parse_request(json);

        assert!(parsed.is_err());
        let err = parsed.unwrap_err();
        assert_eq!(err.error.code, error_codes::INVALID_REQUEST);
        assert!(err.error.message.contains("Invalid method name"));
    }

    #[test]
    fn test_parse_request_batch_notification_invalid_method_name() {
        let json = r#"[
            {"jsonrpc":"2.0","method":"valid","id":1},
            {"jsonrpc":"2.0","method":"invalid method"}
        ]"#;
        let parsed = JsonRpcRouter::parse_request(json);

        assert!(parsed.is_err());
        let err = parsed.unwrap_err();
        assert_eq!(err.error.code, error_codes::INVALID_REQUEST);
        assert!(err.error.message.contains("Invalid method name"));
    }

    #[test]
    fn test_parse_request_method_name_dos_prevention() {
        // Test that very long method names are rejected
        let json = format!(r#"{{"jsonrpc":"2.0","method":"{}","id":1}}"#, "a".repeat(10000));
        let parsed = JsonRpcRouter::parse_request(&json);

        assert!(parsed.is_err());
        let err = parsed.unwrap_err();
        assert_eq!(err.error.code, error_codes::INVALID_REQUEST);
        assert!(err.error.message.contains("Invalid method name"));
    }

    #[test]
    fn test_parse_request_valid_method_names_complex() {
        let json = r#"{"jsonrpc":"2.0","method":"user.getById_v1-2","id":1}"#;
        let parsed = JsonRpcRouter::parse_request(json);

        assert!(parsed.is_ok());
        match parsed.unwrap() {
            JsonRpcRequestOrBatch::Single(req) => {
                assert_eq!(req.method, "user.getById_v1-2");
            }
            _ => panic!("Expected single request"),
        }
    }
}

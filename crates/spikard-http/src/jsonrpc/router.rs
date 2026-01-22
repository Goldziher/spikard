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
//! - Method name validation at routing time (defense in depth)
//!
//! # Validation
//!
//! Method names are validated at two points:
//! 1. During request parsing in `parse_request()` - validates method names when requests are deserialized
//! 2. During routing in `route_single()` - provides defense-in-depth validation even for programmatically created requests
//!
//! Invalid method names return a JSON-RPC error with code -32600 (Invalid Request) as per the JSON-RPC 2.0 spec.
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
        if let Err(validation_error) = super::protocol::validate_method_name(&request.method) {
            let id = request.id.unwrap_or(Value::Null);
            return JsonRpcResponseType::Error(JsonRpcErrorResponse::error(
                error_codes::INVALID_REQUEST,
                format!("Invalid method name: {}", validation_error),
                id,
            ));
        }

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

        let _is_notification = request.is_notification();

        let handler_result = handler.call(http_request, request_data.clone()).await;

        match handler_result {
            Ok(response) => {
                let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                    .await
                    .unwrap_or_default();

                let result = if body_bytes.is_empty() {
                    Value::Null
                } else {
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
        if !self.enable_batch {
            return Err(JsonRpcErrorResponse::error(
                error_codes::INVALID_REQUEST,
                "Batch requests not enabled",
                Value::Null,
            ));
        }

        if batch.len() > self.max_batch_size {
            return Err(JsonRpcErrorResponse::error(
                error_codes::INVALID_REQUEST,
                format!("Batch size {} exceeds maximum {}", batch.len(), self.max_batch_size),
                Value::Null,
            ));
        }

        if batch.is_empty() {
            return Err(JsonRpcErrorResponse::error(
                error_codes::INVALID_REQUEST,
                "Batch request cannot be empty",
                Value::Null,
            ));
        }

        let (base_parts, _body) = http_request.into_parts();

        let mut responses = Vec::with_capacity(batch.len());
        for request in batch {
            let is_notification = request.is_notification();

            let req_for_handler = Request::from_parts(base_parts.clone(), Body::empty());

            let response = self.route_single(request, req_for_handler, request_data).await;

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
        if let Ok(request) = serde_json::from_str::<JsonRpcRequest>(body) {
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

        if let Ok(batch) = serde_json::from_str::<Vec<JsonRpcRequest>>(body) {
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
    use crate::jsonrpc::MethodMetadata;
    use axum::body::Body;
    use axum::http::Request;
    use serde_json::json;
    use std::collections::HashMap;
    use std::sync::Arc;

    /// Helper function to create minimal RequestData for tests
    fn create_test_request_data() -> RequestData {
        RequestData {
            path_params: Arc::new(HashMap::new()),
            query_params: Arc::new(Value::Object(serde_json::Map::new())),
            validated_params: None,
            raw_query_params: Arc::new(HashMap::new()),
            body: Arc::new(Value::Null),
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

    /// Mock handler that returns success with non-JSON UTF-8 text
    struct MockTextHandler;

    impl Handler for MockTextHandler {
        fn call(
            &self,
            _request: Request<Body>,
            _request_data: RequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send + '_>> {
            Box::pin(async {
                use axum::response::Response;
                let response = Response::builder()
                    .status(200)
                    .header("content-type", "text/plain")
                    .body(Body::from("hello"))
                    .unwrap();
                Ok(response)
            })
        }
    }

    /// Mock handler that returns success with non-UTF-8 bytes
    struct MockBinaryHandler;

    impl Handler for MockBinaryHandler {
        fn call(
            &self,
            _request: Request<Body>,
            _request_data: RequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send + '_>> {
            Box::pin(async {
                use axum::response::Response;
                let response = Response::builder()
                    .status(200)
                    .header("content-type", "application/octet-stream")
                    .body(Body::from(vec![0xff, 0xfe, 0xfd]))
                    .unwrap();
                Ok(response)
            })
        }
    }

    #[tokio::test]
    async fn test_route_single_invalid_method_name_empty() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        let router = JsonRpcRouter::new(registry, true, 100);

        let request = JsonRpcRequest::new("", None, Some(json!(1)));
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let response = router.route_single(request, http_request, &request_data).await;

        match response {
            JsonRpcResponseType::Error(err) => {
                assert_eq!(err.error.code, error_codes::INVALID_REQUEST);
                assert!(err.error.message.contains("Invalid method name"));
                assert_eq!(err.id, json!(1));
            }
            _ => panic!("Expected error response for invalid method name"),
        }
    }

    #[tokio::test]
    async fn test_route_single_invalid_method_name_with_space() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        let router = JsonRpcRouter::new(registry, true, 100);

        let request = JsonRpcRequest::new("method name", None, Some(json!(1)));
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let response = router.route_single(request, http_request, &request_data).await;

        match response {
            JsonRpcResponseType::Error(err) => {
                assert_eq!(err.error.code, error_codes::INVALID_REQUEST);
                assert!(err.error.message.contains("Invalid method name"));
                assert_eq!(err.id, json!(1));
            }
            _ => panic!("Expected error response for invalid method name"),
        }
    }

    #[tokio::test]
    async fn test_route_single_success_non_json_utf8_body_returns_string() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        registry
            .register("echo", Arc::new(MockTextHandler), MethodMetadata::new("echo"))
            .unwrap();

        let router = JsonRpcRouter::new(registry, true, 100);
        let request = JsonRpcRequest::new("echo", None, Some(json!(1)));
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let response = router.route_single(request, http_request, &request_data).await;
        match response {
            JsonRpcResponseType::Success(ok) => {
                assert_eq!(ok.result, Value::String("hello".to_string()));
            }
            other => panic!("Expected success response, got: {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_route_single_success_non_utf8_body_returns_placeholder_string() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        registry
            .register("bin", Arc::new(MockBinaryHandler), MethodMetadata::new("bin"))
            .unwrap();

        let router = JsonRpcRouter::new(registry, true, 100);
        let request = JsonRpcRequest::new("bin", None, Some(json!(1)));
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let response = router.route_single(request, http_request, &request_data).await;
        match response {
            JsonRpcResponseType::Success(ok) => {
                assert_eq!(ok.result, Value::String("[binary data]".to_string()));
            }
            other => panic!("Expected success response, got: {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_route_single_invalid_method_name_control_char() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        let router = JsonRpcRouter::new(registry, true, 100);

        let request = JsonRpcRequest::new("method\nname", None, Some(json!(1)));
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let response = router.route_single(request, http_request, &request_data).await;

        match response {
            JsonRpcResponseType::Error(err) => {
                assert_eq!(err.error.code, error_codes::INVALID_REQUEST);
                assert!(err.error.message.contains("Invalid method name"));
                assert_eq!(err.id, json!(1));
            }
            _ => panic!("Expected error response for invalid method name"),
        }
    }

    #[tokio::test]
    async fn test_route_single_invalid_method_name_special_char() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        let router = JsonRpcRouter::new(registry, true, 100);

        let request = JsonRpcRequest::new("method@name", None, Some(json!(1)));
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let response = router.route_single(request, http_request, &request_data).await;

        match response {
            JsonRpcResponseType::Error(err) => {
                assert_eq!(err.error.code, error_codes::INVALID_REQUEST);
                assert!(err.error.message.contains("Invalid method name"));
                assert_eq!(err.id, json!(1));
            }
            _ => panic!("Expected error response for invalid method name"),
        }
    }

    #[tokio::test]
    async fn test_route_single_invalid_method_name_too_long() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        let router = JsonRpcRouter::new(registry, true, 100);

        let long_method = "a".repeat(256);
        let request = JsonRpcRequest::new(long_method, None, Some(json!(1)));
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let response = router.route_single(request, http_request, &request_data).await;

        match response {
            JsonRpcResponseType::Error(err) => {
                assert_eq!(err.error.code, error_codes::INVALID_REQUEST);
                assert!(err.error.message.contains("Invalid method name"));
                assert_eq!(err.id, json!(1));
            }
            _ => panic!("Expected error response for invalid method name"),
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

        let handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("notify_method");
        registry.register("notify_method", handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let request = JsonRpcRequest::new("notify_method", None, None);
        assert!(request.is_notification());

        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let response = router.route_single(request, http_request, &request_data).await;

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
                assert_eq!(resp.result, json!({"result":"success"}));
                assert_eq!(resp.id, json!(1));
            }
            _ => panic!("Expected success response"),
        }
    }

    #[tokio::test]
    async fn test_route_single_with_handler_error() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

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

        let handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("method");
        registry.register("method", handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("method", None, Some(json!(1))),
            JsonRpcRequest::new("method", None, None),
            JsonRpcRequest::new("method", None, Some(json!(2))),
        ];

        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;
        assert!(result.is_ok());

        let responses = result.unwrap();
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

    #[tokio::test]
    async fn test_error_code_parse_error_invalid_json() {
        let json = r#"{"invalid json"#;
        let parsed = JsonRpcRouter::parse_request(json);

        assert!(parsed.is_err());
        let err = parsed.unwrap_err();
        assert_eq!(err.error.code, error_codes::PARSE_ERROR);
        assert_eq!(err.error.code, -32700);
    }

    #[tokio::test]
    async fn test_error_code_method_not_found() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        let router = JsonRpcRouter::new(registry, true, 100);

        let request = JsonRpcRequest::new("nonexistent_method", None, Some(json!(1)));
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let response = router.route_single(request, http_request, &request_data).await;

        match response {
            JsonRpcResponseType::Error(err) => {
                assert_eq!(err.error.code, error_codes::METHOD_NOT_FOUND);
                assert_eq!(err.error.code, -32601);
                assert_eq!(err.id, json!(1));
            }
            _ => panic!("Expected error response"),
        }
    }

    #[tokio::test]
    async fn test_error_code_invalid_request_empty_method() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        let router = JsonRpcRouter::new(registry, true, 100);

        let request = JsonRpcRequest::new("", None, Some(json!(1)));
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let response = router.route_single(request, http_request, &request_data).await;

        match response {
            JsonRpcResponseType::Error(err) => {
                assert_eq!(err.error.code, error_codes::INVALID_REQUEST);
                assert_eq!(err.error.code, -32600);
                assert_eq!(err.id, json!(1));
            }
            _ => panic!("Expected error response"),
        }
    }

    #[tokio::test]
    async fn test_error_code_internal_error_from_handler() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

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
                assert_eq!(err.error.code, -32603);
                assert_eq!(err.id, json!(1));
            }
            _ => panic!("Expected error response"),
        }
    }

    #[tokio::test]
    async fn test_error_code_server_error_custom_range() {
        assert!(error_codes::is_server_error(-32000));
        assert!(error_codes::is_server_error(-32050));
        assert!(error_codes::is_server_error(-32099));
        assert!(!error_codes::is_server_error(-31999));
        assert!(!error_codes::is_server_error(-32100));
    }

    #[tokio::test]
    async fn test_error_response_always_has_code() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        let router = JsonRpcRouter::new(registry, true, 100);

        let request = JsonRpcRequest::new("nonexistent", None, Some(json!(1)));
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let response = router.route_single(request, http_request, &request_data).await;

        match response {
            JsonRpcResponseType::Error(err) => {
                assert!(err.error.code != 0);
                assert!(err.error.code < 0);
            }
            _ => panic!("Expected error response"),
        }
    }

    #[tokio::test]
    async fn test_error_response_always_has_message() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        let router = JsonRpcRouter::new(registry, true, 100);

        let request = JsonRpcRequest::new("nonexistent", None, Some(json!(1)));
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let response = router.route_single(request, http_request, &request_data).await;

        match response {
            JsonRpcResponseType::Error(err) => {
                assert!(!err.error.message.is_empty());
                assert!(err.jsonrpc == "2.0");
            }
            _ => panic!("Expected error response"),
        }
    }

    #[tokio::test]
    async fn test_error_response_data_optional() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        let router = JsonRpcRouter::new(registry, true, 100);

        let request = JsonRpcRequest::new("nonexistent", None, Some(json!(1)));
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let response = router.route_single(request, http_request, &request_data).await;

        match response {
            JsonRpcResponseType::Error(err) => {
                assert_eq!(err.error.code, error_codes::METHOD_NOT_FOUND);
            }
            _ => panic!("Expected error response"),
        }
    }

    #[tokio::test]
    async fn test_error_response_id_matches_request() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        let router = JsonRpcRouter::new(registry, true, 100);

        let test_id = json!(42);
        let request = JsonRpcRequest::new("nonexistent", None, Some(test_id.clone()));
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let response = router.route_single(request, http_request, &request_data).await;

        match response {
            JsonRpcResponseType::Error(err) => {
                assert_eq!(err.id, test_id);
            }
            _ => panic!("Expected error response"),
        }
    }

    #[tokio::test]
    async fn test_error_response_no_result_field() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        let router = JsonRpcRouter::new(registry, true, 100);

        let request = JsonRpcRequest::new("nonexistent", None, Some(json!(1)));
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let response = router.route_single(request, http_request, &request_data).await;

        match response {
            JsonRpcResponseType::Error(_) => {}
            _ => panic!("Expected error response"),
        }
    }

    #[tokio::test]
    async fn test_error_data_includes_details() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

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
                assert!(err.error.data.is_some());
                let data = err.error.data.unwrap();
                assert!(data.get("details").is_some());
            }
            _ => panic!("Expected error response"),
        }
    }

    #[tokio::test]
    async fn test_batch_single_request() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        let handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("test");
        registry.register("test", handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![JsonRpcRequest::new("test", None, Some(json!(1)))];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 1);
    }

    #[tokio::test]
    async fn test_batch_three_requests_all_succeed() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        let handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("test");
        registry.register("test", handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("test", None, Some(json!(1))),
            JsonRpcRequest::new("test", None, Some(json!(2))),
            JsonRpcRequest::new("test", None, Some(json!(3))),
        ];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 3);
        for resp in &responses {
            match resp {
                JsonRpcResponseType::Success(_) => {}
                _ => panic!("Expected all success responses"),
            }
        }
    }

    #[tokio::test]
    async fn test_batch_mixed_success_and_errors() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        let handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("valid");
        registry.register("valid", handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("valid", None, Some(json!(1))),
            JsonRpcRequest::new("invalid", None, Some(json!(2))),
            JsonRpcRequest::new("valid", None, Some(json!(3))),
        ];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 3);

        match &responses[0] {
            JsonRpcResponseType::Success(_) => {}
            _ => panic!("Expected success at index 0"),
        }
        match &responses[1] {
            JsonRpcResponseType::Error(_) => {}
            _ => panic!("Expected error at index 1"),
        }
        match &responses[2] {
            JsonRpcResponseType::Success(_) => {}
            _ => panic!("Expected success at index 2"),
        }
    }

    #[tokio::test]
    async fn test_batch_all_errors() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("nonexistent1", None, Some(json!(1))),
            JsonRpcRequest::new("nonexistent2", None, Some(json!(2))),
            JsonRpcRequest::new("nonexistent3", None, Some(json!(3))),
        ];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 3);
        for resp in &responses {
            match resp {
                JsonRpcResponseType::Error(_) => {}
                _ => panic!("Expected all error responses"),
            }
        }
    }

    #[tokio::test]
    async fn test_batch_exceeds_max_size() {
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
    async fn test_batch_empty_array() {
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
    async fn test_batch_disabled_error() {
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
    async fn test_notification_no_response_generated() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        let handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("notify");
        registry.register("notify", handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("notify", None, Some(json!(1))),
            JsonRpcRequest::new("notify", None, None),
        ];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 1);
    }

    #[tokio::test]
    async fn test_batch_mixed_notifications_responses() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        let handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("method");
        registry.register("method", handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("method", None, Some(json!(1))),
            JsonRpcRequest::new("method", None, None),
            JsonRpcRequest::new("method", None, Some(json!(2))),
            JsonRpcRequest::new("method", None, None),
        ];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 2);
    }

    #[tokio::test]
    async fn test_batch_all_notifications_empty_response() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        let handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("method");
        registry.register("method", handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("method", None, None),
            JsonRpcRequest::new("method", None, None),
        ];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 0);
    }

    #[tokio::test]
    async fn test_notification_error_still_not_responded() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("nonexistent", None, Some(json!(1))),
            JsonRpcRequest::new("nonexistent", None, None),
        ];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 1);
        match &responses[0] {
            JsonRpcResponseType::Error(_) => {}
            _ => panic!("Expected error response"),
        }
    }

    #[tokio::test]
    async fn test_batch_requests_independent_state() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        let handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("test");
        registry.register("test", handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("test", None, Some(json!(1))),
            JsonRpcRequest::new("test", None, Some(json!(2))),
            JsonRpcRequest::new("test", None, Some(json!(3))),
        ];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 3);
        for (i, resp) in responses.iter().enumerate() {
            match resp {
                JsonRpcResponseType::Success(s) => {
                    assert_eq!(s.id, json!(i + 1));
                }
                _ => panic!("Expected success response"),
            }
        }
    }

    #[tokio::test]
    async fn test_batch_error_preserves_request_order() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        let handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("valid");
        registry.register("valid", handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("valid", None, Some(json!("first"))),
            JsonRpcRequest::new("invalid", None, Some(json!("second"))),
            JsonRpcRequest::new("valid", None, Some(json!("third"))),
        ];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 3);

        match &responses[0] {
            JsonRpcResponseType::Success(s) => {
                assert_eq!(s.id, json!("first"));
                assert_eq!(s.jsonrpc, "2.0");
            }
            _ => panic!("Expected success at index 0"),
        }
        match &responses[1] {
            JsonRpcResponseType::Error(e) => {
                assert_eq!(e.id, json!("second"));
                assert_eq!(e.error.code, error_codes::METHOD_NOT_FOUND);
            }
            _ => panic!("Expected error at index 1"),
        }
        match &responses[2] {
            JsonRpcResponseType::Success(s) => {
                assert_eq!(s.id, json!("third"));
                assert_eq!(s.jsonrpc, "2.0");
            }
            _ => panic!("Expected success at index 2"),
        }
    }

    #[tokio::test]
    async fn test_batch_handler_execution_order() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        let handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("method1");
        registry.register("method1", handler.clone(), metadata).unwrap();

        let metadata = super::super::method_registry::MethodMetadata::new("method2");
        registry.register("method2", handler.clone(), metadata).unwrap();

        let metadata = super::super::method_registry::MethodMetadata::new("method3");
        registry.register("method3", handler.clone(), metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("method1", None, Some(json!(1))),
            JsonRpcRequest::new("method2", None, Some(json!(2))),
            JsonRpcRequest::new("method3", None, Some(json!(3))),
        ];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 3);
        for resp in &responses {
            match resp {
                JsonRpcResponseType::Success(_) => {}
                _ => panic!("Expected all success responses"),
            }
        }
    }

    #[test]
    fn test_parse_batch_single_request_in_array() {
        let json = r#"[{"jsonrpc":"2.0","method":"test","id":1}]"#;
        let parsed = JsonRpcRouter::parse_request(json);

        assert!(parsed.is_ok());
        match parsed.unwrap() {
            JsonRpcRequestOrBatch::Batch(batch) => {
                assert_eq!(batch.len(), 1);
                assert_eq!(batch[0].method, "test");
            }
            _ => panic!("Expected batch request"),
        }
    }

    #[test]
    fn test_parse_batch_three_requests() {
        let json = r#"[
            {"jsonrpc":"2.0","method":"test1","id":1},
            {"jsonrpc":"2.0","method":"test2","id":2},
            {"jsonrpc":"2.0","method":"test3","id":3}
        ]"#;
        let parsed = JsonRpcRouter::parse_request(json);

        assert!(parsed.is_ok());
        match parsed.unwrap() {
            JsonRpcRequestOrBatch::Batch(batch) => {
                assert_eq!(batch.len(), 3);
                assert_eq!(batch[0].method, "test1");
                assert_eq!(batch[1].method, "test2");
                assert_eq!(batch[2].method, "test3");
            }
            _ => panic!("Expected batch request"),
        }
    }

    #[test]
    fn test_parse_batch_mixed_notifications() {
        let json = r#"[
            {"jsonrpc":"2.0","method":"method","id":1},
            {"jsonrpc":"2.0","method":"method"},
            {"jsonrpc":"2.0","method":"method","id":2}
        ]"#;
        let parsed = JsonRpcRouter::parse_request(json);

        assert!(parsed.is_ok());
        match parsed.unwrap() {
            JsonRpcRequestOrBatch::Batch(batch) => {
                assert_eq!(batch.len(), 3);
                assert!(!batch[0].is_notification());
                assert!(batch[1].is_notification());
                assert!(!batch[2].is_notification());
            }
            _ => panic!("Expected batch request"),
        }
    }

    #[tokio::test]
    async fn test_batch_max_size_boundary_exactly_at_limit() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        let max_size = 10;
        let router = JsonRpcRouter::new(registry, true, max_size);

        let batch: Vec<JsonRpcRequest> = (1..=max_size)
            .map(|i| JsonRpcRequest::new("method", None, Some(json!(i))))
            .collect();

        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), max_size);
    }

    #[tokio::test]
    async fn test_batch_max_size_boundary_one_over_limit() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        let max_size = 10;
        let router = JsonRpcRouter::new(registry, true, max_size);

        let batch: Vec<JsonRpcRequest> = (1..=max_size + 1)
            .map(|i| JsonRpcRequest::new("method", None, Some(json!(i))))
            .collect();

        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.error.code, error_codes::INVALID_REQUEST);
        assert!(err.error.message.contains("exceeds maximum"));
    }

    #[tokio::test]
    async fn test_batch_notification_with_numeric_id_zero() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        let handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("method");
        registry.register("method", handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("method", None, Some(json!(0))),
            JsonRpcRequest::new("method", None, Some(json!(1))),
        ];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 2);
        match &responses[0] {
            JsonRpcResponseType::Success(s) => assert_eq!(s.id, json!(0)),
            _ => panic!("Expected success response"),
        }
    }

    #[tokio::test]
    async fn test_batch_mixed_id_types_string_and_number() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        let handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("method");
        registry.register("method", handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("method", None, Some(json!("string_id"))),
            JsonRpcRequest::new("method", None, Some(json!(42))),
            JsonRpcRequest::new("method", None, Some(json!("another_string"))),
        ];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 3);

        match &responses[0] {
            JsonRpcResponseType::Success(s) => assert_eq!(s.id, json!("string_id")),
            _ => panic!("Expected success response"),
        }
        match &responses[1] {
            JsonRpcResponseType::Success(s) => assert_eq!(s.id, json!(42)),
            _ => panic!("Expected success response"),
        }
        match &responses[2] {
            JsonRpcResponseType::Success(s) => assert_eq!(s.id, json!("another_string")),
            _ => panic!("Expected success response"),
        }
    }

    #[tokio::test]
    async fn test_batch_all_notifications_truly_empty_response() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        for i in 1..=5 {
            let handler = Arc::new(MockSuccessHandler);
            let method_name = format!("method{}", i);
            let metadata = super::super::method_registry::MethodMetadata::new(&method_name);
            registry.register(&method_name, handler, metadata).unwrap();
        }

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("method1", None, None),
            JsonRpcRequest::new("method2", None, None),
            JsonRpcRequest::new("method3", None, None),
            JsonRpcRequest::new("method4", None, None),
            JsonRpcRequest::new("method5", None, None),
        ];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 0);
        assert!(responses.is_empty());
    }

    #[tokio::test]
    async fn test_batch_single_notification_among_requests() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        let handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("method");
        registry.register("method", handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("method", None, Some(json!(1))),
            JsonRpcRequest::new("method", None, None),
            JsonRpcRequest::new("method", None, Some(json!(2))),
        ];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 2);

        match &responses[0] {
            JsonRpcResponseType::Success(s) => assert_eq!(s.id, json!(1)),
            _ => panic!("Expected success at index 0"),
        }
        match &responses[1] {
            JsonRpcResponseType::Success(s) => assert_eq!(s.id, json!(2)),
            _ => panic!("Expected success at index 1"),
        }
    }

    #[tokio::test]
    async fn test_batch_error_one_request_fails_others_succeed() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        let handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("success");
        registry.register("success", handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("success", None, Some(json!(1))),
            JsonRpcRequest::new("failing_method", None, Some(json!(2))),
            JsonRpcRequest::new("success", None, Some(json!(3))),
            JsonRpcRequest::new("failing_method", None, Some(json!(4))),
            JsonRpcRequest::new("success", None, Some(json!(5))),
        ];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 5);

        match &responses[0] {
            JsonRpcResponseType::Success(s) => {
                assert_eq!(s.id, json!(1));
            }
            _ => panic!("Expected success at index 0"),
        }
        match &responses[1] {
            JsonRpcResponseType::Error(e) => {
                assert_eq!(e.id, json!(2));
                assert_eq!(e.error.code, error_codes::METHOD_NOT_FOUND);
            }
            _ => panic!("Expected error at index 1"),
        }
        match &responses[2] {
            JsonRpcResponseType::Success(s) => {
                assert_eq!(s.id, json!(3));
            }
            _ => panic!("Expected success at index 2"),
        }
        match &responses[3] {
            JsonRpcResponseType::Error(e) => {
                assert_eq!(e.id, json!(4));
                assert_eq!(e.error.code, error_codes::METHOD_NOT_FOUND);
            }
            _ => panic!("Expected error at index 3"),
        }
        match &responses[4] {
            JsonRpcResponseType::Success(s) => {
                assert_eq!(s.id, json!(5));
            }
            _ => panic!("Expected success at index 4"),
        }
    }

    #[tokio::test]
    async fn test_batch_response_order_with_complex_id_types() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        let handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("method");
        registry.register("method", handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("method", None, Some(json!("alpha"))),
            JsonRpcRequest::new("method", None, Some(json!(999))),
            JsonRpcRequest::new("method", None, Some(json!(null))),
            JsonRpcRequest::new("method", None, Some(json!("beta"))),
            JsonRpcRequest::new("method", None, Some(json!(0))),
        ];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 5);

        match &responses[0] {
            JsonRpcResponseType::Success(s) => assert_eq!(s.id, json!("alpha")),
            _ => panic!("Expected success at index 0"),
        }
        match &responses[1] {
            JsonRpcResponseType::Success(s) => assert_eq!(s.id, json!(999)),
            _ => panic!("Expected success at index 1"),
        }
        match &responses[2] {
            JsonRpcResponseType::Success(s) => assert_eq!(s.id, json!(null)),
            _ => panic!("Expected success at index 2"),
        }
        match &responses[3] {
            JsonRpcResponseType::Success(s) => assert_eq!(s.id, json!("beta")),
            _ => panic!("Expected success at index 3"),
        }
        match &responses[4] {
            JsonRpcResponseType::Success(s) => assert_eq!(s.id, json!(0)),
            _ => panic!("Expected success at index 4"),
        }
    }

    #[tokio::test]
    async fn test_batch_handler_error_does_not_stop_batch() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        let error_handler = Arc::new(MockErrorHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("failing");
        registry.register("failing", error_handler, metadata).unwrap();

        let success_handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("success");
        registry.register("success", success_handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("success", None, Some(json!(1))),
            JsonRpcRequest::new("failing", None, Some(json!(2))),
            JsonRpcRequest::new("success", None, Some(json!(3))),
            JsonRpcRequest::new("failing", None, Some(json!(4))),
            JsonRpcRequest::new("success", None, Some(json!(5))),
        ];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 5);

        for resp in &responses {
            match resp {
                JsonRpcResponseType::Success(s) => {
                    assert_eq!(s.jsonrpc, "2.0");
                    assert!(s.id != Value::Null);
                }
                JsonRpcResponseType::Error(e) => {
                    assert_eq!(e.error.code, error_codes::INTERNAL_ERROR);
                    assert!(e.id != Value::Null);
                }
            }
        }
    }

    #[tokio::test]
    async fn test_batch_large_batch_within_limits() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        let handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("method");
        registry.register("method", handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch: Vec<JsonRpcRequest> = (1..=50)
            .map(|i| JsonRpcRequest::new("method", None, Some(json!(i))))
            .collect();

        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 50);
        for resp in &responses {
            match resp {
                JsonRpcResponseType::Success(_) => {}
                _ => panic!("Expected all success responses"),
            }
        }
    }

    #[tokio::test]
    async fn test_batch_id_preservation_with_duplicates() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        let handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("method");
        registry.register("method", handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("method", None, Some(json!(1))),
            JsonRpcRequest::new("method", None, Some(json!(1))),
            JsonRpcRequest::new("method", None, Some(json!(2))),
            JsonRpcRequest::new("method", None, Some(json!(1))),
        ];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 4);

        assert_eq!(responses[0].id(), json!(1));
        assert_eq!(responses[1].id(), json!(1));
        assert_eq!(responses[2].id(), json!(2));
        assert_eq!(responses[3].id(), json!(1));
    }

    #[tokio::test]
    async fn test_batch_different_methods_in_sequence() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        for method_name in &["user.getById", "post.create", "comment.delete", "admin.stats"] {
            let handler = Arc::new(MockSuccessHandler);
            let metadata = super::super::method_registry::MethodMetadata::new(method_name.to_string());
            registry.register(method_name.to_string(), handler, metadata).unwrap();
        }

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("user.getById", None, Some(json!(1))),
            JsonRpcRequest::new("post.create", None, Some(json!(2))),
            JsonRpcRequest::new("comment.delete", None, Some(json!(3))),
            JsonRpcRequest::new("admin.stats", None, Some(json!(4))),
        ];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 4);
        for resp in &responses {
            match resp {
                JsonRpcResponseType::Success(_) => {}
                _ => panic!("Expected all success responses"),
            }
        }
    }

    #[tokio::test]
    async fn test_batch_mixed_valid_and_method_not_found_errors() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        let handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("exists");
        registry.register("exists", handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("exists", None, Some(json!(1))),
            JsonRpcRequest::new("not_exists", None, Some(json!(2))),
            JsonRpcRequest::new("exists", None, Some(json!(3))),
            JsonRpcRequest::new("not_exists", None, Some(json!(4))),
            JsonRpcRequest::new("not_exists", None, Some(json!(5))),
            JsonRpcRequest::new("exists", None, Some(json!(6))),
        ];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 6);

        match &responses[0] {
            JsonRpcResponseType::Success(_) => {}
            _ => panic!("Expected success at index 0"),
        }
        for i in [1, 3, 4] {
            match &responses[i] {
                JsonRpcResponseType::Error(e) => {
                    assert_eq!(e.error.code, error_codes::METHOD_NOT_FOUND);
                }
                _ => panic!("Expected error at index {}", i),
            }
        }
        match &responses[2] {
            JsonRpcResponseType::Success(_) => {}
            _ => panic!("Expected success at index 2"),
        }
        match &responses[5] {
            JsonRpcResponseType::Success(_) => {}
            _ => panic!("Expected success at index 5"),
        }
    }

    #[tokio::test]
    async fn test_batch_request_data_shared_correctly() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        let handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("method");
        registry.register("method", handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("method", None, Some(json!(1))),
            JsonRpcRequest::new("method", None, Some(json!(2))),
            JsonRpcRequest::new("method", None, Some(json!(3))),
        ];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 3);

        for resp in &responses {
            match resp {
                JsonRpcResponseType::Success(s) => {
                    assert_eq!(s.jsonrpc, "2.0");
                    assert!(s.result != Value::Null);
                }
                _ => panic!("Expected success response"),
            }
        }
    }

    #[tokio::test]
    async fn test_batch_response_contains_required_fields() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        let handler = Arc::new(MockSuccessHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("method");
        registry.register("method", handler, metadata).unwrap();

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("method", None, Some(json!(1))),
            JsonRpcRequest::new("method", None, Some(json!(2))),
        ];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();

        for resp in &responses {
            match resp {
                JsonRpcResponseType::Success(s) => {
                    assert_eq!(s.jsonrpc, "2.0");
                    assert!(!s.result.is_null());
                    assert!(!s.id.is_null());
                }
                _ => panic!("Expected success response"),
            }
        }
    }

    #[tokio::test]
    async fn test_batch_error_response_required_fields() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("not_found", None, Some(json!(1))),
            JsonRpcRequest::new("also_not_found", None, Some(json!(2))),
        ];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();

        for resp in &responses {
            match resp {
                JsonRpcResponseType::Error(e) => {
                    assert_eq!(e.jsonrpc, "2.0");
                    assert!(e.error.code < 0);
                    assert!(!e.error.message.is_empty());
                    assert!(!e.id.is_null());
                }
                _ => panic!("Expected error response"),
            }
        }
    }

    #[tokio::test]
    async fn test_batch_notification_errors_still_not_responded() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("not_found", None, Some(json!(1))),
            JsonRpcRequest::new("not_found", None, None),
            JsonRpcRequest::new("not_found", None, Some(json!(2))),
        ];
        let http_request = create_test_http_request();
        let request_data = create_test_request_data();

        let result = router.route_batch(batch, http_request, &request_data).await;

        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 2);

        match &responses[0] {
            JsonRpcResponseType::Error(e) => {
                assert_eq!(e.id, json!(1));
            }
            _ => panic!("Expected error at index 0"),
        }
        match &responses[1] {
            JsonRpcResponseType::Error(e) => {
                assert_eq!(e.id, json!(2));
            }
            _ => panic!("Expected error at index 1"),
        }
    }

    impl JsonRpcResponseType {
        fn id(&self) -> Value {
            match self {
                JsonRpcResponseType::Success(s) => s.id.clone(),
                JsonRpcResponseType::Error(e) => e.id.clone(),
            }
        }
    }
}

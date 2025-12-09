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
    /// 3. Invoking the handler
    /// 4. Returning appropriately formatted responses
    ///
    /// For notifications, the server MUST NOT send a response.
    /// This method currently returns a no-op response; actual handler
    /// invocation will be implemented in the HTTP handler layer.
    ///
    /// # Arguments
    ///
    /// * `request` - The JSON-RPC request to route
    ///
    /// # Returns
    ///
    /// A `JsonRpcResponseType` containing either a success response with the
    /// handler's result or an error response if the method is not found
    pub async fn route_single(&self, request: JsonRpcRequest) -> JsonRpcResponseType {
        // Check if method exists in registry
        let _handler = match self.registry.get(&request.method) {
            Some(h) => h,
            None => {
                let id = request.id.unwrap_or(Value::Null);
                return JsonRpcResponseType::Error(JsonRpcErrorResponse::error(
                    error_codes::METHOD_NOT_FOUND,
                    "Method not found",
                    id,
                ));
            }
        };

        // If notification (no id), don't send response
        if request.is_notification() {
            // TODO: Call handler but don't wait for response
            // For now, just return success with null (this won't be sent to client)
            return JsonRpcResponseType::Success(JsonRpcResponse::success(Value::Null, Value::Null));
        }

        // TODO: Call handler with proper error handling
        // For now, echo params back as result
        let id = request.id.unwrap_or(Value::Null);
        let result = request.params.unwrap_or_else(|| Value::Object(serde_json::Map::new()));

        JsonRpcResponseType::Success(JsonRpcResponse::success(result, id))
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

        // Route each request and collect responses
        let mut responses = Vec::with_capacity(batch.len());
        for request in batch {
            // Skip notifications - they should not produce responses
            let is_notification = request.is_notification();
            let response = self.route_single(request).await;

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
    pub fn parse_request(body: &str) -> Result<JsonRpcRequestOrBatch, JsonRpcErrorResponse> {
        // Try to parse as single request first
        if let Ok(request) = serde_json::from_str::<JsonRpcRequest>(body) {
            return Ok(JsonRpcRequestOrBatch::Single(request));
        }

        // Try to parse as batch
        if let Ok(batch) = serde_json::from_str::<Vec<JsonRpcRequest>>(body) {
            return Ok(JsonRpcRequestOrBatch::Batch(batch));
        }

        // Neither single nor batch - parse error
        Err(JsonRpcErrorResponse::error(
            error_codes::PARSE_ERROR,
            "Parse error",
            Value::Null,
        ))
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
    use std::sync::Arc;

    /// Mock handler for testing
    struct MockHandler;

    impl Handler for MockHandler {
        fn call(
            &self,
            _request: Request<Body>,
            _request_data: RequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send + '_>> {
            Box::pin(async { Err((axum::http::StatusCode::OK, "mock".to_string())) })
        }
    }

    #[tokio::test]
    async fn test_route_single_method_not_found() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        let router = JsonRpcRouter::new(registry, true, 100);

        let request = JsonRpcRequest::new("unknown_method", None, Some(json!(1)));
        let response = router.route_single(request).await;

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
        let handler = Arc::new(MockHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("notify_method");
        registry.register("notify_method", handler, metadata);

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let request = JsonRpcRequest::new("notify_method", None, None);
        assert!(request.is_notification());

        let response = router.route_single(request).await;

        // Notifications should return success with null ID (but won't be sent to client)
        match response {
            JsonRpcResponseType::Success(resp) => {
                assert_eq!(resp.id, Value::Null);
            }
            _ => panic!("Expected success response for notification"),
        }
    }

    #[tokio::test]
    async fn test_route_single_with_params() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        // Register a method
        let handler = Arc::new(MockHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("test_method");
        registry.register("test_method", handler, metadata);

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let params = json!({"key": "value"});
        let request = JsonRpcRequest::new("test_method", Some(params.clone()), Some(json!(1)));
        let response = router.route_single(request).await;

        match response {
            JsonRpcResponseType::Success(resp) => {
                assert_eq!(resp.result, params);
                assert_eq!(resp.id, json!(1));
            }
            _ => panic!("Expected success response"),
        }
    }

    #[tokio::test]
    async fn test_route_batch_disabled() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        let router = JsonRpcRouter::new(registry, false, 100);

        let batch = vec![JsonRpcRequest::new("method", None, Some(json!(1)))];
        let result = router.route_batch(batch).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.error.code, error_codes::INVALID_REQUEST);
    }

    #[tokio::test]
    async fn test_route_batch_empty() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());
        let router = JsonRpcRouter::new(registry, true, 100);

        let batch = vec![];
        let result = router.route_batch(batch).await;

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
        let result = router.route_batch(batch).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.error.code, error_codes::INVALID_REQUEST);
    }

    #[tokio::test]
    async fn test_route_batch_filters_notifications() {
        let registry = Arc::new(JsonRpcMethodRegistry::new());

        // Register a method
        let handler = Arc::new(MockHandler);
        let metadata = super::super::method_registry::MethodMetadata::new("method");
        registry.register("method", handler, metadata);

        let router = JsonRpcRouter::new(registry.clone(), true, 100);

        let batch = vec![
            JsonRpcRequest::new("method", None, Some(json!(1))), // Normal request
            JsonRpcRequest::new("method", None, None),           // Notification
            JsonRpcRequest::new("method", None, Some(json!(2))), // Normal request
        ];

        let result = router.route_batch(batch).await;
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
}

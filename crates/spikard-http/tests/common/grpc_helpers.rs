//! gRPC test utilities and helpers for comprehensive integration tests
//!
//! This module provides reusable utilities for writing gRPC integration tests without
//! requiring language bindings. It includes helpers for creating test servers, clients,
//! building protobuf messages, and asserting on responses.
//!
//! # Examples
//!
//! ```ignore
//! use common::grpc_helpers::{
//!     GrpcTestServer, create_grpc_test_client, send_unary_request,
//!     assert_grpc_response, create_test_metadata, ProtobufMessageBuilder,
//! };
//! use std::sync::Arc;
//! use bytes::Bytes;
//!
//! #[tokio::test]
//! async fn test_grpc_service() {
//!     let mut server = GrpcTestServer::new();
//!     let metadata = create_test_metadata();
//!
//!     let message = ProtobufMessageBuilder::new()
//!         .add_field("name", "Alice")
//!         .add_field("age", 30)
//!         .build()
//!         .unwrap();
//!
//!     let response = send_unary_request(
//!         &server,
//!         "mypackage.UserService",
//!         "GetUser",
//!         message,
//!         metadata,
//!     )
//!     .await
//!     .unwrap();
//!
//!     assert_grpc_response(response, serde_json::json!({"id": 1}));
//! }
//! ```

use bytes::Bytes;
use serde_json::{Value, json};
use spikard_http::grpc::{GrpcHandler, GrpcHandlerResult, GrpcRequestData, GrpcResponseData};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tonic::Code;
use tonic::metadata::MetadataMap;

/// Test server for gRPC integration testing
///
/// Provides a simple in-memory server for registering test handlers and managing
/// their lifecycle. This is useful for testing gRPC services without spinning up
/// a real network server.
///
/// # Example
///
/// ```ignore
/// let mut server = GrpcTestServer::new();
/// server.register_service(Arc::new(my_handler));
/// assert!(!server.handlers().is_empty());
/// ```
#[derive(Clone)]
pub struct GrpcTestServer {
    handlers: Arc<std::sync::Mutex<Vec<Arc<dyn GrpcHandler>>>>,
    base_url: String,
}

impl GrpcTestServer {
    /// Create a new gRPC test server
    ///
    /// Initializes an empty test server with a default localhost base URL.
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(std::sync::Mutex::new(Vec::new())),
            base_url: "http://localhost:50051".to_string(),
        }
    }

    /// Create a new gRPC test server with a custom base URL
    ///
    /// # Arguments
    ///
    /// * `url` - The base URL for the server (e.g., <http://localhost:8080>)
    pub fn with_url(url: impl Into<String>) -> Self {
        Self {
            handlers: Arc::new(std::sync::Mutex::new(Vec::new())),
            base_url: url.into(),
        }
    }

    /// Register a gRPC service handler with the test server
    ///
    /// # Arguments
    ///
    /// * `handler` - Arc-wrapped handler implementation to register
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut server = GrpcTestServer::new();
    /// server.register_service(Arc::new(MyHandler));
    /// ```
    pub fn register_service(&self, handler: Arc<dyn GrpcHandler>) {
        let mut handlers = self.handlers.lock().unwrap();
        handlers.push(handler);
    }

    /// Get the base URL of the test server
    pub fn url(&self) -> &str {
        &self.base_url
    }

    /// Get all registered handlers
    ///
    /// Useful for inspecting which handlers have been registered.
    pub fn handlers(&self) -> Vec<Arc<dyn GrpcHandler>> {
        let handlers = self.handlers.lock().unwrap();
        handlers.clone()
    }

    /// Find a handler by service name
    ///
    /// # Arguments
    ///
    /// * `service_name` - The fully qualified service name to look up
    pub fn get_handler(&self, service_name: &str) -> Option<Arc<dyn GrpcHandler>> {
        let handlers = self.handlers.lock().unwrap();
        handlers.iter().find(|h| h.service_name() == service_name).cloned()
    }

    /// Check if a service is registered
    ///
    /// # Arguments
    ///
    /// * `service_name` - The service name to check
    pub fn has_service(&self, service_name: &str) -> bool {
        self.get_handler(service_name).is_some()
    }

    /// Get the count of registered services
    pub fn service_count(&self) -> usize {
        self.handlers.lock().unwrap().len()
    }
}

impl Default for GrpcTestServer {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a simple gRPC test client for testing
///
/// In real-world scenarios, you would use a proper gRPC client library like
/// `tonic::Client`. This helper creates a lightweight wrapper for testing
/// without requiring actual network connections.
///
/// # Example
///
/// ```ignore
/// let _client = create_grpc_test_client();
/// // Use with send_unary_request
/// ```
pub struct GrpcTestClient;

impl GrpcTestClient {
    /// Create a new test client
    pub const fn new() -> Self {
        Self
    }
}

impl Default for GrpcTestClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Factory function to create a gRPC test client
pub const fn create_grpc_test_client() -> GrpcTestClient {
    GrpcTestClient::new()
}

/// Send a unary gRPC request to a test server
///
/// This is a helper function to simulate sending a unary RPC request to a gRPC service.
/// In a real test, you would use a proper gRPC client library, but this helper
/// allows testing the handler logic directly.
///
/// # Arguments
///
/// * `server` - The test server instance
/// * `service` - Fully qualified service name (e.g., "mypackage.UserService")
/// * `method` - Method name (e.g., `GetUser`)
/// * `payload` - Serialized protobuf message bytes
/// * `metadata` - gRPC metadata (headers) to include in the request
///
/// # Errors
///
/// Returns an error if:
/// - The service is not registered on the server
/// - The handler returns an error response
///
/// # Example
///
/// ```ignore
/// let mut server = GrpcTestServer::new();
/// server.register_service(Arc::new(my_handler));
///
/// let response = send_unary_request(
///     &server,
///     "mypackage.UserService",
///     "GetUser",
///     Bytes::from("request"),
///     create_test_metadata(),
/// ).await?;
/// ```
pub async fn send_unary_request(
    server: &GrpcTestServer,
    service: &str,
    method: &str,
    payload: Bytes,
    metadata: MetadataMap,
) -> Result<GrpcResponseData, Box<dyn std::error::Error>> {
    let handler = server.get_handler(service).ok_or_else(|| {
        Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Service not found: {service}"),
        )) as Box<dyn std::error::Error>
    })?;

    let request = GrpcRequestData {
        service_name: service.to_string(),
        method_name: method.to_string(),
        payload,
        metadata,
    };

    handler.call(request).await.map_err(|e| e.message().into())
}

/// Assert that a gRPC response payload matches the expected JSON value
///
/// Parses the response payload as JSON and compares it with the expected value.
/// This is useful for testing JSON-based gRPC messages.
///
/// # Panics
///
/// Panics if the payload cannot be parsed as JSON or doesn't match the expected value.
///
/// # Arguments
///
/// * `response` - The gRPC response data
/// * `expected` - The expected JSON value
///
/// # Example
///
/// ```ignore
/// let response = send_unary_request(...).await?;
/// assert_grpc_response(response, json!({"id": 1, "name": "Alice"}));
/// ```
pub fn assert_grpc_response(response: &GrpcResponseData, expected: &Value) {
    let actual = serde_json::from_slice::<Value>(&response.payload).expect("Failed to parse response payload as JSON");

    assert_eq!(
        actual, *expected,
        "Response payload mismatch.\nExpected: {expected}\nActual: {actual}",
    );
}

/// Assert that a gRPC handler result has a specific status code
///
/// Useful for testing error handling and status code responses.
///
/// # Panics
///
/// Panics if the status code doesn't match.
///
/// # Arguments
///
/// * `result` - The gRPC handler result
/// * `expected_status` - The expected `tonic::Code`
///
/// # Example
///
/// ```ignore
/// let result = handler.call(request).await;
/// assert_grpc_status(&result, tonic::Code::NotFound);
/// ```
pub fn assert_grpc_status(result: &GrpcHandlerResult, expected_status: Code) {
    match result {
        Err(status) => {
            assert_eq!(
                status.code(),
                expected_status,
                "Expected status {:?} but got {:?}: {}",
                expected_status,
                status.code(),
                status.message()
            );
        }
        Ok(_) => {
            panic!("Expected error status {expected_status:?} but got success response");
        }
    }
}

/// Fluent builder for creating test protobuf messages
///
/// Provides a simple way to construct JSON-serializable test messages
/// that can be used as gRPC payloads. In practice, you would use actual
/// protobuf code generation, but this is useful for simple tests.
///
/// # Example
///
/// ```ignore
/// let message = ProtobufMessageBuilder::new()
///     .add_field("id", 42)
///     .add_field("name", "Alice")
///     .add_field("email", "alice@example.com")
///     .build()?;
/// ```
pub struct ProtobufMessageBuilder {
    fields: serde_json::Map<String, Value>,
}

impl ProtobufMessageBuilder {
    /// Create a new message builder with no fields
    pub fn new() -> Self {
        Self {
            fields: serde_json::Map::new(),
        }
    }

    /// Add a field to the message
    ///
    /// # Arguments
    ///
    /// * `name` - The field name
    /// * `value` - The field value (automatically converted to JSON)
    ///
    /// # Example
    ///
    /// ```ignore
    /// builder.add_field("name", "Alice")
    ///        .add_field("age", 30)
    ///        .add_field("active", true);
    /// ```
    pub fn add_field(&mut self, name: &str, value: impl Into<Value>) -> &mut Self {
        self.fields.insert(name.to_string(), value.into());
        self
    }

    /// Add a string field to the message
    pub fn add_string_field(&mut self, name: &str, value: &str) -> &mut Self {
        self.fields.insert(name.to_string(), Value::String(value.to_string()));
        self
    }

    /// Add an integer field to the message
    pub fn add_int_field(&mut self, name: &str, value: i64) -> &mut Self {
        self.fields.insert(name.to_string(), json!(value));
        self
    }

    /// Add a boolean field to the message
    pub fn add_bool_field(&mut self, name: &str, value: bool) -> &mut Self {
        self.fields.insert(name.to_string(), json!(value));
        self
    }

    /// Add a nested object field to the message
    pub fn add_object_field(&mut self, name: &str, value: Value) -> &mut Self {
        self.fields.insert(name.to_string(), value);
        self
    }

    /// Add an array field to the message
    pub fn add_array_field(&mut self, name: &str, values: Vec<Value>) -> &mut Self {
        self.fields.insert(name.to_string(), Value::Array(values));
        self
    }

    /// Build the message into serialized bytes
    ///
    /// # Errors
    ///
    /// Returns an error if the message cannot be serialized to JSON.
    pub fn build(&self) -> Result<Bytes, Box<dyn std::error::Error>> {
        let json_value = Value::Object(self.fields.clone());
        let serialized = serde_json::to_vec(&json_value)?;
        Ok(Bytes::from(serialized))
    }

    /// Clear all fields from the message
    pub fn clear(&mut self) -> &mut Self {
        self.fields.clear();
        self
    }

    /// Get the current fields as a JSON object
    pub fn as_json(&self) -> Value {
        Value::Object(self.fields.clone())
    }

    /// Get the number of fields in the message
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }
}

impl Default for ProtobufMessageBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Create test metadata with common default headers
///
/// Useful for constructing standard gRPC request metadata for testing.
///
/// # Example
///
/// ```ignore
/// let metadata = create_test_metadata();
/// // metadata now contains typical gRPC headers
/// ```
pub fn create_test_metadata() -> MetadataMap {
    let mut metadata = MetadataMap::new();
    metadata.insert("user-agent", "spikard-test/1.0".parse().unwrap());
    metadata.insert("content-type", "application/grpc".parse().unwrap());
    metadata
}

/// Create test metadata with custom headers
///
/// Allows building metadata from a `HashMap` of key-value pairs.
///
/// # Arguments
///
/// * `headers` - `HashMap` of header names to values (String-based)
///
/// # Example
///
/// ```ignore
/// use std::collections::HashMap;
///
/// let mut headers = HashMap::new();
/// headers.insert("authorization".to_string(), "Bearer token123".to_string());
/// headers.insert("x-custom".to_string(), "value".to_string());
///
/// let metadata = create_test_metadata_with_headers(&headers).unwrap();
/// ```
pub fn create_test_metadata_with_headers(
    headers: &std::collections::HashMap<String, String>,
) -> Result<MetadataMap, Box<dyn std::error::Error>> {
    use std::str::FromStr;
    let mut metadata = MetadataMap::new();
    for (key, value) in headers {
        let meta_key = tonic::metadata::MetadataKey::from_str(key)?;
        metadata.insert(meta_key, value.parse()?);
    }
    Ok(metadata)
}

/// Add authentication metadata to an existing `MetadataMap`
///
/// Adds a standard Bearer token authorization header.
///
/// # Arguments
///
/// * `metadata` - The metadata map to modify
/// * `token` - The authentication token
///
/// # Example
///
/// ```ignore
/// let mut metadata = create_test_metadata();
/// add_auth_metadata(&mut metadata, "secret_token_123");
/// ```
pub fn add_auth_metadata(metadata: &mut MetadataMap, token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let auth_value = format!("Bearer {token}");
    metadata.insert("authorization", auth_value.parse()?);
    Ok(())
}

/// Add custom metadata header to an existing `MetadataMap`
///
/// # Arguments
///
/// * `metadata` - The metadata map to modify
/// * `key` - The header name
/// * `value` - The header value
///
/// # Example
///
/// ```ignore
/// let mut metadata = create_test_metadata();
/// add_metadata_header(&mut metadata, "x-request-id", "req-123")?;
/// ```
pub fn add_metadata_header(
    metadata: &mut MetadataMap,
    key: &str,
    value: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::str::FromStr;
    let meta_key = tonic::metadata::MetadataKey::from_str(key)?;
    metadata.insert(meta_key, value.parse()?);
    Ok(())
}

/// Mock gRPC handler for testing
///
/// A simple mock handler that always returns a fixed response. Useful for
/// basic integration tests that just need a handler to exist.
pub struct MockGrpcHandler {
    service_name: &'static str,
    response_payload: Bytes,
}

impl MockGrpcHandler {
    /// Create a new mock handler
    ///
    /// # Arguments
    ///
    /// * `service_name` - The service name this handler serves (must be a static string)
    /// * `response_payload` - The payload to return in responses
    pub fn new(service_name: &'static str, response_payload: impl Into<Bytes>) -> Self {
        Self {
            service_name,
            response_payload: response_payload.into(),
        }
    }

    /// Create a mock handler that returns JSON
    pub fn with_json(service_name: &'static str, json: &Value) -> Self {
        let bytes = serde_json::to_vec(json).unwrap_or_default();
        Self::new(service_name, Bytes::from(bytes))
    }
}

impl GrpcHandler for MockGrpcHandler {
    fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        let response = GrpcResponseData {
            payload: self.response_payload.clone(),
            metadata: MetadataMap::new(),
        };
        Box::pin(async move { Ok(response) })
    }

    fn service_name(&self) -> &'static str {
        self.service_name
    }
}

/// Error mock handler for testing error responses
///
/// A mock handler that always returns an error. Useful for testing error handling.
pub struct ErrorMockHandler {
    service_name: String,
    error_code: Code,
    error_message: String,
}

impl ErrorMockHandler {
    /// Create a new error mock handler
    ///
    /// # Arguments
    ///
    /// * `service_name` - The service name this handler serves
    /// * `error_code` - The gRPC error code to return
    /// * `error_message` - The error message
    pub fn new(service_name: impl Into<String>, error_code: Code, error_message: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
            error_code,
            error_message: error_message.into(),
        }
    }
}

impl GrpcHandler for ErrorMockHandler {
    fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        let message = self.error_message.clone();
        let code = self.error_code;
        Box::pin(async move { Err(tonic::Status::new(code, message)) })
    }

    fn service_name(&self) -> &'static str {
        "mock.ErrorMockService"
    }
}

/// Echo mock handler for testing request/response flow
///
/// A mock handler that echoes the request payload back as the response.
pub struct EchoMockHandler {
    service_name: String,
}

impl EchoMockHandler {
    /// Create a new echo mock handler
    ///
    /// # Arguments
    ///
    /// * `service_name` - The service name this handler serves
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
        }
    }
}

impl GrpcHandler for EchoMockHandler {
    fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        let payload = request.payload;
        Box::pin(async move {
            Ok(GrpcResponseData {
                payload,
                metadata: MetadataMap::new(),
            })
        })
    }

    fn service_name(&self) -> &'static str {
        "mock.EchoMockService"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock handler for testing gRPC functionality
    struct TestHandler;
    impl GrpcHandler for TestHandler {
        fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
            Box::pin(async {
                Ok(GrpcResponseData {
                    payload: serde_json::to_vec(&json!({"result": "success"})).unwrap().into(),
                    metadata: MetadataMap::new(),
                })
            })
        }
        fn service_name(&self) -> &'static str {
            "test.TestService"
        }
    }

    #[test]
    fn test_grpc_test_server_new() {
        let server = GrpcTestServer::new();
        assert_eq!(server.url(), "http://localhost:50051");
        assert_eq!(server.service_count(), 0);
        assert!(server.handlers().is_empty());
    }

    #[test]
    fn test_grpc_test_server_with_url() {
        let server = GrpcTestServer::with_url("http://localhost:8080");
        assert_eq!(server.url(), "http://localhost:8080");
    }

    #[test]
    fn test_grpc_test_server_register_service() {
        let server = GrpcTestServer::new();
        let handler = Arc::new(MockGrpcHandler::new("test.Service", "response"));

        server.register_service(handler);

        assert_eq!(server.service_count(), 1);
        assert!(!server.handlers().is_empty());
    }

    #[test]
    fn test_grpc_test_server_register_multiple_services() {
        let server = GrpcTestServer::new();
        let handler1 = Arc::new(MockGrpcHandler::new("service1", "response1"));
        let handler2 = Arc::new(MockGrpcHandler::new("service2", "response2"));

        server.register_service(handler1);
        server.register_service(handler2);

        assert_eq!(server.service_count(), 2);
    }

    #[test]
    fn test_grpc_test_server_default() {
        let server = GrpcTestServer::default();
        assert_eq!(server.service_count(), 0);
    }

    #[tokio::test]
    async fn test_mock_grpc_handler_basic() {
        let handler = MockGrpcHandler::new("test.Service", Bytes::from("response"));
        let request = GrpcRequestData {
            service_name: "test.Service".to_string(),
            method_name: "TestMethod".to_string(),
            payload: Bytes::from("request"),
            metadata: MetadataMap::new(),
        };

        let result = handler.call(request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.payload, Bytes::from("response"));
    }

    #[tokio::test]
    async fn test_mock_grpc_handler_with_json() {
        let json_response = json!({"id": 1, "name": "Alice"});
        let handler = MockGrpcHandler::with_json("test.UserService", &json_response);

        let request = GrpcRequestData {
            service_name: "test.UserService".to_string(),
            method_name: "GetUser".to_string(),
            payload: Bytes::from("{}"),
            metadata: MetadataMap::new(),
        };

        let result = handler.call(request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        let deserialized = serde_json::from_slice::<Value>(&response.payload).unwrap();
        assert_eq!(deserialized, json_response);
    }

    #[tokio::test]
    async fn test_error_mock_handler() {
        let handler = ErrorMockHandler::new("test.Service", Code::NotFound, "Resource not found");

        let request = GrpcRequestData {
            service_name: "test.Service".to_string(),
            method_name: "GetResource".to_string(),
            payload: Bytes::new(),
            metadata: MetadataMap::new(),
        };

        let result = handler.call(request).await;
        assert!(result.is_err());

        let status = result.unwrap_err();
        assert_eq!(status.code(), Code::NotFound);
        assert_eq!(status.message(), "Resource not found");
    }

    #[tokio::test]
    async fn test_echo_mock_handler() {
        let handler = EchoMockHandler::new("test.Service");
        let payload = Bytes::from("echo this");

        let request = GrpcRequestData {
            service_name: "test.Service".to_string(),
            method_name: "Echo".to_string(),
            payload: payload.clone(),
            metadata: MetadataMap::new(),
        };

        let result = handler.call(request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.payload, payload);
    }

    #[test]
    fn test_protobuf_message_builder_new() {
        let builder = ProtobufMessageBuilder::new();
        assert_eq!(builder.field_count(), 0);
    }

    #[test]
    fn test_protobuf_message_builder_add_field() {
        let mut builder = ProtobufMessageBuilder::new();
        builder.add_field("name", "Alice").add_field("age", 30);

        assert_eq!(builder.field_count(), 2);
    }

    #[test]
    fn test_protobuf_message_builder_add_string_field() {
        let mut builder = ProtobufMessageBuilder::new();
        builder.add_string_field("name", "Bob");

        let json = builder.as_json();
        assert_eq!(json["name"], "Bob");
    }

    #[test]
    fn test_protobuf_message_builder_add_int_field() {
        let mut builder = ProtobufMessageBuilder::new();
        builder.add_int_field("age", 42);

        let json = builder.as_json();
        assert_eq!(json["age"], 42);
    }

    #[test]
    fn test_protobuf_message_builder_add_bool_field() {
        let mut builder = ProtobufMessageBuilder::new();
        builder.add_bool_field("active", true);

        let json = builder.as_json();
        assert_eq!(json["active"], true);
    }

    #[test]
    fn test_protobuf_message_builder_add_object_field() {
        let mut builder = ProtobufMessageBuilder::new();
        builder.add_object_field("user", json!({"name": "Alice", "id": 1}));

        let json = builder.as_json();
        assert_eq!(json["user"]["name"], "Alice");
        assert_eq!(json["user"]["id"], 1);
    }

    #[test]
    fn test_protobuf_message_builder_add_array_field() {
        let mut builder = ProtobufMessageBuilder::new();
        builder.add_array_field("tags", vec![json!("rust"), json!("testing"), json!("grpc")]);

        let json = builder.as_json();
        assert_eq!(json["tags"].as_array().unwrap().len(), 3);
        assert_eq!(json["tags"][0], "rust");
    }

    #[test]
    fn test_protobuf_message_builder_build() {
        let mut builder = ProtobufMessageBuilder::new();
        builder.add_field("id", 1).add_field("name", "Alice");

        let bytes = builder.build().unwrap();
        let deserialized = serde_json::from_slice::<Value>(&bytes).unwrap();

        assert_eq!(deserialized["id"], 1);
        assert_eq!(deserialized["name"], "Alice");
    }

    #[test]
    fn test_protobuf_message_builder_clear() {
        let mut builder = ProtobufMessageBuilder::new();
        builder.add_field("id", 1).add_field("name", "Alice");
        assert_eq!(builder.field_count(), 2);

        builder.clear();
        assert_eq!(builder.field_count(), 0);
    }

    #[test]
    fn test_protobuf_message_builder_fluent_api() {
        let mut builder = ProtobufMessageBuilder::new();
        builder
            .add_string_field("email", "alice@example.com")
            .add_int_field("age", 30)
            .add_bool_field("verified", true);

        assert_eq!(builder.field_count(), 3);

        let json = builder.as_json();
        assert_eq!(json["email"], "alice@example.com");
        assert_eq!(json["age"], 30);
        assert_eq!(json["verified"], true);
    }

    #[test]
    fn test_create_test_metadata() {
        let metadata = create_test_metadata();

        assert!(metadata.get("user-agent").is_some());
        assert!(metadata.get("content-type").is_some());
    }

    #[test]
    fn test_create_test_metadata_with_headers() {
        use std::collections::HashMap;

        let mut headers = HashMap::new();
        headers.insert("authorization".to_string(), "Bearer token".to_string());
        headers.insert("x-custom".to_string(), "value".to_string());

        let metadata = create_test_metadata_with_headers(&headers).unwrap();

        assert!(metadata.get("authorization").is_some());
        assert!(metadata.get("x-custom").is_some());
    }

    #[test]
    fn test_add_auth_metadata() {
        let mut metadata = create_test_metadata();
        add_auth_metadata(&mut metadata, "secret_token_123").unwrap();

        let auth_header = metadata.get("authorization").unwrap();
        assert_eq!(auth_header.to_str().unwrap(), "Bearer secret_token_123");
    }

    #[test]
    fn test_add_metadata_header() {
        let mut metadata = create_test_metadata();
        add_metadata_header(&mut metadata, "x-request-id", "req-123").unwrap();

        let header = metadata.get("x-request-id").unwrap();
        assert_eq!(header.to_str().unwrap(), "req-123");
    }

    #[test]
    fn test_assert_grpc_response_matching() {
        let response = GrpcResponseData {
            payload: Bytes::from(r#"{"id": 1, "name": "Alice"}"#),
            metadata: MetadataMap::new(),
        };

        let expected = json!({"id": 1, "name": "Alice"});
        assert_grpc_response(&response, &expected);
    }

    #[test]
    fn test_assert_grpc_status_error() {
        let result: GrpcHandlerResult = Err(tonic::Status::not_found("Resource not found"));

        assert_grpc_status(&result, Code::NotFound);
    }

    #[test]
    #[should_panic(expected = "Expected status")]
    fn test_assert_grpc_status_mismatch() {
        let result: GrpcHandlerResult = Err(tonic::Status::not_found("Resource not found"));

        assert_grpc_status(&result, Code::InvalidArgument);
    }

    #[test]
    #[should_panic(expected = "Expected error status")]
    fn test_assert_grpc_status_on_success() {
        let result: GrpcHandlerResult = Ok(GrpcResponseData {
            payload: Bytes::new(),
            metadata: MetadataMap::new(),
        });

        assert_grpc_status(&result, Code::NotFound);
    }

    #[test]
    fn test_protobuf_message_builder_default() {
        let builder = ProtobufMessageBuilder::default();
        assert_eq!(builder.field_count(), 0);
    }

    #[tokio::test]
    async fn test_send_unary_request_with_mock_handler() {
        let server = GrpcTestServer::new();
        let _response_payload = json!({"result": "success"});

        server.register_service(Arc::new(TestHandler));

        let message = ProtobufMessageBuilder::new()
            .add_field("input", "test")
            .build()
            .unwrap();

        let result = send_unary_request(
            &server,
            "test.TestService",
            "TestMethod",
            message,
            create_test_metadata(),
        )
        .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_unary_request_service_not_found() {
        let server = GrpcTestServer::new();
        let message = Bytes::from("test");

        let result = send_unary_request(
            &server,
            "nonexistent.Service",
            "Method",
            message,
            create_test_metadata(),
        )
        .await;

        assert!(result.is_err());
    }

    #[test]
    fn test_error_mock_handler_invalid_argument() {
        let handler = ErrorMockHandler::new("test.Service", Code::InvalidArgument, "Bad input");
        assert_eq!(handler.error_code, Code::InvalidArgument);
        assert_eq!(handler.error_message, "Bad input");
    }

    #[test]
    fn test_echo_mock_handler_creates_with_service_name() {
        let handler = EchoMockHandler::new("mypackage.EchoService");
        assert_eq!(handler.service_name, "mypackage.EchoService");
    }

    #[test]
    fn test_grpc_test_client_creation() {
        let _client = create_grpc_test_client();
        let _new_client = GrpcTestClient::new();
        // Just ensure it can be created without panicking
    }
}

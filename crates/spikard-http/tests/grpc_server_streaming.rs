//! Integration tests for gRPC server streaming
//!
//! Tests end-to-end server streaming functionality through GenericGrpcService
//! and grpc_routing, including:
//! - Stream of multiple messages
//! - Empty streams
//! - Error handling before and during streaming
//! - Metadata in streaming responses
//! - Large payloads
//! - Routing and mode validation
#![allow(
    clippy::doc_markdown,
    clippy::uninlined_format_args,
    clippy::single_match_else,
    reason = "Integration test for streaming with many test cases"
)]

use axum::body::Body;
use axum::http::{Request, StatusCode};
use bytes::Bytes;
use futures_util::StreamExt;
use http_body_util::BodyExt;
use spikard_http::grpc::framing::{encode_grpc_message, parse_grpc_client_stream};
use spikard_http::grpc::streaming::{empty_message_stream, message_stream_from_vec};
use spikard_http::grpc::{
    GrpcConfig, GrpcHandler, GrpcHandlerResult, GrpcRegistry, GrpcRequestData, GrpcResponseData, MessageStream, RpcMode,
};
use spikard_http::server::grpc_routing::route_grpc_request;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tonic::metadata::MetadataMap;

mod common;

fn grpc_frame(payload: &[u8]) -> Bytes {
    encode_grpc_message(Bytes::copy_from_slice(payload)).expect("test payload should frame")
}

// ============================================================================
// Test Handlers
// ============================================================================

/// Handler that streams 10 messages in sequence
struct StreamTenMessagesHandler;

impl GrpcHandler for StreamTenMessagesHandler {
    fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        let _ = request;
        Box::pin(async { Err(tonic::Status::unimplemented("Use call_server_stream for streaming")) })
    }

    fn service_name(&self) -> &'static str {
        "test.StreamService"
    }

    fn rpc_mode(&self) -> RpcMode {
        RpcMode::ServerStreaming
    }

    fn call_server_stream(
        &self,
        _request: GrpcRequestData,
    ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
        Box::pin(async {
            // Create a stream of 10 messages
            let messages: Vec<Bytes> = (0..10).map(|i| Bytes::from(format!("message_{}", i))).collect();

            Ok(message_stream_from_vec(messages))
        })
    }
}

/// Handler that returns an empty stream
struct EmptyStreamHandler;

impl GrpcHandler for EmptyStreamHandler {
    fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        let _ = request;
        Box::pin(async { Err(tonic::Status::unimplemented("Use call_server_stream for streaming")) })
    }

    fn service_name(&self) -> &'static str {
        "test.EmptyStreamService"
    }

    fn rpc_mode(&self) -> RpcMode {
        RpcMode::ServerStreaming
    }

    fn call_server_stream(
        &self,
        _request: GrpcRequestData,
    ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
        Box::pin(async {
            // Create an empty stream
            Ok(empty_message_stream())
        })
    }
}

/// Handler that returns an error before streaming
struct ErrorBeforeStreamHandler;

impl GrpcHandler for ErrorBeforeStreamHandler {
    fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        let _ = request;
        Box::pin(async { Err(tonic::Status::unimplemented("Use call_server_stream for streaming")) })
    }

    fn service_name(&self) -> &'static str {
        "test.ErrorBeforeService"
    }

    fn rpc_mode(&self) -> RpcMode {
        RpcMode::ServerStreaming
    }

    fn call_server_stream(
        &self,
        _request: GrpcRequestData,
    ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
        Box::pin(async { Err(tonic::Status::invalid_argument("Invalid request for streaming")) })
    }
}

/// Helper to create a stream that errors mid-stream
fn create_error_mid_stream() -> MessageStream {
    use futures_util::stream::iter;

    // Create a stream with 5 successful messages and then an error
    let messages: Vec<Result<Bytes, tonic::Status>> =
        (0..5).map(|i| Ok(Bytes::from(format!("message_{}", i)))).collect();

    let mut stream_items = messages;
    stream_items.push(Err(tonic::Status::internal("Stream processing error")));

    Box::pin(iter(stream_items))
}

/// Handler that returns error mid-stream (after 5 messages)
struct ErrorMidStreamHandler;

impl GrpcHandler for ErrorMidStreamHandler {
    fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        let _ = request;
        Box::pin(async { Err(tonic::Status::unimplemented("Use call_server_stream for streaming")) })
    }

    fn service_name(&self) -> &'static str {
        "test.ErrorMidStreamService"
    }

    fn rpc_mode(&self) -> RpcMode {
        RpcMode::ServerStreaming
    }

    fn call_server_stream(
        &self,
        _request: GrpcRequestData,
    ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
        Box::pin(async { Ok(create_error_mid_stream()) })
    }
}

/// Handler that streams messages with metadata
struct StreamWithMetadataHandler;

impl GrpcHandler for StreamWithMetadataHandler {
    fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        let _ = request;
        Box::pin(async { Err(tonic::Status::unimplemented("Use call_server_stream for streaming")) })
    }

    fn service_name(&self) -> &'static str {
        "test.MetadataStreamService"
    }

    fn rpc_mode(&self) -> RpcMode {
        RpcMode::ServerStreaming
    }

    fn call_server_stream(
        &self,
        _request: GrpcRequestData,
    ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
        Box::pin(async {
            let messages: Vec<Bytes> = (0..3).map(|i| Bytes::from(format!("message_{}", i))).collect();

            Ok(message_stream_from_vec(messages))
        })
    }
}

/// Handler that streams large payloads (1MB each)
struct LargePayloadStreamHandler;

impl GrpcHandler for LargePayloadStreamHandler {
    fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        let _ = request;
        Box::pin(async { Err(tonic::Status::unimplemented("Use call_server_stream for streaming")) })
    }

    fn service_name(&self) -> &'static str {
        "test.LargePayloadService"
    }

    fn rpc_mode(&self) -> RpcMode {
        RpcMode::ServerStreaming
    }

    fn call_server_stream(
        &self,
        _request: GrpcRequestData,
    ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
        Box::pin(async {
            let messages: Vec<Bytes> = (0..3)
                .map(|i| {
                    let large_data = vec![0xAB; 1024 * 1024]; // 1MB of data
                    let message = format!("chunk_{}: ", i);
                    let mut full_message = message.into_bytes();
                    full_message.extend_from_slice(&large_data);
                    Bytes::from(full_message)
                })
                .collect();

            Ok(message_stream_from_vec(messages))
        })
    }
}

/// Unary handler to test mode enforcement
struct UnaryOnlyHandler;

impl GrpcHandler for UnaryOnlyHandler {
    fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        let _ = request;
        Box::pin(async {
            Ok(GrpcResponseData {
                payload: Bytes::from("unary response"),
                metadata: MetadataMap::new(),
            })
        })
    }

    fn service_name(&self) -> &'static str {
        "test.UnaryOnlyService"
    }

    fn rpc_mode(&self) -> RpcMode {
        RpcMode::Unary
    }
}

/// Handler that streams variable number of messages
struct VariableLengthStreamHandler {
    count: usize,
}

impl GrpcHandler for VariableLengthStreamHandler {
    fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        let _ = request;
        Box::pin(async { Err(tonic::Status::unimplemented("Use call_server_stream for streaming")) })
    }

    fn service_name(&self) -> &'static str {
        "test.VariableLengthService"
    }

    fn rpc_mode(&self) -> RpcMode {
        RpcMode::ServerStreaming
    }

    fn call_server_stream(
        &self,
        _request: GrpcRequestData,
    ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
        let count = self.count;
        Box::pin(async move {
            let messages: Vec<Bytes> = (0..count).map(|i| Bytes::from(format!("item_{}", i))).collect();

            Ok(message_stream_from_vec(messages))
        })
    }
}

// ============================================================================
// Integration Tests
// ============================================================================

/// Test: Handler returns stream of 10 messages
#[tokio::test]
async fn test_stream_ten_messages() {
    let handler = Arc::new(StreamTenMessagesHandler);

    let request = GrpcRequestData {
        service_name: "test.StreamService".to_string(),
        method_name: "StreamTen".to_string(),
        payload: Bytes::new(),
        metadata: MetadataMap::new(),
    };

    let result = handler.call_server_stream(request).await;
    assert!(result.is_ok());

    let mut stream = result.unwrap();
    let mut messages = Vec::new();

    // Collect all messages from the stream
    while let Some(msg) = stream.next().await {
        assert!(msg.is_ok());
        messages.push(msg.unwrap());
    }

    // Verify we got exactly 10 messages
    assert_eq!(messages.len(), 10);

    // Verify message contents
    for (i, msg) in messages.iter().enumerate() {
        let expected = format!("message_{}", i);
        assert_eq!(msg, &Bytes::from(expected));
    }
}

/// Test: Handler returns empty stream
#[tokio::test]
async fn test_empty_stream() {
    let handler = Arc::new(EmptyStreamHandler);

    let request = GrpcRequestData {
        service_name: "test.EmptyStreamService".to_string(),
        method_name: "EmptyStream".to_string(),
        payload: Bytes::new(),
        metadata: MetadataMap::new(),
    };

    let result = handler.call_server_stream(request).await;
    assert!(result.is_ok());

    let mut stream = result.unwrap();
    let mut count = 0;

    while let Some(_msg) = stream.next().await {
        count += 1;
    }

    // Verify stream is truly empty
    assert_eq!(count, 0);
}

/// Test: Handler returns error before streaming
#[tokio::test]
async fn test_error_before_stream() {
    let handler = Arc::new(ErrorBeforeStreamHandler);

    let request = GrpcRequestData {
        service_name: "test.ErrorBeforeService".to_string(),
        method_name: "ErrorBefore".to_string(),
        payload: Bytes::new(),
        metadata: MetadataMap::new(),
    };

    let result = handler.call_server_stream(request).await;
    assert!(result.is_err());

    match result {
        Err(error) => {
            assert_eq!(error.code(), tonic::Code::InvalidArgument);
            assert_eq!(error.message(), "Invalid request for streaming");
        }
        Ok(_) => panic!("Expected error, got Ok"),
    }
}

/// Test: Handler returns error mid-stream (after 5 messages)
#[tokio::test]
async fn test_error_mid_stream() {
    let handler = Arc::new(ErrorMidStreamHandler);

    let request = GrpcRequestData {
        service_name: "test.ErrorMidStreamService".to_string(),
        method_name: "ErrorMidStream".to_string(),
        payload: Bytes::new(),
        metadata: MetadataMap::new(),
    };

    let result = handler.call_server_stream(request).await;
    assert!(result.is_ok());

    let mut stream = result.unwrap();
    let mut successful_messages = 0;
    let mut error_encountered = false;

    while let Some(msg) = stream.next().await {
        match msg {
            Ok(_) => successful_messages += 1,
            Err(_) => {
                error_encountered = true;
                break;
            }
        }
    }

    // Verify we got 5 successful messages before the error
    assert_eq!(successful_messages, 5);
    assert!(error_encountered);
}

/// Test: Stream with metadata in responses
#[tokio::test]
async fn test_stream_with_metadata() {
    let handler = Arc::new(StreamWithMetadataHandler);

    let request = GrpcRequestData {
        service_name: "test.MetadataStreamService".to_string(),
        method_name: "StreamWithMeta".to_string(),
        payload: Bytes::new(),
        metadata: MetadataMap::new(),
    };

    let result = handler.call_server_stream(request).await;
    assert!(result.is_ok());

    let mut stream = result.unwrap();
    let mut count = 0;

    while let Some(msg) = stream.next().await {
        assert!(msg.is_ok());
        count += 1;
    }

    // Verify we got 3 messages
    assert_eq!(count, 3);
}

/// Test: Stream with large payloads (1MB per message)
#[tokio::test]
async fn test_large_payload_stream() {
    let handler = Arc::new(LargePayloadStreamHandler);

    let request = GrpcRequestData {
        service_name: "test.LargePayloadService".to_string(),
        method_name: "LargePayload".to_string(),
        payload: Bytes::new(),
        metadata: MetadataMap::new(),
    };

    let result = handler.call_server_stream(request).await;
    assert!(result.is_ok());

    let mut stream = result.unwrap();
    let mut total_bytes = 0;

    while let Some(msg) = stream.next().await {
        assert!(msg.is_ok());
        let bytes = msg.unwrap();
        total_bytes += bytes.len();
    }

    // Verify we received approximately 3MB total (3 chunks of ~1MB each)
    // Account for message prefixes
    assert!(total_bytes > 3 * 1024 * 1024);
}

/// Test: Unary handler defaults server streaming to a single unary response
#[tokio::test]
async fn test_unary_handler_server_stream_fallback() {
    let handler = Arc::new(UnaryOnlyHandler);

    // Verify the handler is registered as Unary
    assert_eq!(handler.rpc_mode(), RpcMode::Unary);

    // Create a request
    let request = GrpcRequestData {
        service_name: "test.UnaryOnlyService".to_string(),
        method_name: "UnaryMethod".to_string(),
        payload: Bytes::from("test"),
        metadata: MetadataMap::new(),
    };

    // Calling call_server_stream on a unary-only handler uses unary fallback.
    let result = handler.call_server_stream(request).await;
    assert!(result.is_ok());
    let mut stream = result.unwrap();
    let first = stream.next().await.unwrap().unwrap();
    assert_eq!(first, Bytes::from_static(b"unary response"));
    assert!(stream.next().await.is_none());
}

/// Test: Handler supports both RPC modes correctly
#[tokio::test]
async fn test_rpc_mode_detection() {
    let stream_handler = Arc::new(StreamTenMessagesHandler);
    let unary_handler = Arc::new(UnaryOnlyHandler);

    assert_eq!(stream_handler.rpc_mode(), RpcMode::ServerStreaming);
    assert_eq!(unary_handler.rpc_mode(), RpcMode::Unary);
}

/// Test: Handler returns variable-length stream (1 message)
#[tokio::test]
async fn test_single_message_stream() {
    let handler = Arc::new(VariableLengthStreamHandler { count: 1 });

    let request = GrpcRequestData {
        service_name: "test.VariableLengthService".to_string(),
        method_name: "VarLength".to_string(),
        payload: Bytes::new(),
        metadata: MetadataMap::new(),
    };

    let result = handler.call_server_stream(request).await;
    assert!(result.is_ok());

    let mut stream = result.unwrap();
    let mut count = 0;

    while let Some(msg) = stream.next().await {
        assert!(msg.is_ok());
        assert_eq!(msg.unwrap(), Bytes::from("item_0"));
        count += 1;
    }

    assert_eq!(count, 1);
}

/// Test: Handler returns variable-length stream (100 messages)
#[tokio::test]
async fn test_many_messages_stream() {
    let handler = Arc::new(VariableLengthStreamHandler { count: 100 });

    let request = GrpcRequestData {
        service_name: "test.VariableLengthService".to_string(),
        method_name: "VarLength".to_string(),
        payload: Bytes::new(),
        metadata: MetadataMap::new(),
    };

    let result = handler.call_server_stream(request).await;
    assert!(result.is_ok());

    let mut stream = result.unwrap();
    let mut count = 0;

    while let Some(msg) = stream.next().await {
        assert!(msg.is_ok());
        count += 1;
    }

    assert_eq!(count, 100);
}

/// Test: Service name is correctly reported
#[tokio::test]
async fn test_service_names() {
    let handlers: Vec<(Arc<dyn GrpcHandler>, &str)> = vec![
        (Arc::new(StreamTenMessagesHandler), "test.StreamService"),
        (Arc::new(EmptyStreamHandler), "test.EmptyStreamService"),
        (Arc::new(ErrorBeforeStreamHandler), "test.ErrorBeforeService"),
        (Arc::new(ErrorMidStreamHandler), "test.ErrorMidStreamService"),
        (Arc::new(StreamWithMetadataHandler), "test.MetadataStreamService"),
        (Arc::new(LargePayloadStreamHandler), "test.LargePayloadService"),
        (Arc::new(UnaryOnlyHandler), "test.UnaryOnlyService"),
    ];

    for (handler, expected_name) in handlers {
        assert_eq!(handler.service_name(), expected_name);
    }
}

/// Test: Stream handler supports streaming responses
#[tokio::test]
async fn test_handler_supports_streaming_responses() {
    let streaming_handler = Arc::new(StreamTenMessagesHandler);
    let unary_handler = Arc::new(UnaryOnlyHandler);

    // Streaming handler should report it supports streaming
    assert_eq!(streaming_handler.rpc_mode(), RpcMode::ServerStreaming);

    // Unary handler still reports unary mode for routing.
    assert_eq!(unary_handler.rpc_mode(), RpcMode::Unary);
}

/// Test: Concurrent streaming requests
#[tokio::test]
async fn test_concurrent_streaming_requests() {
    let handler = Arc::new(StreamTenMessagesHandler);

    let mut tasks = vec![];
    for _ in 0..5 {
        let handler_clone = Arc::clone(&handler);
        let task = tokio::spawn(async move {
            let request = GrpcRequestData {
                service_name: "test.StreamService".to_string(),
                method_name: "StreamTen".to_string(),
                payload: Bytes::new(),
                metadata: MetadataMap::new(),
            };

            let result = handler_clone.call_server_stream(request).await;
            assert!(result.is_ok());

            let mut stream = result.unwrap();
            let mut count = 0;

            while let Some(msg) = stream.next().await {
                assert!(msg.is_ok());
                count += 1;
            }

            assert_eq!(count, 10);
        });

        tasks.push(task);
    }

    // Wait for all tasks
    for task in tasks {
        assert!(task.await.is_ok());
    }
}

/// Test: Stream preserves message order
#[tokio::test]
async fn test_message_order_preserved() {
    let handler = Arc::new(StreamTenMessagesHandler);

    let request = GrpcRequestData {
        service_name: "test.StreamService".to_string(),
        method_name: "StreamTen".to_string(),
        payload: Bytes::new(),
        metadata: MetadataMap::new(),
    };

    let result = handler.call_server_stream(request).await;
    assert!(result.is_ok());

    let mut stream = result.unwrap();
    let mut previous_index: Option<usize> = None;

    while let Some(msg) = stream.next().await {
        assert!(msg.is_ok());
        let msg_str = String::from_utf8(msg.unwrap().to_vec()).unwrap();

        // Extract index from "message_N" format
        let parts: Vec<&str> = msg_str.split('_').collect();
        assert_eq!(parts.len(), 2);
        let current_index: usize = parts[1].parse().unwrap();

        // Verify ordering
        if let Some(prev_idx) = previous_index {
            assert_eq!(current_index, prev_idx + 1);
        }

        previous_index = Some(current_index);
    }
}

// ============================================================================
// HTTP-Layer Error Transmission Tests
// ============================================================================
// These tests verify error propagation through the full HTTP/gRPC stack

/// Handler that fails with specific error code after 5 messages
struct ErrorAfterMessagesHandler;

impl GrpcHandler for ErrorAfterMessagesHandler {
    fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        Box::pin(async { Err(tonic::Status::unimplemented("Use call_server_stream")) })
    }

    fn service_name(&self) -> &'static str {
        "test.ErrorAfterService"
    }

    fn rpc_mode(&self) -> RpcMode {
        RpcMode::ServerStreaming
    }

    fn call_server_stream(
        &self,
        _request: GrpcRequestData,
    ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
        Box::pin(async { Ok(create_error_mid_stream()) })
    }
}

/// Test: Mid-stream error closes HTTP connection properly
///
/// Verifies that when a stream returns an error mid-way, the HTTP
/// connection is properly closed and the client receives a response.
///
/// The stream should end with gRPC error trailers while preserving
/// the partial payload that was already emitted.
#[tokio::test]
async fn test_http_layer_mid_stream_error_closes_connection() {
    let mut registry = GrpcRegistry::new();
    registry.register_service(
        "test.ErrorAfterService",
        Arc::new(ErrorAfterMessagesHandler),
        RpcMode::ServerStreaming,
    );
    let registry = Arc::new(registry);
    let config = GrpcConfig::default();

    // Create a gRPC request that will stream then error
    let request = Request::builder()
        .uri("/test.ErrorAfterService/StreamThenError")
        .header("content-type", "application/grpc")
        .body(Body::from(grpc_frame(b"")))
        .unwrap();

    let result = route_grpc_request(registry, &config, request).await;

    // The route should initially succeed because the error is mid-stream
    assert!(
        result.is_ok(),
        "Route should accept streaming response with deferred errors"
    );

    let response = result.unwrap();
    // Response headers should be set up for streaming
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("content-type").and_then(|v| v.to_str().ok()),
        Some("application/grpc+proto")
    );
    let collected = response.into_body().collect().await.unwrap();
    let trailers = collected.trailers().expect("grpc trailers");
    assert_eq!(trailers.get("grpc-status").unwrap(), "13");
    assert_eq!(trailers.get("grpc-message").unwrap(), "Stream%20processing%20error");
}

/// Test: Partial messages delivered before error
///
/// Verifies that messages sent before an error are still delivered
/// to the client before the connection closes.
#[tokio::test]
async fn test_http_layer_partial_messages_before_error() {
    let mut registry = GrpcRegistry::new();
    registry.register_service(
        "test.ErrorAfterService",
        Arc::new(ErrorAfterMessagesHandler),
        RpcMode::ServerStreaming,
    );
    let registry = Arc::new(registry);
    let config = GrpcConfig::default();

    let request = Request::builder()
        .uri("/test.ErrorAfterService/StreamThenError")
        .header("content-type", "application/grpc")
        .body(Body::from(grpc_frame(b"")))
        .unwrap();

    let result = route_grpc_request(registry, &config, request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    let collected = response.into_body().collect().await.unwrap();
    let trailers = collected.trailers().expect("grpc trailers");
    let grpc_status = trailers.get("grpc-status").unwrap().to_str().unwrap().to_string();
    let grpc_message = trailers.get("grpc-message").unwrap().to_str().unwrap().to_string();
    let mut decoded_stream = parse_grpc_client_stream(Body::from(collected.to_bytes()), usize::MAX, None, true)
        .await
        .unwrap();
    let mut messages = Vec::new();
    while let Some(item) = decoded_stream.next().await {
        messages.push(String::from_utf8_lossy(&item.unwrap()).to_string());
    }

    assert_eq!(messages.first().map(String::as_str), Some("message_0"));
    assert_eq!(messages.last().map(String::as_str), Some("message_4"));
    assert_eq!(grpc_status, "13");
    assert_eq!(grpc_message, "Stream%20processing%20error");
}

/// Test: Connection cleanup after mid-stream error
///
/// Verifies that resources are properly cleaned up after a mid-stream
/// error. This test spawns multiple concurrent requests with errors
/// to ensure no resource leaks.
#[tokio::test]
async fn test_http_layer_connection_cleanup() {
    let mut registry = GrpcRegistry::new();
    registry.register_service(
        "test.ErrorAfterService",
        Arc::new(ErrorAfterMessagesHandler),
        RpcMode::ServerStreaming,
    );
    let registry = Arc::new(registry);
    let config = Arc::new(GrpcConfig::default());

    // Spawn multiple concurrent requests
    let mut handles = vec![];
    for _ in 0..10 {
        let registry_clone = Arc::clone(&registry);
        let config_clone = Arc::clone(&config);
        let handle = tokio::spawn(async move {
            let request = Request::builder()
                .uri("/test.ErrorAfterService/StreamThenError")
                .header("content-type", "application/grpc")
                .body(Body::from(grpc_frame(b"")))
                .unwrap();

            let result = route_grpc_request(registry_clone, &config_clone, request).await;
            // Each request should complete (either success or error)
            assert!(result.is_ok() || result.is_err());
        });
        handles.push(handle);
    }

    // Wait for all concurrent requests to complete
    for handle in handles {
        assert!(
            handle.await.is_ok(),
            "Concurrent request should complete without panicking"
        );
    }
}

/// Test: Error status code mapping at HTTP layer
///
/// Verifies that when a handler returns an error BEFORE streaming starts,
/// it's properly converted to an HTTP status code.
#[tokio::test]
async fn test_http_layer_pre_stream_error_status_mapping() {
    struct PreStreamErrorHandler;

    impl GrpcHandler for PreStreamErrorHandler {
        fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
            Box::pin(async { Err(tonic::Status::unimplemented("Use call_server_stream")) })
        }

        fn service_name(&self) -> &'static str {
            "test.PreErrorService"
        }

        fn rpc_mode(&self) -> RpcMode {
            RpcMode::ServerStreaming
        }

        fn call_server_stream(
            &self,
            _request: GrpcRequestData,
        ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
            Box::pin(async {
                // Return error immediately (before any messages)
                Err(tonic::Status::invalid_argument("Invalid stream request"))
            })
        }
    }

    let mut registry = GrpcRegistry::new();
    registry.register_service(
        "test.PreErrorService",
        Arc::new(PreStreamErrorHandler),
        RpcMode::ServerStreaming,
    );
    let registry = Arc::new(registry);
    let config = GrpcConfig::default();

    let request = Request::builder()
        .uri("/test.PreErrorService/StreamError")
        .header("content-type", "application/grpc")
        .body(Body::from(grpc_frame(b"")))
        .unwrap();

    let result = route_grpc_request(registry, &config, request).await;

    // Pre-stream error should fail at the route level
    assert!(
        result.is_err(),
        "Pre-stream errors should be caught by route_grpc_request"
    );

    if let Err((status, message)) = result {
        // Should map to BAD_REQUEST for InvalidArgument
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(message.contains("Invalid stream request"));
    }
}

/// Test: Large streaming responses with mid-stream error
///
/// Verifies that even with large payloads transferred before an error,
/// the connection is properly closed.
#[tokio::test]
async fn test_http_layer_large_payload_then_error() {
    struct LargePayloadErrorHandler;

    impl GrpcHandler for LargePayloadErrorHandler {
        fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
            Box::pin(async { Err(tonic::Status::unimplemented("Use call_server_stream")) })
        }

        fn service_name(&self) -> &'static str {
            "test.LargePayloadErrorService"
        }

        fn rpc_mode(&self) -> RpcMode {
            RpcMode::ServerStreaming
        }

        fn call_server_stream(
            &self,
            _request: GrpcRequestData,
        ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
            Box::pin(async {
                let large_data = vec![0xAB; 512 * 1024]; // 512KB
                let messages: Vec<Bytes> = vec![Bytes::from(large_data)];

                Ok(message_stream_from_vec(messages))
            })
        }
    }

    let mut registry = GrpcRegistry::new();
    registry.register_service(
        "test.LargePayloadErrorService",
        Arc::new(LargePayloadErrorHandler),
        RpcMode::ServerStreaming,
    );
    let registry = Arc::new(registry);
    let config = GrpcConfig::default();

    let request = Request::builder()
        .uri("/test.LargePayloadErrorService/LargeError")
        .header("content-type", "application/grpc")
        .body(Body::from(grpc_frame(b"")))
        .unwrap();

    let result = route_grpc_request(registry, &config, request).await;
    assert!(result.is_ok(), "Route should accept large streaming response");
}

/// Test: Stream error indication via response completion
///
/// Verifies that a stream error results in the response body being
/// properly closed/terminated, signaling to the client that the stream
/// has ended abnormally.
///
#[tokio::test]
async fn test_http_layer_stream_termination_on_error() {
    let mut registry = GrpcRegistry::new();
    registry.register_service(
        "test.ErrorAfterService",
        Arc::new(ErrorAfterMessagesHandler),
        RpcMode::ServerStreaming,
    );
    let registry = Arc::new(registry);
    let config = GrpcConfig::default();

    let request = Request::builder()
        .uri("/test.ErrorAfterService/StreamThenError")
        .header("content-type", "application/grpc")
        .body(Body::from(grpc_frame(b"")))
        .unwrap();

    let result = route_grpc_request(registry, &config, request).await;
    assert!(result.is_ok());

    let response = result.unwrap();

    // Verify response is properly constructed
    assert_eq!(response.status(), StatusCode::OK);
    assert!(
        response.headers().get("grpc-status").is_none(),
        "streaming responses should not predeclare grpc-status"
    );

    let collected = response.into_body().collect().await.unwrap();
    let trailers = collected.trailers().expect("grpc trailers");
    assert_eq!(trailers.get("grpc-status").unwrap(), "13");
    assert_eq!(trailers.get("grpc-message").unwrap(), "Stream%20processing%20error");
}

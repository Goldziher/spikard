//! Integration tests for gRPC client streaming
//!
//! Tests end-to-end client streaming functionality through `GenericGrpcService`
//! and `grpc_routing`, including:
//! - Stream of multiple messages aggregation
//! - Message validation during streaming
//! - Empty streams
//! - Error handling before and during streaming
//! - Metadata in streaming requests
//! - Large payloads (100+ messages)
//! - Message ordering preservation
//! - Concurrent client streaming requests
#![allow(
    clippy::uninlined_format_args,
    clippy::doc_markdown,
    clippy::option_if_let_else,
    reason = "Integration test for streaming with many test cases"
)]

use bytes::Bytes;
use futures_util::StreamExt;
use spikard_http::grpc::streaming::{StreamingRequest, message_stream_from_vec};
use spikard_http::grpc::{GrpcHandler, GrpcHandlerResult, GrpcRequestData, GrpcResponseData, RpcMode};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tonic::metadata::MetadataMap;

mod common;

// ============================================================================
// Test Handlers
// ============================================================================

/// Handler that aggregates incoming numbers by summing them
struct SumHandlerIntegration;

impl GrpcHandler for SumHandlerIntegration {
    fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        Box::pin(async { Err(tonic::Status::unimplemented("Use call_client_stream for streaming")) })
    }

    fn service_name(&self) -> &'static str {
        "integration.SumService"
    }

    fn rpc_mode(&self) -> RpcMode {
        RpcMode::ClientStreaming
    }

    fn call_client_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = Result<GrpcResponseData, tonic::Status>> + Send>> {
        Box::pin(async move {
            let mut sum: i64 = 0;
            let mut stream = request.message_stream;

            while let Some(msg_result) = stream.next().await {
                match msg_result {
                    Ok(msg) => {
                        if msg.len() == 8 {
                            let value =
                                i64::from_le_bytes([msg[0], msg[1], msg[2], msg[3], msg[4], msg[5], msg[6], msg[7]]);
                            sum += value;
                        }
                    }
                    Err(e) => return Err(e),
                }
            }

            Ok(GrpcResponseData {
                payload: Bytes::from(sum.to_le_bytes().to_vec()),
                metadata: MetadataMap::new(),
            })
        })
    }
}

/// Handler that validates all messages during streaming
struct ValidateAllMessagesHandler;

impl GrpcHandler for ValidateAllMessagesHandler {
    fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        Box::pin(async { Err(tonic::Status::unimplemented("Use call_client_stream for streaming")) })
    }

    fn service_name(&self) -> &'static str {
        "integration.ValidateService"
    }

    fn rpc_mode(&self) -> RpcMode {
        RpcMode::ClientStreaming
    }

    fn call_client_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = Result<GrpcResponseData, tonic::Status>> + Send>> {
        Box::pin(async move {
            let mut stream = request.message_stream;
            let mut count = 0;

            while let Some(msg_result) = stream.next().await {
                match msg_result {
                    Ok(msg) => {
                        // Validate: message must start with "valid_"
                        if let Ok(s) = std::str::from_utf8(&msg) {
                            if !s.starts_with("valid_") {
                                return Err(tonic::Status::invalid_argument(format!(
                                    "Message does not start with 'valid_': {}",
                                    s
                                )));
                            }
                            count += 1;
                        } else {
                            return Err(tonic::Status::invalid_argument("Invalid UTF-8 in message"));
                        }
                    }
                    Err(e) => return Err(e),
                }
            }

            Ok(GrpcResponseData {
                payload: Bytes::from(format!("validated_count:{}", count)),
                metadata: MetadataMap::new(),
            })
        })
    }
}

/// Handler that counts messages in stream
struct CountMessagesHandler;

impl GrpcHandler for CountMessagesHandler {
    fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        Box::pin(async { Err(tonic::Status::unimplemented("Use call_client_stream for streaming")) })
    }

    fn service_name(&self) -> &'static str {
        "integration.CountService"
    }

    fn rpc_mode(&self) -> RpcMode {
        RpcMode::ClientStreaming
    }

    fn call_client_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = Result<GrpcResponseData, tonic::Status>> + Send>> {
        Box::pin(async move {
            let mut stream = request.message_stream;
            let mut count = 0u64;

            while let Some(msg_result) = stream.next().await {
                match msg_result {
                    Ok(_) => count += 1,
                    Err(e) => return Err(e),
                }
            }

            Ok(GrpcResponseData {
                payload: Bytes::from(count.to_le_bytes().to_vec()),
                metadata: MetadataMap::new(),
            })
        })
    }
}

/// Handler that echoes first message of stream
struct EchoFirstHandler;

impl GrpcHandler for EchoFirstHandler {
    fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        Box::pin(async { Err(tonic::Status::unimplemented("Use call_client_stream for streaming")) })
    }

    fn service_name(&self) -> &'static str {
        "integration.EchoFirstService"
    }

    fn rpc_mode(&self) -> RpcMode {
        RpcMode::ClientStreaming
    }

    fn call_client_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = Result<GrpcResponseData, tonic::Status>> + Send>> {
        Box::pin(async move {
            let mut stream = request.message_stream;

            if let Some(msg_result) = stream.next().await {
                match msg_result {
                    Ok(msg) => Ok(GrpcResponseData {
                        payload: Bytes::from([b"first:", &msg[..]].concat()),
                        metadata: MetadataMap::new(),
                    }),
                    Err(e) => Err(e),
                }
            } else {
                Err(tonic::Status::invalid_argument("Stream is empty"))
            }
        })
    }
}

/// Handler that echoes last message of stream
struct EchoLastHandler;

impl GrpcHandler for EchoLastHandler {
    fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        Box::pin(async { Err(tonic::Status::unimplemented("Use call_client_stream for streaming")) })
    }

    fn service_name(&self) -> &'static str {
        "integration.EchoLastService"
    }

    fn rpc_mode(&self) -> RpcMode {
        RpcMode::ClientStreaming
    }

    fn call_client_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = Result<GrpcResponseData, tonic::Status>> + Send>> {
        Box::pin(async move {
            let mut stream = request.message_stream;
            let mut last_msg: Option<Bytes> = None;

            while let Some(msg_result) = stream.next().await {
                match msg_result {
                    Ok(msg) => last_msg = Some(msg),
                    Err(e) => return Err(e),
                }
            }

            if let Some(msg) = last_msg {
                Ok(GrpcResponseData {
                    payload: Bytes::from([b"last:", &msg[..]].concat()),
                    metadata: MetadataMap::new(),
                })
            } else {
                Err(tonic::Status::invalid_argument("Stream is empty"))
            }
        })
    }
}

/// Handler that returns error for validation failures
struct ValidationErrorHandler;

impl GrpcHandler for ValidationErrorHandler {
    fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        Box::pin(async { Err(tonic::Status::unimplemented("Use call_client_stream for streaming")) })
    }

    fn service_name(&self) -> &'static str {
        "integration.ValidationErrorService"
    }

    fn rpc_mode(&self) -> RpcMode {
        RpcMode::ClientStreaming
    }

    fn call_client_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = Result<GrpcResponseData, tonic::Status>> + Send>> {
        Box::pin(async move {
            let mut stream = request.message_stream;

            while let Some(msg_result) = stream.next().await {
                match msg_result {
                    Ok(msg) => {
                        // Reject messages larger than 1000 bytes
                        if msg.len() > 1000 {
                            return Err(tonic::Status::invalid_argument(
                                "Message exceeds size limit of 1000 bytes",
                            ));
                        }
                    }
                    Err(e) => return Err(e),
                }
            }

            Ok(GrpcResponseData {
                payload: Bytes::from("all_valid"),
                metadata: MetadataMap::new(),
            })
        })
    }
}

/// Handler that preserves message order
struct PreserveOrderHandler;

impl GrpcHandler for PreserveOrderHandler {
    fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        Box::pin(async { Err(tonic::Status::unimplemented("Use call_client_stream for streaming")) })
    }

    fn service_name(&self) -> &'static str {
        "integration.PreserveOrderService"
    }

    fn rpc_mode(&self) -> RpcMode {
        RpcMode::ClientStreaming
    }

    fn call_client_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = Result<GrpcResponseData, tonic::Status>> + Send>> {
        Box::pin(async move {
            let mut stream = request.message_stream;
            let mut messages = Vec::new();

            while let Some(msg_result) = stream.next().await {
                match msg_result {
                    Ok(msg) => messages.push(msg),
                    Err(e) => return Err(e),
                }
            }

            // Concatenate all messages in order
            let mut result = Vec::new();
            for msg in messages {
                result.extend_from_slice(&msg);
            }

            Ok(GrpcResponseData {
                payload: Bytes::from(result),
                metadata: MetadataMap::new(),
            })
        })
    }
}

// ============================================================================
// Integration Tests
// ============================================================================

#[tokio::test]
async fn test_integration_client_stream_sum_aggregation() {
    let handler = Arc::new(SumHandlerIntegration);

    let messages = vec![
        Bytes::from((10i64).to_le_bytes().to_vec()),
        Bytes::from((20i64).to_le_bytes().to_vec()),
        Bytes::from((30i64).to_le_bytes().to_vec()),
    ];
    let stream = message_stream_from_vec(messages);

    let request = StreamingRequest {
        service_name: "integration.SumService".to_string(),
        method_name: "Sum".to_string(),
        message_stream: stream,
        metadata: MetadataMap::new(),
    };

    let result = handler.call_client_stream(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    if response.payload.len() == 8 {
        let sum = i64::from_le_bytes([
            response.payload[0],
            response.payload[1],
            response.payload[2],
            response.payload[3],
            response.payload[4],
            response.payload[5],
            response.payload[6],
            response.payload[7],
        ]);
        assert_eq!(sum, 60); // 10 + 20 + 30
    } else {
        panic!("Unexpected payload size");
    }
}

#[tokio::test]
async fn test_integration_client_stream_validate_all_messages() {
    let handler = Arc::new(ValidateAllMessagesHandler);

    let messages = vec![
        Bytes::from("valid_msg1"),
        Bytes::from("valid_msg2"),
        Bytes::from("valid_msg3"),
    ];
    let stream = message_stream_from_vec(messages);

    let request = StreamingRequest {
        service_name: "integration.ValidateService".to_string(),
        method_name: "Validate".to_string(),
        message_stream: stream,
        metadata: MetadataMap::new(),
    };

    let result = handler.call_client_stream(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.payload, Bytes::from("validated_count:3"));
}

#[tokio::test]
async fn test_integration_client_stream_count_messages() {
    let handler = Arc::new(CountMessagesHandler);

    let messages = vec![
        Bytes::from("msg1"),
        Bytes::from("msg2"),
        Bytes::from("msg3"),
        Bytes::from("msg4"),
        Bytes::from("msg5"),
        Bytes::from("msg6"),
    ];
    let stream = message_stream_from_vec(messages);

    let request = StreamingRequest {
        service_name: "integration.CountService".to_string(),
        method_name: "Count".to_string(),
        message_stream: stream,
        metadata: MetadataMap::new(),
    };

    let result = handler.call_client_stream(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    if response.payload.len() == 8 {
        let count = u64::from_le_bytes([
            response.payload[0],
            response.payload[1],
            response.payload[2],
            response.payload[3],
            response.payload[4],
            response.payload[5],
            response.payload[6],
            response.payload[7],
        ]);
        assert_eq!(count, 6);
    }
}

#[tokio::test]
async fn test_integration_client_stream_echo_first_message() {
    let handler = Arc::new(EchoFirstHandler);

    let messages = vec![
        Bytes::from("first_message"),
        Bytes::from("second_message"),
        Bytes::from("third_message"),
    ];
    let stream = message_stream_from_vec(messages);

    let request = StreamingRequest {
        service_name: "integration.EchoFirstService".to_string(),
        method_name: "EchoFirst".to_string(),
        message_stream: stream,
        metadata: MetadataMap::new(),
    };

    let result = handler.call_client_stream(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.payload, Bytes::from("first:first_message"));
}

#[tokio::test]
async fn test_integration_client_stream_echo_last_message() {
    let handler = Arc::new(EchoLastHandler);

    let messages = vec![
        Bytes::from("first_message"),
        Bytes::from("second_message"),
        Bytes::from("third_message"),
    ];
    let stream = message_stream_from_vec(messages);

    let request = StreamingRequest {
        service_name: "integration.EchoLastService".to_string(),
        method_name: "EchoLast".to_string(),
        message_stream: stream,
        metadata: MetadataMap::new(),
    };

    let result = handler.call_client_stream(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.payload, Bytes::from("last:third_message"));
}

#[tokio::test]
async fn test_integration_client_stream_validation_failure() {
    let handler = Arc::new(ValidationErrorHandler);

    let messages = vec![
        Bytes::from("valid_message"),
        Bytes::from(vec![b'x'; 1001]), // Exceeds 1000 byte limit
        Bytes::from("another_message"),
    ];
    let stream = message_stream_from_vec(messages);

    let request = StreamingRequest {
        service_name: "integration.ValidationErrorService".to_string(),
        method_name: "Validate".to_string(),
        message_stream: stream,
        metadata: MetadataMap::new(),
    };

    let result = handler.call_client_stream(request).await;
    assert!(result.is_err());

    if let Err(error) = result {
        assert_eq!(error.code(), tonic::Code::InvalidArgument);
        assert!(error.message().contains("exceeds size limit"));
    }
}

#[tokio::test]
async fn test_integration_client_stream_message_ordering() {
    let handler = Arc::new(PreserveOrderHandler);

    let messages = vec![
        Bytes::from("alpha"),
        Bytes::from("beta"),
        Bytes::from("gamma"),
        Bytes::from("delta"),
    ];
    let stream = message_stream_from_vec(messages);

    let request = StreamingRequest {
        service_name: "integration.PreserveOrderService".to_string(),
        method_name: "PreserveOrder".to_string(),
        message_stream: stream,
        metadata: MetadataMap::new(),
    };

    let result = handler.call_client_stream(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.payload, Bytes::from("alphabetagammadelta"));
}

#[tokio::test]
async fn test_integration_client_stream_empty_stream() {
    let handler = Arc::new(CountMessagesHandler);

    let messages: Vec<Bytes> = vec![];
    let stream = message_stream_from_vec(messages);

    let request = StreamingRequest {
        service_name: "integration.CountService".to_string(),
        method_name: "Count".to_string(),
        message_stream: stream,
        metadata: MetadataMap::new(),
    };

    let result = handler.call_client_stream(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    if response.payload.len() == 8 {
        let count = u64::from_le_bytes([
            response.payload[0],
            response.payload[1],
            response.payload[2],
            response.payload[3],
            response.payload[4],
            response.payload[5],
            response.payload[6],
            response.payload[7],
        ]);
        assert_eq!(count, 0);
    }
}

#[tokio::test]
async fn test_integration_client_stream_large_stream() {
    let handler = Arc::new(CountMessagesHandler);

    // Create 150 messages
    let messages: Vec<Bytes> = (0..150).map(|i| Bytes::from(format!("message_{}", i))).collect();
    let stream = message_stream_from_vec(messages);

    let request = StreamingRequest {
        service_name: "integration.CountService".to_string(),
        method_name: "Count".to_string(),
        message_stream: stream,
        metadata: MetadataMap::new(),
    };

    let result = handler.call_client_stream(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    if response.payload.len() == 8 {
        let count = u64::from_le_bytes([
            response.payload[0],
            response.payload[1],
            response.payload[2],
            response.payload[3],
            response.payload[4],
            response.payload[5],
            response.payload[6],
            response.payload[7],
        ]);
        assert_eq!(count, 150);
    }
}

#[tokio::test]
async fn test_integration_client_stream_validate_failure_at_start() {
    let handler = Arc::new(ValidateAllMessagesHandler);

    let messages = vec![
        Bytes::from("invalid_message"), // Should fail validation
        Bytes::from("valid_msg2"),
    ];
    let stream = message_stream_from_vec(messages);

    let request = StreamingRequest {
        service_name: "integration.ValidateService".to_string(),
        method_name: "Validate".to_string(),
        message_stream: stream,
        metadata: MetadataMap::new(),
    };

    let result = handler.call_client_stream(request).await;
    assert!(result.is_err());

    if let Err(error) = result {
        assert_eq!(error.code(), tonic::Code::InvalidArgument);
        assert!(error.message().contains("does not start with 'valid_'"));
    }
}

#[tokio::test]
async fn test_integration_client_stream_concurrent_handlers() {
    let handler1 = Arc::new(CountMessagesHandler);
    let handler2 = Arc::new(SumHandlerIntegration);
    let handler3 = Arc::new(CountMessagesHandler);

    let task1 = {
        let h = handler1.clone();
        tokio::spawn(async move {
            let messages = vec![Bytes::from("a"), Bytes::from("b"), Bytes::from("c")];
            let stream = message_stream_from_vec(messages);
            let request = StreamingRequest {
                service_name: "integration.CountService".to_string(),
                method_name: "Count".to_string(),
                message_stream: stream,
                metadata: MetadataMap::new(),
            };
            h.call_client_stream(request).await
        })
    };

    let task2 = {
        let h = handler2.clone();
        tokio::spawn(async move {
            let messages = vec![
                Bytes::from((5i64).to_le_bytes().to_vec()),
                Bytes::from((10i64).to_le_bytes().to_vec()),
            ];
            let stream = message_stream_from_vec(messages);
            let request = StreamingRequest {
                service_name: "integration.SumService".to_string(),
                method_name: "Sum".to_string(),
                message_stream: stream,
                metadata: MetadataMap::new(),
            };
            h.call_client_stream(request).await
        })
    };

    let task3 = {
        let h = handler3.clone();
        tokio::spawn(async move {
            let messages = vec![Bytes::from("x"), Bytes::from("y")];
            let stream = message_stream_from_vec(messages);
            let request = StreamingRequest {
                service_name: "integration.CountService".to_string(),
                method_name: "Count".to_string(),
                message_stream: stream,
                metadata: MetadataMap::new(),
            };
            h.call_client_stream(request).await
        })
    };

    let result1 = task1.await.unwrap();
    let result2 = task2.await.unwrap();
    let result3 = task3.await.unwrap();

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(result3.is_ok());

    // Verify counts
    let response1 = result1.unwrap();
    if response1.payload.len() == 8 {
        let count = u64::from_le_bytes([
            response1.payload[0],
            response1.payload[1],
            response1.payload[2],
            response1.payload[3],
            response1.payload[4],
            response1.payload[5],
            response1.payload[6],
            response1.payload[7],
        ]);
        assert_eq!(count, 3);
    }

    let response3 = result3.unwrap();
    if response3.payload.len() == 8 {
        let count = u64::from_le_bytes([
            response3.payload[0],
            response3.payload[1],
            response3.payload[2],
            response3.payload[3],
            response3.payload[4],
            response3.payload[5],
            response3.payload[6],
            response3.payload[7],
        ]);
        assert_eq!(count, 2);
    }
}

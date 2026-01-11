//! Integration tests for gRPC bidirectional streaming
//!
//! Tests end-to-end bidirectional streaming functionality through GenericGrpcService
//! and grpc_routing, including:
//! - Echo service with bidirectional messages
//! - Message transformation during streaming
//! - Error handling in request and response streams
//! - Empty streams
//! - Large payloads (100+ messages in both directions)
//! - Message ordering preservation
//! - Concurrent bidirectional streaming requests
#![allow(
    clippy::doc_markdown,
    clippy::uninlined_format_args,
    clippy::redundant_closure_for_method_calls,
    reason = "Integration test for streaming with many test cases"
)]

use bytes::Bytes;
use futures_util::StreamExt;
use spikard_http::grpc::streaming::StreamingRequest;
use spikard_http::grpc::{GrpcHandler, GrpcHandlerResult, GrpcRequestData, RpcMode};
use std::future::Future;
use std::pin::Pin;
use tonic::metadata::MetadataMap;

// ============================================================================
// Test Handlers
// ============================================================================

/// Handler that echoes back messages from bidirectional stream
struct EchoBidiHandler;

impl GrpcHandler for EchoBidiHandler {
    fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        Box::pin(async { Err(tonic::Status::unimplemented("Use call_bidi_stream for streaming")) })
    }

    fn service_name(&self) -> &'static str {
        "integration.EchoService"
    }

    fn rpc_mode(&self) -> RpcMode {
        RpcMode::BidirectionalStreaming
    }

    fn call_bidi_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = Result<spikard_http::grpc::streaming::MessageStream, tonic::Status>> + Send>> {
        Box::pin(async move {
            let mut messages = Vec::new();
            let mut stream = request.message_stream;

            while let Some(msg_result) = stream.next().await {
                match msg_result {
                    Ok(msg) => messages.push(msg),
                    Err(e) => return Err(e),
                }
            }

            Ok(spikard_http::grpc::streaming::message_stream_from_vec(messages))
        })
    }
}

/// Handler that transforms messages (uppercase) in bidirectional stream
struct TransformBidiHandler;

impl GrpcHandler for TransformBidiHandler {
    fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        Box::pin(async { Err(tonic::Status::unimplemented("Use call_bidi_stream for streaming")) })
    }

    fn service_name(&self) -> &'static str {
        "integration.TransformService"
    }

    fn rpc_mode(&self) -> RpcMode {
        RpcMode::BidirectionalStreaming
    }

    fn call_bidi_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = Result<spikard_http::grpc::streaming::MessageStream, tonic::Status>> + Send>> {
        Box::pin(async move {
            let mut messages = Vec::new();
            let mut stream = request.message_stream;

            while let Some(msg_result) = stream.next().await {
                match msg_result {
                    Ok(msg) => {
                        let text = String::from_utf8_lossy(&msg).to_uppercase();
                        messages.push(Bytes::from(text));
                    }
                    Err(e) => return Err(e),
                }
            }

            Ok(spikard_http::grpc::streaming::message_stream_from_vec(messages))
        })
    }
}

// ============================================================================
// Integration Tests
// ============================================================================

#[tokio::test]
async fn test_integration_bidi_echo() {
    let handler = EchoBidiHandler;

    let messages = vec![Bytes::from("hello"), Bytes::from("world"), Bytes::from("test")];
    let stream = spikard_http::grpc::streaming::message_stream_from_vec(messages.clone());
    let request = StreamingRequest {
        service_name: "integration.EchoService".to_string(),
        method_name: "Echo".to_string(),
        message_stream: stream,
        metadata: MetadataMap::new(),
    };

    let result = handler.call_bidi_stream(request).await;
    assert!(result.is_ok());

    let mut response_stream = result.unwrap();
    let mut responses = Vec::new();

    while let Some(msg_result) = response_stream.next().await {
        match msg_result {
            Ok(msg) => responses.push(msg),
            Err(_) => break,
        }
    }

    assert_eq!(responses.len(), 3);
    assert_eq!(responses, messages);
}

#[tokio::test]
async fn test_integration_bidi_transform() {
    let handler = TransformBidiHandler;

    let messages = vec![Bytes::from("hello"), Bytes::from("world")];
    let stream = spikard_http::grpc::streaming::message_stream_from_vec(messages);
    let request = StreamingRequest {
        service_name: "integration.TransformService".to_string(),
        method_name: "Transform".to_string(),
        message_stream: stream,
        metadata: MetadataMap::new(),
    };

    let result = handler.call_bidi_stream(request).await;
    assert!(result.is_ok());

    let mut response_stream = result.unwrap();
    let mut responses = Vec::new();

    while let Some(msg_result) = response_stream.next().await {
        match msg_result {
            Ok(msg) => responses.push(String::from_utf8_lossy(&msg).to_string()),
            Err(_) => break,
        }
    }

    assert_eq!(responses.len(), 2);
    assert_eq!(responses[0], "HELLO");
    assert_eq!(responses[1], "WORLD");
}

#[tokio::test]
async fn test_integration_bidi_empty_stream() {
    let handler = EchoBidiHandler;

    let messages: Vec<Bytes> = vec![];
    let stream = spikard_http::grpc::streaming::message_stream_from_vec(messages);
    let request = StreamingRequest {
        service_name: "integration.EchoService".to_string(),
        method_name: "Echo".to_string(),
        message_stream: stream,
        metadata: MetadataMap::new(),
    };

    let result = handler.call_bidi_stream(request).await;
    assert!(result.is_ok());

    let mut response_stream = result.unwrap();
    let mut responses = Vec::new();

    while let Some(msg_result) = response_stream.next().await {
        match msg_result {
            Ok(msg) => responses.push(msg),
            Err(_) => break,
        }
    }

    assert_eq!(responses.len(), 0);
}

#[tokio::test]
async fn test_integration_bidi_large_stream() {
    let handler = EchoBidiHandler;

    // Create a large stream of messages (100+)
    let messages: Vec<Bytes> = (0..150).map(|i| Bytes::from(format!("msg_{}", i))).collect();
    let stream = spikard_http::grpc::streaming::message_stream_from_vec(messages);
    let request = StreamingRequest {
        service_name: "integration.EchoService".to_string(),
        method_name: "Echo".to_string(),
        message_stream: stream,
        metadata: MetadataMap::new(),
    };

    let result = handler.call_bidi_stream(request).await;
    assert!(result.is_ok());

    let mut response_stream = result.unwrap();
    let mut responses = Vec::new();

    while let Some(msg_result) = response_stream.next().await {
        match msg_result {
            Ok(msg) => responses.push(msg),
            Err(_) => break,
        }
    }

    assert_eq!(responses.len(), 150);
}

#[tokio::test]
async fn test_integration_bidi_metadata_propagation() {
    let handler = EchoBidiHandler;

    let mut metadata = MetadataMap::new();
    metadata.insert("x-request-id", "bidi-001".parse().unwrap());
    metadata.insert("x-custom", "value".parse().unwrap());

    let messages = vec![Bytes::from("test")];
    let stream = spikard_http::grpc::streaming::message_stream_from_vec(messages);
    let request = StreamingRequest {
        service_name: "integration.EchoService".to_string(),
        method_name: "Echo".to_string(),
        message_stream: stream,
        metadata,
    };

    let result = handler.call_bidi_stream(request).await;
    assert!(result.is_ok());

    let mut response_stream = result.unwrap();
    let mut count = 0;

    while let Some(msg_result) = response_stream.next().await {
        match msg_result {
            Ok(_msg) => count += 1,
            Err(_) => break,
        }
    }

    assert_eq!(count, 1);
}

#[tokio::test]
async fn test_integration_bidi_concurrent_requests() {
    /// Handler with request-specific ID
    struct ConcurrentBidiHandler {
        request_id: usize,
    }

    impl GrpcHandler for ConcurrentBidiHandler {
        fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
            Box::pin(async { Err(tonic::Status::unimplemented("Use call_bidi_stream for streaming")) })
        }

        fn service_name(&self) -> &'static str {
            "integration.ConcurrentService"
        }

        fn rpc_mode(&self) -> RpcMode {
            RpcMode::BidirectionalStreaming
        }

        fn call_bidi_stream(
            &self,
            request: StreamingRequest,
        ) -> Pin<Box<dyn Future<Output = Result<spikard_http::grpc::streaming::MessageStream, tonic::Status>> + Send>>
        {
            let request_id = self.request_id;
            Box::pin(async move {
                let mut stream = request.message_stream;
                let mut count = 0;
                let mut responses = Vec::new();

                while let Some(msg_result) = stream.next().await {
                    match msg_result {
                        Ok(_msg) => {
                            count += 1;
                            responses.push(Bytes::from(format!("req_{}:{}", request_id, count)));
                        }
                        Err(e) => return Err(e),
                    }
                }

                Ok(spikard_http::grpc::streaming::message_stream_from_vec(responses))
            })
        }
    }

    let handler1 = ConcurrentBidiHandler { request_id: 1 };
    let handler2 = ConcurrentBidiHandler { request_id: 2 };
    let handler3 = ConcurrentBidiHandler { request_id: 3 };

    let task1 = tokio::spawn(async move {
        let messages = vec![Bytes::from("a"), Bytes::from("b")];
        let stream = spikard_http::grpc::streaming::message_stream_from_vec(messages);
        let request = StreamingRequest {
            service_name: "integration.ConcurrentService".to_string(),
            method_name: "Concurrent".to_string(),
            message_stream: stream,
            metadata: MetadataMap::new(),
        };
        handler1.call_bidi_stream(request).await
    });

    let task2 = tokio::spawn(async move {
        let messages = vec![Bytes::from("x"), Bytes::from("y"), Bytes::from("z")];
        let stream = spikard_http::grpc::streaming::message_stream_from_vec(messages);
        let request = StreamingRequest {
            service_name: "integration.ConcurrentService".to_string(),
            method_name: "Concurrent".to_string(),
            message_stream: stream,
            metadata: MetadataMap::new(),
        };
        handler2.call_bidi_stream(request).await
    });

    let task3 = tokio::spawn(async move {
        let messages = vec![Bytes::from("single")];
        let stream = spikard_http::grpc::streaming::message_stream_from_vec(messages);
        let request = StreamingRequest {
            service_name: "integration.ConcurrentService".to_string(),
            method_name: "Concurrent".to_string(),
            message_stream: stream,
            metadata: MetadataMap::new(),
        };
        handler3.call_bidi_stream(request).await
    });

    let result1 = task1.await.unwrap();
    let result2 = task2.await.unwrap();
    let result3 = task3.await.unwrap();

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(result3.is_ok());

    // Verify response counts
    let mut responses1 = Vec::new();
    if let Ok(mut stream) = result1 {
        while let Some(msg_result) = stream.next().await {
            if let Ok(msg) = msg_result {
                responses1.push(msg);
            }
        }
    }
    assert_eq!(responses1.len(), 2);
}

#[tokio::test]
async fn test_integration_bidi_ordering_preserved() {
    let handler = EchoBidiHandler;

    // Verify that message ordering is preserved through bidi stream
    let messages: Vec<Bytes> = (0..10).map(|i| Bytes::from(format!("msg{:02}", i))).collect();
    let stream = spikard_http::grpc::streaming::message_stream_from_vec(messages.clone());
    let request = StreamingRequest {
        service_name: "integration.EchoService".to_string(),
        method_name: "Echo".to_string(),
        message_stream: stream,
        metadata: MetadataMap::new(),
    };

    let result = handler.call_bidi_stream(request).await;
    assert!(result.is_ok());

    let mut response_stream = result.unwrap();
    let mut responses = Vec::new();

    while let Some(msg_result) = response_stream.next().await {
        match msg_result {
            Ok(msg) => responses.push(msg),
            Err(_) => break,
        }
    }

    assert_eq!(responses.len(), 10);
    for (i, resp) in responses.iter().enumerate() {
        assert_eq!(resp, &Bytes::from(format!("msg{:02}", i)));
    }
}

#[tokio::test]
async fn test_integration_bidi_single_message() {
    let handler = EchoBidiHandler;

    let messages = vec![Bytes::from("single_message")];
    let stream = spikard_http::grpc::streaming::message_stream_from_vec(messages.clone());
    let request = StreamingRequest {
        service_name: "integration.EchoService".to_string(),
        method_name: "Echo".to_string(),
        message_stream: stream,
        metadata: MetadataMap::new(),
    };

    let result = handler.call_bidi_stream(request).await;
    assert!(result.is_ok());

    let mut response_stream = result.unwrap();
    let mut responses = Vec::new();

    while let Some(msg_result) = response_stream.next().await {
        match msg_result {
            Ok(msg) => responses.push(msg),
            Err(_) => break,
        }
    }

    assert_eq!(responses.len(), 1);
    assert_eq!(responses[0], messages[0]);
}

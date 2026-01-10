//! Core GrpcHandler trait for language-agnostic gRPC request handling
//!
//! This module defines the handler trait that language bindings implement
//! to handle gRPC requests. Similar to the HttpHandler pattern but designed
//! specifically for gRPC's protobuf-based message format.

use bytes::Bytes;
use std::future::Future;
use std::pin::Pin;
use tonic::metadata::MetadataMap;

use super::streaming::MessageStream;

/// RPC mode enum for declaring handler capabilities
///
/// Indicates which type of RPC this handler supports. This is used at
/// handler registration to route requests to the appropriate handler method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RpcMode {
    /// Unary RPC: single request, single response
    Unary,
    /// Server streaming RPC: single request, stream of responses
    ServerStreaming,
    /// Client streaming RPC: stream of requests, single response
    ClientStreaming,
    /// Bidirectional streaming RPC: stream of requests, stream of responses
    BidirectionalStreaming,
}

/// gRPC request data passed to handlers
///
/// Contains the parsed components of a gRPC request:
/// - Service and method names from the request path
/// - Serialized protobuf payload as bytes
/// - Request metadata (headers)
#[derive(Debug, Clone)]
pub struct GrpcRequestData {
    /// Fully qualified service name (e.g., "mypackage.MyService")
    pub service_name: String,
    /// Method name (e.g., "GetUser")
    pub method_name: String,
    /// Serialized protobuf message bytes
    pub payload: Bytes,
    /// gRPC metadata (similar to HTTP headers)
    pub metadata: MetadataMap,
}

/// gRPC response data returned by handlers
///
/// Contains the serialized protobuf response and any metadata to include
/// in the response headers.
#[derive(Debug, Clone)]
pub struct GrpcResponseData {
    /// Serialized protobuf message bytes
    pub payload: Bytes,
    /// gRPC metadata to include in response (similar to HTTP headers)
    pub metadata: MetadataMap,
}

/// Result type for gRPC handlers
///
/// Returns either:
/// - Ok(GrpcResponseData): A successful response with payload and metadata
/// - Err(tonic::Status): A gRPC error status with code and message
pub type GrpcHandlerResult = Result<GrpcResponseData, tonic::Status>;

/// Handler trait for gRPC requests
///
/// This is the language-agnostic interface that all gRPC handler implementations
/// must satisfy. Language bindings (Python, TypeScript, Ruby, PHP) will implement
/// this trait to bridge their runtime to Spikard's gRPC server.
///
/// Handlers declare their RPC mode (unary vs streaming) via the `rpc_mode()` method.
/// The gRPC server uses this to route requests to either `call()` or `call_server_stream()`.
///
/// # Examples
///
/// ## Basic unary handler
///
/// ```ignore
/// use spikard_http::grpc::{GrpcHandler, RpcMode, GrpcRequestData, GrpcResponseData, GrpcHandlerResult};
/// use bytes::Bytes;
/// use std::pin::Pin;
/// use std::future::Future;
///
/// struct UnaryHandler;
///
/// impl GrpcHandler for UnaryHandler {
///     fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
///         Box::pin(async move {
///             // Parse request.payload using protobuf deserialization
///             let user_id = extract_id_from_payload(&request.payload);
///
///             // Process business logic
///             let response_data = lookup_user(user_id).await?;
///
///             // Serialize response and return
///             Ok(GrpcResponseData {
///                 payload: serialize_user(&response_data),
///                 metadata: tonic::metadata::MetadataMap::new(),
///             })
///         })
///     }
///
///     fn service_name(&self) -> &str {
///         "users.UserService"
///     }
///
///     // Default rpc_mode() returns RpcMode::Unary
/// }
/// ```
///
/// ## Server streaming handler
///
/// ```ignore
/// use spikard_http::grpc::{GrpcHandler, RpcMode, GrpcRequestData, MessageStream};
/// use crate::grpc::streaming::message_stream_from_vec;
/// use bytes::Bytes;
/// use std::pin::Pin;
/// use std::future::Future;
///
/// struct StreamingHandler;
///
/// impl GrpcHandler for StreamingHandler {
///     fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = Result<GrpcResponseData, tonic::Status>> + Send>> {
///         // Unary call not used for streaming handlers, but must be implemented
///         Box::pin(async {
///             Err(tonic::Status::unimplemented("Use server streaming instead"))
///         })
///     }
///
///     fn service_name(&self) -> &str {
///         "events.EventService"
///     }
///
///     fn rpc_mode(&self) -> RpcMode {
///         RpcMode::ServerStreaming
///     }
///
///     fn call_server_stream(
///         &self,
///         request: GrpcRequestData,
///     ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
///         Box::pin(async move {
///             // Parse request to extract stream criteria (e.g., user_id)
///             let user_id = extract_id_from_payload(&request.payload);
///
///             // Generate messages (e.g., fetch events from database)
///             let events = fetch_user_events(user_id).await?;
///             let mut messages = Vec::new();
///
///             for event in events {
///                 let serialized = serialize_event(&event);
///                 messages.push(serialized);
///             }
///
///             // Convert to stream and return
///             Ok(message_stream_from_vec(messages))
///         })
///     }
/// }
/// ```
///
/// # Dispatch Behavior
///
/// The gRPC server uses `rpc_mode()` to determine which handler method to call:
///
/// | RpcMode | Handler Method | Use Case |
/// |---------|---|---|
/// | `Unary` | `call()` | Single request, single response |
/// | `ServerStreaming` | `call_server_stream()` | Single request, multiple responses |
/// | `ClientStreaming` | Not yet implemented | Multiple requests, single response |
/// | `BidirectionalStreaming` | Not yet implemented | Multiple requests, multiple responses |
///
/// # Error Handling
///
/// Both `call()` and `call_server_stream()` return gRPC error status values:
///
/// ```ignore
/// // Return a specific gRPC error
/// fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
///     Box::pin(async {
///         let Some(id) = parse_id(&request.payload) else {
///             return Err(tonic::Status::invalid_argument("Missing user ID"));
///         };
///
///         // ... process ...
///     })
/// }
/// ```
pub trait GrpcHandler: Send + Sync {
    /// Handle a gRPC request
    ///
    /// Takes the parsed request data and returns a future that resolves to either:
    /// - Ok(GrpcResponseData): A successful response
    /// - Err(tonic::Status): An error with appropriate gRPC status code
    ///
    /// # Arguments
    ///
    /// * `request` - The parsed gRPC request containing service/method names,
    ///   serialized payload, and metadata
    ///
    /// # Returns
    ///
    /// A future that resolves to a GrpcHandlerResult
    fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>>;

    /// Get the fully qualified service name this handler serves
    ///
    /// This is used for routing requests to the appropriate handler.
    /// Should return the fully qualified service name as defined in the .proto file.
    ///
    /// # Example
    ///
    /// For a service defined as:
    /// ```proto
    /// package mypackage;
    /// service UserService { ... }
    /// ```
    ///
    /// This should return "mypackage.UserService"
    fn service_name(&self) -> &str;

    /// Get the RPC mode this handler supports
    ///
    /// Returns the type of RPC this handler implements. Used at handler registration
    /// to route requests to the appropriate handler method.
    ///
    /// Default implementation returns `RpcMode::Unary` for backward compatibility.
    fn rpc_mode(&self) -> RpcMode {
        RpcMode::Unary
    }

    /// Handle a server streaming RPC request
    ///
    /// Takes a single request and returns a stream of response messages.
    /// Default implementation returns `UNIMPLEMENTED` status.
    ///
    /// # Arguments
    ///
    /// * `request` - The parsed gRPC request
    ///
    /// # Returns
    ///
    /// A future that resolves to either a stream of messages or an error status
    fn call_server_stream(
        &self,
        _request: GrpcRequestData,
    ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
        Box::pin(async { Err(tonic::Status::unimplemented("Server streaming not supported")) })
    }

    /// Handle a client streaming RPC call
    ///
    /// Takes a stream of request messages and returns a single response message.
    /// Default implementation returns `UNIMPLEMENTED` status.
    fn call_client_stream(
        &self,
        _request: crate::grpc::streaming::StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = Result<GrpcResponseData, tonic::Status>> + Send>> {
        Box::pin(async { Err(tonic::Status::unimplemented("Client streaming not supported")) })
    }

    /// Handle a bidirectional streaming RPC call
    ///
    /// Takes a stream of request messages and returns a stream of response messages.
    /// Default implementation returns `UNIMPLEMENTED` status.
    fn call_bidi_stream(
        &self,
        _request: crate::grpc::streaming::StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = Result<crate::grpc::streaming::MessageStream, tonic::Status>> + Send>> {
        Box::pin(async { Err(tonic::Status::unimplemented("Bidirectional streaming not supported")) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestGrpcHandler;

    impl GrpcHandler for TestGrpcHandler {
        fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
            Box::pin(async {
                Ok(GrpcResponseData {
                    payload: Bytes::from("test response"),
                    metadata: MetadataMap::new(),
                })
            })
        }

        fn service_name(&self) -> &str {
            "test.TestService"
        }
    }

    #[tokio::test]
    async fn test_grpc_handler_basic_call() {
        let handler = TestGrpcHandler;
        let request = GrpcRequestData {
            service_name: "test.TestService".to_string(),
            method_name: "TestMethod".to_string(),
            payload: Bytes::from("test payload"),
            metadata: MetadataMap::new(),
        };

        let result = handler.call(request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.payload, Bytes::from("test response"));
    }

    #[test]
    fn test_grpc_handler_service_name() {
        let handler = TestGrpcHandler;
        assert_eq!(handler.service_name(), "test.TestService");
    }

    #[test]
    fn test_grpc_handler_default_rpc_mode() {
        let handler = TestGrpcHandler;
        assert_eq!(handler.rpc_mode(), RpcMode::Unary);
    }

    #[tokio::test]
    async fn test_grpc_handler_default_server_stream_unimplemented() {
        let handler = TestGrpcHandler;
        let request = GrpcRequestData {
            service_name: "test.TestService".to_string(),
            method_name: "StreamMethod".to_string(),
            payload: Bytes::new(),
            metadata: MetadataMap::new(),
        };

        let result = handler.call_server_stream(request).await;
        assert!(result.is_err());

        match result {
            Err(error) => {
                assert_eq!(error.code(), tonic::Code::Unimplemented);
                assert_eq!(error.message(), "Server streaming not supported");
            }
            Ok(_) => panic!("Expected error, got Ok"),
        }
    }

    #[test]
    fn test_grpc_request_data_creation() {
        let request = GrpcRequestData {
            service_name: "mypackage.MyService".to_string(),
            method_name: "GetUser".to_string(),
            payload: Bytes::from("payload"),
            metadata: MetadataMap::new(),
        };

        assert_eq!(request.service_name, "mypackage.MyService");
        assert_eq!(request.method_name, "GetUser");
        assert_eq!(request.payload, Bytes::from("payload"));
    }

    #[test]
    fn test_grpc_response_data_creation() {
        let response = GrpcResponseData {
            payload: Bytes::from("response"),
            metadata: MetadataMap::new(),
        };

        assert_eq!(response.payload, Bytes::from("response"));
        assert!(response.metadata.is_empty());
    }

    #[test]
    fn test_grpc_request_data_clone() {
        let original = GrpcRequestData {
            service_name: "test.Service".to_string(),
            method_name: "Method".to_string(),
            payload: Bytes::from("data"),
            metadata: MetadataMap::new(),
        };

        let cloned = original.clone();
        assert_eq!(original.service_name, cloned.service_name);
        assert_eq!(original.method_name, cloned.method_name);
        assert_eq!(original.payload, cloned.payload);
    }

    #[test]
    fn test_grpc_response_data_clone() {
        let original = GrpcResponseData {
            payload: Bytes::from("response data"),
            metadata: MetadataMap::new(),
        };

        let cloned = original.clone();
        assert_eq!(original.payload, cloned.payload);
    }

    #[tokio::test]
    async fn test_grpc_handler_error_response() {
        struct ErrorHandler;

        impl GrpcHandler for ErrorHandler {
            fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async { Err(tonic::Status::not_found("Resource not found")) })
            }

            fn service_name(&self) -> &str {
                "test.ErrorService"
            }
        }

        let handler = ErrorHandler;
        let request = GrpcRequestData {
            service_name: "test.ErrorService".to_string(),
            method_name: "ErrorMethod".to_string(),
            payload: Bytes::new(),
            metadata: MetadataMap::new(),
        };

        let result = handler.call(request).await;
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code(), tonic::Code::NotFound);
        assert_eq!(error.message(), "Resource not found");
    }

    // ==================== Server Streaming Tests ====================

    #[tokio::test]
    async fn test_server_stream_with_multiple_messages() {
        use futures_util::StreamExt;

        struct ServerStreamHandler;

        impl GrpcHandler for ServerStreamHandler {
            fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async {
                    Ok(GrpcResponseData {
                        payload: Bytes::from("unary response"),
                        metadata: MetadataMap::new(),
                    })
                })
            }

            fn service_name(&self) -> &str {
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
                    let messages = vec![
                        Bytes::from("message1"),
                        Bytes::from("message2"),
                        Bytes::from("message3"),
                    ];
                    Ok(super::super::streaming::message_stream_from_vec(messages))
                })
            }
        }

        let handler = ServerStreamHandler;
        let request = GrpcRequestData {
            service_name: "test.StreamService".to_string(),
            method_name: "StreamMethod".to_string(),
            payload: Bytes::from("request data"),
            metadata: MetadataMap::new(),
        };

        let result = handler.call_server_stream(request).await;
        assert!(result.is_ok());

        let mut stream = result.unwrap();
        let msg1 = stream.next().await.unwrap().unwrap();
        assert_eq!(msg1, Bytes::from("message1"));

        let msg2 = stream.next().await.unwrap().unwrap();
        assert_eq!(msg2, Bytes::from("message2"));

        let msg3 = stream.next().await.unwrap().unwrap();
        assert_eq!(msg3, Bytes::from("message3"));

        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn test_server_stream_empty_stream() {
        use futures_util::StreamExt;

        struct EmptyStreamHandler;

        impl GrpcHandler for EmptyStreamHandler {
            fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async {
                    Ok(GrpcResponseData {
                        payload: Bytes::new(),
                        metadata: MetadataMap::new(),
                    })
                })
            }

            fn service_name(&self) -> &str {
                "test.EmptyStreamService"
            }

            fn rpc_mode(&self) -> RpcMode {
                RpcMode::ServerStreaming
            }

            fn call_server_stream(
                &self,
                _request: GrpcRequestData,
            ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
                Box::pin(async { Ok(super::super::streaming::empty_message_stream()) })
            }
        }

        let handler = EmptyStreamHandler;
        let request = GrpcRequestData {
            service_name: "test.EmptyStreamService".to_string(),
            method_name: "EmptyStream".to_string(),
            payload: Bytes::new(),
            metadata: MetadataMap::new(),
        };

        let result = handler.call_server_stream(request).await;
        assert!(result.is_ok());

        let mut stream = result.unwrap();
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn test_server_stream_with_error_mid_stream() {
        use futures_util::StreamExt;

        struct ErrorMidStreamHandler;

        impl GrpcHandler for ErrorMidStreamHandler {
            fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async {
                    Ok(GrpcResponseData {
                        payload: Bytes::new(),
                        metadata: MetadataMap::new(),
                    })
                })
            }

            fn service_name(&self) -> &str {
                "test.ErrorMidStreamService"
            }

            fn rpc_mode(&self) -> RpcMode {
                RpcMode::ServerStreaming
            }

            fn call_server_stream(
                &self,
                _request: GrpcRequestData,
            ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
                Box::pin(async {
                    let messages = vec![
                        Ok(Bytes::from("message1")),
                        Ok(Bytes::from("message2")),
                        Err(tonic::Status::internal("Stream error")),
                    ];
                    let stream: MessageStream = Box::pin(futures_util::stream::iter(messages));
                    Ok(stream)
                })
            }
        }

        let handler = ErrorMidStreamHandler;
        let request = GrpcRequestData {
            service_name: "test.ErrorMidStreamService".to_string(),
            method_name: "ErrorStream".to_string(),
            payload: Bytes::new(),
            metadata: MetadataMap::new(),
        };

        let result = handler.call_server_stream(request).await;
        assert!(result.is_ok());

        let mut stream = result.unwrap();

        let msg1 = stream.next().await.unwrap().unwrap();
        assert_eq!(msg1, Bytes::from("message1"));

        let msg2 = stream.next().await.unwrap().unwrap();
        assert_eq!(msg2, Bytes::from("message2"));

        let error_result = stream.next().await.unwrap();
        assert!(error_result.is_err());
        let error = error_result.unwrap_err();
        assert_eq!(error.code(), tonic::Code::Internal);
        assert_eq!(error.message(), "Stream error");

        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn test_server_stream_returns_error() {
        struct FailingStreamHandler;

        impl GrpcHandler for FailingStreamHandler {
            fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async {
                    Ok(GrpcResponseData {
                        payload: Bytes::new(),
                        metadata: MetadataMap::new(),
                    })
                })
            }

            fn service_name(&self) -> &str {
                "test.FailingStreamService"
            }

            fn rpc_mode(&self) -> RpcMode {
                RpcMode::ServerStreaming
            }

            fn call_server_stream(
                &self,
                _request: GrpcRequestData,
            ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
                Box::pin(async { Err(tonic::Status::unavailable("Stream unavailable")) })
            }
        }

        let handler = FailingStreamHandler;
        let request = GrpcRequestData {
            service_name: "test.FailingStreamService".to_string(),
            method_name: "FailingStream".to_string(),
            payload: Bytes::new(),
            metadata: MetadataMap::new(),
        };

        let result = handler.call_server_stream(request).await;
        assert!(result.is_err());

        if let Err(error) = result {
            assert_eq!(error.code(), tonic::Code::Unavailable);
            assert_eq!(error.message(), "Stream unavailable");
        } else {
            panic!("Expected error");
        }
    }

    #[tokio::test]
    async fn test_server_stream_with_metadata() {
        use futures_util::StreamExt;

        struct MetadataStreamHandler;

        impl GrpcHandler for MetadataStreamHandler {
            fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async {
                    Ok(GrpcResponseData {
                        payload: Bytes::new(),
                        metadata: MetadataMap::new(),
                    })
                })
            }

            fn service_name(&self) -> &str {
                "test.MetadataStreamService"
            }

            fn rpc_mode(&self) -> RpcMode {
                RpcMode::ServerStreaming
            }

            fn call_server_stream(
                &self,
                request: GrpcRequestData,
            ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
                Box::pin(async move {
                    // Verify metadata is received
                    assert!(!request.metadata.is_empty());
                    let messages = vec![Bytes::from("metadata_message")];
                    Ok(super::super::streaming::message_stream_from_vec(messages))
                })
            }
        }

        let handler = MetadataStreamHandler;
        let mut metadata = MetadataMap::new();
        metadata.insert(
            "x-request-id",
            "test-request-123"
                .parse::<tonic::metadata::MetadataValue<tonic::metadata::Ascii>>()
                .unwrap(),
        );

        let request = GrpcRequestData {
            service_name: "test.MetadataStreamService".to_string(),
            method_name: "MetadataStream".to_string(),
            payload: Bytes::new(),
            metadata,
        };

        let result = handler.call_server_stream(request).await;
        assert!(result.is_ok());

        let mut stream = result.unwrap();
        let msg = stream.next().await.unwrap().unwrap();
        assert_eq!(msg, Bytes::from("metadata_message"));
    }

    #[tokio::test]
    async fn test_server_stream_large_stream_100_messages() {
        use futures_util::StreamExt;

        struct LargeStreamHandler;

        impl GrpcHandler for LargeStreamHandler {
            fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async {
                    Ok(GrpcResponseData {
                        payload: Bytes::new(),
                        metadata: MetadataMap::new(),
                    })
                })
            }

            fn service_name(&self) -> &str {
                "test.LargeStreamService"
            }

            fn rpc_mode(&self) -> RpcMode {
                RpcMode::ServerStreaming
            }

            fn call_server_stream(
                &self,
                _request: GrpcRequestData,
            ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
                Box::pin(async {
                    let mut messages = Vec::new();
                    for i in 0..100 {
                        messages.push(Bytes::from(format!("message_{}", i)));
                    }
                    Ok(super::super::streaming::message_stream_from_vec(messages))
                })
            }
        }

        let handler = LargeStreamHandler;
        let request = GrpcRequestData {
            service_name: "test.LargeStreamService".to_string(),
            method_name: "LargeStream".to_string(),
            payload: Bytes::new(),
            metadata: MetadataMap::new(),
        };

        let result = handler.call_server_stream(request).await;
        assert!(result.is_ok());

        let mut stream = result.unwrap();
        for i in 0..100 {
            let msg = stream.next().await.unwrap().unwrap();
            assert_eq!(msg, Bytes::from(format!("message_{}", i)));
        }

        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn test_server_stream_large_stream_500_messages() {
        use futures_util::StreamExt;

        struct VeryLargeStreamHandler;

        impl GrpcHandler for VeryLargeStreamHandler {
            fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async {
                    Ok(GrpcResponseData {
                        payload: Bytes::new(),
                        metadata: MetadataMap::new(),
                    })
                })
            }

            fn service_name(&self) -> &str {
                "test.VeryLargeStreamService"
            }

            fn rpc_mode(&self) -> RpcMode {
                RpcMode::ServerStreaming
            }

            fn call_server_stream(
                &self,
                _request: GrpcRequestData,
            ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
                Box::pin(async {
                    let mut messages = Vec::new();
                    for i in 0..500 {
                        messages.push(Bytes::from(format!("msg_{}", i)));
                    }
                    Ok(super::super::streaming::message_stream_from_vec(messages))
                })
            }
        }

        let handler = VeryLargeStreamHandler;
        let request = GrpcRequestData {
            service_name: "test.VeryLargeStreamService".to_string(),
            method_name: "VeryLargeStream".to_string(),
            payload: Bytes::new(),
            metadata: MetadataMap::new(),
        };

        let result = handler.call_server_stream(request).await;
        assert!(result.is_ok());

        let mut stream = result.unwrap();
        let mut count = 0;
        while let Some(item) = stream.next().await {
            let msg = item.unwrap();
            assert_eq!(msg, Bytes::from(format!("msg_{}", count)));
            count += 1;
        }
        assert_eq!(count, 500);
    }

    #[test]
    fn test_rpc_mode_unary() {
        let handler = TestGrpcHandler;
        assert_eq!(handler.rpc_mode(), RpcMode::Unary);
    }

    #[test]
    fn test_rpc_mode_server_streaming() {
        struct ServerStreamTestHandler;

        impl GrpcHandler for ServerStreamTestHandler {
            fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async {
                    Ok(GrpcResponseData {
                        payload: Bytes::new(),
                        metadata: MetadataMap::new(),
                    })
                })
            }

            fn service_name(&self) -> &str {
                "test.ServerStreamTestService"
            }

            fn rpc_mode(&self) -> RpcMode {
                RpcMode::ServerStreaming
            }
        }

        let handler = ServerStreamTestHandler;
        assert_eq!(handler.rpc_mode(), RpcMode::ServerStreaming);
    }

    #[test]
    fn test_rpc_mode_client_streaming() {
        struct ClientStreamTestHandler;

        impl GrpcHandler for ClientStreamTestHandler {
            fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async {
                    Ok(GrpcResponseData {
                        payload: Bytes::new(),
                        metadata: MetadataMap::new(),
                    })
                })
            }

            fn service_name(&self) -> &str {
                "test.ClientStreamTestService"
            }

            fn rpc_mode(&self) -> RpcMode {
                RpcMode::ClientStreaming
            }
        }

        let handler = ClientStreamTestHandler;
        assert_eq!(handler.rpc_mode(), RpcMode::ClientStreaming);
    }

    #[test]
    fn test_rpc_mode_bidirectional_streaming() {
        struct BiDirectionalStreamTestHandler;

        impl GrpcHandler for BiDirectionalStreamTestHandler {
            fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async {
                    Ok(GrpcResponseData {
                        payload: Bytes::new(),
                        metadata: MetadataMap::new(),
                    })
                })
            }

            fn service_name(&self) -> &str {
                "test.BiDirectionalStreamTestService"
            }

            fn rpc_mode(&self) -> RpcMode {
                RpcMode::BidirectionalStreaming
            }
        }

        let handler = BiDirectionalStreamTestHandler;
        assert_eq!(handler.rpc_mode(), RpcMode::BidirectionalStreaming);
    }

    #[tokio::test]
    async fn test_server_stream_single_message() {
        use futures_util::StreamExt;

        struct SingleMessageStreamHandler;

        impl GrpcHandler for SingleMessageStreamHandler {
            fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async {
                    Ok(GrpcResponseData {
                        payload: Bytes::new(),
                        metadata: MetadataMap::new(),
                    })
                })
            }

            fn service_name(&self) -> &str {
                "test.SingleMessageStreamService"
            }

            fn rpc_mode(&self) -> RpcMode {
                RpcMode::ServerStreaming
            }

            fn call_server_stream(
                &self,
                _request: GrpcRequestData,
            ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
                Box::pin(async {
                    Ok(super::super::streaming::single_message_stream(Bytes::from(
                        "single_msg",
                    )))
                })
            }
        }

        let handler = SingleMessageStreamHandler;
        let request = GrpcRequestData {
            service_name: "test.SingleMessageStreamService".to_string(),
            method_name: "SingleMessage".to_string(),
            payload: Bytes::new(),
            metadata: MetadataMap::new(),
        };

        let result = handler.call_server_stream(request).await;
        assert!(result.is_ok());

        let mut stream = result.unwrap();
        let msg = stream.next().await.unwrap().unwrap();
        assert_eq!(msg, Bytes::from("single_msg"));
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn test_server_stream_preserves_request_data() {
        use futures_util::StreamExt;

        struct RequestPreservingStreamHandler;

        impl GrpcHandler for RequestPreservingStreamHandler {
            fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async {
                    Ok(GrpcResponseData {
                        payload: Bytes::new(),
                        metadata: MetadataMap::new(),
                    })
                })
            }

            fn service_name(&self) -> &str {
                "test.RequestPreservingService"
            }

            fn rpc_mode(&self) -> RpcMode {
                RpcMode::ServerStreaming
            }

            fn call_server_stream(
                &self,
                request: GrpcRequestData,
            ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
                Box::pin(async move {
                    // Verify request data is preserved
                    assert_eq!(request.service_name, "test.RequestPreservingService");
                    assert_eq!(request.method_name, "PreserveTest");
                    assert_eq!(request.payload, Bytes::from("test_payload"));

                    let messages = vec![Bytes::from("response")];
                    Ok(super::super::streaming::message_stream_from_vec(messages))
                })
            }
        }

        let handler = RequestPreservingStreamHandler;
        let request = GrpcRequestData {
            service_name: "test.RequestPreservingService".to_string(),
            method_name: "PreserveTest".to_string(),
            payload: Bytes::from("test_payload"),
            metadata: MetadataMap::new(),
        };

        let result = handler.call_server_stream(request).await;
        assert!(result.is_ok());

        let mut stream = result.unwrap();
        let msg = stream.next().await.unwrap().unwrap();
        assert_eq!(msg, Bytes::from("response"));
    }

    #[tokio::test]
    async fn test_server_stream_with_various_error_codes() {
        struct ErrorCodeStreamHandler {
            error_code: tonic::Code,
        }

        impl GrpcHandler for ErrorCodeStreamHandler {
            fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async {
                    Ok(GrpcResponseData {
                        payload: Bytes::new(),
                        metadata: MetadataMap::new(),
                    })
                })
            }

            fn service_name(&self) -> &str {
                "test.ErrorCodeService"
            }

            fn rpc_mode(&self) -> RpcMode {
                RpcMode::ServerStreaming
            }

            fn call_server_stream(
                &self,
                _request: GrpcRequestData,
            ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
                let code = self.error_code;
                Box::pin(async move {
                    match code {
                        tonic::Code::InvalidArgument => Err(tonic::Status::invalid_argument("Invalid argument")),
                        tonic::Code::FailedPrecondition => {
                            Err(tonic::Status::failed_precondition("Failed precondition"))
                        }
                        tonic::Code::PermissionDenied => Err(tonic::Status::permission_denied("Permission denied")),
                        _ => Err(tonic::Status::internal("Internal error")),
                    }
                })
            }
        }

        // Test InvalidArgument
        let handler = ErrorCodeStreamHandler {
            error_code: tonic::Code::InvalidArgument,
        };
        let request = GrpcRequestData {
            service_name: "test.ErrorCodeService".to_string(),
            method_name: "Error".to_string(),
            payload: Bytes::new(),
            metadata: MetadataMap::new(),
        };
        let result = handler.call_server_stream(request).await;
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.code(), tonic::Code::InvalidArgument);
        }

        // Test FailedPrecondition
        let handler = ErrorCodeStreamHandler {
            error_code: tonic::Code::FailedPrecondition,
        };
        let request = GrpcRequestData {
            service_name: "test.ErrorCodeService".to_string(),
            method_name: "Error".to_string(),
            payload: Bytes::new(),
            metadata: MetadataMap::new(),
        };
        let result = handler.call_server_stream(request).await;
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.code(), tonic::Code::FailedPrecondition);
        }

        // Test PermissionDenied
        let handler = ErrorCodeStreamHandler {
            error_code: tonic::Code::PermissionDenied,
        };
        let request = GrpcRequestData {
            service_name: "test.ErrorCodeService".to_string(),
            method_name: "Error".to_string(),
            payload: Bytes::new(),
            metadata: MetadataMap::new(),
        };
        let result = handler.call_server_stream(request).await;
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.code(), tonic::Code::PermissionDenied);
        }
    }
}

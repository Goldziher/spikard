//! Core GrpcHandler trait for language-agnostic gRPC request handling
//!
//! This module defines the handler trait that language bindings implement
//! to handle gRPC requests. Similar to the HttpHandler pattern but designed
//! specifically for gRPC's protobuf-based message format.

use bytes::Bytes;
use futures_util::StreamExt;
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
///             Ok(Box::pin(futures_util::stream::iter(messages.into_iter().map(Ok))))
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
/// | `ClientStreaming` | `call_client_stream()` | Multiple requests, single response |
/// | `BidirectionalStreaming` | `call_bidi_stream()` | Multiple requests, multiple responses |
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
    fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send + '_>>;

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
    /// Default implementation adapts the unary `call()` response into a
    /// single-message stream.
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
        request: GrpcRequestData,
    ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send + '_>> {
        let unary_future = self.call(request);
        Box::pin(async move {
            let response = unary_future.await?;
            Ok(crate::grpc::streaming::single_message_stream(response.payload))
        })
    }

    /// Handle a client streaming RPC call
    ///
    /// Takes a stream of request messages and returns a single response message.
    /// Default implementation adapts to unary by requiring exactly one
    /// request message in the stream.
    fn call_client_stream(
        &self,
        request: crate::grpc::streaming::StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = Result<GrpcResponseData, tonic::Status>> + Send + '_>> {
        Box::pin(async move {
            let crate::grpc::streaming::StreamingRequest {
                service_name,
                method_name,
                mut message_stream,
                metadata,
            } = request;

            let first_message = match message_stream.next().await {
                Some(Ok(message)) => message,
                Some(Err(status)) => return Err(status),
                None => {
                    return Err(tonic::Status::invalid_argument(
                        "Client stream is empty; unary fallback requires exactly one request message",
                    ));
                }
            };

            if let Some(next_message) = message_stream.next().await {
                match next_message {
                    Ok(_) => {
                        return Err(tonic::Status::invalid_argument(
                            "Unary fallback requires exactly one request message",
                        ));
                    }
                    Err(status) => return Err(status),
                }
            }

            self.call(GrpcRequestData {
                service_name,
                method_name,
                payload: first_message,
                metadata,
            })
            .await
        })
    }

    /// Handle a bidirectional streaming RPC call
    ///
    /// Takes a stream of request messages and returns a stream of response messages.
    /// Default implementation adapts to unary by requiring exactly one
    /// request message and returning a single-message response stream.
    fn call_bidi_stream(
        &self,
        request: crate::grpc::streaming::StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = Result<crate::grpc::streaming::MessageStream, tonic::Status>> + Send + '_>> {
        Box::pin(async move {
            let response = self.call_client_stream(request).await?;
            Ok(crate::grpc::streaming::single_message_stream(response.payload))
        })
    }
}


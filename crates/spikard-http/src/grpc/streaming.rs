//! Streaming support utilities for gRPC
//!
//! This module provides utilities for handling streaming RPCs:
//! - Client streaming (receiving stream of messages)
//! - Server streaming (sending stream of messages)
//! - Bidirectional streaming (both directions)

use bytes::Bytes;
use futures_util::Stream;
use std::pin::Pin;
use tonic::Status;

/// Type alias for a stream of protobuf message bytes
///
/// Used for both client streaming (incoming) and server streaming (outgoing).
/// Each item in the stream is either:
/// - Ok(Bytes): A serialized protobuf message
/// - Err(Status): A gRPC error
///
/// # Backpressure Considerations
///
/// Streaming responses should implement backpressure handling to avoid memory buildup with slow clients:
///
/// - **Problem**: If a client reads slowly but the handler produces messages quickly, messages will
///   queue in memory, potentially causing high memory usage or OOM errors.
/// - **Solution**: The gRPC layer (Tonic) handles backpressure automatically via the underlying TCP/HTTP/2
///   connection. However, handlers should be aware of this behavior.
/// - **Best Practice**: For long-running or high-volume streams, implement rate limiting or flow control
///   in the handler to avoid overwhelming the network buffer.
///
/// # Example: Rate-limited streaming
///
/// ```ignore
/// use spikard_http::grpc::streaming::MessageStream;
/// use bytes::Bytes;
/// use std::pin::Pin;
/// use std::time::Duration;
/// use tokio::time::sleep;
/// use futures_util::stream::{self, StreamExt};
///
/// // Handler that sends 1000 messages with rate limiting
/// fn create_rate_limited_stream() -> MessageStream {
///     let messages = (0..1000).map(|i| {
///         Ok(Bytes::from(format!("message_{}", i)))
///     });
///
///     // Stream with delay between messages to avoid overwhelming the client
///     let stream = stream::iter(messages)
///         .then(|msg| async {
///             sleep(Duration::from_millis(1)).await;  // 1ms between messages
///             msg
///         });
///
///     Box::pin(stream)
/// }
/// ```
///
/// # Memory Management
///
/// Keep the following in mind when implementing large streams:
///
/// - Messages are buffered in the gRPC transport layer's internal queue
/// - Slow clients will cause the queue to grow, increasing memory usage
/// - Very large individual messages may cause buffer allocation spikes
/// - Consider implementing stream chunking for very large responses (split one large message into many small ones)
pub type MessageStream = Pin<Box<dyn Stream<Item = Result<Bytes, Status>> + Send>>;

/// Request for client streaming RPC
///
/// Contains metadata and a stream of incoming messages from the client.
pub struct StreamingRequest {
    /// Service name
    pub service_name: String,
    /// Method name
    pub method_name: String,
    /// Stream of incoming protobuf messages
    pub message_stream: MessageStream,
    /// Request metadata
    pub metadata: tonic::metadata::MetadataMap,
}

/// Response for server streaming RPC
///
/// Contains metadata, a stream of outgoing messages, and optional trailers.
/// Trailers are metadata sent after the stream completes (after all messages).
pub struct StreamingResponse {
    /// Stream of outgoing protobuf messages
    pub message_stream: MessageStream,
    /// Response metadata (sent before messages)
    pub metadata: tonic::metadata::MetadataMap,
    /// Optional trailers (sent after stream completes)
    ///
    /// Trailers are useful for sending status information or metrics
    /// after all messages have been sent.
    pub trailers: Option<tonic::metadata::MetadataMap>,
}

/// Helper to create a single-message stream
///
/// Useful for converting unary responses to streaming responses.
///
/// # Example
///
/// ```ignore
/// use spikard_http::grpc::streaming::single_message_stream;
/// use bytes::Bytes;
///
/// let stream = single_message_stream(Bytes::from("response"));
/// ```
pub(crate) fn single_message_stream(message: Bytes) -> MessageStream {
    Box::pin(futures_util::stream::once(async move { Ok(message) }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::StreamExt;

    #[tokio::test]
    async fn test_single_message_stream() {
        let mut stream = single_message_stream(Bytes::from("single"));

        let msg = stream.next().await.unwrap().unwrap();
        assert_eq!(msg, Bytes::from("single"));

        assert!(stream.next().await.is_none());
    }
}

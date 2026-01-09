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

/// Helper to create a message stream from a vector of bytes
///
/// Useful for testing and for handlers that want to create a stream
/// from a fixed set of messages.
///
/// # Example
///
/// ```ignore
/// use spikard_http::grpc::streaming::message_stream_from_vec;
/// use bytes::Bytes;
///
/// let messages = vec![
///     Bytes::from("message1"),
///     Bytes::from("message2"),
/// ];
///
/// let stream = message_stream_from_vec(messages);
/// ```
pub fn message_stream_from_vec(messages: Vec<Bytes>) -> MessageStream {
    Box::pin(futures_util::stream::iter(messages.into_iter().map(Ok)))
}

/// Helper to create an empty message stream
///
/// Useful for testing or for handlers that need to return an empty stream.
pub fn empty_message_stream() -> MessageStream {
    Box::pin(futures_util::stream::empty())
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
pub fn single_message_stream(message: Bytes) -> MessageStream {
    Box::pin(futures_util::stream::once(async move { Ok(message) }))
}

/// Helper to create an error stream
///
/// Returns a stream that immediately yields a gRPC error.
///
/// # Example
///
/// ```ignore
/// use spikard_http::grpc::streaming::error_stream;
/// use tonic::Status;
///
/// let stream = error_stream(Status::internal("Something went wrong"));
/// ```
pub fn error_stream(status: Status) -> MessageStream {
    Box::pin(futures_util::stream::once(async move { Err(status) }))
}

/// Helper to convert a Tonic ReceiverStream to our MessageStream
///
/// This is used in the service bridge to convert Tonic's streaming types
/// to our internal representation.
pub fn from_tonic_stream<S>(stream: S) -> MessageStream
where
    S: Stream<Item = Result<Bytes, Status>> + Send + 'static,
{
    Box::pin(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::StreamExt;

    #[tokio::test]
    async fn test_message_stream_from_vec() {
        let messages = vec![Bytes::from("msg1"), Bytes::from("msg2"), Bytes::from("msg3")];

        let mut stream = message_stream_from_vec(messages.clone());

        let msg1 = stream.next().await.unwrap().unwrap();
        assert_eq!(msg1, Bytes::from("msg1"));

        let msg2 = stream.next().await.unwrap().unwrap();
        assert_eq!(msg2, Bytes::from("msg2"));

        let msg3 = stream.next().await.unwrap().unwrap();
        assert_eq!(msg3, Bytes::from("msg3"));

        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn test_empty_message_stream() {
        let mut stream = empty_message_stream();
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn test_single_message_stream() {
        let mut stream = single_message_stream(Bytes::from("single"));

        let msg = stream.next().await.unwrap().unwrap();
        assert_eq!(msg, Bytes::from("single"));

        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn test_error_stream() {
        let mut stream = error_stream(Status::internal("test error"));

        let result = stream.next().await.unwrap();
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code(), tonic::Code::Internal);
        assert_eq!(error.message(), "test error");

        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn test_message_stream_from_vec_empty() {
        let messages: Vec<Bytes> = vec![];
        let mut stream = message_stream_from_vec(messages);
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn test_message_stream_from_vec_large() {
        let mut messages = vec![];
        for i in 0..100 {
            messages.push(Bytes::from(format!("message{}", i)));
        }

        let mut stream = message_stream_from_vec(messages);

        for i in 0..100 {
            let msg = stream.next().await.unwrap().unwrap();
            assert_eq!(msg, Bytes::from(format!("message{}", i)));
        }

        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn test_from_tonic_stream() {
        let messages = vec![
            Ok(Bytes::from("a")),
            Ok(Bytes::from("b")),
            Err(Status::cancelled("done")),
        ];

        let tonic_stream = futures_util::stream::iter(messages);
        let mut stream = from_tonic_stream(tonic_stream);

        let msg1 = stream.next().await.unwrap().unwrap();
        assert_eq!(msg1, Bytes::from("a"));

        let msg2 = stream.next().await.unwrap().unwrap();
        assert_eq!(msg2, Bytes::from("b"));

        let result = stream.next().await.unwrap();
        assert!(result.is_err());

        assert!(stream.next().await.is_none());
    }

    #[test]
    fn test_streaming_request_creation() {
        let stream = empty_message_stream();
        let request = StreamingRequest {
            service_name: "test.Service".to_string(),
            method_name: "StreamMethod".to_string(),
            message_stream: stream,
            metadata: tonic::metadata::MetadataMap::new(),
        };

        assert_eq!(request.service_name, "test.Service");
        assert_eq!(request.method_name, "StreamMethod");
    }

    #[test]
    fn test_streaming_response_creation() {
        let stream = empty_message_stream();
        let response = StreamingResponse {
            message_stream: stream,
            metadata: tonic::metadata::MetadataMap::new(),
            trailers: None,
        };

        assert!(response.metadata.is_empty());
        assert!(response.trailers.is_none());
    }

    #[test]
    fn test_streaming_response_with_trailers() {
        let stream = empty_message_stream();
        let mut trailers = tonic::metadata::MetadataMap::new();
        trailers.insert(
            "x-request-id",
            "test-123"
                .parse::<tonic::metadata::MetadataValue<tonic::metadata::Ascii>>()
                .unwrap(),
        );

        let response = StreamingResponse {
            message_stream: stream,
            metadata: tonic::metadata::MetadataMap::new(),
            trailers: Some(trailers),
        };

        assert!(response.metadata.is_empty());
        assert!(response.trailers.is_some());
        let trailers = response.trailers.unwrap();
        assert_eq!(trailers.len(), 1);
    }
}

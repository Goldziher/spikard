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
/// Contains metadata and a stream of outgoing messages to the client.
pub struct StreamingResponse {
    /// Stream of outgoing protobuf messages
    pub message_stream: MessageStream,
    /// Response metadata
    pub metadata: tonic::metadata::MetadataMap,
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
        let messages = vec![Ok(Bytes::from("a")), Ok(Bytes::from("b")), Err(Status::cancelled("done"))];

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
        };

        assert!(response.metadata.is_empty());
    }
}

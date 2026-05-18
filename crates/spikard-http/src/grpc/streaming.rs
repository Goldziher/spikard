//! Streaming support utilities for gRPC
//!
//! This module provides utilities for handling streaming RPCs:
//! - Client streaming (receiving stream of messages)
//! - Server streaming (sending stream of messages)
//! - Bidirectional streaming (both directions)

use bytes::Bytes;
use futures_util::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
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

/// Wrap a `MessageStream` with a cumulative byte cap.
///
/// When `limit` is `None` the original stream is returned unchanged.
/// When `limit` is `Some(n)`, each successful item's byte length is added to an
/// internal counter. If the counter exceeds `n` **after** adding the item, the
/// current item is still yielded (matching the "after" semantics), and the next
/// `poll_next` returns `Err(Status::resource_exhausted(...))` followed by `None`.
pub fn limit_message_stream(inner: MessageStream, limit: Option<usize>) -> MessageStream {
    match limit {
        None => inner,
        Some(max_bytes) => Box::pin(LimitedMessageStream {
            inner,
            limit: max_bytes,
            consumed: 0,
            exhausted: false,
        }),
    }
}

/// A stream adapter that enforces a cumulative byte limit on a `MessageStream`.
///
/// Tracks the total encoded bytes yielded so far. Once the running total exceeds
/// `limit`, the stream emits a single `Err(Status::resource_exhausted(...))` item
/// and then terminates permanently.
struct LimitedMessageStream {
    inner: MessageStream,
    /// Maximum cumulative bytes allowed across the entire stream.
    limit: usize,
    /// Running total of bytes from successful items.
    consumed: usize,
    /// Set to `true` after the resource-exhausted error has been emitted.
    exhausted: bool,
}

impl Stream for LimitedMessageStream {
    type Item = Result<Bytes, Status>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.exhausted {
            return Poll::Ready(None);
        }

        match Pin::new(&mut self.inner).poll_next(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(Err(status))) => Poll::Ready(Some(Err(status))),
            Poll::Ready(Some(Ok(bytes))) => {
                self.consumed = self.consumed.saturating_add(bytes.len());
                if self.consumed > self.limit {
                    self.exhausted = true;
                    Poll::Ready(Some(Err(Status::resource_exhausted(format!(
                        "stream response size exceeded {} bytes",
                        self.limit
                    )))))
                } else {
                    Poll::Ready(Some(Ok(bytes)))
                }
            }
        }
    }
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

    fn make_four_message_stream() -> MessageStream {
        let messages: Vec<Result<Bytes, Status>> = vec![
            Ok(Bytes::from(vec![0u8; 100])),
            Ok(Bytes::from(vec![1u8; 100])),
            Ok(Bytes::from(vec![2u8; 100])),
            Ok(Bytes::from(vec![3u8; 100])),
        ];
        Box::pin(futures_util::stream::iter(messages))
    }

    #[tokio::test]
    async fn test_limit_message_stream_none_passes_all_messages() {
        let stream = make_four_message_stream();
        let mut limited = limit_message_stream(stream, None);

        let mut count = 0;
        while let Some(item) = limited.next().await {
            assert!(item.is_ok(), "expected Ok but got error: {:?}", item);
            count += 1;
        }
        assert_eq!(count, 4, "all four messages should pass through when limit is None");
    }

    #[tokio::test]
    async fn test_limit_message_stream_exact_limit_passes_all_messages() {
        // Limit exactly equals total size (4 * 100 = 400): all four should pass.
        let stream = make_four_message_stream();
        let mut limited = limit_message_stream(stream, Some(400));

        let mut count = 0;
        while let Some(item) = limited.next().await {
            assert!(item.is_ok(), "expected Ok but got error: {:?}", item);
            count += 1;
        }
        assert_eq!(count, 4, "all four messages should pass when limit == total size");
    }

    #[tokio::test]
    async fn test_limit_message_stream_exceeded_aborts_stream() {
        // Limit of 200 bytes: first two messages (200 bytes total) pass; the
        // third message (300 bytes consumed) triggers resource_exhausted.
        let stream = make_four_message_stream();
        let mut limited = limit_message_stream(stream, Some(200));

        let item1 = limited.next().await.expect("should have item 1");
        assert!(item1.is_ok(), "item 1 should be Ok");

        let item2 = limited.next().await.expect("should have item 2");
        assert!(item2.is_ok(), "item 2 should be Ok");

        // Third poll: consumed would be 300 > 200 → resource_exhausted error
        let item3 = limited.next().await.expect("should have item 3");
        let err = item3.expect_err("item 3 should be a resource_exhausted error");
        assert_eq!(err.code(), tonic::Code::ResourceExhausted);
        assert!(
            err.message().contains("200"),
            "error message should mention the limit: {}",
            err.message()
        );

        // Stream terminates: no fourth message
        let item4 = limited.next().await;
        assert!(item4.is_none(), "stream should be terminated after resource_exhausted");
    }

    #[tokio::test]
    async fn test_limit_message_stream_propagates_inner_errors() {
        let messages: Vec<Result<Bytes, Status>> = vec![
            Ok(Bytes::from(vec![0u8; 50])),
            Err(Status::internal("upstream failure")),
            Ok(Bytes::from(vec![0u8; 50])),
        ];
        let stream = Box::pin(futures_util::stream::iter(messages));
        let mut limited = limit_message_stream(stream, Some(1000));

        let item1 = limited.next().await.unwrap();
        assert!(item1.is_ok());

        let item2 = limited.next().await.unwrap();
        let err = item2.expect_err("should propagate inner error");
        assert_eq!(err.code(), tonic::Code::Internal);
        assert_eq!(err.message(), "upstream failure");

        // After an inner error the underlying stream may or may not continue;
        // what matters is the error was faithfully propagated.
    }
}

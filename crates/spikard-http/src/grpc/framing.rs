//! HTTP/2 gRPC frame parsing for client streaming support
//!
//! This module provides parsing of gRPC messages from HTTP/2 request bodies.
//! gRPC frames are structured according to RFC 9109 (gRPC over HTTP/2):
//!
//! ```text
//! +----------+----------+-+-+-+-+-+-+-+-+
//! |Compression|          Length        |
//! | Flags (1) |        (4 bytes)       |
//! +----------+----------+-+-+-+-+-+-+-+-+-+-+
//! |                                     |
//! |      Serialized Message (N bytes)  |
//! |                                     |
//! +-------------------------------------+
//! ```
//!
//! The compression flag indicates whether the message is compressed (1 = compressed, 0 = uncompressed).
//! The length is encoded as a big-endian u32, indicating the size of the message bytes.
//!
//! # Protocol Details
//!
//! - **Compression Flag**: 1 byte, value 0 or 1
//! - **Message Length**: 4 bytes, big-endian u32, maximum 4GB
//! - **Message Data**: N bytes, where N is the length from the header
//!
//! # Stream Processing
//!
//! The parser processes the HTTP/2 body stream by:
//! 1. Reading the 5-byte frame header (compression flag + length)
//! 2. Parsing the length as big-endian u32
//! 3. Validating the length against `max_message_size`
//! 4. Reading the message bytes
//! 5. Yielding the message
//! 6. Repeating until the body is exhausted
//!
//! # Error Handling
//!
//! The parser returns gRPC status codes according to RFC 9110:
//! - `INTERNAL`: Protocol parsing errors (incomplete frames, read errors)
//! - `RESOURCE_EXHAUSTED`: Message size exceeds limit
//! - `UNIMPLEMENTED`: Compression requested (not supported)
//!
//! # Example
//!
//! ```ignore
//! use spikard_http::grpc::framing::parse_grpc_client_stream;
//! use axum::body::Body;
//! use bytes::Bytes;
//! use futures_util::StreamExt;
//!
//! let body = Body::from("...");
//! let max_size = 4 * 1024 * 1024; // 4MB
//! let mut stream = parse_grpc_client_stream(body, max_size).await?;
//!
//! while let Some(result) = stream.next().await {
//!     match result {
//!         Ok(message) => println!("Message: {:?}", message),
//!         Err(status) => eprintln!("Error: {}", status),
//!     }
//! }
//! ```

use bytes::{Buf, Bytes, BytesMut};
use futures_util::stream;
use tonic::Status;

use super::streaming::MessageStream;

/// Parses an HTTP/2 gRPC request body as a stream of messages
///
/// Reads the gRPC frame format from the body stream, validating each frame
/// and yielding individual message bytes.
///
/// # Arguments
///
/// * `body` - The HTTP/2 request body stream
/// * `max_message_size` - Maximum allowed message size in bytes (validated per message)
///
/// # Returns
///
/// A `MessageStream` yielding:
/// - `Ok(Bytes)`: A complete parsed message
/// - `Err(Status)`: A gRPC protocol error
///
/// # Errors
///
/// Returns gRPC errors for:
/// - Incomplete frame (EOF before 5-byte header): `INTERNAL`
/// - Incomplete message (EOF before all message bytes): `INTERNAL`
/// - Message size > max_message_size: `RESOURCE_EXHAUSTED`
/// - Compression flag != 0: `UNIMPLEMENTED`
/// - Read errors from the body stream: `INTERNAL`
///
/// # Example
///
/// ```ignore
/// let body = Body::from(vec![
///     0x00,                      // compression: no
///     0x00, 0x00, 0x00, 0x05,   // length: 5 bytes
///     b'h', b'e', b'l', b'l', b'o',  // message
/// ]);
///
/// let stream = parse_grpc_client_stream(body, 1024).await?;
/// ```
pub async fn parse_grpc_client_stream(
    body: axum::body::Body,
    max_message_size: usize,
) -> Result<MessageStream, Status> {
    // Convert body into bytes
    let body_bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|e| Status::internal(format!("Failed to read body: {}", e)))?;

    // Create a buffered reader
    let buffer = BytesMut::from(&body_bytes[..]);

    // Parse frames from the buffer
    let messages = parse_all_frames(buffer, max_message_size)?;

    // Convert to a MessageStream
    Ok(Box::pin(stream::iter(messages.into_iter().map(Ok))))
}

/// Internal: Parse all frames from a buffer
fn parse_all_frames(mut buffer: BytesMut, max_message_size: usize) -> Result<Vec<Bytes>, Status> {
    let mut messages = Vec::new();

    while !buffer.is_empty() {
        // Check if we have enough bytes for the frame header
        if buffer.len() < 5 {
            return Err(Status::internal(
                "Incomplete gRPC frame header: expected 5 bytes, got less",
            ));
        }

        // Read the compression flag (1 byte)
        let compression_flag = buffer[0];
        if compression_flag != 0 {
            return Err(Status::unimplemented("Message compression not supported"));
        }

        // Read the message length (4 bytes, big-endian)
        let length_bytes = &buffer[1..5];
        let message_length =
            u32::from_be_bytes([length_bytes[0], length_bytes[1], length_bytes[2], length_bytes[3]]) as usize;

        // Validate message length against max size
        if message_length > max_message_size {
            return Err(Status::resource_exhausted(format!(
                "Message size {} exceeds maximum allowed size of {}",
                message_length, max_message_size
            )));
        }

        // Check if we have the complete message
        let total_frame_size = 5 + message_length;
        if buffer.len() < total_frame_size {
            return Err(Status::internal(
                "Incomplete gRPC message: expected more bytes than available",
            ));
        }

        // Extract the message bytes
        let message = buffer[5..total_frame_size].to_vec();
        messages.push(Bytes::from(message));

        // Advance the buffer
        buffer.advance(total_frame_size);
    }

    Ok(messages)
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::StreamExt;

    #[tokio::test]
    async fn test_single_frame_parsing() {
        // Frame: compression=0, length=5, message="hello"
        let frame = vec![
            0x00, // compression: no
            0x00, 0x00, 0x00, 0x05, // length: 5 bytes (big-endian)
            b'h', b'e', b'l', b'l', b'o', // message
        ];

        let body = axum::body::Body::from(frame);
        let mut stream = parse_grpc_client_stream(body, 1024).await.unwrap();
        let msg = stream.next().await;

        assert!(msg.is_some());
        assert!(msg.unwrap().is_ok());
        let result = stream.next().await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_multiple_frames() {
        // Two frames back-to-back
        let mut frame = Vec::new();

        // Frame 1: "hello"
        frame.push(0x00);
        frame.extend_from_slice(&[0x00, 0x00, 0x00, 0x05]);
        frame.extend_from_slice(b"hello");

        // Frame 2: "world"
        frame.push(0x00);
        frame.extend_from_slice(&[0x00, 0x00, 0x00, 0x05]);
        frame.extend_from_slice(b"world");

        let body = axum::body::Body::from(frame);
        let mut stream = parse_grpc_client_stream(body, 1024).await.unwrap();

        let msg1 = stream.next().await;
        assert!(msg1.is_some());
        assert_eq!(msg1.unwrap().unwrap(), b"hello"[..]);

        let msg2 = stream.next().await;
        assert!(msg2.is_some());
        assert_eq!(msg2.unwrap().unwrap(), b"world"[..]);

        let msg3 = stream.next().await;
        assert!(msg3.is_none());
    }

    #[tokio::test]
    async fn test_empty_body() {
        let body = axum::body::Body::from(Vec::<u8>::new());
        let mut stream = parse_grpc_client_stream(body, 1024).await.unwrap();

        let result = stream.next().await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_frame_size_at_limit() {
        let max_size = 10;
        let message = b"0123456789"; // exactly 10 bytes

        let mut frame = Vec::new();
        frame.push(0x00);
        frame.extend_from_slice(&[0x00, 0x00, 0x00, 0x0a]); // length: 10
        frame.extend_from_slice(message);

        let body = axum::body::Body::from(frame);
        let mut stream = parse_grpc_client_stream(body, max_size).await.unwrap();

        let msg = stream.next().await;
        assert!(msg.is_some());
        assert_eq!(msg.unwrap().unwrap(), message[..]);
    }

    #[tokio::test]
    async fn test_frame_exceeds_limit() {
        let max_size = 5;
        let message = b"toolong"; // 7 bytes, exceeds 5-byte limit

        let mut frame = Vec::new();
        frame.push(0x00);
        frame.extend_from_slice(&[0x00, 0x00, 0x00, 0x07]); // length: 7
        frame.extend_from_slice(message);

        let body = axum::body::Body::from(frame);
        let result = parse_grpc_client_stream(body, max_size).await;

        assert!(result.is_err());
        if let Err(status) = result {
            assert_eq!(status.code(), tonic::Code::ResourceExhausted);
        }
    }

    #[tokio::test]
    async fn test_incomplete_frame_header() {
        // Only 3 bytes of 5-byte header
        let frame = vec![0x00, 0x00, 0x00];

        let body = axum::body::Body::from(frame);
        let result = parse_grpc_client_stream(body, 1024).await;

        assert!(result.is_err());
        if let Err(status) = result {
            assert_eq!(status.code(), tonic::Code::Internal);
        }
    }

    #[tokio::test]
    async fn test_incomplete_frame_body() {
        // Header says 10 bytes but only provide 5
        let mut frame = Vec::new();
        frame.push(0x00);
        frame.extend_from_slice(&[0x00, 0x00, 0x00, 0x0a]); // length: 10
        frame.extend_from_slice(b"short"); // only 5 bytes

        let body = axum::body::Body::from(frame);
        let result = parse_grpc_client_stream(body, 1024).await;

        assert!(result.is_err());
        if let Err(status) = result {
            assert_eq!(status.code(), tonic::Code::Internal);
        }
    }

    #[tokio::test]
    async fn test_compression_flag_set() {
        // Frame with compression flag = 1 (not supported)
        let mut frame = Vec::new();
        frame.push(0x01); // compression: yes (unsupported)
        frame.extend_from_slice(&[0x00, 0x00, 0x00, 0x05]);
        frame.extend_from_slice(b"hello");

        let body = axum::body::Body::from(frame);
        let result = parse_grpc_client_stream(body, 1024).await;

        assert!(result.is_err());
        if let Err(status) = result {
            assert_eq!(status.code(), tonic::Code::Unimplemented);
        }
    }

    #[tokio::test]
    async fn test_large_message_length() {
        // Test with large length value (but within max_message_size for this test)
        let message = b"x".repeat(1000);
        let mut frame = Vec::new();
        frame.push(0x00);
        frame.extend_from_slice(&[0x00, 0x00, 0x03, 0xe8]); // length: 1000 (big-endian)
        frame.extend_from_slice(&message);

        let body = axum::body::Body::from(frame);
        let mut stream = parse_grpc_client_stream(body, 2000).await.unwrap();

        let msg = stream.next().await;
        assert!(msg.is_some());
        assert_eq!(msg.unwrap().unwrap().len(), 1000);
    }

    #[tokio::test]
    async fn test_zero_length_message() {
        // Frame with 0-byte message (valid in gRPC)
        let mut frame = Vec::new();
        frame.push(0x00);
        frame.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // length: 0

        let body = axum::body::Body::from(frame);
        let mut stream = parse_grpc_client_stream(body, 1024).await.unwrap();

        let msg = stream.next().await;
        assert!(msg.is_some());
        assert_eq!(msg.unwrap().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_multiple_frames_with_mixed_sizes() {
        let mut frame = Vec::new();

        // Frame 1: "abc" (3 bytes)
        frame.push(0x00);
        frame.extend_from_slice(&[0x00, 0x00, 0x00, 0x03]);
        frame.extend_from_slice(b"abc");

        // Frame 2: "defghij" (7 bytes)
        frame.push(0x00);
        frame.extend_from_slice(&[0x00, 0x00, 0x00, 0x07]);
        frame.extend_from_slice(b"defghij");

        // Frame 3: "" (0 bytes)
        frame.push(0x00);
        frame.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);

        // Frame 4: "x" (1 byte)
        frame.push(0x00);
        frame.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]);
        frame.extend_from_slice(b"x");

        let body = axum::body::Body::from(frame);
        let mut stream = parse_grpc_client_stream(body, 1024).await.unwrap();

        let msg1 = stream.next().await.unwrap().unwrap();
        assert_eq!(msg1, b"abc"[..]);

        let msg2 = stream.next().await.unwrap().unwrap();
        assert_eq!(msg2, b"defghij"[..]);

        let msg3 = stream.next().await.unwrap().unwrap();
        assert_eq!(msg3.len(), 0);

        let msg4 = stream.next().await.unwrap().unwrap();
        assert_eq!(msg4, b"x"[..]);

        let msg5 = stream.next().await;
        assert!(msg5.is_none());
    }

    #[test]
    fn test_big_endian_length_parsing() {
        // Test that length is correctly parsed as big-endian
        // Big-endian u32(256) = bytes [0x00, 0x00, 0x01, 0x00]
        let buffer = BytesMut::from(
            &[
                0x00, // compression flag
                0x00, 0x00, 0x01, 0x00, // length: 256 in big-endian
            ][..],
        );

        // Extract the 4-byte length manually to verify
        let length_bytes = &buffer[1..5];
        let length = u32::from_be_bytes([length_bytes[0], length_bytes[1], length_bytes[2], length_bytes[3]]);

        assert_eq!(length, 256);
    }

    #[test]
    fn test_big_endian_max_value() {
        // Test maximum u32 value in big-endian
        let buffer = BytesMut::from(
            &[
                0x00, 0xff, 0xff, 0xff, 0xff, // max u32
            ][..],
        );

        let length_bytes = &buffer[1..5];
        let length = u32::from_be_bytes([length_bytes[0], length_bytes[1], length_bytes[2], length_bytes[3]]);

        assert_eq!(length, u32::MAX);
    }

    #[tokio::test]
    async fn test_error_message_includes_size_info() {
        let max_size = 100;
        let message = b"x".repeat(150);

        let mut frame = Vec::new();
        frame.push(0x00);
        frame.extend_from_slice(&[0x00, 0x00, 0x00, 0x96]); // length: 150
        frame.extend_from_slice(&message);

        let body = axum::body::Body::from(frame);
        let result = parse_grpc_client_stream(body, max_size).await;

        assert!(result.is_err());
        if let Err(status) = result {
            assert!(status.message().contains("150"));
            assert!(status.message().contains("100"));
        }
    }

    #[tokio::test]
    async fn test_stream_collects_all_messages() {
        // Ensure that the returned stream properly yields all messages
        let mut frame = Vec::new();

        for i in 0..10 {
            frame.push(0x00);
            frame.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]);
            frame.push(b'0' + i as u8);
        }

        let body = axum::body::Body::from(frame);
        let stream = parse_grpc_client_stream(body, 1024).await.unwrap();
        let messages: Vec<_> = futures_util::StreamExt::collect(stream).await;

        assert_eq!(messages.len(), 10);
        for (i, msg) in messages.iter().enumerate() {
            assert_eq!(msg.as_ref().unwrap()[0], b'0' + i as u8);
        }
    }
}

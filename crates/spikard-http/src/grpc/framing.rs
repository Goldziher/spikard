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
//! - `UNIMPLEMENTED`: Unsupported compression algorithm or compression disabled
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
//! let mut stream = parse_grpc_client_stream(body, max_size, None, true).await?;
//!
//! while let Some(result) = stream.next().await {
//!     match result {
//!         Ok(message) => println!("Message: {:?}", message),
//!         Err(status) => eprintln!("Error: {}", status),
//!     }
//! }
//! ```

use bytes::{Buf, BufMut, Bytes, BytesMut};
use flate2::read::GzDecoder;
use futures_util::stream;
use std::io::Read;
use tonic::Status;

use super::streaming::MessageStream;

/// Size of the gRPC message header in bytes.
///
/// The header consists of:
/// - 1 byte compression flag
/// - 4 bytes big-endian message length
pub const GRPC_MESSAGE_HEADER_LEN: usize = 5;

/// Parses a unary gRPC payload from framed HTTP/2 body bytes.
///
/// Unary and server-streaming requests must carry exactly one framed message.
pub fn parse_unary_grpc_message(
    framed_body: &[u8],
    max_message_size: usize,
    grpc_encoding: Option<&str>,
    compression_enabled: bool,
) -> Result<Bytes, Status> {
    let messages = parse_all_frames(
        BytesMut::from(framed_body),
        max_message_size,
        grpc_encoding,
        compression_enabled,
    )?;

    match messages.len() {
        1 => Ok(messages.into_iter().next().expect("single message exists")),
        count => Err(Status::invalid_argument(format!(
            "Unary gRPC request must contain exactly one message frame, got {}",
            count
        ))),
    }
}

/// Encodes a protobuf payload into a single gRPC message frame.
pub fn encode_grpc_message(payload: Bytes) -> Result<Bytes, Status> {
    let message_length = u32::try_from(payload.len())
        .map_err(|_| Status::resource_exhausted("gRPC message exceeds 4GB frame length limit"))?;

    let mut framed = BytesMut::with_capacity(GRPC_MESSAGE_HEADER_LEN + payload.len());
    framed.put_u8(0); // compression flag: uncompressed
    framed.put_u32(message_length);
    framed.extend_from_slice(&payload);

    Ok(framed.freeze())
}

/// Parses an HTTP/2 gRPC request body as a stream of messages
///
/// Reads the gRPC frame format from the body stream, validating each frame
/// and yielding individual message bytes.
///
/// # Arguments
///
/// * `body` - The HTTP/2 request body stream
/// * `max_message_size` - Maximum allowed message size in bytes (validated per message)
/// * `grpc_encoding` - Value of the request `grpc-encoding` header, if present
/// * `compression_enabled` - Whether compressed gRPC payloads are allowed
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
/// - Compression flag set without valid `grpc-encoding`: `INVALID_ARGUMENT`
/// - Unsupported or disabled compression: `UNIMPLEMENTED`
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
/// let stream = parse_grpc_client_stream(body, 1024, None, true).await?;
/// ```
pub async fn parse_grpc_client_stream(
    body: axum::body::Body,
    max_message_size: usize,
    grpc_encoding: Option<&str>,
    compression_enabled: bool,
) -> Result<MessageStream, Status> {
    // Convert body into bytes
    let body_bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|e| Status::internal(format!("Failed to read body: {}", e)))?;

    // Create a buffered reader
    let buffer = BytesMut::from(&body_bytes[..]);

    // Parse frames from the buffer
    let messages = parse_all_frames(buffer, max_message_size, grpc_encoding, compression_enabled)?;

    // Convert to a MessageStream
    Ok(Box::pin(stream::iter(messages.into_iter().map(Ok))))
}

/// Internal: Parse all frames from a buffer
fn parse_all_frames(
    mut buffer: BytesMut,
    max_message_size: usize,
    grpc_encoding: Option<&str>,
    compression_enabled: bool,
) -> Result<Vec<Bytes>, Status> {
    let mut messages = Vec::new();

    while !buffer.is_empty() {
        // Check if we have enough bytes for the frame header
        if buffer.len() < GRPC_MESSAGE_HEADER_LEN {
            return Err(Status::internal(
                "Incomplete gRPC frame header: expected 5 bytes, got less",
            ));
        }

        // Read the compression flag (1 byte)
        let compression_flag = buffer[0];
        if compression_flag > 1 {
            return Err(Status::invalid_argument(format!(
                "Invalid gRPC compression flag: {}",
                compression_flag
            )));
        }

        // Read the message length (4 bytes, big-endian)
        let length_bytes = &buffer[1..GRPC_MESSAGE_HEADER_LEN];
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
        let total_frame_size = GRPC_MESSAGE_HEADER_LEN + message_length;
        if buffer.len() < total_frame_size {
            return Err(Status::internal(
                "Incomplete gRPC message: expected more bytes than available",
            ));
        }

        // Extract the message bytes and decompress if needed.
        let message_bytes = &buffer[GRPC_MESSAGE_HEADER_LEN..total_frame_size];
        let message = if compression_flag == 0 {
            Bytes::copy_from_slice(message_bytes)
        } else {
            decompress_message(message_bytes, grpc_encoding, compression_enabled, max_message_size)?
        };
        messages.push(message);

        // Advance the buffer
        buffer.advance(total_frame_size);
    }

    Ok(messages)
}

fn decompress_message(
    message_bytes: &[u8],
    grpc_encoding: Option<&str>,
    compression_enabled: bool,
    max_message_size: usize,
) -> Result<Bytes, Status> {
    if !compression_enabled {
        return Err(Status::unimplemented(
            "gRPC message compression is disabled by server configuration",
        ));
    }

    let encoding = grpc_encoding
        .map(|value| value.trim().to_ascii_lowercase())
        .ok_or_else(|| Status::invalid_argument("Compressed gRPC message missing grpc-encoding header"))?;

    let decompressed = match encoding.as_str() {
        "gzip" => {
            let mut decoder = GzDecoder::new(message_bytes);
            let mut out = Vec::new();
            decoder
                .read_to_end(&mut out)
                .map_err(|e| Status::internal(format!("Failed to decompress gzip gRPC frame: {}", e)))?;
            out
        }
        "identity" => {
            return Err(Status::invalid_argument(
                "Compressed gRPC frame cannot use grpc-encoding=identity",
            ));
        }
        other => {
            return Err(Status::unimplemented(format!("Unsupported grpc-encoding '{}'", other)));
        }
    };

    if decompressed.len() > max_message_size {
        return Err(Status::resource_exhausted(format!(
            "Decompressed message size {} exceeds maximum allowed size of {}",
            decompressed.len(),
            max_message_size
        )));
    }

    Ok(Bytes::from(decompressed))
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::{Compression, write::GzEncoder};
    use futures_util::StreamExt;
    use std::io::Write;

    #[tokio::test]
    async fn test_single_frame_parsing() {
        // Frame: compression=0, length=5, message="hello"
        let frame = vec![
            0x00, // compression: no
            0x00, 0x00, 0x00, 0x05, // length: 5 bytes (big-endian)
            b'h', b'e', b'l', b'l', b'o', // message
        ];

        let body = axum::body::Body::from(frame);
        let mut stream = parse_grpc_client_stream(body, 1024, None, true).await.unwrap();
        let msg = stream.next().await;

        assert!(msg.is_some());
        assert!(msg.unwrap().is_ok());
        let result = stream.next().await;
        assert!(result.is_none());
    }

    #[test]
    fn test_encode_grpc_message_adds_framing_header() {
        let framed = encode_grpc_message(Bytes::from_static(b"hello")).unwrap();

        assert_eq!(framed[0], 0x00);
        assert_eq!(&framed[1..5], &[0x00, 0x00, 0x00, 0x05]);
        assert_eq!(&framed[5..], b"hello");
    }

    #[test]
    fn test_parse_unary_grpc_message_requires_exactly_one_frame() {
        let mut body = Vec::new();
        body.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x01, b'a']);
        body.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x01, b'b']);

        let err = parse_unary_grpc_message(&body, 1024, None, true).unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
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
        let mut stream = parse_grpc_client_stream(body, 1024, None, true).await.unwrap();

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
        let mut stream = parse_grpc_client_stream(body, 1024, None, true).await.unwrap();

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
        let mut stream = parse_grpc_client_stream(body, max_size, None, true).await.unwrap();

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
        let result = parse_grpc_client_stream(body, max_size, None, true).await;

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
        let result = parse_grpc_client_stream(body, 1024, None, true).await;

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
        let result = parse_grpc_client_stream(body, 1024, None, true).await;

        assert!(result.is_err());
        if let Err(status) = result {
            assert_eq!(status.code(), tonic::Code::Internal);
        }
    }

    #[tokio::test]
    async fn test_compression_flag_set_with_missing_encoding_header() {
        let mut frame = Vec::new();
        frame.push(0x01); // compression: yes
        frame.extend_from_slice(&[0x00, 0x00, 0x00, 0x05]);
        frame.extend_from_slice(b"hello");

        let body = axum::body::Body::from(frame);
        let result = parse_grpc_client_stream(body, 1024, None, true).await;

        assert!(result.is_err());
        if let Err(status) = result {
            assert_eq!(status.code(), tonic::Code::InvalidArgument);
        }
    }

    #[tokio::test]
    async fn test_compression_flag_set_with_unsupported_encoding() {
        let mut frame = Vec::new();
        frame.push(0x01); // compression: yes
        frame.extend_from_slice(&[0x00, 0x00, 0x00, 0x05]);
        frame.extend_from_slice(b"hello");

        let body = axum::body::Body::from(frame);
        let result = parse_grpc_client_stream(body, 1024, Some("br"), true).await;

        assert!(result.is_err());
        if let Err(status) = result {
            assert_eq!(status.code(), tonic::Code::Unimplemented);
            assert!(status.message().contains("Unsupported grpc-encoding"));
        }
    }

    #[tokio::test]
    async fn test_compression_flag_set_when_compression_disabled() {
        let mut frame = Vec::new();
        frame.push(0x01); // compression: yes
        frame.extend_from_slice(&[0x00, 0x00, 0x00, 0x05]);
        frame.extend_from_slice(b"hello");

        let body = axum::body::Body::from(frame);
        let result = parse_grpc_client_stream(body, 1024, Some("gzip"), false).await;

        assert!(result.is_err());
        if let Err(status) = result {
            assert_eq!(status.code(), tonic::Code::Unimplemented);
            assert!(status.message().contains("disabled"));
        }
    }

    #[tokio::test]
    async fn test_compression_flag_set_with_gzip_encoding_decompresses_message() {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(b"hello").unwrap();
        let compressed = encoder.finish().unwrap();

        let mut frame = Vec::new();
        frame.push(0x01); // compression: yes
        frame.extend_from_slice(&(compressed.len() as u32).to_be_bytes());
        frame.extend_from_slice(&compressed);

        let body = axum::body::Body::from(frame);
        let mut stream = parse_grpc_client_stream(body, 1024, Some("gzip"), true).await.unwrap();

        let msg = stream.next().await.unwrap().unwrap();
        assert_eq!(msg, Bytes::from_static(b"hello"));
        assert!(stream.next().await.is_none());
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
        let mut stream = parse_grpc_client_stream(body, 2000, None, true).await.unwrap();

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
        let mut stream = parse_grpc_client_stream(body, 1024, None, true).await.unwrap();

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
        let mut stream = parse_grpc_client_stream(body, 1024, None, true).await.unwrap();

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
        let result = parse_grpc_client_stream(body, max_size, None, true).await;

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
        let stream = parse_grpc_client_stream(body, 1024, None, true).await.unwrap();
        let messages: Vec<_> = futures_util::StreamExt::collect(stream).await;

        assert_eq!(messages.len(), 10);
        for (i, msg) in messages.iter().enumerate() {
            assert_eq!(msg.as_ref().unwrap()[0], b'0' + i as u8);
        }
    }
}

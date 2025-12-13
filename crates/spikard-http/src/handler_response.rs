use axum::{
    BoxError,
    body::Body,
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::Response as AxumResponse,
};
use bytes::Bytes;
use futures::{Stream, StreamExt};
use std::pin::Pin;

/// Unified response type that can represent either a ready response or a streaming body.
///
/// This enum allows handlers to return either:
/// - A complete response that's ready to send (`Response` variant)
/// - A streaming response with potentially unbounded data (`Stream` variant)
///
/// # Variants
///
/// * `Response` - A complete Axum response ready to send to the client. Use this for
///   responses where you have all the data ready (files, JSON bodies, HTML, etc.)
///
/// * `Stream` - A streaming response that produces data chunks over time. Use this for:
///   - Large files (avoid loading entire file in memory)
///   - Server-Sent Events (SSE)
///   - Long-polling responses
///   - Real-time data feeds
///   - Any unbounded or very large responses
///
/// # Examples
///
/// ```ignore
/// // Regular response
/// let response = AxumResponse::builder()
///     .status(StatusCode::OK)
///     .body(Body::from("Hello"))
///     .unwrap();
/// let handler_response = HandlerResponse::from(response);
///
/// // Streaming response
/// let stream = futures::stream::iter(vec![
///     Ok::<_, Box<dyn std::error::Error>>(Bytes::from("chunk1")),
///     Ok(Bytes::from("chunk2")),
/// ]);
/// let response = HandlerResponse::stream(stream)
///     .with_status(StatusCode::OK);
/// ```
pub enum HandlerResponse {
    /// A complete response ready to send
    Response(AxumResponse<Body>),
    /// A streaming response with custom status and headers
    Stream {
        /// The byte stream that will be sent to the client
        stream: Pin<Box<dyn Stream<Item = Result<Bytes, BoxError>> + Send + 'static>>,
        /// HTTP status code for the response
        status: StatusCode,
        /// Response headers to send
        headers: HeaderMap,
    },
}

impl From<AxumResponse<Body>> for HandlerResponse {
    fn from(response: AxumResponse<Body>) -> Self {
        HandlerResponse::Response(response)
    }
}

impl HandlerResponse {
    /// Convert the handler response into an Axum response.
    ///
    /// Consumes the `HandlerResponse` and produces an `AxumResponse<Body>` ready
    /// to be sent to the client. For streaming responses, wraps the stream in an
    /// Axum Body.
    ///
    /// # Returns
    /// An `AxumResponse<Body>` ready to be returned from an Axum handler
    pub fn into_response(self) -> AxumResponse<Body> {
        match self {
            HandlerResponse::Response(response) => response,
            HandlerResponse::Stream {
                stream,
                status,
                mut headers,
            } => {
                let body = Body::from_stream(stream);
                let mut response = AxumResponse::new(body);
                *response.status_mut() = status;
                response.headers_mut().extend(headers.drain());
                response
            }
        }
    }

    /// Create a streaming response from any async stream of byte chunks.
    ///
    /// Wraps an async stream of byte chunks into a `HandlerResponse::Stream`.
    /// This is useful for large files, real-time data, or any unbounded response.
    ///
    /// # Type Parameters
    /// * `S` - The stream type implementing `Stream<Item = Result<Bytes, E>>`
    /// * `E` - The error type that can be converted to `BoxError`
    ///
    /// # Arguments
    /// * `stream` - An async stream that yields byte chunks or errors
    ///
    /// # Returns
    /// A `HandlerResponse` with 200 OK status and empty headers (customize with
    /// `with_status()` and `with_header()`)
    ///
    /// # Example
    ///
    /// ```ignore
    /// use futures::stream;
    /// use spikard_http::HandlerResponse;
    /// use bytes::Bytes;
    ///
    /// let stream = stream::iter(vec![
    ///     Ok::<_, Box<dyn std::error::Error>>(Bytes::from("Hello ")),
    ///     Ok(Bytes::from("World")),
    /// ]);
    /// let response = HandlerResponse::stream(stream)
    ///     .with_status(StatusCode::OK);
    /// ```
    pub fn stream<S, E>(stream: S) -> Self
    where
        S: Stream<Item = Result<Bytes, E>> + Send + 'static,
        E: Into<BoxError>,
    {
        let mapped = stream.map(|chunk| chunk.map_err(Into::into));
        HandlerResponse::Stream {
            stream: Box::pin(mapped),
            status: StatusCode::OK,
            headers: HeaderMap::new(),
        }
    }

    /// Override the HTTP status code for the streaming response.
    ///
    /// Sets the HTTP status code to be used in the response. This only affects
    /// `Stream` variants; regular responses already have their status set.
    ///
    /// # Arguments
    /// * `status` - The HTTP status code to use (e.g., `StatusCode::OK`)
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Example
    ///
    /// ```ignore
    /// let response = HandlerResponse::stream(my_stream)
    ///     .with_status(StatusCode::PARTIAL_CONTENT);
    /// ```
    pub fn with_status(mut self, status: StatusCode) -> Self {
        if let HandlerResponse::Stream { status: s, .. } = &mut self {
            *s = status;
        }
        self
    }

    /// Insert or replace a header on the streaming response.
    ///
    /// Adds an HTTP header to the response. This only affects `Stream` variants;
    /// regular responses already have their headers set. If a header with the same
    /// name already exists, it will be replaced.
    ///
    /// # Arguments
    /// * `name` - The header name (e.g., `HeaderName::from_static("content-type")`)
    /// * `value` - The header value
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Example
    ///
    /// ```ignore
    /// use axum::http::{HeaderName, HeaderValue};
    ///
    /// let response = HandlerResponse::stream(my_stream)
    ///     .with_header(
    ///         HeaderName::from_static("content-type"),
    ///         HeaderValue::from_static("application/octet-stream")
    ///     );
    /// ```
    pub fn with_header(mut self, name: HeaderName, value: HeaderValue) -> Self {
        if let HandlerResponse::Stream { headers, .. } = &mut self {
            headers.insert(name, value);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::header;
    use http_body_util::BodyExt;

    /// Test 1: Convert AxumResponse → HandlerResponse::Response
    #[test]
    fn test_from_axum_response() {
        let axum_response = AxumResponse::new(Body::from("test body"));
        let handler_response = HandlerResponse::from(axum_response);

        match handler_response {
            HandlerResponse::Response(_) => {}
            HandlerResponse::Stream { .. } => panic!("Expected Response variant"),
        }
    }

    /// Test 2: Create stream with chunks, verify wrapping
    #[tokio::test]
    async fn test_stream_creation_with_chunks() {
        let chunks = vec![
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(Bytes::from("chunk1")),
            Ok(Bytes::from("chunk2")),
            Ok(Bytes::from("chunk3")),
        ];
        let stream = futures::stream::iter(chunks);
        let handler_response = HandlerResponse::stream(stream);

        match handler_response {
            HandlerResponse::Stream { status, headers, .. } => {
                assert_eq!(status, StatusCode::OK);
                assert!(headers.is_empty());
            }
            HandlerResponse::Response(_) => panic!("Expected Stream variant"),
        }
    }

    /// Test 3: Stream with custom status code (206 Partial Content)
    #[tokio::test]
    async fn test_stream_with_custom_status() {
        let chunks = vec![Ok::<_, Box<dyn std::error::Error + Send + Sync>>(Bytes::from(
            "partial",
        ))];
        let stream = futures::stream::iter(chunks);
        let handler_response = HandlerResponse::stream(stream).with_status(StatusCode::PARTIAL_CONTENT);

        match handler_response {
            HandlerResponse::Stream { status, .. } => {
                assert_eq!(status, StatusCode::PARTIAL_CONTENT);
            }
            HandlerResponse::Response(_) => panic!("Expected Stream variant"),
        }
    }

    /// Test 4: Stream with headers via with_header()
    #[tokio::test]
    async fn test_stream_with_headers() {
        let chunks = vec![Ok::<_, Box<dyn std::error::Error + Send + Sync>>(Bytes::from("test"))];
        let stream = futures::stream::iter(chunks);
        let handler_response = HandlerResponse::stream(stream)
            .with_header(header::CONTENT_TYPE, HeaderValue::from_static("application/x-ndjson"))
            .with_header(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"));

        match handler_response {
            HandlerResponse::Stream { headers, .. } => {
                assert_eq!(headers.get(header::CONTENT_TYPE).unwrap(), "application/x-ndjson");
                assert_eq!(headers.get(header::CACHE_CONTROL).unwrap(), "no-cache");
            }
            HandlerResponse::Response(_) => panic!("Expected Stream variant"),
        }
    }

    /// Test 5: Stream body consumption - read all chunks from stream
    #[tokio::test]
    async fn test_stream_body_consumption() {
        let chunks = vec![
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(Bytes::from("hello ")),
            Ok(Bytes::from("world")),
            Ok(Bytes::from("!")),
        ];
        let stream = futures::stream::iter(chunks);
        let handler_response = HandlerResponse::stream(stream).with_status(StatusCode::OK);

        let axum_response = handler_response.into_response();
        let body = axum_response.into_body().collect().await.unwrap();
        let bytes = body.to_bytes();

        assert_eq!(bytes, "hello world!");
    }

    /// Test 6: Into response for Response variant - passthrough conversion
    #[tokio::test]
    async fn test_into_response_for_response_variant() {
        let original_body = "test response body";
        let axum_response = AxumResponse::new(Body::from(original_body));
        let handler_response = HandlerResponse::from(axum_response);

        let result = handler_response.into_response();
        let body = result.into_body().collect().await.unwrap();
        let bytes = body.to_bytes();

        assert_eq!(bytes, original_body);
    }

    /// Test 7: Method chaining - with_status() → with_header() → with_header()
    #[tokio::test]
    async fn test_method_chaining() {
        let chunks = vec![Ok::<_, Box<dyn std::error::Error + Send + Sync>>(Bytes::from(
            "chained",
        ))];
        let stream = futures::stream::iter(chunks);

        let handler_response = HandlerResponse::stream(stream)
            .with_status(StatusCode::CREATED)
            .with_header(header::CONTENT_TYPE, HeaderValue::from_static("text/plain"))
            .with_header(header::ETAG, HeaderValue::from_static("\"abc123\""));

        match handler_response {
            HandlerResponse::Stream { status, headers, .. } => {
                assert_eq!(status, StatusCode::CREATED);
                assert_eq!(headers.get(header::CONTENT_TYPE).unwrap(), "text/plain");
                assert_eq!(headers.get(header::ETAG).unwrap(), "\"abc123\"");
            }
            HandlerResponse::Response(_) => panic!("Expected Stream variant"),
        }
    }

    /// Test 8: Empty stream - zero-byte stream handling
    #[tokio::test]
    async fn test_empty_stream() {
        let chunks: Vec<Result<Bytes, Box<dyn std::error::Error + Send + Sync>>> = vec![];
        let stream = futures::stream::iter(chunks);
        let handler_response = HandlerResponse::stream(stream).with_status(StatusCode::NO_CONTENT);

        let axum_response = handler_response.into_response();
        let status = axum_response.status();
        let body = axum_response.into_body().collect().await.unwrap();
        let bytes = body.to_bytes();

        assert!(bytes.is_empty());
        assert_eq!(status, StatusCode::NO_CONTENT);
    }

    /// Test 9: Large stream - many chunks (100+ items)
    #[tokio::test]
    async fn test_large_stream() {
        let chunks: Vec<Result<Bytes, Box<dyn std::error::Error + Send + Sync>>> =
            (0..150).map(|i| Ok(Bytes::from(format!("chunk{} ", i)))).collect();

        let stream = futures::stream::iter(chunks);
        let handler_response = HandlerResponse::stream(stream);

        let axum_response = handler_response.into_response();
        let body = axum_response.into_body().collect().await.unwrap();
        let bytes = body.to_bytes();

        assert!(bytes.len() > 1000);
        for i in 0..150 {
            let expected = format!("chunk{} ", i);
            assert!(std::str::from_utf8(&bytes).unwrap().contains(&expected));
        }
    }

    /// Test 10: Error in stream - stream item returns Err, verify error propagation
    #[tokio::test]
    async fn test_stream_error_propagation() {
        let chunks: Vec<Result<Bytes, Box<dyn std::error::Error + Send + Sync>>> = vec![
            Ok(Bytes::from("good1 ")),
            Err("custom error".into()),
            Ok(Bytes::from("good2")),
        ];

        let stream = futures::stream::iter(chunks);
        let handler_response = HandlerResponse::stream(stream);

        let axum_response = handler_response.into_response();
        let result = axum_response.into_body().collect().await;

        assert!(result.is_err());
    }

    /// Test 11: Response variant ignores with_status()
    #[test]
    fn test_response_variant_ignores_with_status() {
        let axum_response = AxumResponse::builder()
            .status(StatusCode::OK)
            .body(Body::from("test"))
            .unwrap();
        let handler_response = HandlerResponse::from(axum_response);

        let result = handler_response.with_status(StatusCode::NOT_FOUND);

        match result {
            HandlerResponse::Response(resp) => {
                assert_eq!(resp.status(), StatusCode::OK);
            }
            HandlerResponse::Stream { .. } => panic!("Expected Response variant"),
        }
    }

    /// Test 12: Response variant ignores with_header()
    #[test]
    fn test_response_variant_ignores_with_header() {
        let axum_response = AxumResponse::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/plain")
            .body(Body::from("test"))
            .unwrap();
        let handler_response = HandlerResponse::from(axum_response);

        let result = handler_response.with_header(header::CACHE_CONTROL, HeaderValue::from_static("max-age=3600"));

        match result {
            HandlerResponse::Response(resp) => {
                assert!(resp.headers().get(header::CACHE_CONTROL).is_none());
            }
            HandlerResponse::Stream { .. } => panic!("Expected Response variant"),
        }
    }

    /// Test 13: Stream into_response applies status and headers
    #[tokio::test]
    async fn test_stream_into_response_applies_status_and_headers() {
        let chunks = vec![Ok::<_, Box<dyn std::error::Error + Send + Sync>>(Bytes::from(
            "stream data",
        ))];
        let stream = futures::stream::iter(chunks);

        let handler_response = HandlerResponse::stream(stream)
            .with_status(StatusCode::PARTIAL_CONTENT)
            .with_header(header::CONTENT_RANGE, HeaderValue::from_static("bytes 0-10/100"));

        let axum_response = handler_response.into_response();

        assert_eq!(axum_response.status(), StatusCode::PARTIAL_CONTENT);
        assert_eq!(
            axum_response.headers().get(header::CONTENT_RANGE).unwrap(),
            "bytes 0-10/100"
        );

        let body = axum_response.into_body().collect().await.unwrap();
        assert_eq!(body.to_bytes(), "stream data");
    }

    /// Test 14: Multiple header replacements via with_header()
    #[tokio::test]
    async fn test_multiple_header_replacements() {
        let chunks = vec![Ok::<_, Box<dyn std::error::Error + Send + Sync>>(Bytes::from("data"))];
        let stream = futures::stream::iter(chunks);

        let handler_response = HandlerResponse::stream(stream)
            .with_header(header::CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .with_header(header::CONTENT_TYPE, HeaderValue::from_static("application/x-ndjson"));

        match handler_response {
            HandlerResponse::Stream { headers, .. } => {
                assert_eq!(headers.get(header::CONTENT_TYPE).unwrap(), "application/x-ndjson");
            }
            HandlerResponse::Response(_) => panic!("Expected Stream variant"),
        }
    }

    /// Test 15: Stream with various status codes
    #[tokio::test]
    async fn test_stream_with_various_status_codes() {
        let status_codes = vec![
            StatusCode::OK,
            StatusCode::CREATED,
            StatusCode::ACCEPTED,
            StatusCode::PARTIAL_CONTENT,
            StatusCode::MULTI_STATUS,
        ];

        for status in status_codes {
            let chunks = vec![Ok::<_, Box<dyn std::error::Error + Send + Sync>>(Bytes::from("test"))];
            let stream = futures::stream::iter(chunks);
            let handler_response = HandlerResponse::stream(stream).with_status(status);

            match handler_response {
                HandlerResponse::Stream { status: s, .. } => {
                    assert_eq!(s, status);
                }
                HandlerResponse::Response(_) => panic!("Expected Stream variant"),
            }
        }
    }

    /// Test 16: Stream with JSON lines content (fixture-based)
    #[tokio::test]
    async fn test_stream_with_json_lines_content() {
        let chunks = vec![
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(Bytes::from(r#"{"index":0,"payload":"alpha"}"#)),
            Ok(Bytes::from("\n")),
            Ok(Bytes::from(r#"{"index":1,"payload":"beta"}"#)),
            Ok(Bytes::from("\n")),
            Ok(Bytes::from(r#"{"index":2,"payload":"gamma"}"#)),
            Ok(Bytes::from("\n")),
        ];

        let stream = futures::stream::iter(chunks);
        let handler_response = HandlerResponse::stream(stream)
            .with_status(StatusCode::OK)
            .with_header(header::CONTENT_TYPE, HeaderValue::from_static("application/x-ndjson"));

        let axum_response = handler_response.into_response();
        let status = axum_response.status();
        let body = axum_response.into_body().collect().await.unwrap();
        let bytes = body.to_bytes();
        let body_str = std::str::from_utf8(&bytes).unwrap();

        assert_eq!(status, StatusCode::OK);
        assert!(body_str.contains("alpha"));
        assert!(body_str.contains("beta"));
        assert!(body_str.contains("gamma"));
    }

    /// Test 17: Round-trip Response → HandlerResponse → Response
    #[tokio::test]
    async fn test_response_roundtrip() {
        let original = AxumResponse::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/plain")
            .body(Body::from("roundtrip test"))
            .unwrap();

        let handler_response = HandlerResponse::from(original);
        let result = handler_response.into_response();

        assert_eq!(result.status(), StatusCode::OK);
        assert_eq!(result.headers().get(header::CONTENT_TYPE).unwrap(), "text/plain");

        let body = result.into_body().collect().await.unwrap();
        assert_eq!(body.to_bytes(), "roundtrip test");
    }

    /// Test 18: Single chunk stream (minimal data)
    #[tokio::test]
    async fn test_single_chunk_stream() {
        let chunks = vec![Ok::<_, Box<dyn std::error::Error + Send + Sync>>(Bytes::from("only"))];
        let stream = futures::stream::iter(chunks);
        let handler_response = HandlerResponse::stream(stream).with_status(StatusCode::OK);

        let axum_response = handler_response.into_response();
        let status = axum_response.status();
        let body = axum_response.into_body().collect().await.unwrap();
        let bytes = body.to_bytes();

        assert_eq!(bytes, "only");
        assert_eq!(status, StatusCode::OK);
    }

    /// Test 19: Stream with 1000+ chunks (performance edge case)
    #[tokio::test]
    async fn test_very_large_stream_many_chunks() {
        let chunk_count = 1500;
        let chunks: Vec<Result<Bytes, Box<dyn std::error::Error + Send + Sync>>> =
            (0..chunk_count).map(|_| Ok(Bytes::from(format!("x")))).collect();

        let stream = futures::stream::iter(chunks);
        let handler_response = HandlerResponse::stream(stream);

        let axum_response = handler_response.into_response();
        let body = axum_response.into_body().collect().await.unwrap();
        let bytes = body.to_bytes();

        assert_eq!(bytes.len(), chunk_count);
    }

    /// Test 20: Stream with varying chunk sizes (1 byte to 1MB)
    #[tokio::test]
    async fn test_stream_with_varying_chunk_sizes() {
        let chunks: Vec<Result<Bytes, Box<dyn std::error::Error + Send + Sync>>> = vec![
            Ok(Bytes::from("x")),
            Ok(Bytes::from("xx".repeat(100))),
            Ok(Bytes::from("x".repeat(10_000))),
            Ok(Bytes::from("x".repeat(100_000))),
        ];

        let stream = futures::stream::iter(chunks);
        let handler_response = HandlerResponse::stream(stream);

        let axum_response = handler_response.into_response();
        let body = axum_response.into_body().collect().await.unwrap();
        let bytes = body.to_bytes();

        assert_eq!(bytes.len(), 110_201);
    }

    /// Test 21: Stream with error in the middle (chunk 500/1000)
    #[tokio::test]
    async fn test_stream_error_in_middle() {
        let chunks: Vec<Result<Bytes, Box<dyn std::error::Error + Send + Sync>>> = (0..1000)
            .map(|i| {
                if i == 500 {
                    Err("midstream error".into())
                } else {
                    Ok(Bytes::from("chunk"))
                }
            })
            .collect();

        let stream = futures::stream::iter(chunks);
        let handler_response = HandlerResponse::stream(stream);

        let axum_response = handler_response.into_response();
        let result = axum_response.into_body().collect().await;

        assert!(result.is_err());
    }

    /// Test 22: Stream with SSE-like headers
    #[tokio::test]
    async fn test_stream_with_sse_headers() {
        let chunks = vec![
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(Bytes::from("event: message\n")),
            Ok(Bytes::from("data: {\"msg\": \"hello\"}\n\n")),
        ];
        let stream = futures::stream::iter(chunks);

        let handler_response = HandlerResponse::stream(stream)
            .with_status(StatusCode::OK)
            .with_header(header::CONTENT_TYPE, HeaderValue::from_static("text/event-stream"))
            .with_header(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"))
            .with_header(header::CONNECTION, HeaderValue::from_static("keep-alive"));

        let axum_response = handler_response.into_response();

        assert_eq!(axum_response.status(), StatusCode::OK);
        assert_eq!(
            axum_response.headers().get(header::CONTENT_TYPE).unwrap(),
            "text/event-stream"
        );
        assert_eq!(axum_response.headers().get(header::CACHE_CONTROL).unwrap(), "no-cache");

        let body = axum_response.into_body().collect().await.unwrap();
        let body_bytes = body.to_bytes();
        let body_str = std::str::from_utf8(&body_bytes).unwrap();
        assert!(body_str.contains("event: message"));
    }

    /// Test 23: Stream with WebSocket-like upgrade headers (200 OK with Upgrade)
    #[tokio::test]
    async fn test_stream_with_websocket_headers() {
        let chunks = vec![Ok::<_, Box<dyn std::error::Error + Send + Sync>>(Bytes::from(
            "ws-frame-data",
        ))];
        let stream = futures::stream::iter(chunks);

        let handler_response = HandlerResponse::stream(stream)
            .with_status(StatusCode::OK)
            .with_header(header::UPGRADE, HeaderValue::from_static("websocket"))
            .with_header(
                HeaderName::from_static("sec-websocket-accept"),
                HeaderValue::from_static("s3pPLMBiTxaQ9kYGzzhZRbK+xOo="),
            );

        let axum_response = handler_response.into_response();

        assert_eq!(axum_response.status(), StatusCode::OK);
        assert_eq!(axum_response.headers().get(header::UPGRADE).unwrap(), "websocket");

        let body = axum_response.into_body().collect().await.unwrap();
        assert_eq!(body.to_bytes(), "ws-frame-data");
    }

    /// Test 24: Stream status transitions (from 200 OK to 206 Partial Content)
    #[tokio::test]
    async fn test_stream_status_transition() {
        let chunks = vec![Ok::<_, Box<dyn std::error::Error + Send + Sync>>(Bytes::from("data"))];
        let stream = futures::stream::iter(chunks);

        let handler_response = HandlerResponse::stream(stream)
            .with_status(StatusCode::OK)
            .with_status(StatusCode::PARTIAL_CONTENT);

        match handler_response {
            HandlerResponse::Stream { status, .. } => {
                assert_eq!(status, StatusCode::PARTIAL_CONTENT);
            }
            HandlerResponse::Response(_) => panic!("Expected Stream variant"),
        }
    }

    /// Test 25: Stream with chunked transfer encoding simulation
    #[tokio::test]
    async fn test_stream_chunked_encoding_simulation() {
        let chunks = vec![
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(Bytes::from("5\r\nhello\r\n")),
            Ok(Bytes::from("5\r\nworld\r\n")),
            Ok(Bytes::from("0\r\n\r\n")),
        ];

        let stream = futures::stream::iter(chunks);
        let handler_response =
            HandlerResponse::stream(stream).with_header(header::TRANSFER_ENCODING, HeaderValue::from_static("chunked"));

        let axum_response = handler_response.into_response();
        let body = axum_response.into_body().collect().await.unwrap();
        let body_bytes = body.to_bytes();

        assert!(std::str::from_utf8(&body_bytes).unwrap().contains("hello"));
    }

    /// Test 26: Stream with binary data (non-UTF8)
    #[tokio::test]
    async fn test_stream_with_binary_data() {
        let chunks = vec![
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(Bytes::from(vec![0xFF, 0xD8, 0xFF])),
            Ok(Bytes::from(vec![0xE0, 0x00, 0x10])),
            Ok(Bytes::from(vec![0x4A, 0x46, 0x49])),
        ];

        let stream = futures::stream::iter(chunks);
        let handler_response = HandlerResponse::stream(stream).with_header(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        );

        let axum_response = handler_response.into_response();
        let body = axum_response.into_body().collect().await.unwrap();
        let bytes = body.to_bytes();

        assert_eq!(bytes[0], 0xFF);
        assert_eq!(bytes[1], 0xD8);
        assert_eq!(bytes[2], 0xFF);
        assert_eq!(bytes[3], 0xE0);
        assert_eq!(bytes[4], 0x00);
    }

    /// Test 27: Stream with null bytes in payload
    #[tokio::test]
    async fn test_stream_with_null_bytes() {
        let chunks = vec![
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(Bytes::from(vec![0x00, 0x01, 0x02])),
            Ok(Bytes::from(vec![0x00, 0x00, 0x00])),
            Ok(Bytes::from(vec![0xFF, 0xFE, 0xFD])),
        ];

        let stream = futures::stream::iter(chunks);
        let handler_response = HandlerResponse::stream(stream);

        let axum_response = handler_response.into_response();
        let body = axum_response.into_body().collect().await.unwrap();
        let bytes = body.to_bytes();

        assert_eq!(bytes.len(), 9);
        assert_eq!(bytes[0], 0x00);
        assert_eq!(bytes[4], 0x00);
        assert_eq!(bytes[8], 0xFD);
    }

    /// Test 28: Stream with maximum header count
    #[tokio::test]
    async fn test_stream_with_many_headers() {
        let chunks = vec![Ok::<_, Box<dyn std::error::Error + Send + Sync>>(Bytes::from("data"))];
        let stream = futures::stream::iter(chunks);

        let mut handler_response = HandlerResponse::stream(stream);

        for i in 0..50 {
            let header_name = format!("x-custom-{}", i);
            handler_response = handler_response.with_header(
                HeaderName::from_bytes(header_name.as_bytes()).unwrap(),
                HeaderValue::from_static("value"),
            );
        }

        let axum_response = handler_response.into_response();
        assert_eq!(axum_response.status(), StatusCode::OK);
        assert_eq!(axum_response.headers().len(), 50);
    }

    /// Test 29: Empty stream with 204 No Content status
    #[tokio::test]
    async fn test_empty_stream_with_204_no_content() {
        let chunks: Vec<Result<Bytes, Box<dyn std::error::Error + Send + Sync>>> = vec![];
        let stream = futures::stream::iter(chunks);

        let handler_response = HandlerResponse::stream(stream).with_status(StatusCode::NO_CONTENT);

        let axum_response = handler_response.into_response();

        assert_eq!(axum_response.status(), StatusCode::NO_CONTENT);
        let body = axum_response.into_body().collect().await.unwrap();
        assert!(body.to_bytes().is_empty());
    }

    /// Test 30: Stream with repeated header replacement
    #[tokio::test]
    async fn test_stream_repeated_header_updates() {
        let chunks = vec![Ok::<_, Box<dyn std::error::Error + Send + Sync>>(Bytes::from("test"))];
        let stream = futures::stream::iter(chunks);

        let handler_response = HandlerResponse::stream(stream)
            .with_header(header::CONTENT_TYPE, HeaderValue::from_static("text/plain"))
            .with_header(header::CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .with_header(header::CONTENT_TYPE, HeaderValue::from_static("application/xml"));

        match handler_response {
            HandlerResponse::Stream { headers, .. } => {
                assert_eq!(headers.get(header::CONTENT_TYPE).unwrap(), "application/xml");
            }
            HandlerResponse::Response(_) => panic!("Expected Stream variant"),
        }
    }

    /// Test 31: Stream with extremely long chunk
    #[tokio::test]
    async fn test_stream_with_extremely_long_chunk() {
        let large_chunk = "x".repeat(10_000_000);
        let chunks = vec![Ok::<_, Box<dyn std::error::Error + Send + Sync>>(Bytes::from(
            large_chunk,
        ))];
        let stream = futures::stream::iter(chunks);

        let handler_response = HandlerResponse::stream(stream);

        let axum_response = handler_response.into_response();
        let body = axum_response.into_body().collect().await.unwrap();
        let bytes = body.to_bytes();

        assert_eq!(bytes.len(), 10_000_000);
    }

    /// Test 32: Stream with zero-length chunks mixed in
    #[tokio::test]
    async fn test_stream_with_zero_length_chunks() {
        let chunks: Vec<Result<Bytes, Box<dyn std::error::Error + Send + Sync>>> = vec![
            Ok(Bytes::from("hello")),
            Ok(Bytes::new()),
            Ok(Bytes::from("world")),
            Ok(Bytes::new()),
            Ok(Bytes::from("!")),
        ];

        let stream = futures::stream::iter(chunks);
        let handler_response = HandlerResponse::stream(stream);

        let axum_response = handler_response.into_response();
        let body = axum_response.into_body().collect().await.unwrap();
        let bytes = body.to_bytes();

        assert_eq!(bytes, "helloworld!");
    }

    /// Test 33: Handler response variant preserves custom status on failure
    #[test]
    fn test_response_variant_preserves_original_status() {
        let axum_response = AxumResponse::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("error"))
            .unwrap();

        let handler_response = HandlerResponse::from(axum_response);

        let result = handler_response
            .with_status(StatusCode::OK)
            .with_status(StatusCode::INTERNAL_SERVER_ERROR);

        match result {
            HandlerResponse::Response(resp) => {
                assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
            }
            HandlerResponse::Stream { .. } => panic!("Expected Response variant"),
        }
    }

    /// Test 34: Stream response conversion preserves header ordering
    #[tokio::test]
    async fn test_stream_into_response_preserves_headers() {
        let chunks = vec![Ok::<_, Box<dyn std::error::Error + Send + Sync>>(Bytes::from("data"))];
        let stream = futures::stream::iter(chunks);

        let handler_response = HandlerResponse::stream(stream)
            .with_header(header::CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .with_header(header::CACHE_CONTROL, HeaderValue::from_static("max-age=3600"))
            .with_header(header::ETAG, HeaderValue::from_static("\"abc123\""));

        let axum_response = handler_response.into_response();

        assert!(axum_response.headers().get(header::CONTENT_TYPE).is_some());
        assert!(axum_response.headers().get(header::CACHE_CONTROL).is_some());
        assert!(axum_response.headers().get(header::ETAG).is_some());
        assert_eq!(axum_response.headers().len(), 3);
    }

    /// Test 35: Stream with 5xx status codes
    #[tokio::test]
    async fn test_stream_with_error_status_codes() {
        let error_statuses = vec![
            StatusCode::INTERNAL_SERVER_ERROR,
            StatusCode::SERVICE_UNAVAILABLE,
            StatusCode::GATEWAY_TIMEOUT,
        ];

        for status in error_statuses {
            let chunks = vec![Ok::<_, Box<dyn std::error::Error + Send + Sync>>(Bytes::from("error"))];
            let stream = futures::stream::iter(chunks);
            let handler_response = HandlerResponse::stream(stream).with_status(status);

            match handler_response {
                HandlerResponse::Stream { status: s, .. } => {
                    assert_eq!(s, status);
                }
                HandlerResponse::Response(_) => panic!("Expected Stream variant"),
            }
        }
    }
}

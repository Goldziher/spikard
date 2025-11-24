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

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
pub enum HandlerResponse {
    Response(AxumResponse<Body>),
    Stream {
        stream: Pin<Box<dyn Stream<Item = Result<Bytes, BoxError>> + Send + 'static>>,
        status: StatusCode,
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
    pub fn with_status(mut self, status: StatusCode) -> Self {
        if let HandlerResponse::Stream { status: s, .. } = &mut self {
            *s = status;
        }
        self
    }

    /// Insert or replace a header on the streaming response.
    pub fn with_header(mut self, name: HeaderName, value: HeaderValue) -> Self {
        if let HandlerResponse::Stream { headers, .. } = &mut self {
            headers.insert(name, value);
        }
        self
    }
}

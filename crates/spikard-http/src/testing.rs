use axum::body::Body;
use axum::http::Request as AxumRequest;
use axum_test::{TestResponse as AxumTestResponse, TestServer, TestWebSocket, WsMessage};

pub mod multipart;
pub use multipart::{MultipartFilePart, build_multipart_body};

pub mod form;

pub mod test_client;
pub use test_client::TestClient;

use brotli::Decompressor;
use flate2::read::GzDecoder;
pub use form::encode_urlencoded_body;
use http_body_util::BodyExt;
use serde_json::Value;
use std::collections::HashMap;
use std::io::{Cursor, Read};

/// Snapshot of an Axum response used by higher-level language bindings.
#[derive(Debug, Clone)]
pub struct ResponseSnapshot {
    /// HTTP status code.
    pub status: u16,
    /// Response headers (lowercase keys for predictable lookups).
    pub headers: HashMap<String, String>,
    /// Response body bytes (decoded for supported encodings).
    pub body: Vec<u8>,
}

impl ResponseSnapshot {
    /// Return response body as UTF-8 string.
    pub fn text(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.body.clone())
    }

    /// Parse response body as JSON.
    pub fn json(&self) -> Result<Value, serde_json::Error> {
        serde_json::from_slice(&self.body)
    }

    /// Lookup header by case-insensitive name.
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.get(&name.to_ascii_lowercase()).map(|s| s.as_str())
    }

    /// Extract GraphQL data from response
    pub fn graphql_data(&self) -> Result<Value, SnapshotError> {
        let body: Value = serde_json::from_slice(&self.body)
            .map_err(|e| SnapshotError::Decompression(format!("Failed to parse JSON: {}", e)))?;

        body.get("data")
            .cloned()
            .ok_or_else(|| SnapshotError::Decompression("No 'data' field in GraphQL response".to_string()))
    }

    /// Extract GraphQL errors from response
    pub fn graphql_errors(&self) -> Result<Vec<Value>, SnapshotError> {
        let body: Value = serde_json::from_slice(&self.body)
            .map_err(|e| SnapshotError::Decompression(format!("Failed to parse JSON: {}", e)))?;

        Ok(body
            .get("errors")
            .and_then(|e| e.as_array())
            .cloned()
            .unwrap_or_default())
    }
}

/// Possible errors while converting an Axum response into a snapshot.
#[derive(Debug)]
pub enum SnapshotError {
    /// Response header could not be decoded to UTF-8.
    InvalidHeader(String),
    /// Body decompression failed.
    Decompression(String),
}

impl std::fmt::Display for SnapshotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnapshotError::InvalidHeader(msg) => write!(f, "Invalid header: {}", msg),
            SnapshotError::Decompression(msg) => write!(f, "Failed to decode body: {}", msg),
        }
    }
}

impl std::error::Error for SnapshotError {}

/// Execute an HTTP request against an Axum [`TestServer`] by rehydrating it
/// into the server's own [`axum_test::TestRequest`] builder.
pub async fn call_test_server(server: &TestServer, request: AxumRequest<Body>) -> AxumTestResponse {
    let (parts, body) = request.into_parts();

    let mut path = parts.uri.path().to_string();
    if let Some(query) = parts.uri.query()
        && !query.is_empty()
    {
        path.push('?');
        path.push_str(query);
    }

    let mut test_request = server.method(parts.method.clone(), &path);

    for (name, value) in parts.headers.iter() {
        test_request = test_request.add_header(name.clone(), value.clone());
    }

    let collected = body
        .collect()
        .await
        .expect("failed to read request body for test dispatch");
    let bytes = collected.to_bytes();
    if !bytes.is_empty() {
        test_request = test_request.bytes(bytes);
    }

    test_request.await
}

/// Convert an `AxumTestResponse` into a reusable [`ResponseSnapshot`].
pub async fn snapshot_response(response: AxumTestResponse) -> Result<ResponseSnapshot, SnapshotError> {
    let status = response.status_code().as_u16();

    let mut headers = HashMap::new();
    for (name, value) in response.headers() {
        let header_value = value
            .to_str()
            .map_err(|e| SnapshotError::InvalidHeader(e.to_string()))?;
        headers.insert(name.to_string().to_ascii_lowercase(), header_value.to_string());
    }

    let body = response.into_bytes();
    let decoded_body = decode_body(&headers, body.to_vec())?;

    Ok(ResponseSnapshot {
        status,
        headers,
        body: decoded_body,
    })
}

/// Convert an Axum response into a reusable [`ResponseSnapshot`].
pub async fn snapshot_http_response(
    response: axum::response::Response<Body>,
) -> Result<ResponseSnapshot, SnapshotError> {
    let (parts, body) = response.into_parts();
    let status = parts.status.as_u16();

    let mut headers = HashMap::new();
    for (name, value) in parts.headers.iter() {
        let header_value = value
            .to_str()
            .map_err(|e| SnapshotError::InvalidHeader(e.to_string()))?;
        headers.insert(name.to_string().to_ascii_lowercase(), header_value.to_string());
    }

    let collected = body
        .collect()
        .await
        .map_err(|e| SnapshotError::Decompression(e.to_string()))?;
    let bytes = collected.to_bytes();
    let decoded_body = decode_body(&headers, bytes.to_vec())?;

    Ok(ResponseSnapshot {
        status,
        headers,
        body: decoded_body,
    })
}

fn decode_body(headers: &HashMap<String, String>, body: Vec<u8>) -> Result<Vec<u8>, SnapshotError> {
    let encoding = headers
        .get("content-encoding")
        .map(|value| value.trim().to_ascii_lowercase());

    match encoding.as_deref() {
        Some("gzip" | "x-gzip") => decode_gzip(body),
        Some("br") => decode_brotli(body),
        _ => Ok(body),
    }
}

fn decode_gzip(body: Vec<u8>) -> Result<Vec<u8>, SnapshotError> {
    let mut decoder = GzDecoder::new(Cursor::new(body));
    let mut decoded_bytes = Vec::new();
    decoder
        .read_to_end(&mut decoded_bytes)
        .map_err(|e| SnapshotError::Decompression(e.to_string()))?;
    Ok(decoded_bytes)
}

fn decode_brotli(body: Vec<u8>) -> Result<Vec<u8>, SnapshotError> {
    let mut decoder = Decompressor::new(Cursor::new(body), 4096);
    let mut decoded_bytes = Vec::new();
    decoder
        .read_to_end(&mut decoded_bytes)
        .map_err(|e| SnapshotError::Decompression(e.to_string()))?;
    Ok(decoded_bytes)
}

/// WebSocket connection wrapper for testing.
///
/// Provides a simple interface for sending and receiving WebSocket messages
/// during tests without needing a real network connection.
pub struct WebSocketConnection {
    inner: TestWebSocket,
}

impl WebSocketConnection {
    /// Create a new WebSocket connection from an axum-test TestWebSocket.
    pub fn new(inner: TestWebSocket) -> Self {
        Self { inner }
    }

    /// Send a text message over the WebSocket.
    pub async fn send_text(&mut self, text: impl std::fmt::Display) {
        self.inner.send_text(text).await;
    }

    /// Send a JSON message over the WebSocket.
    pub async fn send_json<T: serde::Serialize>(&mut self, value: &T) {
        self.inner.send_json(value).await;
    }

    /// Send a raw WebSocket message.
    pub async fn send_message(&mut self, msg: WsMessage) {
        self.inner.send_message(msg).await;
    }

    /// Receive the next text message from the WebSocket.
    pub async fn receive_text(&mut self) -> String {
        self.inner.receive_text().await
    }

    /// Receive and parse a JSON message from the WebSocket.
    pub async fn receive_json<T: serde::de::DeserializeOwned>(&mut self) -> T {
        self.inner.receive_json().await
    }

    /// Receive raw bytes from the WebSocket.
    pub async fn receive_bytes(&mut self) -> bytes::Bytes {
        self.inner.receive_bytes().await
    }

    /// Receive the next raw message from the WebSocket.
    pub async fn receive_message(&mut self) -> WebSocketMessage {
        let msg = self.inner.receive_message().await;
        WebSocketMessage::from_ws_message(msg)
    }

    /// Close the WebSocket connection.
    pub async fn close(self) {
        self.inner.close().await;
    }
}

/// A WebSocket message that can be text or binary.
#[derive(Debug, Clone)]
pub enum WebSocketMessage {
    /// A text message.
    Text(String),
    /// A binary message.
    Binary(Vec<u8>),
    /// A close message.
    Close(Option<String>),
    /// A ping message.
    Ping(Vec<u8>),
    /// A pong message.
    Pong(Vec<u8>),
}

impl WebSocketMessage {
    fn from_ws_message(msg: WsMessage) -> Self {
        match msg {
            WsMessage::Text(text) => WebSocketMessage::Text(text.to_string()),
            WsMessage::Binary(data) => WebSocketMessage::Binary(data.to_vec()),
            WsMessage::Close(frame) => WebSocketMessage::Close(frame.map(|f| f.reason.to_string())),
            WsMessage::Ping(data) => WebSocketMessage::Ping(data.to_vec()),
            WsMessage::Pong(data) => WebSocketMessage::Pong(data.to_vec()),
            WsMessage::Frame(_) => WebSocketMessage::Close(None),
        }
    }

    /// Get the message as text, if it's a text message.
    pub fn as_text(&self) -> Option<&str> {
        match self {
            WebSocketMessage::Text(text) => Some(text),
            _ => None,
        }
    }

    /// Get the message as JSON, if it's a text message containing JSON.
    pub fn as_json(&self) -> Result<Value, String> {
        match self {
            WebSocketMessage::Text(text) => {
                serde_json::from_str(text).map_err(|e| format!("Failed to parse JSON: {}", e))
            }
            _ => Err("Message is not text".to_string()),
        }
    }

    /// Get the message as binary, if it's a binary message.
    pub fn as_binary(&self) -> Option<&[u8]> {
        match self {
            WebSocketMessage::Binary(data) => Some(data),
            _ => None,
        }
    }

    /// Check if this is a close message.
    pub fn is_close(&self) -> bool {
        matches!(self, WebSocketMessage::Close(_))
    }
}

/// Connect to a WebSocket endpoint on the test server.
pub async fn connect_websocket(server: &TestServer, path: &str) -> WebSocketConnection {
    let ws = server.get_websocket(path).await.into_websocket().await;
    WebSocketConnection::new(ws)
}

/// Server-Sent Events (SSE) stream for testing.
///
/// Wraps a response body and provides methods to parse SSE events.
#[derive(Debug)]
pub struct SseStream {
    body: String,
    events: Vec<SseEvent>,
}

impl SseStream {
    /// Create a new SSE stream from a response.
    pub fn from_response(response: &ResponseSnapshot) -> Result<Self, String> {
        let body = response
            .text()
            .map_err(|e| format!("Failed to read response body: {}", e))?;

        let events = Self::parse_events(&body);

        Ok(Self { body, events })
    }

    fn parse_events(body: &str) -> Vec<SseEvent> {
        let mut events = Vec::new();
        let lines: Vec<&str> = body.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            if lines[i].starts_with("data:") {
                let data = lines[i].trim_start_matches("data:").trim().to_string();
                events.push(SseEvent { data });
            } else if lines[i].starts_with("data") {
                let data = lines[i].trim_start_matches("data").trim().to_string();
                if !data.is_empty() || lines[i].len() == 4 {
                    events.push(SseEvent { data });
                }
            }
            i += 1;
        }

        events
    }

    /// Get all events from the stream.
    pub fn events(&self) -> &[SseEvent] {
        &self.events
    }

    /// Get the raw body of the SSE response.
    pub fn body(&self) -> &str {
        &self.body
    }

    /// Get events as JSON values.
    pub fn events_as_json(&self) -> Result<Vec<Value>, String> {
        self.events
            .iter()
            .map(|event| event.as_json())
            .collect::<Result<Vec<_>, _>>()
    }
}

/// A single Server-Sent Event.
#[derive(Debug, Clone)]
pub struct SseEvent {
    /// The data field of the event.
    pub data: String,
}

impl SseEvent {
    /// Parse the event data as JSON.
    pub fn as_json(&self) -> Result<Value, String> {
        serde_json::from_str(&self.data).map_err(|e| format!("Failed to parse JSON: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::response::Response;
    use std::io::Write;

    #[test]
    fn sse_stream_parses_multiple_events() {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "text/event-stream".to_string());

        let snapshot = ResponseSnapshot {
            status: 200,
            headers,
            body: b"data: {\"id\": 1}\n\ndata: \"hello\"\n\n".to_vec(),
        };

        let stream = SseStream::from_response(&snapshot).expect("stream");
        assert_eq!(stream.events().len(), 2);
        assert_eq!(stream.events()[0].as_json().unwrap()["id"], serde_json::json!(1));
        assert_eq!(stream.events()[1].data, "\"hello\"");
        assert_eq!(stream.events_as_json().unwrap().len(), 2);
    }

    #[test]
    fn sse_event_reports_invalid_json() {
        let event = SseEvent {
            data: "not-json".to_string(),
        };
        assert!(event.as_json().is_err());
    }

    #[test]
    fn test_graphql_data_extraction() {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        let graphql_response = serde_json::json!({
            "data": {
                "user": {
                    "id": "1",
                    "name": "Alice"
                }
            }
        });

        let snapshot = ResponseSnapshot {
            status: 200,
            headers,
            body: serde_json::to_vec(&graphql_response).unwrap(),
        };

        let data = snapshot.graphql_data().expect("data extraction");
        assert_eq!(data["user"]["id"], "1");
        assert_eq!(data["user"]["name"], "Alice");
    }

    #[test]
    fn test_graphql_errors_extraction() {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        let graphql_response = serde_json::json!({
            "errors": [
                {
                    "message": "Field not found",
                    "path": ["user", "email"]
                },
                {
                    "message": "Unauthorized",
                    "extensions": { "code": "UNAUTHENTICATED" }
                }
            ]
        });

        let snapshot = ResponseSnapshot {
            status: 400,
            headers,
            body: serde_json::to_vec(&graphql_response).unwrap(),
        };

        let errors = snapshot.graphql_errors().expect("errors extraction");
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0]["message"], "Field not found");
        assert_eq!(errors[1]["message"], "Unauthorized");
    }

    #[test]
    fn test_graphql_missing_data_field() {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        let graphql_response = serde_json::json!({
            "errors": [{ "message": "Query failed" }]
        });

        let snapshot = ResponseSnapshot {
            status: 400,
            headers,
            body: serde_json::to_vec(&graphql_response).unwrap(),
        };

        let result = snapshot.graphql_data();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("No 'data' field"));
    }

    #[test]
    fn test_graphql_empty_errors() {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        let graphql_response = serde_json::json!({
            "data": { "result": null }
        });

        let snapshot = ResponseSnapshot {
            status: 200,
            headers,
            body: serde_json::to_vec(&graphql_response).unwrap(),
        };

        let errors = snapshot.graphql_errors().expect("errors extraction");
        assert!(errors.is_empty());
    }

    fn gzip_bytes(input: &[u8]) -> Vec<u8> {
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(input).expect("gzip write");
        encoder.finish().expect("gzip finish")
    }

    fn brotli_bytes(input: &[u8]) -> Vec<u8> {
        let mut encoder = brotli::CompressorWriter::new(Vec::new(), 4096, 5, 22);
        encoder.write_all(input).expect("brotli write");
        encoder.into_inner()
    }

    #[tokio::test]
    async fn snapshot_http_response_decodes_gzip_body() {
        let body = b"hello gzip";
        let compressed = gzip_bytes(body);
        let response = Response::builder()
            .status(200)
            .header("content-encoding", "gzip")
            .body(Body::from(compressed))
            .unwrap();

        let snapshot = snapshot_http_response(response).await.expect("snapshot");
        assert_eq!(snapshot.body, body);
    }

    #[tokio::test]
    async fn snapshot_http_response_decodes_brotli_body() {
        let body = b"hello brotli";
        let compressed = brotli_bytes(body);
        let response = Response::builder()
            .status(200)
            .header("content-encoding", "br")
            .body(Body::from(compressed))
            .unwrap();

        let snapshot = snapshot_http_response(response).await.expect("snapshot");
        assert_eq!(snapshot.body, body);
    }

    #[tokio::test]
    async fn snapshot_http_response_leaves_plain_body() {
        let body = b"plain";
        let response = Response::builder()
            .status(200)
            .body(Body::from(body.as_slice()))
            .unwrap();

        let snapshot = snapshot_http_response(response).await.expect("snapshot");
        assert_eq!(snapshot.body, body);
    }
}

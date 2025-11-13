use axum::body::Body;
use axum::http::Request as AxumRequest;
use axum_test::{TestResponse as AxumTestResponse, TestServer};
use brotli::Decompressor;
use flate2::read::GzDecoder;
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

fn decode_body(headers: &HashMap<String, String>, body: Vec<u8>) -> Result<Vec<u8>, SnapshotError> {
    let encoding = headers
        .get("content-encoding")
        .map(|value| value.trim().to_ascii_lowercase());

    match encoding.as_deref() {
        Some("gzip") | Some("x-gzip") => decode_gzip(body),
        Some("br") => decode_brotli(body),
        _ => Ok(body),
    }
}

fn decode_gzip(body: Vec<u8>) -> Result<Vec<u8>, SnapshotError> {
    let mut decoder = GzDecoder::new(Cursor::new(body));
    let mut decoded = Vec::new();
    decoder
        .read_to_end(&mut decoded)
        .map_err(|e| SnapshotError::Decompression(e.to_string()))?;
    Ok(decoded)
}

fn decode_brotli(body: Vec<u8>) -> Result<Vec<u8>, SnapshotError> {
    let mut decoder = Decompressor::new(Cursor::new(body), 4096);
    let mut decoded = Vec::new();
    decoder
        .read_to_end(&mut decoded)
        .map_err(|e| SnapshotError::Decompression(e.to_string()))?;
    Ok(decoded)
}

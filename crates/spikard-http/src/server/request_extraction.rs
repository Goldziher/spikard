//! Request parsing and data extraction utilities

use crate::handler_trait::RequestData;
use crate::query_parser::parse_query_string_to_json;
use axum::body::Body;
use http_body_util::BodyExt;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// Extract and parse query parameters from request URI
pub fn extract_query_params(uri: &axum::http::Uri) -> Value {
    let query_string = uri.query().unwrap_or("");
    if query_string.is_empty() {
        Value::Object(serde_json::Map::new())
    } else {
        parse_query_string_to_json(query_string.as_bytes(), true)
    }
}

/// Extract raw query parameters as strings (no type conversion)
/// Used for validation error messages to show the actual input values
pub fn extract_raw_query_params(uri: &axum::http::Uri) -> HashMap<String, Vec<String>> {
    let query_string = uri.query().unwrap_or("");
    if query_string.is_empty() {
        HashMap::new()
    } else {
        crate::query_parser::parse_query_string(query_string.as_bytes(), '&')
            .into_iter()
            .fold(HashMap::new(), |mut acc, (k, v)| {
                acc.entry(k).or_insert_with(Vec::new).push(v);
                acc
            })
    }
}

/// Extract headers from request
pub fn extract_headers(headers: &axum::http::HeaderMap) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for (name, value) in headers.iter() {
        if let Ok(val_str) = value.to_str() {
            map.insert(name.as_str().to_lowercase(), val_str.to_string());
        }
    }
    map
}

/// Extract cookies from request headers
pub fn extract_cookies(headers: &axum::http::HeaderMap) -> HashMap<String, String> {
    let mut cookies = HashMap::new();

    if let Some(cookie_str) = headers.get(axum::http::header::COOKIE).and_then(|h| h.to_str().ok()) {
        for cookie in cookie::Cookie::split_parse(cookie_str).flatten() {
            cookies.insert(cookie.name().to_string(), cookie.value().to_string());
        }
    }

    cookies
}

/// Create RequestData from request parts (for requests without body)
///
/// Wraps HashMaps in Arc to enable cheap cloning without duplicating data.
pub fn create_request_data_without_body(
    uri: &axum::http::Uri,
    method: &axum::http::Method,
    headers: &axum::http::HeaderMap,
    path_params: HashMap<String, String>,
) -> RequestData {
    RequestData {
        path_params: Arc::new(path_params),
        query_params: extract_query_params(uri),
        raw_query_params: Arc::new(extract_raw_query_params(uri)),
        headers: Arc::new(extract_headers(headers)),
        cookies: Arc::new(extract_cookies(headers)),
        body: Value::Null,
        raw_body: None,
        method: method.as_str().to_string(),
        path: uri.path().to_string(),
        #[cfg(feature = "di")]
        dependencies: None,
    }
}

/// Create RequestData from request parts (for requests with body)
///
/// Wraps HashMaps in Arc to enable cheap cloning without duplicating data.
/// Performance optimization: stores raw body bytes without parsing JSON.
/// JSON parsing is deferred until actually needed (e.g., for validation).
pub async fn create_request_data_with_body(
    parts: &axum::http::request::Parts,
    path_params: HashMap<String, String>,
    body: Body,
) -> Result<RequestData, (axum::http::StatusCode, String)> {
    let body_bytes = body
        .collect()
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::BAD_REQUEST,
                format!("Failed to read body: {}", e),
            )
        })?
        .to_bytes();

    Ok(RequestData {
        path_params: Arc::new(path_params),
        query_params: extract_query_params(&parts.uri),
        raw_query_params: Arc::new(extract_raw_query_params(&parts.uri)),
        headers: Arc::new(extract_headers(&parts.headers)),
        cookies: Arc::new(extract_cookies(&parts.headers)),
        body: Value::Null,
        raw_body: if body_bytes.is_empty() { None } else { Some(body_bytes) },
        method: parts.method.as_str().to_string(),
        path: parts.uri.path().to_string(),
        #[cfg(feature = "di")]
        dependencies: None,
    })
}

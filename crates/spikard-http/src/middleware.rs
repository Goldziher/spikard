//! HTTP middleware for request validation

use crate::problem::{CONTENT_TYPE_PROBLEM_JSON, ProblemDetails};
use axum::{
    body::Body,
    extract::{FromRequest, Multipart, Request},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;

/// Route information for middleware validation
#[derive(Debug, Clone)]
pub struct RouteInfo {
    /// Whether this route expects a JSON request body
    pub expects_json_body: bool,
}

/// Registry of route metadata indexed by (method, path)
pub type RouteRegistry = Arc<HashMap<(String, String), RouteInfo>>;

/// Validate that Content-Type is JSON-compatible when route expects JSON
#[allow(clippy::result_large_err)]
fn validate_json_content_type(headers: &HeaderMap) -> Result<(), Response> {
    if let Some(content_type_header) = headers.get(axum::http::header::CONTENT_TYPE)
        && let Ok(content_type_str) = content_type_header.to_str()
    {
        if let Ok(parsed_mime) = content_type_str.parse::<mime::Mime>() {
            let is_json = (parsed_mime.type_() == mime::APPLICATION && parsed_mime.subtype() == mime::JSON)
                || parsed_mime.suffix() == Some(mime::JSON);

            let is_form = (parsed_mime.type_() == mime::APPLICATION
                && parsed_mime.subtype() == "x-www-form-urlencoded")
                || (parsed_mime.type_() == mime::MULTIPART && parsed_mime.subtype() == "form-data");

            if !is_json && !is_form {
                let problem = ProblemDetails::new(
                    "https://spikard.dev/errors/unsupported-media-type",
                    "Unsupported Media Type",
                    StatusCode::UNSUPPORTED_MEDIA_TYPE,
                )
                .with_detail("Unsupported media type");

                let body = problem.to_json().unwrap_or_else(|_| "{}".to_string());
                return Err((
                    StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    [(axum::http::header::CONTENT_TYPE, CONTENT_TYPE_PROBLEM_JSON)],
                    body,
                )
                    .into_response());
            }
        }
    }
    Ok(())
}

/// Validate Content-Length header matches actual body size
#[allow(clippy::result_large_err, clippy::collapsible_if)]
fn validate_content_length(headers: &HeaderMap, actual_size: usize) -> Result<(), Response> {
    if let Some(content_length_header) = headers.get(axum::http::header::CONTENT_LENGTH) {
        if let Ok(content_length_str) = content_length_header.to_str() {
            if let Ok(declared_length) = content_length_str.parse::<usize>() {
                if declared_length != actual_size {
                    let problem = ProblemDetails::bad_request(format!(
                        "Content-Length header ({}) does not match actual body size ({})",
                        declared_length, actual_size
                    ));

                    let body = problem.to_json().unwrap_or_else(|_| "{}".to_string());
                    return Err((
                        StatusCode::BAD_REQUEST,
                        [(axum::http::header::CONTENT_TYPE, CONTENT_TYPE_PROBLEM_JSON)],
                        body,
                    )
                        .into_response());
                }
            }
        }
    }
    Ok(())
}

/// Middleware to validate Content-Type headers and related requirements
pub async fn validate_content_type_middleware(request: Request, next: Next) -> Result<Response, Response> {
    use axum::body::to_bytes;
    use axum::http::Request as HttpRequest;

    let (parts, body) = request.into_parts();
    let headers = &parts.headers;

    let route_info = parts.extensions.get::<RouteRegistry>().and_then(|registry| {
        let method = parts.method.as_str();
        let path = parts.uri.path();
        registry.get(&(method.to_string(), path.to_string())).cloned()
    });

    let method = &parts.method;
    if method == axum::http::Method::POST || method == axum::http::Method::PUT || method == axum::http::Method::PATCH {
        if let Some(info) = &route_info
            && info.expects_json_body
        {
            validate_json_content_type(headers)?;
        }

        validate_content_type_headers(headers, 0)?;

        let (final_parts, final_body) = if let Some(content_type) = headers.get(axum::http::header::CONTENT_TYPE) {
            if let Ok(content_type_str) = content_type.to_str() {
                let parsed_mime = content_type_str.parse::<mime::Mime>().ok();

                let is_multipart = parsed_mime
                    .as_ref()
                    .map(|mime| mime.type_() == mime::MULTIPART && mime.subtype() == "form-data")
                    .unwrap_or(false);

                let is_form_urlencoded = parsed_mime
                    .as_ref()
                    .map(|mime| mime.type_() == mime::APPLICATION && mime.subtype() == "x-www-form-urlencoded")
                    .unwrap_or(false);

                if is_multipart {
                    let mut response_headers = parts.headers.clone();

                    let request = HttpRequest::from_parts(parts, body);
                    let multipart = match Multipart::from_request(request, &()).await {
                        Ok(mp) => mp,
                        Err(e) => {
                            let error_body = json!({
                                "error": format!("Failed to parse multipart data: {}", e)
                            });
                            return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
                        }
                    };

                    let json_body = match parse_multipart_to_json(multipart).await {
                        Ok(json) => json,
                        Err(e) => {
                            let error_body = json!({
                                "error": format!("Failed to process multipart data: {}", e)
                            });
                            return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
                        }
                    };

                    let json_bytes = serde_json::to_vec(&json_body).unwrap();

                    response_headers.insert(
                        axum::http::header::CONTENT_TYPE,
                        axum::http::HeaderValue::from_static("application/json"),
                    );

                    let mut new_request = axum::http::Request::new(Body::from(json_bytes));
                    *new_request.headers_mut() = response_headers;

                    return Ok(next.run(new_request).await);
                } else if is_form_urlencoded {
                    let body_bytes = match to_bytes(body, usize::MAX).await {
                        Ok(bytes) => bytes,
                        Err(_) => {
                            let error_body = json!({
                                "error": "Failed to read request body"
                            });
                            return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
                        }
                    };

                    validate_content_length(headers, body_bytes.len())?;

                    let json_body = if body_bytes.is_empty() {
                        serde_json::json!({})
                    } else {
                        match parse_urlencoded_to_json(&body_bytes) {
                            Ok(json_body) => json_body,
                            Err(e) => {
                                let error_body = json!({
                                    "error": format!("Failed to parse URL-encoded form data: {}", e)
                                });
                                return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
                            }
                        }
                    };

                    let json_bytes = serde_json::to_vec(&json_body).unwrap();

                    let mut new_parts = parts;
                    new_parts.headers.insert(
                        axum::http::header::CONTENT_TYPE,
                        axum::http::HeaderValue::from_static("application/json"),
                    );

                    (new_parts, Body::from(json_bytes))
                } else {
                    let body_bytes = match to_bytes(body, usize::MAX).await {
                        Ok(bytes) => bytes,
                        Err(_) => {
                            let error_body = json!({
                                "error": "Failed to read request body"
                            });
                            return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
                        }
                    };

                    validate_content_length(headers, body_bytes.len())?;

                    let is_json = parsed_mime.as_ref().map(is_json_content_type).unwrap_or(false);

                    if is_json
                        && !body_bytes.is_empty()
                        && serde_json::from_slice::<serde_json::Value>(&body_bytes).is_err()
                    {
                        let error_body = json!({
                            "detail": "Invalid request format"
                        });
                        return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
                    }

                    (parts, Body::from(body_bytes))
                }
            } else {
                let body_bytes = match to_bytes(body, usize::MAX).await {
                    Ok(bytes) => bytes,
                    Err(_) => {
                        let error_body = json!({
                            "error": "Failed to read request body"
                        });
                        return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
                    }
                };

                validate_content_length(headers, body_bytes.len())?;

                (parts, Body::from(body_bytes))
            }
        } else {
            let body_bytes = match to_bytes(body, usize::MAX).await {
                Ok(bytes) => bytes,
                Err(_) => {
                    let error_body = json!({
                        "error": "Failed to read request body"
                    });
                    return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
                }
            };

            validate_content_length(headers, body_bytes.len())?;

            (parts, Body::from(body_bytes))
        };

        let request = HttpRequest::from_parts(final_parts, final_body);
        Ok(next.run(request).await)
    } else {
        validate_content_type_headers(headers, 0)?;

        let request = HttpRequest::from_parts(parts, body);
        Ok(next.run(request).await)
    }
}

/// Parse URL-encoded form data to JSON
///
/// This handles:
/// - Array notation: tags[]=value1&tags[]=value2 → {"tags": ["value1", "value2"]}
/// - Nested objects: profile[name]=John → {"profile": {"name": "John"}}
/// - Type conversion: age=30 → {"age": 30}, active=true → {"active": true}
/// - Multiple values: tags=a&tags=b → {"tags": ["a", "b"]}
/// - Empty strings: Preserved as empty strings (unlike query parameter parsing)
///
/// Strategy:
/// - If brackets present → use serde_qs (handles nested objects, arrays with [])
/// - Otherwise → use custom parser that preserves empty strings and handles duplicate keys
fn parse_urlencoded_to_json(data: &[u8]) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    use std::collections::HashMap;
    let body_str = std::str::from_utf8(data)?;

    if body_str.contains('[') {
        let config = serde_qs::Config::new(10, false);
        let parsed: HashMap<String, serde_json::Value> = config.deserialize_str(body_str)?;
        let mut json_value = serde_json::to_value(parsed)?;
        convert_types_recursive(&mut json_value);
        Ok(json_value)
    } else {
        Ok(parse_urlencoded_simple(data))
    }
}

/// Parse simple URL-encoded data (no brackets) while preserving empty strings
fn parse_urlencoded_simple(data: &[u8]) -> serde_json::Value {
    use rustc_hash::FxHashMap;
    use urlencoding::decode;

    let mut array_map: FxHashMap<String, Vec<serde_json::Value>> = FxHashMap::default();

    let body_str = String::from_utf8_lossy(data);
    let body_str = body_str.replace('+', " ");

    for pair in body_str.split('&') {
        if pair.is_empty() {
            continue;
        }

        let (key, value) = if let Some((k, v)) = pair.split_once('=') {
            (
                decode(k).unwrap_or_default().to_string(),
                decode(v).unwrap_or_default().to_string(),
            )
        } else {
            (pair.to_string(), String::new())
        };

        let json_value = convert_string_to_json_value(&value);

        match array_map.get_mut(&key) {
            Some(entry) => {
                entry.push(json_value);
            }
            None => {
                array_map.insert(key, vec![json_value]);
            }
        }
    }

    array_map
        .iter()
        .map(|(key, value)| {
            if value.len() == 1 {
                (key, value[0].clone())
            } else {
                (key, serde_json::Value::Array(value.clone()))
            }
        })
        .collect::<serde_json::Value>()
}

/// Try to parse a string as an integer
fn try_parse_integer(s: &str) -> Option<serde_json::Value> {
    s.parse::<i64>().ok().map(|i| serde_json::Value::Number(i.into()))
}

/// Try to parse a string as a float
fn try_parse_float(s: &str) -> Option<serde_json::Value> {
    s.parse::<f64>()
        .ok()
        .and_then(|f| serde_json::Number::from_f64(f).map(serde_json::Value::Number))
}

/// Try to parse a string as a boolean (true/false, case-insensitive)
fn try_parse_boolean(s: &str) -> Option<serde_json::Value> {
    match s.to_lowercase().as_str() {
        "true" => Some(serde_json::Value::Bool(true)),
        "false" => Some(serde_json::Value::Bool(false)),
        _ => None,
    }
}

/// Convert a string value to appropriate JSON type while preserving empty strings
fn convert_string_to_json_value(s: &str) -> serde_json::Value {
    if s.is_empty() {
        return serde_json::Value::String(String::new());
    }

    try_parse_integer(s)
        .or_else(|| try_parse_float(s))
        .or_else(|| try_parse_boolean(s))
        .or_else(|| {
            if s == "null" {
                Some(serde_json::Value::Null)
            } else {
                None
            }
        })
        .unwrap_or_else(|| serde_json::Value::String(s.to_string()))
}

/// Recursively convert string values to appropriate types (numbers, booleans)
/// while preserving empty strings
fn convert_types_recursive(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::String(s) => {
            if s.is_empty() {
                return;
            }

            if let Some(parsed) = try_parse_integer(s)
                .or_else(|| try_parse_float(s))
                .or_else(|| try_parse_boolean(s))
            {
                *value = parsed;
            }
        }
        serde_json::Value::Array(arr) => {
            for item in arr.iter_mut() {
                convert_types_recursive(item);
            }
        }
        serde_json::Value::Object(obj) => {
            for (_, v) in obj.iter_mut() {
                convert_types_recursive(v);
            }
        }
        _ => {}
    }
}

/// Size threshold for streaming vs buffering multipart fields
/// Fields larger than this will be streamed chunk-by-chunk
const MULTIPART_STREAMING_THRESHOLD: usize = 1024 * 1024; 

/// Parse multipart/form-data to JSON
///
/// This handles:
/// - File uploads → {"filename": "...", "size": N, "content": "...", "content_type": "..."}
/// - Form fields → plain string values
/// - Mixed files and data → combined in single JSON object
/// - Large files → streamed chunk-by-chunk (async)
/// - Small files → buffered in memory
/// - Multiple values with same field name → aggregated into arrays
///
/// Streaming strategy:
/// - Files > 1MB: Use field.chunk().await for async streaming
/// - Files <= 1MB: Use field.bytes().await for buffered loading
async fn parse_multipart_to_json(
    mut multipart: Multipart,
) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    use rustc_hash::FxHashMap;

    let mut field_values: FxHashMap<String, Vec<serde_json::Value>> = FxHashMap::default();

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().ok_or("Field missing name")?.to_string();

        let field_value = if let Some(filename) = field.file_name() {
            let filename = filename.to_string();
            let content_type = field
                .content_type()
                .map(|ct| ct.to_string())
                .unwrap_or_else(|| "application/octet-stream".to_string());

            let bytes = field.bytes().await?;
            let size = bytes.len();

            let content = if content_type.starts_with("text/") || content_type == "application/json" {
                String::from_utf8_lossy(&bytes).to_string()
            } else if size <= MULTIPART_STREAMING_THRESHOLD {
                String::from_utf8_lossy(&bytes).to_string()
            } else {
                format!("<binary data, {} bytes>", size)
            };

            json!({
                "filename": filename,
                "size": size,
                "content": content,
                "content_type": content_type
            })
        } else {
            let value = field.text().await?;

            if (value.starts_with('[') && value.ends_with(']')) || (value.starts_with('{') && value.ends_with('}')) {
                if let Ok(parsed_json) = serde_json::from_str::<serde_json::Value>(&value) {
                    parsed_json
                } else {
                    json!(value)
                }
            } else {
                json!(value)
            }
        };

        field_values.entry(name).or_default().push(field_value);
    }

    let result: serde_json::Map<String, serde_json::Value> = field_values
        .into_iter()
        .map(|(key, values)| {
            if values.len() == 1 {
                (key, values.into_iter().next().unwrap())
            } else {
                (key, serde_json::Value::Array(values))
            }
        })
        .collect();

    Ok(json!(result))
}

/// Check if a media type is JSON or has a +json suffix
fn is_json_content_type(mime: &mime::Mime) -> bool {
    (mime.type_() == mime::APPLICATION && mime.subtype() == mime::JSON) || mime.suffix() == Some(mime::JSON)
}

/// Validate Content-Type header and related requirements
#[allow(clippy::result_large_err)]
fn validate_content_type_headers(headers: &HeaderMap, _declared_body_size: usize) -> Result<(), Response> {
    if let Some(content_type_str) = headers
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|h| h.to_str().ok())
    {
        let parsed_mime = match content_type_str.parse::<mime::Mime>() {
            Ok(m) => m,
            Err(_) => {
                let error_body = json!({
                    "error": format!("Invalid Content-Type header: {}", content_type_str)
                });
                return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
            }
        };

        let is_json = is_json_content_type(&parsed_mime);
        let is_multipart = parsed_mime.type_() == mime::MULTIPART && parsed_mime.subtype() == "form-data";

        if is_multipart && parsed_mime.get_param(mime::BOUNDARY).is_none() {
            let error_body = json!({
                "error": "multipart/form-data requires 'boundary' parameter"
            });
            return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
        }

        #[allow(clippy::collapsible_if)]
        if is_json {
            if let Some(charset) = parsed_mime.get_param(mime::CHARSET).map(|c| c.as_str()) {
                if !charset.eq_ignore_ascii_case("utf-8") && !charset.eq_ignore_ascii_case("utf8") {
                    let problem = ProblemDetails::new(
                        "https://spikard.dev/errors/unsupported-charset",
                        "Unsupported Charset",
                        StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    )
                    .with_detail(format!(
                        "Unsupported charset '{}' for JSON. Only UTF-8 is supported.",
                        charset
                    ));

                    let body = problem.to_json().unwrap_or_else(|_| "{}".to_string());
                    return Err((
                        StatusCode::UNSUPPORTED_MEDIA_TYPE,
                        [(axum::http::header::CONTENT_TYPE, CONTENT_TYPE_PROBLEM_JSON)],
                        body,
                    )
                        .into_response());
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn validate_content_length_accepts_matching_sizes() {
        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::CONTENT_LENGTH, HeaderValue::from_static("5"));

        assert!(validate_content_length(&headers, 5).is_ok());
    }

    #[test]
    fn validate_content_length_rejects_mismatched_sizes() {
        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::CONTENT_LENGTH, HeaderValue::from_static("10"));

        let err = validate_content_length(&headers, 4).expect_err("expected mismatch");
        assert_eq!(err.status(), StatusCode::BAD_REQUEST);
        assert_eq!(
            err.headers()
                .get(axum::http::header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok()),
            Some(CONTENT_TYPE_PROBLEM_JSON)
        );
    }

    #[test]
    fn test_multipart_without_boundary() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("multipart/form-data"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_multipart_with_boundary() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("multipart/form-data; boundary=----WebKitFormBoundary"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_with_utf16_charset() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-16"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_json_with_utf8_charset() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_without_charset() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_vendor_json_accepted() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/vnd.api+json"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_problem_json_accepted() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/problem+json"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_vendor_json_with_utf16_charset_rejected() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/vnd.api+json; charset=utf-16"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_vendor_json_with_utf8_charset_accepted() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/vnd.api+json; charset=utf-8"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_is_json_content_type() {
        let mime = "application/json".parse::<mime::Mime>().unwrap();
        assert!(is_json_content_type(&mime));

        let mime = "application/vnd.api+json".parse::<mime::Mime>().unwrap();
        assert!(is_json_content_type(&mime));

        let mime = "application/problem+json".parse::<mime::Mime>().unwrap();
        assert!(is_json_content_type(&mime));

        let mime = "application/hal+json".parse::<mime::Mime>().unwrap();
        assert!(is_json_content_type(&mime));

        let mime = "text/plain".parse::<mime::Mime>().unwrap();
        assert!(!is_json_content_type(&mime));

        let mime = "application/xml".parse::<mime::Mime>().unwrap();
        assert!(!is_json_content_type(&mime));

        let mime = "application/x-www-form-urlencoded".parse::<mime::Mime>().unwrap();
        assert!(!is_json_content_type(&mime));
    }
}

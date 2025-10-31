//! HTTP middleware for request validation

use axum::{
    body::Body,
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde_json::json;

/// Middleware to validate Content-Type headers and related requirements
pub async fn validate_content_type_middleware(request: Request, next: Next) -> Result<Response, Response> {
    use axum::body::to_bytes;
    use axum::http::Request as HttpRequest;

    let (parts, body) = request.into_parts();
    let headers = &parts.headers;

    // Get declared body size from Content-Length header if present
    let declared_length = headers
        .get(axum::http::header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<usize>().ok());

    // Only validate for methods that have bodies
    let method = &parts.method;
    if method == axum::http::Method::POST || method == axum::http::Method::PUT || method == axum::http::Method::PATCH {
        // Read the body to get actual size
        let body_bytes = match to_bytes(body, usize::MAX).await {
            Ok(bytes) => bytes,
            Err(_) => {
                let error_body = json!({
                    "error": "Failed to read request body"
                });
                return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
            }
        };

        let actual_length = body_bytes.len();

        // Validation 3: Content-Length must match actual body size
        #[allow(clippy::collapsible_if)]
        if let Some(declared) = declared_length {
            if declared != actual_length {
                let error_body = json!({
                    "error": "Content-Length header does not match actual body size"
                });
                return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
            }
        }

        // Validate Content-Type headers
        validate_content_type_headers(headers, actual_length)?;

        // Check if we need to convert URL-encoded form data to JSON
        let (final_parts, final_body) = if let Some(content_type) = headers.get(axum::http::header::CONTENT_TYPE) {
            if let Ok(content_type_str) = content_type.to_str() {
                // Parse Content-Type using the mime crate
                let is_form_urlencoded = content_type_str
                    .parse::<mime::Mime>()
                    .map(|mime| mime.type_() == mime::APPLICATION && mime.subtype() == "x-www-form-urlencoded")
                    .unwrap_or(false);

                if is_form_urlencoded {
                    // Parse URL-encoded form data to JSON
                    let json_body = if body_bytes.is_empty() {
                        // Empty form data becomes empty JSON object
                        serde_json::json!({})
                    } else {
                        match parse_urlencoded_to_json(&body_bytes) {
                            Ok(json_body) => json_body,
                            Err(e) => {
                                // If parsing fails, return error
                                let error_body = json!({
                                    "error": format!("Failed to parse URL-encoded form data: {}", e)
                                });
                                return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
                            }
                        }
                    };

                    // Convert JSON to bytes
                    let json_bytes = serde_json::to_vec(&json_body).unwrap();

                    // Update Content-Type header to application/json
                    let mut new_parts = parts;
                    new_parts.headers.insert(
                        axum::http::header::CONTENT_TYPE,
                        axum::http::HeaderValue::from_static("application/json"),
                    );

                    (new_parts, Body::from(json_bytes))
                } else {
                    (parts, Body::from(body_bytes))
                }
            } else {
                (parts, Body::from(body_bytes))
            }
        } else {
            (parts, Body::from(body_bytes))
        };

        // Reconstruct request with the (possibly converted) body
        let request = HttpRequest::from_parts(final_parts, final_body);
        Ok(next.run(request).await)
    } else {
        // For methods without bodies (GET, DELETE, etc.), just validate headers
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
/// - Empty strings: Handled by serde_qs when brackets present, otherwise converted to boolean false
///
/// Strategy:
/// - If brackets present → use serde_qs (handles nested objects, arrays with [], preserves empty strings)
/// - Otherwise → use query_parser (handles duplicate keys, type conversion)
///
/// Known limitation: Empty string values without brackets (e.g., "field=") are converted to boolean false
/// by the query parser. This is acceptable for most use cases.
fn parse_urlencoded_to_json(data: &[u8]) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    use std::collections::HashMap;
    let body_str = std::str::from_utf8(data)?;

    // Check for bracket notation
    if body_str.contains('[') {
        // Use serde_qs for bracket notation
        let config = serde_qs::Config::new(10, false);
        let parsed: HashMap<String, serde_json::Value> = config.deserialize_str(body_str)?;
        let mut json_value = serde_json::to_value(parsed)?;
        convert_types_recursive(&mut json_value);
        Ok(json_value)
    } else {
        // Use query parser (handles duplicate keys by creating arrays automatically)
        // This also does type conversion
        Ok(crate::query_parser::parse_query_string_to_json(data, true))
    }
}

/// Recursively convert string values to appropriate types (numbers, booleans)
/// while preserving empty strings
fn convert_types_recursive(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::String(s) => {
            // Don't convert empty strings
            if s.is_empty() {
                return;
            }

            // Try to parse as number
            if let Ok(i) = s.parse::<i64>() {
                *value = serde_json::Value::Number(i.into());
                return;
            }
            #[allow(clippy::collapsible_if)]
            if let Ok(f) = s.parse::<f64>() {
                if let Some(n) = serde_json::Number::from_f64(f) {
                    *value = serde_json::Value::Number(n);
                    return;
                }
            }

            // Try to parse as boolean
            let lower = s.to_lowercase();
            if lower == "true" {
                *value = serde_json::Value::Bool(true);
            } else if lower == "false" {
                *value = serde_json::Value::Bool(false);
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

/// Check if a media type is JSON or has a +json suffix
fn is_json_content_type(mime: &mime::Mime) -> bool {
    // Accept application/json or any type with +json suffix (e.g., application/vnd.api+json)
    (mime.type_() == mime::APPLICATION && mime.subtype() == mime::JSON) || mime.suffix() == Some(mime::JSON)
}

/// Validate Content-Type header and related requirements
#[allow(clippy::result_large_err)]
fn validate_content_type_headers(headers: &HeaderMap, _declared_body_size: usize) -> Result<(), Response> {
    // Check Content-Type header if present
    #[allow(clippy::collapsible_if)]
    if let Some(content_type_header) = headers.get(axum::http::header::CONTENT_TYPE) {
        if let Ok(content_type_str) = content_type_header.to_str() {
            // Parse Content-Type using the mime crate
            let parsed_mime = match content_type_str.parse::<mime::Mime>() {
                Ok(m) => m,
                Err(_) => {
                    // If parsing fails, it's an invalid Content-Type
                    let error_body = json!({
                        "error": format!("Invalid Content-Type header: {}", content_type_str)
                    });
                    return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
                }
            };

            // Validation 1: multipart/form-data MUST have boundary parameter
            if parsed_mime.type_() == mime::MULTIPART
                && parsed_mime.subtype() == "form-data"
                && parsed_mime.get_param(mime::BOUNDARY).is_none()
            {
                let error_body = json!({
                    "error": "multipart/form-data requires 'boundary' parameter"
                });
                return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
            }

            // Validation 2: JSON content types (including +json variants) must use UTF-8 charset (or have no charset)
            #[allow(clippy::collapsible_if)]
            if is_json_content_type(&parsed_mime) {
                if let Some(charset) = parsed_mime.get_param(mime::CHARSET) {
                    // Only UTF-8 is allowed (the mime crate normalizes charset names)
                    let charset_str = charset.as_str();
                    if !charset_str.eq_ignore_ascii_case("utf-8") && !charset_str.eq_ignore_ascii_case("utf8") {
                        let error_body = json!({
                            "error": format!("Unsupported charset '{}' for JSON. Only UTF-8 is supported.", charset_str)
                        });
                        return Err((StatusCode::UNSUPPORTED_MEDIA_TYPE, axum::Json(error_body)).into_response());
                    }
                }
            }

            // Validation 3: application/x-www-form-urlencoded is allowed without additional validation
            // (standard form encoding, no special parameters required)
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

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
        // Test standard JSON
        let mime = "application/json".parse::<mime::Mime>().unwrap();
        assert!(is_json_content_type(&mime));

        // Test +json suffix variants
        let mime = "application/vnd.api+json".parse::<mime::Mime>().unwrap();
        assert!(is_json_content_type(&mime));

        let mime = "application/problem+json".parse::<mime::Mime>().unwrap();
        assert!(is_json_content_type(&mime));

        let mime = "application/hal+json".parse::<mime::Mime>().unwrap();
        assert!(is_json_content_type(&mime));

        // Test non-JSON types
        let mime = "text/plain".parse::<mime::Mime>().unwrap();
        assert!(!is_json_content_type(&mime));

        let mime = "application/xml".parse::<mime::Mime>().unwrap();
        assert!(!is_json_content_type(&mime));

        let mime = "application/x-www-form-urlencoded".parse::<mime::Mime>().unwrap();
        assert!(!is_json_content_type(&mime));
    }
}

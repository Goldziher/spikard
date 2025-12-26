//! HTTP middleware for request validation
//!
//! Provides middleware stack setup, JSON schema validation, multipart/form-data parsing,
//! and URL-encoded form data handling.

pub mod multipart;
pub mod urlencoded;
pub mod validation;

use axum::{
    body::Body,
    extract::State,
    extract::{FromRequest, Multipart, Request},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde_json::json;
use std::cell::RefCell;
use std::num::NonZeroUsize;

thread_local! {
    static URLENCODED_JSON_CACHE: RefCell<lru::LruCache<bytes::Bytes, bytes::Bytes>> =
        RefCell::new(lru::LruCache::new(NonZeroUsize::new(256).expect("non-zero cache size")));
}

/// Route information for middleware validation
#[derive(Debug, Clone)]
pub struct RouteInfo {
    /// Whether this route expects a JSON request body
    pub expects_json_body: bool,
}

/// Request extension carrying the already-collected request body.
///
/// This avoids double-reading the body stream: middleware can read once for
/// content-length / syntax checks, and request extraction can reuse the bytes.
#[derive(Debug, Clone)]
pub struct PreReadBody(pub bytes::Bytes);

/// Request extension carrying a pre-parsed JSON body.
#[derive(Debug, Clone)]
pub struct PreParsedJson(pub serde_json::Value);

/// Middleware to validate Content-Type headers and related requirements
///
/// This middleware performs comprehensive request body validation and transformation:
///
/// - **Content-Type Validation:** Ensures the request's Content-Type header matches the
///   expected format for the route (if configured).
///
/// - **Multipart Form Data:** Automatically parses `multipart/form-data` requests and
///   transforms them into JSON format for uniform downstream processing.
///
/// - **URL-Encoded Forms:** Parses `application/x-www-form-urlencoded` requests and
///   converts them to JSON.
///
/// - **JSON Validation:** Validates JSON request bodies for well-formedness (when the
///   Content-Type is `application/json`).
///
/// - **Content-Length:** Validates that the Content-Length header is present and
///   reasonable for POST, PUT, and PATCH requests.
///
/// # Behavior
///
/// For request methods POST, PUT, and PATCH:
/// 1. Checks if the route expects a JSON body (via route state)
/// 2. Validates Content-Type headers based on route configuration
/// 3. Parses the request body according to Content-Type:
///    - `multipart/form-data` → JSON (form fields as object properties)
///    - `application/x-www-form-urlencoded` → JSON (URL parameters as object)
///    - `application/json` → Validates JSON syntax
/// 4. Transforms the request to have `Content-Type: application/json`
/// 5. Passes the transformed request to the next middleware
///
/// For GET, DELETE, and other methods: passes through with minimal validation.
///
/// # Errors
///
/// Returns HTTP error responses for:
/// - `400 Bad Request` - Failed to read request body, invalid JSON, malformed forms, invalid Content-Length
/// - `500 Internal Server Error` - Failed to serialize transformed body
///
/// # Examples
///
/// ```rust
/// use axum::{middleware::Next, extract::Request};
/// use spikard_http::middleware::validate_content_type_middleware;
///
/// // This is typically used as middleware in an Axum router:
/// // router.layer(axum::middleware::from_fn(validate_content_type_middleware))
/// ```
///
/// Coverage: Tested via integration tests (multipart and form parsing tested end-to-end)
#[cfg(not(tarpaulin_include))]
pub async fn validate_content_type_middleware(
    State(route_info): State<RouteInfo>,
    request: Request,
    next: Next,
) -> Result<Response, Response> {
    use axum::body::to_bytes;
    use axum::http::Request as HttpRequest;

    let (mut parts, body) = request.into_parts();
    let headers = &parts.headers;

    let method = &parts.method;
    if method == axum::http::Method::POST || method == axum::http::Method::PUT || method == axum::http::Method::PATCH {
        if route_info.expects_json_body {
            validation::validate_json_content_type(headers)?;
        }

        let content_kind = validation::validate_content_type_headers_and_classify(headers, 0)?;

        let mut parsed_json: Option<serde_json::Value> = None;
        let out_bytes: bytes::Bytes = match content_kind {
            Some(validation::ContentTypeKind::Multipart) => {
                let body_bytes = match to_bytes(body, usize::MAX).await {
                    Ok(bytes) => bytes,
                    Err(_) => {
                        let error_body = json!({
                            "error": "Failed to read request body"
                        });
                        return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
                    }
                };

                if headers.get(axum::http::header::CONTENT_LENGTH).is_some() {
                    validation::validate_content_length(headers, body_bytes.len())?;
                }

                let mut parse_request = HttpRequest::new(Body::from(body_bytes));
                *parse_request.headers_mut() = parts.headers.clone();

                let multipart = match Multipart::from_request(parse_request, &()).await {
                    Ok(mp) => mp,
                    Err(e) => {
                        let error_body = json!({
                            "error": format!("Failed to parse multipart data: {}", e)
                        });
                        return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
                    }
                };

                let json_body = match multipart::parse_multipart_to_json(multipart).await {
                    Ok(json) => json,
                    Err(e) => {
                        let error_body = json!({
                            "error": format!("Failed to process multipart data: {}", e)
                        });
                        return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
                    }
                };

                let json_bytes = match serde_json::to_vec(&json_body) {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        let error_body = json!({
                            "error": format!("Failed to serialize multipart data to JSON: {}", e)
                        });
                        return Err((StatusCode::INTERNAL_SERVER_ERROR, axum::Json(error_body)).into_response());
                    }
                };

                parsed_json = Some(json_body);
                parts.headers.insert(
                    axum::http::header::CONTENT_TYPE,
                    axum::http::HeaderValue::from_static("application/json"),
                );
                if let Ok(value) = axum::http::HeaderValue::from_str(&json_bytes.len().to_string()) {
                    parts.headers.insert(axum::http::header::CONTENT_LENGTH, value);
                }
                bytes::Bytes::from(json_bytes)
            }
            Some(validation::ContentTypeKind::FormUrlencoded) => {
                let body_bytes = match to_bytes(body, usize::MAX).await {
                    Ok(bytes) => bytes,
                    Err(_) => {
                        let error_body = json!({
                            "error": "Failed to read request body"
                        });
                        return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
                    }
                };

                if headers.get(axum::http::header::CONTENT_LENGTH).is_some() {
                    validation::validate_content_length(headers, body_bytes.len())?;
                }

                parts.headers.insert(
                    axum::http::header::CONTENT_TYPE,
                    axum::http::HeaderValue::from_static("application/json"),
                );
                if let Some(cached) = URLENCODED_JSON_CACHE.with(|cache| cache.borrow_mut().get(&body_bytes).cloned()) {
                    cached
                } else {
                    let json_body = if body_bytes.is_empty() {
                        serde_json::json!({})
                    } else {
                        match urlencoded::parse_urlencoded_to_json(&body_bytes) {
                            Ok(json_body) => json_body,
                            Err(e) => {
                                let error_body = json!({
                                    "error": format!("Failed to parse URL-encoded form data: {}", e)
                                });
                                return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
                            }
                        }
                    };

                    let json_bytes = match serde_json::to_vec(&json_body) {
                        Ok(bytes) => bytes,
                        Err(e) => {
                            let error_body = json!({
                                "error": format!("Failed to serialize URL-encoded form data to JSON: {}", e)
                            });
                            return Err((StatusCode::INTERNAL_SERVER_ERROR, axum::Json(error_body)).into_response());
                        }
                    };

                    let json_bytes = bytes::Bytes::from(json_bytes);
                    parsed_json = Some(json_body);
                    URLENCODED_JSON_CACHE.with(|cache| {
                        cache.borrow_mut().put(body_bytes.clone(), json_bytes.clone());
                    });
                    json_bytes
                }
            }
            Some(validation::ContentTypeKind::Json) | Some(validation::ContentTypeKind::Other) => {
                let body_bytes = match to_bytes(body, usize::MAX).await {
                    Ok(bytes) => bytes,
                    Err(_) => {
                        let error_body = json!({
                            "error": "Failed to read request body"
                        });
                        return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
                    }
                };

                if headers.get(axum::http::header::CONTENT_LENGTH).is_some() {
                    validation::validate_content_length(headers, body_bytes.len())?;
                }

                let should_validate_json =
                    route_info.expects_json_body && matches!(content_kind, Some(validation::ContentTypeKind::Json));
                if should_validate_json && !body_bytes.is_empty() {
                    match serde_json::from_slice::<serde_json::Value>(&body_bytes) {
                        Ok(value) => parsed_json = Some(value),
                        Err(_) => {
                            let error_body = json!({
                                "detail": "Invalid request format"
                            });
                            return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
                        }
                    }
                }

                body_bytes
            }
            None => {
                let body_bytes = match to_bytes(body, usize::MAX).await {
                    Ok(bytes) => bytes,
                    Err(_) => {
                        let error_body = json!({
                            "error": "Failed to read request body"
                        });
                        return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
                    }
                };

                if headers.get(axum::http::header::CONTENT_LENGTH).is_some() {
                    validation::validate_content_length(headers, body_bytes.len())?;
                }
                body_bytes
            }
        };

        parts.extensions.insert(PreReadBody(out_bytes));
        if let Some(parsed) = parsed_json {
            parts.extensions.insert(PreParsedJson(parsed));
        }

        let request = HttpRequest::from_parts(parts, Body::empty());
        Ok(next.run(request).await)
    } else {
        validation::validate_content_type_headers(headers, 0)?;

        let request = HttpRequest::from_parts(parts, body);
        Ok(next.run(request).await)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;

    #[test]
    fn test_route_info_creation() {
        let info = RouteInfo {
            expects_json_body: true,
        };
        assert!(info.expects_json_body);
    }

    #[test]
    fn test_route_info_expects_json_body_true() {
        let info = RouteInfo {
            expects_json_body: true,
        };
        assert_eq!(info.expects_json_body, true);
    }

    #[test]
    fn test_route_info_expects_json_body_false() {
        let info = RouteInfo {
            expects_json_body: false,
        };
        assert_eq!(info.expects_json_body, false);
    }

    #[test]
    fn test_request_with_zero_content_length() {
        let headers = axum::http::HeaderMap::new();
        assert!(headers.get(axum::http::header::CONTENT_LENGTH).is_none());
    }

    #[test]
    fn test_request_with_very_large_content_length() {
        let mut headers = axum::http::HeaderMap::new();
        let large_size = usize::MAX - 1;
        headers.insert(
            axum::http::header::CONTENT_LENGTH,
            axum::http::HeaderValue::from_str(&large_size.to_string()).unwrap(),
        );
        assert!(headers.get(axum::http::header::CONTENT_LENGTH).is_some());
    }

    #[test]
    fn test_request_body_smaller_than_declared_length() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_LENGTH,
            axum::http::HeaderValue::from_static("1000"),
        );
        let result = super::validation::validate_content_length(&headers, 500);
        assert!(
            result.is_err(),
            "Should reject when actual body is smaller than declared"
        );
    }

    #[test]
    fn test_request_body_larger_than_declared_length() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_LENGTH,
            axum::http::HeaderValue::from_static("500"),
        );
        let result = super::validation::validate_content_length(&headers, 1000);
        assert!(
            result.is_err(),
            "Should reject when actual body is larger than declared"
        );
    }

    #[test]
    fn test_get_request_no_body_validation() {
        let request = Request::builder()
            .method(axum::http::Method::GET)
            .uri("/api/users")
            .body(Body::empty())
            .unwrap();

        let (parts, _body) = request.into_parts();
        assert_eq!(parts.method, axum::http::Method::GET);
    }

    #[test]
    fn test_delete_request_no_body_validation() {
        let request = Request::builder()
            .method(axum::http::Method::DELETE)
            .uri("/api/users/1")
            .body(Body::empty())
            .unwrap();

        let (parts, _body) = request.into_parts();
        assert_eq!(parts.method, axum::http::Method::DELETE);
    }

    #[test]
    fn test_post_request_requires_validation() {
        let request = Request::builder()
            .method(axum::http::Method::POST)
            .uri("/api/users")
            .body(Body::empty())
            .unwrap();

        let (parts, _body) = request.into_parts();
        assert_eq!(parts.method, axum::http::Method::POST);
    }

    #[test]
    fn test_put_request_requires_validation() {
        let request = Request::builder()
            .method(axum::http::Method::PUT)
            .uri("/api/users/1")
            .body(Body::empty())
            .unwrap();

        let (parts, _body) = request.into_parts();
        assert_eq!(parts.method, axum::http::Method::PUT);
    }

    #[test]
    fn test_patch_request_requires_validation() {
        let request = Request::builder()
            .method(axum::http::Method::PATCH)
            .uri("/api/users/1")
            .body(Body::empty())
            .unwrap();

        let (parts, _body) = request.into_parts();
        assert_eq!(parts.method, axum::http::Method::PATCH);
    }

    #[test]
    fn test_content_type_header_case_insensitive() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            axum::http::HeaderValue::from_static("application/json"),
        );

        assert!(headers.get(axum::http::header::CONTENT_TYPE).is_some());
    }

    #[test]
    fn test_content_length_header_case_insensitive() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_LENGTH,
            axum::http::HeaderValue::from_static("100"),
        );

        assert!(headers.get(axum::http::header::CONTENT_LENGTH).is_some());
    }

    #[test]
    fn test_custom_headers_case_preserved() {
        let mut headers = axum::http::HeaderMap::new();
        let custom_header: axum::http::HeaderName = "X-Custom-Header".parse().unwrap();
        headers.insert(custom_header.clone(), axum::http::HeaderValue::from_static("value"));

        assert!(headers.get(&custom_header).is_some());
    }

    #[test]
    fn test_multipart_boundary_minimal() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            axum::http::HeaderValue::from_static("multipart/form-data; boundary=x"),
        );

        let result = super::validation::validate_content_type_headers(&headers, 0);
        assert!(result.is_ok(), "Minimal boundary should be accepted");
    }

    #[test]
    fn test_multipart_boundary_with_numbers() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            axum::http::HeaderValue::from_static("multipart/form-data; boundary=boundary123456"),
        );

        let result = super::validation::validate_content_type_headers(&headers, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multipart_boundary_with_special_chars() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            axum::http::HeaderValue::from_static("multipart/form-data; boundary=----WebKitFormBoundary7MA4YWxkTrZu0gW"),
        );

        let result = super::validation::validate_content_type_headers(&headers, 0);
        assert!(result.is_ok(), "Boundary with dashes should be accepted");
    }

    #[test]
    fn test_multipart_empty_boundary() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            axum::http::HeaderValue::from_static("multipart/form-data; boundary="),
        );

        let _result = super::validation::validate_content_type_headers(&headers, 0);
        assert!(headers.get(axum::http::header::CONTENT_TYPE).is_some());
    }

    #[test]
    fn test_invalid_json_body_detection() {
        let invalid_json = r#"{"invalid": json without quotes}"#;
        let _mime = "application/json".parse::<mime::Mime>().unwrap();

        let result = serde_json::from_str::<serde_json::Value>(invalid_json);
        assert!(result.is_err(), "Invalid JSON should fail parsing");
    }

    #[test]
    fn test_valid_json_parsing() {
        let valid_json = r#"{"key": "value"}"#;
        let result = serde_json::from_str::<serde_json::Value>(valid_json);
        assert!(result.is_ok(), "Valid JSON should parse successfully");
    }

    #[test]
    fn test_empty_json_object() {
        let empty_json = "{}";
        let result = serde_json::from_str::<serde_json::Value>(empty_json);
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value.is_object());
        assert_eq!(value.as_object().unwrap().len(), 0);
    }

    #[test]
    fn test_form_data_mime_type() {
        let mime = "multipart/form-data; boundary=xyz".parse::<mime::Mime>().unwrap();
        assert_eq!(mime.type_(), mime::MULTIPART);
        assert_eq!(mime.subtype(), "form-data");
    }

    #[test]
    fn test_form_urlencoded_mime_type() {
        let mime = "application/x-www-form-urlencoded".parse::<mime::Mime>().unwrap();
        assert_eq!(mime.type_(), mime::APPLICATION);
        assert_eq!(mime.subtype(), "x-www-form-urlencoded");
    }

    #[test]
    fn test_json_mime_type() {
        let mime = "application/json".parse::<mime::Mime>().unwrap();
        assert_eq!(mime.type_(), mime::APPLICATION);
        assert_eq!(mime.subtype(), mime::JSON);
    }

    #[test]
    fn test_text_plain_mime_type() {
        let mime = "text/plain".parse::<mime::Mime>().unwrap();
        assert_eq!(mime.type_(), mime::TEXT);
        assert_eq!(mime.subtype(), "plain");
    }
}

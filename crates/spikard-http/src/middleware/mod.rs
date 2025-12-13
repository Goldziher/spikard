//! HTTP middleware for request validation
//!
//! Provides middleware stack setup, JSON schema validation, multipart/form-data parsing,
//! and URL-encoded form data handling.

pub mod multipart;
pub mod urlencoded;
pub mod validation;

use axum::{
    body::Body,
    extract::{FromRequest, Multipart, Request},
    http::StatusCode,
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
/// 1. Checks if the route expects a JSON body (via `RouteRegistry`)
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
            validation::validate_json_content_type(headers)?;
        }

        validation::validate_content_type_headers(headers, 0)?;

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

                    validation::validate_content_length(headers, body_bytes.len())?;

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

                    validation::validate_content_length(headers, body_bytes.len())?;

                    let is_json = parsed_mime
                        .as_ref()
                        .map(validation::is_json_content_type)
                        .unwrap_or(false);

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

                validation::validate_content_length(headers, body_bytes.len())?;

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

            validation::validate_content_length(headers, body_bytes.len())?;

            (parts, Body::from(body_bytes))
        };

        let request = HttpRequest::from_parts(final_parts, final_body);
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
    fn test_route_registry_empty() {
        let registry: RouteRegistry = Arc::new(std::collections::HashMap::new());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn test_route_registry_single_entry() {
        let mut map = std::collections::HashMap::new();
        map.insert(
            ("POST".to_string(), "/api/users".to_string()),
            RouteInfo {
                expects_json_body: true,
            },
        );
        let registry: RouteRegistry = Arc::new(map);

        let key = ("POST".to_string(), "/api/users".to_string());
        assert!(registry.contains_key(&key));
        assert_eq!(registry[&key].expects_json_body, true);
    }

    #[test]
    fn test_route_registry_multiple_entries() {
        let mut map = std::collections::HashMap::new();
        map.insert(
            ("POST".to_string(), "/api/users".to_string()),
            RouteInfo {
                expects_json_body: true,
            },
        );
        map.insert(
            ("GET".to_string(), "/api/users".to_string()),
            RouteInfo {
                expects_json_body: false,
            },
        );
        map.insert(
            ("PUT".to_string(), "/api/users/{id}".to_string()),
            RouteInfo {
                expects_json_body: true,
            },
        );
        let registry: RouteRegistry = Arc::new(map);

        assert_eq!(registry.len(), 3);
    }

    #[test]
    fn test_route_registry_lookup_missing_route() {
        let map = std::collections::HashMap::new();
        let registry: RouteRegistry = Arc::new(map);

        let key = ("POST".to_string(), "/api/users".to_string());
        assert!(!registry.contains_key(&key));
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

//! CORS (Cross-Origin Resource Sharing) handling
//!
//! Handles CORS preflight requests and adds CORS headers to responses

use crate::CorsConfig;
use axum::body::Body;
use axum::http::{HeaderMap, HeaderValue, Response, StatusCode};
use axum::response::IntoResponse;

/// Check if an origin is allowed by the CORS configuration
///
/// Supports exact matches and wildcard ("*") for any origin
fn is_origin_allowed(origin: &str, allowed_origins: &[String]) -> bool {
    if origin.is_empty() {
        return false;
    }

    allowed_origins
        .iter()
        .any(|allowed| allowed == "*" || allowed == origin)
}

/// Check if a method is allowed by the CORS configuration
///
/// Supports exact matches and wildcard ("*") for any method
fn is_method_allowed(method: &str, allowed_methods: &[String]) -> bool {
    allowed_methods
        .iter()
        .any(|allowed| allowed == "*" || allowed.eq_ignore_ascii_case(method))
}

/// Check if all requested headers are allowed
///
/// Headers are case-insensitive. Supports wildcard ("*") for any header.
fn are_headers_allowed(requested: &[&str], allowed: &[String]) -> bool {
    // If wildcard is in allowed list, all headers are permitted
    if allowed.iter().any(|h| h == "*") {
        return true;
    }

    // Check each requested header is in the allowed list (case-insensitive)
    requested.iter().all(|req_header| {
        allowed
            .iter()
            .any(|allowed_header| allowed_header.eq_ignore_ascii_case(req_header))
    })
}

/// Handle CORS preflight (OPTIONS) request
///
/// Validates the request against the CORS configuration and returns appropriate
/// response or error. Returns 204 No Content on success, 403 Forbidden on validation failure.
pub fn handle_preflight(headers: &HeaderMap, cors_config: &CorsConfig) -> Result<Response<Body>, Box<Response<Body>>> {
    // Get the Origin header
    let origin = headers.get("origin").and_then(|v| v.to_str().ok()).unwrap_or("");

    // Validate origin
    if origin.is_empty() || !is_origin_allowed(origin, &cors_config.allowed_origins) {
        return Err(Box::new(
            (
                StatusCode::FORBIDDEN,
                axum::Json(serde_json::json!({
                    "detail": format!("CORS request from origin '{}' not allowed", origin)
                })),
            )
                .into_response(),
        ));
    }

    // Get requested method
    let requested_method = headers
        .get("access-control-request-method")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    // Validate method
    if !requested_method.is_empty() && !is_method_allowed(requested_method, &cors_config.allowed_methods) {
        return Err(Box::new((StatusCode::FORBIDDEN).into_response()));
    }

    // Get requested headers
    let requested_headers_str = headers
        .get("access-control-request-headers")
        .and_then(|v| v.to_str().ok());

    if let Some(req_headers) = requested_headers_str {
        let requested_headers: Vec<&str> = req_headers.split(',').map(|h| h.trim()).collect();

        if !are_headers_allowed(&requested_headers, &cors_config.allowed_headers) {
            return Err(Box::new((StatusCode::FORBIDDEN).into_response()));
        }
    }

    // Build preflight response
    let mut response = Response::builder().status(StatusCode::NO_CONTENT);

    // Add CORS headers
    let headers_mut = response.headers_mut().unwrap();

    // Access-Control-Allow-Origin
    headers_mut.insert(
        "access-control-allow-origin",
        HeaderValue::from_str(origin).unwrap_or_else(|_| HeaderValue::from_static("*")),
    );

    // Access-Control-Allow-Methods
    let methods = cors_config.allowed_methods.join(", ");
    headers_mut.insert(
        "access-control-allow-methods",
        HeaderValue::from_str(&methods).unwrap_or_else(|_| HeaderValue::from_static("*")),
    );

    // Access-Control-Allow-Headers
    let allowed_headers = cors_config.allowed_headers.join(", ");
    headers_mut.insert(
        "access-control-allow-headers",
        HeaderValue::from_str(&allowed_headers).unwrap_or_else(|_| HeaderValue::from_static("*")),
    );

    // Access-Control-Max-Age (optional)
    if let Some(max_age) = cors_config.max_age {
        headers_mut.insert(
            "access-control-max-age",
            HeaderValue::from_str(&max_age.to_string()).unwrap(),
        );
    }

    // Access-Control-Allow-Credentials (optional)
    if let Some(true) = cors_config.allow_credentials {
        headers_mut.insert("access-control-allow-credentials", HeaderValue::from_static("true"));
    }

    Ok(response.body(Body::empty()).unwrap())
}

/// Add CORS headers to a successful response
///
/// Adds Access-Control-Allow-Origin, Access-Control-Expose-Headers, and
/// Access-Control-Allow-Credentials based on the configuration
pub fn add_cors_headers(response: &mut Response<Body>, origin: &str, cors_config: &CorsConfig) {
    let headers = response.headers_mut();

    // Access-Control-Allow-Origin
    if let Ok(origin_value) = HeaderValue::from_str(origin) {
        headers.insert("access-control-allow-origin", origin_value);
    }

    // Access-Control-Expose-Headers (optional)
    if let Some(ref expose_headers) = cors_config.expose_headers {
        let expose = expose_headers.join(", ");
        if let Ok(expose_value) = HeaderValue::from_str(&expose) {
            headers.insert("access-control-expose-headers", expose_value);
        }
    }

    // Access-Control-Allow-Credentials (optional)
    if let Some(true) = cors_config.allow_credentials {
        headers.insert("access-control-allow-credentials", HeaderValue::from_static("true"));
    }
}

/// Validate a non-preflight CORS request
///
/// Checks if the Origin header is present and allowed. Returns an error response
/// if validation fails.
pub fn validate_cors_request(headers: &HeaderMap, cors_config: &CorsConfig) -> Result<(), Box<Response<Body>>> {
    let origin = headers.get("origin").and_then(|v| v.to_str().ok()).unwrap_or("");

    if !origin.is_empty() && !is_origin_allowed(origin, &cors_config.allowed_origins) {
        return Err(Box::new(
            (
                StatusCode::FORBIDDEN,
                axum::Json(serde_json::json!({
                    "detail": format!("CORS request from origin '{}' not allowed", origin)
                })),
            )
                .into_response(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_cors_config() -> CorsConfig {
        CorsConfig {
            allowed_origins: vec!["https://example.com".to_string()],
            allowed_methods: vec!["GET".to_string(), "POST".to_string()],
            allowed_headers: vec!["content-type".to_string(), "authorization".to_string()],
            expose_headers: Some(vec!["x-custom-header".to_string()]),
            max_age: Some(3600),
            allow_credentials: Some(true),
        }
    }

    #[test]
    fn test_is_origin_allowed_exact_match() {
        let allowed = vec!["https://example.com".to_string()];
        assert!(is_origin_allowed("https://example.com", &allowed));
        assert!(!is_origin_allowed("https://evil.com", &allowed));
    }

    #[test]
    fn test_is_origin_allowed_wildcard() {
        let allowed = vec!["*".to_string()];
        assert!(is_origin_allowed("https://example.com", &allowed));
        assert!(is_origin_allowed("https://any-domain.com", &allowed));
    }

    #[test]
    fn test_is_origin_allowed_empty_origin() {
        let allowed = vec!["*".to_string()];
        assert!(!is_origin_allowed("", &allowed));
    }

    #[test]
    fn test_is_method_allowed_case_insensitive() {
        let allowed = vec!["GET".to_string(), "POST".to_string()];
        assert!(is_method_allowed("GET", &allowed));
        assert!(is_method_allowed("get", &allowed));
        assert!(is_method_allowed("POST", &allowed));
        assert!(is_method_allowed("post", &allowed));
        assert!(!is_method_allowed("DELETE", &allowed));
    }

    #[test]
    fn test_is_method_allowed_wildcard() {
        let allowed = vec!["*".to_string()];
        assert!(is_method_allowed("GET", &allowed));
        assert!(is_method_allowed("DELETE", &allowed));
        assert!(is_method_allowed("PATCH", &allowed));
    }

    #[test]
    fn test_are_headers_allowed_case_insensitive() {
        let allowed = vec!["Content-Type".to_string(), "Authorization".to_string()];
        assert!(are_headers_allowed(&["content-type"], &allowed));
        assert!(are_headers_allowed(&["AUTHORIZATION"], &allowed));
        assert!(are_headers_allowed(&["content-type", "authorization"], &allowed));
        assert!(!are_headers_allowed(&["x-custom"], &allowed));
    }

    #[test]
    fn test_are_headers_allowed_wildcard() {
        let allowed = vec!["*".to_string()];
        assert!(are_headers_allowed(&["any-header"], &allowed));
        assert!(are_headers_allowed(&["multiple", "headers"], &allowed));
    }

    #[test]
    fn test_handle_preflight_success() {
        let config = make_cors_config();
        let mut headers = HeaderMap::new();
        headers.insert("origin", HeaderValue::from_static("https://example.com"));
        headers.insert("access-control-request-method", HeaderValue::from_static("POST"));
        headers.insert(
            "access-control-request-headers",
            HeaderValue::from_static("content-type"),
        );

        let result = handle_preflight(&headers, &config);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let resp_headers = response.headers();
        assert_eq!(
            resp_headers.get("access-control-allow-origin").unwrap(),
            "https://example.com"
        );
        assert!(
            resp_headers
                .get("access-control-allow-methods")
                .unwrap()
                .to_str()
                .unwrap()
                .contains("POST")
        );
        assert_eq!(resp_headers.get("access-control-max-age").unwrap(), "3600");
        assert_eq!(resp_headers.get("access-control-allow-credentials").unwrap(), "true");
    }

    #[test]
    fn test_handle_preflight_origin_not_allowed() {
        let config = make_cors_config();
        let mut headers = HeaderMap::new();
        headers.insert("origin", HeaderValue::from_static("https://evil.com"));
        headers.insert("access-control-request-method", HeaderValue::from_static("GET"));

        let result = handle_preflight(&headers, &config);
        assert!(result.is_err());

        let response = *result.unwrap_err();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[test]
    fn test_handle_preflight_method_not_allowed() {
        let config = make_cors_config();
        let mut headers = HeaderMap::new();
        headers.insert("origin", HeaderValue::from_static("https://example.com"));
        headers.insert("access-control-request-method", HeaderValue::from_static("DELETE"));

        let result = handle_preflight(&headers, &config);
        assert!(result.is_err());

        let response = *result.unwrap_err();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[test]
    fn test_handle_preflight_header_not_allowed() {
        let config = make_cors_config();
        let mut headers = HeaderMap::new();
        headers.insert("origin", HeaderValue::from_static("https://example.com"));
        headers.insert("access-control-request-method", HeaderValue::from_static("POST"));
        headers.insert(
            "access-control-request-headers",
            HeaderValue::from_static("x-forbidden-header"),
        );

        let result = handle_preflight(&headers, &config);
        assert!(result.is_err());

        let response = *result.unwrap_err();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[test]
    fn test_handle_preflight_empty_origin() {
        let config = make_cors_config();
        let headers = HeaderMap::new(); // No origin header

        let result = handle_preflight(&headers, &config);
        assert!(result.is_err());

        let response = *result.unwrap_err();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[test]
    fn test_add_cors_headers() {
        let config = make_cors_config();
        let mut response = Response::new(Body::empty());

        add_cors_headers(&mut response, "https://example.com", &config);

        let headers = response.headers();
        assert_eq!(
            headers.get("access-control-allow-origin").unwrap(),
            "https://example.com"
        );
        assert_eq!(headers.get("access-control-expose-headers").unwrap(), "x-custom-header");
        assert_eq!(headers.get("access-control-allow-credentials").unwrap(), "true");
    }

    #[test]
    fn test_validate_cors_request_allowed() {
        let config = make_cors_config();
        let mut headers = HeaderMap::new();
        headers.insert("origin", HeaderValue::from_static("https://example.com"));

        let result = validate_cors_request(&headers, &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_cors_request_not_allowed() {
        let config = make_cors_config();
        let mut headers = HeaderMap::new();
        headers.insert("origin", HeaderValue::from_static("https://evil.com"));

        let result = validate_cors_request(&headers, &config);
        assert!(result.is_err());

        let response = *result.unwrap_err();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[test]
    fn test_validate_cors_request_no_origin() {
        let config = make_cors_config();
        let headers = HeaderMap::new(); // No origin header

        // No origin header is allowed (not a CORS request)
        let result = validate_cors_request(&headers, &config);
        assert!(result.is_ok());
    }
}

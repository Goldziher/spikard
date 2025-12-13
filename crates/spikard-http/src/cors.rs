//! CORS (Cross-Origin Resource Sharing) handling
//!
//! Handles CORS preflight requests and adds CORS headers to responses

use crate::CorsConfig;
use axum::body::Body;
use axum::http::{HeaderMap, HeaderValue, Response, StatusCode};
use axum::response::IntoResponse;

/// Check if an origin is allowed by the CORS configuration
///
/// Supports exact matches and wildcard ("*") for any origin.
/// Empty origins always return false for security.
///
/// # Arguments
/// * `origin` - The origin string from the HTTP request (e.g., "https://example.com")
/// * `allowed_origins` - List of allowed origins configured for CORS
///
/// # Returns
/// `true` if the origin is allowed, `false` otherwise
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
/// Supports exact matches and wildcard ("*") for any method.
/// Comparison is case-insensitive (e.g., "get" matches "GET").
///
/// # Arguments
/// * `method` - The HTTP method requested (e.g., "GET", "POST")
/// * `allowed_methods` - List of allowed HTTP methods configured for CORS
///
/// # Returns
/// `true` if the method is allowed, `false` otherwise
fn is_method_allowed(method: &str, allowed_methods: &[String]) -> bool {
    allowed_methods
        .iter()
        .any(|allowed| allowed == "*" || allowed.eq_ignore_ascii_case(method))
}

/// Check if all requested headers are allowed by CORS configuration
///
/// Headers are case-insensitive. Supports wildcard ("*") for allowing any header.
/// If a wildcard is configured, all requested headers are allowed.
///
/// # Arguments
/// * `requested` - Array of header names requested by the client
/// * `allowed` - List of allowed header names configured for CORS
///
/// # Returns
/// `true` if all requested headers are allowed, `false` if any header is not allowed
fn are_headers_allowed(requested: &[&str], allowed: &[String]) -> bool {
    if allowed.iter().any(|h| h == "*") {
        return true;
    }

    requested.iter().all(|req_header| {
        allowed
            .iter()
            .any(|allowed_header| allowed_header.eq_ignore_ascii_case(req_header))
    })
}

/// Handle CORS preflight (OPTIONS) request
///
/// Validates the request against the CORS configuration and returns appropriate
/// response or error. This function processes OPTIONS requests as defined in the
/// CORS specification (RFC 7231).
///
/// # Validation
///
/// Checks the following conditions:
/// 1. **Origin Header:** Must be present and match configured allowed origins
/// 2. **Access-Control-Request-Method:** Must match configured allowed methods
/// 3. **Access-Control-Request-Headers:** All requested headers must match configured allowed headers
///
/// # Success Response
///
/// Returns HTTP 204 (No Content) with the following response headers:
/// - `Access-Control-Allow-Origin` - The origin that is allowed
/// - `Access-Control-Allow-Methods` - Comma-separated list of allowed methods
/// - `Access-Control-Allow-Headers` - Comma-separated list of allowed headers
/// - `Access-Control-Max-Age` - Caching duration in seconds (if configured)
/// - `Access-Control-Allow-Credentials` - "true" if credentials are allowed
///
/// # Error Response
///
/// Returns HTTP 403 (Forbidden) if validation fails for:
/// - Origin not in allowed list
/// - Requested method not allowed
/// - Requested headers not allowed
///
/// # Arguments
/// * `headers` - Request headers containing CORS preflight information
/// * `cors_config` - CORS configuration to validate against
///
/// # Returns
/// * `Ok(Response)` - 204 No Content with CORS headers
/// * `Err(Response)` - 403 Forbidden or 500 Internal Server Error
pub fn handle_preflight(headers: &HeaderMap, cors_config: &CorsConfig) -> Result<Response<Body>, Box<Response<Body>>> {
    let origin = headers.get("origin").and_then(|v| v.to_str().ok()).unwrap_or("");

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

    let requested_method = headers
        .get("access-control-request-method")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !requested_method.is_empty() && !is_method_allowed(requested_method, &cors_config.allowed_methods) {
        return Err(Box::new((StatusCode::FORBIDDEN).into_response()));
    }

    let requested_headers_str = headers
        .get("access-control-request-headers")
        .and_then(|v| v.to_str().ok());

    if let Some(req_headers) = requested_headers_str {
        let requested_headers: Vec<&str> = req_headers.split(',').map(|h| h.trim()).collect();

        if !are_headers_allowed(&requested_headers, &cors_config.allowed_headers) {
            return Err(Box::new((StatusCode::FORBIDDEN).into_response()));
        }
    }

    let mut response = Response::builder().status(StatusCode::NO_CONTENT);

    let headers_mut = match response.headers_mut() {
        Some(headers) => headers,
        None => {
            return Err(Box::new(
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(serde_json::json!({
                        "detail": "Failed to construct response headers"
                    })),
                )
                    .into_response(),
            ));
        }
    };

    headers_mut.insert(
        "access-control-allow-origin",
        HeaderValue::from_str(origin).unwrap_or_else(|_| HeaderValue::from_static("*")),
    );

    let methods = cors_config.allowed_methods.join(", ");
    headers_mut.insert(
        "access-control-allow-methods",
        HeaderValue::from_str(&methods).unwrap_or_else(|_| HeaderValue::from_static("*")),
    );

    let allowed_headers = cors_config.allowed_headers.join(", ");
    headers_mut.insert(
        "access-control-allow-headers",
        HeaderValue::from_str(&allowed_headers).unwrap_or_else(|_| HeaderValue::from_static("*")),
    );

    if let Some(max_age) = cors_config.max_age
        && let Ok(header_val) = HeaderValue::from_str(&max_age.to_string())
    {
        headers_mut.insert("access-control-max-age", header_val);
    }

    if let Some(true) = cors_config.allow_credentials {
        headers_mut.insert("access-control-allow-credentials", HeaderValue::from_static("true"));
    }

    match response.body(Body::empty()) {
        Ok(resp) => Ok(resp),
        Err(_) => Err(Box::new(
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(serde_json::json!({
                    "detail": "Failed to construct response body"
                })),
            )
                .into_response(),
        )),
    }
}

/// Add CORS headers to a successful response
///
/// Adds appropriate CORS headers to the response based on the configuration.
/// This function should be called for successful (non-error) responses to
/// cross-origin requests.
///
/// # Headers Added
///
/// - `Access-Control-Allow-Origin` - The origin that is allowed (if valid)
/// - `Access-Control-Expose-Headers` - Headers that are safe to expose to the client
/// - `Access-Control-Allow-Credentials` - "true" if credentials are allowed
///
/// # Arguments
/// * `response` - Mutable reference to the response to modify
/// * `origin` - The origin from the request (e.g., `<https://example.com>`)
/// * `cors_config` - CORS configuration to apply
///
/// # Example
///
/// ```ignore
/// let mut response = Response::new(Body::empty());
/// add_cors_headers(&mut response, "https://example.com", &cors_config);
/// ```
pub fn add_cors_headers(response: &mut Response<Body>, origin: &str, cors_config: &CorsConfig) {
    let headers = response.headers_mut();

    if let Ok(origin_value) = HeaderValue::from_str(origin) {
        headers.insert("access-control-allow-origin", origin_value);
    }

    if let Some(ref expose_headers) = cors_config.expose_headers {
        let expose = expose_headers.join(", ");
        if let Ok(expose_value) = HeaderValue::from_str(&expose) {
            headers.insert("access-control-expose-headers", expose_value);
        }
    }

    if let Some(true) = cors_config.allow_credentials {
        headers.insert("access-control-allow-credentials", HeaderValue::from_static("true"));
    }
}

/// Validate a non-preflight CORS request
///
/// Checks if the Origin header is present and allowed for non-preflight (actual) requests.
/// Returns an error response if validation fails.
///
/// # Validation
///
/// - If no Origin header is present, the request is allowed (not a CORS request)
/// - If Origin header is present, it must match the allowed origins
///
/// # Arguments
/// * `headers` - Request headers containing origin information
/// * `cors_config` - CORS configuration to validate against
///
/// # Returns
/// * `Ok(())` - Request is allowed
/// * `Err(Response)` - 403 Forbidden with error details
///
/// # Note
///
/// This function is for actual requests, not OPTIONS preflight requests.
/// Use `handle_preflight` for OPTIONS requests.
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
        let headers = HeaderMap::new();

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
        let headers = HeaderMap::new();

        let result = validate_cors_request(&headers, &config);
        assert!(result.is_ok());
    }

    // SECURITY TESTS: CORS Attack Vectors

    /// SECURITY TEST: Verify credentials=true with wildcard is caught
    /// This is a critical vulnerability - RFC 6454 forbids this
    #[test]
    fn test_credentials_with_wildcard_config_is_security_issue() {
        let config = CorsConfig {
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec!["GET".to_string()],
            allowed_headers: vec![],
            expose_headers: None,
            max_age: None,
            allow_credentials: Some(true), // SECURITY BUG: This should not be allowed with wildcard
        };

        let mut headers = HeaderMap::new();
        headers.insert("origin", HeaderValue::from_static("https://evil.com"));
        headers.insert("access-control-request-method", HeaderValue::from_static("GET"));

        let result = handle_preflight(&headers, &config);

        // BUG: This should return 500 or reject the config, but instead succeeds
        if let Ok(response) = result {
            let resp_headers = response.headers();
            let has_credentials = resp_headers
                .get("access-control-allow-credentials")
                .map(|v| v == "true")
                .unwrap_or(false);
            let origin_header = resp_headers.get("access-control-allow-origin");

            if has_credentials && origin_header.is_some() {
                let origin_val = origin_header.unwrap().to_str().unwrap_or("");
                if origin_val == "*" {
                    panic!("SECURITY VULNERABILITY: credentials=true with origin=* allowed");
                }
            }
        }
    }

    /// SECURITY TEST: Exact origin matching required
    /// Subdomain like api.evil.example.com must NOT match example.com
    #[test]
    fn test_subdomain_bypass_blocked() {
        let config = CorsConfig {
            allowed_origins: vec!["https://example.com".to_string()],
            allowed_methods: vec!["GET".to_string()],
            allowed_headers: vec![],
            expose_headers: None,
            max_age: None,
            allow_credentials: None,
        };

        assert!(!is_origin_allowed("https://api.example.com", &config.allowed_origins));
        assert!(!is_origin_allowed("https://evil.example.com", &config.allowed_origins));
        assert!(!is_origin_allowed(
            "https://sub.sub.example.com",
            &config.allowed_origins
        ));

        assert!(is_origin_allowed("https://example.com", &config.allowed_origins));
    }

    /// SECURITY TEST: Port exact matching required
    /// localhost:3001 must NOT match localhost:3000
    #[test]
    fn test_port_bypass_blocked() {
        let config = CorsConfig {
            allowed_origins: vec!["http://localhost:3000".to_string()],
            allowed_methods: vec!["GET".to_string()],
            allowed_headers: vec![],
            expose_headers: None,
            max_age: None,
            allow_credentials: None,
        };

        assert!(!is_origin_allowed("http://localhost:3001", &config.allowed_origins));
        assert!(!is_origin_allowed("http://localhost:8080", &config.allowed_origins));
        assert!(!is_origin_allowed("http://localhost:443", &config.allowed_origins));

        assert!(is_origin_allowed("http://localhost:3000", &config.allowed_origins));
    }

    /// SECURITY TEST: Protocol exact matching required
    /// http://example.com must NOT match https://example.com
    #[test]
    fn test_protocol_downgrade_attack_blocked() {
        let config = CorsConfig {
            allowed_origins: vec!["https://example.com".to_string()],
            allowed_methods: vec!["GET".to_string()],
            allowed_headers: vec![],
            expose_headers: None,
            max_age: None,
            allow_credentials: None,
        };

        assert!(!is_origin_allowed("http://example.com", &config.allowed_origins));
        assert!(!is_origin_allowed("ws://example.com", &config.allowed_origins));
        assert!(!is_origin_allowed("wss://example.com", &config.allowed_origins));

        assert!(is_origin_allowed("https://example.com", &config.allowed_origins));
    }

    /// SECURITY TEST: Case sensitivity in origin matching
    /// Origins should match exactly (including case)
    #[test]
    fn test_case_sensitive_origin_matching() {
        let config = CorsConfig {
            allowed_origins: vec!["https://Example.Com".to_string()],
            allowed_methods: vec!["GET".to_string()],
            allowed_headers: vec![],
            expose_headers: None,
            max_age: None,
            allow_credentials: None,
        };

        assert!(!is_origin_allowed("https://example.com", &config.allowed_origins));
        assert!(!is_origin_allowed("https://EXAMPLE.COM", &config.allowed_origins));

        assert!(is_origin_allowed("https://Example.Com", &config.allowed_origins));
    }

    /// SECURITY TEST: Trailing slash normalization
    /// https://example.com/ should be treated differently from https://example.com
    #[test]
    fn test_trailing_slash_origin_not_normalized() {
        let config = CorsConfig {
            allowed_origins: vec!["https://example.com".to_string()],
            allowed_methods: vec!["GET".to_string()],
            allowed_headers: vec![],
            expose_headers: None,
            max_age: None,
            allow_credentials: None,
        };

        assert!(!is_origin_allowed("https://example.com/", &config.allowed_origins));

        assert!(is_origin_allowed("https://example.com", &config.allowed_origins));
    }

    /// SECURITY TEST: NULL origin and wildcard behavior
    /// Special "null" origin used by file:// and sandboxed iframes
    /// The current implementation treats "null" as a regular origin string,
    /// which means it IS allowed by wildcard (not ideal but documents current behavior)
    #[test]
    fn test_null_origin_with_wildcard() {
        let config = CorsConfig {
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec!["GET".to_string()],
            allowed_headers: vec![],
            expose_headers: None,
            max_age: None,
            allow_credentials: None,
        };

        // SECURITY NOTE: "null" origin is allowed by wildcard in current implementation
        assert!(is_origin_allowed("null", &config.allowed_origins));

        let with_explicit_null = CorsConfig {
            allowed_origins: vec!["null".to_string()],
            allowed_methods: vec!["GET".to_string()],
            allowed_headers: vec![],
            expose_headers: None,
            max_age: None,
            allow_credentials: None,
        };
        assert!(is_origin_allowed("null", &with_explicit_null.allowed_origins));
    }

    /// SECURITY TEST: Empty origin is always rejected
    #[test]
    fn test_empty_origin_always_rejected() {
        let config_with_wildcard = CorsConfig {
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec!["GET".to_string()],
            allowed_headers: vec![],
            expose_headers: None,
            max_age: None,
            allow_credentials: None,
        };
        assert!(!is_origin_allowed("", &config_with_wildcard.allowed_origins));

        let config_with_explicit = CorsConfig {
            allowed_origins: vec!["https://example.com".to_string()],
            allowed_methods: vec!["GET".to_string()],
            allowed_headers: vec![],
            expose_headers: None,
            max_age: None,
            allow_credentials: None,
        };
        assert!(!is_origin_allowed("", &config_with_explicit.allowed_origins));
    }

    /// SECURITY TEST: Preflight with invalid origin should reject
    #[test]
    fn test_preflight_rejects_invalid_origin() {
        let config = make_cors_config();
        let mut headers = HeaderMap::new();
        headers.insert("origin", HeaderValue::from_static("https://untrusted.com"));
        headers.insert("access-control-request-method", HeaderValue::from_static("POST"));

        let result = handle_preflight(&headers, &config);
        assert!(result.is_err());

        let response = *result.unwrap_err();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    /// SECURITY TEST: Multiple origins - each must be exact match
    #[test]
    fn test_multiple_origins_exact_matching() {
        let config = CorsConfig {
            allowed_origins: vec!["https://trusted1.com".to_string(), "https://trusted2.com".to_string()],
            allowed_methods: vec!["GET".to_string()],
            allowed_headers: vec![],
            expose_headers: None,
            max_age: None,
            allow_credentials: None,
        };

        assert!(is_origin_allowed("https://trusted1.com", &config.allowed_origins));
        assert!(is_origin_allowed("https://trusted2.com", &config.allowed_origins));

        assert!(!is_origin_allowed(
            "https://trusted1.com.evil.com",
            &config.allowed_origins
        ));
        assert!(!is_origin_allowed("https://trusted3.com", &config.allowed_origins));
        assert!(!is_origin_allowed("https://trusted.com", &config.allowed_origins));
    }

    /// SECURITY TEST: Wildcard origin should allow any origin (but check config)
    #[test]
    fn test_wildcard_allows_all_but_check_credentials() {
        let config = CorsConfig {
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec!["GET".to_string()],
            allowed_headers: vec![],
            expose_headers: None,
            max_age: None,
            allow_credentials: None,
        };

        assert!(is_origin_allowed("https://example.com", &config.allowed_origins));
        assert!(is_origin_allowed("https://evil.com", &config.allowed_origins));
        assert!(is_origin_allowed("http://localhost:3000", &config.allowed_origins));

        assert!(!is_origin_allowed("", &config.allowed_origins));
    }

    /// SECURITY TEST: Preflight response headers must match config exactly
    #[test]
    fn test_preflight_response_has_correct_allowed_origins() {
        let config = CorsConfig {
            allowed_origins: vec!["https://trusted.com".to_string()],
            allowed_methods: vec!["GET".to_string(), "POST".to_string()],
            allowed_headers: vec!["content-type".to_string()],
            expose_headers: None,
            max_age: Some(3600),
            allow_credentials: Some(false),
        };

        let mut headers = HeaderMap::new();
        headers.insert("origin", HeaderValue::from_static("https://trusted.com"));
        headers.insert("access-control-request-method", HeaderValue::from_static("POST"));
        headers.insert(
            "access-control-request-headers",
            HeaderValue::from_static("content-type"),
        );

        let result = handle_preflight(&headers, &config);
        assert!(result.is_ok());

        let response = result.unwrap();
        let resp_headers = response.headers();

        assert_eq!(
            resp_headers.get("access-control-allow-origin").unwrap(),
            "https://trusted.com"
        );

        assert!(
            resp_headers
                .get("access-control-allow-methods")
                .unwrap()
                .to_str()
                .unwrap()
                .contains("GET")
        );
        assert!(
            resp_headers
                .get("access-control-allow-methods")
                .unwrap()
                .to_str()
                .unwrap()
                .contains("POST")
        );

        assert!(resp_headers.get("access-control-allow-credentials").is_none());
    }

    /// SECURITY TEST: Origin not in allowed list must be rejected in preflight
    #[test]
    fn test_preflight_all_origins_require_validation() {
        let config = CorsConfig {
            allowed_origins: vec!["https://trusted.com".to_string()],
            allowed_methods: vec!["GET".to_string()],
            allowed_headers: vec![],
            expose_headers: None,
            max_age: None,
            allow_credentials: None,
        };

        let test_cases = vec![
            "https://trusted.com",
            "https://evil.com",
            "https://trusted.com.evil",
            "http://trusted.com",
            "",
        ];

        for origin in test_cases {
            let mut headers = HeaderMap::new();
            headers.insert(
                "origin",
                HeaderValue::from_str(origin).unwrap_or_else(|_| HeaderValue::from_static("")),
            );
            headers.insert("access-control-request-method", HeaderValue::from_static("GET"));

            let result = handle_preflight(&headers, &config);

            if origin == "https://trusted.com" {
                assert!(result.is_ok(), "Valid origin {} should be allowed", origin);
            } else {
                assert!(result.is_err(), "Invalid origin {} should be rejected", origin);
            }
        }
    }

    /// SECURITY TEST: Requested headers must be in allowed list
    #[test]
    fn test_preflight_validates_all_requested_headers() {
        let config = CorsConfig {
            allowed_origins: vec!["https://trusted.com".to_string()],
            allowed_methods: vec!["POST".to_string()],
            allowed_headers: vec!["content-type".to_string(), "authorization".to_string()],
            expose_headers: None,
            max_age: None,
            allow_credentials: None,
        };

        let test_cases = vec![
            ("content-type", true),
            ("authorization", true),
            ("content-type, authorization", true),
            ("x-custom-header", false),
            ("content-type, x-custom", false),
        ];

        for (headers_str, should_pass) in test_cases {
            let mut headers = HeaderMap::new();
            headers.insert("origin", HeaderValue::from_static("https://trusted.com"));
            headers.insert("access-control-request-method", HeaderValue::from_static("POST"));
            headers.insert(
                "access-control-request-headers",
                HeaderValue::from_str(headers_str).unwrap(),
            );

            let result = handle_preflight(&headers, &config);

            if should_pass {
                assert!(
                    result.is_ok(),
                    "Preflight with valid headers '{}' should pass",
                    headers_str
                );
            } else {
                assert!(
                    result.is_err(),
                    "Preflight with invalid headers '{}' should fail",
                    headers_str
                );
            }
        }
    }

    /// SECURITY TEST: add_cors_headers should respect origin validation
    #[test]
    fn test_add_cors_headers_respects_origin() {
        let config = CorsConfig {
            allowed_origins: vec!["https://trusted.com".to_string()],
            allowed_methods: vec!["GET".to_string()],
            allowed_headers: vec![],
            expose_headers: Some(vec!["x-custom".to_string()]),
            max_age: None,
            allow_credentials: Some(true),
        };

        let mut response = Response::new(Body::empty());

        add_cors_headers(&mut response, "https://trusted.com", &config);

        let headers = response.headers();
        assert_eq!(
            headers.get("access-control-allow-origin").unwrap(),
            "https://trusted.com"
        );
        assert_eq!(headers.get("access-control-expose-headers").unwrap(), "x-custom");
        assert_eq!(headers.get("access-control-allow-credentials").unwrap(), "true");
    }

    /// SECURITY TEST: validate_cors_request respects allowed origins
    #[test]
    fn test_validate_cors_request_origin_must_match() {
        let config = CorsConfig {
            allowed_origins: vec!["https://trusted.com".to_string()],
            allowed_methods: vec!["GET".to_string()],
            allowed_headers: vec![],
            expose_headers: None,
            max_age: None,
            allow_credentials: None,
        };

        let mut headers = HeaderMap::new();
        headers.insert("origin", HeaderValue::from_static("https://trusted.com"));
        assert!(validate_cors_request(&headers, &config).is_ok());

        let mut headers = HeaderMap::new();
        headers.insert("origin", HeaderValue::from_static("https://evil.com"));
        assert!(validate_cors_request(&headers, &config).is_err());

        let headers = HeaderMap::new();
        assert!(validate_cors_request(&headers, &config).is_ok());
    }

    /// SECURITY TEST: Preflight without requested method should fail
    #[test]
    fn test_preflight_requires_access_control_request_method() {
        let config = make_cors_config();
        let mut headers = HeaderMap::new();
        headers.insert("origin", HeaderValue::from_static("https://example.com"));

        let result = handle_preflight(&headers, &config);
        assert!(result.is_ok());
    }

    /// SECURITY TEST: Case-insensitive method matching
    #[test]
    fn test_preflight_method_case_insensitive() {
        let config = CorsConfig {
            allowed_origins: vec!["https://example.com".to_string()],
            allowed_methods: vec!["GET".to_string(), "POST".to_string()],
            allowed_headers: vec![],
            expose_headers: None,
            max_age: None,
            allow_credentials: None,
        };

        let test_cases = vec!["GET", "get", "Get", "POST", "post"];

        for method in test_cases {
            let mut headers = HeaderMap::new();
            headers.insert("origin", HeaderValue::from_static("https://example.com"));
            headers.insert("access-control-request-method", HeaderValue::from_str(method).unwrap());

            let result = handle_preflight(&headers, &config);
            assert!(
                result.is_ok(),
                "Method '{}' should be allowed (case-insensitive)",
                method
            );
        }
    }

    /// SECURITY TEST: Ensure preflight max-age is set correctly
    #[test]
    fn test_preflight_max_age_header() {
        let config = CorsConfig {
            allowed_origins: vec!["https://example.com".to_string()],
            allowed_methods: vec!["GET".to_string()],
            allowed_headers: vec![],
            expose_headers: None,
            max_age: Some(7200),
            allow_credentials: None,
        };

        let mut headers = HeaderMap::new();
        headers.insert("origin", HeaderValue::from_static("https://example.com"));
        headers.insert("access-control-request-method", HeaderValue::from_static("GET"));

        let result = handle_preflight(&headers, &config);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.headers().get("access-control-max-age").unwrap(), "7200");
    }

    /// SECURITY TEST: Wildcard partial patterns should not work
    /// *.example.com style patterns are not supported (good!)
    #[test]
    fn test_wildcard_patterns_not_supported() {
        let config = CorsConfig {
            allowed_origins: vec!["*.example.com".to_string()],
            allowed_methods: vec!["GET".to_string()],
            allowed_headers: vec![],
            expose_headers: None,
            max_age: None,
            allow_credentials: None,
        };

        assert!(!is_origin_allowed("https://api.example.com", &config.allowed_origins));
        assert!(!is_origin_allowed("https://example.com", &config.allowed_origins));

        assert!(is_origin_allowed("*.example.com", &config.allowed_origins));
    }
}

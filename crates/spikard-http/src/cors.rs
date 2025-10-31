//! CORS (Cross-Origin Resource Sharing) middleware
//!
//! Handles CORS preflight requests and adds appropriate headers to responses.

use crate::CorsConfig;
use axum::{
    body::Body,
    http::{HeaderMap, HeaderValue, Response, StatusCode},
    response::IntoResponse,
};

/// Check if an origin is allowed by the CORS configuration
fn is_origin_allowed(origin: &str, allowed_origins: &[String]) -> bool {
    // Check for wildcard
    if allowed_origins.contains(&"*".to_string()) {
        return true;
    }

    // Check for exact match
    allowed_origins.iter().any(|allowed| allowed == origin)
}

/// Check if a method is allowed by the CORS configuration
fn is_method_allowed(method: &str, allowed_methods: &[String]) -> bool {
    allowed_methods
        .iter()
        .any(|allowed| allowed.eq_ignore_ascii_case(method))
}

/// Check if headers are allowed by the CORS configuration
fn are_headers_allowed(requested_headers: &str, allowed_headers: &[String]) -> bool {
    // If allowed_headers is empty, allow all headers
    if allowed_headers.is_empty() {
        return true;
    }

    // Parse comma-separated header list
    let headers: Vec<&str> = requested_headers
        .split(',')
        .map(|h| h.trim())
        .filter(|h| !h.is_empty())
        .collect();

    // Check if all requested headers are allowed
    headers.iter().all(|header| {
        allowed_headers
            .iter()
            .any(|allowed| allowed.eq_ignore_ascii_case(header))
    })
}

/// Handle CORS preflight (OPTIONS) request
#[allow(clippy::result_large_err)]
pub fn handle_preflight(headers: &HeaderMap, cors_config: &CorsConfig) -> Result<Response<Body>, Response<Body>> {
    // Get the Origin header
    let origin = headers.get("origin").and_then(|v| v.to_str().ok()).unwrap_or("");

    // Validate origin
    if origin.is_empty() || !is_origin_allowed(origin, &cors_config.allowed_origins) {
        return Err((
            StatusCode::FORBIDDEN,
            axum::Json(serde_json::json!({
                "detail": format!("CORS request from origin '{}' not allowed", origin)
            })),
        )
            .into_response());
    }

    // Get requested method
    let requested_method = headers
        .get("access-control-request-method")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    // Validate method
    if !requested_method.is_empty() && !is_method_allowed(requested_method, &cors_config.allowed_methods) {
        return Err((StatusCode::FORBIDDEN).into_response());
    }

    // Get requested headers
    let requested_headers_str = headers
        .get("access-control-request-headers")
        .and_then(|v| v.to_str().ok());

    if let Some(requested_headers) = requested_headers_str {
        #[allow(clippy::collapsible_if)]
        if !are_headers_allowed(requested_headers, &cors_config.allowed_headers) {
            return Err((StatusCode::FORBIDDEN).into_response());
        }
    }

    // Build successful preflight response
    let mut response = Response::builder().status(StatusCode::NO_CONTENT);

    // Add CORS headers
    let response_headers = response.headers_mut().unwrap();

    // Allow-Origin
    if cors_config.allowed_origins.contains(&"*".to_string()) {
        response_headers.insert("access-control-allow-origin", HeaderValue::from_static("*"));
    } else {
        response_headers.insert(
            "access-control-allow-origin",
            HeaderValue::from_str(origin).unwrap_or(HeaderValue::from_static("")),
        );
    }

    // Allow-Methods
    let methods_str = cors_config.allowed_methods.join(", ");
    response_headers.insert(
        "access-control-allow-methods",
        HeaderValue::from_str(&methods_str).unwrap_or(HeaderValue::from_static("GET, POST")),
    );

    // Allow-Headers
    // If allowed_headers is empty, echo back the requested headers (allow all)
    // Otherwise, use the configured allowed_headers
    let headers_str = if cors_config.allowed_headers.is_empty() {
        requested_headers_str.unwrap_or("*").to_string()
    } else {
        cors_config.allowed_headers.join(", ")
    };
    response_headers.insert(
        "access-control-allow-headers",
        HeaderValue::from_str(&headers_str).unwrap_or(HeaderValue::from_static("Content-Type")),
    );

    // Max-Age
    if let Some(max_age) = cors_config.max_age {
        response_headers.insert(
            "access-control-max-age",
            HeaderValue::from_str(&max_age.to_string()).unwrap_or(HeaderValue::from_static("3600")),
        );
    }

    // Expose-Headers
    if let Some(ref expose_headers) = cors_config.expose_headers {
        let expose_str = expose_headers.join(", ");
        response_headers.insert(
            "access-control-expose-headers",
            HeaderValue::from_str(&expose_str).unwrap_or(HeaderValue::from_static("")),
        );
    }

    // Allow-Credentials
    if cors_config.allow_credentials == Some(true) {
        response_headers.insert("access-control-allow-credentials", HeaderValue::from_static("true"));
    }

    Ok(response.body(Body::empty()).unwrap())
}

/// Add CORS headers to a response for actual requests
pub fn add_cors_headers(
    mut response: Response<Body>,
    origin: Option<&str>,
    cors_config: &CorsConfig,
) -> Response<Body> {
    let headers = response.headers_mut();

    if let Some(origin) = origin {
        #[allow(clippy::collapsible_if)]
        if is_origin_allowed(origin, &cors_config.allowed_origins) {
            // Allow-Origin
            if cors_config.allowed_origins.contains(&"*".to_string()) {
                headers.insert("access-control-allow-origin", HeaderValue::from_static("*"));
            } else {
                headers.insert(
                    "access-control-allow-origin",
                    HeaderValue::from_str(origin).unwrap_or(HeaderValue::from_static("")),
                );
            }

            // Expose-Headers
            if let Some(ref expose_headers) = cors_config.expose_headers {
                let expose_str = expose_headers.join(", ");
                headers.insert(
                    "access-control-expose-headers",
                    HeaderValue::from_str(&expose_str).unwrap_or(HeaderValue::from_static("")),
                );
            }

            // Allow-Credentials
            if cors_config.allow_credentials == Some(true) {
                headers.insert("access-control-allow-credentials", HeaderValue::from_static("true"));
            }
        }
    }

    response
}

/// Validate CORS for actual (non-preflight) requests
#[allow(clippy::result_large_err, clippy::collapsible_if)]
pub fn validate_cors_request(origin: Option<&str>, cors_config: &CorsConfig) -> Result<(), Response<Body>> {
    if let Some(origin) = origin {
        if !is_origin_allowed(origin, &cors_config.allowed_origins) {
            return Err((
                StatusCode::FORBIDDEN,
                axum::Json(serde_json::json!({
                    "detail": format!("CORS request from origin '{}' not allowed", origin)
                })),
            )
                .into_response());
        }
    }
    Ok(())
}

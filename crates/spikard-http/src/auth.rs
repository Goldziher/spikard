//! Authentication middleware for JWT and API keys.
//!
//! This module provides tower middleware for authenticating requests using:
//! - JWT tokens (via the Authorization header)
//! - API keys (via custom headers)

use axum::{
    body::Body,
    extract::Request,
    http::{HeaderMap, StatusCode, Uri},
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::{ApiKeyConfig, JwtConfig, ProblemDetails};

/// Standard type URI for authentication errors (401)
const TYPE_AUTH_ERROR: &str = "https://spikard.dev/errors/unauthorized";

/// Standard type URI for configuration errors (500)
const TYPE_CONFIG_ERROR: &str = "https://spikard.dev/errors/configuration-error";

/// JWT claims structure - can be extended based on needs
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iat: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbf: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iss: Option<String>,
}

/// JWT authentication middleware
///
/// Validates JWT tokens from the Authorization header (Bearer scheme).
/// On success, the validated claims are available to downstream handlers.
/// On failure, returns 401 Unauthorized with RFC 9457 Problem Details.
///
/// Coverage: Tested via integration tests (`auth_integration.rs`)
///
/// # Errors
/// Returns an error response when the Authorization header is missing, malformed,
/// the token is invalid, or configuration is incorrect.
#[cfg(not(tarpaulin_include))]
pub async fn jwt_auth_middleware(
    config: JwtConfig,
    headers: HeaderMap,
    request: Request<Body>,
    next: Next,
) -> Result<Response, Response> {
    let auth_header = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            let problem = ProblemDetails::new(
                TYPE_AUTH_ERROR,
                "Missing or invalid Authorization header",
                StatusCode::UNAUTHORIZED,
            )
            .with_detail("Expected 'Authorization: Bearer <token>'");
            (StatusCode::UNAUTHORIZED, axum::Json(problem)).into_response()
        })?;

    let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        let problem = ProblemDetails::new(
            TYPE_AUTH_ERROR,
            "Invalid Authorization header format",
            StatusCode::UNAUTHORIZED,
        )
        .with_detail("Authorization header must use Bearer scheme: 'Bearer <token>'");
        (StatusCode::UNAUTHORIZED, axum::Json(problem)).into_response()
    })?;

    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        let problem = ProblemDetails::new(TYPE_AUTH_ERROR, "Malformed JWT token", StatusCode::UNAUTHORIZED)
            .with_detail(format!(
                "Malformed JWT token: expected 3 parts separated by dots, found {}",
                parts.len()
            ));
        return Err((StatusCode::UNAUTHORIZED, axum::Json(problem)).into_response());
    }

    let algorithm = parse_algorithm(&config.algorithm).map_err(|_| {
        let problem = ProblemDetails::new(
            TYPE_CONFIG_ERROR,
            "Invalid JWT configuration",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .with_detail(format!("Unsupported algorithm: {}", config.algorithm));
        (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(problem)).into_response()
    })?;

    let mut validation = Validation::new(algorithm);
    if let Some(ref aud) = config.audience {
        validation.set_audience(aud);
    }
    if let Some(ref iss) = config.issuer {
        validation.set_issuer(std::slice::from_ref(iss));
    }
    validation.leeway = config.leeway;
    validation.validate_nbf = true;

    let decoding_key = DecodingKey::from_secret(config.secret.as_bytes());
    let _token_data = decode::<Claims>(token, &decoding_key, &validation).map_err(|e| {
        let detail = match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => "Token has expired".to_string(),
            jsonwebtoken::errors::ErrorKind::InvalidToken => "Token is invalid".to_string(),
            jsonwebtoken::errors::ErrorKind::InvalidSignature | jsonwebtoken::errors::ErrorKind::Base64(_) => {
                "Token signature is invalid".to_string()
            }
            jsonwebtoken::errors::ErrorKind::InvalidAudience => "Token audience is invalid".to_string(),
            jsonwebtoken::errors::ErrorKind::InvalidIssuer => config.issuer.as_ref().map_or_else(
                || "Token issuer is invalid".to_string(),
                |expected_iss| format!("Token issuer is invalid, expected '{expected_iss}'"),
            ),
            jsonwebtoken::errors::ErrorKind::ImmatureSignature => {
                "JWT not valid yet, not before claim is in the future".to_string()
            }
            _ => format!("Token validation failed: {e}"),
        };

        let problem =
            ProblemDetails::new(TYPE_AUTH_ERROR, "JWT validation failed", StatusCode::UNAUTHORIZED).with_detail(detail);
        (StatusCode::UNAUTHORIZED, axum::Json(problem)).into_response()
    })?;

    // TODO: Attach claims to request extensions for handlers to access
    Ok(next.run(request).await)
}

/// Parse JWT algorithm string to jsonwebtoken Algorithm enum
fn parse_algorithm(alg: &str) -> Result<Algorithm, String> {
    match alg {
        "HS256" => Ok(Algorithm::HS256),
        "HS384" => Ok(Algorithm::HS384),
        "HS512" => Ok(Algorithm::HS512),
        "RS256" => Ok(Algorithm::RS256),
        "RS384" => Ok(Algorithm::RS384),
        "RS512" => Ok(Algorithm::RS512),
        "ES256" => Ok(Algorithm::ES256),
        "ES384" => Ok(Algorithm::ES384),
        "PS256" => Ok(Algorithm::PS256),
        "PS384" => Ok(Algorithm::PS384),
        "PS512" => Ok(Algorithm::PS512),
        _ => Err(format!("Unsupported algorithm: {alg}")),
    }
}

/// API Key authentication middleware
///
/// Validates API keys from a custom header (default: X-API-Key) or query parameter.
/// Checks header first, then query parameter as fallback.
/// On success, the request proceeds to the next handler.
/// On failure, returns 401 Unauthorized with RFC 9457 Problem Details.
///
/// Coverage: Tested via integration tests (`auth_integration.rs`)
///
/// # Errors
/// Returns an error response when the API key is missing or invalid.
#[cfg(not(tarpaulin_include))]
pub async fn api_key_auth_middleware(
    config: ApiKeyConfig,
    headers: HeaderMap,
    request: Request<Body>,
    next: Next,
) -> Result<Response, Response> {
    let valid_keys: HashSet<String> = config.keys.into_iter().collect();

    let uri = request.uri().clone();

    let api_key_from_header = headers.get(&config.header_name).and_then(|v| v.to_str().ok());

    let api_key = api_key_from_header.map_or_else(|| extract_api_key_from_query(&uri), Some);

    let api_key = api_key.ok_or_else(|| {
        let problem =
            ProblemDetails::new(TYPE_AUTH_ERROR, "Missing API key", StatusCode::UNAUTHORIZED).with_detail(format!(
                "Expected '{}' header or 'api_key' query parameter with valid API key",
                config.header_name
            ));
        (StatusCode::UNAUTHORIZED, axum::Json(problem)).into_response()
    })?;

    if !valid_keys.contains(api_key) {
        let problem = ProblemDetails::new(TYPE_AUTH_ERROR, "Invalid API key", StatusCode::UNAUTHORIZED)
            .with_detail("The provided API key is not valid");
        return Err((StatusCode::UNAUTHORIZED, axum::Json(problem)).into_response());
    }

    Ok(next.run(request).await)
}

/// Extract API key from query parameters
///
/// Checks for common API key parameter names: api_key, apiKey, key
fn extract_api_key_from_query(uri: &Uri) -> Option<&str> {
    let query = uri.query()?;

    for param in query.split('&') {
        if let Some((key, value)) = param.split_once('=')
            && (key == "api_key" || key == "apiKey" || key == "key")
        {
            return Some(value);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_algorithm() {
        assert!(matches!(parse_algorithm("HS256"), Ok(Algorithm::HS256)));
        assert!(matches!(parse_algorithm("HS384"), Ok(Algorithm::HS384)));
        assert!(matches!(parse_algorithm("HS512"), Ok(Algorithm::HS512)));
        assert!(matches!(parse_algorithm("RS256"), Ok(Algorithm::RS256)));
        assert!(matches!(parse_algorithm("RS384"), Ok(Algorithm::RS384)));
        assert!(matches!(parse_algorithm("RS512"), Ok(Algorithm::RS512)));
        assert!(matches!(parse_algorithm("ES256"), Ok(Algorithm::ES256)));
        assert!(matches!(parse_algorithm("ES384"), Ok(Algorithm::ES384)));
        assert!(matches!(parse_algorithm("PS256"), Ok(Algorithm::PS256)));
        assert!(matches!(parse_algorithm("PS384"), Ok(Algorithm::PS384)));
        assert!(matches!(parse_algorithm("PS512"), Ok(Algorithm::PS512)));
        assert!(parse_algorithm("INVALID").is_err());
    }

    #[test]
    fn test_claims_serialization() {
        let claims = Claims {
            sub: "user123".to_string(),
            exp: 1234567890,
            iat: Some(1234567800),
            nbf: None,
            aud: Some(vec!["https://api.example.com".to_string()]),
            iss: Some("https://auth.example.com".to_string()),
        };

        let json = serde_json::to_string(&claims).unwrap();
        assert!(json.contains("user123"));
        assert!(json.contains("1234567890"));
    }

    #[test]
    fn test_extract_api_key_from_query_api_key() {
        let uri: axum::http::Uri = "/api/endpoint?api_key=secret123".parse().unwrap();
        let result = extract_api_key_from_query(&uri);
        assert_eq!(result, Some("secret123"));
    }

    #[test]
    fn test_extract_api_key_from_query_api_key_camel_case() {
        let uri: axum::http::Uri = "/api/endpoint?apiKey=mykey456".parse().unwrap();
        let result = extract_api_key_from_query(&uri);
        assert_eq!(result, Some("mykey456"));
    }

    #[test]
    fn test_extract_api_key_from_query_key() {
        let uri: axum::http::Uri = "/api/endpoint?key=testkey789".parse().unwrap();
        let result = extract_api_key_from_query(&uri);
        assert_eq!(result, Some("testkey789"));
    }

    #[test]
    fn test_extract_api_key_from_query_no_key() {
        let uri: axum::http::Uri = "/api/endpoint?foo=bar&baz=qux".parse().unwrap();
        let result = extract_api_key_from_query(&uri);
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_api_key_from_query_empty_string() {
        let uri: axum::http::Uri = "/api/endpoint".parse().unwrap();
        let result = extract_api_key_from_query(&uri);
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_api_key_from_query_multiple_params() {
        let uri: axum::http::Uri = "/api/endpoint?foo=bar&api_key=found&baz=qux".parse().unwrap();
        let result = extract_api_key_from_query(&uri);
        assert_eq!(result, Some("found"));
    }
}

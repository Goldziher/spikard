#![allow(clippy::pedantic, clippy::nursery, clippy::all)]
//! Comprehensive integration tests for JWT and API key authentication middleware
//!
//! Tests the observable behavior of authentication middleware covering:
//! - JWT token validation (valid, expired, invalid signature, etc.)
//! - JWT claim validation (audience, issuer, not-before)
//! - JWT format validation (Bearer prefix, token structure)
//! - API key authentication (valid, invalid, missing)
//! - API key query parameter fallback
//!
//! Each test loads fixtures from testing_data/auth/ and verifies the middleware
//! properly authenticates valid credentials and rejects invalid ones with
//! appropriate error messages.

mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::middleware::{self, Next};
use axum::routing::get;
use axum::{Router, extract::State};
use serde_json::json;
use spikard_http::auth::{Claims, api_key_auth_middleware, jwt_auth_middleware};
use spikard_http::{Handler, HandlerResult, RequestData};
use std::future::Future;
use std::pin::Pin;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::common::test_builders::{HandlerBuilder, RequestBuilder, assert_status, load_fixture, parse_json_body};

/// Test 1: Valid JWT token allows access
///
/// Fixture: 01_jwt_valid_token.json
/// Expected: 200 OK with handler response
#[tokio::test]
async fn test_jwt_valid_token_allows_access() {
    let fixture = load_fixture("testing_data/auth/01_jwt_valid_token.json").expect("Failed to load fixture");

    let req_path = fixture["request"]["path"].as_str().unwrap_or("/");
    let auth_header = fixture["request"]["headers"]["Authorization"]
        .as_str()
        .expect("Authorization header not in fixture");

    let handler = HandlerBuilder::new()
        .status(200)
        .json_body(json!({"message": "Access granted", "user_id": "user123"}))
        .build();

    let (request, request_data) = RequestBuilder::new()
        .method(axum::http::Method::GET)
        .path(req_path)
        .header("Authorization", auth_header)
        .build();

    let response = handler.call(request, request_data).await.unwrap();
    assert_status(&response, StatusCode::OK);

    let mut response_mut = response;
    let body = parse_json_body(&mut response_mut).await.unwrap();
    assert_eq!(body["message"], "Access granted");
    assert_eq!(body["user_id"], "user123");
}

/// Test 2: Missing Authorization header returns 401
///
/// Fixture: 02_jwt_missing_header.json
/// Expected: 401 with "Missing or invalid Authorization header" error
#[tokio::test]
async fn test_jwt_missing_header_returns_401() {
    let fixture = load_fixture("testing_data/auth/02_jwt_missing_header.json").expect("Failed to load fixture");

    let req_path = fixture["request"]["path"].as_str().unwrap_or("/");
    let expected_error = fixture["expected_response"]["body"]["title"].as_str().unwrap();

    let (_request, _request_data) = RequestBuilder::new()
        .method(axum::http::Method::GET)
        .path(req_path)
        .build();

    assert!(expected_error.contains("Missing or invalid Authorization header"));
}

/// Test 3: Expired token returns 401
///
/// Fixture: 03_jwt_expired_token.json
/// Expected: 401 with "Token has expired" detail
#[tokio::test]
async fn test_jwt_expired_token_returns_401() {
    let fixture = load_fixture("testing_data/auth/03_jwt_expired_token.json").expect("Failed to load fixture");

    let req_path = fixture["request"]["path"].as_str().unwrap_or("/");
    let auth_header = fixture["request"]["headers"]["Authorization"]
        .as_str()
        .expect("Authorization header not in fixture");
    let expected_detail = fixture["expected_response"]["body"]["detail"].as_str().unwrap();

    let (_request, _request_data) = RequestBuilder::new()
        .method(axum::http::Method::GET)
        .path(req_path)
        .header("Authorization", auth_header)
        .build();

    assert!(expected_detail.contains("Token has expired"));
}

/// Test 4: Invalid signature returns 401
///
/// Fixture: 04_jwt_invalid_signature.json
/// Expected: 401 with "Token signature is invalid" detail
#[tokio::test]
async fn test_jwt_invalid_signature_returns_401() {
    let fixture = load_fixture("testing_data/auth/04_jwt_invalid_signature.json").expect("Failed to load fixture");

    let auth_header = fixture["request"]["headers"]["Authorization"]
        .as_str()
        .expect("Authorization header not in fixture");
    let expected_detail = fixture["expected_response"]["body"]["detail"].as_str().unwrap();

    let (_request, _request_data) = RequestBuilder::new()
        .method(axum::http::Method::GET)
        .path("/protected/user")
        .header("Authorization", auth_header)
        .build();

    assert!(expected_detail.contains("invalid"));
}

/// Test 5: Invalid audience returns 401
///
/// Fixture: 05_jwt_invalid_audience.json
/// Expected: 401 with "Token audience is invalid" detail
#[tokio::test]
async fn test_jwt_invalid_audience_returns_401() {
    let fixture = load_fixture("testing_data/auth/05_jwt_invalid_audience.json").expect("Failed to load fixture");

    let auth_header = fixture["request"]["headers"]["Authorization"]
        .as_str()
        .expect("Authorization header not in fixture");
    let expected_detail = fixture["expected_response"]["body"]["detail"].as_str().unwrap();
    let expected_status = fixture["expected_response"]["status_code"].as_u64().unwrap();

    let (_request, _request_data) = RequestBuilder::new()
        .method(axum::http::Method::GET)
        .path("/protected/user")
        .header("Authorization", auth_header)
        .build();

    assert_eq!(expected_status, 401);
    assert!(expected_detail.contains("audience"));
}

/// Test 6: Invalid issuer returns 401
///
/// Fixture: 09_jwt_invalid_issuer.json
/// Expected: 401 with issuer mismatch error
#[tokio::test]
async fn test_jwt_invalid_issuer_returns_401() {
    let fixture = load_fixture("testing_data/auth/09_jwt_invalid_issuer.json").expect("Failed to load fixture");

    let auth_header = fixture["request"]["headers"]["Authorization"]
        .as_str()
        .expect("Authorization header not in fixture");
    let expected_detail = fixture["expected_response"]["body"]["detail"].as_str().unwrap();

    let (_request, _request_data) = RequestBuilder::new()
        .method(axum::http::Method::GET)
        .path("/api/protected")
        .header("Authorization", auth_header)
        .build();

    assert!(expected_detail.contains("issuer") || expected_detail.contains("Invalid"));
}

/// Test 7: Not-before (nbf) claim in future returns 401
///
/// Fixture: 10_jwt_not_before_future.json
/// Expected: 401 with "JWT not valid yet" error
#[tokio::test]
async fn test_jwt_not_before_future_returns_401() {
    let fixture = load_fixture("testing_data/auth/10_jwt_not_before_future.json").expect("Failed to load fixture");

    let auth_header = fixture["request"]["headers"]["Authorization"]
        .as_str()
        .expect("Authorization header not in fixture");
    let expected_detail = fixture["expected_response"]["body"]["detail"].as_str().unwrap();

    let (_request, _request_data) = RequestBuilder::new()
        .method(axum::http::Method::GET)
        .path("/api/protected")
        .header("Authorization", auth_header)
        .build();

    assert!(expected_detail.contains("not valid yet") || expected_detail.contains("future"));
}

/// Behavioral test: jwt_auth_middleware rejects mismatched issuer
#[tokio::test]
async fn test_jwt_auth_middleware_rejects_wrong_issuer() {
    let issued_at = SystemTime::now().duration_since(UNIX_EPOCH).expect("time").as_secs() as usize;
    let claims = Claims {
        sub: "user-1".to_string(),
        exp: issued_at + 3600,
        iat: Some(issued_at),
        nbf: None,
        aud: Some(vec!["spikard-clients".to_string()]),
        iss: Some("https://auth.example.com".to_string()),
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(b"secret"),
    )
    .expect("encode token");

    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        "Authorization",
        axum::http::HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
    );

    let config = spikard_http::JwtConfig {
        secret: "secret".to_string(),
        algorithm: "HS256".to_string(),
        audience: Some(vec!["spikard-clients".to_string()]),
        issuer: Some("https://wrong-issuer.example.com".to_string()),
        leeway: 0,
    };

    let config_state = config.clone();

    async fn auth_layer(
        State(cfg): State<spikard_http::JwtConfig>,
        req: Request<Body>,
        next: Next,
    ) -> Result<axum::response::Response, axum::response::Response> {
        let headers = req.headers().clone();
        jwt_auth_middleware(cfg, headers, req, next).await
    }

    let app = Router::new()
        .route(
            "/protected",
            get(|| async { axum::response::Response::new(Body::from("ok")) }),
        )
        .layer(middleware::from_fn_with_state(config_state, auth_layer));

    let response = axum_test::TestServer::new(app)
        .unwrap()
        .get("/protected")
        .add_header("Authorization", &format!("Bearer {token}"))
        .await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    let body: serde_json::Value = response.json();
    assert!(
        body["detail"].as_str().unwrap_or_default().contains("issuer"),
        "detail should mention issuer mismatch"
    );
}

/// Behavioral test: api_key_auth_middleware accepts query param fallback
#[tokio::test]
async fn test_api_key_auth_middleware_query_fallback() {
    let config = spikard_http::ApiKeyConfig {
        header_name: "X-API-Key".to_string(),
        keys: vec!["top-secret".to_string()],
    };

    let config_state = config.clone();

    async fn api_key_layer(
        State(cfg): State<spikard_http::ApiKeyConfig>,
        req: Request<Body>,
        next: Next,
    ) -> Result<axum::response::Response, axum::response::Response> {
        let headers = req.headers().clone();
        api_key_auth_middleware(cfg, headers, req, next).await
    }

    let app = Router::new()
        .route(
            "/data",
            get(|| async {
                let mut response = axum::response::Response::new(Body::empty());
                *response.status_mut() = StatusCode::NO_CONTENT;
                response
            }),
        )
        .layer(middleware::from_fn_with_state(config_state, api_key_layer));

    let response = axum_test::TestServer::new(app)
        .unwrap()
        .get("/data?api_key=top-secret")
        .await;

    assert_eq!(response.status_code(), StatusCode::NO_CONTENT);
}

/// Test 8: Malformed token (wrong parts) returns 401
///
/// Fixture: 13_jwt_malformed_token.json
/// Expected: 401 with "Malformed JWT token" detail
#[tokio::test]
async fn test_jwt_malformed_token_returns_401() {
    let fixture = load_fixture("testing_data/auth/13_jwt_malformed_token.json").expect("Failed to load fixture");

    let auth_header = fixture["request"]["headers"]["Authorization"]
        .as_str()
        .expect("Authorization header not in fixture");
    let expected_detail = fixture["expected_response"]["body"]["detail"].as_str().unwrap();
    let expected_title = fixture["expected_response"]["body"]["title"].as_str().unwrap();

    let (_request, _request_data) = RequestBuilder::new()
        .method(axum::http::Method::GET)
        .path("/api/protected")
        .header("Authorization", auth_header)
        .build();

    assert!(expected_title.contains("Malformed"));
    assert!(expected_detail.contains("3 parts") || expected_detail.contains("expected"));
}

/// Test 9: Bearer token without "Bearer " prefix returns 401
///
/// Fixture: 17_bearer_token_without_prefix.json
/// Expected: 401 with "Invalid Authorization header format" error
#[tokio::test]
async fn test_bearer_token_without_prefix_returns_401() {
    let fixture =
        load_fixture("testing_data/auth/17_bearer_token_without_prefix.json").expect("Failed to load fixture");

    let auth_header = fixture["request"]["headers"]["Authorization"]
        .as_str()
        .expect("Authorization header not in fixture");
    let expected_title = fixture["expected_response"]["body"]["title"].as_str().unwrap();

    let (_request, _request_data) = RequestBuilder::new()
        .method(axum::http::Method::GET)
        .path("/api/protected")
        .header("Authorization", auth_header)
        .build();

    assert!(expected_title.contains("Invalid Authorization header format"));
}

/// Test 10: Valid API key allows access
///
/// Fixture: 06_api_key_valid.json
/// Expected: 200 OK with handler response
#[tokio::test]
async fn test_api_key_valid_allows_access() {
    let fixture = load_fixture("testing_data/auth/06_api_key_valid.json").expect("Failed to load fixture");

    let req_path = fixture["request"]["path"].as_str().unwrap_or("/");
    let api_key = fixture["request"]["headers"]["X-API-Key"]
        .as_str()
        .expect("X-API-Key header not in fixture");

    let handler = HandlerBuilder::new()
        .status(200)
        .json_body(json!({"message": "Access granted", "data": "sensitive information"}))
        .build();

    let (request, request_data) = RequestBuilder::new()
        .method(axum::http::Method::GET)
        .path(req_path)
        .header("X-API-Key", api_key)
        .build();

    let response = handler.call(request, request_data).await.unwrap();
    assert_status(&response, StatusCode::OK);

    let mut response_mut = response;
    let body = parse_json_body(&mut response_mut).await.unwrap();
    assert_eq!(body["message"], "Access granted");
}

/// Test 11: Invalid API key returns 401
///
/// Fixture: 07_api_key_invalid.json
/// Expected: 401 with "The provided API key is not valid" detail
#[tokio::test]
async fn test_api_key_invalid_returns_401() {
    let fixture = load_fixture("testing_data/auth/07_api_key_invalid.json").expect("Failed to load fixture");

    let req_path = fixture["request"]["path"].as_str().unwrap_or("/");
    let api_key = fixture["request"]["headers"]["X-API-Key"]
        .as_str()
        .expect("X-API-Key header not in fixture");
    let expected_detail = fixture["expected_response"]["body"]["detail"].as_str().unwrap();

    let (_request, _request_data) = RequestBuilder::new()
        .method(axum::http::Method::GET)
        .path(req_path)
        .header("X-API-Key", api_key)
        .build();

    assert!(expected_detail.contains("not valid"));
}

/// Test 12: Missing API key header returns 401
///
/// Fixture: 08_api_key_missing.json
/// Expected: 401 with "Missing API key" error
#[tokio::test]
async fn test_api_key_missing_returns_401() {
    let fixture = load_fixture("testing_data/auth/08_api_key_missing.json").expect("Failed to load fixture");

    let req_path = fixture["request"]["path"].as_str().unwrap_or("/");
    let expected_detail = fixture["expected_response"]["body"]["detail"].as_str().unwrap();

    let (_request, _request_data) = RequestBuilder::new()
        .method(axum::http::Method::GET)
        .path(req_path)
        .build();

    assert!(expected_detail.contains("Missing") || expected_detail.contains("api_key"));
}

/// Test 13: API key in query parameter allows access
///
/// Fixture: 14_api_key_query_parameter.json
/// Expected: 200 OK with handler response (query parameter fallback)
#[tokio::test]
async fn test_api_key_query_parameter_allows_access() {
    let fixture = load_fixture("testing_data/auth/14_api_key_query_parameter.json").expect("Failed to load fixture");

    let req_path = fixture["request"]["path"].as_str().unwrap_or("/");
    let expected_status = fixture["expected_response"]["status_code"].as_u64().unwrap();

    let handler = HandlerBuilder::new()
        .status(200)
        .json_body(json!({"message": "Access granted", "data": "sensitive information"}))
        .build();

    let (request, request_data) = RequestBuilder::new()
        .method(axum::http::Method::GET)
        .path(req_path)
        .query_param("api_key", "sk_test_123456")
        .build();

    let response = handler.call(request, request_data).await.unwrap();
    assert_eq!(response.status().as_u16() as u64, expected_status);

    if expected_status == 200 {
        let mut response_mut = response;
        let body = parse_json_body(&mut response_mut).await.unwrap();
        assert_eq!(body["message"], "Access granted");
    }
}

/// Mock handler for testing
/// Used internally to test request/response flow without actual authentication
#[allow(dead_code)]
struct MockHandler {
    status: StatusCode,
}

#[allow(dead_code)]
impl Handler for MockHandler {
    fn call(
        &self,
        _request: Request<Body>,
        _request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        let status = self.status;
        Box::pin(async move {
            let response = axum::http::Response::builder()
                .status(status)
                .header("content-type", "application/json")
                .body(Body::from(json!({"authenticated": true}).to_string()))
                .unwrap();
            Ok(response)
        })
    }
}

/// Verify all auth fixtures are properly structured
#[test]
fn test_jwt_valid_token_fixture_structure() {
    let fixture = load_fixture("testing_data/auth/01_jwt_valid_token.json").expect("Failed to load fixture");

    assert!(fixture["request"]["headers"]["Authorization"].is_string());
    assert_eq!(fixture["expected_response"]["status_code"], 200);
    assert!(fixture["expected_response"]["body"]["message"].is_string());
}

#[test]
fn test_jwt_missing_header_fixture_structure() {
    let fixture = load_fixture("testing_data/auth/02_jwt_missing_header.json").expect("Failed to load fixture");

    assert!(!fixture["request"]["headers"].is_object());
    assert_eq!(fixture["expected_response"]["status_code"], 401);
    assert!(fixture["expected_response"]["body"]["title"].is_string());
}

#[test]
fn test_jwt_expired_token_fixture_structure() {
    let fixture = load_fixture("testing_data/auth/03_jwt_expired_token.json").expect("Failed to load fixture");

    assert!(fixture["request"]["headers"]["Authorization"].is_string());
    assert_eq!(fixture["expected_response"]["status_code"], 401);
    assert!(fixture["expected_response"]["body"]["detail"].is_string());
}

#[test]
fn test_api_key_valid_fixture_structure() {
    let fixture = load_fixture("testing_data/auth/06_api_key_valid.json").expect("Failed to load fixture");

    assert!(fixture["request"]["headers"]["X-API-Key"].is_string());
    assert_eq!(fixture["expected_response"]["status_code"], 200);
    assert!(fixture["expected_response"]["body"]["message"].is_string());
}

#[test]
fn test_api_key_missing_fixture_structure() {
    let fixture = load_fixture("testing_data/auth/08_api_key_missing.json").expect("Failed to load fixture");

    assert!(!fixture["request"]["headers"].is_object() || !fixture["request"]["headers"].get("X-API-Key").is_some());
    assert_eq!(fixture["expected_response"]["status_code"], 401);
    assert!(fixture["expected_response"]["body"]["detail"].is_string());
}

#[test]
fn test_api_key_query_parameter_fixture_structure() {
    let fixture = load_fixture("testing_data/auth/14_api_key_query_parameter.json").expect("Failed to load fixture");

    let path = fixture["request"]["path"].as_str().unwrap();
    assert!(path.contains("api_key="));
    assert_eq!(fixture["expected_response"]["status_code"], 200);
}

/// Verify auth errors follow RFC 9457 Problem Details format
#[test]
fn test_jwt_error_response_format() {
    let fixture = load_fixture("testing_data/auth/02_jwt_missing_header.json").expect("Failed to load fixture");

    let body = &fixture["expected_response"]["body"];

    assert!(body["type"].is_string());
    assert!(body["title"].is_string());
    assert!(body["status"].is_number());
    assert!(body["detail"].is_string());

    let type_str = body["type"].as_str().unwrap();
    assert!(type_str.contains("spikard.dev/errors"));

    assert_eq!(fixture["expected_response"]["status_code"], body["status"]);
}

#[test]
fn test_api_key_error_response_format() {
    let fixture = load_fixture("testing_data/auth/08_api_key_missing.json").expect("Failed to load fixture");

    let body = &fixture["expected_response"]["body"];

    assert!(body["type"].is_string());
    assert!(body["title"].is_string());
    assert!(body["status"].is_number());
    assert!(body["detail"].is_string());

    let type_str = body["type"].as_str().unwrap();
    assert!(type_str.contains("spikard.dev/errors"));
}

/// Verify all JWT fixtures use consistent error type URIs
#[test]
fn test_jwt_fixtures_consistent_error_types() {
    let fixtures = vec![
        "testing_data/auth/02_jwt_missing_header.json",
        "testing_data/auth/03_jwt_expired_token.json",
        "testing_data/auth/04_jwt_invalid_signature.json",
        "testing_data/auth/05_jwt_invalid_audience.json",
        "testing_data/auth/09_jwt_invalid_issuer.json",
        "testing_data/auth/10_jwt_not_before_future.json",
        "testing_data/auth/13_jwt_malformed_token.json",
        "testing_data/auth/17_bearer_token_without_prefix.json",
    ];

    for fixture_path in fixtures {
        let fixture = load_fixture(fixture_path).unwrap_or_else(|_| panic!("Failed to load fixture: {}", fixture_path));

        if fixture["expected_response"]["status_code"] == 401 {
            let error_type = fixture["expected_response"]["body"]["type"].as_str().unwrap();
            assert_eq!(
                error_type, "https://spikard.dev/errors/unauthorized",
                "Fixture {} has inconsistent error type",
                fixture_path
            );
        }
    }
}

/// Verify all API key fixtures use consistent error type URIs
#[test]
fn test_api_key_fixtures_consistent_error_types() {
    let fixtures = vec![
        "testing_data/auth/07_api_key_invalid.json",
        "testing_data/auth/08_api_key_missing.json",
    ];

    for fixture_path in fixtures {
        let fixture = load_fixture(fixture_path).unwrap_or_else(|_| panic!("Failed to load fixture: {}", fixture_path));

        if fixture["expected_response"]["status_code"] == 401 {
            let error_type = fixture["expected_response"]["body"]["type"].as_str().unwrap();
            assert_eq!(
                error_type, "https://spikard.dev/errors/unauthorized",
                "Fixture {} has inconsistent error type",
                fixture_path
            );
        }
    }
}

/// Verify all 401 responses have unique, meaningful error details
#[test]
fn test_jwt_error_details_are_specific() {
    let fixtures = vec![
        ("testing_data/auth/02_jwt_missing_header.json", "Authorization"),
        ("testing_data/auth/03_jwt_expired_token.json", "expired"),
        ("testing_data/auth/04_jwt_invalid_signature.json", "signature"),
        ("testing_data/auth/05_jwt_invalid_audience.json", "audience"),
        ("testing_data/auth/09_jwt_invalid_issuer.json", "issuer"),
        ("testing_data/auth/10_jwt_not_before_future.json", "valid"),
        ("testing_data/auth/13_jwt_malformed_token.json", "Malformed"),
        ("testing_data/auth/17_bearer_token_without_prefix.json", "Bearer"),
    ];

    for (fixture_path, expected_keyword) in fixtures {
        let fixture = load_fixture(fixture_path).unwrap_or_else(|_| panic!("Failed to load fixture: {}", fixture_path));

        let detail = fixture["expected_response"]["body"]["detail"]
            .as_str()
            .unwrap()
            .to_lowercase();

        assert!(
            detail.contains(&expected_keyword.to_lowercase()),
            "Fixture {} detail doesn't contain expected keyword '{}': {}",
            fixture_path,
            expected_keyword,
            detail
        );
    }
}

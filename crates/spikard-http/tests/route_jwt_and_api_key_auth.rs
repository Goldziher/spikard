//! Per-route JWT and API key authentication: `Route.jwt_auth` / `Route.api_key_auth` must
//! protect only the specific route they're configured on. No server-global `jwt_auth` /
//! `api_key_auth` is configured in these tests, so a 401 can only come from the per-route layer,
//! and — critically — a route with no auth config in the same router must stay open.

use axum::http::StatusCode;
use jsonwebtoken::{EncodingKey, Header, encode};
use spikard_http::server::build_router_with_handlers_and_config;
use spikard_http::{ApiKeyAuthConfig, Claims, Handler, HandlerResult, JwtAuthConfig, RequestData, Route, ServerConfig};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

struct OkHandler;

impl Handler for OkHandler {
    fn call(
        &self,
        _request: axum::http::Request<axum::body::Body>,
        _request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(async move {
            Ok(axum::http::Response::builder()
                .status(StatusCode::OK)
                .body(axum::body::Body::from("ok"))
                .unwrap())
        })
    }
}

fn now_plus(seconds: u64) -> usize {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("time");
    usize::try_from((now + Duration::from_secs(seconds)).as_secs()).expect("timestamp fits usize")
}

fn jwt_route() -> Route {
    Route {
        method: "GET".parse().unwrap(),
        path: "/jwt-protected".to_string(),
        handler_name: "ok".to_string(),
        jwt_auth: Some(JwtAuthConfig {
            enabled: true,
            secret: Some("test-secret".to_string()),
            public_key: None,
            algorithm: "HS256".to_string(),
            audience: None,
            issuer: None,
            leeway: 0,
        }),
        ..Default::default()
    }
}

fn api_key_route() -> Route {
    Route {
        method: "GET".parse().unwrap(),
        path: "/api-key-protected".to_string(),
        handler_name: "ok".to_string(),
        api_key_auth: Some(ApiKeyAuthConfig {
            enabled: true,
            keys: vec!["valid-key".to_string()],
            header_name: "X-API-Key".to_string(),
        }),
        ..Default::default()
    }
}

fn open_route() -> Route {
    Route {
        method: "GET".parse().unwrap(),
        path: "/open".to_string(),
        handler_name: "ok".to_string(),
        ..Default::default()
    }
}

fn build_router() -> axum::Router {
    let config = ServerConfig::default();
    build_router_with_handlers_and_config(
        vec![
            (jwt_route(), Arc::new(OkHandler) as Arc<dyn Handler>),
            (api_key_route(), Arc::new(OkHandler) as Arc<dyn Handler>),
            (open_route(), Arc::new(OkHandler) as Arc<dyn Handler>),
        ],
        config,
        Vec::new(),
    )
    .expect("router")
}

fn valid_token() -> String {
    let claims = Claims {
        sub: "user-1".to_string(),
        exp: now_plus(3600),
        iat: None,
        nbf: None,
        aud: None,
        iss: None,
        roles: vec![],
        scopes: vec![],
        permissions: vec![],
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(b"test-secret")).expect("token encodes")
}

#[tokio::test]
async fn jwt_route_rejects_missing_token() {
    let server = axum_test::TestServer::new(build_router());
    let response = server.get("/jwt-protected").await;
    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn jwt_route_accepts_valid_token() {
    let server = axum_test::TestServer::new(build_router());
    let response = server
        .get("/jwt-protected")
        .add_header("authorization", format!("Bearer {}", valid_token()))
        .await;
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn api_key_route_rejects_missing_key() {
    let server = axum_test::TestServer::new(build_router());
    let response = server.get("/api-key-protected").await;
    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn api_key_route_accepts_valid_key() {
    let server = axum_test::TestServer::new(build_router());
    let response = server
        .get("/api-key-protected")
        .add_header("x-api-key", "valid-key")
        .await;
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn api_key_route_rejects_invalid_key() {
    let server = axum_test::TestServer::new(build_router());
    let response = server
        .get("/api-key-protected")
        .add_header("x-api-key", "wrong-key")
        .await;
    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn route_without_auth_config_stays_open_alongside_protected_routes() {
    let server = axum_test::TestServer::new(build_router());
    let response = server.get("/open").await;
    assert_eq!(
        response.status_code(),
        StatusCode::OK,
        "a route with neither `jwt_auth` nor `api_key_auth` configured must not require \
         credentials just because other routes in the same router do"
    );
}

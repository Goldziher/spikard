//! Per-route authorization: `Route.authorization` enforces role/scope/permission requirements
//! against the `Claims` populated by JWT auth. Authorization is layered to run *after* JWT auth
//! (see `server/mod.rs`'s per-route middleware ordering), so these tests combine both.

use axum::http::StatusCode;
use jsonwebtoken::{EncodingKey, Header, encode};
use spikard_http::server::build_router_with_handlers_and_config;
use spikard_http::{
    AuthorizationConfig, Claims, Handler, HandlerResult, JwtAuthConfig, RequestData, Route, ServerConfig,
};
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

fn jwt_config() -> JwtAuthConfig {
    JwtAuthConfig {
        enabled: true,
        secret: Some("authz-secret".to_string()),
        public_key: None,
        algorithm: "HS256".to_string(),
        audience: None,
        issuer: None,
        leeway: 0,
    }
}

fn admin_only_route() -> Route {
    Route {
        method: "GET".parse().unwrap(),
        path: "/admin".to_string(),
        handler_name: "ok".to_string(),
        jwt_auth: Some(jwt_config()),
        authorization: Some(AuthorizationConfig {
            required_roles: vec!["admin".to_string()],
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn jwt_only_route() -> Route {
    Route {
        method: "GET".parse().unwrap(),
        path: "/jwt-only".to_string(),
        handler_name: "ok".to_string(),
        jwt_auth: Some(jwt_config()),
        ..Default::default()
    }
}

/// `authorization` with no `jwt_auth` at all: nothing upstream ever populates the `Claims`
/// extension `authorization_middleware` reads, so every request must fail closed with 403 from
/// the authorization layer itself — not 401 from a JWT layer that isn't even present.
fn authz_only_route() -> Route {
    Route {
        method: "GET".parse().unwrap(),
        path: "/authz-only".to_string(),
        handler_name: "ok".to_string(),
        authorization: Some(AuthorizationConfig {
            required_roles: vec!["admin".to_string()],
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn build_router() -> axum::Router {
    let config = ServerConfig::default();
    build_router_with_handlers_and_config(
        vec![
            (admin_only_route(), Arc::new(OkHandler) as Arc<dyn Handler>),
            (jwt_only_route(), Arc::new(OkHandler) as Arc<dyn Handler>),
            (authz_only_route(), Arc::new(OkHandler) as Arc<dyn Handler>),
        ],
        config,
        Vec::new(),
    )
    .expect("router")
}

fn token_with_roles(roles: Vec<String>) -> String {
    let claims = Claims {
        sub: "user-1".to_string(),
        exp: now_plus(3600),
        iat: None,
        nbf: None,
        aud: None,
        iss: None,
        roles,
        scopes: vec![],
        permissions: vec![],
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(b"authz-secret")).expect("token encodes")
}

#[tokio::test]
async fn route_with_required_role_rejects_caller_missing_it() {
    let server = axum_test::TestServer::new(build_router());
    let response = server
        .get("/admin")
        .add_header(
            "authorization",
            format!("Bearer {}", token_with_roles(vec!["user".to_string()])),
        )
        .await;
    assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn route_with_required_role_accepts_caller_with_it() {
    let server = axum_test::TestServer::new(build_router());
    let response = server
        .get("/admin")
        .add_header(
            "authorization",
            format!("Bearer {}", token_with_roles(vec!["admin".to_string()])),
        )
        .await;
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn route_with_authorization_but_no_claims_fails_closed() {
    let server = axum_test::TestServer::new(build_router());
    let response = server.get("/authz-only").await;
    assert_eq!(
        response.status_code(),
        StatusCode::FORBIDDEN,
        "authorization with no upstream JWT auth has no Claims to check and must fail closed \
         with 403, not silently allow the request through"
    );
}

#[tokio::test]
async fn route_without_authorization_config_ignores_roles() {
    let server = axum_test::TestServer::new(build_router());
    let response = server
        .get("/jwt-only")
        .add_header("authorization", format!("Bearer {}", token_with_roles(vec![])))
        .await;
    assert_eq!(
        response.status_code(),
        StatusCode::OK,
        "a route with `jwt_auth` but no `authorization` config must accept any authenticated \
         caller regardless of roles — otherwise authorization would be leaking onto routes that \
         never asked for it"
    );
}

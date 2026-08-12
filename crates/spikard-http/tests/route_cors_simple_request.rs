//! Per-route CORS on the simple-request path (as opposed to the OPTIONS preflight, already
//! covered by `server_cors_preflight.rs`). `validate_cors_request` and `add_cors_headers` in
//! `spikard_http::cors` existed but had no caller anywhere in the request path before this —
//! this exercises the middleware that finally wires them in.
//!
//! `ServerConfig` has no server-global CORS field (CORS is per-route by design), so there is no
//! interaction with a global layer to worry about here, unlike `body_limit`/`timeout`.

use axum::http::StatusCode;
use spikard_http::server::build_router_with_handlers_and_config;
use spikard_http::{CorsConfig, Handler, HandlerResult, RequestData, Route, ServerConfig};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

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
                .header("content-type", "application/json")
                .body(axum::body::Body::from("{\"ok\":true}"))
                .unwrap())
        })
    }
}

fn cors_config() -> CorsConfig {
    CorsConfig {
        allowed_origins: vec!["https://trusted.example.com".to_string()],
        allowed_methods: vec!["GET".to_string()],
        allowed_headers: vec![],
        expose_headers: None,
        max_age: None,
        allow_credentials: None,
        ..Default::default()
    }
}

fn cors_route() -> Route {
    Route {
        method: "GET".parse().unwrap(),
        path: "/cors-simple".to_string(),
        handler_name: "ok".to_string(),
        cors: Some(cors_config()),
        ..Default::default()
    }
}

fn plain_route() -> Route {
    Route {
        method: "GET".parse().unwrap(),
        path: "/plain-simple".to_string(),
        handler_name: "ok".to_string(),
        ..Default::default()
    }
}

fn build_router() -> axum::Router {
    let config = ServerConfig::default();
    build_router_with_handlers_and_config(
        vec![
            (cors_route(), Arc::new(OkHandler) as Arc<dyn Handler>),
            (plain_route(), Arc::new(OkHandler) as Arc<dyn Handler>),
        ],
        config,
        Vec::new(),
    )
    .expect("router")
}

#[tokio::test]
async fn allowed_origin_gets_cors_headers_on_simple_request() {
    let server = axum_test::TestServer::new(build_router());

    let response = server
        .get("/cors-simple")
        .add_header("origin", "https://trusted.example.com")
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(
        response.header("access-control-allow-origin").to_str().unwrap(),
        "https://trusted.example.com"
    );
    assert_eq!(response.header("vary").to_str().unwrap(), "Origin");
}

#[tokio::test]
async fn disallowed_origin_is_rejected_with_403_on_simple_request() {
    let server = axum_test::TestServer::new(build_router());

    let response = server
        .get("/cors-simple")
        .add_header("origin", "https://evil.example.com")
        .await;

    assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn route_without_cors_config_is_unaffected_by_origin() {
    let server = axum_test::TestServer::new(build_router());

    let response = server
        .get("/plain-simple")
        .add_header("origin", "https://evil.example.com")
        .await;

    assert_eq!(
        response.status_code(),
        StatusCode::OK,
        "a route with no `cors` config must not enforce origin checks, even for an origin \
         that would be rejected on a CORS-configured route"
    );
    assert!(
        !response.contains_header("access-control-allow-origin"),
        "a route with no `cors` config must not stamp CORS response headers"
    );
}

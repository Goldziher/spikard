//! Per-route request-id: `Route.request_id` lets a route opt into `x-request-id`
//! generation/propagation even when `ServerConfig.enable_request_id` (the server-global default)
//! is off. `ServerConfig::default()` has `enable_request_id: false`, so any `x-request-id` header
//! observed in these tests can only have come from the per-route layer.

use axum::http::StatusCode;
use spikard_http::server::build_router_with_handlers_and_config;
use spikard_http::{Handler, HandlerResult, RequestData, RequestIdConfig, Route, ServerConfig};
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
                .body(axum::body::Body::from("ok"))
                .unwrap())
        })
    }
}

fn request_id_route() -> Route {
    Route {
        method: "GET".parse().unwrap(),
        path: "/with-id".to_string(),
        handler_name: "ok".to_string(),
        request_id: Some(RequestIdConfig { enabled: true }),
        ..Default::default()
    }
}

fn plain_route() -> Route {
    Route {
        method: "GET".parse().unwrap(),
        path: "/without-id".to_string(),
        handler_name: "ok".to_string(),
        ..Default::default()
    }
}

fn build_router() -> axum::Router {
    let config = ServerConfig {
        enable_request_id: false,
        ..ServerConfig::default()
    };
    build_router_with_handlers_and_config(
        vec![
            (request_id_route(), Arc::new(OkHandler) as Arc<dyn Handler>),
            (plain_route(), Arc::new(OkHandler) as Arc<dyn Handler>),
        ],
        config,
        Vec::new(),
    )
    .expect("router")
}

#[tokio::test]
async fn route_with_request_id_enabled_gets_header_despite_disabled_server_global() {
    let server = axum_test::TestServer::new(build_router());

    let response = server.get("/with-id").await;

    assert_eq!(response.status_code(), StatusCode::OK);
    assert!(
        response.contains_header("x-request-id"),
        "route-level `request_id: {{enabled: true}}` should generate x-request-id even though \
         the server-global default is off"
    );
    assert!(!response.header("x-request-id").to_str().unwrap().is_empty());
}

#[tokio::test]
async fn route_without_request_id_config_gets_no_header() {
    let server = axum_test::TestServer::new(build_router());

    let response = server.get("/without-id").await;

    assert_eq!(response.status_code(), StatusCode::OK);
    assert!(
        !response.contains_header("x-request-id"),
        "a route with no `request_id` config, and a disabled server-global default, must not \
         carry x-request-id — otherwise this would just be global behavior wearing a per-route hat"
    );
}

//! Per-route rate limiting: `Route.rate_limit` must add a real `GovernorLayer` scoped to that
//! route, independent of any other route's traffic. No server-global `rate_limit` is configured
//! in these tests, so a 429 can only come from the per-route layer.
//!
//! Uses `ip_based: false` (the `GlobalKeyExtractor`) rather than the default IP-based extractor:
//! `axum-test`'s mock transport does not provide real per-connection socket info, and the global
//! bucket is what makes the burst limit deterministic here regardless of how the test client
//! is wired up.

use axum::http::StatusCode;
use spikard_http::server::build_router_with_handlers_and_config;
use spikard_http::{Handler, HandlerResult, RateLimitConfig, RequestData, Route, ServerConfig};
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

fn limited_route() -> Route {
    Route {
        method: "GET".parse().unwrap(),
        path: "/limited".to_string(),
        handler_name: "ok".to_string(),
        rate_limit: Some(RateLimitConfig {
            per_second: 1,
            burst: 1,
            ip_based: false,
        }),
        ..Default::default()
    }
}

fn unlimited_route() -> Route {
    Route {
        method: "GET".parse().unwrap(),
        path: "/unlimited".to_string(),
        handler_name: "ok".to_string(),
        ..Default::default()
    }
}

fn build_router() -> axum::Router {
    let config = ServerConfig::default();
    build_router_with_handlers_and_config(
        vec![
            (limited_route(), Arc::new(OkHandler) as Arc<dyn Handler>),
            (unlimited_route(), Arc::new(OkHandler) as Arc<dyn Handler>),
        ],
        config,
        Vec::new(),
    )
    .expect("router")
}

#[tokio::test]
async fn route_with_rate_limit_returns_429_with_retry_after_once_burst_is_exhausted() {
    let server = axum_test::TestServer::new(build_router());

    let first = server.get("/limited").await;
    assert_eq!(first.status_code(), StatusCode::OK);

    let second = server.get("/limited").await;
    assert_eq!(second.status_code(), StatusCode::TOO_MANY_REQUESTS);
    assert!(
        second.contains_header("retry-after"),
        "a 429 from the per-route governor layer must carry Retry-After"
    );
}

#[tokio::test]
async fn route_without_rate_limit_config_is_never_limited() {
    let server = axum_test::TestServer::new(build_router());

    for _ in 0..10 {
        let response = server.get("/unlimited").await;
        assert_eq!(
            response.status_code(),
            StatusCode::OK,
            "a route with no `rate_limit` config, and no server-global rate limit, must never return 429"
        );
    }
}

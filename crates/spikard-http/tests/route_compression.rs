//! Per-route compression: `Route.compression` must add a real `CompressionLayer` to that
//! route's `MethodRouter`, not merely flow into `OpenAPI` metadata. No server-global compression
//! is configured in these tests, so any `content-encoding` header observed can only have come
//! from the per-route layer — proving it is genuinely per-route, not accidentally global.

use axum::http::StatusCode;
use spikard_http::server::build_router_with_handlers_and_config;
use spikard_http::{CompressionConfig, Handler, HandlerResult, RequestData, Route, ServerConfig};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Handler returning a compressible JSON body comfortably above any reasonable `min_size`
/// threshold.
struct LargeBodyHandler;

impl Handler for LargeBodyHandler {
    fn call(
        &self,
        _request: axum::http::Request<axum::body::Body>,
        _request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(async move {
            let payload = format!("{{\"data\":\"{}\"}}", "x".repeat(2000));
            Ok(axum::http::Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(axum::body::Body::from(payload))
                .unwrap())
        })
    }
}

fn compressed_route() -> Route {
    Route {
        method: "GET".parse().unwrap(),
        path: "/compressed".to_string(),
        handler_name: "large_body".to_string(),
        compression: Some(CompressionConfig {
            gzip: true,
            brotli: true,
            min_size: 0,
            quality: 6,
        }),
        ..Default::default()
    }
}

fn plain_route() -> Route {
    Route {
        method: "GET".parse().unwrap(),
        path: "/plain".to_string(),
        handler_name: "large_body".to_string(),
        ..Default::default()
    }
}

#[tokio::test]
async fn route_with_compression_config_gets_content_encoding() {
    let config = ServerConfig::default();
    let router = build_router_with_handlers_and_config(
        vec![(compressed_route(), Arc::new(LargeBodyHandler) as Arc<dyn Handler>)],
        config,
        Vec::new(),
    )
    .expect("router");

    let server = axum_test::TestServer::new(router);

    let response = server.get("/compressed").add_header("accept-encoding", "gzip").await;

    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(
        response.header("content-encoding").to_str().unwrap(),
        "gzip",
        "route-level compression should encode the response"
    );
}

#[tokio::test]
async fn route_without_compression_config_gets_no_content_encoding() {
    let config = ServerConfig::default();
    let router = build_router_with_handlers_and_config(
        vec![
            (compressed_route(), Arc::new(LargeBodyHandler) as Arc<dyn Handler>),
            (plain_route(), Arc::new(LargeBodyHandler) as Arc<dyn Handler>),
        ],
        config,
        Vec::new(),
    )
    .expect("router");

    let server = axum_test::TestServer::new(router);

    let response = server.get("/plain").add_header("accept-encoding", "gzip").await;

    assert_eq!(response.status_code(), StatusCode::OK);
    assert!(
        !response.contains_header("content-encoding"),
        "a route with no compression config, and no server-global compression, must not be compressed"
    );
}

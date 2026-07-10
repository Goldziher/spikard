use axum::body::Body;
use axum::http::{Request, StatusCode};
use brotli::Decompressor;
use spikard_http::server::build_router_with_handlers_and_config;
use spikard_http::{
    CompressionConfig, Handler, HandlerResult, Method, RateLimitConfig, RequestData, Route, ServerConfig,
};
use std::future::Future;
use std::io::Read;
use std::pin::Pin;
use std::sync::Arc;
use uuid::Uuid;

struct PlainTextHandler {
    body: String,
}

impl Handler for PlainTextHandler {
    fn call(
        &self,
        _request: Request<Body>,
        _request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        let body = self.body.clone();
        Box::pin(async move {
            Ok(axum::http::Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "text/plain")
                .body(Body::from(body))
                .expect("response builder"))
        })
    }
}

fn basic_route(method: Method, path: &str, expects_json_body: bool) -> Route {
    Route {
        method,
        path: path.to_string(),
        handler_name: "plain".to_string(),
        expects_json_body,
        cors: None,
        is_async: true,
        file_params: None,
        request_validator: None,
        response_validator: None,
        parameter_validator: None,
        jsonrpc_method: None,
        compression: None,
        #[cfg(feature = "di")]
        handler_dependencies: Vec::new(),
    }
}

#[tokio::test]
async fn request_id_is_generated_and_propagated() {
    let route = basic_route(Method::Get, "/rid", false);
    let config = ServerConfig {
        enable_request_id: true,
        ..Default::default()
    };
    let router = build_router_with_handlers_and_config(
        vec![(
            route,
            Arc::new(PlainTextHandler { body: "ok".to_string() }) as Arc<dyn Handler>,
        )],
        config,
        Vec::new(),
    )
    .expect("router");

    let server = axum_test::TestServer::new(router);
    let response = server.get("/rid").await;
    assert_eq!(response.status_code(), StatusCode::OK);

    let header = response.header("x-request-id");
    let request_id = header.to_str().expect("request id");
    assert!(Uuid::parse_str(request_id).is_ok());

    let response2 = server.get("/rid").add_header("x-request-id", "req-123").await;
    assert_eq!(response2.status_code(), StatusCode::OK);
    assert_eq!(
        response2.header("x-request-id").to_str().expect("request id"),
        "req-123"
    );
}

#[tokio::test]
async fn default_body_limit_can_be_disabled() {
    let route = basic_route(Method::Post, "/upload", false);
    let config = ServerConfig {
        max_body_size: None,
        ..Default::default()
    };

    let router = build_router_with_handlers_and_config(
        vec![(
            route,
            Arc::new(PlainTextHandler { body: "ok".to_string() }) as Arc<dyn Handler>,
        )],
        config,
        Vec::new(),
    )
    .expect("router");

    let server = axum_test::TestServer::new(router);
    let payload = vec![b'a'; 1024 * 128];
    let response = server.post("/upload").bytes(payload.into()).await;
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn default_body_limit_allows_payloads_within_limit() {
    let route = basic_route(Method::Post, "/upload", false);
    let config = ServerConfig {
        max_body_size: Some(16),
        ..Default::default()
    };

    let router = build_router_with_handlers_and_config(
        vec![(
            route,
            Arc::new(PlainTextHandler { body: "ok".to_string() }) as Arc<dyn Handler>,
        )],
        config,
        Vec::new(),
    )
    .expect("router");

    let server = axum_test::TestServer::new(router);
    let payload = vec![b'a'; 8];
    let response = server.post("/upload").bytes(payload.into()).await;
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn compression_br_is_applied_when_accepted() {
    let original_body = "x".repeat(2048);
    let route = basic_route(Method::Get, "/compressed", false);

    let config = ServerConfig {
        compression: Some(CompressionConfig {
            gzip: false,
            brotli: true,
            min_size: 0,
            quality: 3,
        }),
        ..Default::default()
    };

    let router = build_router_with_handlers_and_config(
        vec![(
            route,
            Arc::new(PlainTextHandler {
                body: original_body.clone(),
            }) as Arc<dyn Handler>,
        )],
        config,
        Vec::new(),
    )
    .expect("router");

    let server = axum_test::TestServer::new(router);
    let response = server.get("/compressed").add_header("accept-encoding", "br").await;
    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(response.header("content-encoding").to_str().expect("encoding"), "br");

    let mut decoder = Decompressor::new(response.as_bytes().as_ref(), 4096);
    let mut decoded_body = String::new();
    decoder.read_to_string(&mut decoded_body).expect("decompress");
    assert_eq!(decoded_body, original_body);
}

#[tokio::test]
async fn rate_limit_builder_covers_ip_and_global_key_extractors() {
    let route = basic_route(Method::Get, "/rl", false);
    let handler: Arc<dyn Handler> = Arc::new(PlainTextHandler { body: "ok".to_string() });

    let ip_config = ServerConfig {
        rate_limit: Some(RateLimitConfig {
            per_second: 100,
            burst: 10,
            ip_based: true,
        }),
        ..Default::default()
    };
    let router_ip =
        build_router_with_handlers_and_config(vec![(route.clone(), Arc::clone(&handler))], ip_config, Vec::new())
            .expect("router");
    let server_ip = axum_test::TestServer::new(router_ip.into_make_service_with_connect_info::<std::net::SocketAddr>());
    assert_eq!(server_ip.get("/rl").await.status_code(), StatusCode::OK);

    let global_config = ServerConfig {
        rate_limit: Some(RateLimitConfig {
            per_second: 100,
            burst: 10,
            ip_based: false,
        }),
        ..Default::default()
    };
    let router_global =
        build_router_with_handlers_and_config(vec![(route, handler)], global_config, Vec::new()).expect("router");
    let server_global =
        axum_test::TestServer::new(router_global.into_make_service_with_connect_info::<std::net::SocketAddr>());
    assert_eq!(server_global.get("/rl").await.status_code(), StatusCode::OK);
}

/// Verify that POST requests with application/grpc content-type are not rejected with 415
/// even when the route has a request schema (`expects_json_body` = true).
#[tokio::test]
async fn grpc_content_type_is_not_rejected_on_json_route() {
    let schema = serde_json::json!({
        "type": "object",
        "properties": { "name": { "type": "string" } },
        "required": ["name"]
    });
    let route = Route {
        method: Method::Post,
        path: "/grpc/test".to_string(),
        handler_name: "grpc_test".to_string(),
        expects_json_body: true,
        cors: None,
        is_async: true,
        file_params: None,
        request_validator: Some(Arc::new(spikard_core::SchemaValidator::new(schema).unwrap())),
        response_validator: None,
        parameter_validator: None,
        jsonrpc_method: None,
        compression: None,
        #[cfg(feature = "di")]
        handler_dependencies: Vec::new(),
    };

    let handler: Arc<dyn Handler> = Arc::new(PlainTextHandler {
        body: "grpc_ok".to_string(),
    });
    let router = build_router_with_handlers_and_config(vec![(route, handler)], ServerConfig::default(), Vec::new())
        .expect("router");
    let server = axum_test::TestServer::new(router);

    for content_type in &[
        "application/grpc",
        "application/grpc+proto",
        "application/grpc+json",
        "application/grpc-web",
        "application/grpc-web+proto",
        "application/grpc-web+json",
    ] {
        let response = server
            .post("/grpc/test")
            .add_header("content-type", *content_type)
            .bytes(bytes::Bytes::from_static(b"\x00\x00\x00\x00\x04test"))
            .await;
        assert_ne!(
            response.status_code(),
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "content-type {content_type} should not produce 415"
        );
    }
}

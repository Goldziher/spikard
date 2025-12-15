use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::json;
use spikard_http::server::build_router_with_handlers_and_config;
use spikard_http::{Handler, HandlerResult, Method, RequestData, Route, ServerConfig, StaticFilesConfig};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tempfile::tempdir;
use tokio::time::sleep;
use tower::ServiceExt;

struct EchoRequestDataHandler;

impl Handler for EchoRequestDataHandler {
    fn call(
        &self,
        _request: Request<Body>,
        request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(async move {
            let payload = json!({
                "method": request_data.method,
                "path": request_data.path,
                "path_params": request_data.path_params.as_ref(),
                "query_params": request_data.query_params,
                "body": request_data.body,
            });
            let bytes = serde_json::to_vec(&payload).expect("serialize");
            Ok(axum::http::Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(bytes))
                .expect("response"))
        })
    }
}

struct SlowHandler {
    sleep_for: Duration,
}

impl Handler for SlowHandler {
    fn call(
        &self,
        _request: Request<Body>,
        _request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        let duration = self.sleep_for;
        Box::pin(async move {
            sleep(duration).await;
            Ok(axum::http::Response::builder()
                .status(StatusCode::OK)
                .body(Body::from("ok"))
                .expect("response"))
        })
    }
}

fn route(method: Method, path: &str, handler_name: &str) -> Route {
    Route {
        method,
        path: path.to_string(),
        handler_name: handler_name.to_string(),
        expects_json_body: true,
        cors: None,
        is_async: true,
        file_params: None,
        request_validator: None,
        response_validator: None,
        parameter_validator: None,
        jsonrpc_method: None,
        #[cfg(feature = "di")]
        handler_dependencies: Vec::new(),
    }
}

async fn json_response(router: axum::Router, request: Request<Body>) -> serde_json::Value {
    let response = router.oneshot(request).await.expect("response");
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = response.into_body().collect().await.expect("collect body").to_bytes();
    serde_json::from_slice(&bytes).expect("json")
}

#[tokio::test]
async fn put_with_path_params_and_json_body_is_extracted() {
    let router = build_router_with_handlers_and_config(
        vec![(
            route(Method::Put, "/items/{id}", "echo"),
            Arc::new(EchoRequestDataHandler) as Arc<dyn Handler>,
        )],
        ServerConfig::default(),
        Vec::new(),
    )
    .expect("router");

    let payload = json!({"name":"widget"});
    let request = Request::builder()
        .method("PUT")
        .uri("/items/123")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload).expect("encode")))
        .expect("request");

    let body = json_response(router, request).await;
    assert_eq!(body["method"], "PUT");
    assert_eq!(body["path"], "/items/123");
    assert_eq!(body["path_params"]["id"], "123");
    assert_eq!(body["body"]["name"], "widget");
}

#[tokio::test]
async fn patch_with_path_params_and_json_body_is_extracted() {
    let router = build_router_with_handlers_and_config(
        vec![(
            route(Method::Patch, "/items/{id}", "echo"),
            Arc::new(EchoRequestDataHandler) as Arc<dyn Handler>,
        )],
        ServerConfig::default(),
        Vec::new(),
    )
    .expect("router");

    let payload = json!({"active":true});
    let request = Request::builder()
        .method("PATCH")
        .uri("/items/777")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload).expect("encode")))
        .expect("request");

    let body = json_response(router, request).await;
    assert_eq!(body["method"], "PATCH");
    assert_eq!(body["path_params"]["id"], "777");
    assert_eq!(body["body"]["active"], true);
}

#[tokio::test]
async fn head_with_path_params_hits_handler() {
    let router = build_router_with_handlers_and_config(
        vec![(
            route(Method::Head, "/health/{id}", "echo"),
            Arc::new(EchoRequestDataHandler) as Arc<dyn Handler>,
        )],
        ServerConfig::default(),
        Vec::new(),
    )
    .expect("router");

    let request = Request::builder()
        .method("HEAD")
        .uri("/health/abc")
        .body(Body::empty())
        .expect("request");

    let response = router.oneshot(request).await.expect("response");
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("content-type").and_then(|h| h.to_str().ok()),
        Some("application/json")
    );
    let bytes = response.into_body().collect().await.expect("collect body").to_bytes();
    assert!(bytes.is_empty());
}

#[tokio::test]
async fn trace_with_path_params_hits_handler() {
    let router = build_router_with_handlers_and_config(
        vec![(
            route(Method::Trace, "/trace/{id}", "echo"),
            Arc::new(EchoRequestDataHandler) as Arc<dyn Handler>,
        )],
        ServerConfig::default(),
        Vec::new(),
    )
    .expect("router");

    let request = Request::builder()
        .method("TRACE")
        .uri("/trace/xyz")
        .body(Body::empty())
        .expect("request");

    let body = json_response(router, request).await;
    assert_eq!(body["method"], "TRACE");
    assert_eq!(body["path_params"]["id"], "xyz");
}

#[tokio::test]
async fn options_with_path_params_hits_handler_when_no_cors_configured() {
    let router = build_router_with_handlers_and_config(
        vec![(
            route(Method::Options, "/options/{id}", "echo"),
            Arc::new(EchoRequestDataHandler) as Arc<dyn Handler>,
        )],
        ServerConfig::default(),
        Vec::new(),
    )
    .expect("router");

    let request = Request::builder()
        .method("OPTIONS")
        .uri("/options/1")
        .body(Body::empty())
        .expect("request");

    let body = json_response(router, request).await;
    assert_eq!(body["method"], "OPTIONS");
    assert_eq!(body["path_params"]["id"], "1");
}

#[tokio::test]
async fn request_timeout_returns_408() {
    let config = ServerConfig {
        request_timeout: Some(1),
        ..Default::default()
    };

    let router = build_router_with_handlers_and_config(
        vec![(
            route(Method::Get, "/slow", "slow"),
            Arc::new(SlowHandler {
                sleep_for: Duration::from_secs(2),
            }) as Arc<dyn Handler>,
        )],
        config,
        Vec::new(),
    )
    .expect("router");

    let response = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/slow")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::REQUEST_TIMEOUT);
}

#[tokio::test]
async fn static_files_cache_control_header_is_set_when_configured() {
    let dir = tempdir().expect("temp dir");
    let file_path = dir.path().join("hello.txt");
    std::fs::write(&file_path, "hi").expect("write file");

    let config = ServerConfig {
        static_files: vec![StaticFilesConfig {
            directory: dir.path().to_string_lossy().to_string(),
            route_prefix: "/static".to_string(),
            index_file: true,
            cache_control: Some("max-age=60".to_string()),
        }],
        ..Default::default()
    };

    let router = build_router_with_handlers_and_config(Vec::new(), config, Vec::new()).expect("router");

    let response = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/static/hello.txt")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get("cache-control")
            .expect("cache-control")
            .to_str()
            .expect("header"),
        "max-age=60"
    );
}

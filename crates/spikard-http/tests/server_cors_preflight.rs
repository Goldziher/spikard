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

#[tokio::test]
async fn router_generates_cors_preflight_when_missing_options_handler() {
    let cors = CorsConfig {
        allowed_origins: vec!["https://example.com".to_string()],
        allowed_methods: vec!["GET".to_string(), "OPTIONS".to_string()],
        allowed_headers: vec!["x-test".to_string()],
        expose_headers: None,
        max_age: Some(600),
        allow_credentials: Some(true),
    };

    let route = Route {
        method: "GET".parse().unwrap(),
        path: "/cors".to_string(),
        handler_name: "ok".to_string(),
        expects_json_body: false,
        cors: Some(cors.clone()),
        is_async: true,
        file_params: None,
        request_validator: None,
        response_validator: None,
        parameter_validator: None,
        jsonrpc_method: None,
        #[cfg(feature = "di")]
        handler_dependencies: vec![],
    };

    let config = ServerConfig::default();
    let router =
        build_router_with_handlers_and_config(vec![(route, Arc::new(OkHandler))], config, Vec::new()).expect("router");

    let server = axum_test::TestServer::new(router).unwrap();

    let preflight = server
        .method(axum::http::Method::OPTIONS, "/cors")
        .add_header("origin", "https://example.com")
        .add_header("access-control-request-method", "GET")
        .add_header("access-control-request-headers", "x-test")
        .await;

    assert_eq!(preflight.status_code(), StatusCode::NO_CONTENT);
    assert_eq!(
        preflight.header("access-control-allow-origin").to_str().unwrap(),
        "https://example.com"
    );
    assert!(
        preflight
            .header("access-control-allow-methods")
            .to_str()
            .unwrap()
            .contains("GET")
    );

    let response = server.get("/cors").add_header("origin", "https://example.com").await;
    assert_eq!(response.status_code(), StatusCode::OK);
    assert!(response.text().contains("\"ok\":true"));
}

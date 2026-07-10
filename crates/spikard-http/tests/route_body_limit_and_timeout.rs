use axum::http::StatusCode;
use spikard_http::server::build_router_with_handlers_and_config;
use spikard_http::{Handler, HandlerResult, RequestData, Route, ServerConfig};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

/// Handler that always succeeds, echoing a small JSON acknowledgement.
struct AcceptHandler;

impl Handler for AcceptHandler {
    fn call(
        &self,
        _request: axum::http::Request<axum::body::Body>,
        _request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(async move {
            Ok(axum::http::Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(axum::body::Body::from("{\"accepted\":true}"))
                .unwrap())
        })
    }
}

/// Handler that sleeps for a configurable duration before responding, used to
/// exercise the per-route request timeout.
struct SleepingHandler {
    sleep: Duration,
}

impl Handler for SleepingHandler {
    fn call(
        &self,
        _request: axum::http::Request<axum::body::Body>,
        _request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        let sleep = self.sleep;
        Box::pin(async move {
            tokio::time::sleep(sleep).await;
            Ok(axum::http::Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(axum::body::Body::from(r#"{"status":"ok","duration":"fast"}"#))
                .unwrap())
        })
    }
}

fn body_limit_route(max_bytes: usize) -> Route {
    Route {
        method: "POST".parse().unwrap(),
        path: "/body-limit".to_string(),
        handler_name: "accept".to_string(),
        expects_json_body: false,
        cors: None,
        is_async: true,
        file_params: None,
        request_validator: None,
        response_validator: None,
        parameter_validator: None,
        jsonrpc_method: None,
        compression: None,
        body_limit: Some(max_bytes),
        request_timeout_secs: None,
        #[cfg(feature = "di")]
        handler_dependencies: vec![],
    }
}

fn timeout_route(timeout_secs: u64) -> Route {
    Route {
        method: "GET".parse().unwrap(),
        path: "/timeout".to_string(),
        handler_name: "sleepy".to_string(),
        expects_json_body: false,
        cors: None,
        is_async: true,
        file_params: None,
        request_validator: None,
        response_validator: None,
        parameter_validator: None,
        jsonrpc_method: None,
        compression: None,
        body_limit: None,
        request_timeout_secs: Some(timeout_secs),
        #[cfg(feature = "di")]
        handler_dependencies: vec![],
    }
}

#[tokio::test]
async fn body_under_limit_succeeds_with_route_override() {
    let route = body_limit_route(64);
    let config = ServerConfig::default();
    let router = build_router_with_handlers_and_config(vec![(route, Arc::new(AcceptHandler))], config, Vec::new())
        .expect("router");

    let server = axum_test::TestServer::new(router);

    let response = server
        .post("/body-limit")
        .add_header("content-type", "application/json")
        .text(r#"{"note":"small"}"#)
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn body_over_limit_returns_413_with_route_override() {
    let route = body_limit_route(64);
    let config = ServerConfig::default();
    let router = build_router_with_handlers_and_config(vec![(route, Arc::new(AcceptHandler))], config, Vec::new())
        .expect("router");

    let server = axum_test::TestServer::new(router);

    let oversized_note = "x".repeat(200);
    let response = server
        .post("/body-limit")
        .add_header("content-type", "application/json")
        .text(format!(r#"{{"note":"{oversized_note}"}}"#))
        .await;

    assert_eq!(response.status_code(), StatusCode::PAYLOAD_TOO_LARGE);
    assert_eq!(
        response.header("content-type").to_str().unwrap(),
        "application/problem+json"
    );

    let body: serde_json::Value = response.json();
    assert_eq!(body["status"], 413);
}

#[tokio::test]
async fn fast_handler_completes_before_route_timeout() {
    let route = timeout_route(2);
    let handler = SleepingHandler {
        sleep: Duration::from_millis(100),
    };
    let config = ServerConfig::default();
    let router =
        build_router_with_handlers_and_config(vec![(route, Arc::new(handler))], config, Vec::new()).expect("router");

    let server = axum_test::TestServer::new(router);

    let response = server.get("/timeout").await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let body: serde_json::Value = response.json();
    assert_eq!(body["status"], "ok");
    assert_eq!(body["duration"], "fast");
}

#[tokio::test]
async fn slow_handler_exceeds_route_timeout_returns_408() {
    let route = timeout_route(1);
    let handler = SleepingHandler {
        sleep: Duration::from_millis(1500),
    };
    let config = ServerConfig::default();
    let router =
        build_router_with_handlers_and_config(vec![(route, Arc::new(handler))], config, Vec::new()).expect("router");

    let server = axum_test::TestServer::new(router);

    let response = server.get("/timeout").await;

    assert_eq!(response.status_code(), StatusCode::REQUEST_TIMEOUT);
}

#[tokio::test]
async fn route_level_body_limit_overrides_looser_server_global_limit() {
    // Server-global limit is generous (10 MiB); the route-level override is tight (64 bytes).
    let route = body_limit_route(64);
    let config = ServerConfig {
        max_body_size: Some(10 * 1024 * 1024),
        ..ServerConfig::default()
    };
    let router = build_router_with_handlers_and_config(vec![(route, Arc::new(AcceptHandler))], config, Vec::new())
        .expect("router");

    let server = axum_test::TestServer::new(router);

    // Body is well under the server-global limit but exceeds the route-level limit.
    let oversized_note = "x".repeat(200);
    let response = server
        .post("/body-limit")
        .add_header("content-type", "application/json")
        .text(format!(r#"{{"note":"{oversized_note}"}}"#))
        .await;

    assert_eq!(response.status_code(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn route_level_timeout_overrides_unset_server_global_timeout() {
    // Server-global timeout is unset (no timeout); the route-level override is tight (1 second).
    let route = timeout_route(1);
    let handler = SleepingHandler {
        sleep: Duration::from_millis(1500),
    };
    let config = ServerConfig {
        request_timeout: None,
        ..ServerConfig::default()
    };
    let router =
        build_router_with_handlers_and_config(vec![(route, Arc::new(handler))], config, Vec::new()).expect("router");

    let server = axum_test::TestServer::new(router);

    let response = server.get("/timeout").await;

    assert_eq!(response.status_code(), StatusCode::REQUEST_TIMEOUT);
}

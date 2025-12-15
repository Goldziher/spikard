use axum::body::Body;
use http_body_util::BodyExt;
use spikard_http::handler_trait::{Handler, HandlerResult, RequestData};
use spikard_http::{CompressionConfig, Method, Route, Server, ServerConfig};
use std::pin::Pin;
use std::sync::Arc;
use tower::ServiceExt;
use uuid::Uuid;

struct LargeJsonHandler;

impl Handler for LargeJsonHandler {
    fn call(
        &self,
        _request: axum::http::Request<Body>,
        _request_data: RequestData,
    ) -> Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(async move {
            let big_value = "x".repeat(2048);
            let body = format!(r#"{{"payload":"{big_value}"}}"#);
            Ok(axum::http::Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap())
        })
    }
}

fn route(path: &str) -> Route {
    Route {
        path: path.to_string(),
        method: Method::Get,
        handler_name: "large".to_string(),
        expects_json_body: false,
        cors: None,
        is_async: true,
        file_params: None,
        request_validator: None,
        response_validator: None,
        parameter_validator: None,
        jsonrpc_method: None,
        #[cfg(feature = "di")]
        handler_dependencies: vec![],
    }
}

#[tokio::test]
async fn server_applies_request_id_and_gzip_compression_when_configured() {
    let config = ServerConfig {
        compression: Some(CompressionConfig {
            gzip: true,
            brotli: false,
            min_size: 0,
            quality: 6,
        }),
        enable_request_id: true,
        ..Default::default()
    };

    let handler: Arc<dyn Handler> = Arc::new(LargeJsonHandler);
    let app = Server::with_handlers(config, vec![(route("/data"), handler)]).unwrap();

    let response = app
        .oneshot(
            axum::http::Request::builder()
                .method("GET")
                .uri("/data")
                .header("accept-encoding", "gzip")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let request_id = response
        .headers()
        .get("x-request-id")
        .expect("x-request-id header missing")
        .to_str()
        .unwrap();
    assert!(
        Uuid::parse_str(request_id).is_ok(),
        "x-request-id is not a UUID: {request_id}"
    );

    assert_eq!(
        response
            .headers()
            .get("content-encoding")
            .expect("content-encoding missing")
            .to_str()
            .unwrap(),
        "gzip"
    );

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    assert!(!bytes.is_empty());
}

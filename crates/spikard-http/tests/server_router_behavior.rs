use axum::body::Body;
use http_body_util::BodyExt;
use spikard_http::handler_trait::{Handler, HandlerResult, RequestData};
use spikard_http::server::build_router_with_handlers;
use spikard_http::{Method, Route};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tower::ServiceExt;

#[cfg(feature = "di")]
fn build_app(routes: Vec<(Route, Arc<dyn Handler>)>) -> axum::Router {
    build_router_with_handlers(routes, None, None).unwrap()
}

#[cfg(not(feature = "di"))]
fn build_app(routes: Vec<(Route, Arc<dyn Handler>)>) -> axum::Router {
    build_router_with_handlers(routes, None).unwrap()
}

struct CaptureHandler {
    tx: Mutex<Option<tokio::sync::oneshot::Sender<RequestData>>>,
}

impl CaptureHandler {
    const fn new(tx: tokio::sync::oneshot::Sender<RequestData>) -> Self {
        Self {
            tx: Mutex::new(Some(tx)),
        }
    }
}

impl Handler for CaptureHandler {
    fn call(
        &self,
        _request: axum::http::Request<Body>,
        request_data: RequestData,
    ) -> Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(async move {
            let maybe_tx = self.tx.lock().expect("lock").take();
            if let Some(tx) = maybe_tx {
                let _ = tx.send(request_data);
            }
            Ok(axum::http::Response::builder().status(200).body(Body::empty()).unwrap())
        })
    }
}

fn route(path: &str, method: Method) -> Route {
    Route {
        path: path.to_string(),
        method,
        handler_name: "capture".to_string(),
        expects_json_body: true,
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
async fn post_route_with_path_params_extracts_raw_body_and_path_params() {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let handler: Arc<dyn Handler> = Arc::new(CaptureHandler::new(tx));

    let path = ["/items/", "{", "id:int", "}"].concat();
    let app = build_app(vec![(route(&path, Method::Post), handler)]);

    let response = app
        .oneshot(
            axum::http::Request::builder()
                .method("POST")
                .uri("/items/123")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"ok":true}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let _ = response.into_body().collect().await.unwrap();

    let captured = rx.await.expect("handler should send request_data");
    assert_eq!(captured.method, "POST");
    assert_eq!(captured.path, "/items/123");
    assert_eq!(captured.path_params.get("id").map(String::as_str), Some("123"));
    assert_eq!(captured.raw_body.as_deref(), Some(&br#"{"ok":true}"#[..]));
}

#[tokio::test]
async fn get_route_without_body_does_not_set_raw_body() {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let handler: Arc<dyn Handler> = Arc::new(CaptureHandler::new(tx));

    let app = build_app(vec![(route("/health", Method::Get), handler)]);

    let response = app
        .oneshot(
            axum::http::Request::builder()
                .method("GET")
                .uri("/health?x=1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let _ = response.into_body().collect().await.unwrap();

    let captured = rx.await.expect("handler should send request_data");
    assert_eq!(captured.method, "GET");
    assert_eq!(captured.path, "/health");
    assert!(captured.raw_body.is_none());
}

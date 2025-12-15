use axum::body::Body;
use axum::http::{Method as AxumMethod, Request, StatusCode};
use serde_json::Value;
use spikard_http::server::build_router_with_handlers_and_config;
use spikard_http::{Handler, HandlerResult, Method, RequestData, Route, ServerConfig};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

struct EchoMethodHandler;

impl Handler for EchoMethodHandler {
    fn call(
        &self,
        _request: Request<Body>,
        request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(async move {
            let body_len = request_data.raw_body.as_ref().map_or(0, bytes::Bytes::len);
            let response_json = serde_json::json!({
                "method": request_data.method,
                "path": request_data.path,
                "path_params": &*request_data.path_params,
                "raw_body_len": body_len,
            });
            Ok(axum::http::Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(response_json.to_string()))
                .expect("response builder"))
        })
    }
}

fn route(method: Method, path: &str, expects_json_body: bool) -> Route {
    Route {
        method,
        path: path.to_string(),
        handler_name: "echo_method".to_string(),
        expects_json_body,
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

async fn assert_method(server: &axum_test::TestServer, method: AxumMethod, path: &str) {
    let response = server.method(method.clone(), path).await;
    assert_eq!(response.status_code(), StatusCode::OK);
    if method == AxumMethod::HEAD {
        return;
    }
    let json: Value = serde_json::from_str(&response.text()).expect("json");
    assert_eq!(json["method"], method.as_str());
}

#[tokio::test]
async fn router_covers_all_http_methods_with_and_without_path_params() {
    let handler: Arc<dyn Handler> = Arc::new(EchoMethodHandler);
    let route_entries = vec![
        route(Method::Get, "/m", false),
        route(Method::Delete, "/m", false),
        route(Method::Head, "/m", false),
        route(Method::Trace, "/m", false),
        route(Method::Options, "/m", false),
        route(Method::Post, "/m", false),
        route(Method::Put, "/m", false),
        route(Method::Patch, "/m", false),
        route(Method::Get, "/params/{id}", false),
        route(Method::Delete, "/params/{id}", false),
        route(Method::Head, "/params/{id}", false),
        route(Method::Trace, "/params/{id}", false),
        route(Method::Options, "/params/{id}", false),
        route(Method::Post, "/params/{id}", false),
        route(Method::Put, "/params/{id}", false),
        route(Method::Patch, "/params/{id}", false),
    ]
    .into_iter()
    .map(|r| (r, Arc::clone(&handler)))
    .collect();

    let app_router =
        build_router_with_handlers_and_config(route_entries, ServerConfig::default(), Vec::new()).expect("router");
    let server = axum_test::TestServer::new(app_router).expect("server");

    assert_method(&server, AxumMethod::GET, "/m").await;
    assert_method(&server, AxumMethod::DELETE, "/m").await;
    assert_method(&server, AxumMethod::HEAD, "/m").await;
    assert_method(&server, AxumMethod::TRACE, "/m").await;
    assert_method(&server, AxumMethod::OPTIONS, "/m").await;

    let post = server.post("/m").bytes(Vec::from("body").into()).await;
    let post_json: Value = serde_json::from_str(&post.text()).expect("json");
    assert_eq!(post_json["method"], "POST");
    assert_eq!(post_json["raw_body_len"], 4);

    let put = server.put("/m").bytes(Vec::from("body").into()).await;
    let put_json: Value = serde_json::from_str(&put.text()).expect("json");
    assert_eq!(put_json["method"], "PUT");
    assert_eq!(put_json["raw_body_len"], 4);

    let patch = server.patch("/m").bytes(Vec::from("body").into()).await;
    let patch_json: Value = serde_json::from_str(&patch.text()).expect("json");
    assert_eq!(patch_json["method"], "PATCH");
    assert_eq!(patch_json["raw_body_len"], 4);

    assert_method(&server, AxumMethod::GET, "/params/123").await;
    let response = server.method(AxumMethod::DELETE, "/params/123").await;
    assert_eq!(response.status_code(), StatusCode::OK);
    let json: Value = serde_json::from_str(&response.text()).expect("json");
    assert_eq!(json["path_params"]["id"], "123");
}

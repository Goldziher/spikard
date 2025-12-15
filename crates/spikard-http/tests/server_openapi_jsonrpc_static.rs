use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::Value;
use spikard_core::router::JsonRpcMethodInfo;
use spikard_http::server::build_router_with_handlers_and_config;
use spikard_http::{
    Handler, HandlerResult, JsonRpcConfig, Method, OpenApiConfig, RateLimitConfig, RequestData, Route, RouteMetadata,
    ServerConfig, StaticFilesConfig,
};
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;

struct EchoHandler;

impl Handler for EchoHandler {
    fn call(
        &self,
        _request: Request<Body>,
        request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(async move {
            let response_json = serde_json::json!({
                "method": request_data.method,
                "path": request_data.path,
                "path_params": &*request_data.path_params,
                "body": request_data.body,
                "raw_body_json": request_data
                    .raw_body
                    .as_ref()
                    .and_then(|bytes| serde_json::from_slice::<Value>(bytes.as_ref()).ok()),
            });

            Ok(axum::http::Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(response_json.to_string()))
                .expect("response builder"))
        })
    }
}

fn api_items_path() -> String {
    ["api/items/", "{", "id:uuid", "}"].concat()
}

fn build_routes(path: &str) -> Vec<(Route, Arc<dyn Handler>)> {
    vec![
        (
            Route {
                method: Method::Get,
                path: path.to_string(),
                handler_name: "echo_get".to_string(),
                expects_json_body: false,
                cors: None,
                is_async: true,
                file_params: None,
                request_validator: None,
                response_validator: None,
                parameter_validator: None,
                jsonrpc_method: Some(JsonRpcMethodInfo {
                    method_name: "spikard.test.echo".to_string(),
                    description: Some("Echo JSON-RPC".to_string()),
                    params_schema: None,
                    result_schema: None,
                    deprecated: false,
                    tags: vec!["test".to_string()],
                }),
                #[cfg(feature = "di")]
                handler_dependencies: Vec::new(),
            },
            Arc::new(EchoHandler) as Arc<dyn Handler>,
        ),
        (
            Route {
                method: Method::Post,
                path: path.to_string(),
                handler_name: "echo_post".to_string(),
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
            },
            Arc::new(EchoHandler) as Arc<dyn Handler>,
        ),
    ]
}

fn build_route_metadata(path: &str) -> Vec<RouteMetadata> {
    let request_schema = serde_json::json!({
        "type": "object",
        "properties": {
            "name": {"type": "string"}
        },
        "required": ["name"]
    });

    let response_schema = serde_json::json!({
        "type": "object",
        "properties": {
            "ok": {"type": "boolean"}
        },
        "required": ["ok"]
    });

    vec![
        RouteMetadata {
            method: "GET".to_string(),
            path: path.to_string(),
            handler_name: "echo_get".to_string(),
            request_schema: None,
            response_schema: None,
            parameter_schema: None,
            file_params: None,
            is_async: true,
            cors: None,
            body_param_name: None,
            #[cfg(feature = "di")]
            handler_dependencies: None,
            jsonrpc_method: Some(
                serde_json::to_value(JsonRpcMethodInfo {
                    method_name: "spikard.test.echo".to_string(),
                    description: Some("Echo JSON-RPC".to_string()),
                    params_schema: None,
                    result_schema: None,
                    deprecated: false,
                    tags: vec!["test".to_string()],
                })
                .expect("jsonrpc method info"),
            ),
        },
        RouteMetadata {
            method: "POST".to_string(),
            path: path.to_string(),
            handler_name: "echo_post".to_string(),
            request_schema: Some(request_schema),
            response_schema: Some(response_schema),
            parameter_schema: None,
            file_params: None,
            is_async: true,
            cors: None,
            body_param_name: Some("body".to_string()),
            #[cfg(feature = "di")]
            handler_dependencies: None,
            jsonrpc_method: None,
        },
    ]
}

fn build_config(static_dir: &Path) -> ServerConfig {
    ServerConfig {
        openapi: Some(OpenApiConfig {
            enabled: true,
            title: "Spikard Test API".to_string(),
            version: "0.1.0".to_string(),
            ..OpenApiConfig::default()
        }),
        jsonrpc: Some(JsonRpcConfig::default()),
        static_files: vec![StaticFilesConfig {
            directory: static_dir.to_string_lossy().into_owned(),
            route_prefix: "/static".to_string(),
            index_file: true,
            cache_control: Some("public, max-age=60".to_string()),
        }],
        rate_limit: Some(RateLimitConfig {
            per_second: 100,
            burst: 10,
            ip_based: false,
        }),
        ..Default::default()
    }
}

async fn assert_openapi_docs_and_redoc(server: &axum_test::TestServer) {
    let openapi_response = server.get("/openapi.json").await;
    assert_eq!(openapi_response.status_code(), StatusCode::OK);
    let openapi: Value = serde_json::from_str(&openapi_response.text()).expect("openapi json");
    assert!(openapi.get("openapi").is_some());
    assert!(openapi.get("paths").is_some());

    let swagger_html = server.get("/docs").await;
    assert_eq!(swagger_html.status_code(), StatusCode::OK);
    assert!(swagger_html.text().contains("SwaggerUIBundle"));
    assert!(swagger_html.text().contains("/openapi.json"));

    let redoc_html = server.get("/redoc").await;
    assert_eq!(redoc_html.status_code(), StatusCode::OK);
    assert!(redoc_html.text().contains("<redoc"));
    assert!(redoc_html.text().contains("/openapi.json"));
}

async fn assert_static_files(server: &axum_test::TestServer) {
    let static_index = server.get("/static/").await;
    assert_eq!(static_index.status_code(), StatusCode::OK);
    assert!(static_index.text().contains("static index"));
    assert_eq!(
        static_index.header("cache-control").to_str().expect("cache-control"),
        "public, max-age=60"
    );

    let static_file = server.get("/static/hello.txt").await;
    assert_eq!(static_file.status_code(), StatusCode::OK);
    assert_eq!(static_file.text(), "hello world");
    assert_eq!(
        static_file.header("cache-control").to_str().expect("cache-control"),
        "public, max-age=60"
    );
}

async fn assert_jsonrpc_and_http_routes(server: &axum_test::TestServer) {
    let rpc_request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "spikard.test.echo",
        "params": {"any": "thing"},
        "id": 1
    });
    let rpc_response = server.post("/rpc").json(&rpc_request).await;
    assert_eq!(rpc_response.status_code(), StatusCode::OK);
    let rpc_json: Value = serde_json::from_str(&rpc_response.text()).expect("jsonrpc response");
    assert_eq!(rpc_json["jsonrpc"], "2.0");
    assert_eq!(rpc_json["id"], 1);
    assert_eq!(rpc_json["result"]["path"], "/rpc");
    assert_eq!(rpc_json["result"]["method"], "POST");

    let ok_get = server.get("/api/items/550e8400-e29b-41d4-a716-446655440000").await;
    assert_eq!(ok_get.status_code(), StatusCode::OK);
    let ok_get_json: Value = serde_json::from_str(&ok_get.text()).expect("get json");
    assert_eq!(ok_get_json["path_params"]["id"], "550e8400-e29b-41d4-a716-446655440000");

    let ok_post = server
        .post("/api/items/550e8400-e29b-41d4-a716-446655440000")
        .json(&serde_json::json!({"name": "spikard"}))
        .await;
    assert_eq!(ok_post.status_code(), StatusCode::OK);
    let ok_post_json: Value = serde_json::from_str(&ok_post.text()).expect("post json");
    assert_eq!(ok_post_json["raw_body_json"]["name"], "spikard");
}

#[tokio::test]
async fn router_supports_openapi_jsonrpc_and_static_files_in_one_config() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("index.html"), "<h1>static index</h1>").expect("write index.html");
    std::fs::write(dir.path().join("hello.txt"), "hello world").expect("write hello.txt");

    let api_items_path = api_items_path();
    let route_entries = build_routes(&api_items_path);
    let route_metadata = build_route_metadata(&api_items_path);
    let config = build_config(dir.path());

    let app_router = build_router_with_handlers_and_config(route_entries, config, route_metadata).expect("router");
    let server = axum_test::TestServer::new(app_router).expect("test server");

    assert_openapi_docs_and_redoc(&server).await;
    assert_static_files(&server).await;
    assert_jsonrpc_and_http_routes(&server).await;
}

#[test]
fn router_returns_error_for_invalid_cache_control_header_value() {
    let routes: Vec<(Route, Arc<dyn Handler>)> = Vec::new();

    let config = ServerConfig {
        static_files: vec![StaticFilesConfig {
            directory: "/tmp".to_string(),
            route_prefix: "/static".to_string(),
            index_file: true,
            cache_control: Some("\n".to_string()),
        }],
        ..Default::default()
    };

    let err = build_router_with_handlers_and_config(routes, config, Vec::new()).expect_err("invalid header");
    assert!(err.contains("Invalid cache-control header"));
}

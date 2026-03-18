use axum::body::Body;
use http_body_util::BodyExt;
use spikard_core::JsonRpcMethodInfo;
use spikard_http::grpc::framing::{encode_grpc_message, parse_unary_grpc_message};
use spikard_http::grpc::{GrpcConfig, GrpcHandler, GrpcHandlerResult, GrpcRegistry, GrpcRequestData, GrpcResponseData};
use spikard_http::handler_trait::{Handler, HandlerResult, RequestData};
use spikard_http::jsonrpc::JsonRpcConfig;
use spikard_http::openapi::OpenApiConfig;
use spikard_http::{Method, ParameterValidator, Route, SchemaValidator, Server, ServerConfig, StaticFilesConfig};
use std::pin::Pin;
use std::sync::Arc;
use tempfile::tempdir;
use tower::ServiceExt;

struct JsonOkHandler;

impl Handler for JsonOkHandler {
    fn call(
        &self,
        _request: axum::http::Request<Body>,
        _request_data: RequestData,
    ) -> Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(async move {
            Ok(axum::http::Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"ok":true}"#))
                .unwrap())
        })
    }
}

struct UnaryGrpcHandler;

impl GrpcHandler for UnaryGrpcHandler {
    fn service_name(&self) -> &'static str {
        "example.UserService"
    }

    fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn std::future::Future<Output = GrpcHandlerResult> + Send>> {
        Box::pin(async move {
            let mut metadata = tonic::metadata::MetadataMap::new();
            metadata.insert("x-service", "users".parse().unwrap());
            Ok(GrpcResponseData {
                payload: request.payload,
                metadata,
            })
        })
    }
}

fn route(path: &str, method: Method) -> Route {
    Route {
        path: path.to_string(),
        method,
        handler_name: "ok".to_string(),
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
async fn server_routes_grpc_requests_when_registry_is_registered() {
    let mut grpc_registry = GrpcRegistry::new();
    grpc_registry.register_service(
        "example.UserService",
        Arc::new(UnaryGrpcHandler),
        spikard_http::grpc::RpcMode::Unary,
    );

    let config = ServerConfig {
        grpc: Some(GrpcConfig::default()),
        ..Default::default()
    };

    let app = Server::with_handlers_metadata_and_grpc(config, Vec::new(), Vec::new(), Arc::new(grpc_registry)).unwrap();

    let response = app
        .oneshot(
            axum::http::Request::builder()
                .method("POST")
                .uri("/example.UserService/GetUser")
                .header("content-type", "application/grpc")
                .body(Body::from(
                    encode_grpc_message(bytes::Bytes::from_static(b"hello")).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    assert_eq!(
        response.headers().get("content-type").unwrap().to_str().unwrap(),
        "application/grpc+proto"
    );
    assert_eq!(response.headers().get("grpc-status").unwrap(), "0");
    assert_eq!(response.headers().get("x-service").unwrap(), "users");

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let payload = parse_unary_grpc_message(&bytes, 1024, None, true).unwrap();
    assert_eq!(payload, bytes::Bytes::from_static(b"hello"));
}

#[tokio::test]
async fn server_with_openapi_and_static_files_serves_expected_endpoints() {
    let dir = tempdir().unwrap();
    let index_path = dir.path().join("index.html");
    std::fs::write(&index_path, "<h1>hello</h1>").unwrap();

    let openapi = OpenApiConfig {
        enabled: true,
        title: "Test".to_string(),
        version: "0.4.0".to_string(),
        openapi_json_path: "/openapi.json".to_string(),
        swagger_ui_path: "/docs".to_string(),
        redoc_path: "/redoc".to_string(),
        ..Default::default()
    };

    let jsonrpc = JsonRpcConfig {
        enabled: true,
        endpoint_path: "/rpc".to_string(),
        ..Default::default()
    };

    let config = ServerConfig {
        openapi: Some(openapi),
        jsonrpc: Some(jsonrpc),
        static_files: vec![StaticFilesConfig {
            directory: dir.path().display().to_string(),
            route_prefix: "/static".to_string(),
            index_file: true,
            cache_control: Some("public, max-age=60".to_string()),
        }],
        ..Default::default()
    };

    let handler: Arc<dyn Handler> = Arc::new(JsonOkHandler);
    let app = Server::with_handlers(config, vec![(route("/ping", Method::Get), handler)]).unwrap();

    let openapi_response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method("GET")
                .uri("/openapi.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(openapi_response.status(), 200);
    assert_eq!(
        openapi_response
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap(),
        "application/json"
    );

    let openapi_body = openapi_response.into_body().collect().await.unwrap().to_bytes();
    let openapi_json: serde_json::Value = serde_json::from_slice(&openapi_body).unwrap();
    assert_eq!(openapi_json.get("openapi").and_then(|v| v.as_str()), Some("3.1.0"));

    let docs_response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method("GET")
                .uri("/docs")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(docs_response.status(), 200);
    let docs_body = docs_response.into_body().collect().await.unwrap().to_bytes();
    assert!(String::from_utf8_lossy(&docs_body).contains("SwaggerUIBundle"));

    let static_response = app
        .oneshot(
            axum::http::Request::builder()
                .method("GET")
                .uri("/static/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(static_response.status(), 200);
    assert_eq!(
        static_response
            .headers()
            .get(axum::http::header::CACHE_CONTROL)
            .unwrap()
            .to_str()
            .unwrap(),
        "public, max-age=60"
    );
    let static_body = static_response.into_body().collect().await.unwrap().to_bytes();
    assert!(String::from_utf8_lossy(&static_body).contains("<h1>hello</h1>"));
}

#[tokio::test]
async fn server_registers_jsonrpc_endpoint_when_method_metadata_present() {
    let jsonrpc = JsonRpcConfig {
        enabled: true,
        endpoint_path: "/rpc".to_string(),
        ..Default::default()
    };

    let config = ServerConfig {
        jsonrpc: Some(jsonrpc),
        ..Default::default()
    };

    let mut rpc_route = route("/rpc_method", Method::Post);
    rpc_route.jsonrpc_method = Some(JsonRpcMethodInfo {
        method_name: "math.ping".to_string(),
        description: Some("ping".to_string()),
        params_schema: None,
        result_schema: None,
        deprecated: false,
        tags: vec!["math".to_string()],
    });

    let handler: Arc<dyn Handler> = Arc::new(JsonOkHandler);
    let app = Server::with_handlers(config, vec![(rpc_route, handler)]).unwrap();

    let response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method("POST")
                .uri("/rpc")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"jsonrpc":"2.0","method":"math.ping","id":1}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    assert_eq!(
        response.headers().get("content-type").unwrap().to_str().unwrap(),
        "application/json"
    );

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json.get("jsonrpc").and_then(|v| v.as_str()), Some("2.0"));
    assert_eq!(json.get("id").and_then(serde_json::Value::as_i64), Some(1));
    assert_eq!(json.get("result"), Some(&serde_json::json!({"ok": true})));

    let openrpc_response = app
        .oneshot(
            axum::http::Request::builder()
                .method("GET")
                .uri("/openrpc.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(openrpc_response.status(), 200);
    let openrpc_bytes = openrpc_response.into_body().collect().await.unwrap().to_bytes();
    let openrpc_json: serde_json::Value = serde_json::from_slice(&openrpc_bytes).unwrap();
    assert_eq!(openrpc_json.get("openrpc").and_then(|v| v.as_str()), Some("1.3.2"));
    assert_eq!(openrpc_json["methods"][0]["name"], "math.ping");
    assert_eq!(openrpc_json["methods"][0]["tags"][0]["name"], "math");
}

#[tokio::test]
async fn server_with_handlers_preserves_route_schema_metadata_for_openapi() {
    let openapi = OpenApiConfig {
        enabled: true,
        title: "Metadata".to_string(),
        version: "0.1.0".to_string(),
        openapi_json_path: "/openapi.json".to_string(),
        ..Default::default()
    };

    let config = ServerConfig {
        openapi: Some(openapi),
        ..Default::default()
    };

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
    let parameter_schema = serde_json::json!({
        "type": "object",
        "properties": {
            "id": {
                "type": "string",
                "source": "path",
                "format": "uuid"
            }
        },
        "required": ["id"]
    });

    let handler: Arc<dyn Handler> = Arc::new(JsonOkHandler);
    let app = Server::with_handlers(
        config,
        vec![(
            Route {
                path: "/items/{id}".to_string(),
                method: Method::Post,
                handler_name: "create_item".to_string(),
                expects_json_body: true,
                cors: None,
                is_async: true,
                file_params: None,
                request_validator: Some(Arc::new(SchemaValidator::new(request_schema).unwrap())),
                response_validator: Some(Arc::new(SchemaValidator::new(response_schema).unwrap())),
                parameter_validator: Some(ParameterValidator::new(parameter_schema).unwrap()),
                jsonrpc_method: None,
                #[cfg(feature = "di")]
                handler_dependencies: vec![],
            },
            handler,
        )],
    )
    .unwrap();

    let openapi_response = app
        .oneshot(
            axum::http::Request::builder()
                .method("GET")
                .uri("/openapi.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(openapi_response.status(), 200);
    let openapi_bytes = openapi_response.into_body().collect().await.unwrap().to_bytes();
    let openapi_json: serde_json::Value = serde_json::from_slice(&openapi_bytes).unwrap();
    let operation = &openapi_json["paths"]["/items/{id}"]["post"];
    assert_eq!(operation["parameters"][0]["name"].as_str(), Some("id"));
    assert_eq!(
        operation["requestBody"]["content"]["application/json"]["schema"]["properties"]["name"]["type"].as_str(),
        Some("string")
    );
    assert_eq!(
        operation["responses"]["200"]["content"]["application/json"]["schema"]["properties"]["ok"]["type"].as_str(),
        Some("boolean")
    );
}

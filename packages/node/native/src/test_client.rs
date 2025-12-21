//! Test client for making HTTP requests to Spikard applications
//!
//! Uses the shared spikard_http router pipeline and bridges into JavaScript
//! handlers via napi ThreadsafeFunction.
//!
//! NOTE: This module is temporarily disabled pending handler trait refactor

#![allow(dead_code, unused_imports)]

use crate::response::{HandlerReturnValue, TestResponse};
use axum::body::Body;
use axum::extract::Request;
use axum::http::{HeaderMap, HeaderValue, Method, StatusCode, header};
use axum_test::{TestServer, Transport};
use bytes::Bytes;
use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunction;
use napi_derive::napi;
use serde_json::{Map as JsonMap, Value, json};
// TODO: Update to use current handler trait API
use crate::test_sse;
use crate::test_websocket;
use spikard_http::problem::ProblemDetails;
use spikard_http::testing::{
    MultipartFilePart, SnapshotError, build_multipart_body, encode_urlencoded_body, snapshot_response,
};
use spikard_http::{HandlerResult, RequestData, ResponseBodySize, Server};
use spikard_http::{Route, RouteMetadata, SchemaValidator};
use std::collections::HashMap;
use std::sync::Arc;

fn is_debug_mode() -> bool {
    std::env::var("DEBUG")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

fn build_js_payload(_handler: &JsHandler, request_data: &RequestData, validated_params: Option<Value>) -> Value {
    let path_params =
        validated_params.unwrap_or_else(|| serde_json::to_value(&*request_data.path_params).unwrap_or(Value::Null));
    let headers = serde_json::to_value(&*request_data.headers).unwrap_or(Value::Null);
    let cookies = serde_json::to_value(&*request_data.cookies).unwrap_or(Value::Null);
    #[cfg(feature = "di")]
    let dependencies = if let Some(resolved) = &request_data.dependencies {
        let mut deps_map: JsonMap<String, Value> = JsonMap::new();
        for key in resolved.keys() {
            if let Some(value_json) = resolved.get::<String>(&key)
                && let Ok(parsed) = serde_json::from_str::<Value>(&value_json)
            {
                deps_map.insert(key, parsed);
            }
        }
        Value::Object(deps_map)
    } else {
        Value::Null
    };
    #[cfg(not(feature = "di"))]
    let dependencies = Value::Null;

    let body_bytes = request_data
        .raw_body
        .as_ref()
        .map(|bytes| Value::Array(bytes.iter().copied().map(|b| Value::Number(b.into())).collect()))
        .unwrap_or(Value::Null);
    let query: JsonMap<String, Value> = request_data
        .raw_query_params
        .iter()
        .filter_map(|(k, values)| values.first().map(|value| (k.clone(), Value::String(value.clone()))))
        .collect();

    json!({
        "method": request_data.method,
        "path": request_data.path,
        "params": path_params,
        "query": Value::Object(query),
        "headers": headers,
        "cookies": cookies,
        "body": body_bytes,
        "dependencies": dependencies,
    })
}

#[derive(Clone)]
struct HandlerResponsePayload {
    status: u16,
    headers: HashMap<String, String>,
    body: Option<Value>,
}

fn interpret_handler_response(value: Value) -> HandlerResponsePayload {
    if let Value::Object(mut obj) = value {
        let status = obj
            .remove("status")
            .or_else(|| obj.remove("statusCode"))
            .and_then(|v| v.as_u64())
            .map(|n| n as u16)
            .unwrap_or(200);

        let headers = obj
            .remove("headers")
            .and_then(|v| v.as_object().cloned())
            .map(|map| {
                map.into_iter()
                    .filter_map(|(k, v)| match v {
                        Value::String(s) => Some((k, s)),
                        Value::Number(n) => Some((k, n.to_string())),
                        Value::Bool(b) => Some((k, b.to_string())),
                        _ => None,
                    })
                    .collect::<HashMap<String, String>>()
            })
            .unwrap_or_default();

        let body = if let Some(body_value) = obj.remove("body") {
            match body_value {
                Value::Null => None,
                other => Some(other),
            }
        } else if obj.is_empty() {
            None
        } else {
            Some(Value::Object(obj))
        };

        HandlerResponsePayload { status, headers, body }
    } else {
        HandlerResponsePayload {
            status: 200,
            headers: HashMap::new(),
            body: Some(value),
        }
    }
}

fn problem_to_json(problem: &ProblemDetails) -> String {
    problem
        .to_json_pretty()
        .unwrap_or_else(|e| format!("Failed to serialize problem details: {}", e))
}

/// JavaScript handler wrapper that can be called from Rust async context
#[derive(Clone)]
struct JsHandler {
    handler_fn: Arc<ThreadsafeFunction<String, Promise<HandlerReturnValue>, Vec<String>, napi::Status, false>>,
    handler_name: String,
    method: String,
    path: String,
    request_validator: Option<Arc<SchemaValidator>>,
    response_validator: Option<Arc<SchemaValidator>>,
}

impl JsHandler {
    fn new(js_fn: Function<String, Promise<HandlerReturnValue>>, route: &Route) -> Result<Self> {
        let tsfn = js_fn
            .build_threadsafe_function()
            .build_callback(|ctx| Ok(vec![ctx.value]))?;

        Ok(Self {
            handler_fn: Arc::new(tsfn),
            handler_name: route.handler_name.clone(),
            method: route.method.as_str().to_string(),
            path: route.path.clone(),
            request_validator: route.request_validator.clone(),
            response_validator: route.response_validator.clone(),
        })
    }

    async fn handle(&self, request_data: RequestData) -> HandlerResult {
        if let Some(validator) = &self.request_validator
            && let Err(errors) = validator.validate(&request_data.body)
        {
            let problem = ProblemDetails::from_validation_error(&errors);
            let error_json = problem_to_json(&problem);
            return Err((problem.status_code(), error_json));
        }

        let payload = build_js_payload(self, &request_data, request_data.validated_params.clone());

        let js_result = self.call_js(payload).await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Handler invocation failed: {}", e),
            )
        })?;

        if let HandlerReturnValue::Streaming(streaming) = js_result {
            let response = streaming
                .into_handler_response()
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            return Ok(response.into_response());
        }

        let json_value = match js_result {
            HandlerReturnValue::Json(text) => serde_json::from_str(&text).map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to parse handler response: {}", e),
                )
            })?,
            HandlerReturnValue::Streaming(_) => unreachable!(),
        };

        let mut handler_response = interpret_handler_response(json_value);
        let response_body_clone = handler_response.body.clone();

        if let (Some(validator), Some(body_value)) = (&self.response_validator, response_body_clone.as_ref())
            && let Err(errors) = validator.validate(body_value)
        {
            let error_message = if is_debug_mode() {
                json!({
                    "error": "Response validation failed",
                    "validation_errors": format!("{:?}", errors),
                    "response_body": body_value,
                    "handler": self.handler_name,
                })
                .to_string()
            } else {
                "Internal server error".to_string()
            };
            eprintln!(
                "[spikard-node:test-client] response validation failed for {}: {}",
                self.handler_name, error_message
            );
            return Err((StatusCode::INTERNAL_SERVER_ERROR, error_message));
        }

        if handler_response.status >= 400
            && let Some(debug_json) = handler_response
                .body
                .as_ref()
                .and_then(|body| serde_json::to_string(body).ok())
        {
            eprintln!(
                "[spikard-node:test-client] handler returned error for {}: status={} body={}",
                self.handler_name, handler_response.status, debug_json
            );
        }

        let mut response_builder = axum::http::Response::builder().status(handler_response.status);
        let mut has_content_type = false;

        for (name, value) in handler_response.headers.iter() {
            if let Ok(header_value) = HeaderValue::from_str(value) {
                if name.eq_ignore_ascii_case("content-type") {
                    has_content_type = true;
                }
                response_builder = response_builder.header(name, header_value);
            }
        }

        if !has_content_type {
            response_builder = response_builder.header("content-type", HeaderValue::from_static("application/json"));
        }

        let body_bytes = if let Some(body_value) = handler_response.body.take() {
            serde_json::to_vec(&body_value).map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to serialize handler response: {}", e),
                )
            })?
        } else {
            Vec::new()
        };
        let body_len = body_bytes.len();

        if !response_builder
            .headers_ref()
            .map(|headers| headers.contains_key(header::CONTENT_LENGTH))
            .unwrap_or(false)
        {
            let content_length = body_len.to_string();
            if let Ok(header_value) = HeaderValue::from_str(&content_length) {
                response_builder = response_builder.header(header::CONTENT_LENGTH, header_value);
            }
        }

        let mut response = response_builder.body(Body::from(body_bytes)).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to build response: {}", e),
            )
        })?;
        response.extensions_mut().insert(ResponseBodySize(body_len));
        Ok(response)
    }

    async fn call_js(&self, payload: Value) -> Result<HandlerReturnValue> {
        let request_json = serde_json::to_string(&payload)
            .map_err(|e| Error::from_reason(format!("Failed to serialize request payload: {}", e)))?;

        let result = self
            .handler_fn
            .call_async(request_json)
            .await
            .map_err(|e| Error::from_reason(format!("Handler call failed: {}", e)))?
            .await
            .map_err(|e| Error::from_reason(format!("Handler promise failed: {}", e)))?;

        Ok(result)
    }
}

impl spikard_http::Handler for JsHandler {
    fn call(
        &self,
        _req: Request<Body>,
        request_data: RequestData,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send + '_>> {
        let handler = self.clone();
        Box::pin(async move { handler.handle(request_data).await })
    }
}

/// Test client for making HTTP requests to a Spikard application
#[napi]
pub struct TestClient {
    server: Arc<TestServer>,
    #[allow(dead_code)]
    http_runtime: Option<std::sync::Arc<tokio::runtime::Runtime>>,
}

#[napi]
impl TestClient {
    /// Create a new test client from routes and handlers
    #[napi(constructor)]
    pub fn new(
        routes_json: String,
        websocket_routes_json: Option<String>,
        handlers_map: Object,
        websocket_handlers: Option<Object>,
        dependencies: Option<Object>,
        lifecycle_hooks: Option<Object>,
        config: Option<Object>,
    ) -> Result<Self> {
        let routes_data: Vec<RouteMetadata> = serde_json::from_str(&routes_json)
            .map_err(|e| Error::from_reason(format!("Failed to parse routes: {}", e)))?;

        let websocket_routes_data: Vec<RouteMetadata> = websocket_routes_json
            .as_ref()
            .and_then(|json| serde_json::from_str(json).ok())
            .unwrap_or_default();

        let mut server_config = if let Some(cfg) = config {
            crate::extract_server_config(&cfg)?
        } else {
            spikard_http::ServerConfig {
                compression: None,
                ..Default::default()
            }
        };

        #[cfg(feature = "di")]
        let dependency_container = if let Some(deps_obj) = dependencies {
            crate::di::extract_dependency_container(&deps_obj)?
        } else {
            None
        };
        #[cfg(not(feature = "di"))]
        let dependency_container: Option<std::sync::Arc<spikard_core::di::DependencyContainer>> = None;

        let lifecycle_hooks = if let Some(hooks_obj) = lifecycle_hooks {
            let mut hooks = spikard_http::LifecycleHooks::new();

            let extract_hooks =
                |hooks_obj: &Object, hook_type: &str| -> Result<Vec<crate::lifecycle::NodeLifecycleHook>> {
                    let hook_array: Result<Object> = hooks_obj.get_named_property(hook_type);
                    if let Ok(arr) = hook_array {
                        let length = arr.get_array_length()?;
                        let mut result = Vec::new();

                        for i in 0..length {
                            let js_fn: Function<String, Promise<String>> = arr.get_element(i)?;
                            let name = format!("{}_{}", hook_type, i);

                            let tsfn = js_fn
                                .build_threadsafe_function()
                                .build_callback(|ctx| Ok(vec![ctx.value]))
                                .map_err(|e| {
                                    Error::from_reason(format!(
                                        "Failed to build ThreadsafeFunction for hook '{}': {}",
                                        name, e
                                    ))
                                })?;

                            result.push(crate::lifecycle::NodeLifecycleHook::new(name, tsfn));
                        }

                        Ok(result)
                    } else {
                        Ok(Vec::new())
                    }
                };

            for hook in extract_hooks(&hooks_obj, "onRequest")? {
                hooks.add_on_request(std::sync::Arc::new(hook));
            }

            for hook in extract_hooks(&hooks_obj, "preValidation")? {
                hooks.add_pre_validation(std::sync::Arc::new(hook));
            }

            for hook in extract_hooks(&hooks_obj, "preHandler")? {
                hooks.add_pre_handler(std::sync::Arc::new(hook));
            }

            for hook in extract_hooks(&hooks_obj, "onResponse")? {
                hooks.add_on_response(std::sync::Arc::new(hook));
            }

            for hook in extract_hooks(&hooks_obj, "onError")? {
                hooks.add_on_error(std::sync::Arc::new(hook));
            }

            Some(hooks)
        } else {
            None
        };

        server_config.lifecycle_hooks = lifecycle_hooks.map(std::sync::Arc::new);
        server_config.di_container = dependency_container;

        let schema_registry = spikard_http::SchemaRegistry::new();

        let regular_routes_data = routes_data;

        let mut prepared_routes: Vec<(Route, Arc<dyn spikard_http::Handler>)> = Vec::new();
        let mut metadata_list: Vec<RouteMetadata> = Vec::new();

        for metadata in regular_routes_data {
            metadata_list.push(metadata.clone());

            let route = Route::from_metadata(metadata, &schema_registry)
                .map_err(|e| Error::from_reason(format!("Failed to build route: {}", e)))?;

            let handler_name = route.handler_name.clone();
            let js_fn: Function<String, Promise<HandlerReturnValue>> =
                handlers_map
                    .get_named_property(&handler_name)
                    .map_err(|e| Error::from_reason(format!("Failed to get handler '{}': {}", handler_name, e)))?;

            let js_handler = JsHandler::new(js_fn, &route)?;
            prepared_routes.push((route, Arc::new(js_handler) as Arc<dyn spikard_http::Handler>));
        }

        let mut axum_router =
            spikard_http::Server::with_handlers_and_metadata(server_config, prepared_routes, metadata_list)
                .map_err(|e| Error::from_reason(format!("Failed to build router: {}", e)))?;

        for ws_metadata in websocket_routes_data {
            let path = ws_metadata.path.clone();
            let handler_name = ws_metadata.handler_name.clone();

            let ws_map = websocket_handlers
                .as_ref()
                .ok_or_else(|| Error::from_reason(format!("Missing websocketHandlers map for {}", handler_name)))?;

            let handler_obj: Object = ws_map
                .get_named_property(&handler_name)
                .map_err(|e| Error::from_reason(format!("Failed to get WebSocket handler {}: {}", handler_name, e)))?;

            let ws_state = crate::websocket::create_websocket_state(&handler_obj)
                .map_err(|e| Error::from_reason(format!("Failed to build WebSocket state: {}", e)))?;

            use axum::routing::get;
            axum_router = axum_router.route(
                &path,
                get(spikard_http::websocket_handler::<crate::websocket::NodeWebSocketHandler>).with_state(ws_state),
            );
        }

        let runtime = std::sync::Arc::new(
            tokio::runtime::Runtime::new()
                .map_err(|e| Error::from_reason(format!("Failed to create Tokio runtime: {}", e)))?,
        );
        let handle = runtime.handle().clone();
        let _guard = handle.enter();
        let server = TestServer::builder()
            .transport(Transport::HttpRandomPort)
            .build(axum_router)
            .map_err(|e| Error::from_reason(format!("Failed to create test server: {}", e)))?;
        drop(_guard);

        Ok(Self {
            server: Arc::new(server),
            http_runtime: Some(runtime),
        })
    }

    #[napi]
    pub async fn get(&self, path: String, headers: Option<Value>) -> Result<TestResponse> {
        self.request("GET", path, headers, None).await
    }

    #[napi]
    pub async fn post(&self, path: String, headers: Option<Value>, json: Option<Value>) -> Result<TestResponse> {
        self.request("POST", path, headers, json).await
    }

    #[napi]
    pub async fn put(&self, path: String, headers: Option<Value>, json: Option<Value>) -> Result<TestResponse> {
        self.request("PUT", path, headers, json).await
    }

    #[napi]
    pub async fn delete(&self, path: String, headers: Option<Value>) -> Result<TestResponse> {
        self.request("DELETE", path, headers, None).await
    }

    #[napi]
    pub async fn patch(&self, path: String, headers: Option<Value>, json: Option<Value>) -> Result<TestResponse> {
        self.request("PATCH", path, headers, json).await
    }

    #[napi]
    pub async fn head(&self, path: String, headers: Option<Value>) -> Result<TestResponse> {
        self.request("HEAD", path, headers, None).await
    }

    #[napi]
    pub async fn options(&self, path: String, headers: Option<Value>) -> Result<TestResponse> {
        self.request("OPTIONS", path, headers, None).await
    }

    #[napi]
    pub async fn trace(&self, path: String, headers: Option<Value>) -> Result<TestResponse> {
        self.request("TRACE", path, headers, None).await
    }

    /// Connect to a Server-Sent Events endpoint
    #[napi]
    pub async fn sse(&self, path: String) -> Result<test_sse::SseStream> {
        let axum_response = self.server.get(&path).await;
        let snapshot = snapshot_response(axum_response).await.map_err(map_snapshot_error)?;

        test_sse::sse_stream_from_response(&snapshot)
    }

    /// Connect to a WebSocket endpoint
    #[napi]
    pub async fn websocket(&self, path: String) -> Result<test_websocket::WebSocketTestConnection> {
        let ws_conn = spikard_http::testing::connect_websocket(&self.server, &path).await;

        Ok(test_websocket::WebSocketTestConnection::new(ws_conn))
    }

    async fn request(
        &self,
        method: &str,
        path: String,
        headers: Option<Value>,
        body: Option<Value>,
    ) -> Result<TestResponse> {
        let mut request = match method {
            "GET" => self.server.get(&path),
            "POST" => self.server.post(&path),
            "PUT" => self.server.put(&path),
            "DELETE" => self.server.delete(&path),
            "PATCH" => self.server.patch(&path),
            "HEAD" | "OPTIONS" | "TRACE" => self.server.method(
                Method::from_bytes(method.as_bytes())
                    .map_err(|e| Error::from_reason(format!("Invalid method: {}", e)))?,
                &path,
            ),
            _ => return Err(Error::from_reason(format!("Unsupported method: {}", method))),
        };

        let mut headers_map = HeaderMap::new();

        if let Some(headers_obj) = headers.as_ref().and_then(Value::as_object) {
            for (name, value) in headers_obj {
                let value_str = value
                    .as_str()
                    .ok_or_else(|| Error::from_reason(format!("Header '{}' must be a string value", name)))?;

                let header_name = axum::http::HeaderName::from_bytes(name.as_bytes())
                    .map_err(|e| Error::from_reason(format!("Invalid header name: {}", e)))?;
                let header_value = axum::http::HeaderValue::from_str(value_str)
                    .map_err(|e| Error::from_reason(format!("Invalid header value: {}", e)))?;

                headers_map.insert(header_name.clone(), header_value.clone());
                request = request.add_header(header_name, header_value);
            }
        }

        let content_type = headers_map
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok());

        if let Some(body_value) = body {
            match determine_body_payload(&body_value, content_type) {
                BodyPayload::Json(json_data) => {
                    let body_vec = serde_json::to_vec(&json_data)
                        .map_err(|e| Error::from_reason(format!("Failed to serialize JSON body: {}", e)))?;
                    request = request.bytes(Bytes::from(body_vec));

                    if !headers_map
                        .keys()
                        .any(|name| name.as_str().eq_ignore_ascii_case("content-type"))
                    {
                        request = request.content_type("application/json");
                    }
                }
                BodyPayload::Form(form_data) => {
                    let body_vec = encode_urlencoded_body(&form_data)
                        .map_err(|err| Error::from_reason(format!("Failed to encode form body: {}", err)))?;
                    request = request.bytes(Bytes::from(body_vec));

                    if !headers_map
                        .keys()
                        .any(|name| name.as_str().eq_ignore_ascii_case("content-type"))
                    {
                        request = request.content_type("application/x-www-form-urlencoded");
                    }
                }
                BodyPayload::Multipart(multipart) => {
                    let (body_bytes, content_type) = encode_multipart_body(&multipart).map_err(Error::from_reason)?;

                    request = request.bytes(Bytes::from(body_bytes));
                    request = request.content_type(&content_type);
                }
                BodyPayload::Bytes(body_bytes) => {
                    request = request.bytes(Bytes::from(body_bytes));
                }
            }
        }

        let response = request.await;
        let snapshot = snapshot_response(response).await.map_err(map_snapshot_error)?;
        Ok(TestResponse::from_snapshot(snapshot))
    }
}

enum BodyPayload {
    Json(Value),
    Form(Value),
    Multipart(Value),
    Bytes(Vec<u8>),
}

fn determine_body_payload(value: &Value, content_type: Option<&str>) -> BodyPayload {
    if let Some(bytes) = extract_bytes(value) {
        if let Some(payload) = payload_from_bytes(&bytes, content_type) {
            return payload;
        }
        return BodyPayload::Bytes(bytes);
    }

    if let Some(payload) = payload_from_object(value, content_type) {
        return payload;
    }

    BodyPayload::Json(value.clone())
}

fn payload_from_bytes(bytes: &[u8], content_type: Option<&str>) -> Option<BodyPayload> {
    let decoded = std::str::from_utf8(bytes).ok()?;
    let parsed: Value = serde_json::from_str(decoded).ok()?;
    payload_from_object(&parsed, content_type)
}

fn payload_from_object(value: &Value, content_type: Option<&str>) -> Option<BodyPayload> {
    let obj = value.as_object()?;
    if let Some(form) = obj.get("__spikard_form__") {
        if content_type.is_some_and(is_form_content_type) {
            return Some(BodyPayload::Form(form.clone()));
        }
        return Some(BodyPayload::Json(value.clone()));
    }

    if let Some(multipart) = obj.get("__spikard_multipart__") {
        if content_type.is_some_and(is_multipart_content_type) {
            return Some(BodyPayload::Multipart(multipart.clone()));
        }
        return Some(BodyPayload::Json(value.clone()));
    }

    None
}

fn is_form_content_type(value: &str) -> bool {
    value.trim().to_ascii_lowercase().starts_with("application/x-www-form-urlencoded")
}

fn is_multipart_content_type(value: &str) -> bool {
    value.trim().to_ascii_lowercase().starts_with("multipart/form-data")
}

fn extract_bytes(value: &Value) -> Option<Vec<u8>> {
    let items = value.as_array()?;
    let mut bytes = Vec::with_capacity(items.len());
    for item in items {
        let number = item.as_u64()?;
        if number > u8::MAX as u64 {
            return None;
        }
        bytes.push(number as u8);
    }
    Some(bytes)
}

fn encode_multipart_body(value: &Value) -> std::result::Result<(Vec<u8>, String), String> {
    let obj = value
        .as_object()
        .ok_or_else(|| "Multipart payload must be an object".to_string())?;

    let mut fields: Vec<(String, String)> = Vec::new();
    if let Some(field_map) = obj.get("fields").and_then(|v| v.as_object()) {
        for (name, field_value) in field_map {
            collect_field_values(&mut fields, name.clone(), field_value)?;
        }
    }

    let mut files: Vec<MultipartFilePart> = Vec::new();
    if let Some(file_values) = obj.get("files").and_then(|v| v.as_array()) {
        for file in file_values {
            files.push(parse_multipart_file(file)?);
        }
    }

    let (body, boundary) = build_multipart_body(&fields, &files);
    let content_type = format!("multipart/form-data; boundary={}", boundary);
    Ok((body, content_type))
}

fn collect_field_values(
    fields: &mut Vec<(String, String)>,
    name: String,
    value: &Value,
) -> std::result::Result<(), String> {
    match value {
        Value::Array(items) => {
            for item in items {
                collect_field_values(fields, name.clone(), item)?;
            }
            Ok(())
        }
        _ => {
            let string_value = match value {
                Value::String(s) => s.clone(),
                Value::Null => String::new(),
                Value::Bool(b) => b.to_string(),
                Value::Number(n) => n.to_string(),
                other => serde_json::to_string(other).map_err(|e| e.to_string())?,
            };
            fields.push((name, string_value));
            Ok(())
        }
    }
}

fn parse_multipart_file(file: &Value) -> std::result::Result<MultipartFilePart, String> {
    let file_obj = file
        .as_object()
        .ok_or_else(|| "File entry must be an object".to_string())?;

    let field_name = file_obj
        .get("name")
        .or_else(|| file_obj.get("field_name"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| "File entry missing 'name'".to_string())?
        .to_string();

    let filename = file_obj
        .get("filename")
        .and_then(|v| v.as_str())
        .unwrap_or("file")
        .to_string();
    let content_type = file_obj
        .get("contentType")
        .or_else(|| file_obj.get("content_type"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let content = if let Some(content) = file_obj.get("content").and_then(|v| v.as_str()) {
        content.as_bytes().to_vec()
    } else if let Some(magic) = file_obj.get("magic_bytes").and_then(|v| v.as_str()) {
        decode_magic_bytes(magic)?
    } else {
        Vec::new()
    };

    Ok(MultipartFilePart {
        field_name,
        filename,
        content_type,
        content,
    })
}

fn map_snapshot_error(err: SnapshotError) -> Error {
    Error::from_reason(err.to_string())
}

fn decode_magic_bytes(hex_str: &str) -> std::result::Result<Vec<u8>, String> {
    if !hex_str.len().is_multiple_of(2) {
        return Err("magic_bytes must have an even length".to_string());
    }

    let mut bytes = Vec::with_capacity(hex_str.len() / 2);

    for chunk in hex_str.as_bytes().chunks(2) {
        let hex_pair = std::str::from_utf8(chunk).map_err(|e| e.to_string())?;
        let value = u8::from_str_radix(hex_pair, 16).map_err(|e| e.to_string())?;
        bytes.push(value);
    }

    Ok(bytes)
}

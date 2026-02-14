//! Test client for making HTTP requests to Spikard applications
//!
//! Uses the shared spikard_http router pipeline and bridges into JavaScript
//! handlers via napi ThreadsafeFunction.

#![allow(unused_imports)]

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
#[cfg(feature = "di")]
use crate::di::NO_DI_DEP_KEY;
use crate::test_sse;
use crate::test_websocket;
use spikard_http::ProblemDetails;
use spikard_http::testing::{
    MultipartFilePart, SnapshotError, build_multipart_body, encode_urlencoded_body, snapshot_response,
};
use spikard_http::{HandlerResult, RequestData, ResponseBodySize, Server};
use spikard_http::{Route, RouteMetadata, SchemaValidator};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

fn is_debug_mode() -> bool {
    std::env::var("DEBUG")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

fn normalize_route_path(path: &str) -> String {
    if !path.contains(':') {
        return path.to_string();
    }

    let mut normalized = String::new();
    for (idx, segment) in path.split('/').enumerate() {
        if idx > 0 {
            normalized.push('/');
        }

        if segment.starts_with('{') {
            normalized.push_str(segment);
            continue;
        }

        if let Some(stripped) = segment.strip_prefix(':') {
            if let Some(base) = stripped.strip_suffix(":path") {
                let base = if base.is_empty() { "path" } else { base };
                normalized.push_str(&format!("{{{}:path}}", base));
            } else {
                normalized.push_str(&format!("{{{}}}", stripped));
            }
        } else {
            normalized.push_str(segment);
        }
    }

    normalized
}

fn build_js_payload(handler: &JsHandler, request_data: &RequestData, validated_params: Option<Value>) -> Value {
    let path_params =
        validated_params.unwrap_or_else(|| serde_json::to_value(&*request_data.path_params).unwrap_or(Value::Null));
    let headers = serde_json::to_value(&*request_data.headers).unwrap_or(Value::Null);
    let cookies = serde_json::to_value(&*request_data.cookies).unwrap_or(Value::Null);
    #[cfg(feature = "di")]
    let dependencies = if let Some(resolved) = &request_data.dependencies {
        let mut deps_map: JsonMap<String, Value> = JsonMap::new();
        for key in resolved.keys() {
            if key == NO_DI_DEP_KEY {
                continue;
            }
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

    let body_value = if handler.prefers_raw_body {
        if let Some(raw_body) = request_data.raw_body.as_ref() {
            Value::Array(raw_body.iter().copied().map(|b| Value::Number(b.into())).collect())
        } else if !request_data.body.is_null() {
            match &*request_data.body {
                Value::String(text) => Value::Array(
                    text.as_bytes()
                        .iter()
                        .copied()
                        .map(|b| Value::Number(b.into()))
                        .collect(),
                ),
                other => serde_json::to_vec(other)
                    .map(|bytes| Value::Array(bytes.into_iter().map(|b| Value::Number(b.into())).collect()))
                    .unwrap_or(Value::Null),
            }
        } else {
            Value::Null
        }
    } else if !request_data.body.is_null() {
        (*request_data.body).clone()
    } else if let Some(raw_body) = request_data.raw_body.as_ref() {
        let content_type = request_data
            .headers
            .get("content-type")
            .map(|value| value.to_ascii_lowercase())
            .unwrap_or_default();

        if is_form_encoded(&content_type) || is_binary_body(&content_type) {
            Value::Array(raw_body.iter().copied().map(|b| Value::Number(b.into())).collect())
        } else {
            match serde_json::from_slice::<Value>(raw_body) {
                Ok(parsed) => parsed,
                Err(_) => Value::String(String::from_utf8_lossy(raw_body).to_string()),
            }
        }
    } else {
        Value::Null
    };
    let query: JsonMap<String, Value> = if request_data.raw_query_params.is_empty() {
        match &*request_data.query_params {
            Value::Object(map) => map.clone(),
            _ => JsonMap::new(),
        }
    } else {
        request_data
            .raw_query_params
            .iter()
            .filter_map(|(k, values)| values.first().map(|value| (k.clone(), Value::String(value.clone()))))
            .collect()
    };

    json!({
        "method": request_data.method,
        "path": request_data.path,
        "params": path_params,
        "query": Value::Object(query),
        "headers": headers,
        "cookies": cookies,
        "body": body_value,
        "dependencies": dependencies,
    })
}

#[derive(Clone)]
struct HandlerResponsePayload {
    status: u16,
    headers: HashMap<String, String>,
    body: Option<Value>,
    raw_body: Option<Vec<u8>>,
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

        let raw_body = match body.as_ref() {
            Some(Value::String(text)) => Some(text.as_bytes().to_vec()),
            _ => None,
        };

        HandlerResponsePayload {
            status,
            headers,
            body,
            raw_body,
        }
    } else {
        HandlerResponsePayload {
            status: 200,
            headers: HashMap::new(),
            body: Some(value),
            raw_body: None,
        }
    }
}

fn plain_text_response(text: String) -> HandlerResponsePayload {
    HandlerResponsePayload {
        status: 200,
        headers: HashMap::new(),
        body: Some(Value::String(text.clone())),
        raw_body: Some(text.into_bytes()),
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
    #[allow(dead_code)]
    method: String,
    #[allow(dead_code)]
    path: String,
    request_validator: Option<Arc<SchemaValidator>>,
    response_validator: Option<Arc<SchemaValidator>>,
    prefers_raw_body: bool,
}

impl JsHandler {
    fn new(js_fn: Function<String, Promise<HandlerReturnValue>>, route: &Route) -> Result<Self> {
        let tsfn = js_fn
            .build_threadsafe_function()
            .build_callback(|ctx| Ok(vec![ctx.value]))?;
        let prefers_raw_body = js_fn.get_named_property::<bool>("__spikard_raw_body").unwrap_or(true);

        Ok(Self {
            handler_fn: Arc::new(tsfn),
            handler_name: route.handler_name.clone(),
            method: route.method.as_str().to_string(),
            path: route.path.clone(),
            request_validator: route.request_validator.clone(),
            response_validator: route.response_validator.clone(),
            prefers_raw_body,
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

        let payload = build_js_payload(
            self,
            &request_data,
            request_data.validated_params.as_ref().map(|arc| (**arc).clone()),
        );

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

        let mut handler_response = match js_result {
            HandlerReturnValue::Json(text) => {
                let parsed = serde_json::from_str::<Value>(&text).ok();
                match parsed {
                    Some(Value::Object(obj))
                        if obj.contains_key("status") || obj.contains_key("statusCode") || obj.contains_key("body") =>
                    {
                        interpret_handler_response(Value::Object(obj))
                    }
                    _ => plain_text_response(text),
                }
            }
            HandlerReturnValue::Streaming(_) => unreachable!(),
        };
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
            let default_content_type = if handler_response.raw_body.is_some() {
                "text/plain; charset=utf-8"
            } else {
                "application/json"
            };
            response_builder = response_builder.header("content-type", HeaderValue::from_static(default_content_type));
        }

        let body_bytes = if let Some(raw_body) = handler_response.raw_body.take() {
            raw_body
        } else if let Some(body_value) = handler_response.body.take() {
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
// codeql[rust/access-invalid-pointer]
pub struct TestClient {
    server: Arc<TestServer>,
    #[allow(dead_code)]
    http_runtime: Option<std::sync::Arc<tokio::runtime::Runtime>>,
    websocket_paths: HashSet<String>,
    route_patterns: Vec<RoutePattern>,
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
        let mut routes_data: Vec<RouteMetadata> = serde_json::from_str(&routes_json)
            .map_err(|e| Error::from_reason(format!("Failed to parse routes: {}", e)))?;

        let mut websocket_routes_data: Vec<RouteMetadata> = websocket_routes_json
            .as_ref()
            .and_then(|json| serde_json::from_str(json).ok())
            .unwrap_or_default();

        for route in &mut routes_data {
            route.path = normalize_route_path(&route.path);
        }

        for route in &mut websocket_routes_data {
            route.path = normalize_route_path(&route.path);
        }

        let websocket_paths: HashSet<String> = websocket_routes_data.iter().map(|route| route.path.clone()).collect();
        let mut route_patterns: Vec<RoutePattern> = Vec::new();

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
                                .build_callback(|ctx| Ok(ctx.value))
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

        #[cfg(feature = "di")]
        if server_config.di_container.is_some() {
            for route in &mut routes_data {
                // Preserve any explicitly declared dependency list; only mark routes that
                // did not declare dependencies with the sentinel noop key.
                if route.handler_dependencies.is_none() {
                    route.handler_dependencies = Some(vec![NO_DI_DEP_KEY.to_string()]);
                }
            }
        }

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
            route_patterns.push(RoutePattern {
                method: route.method.as_str().to_string(),
                path: route.path.clone(),
                prefers_raw_body: js_handler.prefers_raw_body,
            });
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
        let transport = if websocket_paths.is_empty() {
            Transport::MockHttp
        } else {
            Transport::HttpRandomPort
        };
        let server = TestServer::builder()
            .transport(transport)
            .build(axum_router)
            .map_err(|e| Error::from_reason(format!("Failed to create test server: {}", e)))?;
        drop(_guard);

        Ok(Self {
            server: Arc::new(server),
            http_runtime: Some(runtime),
            websocket_paths,
            route_patterns,
        })
    }

    /// Make a GET request.
    #[napi]
    pub async fn get(&self, path: String, headers: Option<Value>) -> Result<TestResponse> {
        self.request("GET", path, headers, None).await
    }

    /// Make a POST request with an optional JSON body.
    #[napi]
    pub async fn post(&self, path: String, headers: Option<Value>, json: Option<Value>) -> Result<TestResponse> {
        self.request("POST", path, headers, json).await
    }

    /// Make a PUT request with an optional JSON body.
    #[napi]
    pub async fn put(&self, path: String, headers: Option<Value>, json: Option<Value>) -> Result<TestResponse> {
        self.request("PUT", path, headers, json).await
    }

    /// Make a DELETE request.
    #[napi]
    pub async fn delete(&self, path: String, headers: Option<Value>) -> Result<TestResponse> {
        self.request("DELETE", path, headers, None).await
    }

    /// Make a PATCH request with an optional JSON body.
    #[napi]
    pub async fn patch(&self, path: String, headers: Option<Value>, json: Option<Value>) -> Result<TestResponse> {
        self.request("PATCH", path, headers, json).await
    }

    /// Make a HEAD request.
    #[napi]
    pub async fn head(&self, path: String, headers: Option<Value>) -> Result<TestResponse> {
        self.request("HEAD", path, headers, None).await
    }

    /// Make an OPTIONS request.
    #[napi]
    pub async fn options(&self, path: String, headers: Option<Value>) -> Result<TestResponse> {
        self.request("OPTIONS", path, headers, None).await
    }

    /// Make a TRACE request.
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
        if !self.websocket_paths.contains(&path) {
            return Err(Error::from_reason(format!("WebSocket route not found: {}", path)));
        }
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
        let path_only = strip_query(&path);
        let matched_pattern = self
            .route_patterns
            .iter()
            .find(|pattern| pattern.matches(method, path_only));
        let route_matches = matched_pattern.is_some();
        let prefers_raw_body = matched_pattern.map(|pattern| pattern.prefers_raw_body).unwrap_or(true);
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

        if let Some(body_value) = body {
            if body_value.is_null() {
                // Explicit null matches JS client behavior: omit the request body.
            } else {
                match determine_body_payload(&body_value, prefers_raw_body) {
                    BodyPayload::Json(json_data) => {
                        match json_data {
                            Value::String(raw_body) => {
                                request = request.bytes(Bytes::from(raw_body.into_bytes()));
                            }
                            Value::Null => {}
                            other => {
                                let body_vec = serde_json::to_vec(&other)
                                    .map_err(|e| Error::from_reason(format!("Failed to serialize JSON body: {}", e)))?;
                                request = request.bytes(Bytes::from(body_vec));
                            }
                        }

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
                        let (body_bytes, content_type) =
                            encode_multipart_body(&multipart).map_err(Error::from_reason)?;

                        request = request.bytes(Bytes::from(body_bytes));
                        request = request.content_type(&content_type);
                    }
                }
            }
        }

        let response = request.await;
        if !route_matches && response.status_code() == StatusCode::NOT_FOUND {
            return Err(Error::from_reason(format!("No route matched {} {}", method, path_only)));
        }
        let snapshot = snapshot_response(response).await.map_err(map_snapshot_error)?;
        Ok(TestResponse::from_snapshot(snapshot))
    }
}

#[derive(Clone)]
struct RoutePattern {
    method: String,
    path: String,
    prefers_raw_body: bool,
}

impl RoutePattern {
    fn matches(&self, method: &str, path: &str) -> bool {
        if !self.method.eq_ignore_ascii_case(method) {
            return false;
        }
        path_matches(&self.path, path)
    }
}

fn strip_query(path: &str) -> &str {
    path.split('?').next().unwrap_or(path)
}

fn path_matches(pattern: &str, path: &str) -> bool {
    let pattern_segments = split_path_segments(pattern);
    let path_segments = split_path_segments(path);

    let mut index = 0usize;
    while index < pattern_segments.len() {
        let pattern_segment = pattern_segments[index];
        if let Some(is_wildcard) = is_param_segment(pattern_segment) {
            if is_wildcard {
                return true;
            }
            if path_segments.get(index).is_none() {
                return false;
            }
        } else if path_segments.get(index) != Some(&pattern_segment) {
            return false;
        }
        index += 1;
    }

    path_segments.len() == pattern_segments.len()
}

fn split_path_segments(path: &str) -> Vec<&str> {
    let trimmed = path.trim_start_matches('/');
    if trimmed.is_empty() {
        Vec::new()
    } else {
        trimmed.split('/').collect()
    }
}

fn is_param_segment(segment: &str) -> Option<bool> {
    if !segment.starts_with('{') || !segment.ends_with('}') {
        return None;
    }
    let inner = &segment[1..segment.len() - 1];
    if let Some((_name, kind)) = inner.split_once(':') {
        return Some(kind == "path");
    }
    Some(false)
}

fn is_form_encoded(content_type: &str) -> bool {
    let value = content_type.trim();
    value.starts_with("application/x-www-form-urlencoded") || value.starts_with("multipart/form-data")
}

fn is_binary_body(content_type: &str) -> bool {
    let value = content_type.trim();
    value.starts_with("application/octet-stream")
}

enum BodyPayload {
    Json(Value),
    Form(Value),
    Multipart(Value),
}

fn determine_body_payload(value: &Value, prefers_raw_body: bool) -> BodyPayload {
    if value.is_null() {
        return BodyPayload::Json(Value::Null);
    }

    if !value.is_object() {
        return BodyPayload::Json(value.clone());
    }

    if let Some(obj) = value.as_object() {
        if let Some(form) = obj.get("__spikard_form__") {
            if prefers_raw_body {
                return BodyPayload::Json(value.clone());
            }
            return BodyPayload::Form(form.clone());
        }

        if let Some(multipart) = obj.get("__spikard_multipart__") {
            if prefers_raw_body {
                return BodyPayload::Json(value.clone());
            }
            return BodyPayload::Multipart(multipart.clone());
        }
    }

    BodyPayload::Json(value.clone())
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
    if let Value::Array(items) = value {
        for item in items {
            collect_field_values(fields, name.clone(), item)?;
        }
    } else {
        let string_value = match value {
            Value::String(s) => s.clone(),
            Value::Null => String::new(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            other => serde_json::to_string(other).map_err(|e| e.to_string())?,
        };
        fields.push((name, string_value));
    }
    Ok(())
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

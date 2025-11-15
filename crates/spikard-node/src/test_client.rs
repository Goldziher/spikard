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
use axum_test::TestServer;
use bytes::Bytes;
use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunction;
use napi_derive::napi;
use serde_json::{Map as JsonMap, Value, json};
// TODO: Update to use current handler trait API
// use spikard_http::handler::{ForeignHandler, HandlerFuture, HandlerResult, RequestData};
use crate::test_sse;
use crate::test_websocket;
use spikard_http::problem::ProblemDetails;
use spikard_http::testing::{SnapshotError, snapshot_response};
use spikard_http::{HandlerResult, RequestData, ResponseBodySize, Server};
use spikard_http::{ParameterValidator, Route, RouteMetadata, SchemaValidator};
use std::collections::HashMap;
use std::sync::Arc;

fn is_debug_mode() -> bool {
    std::env::var("DEBUG")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

fn default_params(request_data: &RequestData) -> Value {
    let mut params = JsonMap::new();

    for (key, value) in &*request_data.path_params {
        params.insert(key.clone(), Value::String(value.clone()));
    }

    if let Value::Object(query) = &request_data.query_params {
        for (key, value) in query {
            params.insert(key.clone(), value.clone());
        }
    }

    for (key, value) in &*request_data.headers {
        params.insert(key.clone(), Value::String(value.clone()));
    }

    for (key, value) in &*request_data.cookies {
        params.insert(key.clone(), Value::String(value.clone()));
    }

    Value::Object(params)
}

fn build_js_payload(handler: &JsHandler, request_data: &RequestData, validated_params: Option<Value>) -> Value {
    let params_value = validated_params.unwrap_or_else(|| default_params(request_data));

    let path_params = serde_json::to_value(&*request_data.path_params).unwrap_or(Value::Null);
    let raw_query = serde_json::to_value(&*request_data.raw_query_params).unwrap_or(Value::Null);
    let headers = serde_json::to_value(&*request_data.headers).unwrap_or(Value::Null);
    let cookies = serde_json::to_value(&*request_data.cookies).unwrap_or(Value::Null);
    let body = request_data.body.clone();

    json!({
        "method": handler.method,
        "path": handler.path,
        "pathParams": path_params,
        "query": request_data.query_params,
        "rawQuery": raw_query,
        "headers": headers,
        "cookies": cookies,
        "params": params_value,
        "body": body
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
    parameter_validator: Option<ParameterValidator>,
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
            parameter_validator: route.parameter_validator.clone(),
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

        let validated_params = if let Some(validator) = &self.parameter_validator {
            // Convert raw_query_params from Vec<String> to String (take first value)
            let raw_query_strings: HashMap<String, String> = request_data
                .raw_query_params
                .iter()
                .filter_map(|(k, v)| v.first().map(|first| (k.clone(), first.clone())))
                .collect();

            match validator.validate_and_extract(
                &request_data.query_params,
                &raw_query_strings,
                &request_data.path_params,
                &request_data.headers,
                &request_data.cookies,
            ) {
                Ok(params) => Some(params),
                Err(errors) => {
                    let problem = ProblemDetails::from_validation_error(&errors);
                    let error_json = problem_to_json(&problem);
                    return Err((problem.status_code(), error_json));
                }
            }
        } else {
            None
        };

        let payload = build_js_payload(self, &request_data, validated_params.clone());

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
}

#[napi]
impl TestClient {
    /// Create a new test client from routes and handlers
    #[napi(constructor)]
    pub fn new(routes_json: String, handlers_map: Object, config: Option<Object>) -> Result<Self> {
        let routes_data: Vec<RouteMetadata> = serde_json::from_str(&routes_json)
            .map_err(|e| Error::from_reason(format!("Failed to parse routes: {}", e)))?;

        // Extract config or use test-friendly defaults (no compression)
        let server_config = if let Some(cfg) = config {
            crate::extract_server_config(&cfg)?
        } else {
            // Use defaults but disable compression for test clarity
            spikard_http::ServerConfig {
                compression: None,
                ..Default::default()
            }
        };

        let schema_registry = spikard_http::SchemaRegistry::new();
        let mut prepared_routes: Vec<(Route, Arc<dyn spikard_http::Handler>)> = Vec::new();
        let mut metadata_list: Vec<RouteMetadata> = Vec::new();

        for metadata in routes_data {
            // Clone metadata before converting to Route
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

        // Use Server::with_handlers_and_metadata to build router with full config support
        let axum_router =
            spikard_http::Server::with_handlers_and_metadata(server_config, prepared_routes, metadata_list)
                .map_err(|e| Error::from_reason(format!("Failed to build router: {}", e)))?;

        // Create test server with in-memory transport for HTTP tests
        let server = TestServer::new(axum_router)
            .map_err(|e| Error::from_reason(format!("Failed to create test server: {}", e)))?;

        Ok(Self {
            server: Arc::new(server),
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
        // Make GET request to SSE endpoint
        let axum_response = self.server.get(&path).await;
        let snapshot = snapshot_response(axum_response).await.map_err(map_snapshot_error)?;

        // Parse SSE stream from response
        test_sse::sse_stream_from_response(&snapshot)
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

        if let Some(body_value) = body {
            match determine_body_payload(&body_value) {
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
                    let body_vec = encode_form_body(form_data)
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
            }
        }

        let response = request.await;
        let snapshot = snapshot_response(response).await.map_err(map_snapshot_error)?;
        Ok(TestResponse::from_snapshot(snapshot))
    }
}

fn encode_form_body(body: Value) -> std::result::Result<Vec<u8>, String> {
    match body {
        Value::String(s) => Ok(s.into_bytes()),
        Value::Object(map) => {
            let mut parts = Vec::new();
            for (key, value) in map {
                append_form_value(&mut parts, key, value)?;
            }
            Ok(parts.join("&").into_bytes())
        }
        other => serde_qs::to_string(&other)
            .map(|encoded| encoded.into_bytes())
            .map_err(|e| e.to_string()),
    }
}

fn append_form_value(parts: &mut Vec<String>, key: String, value: Value) -> std::result::Result<(), String> {
    match value {
        Value::Array(items) => {
            for item in items {
                append_form_value(parts, key.clone(), item)?;
            }
            Ok(())
        }
        Value::Object(obj) => {
            for (nested_key, nested_value) in obj {
                let new_key = format!("{}[{}]", key, nested_key);
                append_form_value(parts, new_key, nested_value)?;
            }
            Ok(())
        }
        other => {
            let encoded_key = urlencoding::encode(&key).into_owned();
            let value_string = value_to_form_string(&other);
            let encoded_value = urlencoding::encode(&value_string).into_owned();
            parts.push(format!("{}={}", encoded_key, encoded_value));
            Ok(())
        }
    }
}

fn value_to_form_string(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

enum BodyPayload {
    Json(Value),
    Form(Value),
    Multipart(Value),
}

fn determine_body_payload(value: &Value) -> BodyPayload {
    if !value.is_object() {
        return BodyPayload::Json(value.clone());
    }

    if let Some(obj) = value.as_object() {
        if let Some(form) = obj.get("__spikard_form__") {
            return BodyPayload::Form(form.clone());
        }

        if let Some(multipart) = obj.get("__spikard_multipart__") {
            return BodyPayload::Multipart(multipart.clone());
        }
    }

    BodyPayload::Json(value.clone())
}

fn encode_multipart_body(value: &Value) -> std::result::Result<(Vec<u8>, String), String> {
    let obj = value
        .as_object()
        .ok_or_else(|| "Multipart payload must be an object".to_string())?;

    let fields = obj
        .get("fields")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();
    let files = obj.get("files").and_then(|v| v.as_array()).cloned().unwrap_or_default();

    let boundary = format!(
        "spikard-boundary-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_nanos()
    );

    let mut body = Vec::new();

    // Encode form fields
    for (name, value) in fields {
        append_field(&mut body, &boundary, &name, &value)?;
    }

    // Encode files
    for file in files {
        append_file(&mut body, &boundary, &file)?;
    }

    body.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());

    let content_type = format!("multipart/form-data; boundary={}", boundary);
    Ok((body, content_type))
}

fn append_field(body: &mut Vec<u8>, boundary: &str, name: &str, value: &Value) -> std::result::Result<(), String> {
    match value {
        Value::Array(values) => {
            for item in values {
                append_field(body, boundary, name, item)?;
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

            body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
            body.extend_from_slice(format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", name).as_bytes());
            body.extend_from_slice(string_value.as_bytes());
            body.extend_from_slice(b"\r\n");
            Ok(())
        }
    }
}

fn append_file(body: &mut Vec<u8>, boundary: &str, file: &Value) -> std::result::Result<(), String> {
    let file_obj = file
        .as_object()
        .ok_or_else(|| "File entry must be an object".to_string())?;

    let field_name = file_obj
        .get("name")
        .or_else(|| file_obj.get("field_name"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| "File entry missing 'name'".to_string())?;

    let filename = file_obj.get("filename").and_then(|v| v.as_str()).unwrap_or("file");
    let content_type = file_obj
        .get("contentType")
        .or_else(|| file_obj.get("content_type"))
        .and_then(|v| v.as_str())
        .unwrap_or("application/octet-stream");

    let content = if let Some(content) = file_obj.get("content").and_then(|v| v.as_str()) {
        content.as_bytes().to_vec()
    } else if let Some(magic) = file_obj.get("magic_bytes").and_then(|v| v.as_str()) {
        decode_magic_bytes(magic)?
    } else {
        Vec::new()
    };

    body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
    body.extend_from_slice(
        format!(
            "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
            field_name, filename
        )
        .as_bytes(),
    );
    body.extend_from_slice(format!("Content-Type: {}\r\n\r\n", content_type).as_bytes());
    body.extend_from_slice(&content);
    body.extend_from_slice(b"\r\n");

    Ok(())
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

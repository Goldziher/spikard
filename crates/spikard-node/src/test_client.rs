//! Test client for making HTTP requests to Spikard applications
//!
//! Implements JavaScript handler bridge using napi-rs ThreadsafeFunction
//! for async handler invocation, following patterns from kreuzberg.

use crate::response::TestResponse;
use axum::Router as AxumRouter;
use axum::body::Body;
use axum::extract::{Path, Request};
use axum::http::{HeaderMap, HeaderValue, Method, StatusCode, Uri};
use axum::routing::{any, delete, get, patch, post, put};
use axum_test::TestServer;
use cookie::Cookie;
use http_body_util::BodyExt;
use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunction;
use napi_derive::napi;
use serde_json::{Map as JsonMap, Value};
use spikard_http::handler::RequestData;
use spikard_http::problem::ProblemDetails;
use spikard_http::query_parser::parse_query_string_to_json;
use spikard_http::{Route, RouteMetadata};
use std::collections::HashMap;
use std::sync::Arc;

fn to_camel_case(s: &str) -> String {
    let mut parts = s
        .split(|c: char| c == '_' || c == '-' || c == ' ')
        .filter(|part| !part.is_empty());

    let mut result = String::new();
    if let Some(first) = parts.next() {
        result.push_str(&first.to_lowercase());
    }
    for part in parts {
        let mut chars = part.chars();
        if let Some(first_char) = chars.next() {
            result.push(first_char.to_ascii_uppercase());
            for c in chars {
                result.push(c);
            }
        }
    }

    if result.is_empty() { s.to_string() } else { result }
}

fn map_strings_to_json(input: &HashMap<String, String>) -> JsonMap<String, Value> {
    let mut map = JsonMap::new();
    for (key, value) in input {
        map.insert(to_camel_case(key), Value::String(value.clone()));
    }
    map
}

fn headers_to_json(headers: &HeaderMap) -> JsonMap<String, Value> {
    let mut map = JsonMap::new();
    for (name, value) in headers.iter() {
        if let Ok(val_str) = value.to_str() {
            map.insert(to_camel_case(name.as_str()), Value::String(val_str.to_string()));
        }
    }
    map
}

fn cookies_to_json(headers: &HeaderMap) -> JsonMap<String, Value> {
    let mut map = JsonMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in Cookie::split_parse(cookie_str).flatten() {
                map.insert(to_camel_case(cookie.name()), Value::String(cookie.value().to_string()));
            }
        }
    }
    map
}

fn headers_to_lowercase_map(headers: &HeaderMap) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for (name, value) in headers.iter() {
        if let Ok(val_str) = value.to_str() {
            map.insert(name.as_str().to_lowercase(), val_str.to_string());
        }
    }
    map
}

fn cookies_to_map(headers: &HeaderMap) -> HashMap<String, String> {
    let mut map = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in Cookie::split_parse(cookie_str).flatten() {
                map.insert(cookie.name().to_string(), cookie.value().to_string());
            }
        }
    }
    map
}

fn parse_urlencoded_to_json(data: &[u8]) -> std::result::Result<Value, String> {
    use std::collections::HashMap;

    let body_str = std::str::from_utf8(data).map_err(|e| e.to_string())?;

    if body_str.contains('[') {
        let config = serde_qs::Config::new(10, false);
        let parsed: HashMap<String, Value> = config.deserialize_str(body_str).map_err(|e| e.to_string())?;
        let mut json_value = serde_json::to_value(parsed).map_err(|e| e.to_string())?;
        convert_types_recursive(&mut json_value);
        Ok(json_value)
    } else {
        Ok(parse_urlencoded_simple(data))
    }
}

fn parse_urlencoded_simple(data: &[u8]) -> Value {
    use rustc_hash::FxHashMap;
    use urlencoding::decode;

    let mut array_map: FxHashMap<String, Vec<Value>> = FxHashMap::default();

    let body_str = String::from_utf8_lossy(data);
    let body_str = body_str.replace('+', " ");

    for pair in body_str.split('&') {
        if pair.is_empty() {
            continue;
        }

        let (key, value) = if let Some((k, v)) = pair.split_once('=') {
            (
                decode(k).unwrap_or_default().to_string(),
                decode(v).unwrap_or_default().to_string(),
            )
        } else {
            (pair.to_string(), String::new())
        };

        let json_value = convert_string_to_json_value(&value);

        match array_map.get_mut(&key) {
            Some(entry) => entry.push(json_value),
            None => {
                array_map.insert(key, vec![json_value]);
            }
        }
    }

    array_map
        .iter()
        .map(|(key, value)| {
            if value.len() == 1 {
                (key, value[0].clone())
            } else {
                (key, Value::Array(value.clone()))
            }
        })
        .collect::<Value>()
}

fn convert_string_to_json_value(s: &str) -> Value {
    if s.is_empty() {
        return Value::String(String::new());
    }

    if let Some(int_value) = try_parse_integer(s) {
        return int_value;
    }

    if let Some(float_value) = try_parse_float(s) {
        return float_value;
    }

    if let Some(bool_value) = try_parse_boolean(s) {
        return bool_value;
    }

    Value::String(s.to_string())
}

fn try_parse_integer(s: &str) -> Option<Value> {
    s.parse::<i64>().ok().map(|i| Value::Number(i.into()))
}

fn try_parse_float(s: &str) -> Option<Value> {
    s.parse::<f64>()
        .ok()
        .and_then(|f| serde_json::Number::from_f64(f).map(|num| Value::Number(num)))
}

fn try_parse_boolean(s: &str) -> Option<Value> {
    match s.to_ascii_lowercase().as_str() {
        "true" => Some(Value::Bool(true)),
        "false" => Some(Value::Bool(false)),
        _ => None,
    }
}

fn convert_types_recursive(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for (_, v) in map.iter_mut() {
                convert_types_recursive(v);
            }
        }
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                convert_types_recursive(item);
            }
        }
        Value::String(s) => {
            if let Some(int_value) = try_parse_integer(s) {
                *value = int_value;
            } else if let Some(float_value) = try_parse_float(s) {
                *value = float_value;
            } else if let Some(bool_value) = try_parse_boolean(s) {
                *value = bool_value;
            }
        }
        _ => {}
    }
}

fn value_to_camel_case_map(value: Value) -> JsonMap<String, Value> {
    match value {
        Value::Object(obj) => obj.into_iter().map(|(k, v)| (to_camel_case(&k), v)).collect(),
        _ => JsonMap::new(),
    }
}

fn merge_param_maps(maps: &[JsonMap<String, Value>]) -> JsonMap<String, Value> {
    let mut merged = JsonMap::new();
    for map in maps {
        for (key, value) in map {
            merged.insert(key.clone(), value.clone());
        }
    }
    merged
}

fn parse_query_maps(uri: &Uri) -> (Value, HashMap<String, String>) {
    let query_str = uri.query().unwrap_or("");
    if query_str.is_empty() {
        return (Value::Object(JsonMap::new()), HashMap::new());
    }

    let typed = parse_query_string_to_json(query_str.as_bytes(), true);

    let mut raw_map = HashMap::new();
    for (key, value) in url::form_urlencoded::parse(query_str.as_bytes()) {
        raw_map.entry(key.to_string()).or_insert(value.to_string());
    }

    (typed, raw_map)
}

fn build_request_payload(
    method: &str,
    path: &str,
    path_params: JsonMap<String, Value>,
    query_params: JsonMap<String, Value>,
    headers: JsonMap<String, Value>,
    cookies: JsonMap<String, Value>,
    params: JsonMap<String, Value>,
    raw_query: HashMap<String, String>,
    body: Option<Value>,
) -> Value {
    let mut payload = JsonMap::new();
    payload.insert("method".to_string(), Value::String(method.to_string()));
    payload.insert("path".to_string(), Value::String(path.to_string()));
    payload.insert("pathParams".to_string(), Value::Object(path_params));
    payload.insert("query".to_string(), Value::Object(query_params));
    payload.insert("headers".to_string(), Value::Object(headers));
    payload.insert("cookies".to_string(), Value::Object(cookies));
    payload.insert("params".to_string(), Value::Object(params));
    let raw_query_map: JsonMap<String, Value> = raw_query
        .into_iter()
        .map(|(k, v)| (to_camel_case(&k), Value::String(v)))
        .collect();
    payload.insert("rawQuery".to_string(), Value::Object(raw_query_map));
    match body {
        Some(value) => payload.insert("body".to_string(), value),
        None => payload.insert("body".to_string(), Value::Null),
    };
    Value::Object(payload)
}

fn to_axum_route_path(path: &str) -> String {
    spikard_http::type_hints::strip_type_hints(path)
}

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

fn convert_parameter_schema(schema: Value) -> Option<Value> {
    let container = schema.as_object()?;
    let mut properties = JsonMap::new();
    let mut required = Vec::new();

    for (section, entries_value) in container {
        let source = match section.as_str() {
            "path" => "path",
            "query" => "query",
            "headers" => "header",
            "cookies" => "cookie",
            _ => continue,
        };

        let entries = match entries_value.as_object() {
            Some(obj) => obj,
            None => continue,
        };

        for (name, schema_value) in entries {
            let mut map = match schema_value {
                Value::Object(obj) => obj.clone(),
                Value::String(s) => {
                    let mut m = JsonMap::new();
                    m.insert("type".to_string(), Value::String(s.clone()));
                    m
                }
                _ => {
                    let mut m = JsonMap::new();
                    m.insert("type".to_string(), schema_value.clone());
                    m
                }
            };

            map.entry("type".to_string())
                .or_insert_with(|| Value::String("string".to_string()));
            map.insert("source".to_string(), Value::String(source.to_string()));

            let is_optional = map.get("optional").and_then(|v| v.as_bool()).unwrap_or(false);
            if !is_optional {
                required.push(Value::String(name.clone()));
            }

            properties.insert(name.clone(), Value::Object(map));
        }
    }

    if properties.is_empty() {
        return None;
    }

    let mut result = JsonMap::new();
    result.insert("type".to_string(), Value::String("object".to_string()));
    result.insert("properties".to_string(), Value::Object(properties));
    if !required.is_empty() {
        result.insert("required".to_string(), Value::Array(required));
    }

    Some(Value::Object(result))
}

/// JavaScript handler wrapper that can be called from Rust async context
#[derive(Clone)]
struct JsHandler {
    /// Thread-safe reference to the JavaScript async function
    /// Takes JSON string, returns Promise<JSON string>
    /// Full signature: ThreadsafeFunction<InputType, OutputType, ReturnType, ErrorType, IsWeakRef>
    handler_fn: Arc<ThreadsafeFunction<String, Promise<String>, Vec<String>, napi::Status, false>>,
}

// SAFETY: ThreadsafeFunction from napi-rs is designed to be Send + Sync.
// - ThreadsafeFunction uses internal synchronization to safely call JavaScript from any thread
// - NAPI-RS guarantees thread-safe execution by marshaling through Node.js event loop
// - The JavaScript function reference is managed by Node.js runtime
// - Arc provides shared ownership with atomic reference counting
unsafe impl Send for JsHandler {}
unsafe impl Sync for JsHandler {}

impl JsHandler {
    /// Create a new handler from a JavaScript function
    fn new(js_fn: Function<String, Promise<String>>) -> Result<Self> {
        // Build ThreadsafeFunction with callback to wrap arguments
        let tsfn = js_fn.build_threadsafe_function().build_callback(|ctx| {
            // Wrap value in vec so JS receives it as separate argument
            Ok(vec![ctx.value])
        })?;

        Ok(Self {
            handler_fn: Arc::new(tsfn),
        })
    }

    /// Call the JavaScript handler with request parameters
    ///
    /// Uses double await pattern:
    /// - First await: enqueues callback on Node.js event loop
    /// - Second await: waits for JavaScript Promise to resolve
    async fn call(&self, payload: Value) -> Result<Value> {
        let request_json = serde_json::to_string(&payload)
            .map_err(|e| Error::from_reason(format!("Failed to serialize request payload: {}", e)))?;

        let result_json = self
            .handler_fn
            .call_async(request_json)
            .await
            .map_err(|e| Error::from_reason(format!("Handler call failed: {}", e)))?
            .await
            .map_err(|e| Error::from_reason(format!("Handler promise failed: {}", e)))?;

        serde_json::from_str(&result_json)
            .map_err(|e| Error::from_reason(format!("Failed to parse handler response: {}", e)))
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
    ///
    /// # Arguments
    /// * `routes_json` - JSON array of route metadata objects
    /// * `handlers_map` - JavaScript object mapping handler names to handler functions
    #[napi(constructor)]
    pub fn new(routes_json: String, handlers_map: Object) -> Result<Self> {
        // Parse routes
        let routes_data: Vec<RouteMetadata> = serde_json::from_str(&routes_json)
            .map_err(|e| Error::from_reason(format!("Failed to parse routes: {}", e)))?;

        // Extract handler functions from the JavaScript object
        let mut handlers: HashMap<String, JsHandler> = HashMap::new();

        for route_meta in &routes_data {
            let handler_name = &route_meta.handler_name;

            // Get the JavaScript function from the handlers map
            // Type it as Function<InputType, OutputType> to enable proper ThreadsafeFunction creation
            let js_fn: Function<String, Promise<String>> = handlers_map
                .get_named_property(handler_name)
                .map_err(|e| Error::from_reason(format!("Failed to get handler '{}': {}", handler_name, e)))?;

            // Create a JsHandler wrapper
            let js_handler = JsHandler::new(js_fn)?;
            handlers.insert(handler_name.clone(), js_handler);
        }

        // Convert to Route objects
        let routes: Vec<Route> = routes_data
            .into_iter()
            .map(|mut metadata| {
                metadata.parameter_schema = metadata.parameter_schema.take().and_then(convert_parameter_schema);
                Route::from_metadata(metadata).map_err(|e| Error::from_reason(format!("Failed to create route: {}", e)))
            })
            .collect::<Result<Vec<_>>>()?;

        // Build Axum router with JavaScript handlers
        let mut axum_router = AxumRouter::new();

        for route in routes {
            let handler = handlers
                .get(&route.handler_name)
                .ok_or_else(|| Error::from_reason(format!("Handler '{}' not found", route.handler_name)))?;

            let handler_clone = handler.clone();
            let path = route.path.clone();
            let has_path_params = path.contains('{');
            let axum_path = to_axum_route_path(&path);
            println!(
                "[spikard-node:test-client] register {} {} -> {}",
                route.method.as_str(),
                route.path,
                axum_path
            );

            // Create handler function that calls JavaScript handler
            let route_clone = route.clone();
            let route_handler = move |path_params: Option<Path<HashMap<String, String>>>,
                                      uri: Uri,
                                      headers: HeaderMap,
                                      request: Request| {
                let handler = handler_clone.clone();
                let route = route_clone.clone();
                async move {
                    let path_params_map = path_params.map(|p| p.0).unwrap_or_default();
                    let path_json = map_strings_to_json(&path_params_map);

                    let (query_value, raw_query_map) = parse_query_maps(&uri);
                    let query_json = value_to_camel_case_map(query_value.clone());

                    let headers_map = headers_to_lowercase_map(&headers);
                    let headers_json = headers_to_json(&headers);

                    let cookies_map = cookies_to_map(&headers);
                    let cookies_json = cookies_to_json(&headers);

                    let (_parts, body_stream) = request.into_parts();
                    let body_result = body_stream.collect().await;

                    println!(
                        "[spikard-node:test-client] handling {} {}",
                        route.method.as_str(),
                        route.path
                    );

                    let mut body_value: Option<Value> = None;
                    let content_type_header = headers_map.get("content-type").cloned();

                    match body_result {
                        Ok(bytes) => {
                            let body_bytes = bytes.to_bytes();
                            if !body_bytes.is_empty() {
                                let is_urlencoded = content_type_header
                                    .as_deref()
                                    .map(|ct| ct.starts_with("application/x-www-form-urlencoded"))
                                    .unwrap_or(false);

                                if is_urlencoded {
                                    match parse_urlencoded_to_json(body_bytes.as_ref()) {
                                        Ok(json) => body_value = Some(json),
                                        Err(err) => {
                                            let problem = ProblemDetails::bad_request("Failed to parse form data")
                                                .with_extension("error", Value::String(err));
                                            let json_bytes = serde_json::to_vec(&problem).unwrap_or_default();
                                            return axum::response::Response::builder()
                                                .status(problem.status_code())
                                                .header("content-type", "application/problem+json")
                                                .body(Body::from(json_bytes))
                                                .unwrap();
                                        }
                                    }
                                } else {
                                    match serde_json::from_slice::<Value>(&body_bytes) {
                                        Ok(json) => body_value = Some(json),
                                        Err(_) => {
                                            let text = String::from_utf8_lossy(&body_bytes).to_string();
                                            body_value = Some(Value::String(text));
                                        }
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            let problem = ProblemDetails::bad_request("Failed to read request body")
                                .with_extension("message", Value::String(err.to_string()));
                            let json_bytes = serde_json::to_vec(&problem).unwrap_or_default();
                            return axum::response::Response::builder()
                                .status(problem.status_code())
                                .header("content-type", "application/problem+json")
                                .body(Body::from(json_bytes))
                                .unwrap();
                        }
                    }

                    let request_data = RequestData {
                        path_params: path_params_map.clone(),
                        query_params: query_value.clone(),
                        raw_query_params: raw_query_map.clone(),
                        headers: headers_map.clone(),
                        cookies: cookies_map.clone(),
                        body: body_value.clone(),
                    };

                    if let Some(validator) = &route.request_validator {
                        let body_for_validation = request_data.body.clone().unwrap_or(Value::Null);
                        if let Err(errors) = validator.validate(&body_for_validation) {
                            let problem = ProblemDetails::from_validation_error(&errors);
                            let json_bytes = serde_json::to_vec(&problem).unwrap_or_default();
                            return axum::response::Response::builder()
                                .status(problem.status_code())
                                .header("content-type", "application/problem+json")
                                .body(Body::from(json_bytes))
                                .unwrap();
                        }
                    }

                    let validated_params_value = if let Some(param_validator) = &route.parameter_validator {
                        match param_validator.validate_and_extract(
                            &request_data.query_params,
                            &request_data.raw_query_params,
                            &request_data.path_params,
                            &request_data.headers,
                            &request_data.cookies,
                        ) {
                            Ok(params) => Some(params),
                            Err(errors) => {
                                let problem = ProblemDetails::from_validation_error(&errors);
                                let json_bytes = serde_json::to_vec(&problem).unwrap_or_default();
                                return axum::response::Response::builder()
                                    .status(problem.status_code())
                                    .header("content-type", "application/problem+json")
                                    .body(Body::from(json_bytes))
                                    .unwrap();
                            }
                        }
                    } else {
                        None
                    };

                    let params_combined = {
                        let maps = vec![
                            path_json.clone(),
                            query_json.clone(),
                            headers_json.clone(),
                            cookies_json.clone(),
                        ];
                        merge_param_maps(&maps)
                    };

                    let params_for_handler = if let Some(validated) = validated_params_value.clone() {
                        value_to_camel_case_map(validated)
                    } else {
                        params_combined.clone()
                    };

                    let payload = build_request_payload(
                        route.method.as_str(),
                        &route.path,
                        path_json,
                        query_json,
                        headers_json,
                        cookies_json,
                        params_for_handler,
                        raw_query_map,
                        body_value,
                    );

                    match handler.call(payload).await {
                        Ok(result) => {
                            let handler_response = interpret_handler_response(result);
                            let mut response_builder =
                                axum::response::Response::builder().status(handler_response.status);
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
                                response_builder = response_builder
                                    .header("content-type", HeaderValue::from_static("application/json"));
                            }

                            let body_bytes = handler_response
                                .body
                                .map(|value| serde_json::to_vec(&value).unwrap_or_default())
                                .unwrap_or_default();

                            response_builder.body(Body::from(body_bytes)).unwrap()
                        }
                        Err(e) => {
                            let error = serde_json::json!({
                                "error": "Handler failed",
                                "message": e.to_string()
                            });
                            let json_bytes = serde_json::to_vec(&error).unwrap_or_default();
                            axum::response::Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .header("content-type", "application/json")
                                .body(Body::from(json_bytes))
                                .unwrap()
                        }
                    }
                }
            };

            // Strip type hints from path for Axum compatibility
            // Register route based on HTTP method
            axum_router = match route.method.as_str() {
                "GET" => {
                    if has_path_params {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            get(
                                move |path: Path<HashMap<String, String>>,
                                      uri: Uri,
                                      headers: HeaderMap,
                                      req: Request| async move {
                                    handler(Some(path), uri, headers, req).await
                                },
                            ),
                        )
                    } else {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            get(move |uri: Uri, headers: HeaderMap, req: Request| async move {
                                handler(None, uri, headers, req).await
                            }),
                        )
                    }
                }
                "POST" => {
                    if has_path_params {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            post(
                                move |path: Path<HashMap<String, String>>,
                                      uri: Uri,
                                      headers: HeaderMap,
                                      req: Request| async move {
                                    handler(Some(path), uri, headers, req).await
                                },
                            ),
                        )
                    } else {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            post(move |uri: Uri, headers: HeaderMap, req: Request| async move {
                                handler(None, uri, headers, req).await
                            }),
                        )
                    }
                }
                "PUT" => {
                    if has_path_params {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            put(
                                move |path: Path<HashMap<String, String>>,
                                      uri: Uri,
                                      headers: HeaderMap,
                                      req: Request| async move {
                                    handler(Some(path), uri, headers, req).await
                                },
                            ),
                        )
                    } else {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            put(move |uri: Uri, headers: HeaderMap, req: Request| async move {
                                handler(None, uri, headers, req).await
                            }),
                        )
                    }
                }
                "DELETE" => {
                    if has_path_params {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            delete(
                                move |path: Path<HashMap<String, String>>,
                                      uri: Uri,
                                      headers: HeaderMap,
                                      req: Request| async move {
                                    handler(Some(path), uri, headers, req).await
                                },
                            ),
                        )
                    } else {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            delete(move |uri: Uri, headers: HeaderMap, req: Request| async move {
                                handler(None, uri, headers, req).await
                            }),
                        )
                    }
                }
                "PATCH" => {
                    if has_path_params {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            patch(
                                move |path: Path<HashMap<String, String>>,
                                      uri: Uri,
                                      headers: HeaderMap,
                                      req: Request| async move {
                                    handler(Some(path), uri, headers, req).await
                                },
                            ),
                        )
                    } else {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            patch(move |uri: Uri, headers: HeaderMap, req: Request| async move {
                                handler(None, uri, headers, req).await
                            }),
                        )
                    }
                }
                "HEAD" => {
                    if has_path_params {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            any(
                                move |path: Path<HashMap<String, String>>,
                                      uri: Uri,
                                      headers: HeaderMap,
                                      method: Method,
                                      req: Request| async move {
                                    if method == Method::HEAD {
                                        handler(Some(path), uri, headers, req).await
                                    } else {
                                        axum::response::Response::builder()
                                            .status(405)
                                            .body(Body::empty())
                                            .unwrap()
                                    }
                                },
                            ),
                        )
                    } else {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            any(
                                move |uri: Uri, headers: HeaderMap, method: Method, req: Request| async move {
                                    if method == Method::HEAD {
                                        handler(None, uri, headers, req).await
                                    } else {
                                        axum::response::Response::builder()
                                            .status(405)
                                            .body(Body::empty())
                                            .unwrap()
                                    }
                                },
                            ),
                        )
                    }
                }
                "OPTIONS" => {
                    if has_path_params {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            any(
                                move |path: Path<HashMap<String, String>>,
                                      uri: Uri,
                                      headers: HeaderMap,
                                      method: Method,
                                      req: Request| async move {
                                    if method == Method::OPTIONS {
                                        handler(Some(path), uri, headers, req).await
                                    } else {
                                        axum::response::Response::builder()
                                            .status(405)
                                            .body(Body::empty())
                                            .unwrap()
                                    }
                                },
                            ),
                        )
                    } else {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            any(
                                move |uri: Uri, headers: HeaderMap, method: Method, req: Request| async move {
                                    if method == Method::OPTIONS {
                                        handler(None, uri, headers, req).await
                                    } else {
                                        axum::response::Response::builder()
                                            .status(405)
                                            .body(Body::empty())
                                            .unwrap()
                                    }
                                },
                            ),
                        )
                    }
                }
                _ => {
                    return Err(Error::from_reason(format!(
                        "Unsupported HTTP method: {}",
                        route.method.as_str()
                    )));
                }
            };
        }

        // Create test server from the router
        let server = TestServer::new(axum_router.into_make_service())
            .map_err(|e| Error::from_reason(format!("Failed to create test server: {}", e)))?;

        Ok(Self {
            server: Arc::new(server),
        })
    }

    /// Make a GET request
    #[napi]
    pub async fn get(&self, path: String, headers: Option<serde_json::Value>) -> Result<TestResponse> {
        self.request("GET", path, headers, None).await
    }

    /// Make a POST request
    #[napi]
    pub async fn post(
        &self,
        path: String,
        headers: Option<serde_json::Value>,
        json: Option<serde_json::Value>,
    ) -> Result<TestResponse> {
        self.request("POST", path, headers, json).await
    }

    /// Make a PUT request
    #[napi]
    pub async fn put(
        &self,
        path: String,
        headers: Option<serde_json::Value>,
        json: Option<serde_json::Value>,
    ) -> Result<TestResponse> {
        self.request("PUT", path, headers, json).await
    }

    /// Make a DELETE request
    #[napi]
    pub async fn delete(&self, path: String, headers: Option<serde_json::Value>) -> Result<TestResponse> {
        self.request("DELETE", path, headers, None).await
    }

    /// Make a PATCH request
    #[napi]
    pub async fn patch(
        &self,
        path: String,
        headers: Option<serde_json::Value>,
        json: Option<serde_json::Value>,
    ) -> Result<TestResponse> {
        self.request("PATCH", path, headers, json).await
    }

    /// Make a HEAD request
    #[napi]
    pub async fn head(&self, path: String, headers: Option<serde_json::Value>) -> Result<TestResponse> {
        self.request("HEAD", path, headers, None).await
    }

    /// Make an OPTIONS request
    #[napi]
    pub async fn options(&self, path: String, headers: Option<serde_json::Value>) -> Result<TestResponse> {
        self.request("OPTIONS", path, headers, None).await
    }

    /// Generic request method using axum-test
    async fn request(
        &self,
        method: &str,
        path: String,
        headers: Option<serde_json::Value>,
        json: Option<serde_json::Value>,
    ) -> Result<TestResponse> {
        // Build request using axum-test
        let mut request = match method {
            "GET" => self.server.get(&path),
            "POST" => {
                let mut req = self.server.post(&path);
                if let Some(json_data) = json {
                    req = req.json(&json_data);
                }
                req
            }
            "PUT" => {
                let mut req = self.server.put(&path);
                if let Some(json_data) = json {
                    req = req.json(&json_data);
                }
                req
            }
            "DELETE" => self.server.delete(&path),
            "PATCH" => {
                let mut req = self.server.patch(&path);
                if let Some(json_data) = json {
                    req = req.json(&json_data);
                }
                req
            }
            "HEAD" | "OPTIONS" => {
                // Use method() for HEAD and OPTIONS
                self.server.method(
                    axum::http::Method::from_bytes(method.as_bytes())
                        .map_err(|e| Error::from_reason(format!("Invalid method: {}", e)))?,
                    &path,
                )
            }
            _ => return Err(Error::from_reason(format!("Unsupported method: {}", method))),
        };

        // Add headers if provided
        #[allow(clippy::collapsible_if)]
        if let Some(headers_val) = headers {
            if let Some(headers_obj) = headers_val.as_object() {
                for (name, value) in headers_obj {
                    if let Some(value_str) = value.as_str() {
                        request = request.add_header(
                            axum::http::HeaderName::from_bytes(name.as_bytes())
                                .map_err(|e| Error::from_reason(format!("Invalid header name: {}", e)))?,
                            axum::http::HeaderValue::from_str(value_str)
                                .map_err(|e| Error::from_reason(format!("Invalid header value: {}", e)))?,
                        );
                    }
                }
            }
        }

        // Execute request
        let response = request.await;

        // Extract response parts
        let status = response.status_code().as_u16();
        let headers_map = response.headers();

        // Convert headers to JSON map
        let mut headers_json = serde_json::Map::new();
        for (name, value) in headers_map.iter() {
            if let Ok(value_str) = value.to_str() {
                headers_json.insert(name.to_string(), Value::String(value_str.to_string()));
            }
        }

        // Get body bytes
        let body_bytes = response.into_bytes().to_vec();

        Ok(TestResponse::new(status, headers_json, body_bytes))
    }
}

//! WebAssembly bindings for Spikard.
//!
//! The goal of this crate is to provide a lightweight runtime that mirrors the
//! semantics of the existing Node bindings while remaining portable across
//! edge runtimes. The bindings expose a [`TestClient`] that receives the JSON
//! route metadata and handler map that the generated E2E apps produce.

mod matching;
mod types;

use crate::matching::{QueryParams, match_route};
use crate::types::{HandlerResponsePayload, RequestPayload, RouteDefinition, ServerConfig, build_params};
use brotli::CompressorWriter;
use flate2::Compression;
use flate2::write::GzEncoder;
use js_sys::{Date as JsDate, Function, Object, Promise};
use jsonschema::{Validator, validator_for};
use serde_json::{Map as JsonMap, Number as JsonNumber, Value};
use std::collections::{HashMap, HashSet};
use std::io::Write;
use time::format_description::well_known::Iso8601;
use time::macros::format_description;
use time::{Date, OffsetDateTime};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

/// Initialize the WASM module (sets panic hook).
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct TestClient {
    routes: Vec<RouteDefinition>,
    handlers: HashMap<String, Function>,
    config: ServerConfig,
    rate_state: std::rc::Rc<std::cell::RefCell<HashMap<String, RateLimiterState>>>,
}

#[derive(Default)]
struct RateLimiterState {
    tokens: f64,
    last_refill: f64,
    per_second: f64,
    burst: f64,
}

#[wasm_bindgen]
impl TestClient {
    /// Build a [`TestClient`] from serialized route metadata, handler map, and server config.
    #[wasm_bindgen(constructor)]
    pub fn new(routes_json: &str, handlers: JsValue, config: JsValue) -> Result<TestClient, JsValue> {
        let routes: Vec<RouteDefinition> = serde_json::from_str(routes_json)
            .map_err(|err| JsValue::from_str(&format!("Invalid routes JSON: {err}")))?;

        let config: ServerConfig = if config.is_undefined() || config.is_null() {
            ServerConfig::default()
        } else {
            let config_str = config
                .as_string()
                .ok_or_else(|| JsValue::from_str("Config must be a JSON string"))?;
            serde_json::from_str(&config_str).map_err(|err| JsValue::from_str(&err.to_string()))?
        };

        let handlers_object: Object = handlers
            .dyn_into()
            .map_err(|_| JsValue::from_str("Handlers must be an object map"))?;

        let handler_names = js_sys::Object::keys(&handlers_object);
        let mut handler_map = HashMap::new();
        for idx in 0..handler_names.length() {
            let name = handler_names.get(idx).as_string().unwrap_or_default();
            let value = js_sys::Reflect::get(&handlers_object, &JsValue::from(&name))
                .map_err(|_| JsValue::from_str("Failed to read handler function"))?;
            let func: Function = value
                .dyn_into()
                .map_err(|_| JsValue::from_str("Handler must be a function"))?;
            handler_map.insert(name, func);
        }

        Ok(TestClient {
            routes,
            handlers: handler_map,
            config,
            rate_state: std::rc::Rc::new(std::cell::RefCell::new(HashMap::new())),
        })
    }

    #[wasm_bindgen]
    pub fn get(&self, path: String, headers: JsValue) -> Promise {
        self.dispatch("GET", path, headers, JsValue::NULL)
    }

    #[wasm_bindgen]
    pub fn delete(&self, path: String, headers: JsValue) -> Promise {
        self.dispatch("DELETE", path, headers, JsValue::NULL)
    }

    #[wasm_bindgen]
    pub fn head(&self, path: String, headers: JsValue) -> Promise {
        self.dispatch("HEAD", path, headers, JsValue::NULL)
    }

    #[wasm_bindgen]
    pub fn options(&self, path: String, headers: JsValue) -> Promise {
        self.dispatch("OPTIONS", path, headers, JsValue::NULL)
    }

    #[wasm_bindgen]
    pub fn trace(&self, path: String, headers: JsValue) -> Promise {
        self.dispatch("TRACE", path, headers, JsValue::NULL)
    }

    #[wasm_bindgen]
    pub fn post(&self, path: String, options: JsValue) -> Promise {
        self.dispatch("POST", path, JsValue::NULL, options)
    }

    #[wasm_bindgen]
    pub fn put(&self, path: String, options: JsValue) -> Promise {
        self.dispatch("PUT", path, JsValue::NULL, options)
    }

    #[wasm_bindgen]
    pub fn patch(&self, path: String, options: JsValue) -> Promise {
        self.dispatch("PATCH", path, JsValue::NULL, options)
    }

    fn dispatch(&self, method: &str, path: String, headers: JsValue, options: JsValue) -> Promise {
        let context = RequestContext {
            method: method.to_string(),
            path,
            headers_val: headers,
            options,
            routes: self.routes.clone(),
            handlers: self.handlers.clone(),
            config: self.config.clone(),
            rate_state: self.rate_state.clone(),
        };

        future_to_promise(async move {
            let response = exec_request(context).await?;
            serde_wasm_bindgen::to_value(&response).map_err(|err| JsValue::from_str(&err.to_string()))
        })
    }
}

struct RequestContext {
    method: String,
    path: String,
    headers_val: JsValue,
    options: JsValue,
    routes: Vec<RouteDefinition>,
    handlers: HashMap<String, Function>,
    config: ServerConfig,
    rate_state: std::rc::Rc<std::cell::RefCell<HashMap<String, RateLimiterState>>>,
}

async fn exec_request(context: RequestContext) -> Result<ResponseSnapshot, JsValue> {
    let mut headers = read_headers(context.headers_val)?;
    let request_options = types::RequestOptions::from_js(context.options)?;
    if !request_options.headers.is_empty() {
        headers.extend(request_options.headers.clone());
    }

    if let Some(snapshot) = serve_static_from_manifest(&context.method, &context.path, &context.config) {
        return Ok(snapshot);
    }

    let (route, path_params, path_without_query, query) = match_route(&context.routes, &context.method, &context.path)?;

    let rate_limit_id = headers
        .get("x-forwarded-for")
        .cloned()
        .or_else(|| headers.get("x-real-ip").cloned());

    if let Some(snapshot) = enforce_rate_limit(&route.handler_name, &context.config, &context.rate_state, rate_limit_id)
    {
        return Ok(snapshot);
    }

    let mut params = build_params(&path_params, &query.normalized, &headers);
    if let Some(snapshot) = coerce_path_params(route.parameter_schema.as_ref(), &path_params, &mut params) {
        return Ok(snapshot);
    }
    if let Some(snapshot) = validate_parameters(&route, &params) {
        return Ok(snapshot);
    }

    let body_value = request_options.body_payload();
    if let Some(snapshot) = validate_request_body(route.request_schema.as_ref(), body_value.as_ref()) {
        return Ok(snapshot);
    }

    let request_payload = RequestPayload::new(
        context.method.clone(),
        &path_without_query,
        path_params,
        &headers,
        query.clone(),
        params,
        body_value.clone(),
    );

    let handler_fn = context
        .handlers
        .get(&route.handler_name)
        .ok_or_else(|| JsValue::from_str(&format!("Handler {} not registered", route.handler_name)))?;

    let request_json = serde_json::to_string(&request_payload)
        .map_err(|err| JsValue::from_str(&format!("Failed to serialize request: {err}")))?;

    let js_promise = handler_fn.call1(&JsValue::NULL, &JsValue::from_str(&request_json))?;

    let promise: Promise = js_promise
        .dyn_into()
        .map_err(|_| JsValue::from_str("Handler must return a Promise"))?;
    let result = wasm_bindgen_futures::JsFuture::from(promise).await?;

    // Handlers typically return a JSON string; fall back to object.
    let response_value = if let Some(text) = result.as_string() {
        serde_json::from_str::<Value>(&text)
            .map_err(|err| JsValue::from_str(&format!("Invalid handler response JSON: {err}")))?
    } else {
        serde_wasm_bindgen::from_value::<Value>(result)
            .map_err(|err| JsValue::from_str(&format!("Invalid handler response object: {err}")))?
    };

    if let Some(snapshot) = validate_response_body(route.response_schema.as_ref(), &response_value) {
        return Ok(snapshot);
    }

    let response_payload = HandlerResponsePayload::from_value(response_value)?;

    let snapshot = ResponseSnapshot::from_handler(response_payload, &headers, query, context.config);
    Ok(snapshot)
}

fn read_headers(value: JsValue) -> Result<HashMap<String, String>, JsValue> {
    if value.is_null() || value.is_undefined() {
        return Ok(HashMap::new());
    }
    let obj: Object = value
        .dyn_into()
        .map_err(|_| JsValue::from_str("Headers must be an object map"))?;
    let keys = js_sys::Object::keys(&obj);
    let mut headers = HashMap::new();
    for idx in 0..keys.length() {
        if let Some(key) = keys.get(idx).as_string() {
            let value = js_sys::Reflect::get(&obj, &JsValue::from(&key))
                .map_err(|_| JsValue::from_str("Failed to read header value"))?;
            if let Some(string_val) = value.as_string() {
                headers.insert(key, string_val);
            }
        }
    }
    Ok(headers)
}

fn enforce_rate_limit(
    handler_name: &str,
    config: &ServerConfig,
    rate_state: &std::rc::Rc<std::cell::RefCell<HashMap<String, RateLimiterState>>>,
    client_identifier: Option<String>,
) -> Option<ResponseSnapshot> {
    let limiter = config.rate_limit.as_ref()?;
    let mut state_map = rate_state.borrow_mut();
    let key = if limiter.ip_based {
        format!(
            "{}:{}",
            handler_name,
            client_identifier.unwrap_or_else(|| "global".to_string())
        )
    } else {
        handler_name.to_string()
    };
    let now = JsDate::now();
    let entry = state_map.entry(key).or_insert_with(|| RateLimiterState {
        tokens: limiter.burst as f64,
        last_refill: now,
        per_second: limiter.per_second as f64,
        burst: limiter.burst as f64,
    });
    if entry.per_second > 0.0 {
        let elapsed = ((now - entry.last_refill) / 1000.0).max(0.0);
        let replenished = elapsed * entry.per_second;
        entry.tokens = (entry.tokens + replenished).min(entry.burst);
        entry.last_refill = now;
    }
    if entry.tokens < 1.0 {
        return Some(ResponseSnapshot {
            status: 429,
            headers: HashMap::new(),
            body: Vec::new(),
        });
    }
    entry.tokens -= 1.0;
    None
}

fn validate_parameters(route: &RouteDefinition, params: &HashMap<String, Value>) -> Option<ResponseSnapshot> {
    let schema = route.parameter_schema.as_ref()?;
    let (filtered_schema, allowed_keys) = build_path_only_schema(schema)?;
    let validator = validator_for(&filtered_schema).ok()?;
    let params_value = Value::Object(
        params
            .iter()
            .filter(|(key, _)| allowed_keys.contains(*key))
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect(),
    );
    if let Some(messages) = collect_schema_errors(&validator, &params_value) {
        return Some(ResponseSnapshot::validation_failed(messages));
    }
    None
}

fn validate_request_body(schema: Option<&Value>, body: Option<&Value>) -> Option<ResponseSnapshot> {
    let schema = schema?;
    let validator = validator_for(schema).ok()?;
    let payload = body.unwrap_or(&Value::Null);
    let errors = collect_schema_errors(&validator, payload)?;
    Some(ResponseSnapshot::validation_error(
        422,
        "Request body validation failed",
        errors,
    ))
}

fn validate_response_body(schema: Option<&Value>, body: &Value) -> Option<ResponseSnapshot> {
    let schema = schema?;
    let validator = validator_for(schema).ok()?;
    let errors = collect_schema_errors(&validator, body)?;
    Some(ResponseSnapshot::validation_error(
        500,
        "Response validation failed",
        errors,
    ))
}

fn collect_schema_errors(validator: &Validator, value: &Value) -> Option<Vec<String>> {
    let mut errors = validator.iter_errors(value);
    let first = errors.next()?;
    let mut messages = vec![first.to_string()];
    for error in errors.take(2) {
        messages.push(error.to_string());
    }
    Some(messages)
}

fn serve_static_from_manifest(method: &str, path: &str, config: &ServerConfig) -> Option<ResponseSnapshot> {
    if !method.eq_ignore_ascii_case("GET") && !method.eq_ignore_ascii_case("HEAD") {
        return None;
    }
    if config.wasm_static_manifest.is_empty() {
        return None;
    }
    let normalized_path = path
        .split('?')
        .next()
        .filter(|segment| !segment.is_empty())
        .unwrap_or("/");
    let asset = config
        .wasm_static_manifest
        .iter()
        .find(|entry| entry.route == normalized_path)?;
    let mut headers = asset.headers.clone();
    headers
        .entry("content-length".to_string())
        .or_insert_with(|| asset.body.len().to_string());
    let body = if method.eq_ignore_ascii_case("HEAD") {
        Vec::new()
    } else {
        asset.body.clone()
    };
    Some(ResponseSnapshot {
        status: 200,
        headers,
        body,
    })
}

fn coerce_path_params(
    schema: Option<&Value>,
    path_values: &HashMap<String, String>,
    params: &mut HashMap<String, Value>,
) -> Option<ResponseSnapshot> {
    let schema = schema?;
    let properties = schema.get("properties")?.as_object()?;
    for (key, definition) in properties.iter() {
        let source = definition
            .get("source")
            .and_then(|value| value.as_str())
            .unwrap_or_default();
        if !source.eq_ignore_ascii_case("path") {
            continue;
        }
        let raw_value = match path_values.get(key) {
            Some(value) => value,
            None => continue,
        };
        if let Some(value_type) = definition.get("type").and_then(|value| value.as_str()) {
            let result = match value_type {
                "integer" => match raw_value.parse::<i64>() {
                    Ok(parsed) => {
                        params.insert(key.clone(), Value::Number(JsonNumber::from(parsed)));
                        Ok(())
                    }
                    Err(_) => Err(format!("Invalid integer for path parameter {key}")),
                },
                "number" => match raw_value.parse::<f64>().ok().and_then(JsonNumber::from_f64) {
                    Some(num) => {
                        params.insert(key.clone(), Value::Number(num));
                        Ok(())
                    }
                    None => Err(format!("Invalid number for path parameter {key}")),
                },
                "boolean" => {
                    let normalized = raw_value.to_ascii_lowercase();
                    match normalized.as_str() {
                        "true" | "1" => {
                            params.insert(key.clone(), Value::Bool(true));
                            Ok(())
                        }
                        "false" | "0" => {
                            params.insert(key.clone(), Value::Bool(false));
                            Ok(())
                        }
                        _ => Err(format!("Invalid boolean for path parameter {key}")),
                    }
                }
                "string" => {
                    if let Some(format) = definition.get("format").and_then(|value| value.as_str()) {
                        let valid = match format {
                            "date" => is_valid_date(raw_value),
                            "date-time" => is_valid_datetime(raw_value),
                            _ => true,
                        };
                        if !valid {
                            Err(format!("Invalid value for path parameter {key}"))
                        } else {
                            Ok(())
                        }
                    } else {
                        Ok(())
                    }
                }
                _ => Ok(()),
            };
            if let Err(message) = result {
                return Some(ResponseSnapshot::validation_failed(vec![message]));
            }
        }
    }
    None
}

fn build_path_only_schema(schema: &Value) -> Option<(Value, HashSet<String>)> {
    let properties = schema.get("properties")?.as_object()?;
    let mut filtered_props = JsonMap::new();
    let mut keys = HashSet::new();
    for (key, value) in properties.iter() {
        let source = value.get("source").and_then(|src| src.as_str()).unwrap_or_default();
        if source.eq_ignore_ascii_case("path") {
            filtered_props.insert(key.clone(), value.clone());
            keys.insert(key.clone());
        }
    }
    if filtered_props.is_empty() {
        return None;
    }
    let mut schema_map = JsonMap::new();
    if let Some(ty) = schema.get("type") {
        schema_map.insert("type".to_string(), ty.clone());
    }
    if let Some(additional) = schema.get("additionalProperties") {
        schema_map.insert("additionalProperties".to_string(), additional.clone());
    }
    schema_map.insert("properties".to_string(), Value::Object(filtered_props));
    if let Some(required) = schema.get("required").and_then(|value| value.as_array()) {
        let required_values: Vec<Value> = required
            .iter()
            .filter_map(|entry| entry.as_str())
            .filter(|name| keys.contains(*name))
            .map(|name| Value::String(name.to_string()))
            .collect();
        if !required_values.is_empty() {
            schema_map.insert("required".to_string(), Value::Array(required_values));
        }
    }
    Some((Value::Object(schema_map), keys))
}

fn is_valid_date(value: &str) -> bool {
    Date::parse(value, &format_description!("[year]-[month]-[day]")).is_ok()
}

fn is_valid_datetime(value: &str) -> bool {
    OffsetDateTime::parse(value, &Iso8601::DEFAULT).is_ok()
}

/// HTTP response snapshot returned to JavaScript callers.
#[derive(serde::Serialize)]
struct ResponseSnapshot {
    status: u16,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl ResponseSnapshot {
    fn from_handler(
        payload: HandlerResponsePayload,
        request_headers: &HashMap<String, String>,
        _query: QueryParams,
        config: ServerConfig,
    ) -> Self {
        let mut snapshot = ResponseSnapshot {
            status: payload.status,
            headers: payload.headers,
            body: payload.body_bytes,
        };
        snapshot.apply_response_filters(request_headers, &config);
        snapshot
    }

    fn validation_failed(errors: Vec<String>) -> Self {
        ResponseSnapshot::validation_error(422, "Parameter validation failed", errors)
    }

    fn validation_error(status: u16, detail: &str, errors: Vec<String>) -> Self {
        let payload = serde_json::json!({
            "detail": detail,
            "errors": errors,
        });
        ResponseSnapshot {
            status,
            headers: HashMap::new(),
            body: serde_json::to_vec(&payload).unwrap_or_default(),
        }
    }

    fn apply_response_filters(&mut self, request_headers: &HashMap<String, String>, config: &ServerConfig) {
        if let Some(compression) = &config.compression {
            self.apply_compression(request_headers, compression);
        }
    }

    fn apply_compression(
        &mut self,
        request_headers: &HashMap<String, String>,
        compression: &crate::types::CompressionConfig,
    ) {
        if self.body.is_empty() || self.status == 206 {
            return;
        }
        if self
            .headers
            .keys()
            .any(|key| key.eq_ignore_ascii_case("content-encoding"))
        {
            return;
        }
        if self.body.len() < compression.min_size {
            return;
        }
        let accept_encoding = header_value(request_headers, "Accept-Encoding").map(|value| value.to_ascii_lowercase());
        let accepts_brotli = accept_encoding
            .as_ref()
            .map(|value| value.contains("br"))
            .unwrap_or(false);
        if compression.brotli && accepts_brotli && self.try_compress_brotli(compression) {
            return;
        }
        let accepts_gzip = accept_encoding
            .as_ref()
            .map(|value| value.contains("gzip"))
            .unwrap_or(false);
        if compression.gzip && accepts_gzip {
            self.try_compress_gzip(compression);
        }
    }

    fn try_compress_brotli(&mut self, compression: &crate::types::CompressionConfig) -> bool {
        let quality = compression.quality.min(11) as u32;
        let mut writer = CompressorWriter::new(Vec::new(), 4096, quality, 22);
        if writer.write_all(&self.body).is_err() || writer.flush().is_err() {
            return false;
        }
        let compressed = writer.into_inner();
        if compressed.is_empty() {
            return false;
        }
        self.finalize_encoded_body("br", compressed);
        true
    }

    fn try_compress_gzip(&mut self, compression: &crate::types::CompressionConfig) -> bool {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::new(compression.quality as u32));
        if encoder.write_all(&self.body).is_err() {
            return false;
        }
        let compressed = encoder.finish().unwrap_or_else(|_| Vec::new());
        if compressed.is_empty() {
            return false;
        }
        self.finalize_encoded_body("gzip", compressed);
        true
    }

    fn finalize_encoded_body(&mut self, encoding: &str, compressed: Vec<u8>) {
        self.body = compressed;
        self.headers
            .insert("content-encoding".to_string(), encoding.to_string());
        self.headers.insert("vary".to_string(), "Accept-Encoding".to_string());
        self.headers
            .insert("content-length".to_string(), self.body.len().to_string());
    }
}

fn header_value<'a>(headers: &'a HashMap<String, String>, name: &str) -> Option<&'a str> {
    headers.iter().find_map(|(key, value)| {
        if key.eq_ignore_ascii_case(name) {
            Some(value.as_str())
        } else {
            None
        }
    })
}

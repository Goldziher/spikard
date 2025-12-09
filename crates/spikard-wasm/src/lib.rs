#![cfg(target_arch = "wasm32")]

//! WebAssembly bindings for Spikard.
//!
//! The goal of this crate is to provide a lightweight runtime that mirrors the
//! semantics of the existing Node bindings while remaining portable across
//! edge runtimes. The bindings expose a [`TestClient`] that receives the JSON
//! route metadata and handler map that the generated E2E apps produce.

mod lifecycle;
mod matching;
mod types;

use crate::lifecycle::{
    WasmLifecycleHooks, parse_hooks, request_from_payload, request_into_payload, response_from_value,
    response_into_value,
};
use crate::matching::match_route;
use crate::types::{BodyPayload, HandlerResponsePayload, RequestPayload, RouteDefinition, ServerConfig, build_params};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use http::StatusCode;
use js_sys::{Date as JsDate, Function, JSON, Object, Promise, Reflect, Symbol, Uint8Array};
use serde_json::{Map as JsonMap, Value};
use spikard_core::RouteMetadata;
use spikard_core::bindings::response::{RawResponse, StaticAsset};
use spikard_core::errors::StructuredError;
use spikard_core::lifecycle::HookResult;
use spikard_core::parameters::ParameterValidator;
use spikard_core::problem::ProblemDetails;
use spikard_core::router::Route as CompiledRoute;
use spikard_core::schema_registry::SchemaRegistry;
use spikard_core::validation::{SchemaValidator, ValidationError};
use std::collections::HashMap;
use std::sync::Arc;
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
    lifecycle_hooks: Option<Arc<WasmLifecycleHooks>>,
    route_validators: HashMap<String, RouteValidators>,
    rate_state: std::rc::Rc<std::cell::RefCell<HashMap<String, RateLimiterState>>>,
}

#[derive(Default)]
struct RateLimiterState {
    tokens: f64,
    last_refill: f64,
    per_second: f64,
    burst: f64,
}

#[derive(Clone)]
struct RouteValidators {
    parameter: Option<ParameterValidator>,
    request: Option<Arc<SchemaValidator>>,
    response: Option<Arc<SchemaValidator>>,
}

impl Default for RouteValidators {
    fn default() -> Self {
        Self {
            parameter: None,
            request: None,
            response: None,
        }
    }
}

#[wasm_bindgen]
impl TestClient {
    /// Build a [`TestClient`] from serialized route metadata, handler map, server config, and lifecycle hooks.
    #[wasm_bindgen(constructor)]
    pub fn new(
        routes_json: &str,
        handlers: JsValue,
        config: JsValue,
        lifecycle_hooks: Option<JsValue>,
    ) -> Result<TestClient, JsValue> {
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

        let schema_registry = SchemaRegistry::new();
        let mut validator_map = HashMap::new();
        for route in &routes {
            let metadata = route_metadata_from_definition(route);
            let compiled = CompiledRoute::from_metadata(metadata, &schema_registry)
                .map_err(|err| JsValue::from_str(&format!("Invalid route {}: {}", route.handler_name, err)))?;
            validator_map.insert(
                route.handler_name.clone(),
                RouteValidators {
                    parameter: compiled.parameter_validator.clone(),
                    request: compiled.request_validator.clone(),
                    response: compiled.response_validator.clone(),
                },
            );
        }

        let lifecycle_hooks_value = lifecycle_hooks.unwrap_or(JsValue::UNDEFINED);
        let lifecycle_hooks = parse_hooks(&lifecycle_hooks_value)?.map(Arc::new);

        Ok(TestClient {
            routes,
            handlers: handler_map,
            config,
            lifecycle_hooks,
            route_validators: validator_map,
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

    /// Handle a generic HTTP request. Takes a JSON string with method, path, headers, and body.
    #[wasm_bindgen]
    pub fn handle_request(&self, request_json: String) -> Promise {
        let context = match RequestContext::from_json(&request_json) {
            Ok(ctx) => {
                let mut ctx = ctx;
                ctx.routes = self.routes.clone();
                ctx.handlers = self.handlers.clone();
                ctx.config = self.config.clone();
                ctx.lifecycle_hooks = self.lifecycle_hooks.clone();
                ctx.route_validators = self.route_validators.clone();
                ctx.rate_state = self.rate_state.clone();
                ctx
            }
            Err(err) => {
                return future_to_promise(async move { Err(js_error_from_jsvalue("request_parse_failed", err)) });
            }
        };

        future_to_promise(async move {
            exec_request(context)
                .await
                .and_then(|response| serde_wasm_bindgen::to_value(&response).map_err(js_error_from_serde))
                .map_err(|err| js_error_from_jsvalue("execution_failed", err))
        })
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
            lifecycle_hooks: self.lifecycle_hooks.clone(),
            route_validators: self.route_validators.clone(),
            rate_state: self.rate_state.clone(),
        };

        future_to_promise(async move {
            exec_request(context)
                .await
                .and_then(|response| serde_wasm_bindgen::to_value(&response).map_err(js_error_from_serde))
                .map_err(|err| js_error_from_jsvalue("execution_failed", err))
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
    lifecycle_hooks: Option<Arc<WasmLifecycleHooks>>,
    route_validators: HashMap<String, RouteValidators>,
    rate_state: std::rc::Rc<std::cell::RefCell<HashMap<String, RateLimiterState>>>,
}

impl RequestContext {
    /// Parse a generic HTTP request from JSON string.
    /// Expects format: { method, path, headers?: {}, query?: {}, body?: any }
    fn from_json(request_json: &str) -> Result<Self, JsValue> {
        let value: serde_json::Value =
            serde_json::from_str(request_json).map_err(|err| JsValue::from_str(&err.to_string()))?;

        let method = value
            .get("method")
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsValue::from_str("Request must have a 'method' field"))?
            .to_string();

        let path = value
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsValue::from_str("Request must have a 'path' field"))?
            .to_string();

        let headers_val = if let Some(headers_obj) = value.get("headers").and_then(|v| v.as_object()) {
            let headers_js = Object::new();
            for (key, val) in headers_obj.iter() {
                let val_str = match val {
                    serde_json::Value::String(s) => s.clone(),
                    _ => val.to_string(),
                };
                Reflect::set(&headers_js, &JsValue::from_str(key), &JsValue::from_str(&val_str))
                    .map_err(|_| JsValue::from_str("Failed to set header"))?;
            }
            headers_js.into()
        } else {
            JsValue::NULL
        };

        let options_obj = Object::new();

        // Handle headers in options (for content-type detection)
        if let Some(headers_obj) = value.get("headers").and_then(|v| v.as_object()) {
            let headers_js = Object::new();
            for (key, val) in headers_obj.iter() {
                let val_str = match val {
                    serde_json::Value::String(s) => s.clone(),
                    _ => val.to_string(),
                };
                Reflect::set(&headers_js, &JsValue::from_str(key), &JsValue::from_str(&val_str))
                    .map_err(|_| JsValue::from_str("Failed to set option header"))?;
            }
            Reflect::set(&options_obj, &JsValue::from_str("headers"), &headers_js.into())
                .map_err(|_| JsValue::from_str("Failed to set headers in options"))?;
        }

        // Handle body - try to parse as JSON first, otherwise store as form_raw
        if let Some(body_val) = value.get("body") {
            match body_val {
                serde_json::Value::Null => {
                    // No body
                }
                serde_json::Value::String(s) => {
                    // Try to parse as JSON, fallback to form_raw
                    if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(s) {
                        serde_wasm_bindgen::to_value(&json_val)
                            .ok()
                            .and_then(|js_val| Reflect::set(&options_obj, &JsValue::from_str("json"), &js_val).ok());
                    } else {
                        // Store as form_raw if not valid JSON
                        Reflect::set(&options_obj, &JsValue::from_str("formRaw"), &JsValue::from_str(s))
                            .map_err(|_| JsValue::from_str("Failed to set formRaw"))?;
                    }
                }
                _ => {
                    // Try to convert to JS value
                    serde_wasm_bindgen::to_value(body_val)
                        .ok()
                        .and_then(|js_val| Reflect::set(&options_obj, &JsValue::from_str("json"), &js_val).ok());
                }
            }
        }

        Ok(RequestContext {
            method,
            path,
            headers_val,
            options: options_obj.into(),
            routes: Vec::new(),
            handlers: HashMap::new(),
            config: ServerConfig::default(),
            lifecycle_hooks: None,
            route_validators: HashMap::new(),
            rate_state: std::rc::Rc::new(std::cell::RefCell::new(HashMap::new())),
        })
    }
}

enum LifecycleRequestOutcome {
    Continue(RequestPayload),
    Respond(Value),
}

struct LifecycleRunner {
    hooks: Option<Arc<WasmLifecycleHooks>>,
}

impl LifecycleRunner {
    fn new(hooks: Option<Arc<WasmLifecycleHooks>>) -> Self {
        Self { hooks }
    }

    async fn run_before_handler(&self, payload: RequestPayload) -> Result<LifecycleRequestOutcome, JsValue> {
        let Some(hooks) = &self.hooks else {
            return Ok(LifecycleRequestOutcome::Continue(payload));
        };

        let mut request = request_from_payload(payload).map_err(|err| JsValue::from_str(&err))?;

        request = match hooks
            .execute_on_request(request)
            .await
            .map_err(|err| JsValue::from_str(&format!("onRequest hook failed: {err}")))?
        {
            HookResult::Continue(req) => req,
            HookResult::ShortCircuit(resp) => {
                let value = response_into_value(resp).map_err(|err| JsValue::from_str(&err))?;
                return Ok(LifecycleRequestOutcome::Respond(value));
            }
        };

        request = match hooks
            .execute_pre_validation(request)
            .await
            .map_err(|err| JsValue::from_str(&format!("preValidation hook failed: {err}")))?
        {
            HookResult::Continue(req) => req,
            HookResult::ShortCircuit(resp) => {
                let value = response_into_value(resp).map_err(|err| JsValue::from_str(&err))?;
                return Ok(LifecycleRequestOutcome::Respond(value));
            }
        };

        request = match hooks
            .execute_pre_handler(request)
            .await
            .map_err(|err| js_error_structured("pre_handler_hook_failed", err))?
        {
            HookResult::Continue(req) => req,
            HookResult::ShortCircuit(resp) => {
                let value =
                    response_into_value(resp).map_err(|err| js_error_structured("response_encode_failed", err))?;
                return Ok(LifecycleRequestOutcome::Respond(value));
            }
        };

        let payload = request_into_payload(request).map_err(|err| js_error_structured("request_encode_failed", err))?;
        Ok(LifecycleRequestOutcome::Continue(payload))
    }

    async fn run_response_hooks(&self, response: Value) -> Result<Value, JsValue> {
        let Some(hooks) = &self.hooks else {
            return Ok(response);
        };

        let mut response =
            response_from_value(response).map_err(|err| js_error_structured("response_decode_failed", err))?;
        response = hooks
            .execute_on_response(response)
            .await
            .map_err(|err| js_error_structured("on_response_hook_failed", err))?;
        response_into_value(response).map_err(|err| js_error_structured("response_encode_failed", err))
    }

    async fn run_error_hooks(&self, response: Value) -> Result<Value, JsValue> {
        let Some(hooks) = &self.hooks else {
            return Ok(response);
        };

        let mut response =
            response_from_value(response).map_err(|err| js_error_structured("response_decode_failed", err))?;
        response = hooks
            .execute_on_error(response)
            .await
            .map_err(|err| js_error_structured("on_error_hook_failed", err))?;
        response_into_value(response).map_err(|err| js_error_structured("response_encode_failed", err))
    }
}

async fn exec_request(context: RequestContext) -> Result<ResponseSnapshot, JsValue> {
    let mut headers = read_headers(context.headers_val).map_err(|e| js_error_from_jsvalue("header_parse_failed", e))?;
    let request_options = types::RequestOptions::from_js(context.options)
        .map_err(|e| js_error_from_jsvalue("request_options_invalid", e))?;
    if !request_options.headers.is_empty() {
        headers.extend(request_options.headers.clone());
    }

    if let Some(snapshot) = serve_static_from_manifest(&context.method, &context.path, &headers, &context.config) {
        return Ok(snapshot);
    }

    let (route, path_params, path_without_query, query) = match_route(&context.routes, &context.method, &context.path)
        .map_err(|e| js_error_from_jsvalue("route_match_failed", e))?;

    let rate_limit_id = headers
        .get("x-forwarded-for")
        .cloned()
        .or_else(|| headers.get("x-real-ip").cloned());

    if let Some(snapshot) = enforce_rate_limit(&route.handler_name, &context.config, &context.rate_state, rate_limit_id)
    {
        return Ok(snapshot);
    }

    let validators = context
        .route_validators
        .get(&route.handler_name)
        .cloned()
        .unwrap_or_default();

    let mut params = build_params(&path_params, &query.normalized, &headers);
    if let Some(parameter_validator) = validators.parameter.clone() {
        let mut query_map = JsonMap::new();
        for (key, value) in &query.normalized {
            query_map.insert(key.clone(), value.clone());
        }
        let query_value = Value::Object(query_map);
        match parameter_validator.validate_and_extract(
            &query_value,
            &query.raw,
            &path_params,
            &headers,
            &HashMap::new(),
        ) {
            Ok(value) => {
                params = value_to_param_map(value);
            }
            Err(error) => {
                let problem = ProblemDetails::from_validation_error(&error);
                return Ok(ResponseSnapshot::from_problem(problem, &headers, &context.config));
            }
        }
    }

    let body_payload = request_options.body_payload();
    if let Some(request_validator) = validators.request.as_ref() {
        if let Err(error) = request_validator.validate(body_payload.value.as_ref().unwrap_or(&Value::Null)) {
            let problem = ProblemDetails::from_validation_error(&error);
            return Ok(ResponseSnapshot::from_problem(problem, &headers, &context.config));
        }
    }

    let BodyPayload {
        value: body_value,
        metadata: body_metadata,
    } = body_payload;

    let request_payload = RequestPayload::new(
        context.method.clone(),
        &path_without_query,
        path_params,
        &headers,
        query.clone(),
        params,
        body_value.clone(),
        body_metadata,
    );

    let lifecycle_runner = LifecycleRunner::new(context.lifecycle_hooks.clone());

    let request_payload = match lifecycle_runner.run_before_handler(request_payload).await? {
        LifecycleRequestOutcome::Continue(payload) => payload,
        LifecycleRequestOutcome::Respond(response_value) => {
            let finalized = lifecycle_runner.run_response_hooks(response_value).await?;
            let response_payload = HandlerResponsePayload::from_value(finalized)?;
            return Ok(ResponseSnapshot::from_handler(
                response_payload,
                &headers,
                &context.config,
            ));
        }
    };

    let handler_fn = context
        .handlers
        .get(&route.handler_name)
        .ok_or_else(|| JsValue::from_str(&format!("Handler {} not registered", route.handler_name)))?;

    let request_json = serde_json::to_string(&request_payload)
        .map_err(|err| JsValue::from_str(&format!("Failed to serialize request: {err}")))?;
    let request_value = serde_wasm_bindgen::to_value(&request_payload)
        .map_err(|err| JsValue::from_str(&format!("Failed to build request object: {err}")))?;
    let request_object = js_sys::Object::from(request_value);
    Reflect::set(
        &request_object,
        &JsValue::from_str("__spikard_raw_request__"),
        &JsValue::from_str(&request_json),
    )
    .map_err(|_| JsValue::from_str("Failed to attach raw request payload"))?;

    let js_promise = handler_fn.call1(&JsValue::NULL, &request_object.into())?;

    let promise: Promise = js_promise
        .dyn_into()
        .map_err(|_| JsValue::from_str("Handler must return a Promise"))?;
    let result = wasm_bindgen_futures::JsFuture::from(promise).await;

    let response_value = match result {
        Ok(value) => {
            let normalized = normalize_handler_result(value).await?;
            lifecycle_runner.run_response_hooks(normalized).await?
        }
        Err(err) => {
            let error_response = build_error_response(err);
            lifecycle_runner.run_error_hooks(error_response).await?
        }
    };

    if let Some(response_validator) = validators.response.as_ref() {
        if let Err(error) = response_validator.validate(&response_value) {
            let problem = response_validation_problem(&error);
            return Ok(ResponseSnapshot::from_problem(problem, &headers, &context.config));
        }
    }

    let response_payload = HandlerResponsePayload::from_value(response_value)?;

    let snapshot = ResponseSnapshot::from_handler(response_payload, &headers, &context.config);
    Ok(snapshot)
}

async fn normalize_handler_result(result: JsValue) -> Result<Value, JsValue> {
    if let Some(streaming) = try_streaming_response(&result).await? {
        return Ok(streaming);
    }
    parse_handler_value(result)
}

fn build_error_response(error: JsValue) -> Value {
    let message = extract_error_message(&error);
    let normalized = if message.is_empty() {
        "Internal Server Error".to_string()
    } else {
        message
    };
    serde_json::json!({
        "status": 500,
        "headers": {},
        "body": {
            "error": normalized,
        }
    })
}

fn extract_error_message(error: &JsValue) -> String {
    if let Some(text) = error.as_string() {
        return text;
    }
    if let Some(js_error) = error.dyn_ref::<js_sys::Error>() {
        return js_error.message().into();
    }
    JSON::stringify(error)
        .ok()
        .and_then(|val| val.as_string())
        .unwrap_or_else(|| "Internal Server Error".to_string())
}

fn parse_handler_value(result: JsValue) -> Result<Value, JsValue> {
    if let Some(text) = result.as_string() {
        match serde_json::from_str::<Value>(&text) {
            Ok(value) => Ok(value),
            Err(_) => Ok(Value::String(text)),
        }
    } else if result.is_null() || result.is_undefined() {
        Ok(Value::Null)
    } else {
        serde_wasm_bindgen::from_value::<Value>(result)
            .map_err(|err| JsValue::from_str(&format!("Invalid handler response object: {err}")))
    }
}

async fn try_streaming_response(value: &JsValue) -> Result<Option<Value>, JsValue> {
    if !value.is_object() {
        return Ok(None);
    }
    let symbol = Symbol::for_("spikard.streaming.handle");
    let has_symbol = Reflect::has(value, symbol.as_ref()).unwrap_or(false);
    let has_marker = Reflect::has(value, &JsValue::from_str("__spikard_streaming__")).unwrap_or(false);
    let is_streaming = has_symbol || has_marker;
    let collect = Reflect::get(value, &JsValue::from_str("collect")).unwrap_or(JsValue::UNDEFINED);
    if collect.is_undefined() || collect.is_null() {
        if is_streaming {
            return Err(JsValue::from_str("StreamingResponse missing collect method"));
        }
        return Ok(None);
    }
    let collect_fn: Function = match collect.dyn_into() {
        Ok(func) => func,
        Err(_) => {
            if is_streaming {
                return Err(JsValue::from_str("StreamingResponse collect must be a function"));
            }
            return Ok(None);
        }
    };
    let promise_value = collect_fn
        .call0(value)
        .map_err(|err| JsValue::from_str(&format!("Failed to collect stream: {err:?}")))?;
    let promise: Promise = match promise_value.dyn_into() {
        Ok(p) => p,
        Err(_) => {
            if is_streaming {
                return Err(JsValue::from_str("StreamingResponse collect must return a Promise"));
            }
            return Ok(None);
        }
    };
    let bytes_value = wasm_bindgen_futures::JsFuture::from(promise).await?;
    let buffer = Uint8Array::new(&bytes_value);
    let body_bytes = buffer.to_vec();
    if body_bytes.is_empty() {
        return Err(JsValue::from_str("StreamingResponse produced empty body"));
    }
    let headers = read_handler_headers(value)?;
    let status = read_status_code(value);
    let body_value = encode_stream_body(&body_bytes, header_value(&headers, "content-type"));

    let mut response = JsonMap::new();
    response.insert("statusCode".to_string(), serde_json::json!(status));
    if !headers.is_empty() {
        response.insert(
            "headers".to_string(),
            Value::Object(
                headers
                    .iter()
                    .map(|(key, header_value)| (key.clone(), Value::String(header_value.clone())))
                    .collect(),
            ),
        );
    }
    if let Some(body) = body_value {
        response.insert("body".to_string(), body);
    }
    Ok(Some(Value::Object(response)))
}

fn read_status_code(value: &JsValue) -> u16 {
    Reflect::get(value, &JsValue::from_str("statusCode"))
        .ok()
        .and_then(|val| val.as_f64())
        .map(|num| num as u16)
        .unwrap_or(200)
}

fn read_handler_headers(value: &JsValue) -> Result<HashMap<String, String>, JsValue> {
    let headers_value = Reflect::get(value, &JsValue::from_str("headers")).unwrap_or(JsValue::UNDEFINED);
    if headers_value.is_null() || headers_value.is_undefined() {
        return Ok(HashMap::new());
    }
    serde_wasm_bindgen::from_value::<HashMap<String, String>>(headers_value)
        .map_err(|err| JsValue::from_str(&format!("Invalid response headers: {err}")))
}

fn encode_stream_body(body: &[u8], content_type: Option<&str>) -> Option<Value> {
    if body.is_empty() {
        return None;
    }
    if should_encode_as_text(content_type) {
        match String::from_utf8(body.to_vec()) {
            Ok(text) => Some(Value::String(text)),
            Err(_) => Some(serde_json::json!({ "__spikard_base64__": BASE64_STANDARD.encode(body) })),
        }
    } else {
        Some(serde_json::json!({ "__spikard_base64__": BASE64_STANDARD.encode(body) }))
    }
}

fn should_encode_as_text(content_type: Option<&str>) -> bool {
    let value = match content_type {
        Some(text) => text.to_ascii_lowercase(),
        None => return false,
    };
    value.starts_with("text/") || value.contains("json") || value.contains("javascript")
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

fn serve_static_from_manifest(
    method: &str,
    path: &str,
    request_headers: &HashMap<String, String>,
    config: &ServerConfig,
) -> Option<ResponseSnapshot> {
    if config.wasm_static_manifest.is_empty() {
        return None;
    }
    let normalized_path = path
        .split('?')
        .next()
        .filter(|segment| !segment.is_empty())
        .unwrap_or("/");
    let entry = config
        .wasm_static_manifest
        .iter()
        .find(|asset| asset.route == normalized_path)?;

    let asset = StaticAsset {
        route: entry.route.clone(),
        headers: entry.headers.clone(),
        body: entry.body.clone(),
    };

    let raw = asset.serve(method, normalized_path)?;
    Some(ResponseSnapshot::from_raw(raw, request_headers, config))
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
        config: &ServerConfig,
    ) -> Self {
        let raw = RawResponse::new(payload.status, payload.headers, payload.body_bytes);
        ResponseSnapshot::from_raw(raw, request_headers, config)
    }

    fn from_problem(problem: ProblemDetails, request_headers: &HashMap<String, String>, config: &ServerConfig) -> Self {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/problem+json".to_string());
        let raw = RawResponse::new(
            problem.status,
            headers,
            serde_json::to_vec(&problem).unwrap_or_default(),
        );
        ResponseSnapshot::from_raw(raw, request_headers, config)
    }

    fn from_raw(mut raw: RawResponse, request_headers: &HashMap<String, String>, config: &ServerConfig) -> Self {
        if let Some(compression) = &config.compression {
            let http_config = spikard_core::CompressionConfig {
                gzip: compression.gzip,
                brotli: compression.brotli,
                min_size: compression.min_size,
                quality: compression.quality as u32,
            };
            raw.apply_compression(request_headers, &http_config);
        }

        ResponseSnapshot {
            status: raw.status,
            headers: raw.headers,
            body: raw.body,
        }
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

fn value_to_param_map(value: Value) -> HashMap<String, Value> {
    match value {
        Value::Object(map) => map.into_iter().collect(),
        _ => HashMap::new(),
    }
}

fn js_error_structured(code: &str, details: impl ToString) -> JsValue {
    let payload = StructuredError::new(code.to_string(), code.to_string(), Value::String(details.to_string()));
    match serde_wasm_bindgen::to_value(&payload) {
        Ok(js) => js,
        Err(_) => JsValue::from_str(&format!(r#"{{"error":"{}","code":"{}","details":{{}}}}"#, code, code)),
    }
}

fn js_error_from_jsvalue(code: &str, js_err: JsValue) -> JsValue {
    let details = if let Some(s) = js_err.as_string() {
        s
    } else {
        format!("{:?}", js_err)
    };
    js_error_structured(code, details)
}

fn js_error_from_serde(err: impl std::fmt::Display) -> JsValue {
    js_error_structured("serde_error", err.to_string())
}

fn route_metadata_from_definition(def: &RouteDefinition) -> RouteMetadata {
    RouteMetadata {
        method: def.method.clone(),
        path: def.path.clone(),
        handler_name: def.handler_name.clone(),
        request_schema: def.request_schema.clone(),
        response_schema: def.response_schema.clone(),
        parameter_schema: def.parameter_schema.clone(),
        file_params: None,
        is_async: true,
        cors: None,
        body_param_name: None,
        jsonrpc_method: def.jsonrpc_method.clone(),
    }
}

fn response_validation_problem(error: &ValidationError) -> ProblemDetails {
    let error_count = error.errors.len();
    let detail = if error_count == 1 {
        "1 validation error in response".to_string()
    } else {
        format!("{error_count} validation errors in response")
    };

    let errors_json = serde_json::to_value(&error.errors).unwrap_or_else(|_| Value::Array(vec![]));

    ProblemDetails::new(
        ProblemDetails::TYPE_INTERNAL_SERVER_ERROR,
        "Response Validation Failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
    .with_detail(detail)
    .with_extension("errors", errors_json)
}

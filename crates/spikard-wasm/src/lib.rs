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
use crate::types::{
    ApiKeyConfig, BodyKind, BodyPayload, HandlerResponsePayload, JwtConfig, RequestPayload, RouteDefinition,
    ServerConfig, build_params,
};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::engine::general_purpose::URL_SAFE_NO_PAD as BASE64_URL_SAFE_NO_PAD;
use hmac::{Hmac, Mac};
use http::StatusCode;
use js_sys::{Array, Date as JsDate, Function, JSON, Object, Promise, Reflect, Symbol, Uint8Array};
use serde_json::{Map as JsonMap, Value};
use sha2::Sha256;
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
use uuid::Uuid;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
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
    dependencies: HashMap<String, DependencyDescriptor>,
    dependency_singletons: std::rc::Rc<std::cell::RefCell<HashMap<String, String>>>,
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

#[derive(Clone)]
struct DependencyDescriptor {
    is_factory: bool,
    value_json: Option<String>,
    factory: Option<Function>,
    depends_on: Vec<String>,
    singleton: bool,
    cacheable: bool,
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
        dependencies: Option<JsValue>,
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

        let dependencies_map = parse_dependencies(dependencies.unwrap_or(JsValue::UNDEFINED))?;

        Ok(TestClient {
            routes,
            handlers: handler_map,
            config,
            lifecycle_hooks,
            route_validators: validator_map,
            rate_state: std::rc::Rc::new(std::cell::RefCell::new(HashMap::new())),
            dependencies: dependencies_map,
            dependency_singletons: std::rc::Rc::new(std::cell::RefCell::new(HashMap::new())),
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
            dependencies: self.dependencies.clone(),
            dependency_singletons: self.dependency_singletons.clone(),
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
    dependencies: HashMap<String, DependencyDescriptor>,
    dependency_singletons: std::rc::Rc<std::cell::RefCell<HashMap<String, String>>>,
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

        if let Some(body_val) = value.get("body") {
            match body_val {
                serde_json::Value::Null => {}
                serde_json::Value::String(s) => {
                    if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(s) {
                        serde_wasm_bindgen::to_value(&json_val)
                            .ok()
                            .and_then(|js_val| Reflect::set(&options_obj, &JsValue::from_str("json"), &js_val).ok());
                    } else {
                        Reflect::set(&options_obj, &JsValue::from_str("formRaw"), &JsValue::from_str(s))
                            .map_err(|_| JsValue::from_str("Failed to set formRaw"))?;
                    }
                }
                _ => {
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
            dependencies: HashMap::new(),
            dependency_singletons: std::rc::Rc::new(std::cell::RefCell::new(HashMap::new())),
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
        for (key, value) in &request_options.headers {
            headers.insert(key.clone(), value.clone());
        }
    }

    let lowered_headers = lowercase_header_keys(&headers);
    let cookies = parse_cookie_header(&headers);

    if let Some(snapshot) = serve_static_from_manifest(&context.method, &context.path, &headers, &context.config) {
        return Ok(snapshot);
    }

    let max_body_size = context.config.max_body_size.filter(|max| *max > 0);
    let request_timeout_seconds = context.config.request_timeout.filter(|seconds| *seconds > 0);

    let (route, path_params, path_without_query, query) = match_route(&context.routes, &context.method, &context.path)
        .map_err(|e| js_error_from_jsvalue("route_match_failed", e))?;

    if let Some(snapshot) = enforce_auth(&headers, &query.raw, &context.config) {
        return Ok(snapshot);
    }

    let rate_limit_id = header_value(&headers, "x-forwarded-for")
        .map(|value| value.to_string())
        .or_else(|| header_value(&headers, "x-real-ip").map(|value| value.to_string()));

    if let Some(snapshot) = enforce_rate_limit(&route.handler_name, &context.config, &context.rate_state, rate_limit_id)
    {
        return Ok(snapshot);
    }

    let validators = context
        .route_validators
        .get(&route.handler_name)
        .cloned()
        .unwrap_or_default();

    let mut params = build_params(&path_params, &query.normalized, &lowered_headers);
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
            &lowered_headers,
            &cookies,
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

    let mut body_payload = request_options.body_payload();
    if let Some(max_bytes) = max_body_size {
        if let Some(encoded) = body_payload.metadata.raw_base64.as_deref() {
            let raw_len = match BASE64_STANDARD.decode(encoded) {
                Ok(decoded) => decoded.len(),
                Err(_) => 0,
            };
            if raw_len > max_bytes {
                let problem = ProblemDetails::new(
                    "https://spikard.dev/errors/payload-too-large",
                    "Payload Too Large",
                    StatusCode::PAYLOAD_TOO_LARGE,
                )
                .with_detail(format!("Request body exceeded limit of {max_bytes} bytes"));
                return Ok(ResponseSnapshot::from_problem(problem, &headers, &context.config));
            }
        }
    }
    if let Some(request_validator) = validators.request.as_ref() {
        if body_payload.value.is_some() && !matches!(body_payload.metadata.kind, BodyKind::Form | BodyKind::Multipart) {
            let content_type = header_value(&headers, "content-type");
            if !is_json_content_type(content_type) {
                let problem = ProblemDetails::new(
                    "https://spikard.dev/errors/unsupported-media-type",
                    "Unsupported Media Type",
                    StatusCode::UNSUPPORTED_MEDIA_TYPE,
                )
                .with_detail("Unsupported media type".to_string());
                return Ok(ResponseSnapshot::from_problem(problem, &headers, &context.config));
            }

            if let Some(Value::String(text)) = body_payload.value.clone() {
                if let Ok(parsed) = serde_json::from_str::<Value>(&text) {
                    body_payload.value = Some(parsed);
                    body_payload.metadata = BodyMetadata::from_body_value(body_payload.value.as_ref());
                }
            }
        }

        if matches!(body_payload.metadata.kind, BodyKind::Form | BodyKind::Multipart) {
            if let Some(value) = body_payload.value.take() {
                let coerced = coerce_value_against_schema(value, request_validator.schema());
                body_payload.value = Some(coerced);
            }
        }
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
        cookies.clone(),
        query.clone(),
        params,
        body_value.clone(),
        body_metadata,
    );

    let lifecycle_runner = LifecycleRunner::new(context.lifecycle_hooks.clone());

    let mut request_payload = match lifecycle_runner.run_before_handler(request_payload).await? {
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

    if !context.dependencies.is_empty() {
        let resolved = resolve_dependency_map(&context.dependencies, &context.dependency_singletons).await?;
        if !resolved.is_empty() {
            request_payload.dependencies = Some(resolved);
        }
    }

    let handler_fn = context
        .handlers
        .get(&route.handler_name)
        .ok_or_else(|| JsValue::from_str(&format!("Handler {} not registered", route.handler_name)))?;

    let request_json = serde_json::to_string(&request_payload)
        .map_err(|err| JsValue::from_str(&format!("Failed to serialize request: {err}")))?;
    let request_object = js_sys::Object::new();
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

    let raced = if let Some(seconds) = request_timeout_seconds {
        let (timeout_promise, timeout_id) = build_timeout_promise(seconds)?;
        let guarded = attach_clear_timeout(&promise, timeout_id)?;
        Promise::race(&Array::of2(&guarded, &timeout_promise))
    } else {
        promise
    };

    let result = wasm_bindgen_futures::JsFuture::from(raced).await;

    let response_value = match result {
        Ok(value) => {
            if is_timeout_marker(&value) {
                let problem = ProblemDetails::new(
                    "https://spikard.dev/errors/request-timeout",
                    "Request Timeout",
                    StatusCode::REQUEST_TIMEOUT,
                );
                return Ok(ResponseSnapshot::from_problem(problem, &headers, &context.config));
            }
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

fn parse_dependencies(dependencies: JsValue) -> Result<HashMap<String, DependencyDescriptor>, JsValue> {
    if dependencies.is_undefined() || dependencies.is_null() {
        return Ok(HashMap::new());
    }

    let deps_object: Object = dependencies
        .dyn_into()
        .map_err(|_| JsValue::from_str("Dependencies must be an object map"))?;

    let dep_keys = js_sys::Object::keys(&deps_object);
    let mut dep_map = HashMap::new();

    for idx in 0..dep_keys.length() {
        let key = dep_keys.get(idx).as_string().unwrap_or_default();
        if key.is_empty() {
            continue;
        }

        let dep_val = Reflect::get(&deps_object, &JsValue::from_str(&key))
            .map_err(|_| JsValue::from_str("Failed to read dependency descriptor"))?;
        let dep_obj: Object = dep_val
            .dyn_into()
            .map_err(|_| JsValue::from_str("Dependency descriptor must be an object"))?;

        let is_factory = Reflect::get(&dep_obj, &JsValue::from_str("isFactory"))
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let singleton = Reflect::get(&dep_obj, &JsValue::from_str("singleton"))
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let cacheable = Reflect::get(&dep_obj, &JsValue::from_str("cacheable"))
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(!is_factory);

        let depends_on = Reflect::get(&dep_obj, &JsValue::from_str("dependsOn"))
            .ok()
            .and_then(|v| v.dyn_into::<js_sys::Array>().ok())
            .map(|arr| {
                (0..arr.length())
                    .filter_map(|i| arr.get(i).as_string())
                    .collect::<Vec<String>>()
            })
            .unwrap_or_default();

        if is_factory {
            let factory_val = Reflect::get(&dep_obj, &JsValue::from_str("factory"))
                .map_err(|_| JsValue::from_str("Failed to read dependency factory"))?;
            let factory: Function = factory_val
                .dyn_into()
                .map_err(|_| JsValue::from_str("Dependency factory must be a function"))?;

            dep_map.insert(
                key,
                DependencyDescriptor {
                    is_factory: true,
                    value_json: None,
                    factory: Some(factory),
                    depends_on,
                    singleton,
                    cacheable,
                },
            );
        } else {
            let value_val = Reflect::get(&dep_obj, &JsValue::from_str("value"))
                .map_err(|_| JsValue::from_str("Failed to read dependency value"))?;
            let value_json = JSON::stringify(&value_val)
                .ok()
                .and_then(|v| v.as_string())
                .unwrap_or_else(|| "null".to_string());

            dep_map.insert(
                key,
                DependencyDescriptor {
                    is_factory: false,
                    value_json: Some(value_json),
                    factory: None,
                    depends_on: Vec::new(),
                    singleton,
                    cacheable,
                },
            );
        }
    }

    Ok(dep_map)
}

fn is_timeout_marker(value: &JsValue) -> bool {
    if !value.is_object() {
        return false;
    }
    let obj = Object::from(value.clone());
    Reflect::get(&obj, &JsValue::from_str("__spikard_timeout__"))
        .ok()
        .and_then(|marker| marker.as_bool())
        .unwrap_or(false)
}

fn build_timeout_promise(seconds: u64) -> Result<(Promise, JsValue), JsValue> {
    use std::cell::RefCell;
    use std::rc::Rc;

    let global = js_sys::global();
    let set_timeout = Reflect::get(&global, &JsValue::from_str("setTimeout"))?
        .dyn_into::<Function>()
        .map_err(|_| JsValue::from_str("setTimeout is not a function"))?;

    let timeout_id_cell: Rc<RefCell<Option<JsValue>>> = Rc::new(RefCell::new(None));
    let timeout_id_writer = timeout_id_cell.clone();

    let promise = Promise::new(&mut |resolve, _reject| {
        let resolve_fn = resolve.clone();
        let callback = Closure::once_into_js(move || {
            let marker = Object::new();
            let _ = Reflect::set(&marker, &JsValue::from_str("__spikard_timeout__"), &JsValue::TRUE);
            let _ = resolve_fn.call1(&JsValue::UNDEFINED, &marker);
        });

        let delay_ms = JsValue::from_f64((seconds as f64) * 1000.0);
        let timeout_id = set_timeout
            .call2(&global, &callback, &delay_ms)
            .unwrap_or(JsValue::NULL);
        *timeout_id_writer.borrow_mut() = Some(timeout_id);
    });

    let timeout_id = timeout_id_cell.borrow().as_ref().cloned().unwrap_or(JsValue::NULL);
    Ok((promise, timeout_id))
}

fn attach_clear_timeout(promise: &Promise, timeout_id: JsValue) -> Result<Promise, JsValue> {
    let global = js_sys::global();
    let clear_timeout = Reflect::get(&global, &JsValue::from_str("clearTimeout"))?
        .dyn_into::<Function>()
        .map_err(|_| JsValue::from_str("clearTimeout is not a function"))?;

    let clear_timeout_success = clear_timeout.clone();
    let global_success = global.clone();
    let timeout_id_success = timeout_id.clone();
    let on_fulfilled = Closure::wrap(Box::new(move |value: JsValue| -> JsValue {
        let _ = clear_timeout_success.call1(&global_success, &timeout_id_success);
        value
    }) as Box<dyn FnMut(JsValue) -> JsValue>);

    let clear_timeout_error = clear_timeout.clone();
    let global_error = global.clone();
    let timeout_id_error = timeout_id.clone();
    let on_rejected = Closure::wrap(Box::new(move |err: JsValue| -> JsValue {
        let _ = clear_timeout_error.call1(&global_error, &timeout_id_error);
        Promise::reject(&err).into()
    }) as Box<dyn FnMut(JsValue) -> JsValue>);

    let chained = promise.then2(&on_fulfilled, &on_rejected);
    on_fulfilled.forget();
    on_rejected.forget();
    Ok(chained)
}

async fn resolve_dependency_map(
    registry: &HashMap<String, DependencyDescriptor>,
    singleton_cache: &std::rc::Rc<std::cell::RefCell<HashMap<String, String>>>,
) -> Result<HashMap<String, Value>, JsValue> {
    let mut keys: Vec<String> = registry.keys().cloned().collect();
    keys.sort();

    let mut resolving: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut request_cache: HashMap<String, String> = HashMap::new();
    let mut resolved_values: HashMap<String, Value> = HashMap::new();

    for key in keys {
        let value_json =
            resolve_dependency_key(&key, registry, singleton_cache, &mut resolving, &mut request_cache).await?;

        let parsed = serde_json::from_str::<Value>(&value_json).unwrap_or(Value::String(value_json));
        resolved_values.insert(key, parsed);
    }

    Ok(resolved_values)
}

fn resolve_dependency_key<'a>(
    key: &'a str,
    registry: &'a HashMap<String, DependencyDescriptor>,
    singleton_cache: &'a std::rc::Rc<std::cell::RefCell<HashMap<String, String>>>,
    resolving: &'a mut std::collections::HashSet<String>,
    request_cache: &'a mut HashMap<String, String>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, JsValue>> + 'a>> {
    Box::pin(async move {
        if let Some(cached) = singleton_cache.borrow().get(key) {
            return Ok(cached.clone());
        }
        if let Some(cached) = request_cache.get(key) {
            return Ok(cached.clone());
        }

        if !resolving.insert(key.to_string()) {
            return Err(JsValue::from_str(&format!("Circular dependency detected for '{key}'")));
        }

        let descriptor = registry
            .get(key)
            .ok_or_else(|| JsValue::from_str(&format!("Dependency '{key}' is not registered")))?;

        let value_json = if !descriptor.is_factory {
            descriptor.value_json.clone().unwrap_or_else(|| "null".to_string())
        } else {
            let mut dep_map = JsonMap::new();
            for dep_key in &descriptor.depends_on {
                let dep_json =
                    resolve_dependency_key(dep_key, registry, singleton_cache, resolving, request_cache).await?;
                let dep_value = serde_json::from_str::<Value>(&dep_json).unwrap_or(Value::String(dep_json));
                dep_map.insert(dep_key.clone(), dep_value);
            }
            let deps_json =
                serde_json::to_string(&Value::Object(dep_map)).map_err(|err| JsValue::from_str(&err.to_string()))?;

            let factory = descriptor
                .factory
                .as_ref()
                .ok_or_else(|| JsValue::from_str("Factory dependency missing function"))?;

            let call_result = factory
                .call1(&JsValue::NULL, &JsValue::from_str(&deps_json))
                .map_err(|_| JsValue::from_str("Failed to invoke dependency factory"))?;

            let maybe_promise: Result<Promise, _> = call_result.clone().dyn_into();
            let result_val = match maybe_promise {
                Ok(promise) => wasm_bindgen_futures::JsFuture::from(promise).await?,
                Err(_) => call_result,
            };

            if let Some(text) = result_val.as_string() {
                text
            } else {
                JSON::stringify(&result_val)
                    .ok()
                    .and_then(|v| v.as_string())
                    .unwrap_or_else(|| "null".to_string())
            }
        };

        if descriptor.singleton {
            singleton_cache.borrow_mut().insert(key.to_string(), value_json.clone());
        } else if descriptor.cacheable {
            request_cache.insert(key.to_string(), value_json.clone());
        }

        resolving.remove(key);
        Ok(value_json)
    })
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

fn lowercase_header_keys(headers: &HashMap<String, String>) -> HashMap<String, String> {
    let mut lowered = HashMap::new();
    for (key, value) in headers {
        lowered.insert(key.to_ascii_lowercase(), value.clone());
    }
    lowered
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
        if config.enable_request_id {
            let request_id = header_value(request_headers, "x-request-id")
                .map(|value| value.to_string())
                .unwrap_or_else(|| Uuid::new_v4().to_string());
            raw.headers.insert("x-request-id".to_string(), request_id);
        }

        if let Some(compression) = &config.compression {
            let http_config = spikard_core::CompressionConfig {
                gzip: compression.gzip,
                brotli: compression.brotli,
                min_size: compression.min_size,
                quality: compression.quality as u32,
            };
            raw.apply_compression(request_headers, &http_config);
        }

        raw.headers = raw
            .headers
            .into_iter()
            .map(|(key, value)| (key.to_ascii_lowercase(), value))
            .collect();

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

fn is_json_content_type(content_type: Option<&str>) -> bool {
    let Some(value) = content_type else {
        return true;
    };
    let media_type = value.split(';').next().unwrap_or("").trim().to_ascii_lowercase();
    if media_type.is_empty() {
        return true;
    }
    media_type == "application/json" || media_type.ends_with("+json")
}

fn parse_cookie_header(headers: &HashMap<String, String>) -> HashMap<String, String> {
    let Some(value) = header_value(headers, "cookie") else {
        return HashMap::new();
    };
    let mut cookies = HashMap::new();
    for segment in value.split(';') {
        let trimmed = segment.trim();
        if trimmed.is_empty() {
            continue;
        }
        let Some((name, cookie_value)) = trimmed.split_once('=') else {
            continue;
        };
        let name = name.trim();
        if name.is_empty() {
            continue;
        }
        cookies.insert(name.to_string(), cookie_value.trim().to_string());
    }
    cookies
}

fn coerce_value_against_schema(value: Value, schema: &Value) -> Value {
    let Some(schema_obj) = schema.as_object() else {
        return value;
    };

    if let Some(any_of) = schema_obj.get("anyOf").and_then(|v| v.as_array()) {
        for candidate in any_of {
            let coerced = coerce_value_against_schema(value.clone(), candidate);
            if coerced != value {
                return coerced;
            }
        }
        return value;
    }

    if let Some(one_of) = schema_obj.get("oneOf").and_then(|v| v.as_array()) {
        for candidate in one_of {
            let coerced = coerce_value_against_schema(value.clone(), candidate);
            if coerced != value {
                return coerced;
            }
        }
        return value;
    }

    let expected_type = schema_obj.get("type").and_then(|v| v.as_str());

    match (expected_type, value) {
        (Some("integer"), Value::String(text)) => text
            .trim()
            .parse::<i64>()
            .map(serde_json::Number::from)
            .map(Value::Number)
            .unwrap_or(Value::String(text)),
        (Some("number"), Value::String(text)) => text
            .trim()
            .parse::<f64>()
            .ok()
            .and_then(serde_json::Number::from_f64)
            .map(Value::Number)
            .unwrap_or(Value::String(text)),
        (Some("boolean"), Value::String(text)) => match text.trim().to_ascii_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => Value::Bool(true),
            "false" | "0" | "no" | "off" => Value::Bool(false),
            _ => Value::String(text),
        },
        (Some("array"), Value::Array(items)) => {
            let Some(items_schema) = schema_obj.get("items") else {
                return Value::Array(items);
            };
            let coerced = items
                .into_iter()
                .map(|item| coerce_value_against_schema(item, items_schema))
                .collect::<Vec<_>>();
            Value::Array(coerced)
        }
        (Some("object"), Value::Object(mut obj)) => {
            let Some(properties) = schema_obj.get("properties").and_then(|v| v.as_object()) else {
                return Value::Object(obj);
            };
            for (key, prop_schema) in properties {
                if let Some(current) = obj.get(key).cloned() {
                    obj.insert(key.clone(), coerce_value_against_schema(current, prop_schema));
                }
            }
            Value::Object(obj)
        }
        (_, other) => other,
    }
}

fn value_to_param_map(value: Value) -> HashMap<String, Value> {
    match value {
        Value::Object(map) => map.into_iter().collect(),
        _ => HashMap::new(),
    }
}

fn unauthorized_problem(title: &str, detail: impl Into<String>) -> ProblemDetails {
    ProblemDetails::new(
        "https://spikard.dev/errors/unauthorized",
        title,
        StatusCode::UNAUTHORIZED,
    )
    .with_detail(detail)
}

fn enforce_auth(
    headers: &HashMap<String, String>,
    raw_query: &HashMap<String, Vec<String>>,
    config: &ServerConfig,
) -> Option<ResponseSnapshot> {
    let jwt = config.jwt_auth.as_ref();
    let api_key = config.api_key_auth.as_ref();

    if jwt.is_none() && api_key.is_none() {
        return None;
    }

    let authorization = header_value(headers, "authorization");
    let api_key_candidate = api_key.and_then(|cfg| extract_api_key(headers, raw_query, cfg));

    if let Some(jwt_cfg) = jwt {
        if let Some(auth_header) = authorization {
            match validate_jwt_from_authorization(auth_header, jwt_cfg) {
                Ok(()) => return None,
                Err(problem) => return Some(ResponseSnapshot::from_problem(problem, headers, config)),
            }
        }
    }

    if let Some(api_cfg) = api_key {
        if api_key_candidate.is_some() {
            match validate_api_key(api_key_candidate, api_cfg) {
                Ok(()) => return None,
                Err(problem) => return Some(ResponseSnapshot::from_problem(problem, headers, config)),
            }
        }
    }

    if jwt.is_some() {
        let problem = unauthorized_problem(
            "Missing or invalid Authorization header",
            "Expected 'Authorization: Bearer <token>'",
        );
        return Some(ResponseSnapshot::from_problem(problem, headers, config));
    }

    if let Some(api_cfg) = api_key {
        let problem = unauthorized_problem(
            "Missing API key",
            format!(
                "Expected '{}' header or 'api_key' query parameter with valid API key",
                api_cfg.header_name
            ),
        );
        return Some(ResponseSnapshot::from_problem(problem, headers, config));
    }

    None
}

fn extract_api_key(
    headers: &HashMap<String, String>,
    raw_query: &HashMap<String, Vec<String>>,
    config: &ApiKeyConfig,
) -> Option<String> {
    header_value(headers, &config.header_name)
        .map(|value| value.to_string())
        .or_else(|| raw_query.get("api_key").and_then(|vals| vals.first().cloned()))
}

fn validate_api_key(candidate: Option<String>, config: &ApiKeyConfig) -> Result<(), ProblemDetails> {
    let Some(value) = candidate else {
        return Err(unauthorized_problem(
            "Missing API key",
            format!(
                "Expected '{}' header or 'api_key' query parameter with valid API key",
                config.header_name
            ),
        ));
    };

    if config.keys.iter().any(|key| key == &value) {
        Ok(())
    } else {
        Err(unauthorized_problem(
            "Invalid API key",
            "The provided API key is not valid",
        ))
    }
}

fn validate_jwt_from_authorization(header: &str, config: &JwtConfig) -> Result<(), ProblemDetails> {
    let trimmed = header.trim();
    if !trimmed.to_ascii_lowercase().starts_with("bearer ") {
        return Err(unauthorized_problem(
            "Invalid Authorization header format",
            "Authorization header must use Bearer scheme: 'Bearer <token>'",
        ));
    }
    let token = trimmed[7..].trim();
    if token.is_empty() {
        return Err(unauthorized_problem(
            "Missing or invalid Authorization header",
            "Expected 'Authorization: Bearer <token>'",
        ));
    }
    validate_jwt(token, config)
}

fn validate_jwt(token: &str, config: &JwtConfig) -> Result<(), ProblemDetails> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(unauthorized_problem(
            "Malformed JWT token",
            format!(
                "Malformed JWT token: expected 3 parts separated by dots, found {}",
                parts.len()
            ),
        ));
    }

    let signing_input = format!("{}.{}", parts[0], parts[1]);
    let header_bytes = BASE64_URL_SAFE_NO_PAD
        .decode(parts[0])
        .map_err(|_| unauthorized_problem("JWT validation failed", "Token is invalid"))?;
    let payload_bytes = BASE64_URL_SAFE_NO_PAD
        .decode(parts[1])
        .map_err(|_| unauthorized_problem("JWT validation failed", "Token is invalid"))?;
    let signature_bytes = BASE64_URL_SAFE_NO_PAD
        .decode(parts[2])
        .map_err(|_| unauthorized_problem("JWT validation failed", "Token is invalid"))?;

    let header_json: Value = serde_json::from_slice(&header_bytes)
        .map_err(|_| unauthorized_problem("JWT validation failed", "Token is invalid"))?;
    if let Some(expected_alg) = config.algorithm.as_deref() {
        if let Some(actual) = header_json.get("alg").and_then(|v| v.as_str()) {
            if actual != expected_alg {
                return Err(unauthorized_problem("JWT validation failed", "Token is invalid"));
            }
        }
    }

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(config.secret.as_bytes())
        .map_err(|_| unauthorized_problem("JWT validation failed", "Token is invalid"))?;
    mac.update(signing_input.as_bytes());
    let expected_sig = mac.finalize().into_bytes();
    let expected_sig_bytes: &[u8] = expected_sig.as_ref();
    if expected_sig_bytes != signature_bytes.as_slice() {
        return Err(unauthorized_problem(
            "JWT validation failed",
            "Token signature is invalid",
        ));
    }

    let claims: Value = serde_json::from_slice(&payload_bytes)
        .map_err(|_| unauthorized_problem("JWT validation failed", "Token is invalid"))?;

    let now = (JsDate::now() / 1000.0) as i64;
    let leeway = config.leeway as i64;

    if let Some(exp) = claims.get("exp").and_then(|v| v.as_i64()) {
        if exp + leeway < now {
            return Err(unauthorized_problem("JWT validation failed", "Token has expired"));
        }
    }

    if let Some(nbf) = claims.get("nbf").and_then(|v| v.as_i64()) {
        if nbf - leeway > now {
            return Err(unauthorized_problem(
                "JWT validation failed",
                "JWT not valid yet, not before claim is in the future",
            ));
        }
    }

    if let Some(issuer) = config.issuer.as_deref() {
        match claims.get("iss").and_then(|v| v.as_str()) {
            Some(actual) if actual == issuer => {}
            _ => {
                return Err(unauthorized_problem(
                    "JWT validation failed",
                    format!("Token issuer is invalid, expected '{issuer}'"),
                ));
            }
        }
    }

    if !config.audience.is_empty() {
        let audiences: Vec<String> = match claims.get("aud") {
            Some(Value::String(s)) => vec![s.clone()],
            Some(Value::Array(arr)) => arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect(),
            _ => Vec::new(),
        };
        let matches = audiences
            .iter()
            .any(|aud| config.audience.iter().any(|expected| expected == aud));
        if !matches {
            return Err(unauthorized_problem(
                "JWT validation failed",
                "Token audience is invalid",
            ));
        }
    }

    Ok(())
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
        #[cfg(feature = "di")]
        handler_dependencies: None,
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

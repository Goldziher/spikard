//! PHP exposure of lifecycle hooks (onRequest, preValidation, preHandler, onResponse, onError).
//!
//! This provides full lifecycle hook support by allowing PHP callables to short-circuit or continue.
//! Hook execution order: onRequest → preValidation → preHandler → handler → onResponse → onError (error path)

use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
};
use ext_php_rs::convert::IntoZval;
use ext_php_rs::prelude::*;
use http_body_util::BodyExt;
use serde_json::Value;
use spikard_http::{HookResult, LifecycleHook, LifecycleHooks, LifecycleHooksBuilder};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tracing::error as tracing_error;

use crate::php::{PhpRequest, PhpResponse};

/// Result type exposed to PHP.
#[php_class]
#[php(name = "Spikard\\Lifecycle\\HookResult")]
pub struct PhpHookResult {
    pub(crate) short_circuit: bool,
    pub(crate) response_status: Option<i64>,
    pub(crate) response_body: Option<String>,
}

#[php_impl]
impl PhpHookResult {
    #[php(name = "isShortCircuit")]
    pub fn is_short_circuit(&self) -> bool {
        self.short_circuit
    }

    #[php(name = "getStatus")]
    pub fn get_status(&self) -> Option<i64> {
        self.response_status
    }

    #[php(name = "getBody")]
    pub fn get_body(&self) -> Option<String> {
        self.response_body.clone()
    }
}

/// Lifecycle hooks wrapper exposed to PHP.
#[php_class]
#[php(name = "Spikard\\Lifecycle\\Hooks")]
#[derive(Default)]
pub struct PhpLifecycleHooks {
    on_request_hooks: Vec<Arc<dyn LifecycleHook<Request<Body>, Response<Body>>>>,
    pre_validation_hooks: Vec<Arc<dyn LifecycleHook<Request<Body>, Response<Body>>>>,
    pre_handler_hooks: Vec<Arc<dyn LifecycleHook<Request<Body>, Response<Body>>>>,
    on_response_hooks: Vec<Arc<dyn LifecycleHook<Request<Body>, Response<Body>>>>,
    on_error_hooks: Vec<Arc<dyn LifecycleHook<Request<Body>, Response<Body>>>>,
}

#[php_impl]
impl PhpLifecycleHooks {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register an onRequest hook.
    /// Runs first in the lifecycle, before validation. Can short-circuit the request.
    #[php(name = "onRequest")]
    pub fn on_request(&mut self, name: String, callback: &ext_php_rs::types::Zval) {
        let hook = make_request_hook(name, callback);
        self.on_request_hooks.push(hook);
    }

    /// Register a preValidation hook.
    /// Runs after onRequest, before request validation. Can short-circuit the request.
    #[php(name = "preValidation")]
    pub fn pre_validation(&mut self, name: String, callback: &ext_php_rs::types::Zval) {
        let hook = make_request_hook(name, callback);
        self.pre_validation_hooks.push(hook);
    }

    /// Register a preHandler hook.
    /// Runs after validation, before the main handler. Can short-circuit the request.
    #[php(name = "preHandler")]
    pub fn pre_handler(&mut self, name: String, callback: &ext_php_rs::types::Zval) {
        let hook = make_request_hook(name, callback);
        self.pre_handler_hooks.push(hook);
    }

    /// Register an onResponse hook.
    /// Runs after the handler completes successfully. Can modify the response.
    #[php(name = "onResponse")]
    pub fn on_response(&mut self, name: String, callback: &ext_php_rs::types::Zval) {
        let hook = make_response_hook(name, callback);
        self.on_response_hooks.push(hook);
    }

    /// Register an onError hook.
    /// Runs when an error occurs during request processing. Can modify error responses.
    #[php(name = "onError")]
    pub fn on_error(&mut self, name: String, callback: &ext_php_rs::types::Zval) {
        let hook = make_error_hook(name, callback);
        self.on_error_hooks.push(hook);
    }
}

impl PhpLifecycleHooks {
    /// Build LifecycleHooks from the accumulated hooks.
    /// Internal-only method, not exposed to PHP.
    pub fn build(&self) -> LifecycleHooks {
        let mut builder = LifecycleHooksBuilder::new();

        for hook in &self.on_request_hooks {
            builder = builder.on_request(Arc::clone(hook));
        }
        for hook in &self.pre_validation_hooks {
            builder = builder.pre_validation(Arc::clone(hook));
        }
        for hook in &self.pre_handler_hooks {
            builder = builder.pre_handler(Arc::clone(hook));
        }
        for hook in &self.on_response_hooks {
            builder = builder.on_response(Arc::clone(hook));
        }
        for hook in &self.on_error_hooks {
            builder = builder.on_error(Arc::clone(hook));
        }

        builder.build()
    }
}

thread_local! {
    static PHP_HOOK_REGISTRY: std::cell::RefCell<Vec<ext_php_rs::types::Zval>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// Convert axum Request to PhpRequest for PHP hooks (synchronous extraction).
fn axum_request_to_php_sync(req: &Request<Body>) -> PhpRequest {
    let method = req.method().to_string();
    let path = req.uri().path().to_string();

    let mut headers = HashMap::new();
    for (name, value) in req.headers() {
        if let Ok(v) = value.to_str() {
            headers.insert(name.to_string(), v.to_string());
        }
    }

    let raw_query = req
        .uri()
        .query()
        .map(|q| {
            url::form_urlencoded::parse(q.as_bytes())
                .into_owned()
                .fold(HashMap::new(), |mut acc, (k, v)| {
                    acc.entry(k).or_insert_with(Vec::new).push(v);
                    acc
                })
        })
        .unwrap_or_default();

    PhpRequest::from_parts(
        method,
        path,
        Value::Null,
        Value::Object(serde_json::Map::new()),
        None,
        headers,
        HashMap::new(),
        raw_query,
        HashMap::new(),
    )
}

/// Convert PhpResponse to axum Response for PHP hooks.
fn php_response_to_axum(php_resp: &PhpResponse) -> Result<Response<Body>, String> {
    let status = StatusCode::from_u16(php_resp.status_code as u16)
        .map_err(|e| format!("Invalid status code {}: {}", php_resp.status_code, e))?;

    let body_bytes = match &php_resp.body {
        Value::String(s) => s.as_bytes().to_vec(),
        _ => serde_json::to_vec(&php_resp.body).map_err(|e| format!("Failed to serialize response body: {}", e))?,
    };

    let mut builder = Response::builder().status(status);

    for (key, value) in &php_resp.headers {
        builder = builder.header(key, value);
    }

    builder
        .body(Body::from(body_bytes))
        .map_err(|e| format!("Failed to build response: {}", e))
}

/// Convert axum Response to PhpResponse for PHP hooks.
async fn axum_response_to_php(resp: Response<Body>) -> Result<(PhpResponse, Response<Body>), String> {
    let (parts, body) = resp.into_parts();

    let body_bytes = body
        .collect()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?
        .to_bytes();

    let body_value: Value = serde_json::from_slice(&body_bytes)
        .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&body_bytes).to_string()));

    let mut headers = HashMap::new();
    for (name, value) in &parts.headers {
        if let Ok(v) = value.to_str() {
            headers.insert(name.to_string(), v.to_string());
        }
    }

    let php_resp = PhpResponse {
        status_code: parts.status.as_u16() as i64,
        body: body_value,
        headers: headers.clone(),
        cookies: HashMap::new(),
    };

    let mut builder = Response::builder().status(parts.status);
    for (name, value) in parts.headers {
        if let Some(name) = name {
            builder = builder.header(name, value);
        }
    }

    let rebuilt_resp = builder
        .body(Body::from(body_bytes))
        .map_err(|e| format!("Failed to rebuild response: {}", e))?;

    Ok((php_resp, rebuilt_resp))
}

/// PHP implementation of request lifecycle hook
pub struct PhpRequestHook {
    name: String,
    callback_index: usize,
}

impl PhpRequestHook {
    pub fn new_from_zval(callback: &ext_php_rs::types::Zval) -> Result<Self, String> {
        if !callback.is_callable() {
            return Err("Callback is not callable".to_string());
        }

        let callback_index = PHP_HOOK_REGISTRY.with(|registry| {
            let mut registry = registry.borrow_mut();
            let idx = registry.len();
            let zval = callback.shallow_clone();
            registry.push(zval);
            idx
        });

        Ok(Self {
            name: "PhpRequestHook".to_string(),
            callback_index,
        })
    }
}

impl LifecycleHook<Request<Body>, Response<Body>> for PhpRequestHook {
    fn name(&self) -> &str {
        &self.name
    }

    fn execute_request<'a>(
        &'a self,
        req: Request<Body>,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>, Response<Body>>, String>> + Send + 'a>> {
        let callback_index = self.callback_index;
        Box::pin(async move {
            let php_req = axum_request_to_php_sync(&req);
            let result = PHP_HOOK_REGISTRY.with(|registry| -> Option<Result<Option<PhpResponse>, String>> {
                let registry = registry.borrow();
                let callback_zval = registry.get(callback_index)?;
                let callable = ext_php_rs::types::ZendCallable::new(callback_zval).ok()?;
                let req_zval = match php_req.into_zval(false) {
                    Ok(z) => z,
                    Err(e) => return Some(Err(format!("Failed to convert request to Zval: {:?}", e))),
                };
                let result_zval = match callable.try_call(vec![&req_zval]) {
                    Ok(z) => z,
                    Err(e) => return Some(Err(format!("PHP hook failed: {:?}", e))),
                };
                if result_zval.is_null() {
                    Some(Ok(None))
                } else if let Some(obj) = result_zval.object() {
                    let status = obj
                        .try_call_method("getStatus", vec![])
                        .ok()
                        .and_then(|v| v.long())
                        .unwrap_or(200);
                    let body_str = obj
                        .try_call_method("getBody", vec![])
                        .ok()
                        .and_then(|v| v.string())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| "{}".to_string());
                    let body: Value = serde_json::from_str(&body_str).unwrap_or_else(|_| Value::String(body_str));
                    let mut headers = HashMap::new();
                    if let Ok(headers_zval) = obj.try_call_method("getHeaders", vec![])
                        && let Some(arr) = headers_zval.array()
                    {
                        for (key, val) in arr.iter() {
                            let key_str = match key {
                                ext_php_rs::types::ArrayKey::Long(i) => i.to_string(),
                                ext_php_rs::types::ArrayKey::String(s) => s.to_string(),
                                ext_php_rs::types::ArrayKey::Str(s) => s.to_string(),
                            };
                            if let Some(val_str) = val.string() {
                                headers.insert(key_str, val_str.to_string());
                            }
                        }
                    }
                    Some(Ok(Some(PhpResponse {
                        status_code: status,
                        body,
                        headers,
                        cookies: HashMap::new(),
                    })))
                } else {
                    Some(Err("Hook returned invalid type (expected null or Response)".to_string()))
                }
            });

            match result {
                Some(Ok(None)) => Ok(HookResult::Continue(req)),
                Some(Ok(Some(php_resp))) => match php_response_to_axum(&php_resp) {
                    Ok(resp) => Ok(HookResult::ShortCircuit(resp)),
                    Err(e) => {
                        tracing_error!("Failed to convert PHP response: {}", e);
                        Ok(HookResult::Continue(req))
                    }
                },
                Some(Err(e)) => {
                    tracing_error!("Hook error: {}", e);
                    Ok(HookResult::Continue(req))
                }
                None => Ok(HookResult::Continue(req)),
            }
        })
    }

    fn execute_response<'a>(
        &'a self,
        resp: Response<Body>,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Response<Body>, Response<Body>>, String>> + Send + 'a>> {
        Box::pin(async move { Ok(HookResult::Continue(resp)) })
    }
}

/// PHP implementation of response lifecycle hook
pub struct PhpResponseHook {
    name: String,
    callback_index: usize,
}

impl PhpResponseHook {
    pub fn new_from_zval(callback: &ext_php_rs::types::Zval) -> Result<Self, String> {
        if !callback.is_callable() {
            return Err("Callback is not callable".to_string());
        }

        let callback_index = PHP_HOOK_REGISTRY.with(|registry| {
            let mut registry = registry.borrow_mut();
            let idx = registry.len();
            let zval = callback.shallow_clone();
            registry.push(zval);
            idx
        });

        Ok(Self {
            name: "PhpResponseHook".to_string(),
            callback_index,
        })
    }
}

impl LifecycleHook<Request<Body>, Response<Body>> for PhpResponseHook {
    fn name(&self) -> &str {
        &self.name
    }

    fn execute_request<'a>(
        &'a self,
        req: Request<Body>,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>, Response<Body>>, String>> + Send + 'a>> {
        Box::pin(async move { Ok(HookResult::Continue(req)) })
    }

    fn execute_response<'a>(
        &'a self,
        resp: Response<Body>,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Response<Body>, Response<Body>>, String>> + Send + 'a>> {
        let callback_index = self.callback_index;
        Box::pin(async move {
            let (php_resp, original_resp) = axum_response_to_php(resp).await.unwrap_or_else(|e| {
                tracing_error!("Failed to convert response to PHP: {}", e);
                (
                    PhpResponse {
                        status_code: 500,
                        body: Value::Null,
                        headers: HashMap::new(),
                        cookies: HashMap::new(),
                    },
                    Response::new(Body::empty()),
                )
            });
            let result = tokio::task::spawn_blocking(move || {
                PHP_HOOK_REGISTRY.with(|registry| -> Option<Result<Option<PhpResponse>, String>> {
                    let registry = registry.borrow();
                    let callback_zval = registry.get(callback_index)?;
                    let callable = ext_php_rs::types::ZendCallable::new(callback_zval).ok()?;
                    let resp_zval = match php_resp.into_zval(false) {
                        Ok(z) => z,
                        Err(e) => return Some(Err(format!("Failed to convert response to Zval: {:?}", e))),
                    };
                    let result_zval = match callable.try_call(vec![&resp_zval]) {
                        Ok(z) => z,
                        Err(e) => return Some(Err(format!("PHP hook failed: {:?}", e))),
                    };
                    if result_zval.is_null() {
                        Some(Ok(None))
                    } else if let Some(obj) = result_zval.object() {
                        let status = obj
                            .try_call_method("getStatus", vec![])
                            .ok()
                            .and_then(|v| v.long())
                            .unwrap_or(200);
                        let body_str = obj
                            .try_call_method("getBody", vec![])
                            .ok()
                            .and_then(|v| v.string())
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| "{}".to_string());
                        let body: Value = serde_json::from_str(&body_str).unwrap_or_else(|_| Value::String(body_str));
                        let mut headers = HashMap::new();
                        if let Ok(headers_zval) = obj.try_call_method("getHeaders", vec![])
                            && let Some(arr) = headers_zval.array()
                        {
                            for (key, val) in arr.iter() {
                                let key_str = match key {
                                    ext_php_rs::types::ArrayKey::Long(i) => i.to_string(),
                                    ext_php_rs::types::ArrayKey::String(s) => s.to_string(),
                                    ext_php_rs::types::ArrayKey::Str(s) => s.to_string(),
                                };
                                if let Some(val_str) = val.string() {
                                    headers.insert(key_str, val_str.to_string());
                                }
                            }
                        }
                        Some(Ok(Some(PhpResponse {
                            status_code: status,
                            body,
                            headers,
                            cookies: HashMap::new(),
                        })))
                    } else {
                        Some(Err("Hook returned invalid type (expected null or Response)".to_string()))
                    }
                })
            })
            .await;
            match result {
                Ok(Some(Ok(None))) => Ok(HookResult::Continue(original_resp)),
                Ok(Some(Ok(Some(php_resp)))) => match php_response_to_axum(&php_resp) {
                    Ok(resp) => Ok(HookResult::Continue(resp)),
                    Err(e) => {
                        tracing_error!("Failed to convert PHP response: {}", e);
                        Ok(HookResult::Continue(original_resp))
                    }
                },
                Ok(Some(Err(e))) => {
                    tracing_error!("Hook error: {}", e);
                    Ok(HookResult::Continue(original_resp))
                }
                _ => Ok(HookResult::Continue(original_resp)),
            }
        })
    }
}

/// PHP implementation of error lifecycle hook
pub struct PhpErrorHook {
    name: String,
    callback_index: usize,
}

impl PhpErrorHook {
    pub fn new_from_zval(callback: &ext_php_rs::types::Zval) -> Result<Self, String> {
        if !callback.is_callable() {
            return Err("Callback is not callable".to_string());
        }

        let callback_index = PHP_HOOK_REGISTRY.with(|registry| {
            let mut registry = registry.borrow_mut();
            let idx = registry.len();
            let zval = callback.shallow_clone();
            registry.push(zval);
            idx
        });

        Ok(Self {
            name: "PhpErrorHook".to_string(),
            callback_index,
        })
    }
}

impl LifecycleHook<Request<Body>, Response<Body>> for PhpErrorHook {
    fn name(&self) -> &str {
        &self.name
    }

    fn execute_request<'a>(
        &'a self,
        req: Request<Body>,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>, Response<Body>>, String>> + Send + 'a>> {
        Box::pin(async move { Ok(HookResult::Continue(req)) })
    }

    fn execute_response<'a>(
        &'a self,
        resp: Response<Body>,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Response<Body>, Response<Body>>, String>> + Send + 'a>> {
        let callback_index = self.callback_index;
        Box::pin(async move {
            let (php_resp, original_resp) = axum_response_to_php(resp).await.unwrap_or_else(|e| {
                tracing_error!("Failed to convert response to PHP: {}", e);
                (
                    PhpResponse {
                        status_code: 500,
                        body: Value::Null,
                        headers: HashMap::new(),
                        cookies: HashMap::new(),
                    },
                    Response::new(Body::empty()),
                )
            });
            let result = tokio::task::spawn_blocking(move || {
                PHP_HOOK_REGISTRY.with(|registry| -> Option<Result<Option<PhpResponse>, String>> {
                    let registry = registry.borrow();
                    let callback_zval = registry.get(callback_index)?;
                    let callable = ext_php_rs::types::ZendCallable::new(callback_zval).ok()?;
                    let resp_zval = match php_resp.into_zval(false) {
                        Ok(z) => z,
                        Err(e) => return Some(Err(format!("Failed to convert response to Zval: {:?}", e))),
                    };
                    let result_zval = match callable.try_call(vec![&resp_zval]) {
                        Ok(z) => z,
                        Err(e) => return Some(Err(format!("PHP hook failed: {:?}", e))),
                    };
                    if result_zval.is_null() {
                        Some(Ok(None))
                    } else if let Some(obj) = result_zval.object() {
                        let status = obj
                            .try_call_method("getStatus", vec![])
                            .ok()
                            .and_then(|v| v.long())
                            .unwrap_or(200);
                        let body_str = obj
                            .try_call_method("getBody", vec![])
                            .ok()
                            .and_then(|v| v.string())
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| "{}".to_string());
                        let body: Value = serde_json::from_str(&body_str).unwrap_or_else(|_| Value::String(body_str));
                        let mut headers = HashMap::new();
                        if let Ok(headers_zval) = obj.try_call_method("getHeaders", vec![])
                            && let Some(arr) = headers_zval.array()
                        {
                            for (key, val) in arr.iter() {
                                let key_str = match key {
                                    ext_php_rs::types::ArrayKey::Long(i) => i.to_string(),
                                    ext_php_rs::types::ArrayKey::String(s) => s.to_string(),
                                    ext_php_rs::types::ArrayKey::Str(s) => s.to_string(),
                                };
                                if let Some(val_str) = val.string() {
                                    headers.insert(key_str, val_str.to_string());
                                }
                            }
                        }
                        Some(Ok(Some(PhpResponse {
                            status_code: status,
                            body,
                            headers,
                            cookies: HashMap::new(),
                        })))
                    } else {
                        Some(Err("Hook returned invalid type (expected null or Response)".to_string()))
                    }
                })
            })
            .await;
            match result {
                Ok(Some(Ok(None))) => Ok(HookResult::Continue(original_resp)),
                Ok(Some(Ok(Some(php_resp)))) => match php_response_to_axum(&php_resp) {
                    Ok(resp) => Ok(HookResult::Continue(resp)),
                    Err(e) => {
                        tracing_error!("Failed to convert PHP response: {}", e);
                        Ok(HookResult::Continue(original_resp))
                    }
                },
                Ok(Some(Err(e))) => {
                    tracing_error!("Hook error: {}", e);
                    Ok(HookResult::Continue(original_resp))
                }
                _ => Ok(HookResult::Continue(original_resp)),
            }
        })
    }
}

/// Adapt a PHP callable into a LifecycleHook for requests.
fn make_request_hook(
    name: String,
    callback: &ext_php_rs::types::Zval,
) -> Arc<dyn LifecycleHook<Request<Body>, Response<Body>>> {
    let callback_index = PHP_HOOK_REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let idx = registry.len();
        let zval = callback.shallow_clone();
        registry.push(zval);
        idx
    });

    Arc::new(PhpRequestHook { name, callback_index })
}

/// Adapt a PHP callable into a LifecycleHook for responses.
fn make_response_hook(
    name: String,
    callback: &ext_php_rs::types::Zval,
) -> Arc<dyn LifecycleHook<Request<Body>, Response<Body>>> {
    let callback_index = PHP_HOOK_REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let idx = registry.len();
        let zval = callback.shallow_clone();
        registry.push(zval);
        idx
    });

    Arc::new(PhpResponseHook { name, callback_index })
}

/// Adapt a PHP callable into a LifecycleHook for error handling.
fn make_error_hook(
    name: String,
    callback: &ext_php_rs::types::Zval,
) -> Arc<dyn LifecycleHook<Request<Body>, Response<Body>>> {
    let callback_index = PHP_HOOK_REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let idx = registry.len();
        let zval = callback.shallow_clone();
        registry.push(zval);
        idx
    });

    Arc::new(PhpErrorHook { name, callback_index })
}

//! Ruby lifecycle hooks implementation
//!
//! This module provides the bridge between Ruby blocks/procs and Rust's lifecycle hook system.
//! Uses magnus to safely call Ruby code from Rust async tasks.

use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
};
use magnus::{RHash, Value, gc::Marker, prelude::*, value::InnerValue, value::Opaque};
use serde_json::Value as JsonValue;
use spikard_http::lifecycle::{HookResult, LifecycleHook};
use std::future::Future;
use std::pin::Pin;

/// Ruby lifecycle hook wrapper
///
/// Wraps a Ruby proc/block and makes it callable from Rust's lifecycle system.
/// Handles conversion between Rust HTTP types and Ruby request/response objects.
pub struct RubyLifecycleHook {
    name: String,
    /// Ruby proc/callable object (Opaque for Send safety)
    func: Opaque<Value>,
}

impl RubyLifecycleHook {
    /// Create a new Ruby lifecycle hook
    pub fn new(name: String, func: Value) -> Self {
        Self {
            name,
            func: func.into(),
        }
    }

    /// Mark Ruby values for GC
    #[allow(dead_code)]
    pub fn mark(&self, marker: &Marker) {
        if let Ok(ruby) = magnus::Ruby::get() {
            marker.mark(self.func.get_inner_with(&ruby));
        }
    }
}

impl LifecycleHook<Request<Body>, Response<Body>> for RubyLifecycleHook {
    fn name(&self) -> &str {
        &self.name
    }

    fn execute_request<'a>(
        &'a self,
        req: Request<Body>,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>, Response<Body>>, String>> + Send + 'a>> {
        let func = self.func;
        let name = self.name.clone();
        let name_for_error = name.clone();

        Box::pin(async move {
            let (parts, body) = req.into_parts();
            let body_bytes = axum::body::to_bytes(body, usize::MAX)
                .await
                .map_err(|e| format!("Failed to read request body: {}", e))?;

            let body_value: JsonValue = if body_bytes.is_empty() {
                JsonValue::Null
            } else {
                serde_json::from_slice(&body_bytes)
                    .unwrap_or_else(|_| JsonValue::String(String::from_utf8_lossy(&body_bytes).to_string()))
            };

            let result = magnus::Ruby::get()
                .map_err(|e| format!("Failed to get Ruby: {}", e))
                .and_then(|ruby| {
                    let request_hash = RHash::new();

                    request_hash
                        .aset(ruby.to_symbol("method"), ruby.str_new(parts.method.as_str()))
                        .map_err(|e| format!("Failed to set method: {}", e))?;
                    request_hash
                        .aset(ruby.to_symbol("path"), ruby.str_new(parts.uri.path()))
                        .map_err(|e| format!("Failed to set path: {}", e))?;

                    let headers_hash = RHash::new();
                    for (key, value) in parts.headers.iter() {
                        headers_hash
                            .aset(ruby.str_new(key.as_str()), ruby.str_new(value.to_str().unwrap_or("")))
                            .map_err(|e| format!("Failed to set header: {}", e))?;
                    }
                    request_hash
                        .aset(ruby.to_symbol("headers"), headers_hash)
                        .map_err(|e| format!("Failed to set headers: {}", e))?;

                    let body_str =
                        serde_json::to_string(&body_value).map_err(|e| format!("Failed to serialize body: {}", e))?;
                    request_hash
                        .aset(ruby.to_symbol("body"), ruby.str_new(&body_str))
                        .map_err(|e| format!("Failed to set body: {}", e))?;

                    let func_value = ruby.get_inner(func);
                    let result: Value = func_value
                        .funcall("call", (request_hash,))
                        .map_err(|e| format!("Hook '{}' call failed: {}", name, e))?;

                    if let Some(result_hash) = RHash::from_value(result) {
                        if let Some(status_value) = result_hash.get(ruby.to_symbol("status_code")) {
                            let status = i64::try_convert(status_value)
                                .map_err(|e| format!("Failed to convert status code: {}", e))?;

                            let content = result_hash
                                .get(ruby.to_symbol("content"))
                                .or_else(|| result_hash.get(ruby.to_symbol("body")))
                                .unwrap_or_else(|| ruby.qnil().as_value());

                            let body_str = if content.is_nil() {
                                "{}".to_string()
                            } else {
                                String::try_convert(content).unwrap_or_else(|_| {
                                    content
                                        .to_r_string()
                                        .map(|s| s.to_string().unwrap_or_else(|_| "{}".to_string()))
                                        .unwrap_or_else(|_| "{}".to_string())
                                })
                            };

                            let response = Response::builder()
                                .status(StatusCode::from_u16(status as u16).unwrap_or(StatusCode::OK))
                                .header("content-type", "application/json")
                                .body(Body::from(body_str))
                                .map_err(|e| format!("Failed to build response: {}", e))?;

                            return Ok(HookResult::ShortCircuit(response));
                        }

                        let method = result_hash
                            .get(ruby.to_symbol("method"))
                            .and_then(|v| String::try_convert(v).ok())
                            .unwrap_or_else(|| "GET".to_string());
                        let path = result_hash
                            .get(ruby.to_symbol("path"))
                            .and_then(|v| String::try_convert(v).ok())
                            .unwrap_or_else(|| "/".to_string());

                        let req_builder = Request::builder().method(method.as_str()).uri(path);

                        let body = if let Some(body_val) = result_hash.get(ruby.to_symbol("body")) {
                            if body_val.is_nil() {
                                Body::empty()
                            } else {
                                let body_str = String::try_convert(body_val).unwrap_or_else(|_| {
                                    body_val
                                        .to_r_string()
                                        .map(|s| s.to_string().unwrap_or_default())
                                        .unwrap_or_default()
                                });
                                Body::from(body_str)
                            }
                        } else {
                            Body::empty()
                        };

                        let request = req_builder
                            .body(body)
                            .map_err(|e| format!("Failed to build request: {}", e))?;

                        Ok(HookResult::Continue(request))
                    } else {
                        Err(format!("Hook must return a Hash, got {}", unsafe {
                            result.classname()
                        }))
                    }
                });

            result.map_err(|e| format!("Hook '{}' task error: {}", name_for_error, e))
        })
    }

    fn execute_response<'a>(
        &'a self,
        resp: Response<Body>,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Response<Body>, Response<Body>>, String>> + Send + 'a>> {
        let func = self.func;
        let name = self.name.clone();
        let name_for_error = name.clone();

        Box::pin(async move {
            let (parts, body) = resp.into_parts();
            let body_bytes = axum::body::to_bytes(body, usize::MAX)
                .await
                .map_err(|e| format!("Failed to read response body: {}", e))?;

            let body_value: JsonValue = if body_bytes.is_empty() {
                JsonValue::Null
            } else {
                serde_json::from_slice(&body_bytes)
                    .unwrap_or_else(|_| JsonValue::String(String::from_utf8_lossy(&body_bytes).to_string()))
            };

            let result = magnus::Ruby::get()
                .map_err(|e| format!("Failed to get Ruby: {}", e))
                .and_then(|ruby| {
                    let response_hash = RHash::new();

                    response_hash
                        .aset(
                            ruby.to_symbol("status_code"),
                            ruby.integer_from_i64(parts.status.as_u16() as i64),
                        )
                        .map_err(|e| format!("Failed to set status_code: {}", e))?;

                    let headers_hash = RHash::new();
                    for (key, value) in parts.headers.iter() {
                        headers_hash
                            .aset(ruby.str_new(key.as_str()), ruby.str_new(value.to_str().unwrap_or("")))
                            .map_err(|e| format!("Failed to set header: {}", e))?;
                    }
                    response_hash
                        .aset(ruby.to_symbol("headers"), headers_hash)
                        .map_err(|e| format!("Failed to set headers: {}", e))?;

                    let body_str =
                        serde_json::to_string(&body_value).map_err(|e| format!("Failed to serialize body: {}", e))?;
                    response_hash
                        .aset(ruby.to_symbol("content"), ruby.str_new(&body_str))
                        .map_err(|e| format!("Failed to set content: {}", e))?;

                    let func_value = ruby.get_inner(func);
                    let result: Value = func_value
                        .funcall("call", (response_hash,))
                        .map_err(|e| format!("Hook '{}' call failed: {}", name, e))?;

                    if let Some(result_hash) = RHash::from_value(result) {
                        let status = result_hash
                            .get(ruby.to_symbol("status_code"))
                            .and_then(|v| i64::try_convert(v).ok())
                            .unwrap_or(200);

                        let content = result_hash
                            .get(ruby.to_symbol("content"))
                            .or_else(|| result_hash.get(ruby.to_symbol("body")))
                            .unwrap_or_else(|| ruby.qnil().as_value());

                        let body_str = if content.is_nil() {
                            "{}".to_string()
                        } else {
                            String::try_convert(content).unwrap_or_else(|_| {
                                content
                                    .to_r_string()
                                    .map(|s| s.to_string().unwrap_or_else(|_| "{}".to_string()))
                                    .unwrap_or_else(|_| "{}".to_string())
                            })
                        };

                        let mut response_builder =
                            Response::builder().status(StatusCode::from_u16(status as u16).unwrap_or(StatusCode::OK));

                        response_builder = response_builder.header("content-type", "application/json");

                        let response = response_builder
                            .body(Body::from(body_str))
                            .map_err(|e| format!("Failed to build response: {}", e))?;

                        Ok(HookResult::Continue(response))
                    } else {
                        Err(format!("Hook must return a Hash, got {}", unsafe {
                            result.classname()
                        }))
                    }
                });

            result.map_err(|e| format!("Hook '{}' task error: {}", name_for_error, e))
        })
    }
}

unsafe impl Send for RubyLifecycleHook {}
unsafe impl Sync for RubyLifecycleHook {}

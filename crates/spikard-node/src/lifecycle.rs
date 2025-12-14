//! Node.js lifecycle hooks implementation
//!
//! This module provides the bridge between JavaScript async functions and Rust's lifecycle hook system.
//! Uses napi-rs ThreadsafeFunction to call JavaScript functions from Rust async tasks.

use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
};
use napi::bindgen_prelude::Promise;
use napi::threadsafe_function::ThreadsafeFunction;
use serde_json::Value;
use spikard_http::lifecycle::adapter::error;
use spikard_http::lifecycle::{HookResult, LifecycleHook};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Node.js lifecycle hook wrapper
///
/// Wraps a JavaScript async function and makes it callable from Rust's lifecycle system.
/// Handles conversion between Rust HTTP types and JavaScript request/response objects.
pub struct NodeLifecycleHook {
    name: String,
    /// JavaScript async function via ThreadsafeFunction
    /// Input: String (JSON-serialized request/response)
    /// Return: Promise<String> (JSON-serialized request/response)
    func: Arc<ThreadsafeFunction<String, Promise<String>, String, napi::Status, false>>,
}

impl NodeLifecycleHook {
    /// Create a new Node lifecycle hook
    pub fn new(name: String, func: ThreadsafeFunction<String, Promise<String>, String, napi::Status, false>) -> Self {
        Self {
            name,
            func: Arc::new(func),
        }
    }
}

impl LifecycleHook<Request<Body>, Response<Body>> for NodeLifecycleHook {
    fn name(&self) -> &str {
        &self.name
    }

    fn execute_request<'a>(
        &'a self,
        req: Request<Body>,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>, Response<Body>>, String>> + Send + 'a>> {
        let func = Arc::clone(&self.func);
        let name = self.name.clone();

        Box::pin(async move {
            let (parts, body) = req.into_parts();
            let body_bytes = spikard_http::lifecycle::adapter::serial::extract_body(body).await?;

            let body_value = spikard_http::lifecycle::adapter::serial::parse_json(&body_bytes)?;

            let request_json = serde_json::json!({
                "method": parts.method.as_str(),
                "path": parts.uri.path(),
                "headers": parts.headers.iter()
                    .map(|(k, v)| (k.as_str(), v.to_str().unwrap_or("")))
                    .collect::<std::collections::HashMap<_, _>>(),
                "body": body_value,
            });

            let json_input = serde_json::to_string(&request_json).map_err(|e| error::serialize_failed("request", e))?;

            let json_output = func
                .call_async(json_input)
                .await
                .map_err(|e| error::call_failed(&name, e))?
                .await
                .map_err(|e| error::promise_failed(&name, e))?;

            let result_data: Value =
                serde_json::from_str(&json_output).map_err(|e| error::deserialize_failed("hook response", e))?;

            handle_node_request_hook_result(result_data)
        })
    }

    fn execute_response<'a>(
        &'a self,
        resp: Response<Body>,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Response<Body>, Response<Body>>, String>> + Send + 'a>> {
        let func = Arc::clone(&self.func);
        let name = self.name.clone();

        Box::pin(async move {
            let (parts, body) = resp.into_parts();
            let body_bytes = spikard_http::lifecycle::adapter::serial::extract_body(body).await?;

            let body_value = spikard_http::lifecycle::adapter::serial::parse_json(&body_bytes)?;

            let response_json = serde_json::json!({
                "status_code": parts.status.as_u16(),
                "headers": parts.headers.iter()
                    .map(|(k, v)| (k.as_str(), v.to_str().unwrap_or("")))
                    .collect::<std::collections::HashMap<_, _>>(),
                "content": body_value,
            });

            let json_input =
                serde_json::to_string(&response_json).map_err(|e| error::serialize_failed("response", e))?;

            let json_output = func
                .call_async(json_input)
                .await
                .map_err(|e| error::call_failed(&name, e))?
                .await
                .map_err(|e| error::promise_failed(&name, e))?;

            let result_data: Value =
                serde_json::from_str(&json_output).map_err(|e| error::deserialize_failed("hook response", e))?;

            handle_node_response_hook_result(result_data)
        })
    }
}

/// Handle a Node.js request hook result, converting to HookResult
fn handle_node_request_hook_result(result_data: Value) -> Result<HookResult<Request<Body>, Response<Body>>, String> {
    if let Some(status_code) = result_data.get("status_code").or_else(|| result_data.get("statusCode")) {
        let status = status_code.as_u64().unwrap_or(200) as u16;
        let content = result_data.get("content").or_else(|| result_data.get("body"));

        let body_str = if let Some(content) = content {
            serde_json::to_string(content).map_err(|e| error::serialize_failed("response content", e))?
        } else {
            "{}".to_string()
        };

        let response = Response::builder()
            .status(StatusCode::from_u16(status).unwrap_or(StatusCode::OK))
            .header("content-type", "application/json")
            .body(Body::from(body_str))
            .map_err(|e| error::build_failed("response", e))?;

        return Ok(HookResult::ShortCircuit(response));
    }

    let method = result_data.get("method").and_then(|v| v.as_str()).unwrap_or("GET");
    let path = result_data.get("path").and_then(|v| v.as_str()).unwrap_or("/");
    let mut req_builder = Request::builder().method(method).uri(path);

    if let Some(headers) = result_data.get("headers").and_then(|v| v.as_object()) {
        for (key, value) in headers {
            if let Some(value_str) = value.as_str() {
                req_builder = req_builder.header(key.as_str(), value_str);
            }
        }
    }

    let body = if let Some(body_value) = result_data.get("body") {
        if body_value.is_null() {
            Body::empty()
        } else {
            let body_str = serde_json::to_string(body_value).map_err(|e| error::serialize_failed("request body", e))?;
            Body::from(body_str)
        }
    } else {
        Body::empty()
    };

    let request = req_builder.body(body).map_err(|e| error::build_failed("request", e))?;

    Ok(HookResult::Continue(request))
}

/// Handle a Node.js response hook result, converting to HookResult
fn handle_node_response_hook_result(result_data: Value) -> Result<HookResult<Response<Body>, Response<Body>>, String> {
    let status = result_data
        .get("status_code")
        .or_else(|| result_data.get("statusCode"))
        .and_then(|v| v.as_u64())
        .unwrap_or(200) as u16;

    let content = result_data.get("content").or_else(|| result_data.get("body"));

    let body_str = if let Some(content) = content {
        serde_json::to_string(content).map_err(|e| error::serialize_failed("response content", e))?
    } else {
        "{}".to_string()
    };

    let mut response_builder = Response::builder().status(StatusCode::from_u16(status).unwrap_or(StatusCode::OK));

    if let Some(headers) = result_data.get("headers").and_then(|v| v.as_object()) {
        for (key, value) in headers {
            if let Some(value_str) = value.as_str() {
                response_builder = response_builder.header(key.as_str(), value_str);
            }
        }
    }

    response_builder = response_builder.header("content-type", "application/json");

    let response = response_builder
        .body(Body::from(body_str))
        .map_err(|e| error::build_failed("response", e))?;

    Ok(HookResult::Continue(response))
}

unsafe impl Send for NodeLifecycleHook {}
unsafe impl Sync for NodeLifecycleHook {}

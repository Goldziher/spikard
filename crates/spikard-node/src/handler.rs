//! Node.js handler implementation for spikard_http::Handler trait
//!
//! This module implements the Handler trait using napi-rs ThreadsafeFunction
//! to call JavaScript handlers from Rust's async HTTP server.

use crate::response::HandlerReturnValue;
use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
};
use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunction;
use serde_json::Value;
use spikard_http::{Handler, HandlerResult, RequestData};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Node.js handler wrapper that implements spikard_http::Handler
///
/// Uses ThreadsafeFunction to call JavaScript handlers from Rust threads.
/// The pattern follows kreuzberg's approach:
/// - Input: String (JSON-serialized request data)
/// - Return: Promise<HandlerReturnValue> (JSON-serialized response or streaming handle)
/// - CallJsBackArgs: Vec<String> (from build_callback)
pub struct NodeHandler {
    handler_name: String,
    handler_fn: Arc<ThreadsafeFunction<String, Promise<HandlerReturnValue>, Vec<String>, napi::Status, false>>,
}

#[derive(Clone)]
struct HandlerResponsePayload {
    status: u16,
    headers: std::collections::HashMap<String, String>,
    body: Option<serde_json::Value>,
}

fn interpret_handler_response(value: serde_json::Value) -> HandlerResponsePayload {
    if let serde_json::Value::Object(mut obj) = value {
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
                        serde_json::Value::String(s) => Some((k, s)),
                        serde_json::Value::Number(n) => Some((k, n.to_string())),
                        serde_json::Value::Bool(b) => Some((k, b.to_string())),
                        _ => None,
                    })
                    .collect::<std::collections::HashMap<String, String>>()
            })
            .unwrap_or_default();

        let body = if let Some(body_value) = obj.remove("body") {
            match body_value {
                serde_json::Value::Null => None,
                other => Some(other),
            }
        } else if obj.is_empty() {
            None
        } else {
            Some(serde_json::Value::Object(obj))
        };

        HandlerResponsePayload { status, headers, body }
    } else {
        HandlerResponsePayload {
            status: 200,
            headers: std::collections::HashMap::new(),
            body: Some(value),
        }
    }
}

unsafe impl Send for NodeHandler {}
unsafe impl Sync for NodeHandler {}

impl NodeHandler {
    /// Create a new Node handler wrapper with a JavaScript function
    pub fn new(
        handler_name: String,
        handler_fn: ThreadsafeFunction<String, Promise<HandlerReturnValue>, Vec<String>, napi::Status, false>,
    ) -> Self {
        Self {
            handler_name,
            handler_fn: Arc::new(handler_fn),
        }
    }
}

impl Handler for NodeHandler {
    fn call(
        &self,
        _req: Request<Body>,
        request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(async move {
            #[cfg(feature = "di")]
            let dependencies = if let Some(resolved) = &request_data.dependencies {
                let keys: Vec<String> = resolved.keys();

                let mut deps_map = serde_json::Map::new();
                for key in &keys {
                    if let Some(value_json) = resolved.get::<String>(key)
                        && let Ok(parsed) = serde_json::from_str::<Value>(&value_json)
                    {
                        deps_map.insert(key.to_string(), parsed);
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
                .map(|bytes| bytes.iter().copied().collect::<Vec<u8>>());

            let query: std::collections::HashMap<String, String> = request_data
                .raw_query_params
                .iter()
                .filter_map(|(k, values)| values.first().map(|value| (k.clone(), value.clone())))
                .collect();

            let request_json = serde_json::json!({
                "path": request_data.path,
                "method": request_data.method,
                "params": &*request_data.path_params,
                "query": query,
                "headers": &*request_data.headers,
                "cookies": &*request_data.cookies,
                "body": body_bytes,
                "dependencies": dependencies,
            });

            let json_input = serde_json::to_string(&request_json).map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to serialize request: {}", e),
                )
            })?;

            let handler_output = self
                .handler_fn
                .call_async(json_input)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Handler '{}' call failed: {}", self.handler_name, e),
                    )
                })?
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Handler '{}' promise failed: {}", self.handler_name, e),
                    )
                })?;

            if let HandlerReturnValue::Streaming(streaming) = handler_output {
                let response = streaming.into_handler_response().map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to create streaming response: {}", e),
                    )
                })?;
                return Ok(response.into_response());
            }

            let json_body = match handler_output {
                HandlerReturnValue::Json(json) => json,
                HandlerReturnValue::Streaming(_) => unreachable!(),
            };

            let response_data: Value = serde_json::from_str(&json_body).map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to parse handler response: {}", e),
                )
            })?;

            let mut handler_response = interpret_handler_response(response_data);

            let mut response_builder =
                Response::builder().status(StatusCode::from_u16(handler_response.status).unwrap_or(StatusCode::OK));

            for (key, value) in handler_response.headers.into_iter() {
                response_builder = response_builder.header(key, value);
            }

            match handler_response.body.take() {
                Some(body_value) => {
                    let body_str = serde_json::to_string(&body_value).map_err(|e| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Failed to serialize response: {}", e),
                        )
                    })?;

                    response_builder.body(Body::from(body_str)).map_err(|e| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Failed to build response: {}", e),
                        )
                    })
                }
                None => response_builder.body(Body::empty()).map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to build empty response: {}", e),
                    )
                }),
            }
        })
    }
}

impl Clone for NodeHandler {
    fn clone(&self) -> Self {
        Self {
            handler_name: self.handler_name.clone(),
            handler_fn: Arc::clone(&self.handler_fn),
        }
    }
}

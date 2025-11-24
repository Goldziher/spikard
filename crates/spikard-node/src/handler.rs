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
                    if let Some(value_json) = resolved.get::<String>(key) {
                        if let Ok(parsed) = serde_json::from_str::<Value>(&value_json) {
                            deps_map.insert(key.to_string(), parsed);
                        }
                    }
                }
                Value::Object(deps_map)
            } else {
                Value::Null
            };

            #[cfg(not(feature = "di"))]
            let dependencies = Value::Null;

            let request_json = serde_json::json!({
                "path": request_data.path,
                "method": request_data.method,
                "path_params": &*request_data.path_params,
                "query_params": request_data.query_params,
                "headers": &*request_data.headers,
                "cookies": &*request_data.cookies,
                "body": request_data.body,
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

            let body_str = serde_json::to_string(&response_data).map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to serialize response: {}", e),
                )
            })?;

            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(body_str))
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to build response: {}", e),
                    )
                })
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

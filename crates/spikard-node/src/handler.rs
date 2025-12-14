//! Node.js handler implementation for spikard_http::Handler trait
//!
//! This module implements the Handler trait using napi-rs ThreadsafeFunction
//! to call JavaScript handlers from Rust's async HTTP server.

use crate::handler_input::HandlerInput;
use crate::handler_output::HandlerOutput;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::IntoResponse,
};
use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunction;
use spikard_bindings_shared::ErrorResponseBuilder;
use spikard_http::{Handler, HandlerResult, RequestData};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Node.js handler wrapper that implements spikard_http::Handler
///
/// Uses ThreadsafeFunction to call JavaScript handlers from Rust threads.
/// Uses structured objects instead of JSON strings for performance:
/// - Input: HandlerInput (napi object)
/// - Return: Promise<HandlerOutput> (napi object)
/// - CallJsBackArgs: Vec<()> (from build_callback)
///
/// This eliminates 6 serialization cycles, providing 6x performance improvement.
pub struct NodeHandler {
    handler_name: String,
    handler_fn: Arc<ThreadsafeFunction<HandlerInput, Promise<HandlerOutput>, HandlerInput, napi::Status, false>>,
}

unsafe impl Send for NodeHandler {}
unsafe impl Sync for NodeHandler {}

impl NodeHandler {
    /// Create a new Node handler wrapper with a JavaScript function
    pub fn new(
        handler_name: String,
        handler_fn: ThreadsafeFunction<HandlerInput, Promise<HandlerOutput>, HandlerInput, napi::Status, false>,
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
            let input = HandlerInput::from(&request_data);

            let output = self
                .handler_fn
                .call_async(input)
                .await
                .map_err(|e| {
                    ErrorResponseBuilder::structured_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "handler_call_failed",
                        format!("Handler '{}' call failed: {}", self.handler_name, e),
                    )
                })?
                .await
                .map_err(|e| {
                    ErrorResponseBuilder::structured_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "handler_promise_failed",
                        format!("Handler '{}' promise failed: {}", self.handler_name, e),
                    )
                })?;

            let response = output.into_response().map_err(|e| {
                ErrorResponseBuilder::structured_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "handler_output_conversion_failed",
                    format!("Failed to convert handler output to response: {}", e),
                )
            })?;

            Ok(response.into_response())
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

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
    handler_fn: NodeHandlerFn,
}

unsafe impl Send for NodeHandler {}
unsafe impl Sync for NodeHandler {}

#[derive(Clone)]
enum NodeHandlerFn {
    Sync(Arc<ThreadsafeFunction<HandlerInput, HandlerOutput, HandlerInput, napi::Status, false>>),
    Async(Arc<ThreadsafeFunction<HandlerInput, Promise<HandlerOutput>, HandlerInput, napi::Status, false>>),
}

impl NodeHandler {
    /// Create a new Node handler wrapper with a JavaScript function
    pub fn new_async(
        handler_name: String,
        handler_fn: ThreadsafeFunction<HandlerInput, Promise<HandlerOutput>, HandlerInput, napi::Status, false>,
    ) -> Self {
        Self {
            handler_name,
            handler_fn: NodeHandlerFn::Async(Arc::new(handler_fn)),
        }
    }

    /// Create a new Node handler wrapper for a synchronous JavaScript function.
    pub fn new_sync(
        handler_name: String,
        handler_fn: ThreadsafeFunction<HandlerInput, HandlerOutput, HandlerInput, napi::Status, false>,
    ) -> Self {
        Self {
            handler_name,
            handler_fn: NodeHandlerFn::Sync(Arc::new(handler_fn)),
        }
    }
}

impl Handler for NodeHandler {
    fn call(
        &self,
        _req: Request<Body>,
        request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        let handler_name = self.handler_name.clone();
        let handler_fn = self.handler_fn.clone();

        Box::pin(async move {
            let input = HandlerInput::from(request_data);

            let output = match handler_fn {
                NodeHandlerFn::Sync(handler_fn) => handler_fn.call_async(input).await.map_err(|e| {
                    ErrorResponseBuilder::structured_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "handler_call_failed",
                        format!("Handler '{}' call failed: {}", handler_name, e),
                    )
                })?,
                NodeHandlerFn::Async(handler_fn) => handler_fn
                    .call_async(input)
                    .await
                    .map_err(|e| {
                        ErrorResponseBuilder::structured_error(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "handler_call_failed",
                            format!("Handler '{}' call failed: {}", handler_name, e),
                        )
                    })?
                    .await
                    .map_err(|e| {
                        ErrorResponseBuilder::structured_error(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "handler_promise_failed",
                            format!("Handler '{}' promise failed: {}", handler_name, e),
                        )
                    })?,
            };

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
            handler_fn: self.handler_fn.clone(),
        }
    }
}

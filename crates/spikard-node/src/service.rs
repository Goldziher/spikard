#![allow(clippy::too_many_arguments, clippy::unused_async)]

use napi::bindgen_prelude::*;
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};
use std::sync::Arc;

/// Generated NAPI bridge for the `Handler` contract.
///
/// Wraps a JavaScript callable (async) via ThreadsafeFunction
/// so it can be used as `Arc<dyn Handler>` from Rust async code.
pub struct HandlerBridge {
    handler_fn: ThreadsafeFunction<spikard::RequestData, spikard::Response, (), napi::Error>,
}

impl HandlerBridge {
    /// Create a bridge from a JavaScript callable.
    pub fn new(handler_fn: ThreadsafeFunction<spikard::RequestData, spikard::Response, (), napi::Error>) -> Self {
        Self { handler_fn }
    }
}

// SAFETY: ThreadsafeFunction is Send+Sync. We call it only from async contexts
// where the NAPI env is valid (within the async task spawned by call_async).
unsafe impl Send for HandlerBridge {}
unsafe impl Sync for HandlerBridge {}

impl spikard::Handler for HandlerBridge {
    fn call(
        &self,
        _request: spikard::Request<spikard::Body>,
        request_data: spikard::RequestData,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard::HandlerResult> + Send + '_>> {
        Box::pin(async move {
            // Call the ThreadsafeFunction and await the Promise
            let outcome: Result<spikard::Response, Box<dyn std::error::Error + Send + Sync>> = async move {
                self.handler_fn
                    .call_async::<spikard::Response>(request_data)
                    .await
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }
            .await;

            spikard::handler_result_from_response(outcome)
        })
    }
}

/// Drive `spikard::App::run` from JavaScript.
///
/// Each entry in `registrations` is a `[method_name, metadata, callback]` triple
/// produced by the TypeScript service class.
#[napi]
pub async fn app_run(
    registrations: Vec<(
        String,
        Vec<JsUnknown>,
        ThreadsafeFunction<napi::JsUnknown, napi::JsUnknown, (), napi::Error>,
    )>,
) -> napi::Result<()> {
    let mut owner = spikard::App::new();

    for (method_name, _metadata, handler_fn) in registrations {
        match method_name.as_str() {
            "route" => {
                let bridge = HandlerBridge::new(handler_fn);
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let builder: spikard::RouteBuilder = {
                    let val = _metadata.get(0).ok_or_else(|| {
                        napi::Error::new(napi::Status::InvalidArg, "missing metadata parameter at index 0")
                    })?;
                    serde_json::from_value(
                        serde_json::to_value(&val)
                            .map_err(|e| napi::Error::from_reason(format!("metadata serialization failed: {}", e)))?,
                    )
                    .map_err(|e| napi::Error::from_reason(format!("metadata deserialization failed: {}", e)))?
                };
                owner
                    .route(builder, handler)
                    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
            }
            _ => {}
        }
    }

    owner
        .run()
        .await
        .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
    Ok(())
}

/// Drive `spikard::App::into_router` from JavaScript.
///
/// Each entry in `registrations` is a `[method_name, metadata, callback]` triple
/// produced by the TypeScript service class.
#[napi]
pub async fn app_into_router(
    registrations: Vec<(
        String,
        Vec<JsUnknown>,
        ThreadsafeFunction<napi::JsUnknown, napi::JsUnknown, (), napi::Error>,
    )>,
) -> napi::Result<()> {
    let mut owner = spikard::App::new();

    for (method_name, _metadata, handler_fn) in registrations {
        match method_name.as_str() {
            "route" => {
                let bridge = HandlerBridge::new(handler_fn);
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let builder: spikard::RouteBuilder = {
                    let val = _metadata.get(0).ok_or_else(|| {
                        napi::Error::new(napi::Status::InvalidArg, "missing metadata parameter at index 0")
                    })?;
                    serde_json::from_value(
                        serde_json::to_value(&val)
                            .map_err(|e| napi::Error::from_reason(format!("metadata serialization failed: {}", e)))?,
                    )
                    .map_err(|e| napi::Error::from_reason(format!("metadata deserialization failed: {}", e)))?
                };
                owner
                    .route(builder, handler)
                    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
            }
            _ => {}
        }
    }

    owner
        .into_router()
        .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
    Ok(())
}

/// Register a handler via the `get` variant shortcut.
///
/// Register a GET route at the given path.
#[napi]
pub fn get(
    &mut self,
    path: String,
    handler: napi::bindgen_prelude::ThreadsafeFunction<napi::JsUnknown, napi::JsUnknown, (), napi::Error>,
) -> napi::Result<()> {
    let builder = spikard::RouteBuilder::new(spikard::Method::Get, path);
    let bridge = HandlerBridge::new(handler);
    let handler_arc: std::sync::Arc<dyn spikard::Handler> = std::sync::Arc::new(bridge);
    self.route(builder, path, handler_arc)
        .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
    Ok(())
}

/// Register a handler via the `post` variant shortcut.
///
/// Register a POST route at the given path.
#[napi]
pub fn post(
    &mut self,
    path: String,
    handler: napi::bindgen_prelude::ThreadsafeFunction<napi::JsUnknown, napi::JsUnknown, (), napi::Error>,
) -> napi::Result<()> {
    let builder = spikard::RouteBuilder::new(spikard::Method::Post, path);
    let bridge = HandlerBridge::new(handler);
    let handler_arc: std::sync::Arc<dyn spikard::Handler> = std::sync::Arc::new(bridge);
    self.route(builder, path, handler_arc)
        .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
    Ok(())
}

/// Register a handler via the `put` variant shortcut.
///
/// Register a PUT route at the given path.
#[napi]
pub fn put(
    &mut self,
    path: String,
    handler: napi::bindgen_prelude::ThreadsafeFunction<napi::JsUnknown, napi::JsUnknown, (), napi::Error>,
) -> napi::Result<()> {
    let builder = spikard::RouteBuilder::new(spikard::Method::Put, path);
    let bridge = HandlerBridge::new(handler);
    let handler_arc: std::sync::Arc<dyn spikard::Handler> = std::sync::Arc::new(bridge);
    self.route(builder, path, handler_arc)
        .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
    Ok(())
}

/// Register a handler via the `patch` variant shortcut.
///
/// Register a PATCH route at the given path.
#[napi]
pub fn patch(
    &mut self,
    path: String,
    handler: napi::bindgen_prelude::ThreadsafeFunction<napi::JsUnknown, napi::JsUnknown, (), napi::Error>,
) -> napi::Result<()> {
    let builder = spikard::RouteBuilder::new(spikard::Method::Patch, path);
    let bridge = HandlerBridge::new(handler);
    let handler_arc: std::sync::Arc<dyn spikard::Handler> = std::sync::Arc::new(bridge);
    self.route(builder, path, handler_arc)
        .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
    Ok(())
}

/// Register a handler via the `delete` variant shortcut.
///
/// Register a DELETE route at the given path.
#[napi]
pub fn delete(
    &mut self,
    path: String,
    handler: napi::bindgen_prelude::ThreadsafeFunction<napi::JsUnknown, napi::JsUnknown, (), napi::Error>,
) -> napi::Result<()> {
    let builder = spikard::RouteBuilder::new(spikard::Method::Delete, path);
    let bridge = HandlerBridge::new(handler);
    let handler_arc: std::sync::Arc<dyn spikard::Handler> = std::sync::Arc::new(bridge);
    self.route(builder, path, handler_arc)
        .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
    Ok(())
}

/// Register a handler via the `head` variant shortcut.
///
/// Register a HEAD route at the given path.
#[napi]
pub fn head(
    &mut self,
    path: String,
    handler: napi::bindgen_prelude::ThreadsafeFunction<napi::JsUnknown, napi::JsUnknown, (), napi::Error>,
) -> napi::Result<()> {
    let builder = spikard::RouteBuilder::new(spikard::Method::Head, path);
    let bridge = HandlerBridge::new(handler);
    let handler_arc: std::sync::Arc<dyn spikard::Handler> = std::sync::Arc::new(bridge);
    self.route(builder, path, handler_arc)
        .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
    Ok(())
}

/// Register a handler via the `options` variant shortcut.
///
/// Register an OPTIONS route at the given path.
#[napi]
pub fn options(
    &mut self,
    path: String,
    handler: napi::bindgen_prelude::ThreadsafeFunction<napi::JsUnknown, napi::JsUnknown, (), napi::Error>,
) -> napi::Result<()> {
    let builder = spikard::RouteBuilder::new(spikard::Method::Options, path);
    let bridge = HandlerBridge::new(handler);
    let handler_arc: std::sync::Arc<dyn spikard::Handler> = std::sync::Arc::new(bridge);
    self.route(builder, path, handler_arc)
        .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
    Ok(())
}

/// Register a handler via the `connect` variant shortcut.
///
/// Register a CONNECT route at the given path.
#[napi]
pub fn connect(
    &mut self,
    path: String,
    handler: napi::bindgen_prelude::ThreadsafeFunction<napi::JsUnknown, napi::JsUnknown, (), napi::Error>,
) -> napi::Result<()> {
    let builder = spikard::RouteBuilder::new(spikard::Method::Connect, path);
    let bridge = HandlerBridge::new(handler);
    let handler_arc: std::sync::Arc<dyn spikard::Handler> = std::sync::Arc::new(bridge);
    self.route(builder, path, handler_arc)
        .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
    Ok(())
}

/// Register a handler via the `trace` variant shortcut.
///
/// Register a TRACE route at the given path.
#[napi]
pub fn trace(
    &mut self,
    path: String,
    handler: napi::bindgen_prelude::ThreadsafeFunction<napi::JsUnknown, napi::JsUnknown, (), napi::Error>,
) -> napi::Result<()> {
    let builder = spikard::RouteBuilder::new(spikard::Method::Trace, path);
    let bridge = HandlerBridge::new(handler);
    let handler_arc: std::sync::Arc<dyn spikard::Handler> = std::sync::Arc::new(bridge);
    self.route(builder, path, handler_arc)
        .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
    Ok(())
}

#![allow(clippy::too_many_arguments, clippy::unused_async)]

use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunction;
use napi_derive::napi;

/// Generated NAPI bridge for the `Handler` contract.
///
/// Wraps a JavaScript callable (async) via ThreadsafeFunction
/// so it can be used as `Arc<dyn Handler>` from Rust async code.
pub struct HandlerBridge {
    handler_fn: ThreadsafeFunction<serde_json::Value, serde_json::Value>,
}

impl HandlerBridge {
    /// Create a bridge from a JavaScript callable.
    pub fn new(handler_fn: ThreadsafeFunction<serde_json::Value, serde_json::Value>) -> Self {
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
            // Serialize request to JSON and call the ThreadsafeFunction
            let outcome: std::result::Result<spikard::Response, Box<dyn std::error::Error + Send + Sync>> =
                async move {
                    let req_json = serde_json::to_value(&request_data)
                        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                    let resp_json = self
                        .handler_fn
                        .call_async(Ok(req_json))
                        .await
                        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                    serde_json::from_value(resp_json)
                        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
                }
                .await;

            spikard::handler_result_from_response(outcome)
        })
    }
}

/// Drive `spikard::App::into_router` from JavaScript.
///
/// Each entry in `registrations` is a `[method_name, metadata, callback]` triple
/// produced by the TypeScript service class.
#[napi]
pub async fn app_into_router(
    registrations: Vec<(
        String,
        Vec<serde_json::Value>,
        ThreadsafeFunction<serde_json::Value, serde_json::Value>,
    )>,
) -> napi::Result<()> {
    let mut owner = spikard::App::new();

    for (method_name, _metadata, handler_fn) in registrations {
        match method_name.as_str() {
            _ => {}
        }
    }

    owner
        .into_router()
        .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
    Ok(())
}

use crate::JsApp;

#[napi]
impl JsApp {
    /// Register a handler via the `get` variant shortcut.
    ///
    /// Register a GET route at the given path.
    #[napi]
    pub fn get(
        &self,
        path: String,
        handler: ThreadsafeFunction<serde_json::Value, serde_json::Value>,
    ) -> napi::Result<()> {
        let builder = spikard::RouteBuilder::new(spikard::Method::Get, path);
        let mut routes = self.routes.lock().expect("routes mutex poisoned");
        routes.push((builder, handler));
        Ok(())
    }

    /// Register a handler via the `post` variant shortcut.
    ///
    /// Register a POST route at the given path.
    #[napi]
    pub fn post(
        &self,
        path: String,
        handler: ThreadsafeFunction<serde_json::Value, serde_json::Value>,
    ) -> napi::Result<()> {
        let builder = spikard::RouteBuilder::new(spikard::Method::Post, path);
        let mut routes = self.routes.lock().expect("routes mutex poisoned");
        routes.push((builder, handler));
        Ok(())
    }

    /// Register a handler via the `put` variant shortcut.
    ///
    /// Register a PUT route at the given path.
    #[napi]
    pub fn put(
        &self,
        path: String,
        handler: ThreadsafeFunction<serde_json::Value, serde_json::Value>,
    ) -> napi::Result<()> {
        let builder = spikard::RouteBuilder::new(spikard::Method::Put, path);
        let mut routes = self.routes.lock().expect("routes mutex poisoned");
        routes.push((builder, handler));
        Ok(())
    }

    /// Register a handler via the `patch` variant shortcut.
    ///
    /// Register a PATCH route at the given path.
    #[napi]
    pub fn patch(
        &self,
        path: String,
        handler: ThreadsafeFunction<serde_json::Value, serde_json::Value>,
    ) -> napi::Result<()> {
        let builder = spikard::RouteBuilder::new(spikard::Method::Patch, path);
        let mut routes = self.routes.lock().expect("routes mutex poisoned");
        routes.push((builder, handler));
        Ok(())
    }

    /// Register a handler via the `delete` variant shortcut.
    ///
    /// Register a DELETE route at the given path.
    #[napi]
    pub fn delete(
        &self,
        path: String,
        handler: ThreadsafeFunction<serde_json::Value, serde_json::Value>,
    ) -> napi::Result<()> {
        let builder = spikard::RouteBuilder::new(spikard::Method::Delete, path);
        let mut routes = self.routes.lock().expect("routes mutex poisoned");
        routes.push((builder, handler));
        Ok(())
    }

    /// Register a handler via the `head` variant shortcut.
    ///
    /// Register a HEAD route at the given path.
    #[napi]
    pub fn head(
        &self,
        path: String,
        handler: ThreadsafeFunction<serde_json::Value, serde_json::Value>,
    ) -> napi::Result<()> {
        let builder = spikard::RouteBuilder::new(spikard::Method::Head, path);
        let mut routes = self.routes.lock().expect("routes mutex poisoned");
        routes.push((builder, handler));
        Ok(())
    }

    /// Register a handler via the `options` variant shortcut.
    ///
    /// Register an OPTIONS route at the given path.
    #[napi]
    pub fn options(
        &self,
        path: String,
        handler: ThreadsafeFunction<serde_json::Value, serde_json::Value>,
    ) -> napi::Result<()> {
        let builder = spikard::RouteBuilder::new(spikard::Method::Options, path);
        let mut routes = self.routes.lock().expect("routes mutex poisoned");
        routes.push((builder, handler));
        Ok(())
    }

    /// Register a handler via the `connect` variant shortcut.
    ///
    /// Register a CONNECT route at the given path.
    #[napi]
    pub fn connect(
        &self,
        path: String,
        handler: ThreadsafeFunction<serde_json::Value, serde_json::Value>,
    ) -> napi::Result<()> {
        let builder = spikard::RouteBuilder::new(spikard::Method::Connect, path);
        let mut routes = self.routes.lock().expect("routes mutex poisoned");
        routes.push((builder, handler));
        Ok(())
    }

    /// Register a handler via the `trace` variant shortcut.
    ///
    /// Register a TRACE route at the given path.
    #[napi]
    pub fn trace(
        &self,
        path: String,
        handler: ThreadsafeFunction<serde_json::Value, serde_json::Value>,
    ) -> napi::Result<()> {
        let builder = spikard::RouteBuilder::new(spikard::Method::Trace, path);
        let mut routes = self.routes.lock().expect("routes mutex poisoned");
        routes.push((builder, handler));
        Ok(())
    }
}

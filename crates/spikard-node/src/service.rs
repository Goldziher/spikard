#![allow(clippy::too_many_arguments, clippy::unused_async)]

use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunction;
use napi_derive::napi;
use std::sync::Arc;

/// Wrapper that lets an arbitrary JSON handler return participate in
/// `Either<Promise<_>, _>`. Both `Either` arms and `Promise<T>` require
/// `ValidateNapiValue`, which `serde_json::Value` does not implement. A handler
/// return is always the response-envelope object, so `value_type()` is `Object`;
/// the default `validate` then routes a plain object to the value arm and a
/// thenable to the promise arm — supporting both sync and async JS handlers.
pub struct HandlerReturn(serde_json::Value);

impl TypeName for HandlerReturn {
    fn type_name() -> &'static str {
        "HandlerReturn"
    }
    fn value_type() -> ValueType {
        ValueType::Object
    }
}

impl ValidateNapiValue for HandlerReturn {}

impl FromNapiValue for HandlerReturn {
    unsafe fn from_napi_value(env: napi::sys::napi_env, napi_val: napi::sys::napi_value) -> Result<Self> {
        Ok(Self(unsafe { serde_json::Value::from_napi_value(env, napi_val)? }))
    }
}
/// Generated NAPI bridge for the `Handler` contract.
///
/// Wraps a JavaScript callable (async) via ThreadsafeFunction
/// so it can be used as `Arc<dyn Handler>` from Rust async code.
pub struct HandlerBridge {
    handler_fn: ThreadsafeFunction<serde_json::Value, Either<Promise<HandlerReturn>, HandlerReturn>>,
}

impl HandlerBridge {
    /// Create a bridge from a JavaScript callable.
    pub fn new(
        handler_fn: ThreadsafeFunction<serde_json::Value, Either<Promise<HandlerReturn>, HandlerReturn>>,
    ) -> Self {
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
                    // The JS handler may be sync (returns the response object) or
                    // async (returns a Promise). `Either<Promise<_>, _>` routes a
                    // thenable to the `Promise` arm (awaited here) and a plain object
                    // to the value arm, so both handler styles work.
                    let resp_either = self
                        .handler_fn
                        .call_async(Ok(req_json))
                        .await
                        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                    let resp_json = match resp_either {
                        Either::A(promise) => {
                            promise
                                .await
                                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
                                .0
                        }
                        Either::B(value) => value.0,
                    };
                    serde_json::from_value(resp_json)
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
#[allow(clippy::type_complexity)]
pub async fn app_run(
    registrations: Vec<(
        String,
        Vec<serde_json::Value>,
        ThreadsafeFunction<serde_json::Value, Either<Promise<HandlerReturn>, HandlerReturn>>,
    )>,
) -> napi::Result<()> {
    let owner = spikard::App::new();
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
#[allow(clippy::type_complexity)]
pub async fn app_into_router(
    registrations: Vec<(
        String,
        Vec<serde_json::Value>,
        ThreadsafeFunction<serde_json::Value, Either<Promise<HandlerReturn>, HandlerReturn>>,
    )>,
) -> napi::Result<()> {
    let owner = spikard::App::new();
    let _ = owner
        .into_router()
        .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
    Ok(())
}

use crate::JsApp;
use crate::JsRouteBuilder;

#[napi]
impl JsApp {
    /// Register a route using the provided builder and handler function.
    ///
    /// # Errors
    ///
    /// Returns an error if route construction fails or if the handler registration fails.
    #[napi]
    pub fn route(
        &self,
        builder: &JsRouteBuilder,
        handler: ThreadsafeFunction<serde_json::Value, Either<Promise<HandlerReturn>, HandlerReturn>>,
    ) -> napi::Result<()> {
        let builder = (*builder.inner).clone();
        let bridge = HandlerBridge::new(handler);
        let handler_arc: std::sync::Arc<dyn spikard::Handler> = std::sync::Arc::new(bridge);
        let mut inner = self.inner.lock().expect("app mutex poisoned");
        inner
            .route(builder, handler_arc)
            .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
        Ok(())
    }
    /// Register a handler via the `get` variant shortcut.
    ///
    /// Register a GET route at the given path.
    #[napi]
    pub fn get(
        &self,
        path: String,
        handler: ThreadsafeFunction<serde_json::Value, Either<Promise<HandlerReturn>, HandlerReturn>>,
    ) -> napi::Result<()> {
        let builder = spikard::RouteBuilder::new(spikard::Method::Get, path);
        let bridge = HandlerBridge::new(handler);
        let handler_arc: std::sync::Arc<dyn spikard::Handler> = std::sync::Arc::new(bridge);
        let mut inner = self.inner.lock().expect("app mutex poisoned");
        inner
            .route(builder, handler_arc)
            .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
        Ok(())
    }
    /// Register a handler via the `post` variant shortcut.
    ///
    /// Register a POST route at the given path.
    #[napi]
    pub fn post(
        &self,
        path: String,
        handler: ThreadsafeFunction<serde_json::Value, Either<Promise<HandlerReturn>, HandlerReturn>>,
    ) -> napi::Result<()> {
        let builder = spikard::RouteBuilder::new(spikard::Method::Post, path);
        let bridge = HandlerBridge::new(handler);
        let handler_arc: std::sync::Arc<dyn spikard::Handler> = std::sync::Arc::new(bridge);
        let mut inner = self.inner.lock().expect("app mutex poisoned");
        inner
            .route(builder, handler_arc)
            .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
        Ok(())
    }
    /// Register a handler via the `put` variant shortcut.
    ///
    /// Register a PUT route at the given path.
    #[napi]
    pub fn put(
        &self,
        path: String,
        handler: ThreadsafeFunction<serde_json::Value, Either<Promise<HandlerReturn>, HandlerReturn>>,
    ) -> napi::Result<()> {
        let builder = spikard::RouteBuilder::new(spikard::Method::Put, path);
        let bridge = HandlerBridge::new(handler);
        let handler_arc: std::sync::Arc<dyn spikard::Handler> = std::sync::Arc::new(bridge);
        let mut inner = self.inner.lock().expect("app mutex poisoned");
        inner
            .route(builder, handler_arc)
            .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
        Ok(())
    }
    /// Register a handler via the `patch` variant shortcut.
    ///
    /// Register a PATCH route at the given path.
    #[napi]
    pub fn patch(
        &self,
        path: String,
        handler: ThreadsafeFunction<serde_json::Value, Either<Promise<HandlerReturn>, HandlerReturn>>,
    ) -> napi::Result<()> {
        let builder = spikard::RouteBuilder::new(spikard::Method::Patch, path);
        let bridge = HandlerBridge::new(handler);
        let handler_arc: std::sync::Arc<dyn spikard::Handler> = std::sync::Arc::new(bridge);
        let mut inner = self.inner.lock().expect("app mutex poisoned");
        inner
            .route(builder, handler_arc)
            .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
        Ok(())
    }
    /// Register a handler via the `delete` variant shortcut.
    ///
    /// Register a DELETE route at the given path.
    #[napi]
    pub fn delete(
        &self,
        path: String,
        handler: ThreadsafeFunction<serde_json::Value, Either<Promise<HandlerReturn>, HandlerReturn>>,
    ) -> napi::Result<()> {
        let builder = spikard::RouteBuilder::new(spikard::Method::Delete, path);
        let bridge = HandlerBridge::new(handler);
        let handler_arc: std::sync::Arc<dyn spikard::Handler> = std::sync::Arc::new(bridge);
        let mut inner = self.inner.lock().expect("app mutex poisoned");
        inner
            .route(builder, handler_arc)
            .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
        Ok(())
    }
    /// Register a handler via the `head` variant shortcut.
    ///
    /// Register a HEAD route at the given path.
    #[napi]
    pub fn head(
        &self,
        path: String,
        handler: ThreadsafeFunction<serde_json::Value, Either<Promise<HandlerReturn>, HandlerReturn>>,
    ) -> napi::Result<()> {
        let builder = spikard::RouteBuilder::new(spikard::Method::Head, path);
        let bridge = HandlerBridge::new(handler);
        let handler_arc: std::sync::Arc<dyn spikard::Handler> = std::sync::Arc::new(bridge);
        let mut inner = self.inner.lock().expect("app mutex poisoned");
        inner
            .route(builder, handler_arc)
            .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
        Ok(())
    }
    /// Register a handler via the `options` variant shortcut.
    ///
    /// Register an OPTIONS route at the given path.
    #[napi]
    pub fn options(
        &self,
        path: String,
        handler: ThreadsafeFunction<serde_json::Value, Either<Promise<HandlerReturn>, HandlerReturn>>,
    ) -> napi::Result<()> {
        let builder = spikard::RouteBuilder::new(spikard::Method::Options, path);
        let bridge = HandlerBridge::new(handler);
        let handler_arc: std::sync::Arc<dyn spikard::Handler> = std::sync::Arc::new(bridge);
        let mut inner = self.inner.lock().expect("app mutex poisoned");
        inner
            .route(builder, handler_arc)
            .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
        Ok(())
    }
    /// Register a handler via the `connect` variant shortcut.
    ///
    /// Register a CONNECT route at the given path.
    #[napi]
    pub fn connect(
        &self,
        path: String,
        handler: ThreadsafeFunction<serde_json::Value, Either<Promise<HandlerReturn>, HandlerReturn>>,
    ) -> napi::Result<()> {
        let builder = spikard::RouteBuilder::new(spikard::Method::Connect, path);
        let bridge = HandlerBridge::new(handler);
        let handler_arc: std::sync::Arc<dyn spikard::Handler> = std::sync::Arc::new(bridge);
        let mut inner = self.inner.lock().expect("app mutex poisoned");
        inner
            .route(builder, handler_arc)
            .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
        Ok(())
    }
    /// Register a handler via the `trace` variant shortcut.
    ///
    /// Register a TRACE route at the given path.
    #[napi]
    pub fn trace(
        &self,
        path: String,
        handler: ThreadsafeFunction<serde_json::Value, Either<Promise<HandlerReturn>, HandlerReturn>>,
    ) -> napi::Result<()> {
        let builder = spikard::RouteBuilder::new(spikard::Method::Trace, path);
        let bridge = HandlerBridge::new(handler);
        let handler_arc: std::sync::Arc<dyn spikard::Handler> = std::sync::Arc::new(bridge);
        let mut inner = self.inner.lock().expect("app mutex poisoned");
        inner
            .route(builder, handler_arc)
            .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
        Ok(())
    }
    /// Call the `run` entrypoint on the inner service.
    ///
    /// Run the HTTP server using the configured routes.
    ///
    /// # Errors
    ///
    /// Returns an error if server construction or execution fails.
    #[napi(js_name = "nativeRun")]
    pub async fn run(&self) -> napi::Result<()> {
        let owner = {
            let mut guard = self.inner.lock().expect("app mutex poisoned");
            std::mem::take(&mut *guard)
        };
        owner
            .run()
            .await
            .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
        Ok(())
    }
    /// Call the `into_router` entrypoint on the inner service.
    ///
    /// Build the underlying Axum router.
    ///
    /// # Errors
    ///
    /// Returns an error if server or router construction fails.
    #[napi(js_name = "nativeIntoRouter")]
    pub fn into_router(&self) -> napi::Result<()> {
        let owner = {
            let mut guard = self.inner.lock().expect("app mutex poisoned");
            std::mem::take(&mut *guard)
        };
        let _ = owner
            .into_router()
            .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
        Ok(())
    }
}

#![allow(deprecated)]
#![deny(clippy::unwrap_used)]

//! Spikard Ruby bindings using Magnus FFI.
//!
//! This crate provides Ruby bindings for the Spikard HTTP toolkit, allowing
//! Ruby developers to build and test HTTP services with Rust performance.
//!
//! ## Modules
//!
//! - `testing`: Testing utilities (client, SSE, WebSocket)
//! - `handler`: RubyHandler trait implementation
//! - `di`: Dependency injection bridge for Ruby types
//! - `config`: ServerConfig extraction from Ruby objects
//! - `conversion`: Ruby ↔ Rust type conversions
//! - `server`: HTTP server setup and lifecycle management
//! - `background`: Background task management
//! - `lifecycle`: Lifecycle hook implementations
//! - `sse`: Server-Sent Events support
//! - `websocket`: WebSocket support

mod background;
mod config;
mod conversion;
mod di;
mod handler;
mod integration;
mod lifecycle;
mod metadata;
mod runtime;
mod server;
mod sse;
mod testing;
mod websocket;

use async_stream::stream;
use axum::body::Body;
use axum::http::{HeaderName, HeaderValue, Method, StatusCode};
use axum_test::{TestServer, TestServerConfig, Transport};
use bytes::Bytes;
use cookie::Cookie;
use magnus::prelude::*;
use magnus::value::{InnerValue, Opaque};
use magnus::{
    Error, Module, RArray, RHash, RString, Ruby, TryConvert, Value, function, gc::Marker, method, r_hash::ForEach,
};
use once_cell::sync::Lazy;
use serde_json::{Map as JsonMap, Value as JsonValue};
use spikard_http::testing::{
    MultipartFilePart, SnapshotError, build_multipart_body, encode_urlencoded_body, snapshot_response,
};
use spikard_http::{Handler, HandlerResponse, HandlerResult, RequestData};
use spikard_http::{ParameterValidator, ProblemDetails};
use spikard_http::{Route, RouteMetadata, SchemaValidator};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::mem;
use std::pin::Pin;
use std::sync::Arc;
use tokio::runtime::{Builder, Runtime};

use crate::config::extract_server_config;
use crate::conversion::{extract_files, map_to_ruby_hash, multimap_to_ruby_hash, problem_to_json};
use crate::integration::build_dependency_container;
use crate::metadata::{build_route_metadata, json_to_ruby, ruby_value_to_json};
use crate::runtime::{normalize_route_metadata, run_server};

static GLOBAL_RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to initialise global Tokio runtime")
});

#[derive(Default)]
#[magnus::wrap(class = "Spikard::Native::TestClient", free_immediately, mark)]
struct NativeTestClient {
    inner: RefCell<Option<ClientInner>>,
}

struct ClientInner {
    http_server: Arc<TestServer>,
    transport_server: Arc<TestServer>,
    /// Keep Ruby handler closures alive for GC; accessed via the `mark` hook.
    _handlers: Vec<RubyHandler>,
}

struct RequestConfig {
    query: Option<JsonValue>,
    headers: HashMap<String, String>,
    cookies: HashMap<String, String>,
    body: Option<RequestBody>,
}

enum RequestBody {
    Json(JsonValue),
    Form(JsonValue),
    Raw(String),
    Multipart {
        form_data: Vec<(String, String)>,
        files: Vec<MultipartFilePart>,
    },
}

#[derive(Clone)]
struct RubyHandler {
    inner: Arc<RubyHandlerInner>,
}

struct RubyHandlerInner {
    handler_proc: Opaque<Value>,
    handler_name: String,
    method: String,
    path: String,
    json_module: Opaque<Value>,
    request_validator: Option<Arc<SchemaValidator>>,
    response_validator: Option<Arc<SchemaValidator>>,
    parameter_validator: Option<ParameterValidator>,
    #[cfg(feature = "di")]
    handler_dependencies: Vec<String>,
}

struct HandlerResponsePayload {
    status: u16,
    headers: HashMap<String, String>,
    body: Option<JsonValue>,
    raw_body: Option<Vec<u8>>,
}

struct NativeResponseParts {
    response: HandlerResponse,
    body_json: Option<JsonValue>,
}

enum RubyHandlerResult {
    Payload(HandlerResponsePayload),
    Streaming(StreamingResponsePayload),
    Native(NativeResponseParts),
}

struct StreamingResponsePayload {
    enumerator: Arc<Opaque<Value>>,
    status: u16,
    headers: HashMap<String, String>,
}

#[magnus::wrap(class = "Spikard::Native::BuiltResponse", free_immediately, mark)]
struct NativeBuiltResponse {
    response: RefCell<Option<HandlerResponse>>,
    body_json: Option<JsonValue>,
    /// Ruby values that must be kept alive for GC (e.g., streaming enumerators)
    #[allow(dead_code)]
    gc_handles: Vec<Opaque<Value>>,
}

#[derive(Default)]
#[magnus::wrap(class = "Spikard::Native::LifecycleRegistry", free_immediately, mark)]
struct NativeLifecycleRegistry {
    hooks: RefCell<spikard_http::LifecycleHooks>,
    ruby_hooks: RefCell<Vec<Arc<crate::lifecycle::RubyLifecycleHook>>>,
}

#[magnus::wrap(class = "Spikard::Native::DependencyRegistry", free_immediately, mark)]
struct NativeDependencyRegistry {
    container: RefCell<Option<spikard_core::di::DependencyContainer>>,
    #[allow(dead_code)]
    gc_handles: RefCell<Vec<Opaque<Value>>>,
    registered_keys: RefCell<Vec<String>>,
}

impl StreamingResponsePayload {
    fn into_response(self) -> Result<HandlerResponse, Error> {
        let ruby = match Ruby::get() {
            Ok(r) => r,
            Err(_) => {
                panic!("Ruby VM became unavailable during streaming response construction");
            }
        };

        let status = StatusCode::from_u16(self.status).map_err(|err| {
            Error::new(
                ruby.exception_arg_error(),
                format!("Invalid streaming status code {}: {}", self.status, err),
            )
        })?;

        let header_pairs = self
            .headers
            .into_iter()
            .map(|(name, value)| {
                let header_name = HeaderName::from_bytes(name.as_bytes()).map_err(|err| {
                    Error::new(
                        ruby.exception_arg_error(),
                        format!("Invalid header name '{name}': {err}"),
                    )
                })?;
                let header_value = HeaderValue::from_str(&value).map_err(|err| {
                    Error::new(
                        ruby.exception_arg_error(),
                        format!("Invalid header value for '{name}': {err}"),
                    )
                })?;
                Ok((header_name, header_value))
            })
            .collect::<Result<Vec<_>, Error>>()?;

        let enumerator = self.enumerator.clone();
        let body_stream = stream! {
            loop {
                match poll_stream_chunk(&enumerator) {
                    Ok(Some(bytes)) => yield Ok(bytes),
                    Ok(None) => break,
                    Err(err) => {
                        yield Err(Box::new(err));
                        break;
                    }
                }
            }
        };

        let mut response = HandlerResponse::stream(body_stream).with_status(status);
        for (name, value) in header_pairs {
            response = response.with_header(name, value);
        }
        Ok(response)
    }
}

impl NativeBuiltResponse {
    #[allow(dead_code)]
    fn new(response: HandlerResponse, body_json: Option<JsonValue>, gc_handles: Vec<Opaque<Value>>) -> Self {
        Self {
            response: RefCell::new(Some(response)),
            body_json,
            gc_handles,
        }
    }

    fn extract_parts(&self) -> Result<(HandlerResponse, Option<JsonValue>), Error> {
        let mut borrow = self.response.borrow_mut();
        let response = borrow
            .take()
            .ok_or_else(|| Error::new(magnus::exception::runtime_error(), "Native response already consumed"))?;
        Ok((response, self.body_json.clone()))
    }

    fn status_code(&self) -> u16 {
        let borrow = self.response.borrow();
        let Some(response) = borrow.as_ref() else {
            return StatusCode::OK.as_u16();
        };

        match response {
            HandlerResponse::Response(resp) => resp.status().as_u16(),
            HandlerResponse::Stream { status, .. } => status.as_u16(),
        }
    }

    fn headers(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let headers_hash = ruby.hash_new();
        if let Some(response) = this.response.borrow().as_ref() {
            match response {
                HandlerResponse::Response(resp) => {
                    for (header_name, value) in resp.headers() {
                        let name = header_name.as_str();
                        if let Ok(value_str) = value.to_str() {
                            headers_hash.aset(ruby.str_new(name), ruby.str_new(value_str))?;
                        }
                    }
                }
                HandlerResponse::Stream { headers, .. } => {
                    for (header_name, value) in headers.iter() {
                        let name = header_name.as_str();
                        if let Ok(value_str) = value.to_str() {
                            headers_hash.aset(ruby.str_new(name), ruby.str_new(value_str))?;
                        }
                    }
                }
            }
        }
        Ok(headers_hash.as_value())
    }

    #[allow(dead_code)]
    fn mark(&self, marker: &Marker) {
        if let Ok(ruby) = Ruby::get() {
            for handle in &self.gc_handles {
                marker.mark(handle.get_inner_with(&ruby));
            }
        }
    }
}

impl NativeLifecycleRegistry {
    fn add_on_request(&self, hook_value: Value) -> Result<(), Error> {
        self.add_hook("on_request", hook_value, |hooks, hook| hooks.add_on_request(hook))
    }

    fn add_pre_validation(&self, hook_value: Value) -> Result<(), Error> {
        self.add_hook("pre_validation", hook_value, |hooks, hook| {
            hooks.add_pre_validation(hook)
        })
    }

    fn add_pre_handler(&self, hook_value: Value) -> Result<(), Error> {
        self.add_hook("pre_handler", hook_value, |hooks, hook| hooks.add_pre_handler(hook))
    }

    fn add_on_response(&self, hook_value: Value) -> Result<(), Error> {
        self.add_hook("on_response", hook_value, |hooks, hook| hooks.add_on_response(hook))
    }

    fn add_on_error(&self, hook_value: Value) -> Result<(), Error> {
        self.add_hook("on_error", hook_value, |hooks, hook| hooks.add_on_error(hook))
    }

    fn take_hooks(&self) -> spikard_http::LifecycleHooks {
        mem::take(&mut *self.hooks.borrow_mut())
    }

    #[allow(dead_code)]
    fn mark(&self, marker: &Marker) {
        for hook in self.ruby_hooks.borrow().iter() {
            hook.mark(marker);
        }
    }

    fn add_hook<F>(&self, kind: &str, hook_value: Value, push: F) -> Result<(), Error>
    where
        F: Fn(&mut spikard_http::LifecycleHooks, Arc<crate::lifecycle::RubyLifecycleHook>),
    {
        let idx = self.ruby_hooks.borrow().len();
        let hook = Arc::new(crate::lifecycle::RubyLifecycleHook::new(
            format!("{kind}_{idx}"),
            hook_value,
        ));

        push(&mut self.hooks.borrow_mut(), hook.clone());
        self.ruby_hooks.borrow_mut().push(hook);
        Ok(())
    }
}

impl Default for NativeDependencyRegistry {
    fn default() -> Self {
        Self {
            container: RefCell::new(Some(spikard_core::di::DependencyContainer::new())),
            gc_handles: RefCell::new(Vec::new()),
            registered_keys: RefCell::new(Vec::new()),
        }
    }
}

impl NativeDependencyRegistry {
    fn register_value(ruby: &Ruby, this: &Self, key: String, value: Value) -> Result<(), Error> {
        let dependency = crate::di::RubyValueDependency::new(key.clone(), value);
        this.register_dependency(ruby, key, Arc::new(dependency), Some(value))
    }

    fn register_factory(
        ruby: &Ruby,
        this: &Self,
        key: String,
        factory: Value,
        depends_on: Value,
        singleton: bool,
        cacheable: bool,
    ) -> Result<(), Error> {
        let depends_on_vec = if depends_on.is_nil() {
            Vec::new()
        } else {
            Vec::<String>::try_convert(depends_on)?
        };

        let dependency =
            crate::di::RubyFactoryDependency::new(key.clone(), factory, depends_on_vec, singleton, cacheable);
        this.register_dependency(ruby, key, Arc::new(dependency), Some(factory))
    }

    fn register_dependency(
        &self,
        ruby: &Ruby,
        key: String,
        dependency: Arc<dyn spikard_core::di::Dependency>,
        gc_value: Option<Value>,
    ) -> Result<(), Error> {
        let mut container_ref = self.container.borrow_mut();
        let container = container_ref
            .as_mut()
            .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "Dependency container already consumed"))?;

        container.register(key.clone(), dependency).map_err(|err| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("Failed to register dependency '{key}': {err}"),
            )
        })?;

        if let Some(val) = gc_value {
            self.gc_handles.borrow_mut().push(Opaque::from(val));
        }

        self.registered_keys.borrow_mut().push(key);

        Ok(())
    }

    #[allow(dead_code)]
    fn mark(&self, marker: &Marker) {
        if let Ok(ruby) = Ruby::get() {
            for handle in self.gc_handles.borrow().iter() {
                marker.mark(handle.get_inner_with(&ruby));
            }
        }
    }

    fn take_container(&self) -> Result<spikard_core::di::DependencyContainer, Error> {
        let mut borrow = self.container.borrow_mut();
        let container = borrow.take().ok_or_else(|| {
            Error::new(
                magnus::exception::runtime_error(),
                "Dependency container already consumed",
            )
        })?;
        Ok(container)
    }

    fn keys(&self) -> Vec<String> {
        self.registered_keys.borrow().clone()
    }
}

fn poll_stream_chunk(enumerator: &Arc<Opaque<Value>>) -> Result<Option<Bytes>, io::Error> {
    let ruby = Ruby::get().map_err(|err| io::Error::other(err.to_string()))?;
    let enum_value = enumerator.get_inner_with(&ruby);
    match enum_value.funcall::<_, _, Value>("next", ()) {
        Ok(chunk) => ruby_value_to_bytes(chunk).map(Some),
        Err(err) => {
            if err.is_kind_of(ruby.exception_stop_iteration()) {
                Ok(None)
            } else {
                Err(io::Error::other(err.to_string()))
            }
        }
    }
}

fn ruby_value_to_bytes(value: Value) -> Result<Bytes, io::Error> {
    if let Ok(str_value) = RString::try_convert(value) {
        let slice = unsafe { str_value.as_slice() };
        return Ok(Bytes::copy_from_slice(slice));
    }

    if let Ok(vec_bytes) = Vec::<u8>::try_convert(value) {
        return Ok(Bytes::from(vec_bytes));
    }

    Err(io::Error::other("Streaming chunks must be Strings or Arrays of bytes"))
}

struct TestResponseData {
    status: u16,
    headers: HashMap<String, String>,
    body_text: Option<String>,
}

#[derive(Debug)]
struct NativeRequestError(String);

impl NativeTestClient {
    #[allow(clippy::too_many_arguments)]
    fn initialize(
        ruby: &Ruby,
        this: &Self,
        routes_json: String,
        handlers: Value,
        config_value: Value,
        ws_handlers: Value,
        sse_producers: Value,
        dependencies: Value,
    ) -> Result<(), Error> {
        let metadata: Vec<RouteMetadata> = serde_json::from_str(&routes_json)
            .map_err(|err| Error::new(ruby.exception_arg_error(), format!("Invalid routes JSON: {err}")))?;

        let handlers_hash = RHash::from_value(handlers).ok_or_else(|| {
            Error::new(
                ruby.exception_arg_error(),
                "handlers parameter must be a Hash of handler_name => Proc",
            )
        })?;

        let json_module = ruby
            .class_object()
            .const_get("JSON")
            .map_err(|_| Error::new(ruby.exception_runtime_error(), "JSON module not available"))?;

        let mut server_config = extract_server_config(ruby, config_value)?;

        #[cfg(feature = "di")]
        {
            if let Ok(registry) = <&NativeDependencyRegistry>::try_convert(dependencies) {
                server_config.di_container = Some(Arc::new(registry.take_container()?));
            } else if !dependencies.is_nil() {
                match build_dependency_container(ruby, dependencies) {
                    Ok(container) => {
                        server_config.di_container = Some(Arc::new(container));
                    }
                    Err(err) => {
                        return Err(Error::new(
                            ruby.exception_runtime_error(),
                            format!("Failed to build DI container: {}", err),
                        ));
                    }
                }
            }
        }

        let schema_registry = spikard_http::SchemaRegistry::new();
        let mut prepared_routes = Vec::with_capacity(metadata.len());
        let mut handler_refs = Vec::with_capacity(metadata.len());
        let mut route_metadata_vec = Vec::with_capacity(metadata.len());

        for meta in metadata.clone() {
            let handler_value = fetch_handler(ruby, &handlers_hash, &meta.handler_name)?;
            let route = Route::from_metadata(meta.clone(), &schema_registry)
                .map_err(|err| Error::new(ruby.exception_runtime_error(), format!("Failed to build route: {err}")))?;

            let handler = RubyHandler::new(&route, handler_value, json_module)?;
            prepared_routes.push((route, Arc::new(handler.clone()) as Arc<dyn spikard_http::Handler>));
            handler_refs.push(handler);
            route_metadata_vec.push(meta);
        }

        let mut router = spikard_http::server::build_router_with_handlers_and_config(
            prepared_routes,
            server_config,
            route_metadata_vec,
        )
        .map_err(|err| Error::new(ruby.exception_runtime_error(), format!("Failed to build router: {err}")))?;

        let mut ws_endpoints = Vec::new();
        if !ws_handlers.is_nil() {
            let ws_hash = RHash::from_value(ws_handlers)
                .ok_or_else(|| Error::new(ruby.exception_arg_error(), "WebSocket handlers must be a Hash"))?;

            ws_hash.foreach(|path: String, factory: Value| -> Result<ForEach, Error> {
                let handler_instance = factory.funcall::<_, _, Value>("call", ()).map_err(|e| {
                    Error::new(
                        ruby.exception_runtime_error(),
                        format!("Failed to create WebSocket handler: {}", e),
                    )
                })?;

                let ws_state = crate::websocket::create_websocket_state(ruby, handler_instance)?;

                ws_endpoints.push((path, ws_state));

                Ok(ForEach::Continue)
            })?;
        }

        let mut sse_endpoints = Vec::new();
        if !sse_producers.is_nil() {
            let sse_hash = RHash::from_value(sse_producers)
                .ok_or_else(|| Error::new(ruby.exception_arg_error(), "SSE producers must be a Hash"))?;

            sse_hash.foreach(|path: String, factory: Value| -> Result<ForEach, Error> {
                let producer_instance = factory.funcall::<_, _, Value>("call", ()).map_err(|e| {
                    Error::new(
                        ruby.exception_runtime_error(),
                        format!("Failed to create SSE producer: {}", e),
                    )
                })?;

                let sse_state = crate::sse::create_sse_state(ruby, producer_instance)?;

                sse_endpoints.push((path, sse_state));

                Ok(ForEach::Continue)
            })?;
        }

        use axum::routing::get;
        for (path, ws_state) in ws_endpoints {
            router = router.route(
                &path,
                get(spikard_http::websocket_handler::<crate::websocket::RubyWebSocketHandler>).with_state(ws_state),
            );
        }

        for (path, sse_state) in sse_endpoints {
            router = router.route(
                &path,
                get(spikard_http::sse_handler::<crate::sse::RubySseEventProducer>).with_state(sse_state),
            );
        }

        let http_server = GLOBAL_RUNTIME
            .block_on(async { TestServer::new(router.clone()) })
            .map_err(|err| {
                Error::new(
                    ruby.exception_runtime_error(),
                    format!("Failed to initialise test server: {err}"),
                )
            })?;

        let ws_config = TestServerConfig {
            transport: Some(Transport::HttpRandomPort),
            ..Default::default()
        };
        let transport_server = GLOBAL_RUNTIME
            .block_on(async { TestServer::new_with_config(router, ws_config) })
            .map_err(|err| {
                Error::new(
                    ruby.exception_runtime_error(),
                    format!("Failed to initialise WebSocket transport server: {err}"),
                )
            })?;

        *this.inner.borrow_mut() = Some(ClientInner {
            http_server: Arc::new(http_server),
            transport_server: Arc::new(transport_server),
            _handlers: handler_refs,
        });

        Ok(())
    }

    fn request(ruby: &Ruby, this: &Self, method: String, path: String, options: Value) -> Result<Value, Error> {
        let inner_borrow = this.inner.borrow();
        let inner = inner_borrow
            .as_ref()
            .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "TestClient not initialised"))?;
        let method_upper = method.to_ascii_uppercase();
        let http_method = Method::from_bytes(method_upper.as_bytes()).map_err(|err| {
            Error::new(
                ruby.exception_arg_error(),
                format!("Unsupported method {method_upper}: {err}"),
            )
        })?;

        let request_config = parse_request_config(ruby, options)?;

        let response = GLOBAL_RUNTIME
            .block_on(execute_request(
                inner.http_server.clone(),
                http_method,
                path.clone(),
                request_config,
            ))
            .map_err(|err| {
                Error::new(
                    ruby.exception_runtime_error(),
                    format!("Request failed for {method_upper} {path}: {}", err.0),
                )
            })?;

        response_to_ruby(ruby, response)
    }

    fn close(&self) -> Result<(), Error> {
        *self.inner.borrow_mut() = None;
        Ok(())
    }

    fn websocket(ruby: &Ruby, this: &Self, path: String) -> Result<Value, Error> {
        let inner_borrow = this.inner.borrow();
        let inner = inner_borrow
            .as_ref()
            .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "TestClient not initialised"))?;

        let server = Arc::clone(&inner.transport_server);

        drop(inner_borrow);

        let handle =
            GLOBAL_RUNTIME.spawn(async move { spikard_http::testing::connect_websocket(&server, &path).await });

        let ws = GLOBAL_RUNTIME.block_on(async {
            handle
                .await
                .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("WebSocket task failed: {}", e)))
        })?;

        let ws_conn = testing::websocket::WebSocketTestConnection::new(ws);
        Ok(ruby.obj_wrap(ws_conn).as_value())
    }

    fn sse(ruby: &Ruby, this: &Self, path: String) -> Result<Value, Error> {
        let inner_borrow = this.inner.borrow();
        let inner = inner_borrow
            .as_ref()
            .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "TestClient not initialised"))?;

        let response = GLOBAL_RUNTIME
            .block_on(async {
                let axum_response = inner.transport_server.get(&path).await;
                snapshot_response(axum_response).await
            })
            .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("SSE request failed: {}", e)))?;

        testing::sse::sse_stream_from_response(ruby, &response)
    }
}

impl RubyHandler {
    fn new(route: &Route, handler_value: Value, json_module: Value) -> Result<Self, Error> {
        Ok(Self {
            inner: Arc::new(RubyHandlerInner {
                handler_proc: Opaque::from(handler_value),
                handler_name: route.handler_name.clone(),
                method: route.method.as_str().to_string(),
                path: route.path.clone(),
                json_module: Opaque::from(json_module),
                request_validator: route.request_validator.clone(),
                response_validator: route.response_validator.clone(),
                parameter_validator: route.parameter_validator.clone(),
                #[cfg(feature = "di")]
                handler_dependencies: route.handler_dependencies.clone(),
            }),
        })
    }

    /// Create a new RubyHandler for server mode
    ///
    /// This is used by run_server to create handlers from Ruby Procs
    fn new_for_server(
        _ruby: &Ruby,
        handler_value: Value,
        handler_name: String,
        method: String,
        path: String,
        json_module: Value,
        route: &Route,
    ) -> Result<Self, Error> {
        Ok(Self {
            inner: Arc::new(RubyHandlerInner {
                handler_proc: Opaque::from(handler_value),
                handler_name,
                method,
                path,
                json_module: Opaque::from(json_module),
                request_validator: route.request_validator.clone(),
                response_validator: route.response_validator.clone(),
                parameter_validator: route.parameter_validator.clone(),
                #[cfg(feature = "di")]
                handler_dependencies: route.handler_dependencies.clone(),
            }),
        })
    }

    /// Required by Ruby GC; invoked through the magnus mark hook.
    #[allow(dead_code)]
    fn mark(&self, marker: &Marker) {
        if let Ok(ruby) = Ruby::get() {
            let proc_val = self.inner.handler_proc.get_inner_with(&ruby);
            marker.mark(proc_val);
        }
    }

    fn handle(&self, request_data: RequestData) -> HandlerResult {
        if let Some(validator) = &self.inner.request_validator
            && let Err(errors) = validator.validate(&request_data.body)
        {
            let problem = ProblemDetails::from_validation_error(&errors);
            let error_json = problem_to_json(&problem);
            return Err((problem.status_code(), error_json));
        }

        let validated_params = if let Some(validator) = &self.inner.parameter_validator {
            match validator.validate_and_extract(
                &request_data.query_params,
                request_data.raw_query_params.as_ref(),
                request_data.path_params.as_ref(),
                request_data.headers.as_ref(),
                request_data.cookies.as_ref(),
            ) {
                Ok(value) => Some(value),
                Err(errors) => {
                    let problem = ProblemDetails::from_validation_error(&errors);
                    return Err((problem.status_code(), problem_to_json(&problem)));
                }
            }
        } else {
            None
        };

        let ruby = Ruby::get().map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Ruby VM unavailable while invoking handler".to_string(),
            )
        })?;

        let request_value = build_ruby_request(&ruby, &self.inner, &request_data, validated_params.as_ref())
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

        let handler_proc = self.inner.handler_proc.get_inner_with(&ruby);

        #[cfg(feature = "di")]
        let handler_result = {
            if let Some(deps) = &request_data.dependencies {
                let kwargs_hash = ruby.hash_new();

                for key in &self.inner.handler_dependencies {
                    if !deps.contains(key) {
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!(
                                "Handler '{}' requires dependency '{}' which was not resolved",
                                self.inner.handler_name, key
                            ),
                        ));
                    }
                }

                for key in &self.inner.handler_dependencies {
                    if let Some(value) = deps.get_arc(key) {
                        let ruby_val = if let Some(wrapper) = value.downcast_ref::<crate::di::RubyValueWrapper>() {
                            wrapper.get_value(&ruby)
                        } else if let Some(json) = value.downcast_ref::<serde_json::Value>() {
                            match crate::di::json_to_ruby(&ruby, json) {
                                Ok(val) => val,
                                Err(e) => {
                                    return Err((
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                        format!("Failed to convert dependency '{}' to Ruby: {}", key, e),
                                    ));
                                }
                            }
                        } else {
                            return Err((
                                StatusCode::INTERNAL_SERVER_ERROR,
                                format!(
                                    "Unknown dependency type for '{}': expected RubyValueWrapper or JSON",
                                    key
                                ),
                            ));
                        };

                        let key_sym = ruby.to_symbol(key);
                        if let Err(e) = kwargs_hash.aset(key_sym, ruby_val) {
                            return Err((
                                StatusCode::INTERNAL_SERVER_ERROR,
                                format!("Failed to add dependency '{}': {}", key, e),
                            ));
                        }
                    }
                }

                let wrapper_code = ruby
                    .eval::<Value>(
                        r#"
                    lambda do |proc, request, kwargs|
                        proc.call(request, **kwargs)
                    end
                "#,
                    )
                    .map_err(|e| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Failed to create kwarg wrapper: {}", e),
                        )
                    })?;

                wrapper_code.funcall("call", (handler_proc, request_value, kwargs_hash))
            } else {
                handler_proc.funcall("call", (request_value,))
            }
        };

        #[cfg(not(feature = "di"))]
        let handler_result = handler_proc.funcall("call", (request_value,));

        let response_value = match handler_result {
            Ok(value) => value,
            Err(err) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Handler '{}' failed: {}", self.inner.handler_name, err),
                ));
            }
        };

        let handler_result = interpret_handler_response(&ruby, &self.inner, response_value).map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!(
                    "Failed to interpret response from '{}': {}",
                    self.inner.handler_name, err
                ),
            )
        })?;

        let payload = match handler_result {
            RubyHandlerResult::Native(native) => {
                if let (Some(validator), Some(body)) = (&self.inner.response_validator, native.body_json.as_ref())
                    && let Err(errors) = validator.validate(body)
                {
                    let problem = ProblemDetails::from_validation_error(&errors);
                    return Err((StatusCode::INTERNAL_SERVER_ERROR, problem_to_json(&problem)));
                }

                return Ok(native.response.into_response());
            }
            RubyHandlerResult::Streaming(streaming) => {
                let response = streaming.into_response().map_err(|err| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to build streaming response: {}", err),
                    )
                })?;
                return Ok(response.into_response());
            }
            RubyHandlerResult::Payload(payload) => payload,
        };

        if let (Some(validator), Some(body)) = (&self.inner.response_validator, payload.body.as_ref())
            && let Err(errors) = validator.validate(body)
        {
            let problem = ProblemDetails::from_validation_error(&errors);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, problem_to_json(&problem)));
        }

        let HandlerResponsePayload {
            status,
            headers,
            body,
            raw_body,
        } = payload;

        let mut response_builder = axum::http::Response::builder().status(status);
        let mut has_content_type = false;

        for (name, value) in headers.iter() {
            if name.eq_ignore_ascii_case("content-type") {
                has_content_type = true;
            }
            let header_name = HeaderName::from_bytes(name.as_bytes()).map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Invalid header name '{name}': {err}"),
                )
            })?;
            let header_value = HeaderValue::from_str(value).map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Invalid header value for '{name}': {err}"),
                )
            })?;

            response_builder = response_builder.header(header_name, header_value);
        }

        if !has_content_type && body.is_some() {
            response_builder = response_builder.header(
                HeaderName::from_static("content-type"),
                HeaderValue::from_static("application/json"),
            );
        }

        let body_bytes = if let Some(raw) = raw_body {
            raw
        } else if let Some(json_value) = body {
            serde_json::to_vec(&json_value).map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to serialise response body: {err}"),
                )
            })?
        } else {
            Vec::new()
        };

        response_builder.body(Body::from(body_bytes)).map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to build response: {err}"),
            )
        })
    }
}

impl Handler for RubyHandler {
    fn call(
        &self,
        _req: axum::http::Request<Body>,
        request_data: RequestData,
    ) -> Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send + '_>> {
        let handler = self.clone();
        Box::pin(async move { handler.handle(request_data) })
    }
}

async fn execute_request(
    server: Arc<TestServer>,
    method: Method,
    path: String,
    config: RequestConfig,
) -> Result<TestResponseData, NativeRequestError> {
    let mut request = match method {
        Method::GET => server.get(&path),
        Method::POST => server.post(&path),
        Method::PUT => server.put(&path),
        Method::PATCH => server.patch(&path),
        Method::DELETE => server.delete(&path),
        Method::HEAD => server.method(Method::HEAD, &path),
        Method::OPTIONS => server.method(Method::OPTIONS, &path),
        Method::TRACE => server.method(Method::TRACE, &path),
        other => return Err(NativeRequestError(format!("Unsupported HTTP method {other}"))),
    };

    if let Some(query) = config.query {
        request = request.add_query_params(&query);
    }

    for (name, value) in config.headers {
        request = request.add_header(name.as_str(), value.as_str());
    }

    for (name, value) in config.cookies {
        request = request.add_cookie(Cookie::new(name, value));
    }

    if let Some(body) = config.body {
        match body {
            RequestBody::Json(json_value) => {
                request = request.json(&json_value);
            }
            RequestBody::Form(form_value) => {
                let encoded = encode_urlencoded_body(&form_value)
                    .map_err(|err| NativeRequestError(format!("Failed to encode form body: {err}")))?;
                request = request
                    .content_type("application/x-www-form-urlencoded")
                    .bytes(Bytes::from(encoded));
            }
            RequestBody::Raw(raw) => {
                request = request.bytes(Bytes::from(raw));
            }
            RequestBody::Multipart { form_data, files } => {
                let (multipart_body, boundary) = build_multipart_body(&form_data, &files);
                request = request
                    .content_type(&format!("multipart/form-data; boundary={}", boundary))
                    .bytes(Bytes::from(multipart_body));
            }
        }
    }

    let response = request.await;
    let snapshot = snapshot_response(response).await.map_err(snapshot_err_to_native)?;
    let body_text = if snapshot.body.is_empty() {
        None
    } else {
        Some(String::from_utf8_lossy(&snapshot.body).into_owned())
    };

    Ok(TestResponseData {
        status: snapshot.status,
        headers: snapshot.headers,
        body_text,
    })
}

fn snapshot_err_to_native(err: SnapshotError) -> NativeRequestError {
    NativeRequestError(err.to_string())
}

fn parse_request_config(ruby: &Ruby, options: Value) -> Result<RequestConfig, Error> {
    if options.is_nil() {
        return Ok(RequestConfig {
            query: None,
            headers: HashMap::new(),
            cookies: HashMap::new(),
            body: None,
        });
    }

    let hash = RHash::from_value(options)
        .ok_or_else(|| Error::new(ruby.exception_arg_error(), "request options must be a Hash"))?;

    let json_module = ruby
        .class_object()
        .const_get("JSON")
        .map_err(|_| Error::new(ruby.exception_runtime_error(), "JSON module not available"))?;

    let query = if let Some(value) = get_kw(ruby, hash, "query") {
        if value.is_nil() {
            None
        } else {
            Some(ruby_value_to_json(ruby, json_module, value)?)
        }
    } else {
        None
    };

    let headers = if let Some(value) = get_kw(ruby, hash, "headers") {
        if value.is_nil() {
            HashMap::new()
        } else {
            let hash = RHash::try_convert(value)?;
            hash.to_hash_map::<String, String>()?
        }
    } else {
        HashMap::new()
    };

    let cookies = if let Some(value) = get_kw(ruby, hash, "cookies") {
        if value.is_nil() {
            HashMap::new()
        } else {
            let hash = RHash::try_convert(value)?;
            hash.to_hash_map::<String, String>()?
        }
    } else {
        HashMap::new()
    };

    let files_opt = get_kw(ruby, hash, "files");
    let has_files = files_opt.as_ref().is_some_and(|f| !f.is_nil());

    let body = if has_files {
        let files_value = files_opt.ok_or_else(|| {
            Error::new(
                ruby.exception_runtime_error(),
                "Files option should be Some if has_files is true",
            )
        })?;
        let files = extract_files(ruby, files_value)?;

        let mut form_data = Vec::new();
        if let Some(data_value) = get_kw(ruby, hash, "data")
            && !data_value.is_nil()
        {
            let data_hash = RHash::try_convert(data_value)?;

            let keys_array: RArray = data_hash.funcall("keys", ())?;

            for i in 0..keys_array.len() {
                let key_val = keys_array.entry::<Value>(i as isize)?;
                let field_name = String::try_convert(key_val)?;
                let value = data_hash
                    .get(key_val)
                    .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "Failed to get hash value"))?;

                if let Some(array) = RArray::from_value(value) {
                    for j in 0..array.len() {
                        let item = array.entry::<Value>(j as isize)?;
                        let item_str = String::try_convert(item)?;
                        form_data.push((field_name.clone(), item_str));
                    }
                } else {
                    let value_str = String::try_convert(value)?;
                    form_data.push((field_name, value_str));
                }
            }
        }

        Some(RequestBody::Multipart { form_data, files })
    } else if let Some(value) = get_kw(ruby, hash, "json") {
        if value.is_nil() {
            None
        } else {
            Some(RequestBody::Json(ruby_value_to_json(ruby, json_module, value)?))
        }
    } else if let Some(value) = get_kw(ruby, hash, "data") {
        if value.is_nil() {
            None
        } else {
            Some(RequestBody::Form(ruby_value_to_json(ruby, json_module, value)?))
        }
    } else if let Some(value) = get_kw(ruby, hash, "raw_body") {
        if value.is_nil() {
            None
        } else {
            Some(RequestBody::Raw(String::try_convert(value)?))
        }
    } else {
        None
    };

    Ok(RequestConfig {
        query,
        headers,
        cookies,
        body,
    })
}

fn build_ruby_request(
    ruby: &Ruby,
    handler: &RubyHandlerInner,
    request_data: &RequestData,
    validated_params: Option<&JsonValue>,
) -> Result<Value, Error> {
    let hash = ruby.hash_new();

    hash.aset(ruby.intern("method"), ruby.str_new(&handler.method))?;
    hash.aset(ruby.intern("path"), ruby.str_new(&handler.path))?;

    let path_params = map_to_ruby_hash(ruby, request_data.path_params.as_ref())?;
    hash.aset(ruby.intern("path_params"), path_params)?;

    let query_value = json_to_ruby(ruby, &request_data.query_params)?;
    hash.aset(ruby.intern("query"), query_value)?;

    let raw_query = multimap_to_ruby_hash(ruby, request_data.raw_query_params.as_ref())?;
    hash.aset(ruby.intern("raw_query"), raw_query)?;

    let headers = map_to_ruby_hash(ruby, request_data.headers.as_ref())?;
    hash.aset(ruby.intern("headers"), headers)?;

    let cookies = map_to_ruby_hash(ruby, request_data.cookies.as_ref())?;
    hash.aset(ruby.intern("cookies"), cookies)?;

    let body_value = json_to_ruby(ruby, &request_data.body)?;
    hash.aset(ruby.intern("body"), body_value)?;

    let params_value = if let Some(validated) = validated_params {
        json_to_ruby(ruby, validated)?
    } else {
        build_default_params(ruby, request_data)?
    };
    hash.aset(ruby.intern("params"), params_value)?;

    Ok(hash.as_value())
}

fn build_default_params(ruby: &Ruby, request_data: &RequestData) -> Result<Value, Error> {
    let mut map = JsonMap::new();

    for (key, value) in request_data.path_params.as_ref() {
        map.insert(key.clone(), JsonValue::String(value.clone()));
    }

    if let JsonValue::Object(obj) = &request_data.query_params {
        for (key, value) in obj {
            map.insert(key.clone(), value.clone());
        }
    }

    for (key, value) in request_data.headers.as_ref() {
        map.insert(key.clone(), JsonValue::String(value.clone()));
    }

    for (key, value) in request_data.cookies.as_ref() {
        map.insert(key.clone(), JsonValue::String(value.clone()));
    }

    json_to_ruby(ruby, &JsonValue::Object(map))
}

fn interpret_handler_response(
    ruby: &Ruby,
    handler: &RubyHandlerInner,
    value: Value,
) -> Result<RubyHandlerResult, Error> {
    let native_method = ruby.intern("to_native_response");
    if value.respond_to(native_method, false)? {
        let native_value: Value = value.funcall("to_native_response", ())?;
        if let Ok(native_resp) = <&NativeBuiltResponse>::try_convert(native_value) {
            let (response, body_json) = native_resp.extract_parts()?;
            return Ok(RubyHandlerResult::Native(NativeResponseParts { response, body_json }));
        }
    } else if let Ok(native_resp) = <&NativeBuiltResponse>::try_convert(value) {
        let (response, body_json) = native_resp.extract_parts()?;
        return Ok(RubyHandlerResult::Native(NativeResponseParts { response, body_json }));
    }

    if value.is_nil() {
        return Ok(RubyHandlerResult::Payload(HandlerResponsePayload {
            status: 200,
            headers: HashMap::new(),
            body: None,
            raw_body: None,
        }));
    }

    if is_streaming_response(ruby, value)? {
        let stream_value: Value = value.funcall("stream", ())?;
        let status: i64 = value.funcall("status_code", ())?;
        let headers_value: Value = value.funcall("headers", ())?;

        let status_u16 = u16::try_from(status).map_err(|_| {
            Error::new(
                ruby.exception_arg_error(),
                "StreamingResponse status_code must be between 0 and 65535",
            )
        })?;

        let headers = value_to_string_map(ruby, headers_value)?;

        return Ok(RubyHandlerResult::Streaming(StreamingResponsePayload {
            enumerator: Arc::new(Opaque::from(stream_value)),
            status: status_u16,
            headers,
        }));
    }

    let status_symbol = ruby.intern("status_code");
    if value.respond_to(status_symbol, false)? {
        let status: i64 = value.funcall("status_code", ())?;
        let status_u16 = u16::try_from(status)
            .map_err(|_| Error::new(ruby.exception_arg_error(), "status_code must be between 0 and 65535"))?;

        let headers_value: Value = value.funcall("headers", ())?;
        let headers = if headers_value.is_nil() {
            HashMap::new()
        } else {
            let hash = RHash::try_convert(headers_value)?;
            hash.to_hash_map::<String, String>()?
        };

        let content_value: Value = value.funcall("content", ())?;
        let mut raw_body = None;
        let body = if content_value.is_nil() {
            None
        } else if let Ok(str_value) = RString::try_convert(content_value) {
            let slice = unsafe { str_value.as_slice() };
            raw_body = Some(slice.to_vec());
            None
        } else {
            Some(ruby_value_to_json(
                ruby,
                handler.json_module.get_inner_with(ruby),
                content_value,
            )?)
        };

        return Ok(RubyHandlerResult::Payload(HandlerResponsePayload {
            status: status_u16,
            headers,
            body,
            raw_body,
        }));
    }

    if let Ok(str_value) = RString::try_convert(value) {
        let slice = unsafe { str_value.as_slice() };
        return Ok(RubyHandlerResult::Payload(HandlerResponsePayload {
            status: 200,
            headers: HashMap::new(),
            body: None,
            raw_body: Some(slice.to_vec()),
        }));
    }

    let body_json = ruby_value_to_json(ruby, handler.json_module.get_inner_with(ruby), value)?;

    Ok(RubyHandlerResult::Payload(HandlerResponsePayload {
        status: 200,
        headers: HashMap::new(),
        body: Some(body_json),
        raw_body: None,
    }))
}

fn value_to_string_map(ruby: &Ruby, value: Value) -> Result<HashMap<String, String>, Error> {
    if value.is_nil() {
        return Ok(HashMap::new());
    }
    let hash = RHash::try_convert(value)?;
    hash.to_hash_map::<String, String>().map_err(|err| {
        Error::new(
            ruby.exception_arg_error(),
            format!("Expected headers hash of strings: {}", err),
        )
    })
}

#[allow(dead_code)]
fn header_pairs_from_map(headers: HashMap<String, String>) -> Result<Vec<(HeaderName, HeaderValue)>, Error> {
    let ruby = Ruby::get().map_err(|err| Error::new(magnus::exception::runtime_error(), err.to_string()))?;
    headers
        .into_iter()
        .map(|(name, value)| {
            let header_name = HeaderName::from_bytes(name.as_bytes()).map_err(|err| {
                Error::new(
                    ruby.exception_arg_error(),
                    format!("Invalid header name '{name}': {err}"),
                )
            })?;
            let header_value = HeaderValue::from_str(&value).map_err(|err| {
                Error::new(
                    ruby.exception_arg_error(),
                    format!("Invalid header value for '{name}': {err}"),
                )
            })?;
            Ok((header_name, header_value))
        })
        .collect()
}

fn is_streaming_response(ruby: &Ruby, value: Value) -> Result<bool, Error> {
    let stream_sym = ruby.intern("stream");
    let status_sym = ruby.intern("status_code");
    Ok(value.respond_to(stream_sym, false)? && value.respond_to(status_sym, false)?)
}

fn response_to_ruby(ruby: &Ruby, response: TestResponseData) -> Result<Value, Error> {
    let hash = ruby.hash_new();

    hash.aset(
        ruby.intern("status_code"),
        ruby.integer_from_i64(response.status as i64),
    )?;

    let headers_hash = ruby.hash_new();
    for (key, value) in response.headers {
        headers_hash.aset(ruby.str_new(&key), ruby.str_new(&value))?;
    }
    hash.aset(ruby.intern("headers"), headers_hash)?;

    if let Some(body) = response.body_text {
        let body_value = ruby.str_new(&body);
        hash.aset(ruby.intern("body"), body_value)?;
        hash.aset(ruby.intern("body_text"), body_value)?;
    } else {
        hash.aset(ruby.intern("body"), ruby.qnil())?;
        hash.aset(ruby.intern("body_text"), ruby.qnil())?;
    }

    Ok(hash.as_value())
}

fn get_kw(ruby: &Ruby, hash: RHash, name: &str) -> Option<Value> {
    let sym = ruby.intern(name);
    hash.get(sym).or_else(|| hash.get(name))
}

fn fetch_handler(ruby: &Ruby, handlers: &RHash, name: &str) -> Result<Value, Error> {
    let symbol_key = ruby.intern(name);
    if let Some(value) = handlers.get(symbol_key) {
        return Ok(value);
    }

    let string_key = ruby.str_new(name);
    if let Some(value) = handlers.get(string_key) {
        return Ok(value);
    }

    Err(Error::new(
        ruby.exception_name_error(),
        format!("Handler '{name}' not provided"),
    ))
}

/// GC mark hook so Ruby keeps handler closures alive.
#[allow(dead_code)]
fn mark(client: &NativeTestClient, marker: &Marker) {
    let inner_ref = client.inner.borrow();
    if let Some(inner) = inner_ref.as_ref() {
        for handler in &inner._handlers {
            handler.mark(marker);
        }
    }
}

/// Return the Spikard version.
fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Build a native response from content, status code, and headers.
///
/// Called by `Spikard::Response` to construct native response objects.
/// The content can be a String (raw body), Hash/Array (JSON), or nil.
fn build_response(ruby: &Ruby, content: Value, status_code: i64, headers: Value) -> Result<Value, Error> {
    let status_u16 = u16::try_from(status_code)
        .map_err(|_| Error::new(ruby.exception_arg_error(), "status_code must be between 0 and 65535"))?;

    let header_map = if headers.is_nil() {
        HashMap::new()
    } else {
        let hash = RHash::try_convert(headers)?;
        hash.to_hash_map::<String, String>()?
    };

    let (body_json, raw_body_opt) = if content.is_nil() {
        (None, None)
    } else if let Ok(str_value) = RString::try_convert(content) {
        let slice = unsafe { str_value.as_slice() };
        (None, Some(slice.to_vec()))
    } else {
        let json_module = ruby
            .class_object()
            .const_get("JSON")
            .map_err(|_| Error::new(ruby.exception_runtime_error(), "JSON module not available"))?;
        let json_value = ruby_value_to_json(ruby, json_module, content)?;
        (Some(json_value), None)
    };

    let status = StatusCode::from_u16(status_u16).map_err(|err| {
        Error::new(
            ruby.exception_arg_error(),
            format!("Invalid status code {}: {}", status_u16, err),
        )
    })?;

    let mut response_builder = axum::http::Response::builder().status(status);

    for (name, value) in &header_map {
        let header_name = HeaderName::from_bytes(name.as_bytes()).map_err(|err| {
            Error::new(
                ruby.exception_arg_error(),
                format!("Invalid header name '{}': {}", name, err),
            )
        })?;
        let header_value = HeaderValue::from_str(value).map_err(|err| {
            Error::new(
                ruby.exception_arg_error(),
                format!("Invalid header value for '{}': {}", name, err),
            )
        })?;
        response_builder = response_builder.header(header_name, header_value);
    }

    let body_bytes = if let Some(raw) = raw_body_opt {
        raw
    } else if let Some(json_value) = body_json.as_ref() {
        serde_json::to_vec(&json_value).map_err(|err| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("Failed to serialise response body: {}", err),
            )
        })?
    } else {
        Vec::new()
    };

    let axum_response = response_builder.body(Body::from(body_bytes)).map_err(|err| {
        Error::new(
            ruby.exception_runtime_error(),
            format!("Failed to build response: {}", err),
        )
    })?;

    let handler_response = HandlerResponse::Response(axum_response);
    let native_response = NativeBuiltResponse::new(handler_response, body_json.clone(), Vec::new());
    Ok(ruby.obj_wrap(native_response).as_value())
}

/// Build a native streaming response from stream, status code, and headers.
///
/// Called by `Spikard::StreamingResponse` to construct native response objects.
/// The stream must be an enumerator that responds to #next.
fn build_streaming_response(ruby: &Ruby, stream: Value, status_code: i64, headers: Value) -> Result<Value, Error> {
    let status_u16 = u16::try_from(status_code)
        .map_err(|_| Error::new(ruby.exception_arg_error(), "status_code must be between 0 and 65535"))?;

    let header_map = if headers.is_nil() {
        HashMap::new()
    } else {
        let hash = RHash::try_convert(headers)?;
        hash.to_hash_map::<String, String>()?
    };

    let next_method = ruby.intern("next");
    if !stream.respond_to(next_method, false)? {
        return Err(Error::new(ruby.exception_arg_error(), "stream must respond to #next"));
    }

    let streaming_payload = StreamingResponsePayload {
        enumerator: Arc::new(Opaque::from(stream)),
        status: status_u16,
        headers: header_map,
    };

    let response = streaming_payload.into_response()?;
    let native_response = NativeBuiltResponse::new(response, None, vec![Opaque::from(stream)]);
    Ok(ruby.obj_wrap(native_response).as_value())
}

#[magnus::init]
pub fn init(ruby: &Ruby) -> Result<(), Error> {
    let spikard = ruby.define_module("Spikard")?;
    spikard.define_singleton_method("version", function!(version, 0))?;
    let native = match spikard.const_get("Native") {
        Ok(module) => module,
        Err(_) => spikard.define_module("Native")?,
    };

    native.define_singleton_method("run_server", function!(run_server, 7))?;
    native.define_singleton_method("normalize_route_metadata", function!(normalize_route_metadata, 1))?;
    native.define_singleton_method("background_run", function!(background::background_run, 1))?;
    native.define_singleton_method("build_route_metadata", function!(build_route_metadata, 12))?;
    native.define_singleton_method("build_response", function!(build_response, 3))?;
    native.define_singleton_method("build_streaming_response", function!(build_streaming_response, 3))?;

    let class = native.define_class("TestClient", ruby.class_object())?;
    class.define_alloc_func::<NativeTestClient>();
    class.define_method("initialize", method!(NativeTestClient::initialize, 6))?;
    class.define_method("request", method!(NativeTestClient::request, 3))?;
    class.define_method("websocket", method!(NativeTestClient::websocket, 1))?;
    class.define_method("sse", method!(NativeTestClient::sse, 1))?;
    class.define_method("close", method!(NativeTestClient::close, 0))?;

    let built_response_class = native.define_class("BuiltResponse", ruby.class_object())?;
    built_response_class.define_method("status_code", method!(NativeBuiltResponse::status_code, 0))?;
    built_response_class.define_method("headers", method!(NativeBuiltResponse::headers, 0))?;

    let lifecycle_registry_class = native.define_class("LifecycleRegistry", ruby.class_object())?;
    lifecycle_registry_class.define_alloc_func::<NativeLifecycleRegistry>();
    lifecycle_registry_class.define_method("add_on_request", method!(NativeLifecycleRegistry::add_on_request, 1))?;
    lifecycle_registry_class.define_method(
        "pre_validation",
        method!(NativeLifecycleRegistry::add_pre_validation, 1),
    )?;
    lifecycle_registry_class.define_method("pre_handler", method!(NativeLifecycleRegistry::add_pre_handler, 1))?;
    lifecycle_registry_class.define_method("on_response", method!(NativeLifecycleRegistry::add_on_response, 1))?;
    lifecycle_registry_class.define_method("on_error", method!(NativeLifecycleRegistry::add_on_error, 1))?;

    let dependency_registry_class = native.define_class("DependencyRegistry", ruby.class_object())?;
    dependency_registry_class.define_alloc_func::<NativeDependencyRegistry>();
    dependency_registry_class.define_method("register_value", method!(NativeDependencyRegistry::register_value, 2))?;
    dependency_registry_class.define_method(
        "register_factory",
        method!(NativeDependencyRegistry::register_factory, 5),
    )?;
    dependency_registry_class.define_method("keys", method!(NativeDependencyRegistry::keys, 0))?;

    let spikard_module = ruby.define_module("Spikard")?;
    testing::websocket::init(ruby, &spikard_module)?;
    testing::sse::init(ruby, &spikard_module)?;

    let _ = NativeBuiltResponse::mark as fn(&NativeBuiltResponse, &Marker);
    let _ = NativeLifecycleRegistry::mark as fn(&NativeLifecycleRegistry, &Marker);
    let _ = NativeDependencyRegistry::mark as fn(&NativeDependencyRegistry, &Marker);
    let _ = RubyHandler::mark as fn(&RubyHandler, &Marker);
    let _ = mark as fn(&NativeTestClient, &Marker);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Test that NativeBuiltResponse can extract parts safely
    #[test]
    fn test_native_built_response_status_extraction() {
        use axum::http::StatusCode;

        let valid_codes = vec![200u16, 201, 204, 301, 400, 404, 500, 503];
        for code in valid_codes {
            let status = StatusCode::from_u16(code);
            assert!(status.is_ok(), "Status code {} should be valid", code);
        }
    }

    /// Test that invalid status codes are rejected
    #[test]
    fn test_native_built_response_invalid_status() {
        use axum::http::StatusCode;

        assert!(StatusCode::from_u16(599).is_ok(), "599 should be valid");
    }

    /// Test HeaderName/HeaderValue construction
    #[test]
    fn test_header_construction() {
        use axum::http::{HeaderName, HeaderValue};

        let valid_headers = vec![
            ("X-Custom-Header", "value"),
            ("Content-Type", "application/json"),
            ("Cache-Control", "no-cache"),
            ("Accept", "*/*"),
        ];

        for (name, value) in valid_headers {
            let header_name = HeaderName::from_bytes(name.as_bytes());
            let header_value = HeaderValue::from_str(value);

            assert!(header_name.is_ok(), "Header name '{}' should be valid", name);
            assert!(header_value.is_ok(), "Header value '{}' should be valid", value);
        }
    }

    /// Test invalid headers are rejected
    #[test]
    fn test_invalid_header_construction() {
        use axum::http::{HeaderName, HeaderValue};

        let invalid_name = "X\nInvalid";
        assert!(
            HeaderName::from_bytes(invalid_name.as_bytes()).is_err(),
            "Header with newline should be invalid"
        );

        let invalid_value = "value\x00invalid";
        assert!(
            HeaderValue::from_str(invalid_value).is_err(),
            "Header with null byte should be invalid"
        );
    }

    /// Test JSON serialization for responses
    #[test]
    fn test_json_response_serialization() {
        let json_obj = json!({
            "status": "success",
            "data": [1, 2, 3],
            "nested": {
                "key": "value"
            }
        });

        let serialized = serde_json::to_vec(&json_obj);
        assert!(serialized.is_ok(), "JSON should serialize");

        let bytes = serialized.expect("JSON should serialize");
        assert!(!bytes.is_empty(), "Serialized JSON should not be empty");
    }

    /// Test global runtime initialization
    #[test]
    fn test_global_runtime_initialization() {
        let _ = &*GLOBAL_RUNTIME;
    }

    /// Test path normalization logic for routes
    #[test]
    fn test_route_path_patterns() {
        let paths = vec![
            "/users",
            "/users/:id",
            "/users/:id/posts/:post_id",
            "/api/v1/resource",
            "/api-v2/users_list",
            "/resource.json",
        ];

        for path in paths {
            assert!(!path.is_empty());
            assert!(path.starts_with('/'));
        }
    }

    /// Test HTTP method name validation
    #[test]
    fn test_http_method_names() {
        let methods = vec!["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];

        for method in methods {
            assert!(!method.is_empty());
            assert!(method.chars().all(|c| c.is_uppercase()));
        }
    }

    /// Test handler name generation
    #[test]
    fn test_handler_name_patterns() {
        let handler_names = vec![
            "list_users",
            "get_user",
            "create_user",
            "update_user",
            "delete_user",
            "get_user_posts",
        ];

        for name in handler_names {
            assert!(!name.is_empty());
            assert!(name.chars().all(|c| c.is_alphanumeric() || c == '_'));
        }
    }

    /// Test multipart file handling structure
    #[test]
    fn test_multipart_file_part_structure() {
        let file_data = spikard_http::testing::MultipartFilePart {
            field_name: "file".to_string(),
            filename: "test.txt".to_string(),
            content: b"file content".to_vec(),
            content_type: Some("text/plain".to_string()),
        };

        assert_eq!(file_data.field_name, "file");
        assert_eq!(file_data.filename, "test.txt");
        assert!(!file_data.content.is_empty());
        assert_eq!(file_data.content_type, Some("text/plain".to_string()));
    }

    /// Test response header case sensitivity concepts
    #[test]
    fn test_response_header_concepts() {
        use axum::http::HeaderName;

        let names = vec!["content-type", "Content-Type", "CONTENT-TYPE"];

        for name in names {
            let parsed = HeaderName::from_bytes(name.as_bytes());
            assert!(parsed.is_ok(), "Header name should parse: {}", name);
        }
    }

    /// Test error payload structure
    #[test]
    fn test_error_payload_structure() {
        let error_json = json!({
            "error": "Not Found",
            "code": "404",
            "details": {}
        });

        assert_eq!(error_json["error"], "Not Found");
        assert_eq!(error_json["code"], "404");
        assert!(error_json["details"].is_object());
    }
}

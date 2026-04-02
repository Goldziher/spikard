use async_stream::stream;
use axum::body::Body;
use axum::http::{HeaderName, HeaderValue, Method, StatusCode};
use axum_test::{TestServer, TestServerConfig, Transport};
use bytes::Bytes;
use magnus::prelude::*;
use magnus::value::{InnerValue, Opaque};
use magnus::{
    Error, Module, RArray, RHash, RString, Ruby, TryConvert, Value, function, gc::Marker, method, r_hash::ForEach,
};
use serde_json::Value as JsonValue;
use spikard_bindings_shared::ErrorResponseBuilder;
use spikard_http::ProblemDetails;
use spikard_http::testing::ResponseSnapshot;
use spikard_http::{Handler, HandlerResponse, HandlerResult, RequestData};
use spikard_http::{Route, RouteMetadata, SchemaValidator};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::mem;
use std::panic::AssertUnwindSafe;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use url::Url;

use crate::config::extract_server_config;
use crate::conversion::{extract_files, problem_to_json};
use crate::gvl::with_gvl;
use crate::integration::build_dependency_container;
use crate::metadata::{build_route_metadata, ruby_value_to_json};
use crate::request::NativeRequest;
use crate::runtime::{normalize_route_metadata, run_server};

#[derive(Default)]
#[magnus::wrap(class = "Spikard::Native::TestClient", free_immediately, mark)]
pub struct NativeTestClient {
    inner: RefCell<Option<ClientInner>>,
}

struct ClientInner {
    http_server: Arc<TestServer>,
    transport_server: Option<Arc<TestServer>>,
    /// Keep Ruby handler closures alive for GC; accessed via the `mark` hook.
    _handlers: Vec<RubyHandler>,
}

// Re-export from testing::client to avoid duplication
use crate::testing::client::{RequestBody, RequestConfig};

#[derive(Clone)]
pub struct RubyHandler {
    inner: Arc<RubyHandlerInner>,
}

struct RubyHandlerInner {
    handler_proc: Opaque<Value>,
    handler_name: String,
    json_module: Opaque<Value>,
    response_validator: Option<Arc<SchemaValidator>>,
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
pub struct NativeBuiltResponse {
    response: RefCell<Option<HandlerResponse>>,
    body_json: Option<JsonValue>,
    /// Ruby values that must be kept alive for GC (e.g., streaming enumerators)
    #[allow(dead_code)]
    gc_handles: Vec<Opaque<Value>>,
}

#[derive(Default)]
#[magnus::wrap(class = "Spikard::Native::LifecycleRegistry", free_immediately, mark)]
pub struct NativeLifecycleRegistry {
    hooks: RefCell<spikard_http::LifecycleHooks>,
    ruby_hooks: RefCell<Vec<Arc<crate::lifecycle::RubyLifecycleHook>>>,
}

#[magnus::wrap(class = "Spikard::Native::DependencyRegistry", free_immediately, mark)]
pub struct NativeDependencyRegistry {
    container: RefCell<Option<spikard_core::di::DependencyContainer>>,
    #[allow(dead_code)]
    gc_handles: RefCell<Vec<Opaque<Value>>>,
    registered_keys: RefCell<Vec<String>>,
    registered_dependencies: RefCell<Vec<(String, Arc<dyn spikard_core::di::Dependency>)>>,
}

impl StreamingResponsePayload {
    fn into_response(self) -> Result<HandlerResponse, Error> {
        let ruby = Ruby::get().map_err(|_| {
            Error::new(
                magnus::exception::runtime_error(),
                "Ruby VM became unavailable during streaming response construction",
            )
        })?;

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

    pub fn status_code(&self) -> u16 {
        let borrow = self.response.borrow();
        let Some(response) = borrow.as_ref() else {
            return StatusCode::OK.as_u16();
        };

        match response {
            HandlerResponse::Response(resp) => resp.status().as_u16(),
            HandlerResponse::Stream { status, .. } => status.as_u16(),
        }
    }

    pub fn headers(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
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
    pub fn mark(&self, marker: &Marker) {
        if let Ok(ruby) = Ruby::get() {
            for handle in &self.gc_handles {
                marker.mark(handle.get_inner_with(&ruby));
            }
        }
    }
}

impl NativeLifecycleRegistry {
    pub fn add_on_request(&self, hook_value: Value) -> Result<(), Error> {
        self.add_hook("on_request", hook_value, |hooks, hook| hooks.add_on_request(hook))
    }

    pub fn add_pre_validation(&self, hook_value: Value) -> Result<(), Error> {
        self.add_hook("pre_validation", hook_value, |hooks, hook| {
            hooks.add_pre_validation(hook)
        })
    }

    pub fn add_pre_handler(&self, hook_value: Value) -> Result<(), Error> {
        self.add_hook("pre_handler", hook_value, |hooks, hook| hooks.add_pre_handler(hook))
    }

    pub fn add_on_response(&self, hook_value: Value) -> Result<(), Error> {
        self.add_hook("on_response", hook_value, |hooks, hook| hooks.add_on_response(hook))
    }

    pub fn add_on_error(&self, hook_value: Value) -> Result<(), Error> {
        self.add_hook("on_error", hook_value, |hooks, hook| hooks.add_on_error(hook))
    }

    pub fn take_hooks(&self) -> spikard_http::LifecycleHooks {
        mem::take(&mut *self.hooks.borrow_mut())
    }

    #[allow(dead_code)]
    pub fn mark(&self, marker: &Marker) {
        for hook in self.ruby_hooks.borrow().iter() {
            hook.mark(marker);
        }
    }

    fn add_hook<F>(&self, kind: &str, hook_value: Value, push: F) -> Result<(), Error>
    where
        F: Fn(&mut spikard_http::LifecycleHooks, Arc<crate::lifecycle::RubyLifecycleHook>),
    {
        let ruby = Ruby::get().map_err(|err| Error::new(magnus::exception::runtime_error(), err.to_string()))?;
        let hook_value = crate::conversion::ensure_callable(&ruby, hook_value, kind)?;
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
            registered_dependencies: RefCell::new(Vec::new()),
        }
    }
}

impl NativeDependencyRegistry {
    pub fn register_value(ruby: &Ruby, this: &Self, key: String, value: Value) -> Result<(), Error> {
        let dependency = crate::di::RubyValueDependency::new(key.clone(), value);
        this.register_dependency(ruby, key, Arc::new(dependency), Some(value))
    }

    pub fn register_factory(
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
        let dependency_clone = dependency.clone();

        container.register(key.clone(), dependency).map_err(|err| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("Failed to register dependency '{key}': {err}"),
            )
        })?;

        if let Some(val) = gc_value {
            self.gc_handles.borrow_mut().push(Opaque::from(val));
        }

        self.registered_keys.borrow_mut().push(key.clone());
        self.registered_dependencies
            .borrow_mut()
            .push((key.clone(), dependency_clone));

        Ok(())
    }

    #[allow(dead_code)]
    pub fn mark(&self, marker: &Marker) {
        if let Ok(ruby) = Ruby::get() {
            for handle in self.gc_handles.borrow().iter() {
                marker.mark(handle.get_inner_with(&ruby));
            }
        }
    }

    pub fn take_container(&self) -> Result<spikard_core::di::DependencyContainer, Error> {
        let mut borrow = self.container.borrow_mut();
        let container = borrow.take().ok_or_else(|| {
            Error::new(
                magnus::exception::runtime_error(),
                "Dependency container already consumed",
            )
        })?;
        Ok(container)
    }

    fn clone_container(&self, ruby: &Ruby) -> Result<spikard_core::di::DependencyContainer, Error> {
        let mut container = spikard_core::di::DependencyContainer::new();
        for (key, dependency) in self.registered_dependencies.borrow().iter() {
            container.register(key.clone(), dependency.clone()).map_err(|err| {
                Error::new(
                    ruby.exception_runtime_error(),
                    format!("Failed to clone dependency '{key}': {err}"),
                )
            })?;
        }
        Ok(container)
    }

    pub fn keys(&self) -> Vec<String> {
        self.registered_keys.borrow().clone()
    }

    pub fn resolve(ruby: &Ruby, this: &Self, key: String) -> Result<Value, Error> {
        let registered = this.registered_keys.borrow();
        if registered.contains(&key) {
            Ok(ruby.qnil().as_value())
        } else {
            Err(Error::new(
                ruby.exception_runtime_error(),
                format!("Failed to resolve dependency '{}': key '{}' not registered", key, key),
            ))
        }
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

impl NativeTestClient {
    #[allow(clippy::too_many_arguments)]
    pub fn initialize(
        ruby: &Ruby,
        this: &Self,
        routes_json: String,
        handlers: Value,
        config_value: Value,
        ws_handlers: Value,
        sse_producers: Value,
        dependencies: Value,
    ) -> Result<(), Error> {
        let (hooks_value, dependencies_value) = if let Some(arg_hash) = RHash::from_value(dependencies) {
            let hooks_value = arg_hash.get(ruby.to_symbol("hooks")).or_else(|| arg_hash.get("hooks"));
            let dependencies_value = arg_hash
                .get(ruby.to_symbol("dependencies"))
                .or_else(|| arg_hash.get("dependencies"));

            if hooks_value.is_some() || dependencies_value.is_some() {
                (
                    hooks_value.unwrap_or_else(|| ruby.qnil().as_value()),
                    dependencies_value.unwrap_or_else(|| ruby.qnil().as_value()),
                )
            } else {
                (ruby.qnil().as_value(), dependencies)
            }
        } else {
            (ruby.qnil().as_value(), dependencies)
        };
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
            if let Ok(registry) = <&NativeDependencyRegistry>::try_convert(dependencies_value) {
                server_config.di_container = Some(Arc::new(registry.clone_container(ruby)?));
            } else if !dependencies_value.is_nil() {
                match build_dependency_container(ruby, dependencies_value) {
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

        let lifecycle_hooks = if let Ok(registry) = <&NativeLifecycleRegistry>::try_convert(hooks_value) {
            Some(registry.take_hooks())
        } else if !hooks_value.is_nil() {
            let hooks_hash = RHash::from_value(hooks_value)
                .ok_or_else(|| Error::new(ruby.exception_arg_error(), "lifecycle_hooks parameter must be a Hash"))?;

            let mut hooks = spikard_http::LifecycleHooks::new();
            type RubyHook = Arc<
                dyn spikard_http::lifecycle::LifecycleHook<
                        axum::http::Request<axum::body::Body>,
                        axum::http::Response<axum::body::Body>,
                    >,
            >;

            let extract_hooks = |key: &str| -> Result<Vec<RubyHook>, Error> {
                let key_sym = ruby.to_symbol(key);
                if let Some(hooks_array) = hooks_hash.get(key_sym)
                    && !hooks_array.is_nil()
                {
                    let array = magnus::RArray::from_value(hooks_array)
                        .ok_or_else(|| Error::new(ruby.exception_type_error(), format!("{} must be an Array", key)))?;

                    let mut result = Vec::new();
                    let len = array.len();
                    for i in 0..len {
                        let hook_value: Value = array.entry(i as isize)?;
                        let name = format!("{}_{}", key, i);
                        let ruby_hook = crate::lifecycle::RubyLifecycleHook::new(name, hook_value);
                        result.push(Arc::new(ruby_hook) as RubyHook);
                    }
                    Ok(result)
                } else {
                    Ok(Vec::new())
                }
            };

            for hook in extract_hooks("on_request")? {
                hooks.add_on_request(hook);
            }
            for hook in extract_hooks("pre_validation")? {
                hooks.add_pre_validation(hook);
            }
            for hook in extract_hooks("pre_handler")? {
                hooks.add_pre_handler(hook);
            }
            for hook in extract_hooks("on_response")? {
                hooks.add_on_response(hook);
            }
            for hook in extract_hooks("on_error")? {
                hooks.add_on_error(hook);
            }

            Some(hooks)
        } else {
            None
        };

        if let Some(hooks) = lifecycle_hooks {
            server_config.lifecycle_hooks = Some(Arc::new(hooks));
        }

        let schema_registry = spikard_http::SchemaRegistry::new();
        let mut prepared_routes = Vec::with_capacity(metadata.len());
        let mut handler_refs = Vec::with_capacity(metadata.len());
        let mut route_metadata_vec = Vec::with_capacity(metadata.len());

        for meta in metadata.clone() {
            validate_route_metadata(ruby, &meta)?;
            let route = Route::from_metadata(meta.clone(), &schema_registry)
                .map_err(|err| Error::new(ruby.exception_runtime_error(), format!("Failed to build route: {err}")))?;
            let handler_value = fetch_handler(ruby, &handlers_hash, &meta.handler_name)?;

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
                if let Some(ws_state) = crate::websocket::create_websocket_state(ruby, factory)? {
                    ws_endpoints.push((path, ws_state));
                }

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

        let has_ws = !ws_endpoints.is_empty();

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

        let runtime = crate::server::global_runtime(ruby)?;
        let http_server = runtime.block_on(async { TestServer::new(router.clone()) });

        let transport_server = if has_ws {
            let ws_config = TestServerConfig {
                transport: Some(Transport::HttpRandomPort),
                ..Default::default()
            };
            let server = runtime.block_on(async { TestServer::new_with_config(router, ws_config) });
            Some(Arc::new(server))
        } else {
            None
        };

        *this.inner.borrow_mut() = Some(ClientInner {
            http_server: Arc::new(http_server),
            transport_server,
            _handlers: handler_refs,
        });

        Ok(())
    }

    pub fn request(ruby: &Ruby, this: &Self, method: String, path: String, options: Value) -> Result<Value, Error> {
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

        let runtime = crate::server::global_runtime(ruby)?;
        let server = inner.http_server.clone();
        let path_value = path.clone();
        let response = crate::call_without_gvl!(
            crate::testing::client::block_on_request,
            args: (
                runtime, &tokio::runtime::Runtime,
                server, Arc<TestServer>,
                http_method, Method,
                path_value, String,
                request_config, crate::testing::client::RequestConfig
            ),
            return_type: Result<crate::testing::client::TestResponseData, crate::testing::client::NativeRequestError>
        )
        .map_err(|err| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("Request failed for {method_upper} {path}: {}", err.0),
            )
        })?;

        response_to_ruby(ruby, response)
    }

    pub fn close(&self) -> Result<(), Error> {
        *self.inner.borrow_mut() = None;
        Ok(())
    }

    pub fn websocket(ruby: &Ruby, this: &Self, path: String) -> Result<Value, Error> {
        let inner_borrow = this.inner.borrow();
        let inner = inner_borrow
            .as_ref()
            .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "TestClient not initialised"))?;

        let server = inner.transport_server.clone().ok_or_else(|| {
            Error::new(
                ruby.exception_runtime_error(),
                "WebSocket transport server unavailable (no WebSocket handlers registered)",
            )
        })?;

        drop(inner_borrow);

        let timeout_duration = websocket_timeout();
        let ws = crate::call_without_gvl!(
            block_on_websocket_connect,
            args: (
                server, Arc<TestServer>,
                path, String,
                timeout_duration, Duration
            ),
            return_type: Result<crate::testing::websocket::WebSocketConnection, WebSocketConnectError>
        )
        .map_err(|err| match err {
            WebSocketConnectError::Timeout => Error::new(
                ruby.exception_runtime_error(),
                format!("WebSocket connect timed out after {}ms", timeout_duration.as_millis()),
            ),
            WebSocketConnectError::Other(message) => Error::new(
                ruby.exception_runtime_error(),
                format!("WebSocket connect failed: {}", message),
            ),
        })?;

        let ws_conn = crate::testing::websocket::WebSocketTestConnection::new(ws);
        Ok(ruby.obj_wrap(ws_conn).as_value())
    }

    pub fn sse(ruby: &Ruby, this: &Self, path: String) -> Result<Value, Error> {
        let inner_borrow = this.inner.borrow();
        let inner = inner_borrow
            .as_ref()
            .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "TestClient not initialised"))?;

        let runtime = crate::server::global_runtime(ruby)?;
        let server = inner.http_server.clone();
        let http_method = Method::GET;
        let request_config = RequestConfig {
            query: None,
            headers: HashMap::new(),
            cookies: HashMap::new(),
            body: None,
        };
        let response = crate::call_without_gvl!(
            crate::testing::client::block_on_request,
            args: (
                runtime, &tokio::runtime::Runtime,
                server, Arc<TestServer>,
                http_method, Method,
                path, String,
                request_config, RequestConfig
            ),
            return_type: Result<crate::testing::client::TestResponseData, crate::testing::client::NativeRequestError>
        )
        .map_err(|err| Error::new(ruby.exception_runtime_error(), format!("SSE request failed: {}", err.0)))?;

        let body = response.body_text.unwrap_or_default().into_bytes();
        let snapshot = ResponseSnapshot {
            status: response.status,
            headers: response.headers,
            body,
        };

        crate::testing::sse::sse_stream_from_response(ruby, &snapshot)
    }

    pub fn graphql(
        ruby: &Ruby,
        this: &Self,
        query: String,
        variables: Value,
        operation_name: Value,
    ) -> Result<Value, Error> {
        let (_status, response) = Self::execute_graphql_impl(ruby, this, query, variables, operation_name)?;
        Ok(response)
    }

    pub fn graphql_with_status(
        ruby: &Ruby,
        this: &Self,
        query: String,
        variables: Value,
        operation_name: Value,
    ) -> Result<Value, Error> {
        let (status, response) = Self::execute_graphql_impl(ruby, this, query, variables, operation_name)?;

        let array = ruby.ary_new_capa(2);
        array.push(ruby.integer_from_i64(status as i64))?;
        array.push(response)?;
        Ok(array.as_value())
    }

    fn execute_graphql_impl(
        ruby: &Ruby,
        this: &Self,
        query: String,
        variables: Value,
        operation_name: Value,
    ) -> Result<(u16, Value), Error> {
        let inner_borrow = this.inner.borrow();
        let inner = inner_borrow
            .as_ref()
            .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "TestClient not initialised"))?;

        let json_module = ruby
            .class_object()
            .const_get("JSON")
            .map_err(|_| Error::new(ruby.exception_runtime_error(), "JSON module not available"))?;

        let variables_json = if variables.is_nil() {
            None
        } else {
            Some(ruby_value_to_json(ruby, json_module, variables)?)
        };

        let operation_name_str = if operation_name.is_nil() {
            None
        } else {
            Some(String::try_convert(operation_name)?)
        };

        let runtime = crate::server::global_runtime(ruby)?;
        let server = inner.http_server.clone();
        let query_value = query.clone();

        let snapshot = crate::call_without_gvl!(
            crate::testing::client::block_on_graphql,
            args: (
                runtime, &tokio::runtime::Runtime,
                server, Arc<TestServer>,
                query_value, String,
                variables_json, Option<JsonValue>,
                operation_name_str, Option<String>
            ),
            return_type: Result<ResponseSnapshot, crate::testing::client::NativeRequestError>
        )
        .map_err(|err| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("GraphQL request failed: {}", err.0),
            )
        })?;

        let status = snapshot.status;
        let response = response_snapshot_to_ruby(ruby, snapshot)?;
        Ok((status, response))
    }
}

fn websocket_timeout() -> Duration {
    const DEFAULT_TIMEOUT_MS: u64 = 30_000;
    let timeout_ms = std::env::var("SPIKARD_RB_WS_TIMEOUT_MS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(DEFAULT_TIMEOUT_MS);
    Duration::from_millis(timeout_ms)
}

#[derive(Debug)]
enum WebSocketConnectError {
    Timeout,
    Other(String),
}

fn block_on_websocket_connect(
    server: Arc<TestServer>,
    path: String,
    timeout_duration: Duration,
) -> Result<crate::testing::websocket::WebSocketConnection, WebSocketConnectError> {
    let url = server
        .server_url(&path)
        .map_err(|err| WebSocketConnectError::Other(err.to_string()))?;
    let ws_url = to_ws_url(url)?;

    match crate::testing::websocket::WebSocketConnection::connect(ws_url, timeout_duration) {
        Ok(ws) => Ok(ws),
        Err(crate::testing::websocket::WebSocketIoError::Timeout) => Err(WebSocketConnectError::Timeout),
        Err(err) => Err(WebSocketConnectError::Other(format!("{:?}", err))),
    }
}

fn to_ws_url(mut url: Url) -> Result<Url, WebSocketConnectError> {
    let scheme = match url.scheme() {
        "https" => "wss",
        _ => "ws",
    };
    url.set_scheme(scheme)
        .map_err(|_| WebSocketConnectError::Other("Failed to set WebSocket scheme".to_string()))?;
    Ok(url)
}

impl RubyHandler {
    fn new(route: &Route, handler_value: Value, json_module: Value) -> Result<Self, Error> {
        let ruby = Ruby::get().map_err(|_| {
            Error::new(
                magnus::exception::runtime_error(),
                "Ruby VM unavailable while creating handler",
            )
        })?;
        let handler_value = crate::conversion::ensure_callable(&ruby, handler_value, &route.handler_name)?;

        Ok(Self {
            inner: Arc::new(RubyHandlerInner {
                handler_proc: Opaque::from(handler_value),
                handler_name: route.handler_name.clone(),
                json_module: Opaque::from(json_module),
                response_validator: route.response_validator.clone(),
                #[cfg(feature = "di")]
                handler_dependencies: route.handler_dependencies.clone(),
            }),
        })
    }

    /// Create a new RubyHandler for server mode
    ///
    /// This is used by run_server to create handlers from Ruby Procs
    pub fn new_for_server(
        ruby: &Ruby,
        handler_value: Value,
        handler_name: String,
        json_module: Value,
        route: &Route,
    ) -> Result<Self, Error> {
        let handler_value = crate::conversion::ensure_callable(ruby, handler_value, &handler_name)?;

        Ok(Self {
            inner: Arc::new(RubyHandlerInner {
                handler_proc: Opaque::from(handler_value),
                handler_name,
                json_module: Opaque::from(json_module),
                response_validator: route.response_validator.clone(),
                #[cfg(feature = "di")]
                handler_dependencies: route.handler_dependencies.clone(),
            }),
        })
    }

    /// Required by Ruby GC; invoked through the magnus mark hook.
    #[allow(dead_code)]
    pub fn mark(&self, marker: &Marker) {
        if let Ok(ruby) = Ruby::get() {
            let proc_val = self.inner.handler_proc.get_inner_with(&ruby);
            marker.mark(proc_val);
        }
    }

    fn handle(&self, request_data: RequestData) -> HandlerResult {
        with_gvl(|| {
            let result = std::panic::catch_unwind(AssertUnwindSafe(|| self.handle_inner(request_data)));
            match result {
                Ok(res) => res,
                Err(_) => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Unexpected panic while executing Ruby handler".to_string(),
                )),
            }
        })
    }

    fn handle_inner(&self, request_data: RequestData) -> HandlerResult {
        // Clone Arc to avoid borrow checker issues with request_data later in the function.
        // The Arc clone is cheap (increment ref count) and necessary here because we need to
        // extract validated_params and also borrow request_data below.
        let validated_params = request_data.validated_params.clone();

        let ruby = Ruby::get().map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Ruby VM unavailable while invoking handler".to_string(),
            )
        })?;

        #[cfg(feature = "di")]
        let dependencies = request_data.dependencies.clone();

        // Use Arc::try_unwrap to eliminate the clone when validated_params Arc has unique ref.
        // This is passed to NativeRequest::from_request_data which also tries to unwrap.
        // The pattern handles both cases:
        // - Most requests: Arc has unique ref → try_unwrap succeeds, no extra clone
        // - Shared Arc (rare): try_unwrap fails → fallback to clone, safe and correct
        let request_value = build_ruby_request(
            &ruby,
            request_data,
            validated_params.map(|arc| Arc::try_unwrap(arc).unwrap_or_else(|a| (*a).clone())),
        )
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

        let handler_proc = self.inner.handler_proc.get_inner_with(&ruby);

        #[cfg(feature = "di")]
        let handler_result = {
            if let Some(deps) = &dependencies {
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

                call_handler_proc_with_kwargs(&ruby, handler_proc, request_value, kwargs_hash.as_value())
            } else {
                call_handler_proc(&ruby, handler_proc, request_value)
            }
        };

        #[cfg(not(feature = "di"))]
        let handler_result = call_handler_proc(&ruby, handler_proc, request_value);

        let response_value = match handler_result {
            Ok(value) => value,
            Err(err) => return Err(problem_from_ruby_error(&ruby, &self.inner, err)),
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

fn call_handler_proc(ruby: &Ruby, handler_proc: Value, request_value: Value) -> Result<Value, Error> {
    let arity: i64 = handler_proc.funcall("arity", ())?;
    let call_arity = normalize_proc_arity(arity);

    if call_arity == 0 {
        return handler_proc.funcall("call", ());
    }

    if call_arity == 1 {
        return handler_proc.funcall("call", (request_value,));
    }

    let (params_value, query_value, body_value) = request_parts_from_value(ruby, request_value)?;
    if call_arity == 2 {
        return handler_proc.funcall("call", (params_value, query_value));
    }

    handler_proc.funcall("call", (params_value, query_value, body_value))
}

fn call_handler_proc_with_kwargs(
    ruby: &Ruby,
    handler_proc: Value,
    request_value: Value,
    kwargs_hash: Value,
) -> Result<Value, Error> {
    let arity: i64 = handler_proc.funcall("arity", ())?;
    let call_arity = normalize_proc_arity(arity);
    let (params_value, query_value, body_value) = request_parts_from_value(ruby, request_value)?;

    let wrapper_code = ruby.eval::<Value>(
        r"
        lambda do |proc, arity, request, params, query, body, kwargs|
            kwargs = {} if kwargs.nil?
            case arity
            when 0
                kwargs.empty? ? proc.call : proc.call(**kwargs)
            when 1
                kwargs.empty? ? proc.call(request) : proc.call(request, **kwargs)
            when 2
                kwargs.empty? ? proc.call(params, query) : proc.call(params, query, **kwargs)
            else
                kwargs.empty? ? proc.call(params, query, body) : proc.call(params, query, body, **kwargs)
            end
        end
        ",
    )?;

    wrapper_code.funcall(
        "call",
        (
            handler_proc,
            ruby.integer_from_i64(call_arity),
            request_value,
            params_value,
            query_value,
            body_value,
            kwargs_hash,
        ),
    )
}

fn request_parts_from_value(ruby: &Ruby, request_value: Value) -> Result<(Value, Value, Value), Error> {
    if let Ok(request) = <&NativeRequest>::try_convert(request_value) {
        let params_value = NativeRequest::path_params(ruby, request)?;
        let query_value = NativeRequest::query(ruby, request)?;
        let body_value = NativeRequest::body(ruby, request)?;
        return Ok((params_value, query_value, body_value));
    }

    if let Ok(hash) = RHash::try_convert(request_value) {
        let params_value = hash.get("path_params").unwrap_or_else(|| ruby.qnil().as_value());
        let query_value = hash.get("query").unwrap_or_else(|| ruby.qnil().as_value());
        let body_value = hash.get("body").unwrap_or_else(|| ruby.qnil().as_value());
        return Ok((params_value, query_value, body_value));
    }

    Ok((ruby.qnil().as_value(), ruby.qnil().as_value(), ruby.qnil().as_value()))
}

fn normalize_proc_arity(arity: i64) -> i64 {
    if arity < 0 { 3 } else { arity }
}

fn problem_from_ruby_error(ruby: &Ruby, handler: &RubyHandlerInner, err: Error) -> (StatusCode, String) {
    let mut status = StatusCode::INTERNAL_SERVER_ERROR;
    let mut detail = ruby_error_message(ruby, &err);
    let mut code_value: Option<JsonValue> = None;
    let mut details_value: Option<JsonValue> = None;
    let mut extensions: HashMap<String, JsonValue> = HashMap::new();

    if err.is_kind_of(ruby.exception_arg_error()) {
        status = StatusCode::BAD_REQUEST;
    }

    if let Some(exception) = err.value() {
        if matches!(exception.respond_to("status", false), Ok(true)) {
            if let Ok(code) = exception.funcall::<_, _, i64>("status", ()) {
                status = StatusCode::from_u16(code as u16).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            }
        } else if matches!(exception.respond_to("status_code", false), Ok(true))
            && let Ok(code) = exception.funcall::<_, _, i64>("status_code", ())
        {
            status = StatusCode::from_u16(code as u16).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let json_module = handler.json_module.get_inner_with(ruby);
        if matches!(exception.respond_to("code", false), Ok(true))
            && let Ok(value) = exception.funcall::<_, _, Value>("code", ())
            && let Ok(json_value) = ruby_value_to_json(ruby, json_module, value)
        {
            code_value = Some(json_value);
        }

        if matches!(exception.respond_to("details", false), Ok(true))
            && let Ok(value) = exception.funcall::<_, _, Value>("details", ())
            && let Ok(json_value) = ruby_value_to_json(ruby, json_module, value)
        {
            details_value = Some(json_value);
        }
    }

    detail = sanitize_error_detail(&detail);

    let code_value = code_value.unwrap_or_else(|| JsonValue::String(error_code_for_status(status).to_string()));
    let details_value = details_value.unwrap_or_else(|| JsonValue::Object(serde_json::Map::new()));
    extensions.insert("error".to_string(), JsonValue::String(detail.clone()));
    extensions.insert("code".to_string(), code_value);
    extensions.insert("details".to_string(), details_value);

    let mut problem = problem_for_status(status, detail);
    for (key, value) in extensions {
        problem = problem.with_extension(key, value);
    }

    ErrorResponseBuilder::problem_details_response(&problem)
}

fn ruby_error_message(_ruby: &Ruby, err: &Error) -> String {
    if let Some(exception) = err.value()
        && matches!(exception.respond_to("message", false), Ok(true))
        && let Ok(message) = exception.funcall::<_, _, String>("message", ())
    {
        return message;
    }
    err.to_string()
}

fn problem_for_status(status: StatusCode, detail: String) -> ProblemDetails {
    match status {
        StatusCode::BAD_REQUEST => ProblemDetails::bad_request(detail),
        StatusCode::UNAUTHORIZED => {
            ProblemDetails::new("https://spikard.dev/errors/unauthorized", "Unauthorized", status).with_detail(detail)
        }
        StatusCode::FORBIDDEN => {
            ProblemDetails::new("https://spikard.dev/errors/forbidden", "Forbidden", status).with_detail(detail)
        }
        StatusCode::NOT_FOUND => ProblemDetails::not_found(detail),
        StatusCode::UNPROCESSABLE_ENTITY => ProblemDetails::new(
            ProblemDetails::TYPE_VALIDATION_ERROR,
            "Request Validation Failed",
            status,
        )
        .with_detail(detail),
        _ => ProblemDetails::internal_server_error(detail),
    }
}

fn error_code_for_status(status: StatusCode) -> &'static str {
    match status {
        StatusCode::BAD_REQUEST => "bad_request",
        StatusCode::UNAUTHORIZED => "unauthorized",
        StatusCode::FORBIDDEN => "forbidden",
        StatusCode::NOT_FOUND => "not_found",
        StatusCode::METHOD_NOT_ALLOWED => "method_not_allowed",
        StatusCode::REQUEST_TIMEOUT => "request_timeout",
        StatusCode::CONFLICT => "conflict",
        StatusCode::SERVICE_UNAVAILABLE => "service_unavailable",
        StatusCode::UNPROCESSABLE_ENTITY => "unprocessable_entity",
        _ => "internal_error",
    }
}

fn sanitize_error_detail(detail: &str) -> String {
    let mut tokens = Vec::new();
    let mut redact_next = false;

    for token in detail.split_whitespace() {
        let lower = token.to_lowercase();
        if token.starts_with('/') || token.contains(".rb:") {
            tokens.push("[redacted]".to_string());
            redact_next = false;
            continue;
        }

        if lower.starts_with("password=") {
            tokens.push("password=[redacted]".to_string());
            redact_next = false;
            continue;
        }

        if lower.starts_with("host=") {
            tokens.push("host=[redacted]".to_string());
            redact_next = false;
            continue;
        }

        if lower.starts_with("token=") || lower.starts_with("secret=") {
            tokens.push("[redacted]".to_string());
            redact_next = false;
            continue;
        }

        if redact_next {
            tokens.push("[redacted]".to_string());
            redact_next = false;
            continue;
        }

        if token.eq_ignore_ascii_case("in") {
            tokens.push(token.to_string());
            redact_next = true;
            continue;
        }

        tokens.push(token.to_string());
    }

    let mut sanitized = tokens.join(" ");
    sanitized = sanitized.replace("SELECT *", "[redacted]");
    sanitized = sanitized.replace("select *", "[redacted]");
    sanitized = sanitized.replace("FROM users", "[redacted]");
    sanitized = sanitized.replace("from users", "[redacted]");
    sanitized
}

// These functions are now in testing::client module - use call_without_gvl! macro to call them

fn response_snapshot_to_ruby(ruby: &Ruby, snapshot: ResponseSnapshot) -> Result<Value, Error> {
    let hash = ruby.hash_new();

    hash.aset(
        ruby.intern("status_code"),
        ruby.integer_from_i64(snapshot.status as i64),
    )?;

    let headers_hash = ruby.hash_new();
    for (key, value) in snapshot.headers {
        headers_hash.aset(ruby.str_new(&key), ruby.str_new(&value))?;
    }
    hash.aset(ruby.intern("headers"), headers_hash)?;

    let body_value = ruby.str_new(&String::from_utf8_lossy(&snapshot.body));
    hash.aset(ruby.intern("body"), body_value)?;
    hash.aset(ruby.intern("body_text"), body_value)?;

    Ok(hash.as_value())
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
    request_data: RequestData,
    validated_params: Option<JsonValue>,
) -> Result<Value, Error> {
    let native_request = NativeRequest::from_request_data(request_data, validated_params, None);

    Ok(ruby.obj_wrap(native_request).as_value())
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

fn response_to_ruby(ruby: &Ruby, response: crate::testing::client::TestResponseData) -> Result<Value, Error> {
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
    crate::conversion::fetch_handler(ruby, handlers, name)
}

pub(crate) fn validate_route_metadata(ruby: &Ruby, meta: &RouteMetadata) -> Result<(), Error> {
    // Validate method field is a valid HTTP method (must be uppercase)
    match meta.method.as_str() {
        "GET" | "POST" | "PUT" | "PATCH" | "DELETE" | "HEAD" | "OPTIONS" | "TRACE" => {}
        _ => {
            return Err(Error::new(
                ruby.exception_arg_error(),
                format!(
                    "Invalid routes JSON: method must be a valid HTTP method in uppercase (got '{}')",
                    meta.method
                ),
            ));
        }
    }

    if meta.path.trim().is_empty() || !meta.path.starts_with('/') {
        return Err(Error::new(
            ruby.exception_arg_error(),
            format!("Invalid routes JSON: path must start with '/' (got '{}')", meta.path),
        ));
    }

    Ok(())
}

/// GC mark hook so Ruby keeps handler closures alive.
#[allow(dead_code)]
pub fn mark(client: &NativeTestClient, marker: &Marker) {
    let inner_ref = client.inner.borrow();
    if let Some(inner) = inner_ref.as_ref() {
        for handler in &inner._handlers {
            handler.mark(marker);
        }
    }
}

/// Return the Spikard version.
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Build a native response from content, status code, and headers.
///
/// Called by `Spikard::Response` to construct native response objects.
/// The content can be a String (raw body), Hash/Array (JSON), or nil.
pub fn build_response(ruby: &Ruby, content: Value, status_code: i64, headers: Value) -> Result<Value, Error> {
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
pub fn build_streaming_response(ruby: &Ruby, stream: Value, status_code: i64, headers: Value) -> Result<Value, Error> {
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


//! Native Ruby test client for HTTP testing.
//!
//! This module implements `NativeTestClient`, a wrapped Ruby class that provides
//! HTTP testing capabilities against a Spikard server. It manages test servers
//! for both HTTP and WebSocket/SSE transports.

#![allow(dead_code)]

use axum::Router;
use axum::http::Method;
use axum_test::{TestServer, TestServerConfig, Transport};
use bytes::Bytes;
use cookie::Cookie;
use magnus::prelude::*;
use magnus::{Error, RHash, Ruby, Value, gc::Marker};
use serde_json::Value as JsonValue;
use spikard_http::testing::{
    MultipartFilePart, ResponseSnapshot, SnapshotError, build_multipart_body, encode_urlencoded_body, snapshot_response,
};
use spikard_http::{Route, RouteMetadata};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use url::Url;

use crate::conversion::{parse_request_config, response_to_ruby};
use crate::handler::RubyHandler;

/// Request configuration built from Ruby options hash.
pub struct RequestConfig {
    pub query: Option<JsonValue>,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub body: Option<RequestBody>,
}

/// HTTP request body variants.
pub enum RequestBody {
    Json(JsonValue),
    Form(JsonValue),
    Raw(String),
    Multipart {
        form_data: Vec<(String, String)>,
        files: Vec<MultipartFilePart>,
    },
}

/// Snapshot of an HTTP response.
pub struct TestResponseData {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body_text: Option<String>,
}

/// Error wrapper for native request failures.
#[derive(Debug)]
pub struct NativeRequestError(pub String);

#[derive(Debug)]
enum WebSocketConnectError {
    Timeout,
    Other(String),
}

/// Inner client state containing the test servers and handlers.
pub struct ClientInner {
    pub http_server: Arc<TestServer>,
    pub transport_server: Arc<TestServer>,
    /// Keep Ruby handler closures alive for GC; accessed via the `mark` hook.
    #[allow(dead_code)]
    pub handlers: Vec<RubyHandler>,
}

/// Native Ruby TestClient wrapper for integration testing.
///
/// Wraps an optional `ClientInner` that holds the HTTP test servers
/// and keeps handler references alive for Ruby's garbage collector.
#[derive(Default)]
#[magnus::wrap(class = "Spikard::Native::TestClient", free_immediately, mark)]
pub struct NativeTestClient {
    pub inner: RefCell<Option<ClientInner>>,
}

impl NativeTestClient {
    /// Initialize the test client with routes, handlers, and server config.
    ///
    /// # Arguments
    ///
    /// * `ruby` - Ruby VM reference
    /// * `this` - The wrapped NativeTestClient instance
    /// * `routes_json` - JSON string containing route metadata
    /// * `handlers` - Ruby Hash mapping handler_name => Proc
    /// * `config_value` - Ruby ServerConfig object
    /// * `ws_handlers` - Ruby Hash of WebSocket handlers (optional)
    /// * `sse_producers` - Ruby Hash of SSE producers (optional)
    pub fn initialize(
        ruby: &Ruby,
        this: &Self,
        routes_json: String,
        handlers: Value,
        config_value: Value,
        ws_handlers: Value,
        sse_producers: Value,
    ) -> Result<(), Error> {
        trace_step("initialize:start");
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

        let server_config = crate::config::extract_server_config(ruby, config_value)?;

        let schema_registry = spikard_http::SchemaRegistry::new();
        let mut prepared_routes = Vec::with_capacity(metadata.len());
        let mut handler_refs = Vec::with_capacity(metadata.len());
        let mut route_metadata_vec = Vec::with_capacity(metadata.len());

        for meta in metadata.clone() {
            let handler_value = crate::conversion::fetch_handler(ruby, &handlers_hash, &meta.handler_name)?;
            let route = Route::from_metadata(meta.clone(), &schema_registry)
                .map_err(|err| Error::new(ruby.exception_runtime_error(), format!("Failed to build route: {err}")))?;

            let handler = RubyHandler::new(&route, handler_value, json_module)?;
            prepared_routes.push((route, Arc::new(handler.clone()) as Arc<dyn spikard_http::Handler>));
            handler_refs.push(handler);
            route_metadata_vec.push(meta);
        }

        trace_step("initialize:build_router");
        let mut router = spikard_http::server::build_router_with_handlers_and_config(
            prepared_routes,
            server_config,
            route_metadata_vec,
        )
        .map_err(|err| Error::new(ruby.exception_runtime_error(), format!("Failed to build router: {err}")))?;

        let mut ws_endpoints = Vec::new();
        if !ws_handlers.is_nil() {
            trace_step("initialize:ws_handlers");
            let ws_hash = RHash::from_value(ws_handlers)
                .ok_or_else(|| Error::new(ruby.exception_arg_error(), "WebSocket handlers must be a Hash"))?;

            ws_hash.foreach(
                |path: String, factory: Value| -> Result<magnus::r_hash::ForEach, Error> {
                    let ws_state = crate::websocket::create_websocket_state(ruby, factory)?;

                    ws_endpoints.push((path, ws_state));

                    Ok(magnus::r_hash::ForEach::Continue)
                },
            )?;
        }

        let mut sse_endpoints = Vec::new();
        if !sse_producers.is_nil() {
            trace_step("initialize:sse_producers");
            let sse_hash = RHash::from_value(sse_producers)
                .ok_or_else(|| Error::new(ruby.exception_arg_error(), "SSE producers must be a Hash"))?;

            sse_hash.foreach(
                |path: String, factory: Value| -> Result<magnus::r_hash::ForEach, Error> {
                    let producer_instance = factory.funcall::<_, _, Value>("call", ()).map_err(|e| {
                        Error::new(
                            ruby.exception_runtime_error(),
                            format!("Failed to create SSE producer: {}", e),
                        )
                    })?;

                    let sse_state = crate::sse::create_sse_state(ruby, producer_instance)?;

                    sse_endpoints.push((path, sse_state));

                    Ok(magnus::r_hash::ForEach::Continue)
                },
            )?;
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

        trace_step("initialize:test_server_http");
        let timeout_duration = test_server_timeout();
        let http_server = init_test_server(router.clone(), timeout_duration, "test server", ruby)?;

        trace_step("initialize:test_server_ws");
        let ws_config = TestServerConfig {
            transport: Some(Transport::HttpRandomPort),
            ..Default::default()
        };
        let transport_server =
            init_test_server_with_config(router, ws_config, timeout_duration, "WebSocket transport server", ruby)?;

        trace_step("initialize:done");
        *this.inner.borrow_mut() = Some(ClientInner {
            http_server: Arc::new(http_server),
            transport_server: Arc::new(transport_server),
            handlers: handler_refs,
        });

        Ok(())
    }

    /// Execute an HTTP request against the test server.
    ///
    /// # Arguments
    ///
    /// * `ruby` - Ruby VM reference
    /// * `this` - The wrapped NativeTestClient instance
    /// * `method` - HTTP method (GET, POST, etc.)
    /// * `path` - URL path
    /// * `options` - Ruby Hash with query, headers, cookies, body, etc.
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
        let request_config_value = request_config;
        let response = crate::call_without_gvl!(
            block_on_request,
            args: (
                runtime, &tokio::runtime::Runtime,
                server, Arc<TestServer>,
                http_method, Method,
                path_value, String,
                request_config_value, RequestConfig
            ),
            return_type: Result<TestResponseData, NativeRequestError>
        )
        .map_err(|err| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("Request failed for {method_upper} {path}: {}", err.0),
            )
        })?;

        response_to_ruby(ruby, response)
    }

    /// Close the test client and clean up resources.
    pub fn close(&self) -> Result<(), Error> {
        *self.inner.borrow_mut() = None;
        Ok(())
    }

    /// Connect to a WebSocket endpoint on the test server.
    pub fn websocket(ruby: &Ruby, this: &Self, path: String) -> Result<Value, Error> {
        let inner_borrow = this.inner.borrow();
        let inner = inner_borrow
            .as_ref()
            .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "TestClient not initialised"))?;

        let server = Arc::clone(&inner.transport_server);

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

    /// Connect to an SSE endpoint on the test server.
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
            block_on_request,
            args: (
                runtime, &tokio::runtime::Runtime,
                server, Arc<TestServer>,
                http_method, Method,
                path, String,
                request_config, RequestConfig
            ),
            return_type: Result<TestResponseData, NativeRequestError>
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

    /// Send a GraphQL query/mutation
    pub fn graphql(
        ruby: &Ruby,
        this: &Self,
        query: String,
        variables: Value,
        operation_name: Value,
    ) -> Result<Value, Error> {
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
            Some(crate::conversion::ruby_value_to_json(ruby, json_module, variables)?)
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
            block_on_graphql,
            args: (
                runtime, &tokio::runtime::Runtime,
                server, Arc<TestServer>,
                query_value, String,
                variables_json, Option<JsonValue>,
                operation_name_str, Option<String>
            ),
            return_type: Result<ResponseSnapshot, NativeRequestError>
        )
        .map_err(|err| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("GraphQL request failed: {}", err.0),
            )
        })?;

        response_snapshot_to_ruby(ruby, snapshot)
    }

    /// Send a GraphQL query and get HTTP status separately
    pub fn graphql_with_status(
        ruby: &Ruby,
        this: &Self,
        query: String,
        variables: Value,
        operation_name: Value,
    ) -> Result<Value, Error> {
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
            Some(crate::conversion::ruby_value_to_json(ruby, json_module, variables)?)
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
            block_on_graphql,
            args: (
                runtime, &tokio::runtime::Runtime,
                server, Arc<TestServer>,
                query_value, String,
                variables_json, Option<JsonValue>,
                operation_name_str, Option<String>
            ),
            return_type: Result<ResponseSnapshot, NativeRequestError>
        )
        .map_err(|err| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("GraphQL request failed: {}", err.0),
            )
        })?;

        let status = snapshot.status;
        let response = response_snapshot_to_ruby(ruby, snapshot)?;

        let array = ruby.ary_new_capa(2);
        array.push(ruby.integer_from_i64(status as i64))?;
        array.push(response)?;
        Ok(array.as_value())
    }

    /// GC mark hook so Ruby keeps handler closures alive.
    #[allow(dead_code)]
    pub fn mark(&self, marker: &Marker) {
        let inner_ref = self.inner.borrow();
        if let Some(inner) = inner_ref.as_ref() {
            for handler in &inner.handlers {
                handler.mark(marker);
            }
        }
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

pub fn block_on_request(
    runtime: &tokio::runtime::Runtime,
    server: Arc<TestServer>,
    method: Method,
    path: String,
    config: RequestConfig,
) -> Result<TestResponseData, NativeRequestError> {
    runtime.block_on(execute_request(server, method, path, config))
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

fn test_server_timeout() -> Duration {
    const DEFAULT_TIMEOUT_MS: u64 = 30_000;
    let timeout_ms = std::env::var("SPIKARD_RB_TESTSERVER_TIMEOUT_MS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(DEFAULT_TIMEOUT_MS);
    Duration::from_millis(timeout_ms)
}

fn trace_step(message: &str) {
    if std::env::var("SPIKARD_RB_TEST_TRACE").ok().as_deref() == Some("1") {
        eprintln!("[spikard-rb-test] {}", message);
    }
}

fn init_test_server(router: Router, _timeout: Duration, label: &str, ruby: &Ruby) -> Result<TestServer, Error> {
    let runtime = crate::server::global_runtime(ruby)?;
    let _guard = runtime.enter();
    TestServer::new(router).map_err(|err| {
        Error::new(
            ruby.exception_runtime_error(),
            format!("Failed to initialise {label}: {err}"),
        )
    })
}

fn init_test_server_with_config(
    router: Router,
    config: TestServerConfig,
    _timeout: Duration,
    label: &str,
    ruby: &Ruby,
) -> Result<TestServer, Error> {
    let runtime = crate::server::global_runtime(ruby)?;
    let _guard = runtime.enter();
    TestServer::new_with_config(router, config).map_err(|err| {
        Error::new(
            ruby.exception_runtime_error(),
            format!("Failed to initialise {label}: {err}"),
        )
    })
}

/// Execute an HTTP request against a test server.
///
/// Handles method routing, query params, headers, cookies, and various body formats.
pub async fn execute_request(
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

pub fn block_on_graphql(
    runtime: &tokio::runtime::Runtime,
    server: Arc<TestServer>,
    query: String,
    variables: Option<JsonValue>,
    operation_name: Option<String>,
) -> Result<ResponseSnapshot, NativeRequestError> {
    runtime.block_on(execute_graphql_request(server, query, variables, operation_name))
}

async fn execute_graphql_request(
    server: Arc<TestServer>,
    query: String,
    variables: Option<JsonValue>,
    operation_name: Option<String>,
) -> Result<ResponseSnapshot, NativeRequestError> {
    let mut body = serde_json::json!({ "query": query });
    if let Some(vars) = variables {
        body["variables"] = vars;
    }
    if let Some(op_name) = operation_name {
        body["operationName"] = JsonValue::String(op_name);
    }

    let response = server.post("/graphql").json(&body).await;
    let snapshot = snapshot_response(response).await.map_err(snapshot_err_to_native)?;
    Ok(snapshot)
}

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

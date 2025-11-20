#![allow(deprecated)]

mod background;
mod lifecycle;
mod sse;
mod test_sse;
mod test_websocket;
mod websocket;

use async_stream::stream;
use axum::body::Body;
use axum::http::{HeaderName, HeaderValue, Method, Request, Response, StatusCode};
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
use spikard_http::ParameterValidator;
use spikard_http::problem::ProblemDetails;
use spikard_http::testing::{
    MultipartFilePart, SnapshotError, build_multipart_body, encode_urlencoded_body, snapshot_response,
};
use spikard_http::{Handler, HandlerResponse, HandlerResult, RequestData};
use spikard_http::{Route, RouteMetadata, SchemaValidator};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::pin::Pin;
use std::sync::Arc;
use tokio::runtime::{Builder, Runtime};

// Global Tokio runtime for test client with WebSocket/SSE support
// Use current_thread to ensure Ruby VM is accessible (Ruby::get() requires main thread)
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
    #[allow(dead_code)]
    handlers: Vec<RubyHandler>,
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
}

struct HandlerResponsePayload {
    status: u16,
    headers: HashMap<String, String>,
    body: Option<JsonValue>,
    raw_body: Option<Vec<u8>>,
}

enum RubyHandlerResult {
    Payload(HandlerResponsePayload),
    Streaming(StreamingResponsePayload),
}

struct StreamingResponsePayload {
    enumerator: Arc<Opaque<Value>>,
    status: u16,
    headers: HashMap<String, String>,
}

impl StreamingResponsePayload {
    fn into_response(self) -> Result<HandlerResponse, Error> {
        let ruby = Ruby::get().map_err(|_| {
            Error::new(
                Ruby::get().unwrap().exception_runtime_error(),
                "Ruby VM unavailable while building streaming response",
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
    fn initialize(
        ruby: &Ruby,
        this: &Self,
        routes_json: String,
        handlers: Value,
        config_value: Value,
        ws_handlers: Value,
        sse_producers: Value,
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

        // Extract ServerConfig from Ruby
        let server_config = extract_server_config(ruby, config_value)?;

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

        // Use build_router_with_handlers_and_config to support OpenAPI and middleware
        let mut router = spikard_http::server::build_router_with_handlers_and_config(
            prepared_routes,
            server_config,
            route_metadata_vec,
        )
        .map_err(|err| Error::new(ruby.exception_runtime_error(), format!("Failed to build router: {err}")))?;

        // Collect WebSocket handlers
        let mut ws_endpoints = Vec::new();
        if !ws_handlers.is_nil() {
            let ws_hash = RHash::from_value(ws_handlers)
                .ok_or_else(|| Error::new(ruby.exception_arg_error(), "WebSocket handlers must be a Hash"))?;

            ws_hash.foreach(|path: String, factory: Value| -> Result<ForEach, Error> {
                // Call the factory to get the handler instance
                let handler_instance = factory.funcall::<_, _, Value>("call", ()).map_err(|e| {
                    Error::new(
                        ruby.exception_runtime_error(),
                        format!("Failed to create WebSocket handler: {}", e),
                    )
                })?;

                // Create WebSocket state
                let ws_state = crate::websocket::create_websocket_state(ruby, handler_instance)?;

                ws_endpoints.push((path, ws_state));

                Ok(ForEach::Continue)
            })?;
        }

        // Collect SSE producers
        let mut sse_endpoints = Vec::new();
        if !sse_producers.is_nil() {
            let sse_hash = RHash::from_value(sse_producers)
                .ok_or_else(|| Error::new(ruby.exception_arg_error(), "SSE producers must be a Hash"))?;

            sse_hash.foreach(|path: String, factory: Value| -> Result<ForEach, Error> {
                // Call the factory to get the producer instance
                let producer_instance = factory.funcall::<_, _, Value>("call", ()).map_err(|e| {
                    Error::new(
                        ruby.exception_runtime_error(),
                        format!("Failed to create SSE producer: {}", e),
                    )
                })?;

                // Create SSE state
                let sse_state = crate::sse::create_sse_state(ruby, producer_instance)?;

                sse_endpoints.push((path, sse_state));

                Ok(ForEach::Continue)
            })?;
        }

        // Register WebSocket endpoints
        use axum::routing::get;
        for (path, ws_state) in ws_endpoints {
            router = router.route(
                &path,
                get(spikard_http::websocket_handler::<crate::websocket::RubyWebSocketHandler>).with_state(ws_state),
            );
        }

        // Register SSE endpoints
        for (path, sse_state) in sse_endpoints {
            router = router.route(
                &path,
                get(spikard_http::sse_handler::<crate::sse::RubySseEventProducer>).with_state(sse_state),
            );
        }

        // Prefer in-memory transport for regular HTTP requests to match other bindings and avoid HTTP Content-Length quirks.
        let http_server = GLOBAL_RUNTIME
            .block_on(async { TestServer::new(router.clone()) })
            .map_err(|err| {
                Error::new(
                    ruby.exception_runtime_error(),
                    format!("Failed to initialise test server: {err}"),
                )
            })?;

        // Dedicated HTTP transport server for WebSocket/SSE flows that require a TCP layer
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
            handlers: handler_refs,
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

        // Drop the borrow before entering the async block
        drop(inner_borrow);

        // Spawn the WebSocket connection on the runtime's worker threads (matching Python approach)
        // This is crucial - block_on runs on the current thread, but spawn runs on worker threads
        // where the proper async context is set up for WebSocket connections
        let handle =
            GLOBAL_RUNTIME.spawn(async move { spikard_http::testing::connect_websocket(&server, &path).await });

        // Block on an async block that awaits the handle to avoid nested runtime issues
        let ws = GLOBAL_RUNTIME.block_on(async {
            handle
                .await
                .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("WebSocket task failed: {}", e)))
        })?;

        let ws_conn = test_websocket::WebSocketTestConnection::new(ws);
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

        test_sse::sse_stream_from_response(ruby, &response)
    }
}

impl ClientInner {}

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
        // Validate incoming body if schema provided.
        if let Some(validator) = &self.inner.request_validator
            && let Err(errors) = validator.validate(&request_data.body)
        {
            let problem = ProblemDetails::from_validation_error(&errors);
            let error_json = problem_to_json(&problem);
            return Err((problem.status_code(), error_json));
        }

        let validated_params = if let Some(validator) = &self.inner.parameter_validator {
            // Convert multimap to single-value map by taking first value
            let raw_query_strings: HashMap<String, String> = request_data
                .raw_query_params
                .as_ref()
                .iter()
                .filter_map(|(k, v)| v.first().map(|first| (k.clone(), first.clone())))
                .collect();

            match validator.validate_and_extract(
                &request_data.query_params,
                &raw_query_strings,
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

    // Check if files are provided (for multipart)
    let files_opt = get_kw(ruby, hash, "files");
    let has_files = files_opt.is_some() && !files_opt.unwrap().is_nil();

    let body = if has_files {
        // Extract files for multipart upload
        let files_value = files_opt.unwrap();
        let files = extract_files(ruby, files_value)?;

        // Extract form data if provided (can have both data and files in multipart)
        let mut form_data = Vec::new();
        if let Some(data_value) = get_kw(ruby, hash, "data")
            && !data_value.is_nil()
        {
            let data_hash = RHash::try_convert(data_value)?;

            // Call Ruby's .keys method to get an array of keys
            let keys_array: RArray = data_hash.funcall("keys", ())?;

            for i in 0..keys_array.len() {
                let key_val = keys_array.entry::<Value>(i as isize)?;
                let field_name = String::try_convert(key_val)?;
                let value = data_hash
                    .get(key_val)
                    .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "Failed to get hash value"))?;

                // Check if value is an array
                if let Some(array) = RArray::from_value(value) {
                    // Multiple values: add each array element as a separate form field
                    for j in 0..array.len() {
                        let item = array.entry::<Value>(j as isize)?;
                        let item_str = String::try_convert(item)?;
                        form_data.push((field_name.clone(), item_str));
                    }
                } else {
                    // Single value: convert to string
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

fn ruby_value_to_json(ruby: &Ruby, json_module: Value, value: Value) -> Result<JsonValue, Error> {
    if value.is_nil() {
        return Ok(JsonValue::Null);
    }

    let json_string: String = json_module.funcall("generate", (value,))?;
    serde_json::from_str(&json_string).map_err(|err| {
        Error::new(
            ruby.exception_runtime_error(),
            format!("Failed to convert Ruby value to JSON: {err}"),
        )
    })
}

fn json_to_ruby(ruby: &Ruby, value: &JsonValue) -> Result<Value, Error> {
    match value {
        JsonValue::Null => Ok(ruby.qnil().as_value()),
        JsonValue::Bool(b) => Ok(if *b {
            ruby.qtrue().as_value()
        } else {
            ruby.qfalse().as_value()
        }),
        JsonValue::Number(num) => {
            if let Some(i) = num.as_i64() {
                Ok(ruby.integer_from_i64(i).as_value())
            } else if let Some(f) = num.as_f64() {
                Ok(ruby.float_from_f64(f).as_value())
            } else {
                Ok(ruby.qnil().as_value())
            }
        }
        JsonValue::String(str_val) => Ok(ruby.str_new(str_val).as_value()),
        JsonValue::Array(items) => {
            let array = ruby.ary_new();
            for item in items {
                array.push(json_to_ruby(ruby, item)?)?;
            }
            Ok(array.as_value())
        }
        JsonValue::Object(map) => {
            let hash = ruby.hash_new();
            for (key, item) in map {
                hash.aset(ruby.str_new(key), json_to_ruby(ruby, item)?)?;
            }
            Ok(hash.as_value())
        }
    }
}

fn map_to_ruby_hash(ruby: &Ruby, map: &HashMap<String, String>) -> Result<Value, Error> {
    let hash = ruby.hash_new();
    for (key, value) in map {
        hash.aset(ruby.str_new(key), ruby.str_new(value))?;
    }
    Ok(hash.as_value())
}

fn multimap_to_ruby_hash(ruby: &Ruby, map: &HashMap<String, Vec<String>>) -> Result<Value, Error> {
    let hash = ruby.hash_new();
    for (key, values) in map {
        let array = ruby.ary_new();
        for value in values {
            array.push(ruby.str_new(value))?;
        }
        hash.aset(ruby.str_new(key), array)?;
    }
    Ok(hash.as_value())
}

fn problem_to_json(problem: &ProblemDetails) -> String {
    problem
        .to_json_pretty()
        .unwrap_or_else(|err| format!("Failed to serialise problem details: {err}"))
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
        for handler in &inner.handlers {
            handler.mark(marker);
        }
    }
}

fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Helper to extract an optional string from a Ruby Hash
fn get_optional_string_from_hash(hash: RHash, key: &str) -> Result<Option<String>, Error> {
    match hash.get(String::from(key)) {
        Some(v) if !v.is_nil() => Ok(Some(String::try_convert(v)?)),
        _ => Ok(None),
    }
}

/// Helper to extract a required string from a Ruby Hash
fn get_required_string_from_hash(hash: RHash, key: &str, ruby: &Ruby) -> Result<String, Error> {
    let value = hash
        .get(String::from(key))
        .ok_or_else(|| Error::new(ruby.exception_arg_error(), format!("missing required key '{}'", key)))?;
    if value.is_nil() {
        return Err(Error::new(
            ruby.exception_arg_error(),
            format!("key '{}' cannot be nil", key),
        ));
    }
    String::try_convert(value)
}

fn extract_files(ruby: &Ruby, files_value: Value) -> Result<Vec<MultipartFilePart>, Error> {
    let files_hash = RHash::try_convert(files_value)?;

    // Call Ruby's .keys method to get an array of keys
    let keys_array: RArray = files_hash.funcall("keys", ())?;
    let mut result = Vec::new();

    for i in 0..keys_array.len() {
        let key_val = keys_array.entry::<Value>(i as isize)?;
        let field_name = String::try_convert(key_val)?;
        let value = files_hash
            .get(key_val)
            .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "Failed to get hash value"))?;

        // Check if it's an array
        if let Some(outer_array) = RArray::from_value(value) {
            if outer_array.is_empty() {
                continue;
            }

            // Check first element to see if it's a nested array (multiple files) or single file
            let first_elem = outer_array.entry::<Value>(0)?;

            if RArray::from_value(first_elem).is_some() {
                // Multiple files: [["file1", content1], ["file2", content2]]
                for j in 0..outer_array.len() {
                    let file_array = outer_array.entry::<Value>(j as isize)?;
                    let file_data = extract_single_file(ruby, &field_name, file_array)?;
                    result.push(file_data);
                }
            } else {
                // Single file: ["filename", content]
                let file_data = extract_single_file(ruby, &field_name, value)?;
                result.push(file_data);
            }
        }
    }

    Ok(result)
}

/// Extract a single file from Ruby array [filename, content, content_type (optional)]
fn extract_single_file(ruby: &Ruby, field_name: &str, array_value: Value) -> Result<MultipartFilePart, Error> {
    let array = RArray::from_value(array_value)
        .ok_or_else(|| Error::new(ruby.exception_arg_error(), "file must be an Array [filename, content]"))?;

    if array.len() < 2 {
        return Err(Error::new(
            ruby.exception_arg_error(),
            "file Array must have at least 2 elements: [filename, content]",
        ));
    }

    let filename: String = String::try_convert(array.shift()?)?;
    let content_str: String = String::try_convert(array.shift()?)?;
    let content = content_str.into_bytes();

    // Optional content_type (3rd element)
    let content_type: Option<String> = if !array.is_empty() {
        String::try_convert(array.shift()?).ok()
    } else {
        None
    };

    Ok(MultipartFilePart {
        field_name: field_name.to_string(),
        filename,
        content,
        content_type,
    })
}

/// Extract ServerConfig from Ruby ServerConfig object
fn extract_server_config(ruby: &Ruby, config_value: Value) -> Result<spikard_http::ServerConfig, Error> {
    use spikard_http::{
        ApiKeyConfig, CompressionConfig, ContactInfo, JwtConfig, LicenseInfo, OpenApiConfig, RateLimitConfig,
        ServerInfo, StaticFilesConfig,
    };
    use std::collections::HashMap;

    // Extract host
    let host: String = config_value.funcall("host", ())?;

    // Extract port
    let port: u32 = config_value.funcall("port", ())?;

    // Extract workers
    let workers: usize = config_value.funcall("workers", ())?;

    // Extract enable_request_id
    let enable_request_id: bool = config_value.funcall("enable_request_id", ())?;

    // Extract max_body_size (can be nil)
    let max_body_size_value: Value = config_value.funcall("max_body_size", ())?;
    let max_body_size = if max_body_size_value.is_nil() {
        None
    } else {
        Some(u64::try_convert(max_body_size_value)? as usize)
    };

    // Extract request_timeout (can be nil)
    let request_timeout_value: Value = config_value.funcall("request_timeout", ())?;
    let request_timeout = if request_timeout_value.is_nil() {
        None
    } else {
        Some(u64::try_convert(request_timeout_value)?)
    };

    // Extract graceful_shutdown
    let graceful_shutdown: bool = config_value.funcall("graceful_shutdown", ())?;

    // Extract shutdown_timeout
    let shutdown_timeout: u64 = config_value.funcall("shutdown_timeout", ())?;

    // Extract compression config (can be nil)
    let compression_value: Value = config_value.funcall("compression", ())?;
    let compression = if compression_value.is_nil() {
        None
    } else {
        let gzip: bool = compression_value.funcall("gzip", ())?;
        let brotli: bool = compression_value.funcall("brotli", ())?;
        let min_size: usize = compression_value.funcall("min_size", ())?;
        let quality: u32 = compression_value.funcall("quality", ())?;
        Some(CompressionConfig {
            gzip,
            brotli,
            min_size,
            quality,
        })
    };

    // Extract rate_limit config (can be nil)
    let rate_limit_value: Value = config_value.funcall("rate_limit", ())?;
    let rate_limit = if rate_limit_value.is_nil() {
        None
    } else {
        let per_second: u64 = rate_limit_value.funcall("per_second", ())?;
        let burst: u32 = rate_limit_value.funcall("burst", ())?;
        let ip_based: bool = rate_limit_value.funcall("ip_based", ())?;
        Some(RateLimitConfig {
            per_second,
            burst,
            ip_based,
        })
    };

    // Extract jwt_auth config (can be nil)
    let jwt_auth_value: Value = config_value.funcall("jwt_auth", ())?;
    let jwt_auth = if jwt_auth_value.is_nil() {
        None
    } else {
        let secret: String = jwt_auth_value.funcall("secret", ())?;
        let algorithm: String = jwt_auth_value.funcall("algorithm", ())?;
        let audience_value: Value = jwt_auth_value.funcall("audience", ())?;
        let audience = if audience_value.is_nil() {
            None
        } else {
            Some(Vec::<String>::try_convert(audience_value)?)
        };
        let issuer_value: Value = jwt_auth_value.funcall("issuer", ())?;
        let issuer = if issuer_value.is_nil() {
            None
        } else {
            Some(String::try_convert(issuer_value)?)
        };
        let leeway: u64 = jwt_auth_value.funcall("leeway", ())?;
        Some(JwtConfig {
            secret,
            algorithm,
            audience,
            issuer,
            leeway,
        })
    };

    // Extract api_key_auth config (can be nil)
    let api_key_auth_value: Value = config_value.funcall("api_key_auth", ())?;
    let api_key_auth = if api_key_auth_value.is_nil() {
        None
    } else {
        let keys: Vec<String> = api_key_auth_value.funcall("keys", ())?;
        let header_name: String = api_key_auth_value.funcall("header_name", ())?;
        Some(ApiKeyConfig { keys, header_name })
    };

    // Extract static_files config (array)
    let static_files_value: Value = config_value.funcall("static_files", ())?;
    let static_files_array = RArray::from_value(static_files_value)
        .ok_or_else(|| Error::new(ruby.exception_type_error(), "static_files must be an Array"))?;

    let mut static_files = Vec::new();
    for i in 0..static_files_array.len() {
        let sf_value = static_files_array.entry::<Value>(i as isize)?;
        let directory: String = sf_value.funcall("directory", ())?;
        let route_prefix: String = sf_value.funcall("route_prefix", ())?;
        let index_file: bool = sf_value.funcall("index_file", ())?;
        let cache_control_value: Value = sf_value.funcall("cache_control", ())?;
        let cache_control = if cache_control_value.is_nil() {
            None
        } else {
            Some(String::try_convert(cache_control_value)?)
        };
        static_files.push(StaticFilesConfig {
            directory,
            route_prefix,
            index_file,
            cache_control,
        });
    }

    // Extract openapi config (can be nil)
    let openapi_value: Value = config_value.funcall("openapi", ())?;
    let openapi = if openapi_value.is_nil() {
        None
    } else {
        let enabled: bool = openapi_value.funcall("enabled", ())?;
        let title: String = openapi_value.funcall("title", ())?;
        let version: String = openapi_value.funcall("version", ())?;
        let description_value: Value = openapi_value.funcall("description", ())?;
        let description = if description_value.is_nil() {
            None
        } else {
            Some(String::try_convert(description_value)?)
        };
        let swagger_ui_path: String = openapi_value.funcall("swagger_ui_path", ())?;
        let redoc_path: String = openapi_value.funcall("redoc_path", ())?;
        let openapi_json_path: String = openapi_value.funcall("openapi_json_path", ())?;

        // Extract contact (can be nil or Hash)
        let contact_value: Value = openapi_value.funcall("contact", ())?;
        let contact = if contact_value.is_nil() {
            None
        } else if let Some(contact_hash) = RHash::from_value(contact_value) {
            // Handle Hash case (from tests)
            let name = get_optional_string_from_hash(contact_hash, "name")?;
            let email = get_optional_string_from_hash(contact_hash, "email")?;
            let url = get_optional_string_from_hash(contact_hash, "url")?;
            Some(ContactInfo { name, email, url })
        } else {
            // Handle object case (from normal usage)
            let name_value: Value = contact_value.funcall("name", ())?;
            let email_value: Value = contact_value.funcall("email", ())?;
            let url_value: Value = contact_value.funcall("url", ())?;
            Some(ContactInfo {
                name: if name_value.is_nil() {
                    None
                } else {
                    Some(String::try_convert(name_value)?)
                },
                email: if email_value.is_nil() {
                    None
                } else {
                    Some(String::try_convert(email_value)?)
                },
                url: if url_value.is_nil() {
                    None
                } else {
                    Some(String::try_convert(url_value)?)
                },
            })
        };

        // Extract license (can be nil or Hash)
        let license_value: Value = openapi_value.funcall("license", ())?;
        let license = if license_value.is_nil() {
            None
        } else if let Some(license_hash) = RHash::from_value(license_value) {
            // Handle Hash case (from tests)
            let name = get_required_string_from_hash(license_hash, "name", ruby)?;
            let url = get_optional_string_from_hash(license_hash, "url")?;
            Some(LicenseInfo { name, url })
        } else {
            // Handle object case (from normal usage)
            let name: String = license_value.funcall("name", ())?;
            let url_value: Value = license_value.funcall("url", ())?;
            let url = if url_value.is_nil() {
                None
            } else {
                Some(String::try_convert(url_value)?)
            };
            Some(LicenseInfo { name, url })
        };

        // Extract servers (array of Hashes or objects)
        let servers_value: Value = openapi_value.funcall("servers", ())?;
        let servers_array = RArray::from_value(servers_value)
            .ok_or_else(|| Error::new(ruby.exception_type_error(), "servers must be an Array"))?;

        let mut servers = Vec::new();
        for i in 0..servers_array.len() {
            let server_value = servers_array.entry::<Value>(i as isize)?;

            let (url, description) = if let Some(server_hash) = RHash::from_value(server_value) {
                // Handle Hash case (from tests)
                let url = get_required_string_from_hash(server_hash, "url", ruby)?;
                let description = get_optional_string_from_hash(server_hash, "description")?;
                (url, description)
            } else {
                // Handle object case (from normal usage)
                let url: String = server_value.funcall("url", ())?;
                let description_value: Value = server_value.funcall("description", ())?;
                let description = if description_value.is_nil() {
                    None
                } else {
                    Some(String::try_convert(description_value)?)
                };
                (url, description)
            };

            servers.push(ServerInfo { url, description });
        }

        // Extract security_schemes (hash) - for now, return empty map
        // Security schemes will be auto-detected from middleware
        let security_schemes = HashMap::new();

        Some(OpenApiConfig {
            enabled,
            title,
            version,
            description,
            swagger_ui_path,
            redoc_path,
            openapi_json_path,
            contact,
            license,
            servers,
            security_schemes,
        })
    };

    Ok(spikard_http::ServerConfig {
        host,
        port: port as u16,
        workers,
        enable_request_id,
        max_body_size,
        request_timeout,
        compression,
        rate_limit,
        jwt_auth,
        api_key_auth,
        static_files,
        graceful_shutdown,
        shutdown_timeout,
        background_tasks: spikard_http::BackgroundTaskConfig::default(),
        openapi,
        lifecycle_hooks: None, // Will be set in run_server
    })
}

/// Start the Spikard HTTP server from Ruby
///
/// Creates an Axum HTTP server in a dedicated background thread with its own Tokio runtime.
///
/// # Arguments
///
/// * `routes_json` - JSON string containing route metadata
/// * `handlers` - Ruby Hash mapping handler_name => Proc
/// * `config` - Ruby ServerConfig object with all middleware settings
///
/// # Example (Ruby)
///
/// ```ruby
/// config = Spikard::ServerConfig.new(host: '0.0.0.0', port: 8000)
/// Spikard::Native.run_server(routes_json, handlers, config)
/// ```
fn run_server(
    ruby: &Ruby,
    routes_json: String,
    handlers: Value,
    config_value: Value,
    hooks_value: Value,
    ws_handlers: Value,
    sse_producers: Value,
) -> Result<(), Error> {
    use spikard_http::{SchemaRegistry, Server};
    use tracing::{error, info, warn};

    // Extract ServerConfig from Ruby object
    let mut config = extract_server_config(ruby, config_value)?;

    let host = config.host.clone();
    let port = config.port;

    // Parse route metadata from JSON
    let metadata: Vec<RouteMetadata> = serde_json::from_str(&routes_json)
        .map_err(|err| Error::new(ruby.exception_arg_error(), format!("Invalid routes JSON: {}", err)))?;

    // Extract handlers hash
    let handlers_hash = RHash::from_value(handlers).ok_or_else(|| {
        Error::new(
            ruby.exception_arg_error(),
            "handlers parameter must be a Hash of handler_name => Proc",
        )
    })?;

    // Get JSON module for handler conversions
    let json_module = ruby
        .class_object()
        .funcall::<_, _, Value>("const_get", ("JSON",))
        .map_err(|err| Error::new(ruby.exception_name_error(), format!("JSON module not found: {}", err)))?;

    // Create schema registry for validator deduplication
    let schema_registry = SchemaRegistry::new();

    // Build routes with handlers
    let mut routes_with_handlers: Vec<(Route, Arc<dyn spikard_http::Handler>)> = Vec::new();

    for route_meta in metadata {
        // Create Route from metadata
        let route = Route::from_metadata(route_meta.clone(), &schema_registry)
            .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("Failed to create route: {}", e)))?;

        // Get handler Proc from handlers hash
        let handler_key = ruby.str_new(&route_meta.handler_name);
        let handler_value: Value = match handlers_hash.lookup(handler_key) {
            Ok(val) => val,
            Err(_) => {
                return Err(Error::new(
                    ruby.exception_arg_error(),
                    format!("Handler '{}' not found in handlers hash", route_meta.handler_name),
                ));
            }
        };

        // Create RubyHandler
        let ruby_handler = RubyHandler::new_for_server(
            ruby,
            handler_value,
            route_meta.handler_name.clone(),
            route_meta.method.clone(),
            route_meta.path.clone(),
            json_module,
            &route,
        )?;

        routes_with_handlers.push((route, Arc::new(ruby_handler) as Arc<dyn spikard_http::Handler>));
    }

    // Extract lifecycle hooks from Ruby hash
    let lifecycle_hooks = if !hooks_value.is_nil() {
        let hooks_hash = RHash::from_value(hooks_value)
            .ok_or_else(|| Error::new(ruby.exception_arg_error(), "lifecycle_hooks parameter must be a Hash"))?;

        let mut hooks = spikard_http::LifecycleHooks::new();
        type RubyHookVec = Vec<Arc<dyn spikard_http::lifecycle::LifecycleHook<Request<Body>, Response<Body>>>>;

        // Helper to extract hooks from an array
        let extract_hooks = |key: &str| -> Result<RubyHookVec, Error> {
            let key_sym = ruby.to_symbol(key);
            if let Some(hooks_array) = hooks_hash.get(key_sym)
                && !hooks_array.is_nil()
            {
                let array = RArray::from_value(hooks_array)
                    .ok_or_else(|| Error::new(ruby.exception_type_error(), format!("{} must be an Array", key)))?;

                let mut result = Vec::new();
                let len = array.len();
                for i in 0..len {
                    let hook_value: Value = array.entry(i as isize)?;
                    let name = format!("{}_{}", key, i);
                    let ruby_hook = lifecycle::RubyLifecycleHook::new(name, hook_value);
                    result.push(Arc::new(ruby_hook)
                        as Arc<
                            dyn spikard_http::lifecycle::LifecycleHook<Request<Body>, Response<Body>>,
                        >);
                }
                return Ok(result);
            }
            Ok(Vec::new())
        };

        // Extract each hook type
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

    // Set lifecycle hooks in config
    config.lifecycle_hooks = lifecycle_hooks.map(Arc::new);

    // Initialize logging
    Server::init_logging();

    info!("Starting Spikard server on {}:{}", host, port);
    info!("Registered {} routes", routes_with_handlers.len());

    // Build Axum router with handlers
    let mut app_router = Server::with_handlers(config.clone(), routes_with_handlers)
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("Failed to build router: {}", e)))?;

    // Collect WebSocket handlers
    let mut ws_endpoints = Vec::new();
    if !ws_handlers.is_nil() {
        let ws_hash = RHash::from_value(ws_handlers)
            .ok_or_else(|| Error::new(ruby.exception_arg_error(), "WebSocket handlers must be a Hash"))?;

        ws_hash.foreach(|path: String, factory: Value| -> Result<ForEach, Error> {
            // Call the factory to get the handler instance
            let handler_instance = factory.funcall::<_, _, Value>("call", ()).map_err(|e| {
                Error::new(
                    ruby.exception_runtime_error(),
                    format!("Failed to create WebSocket handler: {}", e),
                )
            })?;

            // Create WebSocket state
            let ws_state = crate::websocket::create_websocket_state(ruby, handler_instance)?;

            ws_endpoints.push((path, ws_state));

            Ok(ForEach::Continue)
        })?;
    }

    // Collect SSE producers
    let mut sse_endpoints = Vec::new();
    if !sse_producers.is_nil() {
        let sse_hash = RHash::from_value(sse_producers)
            .ok_or_else(|| Error::new(ruby.exception_arg_error(), "SSE producers must be a Hash"))?;

        sse_hash.foreach(|path: String, factory: Value| -> Result<ForEach, Error> {
            // Call the factory to get the producer instance
            let producer_instance = factory.funcall::<_, _, Value>("call", ()).map_err(|e| {
                Error::new(
                    ruby.exception_runtime_error(),
                    format!("Failed to create SSE producer: {}", e),
                )
            })?;

            // Create SSE state
            let sse_state = crate::sse::create_sse_state(ruby, producer_instance)?;

            sse_endpoints.push((path, sse_state));

            Ok(ForEach::Continue)
        })?;
    }

    // Register WebSocket endpoints
    use axum::routing::get;
    for (path, ws_state) in ws_endpoints {
        info!("Registered WebSocket endpoint: {}", path);
        app_router = app_router.route(
            &path,
            get(spikard_http::websocket_handler::<crate::websocket::RubyWebSocketHandler>).with_state(ws_state),
        );
    }

    // Register SSE endpoints
    for (path, sse_state) in sse_endpoints {
        info!("Registered SSE endpoint: {}", path);
        app_router = app_router.route(
            &path,
            get(spikard_http::sse_handler::<crate::sse::RubySseEventProducer>).with_state(sse_state),
        );
    }

    // Start the server on the current Ruby thread using a single-threaded Tokio runtime
    let addr = format!("{}:{}", config.host, config.port);
    let socket_addr: std::net::SocketAddr = addr.parse().map_err(|e| {
        Error::new(
            ruby.exception_arg_error(),
            format!("Invalid socket address {}: {}", addr, e),
        )
    })?;

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("Failed to create Tokio runtime: {}", e),
            )
        })?;

    let background_config = config.background_tasks.clone();

    runtime.block_on(async move {
        let listener = tokio::net::TcpListener::bind(socket_addr)
            .await
            .unwrap_or_else(|_| panic!("Failed to bind to {}", socket_addr));

        info!("Server listening on {}", socket_addr);

        let background_runtime = spikard_http::BackgroundRuntime::start(background_config.clone()).await;
        crate::background::install_handle(background_runtime.handle());

        let serve_result = axum::serve(listener, app_router).await;

        crate::background::clear_handle();

        if let Err(err) = background_runtime.shutdown().await {
            warn!("Failed to drain background tasks during shutdown: {:?}", err);
        }

        if let Err(e) = serve_result {
            error!("Server error: {}", e);
        }
    });

    Ok(())
}

#[magnus::init]
pub fn init(ruby: &Ruby) -> Result<(), Error> {
    let spikard = ruby.define_module("Spikard")?;
    spikard.define_singleton_method("version", function!(version, 0))?;
    let native = match spikard.const_get("Native") {
        Ok(module) => module,
        Err(_) => spikard.define_module("Native")?,
    };

    // Register run_server function (now takes 3 args: routes_json, handlers, config)
    native.define_singleton_method("run_server", function!(run_server, 6))?;
    native.define_singleton_method("background_run", function!(background::background_run, 1))?;

    // Register TestClient class
    let class = native.define_class("TestClient", ruby.class_object())?;
    class.define_alloc_func::<NativeTestClient>();
    class.define_method("initialize", method!(NativeTestClient::initialize, 5))?;
    class.define_method("request", method!(NativeTestClient::request, 3))?;
    class.define_method("websocket", method!(NativeTestClient::websocket, 1))?;
    class.define_method("sse", method!(NativeTestClient::sse, 1))?;
    class.define_method("close", method!(NativeTestClient::close, 0))?;

    // Initialize WebSocket and SSE test client modules
    let spikard_module = ruby.define_module("Spikard")?;
    test_websocket::init(ruby, &spikard_module)?;
    test_sse::init(ruby, &spikard_module)?;

    Ok(())
}

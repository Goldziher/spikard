#![allow(deprecated)]

use axum::body::Body;
use axum::http::{HeaderName, HeaderValue, Method, StatusCode};
use axum_test::TestServer;
use bytes::Bytes;
use cookie::Cookie;
use magnus::prelude::*;
use magnus::value::{InnerValue, Opaque};
use magnus::{Error, Module, RArray, RHash, Ruby, Value, function, gc::Marker, method};
use once_cell::sync::Lazy;
use serde_json::{Map as JsonMap, Value as JsonValue};
use spikard_http::ParameterValidator;
use spikard_http::problem::ProblemDetails;
use spikard_http::server::build_router_with_handlers;
use spikard_http::{Handler, HandlerResult, RequestData};
use spikard_http::{Route, RouteMetadata, SchemaValidator};
use std::cell::RefCell;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use tokio::runtime::{Builder, Runtime};

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
    server: Arc<TestServer>,
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
        files: Vec<FileData>,
    },
}

#[derive(Debug, Clone)]
struct FileData {
    field_name: String,
    filename: String,
    content: Vec<u8>,
    content_type: Option<String>,
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
}

struct TestResponseData {
    status: u16,
    headers: HashMap<String, String>,
    body_text: Option<String>,
}

#[derive(Debug)]
struct NativeRequestError(String);

impl NativeTestClient {
    fn initialize(ruby: &Ruby, this: &Self, routes_json: String, handlers: Value) -> Result<(), Error> {
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

        let schema_registry = spikard_http::SchemaRegistry::new();
        let mut prepared_routes = Vec::with_capacity(metadata.len());
        let mut handler_refs = Vec::with_capacity(metadata.len());

        for meta in metadata {
            let handler_value = fetch_handler(ruby, &handlers_hash, &meta.handler_name)?;
            let route = Route::from_metadata(meta, &schema_registry)
                .map_err(|err| Error::new(ruby.exception_runtime_error(), format!("Failed to build route: {err}")))?;

            let handler = RubyHandler::new(&route, handler_value, json_module)?;
            prepared_routes.push((route, Arc::new(handler.clone()) as Arc<dyn spikard_http::Handler>));
            handler_refs.push(handler);
        }

        let router = build_router_with_handlers(prepared_routes)
            .map_err(|err| Error::new(ruby.exception_runtime_error(), format!("Failed to build router: {err}")))?;

        let server = TestServer::new(router).map_err(|err| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("Failed to initialise test server: {err}"),
            )
        })?;

        *this.inner.borrow_mut() = Some(ClientInner {
            server: Arc::new(server),
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
                inner.server.clone(),
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

        let payload = interpret_handler_response(&ruby, &self.inner, response_value).map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!(
                    "Failed to interpret response from '{}': {}",
                    self.inner.handler_name, err
                ),
            )
        })?;

        if let (Some(validator), Some(body)) = (&self.inner.response_validator, payload.body.as_ref())
            && let Err(errors) = validator.validate(body)
        {
            let problem = ProblemDetails::from_validation_error(&errors);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, problem_to_json(&problem)));
        }

        let mut response_builder = axum::http::Response::builder().status(payload.status);
        let mut has_content_type = false;

        for (name, value) in payload.headers.iter() {
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

        if !has_content_type && payload.body.is_some() {
            response_builder = response_builder.header(
                HeaderName::from_static("content-type"),
                HeaderValue::from_static("application/json"),
            );
        }

        let body_bytes = if let Some(json_value) = payload.body {
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
                let encoded = serde_qs::to_string(&form_value)
                    .map_err(|err| NativeRequestError(format!("Failed to encode form body: {err}")))?;
                request = request
                    .content_type("application/x-www-form-urlencoded")
                    .bytes(Bytes::from(encoded));
            }
            RequestBody::Raw(raw) => {
                request = request.bytes(Bytes::from(raw));
            }
            RequestBody::Multipart { form_data, files } => {
                let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
                let multipart_body = build_multipart_body(&form_data, &files, boundary);
                request = request
                    .content_type(&format!("multipart/form-data; boundary={}", boundary))
                    .bytes(Bytes::from(multipart_body));
            }
        }
    }

    let response = request.await;
    let status = response.status_code().as_u16();

    let mut headers = HashMap::new();
    for (key, value) in response.headers().iter() {
        if let Ok(val_str) = value.to_str() {
            headers.insert(key.as_str().to_string(), val_str.to_string());
        }
    }

    let body_bytes = response.into_bytes();
    let body_text = if body_bytes.is_empty() {
        None
    } else {
        Some(String::from_utf8_lossy(&body_bytes).into_owned())
    };

    Ok(TestResponseData {
        status,
        headers,
        body_text,
    })
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
) -> Result<HandlerResponsePayload, Error> {
    if value.is_nil() {
        return Ok(HandlerResponsePayload {
            status: 200,
            headers: HashMap::new(),
            body: None,
        });
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
        let body = if content_value.is_nil() {
            None
        } else {
            Some(ruby_value_to_json(
                ruby,
                handler.json_module.get_inner_with(ruby),
                content_value,
            )?)
        };

        return Ok(HandlerResponsePayload {
            status: status_u16,
            headers,
            body,
        });
    }

    let body_json = ruby_value_to_json(ruby, handler.json_module.get_inner_with(ruby), value)?;

    Ok(HandlerResponsePayload {
        status: 200,
        headers: HashMap::new(),
        body: Some(body_json),
    })
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

/// Extract files from Ruby hash for multipart upload
/// Expects:
/// - Single file: {"field_name" => ["filename", content_bytes, "content_type" (optional)], ...}
/// - Multiple files: {"field_name" => [["file1", content1, "type1"], ["file2", content2, "type2"]], ...}
fn extract_files(ruby: &Ruby, files_value: Value) -> Result<Vec<FileData>, Error> {
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
fn extract_single_file(ruby: &Ruby, field_name: &str, array_value: Value) -> Result<FileData, Error> {
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

    Ok(FileData {
        field_name: field_name.to_string(),
        filename,
        content,
        content_type,
    })
}

/// Build multipart/form-data body
fn build_multipart_body(form_data: &[(String, String)], files: &[FileData], boundary: &str) -> Vec<u8> {
    let mut body = Vec::new();

    // Add form fields first
    for (field_name, field_value) in form_data {
        // Boundary line
        body.extend_from_slice(b"--");
        body.extend_from_slice(boundary.as_bytes());
        body.extend_from_slice(b"\r\n");

        // Content-Disposition header (no filename for regular fields)
        body.extend_from_slice(b"Content-Disposition: form-data; name=\"");
        body.extend_from_slice(field_name.as_bytes());
        body.extend_from_slice(b"\"\r\n");

        // Empty line before content
        body.extend_from_slice(b"\r\n");

        // Field value
        body.extend_from_slice(field_value.as_bytes());

        // CRLF after content
        body.extend_from_slice(b"\r\n");
    }

    // Add files
    for file in files {
        // Boundary line
        body.extend_from_slice(b"--");
        body.extend_from_slice(boundary.as_bytes());
        body.extend_from_slice(b"\r\n");

        // Content-Disposition header
        body.extend_from_slice(b"Content-Disposition: form-data; name=\"");
        body.extend_from_slice(file.field_name.as_bytes());
        body.extend_from_slice(b"\"; filename=\"");
        body.extend_from_slice(file.filename.as_bytes());
        body.extend_from_slice(b"\"\r\n");

        // Content-Type header (if specified)
        if let Some(ref content_type) = file.content_type {
            body.extend_from_slice(b"Content-Type: ");
            body.extend_from_slice(content_type.as_bytes());
            body.extend_from_slice(b"\r\n");
        }

        // Empty line before content
        body.extend_from_slice(b"\r\n");

        // File content
        body.extend_from_slice(&file.content);

        // CRLF after content
        body.extend_from_slice(b"\r\n");
    }

    // Final boundary
    body.extend_from_slice(b"--");
    body.extend_from_slice(boundary.as_bytes());
    body.extend_from_slice(b"--\r\n");

    body
}

/// Start the Spikard HTTP server from Ruby
///
/// Creates an Axum HTTP server in a dedicated background thread with its own Tokio runtime.
///
/// # Arguments
///
/// * `routes_json` - JSON string containing route metadata
/// * `handlers` - Ruby Hash mapping handler_name => Proc
/// * `host` - Host address to bind to (default: "127.0.0.1")
/// * `port` - Port number to listen on (default: 8000)
///
/// # Example (Ruby)
///
/// ```ruby
/// Spikard::Native.run_server(routes_json, handlers, '0.0.0.0', 8000)
/// ```
fn run_server(
    ruby: &Ruby,
    routes_json: String,
    handlers: Value,
    host: Option<String>,
    port: Option<u32>,
) -> Result<(), Error> {
    use spikard_http::{SchemaRegistry, Server, ServerConfig};
    use tracing::{error, info};

    let host = host.unwrap_or_else(|| "127.0.0.1".to_string());
    let port = port.unwrap_or(8000) as u16;

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

    // Create server config
    let config = ServerConfig {
        host: host.clone(),
        port,
        workers: 1,
    };

    // Initialize logging
    Server::init_logging();

    info!("Starting Spikard server on {}:{}", host, port);
    info!("Registered {} routes", routes_with_handlers.len());

    // Build Axum router with handlers
    let app_router = Server::with_handlers(config.clone(), routes_with_handlers)
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("Failed to build router: {}", e)))?;

    // Start the server in a background thread with its own Tokio runtime
    let addr = format!("{}:{}", config.host, config.port);
    let socket_addr: std::net::SocketAddr = addr.parse().map_err(|e| {
        Error::new(
            ruby.exception_arg_error(),
            format!("Invalid socket address {}: {}", addr, e),
        )
    })?;

    std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

        runtime.block_on(async move {
            let listener = tokio::net::TcpListener::bind(socket_addr)
                .await
                .unwrap_or_else(|_| panic!("Failed to bind to {}", socket_addr));

            info!("Server listening on {}", socket_addr);

            if let Err(e) = axum::serve(listener, app_router).await {
                error!("Server error: {}", e);
            }
        });
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

    // Register run_server function
    native.define_singleton_method("run_server", function!(run_server, 4))?;

    // Register TestClient class
    let class = native.define_class("TestClient", ruby.class_object())?;
    class.define_alloc_func::<NativeTestClient>();
    class.define_method("initialize", method!(NativeTestClient::initialize, 2))?;
    class.define_method("request", method!(NativeTestClient::request, 3))?;
    class.define_method("close", method!(NativeTestClient::close, 0))?;

    Ok(())
}

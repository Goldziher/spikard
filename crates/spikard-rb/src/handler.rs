//! Ruby handler wrapper implementing the Handler trait.
//!
//! This module provides the `RubyHandler` struct that wraps Ruby Proc objects
//! and implements Spikard's `Handler` trait for async request processing.

#![allow(dead_code)]
#![deny(clippy::unwrap_used)]

use axum::body::Body;
use axum::http::{HeaderName, HeaderValue, Request, StatusCode};
use magnus::prelude::*;
use magnus::value::LazyId;
use magnus::value::{InnerValue, Opaque};
use magnus::{Error, RHash, RString, Ruby, TryConvert, Value, gc::Marker};
use serde_json::Value as JsonValue;
use spikard_bindings_shared::ErrorResponseBuilder;
use spikard_core::problem::ProblemDetails;
use spikard_http::SchemaValidator;
use spikard_http::{Handler, HandlerResponse, HandlerResult, RequestData};
use std::collections::HashMap;
use std::panic::AssertUnwindSafe;
use std::pin::Pin;
use std::sync::Arc;

use crate::conversion::{
    json_to_ruby, json_to_ruby_with_uploads, map_to_ruby_hash, multimap_to_ruby_hash, ruby_value_to_json,
};
use crate::gvl::with_gvl;

static KEY_METHOD: LazyId = LazyId::new("method");
static KEY_PATH: LazyId = LazyId::new("path");
static KEY_PATH_PARAMS: LazyId = LazyId::new("path_params");
static KEY_QUERY: LazyId = LazyId::new("query");
static KEY_RAW_QUERY: LazyId = LazyId::new("raw_query");
static KEY_HEADERS: LazyId = LazyId::new("headers");
static KEY_COOKIES: LazyId = LazyId::new("cookies");
static KEY_BODY: LazyId = LazyId::new("body");
static KEY_RAW_BODY: LazyId = LazyId::new("raw_body");
static KEY_PARAMS: LazyId = LazyId::new("params");

/// Response payload with status, headers, and body data.
pub struct HandlerResponsePayload {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Option<JsonValue>,
    pub raw_body: Option<Vec<u8>>,
}

/// Streaming response variant containing an enumerator and metadata.
pub struct StreamingResponsePayload {
    pub enumerator: Arc<Opaque<Value>>,
    pub status: u16,
    pub headers: HashMap<String, String>,
}

/// Handler result: either a payload or a streaming response.
pub enum RubyHandlerResult {
    Payload(HandlerResponsePayload),
    Streaming(StreamingResponsePayload),
}

impl StreamingResponsePayload {
    /// Convert streaming response into a `HandlerResponse`.
    pub fn into_response(self) -> Result<HandlerResponse, Error> {
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
        let body_stream = async_stream::stream! {
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

/// Poll a single chunk from a Ruby enumerator.
fn poll_stream_chunk(enumerator: &Arc<Opaque<Value>>) -> Result<Option<bytes::Bytes>, std::io::Error> {
    let ruby = Ruby::get().map_err(|err| std::io::Error::other(err.to_string()))?;
    let enum_value = enumerator.get_inner_with(&ruby);
    match enum_value.funcall::<_, _, Value>("next", ()) {
        Ok(chunk) => crate::conversion::ruby_value_to_bytes(chunk).map(Some),
        Err(err) => {
            if err.is_kind_of(ruby.exception_stop_iteration()) {
                Ok(None)
            } else {
                Err(std::io::Error::other(err.to_string()))
            }
        }
    }
}

/// Inner state of a Ruby handler.
pub struct RubyHandlerInner {
    pub handler_proc: Opaque<Value>,
    pub handler_name: String,
    pub method: String,
    pub path: String,
    method_value: Opaque<Value>,
    path_value: Opaque<Value>,
    pub json_module: Opaque<Value>,
    pub response_validator: Option<Arc<SchemaValidator>>,
    pub upload_file_class: Option<Opaque<Value>>,
}

/// Wrapper around a Ruby Proc that implements the Handler trait.
#[derive(Clone)]
pub struct RubyHandler {
    pub inner: Arc<RubyHandlerInner>,
}

impl RubyHandler {
    /// Create a new RubyHandler from a route and handler Proc.
    pub fn new(route: &spikard_http::Route, handler_value: Value, json_module: Value) -> Result<Self, Error> {
        let upload_file_class = if route.file_params.is_some() {
            lookup_upload_file_class()?
        } else {
            None
        };
        let method = route.method.as_str().to_string();
        let path = route.path.clone();

        let Ok(ruby) = Ruby::get() else {
            return Err(Error::new(
                magnus::exception::runtime_error(),
                "Ruby VM unavailable while creating handler",
            ));
        };
        let method_value = Opaque::from(ruby.str_new(&method).as_value());
        let path_value = Opaque::from(ruby.str_new(&path).as_value());

        Ok(Self {
            inner: Arc::new(RubyHandlerInner {
                handler_proc: Opaque::from(handler_value),
                handler_name: route.handler_name.clone(),
                method,
                path,
                method_value,
                path_value,
                json_module: Opaque::from(json_module),
                response_validator: route.response_validator.clone(),
                upload_file_class,
            }),
        })
    }

    /// Create a new RubyHandler for server mode
    ///
    /// This is used by run_server to create handlers from Ruby Procs
    pub fn new_for_server(
        _ruby: &Ruby,
        handler_value: Value,
        handler_name: String,
        method: String,
        path: String,
        json_module: Value,
        route: &spikard_http::Route,
    ) -> Result<Self, Error> {
        let upload_file_class = if route.file_params.is_some() {
            lookup_upload_file_class()?
        } else {
            None
        };
        let Ok(ruby) = Ruby::get() else {
            return Err(Error::new(
                magnus::exception::runtime_error(),
                "Ruby VM unavailable while creating handler",
            ));
        };
        let method_value = Opaque::from(ruby.str_new(&method).as_value());
        let path_value = Opaque::from(ruby.str_new(&path).as_value());

        Ok(Self {
            inner: Arc::new(RubyHandlerInner {
                handler_proc: Opaque::from(handler_value),
                handler_name,
                method,
                path,
                method_value,
                path_value,
                json_module: Opaque::from(json_module),
                response_validator: route.response_validator.clone(),
                upload_file_class,
            }),
        })
    }

    /// Required by Ruby GC; invoked through the magnus mark hook.
    #[allow(dead_code)]
    pub fn mark(&self, marker: &Marker) {
        if let Ok(ruby) = Ruby::get() {
            let proc_val = self.inner.handler_proc.get_inner_with(&ruby);
            marker.mark(proc_val);
            marker.mark(self.inner.method_value.get_inner_with(&ruby));
            marker.mark(self.inner.path_value.get_inner_with(&ruby));
        }
    }

    /// Handle a request synchronously.
    pub fn handle(&self, request_data: RequestData) -> HandlerResult {
        with_gvl(|| {
            let result = std::panic::catch_unwind(AssertUnwindSafe(|| self.handle_inner(request_data)));
            match result {
                Ok(res) => res,
                Err(_) => Err(ErrorResponseBuilder::structured_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "panic",
                    "Unexpected panic while executing Ruby handler",
                )),
            }
        })
    }

    fn handle_inner(&self, request_data: RequestData) -> HandlerResult {
        let validated_params = request_data.validated_params.clone();

        let ruby = Ruby::get().map_err(|_| {
            ErrorResponseBuilder::structured_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "ruby_vm_unavailable",
                "Ruby VM unavailable while invoking handler",
            )
        })?;

        let request_value = build_ruby_request(&ruby, &self.inner, &request_data, validated_params.as_ref())
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

        let handler_proc = self.inner.handler_proc.get_inner_with(&ruby);
        let response_value = match call_handler_proc(&ruby, handler_proc, request_value) {
            Ok(value) => value,
            Err(err) => return Err(problem_from_ruby_error(&ruby, &self.inner, err)),
        };

        let handler_result = interpret_handler_response(&ruby, &self.inner, response_value).map_err(|err| {
            ErrorResponseBuilder::structured_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "response_interpret_error",
                format!(
                    "Failed to interpret response from '{}': {}",
                    self.inner.handler_name, err
                ),
            )
        })?;

        let payload = match handler_result {
            RubyHandlerResult::Streaming(streaming) => {
                let response = streaming.into_response().map_err(|err| {
                    ErrorResponseBuilder::structured_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "streaming_response_error",
                        format!("Failed to build streaming response: {}", err),
                    )
                })?;
                return Ok(response.into_response());
            }
            RubyHandlerResult::Payload(payload) => payload,
        };

        if let Some(validator) = &self.inner.response_validator {
            let candidate_body = match payload.body.clone() {
                Some(body) => Some(body),
                None => match try_parse_raw_body(payload.raw_body.as_ref()) {
                    Ok(parsed) => parsed,
                    Err(err) => {
                        return Err(ErrorResponseBuilder::structured_error(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "response_body_decode_error",
                            err,
                        ));
                    }
                },
            };

            match candidate_body {
                Some(json_body) => {
                    if let Err(errors) = validator.validate(&json_body) {
                        let problem = ProblemDetails::from_validation_error(&errors);
                        return Err(ErrorResponseBuilder::problem_details_response(&problem));
                    }
                }
                None => {
                    return Err(ErrorResponseBuilder::structured_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "response_validation_failed",
                        "Response validator requires JSON body but handler returned raw bytes",
                    ));
                }
            }
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
        _req: Request<Body>,
        request_data: RequestData,
    ) -> Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send + '_>> {
        let handler = self.clone();
        Box::pin(async move { handler.handle(request_data) })
    }
}

fn try_parse_raw_body(raw_body: Option<&Vec<u8>>) -> Result<Option<JsonValue>, String> {
    let Some(bytes) = raw_body else {
        return Ok(None);
    };
    let text = String::from_utf8(bytes.clone()).map_err(|e| format!("Invalid UTF-8 in response body: {e}"))?;
    if text.is_empty() {
        return Ok(None);
    }
    serde_json::from_str(&text)
        .map(Some)
        .map_err(|e| format!("Failed to parse response body as JSON: {e}"))
}

fn lookup_upload_file_class() -> Result<Option<Opaque<Value>>, Error> {
    let ruby = match Ruby::get() {
        Ok(ruby) => ruby,
        Err(_) => return Ok(None),
    };

    let upload_file = ruby.eval::<Value>("Spikard::UploadFile").ok();
    Ok(upload_file.map(Opaque::from))
}

fn call_handler_proc(ruby: &Ruby, handler_proc: Value, request_value: Value) -> Result<Value, Error> {
    let arity: i64 = handler_proc.funcall("arity", ())?;
    if arity == 0 {
        return handler_proc.funcall("call", ());
    }

    if arity == 1 {
        return handler_proc.funcall("call", (request_value,));
    }

    let mut params_value = ruby.qnil().as_value();
    let mut query_value = ruby.qnil().as_value();
    let mut body_value = ruby.qnil().as_value();

    if let Some(hash) = RHash::from_value(request_value) {
        params_value = hash.get(*KEY_PATH_PARAMS).unwrap_or(ruby.qnil().as_value());
        query_value = hash.get(*KEY_QUERY).unwrap_or(ruby.qnil().as_value());
        body_value = hash.get(*KEY_BODY).unwrap_or(ruby.qnil().as_value());
    }

    if arity == 2 {
        return handler_proc.funcall("call", (params_value, query_value));
    }

    handler_proc.funcall("call", (params_value, query_value, body_value))
}

fn problem_from_ruby_error(ruby: &Ruby, handler: &RubyHandlerInner, err: Error) -> (StatusCode, String) {
    let mut status = StatusCode::INTERNAL_SERVER_ERROR;
    let mut extensions: HashMap<String, JsonValue> = HashMap::new();
    let mut detail = ruby_error_message(ruby, &err);

    if err.is_kind_of(ruby.exception_arg_error()) {
        status = StatusCode::BAD_REQUEST;
    }

    if let Some(exception) = err.value() {
        if let Ok(true) = exception.respond_to("status", false) {
            if let Ok(code) = exception.funcall::<_, _, i64>("status", ()) {
                status = StatusCode::from_u16(code as u16).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            }
        } else if let Ok(true) = exception.respond_to("status_code", false) {
            if let Ok(code) = exception.funcall::<_, _, i64>("status_code", ()) {
                status = StatusCode::from_u16(code as u16).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }

        let json_module = handler.json_module.get_inner_with(ruby);
        if let Ok(true) = exception.respond_to("code", false) {
            if let Ok(value) = exception.funcall::<_, _, Value>("code", ()) {
                if let Ok(json_value) = ruby_value_to_json(ruby, json_module, value) {
                    extensions.insert("code".to_string(), json_value);
                }
            }
        }

        if let Ok(true) = exception.respond_to("details", false) {
            if let Ok(value) = exception.funcall::<_, _, Value>("details", ()) {
                if let Ok(json_value) = ruby_value_to_json(ruby, json_module, value) {
                    extensions.insert("details".to_string(), json_value);
                }
            }
        }
    }

    detail = sanitize_error_detail(&detail);

    let mut problem = problem_for_status(status, detail);
    for (key, value) in extensions {
        problem = problem.with_extension(key, value);
    }

    ErrorResponseBuilder::problem_details_response(&problem)
}

fn ruby_error_message(_ruby: &Ruby, err: &Error) -> String {
    if let Some(exception) = err.value() {
        if let Ok(true) = exception.respond_to("message", false) {
            if let Ok(message) = exception.funcall::<_, _, String>("message", ()) {
                return message;
            }
        }
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

/// Build a Ruby Hash request object from request data.
fn build_ruby_request(
    ruby: &Ruby,
    handler: &RubyHandlerInner,
    request_data: &RequestData,
    validated_params: Option<&JsonValue>,
) -> Result<Value, Error> {
    let hash = ruby.hash_new_capa(9);

    hash.aset(*KEY_METHOD, handler.method_value.get_inner_with(ruby))?;
    hash.aset(*KEY_PATH, handler.path_value.get_inner_with(ruby))?;

    let path_params = map_to_ruby_hash(ruby, request_data.path_params.as_ref())?;
    hash.aset(*KEY_PATH_PARAMS, path_params)?;

    let query_value = json_to_ruby(ruby, &request_data.query_params)?;
    hash.aset(*KEY_QUERY, query_value)?;

    let raw_query = multimap_to_ruby_hash(ruby, request_data.raw_query_params.as_ref())?;
    hash.aset(*KEY_RAW_QUERY, raw_query)?;

    let headers = map_to_ruby_hash(ruby, request_data.headers.as_ref())?;
    hash.aset(*KEY_HEADERS, headers)?;

    let cookies = map_to_ruby_hash(ruby, request_data.cookies.as_ref())?;
    hash.aset(*KEY_COOKIES, cookies)?;

    let upload_class_value = handler.upload_file_class.as_ref().map(|cls| cls.get_inner_with(ruby));
    let body_value = json_to_ruby_with_uploads(ruby, &request_data.body, upload_class_value.as_ref())?;
    hash.aset(*KEY_BODY, body_value)?;
    if let Some(raw) = &request_data.raw_body {
        let raw_str = ruby.str_from_slice(raw);
        hash.aset(*KEY_RAW_BODY, raw_str)?;
    } else {
        hash.aset(*KEY_RAW_BODY, ruby.qnil())?;
    }

    let params_value = if let Some(validated) = validated_params {
        json_to_ruby(ruby, validated)?
    } else {
        build_default_params_from_converted(ruby, path_params, query_value, headers, cookies)?
    };
    hash.aset(*KEY_PARAMS, params_value)?;

    Ok(hash.as_value())
}

/// Build default params from already converted Ruby values, avoiding double conversion.
fn build_default_params_from_converted(
    ruby: &Ruby,
    path_params: Value,
    query: Value,
    headers: Value,
    cookies: Value,
) -> Result<Value, Error> {
    let params = ruby.hash_new();

    if let Some(hash) = RHash::from_value(path_params) {
        let _: Value = params.funcall("merge!", (hash,))?;
    }
    if let Some(hash) = RHash::from_value(query) {
        let _: Value = params.funcall("merge!", (hash,))?;
    }
    if let Some(hash) = RHash::from_value(headers) {
        let _: Value = params.funcall("merge!", (hash,))?;
    }
    if let Some(hash) = RHash::from_value(cookies) {
        let _: Value = params.funcall("merge!", (hash,))?;
    }

    Ok(params.as_value())
}

/// Interpret a Ruby handler response into our response types.
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

/// Convert a Ruby value to a string HashMap.
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

/// Check if a Ruby value is a streaming response.
fn is_streaming_response(ruby: &Ruby, value: Value) -> Result<bool, Error> {
    let stream_sym = ruby.intern("stream");
    let status_sym = ruby.intern("status_code");
    Ok(value.respond_to(stream_sym, false)? && value.respond_to(status_sym, false)?)
}

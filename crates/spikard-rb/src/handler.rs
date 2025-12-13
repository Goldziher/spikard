//! Ruby handler wrapper implementing the Handler trait.
//!
//! This module provides the `RubyHandler` struct that wraps Ruby Proc objects
//! and implements Spikard's `Handler` trait for async request processing.

#![allow(dead_code)]
#![deny(clippy::unwrap_used)]

use axum::body::Body;
use axum::http::{HeaderName, HeaderValue, Request, StatusCode};
use magnus::prelude::*;
use magnus::value::{InnerValue, Opaque};
use magnus::{Error, RHash, RString, Ruby, TryConvert, Value, gc::Marker};
use serde_json::{Map as JsonMap, Value as JsonValue};
use spikard_bindings_shared::ErrorResponseBuilder;
use spikard_core::problem::ProblemDetails;
use spikard_http::ParameterValidator;
use spikard_http::SchemaValidator;
use spikard_http::{Handler, HandlerResponse, HandlerResult, RequestData};
use std::collections::HashMap;
use std::panic::AssertUnwindSafe;
use std::pin::Pin;
use std::sync::Arc;

use crate::conversion::{
    json_to_ruby, json_to_ruby_with_uploads, map_to_ruby_hash, multimap_to_ruby_hash, ruby_value_to_json,
};

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
    pub json_module: Opaque<Value>,
    pub request_validator: Option<Arc<SchemaValidator>>,
    pub response_validator: Option<Arc<SchemaValidator>>,
    pub parameter_validator: Option<ParameterValidator>,
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
        let upload_file_class = lookup_upload_file_class()?;
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
        let upload_file_class = lookup_upload_file_class()?;
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
        }
    }

    /// Handle a request synchronously.
    pub fn handle(&self, request_data: RequestData) -> HandlerResult {
        let cloned = request_data.clone();
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| self.handle_inner(cloned)));
        match result {
            Ok(res) => res,
            Err(_) => Err(ErrorResponseBuilder::structured_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "panic",
                "Unexpected panic while executing Ruby handler",
            )),
        }
    }

    fn handle_inner(&self, request_data: RequestData) -> HandlerResult {
        if let Some(validator) = &self.inner.request_validator
            && let Err(errors) = validator.validate(&request_data.body)
        {
            let problem = ProblemDetails::from_validation_error(&errors);
            return Err(ErrorResponseBuilder::problem_details_response(&problem));
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
                    return Err(ErrorResponseBuilder::problem_details_response(&problem));
                }
            }
        } else {
            None
        };

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
        let handler_result = handler_proc.funcall("call", (request_value,));
        let response_value = match handler_result {
            Ok(value) => value,
            Err(err) => {
                return Err(ErrorResponseBuilder::structured_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "handler_failed",
                    format!("Handler '{}' failed: {}", self.inner.handler_name, err),
                ));
            }
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
                None => match try_parse_raw_body(&payload.raw_body) {
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

fn try_parse_raw_body(raw_body: &Option<Vec<u8>>) -> Result<Option<JsonValue>, String> {
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

/// Build a Ruby Hash request object from request data.
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

    let upload_class_value = handler.upload_file_class.as_ref().map(|cls| cls.get_inner_with(ruby));
    let body_value = json_to_ruby_with_uploads(ruby, &request_data.body, upload_class_value.as_ref())?;
    hash.aset(ruby.intern("body"), body_value)?;
    if let Some(raw) = &request_data.raw_body {
        let raw_str = ruby.str_from_slice(raw);
        hash.aset(ruby.intern("raw_body"), raw_str)?;
    } else {
        hash.aset(ruby.intern("raw_body"), ruby.qnil())?;
    }

    let params_value = if let Some(validated) = validated_params {
        json_to_ruby(ruby, validated)?
    } else {
        build_default_params(ruby, request_data)?
    };
    hash.aset(ruby.intern("params"), params_value)?;

    Ok(hash.as_value())
}

/// Build default params from request data path/query/headers/cookies.
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

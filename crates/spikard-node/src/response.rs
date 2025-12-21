//! HTTP response wrapper for Node.js

use async_stream::stream;
use axum::http::{HeaderName, HeaderValue, StatusCode};
use bytes::Bytes;
use napi::JsString;
use napi::ValueType;
use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunction;
use napi_derive::napi;
use once_cell::sync::Lazy;
use serde_json::Value;
use spikard_http::{HandlerResponse, testing::ResponseSnapshot};
use std::collections::HashMap;
use std::io;
use std::str::FromStr;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

/// HTTP Response wrapper
#[napi]
pub struct TestResponse {
    status: u16,
    headers: serde_json::Map<String, Value>,
    body: Vec<u8>,
}

#[napi]
impl TestResponse {
    /// Construct a response from a shared snapshot.
    pub(crate) fn from_snapshot(snapshot: ResponseSnapshot) -> Self {
        let mut header_map = serde_json::Map::new();
        for (key, value) in snapshot.headers {
            header_map.insert(key, Value::String(value));
        }

        Self {
            status: snapshot.status,
            headers: header_map,
            body: snapshot.body,
        }
    }

    /// Get the HTTP status code
    #[napi(getter)]
    pub fn status_code(&self) -> u16 {
        self.status
    }

    /// Get response headers as JSON
    #[napi]
    pub fn headers(&self) -> serde_json::Value {
        serde_json::Value::Object(self.headers.clone())
    }

    /// Get response body as text
    #[napi]
    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).to_string()
    }

    /// Parse response body as JSON
    #[napi]
    pub fn json(&self) -> napi::Result<serde_json::Value> {
        if self.body.is_empty() {
            return Ok(serde_json::Value::Null);
        }
        serde_json::from_slice(&self.body).or_else(|_| {
            let text = String::from_utf8_lossy(&self.body).to_string();
            Ok(serde_json::Value::String(text))
        })
    }

    /// Get raw response body bytes
    #[napi]
    pub fn bytes(&self) -> Buffer {
        Buffer::from(self.body.clone())
    }
}

/// Optional configuration for a streaming response.
///
/// This struct is exposed to JavaScript via napi and provides configuration
/// options when creating streaming responses from async iterators.
///
/// NOTE: Marked with #[allow(dead_code)] because the #[napi(object)] macro
/// generates access patterns that aren't visible to the Rust dead code checker,
/// though the struct is actually exposed to and used by JavaScript code.
#[napi(object)]
#[allow(dead_code)]
pub struct StreamingResponseInit {
    /// HTTP status code for the streaming response (default 200).
    pub status_code: Option<u16>,
    /// Headers to attach to the streaming response.
    pub headers: Option<HashMap<String, String>>,
}

type NextChunkFn = ThreadsafeFunction<(), Promise<IteratorChunk>, (), napi::Status, false>;
const STREAM_HANDLE_PROP: &str = "__spikard_stream_handle";

pub struct StreamingHandle {
    status_code: u16,
    headers: HashMap<String, String>,
    next_fn: NextChunkFn,
}

static STREAM_HANDLE_REGISTRY: Lazy<Mutex<HashMap<i64, StreamingHandle>>> = Lazy::new(|| Mutex::new(HashMap::new()));

/// Atomic counter for generating unique streaming handle IDs.
///
/// Used by `create_streaming_handle()` to assign monotonically increasing
/// identifiers to each streaming response, enabling correlation in the registry.
///
/// NOTE: Marked with #[allow(dead_code)] because it's used via atomic operations
/// (fetch_add) which the dead code checker doesn't recognize as a direct usage.
#[allow(dead_code)]
static STREAM_HANDLE_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Creates a streaming handle from a JavaScript async iterator.
///
/// This function is exposed to JavaScript via napi. It wraps async iterators
/// in a streaming handle structure that can be used to construct HTTP streaming responses.
///
/// NOTE: Marked with #[allow(dead_code)] because the #[napi] macro generates
/// FFI bindings that aren't visible to the Rust dead code checker, though the
/// function is actually exported and callable from JavaScript.
#[napi]
#[allow(dead_code)]
pub fn create_streaming_handle(iterator: Object, init: Option<StreamingResponseInit>) -> Result<i64> {
    let next_fn: Function<(), Promise<IteratorChunk>> = iterator
        .get_named_property("next")
        .map_err(|_| Error::from_reason("StreamingResponse requires an async iterator with a next() method"))?;

    let bound_next = next_fn.bind(iterator)?;

    let next_tsfn = bound_next
        .build_threadsafe_function()
        .build_callback(|_| Ok(()))
        .map_err(|e| Error::from_reason(format!("Failed to build stream dispatcher: {}", e)))?;

    let status_code = init.as_ref().and_then(|cfg| cfg.status_code).unwrap_or(200);
    let headers = init.and_then(|cfg| cfg.headers).unwrap_or_default();

    let handle = StreamingHandle {
        status_code,
        headers,
        next_fn: next_tsfn,
    };

    let id = STREAM_HANDLE_COUNTER.fetch_add(1, Ordering::Relaxed) as i64;
    let mut registry = STREAM_HANDLE_REGISTRY
        .lock()
        .map_err(|_| Error::from_reason("Streaming handle registry is poisoned"))?;
    registry.insert(id, handle);
    Ok(id)
}

impl StreamingHandle {
    pub fn into_handler_response(self) -> Result<HandlerResponse> {
        let status = StatusCode::from_u16(self.status_code)
            .map_err(|e| Error::from_reason(format!("Invalid streaming status code {}: {}", self.status_code, e)))?;

        let header_pairs = self
            .headers
            .into_iter()
            .map(|(name, value)| {
                let header_name = HeaderName::from_str(&name)
                    .map_err(|e| Error::from_reason(format!("Invalid streaming header '{}': {}", name, e)))?;
                let header_value = HeaderValue::from_str(&value)
                    .map_err(|e| Error::from_reason(format!("Invalid streaming header value '{}': {}", value, e)))?;
                Ok((header_name, header_value))
            })
            .collect::<Result<Vec<_>>>()?;

        let next_fn = self.next_fn;
        let body_stream = stream! {
            loop {
                let iterator_result = match next_fn.call_async(()).await {
                    Ok(promise) => match promise.await {
                        Ok(chunk) => chunk,
                        Err(err) => {
                            yield Err(Box::new(io::Error::other(err.to_string())));
                            break;
                        }
                    },
                    Err(err) => {
                        yield Err(Box::new(io::Error::other(err.to_string())));
                        break;
                    }
                };

                if iterator_result.done {
                    break;
                }

                if let Some(bytes) = iterator_result.chunk {
                    yield Ok(Bytes::from(bytes));
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

/// Value returned from JavaScript handlers.
pub enum HandlerReturnValue {
    Json(String),
    Streaming(StreamingHandle),
}

#[allow(unsafe_op_in_unsafe_fn)]
impl FromNapiValue for HandlerReturnValue {
    unsafe fn from_napi_value(env: napi::sys::napi_env, value: napi::sys::napi_value) -> Result<Self> {
        if let Ok(object) = Object::from_napi_value(env, value)
            && let Ok(handle_value) = object.get_named_property::<Unknown>(STREAM_HANDLE_PROP)
        {
            let handle_type = handle_value.get_type()?;
            if !matches!(handle_type, ValueType::Undefined | ValueType::Null)
                && let Some(handle_id) = extract_stream_handle_id(handle_value)?
            {
                let mut registry = STREAM_HANDLE_REGISTRY
                    .lock()
                    .map_err(|_| Error::from_reason("Streaming handle registry is poisoned"))?;
                if let Some(handle) = registry.remove(&handle_id) {
                    return Ok(HandlerReturnValue::Streaming(handle));
                }

                return Err(Error::from_reason(format!(
                    "Streaming handle {} not found (already consumed)",
                    handle_id
                )));
            }
        }

        let js_string = JsString::from_napi_value(env, value)?;
        let utf8 = js_string.into_utf8()?;
        let owned = utf8.as_str()?.to_owned();
        Ok(HandlerReturnValue::Json(owned))
    }
}

/// Internal struct representing an IteratorResult chunk from JavaScript.
struct IteratorChunk {
    done: bool,
    chunk: Option<Vec<u8>>,
}

#[allow(unsafe_op_in_unsafe_fn)]
impl FromNapiValue for IteratorChunk {
    unsafe fn from_napi_value(env: napi::sys::napi_env, value: napi::sys::napi_value) -> Result<Self> {
        let object = Object::from_napi_value(env, value)?;
        let done: bool = object.get_named_property("done")?;

        if done {
            return Ok(Self {
                done: true,
                chunk: None,
            });
        }

        let value_field: Unknown = object.get_named_property("value")?;
        let value_type = value_field.get_type()?;
        if matches!(value_type, ValueType::Null | ValueType::Undefined) {
            return Ok(Self {
                done: true,
                chunk: None,
            });
        }

        let bytes = extract_chunk_bytes(value_field)?;
        Ok(Self {
            done: false,
            chunk: Some(bytes),
        })
    }
}

fn extract_chunk_bytes(value: Unknown) -> Result<Vec<u8>> {
    let raw = value.value();
    unsafe {
        if let Ok(buffer) = Buffer::from_napi_value(raw.env, raw.value) {
            return Ok(buffer.to_vec());
        }
    }

    if let Ok(string) = value.coerce_to_string() {
        let utf8 = string.into_utf8()?;
        let slice = utf8.as_slice();
        return Ok(slice.to_vec());
    }

    Err(Error::from_reason(
        "StreamingResponse chunks must be strings or Buffer instances",
    ))
}

fn extract_stream_handle_id(handle_value: Unknown) -> Result<Option<i64>> {
    match handle_value.get_type()? {
        ValueType::Number => {
            let handle_id = handle_value.coerce_to_number()?.get_int64()?;
            Ok(Some(handle_id))
        }
        ValueType::Object => {
            let raw = handle_value.value();
            let handle_obj = unsafe { Object::from_napi_value(raw.env, raw.value)? };
            if let Ok(handle_value) = handle_obj.get_named_property::<Unknown>("handle")
                && let ValueType::Number = handle_value.get_type()?
            {
                let handle_id = handle_value.coerce_to_number()?.get_int64()?;
                return Ok(Some(handle_id));
            }
            Ok(None)
        }
        _ => Ok(None),
    }
}

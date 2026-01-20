//! SSE test client bindings for Ruby

use magnus::prelude::*;
use magnus::{Error, Ruby, Value, method};
use serde_json::Value as JsonValue;
use spikard_http::testing::{ResponseSnapshot, SseEvent as RustSseEvent, SseStream as RustSseStream};

/// Ruby wrapper for SSE stream
#[magnus::wrap(class = "Spikard::Native::SseStream", free_immediately)]
pub struct SseStream {
    inner: RustSseStream,
}

impl SseStream {
    pub fn new(inner: RustSseStream) -> Self {
        Self { inner }
    }

    /// Get the raw body of the SSE response
    fn body(&self) -> String {
        self.inner.body().to_string()
    }

    /// Get all events from the stream
    fn events(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let ruby_arr = ruby.ary_new();
        for event in this.inner.events() {
            let sse_event = SseEvent::new(event.clone());
            let ruby_event = ruby.obj_wrap(sse_event);
            ruby_arr.push(ruby_event)?;
        }
        Ok(ruby_arr.as_value())
    }

    /// Get events as JSON values
    fn events_as_json(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let json_events = this.inner.events_as_json().map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("Failed to parse events as JSON: {}", e),
            )
        })?;

        let ruby_arr = ruby.ary_new();
        for value in json_events {
            let ruby_val = json_to_ruby(ruby, &value)?;
            ruby_arr.push(ruby_val)?;
        }
        Ok(ruby_arr.as_value())
    }
}

/// Ruby wrapper for SSE event
#[magnus::wrap(class = "Spikard::Native::SseEvent", free_immediately)]
pub struct SseEvent {
    inner: RustSseEvent,
}

impl SseEvent {
    pub fn new(inner: RustSseEvent) -> Self {
        Self { inner }
    }

    /// Get the data field of the event
    fn data(&self) -> String {
        self.inner.data.clone()
    }

    /// Parse the event data as JSON
    fn as_json(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let value = this
            .inner
            .as_json()
            .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("Failed to parse JSON: {}", e)))?;

        json_to_ruby(ruby, &value)
    }
}

/// Create an SSE stream from a response snapshot
pub fn sse_stream_from_response(ruby: &Ruby, response: &ResponseSnapshot) -> Result<Value, Error> {
    let stream = RustSseStream::from_response(response)
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("Failed to parse SSE: {}", e)))?;

    let sse_stream = SseStream::new(stream);
    Ok(ruby.obj_wrap(sse_stream).as_value())
}

/// Helper to convert JSON to Ruby object
fn json_to_ruby(ruby: &Ruby, value: &JsonValue) -> Result<Value, Error> {
    crate::conversion::json_to_ruby(ruby, value)
}

/// Initialize SSE test client bindings
pub fn init(ruby: &Ruby, module: &magnus::RModule) -> Result<(), Error> {
    let native_module = module.define_module("Native")?;

    let sse_stream_class = native_module.define_class("SseStream", ruby.class_object())?;
    sse_stream_class.define_method("body", method!(SseStream::body, 0))?;
    sse_stream_class.define_method("events", method!(SseStream::events, 0))?;
    sse_stream_class.define_method("events_as_json", method!(SseStream::events_as_json, 0))?;

    let sse_event_class = native_module.define_class("SseEvent", ruby.class_object())?;
    sse_event_class.define_method("data", method!(SseEvent::data, 0))?;
    sse_event_class.define_method("as_json", method!(SseEvent::as_json, 0))?;

    Ok(())
}

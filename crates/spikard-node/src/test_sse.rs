//! SSE test client bindings for Node.js

use napi::bindgen_prelude::*;
use napi_derive::napi;
use spikard_http::testing::{SseEvent as RustSseEvent, SseStream as RustSseStream};

/// Node.js wrapper for SSE stream
#[napi]
pub struct SseStream {
    inner: RustSseStream,
}

impl SseStream {
    pub fn new(inner: RustSseStream) -> Self {
        Self { inner }
    }
}

#[napi]
impl SseStream {
    /// Get the raw body of the SSE response
    #[napi]
    pub fn body(&self) -> String {
        self.inner.body().to_string()
    }

    /// Get all events from the stream
    #[napi]
    pub fn events(&self) -> Vec<SseEvent> {
        self.inner
            .events()
            .iter()
            .map(|event| SseEvent::from_rust(event.clone()))
            .collect()
    }

    /// Get events as JSON values
    #[napi]
    pub fn events_as_json(&self) -> Result<Vec<serde_json::Value>> {
        self.inner.events_as_json().map_err(Error::from_reason)
    }
}

/// Node.js wrapper for SSE event
#[napi]
pub struct SseEvent {
    inner: RustSseEvent,
}

impl SseEvent {
    pub fn from_rust(event: RustSseEvent) -> Self {
        Self { inner: event }
    }
}

#[napi]
impl SseEvent {
    /// Get the data field of the event
    #[napi]
    pub fn get_data(&self) -> String {
        self.inner.data.clone()
    }

    /// Parse the event data as JSON
    #[napi]
    pub fn as_json(&self) -> Result<serde_json::Value> {
        self.inner.as_json().map_err(Error::from_reason)
    }
}

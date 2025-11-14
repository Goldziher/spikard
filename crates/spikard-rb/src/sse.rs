//! Ruby SSE producer bindings
//!
//! This module provides the bridge between Ruby blocks/procs and Rust's SSE system.
//! Uses magnus to safely call Ruby code from Rust async tasks.

use magnus::{RHash, Value, prelude::*, value::Opaque};
use serde_json::Value as JsonValue;
use spikard_http::{SseEvent, SseEventProducer};
use tracing::{debug, error};

/// Ruby implementation of SseEventProducer
pub struct RubySseEventProducer {
    /// Producer name for debugging
    name: String,
    /// Ruby proc/callable for next_event (Opaque for Send safety)
    next_event_proc: Opaque<Value>,
    /// Ruby proc/callable for on_connect (Opaque for Send safety)
    on_connect_proc: Option<Opaque<Value>>,
    /// Ruby proc/callable for on_disconnect (Opaque for Send safety)
    on_disconnect_proc: Option<Opaque<Value>>,
}

impl RubySseEventProducer {
    /// Create a new Ruby SSE event producer
    #[allow(dead_code)]
    pub fn new(
        name: String,
        next_event_proc: Value,
        on_connect_proc: Option<Value>,
        on_disconnect_proc: Option<Value>,
    ) -> Self {
        Self {
            name,
            next_event_proc: next_event_proc.into(),
            on_connect_proc: on_connect_proc.map(|v| v.into()),
            on_disconnect_proc: on_disconnect_proc.map(|v| v.into()),
        }
    }

    /// Convert Ruby value to JSON
    fn ruby_to_json(ruby: &magnus::Ruby, value: Value) -> Result<JsonValue, String> {
        if value.is_nil() {
            return Ok(JsonValue::Null);
        }

        // Get JSON module
        let json_module: Value = ruby
            .class_object()
            .const_get("JSON")
            .map_err(|e| format!("JSON module not available: {}", e))?;

        // Convert Ruby value to JSON string via JSON.generate
        let json_str: String = json_module
            .funcall("generate", (value,))
            .map_err(|e| format!("Failed to generate JSON: {}", e))?;

        // Parse JSON string to JsonValue
        serde_json::from_str(&json_str).map_err(|e| format!("Failed to parse JSON: {}", e))
    }
}

impl SseEventProducer for RubySseEventProducer {
    async fn next_event(&self) -> Option<SseEvent> {
        debug!("Ruby SSE producer '{}': next_event", self.name);

        match magnus::Ruby::get()
            .map_err(|e| format!("Failed to get Ruby: {}", e))
            .and_then(|ruby| {
                debug!("Ruby SSE producer: acquired Ruby VM");

                // Call the Ruby function
                let proc_value = ruby.get_inner(self.next_event_proc);
                let result: Value = proc_value
                    .funcall("call", ())
                    .map_err(|e| format!("Producer '{}' call failed: {}", self.name, e))?;

                debug!("Ruby SSE producer: called next_event proc");

                // Check if result is None (end of stream)
                if result.is_nil() {
                    debug!("Ruby SSE producer: received nil, ending stream");
                    return Ok(None);
                }

                // Extract SseEvent from Ruby hash
                let result_hash = if let Some(hash) = RHash::from_value(result) {
                    hash
                } else {
                    let hash_value: Value = result.funcall("to_h", ()).map_err(|e| {
                        format!(
                            "next_event must return a Hash/SseEvent convertible via to_h ({}): {}",
                            unsafe { result.classname() },
                            e
                        )
                    })?;
                    RHash::from_value(hash_value).ok_or_else(|| {
                        format!("next_event to_h must return a Hash, got {}", unsafe {
                            hash_value.classname()
                        })
                    })?
                };

                // Extract data field (required)
                let data_value = result_hash
                    .get(ruby.to_symbol("data"))
                    .ok_or_else(|| "next_event Hash must have :data key".to_string())?;

                let data_json = Self::ruby_to_json(&ruby, data_value)?;

                // Extract optional event_type field
                let event_type: Option<String> = result_hash
                    .get(ruby.to_symbol("event_type"))
                    .and_then(|v| if v.is_nil() { None } else { String::try_convert(v).ok() });

                // Extract optional id field
                let id: Option<String> = result_hash
                    .get(ruby.to_symbol("id"))
                    .and_then(|v| if v.is_nil() { None } else { String::try_convert(v).ok() });

                // Extract optional retry field
                let retry: Option<u64> = result_hash
                    .get(ruby.to_symbol("retry"))
                    .and_then(|v| if v.is_nil() { None } else { u64::try_convert(v).ok() });

                // Create Rust SseEvent
                let mut event = if let Some(et) = event_type {
                    SseEvent::with_type(et, data_json)
                } else {
                    SseEvent::new(data_json)
                };

                if let Some(id_str) = id {
                    event = event.with_id(id_str);
                }

                if let Some(retry_ms) = retry {
                    event = event.with_retry(retry_ms);
                }

                Ok(Some(event))
            }) {
            Ok(event) => event,
            Err(e) => {
                error!("Ruby error in next_event: {}", e);
                None
            }
        }
    }

    async fn on_connect(&self) {
        debug!("Ruby SSE producer '{}': on_connect", self.name);

        if let Some(on_connect_proc) = self.on_connect_proc
            && let Err(e) = magnus::Ruby::get()
                .map_err(|e| format!("Failed to get Ruby: {}", e))
                .and_then(|ruby| {
                    debug!("Ruby SSE producer: on_connect acquired Ruby VM");
                    let proc_value = ruby.get_inner(on_connect_proc);
                    proc_value
                        .funcall::<_, _, Value>("call", ())
                        .map_err(|e| format!("on_connect '{}' call failed: {}", self.name, e))?;
                    debug!("Ruby SSE producer: on_connect completed");
                    Ok(())
                })
        {
            error!("on_connect error: {}", e);
        }
    }

    async fn on_disconnect(&self) {
        debug!("Ruby SSE producer '{}': on_disconnect", self.name);

        if let Some(on_disconnect_proc) = self.on_disconnect_proc
            && let Err(e) = magnus::Ruby::get()
                .map_err(|e| format!("Failed to get Ruby: {}", e))
                .and_then(|ruby| {
                    let proc_value = ruby.get_inner(on_disconnect_proc);
                    proc_value
                        .funcall::<_, _, Value>("call", ())
                        .map_err(|e| format!("on_disconnect '{}' call failed: {}", self.name, e))?;
                    debug!("Ruby SSE producer: on_disconnect completed");
                    Ok(())
                })
        {
            error!("on_disconnect error: {}", e);
        }
    }
}

// SAFETY: Ruby's GVL ensures thread safety for Ruby objects
unsafe impl Send for RubySseEventProducer {}
unsafe impl Sync for RubySseEventProducer {}

/// Create SseState from Ruby producer object
///
/// This function is designed to be called from Ruby to register SSE producers.
#[allow(dead_code)]
pub fn create_sse_state(
    ruby: &magnus::Ruby,
    producer_obj: Value,
) -> Result<spikard_http::SseState<RubySseEventProducer>, magnus::Error> {
    // Extract the next_event method
    let next_event_proc: Value = producer_obj
        .funcall("method", (ruby.to_symbol("next_event"),))
        .map_err(|e| {
            magnus::Error::new(
                ruby.exception_arg_error(),
                format!("next_event method not found: {}", e),
            )
        })?;

    // Extract optional on_connect method
    let on_connect_proc = producer_obj
        .funcall::<_, _, Value>("method", (ruby.to_symbol("on_connect"),))
        .ok();

    // Extract optional on_disconnect method
    let on_disconnect_proc = producer_obj
        .funcall::<_, _, Value>("method", (ruby.to_symbol("on_disconnect"),))
        .ok();

    // Extract event schema if available
    let event_schema = producer_obj
        .funcall::<_, _, Value>("instance_variable_get", (ruby.to_symbol("@_event_schema"),))
        .ok()
        .and_then(|v| {
            if v.is_nil() {
                None
            } else {
                RubySseEventProducer::ruby_to_json(ruby, v).ok()
            }
        });

    // Create Ruby SSE producer
    let ruby_producer = RubySseEventProducer::new(
        "SseEventProducer".to_string(),
        next_event_proc,
        on_connect_proc,
        on_disconnect_proc,
    );

    // Create and return SSE state with schema
    if event_schema.is_some() {
        spikard_http::SseState::with_schema(ruby_producer, event_schema)
            .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e))
    } else {
        Ok(spikard_http::SseState::new(ruby_producer))
    }
}

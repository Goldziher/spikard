//! Ruby WebSocket handler bindings
//!
//! This module provides the bridge between Ruby blocks/procs and Rust's WebSocket system.
//! Uses magnus to safely call Ruby code from Rust async tasks.

use magnus::{RHash, Value, prelude::*, value::Opaque};
use serde_json::Value as JsonValue;
use spikard_http::WebSocketHandler;
use tracing::{debug, error};

/// Ruby implementation of WebSocketHandler
pub struct RubyWebSocketHandler {
    /// Handler name for debugging
    name: String,
    /// Ruby proc/callable for handle_message (Opaque for Send safety)
    handle_message_proc: Opaque<Value>,
    /// Ruby proc/callable for on_connect (Opaque for Send safety)
    on_connect_proc: Option<Opaque<Value>>,
    /// Ruby proc/callable for on_disconnect (Opaque for Send safety)
    on_disconnect_proc: Option<Opaque<Value>>,
}

impl RubyWebSocketHandler {
    /// Create a new Ruby WebSocket handler
    #[allow(dead_code)]
    pub fn new(
        name: String,
        handle_message_proc: Value,
        on_connect_proc: Option<Value>,
        on_disconnect_proc: Option<Value>,
    ) -> Self {
        Self {
            name,
            handle_message_proc: handle_message_proc.into(),
            on_connect_proc: on_connect_proc.map(|v| v.into()),
            on_disconnect_proc: on_disconnect_proc.map(|v| v.into()),
        }
    }

    /// Convert Ruby value to JSON
    fn ruby_to_json(ruby: &magnus::Ruby, value: Value) -> Result<JsonValue, String> {
        if value.is_nil() {
            return Ok(JsonValue::Null);
        }

        let json_module: Value = ruby
            .class_object()
            .const_get("JSON")
            .map_err(|e| format!("JSON module not available: {}", e))?;

        let json_str: String = json_module
            .funcall("generate", (value,))
            .map_err(|e| format!("Failed to generate JSON: {}", e))?;

        serde_json::from_str(&json_str).map_err(|e| format!("Failed to parse JSON: {}", e))
    }

    /// Convert JSON value to Ruby value
    fn json_to_ruby(ruby: &magnus::Ruby, value: &JsonValue) -> Result<Value, String> {
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
            JsonValue::String(s) => Ok(ruby.str_new(s).as_value()),
            JsonValue::Array(arr) => {
                let ruby_array = ruby.ary_new();
                for item in arr {
                    ruby_array
                        .push(Self::json_to_ruby(ruby, item)?)
                        .map_err(|e| format!("Failed to push to array: {}", e))?;
                }
                Ok(ruby_array.as_value())
            }
            JsonValue::Object(obj) => {
                let ruby_hash = RHash::new();
                for (key, val) in obj {
                    ruby_hash
                        .aset(ruby.str_new(key), Self::json_to_ruby(ruby, val)?)
                        .map_err(|e| format!("Failed to set hash value: {}", e))?;
                }
                Ok(ruby_hash.as_value())
            }
        }
    }
}

impl WebSocketHandler for RubyWebSocketHandler {
    async fn handle_message(&self, message: JsonValue) -> Option<JsonValue> {
        debug!("Ruby WebSocket handler '{}': handle_message", self.name);

        match magnus::Ruby::get()
            .map_err(|e| format!("Failed to get Ruby: {}", e))
            .and_then(|ruby| {
                let message_ruby = Self::json_to_ruby(&ruby, &message)?;

                let proc_value = ruby.get_inner(self.handle_message_proc);
                let result: Value = proc_value
                    .funcall("call", (message_ruby,))
                    .map_err(|e| format!("Handler '{}' call failed: {}", self.name, e))?;

                if result.is_nil() {
                    Ok(None)
                } else {
                    Self::ruby_to_json(&ruby, result).map(Some)
                }
            }) {
            Ok(value) => value,
            Err(e) => {
                error!("Ruby error in handle_message: {}", e);
                None
            }
        }
    }

    async fn on_connect(&self) {
        debug!("Ruby WebSocket handler '{}': on_connect", self.name);

        if let Some(on_connect_proc) = self.on_connect_proc {
            if let Err(e) = magnus::Ruby::get()
                .map_err(|e| format!("Failed to get Ruby: {}", e))
                .and_then(|ruby| {
                    let proc_value = ruby.get_inner(on_connect_proc);
                    proc_value
                        .funcall::<_, _, Value>("call", ())
                        .map_err(|e| format!("on_connect '{}' call failed: {}", self.name, e))?;
                    Ok(())
                })
            {
                error!("on_connect error: {}", e);
            }

            debug!("Ruby WebSocket handler '{}': on_connect completed", self.name);
        }
    }

    async fn on_disconnect(&self) {
        debug!("Ruby WebSocket handler '{}': on_disconnect", self.name);

        if let Some(on_disconnect_proc) = self.on_disconnect_proc {
            if let Err(e) = magnus::Ruby::get()
                .map_err(|e| format!("Failed to get Ruby: {}", e))
                .and_then(|ruby| {
                    let proc_value = ruby.get_inner(on_disconnect_proc);
                    proc_value
                        .funcall::<_, _, Value>("call", ())
                        .map_err(|e| format!("on_disconnect '{}' call failed: {}", self.name, e))?;
                    Ok(())
                })
            {
                error!("on_disconnect error: {}", e);
            }

            debug!("Ruby WebSocket handler '{}': on_disconnect completed", self.name);
        }
    }
}

unsafe impl Send for RubyWebSocketHandler {}
unsafe impl Sync for RubyWebSocketHandler {}

/// Create WebSocketState from Ruby handler object
///
/// This function is designed to be called from Ruby to register WebSocket handlers.
#[allow(dead_code)]
pub fn create_websocket_state(
    ruby: &magnus::Ruby,
    handler_obj: Value,
) -> Result<spikard_http::WebSocketState<RubyWebSocketHandler>, magnus::Error> {
    let handle_message_proc: Value = handler_obj
        .funcall("method", (ruby.to_symbol("handle_message"),))
        .map_err(|e| {
            magnus::Error::new(
                ruby.exception_arg_error(),
                format!("handle_message method not found: {}", e),
            )
        })?;

    let on_connect_proc = handler_obj
        .funcall::<_, _, Value>("method", (ruby.to_symbol("on_connect"),))
        .ok();

    let on_disconnect_proc = handler_obj
        .funcall::<_, _, Value>("method", (ruby.to_symbol("on_disconnect"),))
        .ok();

    let message_schema = handler_obj
        .funcall::<_, _, Value>("instance_variable_get", (ruby.to_symbol("@_message_schema"),))
        .ok()
        .and_then(|v| {
            if v.is_nil() {
                None
            } else {
                RubyWebSocketHandler::ruby_to_json(ruby, v).ok()
            }
        });

    let response_schema = handler_obj
        .funcall::<_, _, Value>("instance_variable_get", (ruby.to_symbol("@_response_schema"),))
        .ok()
        .and_then(|v| {
            if v.is_nil() {
                None
            } else {
                RubyWebSocketHandler::ruby_to_json(ruby, v).ok()
            }
        });

    let ruby_handler = RubyWebSocketHandler::new(
        "WebSocketHandler".to_string(),
        handle_message_proc,
        on_connect_proc,
        on_disconnect_proc,
    );

    if message_schema.is_some() || response_schema.is_some() {
        spikard_http::WebSocketState::with_schemas(ruby_handler, message_schema, response_schema)
            .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e))
    } else {
        Ok(spikard_http::WebSocketState::new(ruby_handler))
    }
}

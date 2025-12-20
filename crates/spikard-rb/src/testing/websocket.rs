//! WebSocket test client bindings for Ruby

use magnus::prelude::*;
use magnus::{Error, Ruby, Value, method};
use serde_json::Value as JsonValue;
use spikard_http::testing::{WebSocketConnection as RustWebSocketConnection, WebSocketMessage as RustWebSocketMessage};
use std::cell::RefCell;

/// Ruby wrapper for WebSocket test client
#[derive(Default)]
#[magnus::wrap(class = "Spikard::Native::WebSocketTestConnection", free_immediately)]
pub struct WebSocketTestConnection {
    inner: RefCell<Option<RustWebSocketConnection>>,
}

impl WebSocketTestConnection {
    /// Create a new WebSocket test connection (public for lib.rs)
    pub(crate) fn new(inner: RustWebSocketConnection) -> Self {
        Self {
            inner: RefCell::new(Some(inner)),
        }
    }

    /// Send a text message
    fn send_text(&self, text: String) -> Result<(), Error> {
        let mut inner = self.inner.borrow_mut();
        let ws = inner
            .as_mut()
            .ok_or_else(|| Error::new(magnus::exception::runtime_error(), "WebSocket closed"))?;

        let runtime = crate::server::global_runtime_or_err()?;
        runtime.block_on(async {
            ws.send_text(text).await;
        });

        Ok(())
    }

    /// Send a JSON message
    fn send_json(ruby: &Ruby, this: &Self, obj: Value) -> Result<(), Error> {
        let json_value = ruby_to_json(ruby, obj)?;
        let mut inner = this.inner.borrow_mut();
        let ws = inner
            .as_mut()
            .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "WebSocket closed"))?;

        let runtime = crate::server::global_runtime(ruby)?;
        runtime.block_on(async {
            ws.send_json(&json_value).await;
        });

        Ok(())
    }

    /// Receive a text message
    fn receive_text(&self) -> Result<String, Error> {
        let mut inner = self.inner.borrow_mut();
        let ws = inner
            .as_mut()
            .ok_or_else(|| Error::new(magnus::exception::runtime_error(), "WebSocket closed"))?;

        let runtime = crate::server::global_runtime_or_err()?;
        let text = runtime.block_on(async { ws.receive_text().await });

        Ok(text)
    }

    /// Receive and parse a JSON message
    fn receive_json(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let mut inner = this.inner.borrow_mut();
        let ws = inner
            .as_mut()
            .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "WebSocket closed"))?;

        let runtime = crate::server::global_runtime(ruby)?;
        let json_value: JsonValue = runtime.block_on(async { ws.receive_json().await });

        json_to_ruby(ruby, &json_value)
    }

    /// Receive raw bytes
    fn receive_bytes(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let mut inner = this.inner.borrow_mut();
        let ws = inner
            .as_mut()
            .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "WebSocket closed"))?;

        let runtime = crate::server::global_runtime(ruby)?;
        let bytes = runtime.block_on(async { ws.receive_bytes().await });

        Ok(ruby.str_from_slice(&bytes).as_value())
    }

    /// Receive a message and return WebSocketMessage
    fn receive_message(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let mut inner = this.inner.borrow_mut();
        let ws = inner
            .as_mut()
            .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "WebSocket closed"))?;

        let runtime = crate::server::global_runtime(ruby)?;
        let msg = runtime.block_on(async { ws.receive_message().await });

        let ws_msg = WebSocketMessage::new(msg);
        Ok(ruby.obj_wrap(ws_msg).as_value())
    }

    /// Close the WebSocket connection
    fn close(&self) -> Result<(), Error> {
        self.inner.borrow_mut().take();
        Ok(())
    }
}

/// Ruby wrapper for WebSocket messages
#[magnus::wrap(class = "Spikard::Native::WebSocketMessage", free_immediately)]
pub struct WebSocketMessage {
    inner: RustWebSocketMessage,
}

impl WebSocketMessage {
    pub fn new(inner: RustWebSocketMessage) -> Self {
        Self { inner }
    }

    /// Get message as text if it's a text message
    fn as_text(&self) -> Result<Option<String>, Error> {
        Ok(self.inner.as_text().map(|s| s.to_string()))
    }

    /// Get message as JSON if it's a text message containing JSON
    fn as_json(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        match this.inner.as_json() {
            Ok(value) => json_to_ruby(ruby, &value),
            Err(_) => Ok(ruby.qnil().as_value()),
        }
    }

    /// Get message as binary if it's a binary message
    fn as_binary(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        match this.inner.as_binary() {
            Some(bytes) => Ok(ruby.str_from_slice(bytes).as_value()),
            None => Ok(ruby.qnil().as_value()),
        }
    }

    /// Check if this is a close message
    fn is_close(&self) -> bool {
        self.inner.is_close()
    }
}

/// Helper to convert Ruby object to JSON
fn ruby_to_json(ruby: &Ruby, value: Value) -> Result<JsonValue, Error> {
    let json_module = ruby.class_object().const_get::<_, magnus::RModule>("JSON")?;
    let json_str: String = json_module.funcall("generate", (value,))?;
    serde_json::from_str(&json_str).map_err(|e| {
        Error::new(
            magnus::exception::runtime_error(),
            format!("Failed to parse JSON: {}", e),
        )
    })
}

/// Helper to convert JSON to Ruby object
fn json_to_ruby(ruby: &Ruby, value: &JsonValue) -> Result<Value, Error> {
    match value {
        JsonValue::Null => Ok(ruby.qnil().as_value()),
        JsonValue::Bool(b) => Ok(if *b {
            ruby.qtrue().as_value()
        } else {
            ruby.qfalse().as_value()
        }),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(ruby.integer_from_i64(i).as_value())
            } else if let Some(u) = n.as_u64() {
                Ok(ruby.integer_from_i64(u as i64).as_value())
            } else if let Some(f) = n.as_f64() {
                Ok(ruby.float_from_f64(f).as_value())
            } else {
                Ok(ruby.qnil().as_value())
            }
        }
        JsonValue::String(s) => Ok(ruby.str_new(s).as_value()),
        JsonValue::Array(arr) => {
            let ruby_arr = ruby.ary_new();
            for item in arr {
                let ruby_val = json_to_ruby(ruby, item)?;
                ruby_arr.push(ruby_val)?;
            }
            Ok(ruby_arr.as_value())
        }
        JsonValue::Object(obj) => {
            let ruby_hash = ruby.hash_new();
            for (key, val) in obj {
                let ruby_val = json_to_ruby(ruby, val)?;
                ruby_hash.aset(ruby.str_new(key), ruby_val)?;
            }
            Ok(ruby_hash.as_value())
        }
    }
}

/// Initialize WebSocket test client bindings
pub fn init(ruby: &Ruby, module: &magnus::RModule) -> Result<(), Error> {
    let native_module = module.define_module("Native")?;

    let ws_conn_class = native_module.define_class("WebSocketTestConnection", ruby.class_object())?;
    ws_conn_class.define_method("send_text", method!(WebSocketTestConnection::send_text, 1))?;
    ws_conn_class.define_method("send_json", method!(WebSocketTestConnection::send_json, 1))?;
    ws_conn_class.define_method("receive_text", method!(WebSocketTestConnection::receive_text, 0))?;
    ws_conn_class.define_method("receive_json", method!(WebSocketTestConnection::receive_json, 0))?;
    ws_conn_class.define_method("receive_bytes", method!(WebSocketTestConnection::receive_bytes, 0))?;
    ws_conn_class.define_method("receive_message", method!(WebSocketTestConnection::receive_message, 0))?;
    ws_conn_class.define_method("close", method!(WebSocketTestConnection::close, 0))?;

    let ws_msg_class = native_module.define_class("WebSocketMessage", ruby.class_object())?;
    ws_msg_class.define_method("as_text", method!(WebSocketMessage::as_text, 0))?;
    ws_msg_class.define_method("as_json", method!(WebSocketMessage::as_json, 0))?;
    ws_msg_class.define_method("as_binary", method!(WebSocketMessage::as_binary, 0))?;
    ws_msg_class.define_method("is_close", method!(WebSocketMessage::is_close, 0))?;

    Ok(())
}

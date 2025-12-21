//! Ruby WebSocket handler bindings
//!
//! This module provides the bridge between Ruby blocks/procs and Rust's WebSocket system.
//! Uses magnus to safely call Ruby code from Rust async tasks.

use magnus::{RHash, Value, prelude::*, value::Opaque};
use serde_json::Value as JsonValue;
use spikard_http::WebSocketHandler;
use std::sync::mpsc;
use tokio::sync::oneshot;
use tracing::{debug, error};

/// Ruby implementation of WebSocketHandler
pub struct RubyWebSocketHandler {
    /// Handler name for debugging
    name: String,
    /// Ruby proc/callable for handle_message (Opaque for Send safety)
    #[allow(dead_code)]
    handle_message_proc: Opaque<Value>,
    /// Ruby proc/callable for on_connect (Opaque for Send safety)
    on_connect_proc: Option<Opaque<Value>>,
    /// Ruby proc/callable for on_disconnect (Opaque for Send safety)
    on_disconnect_proc: Option<Opaque<Value>>,
    /// Work queue for executing Ruby callbacks on a Ruby thread
    work_tx: mpsc::Sender<WebSocketWorkItem>,
}

enum WebSocketWorkItem {
    HandleMessage {
        message: JsonValue,
        reply: oneshot::Sender<Result<Option<JsonValue>, String>>,
    },
    OnConnect {
        reply: oneshot::Sender<Result<(), String>>,
    },
    OnDisconnect {
        reply: oneshot::Sender<Result<(), String>>,
    },
    Shutdown,
}

impl RubyWebSocketHandler {
    /// Create a new Ruby WebSocket handler
    #[allow(dead_code)]
    pub fn new(
        ruby: &magnus::Ruby,
        name: String,
        handle_message_proc: Value,
        on_connect_proc: Option<Value>,
        on_disconnect_proc: Option<Value>,
    ) -> Self {
        let handle_message_proc = Opaque::from(handle_message_proc);
        let on_connect_proc = on_connect_proc.map(Opaque::from);
        let on_disconnect_proc = on_disconnect_proc.map(Opaque::from);
        let (work_tx, work_rx) = mpsc::channel();
        let handler_name = name.clone();

        let handle_message_proc_for_thread = handle_message_proc;
        let on_connect_proc_for_thread = on_connect_proc;
        let on_disconnect_proc_for_thread = on_disconnect_proc;

        ruby.thread_create_from_fn(move |ruby| {
            websocket_worker_loop(
                ruby,
                &handler_name,
                handle_message_proc_for_thread,
                on_connect_proc_for_thread,
                on_disconnect_proc_for_thread,
                work_rx,
            );
            ruby.qnil()
        });

        Self {
            name,
            handle_message_proc,
            on_connect_proc,
            on_disconnect_proc,
            work_tx,
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

        let (reply_tx, reply_rx) = oneshot::channel();
        if self
            .work_tx
            .send(WebSocketWorkItem::HandleMessage { message, reply: reply_tx })
            .is_err()
        {
            error!("Ruby WebSocket handler '{}' worker thread closed", self.name);
            return None;
        }

        let result = match reply_rx.await {
            Ok(result) => result,
            Err(_) => {
                error!("Ruby WebSocket handler '{}' response channel closed", self.name);
                return None;
            }
        };

        match result {
            Ok(value) => value,
            Err(e) => {
                error!("Ruby error in handle_message: {}", e);
                None
            }
        }
    }

    async fn on_connect(&self) {
        debug!("Ruby WebSocket handler '{}': on_connect", self.name);

        if self.on_connect_proc.is_some() {
            let (reply_tx, reply_rx) = oneshot::channel();
            if self
                .work_tx
                .send(WebSocketWorkItem::OnConnect { reply: reply_tx })
                .is_err()
            {
                error!("Ruby WebSocket handler '{}' worker thread closed", self.name);
                return;
            }

            let result = match reply_rx.await {
                Ok(result) => result,
                Err(_) => {
                    error!("Ruby WebSocket handler '{}' on_connect channel closed", self.name);
                    return;
                }
            };

            if let Err(e) = result {
                error!("on_connect error: {}", e);
            }

            debug!("Ruby WebSocket handler '{}': on_connect completed", self.name);
        }
    }

    async fn on_disconnect(&self) {
        debug!("Ruby WebSocket handler '{}': on_disconnect", self.name);

        if self.on_disconnect_proc.is_some() {
            let (reply_tx, reply_rx) = oneshot::channel();
            if self
                .work_tx
                .send(WebSocketWorkItem::OnDisconnect { reply: reply_tx })
                .is_err()
            {
                error!("Ruby WebSocket handler '{}' worker thread closed", self.name);
                return;
            }

            let result = match reply_rx.await {
                Ok(result) => result,
                Err(_) => {
                    error!("Ruby WebSocket handler '{}' on_disconnect channel closed", self.name);
                    return;
                }
            };

            if let Err(e) = result {
                error!("on_disconnect error: {}", e);
            }

            debug!("Ruby WebSocket handler '{}': on_disconnect completed", self.name);
        }
    }
}

impl Drop for RubyWebSocketHandler {
    fn drop(&mut self) {
        let _ = self.work_tx.send(WebSocketWorkItem::Shutdown);
    }
}

fn websocket_worker_loop(
    ruby: &magnus::Ruby,
    handler_name: &str,
    handle_message_proc: Opaque<Value>,
    on_connect_proc: Option<Opaque<Value>>,
    on_disconnect_proc: Option<Opaque<Value>>,
    work_rx: mpsc::Receiver<WebSocketWorkItem>,
) {
    let work_rx_ref = &work_rx;
    loop {
        let work = crate::call_without_gvl!(
            recv_work_item,
            args: (work_rx_ref, &mpsc::Receiver<WebSocketWorkItem>),
            return_type: Option<WebSocketWorkItem>
        );
        let Some(work) = work else {
            break;
        };

        match work {
            WebSocketWorkItem::HandleMessage { message, reply } => {
                let result = (|| {
                    let message_ruby = RubyWebSocketHandler::json_to_ruby(ruby, &message)?;
                    let proc_value = ruby.get_inner(handle_message_proc);
                    let result: Value = proc_value
                        .funcall("call", (message_ruby,))
                        .map_err(|e| format!("Handler '{}' call failed: {}", handler_name, e))?;
                    if result.is_nil() {
                        Ok(None)
                    } else {
                        RubyWebSocketHandler::ruby_to_json(ruby, result).map(Some)
                    }
                })();
                let _ = reply.send(result);
            }
            WebSocketWorkItem::OnConnect { reply } => {
                let result = on_connect_proc
                    .map(|proc| {
                        let proc_value = ruby.get_inner(proc);
                        proc_value
                            .funcall::<_, _, Value>("call", ())
                            .map_err(|e| format!("on_connect '{}' call failed: {}", handler_name, e))?;
                        Ok(())
                    })
                    .unwrap_or(Ok(()));
                let _ = reply.send(result);
            }
            WebSocketWorkItem::OnDisconnect { reply } => {
                let result = on_disconnect_proc
                    .map(|proc| {
                        let proc_value = ruby.get_inner(proc);
                        proc_value
                            .funcall::<_, _, Value>("call", ())
                            .map_err(|e| format!("on_disconnect '{}' call failed: {}", handler_name, e))?;
                        Ok(())
                    })
                    .unwrap_or(Ok(()));
                let _ = reply.send(result);
            }
            WebSocketWorkItem::Shutdown => {
                break;
            }
        }
    }
}

fn recv_work_item(receiver: &mpsc::Receiver<WebSocketWorkItem>) -> Option<WebSocketWorkItem> {
    receiver.recv().ok()
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
        ruby,
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

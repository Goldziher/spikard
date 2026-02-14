//! Elixir WebSocket handler implementation.
//!
//! This module provides the `ElixirWebSocketHandler` struct that wraps Elixir callback functions
//! for handling WebSocket connections. Each connection is managed by an Elixir GenServer
//! that receives messages from the Rust side.
//!
//! # Message Flow
//!
//! ```text
//! WebSocket Connection (Rust)
//!   ↓
//! ElixirWebSocketHandler::on_message
//!   ↓
//! Send {:websocket_message, data} to Elixir GenServer
//!   ↓
//! Elixir handler processes and sends response
//!   ↓
//! websocket_send NIF sends message back to client
//! ```

#![deny(clippy::unwrap_used)]

use once_cell::sync::Lazy;
use rustler::{Encoder, LocalPid, OwnedEnv};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::Mutex;
use tracing::debug;

/// Global map of pending WebSocket response channels.
/// Used to deliver WebSocket responses from Elixir back to waiting Rust handlers.
static PENDING_WEBSOCKET_MESSAGES: Lazy<Mutex<HashMap<u64, tokio::sync::oneshot::Sender<JsonValue>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Counter for generating unique WebSocket message IDs.
static WEBSOCKET_MESSAGE_ID_COUNTER: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));

/// Generate a unique WebSocket message ID.
fn next_websocket_message_id() -> u64 {
    let mut counter = WEBSOCKET_MESSAGE_ID_COUNTER.lock().unwrap_or_else(|e| e.into_inner());
    *counter = counter.wrapping_add(1);
    *counter
}

/// Register a pending WebSocket message and return its ID.
fn register_pending_websocket_message(sender: tokio::sync::oneshot::Sender<JsonValue>) -> u64 {
    let id = next_websocket_message_id();
    let mut pending = PENDING_WEBSOCKET_MESSAGES.lock().unwrap_or_else(|e| e.into_inner());
    pending.insert(id, sender);
    id
}

/// Deliver a WebSocket message response.
pub fn deliver_websocket_message(message_id: u64, response: JsonValue) -> bool {
    let sender = {
        let mut pending = PENDING_WEBSOCKET_MESSAGES.lock().unwrap_or_else(|e| e.into_inner());
        pending.remove(&message_id)
    };

    match sender {
        Some(tx) => tx.send(response).is_ok(),
        None => {
            debug!("No pending WebSocket message found for ID {}", message_id);
            false
        }
    }
}

/// Inner state of an Elixir WebSocket handler.
pub struct ElixirWebSocketHandlerInner {
    /// PID of the WebSocket handler GenServer
    pub ws_handler_pid: LocalPid,
    /// Name of the handler module
    pub handler_name: String,
    /// WebSocket path
    pub path: String,
}

/// Wrapper around an Elixir WebSocket handler.
///
/// When a WebSocket connection is established or receives a message,
/// this sends a message to the handler GenServer which invokes the
/// corresponding Elixir function.
#[derive(Clone)]
pub struct ElixirWebSocketHandler {
    pub inner: std::sync::Arc<ElixirWebSocketHandlerInner>,
}

impl ElixirWebSocketHandler {
    /// Create a new ElixirWebSocketHandler.
    pub fn new(handler_name: String, path: String, ws_handler_pid: LocalPid) -> Self {
        Self {
            inner: std::sync::Arc::new(ElixirWebSocketHandlerInner {
                ws_handler_pid,
                handler_name,
                path,
            }),
        }
    }

    /// Send a WebSocket connect message to the handler.
    pub fn send_connect_message(&self, ws_ref: u64, opts: JsonValue) -> Result<(), String> {
        debug!("Sending WebSocket connect message for ws_ref {}", ws_ref);

        let owned_env = OwnedEnv::new();
        owned_env.run(|env| {
            let connect_atom = rustler::Atom::from_str(env, "websocket_connect")
                .map_err(|_| "Failed to create connect atom".to_string())?;

            let ws_ref_term = ws_ref.encode(env);
            let opts_term = crate::conversion::json_to_elixir(env, &opts)
                .map_err(|e| format!("Failed to convert opts to Elixir: {:?}", e))?;

            let message = (connect_atom, ws_ref_term, opts_term).encode(env);

            if env.send(&self.inner.ws_handler_pid, message).is_err() {
                return Err("Failed to send connect message to handler".to_string());
            }

            Ok(())
        })
    }

    /// Send a WebSocket message to the handler.
    pub fn send_message(&self, ws_ref: u64, message: JsonValue) -> Result<(), String> {
        debug!("Sending WebSocket message for ws_ref {}", ws_ref);

        let owned_env = OwnedEnv::new();
        owned_env.run(|env| {
            let message_atom = rustler::Atom::from_str(env, "websocket_message")
                .map_err(|_| "Failed to create message atom".to_string())?;

            let ws_ref_term = ws_ref.encode(env);
            let message_term = crate::conversion::json_to_elixir(env, &message)
                .map_err(|e| format!("Failed to convert message to Elixir: {:?}", e))?;

            let msg = (message_atom, ws_ref_term, message_term).encode(env);

            if env.send(&self.inner.ws_handler_pid, msg).is_err() {
                return Err("Failed to send message to handler".to_string());
            }

            Ok(())
        })
    }

    /// Send a WebSocket disconnect message to the handler.
    pub fn send_disconnect_message(&self, ws_ref: u64) -> Result<(), String> {
        debug!("Sending WebSocket disconnect message for ws_ref {}", ws_ref);

        let owned_env = OwnedEnv::new();
        owned_env.run(|env| {
            let disconnect_atom = rustler::Atom::from_str(env, "websocket_closed")
                .map_err(|_| "Failed to create disconnect atom".to_string())?;

            let ws_ref_term = ws_ref.encode(env);
            let message = (disconnect_atom, ws_ref_term).encode(env);

            if env.send(&self.inner.ws_handler_pid, message).is_err() {
                return Err("Failed to send disconnect message to handler".to_string());
            }

            Ok(())
        })
    }
}

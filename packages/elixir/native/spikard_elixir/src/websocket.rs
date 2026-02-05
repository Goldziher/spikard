//! Elixir WebSocket support.
//!
//! This module provides WebSocket support for Elixir handlers.
//! WebSocket connections are upgraded from HTTP and managed through NIF callbacks.

#![allow(dead_code)]
#![deny(clippy::unwrap_used)]

use once_cell::sync::Lazy;
use rustler::{Encoder, Env, NifResult, Term};
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::sync::mpsc;
use tracing::{debug, warn};

use crate::atoms;

/// Global map of active WebSocket connections keyed by unique connection ID.
/// Used to send messages to WebSocket clients from Elixir.
static WEBSOCKET_CONNECTIONS: Lazy<Mutex<HashMap<u64, WebSocketConnection>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Counter for generating unique WebSocket connection IDs.
static WEBSOCKET_ID_COUNTER: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));

/// Represents an active WebSocket connection.
pub struct WebSocketConnection {
    /// Channel sender for sending messages to the WebSocket
    pub sender: mpsc::Sender<WebSocketMessage>,
    /// Whether the connection is still active
    pub active: bool,
}

/// Message types that can be sent over WebSocket.
#[derive(Debug, Clone)]
pub enum WebSocketMessage {
    /// Text message
    Text(String),
    /// Binary message
    Binary(Vec<u8>),
    /// Close the connection
    Close,
}

/// WebSocket reference passed to Elixir.
/// Wraps the connection ID for safe FFI passing.
pub struct WebSocketRef {
    pub connection_id: u64,
}

/// Resource type for WebSocket references.
impl WebSocketRef {
    pub fn new(connection_id: u64) -> Self {
        Self { connection_id }
    }
}

/// Generate a unique WebSocket connection ID.
fn next_websocket_id() -> u64 {
    let mut counter = WEBSOCKET_ID_COUNTER.lock().unwrap_or_else(|e| e.into_inner());
    *counter = counter.wrapping_add(1);
    *counter
}

/// Register a new WebSocket connection and return its ID.
pub fn register_websocket_connection(sender: mpsc::Sender<WebSocketMessage>) -> u64 {
    let id = next_websocket_id();
    let mut connections = WEBSOCKET_CONNECTIONS.lock().unwrap_or_else(|e| e.into_inner());
    connections.insert(
        id,
        WebSocketConnection {
            sender,
            active: true,
        },
    );
    debug!("Registered WebSocket connection {}", id);
    id
}

/// Unregister a WebSocket connection.
pub fn unregister_websocket_connection(connection_id: u64) {
    let mut connections = WEBSOCKET_CONNECTIONS.lock().unwrap_or_else(|e| e.into_inner());
    if connections.remove(&connection_id).is_some() {
        debug!("Unregistered WebSocket connection {}", connection_id);
    }
}

/// Send a message to a WebSocket connection.
pub fn send_websocket_message(connection_id: u64, message: WebSocketMessage) -> Result<(), String> {
    let connections = WEBSOCKET_CONNECTIONS.lock().unwrap_or_else(|e| e.into_inner());

    match connections.get(&connection_id) {
        Some(conn) if conn.active => {
            let sender = conn.sender.clone();
            drop(connections); // Release lock before async operation

            // Use try_send for non-blocking send
            match sender.try_send(message) {
                Ok(()) => {
                    debug!("Sent message to WebSocket connection {}", connection_id);
                    Ok(())
                }
                Err(mpsc::error::TrySendError::Full(_)) => {
                    warn!(
                        "WebSocket connection {} message queue full",
                        connection_id
                    );
                    Err("Message queue full".to_string())
                }
                Err(mpsc::error::TrySendError::Closed(_)) => {
                    warn!("WebSocket connection {} is closed", connection_id);
                    Err("Connection closed".to_string())
                }
            }
        }
        Some(_) => {
            warn!("WebSocket connection {} is not active", connection_id);
            Err("Connection not active".to_string())
        }
        None => {
            warn!("WebSocket connection {} not found", connection_id);
            Err("Connection not found".to_string())
        }
    }
}

/// Close a WebSocket connection.
pub fn close_websocket_connection(connection_id: u64) -> Result<(), String> {
    let mut connections = WEBSOCKET_CONNECTIONS.lock().unwrap_or_else(|e| e.into_inner());

    match connections.get_mut(&connection_id) {
        Some(conn) => {
            conn.active = false;
            let sender = conn.sender.clone();
            drop(connections); // Release lock before async operation

            // Send close message
            match sender.try_send(WebSocketMessage::Close) {
                Ok(()) => {
                    debug!("Sent close to WebSocket connection {}", connection_id);
                    Ok(())
                }
                Err(_) => {
                    // Connection might already be closed, that's fine
                    debug!(
                        "WebSocket connection {} already closed or queue full",
                        connection_id
                    );
                    Ok(())
                }
            }
        }
        None => {
            warn!("WebSocket connection {} not found for close", connection_id);
            Err("Connection not found".to_string())
        }
    }
}

/// NIF to send a message to a WebSocket client.
///
/// # Arguments
///
/// * `ws_ref` - WebSocket reference (connection ID as integer)
/// * `message` - Message to send (string or binary)
#[rustler::nif]
pub fn websocket_send<'a>(env: Env<'a>, ws_ref: u64, message: Term<'a>) -> NifResult<Term<'a>> {
    debug!("websocket_send called for connection {}", ws_ref);

    // Determine message type and convert
    let ws_message = if let Ok(text) = message.decode::<String>() {
        WebSocketMessage::Text(text)
    } else if let Ok(binary) = message.decode::<Vec<u8>>() {
        WebSocketMessage::Binary(binary)
    } else {
        // Try to convert term to string representation
        let text = format!("{:?}", message);
        WebSocketMessage::Text(text)
    };

    match send_websocket_message(ws_ref, ws_message) {
        Ok(()) => Ok(atoms::ok().encode(env)),
        Err(reason) => Ok((atoms::error(), reason).encode(env)),
    }
}

/// NIF to close a WebSocket connection.
///
/// # Arguments
///
/// * `ws_ref` - WebSocket reference (connection ID as integer)
#[rustler::nif]
pub fn websocket_close<'a>(env: Env<'a>, ws_ref: u64) -> NifResult<Term<'a>> {
    debug!("websocket_close called for connection {}", ws_ref);

    match close_websocket_connection(ws_ref) {
        Ok(()) => Ok(atoms::ok().encode(env)),
        Err(reason) => Ok((atoms::error(), reason).encode(env)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_websocket_id_increments() {
        let id1 = next_websocket_id();
        let id2 = next_websocket_id();
        assert!(id2 > id1 || id2 == 1); // Allow for wrap-around
    }

    #[test]
    fn test_register_and_unregister_connection() {
        let (tx, _rx) = mpsc::channel(10);
        let id = register_websocket_connection(tx);
        assert!(id > 0);

        // Check connection exists
        {
            let connections = WEBSOCKET_CONNECTIONS.lock().unwrap_or_else(|e| e.into_inner());
            assert!(connections.contains_key(&id));
        }

        // Unregister
        unregister_websocket_connection(id);

        // Check connection removed
        {
            let connections = WEBSOCKET_CONNECTIONS.lock().unwrap_or_else(|e| e.into_inner());
            assert!(!connections.contains_key(&id));
        }
    }

    #[test]
    fn test_send_to_nonexistent_connection() {
        let result = send_websocket_message(999999, WebSocketMessage::Text("test".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_close_nonexistent_connection() {
        let result = close_websocket_connection(999999);
        assert!(result.is_err());
    }
}

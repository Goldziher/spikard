//! WebSocket support for Spikard
//!
//! Provides WebSocket connection handling with message validation and routing.

use axum::{
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use serde_json::Value;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

/// WebSocket message handler trait
///
/// Implement this trait to handle WebSocket messages in your application.
pub trait WebSocketHandler: Send + Sync {
    /// Handle incoming WebSocket message
    ///
    /// Returns an optional response message to send back to the client.
    fn handle_message(&self, message: Value) -> impl std::future::Future<Output = Option<Value>> + Send;

    /// Called when a client connects
    fn on_connect(&self) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }

    /// Called when a client disconnects
    fn on_disconnect(&self) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }
}

/// WebSocket state shared across connections
pub struct WebSocketState<H: WebSocketHandler> {
    handler: Arc<H>,
    /// Optional JSON Schema for validating incoming messages
    message_schema: Option<Arc<jsonschema::Validator>>,
    /// Optional JSON Schema for validating outgoing responses
    response_schema: Option<Arc<jsonschema::Validator>>,
}

impl<H: WebSocketHandler> Clone for WebSocketState<H> {
    fn clone(&self) -> Self {
        Self {
            handler: Arc::clone(&self.handler),
            message_schema: self.message_schema.clone(),
            response_schema: self.response_schema.clone(),
        }
    }
}

impl<H: WebSocketHandler + 'static> WebSocketState<H> {
    /// Create new WebSocket state with a handler
    pub fn new(handler: H) -> Self {
        Self {
            handler: Arc::new(handler),
            message_schema: None,
            response_schema: None,
        }
    }

    /// Create new WebSocket state with a handler and schemas
    pub fn with_schemas(
        handler: H,
        message_schema: Option<serde_json::Value>,
        response_schema: Option<serde_json::Value>,
    ) -> Result<Self, String> {
        let message_validator = if let Some(schema) = message_schema {
            Some(Arc::new(
                jsonschema::validator_for(&schema).map_err(|e| format!("Invalid message schema: {}", e))?,
            ))
        } else {
            None
        };

        let response_validator = if let Some(schema) = response_schema {
            Some(Arc::new(
                jsonschema::validator_for(&schema).map_err(|e| format!("Invalid response schema: {}", e))?,
            ))
        } else {
            None
        };

        Ok(Self {
            handler: Arc::new(handler),
            message_schema: message_validator,
            response_schema: response_validator,
        })
    }
}

/// WebSocket upgrade handler
///
/// This is the main entry point for WebSocket connections.
/// Use this as an Axum route handler.
pub async fn websocket_handler<H: WebSocketHandler + 'static>(
    ws: WebSocketUpgrade,
    State(state): State<WebSocketState<H>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// Handle an individual WebSocket connection
async fn handle_socket<H: WebSocketHandler>(mut socket: WebSocket, state: WebSocketState<H>) {
    println!("websocket handle_socket invoked");
    info!("WebSocket client connected");

    // Notify handler of connection
    state.handler.on_connect().await;

    // Process messages
    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("received text payload: {}", text);
                debug!("Received text message: {}", text);

                // Parse JSON message
                match serde_json::from_str::<Value>(&text) {
                    Ok(json_msg) => {
                        // Validate incoming message if schema is provided
                        if let Some(validator) = &state.message_schema
                            && !validator.is_valid(&json_msg)
                        {
                            error!("Message validation failed");
                            let error_response = serde_json::json!({
                                "error": "Message validation failed"
                            });
                            if let Ok(error_text) = serde_json::to_string(&error_response) {
                                let _ = socket.send(Message::Text(error_text.into())).await;
                            }
                            continue;
                        }

                        // Handle the message
                        if let Some(response) = state.handler.handle_message(json_msg).await {
                            // Validate outgoing response if schema is provided
                            if let Some(validator) = &state.response_schema
                                && !validator.is_valid(&response)
                            {
                                error!("Response validation failed");
                                continue;
                            }

                            // Send response back
                            let response_text = serde_json::to_string(&response).unwrap_or_else(|_| "{}".to_string());

                            if let Err(e) = socket.send(Message::Text(response_text.into())).await {
                                error!("Failed to send response: {}", e);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Failed to parse JSON message: {}", e);
                        // Optionally send error response
                        let error_msg = serde_json::json!({
                            "type": "error",
                            "message": "Invalid JSON"
                        });
                        let error_text = serde_json::to_string(&error_msg).unwrap();
                        let _ = socket.send(Message::Text(error_text.into())).await;
                    }
                }
            }
            Ok(Message::Binary(data)) => {
                debug!("Received binary message: {} bytes", data.len());
                // For now, we'll echo binary messages back
                if let Err(e) = socket.send(Message::Binary(data)).await {
                    error!("Failed to send binary response: {}", e);
                    break;
                }
            }
            Ok(Message::Ping(data)) => {
                debug!("Received ping");
                if let Err(e) = socket.send(Message::Pong(data)).await {
                    error!("Failed to send pong: {}", e);
                    break;
                }
            }
            Ok(Message::Pong(_)) => {
                debug!("Received pong");
            }
            Ok(Message::Close(_)) => {
                info!("Client closed connection");
                break;
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
        }
    }

    // Notify handler of disconnection
    state.handler.on_disconnect().await;
    info!("WebSocket client disconnected");
}

#[cfg(test)]
mod tests {
    use super::*;

    struct EchoHandler;

    impl WebSocketHandler for EchoHandler {
        async fn handle_message(&self, message: Value) -> Option<Value> {
            Some(message)
        }
    }

    #[test]
    fn test_websocket_state_creation() {
        let handler = EchoHandler;
        let state = WebSocketState::new(handler);
        let cloned = state.clone();
        // Verify state can be cloned
        assert!(Arc::ptr_eq(&state.handler, &cloned.handler));
    }
}

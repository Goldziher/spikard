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
/// Implement this trait to create custom WebSocket message handlers for your application.
/// The handler processes JSON messages received from WebSocket clients and can optionally
/// send responses back.
///
/// # Implementing the Trait
///
/// You must implement the `handle_message` method. The `on_connect` and `on_disconnect`
/// methods are optional and provide lifecycle hooks.
///
/// # Example
///
/// ```ignore
/// use spikard_http::websocket::WebSocketHandler;
/// use serde_json::{json, Value};
///
/// struct EchoHandler;
///
/// #[async_trait]
/// impl WebSocketHandler for EchoHandler {
///     async fn handle_message(&self, message: Value) -> Option<Value> {
///         // Echo the message back to the client
///         Some(message)
///     }
///
///     async fn on_connect(&self) {
///         println!("Client connected");
///     }
///
///     async fn on_disconnect(&self) {
///         println!("Client disconnected");
///     }
/// }
/// ```
pub trait WebSocketHandler: Send + Sync {
    /// Handle incoming WebSocket message
    ///
    /// Called whenever a text message is received from a WebSocket client.
    /// Messages are automatically parsed as JSON.
    ///
    /// # Arguments
    /// * `message` - JSON value received from the client
    ///
    /// # Returns
    /// * `Some(value)` - JSON value to send back to the client
    /// * `None` - No response to send
    fn handle_message(&self, message: Value) -> impl std::future::Future<Output = Option<Value>> + Send;

    /// Called when a client connects to the WebSocket
    ///
    /// Optional lifecycle hook invoked when a new WebSocket connection is established.
    /// Default implementation does nothing.
    fn on_connect(&self) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }

    /// Called when a client disconnects from the WebSocket
    ///
    /// Optional lifecycle hook invoked when a WebSocket connection is closed
    /// (either by the client or due to an error). Default implementation does nothing.
    fn on_disconnect(&self) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }
}

/// WebSocket state shared across connections
///
/// Contains the message handler and optional JSON schemas for validating
/// incoming and outgoing messages. This state is shared among all connections
/// to the same WebSocket endpoint.
pub struct WebSocketState<H: WebSocketHandler> {
    /// The message handler implementation
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
    ///
    /// Creates a new state without message or response validation schemas.
    /// Messages and responses are not validated.
    ///
    /// # Arguments
    /// * `handler` - The message handler implementation
    ///
    /// # Example
    ///
    /// ```ignore
    /// let state = WebSocketState::new(MyHandler);
    /// ```
    pub fn new(handler: H) -> Self {
        Self {
            handler: Arc::new(handler),
            message_schema: None,
            response_schema: None,
        }
    }

    /// Create new WebSocket state with a handler and optional validation schemas
    ///
    /// Creates a new state with optional JSON schemas for validating incoming messages
    /// and outgoing responses. If a schema is provided and validation fails, the message
    /// or response is rejected.
    ///
    /// # Arguments
    /// * `handler` - The message handler implementation
    /// * `message_schema` - Optional JSON schema for validating client messages
    /// * `response_schema` - Optional JSON schema for validating handler responses
    ///
    /// # Returns
    /// * `Ok(state)` - Successfully created state
    /// * `Err(msg)` - Invalid schema provided
    ///
    /// # Example
    ///
    /// ```ignore
    /// use serde_json::json;
    ///
    /// let message_schema = json!({
    ///     "type": "object",
    ///     "properties": {
    ///         "type": {"type": "string"},
    ///         "data": {"type": "string"}
    ///     }
    /// });
    ///
    /// let state = WebSocketState::with_schemas(
    ///     MyHandler,
    ///     Some(message_schema),
    ///     None,
    /// )?;
    /// ```
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
/// This is the main entry point for WebSocket connections. Use this as an Axum route
/// handler by passing it to an Axum router's `.route()` method with `get()`.
///
/// # Arguments
/// * `ws` - WebSocket upgrade from Axum
/// * `State(state)` - Application state containing the handler and optional schemas
///
/// # Returns
/// An Axum response that upgrades the connection to WebSocket
///
/// # Example
///
/// ```ignore
/// use axum::{Router, routing::get, extract::State};
///
/// let state = WebSocketState::new(MyHandler);
/// let router = Router::new()
///     .route("/ws", get(websocket_handler::<MyHandler>))
///     .with_state(state);
/// ```
pub async fn websocket_handler<H: WebSocketHandler + 'static>(
    ws: WebSocketUpgrade,
    State(state): State<WebSocketState<H>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// Handle an individual WebSocket connection
async fn handle_socket<H: WebSocketHandler>(mut socket: WebSocket, state: WebSocketState<H>) {
    info!("WebSocket client connected");

    state.handler.on_connect().await;

    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(Message::Text(text)) => {
                debug!("Received text message: {}", text);

                match serde_json::from_str::<Value>(&text) {
                    Ok(json_msg) => {
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

                        if let Some(response) = state.handler.handle_message(json_msg).await {
                            if let Some(validator) = &state.response_schema
                                && !validator.is_valid(&response)
                            {
                                error!("Response validation failed");
                                continue;
                            }

                            let response_text = serde_json::to_string(&response).unwrap_or_else(|_| "{}".to_string());

                            if let Err(e) = socket.send(Message::Text(response_text.into())).await {
                                error!("Failed to send response: {}", e);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Failed to parse JSON message: {}", e);
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
        assert!(Arc::ptr_eq(&state.handler, &cloned.handler));
    }
}

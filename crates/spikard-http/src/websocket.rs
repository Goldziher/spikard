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

fn trace_ws(message: &str) {
    if std::env::var("SPIKARD_WS_TRACE").ok().as_deref() == Some("1") {
        eprintln!("[spikard-ws] {message}");
    }
}

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
    /// Factory for producing per-connection handlers
    handler_factory: Arc<dyn Fn() -> Result<Arc<H>, String> + Send + Sync>,
    /// Optional JSON Schema for validating incoming messages
    message_schema: Option<Arc<jsonschema::Validator>>,
    /// Optional JSON Schema for validating outgoing responses
    response_schema: Option<Arc<jsonschema::Validator>>,
}

impl<H: WebSocketHandler> std::fmt::Debug for WebSocketState<H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebSocketState")
            .field("message_schema", &self.message_schema.is_some())
            .field("response_schema", &self.response_schema.is_some())
            .finish()
    }
}

impl<H: WebSocketHandler> Clone for WebSocketState<H> {
    fn clone(&self) -> Self {
        Self {
            handler: Arc::clone(&self.handler),
            handler_factory: Arc::clone(&self.handler_factory),
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
        let handler = Arc::new(handler);
        Self {
            handler_factory: Arc::new({
                let handler = Arc::clone(&handler);
                move || Ok(Arc::clone(&handler))
            }),
            handler,
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

        let handler = Arc::new(handler);
        Ok(Self {
            handler_factory: Arc::new({
                let handler = Arc::clone(&handler);
                move || Ok(Arc::clone(&handler))
            }),
            handler,
            message_schema: message_validator,
            response_schema: response_validator,
        })
    }

    /// Create new WebSocket state with a handler factory and optional validation schemas.
    ///
    /// The factory is invoked once per connection, enabling per-connection handler state.
    pub fn with_factory<F>(
        factory: F,
        message_schema: Option<serde_json::Value>,
        response_schema: Option<serde_json::Value>,
    ) -> Result<Self, String>
    where
        F: Fn() -> Result<H, String> + Send + Sync + 'static,
    {
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

        let factory = Arc::new(factory);
        let handler = factory()
            .map(Arc::new)
            .map_err(|e| format!("Failed to build WebSocket handler: {}", e))?;

        Ok(Self {
            handler_factory: Arc::new({
                let factory = Arc::clone(&factory);
                move || factory().map(Arc::new)
            }),
            handler,
            message_schema: message_validator,
            response_schema: response_validator,
        })
    }

    /// Invoke the connection hook for testing.
    pub async fn on_connect(&self) {
        self.handler.on_connect().await;
    }

    /// Invoke the disconnect hook for testing.
    pub async fn on_disconnect(&self) {
        self.handler.on_disconnect().await;
    }

    /// Validate and handle an incoming message without a socket transport.
    pub async fn handle_message_validated(&self, message: Value) -> Result<Option<Value>, String> {
        if let Some(validator) = &self.message_schema
            && !validator.is_valid(&message)
        {
            return Err("Message validation failed".to_string());
        }

        let response = self.handler.handle_message(message).await;
        if let Some(ref value) = response
            && let Some(validator) = &self.response_schema
            && !validator.is_valid(value)
        {
            return Ok(None);
        }

        Ok(response)
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
    trace_ws("socket:connected");

    let handler = match (state.handler_factory)() {
        Ok(handler) => handler,
        Err(err) => {
            error!("Failed to create WebSocket handler: {}", err);
            trace_ws("socket:handler-factory:error");
            return;
        }
    };

    handler.on_connect().await;
    trace_ws("socket:on_connect:done");

    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(Message::Text(text)) => {
                debug!("Received text message: {}", text);
                trace_ws(&format!("recv:text len={}", text.len()));

                match serde_json::from_str::<Value>(&text) {
                    Ok(json_msg) => {
                        trace_ws("recv:text:json-ok");
                        if let Some(validator) = &state.message_schema
                            && !validator.is_valid(&json_msg)
                        {
                            error!("Message validation failed");
                            trace_ws("recv:text:validation-failed");
                            let error_response = serde_json::json!({
                                "error": "Message validation failed"
                            });
                            if let Ok(error_text) = serde_json::to_string(&error_response) {
                                trace_ws(&format!("send:validation-error len={}", error_text.len()));
                                let _ = socket.send(Message::Text(error_text.into())).await;
                            }
                            continue;
                        }

                        if let Some(response) = handler.handle_message(json_msg).await {
                            trace_ws("handler:response:some");
                            if let Some(validator) = &state.response_schema
                                && !validator.is_valid(&response)
                            {
                                error!("Response validation failed");
                                trace_ws("send:response:validation-failed");
                                continue;
                            }

                            let response_text = serde_json::to_string(&response).unwrap_or_else(|_| "{}".to_string());
                            let response_len = response_text.len();

                            if let Err(e) = socket.send(Message::Text(response_text.into())).await {
                                error!("Failed to send response: {}", e);
                                trace_ws("send:response:error");
                                break;
                            }
                            trace_ws(&format!("send:response len={}", response_len));
                        } else {
                            trace_ws("handler:response:none");
                        }
                    }
                    Err(e) => {
                        warn!("Failed to parse JSON message: {}", e);
                        trace_ws("recv:text:json-error");
                        let error_msg = serde_json::json!({
                            "type": "error",
                            "message": "Invalid JSON"
                        });
                        let error_text = serde_json::to_string(&error_msg).unwrap_or_else(|_| "{}".to_string());
                        trace_ws(&format!("send:json-error len={}", error_text.len()));
                        let _ = socket.send(Message::Text(error_text.into())).await;
                    }
                }
            }
            Ok(Message::Binary(data)) => {
                debug!("Received binary message: {} bytes", data.len());
                trace_ws(&format!("recv:binary len={}", data.len()));
                if let Err(e) = socket.send(Message::Binary(data)).await {
                    error!("Failed to send binary response: {}", e);
                    trace_ws("send:binary:error");
                    break;
                }
                trace_ws("send:binary:ok");
            }
            Ok(Message::Ping(data)) => {
                debug!("Received ping");
                trace_ws(&format!("recv:ping len={}", data.len()));
                if let Err(e) = socket.send(Message::Pong(data)).await {
                    error!("Failed to send pong: {}", e);
                    trace_ws("send:pong:error");
                    break;
                }
                trace_ws("send:pong:ok");
            }
            Ok(Message::Pong(_)) => {
                debug!("Received pong");
                trace_ws("recv:pong");
            }
            Ok(Message::Close(_)) => {
                info!("Client closed connection");
                trace_ws("recv:close");
                break;
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                trace_ws(&format!("recv:error {}", e));
                break;
            }
        }
    }

    handler.on_disconnect().await;
    trace_ws("socket:on_disconnect:done");
    info!("WebSocket client disconnected");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[derive(Debug)]
    struct EchoHandler;

    impl WebSocketHandler for EchoHandler {
        async fn handle_message(&self, message: Value) -> Option<Value> {
            Some(message)
        }
    }

    #[derive(Debug)]
    struct TrackingHandler {
        connect_count: Arc<AtomicUsize>,
        disconnect_count: Arc<AtomicUsize>,
        message_count: Arc<AtomicUsize>,
        messages: Arc<Mutex<Vec<Value>>>,
    }

    impl TrackingHandler {
        fn new() -> Self {
            Self {
                connect_count: Arc::new(AtomicUsize::new(0)),
                disconnect_count: Arc::new(AtomicUsize::new(0)),
                message_count: Arc::new(AtomicUsize::new(0)),
                messages: Arc::new(Mutex::new(Vec::new())),
            }
        }
    }

    impl WebSocketHandler for TrackingHandler {
        async fn handle_message(&self, message: Value) -> Option<Value> {
            self.message_count.fetch_add(1, Ordering::SeqCst);
            self.messages.lock().unwrap().push(message.clone());
            Some(message)
        }

        async fn on_connect(&self) {
            self.connect_count.fetch_add(1, Ordering::SeqCst);
        }

        async fn on_disconnect(&self) {
            self.disconnect_count.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[derive(Debug)]
    struct SelectiveHandler;

    impl WebSocketHandler for SelectiveHandler {
        async fn handle_message(&self, message: Value) -> Option<Value> {
            if message.get("respond").is_some_and(|v| v.as_bool().unwrap_or(false)) {
                Some(serde_json::json!({"response": "acknowledged"}))
            } else {
                None
            }
        }
    }

    #[derive(Debug)]
    struct TransformHandler;

    impl WebSocketHandler for TransformHandler {
        async fn handle_message(&self, message: Value) -> Option<Value> {
            message.as_object().map_or(None, |obj| {
                let mut resp = obj.clone();
                resp.insert("processed".to_string(), Value::Bool(true));
                Some(Value::Object(resp))
            })
        }
    }

    #[test]
    fn test_websocket_state_creation() {
        let handler: EchoHandler = EchoHandler;
        let state: WebSocketState<EchoHandler> = WebSocketState::new(handler);
        let cloned: WebSocketState<EchoHandler> = state.clone();
        assert!(Arc::ptr_eq(&state.handler, &cloned.handler));
    }

    #[test]
    fn test_websocket_state_with_valid_schema() {
        let handler: EchoHandler = EchoHandler;
        let schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {
                "type": {"type": "string"}
            }
        });

        let result: Result<WebSocketState<EchoHandler>, String> =
            WebSocketState::with_schemas(handler, Some(schema), None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_websocket_state_with_invalid_schema() {
        let handler: EchoHandler = EchoHandler;
        let invalid_schema: serde_json::Value = serde_json::json!({
            "type": "not_a_real_type",
            "invalid": "schema"
        });

        let result: Result<WebSocketState<EchoHandler>, String> =
            WebSocketState::with_schemas(handler, Some(invalid_schema), None);
        assert!(result.is_err());
        if let Err(error_msg) = result {
            assert!(error_msg.contains("Invalid message schema"));
        }
    }

    #[test]
    fn test_websocket_state_with_both_schemas() {
        let handler: EchoHandler = EchoHandler;
        let message_schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {"action": {"type": "string"}}
        });
        let response_schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {"result": {"type": "string"}}
        });

        let result: Result<WebSocketState<EchoHandler>, String> =
            WebSocketState::with_schemas(handler, Some(message_schema), Some(response_schema));
        assert!(result.is_ok());
        let state: WebSocketState<EchoHandler> = result.unwrap();
        assert!(state.message_schema.is_some());
        assert!(state.response_schema.is_some());
    }

    #[test]
    fn test_websocket_state_cloning_preserves_schemas() {
        let handler: EchoHandler = EchoHandler;
        let schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {"id": {"type": "integer"}}
        });

        let state: WebSocketState<EchoHandler> = WebSocketState::with_schemas(handler, Some(schema), None).unwrap();
        let cloned: WebSocketState<EchoHandler> = state.clone();

        assert!(cloned.message_schema.is_some());
        assert!(cloned.response_schema.is_none());
        assert!(Arc::ptr_eq(&state.handler, &cloned.handler));
    }

    #[tokio::test]
    async fn test_tracking_handler_lifecycle() {
        let handler: TrackingHandler = TrackingHandler::new();
        handler.on_connect().await;
        assert_eq!(handler.connect_count.load(Ordering::SeqCst), 1);

        let msg: Value = serde_json::json!({"test": "data"});
        let _response: Option<Value> = handler.handle_message(msg).await;
        assert_eq!(handler.message_count.load(Ordering::SeqCst), 1);

        handler.on_disconnect().await;
        assert_eq!(handler.disconnect_count.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_selective_handler_responds_conditionally() {
        let handler: SelectiveHandler = SelectiveHandler;

        let respond_msg: Value = serde_json::json!({"respond": true});
        let response1: Option<Value> = handler.handle_message(respond_msg).await;
        assert!(response1.is_some());
        assert_eq!(response1.unwrap(), serde_json::json!({"response": "acknowledged"}));

        let no_respond_msg: Value = serde_json::json!({"respond": false});
        let response2: Option<Value> = handler.handle_message(no_respond_msg).await;
        assert!(response2.is_none());
    }

    #[tokio::test]
    async fn test_transform_handler_modifies_message() {
        let handler: TransformHandler = TransformHandler;
        let original: Value = serde_json::json!({"name": "test"});
        let transformed: Option<Value> = handler.handle_message(original).await;

        assert!(transformed.is_some());
        let resp: Value = transformed.unwrap();
        assert_eq!(resp.get("name").unwrap(), "test");
        assert_eq!(resp.get("processed").unwrap(), true);
    }

    #[tokio::test]
    async fn test_echo_handler_preserves_json_types() {
        let handler: EchoHandler = EchoHandler;

        let messages: Vec<Value> = vec![
            serde_json::json!({"string": "value"}),
            serde_json::json!({"number": 42}),
            serde_json::json!({"float": 3.14}),
            serde_json::json!({"bool": true}),
            serde_json::json!({"null": null}),
            serde_json::json!({"array": [1, 2, 3]}),
        ];

        for msg in messages {
            let response: Option<Value> = handler.handle_message(msg.clone()).await;
            assert!(response.is_some());
            assert_eq!(response.unwrap(), msg);
        }
    }

    #[tokio::test]
    async fn test_tracking_handler_accumulates_messages() {
        let handler: TrackingHandler = TrackingHandler::new();

        let messages: Vec<Value> = vec![
            serde_json::json!({"id": 1}),
            serde_json::json!({"id": 2}),
            serde_json::json!({"id": 3}),
        ];

        for msg in messages {
            let _: Option<Value> = handler.handle_message(msg).await;
        }

        assert_eq!(handler.message_count.load(Ordering::SeqCst), 3);
        let stored: Vec<Value> = handler.messages.lock().unwrap().clone();
        assert_eq!(stored.len(), 3);
        assert_eq!(stored[0].get("id").unwrap(), 1);
        assert_eq!(stored[1].get("id").unwrap(), 2);
        assert_eq!(stored[2].get("id").unwrap(), 3);
    }

    #[tokio::test]
    async fn test_echo_handler_with_nested_json() {
        let handler: EchoHandler = EchoHandler;
        let nested: Value = serde_json::json!({
            "level1": {
                "level2": {
                    "level3": {
                        "value": "deeply nested"
                    }
                }
            }
        });

        let response: Option<Value> = handler.handle_message(nested.clone()).await;
        assert!(response.is_some());
        assert_eq!(response.unwrap(), nested);
    }

    #[tokio::test]
    async fn test_echo_handler_with_large_array() {
        let handler: EchoHandler = EchoHandler;
        let large_array: Value = serde_json::json!({
            "items": (0..1000).collect::<Vec<i32>>()
        });

        let response: Option<Value> = handler.handle_message(large_array.clone()).await;
        assert!(response.is_some());
        assert_eq!(response.unwrap(), large_array);
    }

    #[tokio::test]
    async fn test_echo_handler_with_unicode() {
        let handler: EchoHandler = EchoHandler;
        let unicode_msg: Value = serde_json::json!({
            "emoji": "🚀",
            "chinese": "你好",
            "arabic": "مرحبا",
            "mixed": "Hello 世界 🌍"
        });

        let response: Option<Value> = handler.handle_message(unicode_msg.clone()).await;
        assert!(response.is_some());
        assert_eq!(response.unwrap(), unicode_msg);
    }

    #[test]
    fn test_websocket_state_schemas_are_independent() {
        let handler: EchoHandler = EchoHandler;
        let message_schema: serde_json::Value = serde_json::json!({"type": "object"});
        let response_schema: serde_json::Value = serde_json::json!({"type": "array"});

        let state: WebSocketState<EchoHandler> =
            WebSocketState::with_schemas(handler, Some(message_schema), Some(response_schema)).unwrap();

        let cloned: WebSocketState<EchoHandler> = state.clone();

        assert!(state.message_schema.is_some());
        assert!(state.response_schema.is_some());
        assert!(cloned.message_schema.is_some());
        assert!(cloned.response_schema.is_some());
    }

    #[test]
    fn test_message_schema_validation_with_required_field() {
        let handler: EchoHandler = EchoHandler;
        let message_schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {"type": {"type": "string"}},
            "required": ["type"]
        });

        let state: WebSocketState<EchoHandler> =
            WebSocketState::with_schemas(handler, Some(message_schema), None).unwrap();

        assert!(state.message_schema.is_some());
        assert!(state.response_schema.is_none());

        let valid_msg: Value = serde_json::json!({"type": "test"});
        let validator: &jsonschema::Validator = state.message_schema.as_ref().unwrap();
        assert!(validator.is_valid(&valid_msg));

        let invalid_msg: Value = serde_json::json!({"other": "field"});
        assert!(!validator.is_valid(&invalid_msg));
    }

    #[test]
    fn test_response_schema_validation_with_required_field() {
        let handler: EchoHandler = EchoHandler;
        let response_schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {"status": {"type": "string"}},
            "required": ["status"]
        });

        let state: WebSocketState<EchoHandler> =
            WebSocketState::with_schemas(handler, None, Some(response_schema)).unwrap();

        assert!(state.message_schema.is_none());
        assert!(state.response_schema.is_some());

        let valid_response: Value = serde_json::json!({"status": "ok"});
        let validator: &jsonschema::Validator = state.response_schema.as_ref().unwrap();
        assert!(validator.is_valid(&valid_response));

        let invalid_response: Value = serde_json::json!({"other": "field"});
        assert!(!validator.is_valid(&invalid_response));
    }

    #[test]
    fn test_invalid_message_schema_returns_error() {
        let handler: EchoHandler = EchoHandler;
        let invalid_schema: serde_json::Value = serde_json::json!({
            "type": "invalid_type_value",
            "properties": {}
        });

        let result: Result<WebSocketState<EchoHandler>, String> =
            WebSocketState::with_schemas(handler, Some(invalid_schema), None);

        assert!(result.is_err());
        match result {
            Err(error_msg) => assert!(error_msg.contains("Invalid message schema")),
            Ok(_) => panic!("Expected error but got Ok"),
        }
    }

    #[test]
    fn test_invalid_response_schema_returns_error() {
        let handler: EchoHandler = EchoHandler;
        let invalid_schema: serde_json::Value = serde_json::json!({
            "type": "definitely_not_valid"
        });

        let result: Result<WebSocketState<EchoHandler>, String> =
            WebSocketState::with_schemas(handler, None, Some(invalid_schema));

        assert!(result.is_err());
        match result {
            Err(error_msg) => assert!(error_msg.contains("Invalid response schema")),
            Ok(_) => panic!("Expected error but got Ok"),
        }
    }

    #[tokio::test]
    async fn test_handler_returning_none_response() {
        let handler: SelectiveHandler = SelectiveHandler;

        let no_response_msg: Value = serde_json::json!({"respond": false});
        let result: Option<Value> = handler.handle_message(no_response_msg).await;

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_handler_with_complex_schema_validation() {
        let handler: EchoHandler = EchoHandler;
        let message_schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {
                "user": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "integer"},
                        "name": {"type": "string"}
                    },
                    "required": ["id", "name"]
                },
                "action": {"type": "string"}
            },
            "required": ["user", "action"]
        });

        let state: WebSocketState<EchoHandler> =
            WebSocketState::with_schemas(handler, Some(message_schema), None).unwrap();

        let valid_msg: Value = serde_json::json!({
            "user": {"id": 123, "name": "Alice"},
            "action": "create"
        });
        let validator: &jsonschema::Validator = state.message_schema.as_ref().unwrap();
        assert!(validator.is_valid(&valid_msg));

        let invalid_msg: Value = serde_json::json!({
            "user": {"id": "not_an_int", "name": "Bob"},
            "action": "create"
        });
        assert!(!validator.is_valid(&invalid_msg));
    }

    #[tokio::test]
    async fn test_tracking_handler_with_multiple_message_types() {
        let handler: TrackingHandler = TrackingHandler::new();

        let messages: Vec<Value> = vec![
            serde_json::json!({"type": "text", "content": "hello"}),
            serde_json::json!({"type": "image", "url": "http://example.com/image.png"}),
            serde_json::json!({"type": "video", "duration": 120}),
        ];

        for msg in messages {
            let _: Option<Value> = handler.handle_message(msg).await;
        }

        assert_eq!(handler.message_count.load(Ordering::SeqCst), 3);
        let stored: Vec<Value> = handler.messages.lock().unwrap().clone();
        assert_eq!(stored.len(), 3);
        assert_eq!(stored[0].get("type").unwrap(), "text");
        assert_eq!(stored[1].get("type").unwrap(), "image");
        assert_eq!(stored[2].get("type").unwrap(), "video");
    }

    #[tokio::test]
    async fn test_selective_handler_with_explicit_false() {
        let handler: SelectiveHandler = SelectiveHandler;

        let msg: Value = serde_json::json!({"respond": false, "data": "test"});
        let response: Option<Value> = handler.handle_message(msg).await;

        assert!(response.is_none());
    }

    #[tokio::test]
    async fn test_selective_handler_without_respond_field() {
        let handler: SelectiveHandler = SelectiveHandler;

        let msg: Value = serde_json::json!({"data": "test"});
        let response: Option<Value> = handler.handle_message(msg).await;

        assert!(response.is_none());
    }

    #[tokio::test]
    async fn test_transform_handler_with_empty_object() {
        let handler: TransformHandler = TransformHandler;
        let original: Value = serde_json::json!({});
        let transformed: Option<Value> = handler.handle_message(original).await;

        assert!(transformed.is_some());
        let resp: Value = transformed.unwrap();
        assert_eq!(resp.get("processed").unwrap(), true);
        assert_eq!(resp.as_object().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_transform_handler_preserves_all_fields() {
        let handler: TransformHandler = TransformHandler;
        let original: Value = serde_json::json!({
            "field1": "value1",
            "field2": 42,
            "field3": true,
            "nested": {"key": "value"}
        });
        let transformed: Option<Value> = handler.handle_message(original.clone()).await;

        assert!(transformed.is_some());
        let resp: Value = transformed.unwrap();
        assert_eq!(resp.get("field1").unwrap(), "value1");
        assert_eq!(resp.get("field2").unwrap(), 42);
        assert_eq!(resp.get("field3").unwrap(), true);
        assert_eq!(resp.get("nested").unwrap(), &serde_json::json!({"key": "value"}));
        assert_eq!(resp.get("processed").unwrap(), true);
    }

    #[tokio::test]
    async fn test_transform_handler_with_non_object_input() {
        let handler: TransformHandler = TransformHandler;

        let array: Value = serde_json::json!([1, 2, 3]);
        let response1: Option<Value> = handler.handle_message(array).await;
        assert!(response1.is_none());

        let string: Value = serde_json::json!("not an object");
        let response2: Option<Value> = handler.handle_message(string).await;
        assert!(response2.is_none());

        let number: Value = serde_json::json!(42);
        let response3: Option<Value> = handler.handle_message(number).await;
        assert!(response3.is_none());
    }

    /// Test message validation failure with schema constraint
    #[test]
    fn test_message_schema_rejects_wrong_type() {
        let handler: EchoHandler = EchoHandler;
        let message_schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {"id": {"type": "integer"}},
            "required": ["id"]
        });

        let state: WebSocketState<EchoHandler> =
            WebSocketState::with_schemas(handler, Some(message_schema), None).unwrap();

        let invalid_msg: Value = serde_json::json!({"id": "not_an_integer"});
        let validator: &jsonschema::Validator = state.message_schema.as_ref().unwrap();
        assert!(!validator.is_valid(&invalid_msg));
    }

    /// Test response schema validation failure
    #[test]
    fn test_response_schema_rejects_invalid_type() {
        let handler: EchoHandler = EchoHandler;
        let response_schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {"count": {"type": "integer"}},
            "required": ["count"]
        });

        let state: WebSocketState<EchoHandler> =
            WebSocketState::with_schemas(handler, None, Some(response_schema)).unwrap();

        let invalid_response: Value = serde_json::json!([1, 2, 3]);
        let validator: &jsonschema::Validator = state.response_schema.as_ref().unwrap();
        assert!(!validator.is_valid(&invalid_response));
    }

    /// Test message with multiple required fields missing
    #[test]
    fn test_message_missing_multiple_required_fields() {
        let handler: EchoHandler = EchoHandler;
        let message_schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {
                "user_id": {"type": "integer"},
                "action": {"type": "string"},
                "timestamp": {"type": "string"}
            },
            "required": ["user_id", "action", "timestamp"]
        });

        let state: WebSocketState<EchoHandler> =
            WebSocketState::with_schemas(handler, Some(message_schema), None).unwrap();

        let invalid_msg: Value = serde_json::json!({"other": "value"});
        let validator: &jsonschema::Validator = state.message_schema.as_ref().unwrap();
        assert!(!validator.is_valid(&invalid_msg));

        let partial_msg: Value = serde_json::json!({"user_id": 123});
        assert!(!validator.is_valid(&partial_msg));
    }

    /// Test deeply nested schema validation with required nested properties
    #[test]
    fn test_deeply_nested_schema_validation_failure() {
        let handler: EchoHandler = EchoHandler;
        let message_schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {
                "metadata": {
                    "type": "object",
                    "properties": {
                        "request": {
                            "type": "object",
                            "properties": {
                                "id": {"type": "string"}
                            },
                            "required": ["id"]
                        }
                    },
                    "required": ["request"]
                }
            },
            "required": ["metadata"]
        });

        let state: WebSocketState<EchoHandler> =
            WebSocketState::with_schemas(handler, Some(message_schema), None).unwrap();

        let invalid_msg: Value = serde_json::json!({
            "metadata": {
                "request": {}
            }
        });
        let validator: &jsonschema::Validator = state.message_schema.as_ref().unwrap();
        assert!(!validator.is_valid(&invalid_msg));
    }

    /// Test array property validation with items constraint
    #[test]
    fn test_array_property_type_validation() {
        let handler: EchoHandler = EchoHandler;
        let message_schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {
                "ids": {
                    "type": "array",
                    "items": {"type": "integer"}
                }
            }
        });

        let state: WebSocketState<EchoHandler> =
            WebSocketState::with_schemas(handler, Some(message_schema), None).unwrap();

        let validator: &jsonschema::Validator = state.message_schema.as_ref().unwrap();

        let valid_msg: Value = serde_json::json!({"ids": [1, 2, 3]});
        assert!(validator.is_valid(&valid_msg));

        let invalid_msg: Value = serde_json::json!({"ids": [1, "two", 3]});
        assert!(!validator.is_valid(&invalid_msg));

        let invalid_msg2: Value = serde_json::json!({"ids": "not_an_array"});
        assert!(!validator.is_valid(&invalid_msg2));
    }

    /// Test enum/const property validation
    #[test]
    fn test_enum_property_validation() {
        let handler: EchoHandler = EchoHandler;
        let message_schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {
                "status": {
                    "type": "string",
                    "enum": ["pending", "active", "completed"]
                }
            },
            "required": ["status"]
        });

        let state: WebSocketState<EchoHandler> =
            WebSocketState::with_schemas(handler, Some(message_schema), None).unwrap();

        let validator: &jsonschema::Validator = state.message_schema.as_ref().unwrap();

        let valid_msg: Value = serde_json::json!({"status": "active"});
        assert!(validator.is_valid(&valid_msg));

        let invalid_msg: Value = serde_json::json!({"status": "unknown"});
        assert!(!validator.is_valid(&invalid_msg));
    }

    /// Test minimum/maximum constraints on numbers
    #[test]
    fn test_number_range_validation() {
        let handler: EchoHandler = EchoHandler;
        let message_schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {
                "age": {
                    "type": "integer",
                    "minimum": 0,
                    "maximum": 150
                }
            },
            "required": ["age"]
        });

        let state: WebSocketState<EchoHandler> =
            WebSocketState::with_schemas(handler, Some(message_schema), None).unwrap();

        let validator: &jsonschema::Validator = state.message_schema.as_ref().unwrap();

        let valid_msg: Value = serde_json::json!({"age": 25});
        assert!(validator.is_valid(&valid_msg));

        let invalid_msg: Value = serde_json::json!({"age": -1});
        assert!(!validator.is_valid(&invalid_msg));

        let invalid_msg2: Value = serde_json::json!({"age": 200});
        assert!(!validator.is_valid(&invalid_msg2));
    }

    /// Test string length constraints
    #[test]
    fn test_string_length_validation() {
        let handler: EchoHandler = EchoHandler;
        let message_schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {
                "username": {
                    "type": "string",
                    "minLength": 3,
                    "maxLength": 20
                }
            },
            "required": ["username"]
        });

        let state: WebSocketState<EchoHandler> =
            WebSocketState::with_schemas(handler, Some(message_schema), None).unwrap();

        let validator: &jsonschema::Validator = state.message_schema.as_ref().unwrap();

        let valid_msg: Value = serde_json::json!({"username": "alice"});
        assert!(validator.is_valid(&valid_msg));

        let invalid_msg: Value = serde_json::json!({"username": "ab"});
        assert!(!validator.is_valid(&invalid_msg));

        let invalid_msg2: Value =
            serde_json::json!({"username": "this_is_a_very_long_username_over_twenty_characters"});
        assert!(!validator.is_valid(&invalid_msg2));
    }

    /// Test pattern (regex) validation
    #[test]
    fn test_pattern_validation() {
        let handler: EchoHandler = EchoHandler;
        let message_schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {
                "email": {
                    "type": "string",
                    "pattern": "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
                }
            },
            "required": ["email"]
        });

        let state: WebSocketState<EchoHandler> =
            WebSocketState::with_schemas(handler, Some(message_schema), None).unwrap();

        let validator: &jsonschema::Validator = state.message_schema.as_ref().unwrap();

        let valid_msg: Value = serde_json::json!({"email": "user@example.com"});
        assert!(validator.is_valid(&valid_msg));

        let invalid_msg: Value = serde_json::json!({"email": "user@example"});
        assert!(!validator.is_valid(&invalid_msg));

        let invalid_msg2: Value = serde_json::json!({"email": "userexample.com"});
        assert!(!validator.is_valid(&invalid_msg2));
    }

    /// Test additionalProperties constraint
    #[test]
    fn test_additional_properties_validation() {
        let handler: EchoHandler = EchoHandler;
        let message_schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"}
            },
            "additionalProperties": false
        });

        let state: WebSocketState<EchoHandler> =
            WebSocketState::with_schemas(handler, Some(message_schema), None).unwrap();

        let validator: &jsonschema::Validator = state.message_schema.as_ref().unwrap();

        let valid_msg: Value = serde_json::json!({"name": "Alice"});
        assert!(validator.is_valid(&valid_msg));

        let invalid_msg: Value = serde_json::json!({"name": "Bob", "age": 30});
        assert!(!validator.is_valid(&invalid_msg));
    }

    /// Test oneOf constraint (mutually exclusive properties)
    #[test]
    fn test_one_of_constraint() {
        let handler: EchoHandler = EchoHandler;
        let message_schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "oneOf": [
                {
                    "properties": {"type": {"const": "text"}},
                    "required": ["type"]
                },
                {
                    "properties": {"type": {"const": "number"}},
                    "required": ["type"]
                }
            ]
        });

        let state: WebSocketState<EchoHandler> =
            WebSocketState::with_schemas(handler, Some(message_schema), None).unwrap();

        let validator: &jsonschema::Validator = state.message_schema.as_ref().unwrap();

        let valid_msg: Value = serde_json::json!({"type": "text"});
        assert!(validator.is_valid(&valid_msg));

        let invalid_msg: Value = serde_json::json!({"type": "unknown"});
        assert!(!validator.is_valid(&invalid_msg));
    }

    /// Test anyOf constraint (at least one match)
    #[test]
    fn test_any_of_constraint() {
        let handler: EchoHandler = EchoHandler;
        let message_schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {
                "value": {"type": ["string", "integer"]}
            },
            "required": ["value"]
        });

        let state: WebSocketState<EchoHandler> =
            WebSocketState::with_schemas(handler, Some(message_schema), None).unwrap();

        let validator: &jsonschema::Validator = state.message_schema.as_ref().unwrap();

        let msg1: Value = serde_json::json!({"value": "text"});
        assert!(validator.is_valid(&msg1));

        let msg2: Value = serde_json::json!({"value": 42});
        assert!(validator.is_valid(&msg2));

        let invalid_msg: Value = serde_json::json!({"value": true});
        assert!(!validator.is_valid(&invalid_msg));
    }

    /// Test response validation with complex constraints
    #[test]
    fn test_response_schema_with_multiple_constraints() {
        let handler: EchoHandler = EchoHandler;
        let response_schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {
                "success": {"type": "boolean"},
                "data": {
                    "type": "object",
                    "properties": {
                        "items": {
                            "type": "array",
                            "items": {"type": "object"},
                            "minItems": 1
                        }
                    },
                    "required": ["items"]
                }
            },
            "required": ["success", "data"]
        });

        let state: WebSocketState<EchoHandler> =
            WebSocketState::with_schemas(handler, None, Some(response_schema)).unwrap();

        let validator: &jsonschema::Validator = state.response_schema.as_ref().unwrap();

        let valid_response: Value = serde_json::json!({
            "success": true,
            "data": {
                "items": [{"id": 1}]
            }
        });
        assert!(validator.is_valid(&valid_response));

        let invalid_response: Value = serde_json::json!({
            "success": true,
            "data": {
                "items": []
            }
        });
        assert!(!validator.is_valid(&invalid_response));

        let invalid_response2: Value = serde_json::json!({
            "success": true
        });
        assert!(!validator.is_valid(&invalid_response2));
    }

    /// Test null type validation
    #[test]
    fn test_null_value_validation() {
        let handler: EchoHandler = EchoHandler;
        let message_schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {
                "optional_field": {"type": ["string", "null"]},
                "required_field": {"type": "string"}
            },
            "required": ["required_field"]
        });

        let state: WebSocketState<EchoHandler> =
            WebSocketState::with_schemas(handler, Some(message_schema), None).unwrap();

        let validator: &jsonschema::Validator = state.message_schema.as_ref().unwrap();

        let msg1: Value = serde_json::json!({
            "optional_field": null,
            "required_field": "value"
        });
        assert!(validator.is_valid(&msg1));

        let msg2: Value = serde_json::json!({"required_field": "value"});
        assert!(validator.is_valid(&msg2));

        let invalid_msg: Value = serde_json::json!({"required_field": null});
        assert!(!validator.is_valid(&invalid_msg));
    }

    /// Test schema with default values (they don't change validation)
    #[test]
    fn test_schema_with_defaults_still_validates() {
        let handler: EchoHandler = EchoHandler;
        let message_schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {
                "status": {
                    "type": "string",
                    "default": "pending"
                }
            }
        });

        let state: WebSocketState<EchoHandler> =
            WebSocketState::with_schemas(handler, Some(message_schema), None).unwrap();

        let validator: &jsonschema::Validator = state.message_schema.as_ref().unwrap();

        let msg: Value = serde_json::json!({});
        assert!(validator.is_valid(&msg));
    }

    /// Test both message and response schema validation together
    #[test]
    fn test_both_schemas_validate_independently() {
        let handler: EchoHandler = EchoHandler;
        let message_schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {"action": {"type": "string"}},
            "required": ["action"]
        });
        let response_schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {"result": {"type": "string"}},
            "required": ["result"]
        });

        let state: WebSocketState<EchoHandler> =
            WebSocketState::with_schemas(handler, Some(message_schema), Some(response_schema)).unwrap();

        let msg_validator: &jsonschema::Validator = state.message_schema.as_ref().unwrap();
        let resp_validator: &jsonschema::Validator = state.response_schema.as_ref().unwrap();

        let valid_msg: Value = serde_json::json!({"action": "test"});
        let invalid_response: Value = serde_json::json!({"data": "oops"});

        assert!(msg_validator.is_valid(&valid_msg));
        assert!(!resp_validator.is_valid(&invalid_response));

        let invalid_msg: Value = serde_json::json!({"data": "oops"});
        let valid_response: Value = serde_json::json!({"result": "ok"});

        assert!(!msg_validator.is_valid(&invalid_msg));
        assert!(resp_validator.is_valid(&valid_response));
    }

    /// Test validation with very long/large payload
    #[test]
    fn test_validation_with_large_payload() {
        let handler: EchoHandler = EchoHandler;
        let message_schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {
                "items": {
                    "type": "array",
                    "items": {"type": "integer"}
                }
            },
            "required": ["items"]
        });

        let state: WebSocketState<EchoHandler> =
            WebSocketState::with_schemas(handler, Some(message_schema), None).unwrap();

        let validator: &jsonschema::Validator = state.message_schema.as_ref().unwrap();

        let mut items = Vec::new();
        for i in 0..10_000 {
            items.push(i);
        }
        let large_msg: Value = serde_json::json!({"items": items});

        assert!(validator.is_valid(&large_msg));
    }

    /// Test validation error doesn't panic with invalid schema combinations
    #[test]
    fn test_mutually_exclusive_schema_properties() {
        let handler: EchoHandler = EchoHandler;

        let message_schema: serde_json::Value = serde_json::json!({
            "allOf": [
                {
                    "type": "object",
                    "properties": {"a": {"type": "string"}},
                    "required": ["a"]
                },
                {
                    "type": "object",
                    "properties": {"b": {"type": "integer"}},
                    "required": ["b"]
                }
            ]
        });

        let state: WebSocketState<EchoHandler> =
            WebSocketState::with_schemas(handler, Some(message_schema), None).unwrap();

        let validator: &jsonschema::Validator = state.message_schema.as_ref().unwrap();

        let valid_msg: Value = serde_json::json!({"a": "text", "b": 42});
        assert!(validator.is_valid(&valid_msg));

        let invalid_msg: Value = serde_json::json!({"a": "text"});
        assert!(!validator.is_valid(&invalid_msg));
    }
}

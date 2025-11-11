//! Node.js WebSocket handler bindings

use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunction;
use serde_json::Value;
use spikard_http::WebSocketHandler;
use std::sync::Arc;
use tracing::{debug, error};

/// Type alias for Node.js Promise-returning ThreadsafeFunction
#[allow(dead_code)]
type NodeTsfn = ThreadsafeFunction<String, Promise<String>, Vec<String>, napi::Status, false>;

/// Node.js implementation of WebSocketHandler
#[allow(dead_code)]
pub struct NodeWebSocketHandler {
    /// Handler name for debugging
    name: String,
    /// ThreadsafeFunction to call JavaScript handle_message method
    handle_message_tsfn: Arc<NodeTsfn>,
    /// ThreadsafeFunction to call JavaScript on_connect method
    on_connect_tsfn: Option<Arc<NodeTsfn>>,
    /// ThreadsafeFunction to call JavaScript on_disconnect method
    on_disconnect_tsfn: Option<Arc<NodeTsfn>>,
}

impl NodeWebSocketHandler {
    /// Create a new Node.js WebSocket handler
    #[allow(dead_code)]
    pub fn new(
        name: String,
        handle_message_tsfn: NodeTsfn,
        on_connect_tsfn: Option<NodeTsfn>,
        on_disconnect_tsfn: Option<NodeTsfn>,
    ) -> Self {
        Self {
            name,
            handle_message_tsfn: Arc::new(handle_message_tsfn),
            on_connect_tsfn: on_connect_tsfn.map(Arc::new),
            on_disconnect_tsfn: on_disconnect_tsfn.map(Arc::new),
        }
    }
}

impl WebSocketHandler for NodeWebSocketHandler {
    async fn handle_message(&self, message: Value) -> Option<Value> {
        debug!("Node.js WebSocket handler '{}': handle_message", self.name);

        // Serialize message to JSON string for JavaScript
        let json_str = match serde_json::to_string(&message) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to serialize message to JSON: {}", e);
                return None;
            }
        };

        // Call JavaScript function via ThreadsafeFunction
        let func = Arc::clone(&self.handle_message_tsfn);
        let json_output = match func.call_async(json_str).await {
            Ok(promise) => match promise.await {
                Ok(result) => result,
                Err(e) => {
                    error!("JavaScript promise failed in handle_message: {}", e);
                    return None;
                }
            },
            Err(e) => {
                error!("Failed to call JavaScript handle_message: {}", e);
                return None;
            }
        };

        // Parse the JSON response from JavaScript
        match serde_json::from_str::<Value>(&json_output) {
            Ok(value) => Some(value),
            Err(e) => {
                error!("Failed to parse JavaScript response: {}", e);
                None
            }
        }
    }

    async fn on_connect(&self) {
        debug!("Node.js WebSocket handler '{}': on_connect", self.name);

        if let Some(func) = &self.on_connect_tsfn {
            let func = Arc::clone(func);
            let _ = func.call_async("{}".to_string()).await;
            debug!("Node.js WebSocket handler '{}': on_connect completed", self.name);
        }
    }

    async fn on_disconnect(&self) {
        debug!("Node.js WebSocket handler '{}': on_disconnect", self.name);

        if let Some(func) = &self.on_disconnect_tsfn {
            let func = Arc::clone(func);
            let _ = func.call_async("{}".to_string()).await;
            debug!("Node.js WebSocket handler '{}': on_disconnect completed", self.name);
        }
    }
}

/// Create WebSocketState from Node.js handler factory
///
/// This function is designed to be called from JavaScript to register WebSocket handlers.
#[allow(dead_code)]
pub fn create_websocket_state(handler_instance: &Object) -> Result<spikard_http::WebSocketState<NodeWebSocketHandler>> {
    // Extract the handleMessage function
    let handle_message_fn: Function<String, Promise<String>> = handler_instance.get_named_property("handleMessage")?;

    // Build ThreadsafeFunction for handle_message
    let handle_message_tsfn = handle_message_fn
        .build_threadsafe_function()
        .build_callback(|ctx| Ok(vec![ctx.value]))?;

    // Extract optional onConnect function
    let on_connect_tsfn = handler_instance
        .get_named_property::<Function<String, Promise<String>>>("onConnect")
        .ok()
        .and_then(|func| {
            func.build_threadsafe_function()
                .build_callback(|ctx| Ok(vec![ctx.value]))
                .ok()
        });

    // Extract optional onDisconnect function
    let on_disconnect_tsfn = handler_instance
        .get_named_property::<Function<String, Promise<String>>>("onDisconnect")
        .ok()
        .and_then(|func| {
            func.build_threadsafe_function()
                .build_callback(|ctx| Ok(vec![ctx.value]))
                .ok()
        });

    // Create Node WebSocket handler
    let node_handler = NodeWebSocketHandler::new(
        "WebSocketHandler".to_string(),
        handle_message_tsfn,
        on_connect_tsfn,
        on_disconnect_tsfn,
    );

    // Create and return WebSocket state
    Ok(spikard_http::WebSocketState::new(node_handler))
}

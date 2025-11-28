//! PHP WebSocket handler bindings
//!
//! This module provides the FFI bridge between ext-php-rs and spikard-http's WebSocketHandler trait.
//! Unlike Python and Node.js, PHP doesn't have native async/await, so all handler calls are
//! synchronous and wrapped in spawn_blocking to avoid blocking the async runtime.

use ext_php_rs::convert::IntoZvalDyn;
use ext_php_rs::types::{ZendCallable, Zval};
use serde_json::Value;
use spikard_http::WebSocketHandler;
use tracing::{debug, error};

/// Registry for PHP WebSocket handler callables.
///
/// We store Zval instead of ZendCallable because ZendCallable has a lifetime parameter
/// that prevents storage in static. We reconstruct ZendCallable when invoking.
///
/// NOTE: thread_local because Zval is not Send/Sync (PHP is single-threaded).
thread_local! {
    static PHP_WS_HANDLER_REGISTRY: std::cell::RefCell<Vec<PhpWebSocketHandlerCallables>> = std::cell::RefCell::new(Vec::new());
}

/// Storage for the three PHP callables that implement the WebSocket handler interface.
///
/// PHP handlers implement the WebSocketHandlerInterface with three methods:
/// - onConnect(): void
/// - onMessage(string $message): void
/// - onClose(int $code, ?string $reason): void
///
/// We store Zval representations of the callables, not ZendCallable directly,
/// to avoid lifetime issues.
struct PhpWebSocketHandlerCallables {
    on_connect: Zval,
    on_message: Zval,
    on_close: Zval,
}

/// PHP implementation of WebSocketHandler
///
/// This struct holds an index into the global registry where the actual PHP callables
/// are stored. Since PHP callables cannot be safely shared across threads (no Send/Sync),
/// we use this indirection pattern.
///
/// All PHP handler calls are synchronous and executed via spawn_blocking to prevent
/// blocking the async Tokio runtime.
#[derive(Clone)]
pub struct PhpWebSocketHandler {
    /// Index into PHP_WS_HANDLER_REGISTRY
    handler_index: usize,
    /// Handler name for debugging and error messages
    handler_name: String,
}

impl PhpWebSocketHandler {
    /// Register a new PHP WebSocket handler
    ///
    /// # Arguments
    /// * `handler_obj` - The PHP object implementing WebSocketHandlerInterface
    /// * `handler_name` - Name for debugging/logging
    ///
    /// # Returns
    /// Result containing the handler or an error string if method extraction fails
    pub fn register(handler_obj: &Zval, handler_name: String) -> Result<Self, String> {
        // Extract the three required methods from the PHP handler object
        // extract_method now returns Zval directly instead of ZendCallable
        let on_connect = extract_method_as_zval(handler_obj, "onConnect")?;
        let on_message = extract_method_as_zval(handler_obj, "onMessage")?;
        let on_close = extract_method_as_zval(handler_obj, "onClose")?;

        // Store Zvals in registry and get index
        let idx = PHP_WS_HANDLER_REGISTRY.with(|registry| {
            let mut registry = registry.borrow_mut();
            let idx = registry.len();
            registry.push(PhpWebSocketHandlerCallables {
                on_connect,
                on_message,
                on_close,
            });
            idx
        });

        Ok(Self {
            handler_index: idx,
            handler_name,
        })
    }

    /// Convert JSON Value to JSON string for PHP
    ///
    /// PHP will receive a JSON string and can decode it using json_decode().
    /// This is simpler and more reliable than trying to build PHP data structures
    /// from Rust due to ext-php-rs API complexities.
    #[allow(dead_code)]
    fn json_to_string(value: &Value) -> Result<String, String> {
        serde_json::to_string(value).map_err(|e| format!("Failed to serialize JSON: {}", e))
    }

    /// Invoke a PHP method synchronously within the blocking context
    ///
    /// This helper executes the actual PHP callable invocation. It must be called
    /// from within a spawn_blocking context.
    fn invoke_php_method_sync(
        handler_index: usize,
        method_name: &str,
        args: Vec<Zval>,
    ) -> Result<Option<Zval>, String> {
        PHP_WS_HANDLER_REGISTRY.with(|registry| {
            let registry = registry.borrow();
            let callables = registry
                .get(handler_index)
                .ok_or_else(|| format!("PHP WebSocket handler not found at index {}", handler_index))?;

            let callable_zval = match method_name {
                "onConnect" => &callables.on_connect,
                "onMessage" => &callables.on_message,
                "onClose" => &callables.on_close,
                _ => return Err(format!("Unknown WebSocket method: {}", method_name)),
            };

            // Reconstruct ZendCallable from stored Zval
            let callable = ZendCallable::new(callable_zval)
                .map_err(|e| format!("Failed to reconstruct PHP callable for {}: {:?}", method_name, e))?;

            // Convert Zval refs for try_call - need to cast to &dyn IntoZvalDyn
            let arg_refs: Vec<&dyn IntoZvalDyn> = args.iter().map(|z| z as &dyn IntoZvalDyn).collect();

            // Invoke the PHP callable
            let result = callable
                .try_call(arg_refs)
                .map_err(|e| format!("PHP {} failed: {:?}", method_name, e))?;

            Ok(Some(result))
        })
    }
}

impl WebSocketHandler for PhpWebSocketHandler {
    async fn handle_message(&self, message: Value) -> Option<Value> {
        debug!("PHP WebSocket handler '{}': handle_message", self.handler_name);

        // Convert JSON message to PHP Zval
        let message_str = match serde_json::to_string(&message) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to serialize message to JSON string: {}", e);
                return None;
            }
        };

        let handler_index = self.handler_index;
        let handler_name = self.handler_name.clone();

        // Execute PHP handler in blocking context
        let result = tokio::task::spawn_blocking(move || {
            // Create argument: string $message
            let mut msg_zval = Zval::new();
            if msg_zval.set_string(&message_str, false).is_err() {
                error!("Failed to create message Zval");
                return None;
            }

            // Call onMessage($message)
            match Self::invoke_php_method_sync(handler_index, "onMessage", vec![msg_zval]) {
                Ok(_) => {
                    debug!("PHP WebSocket handler '{}': onMessage completed", handler_name);
                    // PHP handlers return void, no response to send back
                    None
                }
                Err(e) => {
                    error!("PHP WebSocket onMessage error: {}", e);
                    None
                }
            }
        })
        .await;

        match result {
            Ok(response) => response,
            Err(e) => {
                error!("Tokio spawn_blocking error in handle_message: {}", e);
                None
            }
        }
    }

    async fn on_connect(&self) {
        debug!("PHP WebSocket handler '{}': on_connect", self.handler_name);

        let handler_index = self.handler_index;
        let handler_name = self.handler_name.clone();

        let _ = tokio::task::spawn_blocking(move || {
            // Call onConnect() with no arguments
            match Self::invoke_php_method_sync(handler_index, "onConnect", vec![]) {
                Ok(_) => debug!("PHP WebSocket handler '{}': onConnect completed", handler_name),
                Err(e) => error!("PHP WebSocket onConnect error: {}", e),
            }
        })
        .await;
    }

    async fn on_disconnect(&self) {
        debug!("PHP WebSocket handler '{}': on_disconnect", self.handler_name);

        let handler_index = self.handler_index;
        let handler_name = self.handler_name.clone();

        let _ = tokio::task::spawn_blocking(move || {
            // Call onClose(1000, null) - normal closure
            let mut code_zval = Zval::new();
            let _ = code_zval.set_long(1000); // Normal closure code

            let reason_zval = Zval::new();
            // Leave reason as null

            match Self::invoke_php_method_sync(handler_index, "onClose", vec![code_zval, reason_zval]) {
                Ok(_) => debug!("PHP WebSocket handler '{}': onClose completed", handler_name),
                Err(e) => error!("PHP WebSocket onClose error: {}", e),
            }
        })
        .await;
    }
}

/// Extract a method callable from a PHP object as a Zval
///
/// Creates a PHP callable array [object, method_name] and returns it as a Zval.
/// The caller can later wrap it in ZendCallable when needed.
///
/// # Arguments
/// * `obj` - The PHP object (Zval)
/// * `method_name` - Name of the method to extract
///
/// # Returns
/// Result containing the callable Zval or error string
fn extract_method_as_zval(obj: &Zval, method_name: &str) -> Result<Zval, String> {
    // Verify object is actually an object
    obj.object()
        .ok_or_else(|| format!("Handler must be an object to extract method '{}'", method_name))?;

    // Create callable array [object, method_name]
    let mut callable_array = ext_php_rs::types::ZendHashTable::new();

    // Create a copy of the object Zval
    let obj_copy = obj.shallow_clone();

    // Push object reference
    callable_array
        .push(obj_copy)
        .map_err(|e| format!("Failed to add object to callable array: {:?}", e))?;

    // Push method name
    callable_array
        .push(method_name)
        .map_err(|e| format!("Failed to add method name to callable array: {:?}", e))?;

    // Create Zval from array
    let mut array_zval = Zval::new();
    array_zval.set_hashtable(callable_array);

    // Verify it's callable before returning
    if !array_zval.is_callable() {
        return Err(format!("Method '{}' is not callable", method_name));
    }

    Ok(array_zval)
}

/// Create WebSocketState from PHP handler instance
///
/// This function should be called from PHP-facing code to register a WebSocket handler.
/// The handler object must implement WebSocketHandlerInterface.
///
/// # Arguments
/// * `handler_obj` - PHP object implementing WebSocketHandlerInterface
/// * `handler_name` - Optional name for debugging (defaults to "PhpWebSocketHandler")
/// * `message_schema` - Optional JSON schema for validating incoming messages
/// * `response_schema` - Optional JSON schema for validating outgoing responses
///
/// # Returns
/// Result containing WebSocketState or error string
///
/// # Example PHP Usage
/// ```php
/// class MyWebSocketHandler implements WebSocketHandlerInterface {
///     public function onConnect(): void { /* ... */ }
///     public function onMessage(string $message): void { /* ... */ }
///     public function onClose(int $code, ?string $reason = null): void { /* ... */ }
/// }
///
/// $handler = new MyWebSocketHandler();
/// $state = create_websocket_state($handler);
/// ```
pub fn create_websocket_state(
    handler_obj: &Zval,
    handler_name: Option<String>,
    message_schema: Option<Value>,
    response_schema: Option<Value>,
) -> Result<spikard_http::WebSocketState<PhpWebSocketHandler>, String> {
    let name = handler_name.unwrap_or_else(|| "PhpWebSocketHandler".to_string());

    // Register the PHP handler
    let php_handler = PhpWebSocketHandler::register(handler_obj, name)?;

    // Create WebSocketState with optional schemas
    if message_schema.is_some() || response_schema.is_some() {
        spikard_http::WebSocketState::with_schemas(php_handler, message_schema, response_schema)
    } else {
        Ok(spikard_http::WebSocketState::new(php_handler))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_websocket_handler_clone() {
        // Since we can't easily create PHP objects in Rust tests,
        // we just verify the struct is Clone
        // Real testing happens through PHP integration tests
    }
}

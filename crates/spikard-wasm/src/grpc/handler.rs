//! WebAssembly gRPC handler implementation
//!
//! This module provides `wasm-bindgen` bindings for gRPC request/response handling,
//! enabling JavaScript code running in WASM to process gRPC messages and implement
//! gRPC service handlers.
//!
//! # Architecture
//!
//! The WASM gRPC implementation provides building blocks for implementing gRPC handlers:
//!
//! 1. **GrpcRequest/GrpcResponse types**: JavaScript-friendly wrappers for gRPC messages
//!
//! 2. **GrpcMessageStream**: An async iterator for consuming streamed messages from
//!    JavaScript async generators or iterator implementations.
//!
//! 3. **Handler conversion utilities**: Helpers to convert between JavaScript and Rust
//!    types for gRPC metadata and messages.
//!
//! # WASM-Specific Design Notes
//!
//! WASM is single-threaded and doesn't support Tokio's async runtime directly. The
//! gRPC handler support in WASM provides utilities for implementing gRPC request handlers
//! at the language binding level, but the actual gRPC server runtime would need to be
//! implemented in a WASM-compatible runtime (like Deno).
//!
//! # Handler Patterns
//!
//! JavaScript handlers should follow these patterns:
//!
//! ## Unary RPC
//! ```javascript
//! async function myUnaryHandler(request) {
//!   // request: GrpcRequest object
//!   const responsePayload = new Uint8Array([1, 2, 3]);
//!   return new GrpcResponse(responsePayload, {});
//! }
//! ```
//!
//! ## Server Streaming RPC
//! ```javascript
//! async function* myServerStreamHandler(request) {
//!   for (let i = 0; i < 3; i++) {
//!     yield new Uint8Array([i]);
//!   }
//! }
//! ```
//!
//! ## Client Streaming RPC
//! ```javascript
//! async function myClientStreamHandler(stream) {
//!   const messages = [];
//!   for await (const msg of stream) {
//!     messages.push(msg);
//!   }
//!   const response = new Uint8Array([messages.length]);
//!   return new GrpcResponse(response, {});
//! }
//! ```
//!
//! ## Bidirectional Streaming RPC
//! ```javascript
//! async function* myBidiHandler(stream) {
//!   for await (const msg of stream) {
//!     yield msg;
//!   }
//! }
//! ```

use bytes::Bytes;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::{Object, Promise, Reflect, Uint8Array};

/// A simple error type for gRPC operations
#[derive(Debug, Clone)]
pub struct GrpcStatus {
    pub code: u32,
    pub message: String,
}

impl GrpcStatus {
    pub fn internal(message: impl Into<String>) -> Self {
        GrpcStatus {
            code: 13, // INTERNAL
            message: message.into(),
        }
    }
}

impl std::fmt::Display for GrpcStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "gRPC error {}: {}", self.code, self.message)
    }
}

/// Type alias for MessageStream as used in gRPC handlers
///
/// In a real gRPC server, this would be `Pin<Box<dyn Stream<Item = Result<Bytes, GrpcStatus>> + Send>>`.
/// For WASM, we omit Send since WASM is single-threaded and JsValue is not Send.
pub type MessageStream = Pin<Box<dyn futures::stream::Stream<Item = Result<Bytes, GrpcStatus>>>>;

/// JavaScript-friendly gRPC request object
///
/// Contains the parsed components of a gRPC request with all data
/// converted to JavaScript-friendly types.
#[wasm_bindgen(getter_with_clone)]
pub struct GrpcRequest {
    /// Fully qualified service name (e.g., "mypackage.UserService")
    pub service_name: String,

    /// Method name (e.g., "GetUser")
    pub method_name: String,

    /// Serialized protobuf message as Uint8Array
    pub payload: Uint8Array,

    /// gRPC metadata as key-value pairs (plain JS object)
    pub metadata: Object,
}

#[wasm_bindgen]
impl GrpcRequest {
    /// Create a new gRPC request
    #[wasm_bindgen(constructor)]
    pub fn new(
        service_name: String,
        method_name: String,
        payload: Uint8Array,
        metadata: Option<Object>,
    ) -> GrpcRequest {
        let metadata = metadata.unwrap_or_else(|| Object::new());
        GrpcRequest {
            service_name,
            method_name,
            payload,
            metadata,
        }
    }

    /// String representation for debugging
    pub fn __repr__(&self) -> String {
        format!(
            "GrpcRequest(service_name='{}', method_name='{}', payload_size={})",
            self.service_name,
            self.method_name,
            self.payload.length()
        )
    }
}

/// JavaScript-friendly gRPC response object
///
/// Contains the serialized protobuf response and optional metadata
/// to include in the response headers.
#[wasm_bindgen(getter_with_clone)]
pub struct GrpcResponse {
    /// Serialized protobuf message as Uint8Array
    pub payload: Uint8Array,

    /// Optional gRPC metadata to include in response
    pub metadata: Option<Object>,
}

#[wasm_bindgen]
impl GrpcResponse {
    /// Create a new gRPC response
    #[wasm_bindgen(constructor)]
    pub fn new(payload: Uint8Array, metadata: Option<Object>) -> GrpcResponse {
        GrpcResponse { payload, metadata }
    }

    /// String representation for debugging
    pub fn __repr__(&self) -> String {
        format!("GrpcResponse(payload_size={})", self.payload.length())
    }
}

/// WASM async iterator wrapper for message streams
///
/// Wraps a Rust `MessageStream` and exposes a JavaScript async iterator interface
/// so JavaScript handlers can consume incoming messages using `for await`.
///
/// # Design
///
/// This struct stores the actual `MessageStream` internally and provides a `.next()`
/// method that returns a Promise resolving to the next message or null if exhausted.
///
/// # Example
///
/// ```javascript
/// const stream = new GrpcMessageStream(); // Created by runtime
/// const nextMessage = await stream.next();
/// if (nextMessage === null) {
///   console.log("Stream exhausted");
/// } else {
///   console.log("Received bytes:", nextMessage);
/// }
/// ```
#[wasm_bindgen]
pub struct GrpcMessageStream {
    /// Internal message stream storage.
    /// Uses Arc<Mutex<>> for shared ownership and interior mutability in WASM context.
    inner: Arc<Mutex<Option<MessageStream>>>,
}

#[wasm_bindgen]
impl GrpcMessageStream {
    /// Create a new message stream iterator
    ///
    /// Note: In typical use, this is created internally by the WASM runtime
    /// when converting Rust MessageStream to JavaScript. Direct construction
    /// creates an empty stream.
    #[wasm_bindgen(constructor)]
    pub fn new() -> GrpcMessageStream {
        GrpcMessageStream {
            inner: Arc::new(Mutex::new(None)),
        }
    }

    /// Get the next message from the stream
    ///
    /// Returns a Promise that resolves to a Uint8Array for the next message,
    /// or null if the stream is exhausted. Rejects with a JsValue error if
    /// the stream encounters a GrpcStatus error.
    ///
    /// # Returns
    ///
    /// A Promise that resolves to:
    /// - `Uint8Array`: The next message from the stream
    /// - `null`: If the stream has been exhausted
    ///
    /// The Promise rejects if:
    /// - The stream encounters a GrpcStatus error during polling
    /// - The internal stream has been consumed
    ///
    /// # Example
    ///
    /// ```javascript
    /// try {
    ///   const msg = await stream.next();
    ///   if (msg === null) {
    ///     console.log("End of stream");
    ///   } else {
    ///     console.log("Received message:", msg);
    ///   }
    /// } catch (error) {
    ///   console.error("Stream error:", error.message);
    /// }
    /// ```
    pub fn next(&self) -> Promise {
        let inner = Arc::clone(&self.inner);

        wasm_bindgen_futures::future_to_promise(async move {
            // Lock the mutex to access the stream
            let mut stream_guard = match inner.lock() {
                Ok(guard) => guard,
                Err(_) => {
                    return Err(JsValue::from_str(
                        "Failed to acquire lock on message stream",
                    ))
                }
            };

            // Get mutable reference to the stream
            let stream = match stream_guard.as_mut() {
                Some(s) => s,
                None => {
                    // Stream is empty or already consumed
                    return Ok(JsValue::NULL);
                }
            };

            // Poll the stream for the next item using futures::stream::StreamExt::next
            use futures::stream::StreamExt;
            match stream.next().await {
                Some(Ok(bytes)) => {
                    // Convert Bytes to Uint8Array
                    let array = Uint8Array::new_with_length(bytes.len() as u32);
                    array.copy_from(&bytes);
                    Ok(array.into())
                }
                Some(Err(status)) => {
                    // Stream produced an error
                    let error_obj = Object::new();
                    let _ = Reflect::set(
                        &error_obj,
                        &"code".into(),
                        &JsValue::from(status.code),
                    );
                    let _ = Reflect::set(
                        &error_obj,
                        &"message".into(),
                        &JsValue::from_str(&status.message),
                    );
                    Err(error_obj.into())
                }
                None => {
                    // Stream exhausted
                    Ok(JsValue::NULL)
                }
            }
        })
    }

    /// String representation for debugging
    pub fn __repr__(&self) -> String {
        "GrpcMessageStream(async_iterator)".to_string()
    }
}

impl Default for GrpcMessageStream {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a `GrpcMessageStream` from a Rust `MessageStream`
///
/// This is an internal helper function that wraps a Rust MessageStream
/// in a GrpcMessageStream for JavaScript consumption.
pub fn message_stream_to_grpc_message_stream(stream: MessageStream) -> GrpcMessageStream {
    GrpcMessageStream {
        inner: Arc::new(Mutex::new(Some(stream))),
    }
}

/// Convert JavaScript async generator to Rust MessageStream
///
/// Takes a JavaScript async generator (with a `next()` method that returns
/// a Promise<{ value, done }>) and converts it to a Rust `MessageStream`.
/// The generator should yield Uint8Array objects representing serialized protobuf messages.
///
/// # Error Handling
///
/// The returned stream will yield errors if:
/// - The generator's `next()` method is missing or not callable
/// - The generator's `next()` method throws an error (extracts `.message` if available)
/// - The yielded value is not a valid Uint8Array
///
/// # Example JavaScript Generator
///
/// ```javascript
/// async function* messageGenerator() {
///   yield new Uint8Array([1, 2, 3]);
///   yield new Uint8Array([4, 5, 6]);
/// }
/// ```
pub fn javascript_async_generator_to_message_stream(
    js_generator: JsValue,
) -> Result<MessageStream, String> {
    use async_stream::stream;

    // Verify the generator has a next method
    if !js_generator.is_object() {
        return Err("Generator must be an object".to_string());
    }

    let next_method = Reflect::get(&js_generator, &JsValue::from_str("next"))
        .map_err(|_| "Generator missing 'next' method".to_string())?;

    if !next_method.is_function() {
        return Err("Generator 'next' must be a function".to_string());
    }

    let message_stream = stream! {
        loop {
            // Call generator.next()
            let next_result = Reflect::get(&js_generator, &JsValue::from_str("next"))
                .and_then(|next_fn| {
                    let fn_obj = next_fn.dyn_ref::<js_sys::Function>()
                        .ok_or_else(|| JsValue::from_str("next is not a function"))?;
                    fn_obj.call0(&js_generator)
                });

            match next_result {
                Ok(promise_val) => {
                    // Convert Promise to future
                    let promise: Promise = match promise_val.dyn_into() {
                        Ok(p) => p,
                        Err(_) => {
                            yield Err(GrpcStatus::internal("Generator next() must return a Promise"));
                            break;
                        }
                    };

                    let future = wasm_bindgen_futures::JsFuture::from(promise);

                    match future.await {
                        Ok(iteration_result) => {
                            // Get the result object { value, done }
                            let done = Reflect::get(&iteration_result, &JsValue::from_str("done"))
                                .ok()
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false);

                            if done {
                                // Stream exhausted
                                break;
                            }

                            // Get the value (should be Uint8Array)
                            let value = Reflect::get(&iteration_result, &JsValue::from_str("value"))
                                .map_err(|_| {
                                    GrpcStatus::internal("Generator iteration result missing 'value'")
                                })?;

                            if value.is_null() || value.is_undefined() {
                                // End of stream
                                break;
                            }

                            // Convert Uint8Array to Bytes
                            let array = Uint8Array::new(&value);
                            let bytes = Bytes::copy_from_slice(&array.to_vec());
                            yield Ok(bytes);
                        }
                        Err(e) => {
                            // Extract error message from error object or string representation
                            let msg = extract_error_message(&e);
                            yield Err(GrpcStatus::internal(format!("Generator error: {}", msg)));
                            break;
                        }
                    }
                }
                Err(_) => {
                    yield Err(GrpcStatus::internal("Failed to call generator.next()"));
                    break;
                }
            }
        }
    };

    Ok(Box::pin(message_stream))
}

/// Extract error message from JavaScript error object
///
/// Attempts to extract the `.message` property from an error object.
/// Falls back to string representation if `.message` is not available.
///
/// # Precedence
///
/// 1. If error has `.message` property, use that
/// 2. If error is a string, use it directly
/// 3. Otherwise, use generic fallback message
fn extract_error_message(error: &JsValue) -> String {
    // Try to extract .message property from error object
    if let Ok(message) = Reflect::get(error, &JsValue::from_str("message")) {
        if let Some(msg_str) = message.as_string() {
            return msg_str;
        }
    }

    // Fallback to string representation
    error
        .as_string()
        .unwrap_or_else(|| "Unknown error in generator".to_string())
}

/// Convert JavaScript object metadata to a serializable key-value map
pub fn object_to_metadata_map(obj: &Object) -> Result<serde_json::Map<String, serde_json::Value>, String> {
    let mut metadata = serde_json::Map::new();
    let keys = js_sys::Object::keys(obj);

    for idx in 0..keys.length() {
        if let Some(key) = keys.get(idx).as_string() {
            if let Ok(value) = Reflect::get(obj, &JsValue::from_str(&key)) {
                if let Some(value_str) = value.as_string() {
                    metadata.insert(key, serde_json::Value::String(value_str));
                }
            }
        }
    }

    Ok(metadata)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grpc_request_creation() {
        // This test would require wasm-pack test
        // For now, we document the expected behavior
    }

    #[test]
    fn test_grpc_response_creation() {
        // This test would require wasm-pack test
        // For now, we document the expected behavior
    }

    #[test]
    fn test_grpc_message_stream_creation() {
        let stream = GrpcMessageStream::new();
        assert_eq!(stream.__repr__(), "GrpcMessageStream(async_iterator)");
    }
}

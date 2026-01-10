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
//! at the language binding level.
//!
//! Key differences from Node.js/Python:
//! - Single-threaded environment (no Send/Sync required)
//! - Use `wasm_bindgen_futures` for JavaScript Promise integration
//! - Direct JavaScript function calls instead of ThreadsafeFunction
//! - Collection-based streaming (pre-collect all messages before processing)
//!
//! # Handler Patterns
//!
//! JavaScript handlers should follow these patterns:
//!
//! ## Unary RPC
//! ```javascript
//! async function myUnaryHandler(request) {
//!   // request: GrpcRequest object with service_name, method_name, payload, metadata
//!   const responsePayload = new Uint8Array([1, 2, 3]);
//!   return new GrpcResponse(responsePayload, {});
//! }
//! ```
//!
//! ## Server Streaming RPC
//! ```javascript
//! async function myServerStreamHandler(request) {
//!   // Return an array of response messages
//!   return {
//!     messages: [new Uint8Array([1]), new Uint8Array([2])],
//!     metadata: {}
//!   };
//! }
//! ```
//!
//! ## Client Streaming RPC
//! ```javascript
//! async function myClientStreamHandler(request) {
//!   // request includes all collected messages in request.messages array
//!   const response = new Uint8Array([request.messages.length]);
//!   return new GrpcResponse(response, {});
//! }
//! ```
//!
//! ## Bidirectional Streaming RPC
//! ```javascript
//! async function myBidiHandler(request) {
//!   // request includes all input messages; return response messages array
//!   return {
//!     messages: request.messages.map(msg => msg),
//!     metadata: {}
//!   };
//! }
//! ```

use bytes::Bytes;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::{Object, Promise, Reflect, Uint8Array};
use spikard_http::grpc::{GrpcHandler, GrpcHandlerResult, GrpcRequestData, GrpcResponseData};
use spikard_http::grpc::streaming::{MessageStream, StreamingRequest};
use std::future::Future;
use async_stream::stream;
use futures_util::StreamExt;

/// Maximum number of messages allowed in a stream
const MAX_STREAM_MESSAGES: usize = 10_000;

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

/// Client streaming request: collected input messages
///
/// Used when calling JavaScript client streaming handlers.
/// The handler receives all collected messages in a single call.
#[wasm_bindgen(getter_with_clone)]
pub struct GrpcClientStreamRequest {
    /// Fully qualified service name (e.g., "mypackage.UserService")
    pub service_name: String,

    /// Method name (e.g., "GetUser")
    pub method_name: String,

    /// gRPC metadata as key-value pairs
    pub metadata: Object,

    /// Collected stream messages as Uint8Arrays
    pub messages: js_sys::Array,
}

#[wasm_bindgen]
impl GrpcClientStreamRequest {
    /// Create a new client streaming request
    #[wasm_bindgen(constructor)]
    pub fn new(
        service_name: String,
        method_name: String,
        metadata: Option<Object>,
        messages: Option<js_sys::Array>,
    ) -> GrpcClientStreamRequest {
        let metadata = metadata.unwrap_or_else(|| Object::new());
        let messages = messages.unwrap_or_else(|| js_sys::Array::new());
        GrpcClientStreamRequest {
            service_name,
            method_name,
            metadata,
            messages,
        }
    }

    /// String representation for debugging
    pub fn __repr__(&self) -> String {
        format!(
            "GrpcClientStreamRequest(service_name='{}', method_name='{}', message_count={})",
            self.service_name,
            self.method_name,
            self.messages.length()
        )
    }
}

/// Bidirectional streaming request: request with collected input messages
///
/// Used when calling JavaScript bidirectional streaming handlers.
/// The handler receives all input messages and returns an array of response messages.
#[wasm_bindgen(getter_with_clone)]
pub struct GrpcBidiStreamRequest {
    /// Fully qualified service name (e.g., "mypackage.UserService")
    pub service_name: String,

    /// Method name (e.g., "GetUser")
    pub method_name: String,

    /// gRPC metadata as key-value pairs
    pub metadata: Object,

    /// Collected stream messages as Uint8Arrays
    pub messages: js_sys::Array,
}

#[wasm_bindgen]
impl GrpcBidiStreamRequest {
    /// Create a new bidirectional streaming request
    #[wasm_bindgen(constructor)]
    pub fn new(
        service_name: String,
        method_name: String,
        metadata: Option<Object>,
        messages: Option<js_sys::Array>,
    ) -> GrpcBidiStreamRequest {
        let metadata = metadata.unwrap_or_else(|| Object::new());
        let messages = messages.unwrap_or_else(|| js_sys::Array::new());
        GrpcBidiStreamRequest {
            service_name,
            method_name,
            metadata,
            messages,
        }
    }

    /// String representation for debugging
    pub fn __repr__(&self) -> String {
        format!(
            "GrpcBidiStreamRequest(service_name='{}', method_name='{}', message_count={})",
            self.service_name,
            self.method_name,
            self.messages.length()
        )
    }
}

/// Bidirectional streaming response: array of response messages
///
/// Returned by JavaScript bidirectional streaming handlers.
/// Each message is converted back to a protobuf message in the response stream.
#[wasm_bindgen(getter_with_clone)]
pub struct GrpcBidiStreamResponse {
    /// Stream response messages as Uint8Arrays
    pub messages: js_sys::Array,

    /// Optional gRPC metadata to include in response
    pub metadata: Option<Object>,
}

#[wasm_bindgen]
impl GrpcBidiStreamResponse {
    /// Create a new bidirectional streaming response
    #[wasm_bindgen(constructor)]
    pub fn new(messages: Option<js_sys::Array>, metadata: Option<Object>) -> GrpcBidiStreamResponse {
        let messages = messages.unwrap_or_else(|| js_sys::Array::new());
        GrpcBidiStreamResponse { messages, metadata }
    }

    /// String representation for debugging
    pub fn __repr__(&self) -> String {
        format!("GrpcBidiStreamResponse(message_count={})", self.messages.length())
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

/// Convert JavaScript object metadata to tonic MetadataMap
fn object_to_metadata_map(obj: &Object) -> Result<tonic::metadata::MetadataMap, String> {
    let mut metadata = tonic::metadata::MetadataMap::new();
    let keys = js_sys::Object::keys(obj);

    for idx in 0..keys.length() {
        if let Some(key) = keys.get(idx).as_string() {
            if let Ok(value) = Reflect::get(obj, &JsValue::from_str(&key)) {
                if let Some(value_str) = value.as_string() {
                    let key_name = key
                        .parse::<tonic::metadata::MetadataKey<tonic::metadata::Ascii>>()
                        .map_err(|e| format!("Invalid metadata key '{}': {}", key, e))?;
                    let value_bytes = value_str
                        .parse::<tonic::metadata::MetadataValue<tonic::metadata::Ascii>>()
                        .map_err(|e| format!("Invalid metadata value '{}': {}", value_str, e))?;
                    metadata.insert(key_name, value_bytes);
                }
            }
        }
    }

    Ok(metadata)
}

/// Convert tonic MetadataMap to JavaScript Object
fn metadata_map_to_object(metadata: &tonic::metadata::MetadataMap) -> Result<Object, String> {
    let obj = Object::new();

    for key_value in metadata.iter() {
        if let tonic::metadata::KeyAndValueRef::Ascii(key, value) = key_value {
            let key_str = key.as_str();
            let value_str = value.to_str().map_err(|e| {
                format!("Invalid metadata value for key {}: {}", key_str, e)
            })?;

            Reflect::set(&obj, &JsValue::from_str(key_str), &JsValue::from_str(value_str))
                .map_err(|_| format!("Failed to set metadata key '{}'", key_str))?;
        }
    }

    Ok(obj)
}

/// WASM gRPC handler wrapper that implements spikard_http::grpc::GrpcHandler
///
/// Uses JavaScript functions directly to call handlers from Rust.
/// Converts between Rust's bytes/metadata types and JavaScript-friendly objects.
/// Supports unary, server streaming, client streaming, and bidirectional streaming RPC modes.
///
/// Note: Since WASM is single-threaded, the handler uses direct function calls
/// instead of ThreadsafeFunction (Node.js) or PyO3's async mechanisms (Python).
pub struct WasmGrpcHandler {
    service_name: String,
    handler_fn: js_sys::Function,
}

impl WasmGrpcHandler {
    /// Create a new WASM gRPC handler wrapper with a JavaScript function
    ///
    /// # Arguments
    ///
    /// * `service_name` - Fully qualified service name
    /// * `handler_fn` - JavaScript function that calls the handler
    pub fn new(service_name: String, handler_fn: js_sys::Function) -> Self {
        Self {
            service_name,
            handler_fn,
        }
    }
}

impl GrpcHandler for WasmGrpcHandler {
    fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        let handler_fn = self.handler_fn.clone();
        let service_name = self.service_name.clone();

        Box::pin(async move {
            // Convert Rust types to JavaScript-friendly types
            let metadata_obj = metadata_map_to_object(&request.metadata)
                .map_err(|e| tonic::Status::internal(format!("Failed to convert metadata: {}", e)))?;

            let array = Uint8Array::new_with_length(request.payload.len() as u32);
            array.copy_from(&request.payload);

            let js_request = GrpcRequest {
                service_name: request.service_name.clone(),
                method_name: request.method_name.clone(),
                payload: array,
                metadata: metadata_obj,
            };

            // Call the JavaScript handler - convert to JsValue for function call
            let js_request_val = serde_wasm_bindgen::to_value(&js_request)
                .map_err(|e| tonic::Status::internal(format!("Failed to serialize request: {}", e)))?;

            let result = handler_fn.call1(&JsValue::undefined(), &js_request_val)
                .map_err(|e| {
                    let msg = extract_error_message(&e);
                    tonic::Status::internal(format!("Handler call failed: {}", msg))
                })?;

            // Check if result is a Promise
            if let Ok(promise) = Promise::resolve(&result).into::<Promise>().ok() {
                let future = wasm_bindgen_futures::JsFuture::from(promise);

                match future.await {
                    Ok(response_val) => {
                        // Extract GrpcResponse from result
                        let response: GrpcResponse = serde_wasm_bindgen::from_value(response_val)
                            .map_err(|e| tonic::Status::internal(format!("Failed to deserialize response: {}", e)))?;

                        let metadata = if let Some(meta_obj) = response.metadata {
                            object_to_metadata_map(&meta_obj)
                                .map_err(|e| tonic::Status::internal(format!("Failed to convert metadata: {}", e)))?
                        } else {
                            tonic::metadata::MetadataMap::new()
                        };

                        Ok(GrpcResponseData {
                            payload: response.payload.to_vec().into(),
                            metadata,
                        })
                    }
                    Err(e) => {
                        let msg = extract_error_message(&e);
                        Err(tonic::Status::internal(format!("Handler promise failed: {}", msg)))
                    }
                }
            } else {
                // Result is not a Promise, treat as immediate response
                let response: GrpcResponse = serde_wasm_bindgen::from_value(result)
                    .map_err(|e| tonic::Status::internal(format!("Failed to deserialize response: {}", e)))?;

                let metadata = if let Some(meta_obj) = response.metadata {
                    object_to_metadata_map(&meta_obj)
                        .map_err(|e| tonic::Status::internal(format!("Failed to convert metadata: {}", e)))?
                } else {
                    tonic::metadata::MetadataMap::new()
                };

                Ok(GrpcResponseData {
                    payload: response.payload.to_vec().into(),
                    metadata,
                })
            }
        })
    }

    fn service_name(&self) -> &str {
        &self.service_name
    }

    fn call_server_stream(
        &self,
        request: GrpcRequestData,
    ) -> Pin<Box<dyn Future<Output = std::result::Result<MessageStream, tonic::Status>> + Send>> {
        let handler_fn = self.handler_fn.clone();
        let service_name = self.service_name.clone();

        Box::pin(async move {
            // Convert Rust request to JavaScript request
            let metadata_obj = metadata_map_to_object(&request.metadata)
                .map_err(|e| tonic::Status::internal(format!("Failed to convert metadata: {}", e)))?;

            let array = Uint8Array::new_with_length(request.payload.len() as u32);
            array.copy_from(&request.payload);

            let js_request = GrpcRequest {
                service_name: request.service_name.clone(),
                method_name: request.method_name.clone(),
                payload: array,
                metadata: metadata_obj,
            };

            let js_request_val = serde_wasm_bindgen::to_value(&js_request)
                .map_err(|e| tonic::Status::internal(format!("Failed to serialize request: {}", e)))?;

            // Call the JavaScript handler
            let result = handler_fn.call1(&JsValue::undefined(), &js_request_val)
                .map_err(|e| {
                    let msg = extract_error_message(&e);
                    tonic::Status::internal(format!("Handler call failed: {}", msg))
                })?;

            // Convert Promise to future
            let promise = Promise::resolve(&result);
            let future = wasm_bindgen_futures::JsFuture::from(promise);

            match future.await {
                Ok(response_val) => {
                    // Deserialize response into GrpcServerStreamResponse
                    #[derive(serde::Deserialize)]
                    struct ServerStreamResponse {
                        #[serde(default)]
                        messages: Vec<Vec<u8>>,
                        metadata: Option<serde_json::Map<String, serde_json::Value>>,
                    }

                    let response: ServerStreamResponse = serde_wasm_bindgen::from_value(response_val)
                        .map_err(|e| tonic::Status::internal(format!("Failed to deserialize response: {}", e)))?;

                    // Convert response messages to MessageStream
                    let message_stream = stream! {
                        for msg_bytes in response.messages {
                            yield Ok(Bytes::from(msg_bytes));
                        }
                    };

                    Ok(Box::pin(message_stream) as MessageStream)
                }
                Err(e) => {
                    let msg = extract_error_message(&e);
                    Err(tonic::Status::internal(format!("Handler promise failed: {}", msg)))
                }
            }
        })
    }

    fn call_client_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = std::result::Result<GrpcResponseData, tonic::Status>> + Send>> {
        let handler_fn = self.handler_fn.clone();
        let service_name = self.service_name.clone();

        Box::pin(async move {
            // Step 1: Collect all messages from input stream into a vector
            let mut collected_messages = Vec::new();
            let mut stream = Box::pin(request.message_stream);

            while let Some(result) = stream.next().await {
                match result {
                    Ok(bytes) => {
                        collected_messages.push(bytes.to_vec());
                    }
                    Err(e) => {
                        return Err(tonic::Status::internal(format!(
                            "Error collecting stream message: {}",
                            e.message()
                        )));
                    }
                }
            }

            // Enforce max stream messages limit
            if collected_messages.len() > MAX_STREAM_MESSAGES {
                return Err(tonic::Status::resource_exhausted(format!(
                    "Client stream exceeded maximum messages: {} > {}",
                    collected_messages.len(),
                    MAX_STREAM_MESSAGES
                )));
            }

            // Step 2: Create request object with collected messages as Uint8Array array
            let messages_array = js_sys::Array::new();
            for msg in collected_messages {
                let array = Uint8Array::new_with_length(msg.len() as u32);
                array.copy_from(&msg);
                messages_array.push(&array);
            }

            let metadata_obj = metadata_map_to_object(&request.metadata)
                .map_err(|e| tonic::Status::internal(format!("Failed to convert metadata: {}", e)))?;

            let client_stream_request = GrpcClientStreamRequest {
                service_name: request.service_name.clone(),
                method_name: request.method_name.clone(),
                metadata: metadata_obj,
                messages: messages_array,
            };

            // Step 3: Call the JavaScript handler with the streaming request
            let js_request_val = serde_wasm_bindgen::to_value(&client_stream_request)
                .map_err(|e| tonic::Status::internal(format!("Failed to serialize request: {}", e)))?;

            let result = handler_fn.call1(&JsValue::undefined(), &js_request_val)
                .map_err(|e| {
                    let msg = extract_error_message(&e);
                    tonic::Status::internal(format!("Handler call failed: {}", msg))
                })?;

            // Convert Promise to future
            let promise = Promise::resolve(&result);
            let future = wasm_bindgen_futures::JsFuture::from(promise);

            match future.await {
                Ok(response_val) => {
                    // Deserialize response
                    let response: GrpcResponse = serde_wasm_bindgen::from_value(response_val)
                        .map_err(|e| tonic::Status::internal(format!("Failed to deserialize response: {}", e)))?;

                    let metadata = if let Some(meta_obj) = response.metadata {
                        object_to_metadata_map(&meta_obj)
                            .map_err(|e| tonic::Status::internal(format!("Failed to convert metadata: {}", e)))?
                    } else {
                        tonic::metadata::MetadataMap::new()
                    };

                    Ok(GrpcResponseData {
                        payload: response.payload.to_vec().into(),
                        metadata,
                    })
                }
                Err(e) => {
                    let msg = extract_error_message(&e);
                    Err(tonic::Status::internal(format!("Handler promise failed: {}", msg)))
                }
            }
        })
    }

    fn call_bidi_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = std::result::Result<MessageStream, tonic::Status>> + Send>> {
        let handler_fn = self.handler_fn.clone();
        let service_name = self.service_name.clone();

        Box::pin(async move {
            // Step 1: Collect all input messages into a vector
            let mut collected_messages = Vec::new();
            let mut stream = Box::pin(request.message_stream);

            while let Some(result) = stream.next().await {
                match result {
                    Ok(bytes) => {
                        collected_messages.push(bytes.to_vec());
                    }
                    Err(e) => {
                        return Err(tonic::Status::internal(format!(
                            "Error collecting stream message: {}",
                            e.message()
                        )));
                    }
                }
            }

            // Enforce max stream messages limit
            if collected_messages.len() > MAX_STREAM_MESSAGES {
                return Err(tonic::Status::resource_exhausted(format!(
                    "Bidirectional stream exceeded maximum messages: {} > {}",
                    collected_messages.len(),
                    MAX_STREAM_MESSAGES
                )));
            }

            // Step 2: Create bidirectional stream request with collected messages
            let messages_array = js_sys::Array::new();
            for msg in collected_messages {
                let array = Uint8Array::new_with_length(msg.len() as u32);
                array.copy_from(&msg);
                messages_array.push(&array);
            }

            let metadata_obj = metadata_map_to_object(&request.metadata)
                .map_err(|e| tonic::Status::internal(format!("Failed to convert metadata: {}", e)))?;

            let bidi_request = GrpcBidiStreamRequest {
                service_name: request.service_name.clone(),
                method_name: request.method_name.clone(),
                metadata: metadata_obj,
                messages: messages_array,
            };

            // Step 3: Call the JavaScript handler with the streaming request
            let js_request_val = serde_wasm_bindgen::to_value(&bidi_request)
                .map_err(|e| tonic::Status::internal(format!("Failed to serialize request: {}", e)))?;

            let result = handler_fn.call1(&JsValue::undefined(), &js_request_val)
                .map_err(|e| {
                    let msg = extract_error_message(&e);
                    tonic::Status::internal(format!("Handler call failed: {}", msg))
                })?;

            // Convert Promise to future
            let promise = Promise::resolve(&result);
            let future = wasm_bindgen_futures::JsFuture::from(promise);

            match future.await {
                Ok(response_val) => {
                    // Deserialize response
                    let response: GrpcBidiStreamResponse = serde_wasm_bindgen::from_value(response_val)
                        .map_err(|e| tonic::Status::internal(format!("Failed to deserialize response: {}", e)))?;

                    // Step 4: Convert response messages array to MessageStream
                    let message_stream = stream! {
                        for idx in 0..response.messages.length() {
                            if let Some(msg_val) = response.messages.get(idx).dyn_ref::<Uint8Array>() {
                                let msg_bytes = msg_val.to_vec();
                                yield Ok(Bytes::from(msg_bytes));
                            } else {
                                yield Err(tonic::Status::internal("Invalid message in response array"));
                            }
                        }
                    };

                    Ok(Box::pin(message_stream) as MessageStream)
                }
                Err(e) => {
                    let msg = extract_error_message(&e);
                    Err(tonic::Status::internal(format!("Handler promise failed: {}", msg)))
                }
            }
        })
    }
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

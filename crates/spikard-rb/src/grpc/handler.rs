//! Ruby gRPC handler implementation
//!
//! This module provides Magnus bindings for gRPC request/response handling,
//! enabling Ruby code to implement gRPC service handlers with full streaming support.

// Allow dead code - these types are exported but not yet integrated into the main Ruby API
#![allow(dead_code)]

use bytes::Bytes;
use futures::stream::StreamExt;
use magnus::prelude::*;
use magnus::value::{InnerValue, Opaque};
use magnus::{Error, Module, RArray, RHash, RString, Ruby, Value};
use spikard_http::grpc::streaming::{MessageStream, StreamingRequest};
use spikard_http::grpc::{GrpcHandler, GrpcHandlerResult, GrpcRequestData, GrpcResponseData};
use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tonic::metadata::MetadataMap;

/// DOS protection limits
const MAX_METADATA_ENTRIES: usize = 128;
const MAX_METADATA_KEY_SIZE: usize = 1024;
const MAX_METADATA_VALUE_SIZE: usize = 8192;
const MAX_PAYLOAD_BYTES: usize = 100 * 1024 * 1024; // 100MB
const MAX_STREAM_MESSAGES: usize = 10_000;
const MAX_STREAM_TOTAL_BYTES: usize = 500 * 1024 * 1024; // 500MB total for streams
const HANDLER_TIMEOUT_SECS: u64 = 30;

/// Ruby gRPC request class
///
/// Represents a gRPC request passed to Ruby handlers.
/// Contains service name, method name, serialized payload, and metadata.
#[derive(Clone)]
#[magnus::wrap(class = "Spikard::Grpc::Request", free_immediately)]
pub struct RubyGrpcRequest {
    service_name: String,
    method_name: String,
    payload: Vec<u8>,
    metadata: HashMap<String, String>,
}

impl RubyGrpcRequest {
    /// Create a new Ruby gRPC request
    pub fn new(service_name: String, method_name: String, payload: Vec<u8>, metadata: HashMap<String, String>) -> Self {
        Self {
            service_name,
            method_name,
            payload,
            metadata,
        }
    }

    /// Get service name
    fn rb_service_name(&self) -> String {
        self.service_name.clone()
    }

    /// Get method name
    fn rb_method_name(&self) -> String {
        self.method_name.clone()
    }

    /// Get payload as Ruby string (binary)
    fn rb_payload(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        Ok(ruby.str_from_slice(&this.payload).as_value())
    }

    /// Get metadata as Ruby hash
    fn rb_metadata(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let hash = ruby.hash_new_capa(this.metadata.len());
        for (key, value) in &this.metadata {
            hash.aset(ruby.str_new(key), ruby.str_new(value))?;
        }
        Ok(hash.as_value())
    }

    /// Get metadata value by key
    fn rb_get_metadata(&self, key: String) -> Option<String> {
        self.metadata.get(&key).cloned()
    }

    /// String representation for debugging
    fn rb_inspect(&self) -> String {
        format!(
            "#<Spikard::Grpc::Request service_name={:?} method_name={:?} payload_size={}>",
            self.service_name,
            self.method_name,
            self.payload.len()
        )
    }
}

/// Ruby gRPC response class
///
/// Represents a gRPC response returned from Ruby handlers.
/// Contains serialized payload and optional metadata.
#[derive(Clone)]
#[magnus::wrap(class = "Spikard::Grpc::Response", free_immediately)]
pub struct RubyGrpcResponse {
    payload: RefCell<Vec<u8>>,
    metadata: RefCell<HashMap<String, String>>,
}

impl RubyGrpcResponse {
    /// Create a new Ruby gRPC response from Ruby
    fn rb_new(payload: RString, metadata: Option<RHash>) -> Result<Self, Error> {
        // SAFETY: RString::as_slice() is safe when the Ruby VM is active (which it is here since
        // we're in a Ruby method call). The slice is immediately copied to owned Vec<u8>.
        let payload_bytes = unsafe { payload.as_slice().to_vec() };

        if payload_bytes.len() > MAX_PAYLOAD_BYTES {
            return Err(Error::new(
                magnus::exception::arg_error(),
                format!(
                    "Payload size {} exceeds maximum {}",
                    payload_bytes.len(),
                    MAX_PAYLOAD_BYTES
                ),
            ));
        }

        let meta = if let Some(hash) = metadata {
            ruby_hash_to_string_map(&hash)?
        } else {
            HashMap::new()
        };

        if meta.len() > MAX_METADATA_ENTRIES {
            return Err(Error::new(
                magnus::exception::arg_error(),
                format!(
                    "Metadata entries {} exceeds maximum {}",
                    meta.len(),
                    MAX_METADATA_ENTRIES
                ),
            ));
        }

        Ok(Self {
            payload: RefCell::new(payload_bytes),
            metadata: RefCell::new(meta),
        })
    }

    /// Get payload
    fn rb_payload(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let payload = this.payload.borrow();
        Ok(ruby.str_from_slice(&payload).as_value())
    }

    /// Set payload
    fn rb_set_payload(&self, payload: RString) -> Result<(), Error> {
        // SAFETY: RString::as_slice() is safe when Ruby VM is active.
        let bytes = unsafe { payload.as_slice().to_vec() };
        if bytes.len() > MAX_PAYLOAD_BYTES {
            return Err(Error::new(
                magnus::exception::arg_error(),
                format!("Payload size {} exceeds maximum {}", bytes.len(), MAX_PAYLOAD_BYTES),
            ));
        }
        *self.payload.borrow_mut() = bytes;
        Ok(())
    }

    /// Get metadata
    fn rb_metadata(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let meta = this.metadata.borrow();
        let hash = ruby.hash_new_capa(meta.len());
        for (key, value) in meta.iter() {
            hash.aset(ruby.str_new(key), ruby.str_new(value))?;
        }
        Ok(hash.as_value())
    }

    /// Set metadata
    fn rb_set_metadata(&self, metadata: RHash) -> Result<(), Error> {
        let meta = ruby_hash_to_string_map(&metadata)?;
        if meta.len() > MAX_METADATA_ENTRIES {
            return Err(Error::new(
                magnus::exception::arg_error(),
                format!(
                    "Metadata entries {} exceeds maximum {}",
                    meta.len(),
                    MAX_METADATA_ENTRIES
                ),
            ));
        }
        *self.metadata.borrow_mut() = meta;
        Ok(())
    }

    /// String representation
    fn rb_inspect(&self) -> String {
        format!(
            "#<Spikard::Grpc::Response payload_size={}>",
            self.payload.borrow().len()
        )
    }
}

/// Convert Ruby hash to HashMap<String, String>
fn ruby_hash_to_string_map(hash: &RHash) -> Result<HashMap<String, String>, Error> {
    let mut map = HashMap::new();
    hash.foreach(|key: Value, value: Value| {
        let key_str = String::try_convert(key)?;
        if key_str.len() > MAX_METADATA_KEY_SIZE {
            return Err(Error::new(
                magnus::exception::arg_error(),
                format!("Metadata key exceeds maximum size {}", MAX_METADATA_KEY_SIZE),
            ));
        }

        let value_str = String::try_convert(value)?;
        if value_str.len() > MAX_METADATA_VALUE_SIZE {
            return Err(Error::new(
                magnus::exception::arg_error(),
                format!("Metadata value exceeds maximum size {}", MAX_METADATA_VALUE_SIZE),
            ));
        }

        map.insert(key_str, value_str);
        Ok(magnus::r_hash::ForEach::Continue)
    })?;
    Ok(map)
}

/// Convert MetadataMap to HashMap<String, String>
fn metadata_map_to_hashmap(metadata: &MetadataMap) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for key_value in metadata.iter() {
        if let tonic::metadata::KeyAndValueRef::Ascii(key, value) = key_value
            && let Ok(value_str) = value.to_str()
        {
            map.insert(key.as_str().to_string(), value_str.to_string());
        }
    }
    map
}

/// Convert HashMap to MetadataMap
fn hashmap_to_metadata_map(map: &HashMap<String, String>) -> Result<MetadataMap, tonic::Status> {
    let mut metadata = MetadataMap::new();
    for (key, value) in map {
        let metadata_key = key
            .parse::<tonic::metadata::MetadataKey<tonic::metadata::Ascii>>()
            .map_err(|e| tonic::Status::invalid_argument(format!("Invalid metadata key '{}': {}", key, e)))?;
        let metadata_value = value
            .parse::<tonic::metadata::MetadataValue<tonic::metadata::Ascii>>()
            .map_err(|e| tonic::Status::invalid_argument(format!("Invalid metadata value for key '{}': {}", key, e)))?;
        metadata.insert(metadata_key, metadata_value);
    }
    Ok(metadata)
}

/// Convert Ruby exception to gRPC status
///
/// Maps Ruby exceptions to appropriate gRPC status codes without exposing
/// internal implementation details to clients.
fn ruby_error_to_grpc_status(err: Error) -> tonic::Status {
    let msg = err.to_string();

    // Log the full error for debugging but return sanitized messages to clients
    tracing::error!(error = %msg, "Ruby handler error");

    // Check common Ruby exception types by message patterns
    // Return sanitized error messages to avoid leaking internal details
    if msg.contains("ArgumentError") || msg.contains("invalid") {
        tonic::Status::invalid_argument("Invalid argument")
    } else if msg.contains("PermissionError") || msg.contains("permission") {
        tonic::Status::permission_denied("Permission denied")
    } else if msg.contains("NotImplementedError") || msg.contains("not implemented") {
        tonic::Status::unimplemented("Method not implemented")
    } else if msg.contains("Timeout") || msg.contains("timeout") {
        tonic::Status::deadline_exceeded("Request timeout")
    } else if msg.contains("NotFoundError") || msg.contains("not found") {
        tonic::Status::not_found("Resource not found")
    } else {
        tonic::Status::internal("Handler error")
    }
}

/// Extract RubyGrpcResponse from a Ruby value
fn extract_ruby_response(response_value: Value) -> Result<(Vec<u8>, HashMap<String, String>), Error> {
    // Try to extract as RubyGrpcResponse
    if let Ok(response) = <&RubyGrpcResponse>::try_convert(response_value) {
        let payload = response.payload.borrow().clone();
        let metadata = response.metadata.borrow().clone();
        return Ok((payload, metadata));
    }

    // Fallback: try to extract as raw bytes (RString)
    if let Ok(bytes) = RString::try_convert(response_value) {
        // SAFETY: RString::as_slice() is safe when Ruby VM is active.
        let payload = unsafe { bytes.as_slice().to_vec() };
        return Ok((payload, HashMap::new()));
    }

    Err(Error::new(
        magnus::exception::type_error(),
        "Response must be a Spikard::Grpc::Response or binary string",
    ))
}

/// Ruby gRPC handler that bridges Ruby code to Rust's GrpcHandler trait
pub struct RubyGrpcHandler {
    /// Ruby handler proc/callable stored safely for cross-thread access
    handler: Opaque<Value>,
    /// Fully qualified service name
    service_name: Arc<str>,
}

impl RubyGrpcHandler {
    /// Create a new Ruby gRPC handler
    pub fn new(handler: Value, service_name: String) -> Self {
        Self {
            handler: Opaque::from(handler),
            service_name: Arc::from(service_name.as_str()),
        }
    }

    /// Create a RubyGrpcRequest from GrpcRequestData
    fn create_ruby_request(_ruby: &Ruby, request: &GrpcRequestData) -> Result<RubyGrpcRequest, Error> {
        let metadata = metadata_map_to_hashmap(&request.metadata);
        Ok(RubyGrpcRequest::new(
            request.service_name.clone(),
            request.method_name.clone(),
            request.payload.to_vec(),
            metadata,
        ))
    }

    /// Call the Ruby handler with a request
    fn call_ruby_handler(ruby: &Ruby, handler: Value, request: RubyGrpcRequest) -> Result<Value, Error> {
        let request_value = ruby.wrap(request);

        // Check if handler is callable or has handle_request method
        if handler.respond_to("call", false)? {
            handler.funcall("call", (request_value,))
        } else if handler.respond_to("handle_request", false)? {
            handler.funcall("handle_request", (request_value,))
        } else {
            Err(Error::new(
                magnus::exception::type_error(),
                "Handler must be callable (respond to #call) or have a #handle_request method",
            ))
        }
    }

    /// Call the Ruby handler for server streaming
    fn call_ruby_server_stream(ruby: &Ruby, handler: Value, request: RubyGrpcRequest) -> Result<Value, Error> {
        let request_value = ruby.wrap(request);

        // Check for handle_server_stream method first, then fall back to call
        if handler.respond_to("handle_server_stream", false)? {
            handler.funcall("handle_server_stream", (request_value,))
        } else if handler.respond_to("call", false)? {
            handler.funcall("call", (request_value,))
        } else {
            Err(Error::new(
                magnus::exception::type_error(),
                "Handler must have #handle_server_stream or #call method for server streaming",
            ))
        }
    }

    /// Call the Ruby handler for client streaming
    fn call_ruby_client_stream(ruby: &Ruby, handler: Value, messages: Vec<Vec<u8>>) -> Result<Value, Error> {
        // Convert messages to Ruby array of binary strings
        let array = ruby.ary_new_capa(messages.len());
        for msg in messages {
            array.push(ruby.str_from_slice(&msg))?;
        }

        // Check for handle_client_stream method first, then fall back to call
        if handler.respond_to("handle_client_stream", false)? {
            handler.funcall("handle_client_stream", (array,))
        } else if handler.respond_to("call", false)? {
            handler.funcall("call", (array,))
        } else {
            Err(Error::new(
                magnus::exception::type_error(),
                "Handler must have #handle_client_stream or #call method for client streaming",
            ))
        }
    }

    /// Call the Ruby handler for bidirectional streaming
    fn call_ruby_bidi_stream(ruby: &Ruby, handler: Value, messages: Vec<Vec<u8>>) -> Result<Value, Error> {
        // Convert messages to Ruby array of binary strings
        let array = ruby.ary_new_capa(messages.len());
        for msg in messages {
            array.push(ruby.str_from_slice(&msg))?;
        }

        // Check for handle_bidi_stream method first, then fall back to call
        if handler.respond_to("handle_bidi_stream", false)? {
            handler.funcall("handle_bidi_stream", (array,))
        } else if handler.respond_to("call", false)? {
            handler.funcall("call", (array,))
        } else {
            Err(Error::new(
                magnus::exception::type_error(),
                "Handler must have #handle_bidi_stream or #call method for bidirectional streaming",
            ))
        }
    }

    /// Collect messages from Ruby Enumerator/Array into Vec<Bytes>
    fn collect_ruby_stream(_ruby: &Ruby, value: Value) -> Result<Vec<Bytes>, Error> {
        let mut messages = Vec::new();

        // Check if it's an Array
        if let Ok(array) = RArray::try_convert(value) {
            for idx in 0..array.len() {
                if messages.len() >= MAX_STREAM_MESSAGES {
                    return Err(Error::new(
                        magnus::exception::runtime_error(),
                        format!("Stream exceeded maximum {} messages", MAX_STREAM_MESSAGES),
                    ));
                }

                let item: Value = array.entry(idx as isize)?;
                let bytes = Self::extract_message_bytes(item)?;
                messages.push(bytes);
            }
            return Ok(messages);
        }

        // Check if it's an Enumerator (responds to each)
        if value.respond_to("each", false)? {
            // Use to_a to collect enumerable
            let array: RArray = value.funcall("to_a", ())?;
            for idx in 0..array.len() {
                if messages.len() >= MAX_STREAM_MESSAGES {
                    return Err(Error::new(
                        magnus::exception::runtime_error(),
                        format!("Stream exceeded maximum {} messages", MAX_STREAM_MESSAGES),
                    ));
                }

                let item: Value = array.entry(idx as isize)?;
                let bytes = Self::extract_message_bytes(item)?;
                messages.push(bytes);
            }
            return Ok(messages);
        }

        Err(Error::new(
            magnus::exception::type_error(),
            "Stream must be an Array or Enumerable",
        ))
    }

    /// Extract bytes from a Ruby value (RubyGrpcResponse or RString)
    fn extract_message_bytes(value: Value) -> Result<Bytes, Error> {
        // Try RubyGrpcResponse
        if let Ok(response) = <&RubyGrpcResponse>::try_convert(value) {
            let payload = response.payload.borrow().clone();
            return Ok(Bytes::from(payload));
        }

        // Try RString
        if let Ok(string) = RString::try_convert(value) {
            // SAFETY: RString::as_slice() is safe when Ruby VM is active.
            let bytes = unsafe { string.as_slice().to_vec() };
            return Ok(Bytes::from(bytes));
        }

        Err(Error::new(
            magnus::exception::type_error(),
            "Stream message must be a Spikard::Grpc::Response or binary string",
        ))
    }
}

impl GrpcHandler for RubyGrpcHandler {
    fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        let handler = self.handler.clone();

        Box::pin(async move {
            // Execute Ruby handler in a blocking context
            tokio::task::spawn_blocking(move || {
                let ruby = match Ruby::get() {
                    Ok(r) => r,
                    Err(e) => return Err(tonic::Status::internal(format!("Failed to get Ruby VM: {}", e))),
                };

                let handler_value = handler.get_inner_with(&ruby);

                // Create Ruby request
                let ruby_request = Self::create_ruby_request(&ruby, &request).map_err(ruby_error_to_grpc_status)?;

                // Call handler
                let response_value =
                    Self::call_ruby_handler(&ruby, handler_value, ruby_request).map_err(ruby_error_to_grpc_status)?;

                // Extract response
                let (payload, metadata_map) =
                    extract_ruby_response(response_value).map_err(ruby_error_to_grpc_status)?;

                let metadata = hashmap_to_metadata_map(&metadata_map)?;

                Ok(GrpcResponseData {
                    payload: Bytes::from(payload),
                    metadata,
                })
            })
            .await
            .map_err(|e| tonic::Status::internal(format!("Task join error: {}", e)))?
        })
    }

    fn service_name(&self) -> &str {
        self.service_name.as_ref()
    }

    fn call_server_stream(
        &self,
        request: GrpcRequestData,
    ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
        let handler = self.handler.clone();

        Box::pin(async move {
            // Execute Ruby handler in blocking context and collect stream
            let messages = tokio::task::spawn_blocking(move || {
                let ruby = match Ruby::get() {
                    Ok(r) => r,
                    Err(e) => return Err(tonic::Status::internal(format!("Failed to get Ruby VM: {}", e))),
                };

                let handler_value = handler.get_inner_with(&ruby);

                // Create Ruby request
                let ruby_request = Self::create_ruby_request(&ruby, &request).map_err(ruby_error_to_grpc_status)?;

                // Call handler for server streaming
                let stream_value = Self::call_ruby_server_stream(&ruby, handler_value, ruby_request)
                    .map_err(ruby_error_to_grpc_status)?;

                // Collect stream messages
                Self::collect_ruby_stream(&ruby, stream_value).map_err(ruby_error_to_grpc_status)
            })
            .await
            .map_err(|e| tonic::Status::internal(format!("Task join error: {}", e)))??;

            // Convert Vec<Bytes> to MessageStream
            let stream = futures::stream::iter(messages.into_iter().map(Ok));
            Ok(Box::pin(stream) as MessageStream)
        })
    }

    fn call_client_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = Result<GrpcResponseData, tonic::Status>> + Send>> {
        let handler = self.handler.clone();

        Box::pin(async move {
            // Collect all incoming messages with size limits
            let mut messages: Vec<Vec<u8>> = Vec::new();
            let mut total_bytes: usize = 0;
            let mut stream = request.message_stream;

            while let Some(result) = stream.next().await {
                if messages.len() >= MAX_STREAM_MESSAGES {
                    return Err(tonic::Status::resource_exhausted(format!(
                        "Client stream exceeded maximum {} messages",
                        MAX_STREAM_MESSAGES
                    )));
                }
                match result {
                    Ok(bytes) => {
                        total_bytes = total_bytes
                            .checked_add(bytes.len())
                            .ok_or_else(|| tonic::Status::resource_exhausted("Stream total size overflow"))?;

                        if total_bytes > MAX_STREAM_TOTAL_BYTES {
                            return Err(tonic::Status::resource_exhausted(format!(
                                "Stream total bytes {} exceeds maximum {}",
                                total_bytes, MAX_STREAM_TOTAL_BYTES
                            )));
                        }

                        messages.push(bytes.to_vec());
                    }
                    Err(status) => return Err(status),
                }
            }

            // Execute Ruby handler in blocking context with timeout
            tokio::time::timeout(
                Duration::from_secs(HANDLER_TIMEOUT_SECS),
                tokio::task::spawn_blocking(move || {
                    let ruby = match Ruby::get() {
                        Ok(r) => r,
                        Err(e) => return Err(tonic::Status::internal(format!("Failed to get Ruby VM: {}", e))),
                    };

                    let handler_value = handler.get_inner_with(&ruby);

                    // Call handler with collected messages
                    let response_value = Self::call_ruby_client_stream(&ruby, handler_value, messages)
                        .map_err(ruby_error_to_grpc_status)?;

                    // Extract response
                    let (payload, metadata_map) =
                        extract_ruby_response(response_value).map_err(ruby_error_to_grpc_status)?;

                    let metadata = hashmap_to_metadata_map(&metadata_map)?;

                    Ok(GrpcResponseData {
                        payload: Bytes::from(payload),
                        metadata,
                    })
                }),
            )
            .await
            .map_err(|_| tonic::Status::deadline_exceeded("Handler timeout"))?
            .map_err(|e| tonic::Status::internal(format!("Task join error: {}", e)))?
        })
    }

    fn call_bidi_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
        let handler = self.handler.clone();

        Box::pin(async move {
            // Collect all incoming messages with size limits
            let mut messages: Vec<Vec<u8>> = Vec::new();
            let mut total_bytes: usize = 0;
            let mut stream = request.message_stream;

            while let Some(result) = stream.next().await {
                if messages.len() >= MAX_STREAM_MESSAGES {
                    return Err(tonic::Status::resource_exhausted(format!(
                        "Client stream exceeded maximum {} messages",
                        MAX_STREAM_MESSAGES
                    )));
                }
                match result {
                    Ok(bytes) => {
                        total_bytes = total_bytes
                            .checked_add(bytes.len())
                            .ok_or_else(|| tonic::Status::resource_exhausted("Stream total size overflow"))?;

                        if total_bytes > MAX_STREAM_TOTAL_BYTES {
                            return Err(tonic::Status::resource_exhausted(format!(
                                "Stream total bytes {} exceeds maximum {}",
                                total_bytes, MAX_STREAM_TOTAL_BYTES
                            )));
                        }

                        messages.push(bytes.to_vec());
                    }
                    Err(status) => return Err(status),
                }
            }

            // Execute Ruby handler in blocking context with timeout
            let response_messages = tokio::time::timeout(
                Duration::from_secs(HANDLER_TIMEOUT_SECS),
                tokio::task::spawn_blocking(move || {
                    let ruby = match Ruby::get() {
                        Ok(r) => r,
                        Err(e) => return Err(tonic::Status::internal(format!("Failed to get Ruby VM: {}", e))),
                    };

                    let handler_value = handler.get_inner_with(&ruby);

                    // Call handler for bidirectional streaming
                    let stream_value = Self::call_ruby_bidi_stream(&ruby, handler_value, messages)
                        .map_err(ruby_error_to_grpc_status)?;

                    // Collect response stream
                    Self::collect_ruby_stream(&ruby, stream_value).map_err(ruby_error_to_grpc_status)
                }),
            )
            .await
            .map_err(|_| tonic::Status::deadline_exceeded("Handler timeout"))?
            .map_err(|e| tonic::Status::internal(format!("Task join error: {}", e)))??;

            // Convert Vec<Bytes> to MessageStream
            let stream = futures::stream::iter(response_messages.into_iter().map(Ok));
            Ok(Box::pin(stream) as MessageStream)
        })
    }
}

/// Register the Ruby gRPC handler module
pub fn init(_ruby: &Ruby, spikard_module: &magnus::RModule) -> Result<(), Error> {
    let grpc_module = spikard_module.define_module("Grpc")?;

    // Define Request class
    let request_class = grpc_module.define_class("Request", _ruby.class_object())?;
    request_class.define_method("service_name", magnus::method!(RubyGrpcRequest::rb_service_name, 0))?;
    request_class.define_method("method_name", magnus::method!(RubyGrpcRequest::rb_method_name, 0))?;
    request_class.define_method("payload", magnus::method!(RubyGrpcRequest::rb_payload, 0))?;
    request_class.define_method("metadata", magnus::method!(RubyGrpcRequest::rb_metadata, 0))?;
    request_class.define_method("get_metadata", magnus::method!(RubyGrpcRequest::rb_get_metadata, 1))?;
    request_class.define_method("inspect", magnus::method!(RubyGrpcRequest::rb_inspect, 0))?;
    request_class.define_method("to_s", magnus::method!(RubyGrpcRequest::rb_inspect, 0))?;

    // Define Response class
    let response_class = grpc_module.define_class("Response", _ruby.class_object())?;
    response_class.define_singleton_method("new", magnus::function!(RubyGrpcResponse::rb_new, 2))?;
    response_class.define_method("payload", magnus::method!(RubyGrpcResponse::rb_payload, 0))?;
    response_class.define_method("payload=", magnus::method!(RubyGrpcResponse::rb_set_payload, 1))?;
    response_class.define_method("metadata", magnus::method!(RubyGrpcResponse::rb_metadata, 0))?;
    response_class.define_method("metadata=", magnus::method!(RubyGrpcResponse::rb_set_metadata, 1))?;
    response_class.define_method("inspect", magnus::method!(RubyGrpcResponse::rb_inspect, 0))?;
    response_class.define_method("to_s", magnus::method!(RubyGrpcResponse::rb_inspect, 0))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_conversion() {
        let mut map = HashMap::new();
        map.insert("authorization".to_string(), "Bearer token".to_string());
        map.insert("content-type".to_string(), "application/grpc".to_string());

        let metadata = hashmap_to_metadata_map(&map).unwrap();
        let converted = metadata_map_to_hashmap(&metadata);

        assert_eq!(converted.get("authorization"), Some(&"Bearer token".to_string()));
        assert_eq!(converted.get("content-type"), Some(&"application/grpc".to_string()));
    }

    #[test]
    fn test_ruby_grpc_request_creation() {
        let mut metadata = HashMap::new();
        metadata.insert("key".to_string(), "value".to_string());

        let request = RubyGrpcRequest::new(
            "test.Service".to_string(),
            "Method".to_string(),
            vec![1, 2, 3],
            metadata,
        );

        assert_eq!(request.service_name, "test.Service");
        assert_eq!(request.method_name, "Method");
        assert_eq!(request.payload, vec![1, 2, 3]);
        assert_eq!(request.metadata.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_inspect_format() {
        let request = RubyGrpcRequest::new(
            "test.Service".to_string(),
            "Method".to_string(),
            vec![1, 2, 3, 4, 5],
            HashMap::new(),
        );

        let repr = request.rb_inspect();
        assert!(repr.contains("test.Service"));
        assert!(repr.contains("Method"));
        assert!(repr.contains("payload_size=5"));
    }
}

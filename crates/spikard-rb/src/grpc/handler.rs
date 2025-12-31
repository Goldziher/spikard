//! Ruby gRPC handler implementation using Magnus FFI
//!
//! This module provides a bridge between Ruby code implementing gRPC handlers
//! and Spikard's Rust-based gRPC runtime. It handles serialization/deserialization
//! of protobuf messages as binary strings.

use bytes::Bytes;
use magnus::prelude::*;
use magnus::value::{InnerValue, Opaque};
use magnus::{Error, RHash, RString, Ruby, TryConvert, Value, gc::Marker};
use spikard_http::grpc::{GrpcHandler, GrpcHandlerResult, GrpcRequestData, GrpcResponseData};
use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::panic::AssertUnwindSafe;
use std::pin::Pin;
use std::sync::Arc;
use tonic::metadata::{MetadataMap, MetadataKey, MetadataValue};

use crate::gvl::with_gvl;

/// Ruby-facing gRPC request object
///
/// This struct is exposed to Ruby code and contains the parsed components
/// of a gRPC request. The payload is provided as a binary string that Ruby
/// code can deserialize using the google-protobuf gem.
#[derive(Debug, Clone)]
#[magnus::wrap(class = "Spikard::Grpc::Request", free_immediately)]
pub struct RubyGrpcRequest {
    service_name: String,
    method_name: String,
    payload: Vec<u8>,
    metadata: HashMap<String, String>,
}

impl RubyGrpcRequest {
    /// Create a new RubyGrpcRequest from GrpcRequestData
    fn from_grpc_request(request: GrpcRequestData) -> Self {
        let metadata = extract_metadata_map(&request.metadata);
        Self {
            service_name: request.service_name,
            method_name: request.method_name,
            payload: request.payload.to_vec(),
            metadata,
        }
    }

    /// Get the service name
    fn service_name(_ruby: &Ruby, rb_self: &Self) -> String {
        rb_self.service_name.clone()
    }

    /// Get the method name
    fn method_name(_ruby: &Ruby, rb_self: &Self) -> String {
        rb_self.method_name.clone()
    }

    /// Get the payload as a binary string
    fn payload(ruby: &Ruby, rb_self: &Self) -> Value {
        ruby.str_from_slice(&rb_self.payload).as_value()
    }

    /// Get metadata as a Ruby hash
    fn metadata(ruby: &Ruby, rb_self: &Self) -> Result<Value, Error> {
        let hash = ruby.hash_new();
        for (key, value) in &rb_self.metadata {
            hash.aset(ruby.str_new(key), ruby.str_new(value))?;
        }
        Ok(hash.as_value())
    }
}

/// Ruby-facing gRPC response object
///
/// Ruby code creates instances of this class to return gRPC responses.
/// The payload should be a binary string containing the serialized protobuf message.
#[derive(Debug, Clone, Default)]
#[magnus::wrap(class = "Spikard::Grpc::Response", free_immediately)]
pub struct RubyGrpcResponse {
    payload: RefCell<Vec<u8>>,
    metadata: RefCell<HashMap<String, String>>,
}

impl RubyGrpcResponse {
    /// Initialize the response with a payload (called by Ruby's new)
    fn initialize(&self, payload: Value) -> Result<(), Error> {
        let ruby = Ruby::get().map_err(|_| {
            Error::new(
                magnus::exception::runtime_error(),
                "Ruby VM unavailable during Response initialization",
            )
        })?;

        let payload_bytes = if let Ok(string) = RString::try_convert(payload) {
            unsafe { string.as_slice() }.to_vec()
        } else {
            return Err(Error::new(
                ruby.exception_arg_error(),
                "payload must be a String (binary)",
            ));
        };

        *self.payload.borrow_mut() = payload_bytes;
        *self.metadata.borrow_mut() = HashMap::new();
        Ok(())
    }

    /// Set metadata on the response
    fn set_metadata(&self, metadata: Value) -> Result<(), Error> {
        if metadata.is_nil() {
            return Ok(());
        }

        let hash = RHash::try_convert(metadata)?;
        let metadata_map = hash.to_hash_map::<String, String>()?;
        *self.metadata.borrow_mut() = metadata_map;
        Ok(())
    }

    /// Get the payload
    fn payload(ruby: &Ruby, rb_self: &Self) -> Value {
        ruby.str_from_slice(&rb_self.payload.borrow()).as_value()
    }

    /// Convert to GrpcResponseData
    fn into_grpc_response(self) -> Result<GrpcResponseData, String> {
        let mut metadata_map = MetadataMap::new();
        for (key, value) in self.metadata.borrow().iter() {
            let metadata_key = MetadataKey::from_bytes(key.as_bytes())
                .map_err(|err| format!("Invalid metadata key '{}': {}", key, err))?;
            let metadata_value = MetadataValue::try_from(value)
                .map_err(|err| format!("Invalid metadata value for '{}': {}", key, err))?;
            metadata_map.insert(metadata_key, metadata_value);
        }

        Ok(GrpcResponseData {
            payload: Bytes::from(self.payload.borrow().clone()),
            metadata: metadata_map,
        })
    }
}

/// Ruby gRPC handler wrapper
///
/// Wraps a Ruby handler object and implements the GrpcHandler trait,
/// allowing Ruby code to handle gRPC requests.
#[derive(Clone)]
pub struct RubyGrpcHandler {
    inner: Arc<RubyGrpcHandlerInner>,
}

struct RubyGrpcHandlerInner {
    handler: Opaque<Value>,
    service_name: String,
}

impl RubyGrpcHandler {
    /// Create a new RubyGrpcHandler
    ///
    /// # Arguments
    ///
    /// * `handler` - A Ruby object that responds to `handle_request(request)`
    /// * `service_name` - The fully qualified service name (e.g., "mypackage.MyService")
    #[allow(dead_code)]
    pub fn new(handler: Value, service_name: String) -> Self {
        Self {
            inner: Arc::new(RubyGrpcHandlerInner {
                handler: Opaque::from(handler),
                service_name,
            }),
        }
    }

    /// Required by Ruby GC; invoked through the magnus mark hook.
    #[allow(dead_code)]
    pub fn mark(&self, marker: &Marker) {
        if let Ok(ruby) = Ruby::get() {
            let handler_val = self.inner.handler.get_inner_with(&ruby);
            marker.mark(handler_val);
        }
    }

    /// Handle a gRPC request by calling into Ruby
    fn handle_request(&self, request: GrpcRequestData) -> GrpcHandlerResult {
        with_gvl(|| {
            let result = std::panic::catch_unwind(AssertUnwindSafe(|| self.handle_request_inner(request)));
            match result {
                Ok(res) => res,
                Err(_) => Err(tonic::Status::internal(
                    "Unexpected panic while executing Ruby gRPC handler",
                )),
            }
        })
    }

    fn handle_request_inner(&self, request: GrpcRequestData) -> GrpcHandlerResult {
        let ruby = Ruby::get().map_err(|_| {
            tonic::Status::internal("Ruby VM unavailable while invoking gRPC handler")
        })?;

        // Convert request to Ruby object
        let ruby_request = RubyGrpcRequest::from_grpc_request(request);
        let request_value = ruby
            .obj_wrap(ruby_request)
            .as_value();

        // Call Ruby handler
        let handler_value = self.inner.handler.get_inner_with(&ruby);
        let response_value = handler_value
            .funcall::<_, _, Value>("handle_request", (request_value,))
            .map_err(|err| {
                tonic::Status::internal(format!("Ruby gRPC handler failed: {}", err))
            })?;

        // Convert Ruby response to GrpcResponseData
        let ruby_response = <&RubyGrpcResponse>::try_convert(response_value)
            .map_err(|err| {
                tonic::Status::internal(format!(
                    "Handler must return Spikard::Grpc::Response, got error: {}",
                    err
                ))
            })?;

        ruby_response
            .clone()
            .into_grpc_response()
            .map_err(|err| tonic::Status::internal(format!("Failed to build gRPC response: {}", err)))
    }
}

impl GrpcHandler for RubyGrpcHandler {
    fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        let handler = self.clone();
        Box::pin(async move { handler.handle_request(request) })
    }

    fn service_name(&self) -> &'static str {
        // We need to return a 'static str, but we have a String.
        // For now, we'll leak the string to get a 'static reference.
        // This is acceptable because service names are registered once at startup.
        Box::leak(self.inner.service_name.clone().into_boxed_str())
    }
}

/// Extract metadata from gRPC MetadataMap to a simple HashMap
fn extract_metadata_map(metadata: &MetadataMap) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for key_value in metadata.iter() {
        match key_value {
            tonic::metadata::KeyAndValueRef::Ascii(key, value) => {
                map.insert(key.as_str().to_string(), value.to_str().unwrap_or("").to_string());
            }
            tonic::metadata::KeyAndValueRef::Binary(key, value) => {
                // Binary metadata - skip or convert to base64 if needed
                let _ = (key, value); // Acknowledge we're not using them
            }
        }
    }
    map
}

/// Initialize the gRPC module in Ruby
pub fn init(ruby: &Ruby, spikard_module: &magnus::RModule) -> Result<(), Error> {
    let grpc_module = spikard_module.define_module("Grpc")?;

    // Define Spikard::Grpc::Request class
    let request_class = grpc_module.define_class("Request", ruby.class_object())?;
    request_class.define_method("service_name", magnus::method!(RubyGrpcRequest::service_name, 0))?;
    request_class.define_method("method_name", magnus::method!(RubyGrpcRequest::method_name, 0))?;
    request_class.define_method("payload", magnus::method!(RubyGrpcRequest::payload, 0))?;
    request_class.define_method("metadata", magnus::method!(RubyGrpcRequest::metadata, 0))?;

    // Define Spikard::Grpc::Response class
    let response_class = grpc_module.define_class("Response", ruby.class_object())?;
    response_class.define_alloc_func::<RubyGrpcResponse>();
    response_class.define_method("initialize", magnus::method!(RubyGrpcResponse::initialize, 1))?;
    response_class.define_method("metadata=", magnus::method!(RubyGrpcResponse::set_metadata, 1))?;
    response_class.define_method("payload", magnus::method!(RubyGrpcResponse::payload, 0))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::metadata::MetadataMap;

    #[test]
    fn test_ruby_grpc_request_creation() {
        let request = GrpcRequestData {
            service_name: "test.TestService".to_string(),
            method_name: "TestMethod".to_string(),
            payload: Bytes::from("test payload"),
            metadata: MetadataMap::new(),
        };

        let ruby_request = RubyGrpcRequest::from_grpc_request(request);
        assert_eq!(ruby_request.service_name, "test.TestService");
        assert_eq!(ruby_request.method_name, "TestMethod");
        assert_eq!(ruby_request.payload, b"test payload");
    }

    #[test]
    fn test_metadata_extraction() {
        let mut metadata = MetadataMap::new();
        metadata.insert("content-type", "application/grpc".parse().unwrap());
        metadata.insert("authorization", "Bearer token123".parse().unwrap());

        let extracted = extract_metadata_map(&metadata);
        assert_eq!(extracted.get("content-type").unwrap(), "application/grpc");
        assert_eq!(extracted.get("authorization").unwrap(), "Bearer token123");
    }

    #[test]
    fn test_grpc_response_conversion() {
        let response = RubyGrpcResponse {
            payload: RefCell::new(b"test response".to_vec()),
            metadata: RefCell::new(HashMap::new()),
        };

        let grpc_response = response.into_grpc_response();
        assert!(grpc_response.is_ok());
        let grpc_response = grpc_response.unwrap();
        assert_eq!(grpc_response.payload, Bytes::from("test response"));
    }

    #[test]
    fn test_grpc_response_with_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("x-custom-header".to_string(), "custom-value".to_string());

        let response = RubyGrpcResponse {
            payload: RefCell::new(b"test".to_vec()),
            metadata: RefCell::new(metadata),
        };

        let grpc_response = response.into_grpc_response();
        assert!(grpc_response.is_ok());
        let grpc_response = grpc_response.unwrap();
        assert!(!grpc_response.metadata.is_empty());
    }

    #[test]
    fn test_invalid_metadata_key() {
        let mut metadata = HashMap::new();
        metadata.insert("invalid\nkey".to_string(), "value".to_string());

        let response = RubyGrpcResponse {
            payload: RefCell::new(b"test".to_vec()),
            metadata: RefCell::new(metadata),
        };

        let result = response.into_grpc_response();
        assert!(result.is_err());
    }
}

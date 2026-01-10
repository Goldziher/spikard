//! Ruby gRPC handler implementation using Magnus FFI
//!
//! This module provides a bridge between Ruby code implementing gRPC handlers
//! and Spikard's Rust-based gRPC runtime. It handles serialization/deserialization
//! of protobuf messages as binary strings.

use bytes::Bytes;
use magnus::prelude::*;
use magnus::value::{InnerValue, Opaque};
use magnus::{Error, RHash, RString, Ruby, Symbol, TryConvert, Value, gc::Marker};
use spikard_bindings_shared::grpc_metadata::{extract_metadata_to_hashmap, hashmap_to_metadata};
use spikard_http::grpc::{GrpcHandler, GrpcHandlerResult, GrpcRequestData, GrpcResponseData};
use spikard_http::grpc::streaming::{MessageStream, StreamingRequest};
use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::panic::AssertUnwindSafe;
use std::pin::Pin;
use std::sync::Arc;

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
        let metadata = extract_metadata_to_hashmap(&request.metadata, true);
        Self {
            service_name: request.service_name,
            method_name: request.method_name,
            payload: request.payload.to_vec(),
            metadata,
        }
    }

    /// Get the service name
    fn service_name(&self) -> &str {
        &self.service_name
    }

    /// Get the method name
    fn method_name(&self) -> &str {
        &self.method_name
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
    fn initialize(&self, args: &[Value]) -> Result<(), Error> {
        // Handle both positional and keyword arguments
        let payload_value = if args.is_empty() {
            return Err(Error::new(magnus::exception::arg_error(), "missing keyword: payload"));
        } else if args.len() == 1 {
            // Check if it's a hash (keyword args) or a string (positional arg)
            if let Ok(hash) = RHash::try_convert(args[0]) {
                // Keyword arguments: { payload: "data" }
                hash.get(Symbol::new("payload"))
                    .ok_or_else(|| Error::new(magnus::exception::arg_error(), "missing keyword: payload"))?
            } else {
                // Positional argument: "data"
                args[0]
            }
        } else {
            return Err(Error::new(magnus::exception::arg_error(), "wrong number of arguments"));
        };

        let payload_str = RString::try_convert(payload_value)
            .map_err(|_| Error::new(magnus::exception::arg_error(), "payload must be a String (binary)"))?;

        let payload_bytes = unsafe { payload_str.as_slice() }.to_vec();

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

    /// Get metadata as a Ruby hash
    fn get_metadata(ruby: &Ruby, rb_self: &Self) -> Result<Value, Error> {
        let hash = ruby.hash_new();
        for (key, value) in rb_self.metadata.borrow().iter() {
            hash.aset(ruby.str_new(key), ruby.str_new(value))?;
        }
        Ok(hash.as_value())
    }

    /// Convert to GrpcResponseData
    fn into_grpc_response(self) -> Result<GrpcResponseData, String> {
        let metadata = hashmap_to_metadata(&self.metadata.borrow())?;

        Ok(GrpcResponseData {
            payload: Bytes::from(self.payload.borrow().clone()),
            metadata,
        })
    }
}

/// Ruby-facing gRPC message stream for streaming RPC
///
/// Wraps a Rust `MessageStream` and exposes it to Ruby handlers
/// so they can consume incoming messages.
#[derive(Clone)]
#[allow(dead_code)]
#[magnus::wrap(class = "Spikard::Grpc::MessageStream", free_immediately)]
pub struct RubyGrpcMessageStream {
    /// The underlying message stream
    stream: Arc<tokio::sync::Mutex<Option<MessageStream>>>,
}

impl RubyGrpcMessageStream {
    /// Create a new RubyGrpcMessageStream
    fn new(stream: MessageStream) -> Self {
        Self {
            stream: Arc::new(tokio::sync::Mutex::new(Some(stream))),
        }
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
        let ruby =
            Ruby::get().map_err(|_| tonic::Status::internal("Ruby VM unavailable while invoking gRPC handler"))?;

        // Convert request to Ruby object
        let ruby_request = RubyGrpcRequest::from_grpc_request(request);
        let request_value = ruby.obj_wrap(ruby_request).as_value();

        // Call Ruby handler
        let handler_value = self.inner.handler.get_inner_with(&ruby);
        let response_value = handler_value
            .funcall::<_, _, Value>("handle_request", (request_value,))
            .map_err(|err| tonic::Status::internal(format!("Ruby gRPC handler failed: {}", err)))?;

        // Convert Ruby response to GrpcResponseData
        let ruby_response = <&RubyGrpcResponse>::try_convert(response_value).map_err(|err| {
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

    /// Handle a server streaming request by calling Ruby handler
    fn handle_server_stream(&self, request: GrpcRequestData) -> Result<MessageStream, tonic::Status> {
        with_gvl(|| {
            let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
                self.handle_server_stream_inner(request)
            }));
            match result {
                Ok(res) => res,
                Err(_) => Err(tonic::Status::internal(
                    "Unexpected panic while executing Ruby server streaming gRPC handler",
                )),
            }
        })
    }

    fn handle_server_stream_inner(&self, request: GrpcRequestData) -> Result<MessageStream, tonic::Status> {
        let ruby = Ruby::get()
            .map_err(|_| tonic::Status::internal("Ruby VM unavailable while invoking server streaming handler"))?;

        // Convert request to Ruby object
        let ruby_request = RubyGrpcRequest::from_grpc_request(request);
        let request_value = ruby.obj_wrap(ruby_request).as_value();

        // Call Ruby handler's server_stream method
        let handler_value = self.inner.handler.get_inner_with(&ruby);
        let enumerator_value = handler_value
            .funcall::<_, _, Value>("handle_server_stream", (request_value,))
            .map_err(|err| tonic::Status::internal(format!("Ruby server stream handler failed: {}", err)))?;

        // Convert Ruby enumerator to MessageStream
        ruby_enumerator_to_message_stream(enumerator_value)
    }

    /// Handle a client streaming request by calling Ruby handler
    fn handle_client_stream(&self, request: StreamingRequest) -> Result<GrpcResponseData, tonic::Status> {
        with_gvl(|| {
            let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
                self.handle_client_stream_inner(request)
            }));
            match result {
                Ok(res) => res,
                Err(_) => Err(tonic::Status::internal(
                    "Unexpected panic while executing Ruby client streaming gRPC handler",
                )),
            }
        })
    }

    fn handle_client_stream_inner(
        &self,
        request: StreamingRequest,
    ) -> Result<GrpcResponseData, tonic::Status> {
        let ruby = Ruby::get()
            .map_err(|_| tonic::Status::internal("Ruby VM unavailable while invoking client streaming handler"))?;

        // Wrap the message stream in a Ruby enumerator
        let stream_obj = ruby.obj_wrap(RubyGrpcMessageStream::new(request.message_stream));

        // Call Ruby handler's client_stream method
        let handler_value = self.inner.handler.get_inner_with(&ruby);
        let response_value = handler_value
            .funcall::<_, _, Value>("handle_client_stream", (stream_obj,))
            .map_err(|err| tonic::Status::internal(format!("Ruby client stream handler failed: {}", err)))?;

        // Convert Ruby response to GrpcResponseData
        let ruby_response = <&RubyGrpcResponse>::try_convert(response_value).map_err(|err| {
            tonic::Status::internal(format!(
                "Client stream handler must return Spikard::Grpc::Response, got error: {}",
                err
            ))
        })?;

        ruby_response
            .clone()
            .into_grpc_response()
            .map_err(|err| tonic::Status::internal(format!("Failed to build gRPC response: {}", err)))
    }

    /// Handle a bidirectional streaming request by calling Ruby handler
    fn handle_bidi_stream(&self, request: StreamingRequest) -> Result<MessageStream, tonic::Status> {
        with_gvl(|| {
            let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
                self.handle_bidi_stream_inner(request)
            }));
            match result {
                Ok(res) => res,
                Err(_) => Err(tonic::Status::internal(
                    "Unexpected panic while executing Ruby bidirectional streaming gRPC handler",
                )),
            }
        })
    }

    fn handle_bidi_stream_inner(&self, request: StreamingRequest) -> Result<MessageStream, tonic::Status> {
        let ruby = Ruby::get()
            .map_err(|_| tonic::Status::internal("Ruby VM unavailable while invoking bidi streaming handler"))?;

        // Wrap the input message stream in a Ruby enumerator
        let stream_obj = ruby.obj_wrap(RubyGrpcMessageStream::new(request.message_stream));

        // Call Ruby handler's bidi_stream method
        let handler_value = self.inner.handler.get_inner_with(&ruby);
        let enumerator_value = handler_value
            .funcall::<_, _, Value>("handle_bidi_stream", (stream_obj,))
            .map_err(|err| tonic::Status::internal(format!("Ruby bidi stream handler failed: {}", err)))?;

        // Convert Ruby enumerator to MessageStream
        ruby_enumerator_to_message_stream(enumerator_value)
    }
}

/// Convert a Ruby enumerator to a Rust MessageStream
///
/// This helper iterates over the Ruby enumerator and converts each yielded
/// response into a message byte sequence that can be sent to the client.
fn ruby_enumerator_to_message_stream(enumerator: Value) -> Result<MessageStream, tonic::Status> {
    // Check if the value responds to 'to_a' method (convert to array)
    if !enumerator
        .respond_to("to_a", true)
        .map_err(|_| tonic::Status::internal("Enumerator does not respond to 'to_a'"))?
    {
        return Err(tonic::Status::invalid_argument(
            "Handler must return an Enumerator or object that responds to 'to_a'",
        ));
    }

    // Convert the enumerator to an array
    let arr: Value = enumerator
        .funcall("to_a", ())
        .map_err(|err| tonic::Status::internal(format!("Failed to convert enumerator to array: {}", err)))?;

    let len: i64 = arr
        .funcall("length", ())
        .map_err(|err| tonic::Status::internal(format!("Failed to get array length: {}", err)))?;

    let mut messages_vec = Vec::new();

    for i in 0..len {
        let element: Value = arr
            .funcall("[]", (i,))
            .map_err(|err| tonic::Status::internal(format!("Failed to access array element {}: {}", i, err)))?;

        // Try to extract as RubyGrpcResponse
        if let Ok(response) = <&RubyGrpcResponse>::try_convert(element) {
            let payload = response.payload.borrow().clone();
            messages_vec.push(Ok(Bytes::from(payload)));
        } else if let Ok(bytes_str) = RString::try_convert(element) {
            let payload = unsafe { bytes_str.as_slice() }.to_vec();
            messages_vec.push(Ok(Bytes::from(payload)));
        } else {
            return Err(tonic::Status::internal(
                "Each yielded value must be a Spikard::Grpc::Response or binary string",
            ));
        }
    }

    // Convert Vec to MessageStream
    Ok(Box::pin(futures_util::stream::iter(messages_vec)))
}

impl GrpcHandler for RubyGrpcHandler {
    fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        let handler = self.clone();
        Box::pin(async move { handler.handle_request(request) })
    }

    fn call_server_stream(
        &self,
        request: GrpcRequestData,
    ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
        let handler = self.clone();
        Box::pin(async move { handler.handle_server_stream(request) })
    }

    fn call_client_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = Result<GrpcResponseData, tonic::Status>> + Send>> {
        let handler = self.clone();
        Box::pin(async move { handler.handle_client_stream(request) })
    }

    fn call_bidi_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
        let handler = self.clone();
        Box::pin(async move { handler.handle_bidi_stream(request) })
    }

    fn service_name(&self) -> &'static str {
        // We need to return a 'static str, but we have a String.
        // For now, we'll leak the string to get a 'static reference.
        // This is acceptable because service names are registered once at startup.
        Box::leak(self.inner.service_name.clone().into_boxed_str())
    }
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
    response_class.define_method("initialize", magnus::method!(RubyGrpcResponse::initialize, -1))?;
    response_class.define_method("metadata=", magnus::method!(RubyGrpcResponse::set_metadata, 1))?;
    response_class.define_method("metadata", magnus::method!(RubyGrpcResponse::get_metadata, 0))?;
    response_class.define_method("payload", magnus::method!(RubyGrpcResponse::payload, 0))?;

    // Define Spikard::Grpc::MessageStream class (internal use only)
    let _stream_class = grpc_module.define_class("MessageStream", ruby.class_object())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
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
        use spikard_bindings_shared::grpc_metadata::extract_metadata_to_hashmap;

        let mut metadata = MetadataMap::new();
        metadata.insert(
            "content-type",
            "application/grpc".parse().expect("Valid metadata value"),
        );
        metadata.insert(
            "authorization",
            "Bearer token123".parse().expect("Valid metadata value"),
        );

        let extracted = extract_metadata_to_hashmap(&metadata, false);
        assert_eq!(
            extracted.get("content-type").expect("content-type header"),
            "application/grpc"
        );
        assert_eq!(
            extracted.get("authorization").expect("authorization header"),
            "Bearer token123"
        );
    }

    #[test]
    fn test_grpc_response_conversion() {
        let response = RubyGrpcResponse {
            payload: RefCell::new(b"test response".to_vec()),
            metadata: RefCell::new(HashMap::new()),
        };

        let grpc_response = response.into_grpc_response();
        assert!(grpc_response.is_ok());
        let grpc_response = grpc_response.expect("Valid grpc response");
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
        let grpc_response = grpc_response.expect("Valid grpc response");
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

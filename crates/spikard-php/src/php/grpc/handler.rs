//! PHP gRPC handler implementation using ext-php-rs
//!
//! This module provides ext-php-rs bindings for gRPC request/response handling,
//! enabling PHP code to implement gRPC service handlers.

use bytes::Bytes;
use ext_php_rs::convert::IntoZval;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendCallable, Zval};
use futures_util::stream::StreamExt;
use spikard_bindings_shared::grpc_metadata::{extract_metadata_to_hashmap, hashmap_to_metadata};
use spikard_http::grpc::{GrpcHandler, GrpcHandlerResult, GrpcRequestData, GrpcResponseData};
use spikard_http::grpc::streaming::{MessageStream, StreamingRequest};
use std::collections::HashMap;
use std::future::Future;
use std::panic::AssertUnwindSafe;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;

/// PHP-side gRPC request
///
/// Represents a gRPC request that is passed to PHP handlers.
/// Contains the service name, method name, serialized protobuf payload,
/// and metadata (gRPC headers).
#[php_class]
#[derive(Debug, Clone)]
pub struct PhpGrpcRequest {
    /// Fully qualified service name (e.g., "mypackage.MyService")
    #[php(prop)]
    pub service_name: String,

    /// Method name (e.g., "GetUser")
    #[php(prop)]
    pub method_name: String,

    /// Serialized protobuf message as bytes
    #[php(prop)]
    pub payload: Vec<u8>,

    /// gRPC metadata (headers) as associative array
    #[php(prop)]
    pub metadata: HashMap<String, String>,
}

#[php_impl]
impl PhpGrpcRequest {
    /// Create a new gRPC request
    #[php(constructor)]
    pub fn __construct(
        service_name: String,
        method_name: String,
        payload: Vec<u8>,
        metadata: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            service_name,
            method_name,
            payload,
            metadata: metadata.unwrap_or_default(),
        }
    }

    /// Get metadata value by key
    pub fn get_metadata(&self, key: String) -> Option<String> {
        self.metadata.get(&key).cloned()
    }

    /// Get the payload size in bytes
    pub fn payload_size(&self) -> usize {
        self.payload.len()
    }

    /// String representation for debugging
    #[allow(non_snake_case)]
    pub fn __toString(&self) -> String {
        format!(
            "PhpGrpcRequest(service_name='{}', method_name='{}', payload_size={})",
            self.service_name,
            self.method_name,
            self.payload.len()
        )
    }
}

impl PhpGrpcRequest {
    /// Convert from Rust GrpcRequestData to PHP PhpGrpcRequest
    pub fn from_request_data(request: &GrpcRequestData) -> Self {
        let metadata = extract_metadata_to_hashmap(&request.metadata, true);

        Self {
            service_name: request.service_name.clone(),
            method_name: request.method_name.clone(),
            payload: request.payload.to_vec(),
            metadata,
        }
    }
}

/// PHP-side gRPC response
///
/// Represents a gRPC response returned from PHP handlers.
/// Contains the serialized protobuf payload and optional metadata.
#[php_class]
#[derive(Debug, Clone)]
pub struct PhpGrpcResponse {
    /// Serialized protobuf message as bytes
    #[php(prop)]
    pub payload: Vec<u8>,

    /// gRPC metadata (headers) to include in response
    #[php(prop)]
    pub metadata: HashMap<String, String>,
}

#[php_impl]
impl PhpGrpcResponse {
    /// Create a new gRPC response
    #[php(constructor)]
    pub fn __construct(payload: Vec<u8>, metadata: Option<HashMap<String, String>>) -> Self {
        Self {
            payload,
            metadata: metadata.unwrap_or_default(),
        }
    }

    /// Get the payload size in bytes
    pub fn payload_size(&self) -> usize {
        self.payload.len()
    }

    /// String representation for debugging
    #[allow(non_snake_case)]
    pub fn __toString(&self) -> String {
        format!("PhpGrpcResponse(payload_size={})", self.payload.len())
    }
}

impl PhpGrpcResponse {
    /// Convert to Rust GrpcResponseData
    pub fn to_response_data(&self) -> Result<GrpcResponseData, String> {
        let payload = Bytes::copy_from_slice(&self.payload);
        let metadata = hashmap_to_metadata(&self.metadata)?;

        Ok(GrpcResponseData { payload, metadata })
    }
}

/// PHP gRPC handler that bridges PHP code to Rust's GrpcHandler trait
///
/// This handler wraps a PHP callable and implements the GrpcHandler trait,
/// allowing it to be used in Spikard's gRPC runtime.
pub struct PhpGrpcHandler {
    /// Index into the handler registry
    handler_index: usize,

    /// Fully qualified service name this handler serves
    service_name: String,
}

// Thread-local registry for PHP gRPC handlers (since ZendCallable is not Send/Sync)
thread_local! {
    static PHP_GRPC_HANDLER_REGISTRY: std::cell::RefCell<Vec<ZendCallable<'static>>> = const {
        std::cell::RefCell::new(Vec::new())
    };
}

/// Clear the PHP gRPC handler registry
pub fn clear_grpc_handler_registry() {
    PHP_GRPC_HANDLER_REGISTRY.with(|registry| {
        registry.borrow_mut().clear();
    });
}

/// Leak the PHP gRPC handler registry for shutdown
pub fn leak_grpc_handler_registry() {
    PHP_GRPC_HANDLER_REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let handlers = std::mem::take(&mut *registry);
        std::mem::forget(handlers);
    });
}

impl PhpGrpcHandler {
    /// Register a PHP callable and return a handler instance
    ///
    /// # Parameters
    /// * `callable_zval` - The Zval containing the callable
    /// * `service_name` - Fully qualified service name
    pub fn register_from_zval(callable_zval: &ext_php_rs::types::Zval, service_name: String) -> Result<Self, String> {
        if !callable_zval.is_callable() {
            return Err(format!("Handler for service '{}' is not callable", service_name));
        }

        let idx = PHP_GRPC_HANDLER_REGISTRY.with(|registry| -> Result<usize, String> {
            let mut registry = registry.borrow_mut();
            let idx = registry.len();

            if idx > 10_000 {
                return Err("gRPC handler registry is full; refusing to register more handlers".to_string());
            }

            let zval_copy = callable_zval.shallow_clone();
            let callable = ZendCallable::new_owned(zval_copy).map_err(|e| {
                format!(
                    "Handler for service '{}' is not callable (callable reconstruction failed): {:?}",
                    service_name, e
                )
            })?;
            registry.push(callable);
            Ok(idx)
        })?;

        Ok(Self {
            handler_index: idx,
            service_name,
        })
    }
}

impl GrpcHandler for PhpGrpcHandler {
    fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        let handler_index = self.handler_index;
        let service_name = self.service_name.clone();

        Box::pin(async move {
            let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
                invoke_php_grpc_handler(handler_index, &service_name, request)
            }));

            match result {
                Ok(inner) => inner,
                Err(_) => Err(tonic::Status::internal(format!(
                    "Unexpected panic while executing PHP gRPC handler for service '{}'",
                    service_name
                ))),
            }
        })
    }

    fn call_server_stream(
        &self,
        request: GrpcRequestData,
    ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
        let handler_index = self.handler_index;
        let service_name = self.service_name.clone();

        Box::pin(async move {
            let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
                invoke_php_server_stream_handler(handler_index, &service_name, request)
            }));

            match result {
                Ok(inner) => inner,
                Err(_) => Err(tonic::Status::internal(format!(
                    "Unexpected panic while executing PHP server streaming gRPC handler for service '{}'",
                    service_name
                ))),
            }
        })
    }

    fn call_client_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = Result<GrpcResponseData, tonic::Status>> + Send>> {
        let handler_index = self.handler_index;
        let service_name = self.service_name.clone();

        Box::pin(async move {
            let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
                invoke_php_client_stream_handler(handler_index, &service_name, request)
            }));

            match result {
                Ok(inner) => inner,
                Err(_) => Err(tonic::Status::internal(format!(
                    "Unexpected panic while executing PHP client streaming gRPC handler for service '{}'",
                    service_name
                ))),
            }
        })
    }

    fn call_bidi_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
        let handler_index = self.handler_index;
        let service_name = self.service_name.clone();

        Box::pin(async move {
            let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
                invoke_php_bidi_stream_handler(handler_index, &service_name, request)
            }));

            match result {
                Ok(inner) => inner,
                Err(_) => Err(tonic::Status::internal(format!(
                    "Unexpected panic while executing PHP bidirectional streaming gRPC handler for service '{}'",
                    service_name
                ))),
            }
        })
    }

    fn service_name(&self) -> &'static str {
        // Leak the string to get a 'static reference
        // This is acceptable for service names which are typically static
        Box::leak(self.service_name.clone().into_boxed_str())
    }
}

/// Invoke the PHP unary gRPC handler
fn invoke_php_grpc_handler(
    handler_index: usize,
    service_name: &str,
    request_data: GrpcRequestData,
) -> GrpcHandlerResult {
    // Convert Rust request to PHP request
    let php_request = PhpGrpcRequest::from_request_data(&request_data);
    let request_zval = php_request.into_zval(false).map_err(|e| {
        tonic::Status::internal(format!(
            "Failed to convert request for PHP gRPC handler '{}': {:?}",
            service_name, e
        ))
    })?;

    // Call the PHP handler
    let response_zval =
        PHP_GRPC_HANDLER_REGISTRY.with(|registry| -> Result<ext_php_rs::types::Zval, tonic::Status> {
            let registry = registry.borrow();
            let Some(callable) = registry.get(handler_index) else {
                return Err(tonic::Status::internal(format!(
                    "PHP gRPC handler not found for service '{}': index {}",
                    service_name, handler_index
                )));
            };

            callable
                .try_call(vec![&request_zval])
                .map_err(|e| tonic::Status::internal(format!("PHP gRPC handler '{}' failed: {:?}", service_name, e)))
        })?;

    // Convert PHP response back to Rust response
    interpret_php_grpc_response(&response_zval, service_name)
}

/// Invoke the PHP server streaming gRPC handler
fn invoke_php_server_stream_handler(
    handler_index: usize,
    service_name: &str,
    request_data: GrpcRequestData,
) -> Result<MessageStream, tonic::Status> {
    // Convert Rust request to PHP request
    let php_request = PhpGrpcRequest::from_request_data(&request_data);
    let request_zval = php_request.into_zval(false).map_err(|e| {
        tonic::Status::internal(format!(
            "Failed to convert request for PHP server stream handler '{}': {:?}",
            service_name, e
        ))
    })?;

    // Call the PHP handler and expect a Generator
    let generator_zval =
        PHP_GRPC_HANDLER_REGISTRY.with(|registry| -> Result<ext_php_rs::types::Zval, tonic::Status> {
            let registry = registry.borrow();
            let Some(callable) = registry.get(handler_index) else {
                return Err(tonic::Status::internal(format!(
                    "PHP gRPC handler not found for service '{}': index {}",
                    service_name, handler_index
                )));
            };

            callable.try_call(vec![&request_zval]).map_err(|e| {
                tonic::Status::internal(format!(
                    "PHP server stream handler '{}' failed: {:?}",
                    service_name, e
                ))
            })
        })?;

    // Convert PHP Generator/Traversable to MessageStream
    php_generator_to_message_stream(&generator_zval, service_name)
}

/// Invoke the PHP client streaming gRPC handler
fn invoke_php_client_stream_handler(
    handler_index: usize,
    service_name: &str,
    request: StreamingRequest,
) -> Result<GrpcResponseData, tonic::Status> {
    // Collect the message stream into a Vec of messages
    // Since we can't iterate async generators from PHP, we pre-collect the stream
    let messages = std::thread::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            collect_message_stream_to_vec(request.message_stream).await
        })
    })?;

    // Convert Vec of messages to PHP array of PhpGrpcRequest objects
    let php_array = messages_to_php_request_array(&messages)?;

    // Call the PHP handler with the array of requests
    let response_zval =
        PHP_GRPC_HANDLER_REGISTRY.with(|registry| -> Result<ext_php_rs::types::Zval, tonic::Status> {
            let registry = registry.borrow();
            let Some(callable) = registry.get(handler_index) else {
                return Err(tonic::Status::internal(format!(
                    "PHP gRPC handler not found for service '{}': index {}",
                    service_name, handler_index
                )));
            };

            callable.try_call(vec![&php_array]).map_err(|e| {
                tonic::Status::internal(format!(
                    "PHP client stream handler '{}' failed: {:?}",
                    service_name, e
                ))
            })
        })?;

    // Convert PHP response back to Rust response
    interpret_php_grpc_response(&response_zval, service_name)
}

/// Invoke the PHP bidirectional streaming gRPC handler
fn invoke_php_bidi_stream_handler(
    handler_index: usize,
    service_name: &str,
    request: StreamingRequest,
) -> Result<MessageStream, tonic::Status> {
    // Collect the message stream into a Vec of messages
    let messages = std::thread::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            collect_message_stream_to_vec(request.message_stream).await
        })
    })?;

    // Convert Vec of messages to PHP array of PhpGrpcRequest objects
    let php_array = messages_to_php_request_array(&messages)?;

    // Call the PHP handler with the array of requests and expect a Generator
    let generator_zval =
        PHP_GRPC_HANDLER_REGISTRY.with(|registry| -> Result<ext_php_rs::types::Zval, tonic::Status> {
            let registry = registry.borrow();
            let Some(callable) = registry.get(handler_index) else {
                return Err(tonic::Status::internal(format!(
                    "PHP gRPC handler not found for service '{}': index {}",
                    service_name, handler_index
                )));
            };

            callable.try_call(vec![&php_array]).map_err(|e| {
                tonic::Status::internal(format!(
                    "PHP bidi stream handler '{}' failed: {:?}",
                    service_name, e
                ))
            })
        })?;

    // Convert PHP Generator/Traversable to MessageStream
    php_generator_to_message_stream(&generator_zval, service_name)
}

/// Interpret a PHP return value as a gRPC response
fn interpret_php_grpc_response(response_zval: &ext_php_rs::types::Zval, service_name: &str) -> GrpcHandlerResult {
    // Check if the response is a PhpGrpcResponse object
    if let Some(obj) = response_zval.object() {
        // Try to get the class name to verify it's a PhpGrpcResponse
        if let Ok(class_name) = obj.get_class_name()
            && (class_name.contains("PhpGrpcResponse") || class_name.contains("GrpcResponse"))
        {
            // Extract the object's properties
            if let Ok(payload_zval) = obj.get_property::<&Zval>("payload") {
                let payload = if let Some(arr) = payload_zval.array() {
                    // Convert PHP array to Vec<u8>
                    let mut bytes = Vec::new();
                    for (_, val) in arr.iter() {
                        if let Some(byte) = val.long() {
                            bytes.push(byte as u8);
                        }
                    }
                    bytes
                } else if let Some(s) = payload_zval.string() {
                    // Handle as binary string
                    s.as_bytes().to_vec()
                } else {
                    return Err(tonic::Status::internal(format!(
                        "PHP gRPC handler '{}' returned invalid payload type",
                        service_name
                    )));
                };

                // Extract metadata
                let metadata = if let Ok(metadata_zval) = obj.get_property::<&Zval>("metadata") {
                    if let Some(arr) = metadata_zval.array() {
                        let mut meta = HashMap::new();
                        for (key, val) in arr.iter() {
                            let key_str = match key {
                                ext_php_rs::types::ArrayKey::String(s) => s.to_string(),
                                ext_php_rs::types::ArrayKey::Str(s) => s.to_string(),
                                ext_php_rs::types::ArrayKey::Long(l) => l.to_string(),
                            };
                            if let Some(val_str) = val.string() {
                                meta.insert(key_str, val_str.to_string());
                            }
                        }
                        meta
                    } else {
                        HashMap::new()
                    }
                } else {
                    HashMap::new()
                };

                let php_response = PhpGrpcResponse { payload, metadata };
                return php_response.to_response_data().map_err(|e| {
                    tonic::Status::internal(format!(
                        "Failed to convert PHP gRPC response from '{}': {}",
                        service_name, e
                    ))
                });
            }
        }
    }

    Err(tonic::Status::internal(format!(
        "PHP gRPC handler '{}' did not return a valid PhpGrpcResponse object",
        service_name
    )))
}

/// Convert a PHP Generator or Traversable to a Rust MessageStream
///
/// This helper iterates over the PHP Generator and converts each yielded
/// response into a message byte sequence that can be sent to the client.
/// Supports both PHP Generators and objects implementing Iterator interface.
fn php_generator_to_message_stream(
    generator_zval: &ext_php_rs::types::Zval,
    service_name: &str,
) -> Result<MessageStream, tonic::Status> {
    if !generator_zval.is_object() {
        return Err(tonic::Status::invalid_argument(
            "Handler must return a Generator or Iterator for server/bidi streaming",
        ));
    }

    // Collect all messages from the PHP Generator
    let messages_vec = PHP_GRPC_HANDLER_REGISTRY.with(|_registry| {
        collect_php_generator_messages(generator_zval, service_name)
    })?;

    if messages_vec.is_empty() {
        return Err(tonic::Status::internal(format!(
            "PHP server stream handler '{}' returned empty generator",
            service_name
        )));
    }

    // Convert Vec to MessageStream
    Ok(Box::pin(futures_util::stream::iter(messages_vec)))
}

/// Collect messages from a PHP Generator/Traversable
fn collect_php_generator_messages(
    generator_zval: &ext_php_rs::types::Zval,
    service_name: &str,
) -> Result<Vec<Result<Bytes, tonic::Status>>, tonic::Status> {
    let mut messages_vec = Vec::new();

    if let Some(obj) = generator_zval.object() {
        // Try to iterate using standard Iterator protocol
        let mut index = 0i64;
        let mut message_count = 0i64;

        // Attempt to call rewind() if the method exists
        let _ = obj.call_method::<Zval>("rewind", vec![]);

        loop {
            // Check if there are more values using valid()
            let valid_zval = match obj.call_method::<Zval>("valid", vec![]) {
                Ok(z) => z,
                Err(_) => {
                    // If valid() doesn't exist, try iteration limit
                    if index > 100_000 {
                        break;
                    }
                    // Try getting current anyway
                    match obj.call_method::<Zval>("current", vec![]) {
                        Ok(_) => {},
                        Err(_) => break,
                    }
                    match obj.call_method::<Zval>("next", vec![]) {
                        Ok(_) => {},
                        Err(_) => break,
                    }
                    index += 1;
                    continue;
                }
            };

            // Check if valid_zval indicates there are more items
            let is_valid = valid_zval.bool().unwrap_or(false);
            if !is_valid {
                break;
            }

            // Get the current value
            match obj.call_method::<Zval>("current", vec![]) {
                Ok(current_val) => {
                    if let Some(current_obj) = current_val.object() {
                        // Try to extract as PhpGrpcResponse
                        if let Ok(class_name) = current_obj.get_class_name() {
                            if class_name.contains("PhpGrpcResponse") || class_name.contains("GrpcResponse") {
                                // Extract payload from PhpGrpcResponse object
                                if let Ok(payload_zval) = current_obj.get_property::<&Zval>("payload") {
                                    if let Some(s) = payload_zval.string() {
                                        messages_vec.push(Ok(Bytes::from(s.as_bytes().to_vec())));
                                        message_count += 1;
                                    }
                                }
                            }
                        }
                    } else if let Some(s) = current_val.string() {
                        // Handle raw binary string (fallback)
                        messages_vec.push(Ok(Bytes::from(s.as_bytes().to_vec())));
                        message_count += 1;
                    }
                }
                Err(_) => break,
            }

            // Move to next item
            if obj.call_method::<Zval>("next", vec![]).is_err() {
                break;
            }

            index += 1;
            if index > 100_000 {
                return Err(tonic::Status::internal(format!(
                    "PHP server stream handler '{}' yielded too many messages (limit: 100,000)",
                    service_name
                )));
            }
        }

        if message_count == 0 {
            return Err(tonic::Status::internal(format!(
                "PHP server stream handler '{}' returned empty generator",
                service_name
            )));
        }
    } else {
        return Err(tonic::Status::invalid_argument(
            "Handler must return a Generator or Iterator for server/bidi streaming",
        ));
    }

    Ok(messages_vec)
}

/// Collect a MessageStream into a Vec for PHP consumption
async fn collect_message_stream_to_vec(mut stream: MessageStream) -> Result<Vec<Bytes>, tonic::Status> {
    let mut messages = Vec::new();

    while let Some(result) = stream.next().await {
        match result {
            Ok(bytes) => messages.push(bytes),
            Err(e) => return Err(e),
        }
    }

    Ok(messages)
}

/// Convert a Vec of message bytes to a PHP array of PhpGrpcRequest objects
fn messages_to_php_request_array(messages: &[Bytes]) -> Result<Zval, tonic::Status> {
    let mut php_requests = Vec::new();

    for message_bytes in messages {
        // Create a PhpGrpcRequest from each message payload
        let php_request = PhpGrpcRequest {
            service_name: String::new(),
            method_name: String::new(),
            payload: message_bytes.to_vec(),
            metadata: HashMap::new(),
        };

        php_requests.push(php_request);
    }

    // Convert to Zval array
    let zval = php_requests.into_zval(false).map_err(|e| {
        tonic::Status::internal(format!("Failed to convert message stream to PHP array: {:?}", e))
    })?;

    Ok(zval)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use tonic::metadata::MetadataMap;

    #[test]
    fn test_php_grpc_request_creation() {
        let request = PhpGrpcRequest::__construct(
            "test.TestService".to_string(),
            "TestMethod".to_string(),
            vec![1, 2, 3, 4],
            None,
        );

        assert_eq!(request.service_name, "test.TestService");
        assert_eq!(request.method_name, "TestMethod");
        assert_eq!(request.payload, vec![1, 2, 3, 4]);
        assert!(request.metadata.is_empty());
    }

    #[test]
    fn test_php_grpc_request_with_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("authorization".to_string(), "Bearer token".to_string());

        let request = PhpGrpcRequest::__construct(
            "test.TestService".to_string(),
            "TestMethod".to_string(),
            vec![],
            Some(metadata),
        );

        assert_eq!(
            request.get_metadata("authorization".to_string()),
            Some("Bearer token".to_string())
        );
    }

    #[test]
    fn test_php_grpc_request_from_request_data() {
        use tonic::metadata::Ascii;
        use tonic::metadata::MetadataKey;
        let mut metadata_map = MetadataMap::new();
        let key: MetadataKey<Ascii> = "authorization".parse().expect("Valid metadata key");
        metadata_map.insert(key, "Bearer token".parse().expect("Valid metadata value"));

        let request_data = GrpcRequestData {
            service_name: "test.Service".to_string(),
            method_name: "Method".to_string(),
            payload: Bytes::from(vec![1, 2, 3]),
            metadata: metadata_map,
        };

        let php_request = PhpGrpcRequest::from_request_data(&request_data);

        assert_eq!(php_request.service_name, "test.Service");
        assert_eq!(php_request.method_name, "Method");
        assert_eq!(php_request.payload, vec![1, 2, 3]);
        assert_eq!(
            php_request.get_metadata("authorization".to_string()),
            Some("Bearer token".to_string())
        );
    }

    #[test]
    fn test_php_grpc_response_creation() {
        let response = PhpGrpcResponse::__construct(vec![5, 6, 7, 8], None);

        assert_eq!(response.payload, vec![5, 6, 7, 8]);
        assert!(response.metadata.is_empty());
    }

    #[test]
    fn test_php_grpc_response_to_response_data() {
        let mut metadata = HashMap::new();
        metadata.insert("content-type".to_string(), "application/grpc".to_string());

        let php_response = PhpGrpcResponse::__construct(vec![1, 2, 3], Some(metadata));

        let response_data = php_response
            .to_response_data()
            .expect("Failed to convert response data");
        assert_eq!(response_data.payload, Bytes::from(vec![1, 2, 3]));
        assert!(response_data.metadata.contains_key("content-type"));
    }

    #[test]
    fn test_php_grpc_request_payload_size() {
        let request = PhpGrpcRequest::__construct(
            "test.Service".to_string(),
            "Method".to_string(),
            vec![1, 2, 3, 4, 5],
            None,
        );

        assert_eq!(request.payload_size(), 5);
    }

    #[test]
    fn test_php_grpc_response_payload_size() {
        let response = PhpGrpcResponse::__construct(vec![1, 2, 3], None);
        assert_eq!(response.payload_size(), 3);
    }

    #[test]
    fn test_php_grpc_request_to_string() {
        let request =
            PhpGrpcRequest::__construct("test.Service".to_string(), "Method".to_string(), vec![1, 2, 3], None);

        let s = request.__toString();
        assert!(s.contains("test.Service"));
        assert!(s.contains("Method"));
        assert!(s.contains("payload_size=3"));
    }

    #[test]
    fn test_php_grpc_response_to_string() {
        let response = PhpGrpcResponse::__construct(vec![1, 2, 3, 4, 5], None);

        let s = response.__toString();
        assert!(s.contains("payload_size=5"));
    }
}

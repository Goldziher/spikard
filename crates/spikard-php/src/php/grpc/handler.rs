//! PHP gRPC handler implementation using ext-php-rs
//!
//! This module provides ext-php-rs bindings for gRPC request/response handling,
//! enabling PHP code to implement gRPC service handlers.

use bytes::Bytes;
use ext_php_rs::convert::IntoZval;
use ext_php_rs::prelude::*;
use ext_php_rs::types::ZendCallable;
use spikard_bindings_shared::grpc_metadata::{extract_metadata_to_hashmap, hashmap_to_metadata};
use spikard_http::grpc::{GrpcHandler, GrpcHandlerResult, GrpcRequestData, GrpcResponseData};
use std::collections::HashMap;
use std::future::Future;
use std::panic::AssertUnwindSafe;
use std::pin::Pin;

/// PHP-side gRPC request
///
/// Represents a gRPC request that is passed to PHP handlers.
/// Contains the service name, method name, serialized protobuf payload,
/// and metadata (gRPC headers).
#[php_class]
#[derive(Debug, Clone)]
pub struct PhpGrpcRequest {
    /// Fully qualified service name (e.g., "mypackage.MyService")
    #[prop(flags = ext_php_rs::flags::PropertyFlags::Public)]
    pub service_name: String,

    /// Method name (e.g., "GetUser")
    #[prop(flags = ext_php_rs::flags::PropertyFlags::Public)]
    pub method_name: String,

    /// Serialized protobuf message as bytes
    #[prop(flags = ext_php_rs::flags::PropertyFlags::Public)]
    pub payload: Vec<u8>,

    /// gRPC metadata (headers) as associative array
    #[prop(flags = ext_php_rs::flags::PropertyFlags::Public)]
    pub metadata: HashMap<String, String>,
}

#[php_impl]
impl PhpGrpcRequest {
    /// Create a new gRPC request
    #[constructor]
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
    #[prop(flags = ext_php_rs::flags::PropertyFlags::Public)]
    pub payload: Vec<u8>,

    /// gRPC metadata (headers) to include in response
    #[prop(flags = ext_php_rs::flags::PropertyFlags::Public)]
    pub metadata: HashMap<String, String>,
}

#[php_impl]
impl PhpGrpcResponse {
    /// Create a new gRPC response
    #[constructor]
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
    pub fn register_from_zval(
        callable_zval: &ext_php_rs::types::Zval,
        service_name: String,
    ) -> Result<Self, String> {
        if !callable_zval.is_callable() {
            return Err(format!(
                "Handler for service '{}' is not callable",
                service_name
            ));
        }

        let idx = PHP_GRPC_HANDLER_REGISTRY.with(|registry| -> Result<usize, String> {
            let mut registry = registry.borrow_mut();
            let idx = registry.len();

            if idx > 10_000 {
                return Err(
                    "gRPC handler registry is full; refusing to register more handlers"
                        .to_string(),
                );
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
    fn call(
        &self,
        request: GrpcRequestData,
    ) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
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

    fn service_name(&self) -> &'static str {
        // Leak the string to get a 'static reference
        // This is acceptable for service names which are typically static
        Box::leak(self.service_name.clone().into_boxed_str())
    }
}

/// Invoke the PHP gRPC handler
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
    let response_zval = PHP_GRPC_HANDLER_REGISTRY.with(
        |registry| -> Result<ext_php_rs::types::Zval, tonic::Status> {
            let registry = registry.borrow();
            let Some(callable) = registry.get(handler_index) else {
                return Err(tonic::Status::internal(format!(
                    "PHP gRPC handler not found for service '{}': index {}",
                    service_name, handler_index
                )));
            };

            callable.try_call(vec![&request_zval]).map_err(|e| {
                tonic::Status::internal(format!(
                    "PHP gRPC handler '{}' failed: {:?}",
                    service_name, e
                ))
            })
        },
    )?;

    // Convert PHP response back to Rust response
    interpret_php_grpc_response(&response_zval, service_name)
}

/// Interpret a PHP return value as a gRPC response
fn interpret_php_grpc_response(
    response_zval: &ext_php_rs::types::Zval,
    service_name: &str,
) -> GrpcHandlerResult {
    // Check if the response is a PhpGrpcResponse object
    if let Some(obj) = response_zval.object() {
        // Try to get the class name to verify it's a PhpGrpcResponse
        if let Ok(class_name) = obj.get_class_name() {
            if class_name.contains("PhpGrpcResponse") || class_name.contains("GrpcResponse") {
                // Extract the object's properties
                if let Ok(Some(payload_zval)) = obj.get_property("payload") {
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
                    let metadata = if let Ok(Some(metadata_zval)) = obj.get_property("metadata") {
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
    }

    Err(tonic::Status::internal(format!(
        "PHP gRPC handler '{}' did not return a valid PhpGrpcResponse object",
        service_name
    )))
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
        let mut metadata_map = MetadataMap::new();
        metadata_map.insert(
            "authorization".parse().unwrap(),
            "Bearer token".parse().unwrap(),
        );

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

        let response_data = php_response.to_response_data().unwrap();
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
        let request = PhpGrpcRequest::__construct(
            "test.Service".to_string(),
            "Method".to_string(),
            vec![1, 2, 3],
            None,
        );

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

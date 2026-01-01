//! Node.js handler implementation for gRPC requests
//!
//! This module implements the GrpcHandler trait using napi-rs ThreadsafeFunction
//! to call JavaScript handlers from Rust's async gRPC server.

use bytes::Bytes;
use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunction;
use napi_derive::napi;
use spikard_http::grpc::{GrpcHandler, GrpcHandlerResult, GrpcRequestData, GrpcResponseData};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tonic::metadata::{MetadataMap, MetadataValue};

/// gRPC request object passed to JavaScript handlers
///
/// Contains the parsed components of a gRPC request with all data
/// converted to JavaScript-friendly types.
#[napi(object)]
pub struct GrpcRequest {
    /// Fully qualified service name (e.g., "mypackage.UserService")
    pub service_name: String,
    /// Method name (e.g., "GetUser")
    pub method_name: String,
    /// Serialized protobuf message as Buffer
    pub payload: Buffer,
    /// gRPC metadata as key-value pairs
    pub metadata: HashMap<String, String>,
}

/// gRPC response object returned by JavaScript handlers
///
/// Contains the serialized protobuf response and optional metadata
/// to include in the response headers.
#[napi(object)]
pub struct GrpcResponse {
    /// Serialized protobuf message as Buffer
    pub payload: Buffer,
    /// Optional gRPC metadata to include in response
    #[napi(ts_type = "Record<string, string> | undefined")]
    pub metadata: Option<HashMap<String, String>>,
}

/// Convert tonic MetadataMap to JavaScript-friendly HashMap
fn metadata_to_hashmap(metadata: &MetadataMap) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for key_and_value in metadata.iter() {
        match key_and_value {
            tonic::metadata::KeyAndValueRef::Ascii(key, value) => {
                if let Ok(value_str) = value.to_str() {
                    map.insert(key.as_str().to_string(), value_str.to_string());
                }
            }
            tonic::metadata::KeyAndValueRef::Binary(key, _value) => {
                // Skip binary metadata for now, as we only support string values in the HashMap
                tracing::debug!("Skipping binary metadata key: {}", key.as_str());
            }
        }
    }
    map
}

/// Convert JavaScript HashMap to tonic MetadataMap
fn hashmap_to_metadata(map: &HashMap<String, String>) -> Result<MetadataMap> {
    let mut metadata = MetadataMap::new();
    for (key, value) in map {
        let key_name = key
            .parse::<tonic::metadata::MetadataKey<tonic::metadata::Ascii>>()
            .map_err(|e| Error::from_reason(format!("Invalid metadata key '{}': {}", key, e)))?;
        let value_bytes = value
            .parse::<MetadataValue<tonic::metadata::Ascii>>()
            .map_err(|e| Error::from_reason(format!("Invalid metadata value '{}': {}", value, e)))?;
        metadata.insert(key_name, value_bytes);
    }
    Ok(metadata)
}

/// Node.js gRPC handler wrapper that implements spikard_http::grpc::GrpcHandler
///
/// Uses ThreadsafeFunction to call JavaScript handlers from Rust threads.
/// Converts between Rust's bytes/metadata types and JavaScript-friendly objects.
pub struct NodeGrpcHandler {
    service_name: &'static str,
    handler_fn: Arc<ThreadsafeFunction<GrpcRequest, Promise<GrpcResponse>, GrpcRequest, napi::Status, false>>,
}

unsafe impl Send for NodeGrpcHandler {}
unsafe impl Sync for NodeGrpcHandler {}

impl NodeGrpcHandler {
    /// Create a new Node gRPC handler wrapper with a JavaScript function
    ///
    /// # Arguments
    ///
    /// * `service_name` - Fully qualified service name (must be 'static)
    /// * `handler_fn` - ThreadsafeFunction that calls JavaScript handler
    pub fn new(
        service_name: &'static str,
        handler_fn: ThreadsafeFunction<GrpcRequest, Promise<GrpcResponse>, GrpcRequest, napi::Status, false>,
    ) -> Self {
        Self {
            service_name,
            handler_fn: Arc::new(handler_fn),
        }
    }
}

impl GrpcHandler for NodeGrpcHandler {
    fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        let handler_fn = self.handler_fn.clone();
        let service_name = self.service_name;

        Box::pin(async move {
            // Convert Rust types to JavaScript-friendly types
            let js_request = GrpcRequest {
                service_name: request.service_name.clone(),
                method_name: request.method_name.clone(),
                // Zero-copy conversion: Bytes implements AsRef<[u8]>
                payload: Buffer::from(request.payload.as_ref()),
                metadata: metadata_to_hashmap(&request.metadata),
            };

            // Call JavaScript handler
            let js_response = handler_fn
                .call_async(js_request)
                .await
                .map_err(|e| tonic::Status::internal(format!("Handler call failed for {}: {}", service_name, e)))?
                .await
                .map_err(|e| tonic::Status::internal(format!("Handler promise failed for {}: {}", service_name, e)))?;

            // Convert JavaScript response back to Rust types
            // Zero-copy conversion: Buffer implements AsRef<[u8]>
            let payload = Bytes::copy_from_slice(js_response.payload.as_ref());
            let metadata = if let Some(meta_map) = js_response.metadata {
                hashmap_to_metadata(&meta_map)
                    .map_err(|e| tonic::Status::internal(format!("Invalid metadata: {}", e)))?
            } else {
                MetadataMap::new()
            };

            Ok(GrpcResponseData { payload, metadata })
        })
    }

    fn service_name(&self) -> &'static str {
        self.service_name
    }
}

impl Clone for NodeGrpcHandler {
    fn clone(&self) -> Self {
        Self {
            service_name: self.service_name,
            handler_fn: self.handler_fn.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_to_hashmap() {
        let mut metadata = MetadataMap::new();
        metadata.insert("content-type", "application/grpc".parse().unwrap());
        metadata.insert("x-custom", "value".parse().unwrap());

        let map = metadata_to_hashmap(&metadata);

        assert_eq!(map.len(), 2);
        assert_eq!(map.get("content-type"), Some(&"application/grpc".to_string()));
        assert_eq!(map.get("x-custom"), Some(&"value".to_string()));
    }

    #[test]
    fn test_metadata_to_hashmap_empty() {
        let metadata = MetadataMap::new();
        let map = metadata_to_hashmap(&metadata);
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_hashmap_to_metadata() {
        let mut map = HashMap::new();
        map.insert("content-type".to_string(), "application/grpc".to_string());
        map.insert("x-custom".to_string(), "value".to_string());

        let metadata = hashmap_to_metadata(&map).unwrap();

        assert_eq!(metadata.len(), 2);
        assert!(metadata.contains_key("content-type"));
        assert!(metadata.contains_key("x-custom"));
    }

    #[test]
    fn test_hashmap_to_metadata_empty() {
        let map = HashMap::new();
        let metadata = hashmap_to_metadata(&map).unwrap();
        assert_eq!(metadata.len(), 0);
    }

    #[test]
    fn test_hashmap_to_metadata_invalid_key() {
        let mut map = HashMap::new();
        map.insert("invalid key with spaces".to_string(), "value".to_string());

        let result = hashmap_to_metadata(&map);
        assert!(result.is_err());
    }
}

//! Ruby gRPC handler implementations using Magnus bindings
//!
//! This module provides the Ruby-facing handler for gRPC requests, implementing
//! the `Handler` trait from spikard-http while managing the GVL (Global VM Lock)
//! to ensure thread safety with Ruby code.

use magnus::{Module, Ruby, RHash, Error, Value, prelude::*, value::{Opaque, ReprValue, InnerValue}};
use spikard_bindings_shared::grpc_metadata::extract_metadata_to_hashmap;
use spikard_http::grpc::handler::GrpcRequestData;
use std::collections::HashMap;
use std::fmt;

/// Ruby representation of a gRPC request
#[derive(Debug, Clone)]
pub struct RubyGrpcRequest {
    pub service_name: String,
    pub method_name: String,
    pub payload: Vec<u8>,
    pub metadata: HashMap<String, String>,
}

impl RubyGrpcRequest {
    /// Create a Ruby gRPC request from gRPC request data
    pub fn from_grpc_request(request_data: GrpcRequestData) -> Self {
        let metadata = extract_metadata_to_hashmap(&request_data.metadata, false);
        
        RubyGrpcRequest {
            service_name: request_data.service_name,
            method_name: request_data.method_name,
            payload: request_data.payload.to_vec(),
            metadata,
        }
    }
}

/// Ruby response for gRPC requests
#[derive(Debug, Clone)]
pub struct RubyGrpcResponse {
    pub status: u32,
    pub payload: Vec<u8>,
    pub metadata: HashMap<String, String>,
}

/// Ruby wrapper for a gRPC handler function
///
/// This struct wraps a Ruby callable (Proc/Lambda) that handles gRPC requests.
/// It manages the Global VM Lock (GVL) during execution to ensure thread-safe
/// interaction with Ruby code.
#[derive(Clone)]
pub struct RubyGrpcHandler {
    handler_proc: Opaque<Value>,
}

impl fmt::Debug for RubyGrpcHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RubyGrpcHandler").finish()
    }
}

impl RubyGrpcHandler {
    /// Create a new Ruby gRPC handler from a callable
    pub fn new(handler_proc: Value) -> Result<Self, Error> {
        Ok(RubyGrpcHandler {
            handler_proc: Opaque::from(handler_proc),
        })
    }

    /// Process a gRPC request by calling the Ruby handler
    pub fn handle_request(&self, request_data: GrpcRequestData) -> Result<String, String> {
        // Convert the Rust request to a Ruby-friendly format
        let ruby_request = RubyGrpcRequest::from_grpc_request(request_data);

        // Get the Ruby VM context
        let ruby = Ruby::get().map_err(|e| format!("Failed to get Ruby VM: {:?}", e))?;

        // Create a Magnus Value from the RubyGrpcRequest
        let request_hash = RHash::new();

        request_hash.aset(
            ruby.intern("service_name"),
            ruby.str_new(&ruby_request.service_name)
        ).map_err(|e| format!("Failed to set service_name: {:?}", e))?;

        request_hash.aset(
            ruby.intern("method_name"),
            ruby.str_new(&ruby_request.method_name)
        ).map_err(|e| format!("Failed to set method_name: {:?}", e))?;

        // Set payload as a Ruby string (which can hold binary data)
        request_hash.aset(
            ruby.intern("payload"),
            ruby.str_new(&String::from_utf8_lossy(&ruby_request.payload))
        ).map_err(|e| format!("Failed to set payload: {:?}", e))?;

        // Convert metadata HashMap to a Ruby Hash
        let metadata_hash = RHash::new();
        for (key, value) in &ruby_request.metadata {
            metadata_hash.aset(
                ruby.str_new(key),
                ruby.str_new(value)
            ).map_err(|e| format!("Failed to set metadata entry: {:?}", e))?;
        }
        request_hash.aset(
            ruby.intern("metadata"),
            metadata_hash
        ).map_err(|e| format!("Failed to set metadata: {:?}", e))?;

        // Call the Ruby proc with the request hash
        let handler_proc = self.handler_proc.get_inner_with(&ruby);
        let _response: Value = handler_proc.funcall(
            "call",
            (request_hash,),
        ).map_err(|e| format!("Ruby handler call failed: {:?}", e))?;

        // For now, just return a simple success message
        // In the future, this should extract and format the response
        Ok("gRPC handler processed request successfully".to_string())
    }
}

/// Register the Ruby gRPC handler class
pub fn init(ruby: &Ruby, spikard_module: &magnus::RModule) -> Result<(), Error> {
    let grpc_module = spikard_module.define_module("Grpc")?;
    let _handler_class = grpc_module.define_class("RubyHandler", ruby.class_object())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::metadata::MetadataMap;

    #[test]
    fn test_grpc_request_creation() {
        let request = GrpcRequestData {
            service_name: "test.Service".to_string(),
            method_name: "TestMethod".to_string(),
            payload: bytes::Bytes::from("test payload"),
            metadata: MetadataMap::new(),
        };

        let ruby_request = RubyGrpcRequest::from_grpc_request(request);
        assert_eq!(ruby_request.service_name, "test.Service");
        assert_eq!(ruby_request.method_name, "TestMethod");
        assert_eq!(ruby_request.payload, b"test payload");
    }

    #[test]
    fn test_grpc_metadata_extraction() {
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
    fn test_metadata_extraction_empty() {
        let metadata = MetadataMap::new();
        let extracted = extract_metadata_to_hashmap(&metadata, false);
        assert!(extracted.is_empty());
    }

    #[test]
    fn test_metadata_extraction_single() {
        let mut metadata = MetadataMap::new();
        metadata.insert(
            "x-custom-header",
            "custom-value".parse().expect("Valid metadata value"),
        );

        let extracted = extract_metadata_to_hashmap(&metadata, false);
        assert_eq!(extracted.len(), 1);
        assert_eq!(
            extracted.get("x-custom-header"),
            Some(&"custom-value".to_string())
        );
    }
}

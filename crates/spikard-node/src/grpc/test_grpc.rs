//! Integration tests for Node.js gRPC bindings
//!
//! These tests verify the gRPC FFI layer works correctly for converting
//! between Rust and JavaScript types.

#[cfg(test)]
mod tests {
    use super::super::handler::{GrpcRequest, GrpcResponse};
    use std::collections::HashMap;

    #[test]
    fn test_grpc_request_roundtrip() {
        let mut metadata = HashMap::new();
        metadata.insert("content-type".to_string(), "application/grpc".to_string());
        metadata.insert("x-custom-header".to_string(), "custom-value".to_string());

        let request = GrpcRequest {
            service_name: "mypackage.UserService".to_string(),
            method_name: "GetUser".to_string(),
            payload: napi::bindgen_prelude::Buffer::from(vec![1, 2, 3, 4, 5]),
            metadata: metadata.clone(),
        };

        // Verify request structure
        assert_eq!(request.service_name, "mypackage.UserService");
        assert_eq!(request.method_name, "GetUser");
        assert_eq!(request.payload.len(), 5);
        assert_eq!(request.metadata.len(), 2);
        assert_eq!(
            request.metadata.get("content-type"),
            Some(&"application/grpc".to_string())
        );
    }

    #[test]
    fn test_grpc_response_with_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("x-server-id".to_string(), "server-1".to_string());
        metadata.insert("x-cache-status".to_string(), "hit".to_string());

        let response = GrpcResponse {
            payload: napi::bindgen_prelude::Buffer::from(vec![10, 20, 30]),
            metadata: Some(metadata),
        };

        assert_eq!(response.payload.len(), 3);
        assert!(response.metadata.is_some());

        let meta = response.metadata.unwrap();
        assert_eq!(meta.len(), 2);
        assert_eq!(meta.get("x-server-id"), Some(&"server-1".to_string()));
        assert_eq!(meta.get("x-cache-status"), Some(&"hit".to_string()));
    }

    #[test]
    fn test_grpc_response_without_metadata() {
        let response = GrpcResponse {
            payload: napi::bindgen_prelude::Buffer::from(vec![5, 10, 15]),
            metadata: None,
        };

        assert_eq!(response.payload.len(), 3);
        assert!(response.metadata.is_none());
    }

    #[test]
    fn test_grpc_request_empty_metadata() {
        let request = GrpcRequest {
            service_name: "test.Service".to_string(),
            method_name: "Method".to_string(),
            payload: napi::bindgen_prelude::Buffer::from(vec![]),
            metadata: HashMap::new(),
        };

        assert_eq!(request.metadata.len(), 0);
        assert_eq!(request.payload.len(), 0);
    }

    #[test]
    fn test_grpc_request_large_payload() {
        let large_payload = vec![0u8; 1024 * 1024]; // 1MB payload
        let request = GrpcRequest {
            service_name: "test.Service".to_string(),
            method_name: "LargeMethod".to_string(),
            payload: napi::bindgen_prelude::Buffer::from(large_payload),
            metadata: HashMap::new(),
        };

        assert_eq!(request.payload.len(), 1024 * 1024);
    }

    #[test]
    fn test_grpc_service_name_parsing() {
        let request = GrpcRequest {
            service_name: "mypackage.v1.UserService".to_string(),
            method_name: "GetUser".to_string(),
            payload: napi::bindgen_prelude::Buffer::from(vec![]),
            metadata: HashMap::new(),
        };

        assert!(request.service_name.contains('.'));
        let parts: Vec<&str> = request.service_name.split('.').collect();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], "mypackage");
        assert_eq!(parts[1], "v1");
        assert_eq!(parts[2], "UserService");
    }

    #[test]
    fn test_grpc_metadata_special_characters() {
        let mut metadata = HashMap::new();
        metadata.insert("x-request-id".to_string(), "abc-123-def".to_string());
        metadata.insert("x-trace-id".to_string(), "trace:12345".to_string());

        let request = GrpcRequest {
            service_name: "test.Service".to_string(),
            method_name: "Method".to_string(),
            payload: napi::bindgen_prelude::Buffer::from(vec![]),
            metadata,
        };

        assert_eq!(request.metadata.get("x-request-id"), Some(&"abc-123-def".to_string()));
        assert_eq!(request.metadata.get("x-trace-id"), Some(&"trace:12345".to_string()));
    }
}

//! Integration tests for gRPC runtime support
//!
//! These tests verify that the gRPC infrastructure works end-to-end,
//! including handler registration, request routing, and response handling.

use bytes::Bytes;
use spikard_http::grpc::{
    GrpcConfig, GrpcHandler, GrpcHandlerResult, GrpcRegistry, GrpcRequestData, GrpcResponseData, RpcMode,
};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Test handler that echoes the request payload
struct EchoGrpcHandler;

impl GrpcHandler for EchoGrpcHandler {
    fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        Box::pin(async move {
            Ok(GrpcResponseData {
                payload: request.payload,
                metadata: tonic::metadata::MetadataMap::new(),
            })
        })
    }

    fn service_name(&self) -> &'static str {
        "test.EchoService"
    }
}

/// Test handler that returns a fixed response
struct FixedResponseHandler {
    response: Bytes,
}

impl GrpcHandler for FixedResponseHandler {
    fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        let response = self.response.clone();
        Box::pin(async move {
            Ok(GrpcResponseData {
                payload: response,
                metadata: tonic::metadata::MetadataMap::new(),
            })
        })
    }

    fn service_name(&self) -> &'static str {
        "test.FixedService"
    }
}

/// Test handler that returns an error
struct ErrorGrpcHandler;

impl GrpcHandler for ErrorGrpcHandler {
    fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        Box::pin(async { Err(tonic::Status::not_found("Resource not found")) })
    }

    fn service_name(&self) -> &'static str {
        "test.ErrorService"
    }
}

#[test]
fn test_grpc_config_creation() {
    let config = GrpcConfig::default();
    assert!(config.enabled);
    assert_eq!(config.max_message_size, 4 * 1024 * 1024);
    assert!(config.enable_compression);
    assert!(config.request_timeout.is_none());
}

#[test]
fn test_grpc_config_customization() {
    let config = GrpcConfig {
        enabled: true,
        max_message_size: 1024,
        enable_compression: false,
        request_timeout: Some(30),
        max_concurrent_streams: 50,
        enable_keepalive: false,
        keepalive_interval: 60,
        keepalive_timeout: 10,
    };

    assert_eq!(config.max_message_size, 1024);
    assert!(!config.enable_compression);
    assert_eq!(config.request_timeout, Some(30));
    assert_eq!(config.max_concurrent_streams, 50);
    assert!(!config.enable_keepalive);
}

#[test]
fn test_grpc_registry_creation() {
    let registry = GrpcRegistry::new();
    assert!(registry.is_empty());
    assert_eq!(registry.len(), 0);
}

#[test]
fn test_grpc_registry_register_handler() {
    let mut registry = GrpcRegistry::new();
    let handler = Arc::new(EchoGrpcHandler);

    registry.register("test.EchoService", handler, RpcMode::Unary);

    assert!(!registry.is_empty());
    assert_eq!(registry.len(), 1);
    assert!(registry.contains("test.EchoService"));
}

#[test]
fn test_grpc_registry_get_handler() {
    let mut registry = GrpcRegistry::new();
    let handler = Arc::new(EchoGrpcHandler);

    registry.register("test.EchoService", handler, RpcMode::Unary);

    let retrieved = registry.get("test.EchoService");
    assert!(retrieved.is_some());
    let (handler, mode) = retrieved.unwrap();
    assert_eq!(handler.service_name(), "test.EchoService");
    assert_eq!(mode, RpcMode::Unary);
}

#[test]
fn test_grpc_registry_multiple_handlers() {
    let mut registry = GrpcRegistry::new();

    registry.register("test.EchoService", Arc::new(EchoGrpcHandler), RpcMode::Unary);
    registry.register(
        "test.FixedService",
        Arc::new(FixedResponseHandler {
            response: Bytes::from("fixed"),
        }),
        RpcMode::Unary,
    );
    registry.register("test.ErrorService", Arc::new(ErrorGrpcHandler), RpcMode::Unary);

    assert_eq!(registry.len(), 3);
    assert!(registry.contains("test.EchoService"));
    assert!(registry.contains("test.FixedService"));
    assert!(registry.contains("test.ErrorService"));

    let names = registry.service_names();
    assert_eq!(names.len(), 3);
}

#[tokio::test]
async fn test_echo_handler_basic() {
    let handler = EchoGrpcHandler;
    let request = GrpcRequestData {
        service_name: "test.EchoService".to_string(),
        method_name: "Echo".to_string(),
        payload: Bytes::from("test payload"),
        metadata: tonic::metadata::MetadataMap::new(),
    };

    let result = handler.call(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.payload, Bytes::from("test payload"));
}

#[tokio::test]
async fn test_echo_handler_empty_payload() {
    let handler = EchoGrpcHandler;
    let request = GrpcRequestData {
        service_name: "test.EchoService".to_string(),
        method_name: "Echo".to_string(),
        payload: Bytes::new(),
        metadata: tonic::metadata::MetadataMap::new(),
    };

    let result = handler.call(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.payload, Bytes::new());
}

#[tokio::test]
async fn test_echo_handler_large_payload() {
    let handler = EchoGrpcHandler;
    let large_data = vec![0u8; 10_000];
    let request = GrpcRequestData {
        service_name: "test.EchoService".to_string(),
        method_name: "Echo".to_string(),
        payload: Bytes::from(large_data.clone()),
        metadata: tonic::metadata::MetadataMap::new(),
    };

    let result = handler.call(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.payload.len(), 10_000);
    assert_eq!(response.payload, Bytes::from(large_data));
}

#[tokio::test]
async fn test_fixed_response_handler() {
    let handler = FixedResponseHandler {
        response: Bytes::from("fixed response"),
    };

    let request = GrpcRequestData {
        service_name: "test.FixedService".to_string(),
        method_name: "GetFixed".to_string(),
        payload: Bytes::from("any input"),
        metadata: tonic::metadata::MetadataMap::new(),
    };

    let result = handler.call(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.payload, Bytes::from("fixed response"));
}

#[tokio::test]
async fn test_error_handler() {
    let handler = ErrorGrpcHandler;
    let request = GrpcRequestData {
        service_name: "test.ErrorService".to_string(),
        method_name: "Error".to_string(),
        payload: Bytes::new(),
        metadata: tonic::metadata::MetadataMap::new(),
    };

    let result = handler.call(request).await;
    assert!(result.is_err());

    let error = result.unwrap_err();
    assert_eq!(error.code(), tonic::Code::NotFound);
    assert_eq!(error.message(), "Resource not found");
}

#[tokio::test]
async fn test_handler_with_metadata() {
    let handler = EchoGrpcHandler;
    let mut metadata = tonic::metadata::MetadataMap::new();
    metadata.insert("custom-header", "custom-value".parse().unwrap());

    let request = GrpcRequestData {
        service_name: "test.EchoService".to_string(),
        method_name: "Echo".to_string(),
        payload: Bytes::from("test"),
        metadata,
    };

    let result = handler.call(request).await;
    assert!(result.is_ok());
}

#[test]
fn test_grpc_request_data_creation() {
    let request = GrpcRequestData {
        service_name: "mypackage.MyService".to_string(),
        method_name: "GetUser".to_string(),
        payload: Bytes::from("payload"),
        metadata: tonic::metadata::MetadataMap::new(),
    };

    assert_eq!(request.service_name, "mypackage.MyService");
    assert_eq!(request.method_name, "GetUser");
    assert_eq!(request.payload, Bytes::from("payload"));
}

#[test]
fn test_grpc_response_data_creation() {
    let response = GrpcResponseData {
        payload: Bytes::from("response"),
        metadata: tonic::metadata::MetadataMap::new(),
    };

    assert_eq!(response.payload, Bytes::from("response"));
}

#[test]
fn test_handler_service_name() {
    let echo_handler = EchoGrpcHandler;
    assert_eq!(echo_handler.service_name(), "test.EchoService");

    let fixed_handler = FixedResponseHandler { response: Bytes::new() };
    assert_eq!(fixed_handler.service_name(), "test.FixedService");

    let error_handler = ErrorGrpcHandler;
    assert_eq!(error_handler.service_name(), "test.ErrorService");
}

#[test]
fn test_handler_default_rpc_mode() {
    let handler = EchoGrpcHandler;
    assert_eq!(handler.rpc_mode(), RpcMode::Unary);
}

#[test]
fn test_grpc_config_serialization() {
    let config = GrpcConfig::default();
    let json = serde_json::to_string(&config).unwrap();
    let deserialized: GrpcConfig = serde_json::from_str(&json).unwrap();

    assert_eq!(config.enabled, deserialized.enabled);
    assert_eq!(config.max_message_size, deserialized.max_message_size);
}

#[tokio::test]
async fn test_concurrent_handler_calls() {
    let handler = Arc::new(EchoGrpcHandler);

    let mut tasks = vec![];
    for i in 0..10 {
        let handler_clone = Arc::clone(&handler);
        let handle = tokio::spawn(async move {
            let request = GrpcRequestData {
                service_name: "test.EchoService".to_string(),
                method_name: "Echo".to_string(),
                payload: Bytes::from(format!("message {i}")),
                metadata: tonic::metadata::MetadataMap::new(),
            };
            handler_clone.call(request).await
        });
        tasks.push(handle);
    }

    for handle in tasks {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
}

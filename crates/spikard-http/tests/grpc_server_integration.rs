//! Comprehensive gRPC server integration tests
//!
//! Tests the gRPC runtime infrastructure including:
//! - Server routing with multiple services
//! - Unary RPC handling
//! - Request/response payload handling
//! - Error responses with different gRPC status codes

use crate::common::grpc_helpers::*;
use bytes::Bytes;
use serde_json::json;
use std::sync::Arc;
use tonic::Code;

mod common;

/// Test successful unary RPC with JSON payload
#[tokio::test]
async fn test_unary_rpc_success_with_json_payload() {
    // Create a custom handler with proper service name
    struct UserServiceHandler;
    impl spikard_http::grpc::GrpcHandler for UserServiceHandler {
        fn call(
            &self,
            _request: spikard_http::grpc::GrpcRequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard_http::grpc::GrpcHandlerResult> + Send>>
        {
            let payload = serde_json::to_vec(&json!({
                "id": 123,
                "name": "Alice Johnson",
                "email": "alice@example.com"
            }))
            .unwrap();
            Box::pin(async {
                Ok(spikard_http::grpc::GrpcResponseData {
                    payload: Bytes::from(payload),
                    metadata: tonic::metadata::MetadataMap::new(),
                })
            })
        }
        fn service_name(&self) -> &'static str {
            "com.example.api.v1.UserService"
        }
    }

    // Arrange: Create test server and handler
    let server = GrpcTestServer::new();

    server.register_service(Arc::new(UserServiceHandler));

    // Act: Create and send request
    let mut request_builder = ProtobufMessageBuilder::new();
    let request_payload = request_builder.add_int_field("user_id", 123).build().unwrap();

    let response = send_unary_request(
        &server,
        "com.example.api.v1.UserService",
        "GetUser",
        request_payload,
        create_test_metadata(),
    )
    .await
    .expect("Failed to send unary request");

    // Assert
    assert_grpc_response(
        &response,
        &json!({
            "id": 123,
            "name": "Alice Johnson",
            "email": "alice@example.com"
        }),
    );
}

/// Test server routing with multiple services
#[tokio::test]
async fn test_server_routes_to_correct_service() {
    struct UserServiceHandler;
    impl spikard_http::grpc::GrpcHandler for UserServiceHandler {
        fn call(
            &self,
            _request: spikard_http::grpc::GrpcRequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard_http::grpc::GrpcHandlerResult> + Send>>
        {
            Box::pin(async {
                Ok(spikard_http::grpc::GrpcResponseData {
                    payload: Bytes::from(r#"{"service": "UserService"}"#),
                    metadata: tonic::metadata::MetadataMap::new(),
                })
            })
        }
        fn service_name(&self) -> &'static str {
            "api.UserService"
        }
    }

    struct OrderServiceHandler;
    impl spikard_http::grpc::GrpcHandler for OrderServiceHandler {
        fn call(
            &self,
            _request: spikard_http::grpc::GrpcRequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard_http::grpc::GrpcHandlerResult> + Send>>
        {
            Box::pin(async {
                Ok(spikard_http::grpc::GrpcResponseData {
                    payload: Bytes::from(r#"{"service": "OrderService"}"#),
                    metadata: tonic::metadata::MetadataMap::new(),
                })
            })
        }
        fn service_name(&self) -> &'static str {
            "api.OrderService"
        }
    }

    // Arrange: Register multiple services
    let server = GrpcTestServer::new();

    server.register_service(Arc::new(UserServiceHandler));
    server.register_service(Arc::new(OrderServiceHandler));

    // Act & Assert: Route to UserService
    let user_response = send_unary_request(
        &server,
        "api.UserService",
        "GetUser",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await
    .expect("UserService request failed");

    assert_grpc_response(&user_response, &json!({"service": "UserService"}));

    // Act & Assert: Route to OrderService
    let order_response = send_unary_request(
        &server,
        "api.OrderService",
        "GetOrder",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await
    .expect("OrderService request failed");

    assert_grpc_response(&order_response, &json!({"service": "OrderService"}));
}

/// Test server correctly counts registered services
#[tokio::test]
async fn test_server_service_registration_count() {
    let server = GrpcTestServer::new();

    assert_eq!(server.service_count(), 0);

    // Register first service
    let handler1 = Arc::new(MockGrpcHandler::with_json("service.One", &json!({"id": 1})));
    server.register_service(handler1);
    assert_eq!(server.service_count(), 1);

    // Register second service
    let handler2 = Arc::new(MockGrpcHandler::with_json("service.Two", &json!({"id": 2})));
    server.register_service(handler2);
    assert_eq!(server.service_count(), 2);

    // Register third service
    let handler3 = Arc::new(MockGrpcHandler::with_json("service.Three", &json!({"id": 3})));
    server.register_service(handler3);
    assert_eq!(server.service_count(), 3);
}

/// Test unary RPC with complex nested JSON payload
#[tokio::test]
async fn test_unary_rpc_with_nested_json_payload() {
    struct ProductServiceHandler;
    impl spikard_http::grpc::GrpcHandler for ProductServiceHandler {
        fn call(
            &self,
            _request: spikard_http::grpc::GrpcRequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard_http::grpc::GrpcHandlerResult> + Send>>
        {
            Box::pin(async {
                Ok(spikard_http::grpc::GrpcResponseData {
                    payload: Bytes::from(
                        r#"{
                        "id": 42,
                        "name": "Laptop",
                        "price": 999.99,
                        "inventory": {
                            "quantity": 100,
                            "warehouse": "US-WEST"
                        },
                        "tags": ["electronics", "computers", "portable"]
                    }"#,
                    ),
                    metadata: tonic::metadata::MetadataMap::new(),
                })
            })
        }
        fn service_name(&self) -> &'static str {
            "shop.ProductService"
        }
    }

    let server = GrpcTestServer::new();

    server.register_service(Arc::new(ProductServiceHandler));

    let response = send_unary_request(
        &server,
        "shop.ProductService",
        "GetProduct",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await
    .expect("Failed to get product");

    // Verify nested structure
    let payload = serde_json::from_slice::<serde_json::Value>(&response.payload).unwrap();
    assert_eq!(payload["id"], 42);
    assert_eq!(payload["name"], "Laptop");
    assert_eq!(payload["price"], 999.99);
    assert_eq!(payload["inventory"]["quantity"], 100);
    assert_eq!(payload["inventory"]["warehouse"], "US-WEST");
    assert_eq!(payload["tags"].as_array().unwrap().len(), 3);
}

/// Test unary RPC with binary payload (raw bytes)
#[tokio::test]
async fn test_unary_rpc_with_binary_payload() {
    struct EchoServiceHandler;
    impl spikard_http::grpc::GrpcHandler for EchoServiceHandler {
        fn call(
            &self,
            request: spikard_http::grpc::GrpcRequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard_http::grpc::GrpcHandlerResult> + Send>>
        {
            let payload = request.payload;
            Box::pin(async {
                Ok(spikard_http::grpc::GrpcResponseData {
                    payload,
                    metadata: tonic::metadata::MetadataMap::new(),
                })
            })
        }
        fn service_name(&self) -> &'static str {
            "echo.EchoService"
        }
    }

    let server = GrpcTestServer::new();

    server.register_service(Arc::new(EchoServiceHandler));

    // Send binary payload
    let binary_data = vec![0x00, 0x01, 0x02, 0xFF, 0xFE, 0xFD];
    let response = send_unary_request(
        &server,
        "echo.EchoService",
        "Echo",
        Bytes::from(binary_data.clone()),
        create_test_metadata(),
    )
    .await
    .expect("Failed to echo");

    // Verify binary data is unchanged
    assert_eq!(response.payload.to_vec(), binary_data);
}

/// Test request with custom metadata is preserved in response
#[tokio::test]
async fn test_request_metadata_handling() {
    struct MetadataAwareHandler;
    impl spikard_http::grpc::GrpcHandler for MetadataAwareHandler {
        fn call(
            &self,
            request: spikard_http::grpc::GrpcRequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard_http::grpc::GrpcHandlerResult> + Send>>
        {
            // Check that metadata was received
            let has_custom_header = request.metadata.get("x-custom-header").is_some();
            let response_payload = if has_custom_header {
                Bytes::from(r#"{"status": "metadata_received"}"#)
            } else {
                Bytes::from(r#"{"status": "no_metadata"}"#)
            };

            Box::pin(async move {
                Ok(spikard_http::grpc::GrpcResponseData {
                    payload: response_payload,
                    metadata: tonic::metadata::MetadataMap::new(),
                })
            })
        }
        fn service_name(&self) -> &'static str {
            "test.MetadataService"
        }
    }

    let server = GrpcTestServer::new();

    server.register_service(Arc::new(MetadataAwareHandler));

    let mut metadata = create_test_metadata();
    add_metadata_header(&mut metadata, "x-custom-header", "test-value").unwrap();

    let response = send_unary_request(
        &server,
        "test.MetadataService",
        "TestMetadata",
        Bytes::from("{}"),
        metadata,
    )
    .await
    .expect("Failed to send request with metadata");

    assert_grpc_response(&response, &json!({"status": "metadata_received"}));
}

/// Test error response with `NOT_FOUND` status
#[tokio::test]
async fn test_error_response_not_found() {
    let server = GrpcTestServer::new();

    let handler = Arc::new(ErrorMockHandler::new(
        "api.NotFoundService",
        Code::NotFound,
        "User with ID 999 not found",
    ));
    server.register_service(handler);

    let result = send_unary_request(
        &server,
        "api.NotFoundService",
        "GetUser",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;

    assert!(result.is_err());
}

/// Test error response with `INVALID_ARGUMENT` status
#[tokio::test]
async fn test_error_response_invalid_argument() {
    let server = GrpcTestServer::new();

    let handler = Arc::new(ErrorMockHandler::new(
        "api.ValidationService",
        Code::InvalidArgument,
        "Email address is invalid",
    ));
    server.register_service(handler);

    let result = send_unary_request(
        &server,
        "api.ValidationService",
        "ValidateEmail",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;

    assert!(result.is_err());
}

/// Test error response with INTERNAL status
#[tokio::test]
async fn test_error_response_internal_server_error() {
    let server = GrpcTestServer::new();

    let handler = Arc::new(ErrorMockHandler::new(
        "api.DatabaseService",
        Code::Internal,
        "Database connection failed",
    ));
    server.register_service(handler);

    let result = send_unary_request(
        &server,
        "api.DatabaseService",
        "QueryData",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;

    assert!(result.is_err());
}

/// Test server with no services registered
#[tokio::test]
async fn test_server_no_services_registered() {
    let server = GrpcTestServer::new();

    assert_eq!(server.service_count(), 0);
    assert!(server.handlers().is_empty());

    // Attempting to call a non-existent service should fail
    let result = send_unary_request(
        &server,
        "nonexistent.Service",
        "Method",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;

    assert!(result.is_err());
}

/// Test unary RPC with empty request payload
#[tokio::test]
async fn test_unary_rpc_empty_request_payload() {
    struct EmptyRequestHandler;
    impl spikard_http::grpc::GrpcHandler for EmptyRequestHandler {
        fn call(
            &self,
            _request: spikard_http::grpc::GrpcRequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard_http::grpc::GrpcHandlerResult> + Send>>
        {
            Box::pin(async {
                Ok(spikard_http::grpc::GrpcResponseData {
                    payload: Bytes::from(r#"{"status": "ok"}"#),
                    metadata: tonic::metadata::MetadataMap::new(),
                })
            })
        }
        fn service_name(&self) -> &'static str {
            "test.EmptyRequestService"
        }
    }

    let server = GrpcTestServer::new();

    server.register_service(Arc::new(EmptyRequestHandler));

    let response = send_unary_request(
        &server,
        "test.EmptyRequestService",
        "HealthCheck",
        Bytes::new(),
        create_test_metadata(),
    )
    .await
    .expect("Failed to send empty request");

    assert_grpc_response(&response, &json!({"status": "ok"}));
}

/// Test echo handler preserves exact request payload
#[tokio::test]
async fn test_echo_handler_payload_preservation() {
    struct TestEchoHandler;
    impl spikard_http::grpc::GrpcHandler for TestEchoHandler {
        fn call(
            &self,
            request: spikard_http::grpc::GrpcRequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard_http::grpc::GrpcHandlerResult> + Send>>
        {
            let payload = request.payload;
            Box::pin(async move {
                Ok(spikard_http::grpc::GrpcResponseData {
                    payload,
                    metadata: tonic::metadata::MetadataMap::new(),
                })
            })
        }
        fn service_name(&self) -> &'static str {
            "test.EchoService"
        }
    }

    let server = GrpcTestServer::new();

    server.register_service(Arc::new(TestEchoHandler));

    let request_payload = Bytes::from(r#"{"echo": "test message"}"#);

    let response = send_unary_request(
        &server,
        "test.EchoService",
        "Echo",
        request_payload.clone(),
        create_test_metadata(),
    )
    .await
    .expect("Failed to echo");

    assert_eq!(response.payload, request_payload);
}

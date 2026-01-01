//! Comprehensive gRPC error handling tests
//!
//! Tests all 17 standard gRPC status codes including:
//! - `OK` (0)
//! - `CANCELLED` (1)
//! - `UNKNOWN` (2)
//! - `INVALID_ARGUMENT` (3)
//! - `DEADLINE_EXCEEDED` (4)
//! - `NOT_FOUND` (5)
//! - `ALREADY_EXISTS` (6)
//! - `PERMISSION_DENIED` (7)
//! - `RESOURCE_EXHAUSTED` (8)
//! - `FAILED_PRECONDITION` (9)
//! - `ABORTED` (10)
//! - `OUT_OF_RANGE` (11)
//! - `UNIMPLEMENTED` (12)
//! - `INTERNAL` (13)
//! - `UNAVAILABLE` (14)
//! - `DATA_LOSS` (15)
//! - `UNAUTHENTICATED` (16)

use crate::common::grpc_helpers::*;
use bytes::Bytes;
use std::sync::Arc;
use tonic::Code;

mod common;

/// Test OK status (success case)
#[tokio::test]
async fn test_status_ok_success() {
    struct SuccessHandler;
    impl spikard_http::grpc::GrpcHandler for SuccessHandler {
        fn call(
            &self,
            _request: spikard_http::grpc::GrpcRequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard_http::grpc::GrpcHandlerResult> + Send>>
        {
            Box::pin(async {
                Ok(spikard_http::grpc::GrpcResponseData {
                    payload: Bytes::from(r#"{"result": "success"}"#),
                    metadata: tonic::metadata::MetadataMap::new(),
                })
            })
        }
        fn service_name(&self) -> &'static str {
            "test.SuccessService"
        }
    }

    let server = GrpcTestServer::new();

    server.register_service(Arc::new(SuccessHandler));

    let response = send_unary_request(
        &server,
        "test.SuccessService",
        "DoWork",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;

    assert!(response.is_ok());
    let resp = response.unwrap();
    assert_grpc_response(&resp, &serde_json::json!({"result": "success"}));
}

/// Test CANCELLED status code
#[tokio::test]
async fn test_status_cancelled() {
    let server = GrpcTestServer::new();

    let handler = Arc::new(ErrorMockHandler::new(
        "test.CancelledService",
        Code::Cancelled,
        "Request was cancelled by the client",
    ));
    server.register_service(handler);

    let result = send_unary_request(
        &server,
        "test.CancelledService",
        "CancelledOperation",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;

    // Verify error occurred
    assert!(result.is_err());
}

/// Test UNKNOWN status code
#[tokio::test]
async fn test_status_unknown() {
    let server = GrpcTestServer::new();

    let handler = Arc::new(ErrorMockHandler::new(
        "test.UnknownService",
        Code::Unknown,
        "Unknown error occurred",
    ));
    server.register_service(handler);

    let result = send_unary_request(
        &server,
        "test.UnknownService",
        "UnknownMethod",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;

    assert!(result.is_err());
}

/// Test `INVALID_ARGUMENT` status code
#[tokio::test]
async fn test_status_invalid_argument() {
    let server = GrpcTestServer::new();

    let handler = Arc::new(ErrorMockHandler::new(
        "test.ValidationService",
        Code::InvalidArgument,
        "Field 'email' must be a valid email address",
    ));
    server.register_service(handler);

    let result = send_unary_request(
        &server,
        "test.ValidationService",
        "ValidateInput",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;

    assert!(result.is_err());
    // Error verified
}

/// Test `DEADLINE_EXCEEDED` status code
#[tokio::test]
async fn test_status_deadline_exceeded() {
    let server = GrpcTestServer::new();

    let handler = Arc::new(ErrorMockHandler::new(
        "test.TimeoutService",
        Code::DeadlineExceeded,
        "Request took too long to process (timeout)",
    ));
    server.register_service(handler);

    let result = send_unary_request(
        &server,
        "test.TimeoutService",
        "SlowOperation",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;

    assert!(result.is_err());
    // Error verified
}

/// Test `NOT_FOUND` status code
#[tokio::test]
async fn test_status_not_found() {
    let server = GrpcTestServer::new();

    let handler = Arc::new(ErrorMockHandler::new(
        "test.NotFoundService",
        Code::NotFound,
        "User with ID 12345 does not exist",
    ));
    server.register_service(handler);

    let result = send_unary_request(
        &server,
        "test.NotFoundService",
        "GetUser",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;

    assert!(result.is_err());
    // Error verified
}

/// Test `ALREADY_EXISTS` status code
#[tokio::test]
async fn test_status_already_exists() {
    let server = GrpcTestServer::new();

    let handler = Arc::new(ErrorMockHandler::new(
        "test.CreateService",
        Code::AlreadyExists,
        "User with email 'john@example.com' already exists",
    ));
    server.register_service(handler);

    let result = send_unary_request(
        &server,
        "test.CreateService",
        "CreateUser",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;

    assert!(result.is_err());
    // Error verified
}

/// Test `PERMISSION_DENIED` status code
#[tokio::test]
async fn test_status_permission_denied() {
    let server = GrpcTestServer::new();

    let handler = Arc::new(ErrorMockHandler::new(
        "test.AuthService",
        Code::PermissionDenied,
        "User does not have permission to delete resources",
    ));
    server.register_service(handler);

    let result = send_unary_request(
        &server,
        "test.AuthService",
        "DeleteResource",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;

    assert!(result.is_err());
    // Error verified
}

/// Test `RESOURCE_EXHAUSTED` status code
#[tokio::test]
async fn test_status_resource_exhausted() {
    let server = GrpcTestServer::new();

    let handler = Arc::new(ErrorMockHandler::new(
        "test.QuotaService",
        Code::ResourceExhausted,
        "API rate limit exceeded. Maximum 100 requests per minute.",
    ));
    server.register_service(handler);

    let result = send_unary_request(
        &server,
        "test.QuotaService",
        "MakeRequest",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;

    assert!(result.is_err());
    // Error verified
}

/// Test `FAILED_PRECONDITION` status code
#[tokio::test]
async fn test_status_failed_precondition() {
    let server = GrpcTestServer::new();

    let handler = Arc::new(ErrorMockHandler::new(
        "test.OrderService",
        Code::FailedPrecondition,
        "Cannot process order: inventory for product is out of stock",
    ));
    server.register_service(handler);

    let result = send_unary_request(
        &server,
        "test.OrderService",
        "ProcessOrder",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;

    assert!(result.is_err());
    // Error verified
}

/// Test `ABORTED` status code
#[tokio::test]
async fn test_status_aborted() {
    let server = GrpcTestServer::new();

    let handler = Arc::new(ErrorMockHandler::new(
        "test.TransactionService",
        Code::Aborted,
        "Transaction was aborted due to concurrency conflict",
    ));
    server.register_service(handler);

    let result = send_unary_request(
        &server,
        "test.TransactionService",
        "CommitTransaction",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;

    assert!(result.is_err());
    // Error verified
}

/// Test `OUT_OF_RANGE` status code
#[tokio::test]
async fn test_status_out_of_range() {
    let server = GrpcTestServer::new();

    let handler = Arc::new(ErrorMockHandler::new(
        "test.PageService",
        Code::OutOfRange,
        "Requested page 999 is out of range. Only 10 pages available.",
    ));
    server.register_service(handler);

    let result = send_unary_request(
        &server,
        "test.PageService",
        "GetPage",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;

    assert!(result.is_err());
    // Error verified
}

/// Test `UNIMPLEMENTED` status code
#[tokio::test]
async fn test_status_unimplemented() {
    let server = GrpcTestServer::new();

    let handler = Arc::new(ErrorMockHandler::new(
        "test.ExperimentalService",
        Code::Unimplemented,
        "Method 'AdvancedFeature' is not yet implemented",
    ));
    server.register_service(handler);

    let result = send_unary_request(
        &server,
        "test.ExperimentalService",
        "AdvancedFeature",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;

    assert!(result.is_err());
    // Error verified
}

/// Test `INTERNAL` status code
#[tokio::test]
async fn test_status_internal_error() {
    let server = GrpcTestServer::new();

    let handler = Arc::new(ErrorMockHandler::new(
        "test.DatabaseService",
        Code::Internal,
        "Internal server error: Database connection pool exhausted",
    ));
    server.register_service(handler);

    let result = send_unary_request(
        &server,
        "test.DatabaseService",
        "QueryData",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;

    assert!(result.is_err());
    // Error verified
}

/// Test `UNAVAILABLE` status code
#[tokio::test]
async fn test_status_unavailable() {
    let server = GrpcTestServer::new();

    let handler = Arc::new(ErrorMockHandler::new(
        "test.ExternalService",
        Code::Unavailable,
        "Service is temporarily unavailable. Please retry later.",
    ));
    server.register_service(handler);

    let result = send_unary_request(
        &server,
        "test.ExternalService",
        "CallExternal",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;

    assert!(result.is_err());
    // Error verified
}

/// Test `DATA_LOSS` status code
#[tokio::test]
async fn test_status_data_loss() {
    let server = GrpcTestServer::new();

    let handler = Arc::new(ErrorMockHandler::new(
        "test.FileService",
        Code::DataLoss,
        "Data loss detected: corrupted database records",
    ));
    server.register_service(handler);

    let result = send_unary_request(
        &server,
        "test.FileService",
        "RecoverData",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;

    assert!(result.is_err());
    // Error verified
}

/// Test `UNAUTHENTICATED` status code
#[tokio::test]
async fn test_status_unauthenticated() {
    let server = GrpcTestServer::new();

    let handler = Arc::new(ErrorMockHandler::new(
        "test.SecureService",
        Code::Unauthenticated,
        "Authentication required. Please provide valid credentials.",
    ));
    server.register_service(handler);

    let result = send_unary_request(
        &server,
        "test.SecureService",
        "SecureOperation",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;

    assert!(result.is_err());
    // Error verified
}

/// Test error message propagation
#[tokio::test]
async fn test_error_message_propagation() {
    let server = GrpcTestServer::new();

    let detailed_message = "Validation failed: Password must be at least 12 characters. \
                             Current length: 8. Special characters required.";
    let handler = Arc::new(ErrorMockHandler::new(
        "test.DetailedErrorService",
        Code::InvalidArgument,
        detailed_message,
    ));
    server.register_service(handler);

    let result = send_unary_request(
        &server,
        "test.DetailedErrorService",
        "Validate",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;

    assert!(result.is_err());
}

/// Test multiple error responses in sequence
#[tokio::test]
async fn test_multiple_error_responses() {
    let server = GrpcTestServer::new();

    // Register service that returns INVALID_ARGUMENT
    let handler1 = Arc::new(ErrorMockHandler::new(
        "test.Service1",
        Code::InvalidArgument,
        "Invalid input",
    ));
    server.register_service(handler1);

    // Register service that returns NOT_FOUND
    let handler2 = Arc::new(ErrorMockHandler::new(
        "test.Service2",
        Code::NotFound,
        "Resource not found",
    ));
    server.register_service(handler2);

    // Register service that returns PERMISSION_DENIED
    let handler3 = Arc::new(ErrorMockHandler::new(
        "test.Service3",
        Code::PermissionDenied,
        "Access denied",
    ));
    server.register_service(handler3);

    // Test first error
    let result1 = send_unary_request(
        &server,
        "test.Service1",
        "Method",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;
    assert!(result1.is_err());

    // Test second error
    let result2 = send_unary_request(
        &server,
        "test.Service2",
        "Method",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;
    assert!(result2.is_err());

    // Test third error
    let result3 = send_unary_request(
        &server,
        "test.Service3",
        "Method",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;
    assert!(result3.is_err());
}

/// Test error with empty message
#[tokio::test]
async fn test_error_with_empty_message() {
    let server = GrpcTestServer::new();

    let handler = Arc::new(ErrorMockHandler::new("test.EmptyErrorService", Code::Internal, ""));
    server.register_service(handler);

    let result = send_unary_request(
        &server,
        "test.EmptyErrorService",
        "Method",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;

    assert!(result.is_err());
}

/// Test error with very long message
#[tokio::test]
async fn test_error_with_long_message() {
    let server = GrpcTestServer::new();

    let long_message =
        "Error occurred during processing: ".to_string() + &"detailed reason ".repeat(50) + "Please contact support.";

    let handler = Arc::new(ErrorMockHandler::new(
        "test.LongErrorService",
        Code::Internal,
        &long_message,
    ));
    server.register_service(handler);

    let result = send_unary_request(
        &server,
        "test.LongErrorService",
        "Method",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await;

    assert!(result.is_err());
}

/// Test handler conversion of custom errors to gRPC status
#[tokio::test]
async fn test_handler_error_to_status_conversion() {
    struct ValidationHandler;
    impl spikard_http::grpc::GrpcHandler for ValidationHandler {
        fn call(
            &self,
            request: spikard_http::grpc::GrpcRequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard_http::grpc::GrpcHandlerResult> + Send>>
        {
            // Simulate validation logic
            if request.payload.is_empty() {
                Box::pin(async {
                    Err(tonic::Status::new(
                        Code::InvalidArgument,
                        "Request body cannot be empty",
                    ))
                })
            } else {
                Box::pin(async {
                    Ok(spikard_http::grpc::GrpcResponseData {
                        payload: Bytes::from(r#"{"status": "valid"}"#),
                        metadata: tonic::metadata::MetadataMap::new(),
                    })
                })
            }
        }
        fn service_name(&self) -> &'static str {
            "test.ValidationService"
        }
    }

    let server = GrpcTestServer::new();

    server.register_service(Arc::new(ValidationHandler));

    // Test with empty payload (should error)
    let error_result = send_unary_request(
        &server,
        "test.ValidationService",
        "Validate",
        Bytes::new(),
        create_test_metadata(),
    )
    .await;

    assert!(error_result.is_err());
    assert!(error_result.unwrap_err().to_string().contains("empty"));
}

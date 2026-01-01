//! Comprehensive gRPC metadata handling tests
//!
//! Tests metadata extraction, setting, and manipulation including:
//! - Metadata extraction from requests
//! - Metadata setting in responses
//! - Authentication headers (Bearer tokens)
//! - Custom headers
//! - Metadata case sensitivity
//! - Special characters in metadata values

use crate::common::grpc_helpers::*;
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::Arc;
use tonic::metadata::MetadataMap;

mod common;

/// Test metadata extraction from request
#[tokio::test]
async fn test_metadata_extraction_from_request() {
    struct MetadataCheckHandler;
    impl spikard_http::grpc::GrpcHandler for MetadataCheckHandler {
        fn call(
            &self,
            request: spikard_http::grpc::GrpcRequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard_http::grpc::GrpcHandlerResult> + Send>>
        {
            // Extract and verify metadata
            let has_user_agent = request.metadata.get("user-agent").is_some();
            let has_content_type = request.metadata.get("content-type").is_some();

            let response = if has_user_agent && has_content_type {
                Bytes::from(r#"{"extracted": true}"#)
            } else {
                Bytes::from(r#"{"extracted": false}"#)
            };

            Box::pin(async move {
                Ok(spikard_http::grpc::GrpcResponseData {
                    payload: response,
                    metadata: MetadataMap::new(),
                })
            })
        }
        fn service_name(&self) -> &'static str {
            "test.MetadataCheckService"
        }
    }

    let server = GrpcTestServer::new();

    server.register_service(Arc::new(MetadataCheckHandler));

    let metadata = create_test_metadata();

    let response = send_unary_request(
        &server,
        "test.MetadataCheckService",
        "Check",
        Bytes::from("{}"),
        metadata,
    )
    .await
    .expect("Failed to send request with metadata");

    assert_grpc_response(&response, &serde_json::json!({"extracted": true}));
}

/// Test Bearer token authentication metadata
#[tokio::test]
async fn test_authentication_bearer_token_metadata() {
    struct AuthServiceHandler;
    impl spikard_http::grpc::GrpcHandler for AuthServiceHandler {
        fn call(
            &self,
            request: spikard_http::grpc::GrpcRequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard_http::grpc::GrpcHandlerResult> + Send>>
        {
            let token_present = request
                .metadata
                .get("authorization")
                .is_some_and(|v| v.to_str().unwrap_or("").starts_with("Bearer "));

            let response = if token_present {
                Bytes::from(r#"{"authenticated": true, "user": "alice"}"#)
            } else {
                Bytes::from(r#"{"authenticated": false}"#)
            };

            Box::pin(async move {
                Ok(spikard_http::grpc::GrpcResponseData {
                    payload: response,
                    metadata: MetadataMap::new(),
                })
            })
        }
        fn service_name(&self) -> &'static str {
            "api.AuthService"
        }
    }

    let server = GrpcTestServer::new();

    server.register_service(Arc::new(AuthServiceHandler));

    let mut metadata = create_test_metadata();
    add_auth_metadata(&mut metadata, "secret_token_abc123").unwrap();

    let response = send_unary_request(&server, "api.AuthService", "Authenticate", Bytes::from("{}"), metadata)
        .await
        .expect("Failed to authenticate");

    assert_grpc_response(
        &response,
        &serde_json::json!({
            "authenticated": true,
            "user": "alice"
        }),
    );
}

/// Test custom header metadata
#[tokio::test]
async fn test_custom_header_metadata() {
    struct CustomHeaderHandler;
    impl spikard_http::grpc::GrpcHandler for CustomHeaderHandler {
        fn call(
            &self,
            request: spikard_http::grpc::GrpcRequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard_http::grpc::GrpcHandlerResult> + Send>>
        {
            let custom_value = request
                .metadata
                .get("x-custom-header")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("missing")
                .to_string();

            let response = format!(r#"{{"custom_value": "{custom_value}"}}"#);

            Box::pin(async move {
                Ok(spikard_http::grpc::GrpcResponseData {
                    payload: Bytes::from(response),
                    metadata: MetadataMap::new(),
                })
            })
        }
        fn service_name(&self) -> &'static str {
            "test.CustomHeaderService"
        }
    }

    let server = GrpcTestServer::new();

    server.register_service(Arc::new(CustomHeaderHandler));

    let mut metadata = create_test_metadata();
    add_metadata_header(&mut metadata, "x-custom-header", "custom_value_123").unwrap();

    let response = send_unary_request(
        &server,
        "test.CustomHeaderService",
        "GetCustom",
        Bytes::from("{}"),
        metadata,
    )
    .await
    .expect("Failed to send custom header");

    assert_grpc_response(&response, &serde_json::json!({"custom_value": "custom_value_123"}));
}

/// Test multiple custom headers
#[tokio::test]
async fn test_multiple_custom_headers() {
    struct MultiHeaderHandler;
    impl spikard_http::grpc::GrpcHandler for MultiHeaderHandler {
        fn call(
            &self,
            request: spikard_http::grpc::GrpcRequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard_http::grpc::GrpcHandlerResult> + Send>>
        {
            let req_id = request
                .metadata
                .get("x-request-id")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("missing")
                .to_string();

            let trace_id = request
                .metadata
                .get("x-trace-id")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("missing")
                .to_string();

            let response = format!(r#"{{"request_id": "{req_id}", "trace_id": "{trace_id}"}}"#);

            Box::pin(async move {
                Ok(spikard_http::grpc::GrpcResponseData {
                    payload: Bytes::from(response),
                    metadata: MetadataMap::new(),
                })
            })
        }
        fn service_name(&self) -> &'static str {
            "test.MultiHeaderService"
        }
    }

    let server = GrpcTestServer::new();

    server.register_service(Arc::new(MultiHeaderHandler));

    let mut metadata = create_test_metadata();
    add_metadata_header(&mut metadata, "x-request-id", "req-12345").unwrap();
    add_metadata_header(&mut metadata, "x-trace-id", "trace-67890").unwrap();

    let response = send_unary_request(
        &server,
        "test.MultiHeaderService",
        "Process",
        Bytes::from("{}"),
        metadata,
    )
    .await
    .expect("Failed to send multiple headers");

    assert_grpc_response(
        &response,
        &serde_json::json!({
            "request_id": "req-12345",
            "trace_id": "trace-67890"
        }),
    );
}

/// Test metadata with special characters in values
#[tokio::test]
async fn test_metadata_special_characters() {
    struct SpecialCharHandler;
    impl spikard_http::grpc::GrpcHandler for SpecialCharHandler {
        fn call(
            &self,
            request: spikard_http::grpc::GrpcRequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard_http::grpc::GrpcHandlerResult> + Send>>
        {
            let special_header = request
                .metadata
                .get("x-special")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .to_string();

            let response = format!(r#"{{"received": "{special_header}"}}"#);

            Box::pin(async move {
                Ok(spikard_http::grpc::GrpcResponseData {
                    payload: Bytes::from(response),
                    metadata: MetadataMap::new(),
                })
            })
        }
        fn service_name(&self) -> &'static str {
            "test.SpecialCharService"
        }
    }

    let server = GrpcTestServer::new();

    server.register_service(Arc::new(SpecialCharHandler));

    let mut metadata = create_test_metadata();
    // Test with special characters that are URL-safe but uncommon
    add_metadata_header(&mut metadata, "x-special", "value-with_underscore.and.dots").unwrap();

    let response = send_unary_request(
        &server,
        "test.SpecialCharService",
        "Process",
        Bytes::from("{}"),
        metadata,
    )
    .await
    .expect("Failed to send special chars");

    assert_grpc_response(
        &response,
        &serde_json::json!({
            "received": "value-with_underscore.and.dots"
        }),
    );
}

/// Test metadata creation with `HashMap`
#[test]
fn test_create_metadata_with_headers_map() {
    let mut headers = HashMap::new();
    headers.insert("authorization".to_string(), "Bearer token123".to_string());
    headers.insert("x-custom-header".to_string(), "custom_value".to_string());
    headers.insert("x-request-id".to_string(), "req-456".to_string());

    let metadata = create_test_metadata_with_headers(&headers).expect("Failed to create metadata");

    assert!(metadata.get("authorization").is_some());
    assert!(metadata.get("x-custom-header").is_some());
    assert!(metadata.get("x-request-id").is_some());
}

/// Test default metadata contains expected headers
#[test]
fn test_default_metadata_headers() {
    let metadata = create_test_metadata();

    let user_agent = metadata.get("user-agent").expect("Missing user-agent");
    assert_eq!(user_agent.to_str().unwrap(), "spikard-test/1.0");

    let content_type = metadata.get("content-type").expect("Missing content-type");
    assert_eq!(content_type.to_str().unwrap(), "application/grpc");
}

/// Test response metadata is preserved
#[tokio::test]
async fn test_response_metadata_preservation() {
    struct ResponseMetadataHandler;
    impl spikard_http::grpc::GrpcHandler for ResponseMetadataHandler {
        fn call(
            &self,
            _request: spikard_http::grpc::GrpcRequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard_http::grpc::GrpcHandlerResult> + Send>>
        {
            let mut response_metadata = MetadataMap::new();
            response_metadata.insert("x-response-header", "response-value".parse().unwrap());

            Box::pin(async {
                Ok(spikard_http::grpc::GrpcResponseData {
                    payload: Bytes::from(r#"{"status": "ok"}"#),
                    metadata: response_metadata,
                })
            })
        }
        fn service_name(&self) -> &'static str {
            "test.ResponseMetadataService"
        }
    }

    let server = GrpcTestServer::new();

    server.register_service(Arc::new(ResponseMetadataHandler));

    let response = send_unary_request(
        &server,
        "test.ResponseMetadataService",
        "Check",
        Bytes::from("{}"),
        create_test_metadata(),
    )
    .await
    .expect("Failed to get response with metadata");

    // Verify metadata is in response
    assert!(!response.metadata.is_empty());
    assert!(response.metadata.get("x-response-header").is_some());
}

/// Test Bearer token format
#[test]
fn test_bearer_token_format() {
    let mut metadata = create_test_metadata();
    let token = "my_secret_token_xyz";

    add_auth_metadata(&mut metadata, token).expect("Failed to add auth");

    let auth_header = metadata.get("authorization").expect("Missing authorization");
    let auth_str = auth_header.to_str().unwrap();

    assert!(auth_str.starts_with("Bearer "));
    assert!(auth_str.contains("my_secret_token_xyz"));
    assert_eq!(auth_str, "Bearer my_secret_token_xyz");
}

/// Test metadata extraction with no headers
#[tokio::test]
async fn test_metadata_extraction_empty_metadata() {
    struct EmptyMetadataHandler;
    impl spikard_http::grpc::GrpcHandler for EmptyMetadataHandler {
        fn call(
            &self,
            request: spikard_http::grpc::GrpcRequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard_http::grpc::GrpcHandlerResult> + Send>>
        {
            let is_empty = request.metadata.is_empty();

            let response = if is_empty {
                Bytes::from(r#"{"metadata_empty": true}"#)
            } else {
                Bytes::from(r#"{"metadata_empty": false}"#)
            };

            Box::pin(async move {
                Ok(spikard_http::grpc::GrpcResponseData {
                    payload: response,
                    metadata: MetadataMap::new(),
                })
            })
        }
        fn service_name(&self) -> &'static str {
            "test.EmptyMetadataService"
        }
    }

    let server = GrpcTestServer::new();

    server.register_service(Arc::new(EmptyMetadataHandler));

    let empty_metadata = MetadataMap::new();

    let response = send_unary_request(
        &server,
        "test.EmptyMetadataService",
        "Check",
        Bytes::from("{}"),
        empty_metadata,
    )
    .await
    .expect("Failed to send with empty metadata");

    assert_grpc_response(&response, &serde_json::json!({"metadata_empty": true}));
}

/// Test metadata header with numeric value
#[tokio::test]
async fn test_metadata_numeric_value() {
    struct NumericHeaderHandler;
    impl spikard_http::grpc::GrpcHandler for NumericHeaderHandler {
        fn call(
            &self,
            request: spikard_http::grpc::GrpcRequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard_http::grpc::GrpcHandlerResult> + Send>>
        {
            let count = request
                .metadata
                .get("x-count")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("0")
                .to_string();

            let response = format!(r#"{{"count": {count}}}"#);

            Box::pin(async move {
                Ok(spikard_http::grpc::GrpcResponseData {
                    payload: Bytes::from(response),
                    metadata: MetadataMap::new(),
                })
            })
        }
        fn service_name(&self) -> &'static str {
            "test.NumericHeaderService"
        }
    }

    let server = GrpcTestServer::new();

    server.register_service(Arc::new(NumericHeaderHandler));

    let mut metadata = create_test_metadata();
    add_metadata_header(&mut metadata, "x-count", "42").unwrap();

    let response = send_unary_request(
        &server,
        "test.NumericHeaderService",
        "Process",
        Bytes::from("{}"),
        metadata,
    )
    .await
    .expect("Failed to send numeric header");

    assert_grpc_response(&response, &serde_json::json!({"count": 42}));
}

/// Test metadata with UUID value
#[tokio::test]
async fn test_metadata_uuid_value() {
    struct UuidHeaderHandler;
    impl spikard_http::grpc::GrpcHandler for UuidHeaderHandler {
        fn call(
            &self,
            request: spikard_http::grpc::GrpcRequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard_http::grpc::GrpcHandlerResult> + Send>>
        {
            let uuid = request
                .metadata
                .get("x-request-uuid")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("invalid")
                .to_string();

            let response = format!(r#"{{"uuid": "{uuid}"}}"#);

            Box::pin(async move {
                Ok(spikard_http::grpc::GrpcResponseData {
                    payload: Bytes::from(response),
                    metadata: MetadataMap::new(),
                })
            })
        }
        fn service_name(&self) -> &'static str {
            "test.UuidService"
        }
    }

    let server = GrpcTestServer::new();

    server.register_service(Arc::new(UuidHeaderHandler));

    let mut metadata = create_test_metadata();
    let uuid = "550e8400-e29b-41d4-a716-446655440000";
    add_metadata_header(&mut metadata, "x-request-uuid", uuid).unwrap();

    let response = send_unary_request(&server, "test.UuidService", "Process", Bytes::from("{}"), metadata)
        .await
        .expect("Failed to send UUID header");

    assert_grpc_response(
        &response,
        &serde_json::json!({
            "uuid": "550e8400-e29b-41d4-a716-446655440000"
        }),
    );
}

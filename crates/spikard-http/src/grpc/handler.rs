//! Core GrpcHandler trait for language-agnostic gRPC request handling
//!
//! This module defines the handler trait that language bindings implement
//! to handle gRPC requests. Similar to the HttpHandler pattern but designed
//! specifically for gRPC's protobuf-based message format.

use bytes::Bytes;
use std::future::Future;
use std::pin::Pin;
use tonic::metadata::MetadataMap;

/// gRPC request data passed to handlers
///
/// Contains the parsed components of a gRPC request:
/// - Service and method names from the request path
/// - Serialized protobuf payload as bytes
/// - Request metadata (headers)
#[derive(Debug, Clone)]
pub struct GrpcRequestData {
    /// Fully qualified service name (e.g., "mypackage.MyService")
    pub service_name: String,
    /// Method name (e.g., "GetUser")
    pub method_name: String,
    /// Serialized protobuf message bytes
    pub payload: Bytes,
    /// gRPC metadata (similar to HTTP headers)
    pub metadata: MetadataMap,
}

/// gRPC response data returned by handlers
///
/// Contains the serialized protobuf response and any metadata to include
/// in the response headers.
#[derive(Debug, Clone)]
pub struct GrpcResponseData {
    /// Serialized protobuf message bytes
    pub payload: Bytes,
    /// gRPC metadata to include in response (similar to HTTP headers)
    pub metadata: MetadataMap,
}

/// Result type for gRPC handlers
///
/// Returns either:
/// - Ok(GrpcResponseData): A successful response with payload and metadata
/// - Err(tonic::Status): A gRPC error status with code and message
pub type GrpcHandlerResult = Result<GrpcResponseData, tonic::Status>;

/// Handler trait for gRPC requests
///
/// This is the language-agnostic interface that all gRPC handler implementations
/// must satisfy. Language bindings (Python, TypeScript, Ruby, PHP) will implement
/// this trait to bridge their runtime to Spikard's gRPC server.
///
/// # Example
///
/// ```ignore
/// use spikard_http::grpc::{GrpcHandler, GrpcRequestData, GrpcHandlerResult};
/// use std::pin::Pin;
/// use std::future::Future;
///
/// struct MyGrpcHandler;
///
/// impl GrpcHandler for MyGrpcHandler {
///     fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
///         Box::pin(async move {
///             // Deserialize request.payload using protobuf
///             // Process the request
///             // Serialize response using protobuf
///             // Return GrpcResponseData
///             Ok(GrpcResponseData {
///                 payload: bytes::Bytes::from("serialized response"),
///                 metadata: tonic::metadata::MetadataMap::new(),
///             })
///         })
///     }
/// }
/// ```
pub trait GrpcHandler: Send + Sync {
    /// Handle a gRPC request
    ///
    /// Takes the parsed request data and returns a future that resolves to either:
    /// - Ok(GrpcResponseData): A successful response
    /// - Err(tonic::Status): An error with appropriate gRPC status code
    ///
    /// # Arguments
    ///
    /// * `request` - The parsed gRPC request containing service/method names,
    ///   serialized payload, and metadata
    ///
    /// # Returns
    ///
    /// A future that resolves to a GrpcHandlerResult
    fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>>;

    /// Get the fully qualified service name this handler serves
    ///
    /// This is used for routing requests to the appropriate handler.
    /// Should return the fully qualified service name as defined in the .proto file.
    ///
    /// # Example
    ///
    /// For a service defined as:
    /// ```proto
    /// package mypackage;
    /// service UserService { ... }
    /// ```
    ///
    /// This should return "mypackage.UserService"
    fn service_name(&self) -> &'static str;

    /// Whether this handler supports streaming requests
    ///
    /// If true, the handler can receive multiple request messages in sequence.
    /// Default implementation returns false (unary requests only).
    fn supports_streaming_requests(&self) -> bool {
        false
    }

    /// Whether this handler supports streaming responses
    ///
    /// If true, the handler can send multiple response messages in sequence.
    /// Default implementation returns false (unary responses only).
    fn supports_streaming_responses(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestGrpcHandler;

    impl GrpcHandler for TestGrpcHandler {
        fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
            Box::pin(async {
                Ok(GrpcResponseData {
                    payload: Bytes::from("test response"),
                    metadata: MetadataMap::new(),
                })
            })
        }

        fn service_name(&self) -> &'static str {
            "test.TestService"
        }
    }

    #[tokio::test]
    async fn test_grpc_handler_basic_call() {
        let handler = TestGrpcHandler;
        let request = GrpcRequestData {
            service_name: "test.TestService".to_string(),
            method_name: "TestMethod".to_string(),
            payload: Bytes::from("test payload"),
            metadata: MetadataMap::new(),
        };

        let result = handler.call(request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.payload, Bytes::from("test response"));
    }

    #[test]
    fn test_grpc_handler_service_name() {
        let handler = TestGrpcHandler;
        assert_eq!(handler.service_name(), "test.TestService");
    }

    #[test]
    fn test_grpc_handler_default_streaming_support() {
        let handler = TestGrpcHandler;
        assert!(!handler.supports_streaming_requests());
        assert!(!handler.supports_streaming_responses());
    }

    #[test]
    fn test_grpc_request_data_creation() {
        let request = GrpcRequestData {
            service_name: "mypackage.MyService".to_string(),
            method_name: "GetUser".to_string(),
            payload: Bytes::from("payload"),
            metadata: MetadataMap::new(),
        };

        assert_eq!(request.service_name, "mypackage.MyService");
        assert_eq!(request.method_name, "GetUser");
        assert_eq!(request.payload, Bytes::from("payload"));
    }

    #[test]
    fn test_grpc_response_data_creation() {
        let response = GrpcResponseData {
            payload: Bytes::from("response"),
            metadata: MetadataMap::new(),
        };

        assert_eq!(response.payload, Bytes::from("response"));
        assert!(response.metadata.is_empty());
    }

    #[test]
    fn test_grpc_request_data_clone() {
        let original = GrpcRequestData {
            service_name: "test.Service".to_string(),
            method_name: "Method".to_string(),
            payload: Bytes::from("data"),
            metadata: MetadataMap::new(),
        };

        let cloned = original.clone();
        assert_eq!(original.service_name, cloned.service_name);
        assert_eq!(original.method_name, cloned.method_name);
        assert_eq!(original.payload, cloned.payload);
    }

    #[test]
    fn test_grpc_response_data_clone() {
        let original = GrpcResponseData {
            payload: Bytes::from("response data"),
            metadata: MetadataMap::new(),
        };

        let cloned = original.clone();
        assert_eq!(original.payload, cloned.payload);
    }

    #[tokio::test]
    async fn test_grpc_handler_error_response() {
        struct ErrorHandler;

        impl GrpcHandler for ErrorHandler {
            fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async { Err(tonic::Status::not_found("Resource not found")) })
            }

            fn service_name(&self) -> &'static str {
                "test.ErrorService"
            }
        }

        let handler = ErrorHandler;
        let request = GrpcRequestData {
            service_name: "test.ErrorService".to_string(),
            method_name: "ErrorMethod".to_string(),
            payload: Bytes::new(),
            metadata: MetadataMap::new(),
        };

        let result = handler.call(request).await;
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code(), tonic::Code::NotFound);
        assert_eq!(error.message(), "Resource not found");
    }
}

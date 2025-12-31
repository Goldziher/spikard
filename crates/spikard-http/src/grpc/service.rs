//! Tonic service bridge
//!
//! This module bridges Tonic's service traits with our GrpcHandler trait.
//! It handles the conversion between Tonic's types and our internal representation,
//! enabling language-agnostic gRPC handling.

use crate::grpc::handler::{GrpcHandler, GrpcHandlerResult, GrpcRequestData, GrpcResponseData};
use bytes::Bytes;
use std::sync::Arc;
use tonic::{Request, Response, Status};

/// Generic gRPC service that routes requests to a GrpcHandler
///
/// This service implements Tonic's server traits and routes all requests
/// to the provided GrpcHandler implementation. It handles serialization
/// at the boundary between Tonic and our handler trait.
///
/// # Example
///
/// ```ignore
/// use spikard_http::grpc::service::GenericGrpcService;
/// use std::sync::Arc;
///
/// let handler = Arc::new(MyGrpcHandler);
/// let service = GenericGrpcService::new(handler);
/// ```
pub struct GenericGrpcService {
    handler: Arc<dyn GrpcHandler>,
}

impl GenericGrpcService {
    /// Create a new generic gRPC service with the given handler
    pub fn new(handler: Arc<dyn GrpcHandler>) -> Self {
        Self { handler }
    }

    /// Handle a unary RPC call
    ///
    /// Converts the Tonic Request into our GrpcRequestData format,
    /// calls the handler, and converts the result back to a Tonic Response.
    ///
    /// # Arguments
    ///
    /// * `service_name` - Fully qualified service name
    /// * `method_name` - Method name
    /// * `request` - Tonic request containing the serialized protobuf message
    pub async fn handle_unary(
        &self,
        service_name: String,
        method_name: String,
        request: Request<Bytes>,
    ) -> Result<Response<Bytes>, Status> {
        // Extract metadata and payload from Tonic request
        let (metadata, _extensions, payload) = request.into_parts();

        // Create our internal request representation
        let grpc_request = GrpcRequestData {
            service_name,
            method_name,
            payload,
            metadata,
        };

        // Call the handler
        let result: GrpcHandlerResult = self.handler.call(grpc_request).await;

        // Convert result to Tonic response
        match result {
            Ok(grpc_response) => {
                let mut response = Response::new(grpc_response.payload);
                copy_metadata(&grpc_response.metadata, response.metadata_mut());
                Ok(response)
            }
            Err(status) => Err(status),
        }
    }

    /// Get the service name from the handler
    pub fn service_name(&self) -> &str {
        self.handler.service_name()
    }
}

/// Helper function to parse gRPC path into service and method names
///
/// gRPC paths follow the format: `/<package>.<service>/<method>`
///
/// # Example
///
/// ```ignore
/// use spikard_http::grpc::service::parse_grpc_path;
///
/// let (service, method) = parse_grpc_path("/mypackage.UserService/GetUser").unwrap();
/// assert_eq!(service, "mypackage.UserService");
/// assert_eq!(method, "GetUser");
/// ```
pub fn parse_grpc_path(path: &str) -> Result<(String, String), Status> {
    // gRPC paths are in the format: /<package>.<service>/<method>
    let path = path.trim_start_matches('/');
    let parts: Vec<&str> = path.split('/').collect();

    if parts.len() != 2 {
        return Err(Status::invalid_argument(format!("Invalid gRPC path: {}", path)));
    }

    let service_name = parts[0].to_string();
    let method_name = parts[1].to_string();

    if service_name.is_empty() || method_name.is_empty() {
        return Err(Status::invalid_argument("Service or method name is empty"));
    }

    Ok((service_name, method_name))
}

/// Check if a request is a gRPC request
///
/// Checks the content-type header for "application/grpc" prefix.
///
/// # Example
///
/// ```ignore
/// use spikard_http::grpc::service::is_grpc_request;
/// use axum::http::HeaderMap;
///
/// let mut headers = HeaderMap::new();
/// headers.insert("content-type", "application/grpc".parse().unwrap());
///
/// assert!(is_grpc_request(&headers));
/// ```
pub fn is_grpc_request(headers: &axum::http::HeaderMap) -> bool {
    headers
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.starts_with("application/grpc"))
        .unwrap_or(false)
}

/// Copy metadata from source to destination MetadataMap
///
/// Efficiently copies all metadata entries (both ASCII and binary)
/// from one MetadataMap to another without unnecessary allocations.
///
/// # Arguments
///
/// * `source` - Source metadata to copy from
/// * `dest` - Destination metadata to copy into
pub fn copy_metadata(source: &tonic::metadata::MetadataMap, dest: &mut tonic::metadata::MetadataMap) {
    for key_value in source.iter() {
        match key_value {
            tonic::metadata::KeyAndValueRef::Ascii(key, value) => {
                dest.insert(key, value.clone());
            }
            tonic::metadata::KeyAndValueRef::Binary(key, value) => {
                dest.insert_bin(key, value.clone());
            }
        }
    }
}

/// Convert GrpcResponseData to Tonic Response
///
/// Helper function to convert our internal response representation
/// to a Tonic Response.
pub fn grpc_response_to_tonic(response: GrpcResponseData) -> Response<Bytes> {
    let mut tonic_response = Response::new(response.payload);
    copy_metadata(&response.metadata, tonic_response.metadata_mut());
    tonic_response
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grpc::handler::GrpcHandler;
    use std::future::Future;
    use std::pin::Pin;
    use tonic::metadata::MetadataMap;

    struct TestHandler;

    impl GrpcHandler for TestHandler {
        fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
            Box::pin(async move {
                // Echo back the request payload
                Ok(GrpcResponseData {
                    payload: request.payload,
                    metadata: MetadataMap::new(),
                })
            })
        }

        fn service_name(&self) -> &'static str {
            "test.TestService"
        }
    }

    #[tokio::test]
    async fn test_generic_grpc_service_handle_unary() {
        let handler = Arc::new(TestHandler);
        let service = GenericGrpcService::new(handler);

        let request = Request::new(Bytes::from("test payload"));
        let result = service.handle_unary("test.TestService".to_string(), "TestMethod".to_string(), request).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.into_inner(), Bytes::from("test payload"));
    }

    #[tokio::test]
    async fn test_generic_grpc_service_with_metadata() {
        let handler = Arc::new(TestHandler);
        let service = GenericGrpcService::new(handler);

        let mut request = Request::new(Bytes::from("payload"));
        request
            .metadata_mut()
            .insert("custom-header", "custom-value".parse().unwrap());

        let result = service.handle_unary("test.TestService".to_string(), "TestMethod".to_string(), request).await;

        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_grpc_path_valid() {
        let (service, method) = parse_grpc_path("/mypackage.UserService/GetUser").unwrap();
        assert_eq!(service, "mypackage.UserService");
        assert_eq!(method, "GetUser");
    }

    #[test]
    fn test_parse_grpc_path_with_nested_package() {
        let (service, method) = parse_grpc_path("/com.example.api.v1.UserService/GetUser").unwrap();
        assert_eq!(service, "com.example.api.v1.UserService");
        assert_eq!(method, "GetUser");
    }

    #[test]
    fn test_parse_grpc_path_invalid_format() {
        let result = parse_grpc_path("/invalid");
        assert!(result.is_err());
        let status = result.unwrap_err();
        assert_eq!(status.code(), tonic::Code::InvalidArgument);
    }

    #[test]
    fn test_parse_grpc_path_empty_service() {
        let result = parse_grpc_path("//Method");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_grpc_path_empty_method() {
        let result = parse_grpc_path("/Service/");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_grpc_path_no_leading_slash() {
        let (service, method) = parse_grpc_path("package.Service/Method").unwrap();
        assert_eq!(service, "package.Service");
        assert_eq!(method, "Method");
    }

    #[test]
    fn test_is_grpc_request_valid() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            "application/grpc".parse().unwrap(),
        );
        assert!(is_grpc_request(&headers));
    }

    #[test]
    fn test_is_grpc_request_with_subtype() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            "application/grpc+proto".parse().unwrap(),
        );
        assert!(is_grpc_request(&headers));
    }

    #[test]
    fn test_is_grpc_request_not_grpc() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        assert!(!is_grpc_request(&headers));
    }

    #[test]
    fn test_is_grpc_request_no_content_type() {
        let headers = axum::http::HeaderMap::new();
        assert!(!is_grpc_request(&headers));
    }

    #[test]
    fn test_grpc_response_to_tonic_basic() {
        let response = GrpcResponseData {
            payload: Bytes::from("response"),
            metadata: MetadataMap::new(),
        };

        let tonic_response = grpc_response_to_tonic(response);
        assert_eq!(tonic_response.into_inner(), Bytes::from("response"));
    }

    #[test]
    fn test_grpc_response_to_tonic_with_metadata() {
        let mut metadata = MetadataMap::new();
        metadata.insert("custom-header", "value".parse().unwrap());

        let response = GrpcResponseData {
            payload: Bytes::from("data"),
            metadata,
        };

        let tonic_response = grpc_response_to_tonic(response);
        assert_eq!(tonic_response.get_ref(), &Bytes::from("data"));
        assert!(tonic_response.metadata().get("custom-header").is_some());
    }

    #[test]
    fn test_generic_grpc_service_service_name() {
        let handler = Arc::new(TestHandler);
        let service = GenericGrpcService::new(handler);
        assert_eq!(service.service_name(), "test.TestService");
    }

    #[test]
    fn test_copy_metadata() {
        let mut source = MetadataMap::new();
        source.insert("key1", "value1".parse().unwrap());
        source.insert("key2", "value2".parse().unwrap());

        let mut dest = MetadataMap::new();
        copy_metadata(&source, &mut dest);

        assert_eq!(dest.get("key1").unwrap(), "value1");
        assert_eq!(dest.get("key2").unwrap(), "value2");
    }

    #[test]
    fn test_copy_metadata_empty() {
        let source = MetadataMap::new();
        let mut dest = MetadataMap::new();
        copy_metadata(&source, &mut dest);
        assert!(dest.is_empty());
    }

    #[test]
    fn test_copy_metadata_binary() {
        let mut source = MetadataMap::new();
        source.insert_bin("binary-key-bin", tonic::metadata::MetadataValue::from_bytes(b"binary"));

        let mut dest = MetadataMap::new();
        copy_metadata(&source, &mut dest);

        assert!(dest.get_bin("binary-key-bin").is_some());
    }

    #[tokio::test]
    async fn test_generic_grpc_service_error_handling() {
        struct ErrorHandler;

        impl GrpcHandler for ErrorHandler {
            fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async { Err(Status::not_found("Resource not found")) })
            }

            fn service_name(&self) -> &'static str {
                "test.ErrorService"
            }
        }

        let handler = Arc::new(ErrorHandler);
        let service = GenericGrpcService::new(handler);

        let request = Request::new(Bytes::new());
        let result = service.handle_unary("test.ErrorService".to_string(), "ErrorMethod".to_string(), request).await;

        assert!(result.is_err());
        let status = result.unwrap_err();
        assert_eq!(status.code(), tonic::Code::NotFound);
        assert_eq!(status.message(), "Resource not found");
    }
}

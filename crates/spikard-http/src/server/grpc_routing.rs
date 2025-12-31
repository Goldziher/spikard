//! gRPC request routing and multiplexing
//!
//! This module handles routing gRPC requests to the appropriate handlers
//! and multiplexing between HTTP/1.1 REST and HTTP/2 gRPC traffic.

use crate::grpc::{GrpcRegistry, parse_grpc_path};
use axum::body::Body;
use axum::http::{Request, Response, StatusCode};
use std::sync::Arc;

/// Convert gRPC status code to HTTP status code
///
/// Maps all gRPC status codes to appropriate HTTP status codes
/// following the gRPC-HTTP status code mapping specification.
fn grpc_status_to_http(code: tonic::Code) -> StatusCode {
    match code {
        tonic::Code::Ok => StatusCode::OK,
        tonic::Code::Cancelled => StatusCode::from_u16(499).unwrap(), // Client Closed Request
        tonic::Code::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        tonic::Code::InvalidArgument => StatusCode::BAD_REQUEST,
        tonic::Code::DeadlineExceeded => StatusCode::GATEWAY_TIMEOUT,
        tonic::Code::NotFound => StatusCode::NOT_FOUND,
        tonic::Code::AlreadyExists => StatusCode::CONFLICT,
        tonic::Code::PermissionDenied => StatusCode::FORBIDDEN,
        tonic::Code::ResourceExhausted => StatusCode::TOO_MANY_REQUESTS,
        tonic::Code::FailedPrecondition => StatusCode::BAD_REQUEST,
        tonic::Code::Aborted => StatusCode::CONFLICT,
        tonic::Code::OutOfRange => StatusCode::BAD_REQUEST,
        tonic::Code::Unimplemented => StatusCode::NOT_IMPLEMENTED,
        tonic::Code::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        tonic::Code::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
        tonic::Code::DataLoss => StatusCode::INTERNAL_SERVER_ERROR,
        tonic::Code::Unauthenticated => StatusCode::UNAUTHORIZED,
    }
}

/// Route a gRPC request to the appropriate handler
///
/// Parses the request path to extract service and method names,
/// looks up the handler in the registry, and invokes it.
///
/// # Arguments
///
/// * `registry` - gRPC handler registry
/// * `request` - The incoming gRPC request
///
/// # Returns
///
/// A future that resolves to a gRPC response or error
pub async fn route_grpc_request(
    registry: Arc<GrpcRegistry>,
    request: Request<Body>,
) -> Result<Response<Body>, (StatusCode, String)> {
    // Extract the request path
    let path = request.uri().path();

    // Parse service and method names from the path
    let (service_name, method_name) = match parse_grpc_path(path) {
        Ok(names) => names,
        Err(status) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Invalid gRPC path: {}", status.message()),
            ));
        }
    };

    // Look up the handler for this service
    let handler = match registry.get(&service_name) {
        Some(h) => h,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                format!("Service not found: {}", service_name),
            ));
        }
    };

    // Convert the Axum request to bytes
    let (parts, body) = request.into_parts();
    let body_bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(e) => {
            return Err((StatusCode::BAD_REQUEST, format!("Failed to read request body: {}", e)));
        }
    };

    // Create a Tonic request
    let mut tonic_request = tonic::Request::new(body_bytes);

    // Copy headers to Tonic metadata
    for (key, value) in parts.headers.iter() {
        if let Ok(value_str) = value.to_str() {
            // Try to parse as ASCII metadata
            if let Ok(metadata_value) = value_str.parse::<tonic::metadata::MetadataValue<tonic::metadata::Ascii>>() {
                // Use key.as_str() directly instead of creating String
                if let Ok(metadata_key) = key.as_str().parse::<tonic::metadata::MetadataKey<tonic::metadata::Ascii>>() {
                    tonic_request.metadata_mut().insert(metadata_key, metadata_value);
                }
            }
        }
    }

    // Use the service bridge to handle the request
    let service = crate::grpc::GenericGrpcService::new(handler);
    let tonic_response = match service.handle_unary(service_name, method_name, tonic_request).await {
        Ok(resp) => resp,
        Err(status) => {
            let status_code = grpc_status_to_http(status.code());
            return Err((status_code, status.message().to_string()));
        }
    };

    // Convert Tonic response to Axum response
    let payload = tonic_response.get_ref().clone();
    let metadata = tonic_response.metadata();

    let mut response = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/grpc+proto");

    // Copy metadata to response headers
    for key_value in metadata.iter() {
        if let tonic::metadata::KeyAndValueRef::Ascii(key, value) = key_value
            && let Ok(header_value) = axum::http::HeaderValue::from_str(value.to_str().unwrap_or(""))
        {
            response = response.header(key.as_str(), header_value);
        }
    }

    // Add gRPC status trailer (success)
    response = response.header("grpc-status", "0");

    // Convert bytes::Bytes to Body
    let response = response
        .body(Body::from(payload))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to build response: {}", e)))?;

    Ok(response)
}

/// Check if an incoming request is a gRPC request
///
/// Returns true if the request has a content-type starting with "application/grpc"
pub fn is_grpc_request(request: &Request<Body>) -> bool {
    request
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.starts_with("application/grpc"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grpc::handler::{GrpcHandler, GrpcHandlerResult, GrpcRequestData, GrpcResponseData};
    use bytes::Bytes;
    use std::future::Future;
    use std::pin::Pin;

    struct EchoHandler;

    impl GrpcHandler for EchoHandler {
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

    #[tokio::test]
    async fn test_route_grpc_request_success() {
        let mut registry = GrpcRegistry::new();
        registry.register("test.EchoService", Arc::new(EchoHandler));
        let registry = Arc::new(registry);

        let request = Request::builder()
            .uri("/test.EchoService/Echo")
            .header("content-type", "application/grpc")
            .body(Body::from(Bytes::from("test payload")))
            .unwrap();

        let result = route_grpc_request(registry, request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_route_grpc_request_service_not_found() {
        let registry = Arc::new(GrpcRegistry::new());

        let request = Request::builder()
            .uri("/nonexistent.Service/Method")
            .header("content-type", "application/grpc")
            .body(Body::from(Bytes::new()))
            .unwrap();

        let result = route_grpc_request(registry, request).await;
        assert!(result.is_err());

        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(message.contains("Service not found"));
    }

    #[tokio::test]
    async fn test_route_grpc_request_invalid_path() {
        let registry = Arc::new(GrpcRegistry::new());

        let request = Request::builder()
            .uri("/invalid")
            .header("content-type", "application/grpc")
            .body(Body::from(Bytes::new()))
            .unwrap();

        let result = route_grpc_request(registry, request).await;
        assert!(result.is_err());

        let (status, _message) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_is_grpc_request_true() {
        let request = Request::builder()
            .header("content-type", "application/grpc")
            .body(Body::empty())
            .unwrap();

        assert!(is_grpc_request(&request));
    }

    #[test]
    fn test_is_grpc_request_with_subtype() {
        let request = Request::builder()
            .header("content-type", "application/grpc+proto")
            .body(Body::empty())
            .unwrap();

        assert!(is_grpc_request(&request));
    }

    #[test]
    fn test_is_grpc_request_false() {
        let request = Request::builder()
            .header("content-type", "application/json")
            .body(Body::empty())
            .unwrap();

        assert!(!is_grpc_request(&request));
    }

    #[test]
    fn test_is_grpc_request_no_content_type() {
        let request = Request::builder().body(Body::empty()).unwrap();

        assert!(!is_grpc_request(&request));
    }

    #[test]
    fn test_grpc_status_to_http_mappings() {
        // Test all gRPC status codes map correctly
        assert_eq!(grpc_status_to_http(tonic::Code::Ok), StatusCode::OK);
        assert_eq!(grpc_status_to_http(tonic::Code::Cancelled), StatusCode::from_u16(499).unwrap());
        assert_eq!(grpc_status_to_http(tonic::Code::Unknown), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(grpc_status_to_http(tonic::Code::InvalidArgument), StatusCode::BAD_REQUEST);
        assert_eq!(grpc_status_to_http(tonic::Code::DeadlineExceeded), StatusCode::GATEWAY_TIMEOUT);
        assert_eq!(grpc_status_to_http(tonic::Code::NotFound), StatusCode::NOT_FOUND);
        assert_eq!(grpc_status_to_http(tonic::Code::AlreadyExists), StatusCode::CONFLICT);
        assert_eq!(grpc_status_to_http(tonic::Code::PermissionDenied), StatusCode::FORBIDDEN);
        assert_eq!(grpc_status_to_http(tonic::Code::ResourceExhausted), StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(grpc_status_to_http(tonic::Code::FailedPrecondition), StatusCode::BAD_REQUEST);
        assert_eq!(grpc_status_to_http(tonic::Code::Aborted), StatusCode::CONFLICT);
        assert_eq!(grpc_status_to_http(tonic::Code::OutOfRange), StatusCode::BAD_REQUEST);
        assert_eq!(grpc_status_to_http(tonic::Code::Unimplemented), StatusCode::NOT_IMPLEMENTED);
        assert_eq!(grpc_status_to_http(tonic::Code::Internal), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(grpc_status_to_http(tonic::Code::Unavailable), StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(grpc_status_to_http(tonic::Code::DataLoss), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(grpc_status_to_http(tonic::Code::Unauthenticated), StatusCode::UNAUTHORIZED);
    }
}

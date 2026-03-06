//! gRPC request routing and multiplexing
//!
//! This module handles routing gRPC requests to the appropriate handlers
//! and multiplexing between HTTP/1.1 REST and HTTP/2 gRPC traffic.

use crate::grpc::{GrpcConfig, GrpcRegistry, RpcMode, parse_grpc_path};
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
/// * `config` - gRPC configuration with size limits
/// * `request` - The incoming gRPC request
///
/// # Returns
///
/// A future that resolves to a gRPC response or error
pub async fn route_grpc_request(
    registry: Arc<GrpcRegistry>,
    config: &GrpcConfig,
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

    // Look up the handler for this service and method
    let (handler, rpc_mode) = match registry.get(&service_name, &method_name) {
        Some((h, mode)) => (h, mode),
        None => {
            let message = if registry.contains_service(&service_name) {
                format!("Method not found: {service_name}/{method_name}")
            } else {
                format!("Service not found: {service_name}")
            };
            return Err((StatusCode::NOT_FOUND, message));
        }
    };

    // Create the service bridge
    let service = crate::grpc::GenericGrpcService::new(handler);

    // Dispatch based on RPC mode
    match rpc_mode {
        RpcMode::Unary => {
            handle_unary_request(service, service_name, method_name, config.max_message_size, request).await
        }
        RpcMode::ServerStreaming => {
            handle_server_streaming_request(service, service_name, method_name, config.max_message_size, request).await
        }
        RpcMode::ClientStreaming => {
            handle_client_streaming_request(
                service,
                service_name,
                method_name,
                config.max_message_size,
                config.enable_compression,
                request,
            )
            .await
        }
        RpcMode::BidirectionalStreaming => {
            handle_bidirectional_streaming_request(
                service,
                service_name,
                method_name,
                config.max_message_size,
                config.enable_compression,
                request,
            )
            .await
        }
    }
}

/// Handle a unary RPC request
async fn handle_unary_request(
    service: crate::grpc::GenericGrpcService,
    service_name: String,
    method_name: String,
    max_message_size: usize,
    request: Request<Body>,
) -> Result<Response<Body>, (StatusCode, String)> {
    // Convert the Axum request to bytes with the configured size limit
    let (parts, body) = request.into_parts();
    let body_bytes = match axum::body::to_bytes(body, max_message_size).await {
        Ok(bytes) => bytes,
        Err(e) => {
            // Check if error is due to size limit (axum returns "body size exceeded" or similar)
            let error_msg = e.to_string();
            if error_msg.contains("body") || error_msg.contains("size") || error_msg.contains("exceeded") {
                return Err((
                    StatusCode::PAYLOAD_TOO_LARGE,
                    format!("Message exceeds maximum size of {} bytes", max_message_size),
                ));
            }
            return Err((StatusCode::BAD_REQUEST, format!("Failed to read request body: {}", e)));
        }
    };

    // Create a Tonic request
    let mut tonic_request = tonic::Request::new(body_bytes);

    // Copy headers to Tonic metadata
    for (key, value) in parts.headers.iter() {
        if let Ok(value_str) = value.to_str()
            && let Ok(metadata_value) = value_str.parse::<tonic::metadata::MetadataValue<tonic::metadata::Ascii>>()
            && let Ok(metadata_key) = key
                .as_str()
                .parse::<tonic::metadata::MetadataKey<tonic::metadata::Ascii>>()
        {
            tonic_request.metadata_mut().insert(metadata_key, metadata_value);
        }
    }

    // Use the service bridge to handle the request
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
    let response = response.body(Body::from(payload)).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to build response: {}", e),
        )
    })?;

    Ok(response)
}

/// Handle a server streaming RPC request
async fn handle_server_streaming_request(
    service: crate::grpc::GenericGrpcService,
    service_name: String,
    method_name: String,
    max_message_size: usize,
    request: Request<Body>,
) -> Result<Response<Body>, (StatusCode, String)> {
    // Convert the Axum request to bytes with the configured size limit
    let (parts, body) = request.into_parts();
    let body_bytes = match axum::body::to_bytes(body, max_message_size).await {
        Ok(bytes) => bytes,
        Err(e) => {
            // Check if error is due to size limit (axum returns "body size exceeded" or similar)
            let error_msg = e.to_string();
            if error_msg.contains("body") || error_msg.contains("size") || error_msg.contains("exceeded") {
                return Err((
                    StatusCode::PAYLOAD_TOO_LARGE,
                    format!("Message exceeds maximum size of {} bytes", max_message_size),
                ));
            }
            return Err((StatusCode::BAD_REQUEST, format!("Failed to read request body: {}", e)));
        }
    };

    // Create a Tonic request
    let mut tonic_request = tonic::Request::new(body_bytes);

    // Copy headers to Tonic metadata
    for (key, value) in parts.headers.iter() {
        if let Ok(value_str) = value.to_str()
            && let Ok(metadata_value) = value_str.parse::<tonic::metadata::MetadataValue<tonic::metadata::Ascii>>()
            && let Ok(metadata_key) = key
                .as_str()
                .parse::<tonic::metadata::MetadataKey<tonic::metadata::Ascii>>()
        {
            tonic_request.metadata_mut().insert(metadata_key, metadata_value);
        }
    }

    // Use the service bridge to handle the streaming request
    let tonic_response = match service
        .handle_server_stream(service_name, method_name, tonic_request)
        .await
    {
        Ok(resp) => resp,
        Err(status) => {
            let status_code = grpc_status_to_http(status.code());
            return Err((status_code, status.message().to_string()));
        }
    };

    // Convert Tonic response to Axum response
    let body = tonic_response.into_inner();
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/grpc+proto");

    let response = response.body(body).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to build response: {}", e),
        )
    })?;

    Ok(response)
}

/// Handle a client streaming RPC request
async fn handle_client_streaming_request(
    service: crate::grpc::GenericGrpcService,
    service_name: String,
    method_name: String,
    max_message_size: usize,
    compression_enabled: bool,
    request: Request<Body>,
) -> Result<Response<Body>, (StatusCode, String)> {
    // Extract request parts - keep body as stream for frame parsing
    let (parts, body) = request.into_parts();

    // Create a Tonic request with streaming body
    // Body will be parsed by service.handle_client_stream using frame parser
    let mut tonic_request = tonic::Request::new(body);

    // Copy headers to Tonic metadata
    for (key, value) in parts.headers.iter() {
        if let Ok(value_str) = value.to_str()
            && let Ok(metadata_value) = value_str.parse::<tonic::metadata::MetadataValue<tonic::metadata::Ascii>>()
            && let Ok(metadata_key) = key
                .as_str()
                .parse::<tonic::metadata::MetadataKey<tonic::metadata::Ascii>>()
        {
            tonic_request.metadata_mut().insert(metadata_key, metadata_value);
        }
    }

    // Use the service bridge to handle the client streaming request
    // Frame parsing and size validation happens in handle_client_stream
    let tonic_response = match service
        .handle_client_stream(
            service_name,
            method_name,
            tonic_request,
            max_message_size,
            compression_enabled,
        )
        .await
    {
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
    let response = response.body(Body::from(payload)).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to build response: {}", e),
        )
    })?;

    Ok(response)
}

/// Handle a bidirectional streaming RPC request
///
/// Bidirectional streaming allows both client and server to send multiple messages.
/// This function:
/// 1. Keeps the request body as a stream (not converting to bytes)
/// 2. Copies HTTP headers to gRPC metadata
/// 3. Passes the streaming body to the service for frame parsing
/// 4. Returns the response stream from the handler
///
/// # Arguments
///
/// * `service` - The GenericGrpcService containing the handler
/// * `service_name` - Fully qualified service name (e.g., "mypackage.ChatService")
/// * `method_name` - Method name (e.g., "Chat")
/// * `max_message_size` - Maximum size per message in bytes
/// * `compression_enabled` - Whether compressed gRPC request frames are accepted
/// * `request` - Axum HTTP request with streaming body
///
/// # Returns
///
/// Response with streaming body containing response messages, or error with status code
async fn handle_bidirectional_streaming_request(
    service: crate::grpc::GenericGrpcService,
    service_name: String,
    method_name: String,
    max_message_size: usize,
    compression_enabled: bool,
    request: Request<Body>,
) -> Result<Response<Body>, (StatusCode, String)> {
    // Extract request parts - keep body as stream for frame parsing
    let (parts, body) = request.into_parts();

    // Create a Tonic request with streaming body
    // Body will be parsed by service.handle_bidi_stream using frame parser
    let mut tonic_request = tonic::Request::new(body);

    // Copy HTTP headers to gRPC metadata
    for (key, value) in parts.headers.iter() {
        if let Ok(value_str) = value.to_str()
            && let Ok(metadata_value) = value_str.parse::<tonic::metadata::MetadataValue<tonic::metadata::Ascii>>()
            && let Ok(metadata_key) = key
                .as_str()
                .parse::<tonic::metadata::MetadataKey<tonic::metadata::Ascii>>()
        {
            tonic_request.metadata_mut().insert(metadata_key, metadata_value);
        }
    }

    // Call service handler - frame parsing and size validation happens inside
    let tonic_response = match service
        .handle_bidi_stream(
            service_name,
            method_name,
            tonic_request,
            max_message_size,
            compression_enabled,
        )
        .await
    {
        Ok(response) => response,
        Err(status) => {
            let status_code = grpc_status_to_http(status.code());
            return Err((status_code, status.message().to_string()));
        }
    };

    // Convert Tonic response to Axum response with streaming body
    let body = tonic_response.into_inner();
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/grpc+proto");

    let response = response.body(body).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to build response: {}", e),
        )
    })?;

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
    use crate::grpc::handler::{GrpcHandler, GrpcHandlerResult, GrpcRequestData, GrpcResponseData, RpcMode};
    use crate::grpc::streaming::{MessageStream, StreamingRequest, message_stream_from_vec};
    use bytes::Bytes;
    use flate2::{Compression, write::GzEncoder};
    use futures_util::StreamExt;
    use http_body_util::BodyExt;
    use std::future::Future;
    use std::io::Write;
    use std::pin::Pin;
    use tonic::metadata::MetadataMap;

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

        fn service_name(&self) -> &str {
            "test.EchoService"
        }
    }

    #[tokio::test]
    async fn test_route_grpc_request_success() {
        let mut registry = GrpcRegistry::new();
        registry.register_service("test.EchoService", Arc::new(EchoHandler), RpcMode::Unary);
        let registry = Arc::new(registry);
        let config = GrpcConfig::default();

        let request = Request::builder()
            .uri("/test.EchoService/Echo")
            .header("content-type", "application/grpc")
            .body(Body::from(Bytes::from("test payload")))
            .unwrap();

        let result = route_grpc_request(registry, &config, request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_route_grpc_request_supports_mixed_rpc_modes_per_service() {
        struct MixedUnaryHandler;

        impl GrpcHandler for MixedUnaryHandler {
            fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async move {
                    Ok(GrpcResponseData {
                        payload: request.payload,
                        metadata: tonic::metadata::MetadataMap::new(),
                    })
                })
            }

            fn service_name(&self) -> &str {
                "test.MixedService"
            }
        }

        struct MixedStreamHandler;

        impl GrpcHandler for MixedStreamHandler {
            fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async { Err(tonic::Status::unimplemented("Use server streaming")) })
            }

            fn service_name(&self) -> &str {
                "test.MixedService"
            }

            fn call_server_stream(
                &self,
                _request: GrpcRequestData,
            ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
                Box::pin(async { Ok(message_stream_from_vec(vec![Bytes::from_static(b"streamed")])) })
            }
        }

        let mut registry = GrpcRegistry::new();
        registry.register(
            "test.MixedService",
            "GetUser",
            Arc::new(MixedUnaryHandler),
            RpcMode::Unary,
        );
        registry.register(
            "test.MixedService",
            "ListUsers",
            Arc::new(MixedStreamHandler),
            RpcMode::ServerStreaming,
        );
        let registry = Arc::new(registry);
        let config = GrpcConfig::default();

        let unary_response = route_grpc_request(
            Arc::clone(&registry),
            &config,
            Request::builder()
                .uri("/test.MixedService/GetUser")
                .header("content-type", "application/grpc")
                .body(Body::from(Bytes::from_static(b"user")))
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(unary_response.headers().get("grpc-status").unwrap(), "0");

        let streaming_response = route_grpc_request(
            registry,
            &config,
            Request::builder()
                .uri("/test.MixedService/ListUsers")
                .header("content-type", "application/grpc")
                .body(Body::from(Bytes::from_static(b"ignored")))
                .unwrap(),
        )
        .await
        .unwrap();
        assert!(streaming_response.headers().get("grpc-status").is_none());
    }

    #[tokio::test]
    async fn test_route_grpc_request_service_not_found() {
        let registry = Arc::new(GrpcRegistry::new());
        let config = GrpcConfig::default();

        let request = Request::builder()
            .uri("/nonexistent.Service/Method")
            .header("content-type", "application/grpc")
            .body(Body::from(Bytes::new()))
            .unwrap();

        let result = route_grpc_request(registry, &config, request).await;
        assert!(result.is_err());

        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(message.contains("Service not found"));
    }

    #[tokio::test]
    async fn test_route_grpc_request_method_not_found() {
        let mut registry = GrpcRegistry::new();
        registry.register("test.EchoService", "Echo", Arc::new(EchoHandler), RpcMode::Unary);
        let registry = Arc::new(registry);
        let config = GrpcConfig::default();

        let request = Request::builder()
            .uri("/test.EchoService/Missing")
            .header("content-type", "application/grpc")
            .body(Body::from(Bytes::new()))
            .unwrap();

        let result = route_grpc_request(registry, &config, request).await;
        assert!(result.is_err());

        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(message.contains("Method not found"));
    }

    #[tokio::test]
    async fn test_route_grpc_request_invalid_path() {
        let registry = Arc::new(GrpcRegistry::new());
        let config = GrpcConfig::default();

        let request = Request::builder()
            .uri("/invalid")
            .header("content-type", "application/grpc")
            .body(Body::from(Bytes::new()))
            .unwrap();

        let result = route_grpc_request(registry, &config, request).await;
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
        assert_eq!(
            grpc_status_to_http(tonic::Code::Cancelled),
            StatusCode::from_u16(499).unwrap()
        );
        assert_eq!(
            grpc_status_to_http(tonic::Code::Unknown),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            grpc_status_to_http(tonic::Code::InvalidArgument),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            grpc_status_to_http(tonic::Code::DeadlineExceeded),
            StatusCode::GATEWAY_TIMEOUT
        );
        assert_eq!(grpc_status_to_http(tonic::Code::NotFound), StatusCode::NOT_FOUND);
        assert_eq!(grpc_status_to_http(tonic::Code::AlreadyExists), StatusCode::CONFLICT);
        assert_eq!(
            grpc_status_to_http(tonic::Code::PermissionDenied),
            StatusCode::FORBIDDEN
        );
        assert_eq!(
            grpc_status_to_http(tonic::Code::ResourceExhausted),
            StatusCode::TOO_MANY_REQUESTS
        );
        assert_eq!(
            grpc_status_to_http(tonic::Code::FailedPrecondition),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(grpc_status_to_http(tonic::Code::Aborted), StatusCode::CONFLICT);
        assert_eq!(grpc_status_to_http(tonic::Code::OutOfRange), StatusCode::BAD_REQUEST);
        assert_eq!(
            grpc_status_to_http(tonic::Code::Unimplemented),
            StatusCode::NOT_IMPLEMENTED
        );
        assert_eq!(
            grpc_status_to_http(tonic::Code::Internal),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            grpc_status_to_http(tonic::Code::Unavailable),
            StatusCode::SERVICE_UNAVAILABLE
        );
        assert_eq!(
            grpc_status_to_http(tonic::Code::DataLoss),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            grpc_status_to_http(tonic::Code::Unauthenticated),
            StatusCode::UNAUTHORIZED
        );
    }

    #[tokio::test]
    async fn test_route_grpc_request_with_custom_max_message_size() {
        let mut registry = GrpcRegistry::new();
        registry.register_service("test.EchoService", Arc::new(EchoHandler), RpcMode::Unary);
        let registry = Arc::new(registry);

        let mut config = GrpcConfig::default();
        config.max_message_size = 100; // Set small limit

        let request = Request::builder()
            .uri("/test.EchoService/Echo")
            .header("content-type", "application/grpc")
            .body(Body::from(Bytes::from("test payload")))
            .unwrap();

        // This should succeed since payload is small
        let result = route_grpc_request(registry.clone(), &config, request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_route_grpc_request_exceeds_max_message_size() {
        let mut registry = GrpcRegistry::new();
        registry.register_service("test.EchoService", Arc::new(EchoHandler), RpcMode::Unary);
        let registry = Arc::new(registry);

        let mut config = GrpcConfig::default();
        config.max_message_size = 10; // Set very small limit

        // Create a large payload that exceeds the limit
        let large_payload = vec![b'x'; 1000];
        let request = Request::builder()
            .uri("/test.EchoService/Echo")
            .header("content-type", "application/grpc")
            .body(Body::from(Bytes::from(large_payload)))
            .unwrap();

        let result = route_grpc_request(registry, &config, request).await;
        assert!(result.is_err());

        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
        assert!(message.contains("Message exceeds maximum size"));
    }

    #[tokio::test]
    async fn test_route_grpc_request_server_streaming_success() {
        struct StreamHandler;

        impl GrpcHandler for StreamHandler {
            fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async { Err(tonic::Status::unimplemented("Use server streaming")) })
            }

            fn service_name(&self) -> &str {
                "test.StreamService"
            }

            fn call_server_stream(
                &self,
                _request: GrpcRequestData,
            ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
                Box::pin(async {
                    let messages = vec![Bytes::from("m1"), Bytes::from("m2")];
                    Ok(message_stream_from_vec(messages))
                })
            }
        }

        let mut registry = GrpcRegistry::new();
        registry.register_service("test.StreamService", Arc::new(StreamHandler), RpcMode::ServerStreaming);
        let registry = Arc::new(registry);
        let config = GrpcConfig::default();

        let request = Request::builder()
            .uri("/test.StreamService/Stream")
            .header("content-type", "application/grpc")
            .body(Body::from(Bytes::from("ignored")))
            .unwrap();

        let response = route_grpc_request(registry, &config, request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "application/grpc+proto"
        );
        assert!(
            response.headers().get("grpc-status").is_none(),
            "server-streaming responses must not predeclare grpc-status"
        );

        let collected = response.into_body().collect().await.unwrap();
        let trailers = collected.trailers().expect("grpc trailers");
        let grpc_status = trailers.get("grpc-status").unwrap().to_str().unwrap().to_string();
        let grpc_message = trailers.get("grpc-message").unwrap().to_str().unwrap().to_string();
        let body = collected.to_bytes();
        assert!(!body.is_empty());
        assert_eq!(grpc_status, "0");
        assert_eq!(grpc_message, "OK");
    }

    #[tokio::test]
    async fn test_route_grpc_request_client_streaming_success_and_response_metadata() {
        struct ClientStreamHandler;

        impl GrpcHandler for ClientStreamHandler {
            fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async { Err(tonic::Status::unimplemented("Use client streaming")) })
            }

            fn service_name(&self) -> &str {
                "test.ClientStreamService"
            }

            fn call_client_stream(
                &self,
                mut request: StreamingRequest,
            ) -> Pin<Box<dyn Future<Output = Result<GrpcResponseData, tonic::Status>> + Send>> {
                Box::pin(async move {
                    // Make sure routing copied request headers into metadata.
                    assert!(request.metadata.get("x-request-id").is_some());

                    let mut first = None;
                    while let Some(item) = request.message_stream.next().await {
                        first = Some(item?);
                        break;
                    }

                    let payload = first.unwrap_or_else(Bytes::new);
                    let mut metadata = MetadataMap::new();
                    metadata.insert("x-response-id", "resp-123".parse().unwrap());

                    Ok(GrpcResponseData { payload, metadata })
                })
            }
        }

        let mut registry = GrpcRegistry::new();
        registry.register_service(
            "test.ClientStreamService",
            Arc::new(ClientStreamHandler),
            RpcMode::ClientStreaming,
        );
        let registry = Arc::new(registry);
        let config = GrpcConfig::default();

        // Single gRPC frame: compression=0, length=5, message="hello"
        let frame = vec![
            0x00, // compression: no
            0x00, 0x00, 0x00, 0x05, // length: 5 bytes
            b'h', b'e', b'l', b'l', b'o',
        ];

        let request = Request::builder()
            .uri("/test.ClientStreamService/ClientStream")
            .header("content-type", "application/grpc")
            .header("x-request-id", "req-123")
            .body(Body::from(frame))
            .unwrap();

        let response = route_grpc_request(registry, &config, request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers().get("grpc-status").unwrap(), "0");
        assert_eq!(response.headers().get("x-response-id").unwrap(), "resp-123");

        let body = axum::body::to_bytes(response.into_body(), 1024).await.unwrap();
        assert_eq!(body, Bytes::from_static(b"hello"));
    }

    #[tokio::test]
    async fn test_route_grpc_request_bidi_streaming_success() {
        struct BidiHandler;

        impl GrpcHandler for BidiHandler {
            fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async { Err(tonic::Status::unimplemented("Use bidi streaming")) })
            }

            fn service_name(&self) -> &str {
                "test.BidiService"
            }

            fn call_bidi_stream(
                &self,
                _request: StreamingRequest,
            ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
                Box::pin(async {
                    let messages = vec![Bytes::from("r1")];
                    Ok(message_stream_from_vec(messages))
                })
            }
        }

        let mut registry = GrpcRegistry::new();
        registry.register_service(
            "test.BidiService",
            Arc::new(BidiHandler),
            RpcMode::BidirectionalStreaming,
        );
        let registry = Arc::new(registry);
        let config = GrpcConfig::default();

        // Provide a well-formed request frame so the frame parser succeeds.
        let frame = vec![
            0x00, // compression: no
            0x00, 0x00, 0x00, 0x01, // length: 1 byte
            b'x',
        ];

        let request = Request::builder()
            .uri("/test.BidiService/Chat")
            .header("content-type", "application/grpc")
            .body(Body::from(frame))
            .unwrap();

        let response = route_grpc_request(registry, &config, request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert!(
            response.headers().get("grpc-status").is_none(),
            "bidi-streaming responses must not predeclare grpc-status"
        );

        let collected = response.into_body().collect().await.unwrap();
        let trailers = collected.trailers().expect("grpc trailers");
        let grpc_status = trailers.get("grpc-status").unwrap().to_str().unwrap().to_string();
        let grpc_message = trailers.get("grpc-message").unwrap().to_str().unwrap().to_string();
        let body = collected.to_bytes();
        assert!(!body.is_empty());
        assert_eq!(grpc_status, "0");
        assert_eq!(grpc_message, "OK");
    }

    #[tokio::test]
    async fn test_route_grpc_request_client_streaming_unsupported_encoding_maps_to_501() {
        // Use a handler that would succeed, but the request is invalid before handler gets called.
        struct NeverCalledHandler;

        impl GrpcHandler for NeverCalledHandler {
            fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async { Err(tonic::Status::unimplemented("not used")) })
            }

            fn service_name(&self) -> &str {
                "test.BadClientStreamService"
            }

            fn call_client_stream(
                &self,
                _request: StreamingRequest,
            ) -> Pin<Box<dyn Future<Output = Result<GrpcResponseData, tonic::Status>> + Send>> {
                Box::pin(async {
                    Ok(GrpcResponseData {
                        payload: Bytes::from_static(b"ok"),
                        metadata: MetadataMap::new(),
                    })
                })
            }
        }

        let mut registry = GrpcRegistry::new();
        registry.register_service(
            "test.BadClientStreamService",
            Arc::new(NeverCalledHandler),
            RpcMode::ClientStreaming,
        );
        let registry = Arc::new(registry);
        let config = GrpcConfig::default();

        // Compression flag = 1 with unsupported grpc-encoding => UNIMPLEMENTED.
        let frame = vec![
            0x01, // compression: yes
            0x00, 0x00, 0x00, 0x01, // length: 1 byte
            b'x',
        ];

        let request = Request::builder()
            .uri("/test.BadClientStreamService/ClientStream")
            .header("content-type", "application/grpc")
            .header("grpc-encoding", "br")
            .body(Body::from(frame))
            .unwrap();

        let err = route_grpc_request(registry, &config, request).await.unwrap_err();
        assert_eq!(err.0, StatusCode::NOT_IMPLEMENTED);
        assert!(err.1.contains("Unsupported grpc-encoding"));
    }

    #[tokio::test]
    async fn test_route_grpc_request_client_streaming_missing_encoding_header_maps_to_400() {
        struct NeverCalledHandler;

        impl GrpcHandler for NeverCalledHandler {
            fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async { Err(tonic::Status::unimplemented("not used")) })
            }

            fn service_name(&self) -> &str {
                "test.BadClientStreamService"
            }

            fn call_client_stream(
                &self,
                _request: StreamingRequest,
            ) -> Pin<Box<dyn Future<Output = Result<GrpcResponseData, tonic::Status>> + Send>> {
                Box::pin(async {
                    Ok(GrpcResponseData {
                        payload: Bytes::from_static(b"ok"),
                        metadata: MetadataMap::new(),
                    })
                })
            }
        }

        let mut registry = GrpcRegistry::new();
        registry.register_service(
            "test.BadClientStreamService",
            Arc::new(NeverCalledHandler),
            RpcMode::ClientStreaming,
        );
        let registry = Arc::new(registry);
        let config = GrpcConfig::default();

        // Compression flag = 1 with no grpc-encoding => INVALID_ARGUMENT.
        let frame = vec![
            0x01, // compression: yes
            0x00, 0x00, 0x00, 0x01, // length: 1 byte
            b'x',
        ];

        let request = Request::builder()
            .uri("/test.BadClientStreamService/ClientStream")
            .header("content-type", "application/grpc")
            .body(Body::from(frame))
            .unwrap();

        let err = route_grpc_request(registry, &config, request).await.unwrap_err();
        assert_eq!(err.0, StatusCode::BAD_REQUEST);
        assert!(err.1.contains("missing grpc-encoding"));
    }

    #[tokio::test]
    async fn test_route_grpc_request_client_streaming_gzip_success() {
        struct ClientStreamHandler;

        impl GrpcHandler for ClientStreamHandler {
            fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async { Err(tonic::Status::unimplemented("Use client streaming")) })
            }

            fn service_name(&self) -> &str {
                "test.CompressedClientStreamService"
            }

            fn call_client_stream(
                &self,
                mut request: StreamingRequest,
            ) -> Pin<Box<dyn Future<Output = Result<GrpcResponseData, tonic::Status>> + Send>> {
                Box::pin(async move {
                    let payload = request
                        .message_stream
                        .next()
                        .await
                        .transpose()?
                        .unwrap_or_else(Bytes::new);
                    Ok(GrpcResponseData {
                        payload,
                        metadata: MetadataMap::new(),
                    })
                })
            }
        }

        let mut registry = GrpcRegistry::new();
        registry.register_service(
            "test.CompressedClientStreamService",
            Arc::new(ClientStreamHandler),
            RpcMode::ClientStreaming,
        );
        let registry = Arc::new(registry);
        let config = GrpcConfig::default();

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(b"hello").unwrap();
        let compressed_payload = encoder.finish().unwrap();

        let mut frame = Vec::new();
        frame.push(0x01); // compression: yes
        frame.extend_from_slice(&(compressed_payload.len() as u32).to_be_bytes());
        frame.extend_from_slice(&compressed_payload);

        let request = Request::builder()
            .uri("/test.CompressedClientStreamService/ClientStream")
            .header("content-type", "application/grpc")
            .header("grpc-encoding", "gzip")
            .body(Body::from(frame))
            .unwrap();

        let response = route_grpc_request(registry, &config, request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), 1024).await.unwrap();
        assert_eq!(body, Bytes::from_static(b"hello"));
    }

    #[tokio::test]
    async fn test_route_grpc_request_client_streaming_gzip_when_compression_disabled_maps_to_501() {
        struct NeverCalledHandler;

        impl GrpcHandler for NeverCalledHandler {
            fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
                Box::pin(async { Err(tonic::Status::unimplemented("not used")) })
            }

            fn service_name(&self) -> &str {
                "test.DisabledCompressionService"
            }

            fn call_client_stream(
                &self,
                _request: StreamingRequest,
            ) -> Pin<Box<dyn Future<Output = Result<GrpcResponseData, tonic::Status>> + Send>> {
                Box::pin(async {
                    Ok(GrpcResponseData {
                        payload: Bytes::from_static(b"ok"),
                        metadata: MetadataMap::new(),
                    })
                })
            }
        }

        let mut registry = GrpcRegistry::new();
        registry.register_service(
            "test.DisabledCompressionService",
            Arc::new(NeverCalledHandler),
            RpcMode::ClientStreaming,
        );
        let registry = Arc::new(registry);
        let mut config = GrpcConfig::default();
        config.enable_compression = false;

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(b"hello").unwrap();
        let compressed_payload = encoder.finish().unwrap();

        let mut frame = Vec::new();
        frame.push(0x01); // compression: yes
        frame.extend_from_slice(&(compressed_payload.len() as u32).to_be_bytes());
        frame.extend_from_slice(&compressed_payload);

        let request = Request::builder()
            .uri("/test.DisabledCompressionService/ClientStream")
            .header("content-type", "application/grpc")
            .header("grpc-encoding", "gzip")
            .body(Body::from(frame))
            .unwrap();

        let err = route_grpc_request(registry, &config, request).await.unwrap_err();
        assert_eq!(err.0, StatusCode::NOT_IMPLEMENTED);
        assert!(err.1.contains("disabled"));
    }
}

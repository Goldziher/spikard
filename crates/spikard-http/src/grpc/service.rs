//! Tonic service bridge
//!
//! This module bridges Tonic's service traits with our GrpcHandler trait.
//! It handles the conversion between Tonic's types and our internal representation,
//! enabling language-agnostic gRPC handling.

use crate::grpc::handler::{GrpcHandler, GrpcHandlerResult, GrpcRequestData, GrpcResponseData};
use crate::grpc::streaming::MessageStream;
use bytes::Bytes;
use futures_util::StreamExt;
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

    /// Handle a server streaming RPC call
    ///
    /// Takes a single request and returns a stream of response messages.
    /// Converts the Tonic Request into our GrpcRequestData format, calls the
    /// handler's call_server_stream method, and converts the MessageStream
    /// into a Tonic streaming response body.
    ///
    /// # Arguments
    ///
    /// * `service_name` - Fully qualified service name
    /// * `method_name` - Method name
    /// * `request` - Tonic request containing the serialized protobuf message
    ///
    /// # Returns
    ///
    /// A Response with a streaming body containing the message stream
    ///
    /// # Error Propagation Limitations
    ///
    /// When a stream returns an error mid-stream (after messages have begun
    /// being sent), the error may not be perfectly transmitted to the client
    /// as a gRPC trailer. This is due to limitations in Axum's `Body::from_stream`:
    ///
    /// - **Pre-stream errors** (before any messages): Properly converted to
    ///   HTTP status codes and returned to the client
    /// - **Mid-stream errors** (after messages have begun): The error is converted
    ///   to a generic `BoxError`, and the stream terminates. The connection is
    ///   properly closed, but the gRPC status code metadata is lost.
    ///
    /// For robust error handling in streaming RPCs:
    /// - Prefer detecting errors early (before sending messages) when possible
    /// - Include error information in the message stream itself if critical
    ///   (application-level error messages in the protobuf)
    /// - For true gRPC trailer support, consider implementing a custom Axum
    ///   body type that wraps the stream and can inject trailers on error
    ///
    /// See: <https://github.com/tokio-rs/axum/discussions/2043>
    pub async fn handle_server_stream(
        &self,
        service_name: String,
        method_name: String,
        request: Request<Bytes>,
    ) -> Result<Response<axum::body::Body>, Status> {
        // Extract metadata and payload from Tonic request
        let (metadata, _extensions, payload) = request.into_parts();

        // Create our internal request representation
        let grpc_request = GrpcRequestData {
            service_name,
            method_name,
            payload,
            metadata,
        };

        // Call the handler's server streaming method
        let message_stream: MessageStream = self.handler.call_server_stream(grpc_request).await?;

        // Convert MessageStream to axum Body
        //
        // LIMITATION: When converting tonic::Status errors from the stream,
        // we lose the gRPC status metadata. The Status is converted to a
        // generic Box<dyn Error>, and Axum's Body::from_stream doesn't have
        // special handling for gRPC error semantics.
        //
        // Current behavior:
        // - Stream errors are converted to BoxError
        // - Body stream terminates on the first error
        // - Connection is properly closed
        // - Error metadata (status code, message) is not transmitted to client
        //
        // TODO: Implement custom Body wrapper that can:
        // 1. Capture tonic::Status errors
        // 2. Extract status code and message
        // 3. Inject gRPC trailers (grpc-status, grpc-message) when stream ends
        // 4. Properly signal error to client while preserving partial messages
        //
        // This would require implementing a custom StreamBody or similar that
        // understands gRPC error semantics.
        let byte_stream =
            message_stream.map(|result| result.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>));

        let body = axum::body::Body::from_stream(byte_stream);

        // Create response with streaming body
        let response = Response::new(body);

        Ok(response)
    }

    /// Handle a client streaming RPC call
    ///
    /// Takes a request body stream of protobuf messages and returns a single response.
    /// Parses the HTTP/2 body stream using gRPC frame parser, creates a MessageStream,
    /// calls the handler's call_client_stream method, and converts the GrpcResponseData
    /// back to a Tonic Response.
    ///
    /// # Arguments
    ///
    /// * `service_name` - Fully qualified service name
    /// * `method_name` - Method name
    /// * `request` - Axum request with streaming body containing HTTP/2 framed protobuf messages
    /// * `max_message_size` - Maximum size per message (bytes)
    ///
    /// # Returns
    ///
    /// A Response with a single message body
    ///
    /// # Stream Handling
    ///
    /// The request body stream contains framed protobuf messages. Each frame is parsed
    /// and validated for size:
    /// - Messages within `max_message_size` are passed to the handler
    /// - Messages exceeding the limit result in a ResourceExhausted error
    /// - Invalid frames result in InvalidArgument errors
    /// - The stream terminates when the client closes the write side
    ///
    /// # Frame Format
    ///
    /// Frames follow the gRPC HTTP/2 protocol format:
    /// - 1 byte: compression flag (0 = uncompressed)
    /// - 4 bytes: message size (big-endian)
    /// - N bytes: message payload
    ///
    /// # Metadata and Trailers
    ///
    /// - Request metadata (headers) from the Tonic request is passed to the handler
    /// - Response metadata from the handler is included in the response headers
    /// - gRPC trailers (like grpc-status) should be handled by the caller
    pub async fn handle_client_stream(
        &self,
        service_name: String,
        method_name: String,
        request: Request<axum::body::Body>,
        max_message_size: usize,
    ) -> Result<Response<Bytes>, Status> {
        // Extract metadata and body from Tonic request
        let (metadata, _extensions, body) = request.into_parts();

        // Parse HTTP/2 body into stream of gRPC frames with size validation
        let message_stream = crate::grpc::framing::parse_grpc_client_stream(body, max_message_size).await?;

        // Create our internal streaming request representation
        let streaming_request = crate::grpc::streaming::StreamingRequest {
            service_name,
            method_name,
            message_stream,
            metadata,
        };

        // Call the handler's client streaming method
        let response: crate::grpc::handler::GrpcHandlerResult =
            self.handler.call_client_stream(streaming_request).await;

        // Convert result to Tonic response
        match response {
            Ok(grpc_response) => {
                let mut tonic_response = Response::new(grpc_response.payload);
                copy_metadata(&grpc_response.metadata, tonic_response.metadata_mut());
                Ok(tonic_response)
            }
            Err(status) => Err(status),
        }
    }

    /// Handle a bidirectional streaming RPC call
    ///
    /// Takes a request body stream and returns a stream of response messages.
    /// Parses the HTTP/2 body stream using gRPC frame parser, creates a StreamingRequest,
    /// calls the handler's call_bidi_stream method, and converts the MessageStream
    /// back to an Axum streaming response body.
    ///
    /// # Arguments
    ///
    /// * `service_name` - Fully qualified service name
    /// * `method_name` - Method name
    /// * `request` - Axum request with streaming body containing HTTP/2 framed protobuf messages
    /// * `max_message_size` - Maximum size per message (bytes)
    ///
    /// # Returns
    ///
    /// A Response with a streaming body containing response messages
    ///
    /// # Stream Handling
    ///
    /// - Request stream: Parsed from HTTP/2 body using frame parser
    /// - Response stream: Converted from MessageStream to Axum Body
    /// - Both streams are independent (full-duplex)
    /// - Errors in either stream are propagated appropriately
    ///
    /// # Error Propagation
    ///
    /// Similar to server streaming, mid-stream errors in the response may not be
    /// perfectly transmitted as gRPC trailers due to Axum Body::from_stream limitations.
    /// See handle_server_stream() documentation for details.
    pub async fn handle_bidi_stream(
        &self,
        service_name: String,
        method_name: String,
        request: Request<axum::body::Body>,
        max_message_size: usize,
    ) -> Result<Response<axum::body::Body>, Status> {
        // Extract metadata and body from Tonic request
        let (metadata, _extensions, body) = request.into_parts();

        // Parse HTTP/2 body into stream of gRPC frames with size validation
        let message_stream = crate::grpc::framing::parse_grpc_client_stream(body, max_message_size).await?;

        // Create our internal streaming request representation
        let streaming_request = crate::grpc::streaming::StreamingRequest {
            service_name,
            method_name,
            message_stream,
            metadata,
        };

        // Call the handler's bidirectional streaming method
        let response_stream: MessageStream = self.handler.call_bidi_stream(streaming_request).await?;

        // Convert MessageStream to axum Body (same as server streaming)
        let byte_stream =
            response_stream.map(|result| result.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>));

        let body = axum::body::Body::from_stream(byte_stream);
        let response = Response::new(body);

        Ok(response)
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

        fn service_name(&self) -> &str {
            "test.TestService"
        }
    }

    #[tokio::test]
    async fn test_generic_grpc_service_handle_unary() {
        let handler = Arc::new(TestHandler);
        let service = GenericGrpcService::new(handler);

        let request = Request::new(Bytes::from("test payload"));
        let result = service
            .handle_unary("test.TestService".to_string(), "TestMethod".to_string(), request)
            .await;

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

        let result = service
            .handle_unary("test.TestService".to_string(), "TestMethod".to_string(), request)
            .await;

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
        headers.insert(axum::http::header::CONTENT_TYPE, "application/grpc".parse().unwrap());
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
        headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().unwrap());
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

            fn service_name(&self) -> &str {
                "test.ErrorService"
            }
        }

        let handler = Arc::new(ErrorHandler);
        let service = GenericGrpcService::new(handler);

        let request = Request::new(Bytes::new());
        let result = service
            .handle_unary("test.ErrorService".to_string(), "ErrorMethod".to_string(), request)
            .await;

        assert!(result.is_err());
        let status = result.unwrap_err();
        assert_eq!(status.code(), tonic::Code::NotFound);
        assert_eq!(status.message(), "Resource not found");
    }
}

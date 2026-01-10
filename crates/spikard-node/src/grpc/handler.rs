//! Node.js handler implementation for gRPC requests
//!
//! This module implements the GrpcHandler trait using napi-rs ThreadsafeFunction
//! to call JavaScript handlers from Rust's async gRPC server.

use async_stream::stream;
use bytes::Bytes;
use futures_util::StreamExt;
use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunction;
use napi_derive::napi;
use spikard_http::grpc::streaming::{MessageStream, StreamingRequest};
use spikard_http::grpc::{GrpcHandler, GrpcHandlerResult, GrpcRequestData, GrpcResponseData};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;
use tokio::sync::mpsc;

// Stream message limit to prevent unbounded memory growth
const MAX_STREAM_MESSAGES: usize = 10_000;

use tonic::metadata::{MetadataMap, MetadataValue};

/// gRPC request object passed to JavaScript handlers
///
/// Contains the parsed components of a gRPC request with all data
/// converted to JavaScript-friendly types.
#[napi(object)]
pub struct GrpcRequest {
    /// Fully qualified service name (e.g., "mypackage.UserService")
    pub service_name: String,
    /// Method name (e.g., "GetUser")
    pub method_name: String,
    /// Serialized protobuf message as Buffer
    pub payload: Buffer,
    /// gRPC metadata as key-value pairs
    pub metadata: HashMap<String, String>,
}

/// gRPC response object returned by JavaScript handlers
///
/// Contains the serialized protobuf response and optional metadata
/// to include in the response headers.
#[napi(object)]
pub struct GrpcResponse {
    /// Serialized protobuf message as Buffer
    pub payload: Buffer,
    /// Optional gRPC metadata to include in response
    #[napi(ts_type = "Record<string, string> | undefined")]
    pub metadata: Option<HashMap<String, String>>,
}

/// Client streaming request: unary request with collected stream messages
///
/// Used when calling JavaScript client streaming handlers.
/// The handler receives all collected messages in a single call.
#[napi(object)]
pub struct GrpcClientStreamRequest {
    /// Fully qualified service name (e.g., "mypackage.UserService")
    pub service_name: String,
    /// Method name (e.g., "GetUser")
    pub method_name: String,
    /// gRPC metadata as key-value pairs
    pub metadata: HashMap<String, String>,
    /// Collected stream messages as Buffers
    #[napi(ts_type = "Buffer[]")]
    pub messages: Vec<Buffer>,
}

/// Bidirectional streaming request: request with collected input messages
///
/// Used when calling JavaScript bidirectional streaming handlers.
/// The handler receives all input messages and returns an array of response messages.
#[napi(object)]
pub struct GrpcBidiStreamRequest {
    /// Fully qualified service name (e.g., "mypackage.UserService")
    pub service_name: String,
    /// Method name (e.g., "GetUser")
    pub method_name: String,
    /// gRPC metadata as key-value pairs
    pub metadata: HashMap<String, String>,
    /// Collected stream messages as Buffers
    #[napi(ts_type = "Buffer[]")]
    pub messages: Vec<Buffer>,
}

/// Bidirectional streaming response: array of response messages
///
/// Returned by JavaScript bidirectional streaming handlers.
/// Each message is converted back to a protobuf message in the response stream.
#[napi(object)]
pub struct GrpcBidiStreamResponse {
    /// Stream response messages as Buffers
    #[napi(ts_type = "Buffer[]")]
    pub messages: Vec<Buffer>,
    /// Optional gRPC metadata to include in response
    #[napi(ts_type = "Record<string, string> | undefined")]
    pub metadata: Option<HashMap<String, String>>,
}

/// JavaScript-side gRPC message stream for client streaming RPC
///
/// Wraps a Rust MessageStream and exposes JavaScript async iterator protocol
/// so JavaScript handlers can consume incoming messages as an async iterable.
#[napi]
pub struct GrpcMessageStream {
    /// Channel to receive messages from the stream
    receiver: Arc<TokioMutex<mpsc::Receiver<std::result::Result<Bytes, String>>>>,
}

#[napi]
impl GrpcMessageStream {
    /// Get the next message from the stream
    ///
    /// Returns a Promise that resolves to the next message Buffer or null if stream is done.
    /// Throws an error if the stream encounters an error.
    #[napi]
    pub async fn next(&self) -> Result<Option<Buffer>> {
        let mut receiver = self.receiver.lock().await;

        match receiver.recv().await {
            Some(Ok(bytes)) => {
                // Convert Bytes to Buffer
                Ok(Some(Buffer::from(bytes.as_ref())))
            }
            Some(Err(err_msg)) => {
                // Stream error
                Err(Error::from_reason(err_msg))
            }
            None => {
                // Stream exhausted
                Ok(None)
            }
        }
    }
}

/// Convert tonic MetadataMap to JavaScript-friendly HashMap
fn metadata_to_hashmap(metadata: &MetadataMap) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for key_and_value in metadata.iter() {
        match key_and_value {
            tonic::metadata::KeyAndValueRef::Ascii(key, value) => {
                if let Ok(value_str) = value.to_str() {
                    map.insert(key.as_str().to_string(), value_str.to_string());
                }
            }
            tonic::metadata::KeyAndValueRef::Binary(key, _value) => {
                // Skip binary metadata for now, as we only support string values in the HashMap
                tracing::debug!("Skipping binary metadata key: {}", key.as_str());
            }
        }
    }
    map
}

/// Convert JavaScript HashMap to tonic MetadataMap
fn hashmap_to_metadata(map: &HashMap<String, String>) -> Result<MetadataMap> {
    let mut metadata = MetadataMap::new();
    for (key, value) in map {
        let key_name = key
            .parse::<tonic::metadata::MetadataKey<tonic::metadata::Ascii>>()
            .map_err(|e| Error::from_reason(format!("Invalid metadata key '{}': {}", key, e)))?;
        let value_bytes = value
            .parse::<MetadataValue<tonic::metadata::Ascii>>()
            .map_err(|e| Error::from_reason(format!("Invalid metadata value '{}': {}", value, e)))?;
        metadata.insert(key_name, value_bytes);
    }
    Ok(metadata)
}

/// Create a JavaScript async iterator from a Rust MessageStream
///
/// Returns a GrpcMessageStream that JavaScript code can consume using `for await`.
/// The stream is converted to a channel-based receiver to avoid shared mutable state.
#[allow(dead_code)]
fn create_js_stream_iterator(stream: MessageStream) -> GrpcMessageStream {
    let (tx, rx) = mpsc::channel(MAX_STREAM_MESSAGES);
    // Spawn a task to forward messages from the stream to the channel
    tokio::spawn(async move {
        let mut stream = stream;
        while let Some(result) = stream.next().await {
            match result {
                Ok(bytes) => {
                    if tx.send(Ok(bytes)).await.is_err() {
                        // Receiver dropped, stop forwarding
                        break;
                    }
                }
                Err(status) => {
                    // Send error and stop
                    let _ = tx.send(Err(status.message().to_string())).await;
                    break;
                }
            }
        }
        // Channel will be closed when tx is dropped
    });

    GrpcMessageStream {
        receiver: Arc::new(TokioMutex::new(rx)),
    }
}

/// Type alias for client stream handler ThreadsafeFunction
type ClientStreamHandler =
    ThreadsafeFunction<GrpcClientStreamRequest, Promise<GrpcResponse>, GrpcClientStreamRequest, napi::Status, false>;

/// Type alias for bidirectional stream handler ThreadsafeFunction
type BidiStreamHandler = ThreadsafeFunction<
    GrpcBidiStreamRequest,
    Promise<GrpcBidiStreamResponse>,
    GrpcBidiStreamRequest,
    napi::Status,
    false,
>;

/// Node.js gRPC handler wrapper that implements spikard_http::grpc::GrpcHandler
///
/// Uses ThreadsafeFunction to call JavaScript handlers from Rust threads.
/// Converts between Rust's bytes/metadata types and JavaScript-friendly objects.
/// Supports unary, server streaming, client streaming, and bidirectional streaming RPC modes.
pub struct NodeGrpcHandler {
    service_name: Arc<str>,
    handler_fn: Arc<ThreadsafeFunction<GrpcRequest, Promise<GrpcResponse>, GrpcRequest, napi::Status, false>>,
    client_stream_fn: Option<Arc<ClientStreamHandler>>,
    bidi_stream_fn: Option<Arc<BidiStreamHandler>>,
}

unsafe impl Send for NodeGrpcHandler {}
unsafe impl Sync for NodeGrpcHandler {}

impl NodeGrpcHandler {
    /// Create a new Node gRPC handler wrapper with a JavaScript function
    ///
    /// # Arguments
    ///
    /// * `service_name` - Fully qualified service name (must be 'static)
    /// * `handler_fn` - ThreadsafeFunction that calls JavaScript handler for unary requests
    pub fn new(
        service_name: Arc<str>,
        handler_fn: ThreadsafeFunction<GrpcRequest, Promise<GrpcResponse>, GrpcRequest, napi::Status, false>,
    ) -> Self {
        Self {
            service_name,
            handler_fn: Arc::new(handler_fn),
            client_stream_fn: None,
            bidi_stream_fn: None,
        }
    }

    /// Add a client streaming handler to this gRPC handler
    ///
    /// # Arguments
    ///
    /// * `client_stream_fn` - ThreadsafeFunction for client streaming requests
    #[must_use]
    pub fn with_client_stream(mut self, client_stream_fn: ClientStreamHandler) -> Self {
        self.client_stream_fn = Some(Arc::new(client_stream_fn));
        self
    }

    /// Add a bidirectional streaming handler to this gRPC handler
    ///
    /// # Arguments
    ///
    /// * `bidi_stream_fn` - ThreadsafeFunction for bidirectional streaming requests
    #[must_use]
    pub fn with_bidi_stream(mut self, bidi_stream_fn: BidiStreamHandler) -> Self {
        self.bidi_stream_fn = Some(Arc::new(bidi_stream_fn));
        self
    }
}

impl GrpcHandler for NodeGrpcHandler {
    fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        let handler_fn = self.handler_fn.clone();
        let service_name = self.service_name.clone();

        Box::pin(async move {
            // Convert Rust types to JavaScript-friendly types
            let js_request = GrpcRequest {
                service_name: request.service_name.clone(),
                method_name: request.method_name.clone(),
                payload: Buffer::from(request.payload.as_ref()),
                metadata: metadata_to_hashmap(&request.metadata),
            };

            // Call the JavaScript handler
            match handler_fn.call_async(js_request).await {
                Ok(promise) => {
                    match promise.await {
                        Ok(response) => {
                            // Convert JavaScript response back to Rust types
                            let metadata = if let Some(meta_map) = response.metadata {
                                match hashmap_to_metadata(&meta_map) {
                                    Ok(m) => m,
                                    Err(e) => {
                                        return Err(tonic::Status::internal(format!(
                                            "Failed to convert metadata for {}: {}",
                                            service_name, e
                                        )));
                                    }
                                }
                            } else {
                                MetadataMap::new()
                            };

                            Ok(GrpcResponseData {
                                payload: response.payload.to_vec().into(),
                                metadata,
                            })
                        }
                        Err(e) => Err(tonic::Status::internal(format!(
                            "Handler promise failed for {}: {:?}",
                            service_name, e
                        ))),
                    }
                }
                Err(e) => Err(tonic::Status::internal(format!(
                    "Handler call failed for {}: {}",
                    service_name, e
                ))),
            }
        })
    }

    fn service_name(&self) -> &str {
        self.service_name.as_ref()
    }

    fn call_server_stream(
        &self,
        request: GrpcRequestData,
    ) -> Pin<Box<dyn Future<Output = std::result::Result<MessageStream, tonic::Status>> + Send>> {
        let handler_fn = self.handler_fn.clone();
        let service_name = self.service_name.clone();

        Box::pin(async move {
            // Convert Rust request to JavaScript request
            let js_request = GrpcRequest {
                service_name: request.service_name.clone(),
                method_name: request.method_name.clone(),
                payload: Buffer::from(request.payload.as_ref()),
                metadata: metadata_to_hashmap(&request.metadata),
            };

            // Create channel to collect messages from handler
            let (tx, mut rx) = mpsc::channel::<std::result::Result<Bytes, tonic::Status>>(MAX_STREAM_MESSAGES);

            // Call JavaScript handler
            match handler_fn.call_async(js_request).await {
                Ok(promise) => {
                    // Spawn task to handle response stream
                    let service_name_clone = service_name.clone();
                    tokio::spawn(async move {
                        match promise.await {
                            Ok(response) => {
                                // For server streaming, the handler returns a GrpcResponse
                                // containing the response payload. In JavaScript, handlers should
                                // pre-collect stream messages or use async generators.
                                // The payload here represents the streamed data.
                                let _ = tx.try_send(Ok(response.payload.to_vec().into()));
                            }
                            Err(e) => {
                                let _ = tx.try_send(Err(tonic::Status::internal(format!(
                                    "Handler promise failed for {}: {}",
                                    service_name_clone, e
                                ))));
                            }
                        }
                    });
                }
                Err(e) => {
                    let _ = tx.try_send(Err(tonic::Status::internal(format!(
                        "Handler call failed for {}: {}",
                        service_name, e
                    ))));
                }
            }

            // Convert the channel receiver to a MessageStream
            let message_stream = stream! {
                while let Some(result) = rx.recv().await {
                    yield result;
                }
            };

            Ok(Box::pin(message_stream) as MessageStream)
        })
    }

    fn call_client_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = std::result::Result<GrpcResponseData, tonic::Status>> + Send>> {
        let handler_fn = self.client_stream_fn.clone();
        let service_name = self.service_name.clone();

        Box::pin(async move {
            // Check if client streaming handler is registered
            let Some(handler_fn) = handler_fn else {
                return Err(tonic::Status::unimplemented(format!(
                    "Client streaming not implemented for service '{}'",
                    service_name
                )));
            };

            // Step 1: Collect all messages from input stream into a vector
            let mut collected_messages = Vec::new();
            let mut stream = Box::pin(request.message_stream);

            while let Some(result) = stream.next().await {
                match result {
                    Ok(bytes) => collected_messages.push(Buffer::from(bytes.as_ref())),
                    Err(e) => {
                        return Err(tonic::Status::internal(format!(
                            "Error collecting stream message: {}",
                            e.message()
                        )));
                    }
                }
            }

            // Enforce max stream messages limit
            if collected_messages.len() > MAX_STREAM_MESSAGES {
                return Err(tonic::Status::resource_exhausted(format!(
                    "Client stream exceeded maximum messages: {} > {}",
                    collected_messages.len(),
                    MAX_STREAM_MESSAGES
                )));
            }

            // Step 2: Create request object with collected messages
            let client_stream_request = GrpcClientStreamRequest {
                service_name: request.service_name.clone(),
                method_name: request.method_name.clone(),
                metadata: metadata_to_hashmap(&request.metadata),
                messages: collected_messages,
            };

            // Step 3: Call the JavaScript handler with the streaming request
            match handler_fn.call_async(client_stream_request).await {
                Ok(promise) => {
                    match promise.await {
                        Ok(response) => {
                            // Convert JavaScript response back to Rust types
                            let metadata = if let Some(meta_map) = response.metadata {
                                match hashmap_to_metadata(&meta_map) {
                                    Ok(m) => m,
                                    Err(e) => {
                                        return Err(tonic::Status::internal(format!(
                                            "Failed to convert metadata for {}: {}",
                                            service_name, e
                                        )));
                                    }
                                }
                            } else {
                                MetadataMap::new()
                            };

                            Ok(GrpcResponseData {
                                payload: response.payload.to_vec().into(),
                                metadata,
                            })
                        }
                        Err(e) => Err(tonic::Status::internal(format!(
                            "Handler promise failed for {}: {:?}",
                            service_name, e
                        ))),
                    }
                }
                Err(e) => Err(tonic::Status::internal(format!(
                    "Handler call failed for {}: {}",
                    service_name, e
                ))),
            }
        })
    }

    fn call_bidi_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = std::result::Result<MessageStream, tonic::Status>> + Send>> {
        let handler_fn = self.bidi_stream_fn.clone();
        let service_name = self.service_name.clone();

        Box::pin(async move {
            // Check if bidirectional streaming handler is registered
            let Some(handler_fn) = handler_fn else {
                return Err(tonic::Status::unimplemented(format!(
                    "Bidirectional streaming not implemented for service '{}'",
                    service_name
                )));
            };

            // Step 1: Collect all input messages into a vector
            let mut collected_messages = Vec::new();
            let mut stream = Box::pin(request.message_stream);

            while let Some(result) = stream.next().await {
                match result {
                    Ok(bytes) => collected_messages.push(Buffer::from(bytes.as_ref())),
                    Err(e) => {
                        return Err(tonic::Status::internal(format!(
                            "Error collecting stream message: {}",
                            e.message()
                        )));
                    }
                }
            }

            // Enforce max stream messages limit
            if collected_messages.len() > MAX_STREAM_MESSAGES {
                return Err(tonic::Status::resource_exhausted(format!(
                    "Bidirectional stream exceeded maximum messages: {} > {}",
                    collected_messages.len(),
                    MAX_STREAM_MESSAGES
                )));
            }

            // Step 2: Create bidirectional stream request with collected messages
            let bidi_request = GrpcBidiStreamRequest {
                service_name: request.service_name.clone(),
                method_name: request.method_name.clone(),
                metadata: metadata_to_hashmap(&request.metadata),
                messages: collected_messages,
            };

            // Step 3: Call the JavaScript handler with the streaming request
            match handler_fn.call_async(bidi_request).await {
                Ok(promise) => {
                    match promise.await {
                        Ok(response) => {
                            // Step 4: Convert response messages array to MessageStream
                            // Create channel to send messages to the stream consumer
                            let (tx, mut rx) =
                                mpsc::channel::<std::result::Result<Bytes, tonic::Status>>(MAX_STREAM_MESSAGES);

                            // Spawn task to forward collected response messages to channel
                            tokio::spawn(async move {
                                for msg in response.messages {
                                    if tx.send(Ok(msg.to_vec().into())).await.is_err() {
                                        // Receiver dropped, stop forwarding
                                        break;
                                    }
                                }
                                // Channel will be closed when tx is dropped
                            });

                            // Convert the channel receiver to a MessageStream
                            let message_stream = stream! {
                                while let Some(result) = rx.recv().await {
                                    yield result;
                                }
                            };

                            Ok(Box::pin(message_stream) as MessageStream)
                        }
                        Err(e) => Err(tonic::Status::internal(format!(
                            "Handler promise failed for {}: {:?}",
                            service_name, e
                        ))),
                    }
                }
                Err(e) => Err(tonic::Status::internal(format!(
                    "Handler call failed for {}: {}",
                    service_name, e
                ))),
            }
        })
    }
}

impl Clone for NodeGrpcHandler {
    fn clone(&self) -> Self {
        Self {
            service_name: self.service_name.clone(),
            handler_fn: self.handler_fn.clone(),
            client_stream_fn: self.client_stream_fn.clone(),
            bidi_stream_fn: self.bidi_stream_fn.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_to_hashmap() {
        let mut metadata = MetadataMap::new();
        metadata.insert("content-type", "application/grpc".parse().unwrap());
        metadata.insert("x-custom", "value".parse().unwrap());

        let map = metadata_to_hashmap(&metadata);

        assert_eq!(map.len(), 2);
        assert_eq!(map.get("content-type"), Some(&"application/grpc".to_string()));
        assert_eq!(map.get("x-custom"), Some(&"value".to_string()));
    }

    #[test]
    fn test_metadata_to_hashmap_empty() {
        let metadata = MetadataMap::new();
        let map = metadata_to_hashmap(&metadata);
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_hashmap_to_metadata() {
        let mut map = HashMap::new();
        map.insert("content-type".to_string(), "application/grpc".to_string());
        map.insert("x-custom".to_string(), "value".to_string());

        let metadata = hashmap_to_metadata(&map).unwrap();

        assert_eq!(metadata.len(), 2);
        assert!(metadata.contains_key("content-type"));
        assert!(metadata.contains_key("x-custom"));
    }

    #[test]
    fn test_hashmap_to_metadata_empty() {
        let map = HashMap::new();
        let metadata = hashmap_to_metadata(&map).unwrap();
        assert_eq!(metadata.len(), 0);
    }

    #[test]
    fn test_hashmap_to_metadata_invalid_key() {
        let mut map = HashMap::new();
        map.insert("invalid key with spaces".to_string(), "value".to_string());

        let result = hashmap_to_metadata(&map);
        assert!(result.is_err());
    }
}

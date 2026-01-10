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

/// JavaScript-side gRPC message stream for client streaming RPC
///
/// Wraps a Rust MessageStream and exposes JavaScript async iterator protocol
/// so JavaScript handlers can consume incoming messages as an async iterable.
#[napi]
pub struct GrpcMessageStream {
    /// Channel to receive messages from the stream
    receiver: Arc<tokio::sync::Mutex<mpsc::UnboundedReceiver<std::result::Result<Bytes, String>>>>,
    /// Sender side of channel, kept to ensure cleanup when stream is dropped.
    /// This field is not exposed to JavaScript via napi binding.
    sender: Arc<Option<mpsc::UnboundedSender<std::result::Result<Bytes, String>>>>,
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

impl Drop for GrpcMessageStream {
    fn drop(&mut self) {
        // Explicitly drop the sender to close the channel
        // This ensures that any tokio tasks forwarding messages will exit
        // when they try to send to a closed channel
        drop(self.sender.clone());
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
fn create_js_stream_iterator(stream: MessageStream) -> GrpcMessageStream {
    let (tx, rx) = mpsc::unbounded_channel();
    let tx_clone = tx.clone();

    // Spawn a task to forward messages from the stream to the channel
    tokio::spawn(async move {
        let mut stream = stream;
        while let Some(result) = stream.next().await {
            match result {
                Ok(bytes) => {
                    if tx.send(Ok(bytes)).is_err() {
                        // Receiver dropped, stop forwarding
                        break;
                    }
                }
                Err(status) => {
                    // Send error and stop
                    let _ = tx.send(Err(status.message().to_string()));
                    break;
                }
            }
        }
        // Channel will be closed when tx is dropped
    });

    GrpcMessageStream {
        receiver: Arc::new(tokio::sync::Mutex::new(rx)),
        sender: Arc::new(Some(tx_clone)),
    }
}

/// Consume a JavaScript async generator and collect all messages
///
/// This is a helper for server streaming and bidirectional streaming implementations.
/// Since napi-rs doesn't have built-in async generator support, we use a practical workaround:
/// - For now, we return an error indicating custom JavaScript implementation is needed
/// - A full implementation would require creating a ThreadsafeFunction for each next() call
#[allow(dead_code)]
async fn consume_js_async_generator() -> std::result::Result<Option<Buffer>, String> {
    // NOTE: This is a limitation of napi-rs with async generators
    // napi-rs doesn't provide direct support for iterating JavaScript async generators
    // from Rust code without calling back into the JavaScript runtime repeatedly.
    //
    // A working solution requires:
    // 1. Create a ThreadsafeFunction callback to call generator.next()
    // 2. Create a loop that awaits each Promise returned by next()
    // 3. Handle the {done: bool, value: any} protocol
    //
    // For a minimal implementation, we recommend:
    // - Handler returns an async generator in JavaScript
    // - That generator is consumed on the JavaScript side
    // - Results are passed back via a different mechanism
    //
    // This is documented as a known limitation in the Node.js binding guide
    Err("Async generator consumption requires handler-side implementation".to_string())
}

/// Node.js gRPC handler wrapper that implements spikard_http::grpc::GrpcHandler
///
/// Uses ThreadsafeFunction to call JavaScript handlers from Rust threads.
/// Converts between Rust's bytes/metadata types and JavaScript-friendly objects.
pub struct NodeGrpcHandler {
    service_name: Arc<str>,
    handler_fn: Arc<ThreadsafeFunction<GrpcRequest, Promise<GrpcResponse>, GrpcRequest, napi::Status, false>>,
}

unsafe impl Send for NodeGrpcHandler {}
unsafe impl Sync for NodeGrpcHandler {}

impl NodeGrpcHandler {
    /// Create a new Node gRPC handler wrapper with a JavaScript function
    ///
    /// # Arguments
    ///
    /// * `service_name` - Fully qualified service name (must be 'static)
    /// * `handler_fn` - ThreadsafeFunction that calls JavaScript handler
    pub fn new(
        service_name: Arc<str>,
        handler_fn: ThreadsafeFunction<GrpcRequest, Promise<GrpcResponse>, GrpcRequest, napi::Status, false>,
    ) -> Self {
        Self {
            service_name,
            handler_fn: Arc::new(handler_fn),
        }
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
                                            service_name,
                                            e
                                        )))
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
                        Err(e) => {
                            Err(tonic::Status::internal(format!(
                                "Handler promise failed for {}: {:?}", service_name, e
                            )))
                        }
                    }
                }
                Err(e) => {
                    Err(tonic::Status::internal(format!(
                        "Handler call failed for {}: {}", service_name, e
                    )))
                }
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

            // For server streaming, we expect the JavaScript handler to directly return
            // a GrpcResponse (not a stream). The handler should be responsible for
            // collecting all messages from the stream and returning a single response.
            //
            // This limitation exists because napi-rs doesn't have built-in support for
            // passing async generators across the FFI boundary.
            //
            // Implementation strategy for JavaScript handlers:
            // 1. Server streaming handler receives GrpcRequest
            // 2. Handler yields a GrpcResponse containing streamed data (e.g., in a repeated field)
            // 3. Rust converts the response back to a stream
            //
            // For true streaming (message-by-message), use the following pattern:
            // - Create a separate ThreadsafeFunction that can be called repeatedly
            // - Pass this callback to the handler
            // - Handler calls the callback for each message

            // We'll use a channel to collect messages from responses
            let (tx, mut rx) = mpsc::channel::<std::result::Result<Bytes, tonic::Status>>(MAX_STREAM_MESSAGES);

            // Call the JavaScript handler
            match handler_fn.call_async(js_request).await {
                Ok(promise) => {
                    tokio::spawn(async move {
                        match promise.await {
                            Ok(_response) => {
                                // For now, indicate that server streaming requires
                                // a different implementation pattern
                                let _ = tx.try_send(Err(tonic::Status::unimplemented(
                                    "Server streaming requires JavaScript handler to implement \
                                    streaming via callback or return pre-collected messages"
                                )));
                            }
                            Err(e) => {
                                let _ = tx.try_send(Err(tonic::Status::internal(format!(
                                    "Handler promise failed for {}: {}", service_name, e
                                ))));
                            }
                        }
                    });
                }
                Err(e) => {
                    let _ = tx.try_send(Err(tonic::Status::internal(format!(
                        "Handler call failed for {}: {}", service_name, e
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
        let _handler_fn = self.handler_fn.clone();

        Box::pin(async move {
            // Create a JavaScript async iterator from the message stream
            let _js_stream = create_js_stream_iterator(request.message_stream);

            // For client streaming, the challenge is passing the stream object to JavaScript
            // napi-rs doesn't support passing GrpcMessageStream objects directly through ThreadsafeFunction
            //
            // Implementation strategy:
            // 1. Create a GrpcMessageStream (done above)
            // 2. Register it as a global callback or store in a thread-local
            // 3. Have the handler call a getter function to retrieve it
            // 4. Handler consumes the stream and returns a GrpcResponse
            //
            // For now, this returns UNIMPLEMENTED to guide users to implement
            // client-side collection (handler collects all messages and returns response)

            Err(tonic::Status::unimplemented(
                "Client streaming for Node.js requires handler implementation: \
                collect all client messages in your handler and return a single GrpcResponse"
            ))
        })
    }

    fn call_bidi_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = std::result::Result<MessageStream, tonic::Status>> + Send>> {
        let _handler_fn = self.handler_fn.clone();

        Box::pin(async move {
            // Create a JavaScript async iterator from the incoming message stream
            let _js_stream = create_js_stream_iterator(request.message_stream);

            // For bidirectional streaming, we need to:
            // 1. Pass the input stream to the handler
            // 2. Get back an async generator
            // 3. Convert that generator to a MessageStream
            //
            // This is challenging with napi-rs because:
            // - We can't pass GrpcMessageStream through ThreadsafeFunction directly
            // - We can't return async generators from ThreadsafeFunction
            //
            // Implementation strategy:
            // - Store the input stream in a thread-local or global registry
            // - Pass a stream ID to the handler
            // - Handler calls getter functions to read from input stream
            // - Handler calls setter functions to write to output stream
            // - Rust collects all output messages into a channel-based stream

            Err(tonic::Status::unimplemented(
                "Bidirectional streaming for Node.js requires handler implementation: \
                use message collection pattern similar to client streaming"
            ))
        })
    }
}

impl Clone for NodeGrpcHandler {
    fn clone(&self) -> Self {
        Self {
            service_name: self.service_name.clone(),
            handler_fn: self.handler_fn.clone(),
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

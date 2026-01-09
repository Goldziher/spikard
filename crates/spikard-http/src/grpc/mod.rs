//! gRPC runtime support for Spikard
//!
//! This module provides gRPC server infrastructure using Tonic, enabling
//! Spikard to handle both HTTP/1.1 REST requests and HTTP/2 gRPC requests.
//!
//! # Architecture
//!
//! The gRPC support follows the same language-agnostic pattern as the HTTP handler:
//!
//! 1. **GrpcHandler trait**: Language-agnostic interface for handling gRPC requests
//! 2. **Service bridge**: Converts between Tonic's types and our internal representation
//! 3. **Streaming support**: Utilities for handling streaming RPCs
//! 4. **Server integration**: Multiplexes HTTP/1.1 and HTTP/2 traffic
//!
//! # Example
//!
//! ```ignore
//! use spikard_http::grpc::{GrpcHandler, GrpcRequestData, GrpcResponseData};
//! use std::sync::Arc;
//!
//! // Implement GrpcHandler for your language binding
//! struct MyGrpcHandler;
//!
//! impl GrpcHandler for MyGrpcHandler {
//!     fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
//!         Box::pin(async move {
//!             // Handle the gRPC request
//!             Ok(GrpcResponseData {
//!                 payload: bytes::Bytes::from("response"),
//!                 metadata: tonic::metadata::MetadataMap::new(),
//!             })
//!         })
//!     }
//!
//!     fn service_name(&self) -> &str {
//!         "mypackage.MyService"
//!     }
//! }
//!
//! // Register with the server
//! let handler = Arc::new(MyGrpcHandler);
//! let config = GrpcConfig::default();
//! ```

pub mod framing;
pub mod handler;
pub mod service;
pub mod streaming;

// Re-export main types
pub use framing::parse_grpc_client_stream;
pub use handler::{GrpcHandler, GrpcHandlerResult, GrpcRequestData, GrpcResponseData, RpcMode};
pub use service::{GenericGrpcService, copy_metadata, is_grpc_request, parse_grpc_path};
pub use streaming::{MessageStream, StreamingRequest, StreamingResponse};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Configuration for gRPC support
///
/// Controls how the server handles gRPC requests, including compression,
/// timeouts, and protocol settings.
///
/// # Stream Limits
///
/// This configuration enforces message-level size limits but delegates
/// concurrent stream limiting to the HTTP/2 transport layer:
///
/// - **Message Size Limits**: The `max_message_size` field is enforced per
///   individual message (request or response) in both unary and streaming RPCs.
///   When a single message exceeds this limit, the request is rejected with
///   `PAYLOAD_TOO_LARGE` (HTTP 413).
///
/// - **Concurrent Stream Limits**: The `max_concurrent_streams` is an advisory
///   configuration passed to the HTTP/2 layer for connection-level stream
///   negotiation. The HTTP/2 transport automatically enforces this limit and
///   returns GOAWAY frames when exceeded. Applications should not rely on
///   custom enforcement of this limit.
///
/// - **Stream Length Limits**: There is currently no built-in limit on the
///   total number of messages in a stream. Handlers should implement their own
///   message counting if needed. Future versions may add a `max_stream_response_bytes`
///   field to limit total response size per stream.
///
/// # Example
///
/// ```ignore
/// let mut config = GrpcConfig::default();
/// config.max_message_size = 10 * 1024 * 1024; // 10MB per message
/// config.max_concurrent_streams = 50; // Advised to HTTP/2 layer
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcConfig {
    /// Enable gRPC support
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Maximum message size in bytes (for both sending and receiving)
    ///
    /// This limit applies to individual messages in both unary and streaming RPCs.
    /// When a single message exceeds this size, the request is rejected with HTTP 413
    /// (Payload Too Large).
    ///
    /// Default: 4MB (4194304 bytes)
    ///
    /// # Note
    /// This limit does NOT apply to the total response size in streaming RPCs.
    /// For multi-message streams, the total response can exceed this limit as long
    /// as each individual message stays within the limit.
    #[serde(default = "default_max_message_size")]
    pub max_message_size: usize,

    /// Enable gzip compression for gRPC messages
    #[serde(default = "default_true")]
    pub enable_compression: bool,

    /// Timeout for gRPC requests in seconds (None = no timeout)
    #[serde(default)]
    pub request_timeout: Option<u64>,

    /// Maximum number of concurrent streams per connection (HTTP/2 advisory)
    ///
    /// This value is communicated to HTTP/2 clients as the server's flow control limit.
    /// The HTTP/2 transport layer enforces this limit automatically via SETTINGS frames
    /// and GOAWAY responses. Applications should NOT implement custom enforcement.
    ///
    /// Default: 100 streams per connection
    ///
    /// # Stream Limiting Strategy
    /// - **Per Connection**: This limit applies per HTTP/2 connection, not globally
    /// - **Transport Enforcement**: HTTP/2 handles all stream limiting; applications
    ///   need not implement custom checks
    /// - **Streaming Requests**: In server streaming or bidi streaming, each logical
    ///   RPC consumes one stream slot. Message ordering within a stream follows
    ///   HTTP/2 frame ordering.
    ///
    /// # Future Enhancement
    /// A future `max_stream_response_bytes` field may be added to limit the total
    /// response size in streaming RPCs (separate from per-message limits).
    #[serde(default = "default_max_concurrent_streams")]
    pub max_concurrent_streams: u32,

    /// Enable HTTP/2 keepalive
    #[serde(default = "default_true")]
    pub enable_keepalive: bool,

    /// HTTP/2 keepalive interval in seconds
    #[serde(default = "default_keepalive_interval")]
    pub keepalive_interval: u64,

    /// HTTP/2 keepalive timeout in seconds
    #[serde(default = "default_keepalive_timeout")]
    pub keepalive_timeout: u64,
    // TODO: Consider adding in future versions:
    // pub max_stream_response_bytes: Option<usize>,  // Total bytes per streaming response
}

impl Default for GrpcConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_message_size: default_max_message_size(),
            enable_compression: true,
            request_timeout: None,
            max_concurrent_streams: default_max_concurrent_streams(),
            enable_keepalive: true,
            keepalive_interval: default_keepalive_interval(),
            keepalive_timeout: default_keepalive_timeout(),
        }
    }
}

const fn default_true() -> bool {
    true
}

const fn default_max_message_size() -> usize {
    4 * 1024 * 1024 // 4MB
}

const fn default_max_concurrent_streams() -> u32 {
    100
}

const fn default_keepalive_interval() -> u64 {
    75 // seconds
}

const fn default_keepalive_timeout() -> u64 {
    20 // seconds
}

/// Registry for gRPC handlers
///
/// Maps service names to their handlers and RPC modes. Used by the server to route
/// incoming gRPC requests to the appropriate handler method based on RPC mode.
///
/// # Example
///
/// ```ignore
/// use spikard_http::grpc::{GrpcRegistry, RpcMode};
/// use std::sync::Arc;
///
/// let mut registry = GrpcRegistry::new();
/// registry.register("mypackage.UserService", Arc::new(user_handler), RpcMode::Unary);
/// registry.register("mypackage.StreamService", Arc::new(stream_handler), RpcMode::ServerStreaming);
/// ```
type GrpcHandlerEntry = (Arc<dyn GrpcHandler>, RpcMode);

#[derive(Clone)]
pub struct GrpcRegistry {
    handlers: Arc<HashMap<String, GrpcHandlerEntry>>,
}

impl GrpcRegistry {
    /// Create a new empty gRPC handler registry
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(HashMap::new()),
        }
    }

    /// Register a gRPC handler for a service
    ///
    /// # Arguments
    ///
    /// * `service_name` - Fully qualified service name (e.g., "mypackage.MyService")
    /// * `handler` - Handler implementation for this service
    /// * `rpc_mode` - The RPC mode this handler supports (Unary, ServerStreaming, etc.)
    pub fn register(&mut self, service_name: impl Into<String>, handler: Arc<dyn GrpcHandler>, rpc_mode: RpcMode) {
        let handlers = Arc::make_mut(&mut self.handlers);
        handlers.insert(service_name.into(), (handler, rpc_mode));
    }

    /// Get a handler and its RPC mode by service name
    ///
    /// Returns both the handler and the RPC mode it was registered with,
    /// allowing the router to dispatch to the appropriate handler method.
    pub fn get(&self, service_name: &str) -> Option<(Arc<dyn GrpcHandler>, RpcMode)> {
        self.handlers.get(service_name).cloned()
    }

    /// Get all registered service names
    pub fn service_names(&self) -> Vec<String> {
        self.handlers.keys().cloned().collect()
    }

    /// Check if a service is registered
    pub fn contains(&self, service_name: &str) -> bool {
        self.handlers.contains_key(service_name)
    }

    /// Get the number of registered services
    pub fn len(&self) -> usize {
        self.handlers.len()
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.handlers.is_empty()
    }
}

impl Default for GrpcRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grpc::handler::{GrpcHandler, GrpcHandlerResult, GrpcRequestData};
    use std::future::Future;
    use std::pin::Pin;

    struct TestHandler;

    impl GrpcHandler for TestHandler {
        fn call(&self, _request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
            Box::pin(async {
                Ok(GrpcResponseData {
                    payload: bytes::Bytes::new(),
                    metadata: tonic::metadata::MetadataMap::new(),
                })
            })
        }

        fn service_name(&self) -> &'static str {
            // Since we can't return a reference to self.0 with 'static lifetime,
            // we need to use a workaround. In real usage, service names should be static.
            "test.Service"
        }
    }

    #[test]
    fn test_grpc_config_default() {
        let config = GrpcConfig::default();
        assert!(config.enabled);
        assert_eq!(config.max_message_size, 4 * 1024 * 1024);
        assert!(config.enable_compression);
        assert!(config.request_timeout.is_none());
        assert_eq!(config.max_concurrent_streams, 100);
        assert!(config.enable_keepalive);
        assert_eq!(config.keepalive_interval, 75);
        assert_eq!(config.keepalive_timeout, 20);
    }

    #[test]
    fn test_grpc_config_serialization() {
        let config = GrpcConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: GrpcConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.enabled, deserialized.enabled);
        assert_eq!(config.max_message_size, deserialized.max_message_size);
        assert_eq!(config.enable_compression, deserialized.enable_compression);
    }

    #[test]
    fn test_grpc_registry_new() {
        let registry = GrpcRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn test_grpc_registry_register() {
        let mut registry = GrpcRegistry::new();
        let handler = Arc::new(TestHandler);

        registry.register("test.Service", handler, RpcMode::Unary);

        assert!(!registry.is_empty());
        assert_eq!(registry.len(), 1);
        assert!(registry.contains("test.Service"));
    }

    #[test]
    fn test_grpc_registry_get() {
        let mut registry = GrpcRegistry::new();
        let handler = Arc::new(TestHandler);

        registry.register("test.Service", handler, RpcMode::Unary);

        let retrieved = registry.get("test.Service");
        assert!(retrieved.is_some());
        let (handler, rpc_mode) = retrieved.unwrap();
        assert_eq!(handler.service_name(), "test.Service");
        assert_eq!(rpc_mode, RpcMode::Unary);
    }

    #[test]
    fn test_grpc_registry_get_nonexistent() {
        let registry = GrpcRegistry::new();
        let result = registry.get("nonexistent.Service");
        assert!(result.is_none());
    }

    #[test]
    fn test_grpc_registry_service_names() {
        let mut registry = GrpcRegistry::new();

        registry.register("service1", Arc::new(TestHandler), RpcMode::Unary);
        registry.register("service2", Arc::new(TestHandler), RpcMode::ServerStreaming);
        registry.register("service3", Arc::new(TestHandler), RpcMode::Unary);

        let mut names = registry.service_names();
        names.sort();

        assert_eq!(names, vec!["service1", "service2", "service3"]);
    }

    #[test]
    fn test_grpc_registry_contains() {
        let mut registry = GrpcRegistry::new();
        registry.register("test.Service", Arc::new(TestHandler), RpcMode::Unary);

        assert!(registry.contains("test.Service"));
        assert!(!registry.contains("other.Service"));
    }

    #[test]
    fn test_grpc_registry_multiple_services() {
        let mut registry = GrpcRegistry::new();

        registry.register("user.Service", Arc::new(TestHandler), RpcMode::Unary);
        registry.register("post.Service", Arc::new(TestHandler), RpcMode::ServerStreaming);

        assert_eq!(registry.len(), 2);
        assert!(registry.contains("user.Service"));
        assert!(registry.contains("post.Service"));
    }

    #[test]
    fn test_grpc_registry_clone() {
        let mut registry = GrpcRegistry::new();
        registry.register("test.Service", Arc::new(TestHandler), RpcMode::Unary);

        let cloned = registry.clone();

        assert_eq!(cloned.len(), 1);
        assert!(cloned.contains("test.Service"));
    }

    #[test]
    fn test_grpc_registry_default() {
        let registry = GrpcRegistry::default();
        assert!(registry.is_empty());
    }

    #[test]
    fn test_grpc_registry_rpc_mode_storage() {
        let mut registry = GrpcRegistry::new();

        registry.register("unary.Service", Arc::new(TestHandler), RpcMode::Unary);
        registry.register("server_stream.Service", Arc::new(TestHandler), RpcMode::ServerStreaming);
        registry.register("client_stream.Service", Arc::new(TestHandler), RpcMode::ClientStreaming);
        registry.register("bidi.Service", Arc::new(TestHandler), RpcMode::BidirectionalStreaming);

        let (_, mode) = registry.get("unary.Service").unwrap();
        assert_eq!(mode, RpcMode::Unary);

        let (_, mode) = registry.get("server_stream.Service").unwrap();
        assert_eq!(mode, RpcMode::ServerStreaming);

        let (_, mode) = registry.get("client_stream.Service").unwrap();
        assert_eq!(mode, RpcMode::ClientStreaming);

        let (_, mode) = registry.get("bidi.Service").unwrap();
        assert_eq!(mode, RpcMode::BidirectionalStreaming);
    }
}

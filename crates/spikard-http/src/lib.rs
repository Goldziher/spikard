#![allow(clippy::pedantic, clippy::nursery)]
#![cfg_attr(test, allow(clippy::all))]
//! Spikard HTTP Server
//!
//! Pure Rust HTTP server with language-agnostic handler trait.
//! Language bindings (Python, Node, WASM) implement the Handler trait.

pub mod auth;
pub mod background;
pub mod bindings;
pub mod body_metadata;
pub mod cors;
pub mod debug;
#[cfg(feature = "di")]
pub mod di_handler;
pub mod grpc;
pub mod handler_response;
pub mod handler_trait;
pub mod jsonrpc;
pub mod lifecycle;
pub mod middleware;
pub mod openapi;
pub mod query_parser;
pub mod response;
pub mod server;
pub mod sse;
pub mod testing;
pub mod websocket;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod handler_trait_tests;

pub use auth::{Claims, api_key_auth_middleware, jwt_auth_middleware};
pub use background::{
    BackgroundHandle, BackgroundJobError, BackgroundJobMetadata, BackgroundRuntime, BackgroundSpawnError,
    BackgroundTaskConfig,
};
pub use body_metadata::ResponseBodySize;
#[cfg(feature = "di")]
pub use di_handler::DependencyInjectingHandler;
pub use grpc::{
    GrpcConfig, GrpcHandler, GrpcHandlerResult, GrpcRegistry, GrpcRequestData, GrpcResponseData, MessageStream,
    StreamingRequest, StreamingResponse,
};
pub use handler_response::HandlerResponse;
pub use handler_trait::{Handler, HandlerResult, RequestData, ValidatedParams};
pub use jsonrpc::JsonRpcConfig;
pub use lifecycle::{HookResult, LifecycleHook, LifecycleHooks, LifecycleHooksBuilder, request_hook, response_hook};
pub use openapi::{ContactInfo, LicenseInfo, OpenApiConfig, SecuritySchemeInfo, ServerInfo};
pub use response::Response;
pub use server::Server;
pub use spikard_core::{
    CompressionConfig, CorsConfig, Method, ParameterValidator, ProblemDetails, RateLimitConfig, Route, RouteHandler,
    RouteMetadata, Router, SchemaRegistry, SchemaValidator,
};
pub use sse::{SseEvent, SseEventProducer, SseState, sse_handler};
pub use testing::{ResponseSnapshot, SnapshotError, snapshot_response};
pub use websocket::{WebSocketHandler, WebSocketState, websocket_handler};

/// Reexport from spikard_core for convenience
pub use spikard_core::problem::CONTENT_TYPE_PROBLEM_JSON;

/// JWT authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    /// Secret key for JWT verification
    pub secret: String,
    /// Required algorithm (HS256, HS384, HS512, RS256, etc.)
    #[serde(default = "default_jwt_algorithm")]
    pub algorithm: String,
    /// Required audience claim
    pub audience: Option<Vec<String>>,
    /// Required issuer claim
    pub issuer: Option<String>,
    /// Leeway for expiration checks (seconds)
    #[serde(default)]
    pub leeway: u64,
}

fn default_jwt_algorithm() -> String {
    "HS256".to_string()
}

/// API Key authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyConfig {
    /// Valid API keys
    pub keys: Vec<String>,
    /// Header name to check (e.g., "X-API-Key")
    #[serde(default = "default_api_key_header")]
    pub header_name: String,
}

fn default_api_key_header() -> String {
    "X-API-Key".to_string()
}

/// Static file serving configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticFilesConfig {
    /// Directory path to serve
    pub directory: String,
    /// URL path prefix (e.g., "/static")
    pub route_prefix: String,
    /// Fallback to index.html for directories
    #[serde(default = "default_true")]
    pub index_file: bool,
    /// Cache-Control header value
    pub cache_control: Option<String>,
}

/// Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Host to bind to
    pub host: String,
    /// Port to bind to
    pub port: u16,
    /// Number of worker threads (unused with tokio)
    pub workers: usize,

    /// Enable request ID generation and propagation
    pub enable_request_id: bool,
    /// Maximum request body size in bytes (None = unlimited, not recommended)
    pub max_body_size: Option<usize>,
    /// Request timeout in seconds (None = no timeout)
    pub request_timeout: Option<u64>,
    /// Enable compression middleware
    pub compression: Option<CompressionConfig>,
    /// Enable rate limiting
    pub rate_limit: Option<RateLimitConfig>,
    /// JWT authentication configuration
    pub jwt_auth: Option<JwtConfig>,
    /// API Key authentication configuration
    pub api_key_auth: Option<ApiKeyConfig>,
    /// Static file serving configuration
    pub static_files: Vec<StaticFilesConfig>,
    /// Enable graceful shutdown on SIGTERM/SIGINT
    pub graceful_shutdown: bool,
    /// Graceful shutdown timeout (seconds)
    pub shutdown_timeout: u64,
    /// OpenAPI documentation configuration
    pub openapi: Option<crate::openapi::OpenApiConfig>,
    /// JSON-RPC configuration
    pub jsonrpc: Option<crate::jsonrpc::JsonRpcConfig>,
    /// gRPC configuration
    pub grpc: Option<crate::grpc::GrpcConfig>,
    /// Lifecycle hooks for request/response processing
    pub lifecycle_hooks: Option<std::sync::Arc<LifecycleHooks>>,
    /// Background task executor configuration
    pub background_tasks: BackgroundTaskConfig,
    /// Enable per-request HTTP tracing (tower-http `TraceLayer`)
    pub enable_http_trace: bool,
    /// Dependency injection container (requires 'di' feature)
    #[cfg(feature = "di")]
    pub di_container: Option<std::sync::Arc<spikard_core::di::DependencyContainer>>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8000,
            workers: 1,
            enable_request_id: false,
            max_body_size: Some(10 * 1024 * 1024),
            request_timeout: None,
            compression: None,
            rate_limit: None,
            jwt_auth: None,
            api_key_auth: None,
            static_files: Vec::new(),
            graceful_shutdown: true,
            shutdown_timeout: 30,
            openapi: None,
            jsonrpc: None,
            grpc: None,
            lifecycle_hooks: None,
            background_tasks: BackgroundTaskConfig::default(),
            enable_http_trace: false,
            #[cfg(feature = "di")]
            di_container: None,
        }
    }
}

impl ServerConfig {
    /// Create a new builder for ServerConfig
    ///
    /// # Example
    ///
    /// ```ignorerust
    /// use spikard_http::ServerConfig;
    ///
    /// let config = ServerConfig::builder()
    ///     .port(3000)
    ///     .host("0.0.0.0")
    ///     .build();
    /// ```
    pub fn builder() -> ServerConfigBuilder {
        ServerConfigBuilder::default()
    }
}

/// Builder for ServerConfig
///
/// Provides a fluent API for configuring a Spikard server with dependency injection support.
///
/// # Dependency Injection
///
/// The builder provides methods to register dependencies that will be injected into handlers:
///
/// ```ignorerust
/// # #[cfg(feature = "di")]
/// # {
/// use spikard_http::ServerConfig;
/// use std::sync::Arc;
///
/// let config = ServerConfig::builder()
///     .port(3000)
///     .provide_value("app_name", "MyApp".to_string())
///     .provide_value("max_connections", 100)
///     .build();
/// # }
/// ```
///
/// For factory dependencies that create values on-demand:
///
/// ```ignorerust
/// # #[cfg(feature = "di")]
/// # {
/// use spikard_http::ServerConfig;
///
/// let config = ServerConfig::builder()
///     .port(3000)
///     .provide_value("db_url", "postgresql://localhost/mydb".to_string())
///     .build();
/// # }
/// ```
#[derive(Debug, Clone, Default)]
pub struct ServerConfigBuilder {
    config: ServerConfig,
}

impl ServerConfigBuilder {
    /// Set the host address to bind to
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.config.host = host.into();
        self
    }

    /// Set the port to bind to
    pub fn port(mut self, port: u16) -> Self {
        self.config.port = port;
        self
    }

    /// Set the number of worker threads (unused with tokio, kept for compatibility)
    pub fn workers(mut self, workers: usize) -> Self {
        self.config.workers = workers;
        self
    }

    /// Enable or disable request ID generation and propagation
    pub fn enable_request_id(mut self, enable: bool) -> Self {
        self.config.enable_request_id = enable;
        self
    }

    /// Enable or disable per-request HTTP tracing (tower-http `TraceLayer`)
    pub fn enable_http_trace(mut self, enable: bool) -> Self {
        self.config.enable_http_trace = enable;
        self
    }

    /// Set maximum request body size in bytes (None = unlimited, not recommended)
    pub fn max_body_size(mut self, size: Option<usize>) -> Self {
        self.config.max_body_size = size;
        self
    }

    /// Set request timeout in seconds (None = no timeout)
    pub fn request_timeout(mut self, timeout: Option<u64>) -> Self {
        self.config.request_timeout = timeout;
        self
    }

    /// Set compression configuration
    pub fn compression(mut self, compression: Option<CompressionConfig>) -> Self {
        self.config.compression = compression;
        self
    }

    /// Set rate limiting configuration
    pub fn rate_limit(mut self, rate_limit: Option<RateLimitConfig>) -> Self {
        self.config.rate_limit = rate_limit;
        self
    }

    /// Set JWT authentication configuration
    pub fn jwt_auth(mut self, jwt_auth: Option<JwtConfig>) -> Self {
        self.config.jwt_auth = jwt_auth;
        self
    }

    /// Set API key authentication configuration
    pub fn api_key_auth(mut self, api_key_auth: Option<ApiKeyConfig>) -> Self {
        self.config.api_key_auth = api_key_auth;
        self
    }

    /// Add static file serving configuration
    pub fn static_files(mut self, static_files: Vec<StaticFilesConfig>) -> Self {
        self.config.static_files = static_files;
        self
    }

    /// Add a single static file serving configuration
    pub fn add_static_files(mut self, static_file: StaticFilesConfig) -> Self {
        self.config.static_files.push(static_file);
        self
    }

    /// Enable or disable graceful shutdown on SIGTERM/SIGINT
    pub fn graceful_shutdown(mut self, enable: bool) -> Self {
        self.config.graceful_shutdown = enable;
        self
    }

    /// Set graceful shutdown timeout in seconds
    pub fn shutdown_timeout(mut self, timeout: u64) -> Self {
        self.config.shutdown_timeout = timeout;
        self
    }

    /// Set OpenAPI documentation configuration
    pub fn openapi(mut self, openapi: Option<crate::openapi::OpenApiConfig>) -> Self {
        self.config.openapi = openapi;
        self
    }

    /// Set JSON-RPC configuration
    pub fn jsonrpc(mut self, jsonrpc: Option<crate::jsonrpc::JsonRpcConfig>) -> Self {
        self.config.jsonrpc = jsonrpc;
        self
    }

    /// Set gRPC configuration
    pub fn grpc(mut self, grpc: Option<crate::grpc::GrpcConfig>) -> Self {
        self.config.grpc = grpc;
        self
    }

    /// Set lifecycle hooks for request/response processing
    pub fn lifecycle_hooks(mut self, hooks: Option<std::sync::Arc<LifecycleHooks>>) -> Self {
        self.config.lifecycle_hooks = hooks;
        self
    }

    /// Set background task executor configuration
    pub fn background_tasks(mut self, config: BackgroundTaskConfig) -> Self {
        self.config.background_tasks = config;
        self
    }

    /// Register a value dependency (like Fastify decorate)
    ///
    /// Value dependencies are static values that are cloned when injected into handlers.
    /// Use this for configuration objects, constants, or small shared state.
    ///
    /// # Example
    ///
    /// ```ignorerust
    /// # #[cfg(feature = "di")]
    /// # {
    /// use spikard_http::ServerConfig;
    ///
    /// let config = ServerConfig::builder()
    ///     .provide_value("app_name", "MyApp".to_string())
    ///     .provide_value("version", "1.0.0".to_string())
    ///     .provide_value("max_connections", 100)
    ///     .build();
    /// # }
    /// ```
    #[cfg(feature = "di")]
    pub fn provide_value<T: Clone + Send + Sync + 'static>(mut self, key: impl Into<String>, value: T) -> Self {
        use spikard_core::di::{DependencyContainer, ValueDependency};
        use std::sync::Arc;

        let key_str = key.into();

        let container = if let Some(container) = self.config.di_container.take() {
            Arc::try_unwrap(container).unwrap_or_else(|_arc| DependencyContainer::new())
        } else {
            DependencyContainer::new()
        };

        let mut container = container;

        let dep = ValueDependency::new(key_str.clone(), value);

        container
            .register(key_str, Arc::new(dep))
            .expect("Failed to register dependency");

        self.config.di_container = Some(Arc::new(container));
        self
    }

    /// Register a factory dependency (like Litestar Provide)
    ///
    /// Factory dependencies create values on-demand, optionally depending on other
    /// registered dependencies. Factories are async and have access to resolved dependencies.
    ///
    /// # Type Parameters
    ///
    /// * `F` - Factory function type
    /// * `Fut` - Future returned by the factory
    /// * `T` - Type of value produced by the factory
    ///
    /// # Arguments
    ///
    /// * `key` - Unique identifier for this dependency
    /// * `factory` - Async function that creates the dependency value
    ///
    /// # Example
    ///
    /// ```ignorerust
    /// # #[cfg(feature = "di")]
    /// # {
    /// use spikard_http::ServerConfig;
    /// use std::sync::Arc;
    ///
    /// let config = ServerConfig::builder()
    ///     .provide_value("db_url", "postgresql://localhost/mydb".to_string())
    ///     .provide_factory("db_pool", |resolved| async move {
    ///         let url: Arc<String> = resolved.get("db_url").ok_or("Missing db_url")?;
    ///         // Create database pool...
    ///         Ok(format!("Pool: {}", url))
    ///     })
    ///     .build();
    /// # }
    /// ```
    #[cfg(feature = "di")]
    pub fn provide_factory<F, Fut, T>(mut self, key: impl Into<String>, factory: F) -> Self
    where
        F: Fn(&spikard_core::di::ResolvedDependencies) -> Fut + Send + Sync + Clone + 'static,
        Fut: std::future::Future<Output = Result<T, String>> + Send + 'static,
        T: Send + Sync + 'static,
    {
        use futures::future::BoxFuture;
        use spikard_core::di::{DependencyContainer, DependencyError, FactoryDependency};
        use std::sync::Arc;

        let key_str = key.into();

        let container = if let Some(container) = self.config.di_container.take() {
            Arc::try_unwrap(container).unwrap_or_else(|_| DependencyContainer::new())
        } else {
            DependencyContainer::new()
        };

        let mut container = container;

        let factory_clone = factory.clone();

        let dep = FactoryDependency::builder(key_str.clone())
            .factory(
                move |_req: &axum::http::Request<()>,
                      _data: &spikard_core::RequestData,
                      resolved: &spikard_core::di::ResolvedDependencies| {
                    let factory = factory_clone.clone();
                    let factory_result = factory(resolved);
                    Box::pin(async move {
                        let result = factory_result
                            .await
                            .map_err(|e| DependencyError::ResolutionFailed { message: e })?;
                        Ok(Arc::new(result) as Arc<dyn std::any::Any + Send + Sync>)
                    })
                        as BoxFuture<'static, Result<Arc<dyn std::any::Any + Send + Sync>, DependencyError>>
                },
            )
            .build();

        container
            .register(key_str, Arc::new(dep))
            .expect("Failed to register dependency");

        self.config.di_container = Some(Arc::new(container));
        self
    }

    /// Register a dependency with full control (advanced API)
    ///
    /// This method allows you to register custom dependency implementations
    /// that implement the `Dependency` trait. Use this for advanced use cases
    /// where you need fine-grained control over dependency resolution.
    ///
    /// # Example
    ///
    /// ```ignorerust
    /// # #[cfg(feature = "di")]
    /// # {
    /// use spikard_http::ServerConfig;
    /// use spikard_core::di::ValueDependency;
    /// use std::sync::Arc;
    ///
    /// let dep = ValueDependency::new("custom", "value".to_string());
    ///
    /// let config = ServerConfig::builder()
    ///     .provide(Arc::new(dep))
    ///     .build();
    /// # }
    /// ```
    #[cfg(feature = "di")]
    pub fn provide(mut self, dependency: std::sync::Arc<dyn spikard_core::di::Dependency>) -> Self {
        use spikard_core::di::DependencyContainer;
        use std::sync::Arc;

        let key = dependency.key().to_string();

        let container = if let Some(container) = self.config.di_container.take() {
            Arc::try_unwrap(container).unwrap_or_else(|_| DependencyContainer::new())
        } else {
            DependencyContainer::new()
        };

        let mut container = container;

        container
            .register(key, dependency)
            .expect("Failed to register dependency");

        self.config.di_container = Some(Arc::new(container));
        self
    }

    /// Build the ServerConfig
    pub fn build(self) -> ServerConfig {
        self.config
    }
}

const fn default_true() -> bool {
    true
}

#![allow(dead_code, unused_imports, unused_variables)]
#![allow(
    clippy::too_many_arguments,
    clippy::let_unit_value,
    clippy::needless_borrow,
    clippy::map_identity,
    clippy::just_underscores_and_digits
)]
#![cfg(feature = "extension-module")]
#![cfg_attr(
    all(windows, target_env = "msvc", feature = "extension-module"),
    feature(abi_vectorcall)
)]

use ext_php_rs::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\Claims")]
#[allow(clippy::similar_names)]
pub struct Claims {
    #[php(prop, name = "sub")]
    pub sub: String,
    #[php(prop, name = "exp")]
    pub exp: i64,
    #[php(prop, name = "iat")]
    pub iat: Option<i64>,
    #[php(prop, name = "nbf")]
    pub nbf: Option<i64>,
    #[php(prop, name = "aud")]
    pub aud: Option<Vec<String>>,
    #[php(prop, name = "iss")]
    pub iss: Option<String>,
}

#[php_impl]
impl Claims {
    pub fn __construct(
        sub: String,
        exp: i64,
        iat: Option<i64>,
        nbf: Option<i64>,
        aud: Option<Vec<String>>,
        iss: Option<String>,
    ) -> Self {
        Self {
            sub,
            exp,
            iat,
            nbf,
            aud,
            iss,
        }
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\BackgroundTaskConfig")]
pub struct BackgroundTaskConfig {
    #[php(prop, name = "max_queue_size")]
    pub max_queue_size: i64,
    #[php(prop, name = "max_concurrent_tasks")]
    pub max_concurrent_tasks: i64,
    #[php(prop, name = "drain_timeout_secs")]
    pub drain_timeout_secs: i64,
}

#[php_impl]
impl BackgroundTaskConfig {
    pub fn __construct(
        max_queue_size: Option<i64>,
        max_concurrent_tasks: Option<i64>,
        drain_timeout_secs: Option<i64>,
    ) -> Self {
        Self {
            max_queue_size: max_queue_size.unwrap_or(1024),
            max_concurrent_tasks: max_concurrent_tasks.unwrap_or(128),
            drain_timeout_secs: drain_timeout_secs.unwrap_or(30),
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> BackgroundTaskConfig {
        spikard_http::BackgroundTaskConfig::default().into()
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\BackgroundJobMetadata")]
pub struct BackgroundJobMetadata {
    #[php(prop, name = "name")]
    pub name: String,
    #[php(prop, name = "request_id")]
    pub request_id: Option<String>,
}

#[php_impl]
impl BackgroundJobMetadata {
    pub fn __construct(name: Option<String>, request_id: Option<String>) -> Self {
        Self {
            name: name.unwrap_or_default(),
            request_id,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> BackgroundJobMetadata {
        spikard_http::BackgroundJobMetadata::default().into()
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\BackgroundJobError")]
pub struct BackgroundJobError {
    #[php(prop, name = "message")]
    pub message: String,
}

#[php_impl]
impl BackgroundJobError {
    pub fn __construct(message: String) -> Self {
        Self { message }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from(message: String) -> BackgroundJobError {
        spikard_http::BackgroundJobError::from(message).into()
    }
}

#[derive(Clone)]
#[php_class]
#[php(name = "Spikard\\Php\\BackgroundHandle")]
pub struct BackgroundHandle {
    inner: Arc<spikard_http::BackgroundHandle>,
}

#[php_impl]
impl BackgroundHandle {}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\CorsConfig")]
pub struct CorsConfig {
    #[php(prop, name = "allowed_origins")]
    pub allowed_origins: Vec<String>,
    #[php(prop, name = "allowed_methods")]
    pub allowed_methods: Vec<String>,
    #[php(prop, name = "allowed_headers")]
    pub allowed_headers: Vec<String>,
    #[php(prop, name = "expose_headers")]
    pub expose_headers: Option<Vec<String>>,
    #[php(prop, name = "max_age")]
    pub max_age: Option<u32>,
    #[php(prop, name = "allow_credentials")]
    pub allow_credentials: Option<bool>,
    #[php(prop, name = "methods_joined_cache")]
    pub methods_joined_cache: String,
    #[php(prop, name = "headers_joined_cache")]
    pub headers_joined_cache: String,
}

#[php_impl]
impl CorsConfig {
    pub fn __construct(
        allowed_origins: Option<Vec<String>>,
        allowed_methods: Option<Vec<String>>,
        allowed_headers: Option<Vec<String>>,
        expose_headers: Option<Vec<String>>,
        max_age: Option<u32>,
        allow_credentials: Option<bool>,
        methods_joined_cache: Option<String>,
        headers_joined_cache: Option<String>,
    ) -> Self {
        Self {
            allowed_origins: allowed_origins.unwrap_or_default(),
            allowed_methods: allowed_methods.unwrap_or_default(),
            allowed_headers: allowed_headers.unwrap_or_default(),
            expose_headers,
            max_age,
            allow_credentials,
            methods_joined_cache: methods_joined_cache.unwrap_or_default(),
            headers_joined_cache: headers_joined_cache.unwrap_or_default(),
        }
    }

    pub fn allowed_methods_joined(&self) -> String {
        let core_self = spikard_http::CorsConfig {
            allowed_origins: self.allowed_origins.clone(),
            allowed_methods: self.allowed_methods.clone(),
            allowed_headers: self.allowed_headers.clone(),
            expose_headers: self.expose_headers.clone(),
            max_age: self.max_age,
            allow_credentials: self.allow_credentials,
            methods_joined_cache: Default::default(),
            headers_joined_cache: Default::default(),
        };
        core_self.allowed_methods_joined().into()
    }

    pub fn allowed_headers_joined(&self) -> String {
        let core_self = spikard_http::CorsConfig {
            allowed_origins: self.allowed_origins.clone(),
            allowed_methods: self.allowed_methods.clone(),
            allowed_headers: self.allowed_headers.clone(),
            expose_headers: self.expose_headers.clone(),
            max_age: self.max_age,
            allow_credentials: self.allow_credentials,
            methods_joined_cache: Default::default(),
            headers_joined_cache: Default::default(),
        };
        core_self.allowed_headers_joined().into()
    }

    pub fn is_origin_allowed(&self, origin: String) -> bool {
        let core_self = spikard_http::CorsConfig {
            allowed_origins: self.allowed_origins.clone(),
            allowed_methods: self.allowed_methods.clone(),
            allowed_headers: self.allowed_headers.clone(),
            expose_headers: self.expose_headers.clone(),
            max_age: self.max_age,
            allow_credentials: self.allow_credentials,
            methods_joined_cache: Default::default(),
            headers_joined_cache: Default::default(),
        };
        core_self.is_origin_allowed(&origin)
    }

    pub fn is_method_allowed(&self, method: String) -> bool {
        let core_self = spikard_http::CorsConfig {
            allowed_origins: self.allowed_origins.clone(),
            allowed_methods: self.allowed_methods.clone(),
            allowed_headers: self.allowed_headers.clone(),
            expose_headers: self.expose_headers.clone(),
            max_age: self.max_age,
            allow_credentials: self.allow_credentials,
            methods_joined_cache: Default::default(),
            headers_joined_cache: Default::default(),
        };
        core_self.is_method_allowed(&method)
    }

    pub fn are_headers_allowed(&self, requested: Vec<String>) -> bool {
        false
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> CorsConfig {
        spikard_http::CorsConfig::default().into()
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\RouteMetadata")]
pub struct RouteMetadata {
    #[php(prop, name = "method")]
    pub method: String,
    #[php(prop, name = "path")]
    pub path: String,
    #[php(prop, name = "handler_name")]
    pub handler_name: String,
    #[php(prop, name = "request_schema")]
    pub request_schema: Option<String>,
    #[php(prop, name = "response_schema")]
    pub response_schema: Option<String>,
    #[php(prop, name = "parameter_schema")]
    pub parameter_schema: Option<String>,
    #[php(prop, name = "file_params")]
    pub file_params: Option<String>,
    #[php(prop, name = "is_async")]
    pub is_async: bool,
    pub cors: Option<CorsConfig>,
    /// Name of the body parameter (defaults to "body" if not specified)
    #[php(prop, name = "body_param_name")]
    pub body_param_name: Option<String>,
    /// List of dependency keys this handler requires (for DI)
    #[php(prop, name = "handler_dependencies")]
    pub handler_dependencies: Option<Vec<String>>,
    /// JSON-RPC method metadata (if this route is exposed as a JSON-RPC method)
    #[php(prop, name = "jsonrpc_method")]
    pub jsonrpc_method: Option<String>,
    /// Optional static response configuration: `{"status": 200, "body": "OK", "content_type": "text/plain"}`
    /// When present, the handler is replaced by a `StaticResponseHandler` that bypasses the full
    /// middleware pipeline for maximum throughput.
    #[php(prop, name = "static_response")]
    pub static_response: Option<String>,
}

#[php_impl]
impl RouteMetadata {
    pub fn __construct() -> PhpResult<Self> {
        Err(PhpException::default(
            "Not implemented: constructor for RouteMetadata requires complex params".to_string(),
        ))
    }

    #[php(getter)]
    pub fn get_cors(&self) -> Option<CorsConfig> {
        self.cors.clone()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> RouteMetadata {
        spikard_http::RouteMetadata::default().into()
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\CompressionConfig")]
pub struct CompressionConfig {
    /// Enable gzip compression
    #[php(prop, name = "gzip")]
    pub gzip: bool,
    /// Enable brotli compression
    #[php(prop, name = "brotli")]
    pub brotli: bool,
    /// Minimum response size to compress (bytes)
    #[php(prop, name = "min_size")]
    pub min_size: i64,
    /// Compression quality (0-11 for brotli, 0-9 for gzip)
    #[php(prop, name = "quality")]
    pub quality: u32,
}

#[php_impl]
impl CompressionConfig {
    pub fn __construct(gzip: Option<bool>, brotli: Option<bool>, min_size: Option<i64>, quality: Option<u32>) -> Self {
        Self {
            gzip: gzip.unwrap_or(true),
            brotli: brotli.unwrap_or(true),
            min_size: min_size.unwrap_or_default(),
            quality: quality.unwrap_or_default(),
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> CompressionConfig {
        spikard_http::CompressionConfig::default().into()
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\RateLimitConfig")]
pub struct RateLimitConfig {
    /// Requests per second
    #[php(prop, name = "per_second")]
    pub per_second: i64,
    /// Burst allowance
    #[php(prop, name = "burst")]
    pub burst: u32,
    /// Use IP-based rate limiting
    #[php(prop, name = "ip_based")]
    pub ip_based: bool,
}

#[php_impl]
impl RateLimitConfig {
    pub fn __construct(per_second: Option<i64>, burst: Option<u32>, ip_based: Option<bool>) -> Self {
        Self {
            per_second: per_second.unwrap_or(100),
            burst: burst.unwrap_or(200),
            ip_based: ip_based.unwrap_or(true),
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> RateLimitConfig {
        spikard_http::RateLimitConfig::default().into()
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\ProblemDetails")]
pub struct ProblemDetails {
    /// A URI reference that identifies the problem type.
    /// Defaults to "about:blank" when absent.
    /// Should be a stable, human-readable identifier for the problem type.
    #[php(prop, name = "type_uri")]
    pub type_uri: String,
    /// A short, human-readable summary of the problem type.
    /// Should not change from occurrence to occurrence of the problem.
    #[php(prop, name = "title")]
    pub title: String,
    /// The HTTP status code generated by the origin server.
    /// This is advisory; the actual HTTP status code takes precedence.
    #[php(prop, name = "status")]
    pub status: u16,
    /// A human-readable explanation specific to this occurrence of the problem.
    #[php(prop, name = "detail")]
    pub detail: Option<String>,
    /// A URI reference that identifies the specific occurrence of the problem.
    /// It may or may not yield further information if dereferenced.
    #[php(prop, name = "instance")]
    pub instance: Option<String>,
    /// Extension members - problem-type-specific data.
    /// For validation errors, this typically contains an "errors" array.
    pub extensions: HashMap<String, String>,
}

#[php_impl]
impl ProblemDetails {
    pub fn __construct(
        type_uri: String,
        title: String,
        status: u16,
        extensions: HashMap<String, String>,
        detail: Option<String>,
        instance: Option<String>,
    ) -> Self {
        Self {
            type_uri,
            title,
            status,
            detail,
            instance,
            extensions,
        }
    }

    #[php(getter)]
    pub fn get_extensions(&self) -> HashMap<String, String> {
        self.extensions.clone()
    }

    pub fn with_detail(&self, detail: String) -> ProblemDetails {
        let core_self = spikard_http::ProblemDetails {
            type_uri: self.type_uri.clone(),
            title: self.title.clone(),
            status: self.status,
            detail: self.detail.clone(),
            instance: self.instance.clone(),
            extensions: Default::default(),
        };
        core_self.with_detail(detail).into()
    }

    pub fn with_instance(&self, instance: String) -> ProblemDetails {
        let core_self = spikard_http::ProblemDetails {
            type_uri: self.type_uri.clone(),
            title: self.title.clone(),
            status: self.status,
            detail: self.detail.clone(),
            instance: self.instance.clone(),
            extensions: Default::default(),
        };
        core_self.with_instance(instance).into()
    }

    pub fn with_extension(&self, key: String, value: String) -> ProblemDetails {
        panic!("alef: with_extension not auto-delegatable")
    }

    pub fn with_extensions(&self, extensions: String) -> ProblemDetails {
        panic!("alef: with_extensions not auto-delegatable")
    }

    pub fn status_code(&self) -> String {
        String::from("[unimplemented: status_code]")
    }

    pub fn to_json(&self) -> PhpResult<String> {
        let core_self = spikard_http::ProblemDetails {
            type_uri: self.type_uri.clone(),
            title: self.title.clone(),
            status: self.status,
            detail: self.detail.clone(),
            instance: self.instance.clone(),
            extensions: Default::default(),
        };
        let result = core_self
            .to_json()
            .map_err(|e| ext_php_rs::exception::PhpException::default(e.to_string()))?;
        Ok(result)
    }

    pub fn to_json_pretty(&self) -> PhpResult<String> {
        let core_self = spikard_http::ProblemDetails {
            type_uri: self.type_uri.clone(),
            title: self.title.clone(),
            status: self.status,
            detail: self.detail.clone(),
            instance: self.instance.clone(),
            extensions: Default::default(),
        };
        let result = core_self
            .to_json_pretty()
            .map_err(|e| ext_php_rs::exception::PhpException::default(e.to_string()))?;
        Ok(result)
    }

    pub fn from_validation_error(error: String) -> ProblemDetails {
        panic!("alef: from_validation_error not auto-delegatable")
    }

    pub fn not_found(detail: String) -> ProblemDetails {
        spikard_http::ProblemDetails::not_found(detail).into()
    }

    pub fn method_not_allowed(detail: String) -> ProblemDetails {
        spikard_http::ProblemDetails::method_not_allowed(detail).into()
    }

    pub fn internal_server_error(detail: String) -> ProblemDetails {
        spikard_http::ProblemDetails::internal_server_error(detail).into()
    }

    pub fn internal_server_error_debug(
        detail: String,
        exception: String,
        traceback: String,
        request_data: String,
    ) -> ProblemDetails {
        panic!("alef: internal_server_error_debug not auto-delegatable")
    }

    pub fn bad_request(detail: String) -> ProblemDetails {
        spikard_http::ProblemDetails::bad_request(detail).into()
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\JsonRpcMethodInfo")]
pub struct JsonRpcMethodInfo {
    /// The JSON-RPC method name (e.g., "user.create")
    #[php(prop, name = "method_name")]
    pub method_name: String,
    /// Optional description of what the method does
    #[php(prop, name = "description")]
    pub description: Option<String>,
    /// Optional JSON Schema for method parameters
    #[php(prop, name = "params_schema")]
    pub params_schema: Option<String>,
    /// Optional JSON Schema for the result
    #[php(prop, name = "result_schema")]
    pub result_schema: Option<String>,
    /// Whether this method is deprecated
    #[php(prop, name = "deprecated")]
    pub deprecated: bool,
    /// Tags for categorizing and grouping methods
    #[php(prop, name = "tags")]
    pub tags: Vec<String>,
}

#[php_impl]
impl JsonRpcMethodInfo {
    pub fn __construct(
        method_name: String,
        deprecated: bool,
        tags: Vec<String>,
        description: Option<String>,
        params_schema: Option<String>,
        result_schema: Option<String>,
    ) -> Self {
        Self {
            method_name,
            description,
            params_schema,
            result_schema,
            deprecated,
            tags,
        }
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\Route")]
pub struct Route {
    #[php(prop, name = "method")]
    pub method: String,
    #[php(prop, name = "path")]
    pub path: String,
    #[php(prop, name = "handler_name")]
    pub handler_name: String,
    #[php(prop, name = "request_validator")]
    pub request_validator: Option<String>,
    #[php(prop, name = "response_validator")]
    pub response_validator: Option<String>,
    #[php(prop, name = "parameter_validator")]
    pub parameter_validator: Option<String>,
    #[php(prop, name = "file_params")]
    pub file_params: Option<String>,
    #[php(prop, name = "is_async")]
    pub is_async: bool,
    pub cors: Option<CorsConfig>,
    /// Precomputed flag: true if this route expects a JSON request body
    /// Used by middleware to validate Content-Type headers
    #[php(prop, name = "expects_json_body")]
    pub expects_json_body: bool,
    /// List of dependency keys this handler requires (for DI)
    #[php(prop, name = "handler_dependencies")]
    pub handler_dependencies: Vec<String>,
    /// Optional JSON-RPC method information
    /// When present, this route can be exposed as a JSON-RPC method
    pub jsonrpc_method: Option<JsonRpcMethodInfo>,
}

#[php_impl]
impl Route {
    pub fn __construct() -> PhpResult<Self> {
        Err(PhpException::default(
            "Not implemented: constructor for Route requires complex params".to_string(),
        ))
    }

    #[php(getter)]
    pub fn get_cors(&self) -> Option<CorsConfig> {
        self.cors.clone()
    }

    #[php(getter)]
    pub fn get_jsonrpc_method(&self) -> Option<JsonRpcMethodInfo> {
        self.jsonrpc_method.clone()
    }

    pub fn with_jsonrpc_method(&self, info: &JsonRpcMethodInfo) -> Route {
        panic!("alef: with_jsonrpc_method not auto-delegatable")
    }

    pub fn is_jsonrpc_method(&self) -> bool {
        #[allow(clippy::needless_update)]
        let core_self = spikard_http::Route {
            method: match self.method.as_str() {
                "Get" => spikard_http::Method::Get,
                "Post" => spikard_http::Method::Post,
                "Put" => spikard_http::Method::Put,
                "Patch" => spikard_http::Method::Patch,
                "Delete" => spikard_http::Method::Delete,
                "Head" => spikard_http::Method::Head,
                "Options" => spikard_http::Method::Options,
                "Trace" => spikard_http::Method::Trace,
                _ => spikard_http::Method::Get,
            },
            path: self.path.clone(),
            handler_name: self.handler_name.clone(),
            request_validator: Default::default(),
            response_validator: Default::default(),
            parameter_validator: Default::default(),
            file_params: Default::default(),
            is_async: self.is_async,
            cors: self.cors.clone().map(Into::into),
            expects_json_body: self.expects_json_body,
            handler_dependencies: self.handler_dependencies.clone(),
            jsonrpc_method: self.jsonrpc_method.clone().map(Into::into),
            ..Default::default()
        };
        core_self.is_jsonrpc_method()
    }

    pub fn jsonrpc_method_name(&self) -> Option<String> {
        #[allow(clippy::needless_update)]
        let core_self = spikard_http::Route {
            method: match self.method.as_str() {
                "Get" => spikard_http::Method::Get,
                "Post" => spikard_http::Method::Post,
                "Put" => spikard_http::Method::Put,
                "Patch" => spikard_http::Method::Patch,
                "Delete" => spikard_http::Method::Delete,
                "Head" => spikard_http::Method::Head,
                "Options" => spikard_http::Method::Options,
                "Trace" => spikard_http::Method::Trace,
                _ => spikard_http::Method::Get,
            },
            path: self.path.clone(),
            handler_name: self.handler_name.clone(),
            request_validator: Default::default(),
            response_validator: Default::default(),
            parameter_validator: Default::default(),
            file_params: Default::default(),
            is_async: self.is_async,
            cors: self.cors.clone().map(Into::into),
            expects_json_body: self.expects_json_body,
            handler_dependencies: self.handler_dependencies.clone(),
            jsonrpc_method: self.jsonrpc_method.clone().map(Into::into),
            ..Default::default()
        };
        core_self.jsonrpc_method_name().map(Into::into)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Route {
        spikard_http::Route::default().into()
    }

    pub fn from_metadata(metadata: &RouteMetadata, registry: String) -> PhpResult<Route> {
        Err(ext_php_rs::exception::PhpException::default(
            "Not implemented: from_metadata".to_string(),
        ))
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\GrpcRequestData")]
pub struct GrpcRequestData {
    /// Fully qualified service name (e.g., "mypackage.MyService")
    #[php(prop, name = "service_name")]
    pub service_name: String,
    /// Method name (e.g., "GetUser")
    #[php(prop, name = "method_name")]
    pub method_name: String,
    /// Serialized protobuf message bytes
    pub payload: Vec<u8>,
    /// gRPC metadata (similar to HTTP headers)
    #[php(prop, name = "metadata")]
    pub metadata: String,
}

#[php_impl]
impl GrpcRequestData {
    pub fn __construct(service_name: String, method_name: String, payload: Vec<u8>, metadata: String) -> Self {
        Self {
            service_name,
            method_name,
            payload,
            metadata,
        }
    }

    #[php(getter)]
    pub fn get_payload(&self) -> Vec<u8> {
        self.payload.clone()
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\GrpcResponseData")]
pub struct GrpcResponseData {
    /// Serialized protobuf message bytes
    pub payload: Vec<u8>,
    /// gRPC metadata to include in response (similar to HTTP headers)
    #[php(prop, name = "metadata")]
    pub metadata: String,
}

#[php_impl]
impl GrpcResponseData {
    pub fn __construct(payload: Vec<u8>, metadata: String) -> Self {
        Self { payload, metadata }
    }

    #[php(getter)]
    pub fn get_payload(&self) -> Vec<u8> {
        self.payload.clone()
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\GrpcConfig")]
pub struct GrpcConfig {
    /// Enable gRPC support
    #[php(prop, name = "enabled")]
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
    #[php(prop, name = "max_message_size")]
    pub max_message_size: i64,
    /// Enable gzip compression for gRPC messages
    #[php(prop, name = "enable_compression")]
    pub enable_compression: bool,
    /// Timeout for gRPC requests in seconds (None = no timeout)
    #[php(prop, name = "request_timeout")]
    pub request_timeout: Option<i64>,
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
    #[php(prop, name = "max_concurrent_streams")]
    pub max_concurrent_streams: u32,
    /// Enable HTTP/2 keepalive
    #[php(prop, name = "enable_keepalive")]
    pub enable_keepalive: bool,
    /// HTTP/2 keepalive interval in seconds
    #[php(prop, name = "keepalive_interval")]
    pub keepalive_interval: i64,
    /// HTTP/2 keepalive timeout in seconds
    #[php(prop, name = "keepalive_timeout")]
    pub keepalive_timeout: i64,
}

#[php_impl]
impl GrpcConfig {
    pub fn __construct(
        enabled: Option<bool>,
        max_message_size: Option<i64>,
        enable_compression: Option<bool>,
        request_timeout: Option<i64>,
        max_concurrent_streams: Option<u32>,
        enable_keepalive: Option<bool>,
        keepalive_interval: Option<i64>,
        keepalive_timeout: Option<i64>,
    ) -> Self {
        Self {
            enabled: enabled.unwrap_or(true),
            max_message_size: max_message_size.unwrap_or_default(),
            enable_compression: enable_compression.unwrap_or(true),
            request_timeout,
            max_concurrent_streams: max_concurrent_streams.unwrap_or_default(),
            enable_keepalive: enable_keepalive.unwrap_or(true),
            keepalive_interval: keepalive_interval.unwrap_or_default(),
            keepalive_timeout: keepalive_timeout.unwrap_or_default(),
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> GrpcConfig {
        spikard_http::GrpcConfig::default().into()
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\ValidatedParams")]
pub struct ValidatedParams {
    pub params: HashMap<String, String>,
}

#[php_impl]
impl ValidatedParams {
    pub fn __construct(params: HashMap<String, String>) -> Self {
        Self { params }
    }

    #[php(getter)]
    pub fn get_params(&self) -> HashMap<String, String> {
        self.params.clone()
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\JsonRpcConfig")]
pub struct JsonRpcConfig {
    /// Enable JSON-RPC endpoint
    #[php(prop, name = "enabled")]
    pub enabled: bool,
    /// HTTP endpoint path for JSON-RPC requests (default: "/rpc")
    #[php(prop, name = "endpoint_path")]
    pub endpoint_path: String,
    /// Enable batch request processing (default: true)
    #[php(prop, name = "enable_batch")]
    pub enable_batch: bool,
    /// Maximum number of requests in a batch (default: 100)
    #[php(prop, name = "max_batch_size")]
    pub max_batch_size: i64,
}

#[php_impl]
impl JsonRpcConfig {
    pub fn __construct(
        enabled: Option<bool>,
        endpoint_path: Option<String>,
        enable_batch: Option<bool>,
        max_batch_size: Option<i64>,
    ) -> Self {
        Self {
            enabled: enabled.unwrap_or(true),
            endpoint_path: endpoint_path.unwrap_or_default(),
            enable_batch: enable_batch.unwrap_or_default(),
            max_batch_size: max_batch_size.unwrap_or_default(),
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> JsonRpcConfig {
        spikard_http::JsonRpcConfig::default().into()
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\OpenApiConfig")]
pub struct OpenApiConfig {
    /// Enable OpenAPI generation (default: false for zero overhead)
    #[php(prop, name = "enabled")]
    pub enabled: bool,
    /// API title
    #[php(prop, name = "title")]
    pub title: String,
    /// API version
    #[php(prop, name = "version")]
    pub version: String,
    /// API description (supports markdown)
    #[php(prop, name = "description")]
    pub description: Option<String>,
    /// Path to serve Swagger UI (default: "/docs")
    #[php(prop, name = "swagger_ui_path")]
    pub swagger_ui_path: String,
    /// Path to serve Redoc (default: "/redoc")
    #[php(prop, name = "redoc_path")]
    pub redoc_path: String,
    /// Path to serve OpenAPI JSON spec (default: "/openapi.json")
    #[php(prop, name = "openapi_json_path")]
    pub openapi_json_path: String,
    /// Contact information
    pub contact: Option<ContactInfo>,
    /// License information
    pub license: Option<LicenseInfo>,
    /// Server definitions
    pub servers: Vec<ServerInfo>,
    /// Security schemes (auto-detected from middleware if not provided)
    pub security_schemes: HashMap<String, String>,
}

#[php_impl]
impl OpenApiConfig {
    pub fn __construct() -> PhpResult<Self> {
        Err(PhpException::default(
            "Not implemented: constructor for OpenApiConfig requires complex params".to_string(),
        ))
    }

    #[php(getter)]
    pub fn get_contact(&self) -> Option<ContactInfo> {
        self.contact.clone()
    }

    #[php(getter)]
    pub fn get_license(&self) -> Option<LicenseInfo> {
        self.license.clone()
    }

    #[php(getter)]
    pub fn get_servers(&self) -> Vec<ServerInfo> {
        self.servers.clone()
    }

    #[php(getter)]
    pub fn get_security_schemes(&self) -> HashMap<String, String> {
        self.security_schemes.clone()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> OpenApiConfig {
        spikard_http::OpenApiConfig::default().into()
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\ContactInfo")]
pub struct ContactInfo {
    #[php(prop, name = "name")]
    pub name: Option<String>,
    #[php(prop, name = "email")]
    pub email: Option<String>,
    #[php(prop, name = "url")]
    pub url: Option<String>,
}

#[php_impl]
impl ContactInfo {
    pub fn __construct(name: Option<String>, email: Option<String>, url: Option<String>) -> Self {
        Self { name, email, url }
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\LicenseInfo")]
pub struct LicenseInfo {
    #[php(prop, name = "name")]
    pub name: String,
    #[php(prop, name = "url")]
    pub url: Option<String>,
}

#[php_impl]
impl LicenseInfo {
    pub fn __construct(name: String, url: Option<String>) -> Self {
        Self { name, url }
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\ServerInfo")]
pub struct ServerInfo {
    #[php(prop, name = "url")]
    pub url: String,
    #[php(prop, name = "description")]
    pub description: Option<String>,
}

#[php_impl]
impl ServerInfo {
    pub fn __construct(url: String, description: Option<String>) -> Self {
        Self { url, description }
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\Response")]
pub struct Response {
    /// Response body content
    #[php(prop, name = "content")]
    pub content: Option<String>,
    /// HTTP status code (defaults to 200)
    #[php(prop, name = "status_code")]
    pub status_code: u16,
    /// Response headers
    pub headers: HashMap<String, String>,
}

#[php_impl]
impl Response {
    pub fn __construct(
        content: Option<String>,
        status_code: Option<u16>,
        headers: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            content,
            status_code: status_code.unwrap_or_default(),
            headers: headers.unwrap_or_default(),
        }
    }

    #[php(getter)]
    pub fn get_headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    pub fn set_header(&self, key: String, value: String) -> () {
        ()
    }

    pub fn set_cookie(
        &self,
        key: String,
        value: String,
        max_age: Option<i64>,
        domain: Option<String>,
        path: Option<String>,
        secure: bool,
        http_only: bool,
        same_site: Option<String>,
    ) -> () {
        ()
    }

    pub fn with_status(content: Option<String>, status_code: u16) -> Response {
        panic!("alef: with_status not auto-delegatable")
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Response {
        spikard_http::Response::default().into()
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\SseEvent")]
pub struct SseEvent {
    /// Event type (optional)
    #[php(prop, name = "event_type")]
    pub event_type: Option<String>,
    /// Event data (JSON value)
    #[php(prop, name = "data")]
    pub data: String,
    /// Event ID (optional, for client-side reconnection)
    #[php(prop, name = "id")]
    pub id: Option<String>,
    /// Retry timeout in milliseconds (optional)
    #[php(prop, name = "retry")]
    pub retry: Option<i64>,
}

#[php_impl]
impl SseEvent {
    pub fn __construct(data: String, event_type: Option<String>, id: Option<String>, retry: Option<i64>) -> Self {
        Self {
            event_type,
            data,
            id,
            retry,
        }
    }

    pub fn with_id(&self, id: String) -> SseEvent {
        let core_self = spikard_http::SseEvent {
            event_type: self.event_type.clone(),
            data: Default::default(),
            id: self.id.clone(),
            retry: self.retry.map(|v| v as u64),
        };
        core_self.with_id(id).into()
    }

    pub fn with_retry(&self, retry_ms: i64) -> SseEvent {
        let core_self = spikard_http::SseEvent {
            event_type: self.event_type.clone(),
            data: Default::default(),
            id: self.id.clone(),
            retry: self.retry.map(|v| v as u64),
        };
        core_self.with_retry(retry_ms as u64).into()
    }

    pub fn with_type(event_type: String, data: String) -> SseEvent {
        panic!("alef: with_type not auto-delegatable")
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\JwtConfig")]
pub struct JwtConfig {
    /// Secret key for JWT verification
    #[php(prop, name = "secret")]
    pub secret: String,
    /// Required algorithm (HS256, HS384, HS512, RS256, etc.)
    #[php(prop, name = "algorithm")]
    pub algorithm: String,
    /// Required audience claim
    #[php(prop, name = "audience")]
    pub audience: Option<Vec<String>>,
    /// Required issuer claim
    #[php(prop, name = "issuer")]
    pub issuer: Option<String>,
    /// Leeway for expiration checks (seconds)
    #[php(prop, name = "leeway")]
    pub leeway: i64,
}

#[php_impl]
impl JwtConfig {
    pub fn __construct(
        secret: String,
        algorithm: String,
        leeway: i64,
        audience: Option<Vec<String>>,
        issuer: Option<String>,
    ) -> Self {
        Self {
            secret,
            algorithm,
            audience,
            issuer,
            leeway,
        }
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\ApiKeyConfig")]
pub struct ApiKeyConfig {
    /// Valid API keys
    #[php(prop, name = "keys")]
    pub keys: Vec<String>,
    /// Header name to check (e.g., "X-API-Key")
    #[php(prop, name = "header_name")]
    pub header_name: String,
}

#[php_impl]
impl ApiKeyConfig {
    pub fn __construct(keys: Vec<String>, header_name: String) -> Self {
        Self { keys, header_name }
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\StaticFilesConfig")]
pub struct StaticFilesConfig {
    /// Directory path to serve
    #[php(prop, name = "directory")]
    pub directory: String,
    /// URL path prefix (e.g., "/static")
    #[php(prop, name = "route_prefix")]
    pub route_prefix: String,
    /// Fallback to index.html for directories
    #[php(prop, name = "index_file")]
    pub index_file: bool,
    /// Cache-Control header value
    #[php(prop, name = "cache_control")]
    pub cache_control: Option<String>,
}

#[php_impl]
impl StaticFilesConfig {
    pub fn __construct(
        directory: String,
        route_prefix: String,
        index_file: bool,
        cache_control: Option<String>,
    ) -> Self {
        Self {
            directory,
            route_prefix,
            index_file,
            cache_control,
        }
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\ServerConfig")]
#[allow(clippy::similar_names)]
pub struct ServerConfig {
    /// Host to bind to
    #[php(prop, name = "host")]
    pub host: String,
    /// Port to bind to
    #[php(prop, name = "port")]
    pub port: u16,
    /// Number of Tokio runtime worker threads used by binding-managed server runtimes
    #[php(prop, name = "workers")]
    pub workers: i64,
    /// Enable request ID generation and propagation
    #[php(prop, name = "enable_request_id")]
    pub enable_request_id: bool,
    /// Maximum request body size in bytes (None = unlimited, not recommended)
    #[php(prop, name = "max_body_size")]
    pub max_body_size: Option<i64>,
    /// Request timeout in seconds (None = no timeout)
    #[php(prop, name = "request_timeout")]
    pub request_timeout: Option<i64>,
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
    #[php(prop, name = "graceful_shutdown")]
    pub graceful_shutdown: bool,
    /// Graceful shutdown timeout (seconds)
    #[php(prop, name = "shutdown_timeout")]
    pub shutdown_timeout: i64,
    /// OpenAPI documentation configuration
    pub openapi: Option<OpenApiConfig>,
    /// JSON-RPC configuration
    pub jsonrpc: Option<JsonRpcConfig>,
    /// gRPC configuration
    pub grpc: Option<GrpcConfig>,
    /// Lifecycle hooks for request/response processing
    #[php(prop, name = "lifecycle_hooks")]
    pub lifecycle_hooks: Option<String>,
    /// Background task executor configuration
    pub background_tasks: BackgroundTaskConfig,
    /// Enable per-request HTTP tracing (tower-http `TraceLayer`)
    #[php(prop, name = "enable_http_trace")]
    pub enable_http_trace: bool,
    /// Dependency injection container (requires 'di' feature)
    #[php(prop, name = "di_container")]
    pub di_container: Option<String>,
}

#[php_impl]
impl ServerConfig {
    pub fn __construct() -> PhpResult<Self> {
        Err(PhpException::default(
            "Not implemented: constructor for ServerConfig requires complex params".to_string(),
        ))
    }

    #[php(getter)]
    pub fn get_compression(&self) -> Option<CompressionConfig> {
        self.compression.clone()
    }

    #[php(getter)]
    pub fn get_rate_limit(&self) -> Option<RateLimitConfig> {
        self.rate_limit.clone()
    }

    #[php(getter)]
    pub fn get_jwt_auth(&self) -> Option<JwtConfig> {
        self.jwt_auth.clone()
    }

    #[php(getter)]
    pub fn get_api_key_auth(&self) -> Option<ApiKeyConfig> {
        self.api_key_auth.clone()
    }

    #[php(getter)]
    pub fn get_static_files(&self) -> Vec<StaticFilesConfig> {
        self.static_files.clone()
    }

    #[php(getter)]
    pub fn get_openapi(&self) -> Option<OpenApiConfig> {
        self.openapi.clone()
    }

    #[php(getter)]
    pub fn get_jsonrpc(&self) -> Option<JsonRpcConfig> {
        self.jsonrpc.clone()
    }

    #[php(getter)]
    pub fn get_grpc(&self) -> Option<GrpcConfig> {
        self.grpc.clone()
    }

    #[php(getter)]
    pub fn get_background_tasks(&self) -> BackgroundTaskConfig {
        self.background_tasks.clone()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> ServerConfig {
        spikard_http::ServerConfig::default().into()
    }

    pub fn builder() -> String {
        String::from("[unimplemented: builder]")
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\UploadFile")]
pub struct UploadFile {
    /// Original filename from the client
    #[php(prop, name = "filename")]
    pub filename: String,
    /// MIME type of the uploaded file
    #[php(prop, name = "content_type")]
    pub content_type: Option<String>,
    /// Size of the file in bytes
    #[php(prop, name = "size")]
    pub size: Option<i64>,
    /// File content (may be base64 encoded)
    pub content: Vec<u8>,
    /// Content encoding type
    #[php(prop, name = "content_encoding")]
    pub content_encoding: Option<String>,
    /// Internal cursor for Read/Seek operations
    #[php(prop, name = "cursor")]
    pub cursor: String,
}

#[php_impl]
impl UploadFile {
    pub fn __construct(
        filename: String,
        content: Vec<u8>,
        cursor: String,
        content_type: Option<String>,
        size: Option<i64>,
        content_encoding: Option<String>,
    ) -> Self {
        Self {
            filename,
            content_type,
            size,
            content,
            content_encoding,
            cursor,
        }
    }

    #[php(getter)]
    pub fn get_content(&self) -> Vec<u8> {
        self.content.clone()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let core_self = spikard::UploadFile {
            filename: self.filename.clone(),
            content_type: self.content_type.clone(),
            size: self.size.map(|v| v as usize),
            content: self.content.clone(),
            content_encoding: self.content_encoding.clone(),
            cursor: Default::default(),
        };
        core_self.as_bytes().into()
    }

    pub fn read_to_string(&self) -> PhpResult<String> {
        let core_self = spikard::UploadFile {
            filename: self.filename.clone(),
            content_type: self.content_type.clone(),
            size: self.size.map(|v| v as usize),
            content: self.content.clone(),
            content_encoding: self.content_encoding.clone(),
            cursor: Default::default(),
        };
        let result = core_self
            .read_to_string()
            .map_err(|e| ext_php_rs::exception::PhpException::default(e.to_string()))?;
        Ok(result)
    }

    pub fn content_type_or_default(&self) -> String {
        let core_self = spikard::UploadFile {
            filename: self.filename.clone(),
            content_type: self.content_type.clone(),
            size: self.size.map(|v| v as usize),
            content: self.content.clone(),
            content_encoding: self.content_encoding.clone(),
            cursor: Default::default(),
        };
        core_self.content_type_or_default().into()
    }
}

#[derive(Clone)]
#[php_class]
#[php(name = "Spikard\\Php\\GraphQLError")]
pub struct GraphQLError {
    inner: Arc<spikard_graphql::GraphQLError>,
}

#[php_impl]
impl GraphQLError {
    pub fn status_code(&self) -> u16 {
        self.inner.status_code()
    }

    pub fn to_graphql_response(&self) -> String {
        self.inner.to_graphql_response()
    }

    pub fn to_http_response(&self) -> String {
        self.inner.to_http_response()
    }
}

#[derive(Clone)]
#[php_class]
#[php(name = "Spikard\\Php\\GraphQLRouteConfig")]
pub struct GraphQLRouteConfig {
    inner: Arc<spikard_graphql::GraphQLRouteConfig>,
}

#[php_impl]
impl GraphQLRouteConfig {
    pub fn path(&self, path: String) -> GraphQLRouteConfig {
        Self {
            inner: Arc::new((*self.inner).clone().path(path)),
        }
    }

    pub fn method(&self, method: String) -> GraphQLRouteConfig {
        Self {
            inner: Arc::new((*self.inner).clone().method(method)),
        }
    }

    pub fn enable_playground(&self, enable: bool) -> GraphQLRouteConfig {
        Self {
            inner: Arc::new((*self.inner).clone().enable_playground(enable)),
        }
    }

    pub fn description(&self, description: String) -> GraphQLRouteConfig {
        Self {
            inner: Arc::new((*self.inner).clone().description(description)),
        }
    }

    pub fn get_path(&self) -> String {
        self.inner.get_path().into()
    }

    pub fn get_method(&self) -> String {
        self.inner.get_method().into()
    }

    pub fn is_playground_enabled(&self) -> bool {
        self.inner.is_playground_enabled()
    }

    pub fn get_description(&self) -> Option<String> {
        self.inner.get_description().map(Into::into)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> GraphQLRouteConfig {
        Self {
            inner: Arc::new(spikard_graphql::GraphQLRouteConfig::default()),
        }
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\SchemaConfig")]
pub struct SchemaConfig {
    /// Enable introspection queries
    #[php(prop, name = "introspection_enabled")]
    pub introspection_enabled: bool,
    /// Maximum query complexity (None = unlimited)
    #[php(prop, name = "complexity_limit")]
    pub complexity_limit: Option<i64>,
    /// Maximum query depth (None = unlimited)
    #[php(prop, name = "depth_limit")]
    pub depth_limit: Option<i64>,
}

#[php_impl]
impl SchemaConfig {
    pub fn __construct(
        introspection_enabled: Option<bool>,
        complexity_limit: Option<i64>,
        depth_limit: Option<i64>,
    ) -> Self {
        Self {
            introspection_enabled: introspection_enabled.unwrap_or(true),
            complexity_limit,
            depth_limit,
        }
    }

    pub fn set_introspection_enabled(&self, enabled: bool) -> SchemaConfig {
        panic!("alef: set_introspection_enabled not auto-delegatable")
    }

    pub fn set_complexity_limit(&self, limit: i64) -> SchemaConfig {
        panic!("alef: set_complexity_limit not auto-delegatable")
    }

    pub fn set_depth_limit(&self, limit: i64) -> SchemaConfig {
        panic!("alef: set_depth_limit not auto-delegatable")
    }

    pub fn validate(&self) -> String {
        String::from("[unimplemented: validate]")
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> SchemaConfig {
        spikard_graphql::SchemaConfig::default().into()
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\QueryOnlyConfig")]
pub struct QueryOnlyConfig {
    /// Enable introspection queries
    #[php(prop, name = "introspection_enabled")]
    pub introspection_enabled: bool,
    /// Maximum query complexity (None = unlimited)
    #[php(prop, name = "complexity_limit")]
    pub complexity_limit: Option<i64>,
    /// Maximum query depth (None = unlimited)
    #[php(prop, name = "depth_limit")]
    pub depth_limit: Option<i64>,
}

#[php_impl]
impl QueryOnlyConfig {
    pub fn __construct(
        introspection_enabled: Option<bool>,
        complexity_limit: Option<i64>,
        depth_limit: Option<i64>,
    ) -> Self {
        Self {
            introspection_enabled: introspection_enabled.unwrap_or(true),
            complexity_limit,
            depth_limit,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> QueryOnlyConfig {
        spikard_graphql::QueryOnlyConfig::default().into()
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\QueryMutationConfig")]
pub struct QueryMutationConfig {
    /// Enable introspection queries
    #[php(prop, name = "introspection_enabled")]
    pub introspection_enabled: bool,
    /// Maximum query complexity (None = unlimited)
    #[php(prop, name = "complexity_limit")]
    pub complexity_limit: Option<i64>,
    /// Maximum query depth (None = unlimited)
    #[php(prop, name = "depth_limit")]
    pub depth_limit: Option<i64>,
}

#[php_impl]
impl QueryMutationConfig {
    pub fn __construct(
        introspection_enabled: Option<bool>,
        complexity_limit: Option<i64>,
        depth_limit: Option<i64>,
    ) -> Self {
        Self {
            introspection_enabled: introspection_enabled.unwrap_or(true),
            complexity_limit,
            depth_limit,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> QueryMutationConfig {
        spikard_graphql::QueryMutationConfig::default().into()
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[php_class]
#[php(name = "Spikard\\Php\\FullSchemaConfig")]
pub struct FullSchemaConfig {
    /// Enable introspection queries
    #[php(prop, name = "introspection_enabled")]
    pub introspection_enabled: bool,
    /// Maximum query complexity (None = unlimited)
    #[php(prop, name = "complexity_limit")]
    pub complexity_limit: Option<i64>,
    /// Maximum query depth (None = unlimited)
    #[php(prop, name = "depth_limit")]
    pub depth_limit: Option<i64>,
}

#[php_impl]
impl FullSchemaConfig {
    pub fn __construct(
        introspection_enabled: Option<bool>,
        complexity_limit: Option<i64>,
        depth_limit: Option<i64>,
    ) -> Self {
        Self {
            introspection_enabled: introspection_enabled.unwrap_or(true),
            complexity_limit,
            depth_limit,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> FullSchemaConfig {
        spikard_graphql::FullSchemaConfig::default().into()
    }
}

// Method enum values
pub const METHOD_GET: &str = "Get";
pub const METHOD_POST: &str = "Post";
pub const METHOD_PUT: &str = "Put";
pub const METHOD_PATCH: &str = "Patch";
pub const METHOD_DELETE: &str = "Delete";
pub const METHOD_HEAD: &str = "Head";
pub const METHOD_OPTIONS: &str = "Options";
pub const METHOD_TRACE: &str = "Trace";

// JsonRpcResponseType enum values
pub const JSONRPCRESPONSETYPE_SUCCESS: &str = "Success";
pub const JSONRPCRESPONSETYPE_ERROR: &str = "Error";

// JsonRpcRequestOrBatch enum values
pub const JSONRPCREQUESTORBATCH_SINGLE: &str = "Single";
pub const JSONRPCREQUESTORBATCH_BATCH: &str = "Batch";

// SecuritySchemeInfo enum values
pub const SECURITYSCHEMEINFO_HTTP: &str = "Http";
pub const SECURITYSCHEMEINFO_APIKEY: &str = "ApiKey";

#[php_class]
#[php(name = "Spikard\\Php\\SpikardPhpApi")]
pub struct SpikardPhpApi;

#[php_impl]
impl SpikardPhpApi {
    pub fn add_cors_headers(response: &Response, origin: String, cors_config: &CorsConfig) -> () {
        let response_core: spikard::Response = response.clone().into();
        let cors_config_core: spikard::CorsConfig = cors_config.clone().into();
        spikard_http::cors::add_cors_headers(&response_core, &origin, &cors_config_core)
    }

    pub fn schema_query_only() -> QueryOnlyConfig {
        spikard_graphql::schema_query_only().into()
    }

    pub fn schema_query_mutation() -> QueryMutationConfig {
        spikard_graphql::schema_query_mutation().into()
    }

    pub fn schema_full() -> FullSchemaConfig {
        spikard_graphql::schema_full().into()
    }
}

impl From<spikard_http::Claims> for Claims {
    fn from(val: spikard_http::Claims) -> Self {
        Self {
            sub: val.sub,
            exp: val.exp as i64,
            iat: val.iat.map(|v| v as i64),
            nbf: val.nbf.map(|v| v as i64),
            aud: val.aud,
            iss: val.iss,
        }
    }
}

impl From<BackgroundTaskConfig> for spikard_http::BackgroundTaskConfig {
    fn from(val: BackgroundTaskConfig) -> Self {
        Self {
            max_queue_size: val.max_queue_size as usize,
            max_concurrent_tasks: val.max_concurrent_tasks as usize,
            drain_timeout_secs: val.drain_timeout_secs as u64,
        }
    }
}

impl From<spikard_http::BackgroundTaskConfig> for BackgroundTaskConfig {
    fn from(val: spikard_http::BackgroundTaskConfig) -> Self {
        Self {
            max_queue_size: val.max_queue_size as i64,
            max_concurrent_tasks: val.max_concurrent_tasks as i64,
            drain_timeout_secs: val.drain_timeout_secs as i64,
        }
    }
}

impl From<BackgroundJobMetadata> for spikard_http::BackgroundJobMetadata {
    fn from(val: BackgroundJobMetadata) -> Self {
        Self {
            name: Default::default(),
            request_id: val.request_id,
        }
    }
}

impl From<spikard_http::BackgroundJobMetadata> for BackgroundJobMetadata {
    fn from(val: spikard_http::BackgroundJobMetadata) -> Self {
        Self {
            name: format!("{:?}", val.name),
            request_id: val.request_id,
        }
    }
}

impl From<BackgroundJobError> for spikard_http::BackgroundJobError {
    fn from(val: BackgroundJobError) -> Self {
        Self { message: val.message }
    }
}

impl From<spikard_http::BackgroundJobError> for BackgroundJobError {
    fn from(val: spikard_http::BackgroundJobError) -> Self {
        Self { message: val.message }
    }
}

impl From<CorsConfig> for spikard_http::CorsConfig {
    fn from(val: CorsConfig) -> Self {
        Self {
            allowed_origins: val.allowed_origins,
            allowed_methods: val.allowed_methods,
            allowed_headers: val.allowed_headers,
            expose_headers: val.expose_headers,
            max_age: val.max_age,
            allow_credentials: val.allow_credentials,
            methods_joined_cache: Default::default(),
            headers_joined_cache: Default::default(),
        }
    }
}

impl From<spikard_http::CorsConfig> for CorsConfig {
    fn from(val: spikard_http::CorsConfig) -> Self {
        Self {
            allowed_origins: val.allowed_origins,
            allowed_methods: val.allowed_methods,
            allowed_headers: val.allowed_headers,
            expose_headers: val.expose_headers,
            max_age: val.max_age,
            allow_credentials: val.allow_credentials,
            methods_joined_cache: format!("{:?}", val.methods_joined_cache),
            headers_joined_cache: format!("{:?}", val.headers_joined_cache),
        }
    }
}

#[allow(clippy::needless_update)]
impl From<RouteMetadata> for spikard_http::RouteMetadata {
    fn from(val: RouteMetadata) -> Self {
        Self {
            method: val.method,
            path: val.path,
            handler_name: val.handler_name,
            request_schema: Default::default(),
            response_schema: Default::default(),
            parameter_schema: Default::default(),
            file_params: Default::default(),
            is_async: val.is_async,
            cors: val.cors.map(Into::into),
            body_param_name: val.body_param_name,
            handler_dependencies: val.handler_dependencies,
            jsonrpc_method: Default::default(),
            static_response: Default::default(),
            ..Default::default()
        }
    }
}

impl From<spikard_http::RouteMetadata> for RouteMetadata {
    fn from(val: spikard_http::RouteMetadata) -> Self {
        Self {
            method: val.method,
            path: val.path,
            handler_name: val.handler_name,
            request_schema: val.request_schema.as_ref().map(|v| format!("{v:?}")),
            response_schema: val.response_schema.as_ref().map(|v| format!("{v:?}")),
            parameter_schema: val.parameter_schema.as_ref().map(|v| format!("{v:?}")),
            file_params: val.file_params.as_ref().map(|v| format!("{v:?}")),
            is_async: val.is_async,
            cors: val.cors.map(Into::into),
            body_param_name: val.body_param_name,
            handler_dependencies: val.handler_dependencies,
            jsonrpc_method: val.jsonrpc_method.as_ref().map(|v| format!("{v:?}")),
            static_response: val.static_response.as_ref().map(|v| format!("{v:?}")),
        }
    }
}

impl From<CompressionConfig> for spikard_http::CompressionConfig {
    fn from(val: CompressionConfig) -> Self {
        Self {
            gzip: val.gzip,
            brotli: val.brotli,
            min_size: val.min_size as usize,
            quality: val.quality,
        }
    }
}

impl From<spikard_http::CompressionConfig> for CompressionConfig {
    fn from(val: spikard_http::CompressionConfig) -> Self {
        Self {
            gzip: val.gzip,
            brotli: val.brotli,
            min_size: val.min_size as i64,
            quality: val.quality,
        }
    }
}

impl From<RateLimitConfig> for spikard_http::RateLimitConfig {
    fn from(val: RateLimitConfig) -> Self {
        Self {
            per_second: val.per_second as u64,
            burst: val.burst,
            ip_based: val.ip_based,
        }
    }
}

impl From<spikard_http::RateLimitConfig> for RateLimitConfig {
    fn from(val: spikard_http::RateLimitConfig) -> Self {
        Self {
            per_second: val.per_second as i64,
            burst: val.burst,
            ip_based: val.ip_based,
        }
    }
}

impl From<ProblemDetails> for spikard_http::ProblemDetails {
    fn from(val: ProblemDetails) -> Self {
        Self {
            type_uri: val.type_uri,
            title: val.title,
            status: val.status,
            detail: val.detail,
            instance: val.instance,
            extensions: Default::default(),
        }
    }
}

impl From<spikard_http::ProblemDetails> for ProblemDetails {
    fn from(val: spikard_http::ProblemDetails) -> Self {
        Self {
            type_uri: val.type_uri,
            title: val.title,
            status: val.status,
            detail: val.detail,
            instance: val.instance,
            extensions: val
                .extensions
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }
}

impl From<JsonRpcMethodInfo> for spikard_http::JsonRpcMethodInfo {
    fn from(val: JsonRpcMethodInfo) -> Self {
        Self {
            method_name: val.method_name,
            description: val.description,
            params_schema: Default::default(),
            result_schema: Default::default(),
            deprecated: val.deprecated,
            tags: val.tags,
        }
    }
}

impl From<spikard_http::JsonRpcMethodInfo> for JsonRpcMethodInfo {
    fn from(val: spikard_http::JsonRpcMethodInfo) -> Self {
        Self {
            method_name: val.method_name,
            description: val.description,
            params_schema: val.params_schema.as_ref().map(|v| format!("{v:?}")),
            result_schema: val.result_schema.as_ref().map(|v| format!("{v:?}")),
            deprecated: val.deprecated,
            tags: val.tags,
        }
    }
}

impl From<Route> for spikard_http::Route {
    fn from(val: Route) -> Self {
        Self {
            method: match val.method.as_str() {
                "Get" => spikard_http::Method::Get,
                "Post" => spikard_http::Method::Post,
                "Put" => spikard_http::Method::Put,
                "Patch" => spikard_http::Method::Patch,
                "Delete" => spikard_http::Method::Delete,
                "Head" => spikard_http::Method::Head,
                "Options" => spikard_http::Method::Options,
                "Trace" => spikard_http::Method::Trace,
                _ => spikard_http::Method::Get,
            },
            path: val.path,
            handler_name: val.handler_name,
            request_validator: Default::default(),
            response_validator: Default::default(),
            parameter_validator: Default::default(),
            file_params: Default::default(),
            is_async: val.is_async,
            cors: val.cors.map(Into::into),
            expects_json_body: val.expects_json_body,
            handler_dependencies: val.handler_dependencies,
            jsonrpc_method: val.jsonrpc_method.map(Into::into),
        }
    }
}

impl From<spikard_http::Route> for Route {
    fn from(val: spikard_http::Route) -> Self {
        Self {
            method: serde_json::to_value(val.method)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            path: val.path,
            handler_name: val.handler_name,
            request_validator: val.request_validator.as_ref().map(|v| format!("{v:?}")),
            response_validator: val.response_validator.as_ref().map(|v| format!("{v:?}")),
            parameter_validator: val.parameter_validator.as_ref().map(|v| format!("{v:?}")),
            file_params: val.file_params.as_ref().map(|v| format!("{v:?}")),
            is_async: val.is_async,
            cors: val.cors.map(Into::into),
            expects_json_body: val.expects_json_body,
            handler_dependencies: val.handler_dependencies,
            jsonrpc_method: val.jsonrpc_method.map(Into::into),
        }
    }
}

impl From<spikard_http::GrpcRequestData> for GrpcRequestData {
    fn from(val: spikard_http::GrpcRequestData) -> Self {
        Self {
            service_name: val.service_name,
            method_name: val.method_name,
            payload: val.payload.to_vec(),
            metadata: format!("{:?}", val.metadata),
        }
    }
}

impl From<spikard_http::GrpcResponseData> for GrpcResponseData {
    fn from(val: spikard_http::GrpcResponseData) -> Self {
        Self {
            payload: val.payload.to_vec(),
            metadata: format!("{:?}", val.metadata),
        }
    }
}

impl From<GrpcConfig> for spikard_http::GrpcConfig {
    fn from(val: GrpcConfig) -> Self {
        Self {
            enabled: val.enabled,
            max_message_size: val.max_message_size as usize,
            enable_compression: val.enable_compression,
            request_timeout: val.request_timeout.map(|v| v as u64),
            max_concurrent_streams: val.max_concurrent_streams,
            enable_keepalive: val.enable_keepalive,
            keepalive_interval: val.keepalive_interval as u64,
            keepalive_timeout: val.keepalive_timeout as u64,
        }
    }
}

impl From<spikard_http::GrpcConfig> for GrpcConfig {
    fn from(val: spikard_http::GrpcConfig) -> Self {
        Self {
            enabled: val.enabled,
            max_message_size: val.max_message_size as i64,
            enable_compression: val.enable_compression,
            request_timeout: val.request_timeout.map(|v| v as i64),
            max_concurrent_streams: val.max_concurrent_streams,
            enable_keepalive: val.enable_keepalive,
            keepalive_interval: val.keepalive_interval as i64,
            keepalive_timeout: val.keepalive_timeout as i64,
        }
    }
}

impl From<spikard_http::ValidatedParams> for ValidatedParams {
    fn from(val: spikard_http::ValidatedParams) -> Self {
        Self {
            params: val
                .params
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }
}

impl From<JsonRpcConfig> for spikard_http::JsonRpcConfig {
    fn from(val: JsonRpcConfig) -> Self {
        Self {
            enabled: val.enabled,
            endpoint_path: val.endpoint_path,
            enable_batch: val.enable_batch,
            max_batch_size: val.max_batch_size as usize,
        }
    }
}

impl From<spikard_http::JsonRpcConfig> for JsonRpcConfig {
    fn from(val: spikard_http::JsonRpcConfig) -> Self {
        Self {
            enabled: val.enabled,
            endpoint_path: val.endpoint_path,
            enable_batch: val.enable_batch,
            max_batch_size: val.max_batch_size as i64,
        }
    }
}

impl From<OpenApiConfig> for spikard_http::OpenApiConfig {
    fn from(val: OpenApiConfig) -> Self {
        Self {
            enabled: val.enabled,
            title: val.title,
            version: val.version,
            description: val.description,
            swagger_ui_path: val.swagger_ui_path,
            redoc_path: val.redoc_path,
            openapi_json_path: val.openapi_json_path,
            contact: val.contact.map(Into::into),
            license: val.license.map(Into::into),
            servers: val.servers.into_iter().map(Into::into).collect(),
            security_schemes: val.security_schemes.into_iter().map(|(k, v)| (k, v.into())).collect(),
        }
    }
}

impl From<spikard_http::OpenApiConfig> for OpenApiConfig {
    fn from(val: spikard_http::OpenApiConfig) -> Self {
        Self {
            enabled: val.enabled,
            title: val.title,
            version: val.version,
            description: val.description,
            swagger_ui_path: val.swagger_ui_path,
            redoc_path: val.redoc_path,
            openapi_json_path: val.openapi_json_path,
            contact: val.contact.map(Into::into),
            license: val.license.map(Into::into),
            servers: val.servers.into_iter().map(Into::into).collect(),
            security_schemes: val.security_schemes.into_iter().map(|(k, v)| (k, v.into())).collect(),
        }
    }
}

impl From<ContactInfo> for spikard_http::ContactInfo {
    fn from(val: ContactInfo) -> Self {
        Self {
            name: val.name,
            email: val.email,
            url: val.url,
        }
    }
}

impl From<spikard_http::ContactInfo> for ContactInfo {
    fn from(val: spikard_http::ContactInfo) -> Self {
        Self {
            name: val.name,
            email: val.email,
            url: val.url,
        }
    }
}

impl From<LicenseInfo> for spikard_http::LicenseInfo {
    fn from(val: LicenseInfo) -> Self {
        Self {
            name: val.name,
            url: val.url,
        }
    }
}

impl From<spikard_http::LicenseInfo> for LicenseInfo {
    fn from(val: spikard_http::LicenseInfo) -> Self {
        Self {
            name: val.name,
            url: val.url,
        }
    }
}

impl From<ServerInfo> for spikard_http::ServerInfo {
    fn from(val: ServerInfo) -> Self {
        Self {
            url: val.url,
            description: val.description,
        }
    }
}

impl From<spikard_http::ServerInfo> for ServerInfo {
    fn from(val: spikard_http::ServerInfo) -> Self {
        Self {
            url: val.url,
            description: val.description,
        }
    }
}

impl From<Response> for spikard_http::Response {
    fn from(val: Response) -> Self {
        Self {
            content: Default::default(),
            status_code: val.status_code,
            headers: val.headers.into_iter().collect(),
        }
    }
}

impl From<spikard_http::Response> for Response {
    fn from(val: spikard_http::Response) -> Self {
        Self {
            content: val.content.as_ref().map(|v| format!("{v:?}")),
            status_code: val.status_code,
            headers: val.headers.into_iter().collect(),
        }
    }
}

impl From<SseEvent> for spikard_http::SseEvent {
    fn from(val: SseEvent) -> Self {
        Self {
            event_type: val.event_type,
            data: Default::default(),
            id: val.id,
            retry: val.retry.map(|v| v as u64),
        }
    }
}

impl From<spikard_http::SseEvent> for SseEvent {
    fn from(val: spikard_http::SseEvent) -> Self {
        Self {
            event_type: val.event_type,
            data: format!("{:?}", val.data),
            id: val.id,
            retry: val.retry.map(|v| v as i64),
        }
    }
}

impl From<JwtConfig> for spikard_http::JwtConfig {
    fn from(val: JwtConfig) -> Self {
        Self {
            secret: val.secret,
            algorithm: val.algorithm,
            audience: val.audience,
            issuer: val.issuer,
            leeway: val.leeway as u64,
        }
    }
}

impl From<spikard_http::JwtConfig> for JwtConfig {
    fn from(val: spikard_http::JwtConfig) -> Self {
        Self {
            secret: val.secret,
            algorithm: val.algorithm,
            audience: val.audience,
            issuer: val.issuer,
            leeway: val.leeway as i64,
        }
    }
}

impl From<ApiKeyConfig> for spikard_http::ApiKeyConfig {
    fn from(val: ApiKeyConfig) -> Self {
        Self {
            keys: val.keys,
            header_name: val.header_name,
        }
    }
}

impl From<spikard_http::ApiKeyConfig> for ApiKeyConfig {
    fn from(val: spikard_http::ApiKeyConfig) -> Self {
        Self {
            keys: val.keys,
            header_name: val.header_name,
        }
    }
}

impl From<StaticFilesConfig> for spikard_http::StaticFilesConfig {
    fn from(val: StaticFilesConfig) -> Self {
        Self {
            directory: val.directory,
            route_prefix: val.route_prefix,
            index_file: val.index_file,
            cache_control: val.cache_control,
        }
    }
}

impl From<spikard_http::StaticFilesConfig> for StaticFilesConfig {
    fn from(val: spikard_http::StaticFilesConfig) -> Self {
        Self {
            directory: val.directory,
            route_prefix: val.route_prefix,
            index_file: val.index_file,
            cache_control: val.cache_control,
        }
    }
}

impl From<ServerConfig> for spikard_http::ServerConfig {
    fn from(val: ServerConfig) -> Self {
        Self {
            host: val.host,
            port: val.port,
            workers: val.workers as usize,
            enable_request_id: val.enable_request_id,
            max_body_size: val.max_body_size.map(|v| v as usize),
            request_timeout: val.request_timeout.map(|v| v as u64),
            compression: val.compression.map(Into::into),
            rate_limit: val.rate_limit.map(Into::into),
            jwt_auth: val.jwt_auth.map(Into::into),
            api_key_auth: val.api_key_auth.map(Into::into),
            static_files: val.static_files.into_iter().map(Into::into).collect(),
            graceful_shutdown: val.graceful_shutdown,
            shutdown_timeout: val.shutdown_timeout as u64,
            openapi: val.openapi.map(Into::into),
            jsonrpc: val.jsonrpc.map(Into::into),
            grpc: val.grpc.map(Into::into),
            lifecycle_hooks: Default::default(),
            background_tasks: val.background_tasks.into(),
            enable_http_trace: val.enable_http_trace,
            di_container: Default::default(),
        }
    }
}

impl From<spikard_http::ServerConfig> for ServerConfig {
    fn from(val: spikard_http::ServerConfig) -> Self {
        Self {
            host: val.host,
            port: val.port,
            workers: val.workers as i64,
            enable_request_id: val.enable_request_id,
            max_body_size: val.max_body_size.map(|v| v as i64),
            request_timeout: val.request_timeout.map(|v| v as i64),
            compression: val.compression.map(Into::into),
            rate_limit: val.rate_limit.map(Into::into),
            jwt_auth: val.jwt_auth.map(Into::into),
            api_key_auth: val.api_key_auth.map(Into::into),
            static_files: val.static_files.into_iter().map(Into::into).collect(),
            graceful_shutdown: val.graceful_shutdown,
            shutdown_timeout: val.shutdown_timeout as i64,
            openapi: val.openapi.map(Into::into),
            jsonrpc: val.jsonrpc.map(Into::into),
            grpc: val.grpc.map(Into::into),
            lifecycle_hooks: val.lifecycle_hooks.as_ref().map(|v| format!("{v:?}")),
            background_tasks: val.background_tasks.into(),
            enable_http_trace: val.enable_http_trace,
            di_container: val.di_container.as_ref().map(|v| format!("{v:?}")),
        }
    }
}

impl From<spikard::UploadFile> for UploadFile {
    fn from(val: spikard::UploadFile) -> Self {
        Self {
            filename: val.filename,
            content_type: val.content_type,
            size: val.size.map(|v| v as i64),
            content: val.content.to_vec(),
            content_encoding: val.content_encoding,
            cursor: format!("{:?}", val.cursor),
        }
    }
}

impl From<SchemaConfig> for spikard_graphql::SchemaConfig {
    fn from(val: SchemaConfig) -> Self {
        Self {
            introspection_enabled: val.introspection_enabled,
            complexity_limit: val.complexity_limit.map(|v| v as usize),
            depth_limit: val.depth_limit.map(|v| v as usize),
        }
    }
}

impl From<spikard_graphql::SchemaConfig> for SchemaConfig {
    fn from(val: spikard_graphql::SchemaConfig) -> Self {
        Self {
            introspection_enabled: val.introspection_enabled,
            complexity_limit: val.complexity_limit.map(|v| v as i64),
            depth_limit: val.depth_limit.map(|v| v as i64),
        }
    }
}

impl From<QueryOnlyConfig> for spikard_graphql::QueryOnlyConfig {
    fn from(val: QueryOnlyConfig) -> Self {
        Self {
            introspection_enabled: val.introspection_enabled,
            complexity_limit: val.complexity_limit.map(|v| v as usize),
            depth_limit: val.depth_limit.map(|v| v as usize),
        }
    }
}

impl From<spikard_graphql::QueryOnlyConfig> for QueryOnlyConfig {
    fn from(val: spikard_graphql::QueryOnlyConfig) -> Self {
        Self {
            introspection_enabled: val.introspection_enabled,
            complexity_limit: val.complexity_limit.map(|v| v as i64),
            depth_limit: val.depth_limit.map(|v| v as i64),
        }
    }
}

impl From<QueryMutationConfig> for spikard_graphql::QueryMutationConfig {
    fn from(val: QueryMutationConfig) -> Self {
        Self {
            introspection_enabled: val.introspection_enabled,
            complexity_limit: val.complexity_limit.map(|v| v as usize),
            depth_limit: val.depth_limit.map(|v| v as usize),
        }
    }
}

impl From<spikard_graphql::QueryMutationConfig> for QueryMutationConfig {
    fn from(val: spikard_graphql::QueryMutationConfig) -> Self {
        Self {
            introspection_enabled: val.introspection_enabled,
            complexity_limit: val.complexity_limit.map(|v| v as i64),
            depth_limit: val.depth_limit.map(|v| v as i64),
        }
    }
}

impl From<FullSchemaConfig> for spikard_graphql::FullSchemaConfig {
    fn from(val: FullSchemaConfig) -> Self {
        Self {
            introspection_enabled: val.introspection_enabled,
            complexity_limit: val.complexity_limit.map(|v| v as usize),
            depth_limit: val.depth_limit.map(|v| v as usize),
        }
    }
}

impl From<spikard_graphql::FullSchemaConfig> for FullSchemaConfig {
    fn from(val: spikard_graphql::FullSchemaConfig) -> Self {
        Self {
            introspection_enabled: val.introspection_enabled,
            complexity_limit: val.complexity_limit.map(|v| v as i64),
            depth_limit: val.depth_limit.map(|v| v as i64),
        }
    }
}

/// Convert a `spikard_graphql::error::GraphQLError` error to a PHP exception.
#[allow(dead_code)]
fn graph_q_l_error_to_php_err(e: spikard_graphql::error::GraphQLError) -> ext_php_rs::exception::PhpException {
    let msg = e.to_string();
    #[allow(unreachable_patterns)]
    match &e {
        spikard_graphql::error::GraphQLError::ExecutionError(..) => {
            ext_php_rs::exception::PhpException::default(format!("[ExecutionError] {}", msg))
        }
        spikard_graphql::error::GraphQLError::SchemaBuildError(..) => {
            ext_php_rs::exception::PhpException::default(format!("[SchemaBuildError] {}", msg))
        }
        spikard_graphql::error::GraphQLError::RequestHandlingError(..) => {
            ext_php_rs::exception::PhpException::default(format!("[RequestHandlingError] {}", msg))
        }
        spikard_graphql::error::GraphQLError::SerializationError(..) => {
            ext_php_rs::exception::PhpException::default(format!("[SerializationError] {}", msg))
        }
        spikard_graphql::error::GraphQLError::JsonError(..) => {
            ext_php_rs::exception::PhpException::default(format!("[JsonError] {}", msg))
        }
        spikard_graphql::error::GraphQLError::ValidationError(..) => {
            ext_php_rs::exception::PhpException::default(format!("[ValidationError] {}", msg))
        }
        spikard_graphql::error::GraphQLError::ParseError(..) => {
            ext_php_rs::exception::PhpException::default(format!("[ParseError] {}", msg))
        }
        spikard_graphql::error::GraphQLError::AuthenticationError(..) => {
            ext_php_rs::exception::PhpException::default(format!("[AuthenticationError] {}", msg))
        }
        spikard_graphql::error::GraphQLError::AuthorizationError(..) => {
            ext_php_rs::exception::PhpException::default(format!("[AuthorizationError] {}", msg))
        }
        spikard_graphql::error::GraphQLError::NotFound(..) => {
            ext_php_rs::exception::PhpException::default(format!("[NotFound] {}", msg))
        }
        spikard_graphql::error::GraphQLError::RateLimitExceeded(..) => {
            ext_php_rs::exception::PhpException::default(format!("[RateLimitExceeded] {}", msg))
        }
        spikard_graphql::error::GraphQLError::InvalidInput { .. } => {
            ext_php_rs::exception::PhpException::default(format!("[InvalidInput] {}", msg))
        }
        spikard_graphql::error::GraphQLError::ComplexityLimitExceeded => {
            ext_php_rs::exception::PhpException::default(format!("[ComplexityLimitExceeded] {}", msg))
        }
        spikard_graphql::error::GraphQLError::DepthLimitExceeded => {
            ext_php_rs::exception::PhpException::default(format!("[DepthLimitExceeded] {}", msg))
        }
        spikard_graphql::error::GraphQLError::InternalError(..) => {
            ext_php_rs::exception::PhpException::default(format!("[InternalError] {}", msg))
        }
        _ => ext_php_rs::exception::PhpException::default(msg),
    }
}

/// Convert a `spikard_graphql::schema::SchemaError` error to a PHP exception.
#[allow(dead_code)]
fn schema_error_to_php_err(e: spikard_graphql::schema::SchemaError) -> ext_php_rs::exception::PhpException {
    let msg = e.to_string();
    #[allow(unreachable_patterns)]
    match &e {
        spikard_graphql::schema::SchemaError::BuildingFailed(..) => {
            ext_php_rs::exception::PhpException::default(format!("[BuildingFailed] {}", msg))
        }
        spikard_graphql::schema::SchemaError::ValidationError(..) => {
            ext_php_rs::exception::PhpException::default(format!("[ValidationError] {}", msg))
        }
        spikard_graphql::schema::SchemaError::ComplexityLimitExceeded { .. } => {
            ext_php_rs::exception::PhpException::default(format!("[ComplexityLimitExceeded] {}", msg))
        }
        spikard_graphql::schema::SchemaError::DepthLimitExceeded { .. } => {
            ext_php_rs::exception::PhpException::default(format!("[DepthLimitExceeded] {}", msg))
        }
        _ => ext_php_rs::exception::PhpException::default(msg),
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .class::<Claims>()
        .class::<BackgroundTaskConfig>()
        .class::<BackgroundJobMetadata>()
        .class::<BackgroundJobError>()
        .class::<BackgroundHandle>()
        .class::<CorsConfig>()
        .class::<RouteMetadata>()
        .class::<CompressionConfig>()
        .class::<RateLimitConfig>()
        .class::<ProblemDetails>()
        .class::<JsonRpcMethodInfo>()
        .class::<Route>()
        .class::<GrpcRequestData>()
        .class::<GrpcResponseData>()
        .class::<GrpcConfig>()
        .class::<ValidatedParams>()
        .class::<JsonRpcConfig>()
        .class::<OpenApiConfig>()
        .class::<ContactInfo>()
        .class::<LicenseInfo>()
        .class::<ServerInfo>()
        .class::<Response>()
        .class::<SseEvent>()
        .class::<JwtConfig>()
        .class::<ApiKeyConfig>()
        .class::<StaticFilesConfig>()
        .class::<ServerConfig>()
        .class::<UploadFile>()
        .class::<GraphQLError>()
        .class::<GraphQLRouteConfig>()
        .class::<SchemaConfig>()
        .class::<QueryOnlyConfig>()
        .class::<QueryMutationConfig>()
        .class::<FullSchemaConfig>()
        .class::<SpikardPhpApi>()
}

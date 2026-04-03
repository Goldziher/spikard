#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::missing_errors_doc)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(clippy::should_implement_trait)]
#![cfg(feature = "extension-module")]
#![cfg_attr(all(windows, target_env = "msvc", feature = "extension-module"), feature(abi_vectorcall))]

use ext_php_rs::prelude::*;
use std::sync::Arc;

pub mod php;

static WORKER_RUNTIME: std::sync::LazyLock<tokio::runtime::Runtime> = std::sync::LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime")
});

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub expose_headers: Option<Vec<String>>,
    pub max_age: Option<u32>,
    pub allow_credentials: Option<bool>,
    pub methods_joined_cache: String,
    pub headers_joined_cache: String,
}

#[php_impl]
impl CorsConfig {
    pub fn __construct(
            allowed_origins: Vec<String>,
            allowed_methods: Vec<String>,
            allowed_headers: Vec<String>,
            methods_joined_cache: String,
            headers_joined_cache: String,
            expose_headers: Option<Vec<String>>,
            max_age: Option<u32>,
            allow_credentials: Option<bool>,
        ) -> Self {
        Self { allowed_origins, allowed_methods, allowed_headers, expose_headers, max_age, allow_credentials, methods_joined_cache, headers_joined_cache }
    }

    pub fn allowed_methods_joined(&self) -> String {
        String::from("[unimplemented: allowed_methods_joined]")
    }

    pub fn allowed_headers_joined(&self) -> String {
        String::from("[unimplemented: allowed_headers_joined]")
    }

    pub fn is_origin_allowed(&self) -> bool {
        false
    }

    pub fn is_method_allowed(&self) -> bool {
        false
    }

    pub fn are_headers_allowed(&self) -> bool {
        false
    }

    pub fn default() -> CorsConfig {
        todo!("Not auto-delegatable: default -- return type requires custom implementation")
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct CompressionConfig {
    pub gzip: bool,
    pub brotli: bool,
    pub min_size: i64,
    pub quality: u32,
}

#[php_impl]
impl CompressionConfig {
    pub fn __construct(gzip: bool, brotli: bool, min_size: i64, quality: u32) -> Self {
        Self { gzip, brotli, min_size, quality }
    }

    pub fn default() -> CompressionConfig {
        todo!("Not auto-delegatable: default -- return type requires custom implementation")
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct RateLimitConfig {
    pub per_second: i64,
    pub burst: u32,
    pub ip_based: bool,
}

#[php_impl]
impl RateLimitConfig {
    pub fn __construct(per_second: i64, burst: u32, ip_based: bool) -> Self {
        Self { per_second, burst, ip_based }
    }

    pub fn default() -> RateLimitConfig {
        todo!("Not auto-delegatable: default -- return type requires custom implementation")
    }
}

#[derive(Clone)]
#[php_class]
pub struct LifecycleHooks {
    inner: Arc<spikard::LifecycleHooks>,
}

#[php_impl]
impl LifecycleHooks {
    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn add_on_request(&self) -> () {
        ()
    }

    pub fn add_pre_validation(&self) -> () {
        ()
    }

    pub fn add_pre_handler(&self) -> () {
        ()
    }

    pub fn add_on_response(&self) -> () {
        ()
    }

    pub fn add_on_error(&self) -> () {
        ()
    }

    pub fn execute_on_request_async(&self) -> PhpResult<String> {
        Err(ext_php_rs::exception::PhpException::default("Not implemented: execute_on_request_async".to_string()).into())
    }

    pub fn execute_pre_validation_async(&self) -> PhpResult<String> {
        Err(ext_php_rs::exception::PhpException::default("Not implemented: execute_pre_validation_async".to_string()).into())
    }

    pub fn execute_pre_handler_async(&self) -> PhpResult<String> {
        Err(ext_php_rs::exception::PhpException::default("Not implemented: execute_pre_handler_async".to_string()).into())
    }

    pub fn execute_on_response_async(&self) -> PhpResult<String> {
        Err(ext_php_rs::exception::PhpException::default("Not implemented: execute_on_response_async".to_string()).into())
    }

    pub fn execute_on_error_async(&self) -> PhpResult<String> {
        Err(ext_php_rs::exception::PhpException::default("Not implemented: execute_on_error_async".to_string()).into())
    }

    pub fn builder() -> LifecycleHooksBuilder {
        todo!("Not auto-delegatable: builder -- return type requires custom implementation")
    }
}

#[derive(Clone)]
#[php_class]
pub struct LifecycleHooksBuilder {
    inner: Arc<spikard::LifecycleHooksBuilder>,
}

#[php_impl]
impl LifecycleHooksBuilder {
    pub fn on_request(&self) -> LifecycleHooksBuilder {
        todo!("Not auto-delegatable: on_request -- return type requires custom implementation")
    }

    pub fn pre_validation(&self) -> LifecycleHooksBuilder {
        todo!("Not auto-delegatable: pre_validation -- return type requires custom implementation")
    }

    pub fn pre_handler(&self) -> LifecycleHooksBuilder {
        todo!("Not auto-delegatable: pre_handler -- return type requires custom implementation")
    }

    pub fn on_response(&self) -> LifecycleHooksBuilder {
        todo!("Not auto-delegatable: on_response -- return type requires custom implementation")
    }

    pub fn on_error(&self) -> LifecycleHooksBuilder {
        todo!("Not auto-delegatable: on_error -- return type requires custom implementation")
    }

    pub fn build(&self) -> LifecycleHooks {
        todo!("Not auto-delegatable: build -- return type requires custom implementation")
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct SseEvent {
    pub event_type: Option<String>,
    pub data: String,
    pub id: Option<String>,
    pub retry: Option<i64>,
}

#[php_impl]
impl SseEvent {
    pub fn __construct(data: String, event_type: Option<String>, id: Option<String>, retry: Option<i64>) -> Self {
        Self { event_type, data, id, retry }
    }

    pub fn with_id(&self) -> SseEvent {
        todo!("Not auto-delegatable: with_id -- return type requires custom implementation")
    }

    pub fn with_retry(&self) -> SseEvent {
        todo!("Not auto-delegatable: with_retry -- return type requires custom implementation")
    }

    pub fn with_type() -> SseEvent {
        todo!("Not auto-delegatable: with_type -- return type requires custom implementation")
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct StaticFilesConfig {
    pub directory: String,
    pub route_prefix: String,
    pub index_file: bool,
    pub cache_control: Option<String>,
}

#[php_impl]
impl StaticFilesConfig {
    pub fn __construct(directory: String, route_prefix: String, index_file: bool, cache_control: Option<String>) -> Self {
        Self { directory, route_prefix, index_file, cache_control }
    }
}

#[derive(Clone)]
#[php_class]
pub struct App {
    inner: Arc<spikard::App>,
}

#[php_impl]
impl App {
    pub fn config(&self) -> App {
        todo!("Not auto-delegatable: config -- return type requires custom implementation")
    }

    pub fn merge_axum_router(&self) -> App {
        todo!("Not auto-delegatable: merge_axum_router -- return type requires custom implementation")
    }

    pub fn attach_axum_router(&self) -> App {
        todo!("Not auto-delegatable: attach_axum_router -- return type requires custom implementation")
    }

    pub fn into_router(&self) -> PhpResult<String> {
        Err(ext_php_rs::exception::PhpException::default("Not implemented: into_router".to_string()).into())
    }

    pub fn run_async(&self) -> PhpResult<()> {
        Err(ext_php_rs::exception::PhpException::default("Not implemented: run_async".to_string()).into())
    }

    pub fn default() -> App {
        todo!("Not auto-delegatable: default -- return type requires custom implementation")
    }
}

#[derive(Clone)]
#[php_class]
pub struct RouteBuilder {
    inner: Arc<spikard::RouteBuilder>,
}

#[php_impl]
impl RouteBuilder {
    pub fn handler_name(&self) -> RouteBuilder {
        todo!("Not auto-delegatable: handler_name -- return type requires custom implementation")
    }

    pub fn request_schema_json(&self) -> RouteBuilder {
        todo!("Not auto-delegatable: request_schema_json -- return type requires custom implementation")
    }

    pub fn response_schema_json(&self) -> RouteBuilder {
        todo!("Not auto-delegatable: response_schema_json -- return type requires custom implementation")
    }

    pub fn params_schema_json(&self) -> RouteBuilder {
        todo!("Not auto-delegatable: params_schema_json -- return type requires custom implementation")
    }

    pub fn file_params_json(&self) -> RouteBuilder {
        todo!("Not auto-delegatable: file_params_json -- return type requires custom implementation")
    }

    pub fn cors(&self) -> RouteBuilder {
        todo!("Not auto-delegatable: cors -- return type requires custom implementation")
    }

    pub fn sync(&self) -> RouteBuilder {
        todo!("Not auto-delegatable: sync -- return type requires custom implementation")
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

// HandlerResponse enum values
pub const HANDLERRESPONSE_RESPONSE: &str = "Response";
pub const HANDLERRESPONSE_STREAM: &str = "Stream";

// AppError enum values
pub const APPERROR_ROUTE: &str = "Route";
pub const APPERROR_SERVER: &str = "Server";
pub const APPERROR_DECODE: &str = "Decode";

#[php_function]
pub fn validate_jsonrpc_method_name() -> PhpResult<()> {
    Err(ext_php_rs::exception::PhpException::default("Not implemented: validate_jsonrpc_method_name".to_string()).into())
}

#[php_function]
pub fn handle_preflight() -> PhpResult<String> {
    Err(ext_php_rs::exception::PhpException::default("Not implemented: handle_preflight".to_string()).into())
}

#[php_function]
pub fn add_cors_headers() -> () {
    ()
}

#[php_function]
pub fn validate_cors_request() -> PhpResult<()> {
    Err(ext_php_rs::exception::PhpException::default("Not implemented: validate_cors_request".to_string()).into())
}

impl From<CompressionConfig> for spikard::CompressionConfig {
    fn from(val: CompressionConfig) -> Self {
        Self {
            gzip: val.gzip,
            brotli: val.brotli,
            min_size: val.min_size as usize,
            quality: val.quality,
        }
    }
}

impl From<spikard::CompressionConfig> for CompressionConfig {
    fn from(val: spikard::CompressionConfig) -> Self {
        Self {
            gzip: val.gzip,
            brotli: val.brotli,
            min_size: val.min_size as i64,
            quality: val.quality,
        }
    }
}

impl From<RateLimitConfig> for spikard::RateLimitConfig {
    fn from(val: RateLimitConfig) -> Self {
        Self {
            per_second: val.per_second as u64,
            burst: val.burst,
            ip_based: val.ip_based,
        }
    }
}

impl From<spikard::RateLimitConfig> for RateLimitConfig {
    fn from(val: spikard::RateLimitConfig) -> Self {
        Self {
            per_second: val.per_second as i64,
            burst: val.burst,
            ip_based: val.ip_based,
        }
    }
}

impl From<StaticFilesConfig> for spikard::StaticFilesConfig {
    fn from(val: StaticFilesConfig) -> Self {
        Self {
            directory: val.directory,
            route_prefix: val.route_prefix,
            index_file: val.index_file,
            cache_control: val.cache_control,
        }
    }
}

impl From<spikard::StaticFilesConfig> for StaticFilesConfig {
    fn from(val: spikard::StaticFilesConfig) -> Self {
        Self {
            directory: val.directory,
            route_prefix: val.route_prefix,
            index_file: val.index_file,
            cache_control: val.cache_control,
        }
    }
}

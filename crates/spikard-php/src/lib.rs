#![cfg(feature = "extension-module")]
#![cfg_attr(all(windows, target_env = "msvc", feature = "extension-module"), feature(abi_vectorcall))]

use ext_php_rs::prelude::*;
use std::collections::HashMap;
use spikard;
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
        todo!("call into core implementation")
    }

    pub fn allowed_headers_joined(&self) -> String {
        todo!("call into core implementation")
    }

    pub fn is_origin_allowed(&self) -> bool {
        todo!("call into core implementation")
    }

    pub fn is_method_allowed(&self) -> bool {
        todo!("call into core implementation")
    }

    pub fn are_headers_allowed(&self) -> bool {
        todo!("call into core implementation")
    }

    pub fn default() -> String {
        todo!("call into core implementation")
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

    pub fn default() -> String {
        todo!("call into core implementation")
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

    pub fn default() -> String {
        todo!("call into core implementation")
    }
}

#[derive(Clone)]
#[php_class]
pub struct LifecycleHooks {
    inner: std::sync::Arc<spikard::LifecycleHooks>,
}

#[php_impl]
impl LifecycleHooks {
    pub fn is_empty(&self) -> bool {
        todo!("call into core implementation")
    }

    pub fn add_on_request(&self) -> () {
        todo!("call into core implementation")
    }

    pub fn add_pre_validation(&self) -> () {
        todo!("call into core implementation")
    }

    pub fn add_pre_handler(&self) -> () {
        todo!("call into core implementation")
    }

    pub fn add_on_response(&self) -> () {
        todo!("call into core implementation")
    }

    pub fn add_on_error(&self) -> () {
        todo!("call into core implementation")
    }

    pub fn execute_on_request_async(&self) -> PhpResult<String> {
        todo!("wire up execute_on_request_async")
    }

    pub fn execute_pre_validation_async(&self) -> PhpResult<String> {
        todo!("wire up execute_pre_validation_async")
    }

    pub fn execute_pre_handler_async(&self) -> PhpResult<String> {
        todo!("wire up execute_pre_handler_async")
    }

    pub fn execute_on_response_async(&self) -> PhpResult<String> {
        todo!("wire up execute_on_response_async")
    }

    pub fn execute_on_error_async(&self) -> PhpResult<String> {
        todo!("wire up execute_on_error_async")
    }

    pub fn builder() -> LifecycleHooksBuilder {
        todo!("call into core implementation")
    }
}

#[derive(Clone)]
#[php_class]
pub struct LifecycleHooksBuilder {
    inner: std::sync::Arc<spikard::LifecycleHooksBuilder>,
}

#[php_impl]
impl LifecycleHooksBuilder {
    pub fn on_request(&self) -> String {
        todo!("call into core implementation")
    }

    pub fn pre_validation(&self) -> String {
        todo!("call into core implementation")
    }

    pub fn pre_handler(&self) -> String {
        todo!("call into core implementation")
    }

    pub fn on_response(&self) -> String {
        todo!("call into core implementation")
    }

    pub fn on_error(&self) -> String {
        todo!("call into core implementation")
    }

    pub fn build(&self) -> LifecycleHooks {
        todo!("call into core implementation")
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

    pub fn with_id(&self) -> String {
        todo!("call into core implementation")
    }

    pub fn with_retry(&self) -> String {
        todo!("call into core implementation")
    }

    pub fn with_type() -> String {
        todo!("call into core implementation")
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
    inner: std::sync::Arc<spikard::App>,
}

#[php_impl]
impl App {
    pub fn config(&self) -> String {
        todo!("call into core implementation")
    }

    pub fn merge_axum_router(&self) -> String {
        todo!("call into core implementation")
    }

    pub fn attach_axum_router(&self) -> String {
        todo!("call into core implementation")
    }

    pub fn into_router(&self) -> PhpResult<String> {
        todo!("call into core implementation")
    }

    pub fn run_async(&self) -> PhpResult<()> {
        todo!("wire up run_async")
    }

    pub fn default() -> String {
        todo!("call into core implementation")
    }
}

#[derive(Clone)]
#[php_class]
pub struct RouteBuilder {
    inner: std::sync::Arc<spikard::RouteBuilder>,
}

#[php_impl]
impl RouteBuilder {
    pub fn handler_name(&self) -> String {
        todo!("call into core implementation")
    }

    pub fn request_schema_json(&self) -> String {
        todo!("call into core implementation")
    }

    pub fn response_schema_json(&self) -> String {
        todo!("call into core implementation")
    }

    pub fn params_schema_json(&self) -> String {
        todo!("call into core implementation")
    }

    pub fn file_params_json(&self) -> String {
        todo!("call into core implementation")
    }

    pub fn cors(&self) -> String {
        todo!("call into core implementation")
    }

    pub fn sync(&self) -> String {
        todo!("call into core implementation")
    }

    pub fn handler_dependencies(&self) -> String {
        todo!("call into core implementation")
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
    todo!("call into core")
}

#[php_function]
pub fn handle_preflight() -> PhpResult<String> {
    todo!("call into core")
}

#[php_function]
pub fn add_cors_headers() -> () {
    todo!("call into core")
}

#[php_function]
pub fn validate_cors_request() -> PhpResult<()> {
    todo!("call into core")
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

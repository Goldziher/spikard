#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::missing_errors_doc)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(clippy::should_implement_trait)]
#![cfg(feature = "extension-module")]
#![cfg_attr(all(windows, target_env = "msvc", feature = "extension-module"), feature(abi_vectorcall))]

use ext_php_rs::prelude::*;
use serde_json;
use std::sync::Arc;

pub mod php;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
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
        let core_self = spikard::CorsConfig {
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
        let core_self = spikard::CorsConfig {
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
        let core_self = spikard::CorsConfig {
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
        let core_self = spikard::CorsConfig {
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

    pub fn default() -> CorsConfig {
        spikard::CorsConfig::default().into()
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
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
        spikard::CompressionConfig::default().into()
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
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
        spikard::RateLimitConfig::default().into()
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
        self.inner.is_empty()
    }

    pub fn add_on_request(&self, hook: String) -> () {
        ()
    }

    pub fn add_pre_validation(&self, hook: String) -> () {
        ()
    }

    pub fn add_pre_handler(&self, hook: String) -> () {
        ()
    }

    pub fn add_on_response(&self, hook: String) -> () {
        ()
    }

    pub fn add_on_error(&self, hook: String) -> () {
        ()
    }

    pub fn builder() -> String {
        String::from("[unimplemented: builder]")
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
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

    pub fn with_id(&self, id: String) -> SseEvent {
        let core_self = spikard::SseEvent {
                event_type: self.event_type.clone(),
                data: Default::default(),
                id: self.id.clone(),
                retry: self.retry,
            };
            core_self.with_id(&id).into()
    }

    pub fn with_retry(&self, retry_ms: i64) -> SseEvent {
        let core_self = spikard::SseEvent {
                event_type: self.event_type.clone(),
                data: Default::default(),
                id: self.id.clone(),
                retry: self.retry,
            };
            core_self.with_retry(retry_ms).into()
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
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
    pub fn default() -> App {
        Self { inner: Arc::new(spikard::App::default()) }
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
pub fn validate_jsonrpc_method_name(name: String) -> PhpResult<()> {
    let result = spikard::validate_jsonrpc_method_name(&name).map_err(|e| ext_php_rs::exception::PhpException::default(e.to_string()))?;
    Ok(result)
}

#[php_function]
pub fn add_cors_headers(response: String, origin: String, cors_config: CorsConfig) -> () {
    ()
}

impl From<spikard::CorsConfig> for CorsConfig {
    fn from(val: spikard::CorsConfig) -> Self {
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

impl From<spikard::SseEvent> for SseEvent {
    fn from(val: spikard::SseEvent) -> Self {
        Self {
            event_type: val.event_type,
            data: format!("{:?}", val.data),
            id: val.id,
            retry: val.retry.map(|v| v as i64),
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

//! PHP HTTP server implementation.
//!
//! This module provides the `PhpServer` class that can be used from PHP
//! to create and run an HTTP server with Spikard's middleware stack.

use axum::body::Body;
use axum::http::{HeaderName, HeaderValue, Request, Response, StatusCode};
use ext_php_rs::boxed::ZBox;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendCallable, ZendHashTable, Zval};
use spikard_http::server::build_router_with_handlers_and_config;
use spikard_http::{
    Handler, HandlerResult, LifecycleHooks, Method, RequestData, Route, Router, ServerConfig, ServerConfigBuilder,
};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::php::handler::PhpHandler;

use super::zval_to_json;

/// A registered route with its handler.
struct RegisteredRoute {
    method: String,
    path: String,
    handler_name: String,
    handler_index: usize,
}

/// PHP-visible HTTP server class.
#[php_class]
#[php(name = "Spikard\\Server")]
pub struct PhpServer {
    routes: Vec<RegisteredRoute>,
    host: String,
    port: u16,
    /// Stored PHP callables for registered routes
    handlers: Vec<ZendCallable>,
    /// Optional server configuration (populated via setters)
    config: ServerConfig,
    /// Lifecycle hooks (not yet exposed to PHP; placeholder for parity)
    lifecycle_hooks: Option<LifecycleHooks>,
}

#[php_impl]
impl PhpServer {
    /// Create a new server instance.
    pub fn new(host: Option<String>, port: Option<i64>) -> Self {
        Self {
            routes: Vec::new(),
            host: host.unwrap_or_else(|| "127.0.0.1".to_string()),
            port: port.map(|p| p as u16).unwrap_or(8000),
            handlers: Vec::new(),
            config: ServerConfig::default(),
            lifecycle_hooks: None,
        }
    }

    /// Register a GET route.
    #[php(name = "get")]
    pub fn register_get(&mut self, path: String, handler: ZendCallable, handler_name: String) {
        let idx = self.handlers.len();
        self.routes.push(RegisteredRoute {
            method: "GET".to_string(),
            path,
            handler_name,
            handler_index: idx,
        });
        self.handlers.push(handler);
    }

    /// Register a POST route.
    #[php(name = "post")]
    pub fn register_post(&mut self, path: String, handler: ZendCallable, handler_name: String) {
        let idx = self.handlers.len();
        self.routes.push(RegisteredRoute {
            method: "POST".to_string(),
            path,
            handler_name,
            handler_index: idx,
        });
        self.handlers.push(handler);
    }

    /// Register a PUT route.
    #[php(name = "put")]
    pub fn register_put(&mut self, path: String, handler: ZendCallable, handler_name: String) {
        let idx = self.handlers.len();
        self.routes.push(RegisteredRoute {
            method: "PUT".to_string(),
            path,
            handler_name,
            handler_index: idx,
        });
        self.handlers.push(handler);
    }

    /// Register a PATCH route.
    #[php(name = "patch")]
    pub fn register_patch(&mut self, path: String, handler: ZendCallable, handler_name: String) {
        let idx = self.handlers.len();
        self.routes.push(RegisteredRoute {
            method: "PATCH".to_string(),
            path,
            handler_name,
            handler_index: idx,
        });
        self.handlers.push(handler);
    }

    /// Register a DELETE route.
    #[php(name = "delete")]
    pub fn register_delete(&mut self, path: String, handler: ZendCallable, handler_name: String) {
        let idx = self.handlers.len();
        self.routes.push(RegisteredRoute {
            method: "DELETE".to_string(),
            path,
            handler_name,
            handler_index: idx,
        });
        self.handlers.push(handler);
    }

    /// Get registered routes as a PHP array.
    #[php(name = "getRoutes")]
    pub fn get_routes(&self) -> PhpResult<ZBox<ZendHashTable>> {
        let mut table = ZendHashTable::new();
        for (i, route) in self.routes.iter().enumerate() {
            let mut route_table = ZendHashTable::new();
            route_table.insert("method", route.method.as_str())?;
            route_table.insert("path", route.path.as_str())?;
            route_table.insert("handler_name", route.handler_name.as_str())?;
            // Use i64 key for array index
            table.insert(i as i64, route_table)?;
        }
        Ok(table)
    }

    /// Get the configured host.
    #[php(name = "getHost")]
    pub fn get_host(&self) -> String {
        self.host.clone()
    }

    /// Get the configured port.
    #[php(name = "getPort")]
    pub fn get_port(&self) -> i64 {
        self.port as i64
    }

    /// Set request timeout (milliseconds)
    #[php(name = "setTimeoutMs")]
    pub fn set_timeout_ms(&mut self, timeout_ms: i64) {
        self.config.timeout_ms = Some(timeout_ms as u64);
    }

    /// Enable/disable compression
    #[php(name = "enableCompression")]
    pub fn enable_compression(&mut self, enabled: bool) {
        self.config.enable_compression = enabled;
    }

    /// Configure compression (quality and minimum size)
    #[php(name = "setCompression")]
    pub fn set_compression(&mut self, enabled: bool, quality: Option<i64>, min_size: Option<i64>) {
        if enabled {
            let mut cfg = spikard_core::CompressionConfig::default();
            if let Some(q) = quality {
                cfg.quality = q as u8;
            }
            if let Some(sz) = min_size {
                cfg.min_size = sz as usize;
            }
            self.config.compression = Some(cfg);
        } else {
            self.config.compression = None;
        }
    }

    /// Enable/disable request ID middleware
    #[php(name = "enableRequestId")]
    pub fn enable_request_id(&mut self, enabled: bool) {
        self.config.enable_request_id = enabled;
    }

    /// Set maximum body size (bytes)
    #[php(name = "setMaxBodySize")]
    pub fn set_max_body_size(&mut self, max_bytes: i64) {
        self.config.max_body_size = Some(max_bytes as usize);
    }

    /// Disable max body size limit
    #[php(name = "disableMaxBodySize")]
    pub fn disable_max_body_size(&mut self) {
        self.config.max_body_size = None;
    }

    /// Configure JWT auth
    #[php(name = "setJwtAuth")]
    pub fn set_jwt_auth(
        &mut self,
        secret: String,
        algorithm: Option<String>,
        audience: Option<Vec<String>>,
        issuer: Option<String>,
        leeway: Option<i64>,
    ) {
        let mut cfg = spikard_http::JwtConfig {
            secret,
            algorithm: algorithm.unwrap_or_else(|| "HS256".to_string()),
            audience,
            issuer,
            leeway: leeway.unwrap_or(0) as u64,
        };
        self.config.jwt_auth = Some(cfg);
    }

    /// Disable JWT auth
    #[php(name = "disableJwtAuth")]
    pub fn disable_jwt_auth(&mut self) {
        self.config.jwt_auth = None;
    }

    /// Configure API key auth
    #[php(name = "setApiKeyAuth")]
    pub fn set_api_key_auth(&mut self, keys: Vec<String>, header_name: Option<String>) {
        let mut cfg = spikard_http::ApiKeyConfig {
            keys,
            header_name: header_name.unwrap_or_else(|| "X-API-Key".to_string()),
        };
        self.config.api_key_auth = Some(cfg);
    }

    /// Disable API key auth
    #[php(name = "disableApiKeyAuth")]
    pub fn disable_api_key_auth(&mut self) {
        self.config.api_key_auth = None;
    }

    /// Configure CORS
    #[php(name = "setCors")]
    pub fn set_cors(
        &mut self,
        allow_origin: Vec<String>,
        allow_methods: Vec<String>,
        allow_headers: Vec<String>,
        expose_headers: Vec<String>,
        allow_credentials: bool,
        max_age_seconds: Option<i64>,
    ) {
        let cfg = spikard_core::CorsConfig {
            allow_origin,
            allow_methods,
            allow_headers,
            expose_headers,
            allow_credentials,
            max_age_seconds: max_age_seconds.map(|v| v as u64),
        };
        self.config.cors = Some(cfg);
    }

    /// Disable CORS
    #[php(name = "disableCors")]
    pub fn disable_cors(&mut self) {
        self.config.cors = None;
    }

    /// Add static files config
    #[php(name = "addStaticFiles")]
    pub fn add_static_files(
        &mut self,
        directory: String,
        route_prefix: String,
        index_file: Option<bool>,
        cache_control: Option<String>,
    ) {
        let cfg = spikard_http::StaticFilesConfig {
            directory,
            route_prefix,
            index_file: index_file.unwrap_or(true),
            cache_control,
        };
        self.config.static_files.push(cfg);
    }

    /// Clear static files configs
    #[php(name = "clearStaticFiles")]
    pub fn clear_static_files(&mut self) {
        self.config.static_files.clear();
    }

    /// Enable/disable graceful shutdown
    #[php(name = "enableGracefulShutdown")]
    pub fn enable_graceful_shutdown(&mut self, enabled: bool) {
        self.config.graceful_shutdown = enabled;
    }

    /// Set lifecycle hooks (placeholder: to be wired via PHP API)
    #[allow(dead_code)]
    pub fn set_lifecycle_hooks(&mut self, hooks: LifecycleHooks) {
        self.lifecycle_hooks = Some(hooks);
    }

    /// Configure rate limiting
    #[php(name = "setRateLimit")]
    pub fn set_rate_limit(&mut self, per_second: i64, burst: Option<i64>, ip_based: Option<bool>) {
        let mut cfg = spikard_core::RateLimitConfig::default();
        cfg.per_second = per_second as u64;
        if let Some(b) = burst {
            cfg.burst = b as u32;
        }
        if let Some(ip) = ip_based {
            cfg.ip_based = ip;
        }
        self.config.rate_limit = Some(cfg);
    }

    /// Disable rate limiting
    #[php(name = "disableRateLimit")]
    pub fn disable_rate_limit(&mut self) {
        self.config.rate_limit = None;
    }

    /// Set graceful shutdown timeout (seconds)
    #[php(name = "setShutdownTimeout")]
    pub fn set_shutdown_timeout(&mut self, seconds: i64) {
        self.config.shutdown_timeout = seconds as u64;
    }

    /// Set host
    #[php(name = "setHost")]
    pub fn set_host(&mut self, host: String) {
        self.host = host;
        self.config.host = host;
    }

    /// Set port
    #[php(name = "setPort")]
    pub fn set_port(&mut self, port: i64) {
        self.port = port as u16;
        self.config.port = self.port;
    }
}

impl PhpServer {
    /// Get server configuration.
    #[allow(dead_code)]
    pub fn config(&self) -> ServerConfig {
        ServerConfig {
            host: self.host.clone(),
            port: self.port,
            ..self.config.clone()
        }
    }

    /// Build a metadata Router (no handler binding) from registered routes.
    pub fn build_router(&self) -> Router {
        let mut router = Router::new();

        for route in &self.routes {
            let method = route.method.parse().unwrap_or_else(|_| Method::GET);

            let route_meta = Route {
                method,
                path: route.path.clone(),
                handler_name: route.handler_name.clone(),
                request_validator: None,
                response_validator: None,
                parameter_validator: None,
                file_params: None,
                is_async: false,
                cors: None,
                expects_json_body: false,
                #[cfg(feature = "di")]
                handler_dependencies: Vec::new(),
            };

            router.add_route(route_meta);
        }

        router
    }

    /// Build route/handler pairs for Server::with_handlers.
    pub fn build_routes_with_handlers(&self) -> Vec<(Route, Arc<dyn Handler>)> {
        self.routes
            .iter()
            .map(|route| {
                let method = route.method.parse().unwrap_or_else(|_| Method::GET);

                let handler = PhpHandler::register(
                    self.handlers
                        .get(route.handler_index)
                        .expect("handler index should be valid")
                        .clone(),
                    route.handler_name.clone(),
                    route.method.clone(),
                    route.path.clone(),
                );

                let metadata = Route {
                    method: method.clone(),
                    path: route.path.clone(),
                    handler_name: route.handler_name.clone(),
                    request_validator: None,
                    response_validator: None,
                    parameter_validator: None,
                    file_params: None,
                    is_async: false,
                    cors: None,
                    expects_json_body: false,
                    #[cfg(feature = "di")]
                    handler_dependencies: Vec::new(),
                };

                (metadata, Arc::new(handler) as Arc<dyn Handler>)
            })
            .collect()
    }

    /// Build an Axum router using the shared tower-http stack.
    pub fn build_axum_router(&self) -> Result<axum::Router, String> {
        let routes = self.build_routes_with_handlers();
        let hooks = self.lifecycle_hooks.as_ref().map(Arc::new);
        build_router_with_handlers_and_config(routes, self.config.clone(), hooks)
    }
}

/// Closure-based handler that captures the handler logic.
///
/// Since PHP callables can't be stored across threads, we use a different approach:
/// The test client and server use a callback-based system where the actual PHP
/// callable is invoked synchronously on the PHP thread.
#[allow(dead_code)]
pub struct ClosureHandler {
    handler_fn: Arc<dyn Fn(RequestData) -> HandlerResult + Send + Sync>,
}

#[allow(dead_code)]
impl ClosureHandler {
    /// Create a new closure handler.
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(RequestData) -> HandlerResult + Send + Sync + 'static,
    {
        Self {
            handler_fn: Arc::new(f),
        }
    }
}

impl Handler for ClosureHandler {
    fn call(
        &self,
        _req: Request<Body>,
        request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        let result = (self.handler_fn)(request_data);
        Box::pin(async move { result })
    }
}

/// Helper to interpret a PHP response Zval into HandlerResult.
#[allow(dead_code)]
pub fn interpret_php_response(response: &Zval, _handler_name: &str) -> HandlerResult {
    // If it's null, return 204 No Content
    if response.is_null() {
        return Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(Body::empty())
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to build response: {}", e),
                )
            })?);
    }

    // Try to extract Response - check if object has our expected methods
    if let Some(obj) = response.object() {
        if let Ok(class_name) = obj.get_class_name() {
            if class_name.contains("Response") {
                // Try to call getStatus method
                if let Ok(status_zval) = obj.try_call_method("getStatus", vec![]) {
                    let status_code = status_zval.long().unwrap_or(200);
                    let status = StatusCode::from_u16(status_code as u16).unwrap_or(StatusCode::OK);

                    let mut builder = Response::builder().status(status);

                    // Try to get headers
                    let mut has_content_type = false;
                    if let Ok(headers_zval) = obj.try_call_method("getHeaders", vec![]) {
                        if let Some(arr) = headers_zval.array() {
                            for (key, val) in arr.iter() {
                                let key_str = match key {
                                    ext_php_rs::types::ArrayKey::Long(i) => i.to_string(),
                                    ext_php_rs::types::ArrayKey::String(s) => s.to_string(),
                                    ext_php_rs::types::ArrayKey::Str(s) => s.to_string(),
                                };
                                if let Some(val_str) = val.string() {
                                    if key_str.eq_ignore_ascii_case("content-type") {
                                        has_content_type = true;
                                    }
                                    if let (Ok(header_name), Ok(header_value)) = (
                                        HeaderName::from_bytes(key_str.as_bytes()),
                                        HeaderValue::from_str(&val_str),
                                    ) {
                                        builder = builder.header(header_name, header_value);
                                    }
                                }
                            }
                        }
                    }

                    // Try to get body
                    let body_str = if let Ok(body_zval) = obj.try_call_method("getBody", vec![]) {
                        body_zval.string().map(|s| s.to_string()).unwrap_or_default()
                    } else {
                        String::new()
                    };

                    // Set content-type if not already set and we have a body
                    if !has_content_type && !body_str.is_empty() {
                        builder = builder.header("content-type", "application/json");
                    }

                    return builder.body(Body::from(body_str)).map_err(|e| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Failed to build response: {}", e),
                        )
                    });
                }
            }
        }
    }

    // If it's a string, return as text/plain
    if let Some(s) = response.string() {
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "text/plain")
            .body(Body::from(s.to_string()))
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to build response: {}", e),
                )
            })?);
    }

    // Try to convert to JSON
    let body_json = zval_to_json(response).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to convert response to JSON: {}", e),
        )
    })?;

    let body_bytes = serde_json::to_vec(&body_json).unwrap_or_default();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(body_bytes))
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to build response: {}", e),
            )
        })?)
}

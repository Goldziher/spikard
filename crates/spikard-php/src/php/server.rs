//! PHP HTTP server implementation.
//!
//! This module provides the `PhpServer` class that can be used from PHP
//! to create and run an HTTP server with Spikard's middleware stack.

use axum::body::Body;
use axum::http::{HeaderName, HeaderValue, Request, Response, StatusCode};
use ext_php_rs::boxed::ZBox;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendHashTable, Zval};
use spikard_http::ParameterValidator;
use spikard_http::server::build_router_with_handlers_and_config;
use spikard_http::{CONTENT_TYPE_PROBLEM_JSON, ProblemDetails};
use spikard_http::{Handler, HandlerResult, LifecycleHooks, Method, Route, Router, SchemaRegistry, ServerConfig};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::php::handler::PhpHandler;
use crate::php::hooks::PhpLifecycleHooks;

use super::zval_to_json;

/// Type alias for routes with handlers.
type RouteWithHandler = (Route, Arc<dyn Handler>);

/// A registered route with its handler.
struct RegisteredRoute {
    method: String,
    path: String,
    handler_name: String,
    handler_index: usize,
    request_schema: Option<serde_json::Value>,
    response_schema: Option<serde_json::Value>,
    parameter_schema: Option<serde_json::Value>,
    cors: Option<spikard_core::CorsConfig>,
}

/// PHP-visible HTTP server class.
#[php_class]
#[php(name = "Spikard\\Server")]
pub struct PhpServer {
    routes: Vec<RegisteredRoute>,
    host: String,
    port: u16,
    /// Stored PHP callables for registered routes (as Zvals)
    /// We store Zval instead of ZendCallable to avoid lifetime issues
    handlers: Vec<ext_php_rs::types::Zval>,
    /// Optional server configuration (populated via setters)
    config: ServerConfig,
    /// Lifecycle hooks (not yet exposed to PHP; placeholder for parity)
    lifecycle_hooks: Option<LifecycleHooks>,
    /// Global CORS configuration to apply to all routes
    global_cors: Option<spikard_core::CorsConfig>,
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
            global_cors: None,
        }
    }

    /// Register a GET route.
    #[php(name = "get")]
    pub fn register_get(&mut self, path: String, handler: &ext_php_rs::types::Zval, handler_name: String) {
        let idx = self.handlers.len();
        self.routes.push(RegisteredRoute {
            method: "GET".to_string(),
            path,
            handler_name,
            handler_index: idx,
            request_schema: None,
            response_schema: None,
            parameter_schema: None,
            cors: self.global_cors.clone(),
        });
        self.handlers.push(handler.shallow_clone());
    }

    /// Register a POST route.
    #[php(name = "post")]
    pub fn register_post(&mut self, path: String, handler: &ext_php_rs::types::Zval, handler_name: String) {
        let idx = self.handlers.len();
        self.routes.push(RegisteredRoute {
            method: "POST".to_string(),
            path,
            handler_name,
            handler_index: idx,
            request_schema: None,
            response_schema: None,
            parameter_schema: None,
            cors: self.global_cors.clone(),
        });
        self.handlers.push(handler.shallow_clone());
    }

    /// Register a PUT route.
    #[php(name = "put")]
    pub fn register_put(&mut self, path: String, handler: &ext_php_rs::types::Zval, handler_name: String) {
        let idx = self.handlers.len();
        self.routes.push(RegisteredRoute {
            method: "PUT".to_string(),
            path,
            handler_name,
            handler_index: idx,
            request_schema: None,
            response_schema: None,
            parameter_schema: None,
            cors: self.global_cors.clone(),
        });
        self.handlers.push(handler.shallow_clone());
    }

    /// Register a PATCH route.
    #[php(name = "patch")]
    pub fn register_patch(&mut self, path: String, handler: &ext_php_rs::types::Zval, handler_name: String) {
        let idx = self.handlers.len();
        self.routes.push(RegisteredRoute {
            method: "PATCH".to_string(),
            path,
            handler_name,
            handler_index: idx,
            request_schema: None,
            response_schema: None,
            parameter_schema: None,
            cors: self.global_cors.clone(),
        });
        self.handlers.push(handler.shallow_clone());
    }

    /// Register a DELETE route.
    #[php(name = "delete")]
    pub fn register_delete(&mut self, path: String, handler: &ext_php_rs::types::Zval, handler_name: String) {
        let idx = self.handlers.len();
        self.routes.push(RegisteredRoute {
            method: "DELETE".to_string(),
            path,
            handler_name,
            handler_index: idx,
            request_schema: None,
            response_schema: None,
            parameter_schema: None,
            cors: self.global_cors.clone(),
        });
        self.handlers.push(handler.shallow_clone());
    }

    /// Register a route with optional schemas (request/response/parameters).
    #[php(name = "register")]
    #[allow(clippy::too_many_arguments)]
    pub fn register_with_schemas(
        &mut self,
        method: String,
        path: String,
        handler: &Zval,
        handler_name: String,
        request_schema_json: Option<String>,
        response_schema_json: Option<String>,
        parameter_schema_json: Option<String>,
    ) -> PhpResult<()> {
        let idx = self.handlers.len();
        self.handlers.push(handler.shallow_clone());

        let request_schema = parse_schema(request_schema_json)?;
        let response_schema = parse_schema(response_schema_json)?;
        let parameter_schema = parse_schema(parameter_schema_json)?;

        self.routes.push(RegisteredRoute {
            method: method.to_uppercase(),
            path,
            handler_name,
            handler_index: idx,
            request_schema,
            response_schema,
            parameter_schema,
            cors: self.global_cors.clone(),
        });
        Ok(())
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
        // Convert milliseconds to seconds for ServerConfig.request_timeout
        self.config.request_timeout = Some((timeout_ms as u64).div_ceil(1000));
    }

    /// Enable/disable compression
    #[php(name = "enableCompression")]
    pub fn enable_compression(&mut self, enabled: bool) {
        if enabled {
            self.config.compression = Some(spikard_core::CompressionConfig::default());
        } else {
            self.config.compression = None;
        }
    }

    /// Configure compression (quality and minimum size)
    #[php(name = "setCompression")]
    pub fn set_compression(&mut self, enabled: bool, quality: Option<i64>, min_size: Option<i64>) {
        if enabled {
            let mut cfg = spikard_core::CompressionConfig::default();
            if let Some(q) = quality {
                cfg.quality = q as u32;
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
        let cfg = spikard_http::JwtConfig {
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
        let cfg = spikard_http::ApiKeyConfig {
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

    /// Configure global CORS for all routes
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
        self.global_cors = Some(spikard_core::CorsConfig {
            allowed_origins: allow_origin,
            allowed_methods: allow_methods,
            allowed_headers: allow_headers,
            expose_headers: if expose_headers.is_empty() {
                None
            } else {
                Some(expose_headers)
            },
            max_age: max_age_seconds.map(|s| s as u32),
            allow_credentials: Some(allow_credentials),
        });
    }

    /// Disable global CORS
    #[php(name = "disableCors")]
    pub fn disable_cors(&mut self) {
        self.global_cors = None;
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

    /// Set lifecycle hooks (onRequest/onResponse). Short-circuit supported.
    #[php(name = "setLifecycleHooks")]
    pub fn set_lifecycle_hooks(&mut self, hooks: &PhpLifecycleHooks) {
        self.lifecycle_hooks = Some(hooks.build());
    }

    /// Configure rate limiting
    #[php(name = "setRateLimit")]
    pub fn set_rate_limit(&mut self, per_second: i64, burst: Option<i64>, ip_based: Option<bool>) {
        let cfg = spikard_core::RateLimitConfig {
            per_second: per_second as u64,
            burst: burst.map(|b| b as u32).unwrap_or(10),
            ip_based: ip_based.unwrap_or(true),
        };
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
        self.host = host.clone();
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
            let method = route.method.parse().unwrap_or(Method::Get);

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
                expects_json_body: route.request_schema.is_some(),
                handler_dependencies: vec![], // PHP routes don't currently declare dependencies
            };

            router.add_route(route_meta);
        }

        router
    }

    /// Build route/handler pairs for Server::with_handlers.
    pub fn build_routes_with_handlers(&self) -> Result<Vec<RouteWithHandler>, String> {
        let registry = SchemaRegistry::new();

        let mut routes_with_handlers = Vec::new();

        for route in &self.routes {
            let method = route.method.parse().unwrap_or(Method::Get);

            let handler_zval = self
                .handlers
                .get(route.handler_index)
                .expect("handler index should be valid");

            let handler = PhpHandler::register_from_zval(
                handler_zval,
                route.handler_name.clone(),
                route.method.clone(),
                route.path.clone(),
            )
            .map_err(|e| format!("Failed to register handler: {}", e))?;

            let request_validator = match &route.request_schema {
                Some(schema) => Some(registry.get_or_compile(schema).map_err(|e| {
                    format!(
                        "Failed to compile request schema for {} {}: {}",
                        route.method, route.path, e
                    )
                })?),
                None => None,
            };

            let response_validator = match &route.response_schema {
                Some(schema) => Some(registry.get_or_compile(schema).map_err(|e| {
                    format!(
                        "Failed to compile response schema for {} {}: {}",
                        route.method, route.path, e
                    )
                })?),
                None => None,
            };

            let parameter_validator = match &route.parameter_schema {
                Some(schema) => Some(ParameterValidator::new(schema.clone()).map_err(|e| {
                    format!(
                        "Failed to compile parameter schema for {} {}: {}",
                        route.method, route.path, e
                    )
                })?),
                None => None,
            };

            let metadata = Route {
                method: method.clone(),
                path: route.path.clone(),
                handler_name: route.handler_name.clone(),
                request_validator,
                response_validator,
                parameter_validator,
                file_params: None,
                is_async: false,
                cors: None,
                expects_json_body: route.request_schema.is_some(),
                handler_dependencies: vec![],
            };

            routes_with_handlers.push((metadata, Arc::new(handler) as Arc<dyn Handler>));
        }

        Ok(routes_with_handlers)
    }

    /// Build an Axum router using the shared tower-http stack.
    pub fn build_axum_router(&self) -> Result<axum::Router, String> {
        let routes = self.build_routes_with_handlers()?;

        // Convert RegisteredRoute to RouteMetadata
        let metadata: Vec<spikard_core::RouteMetadata> = self
            .routes
            .iter()
            .map(|r| {
                spikard_core::RouteMetadata {
                    method: r.method.clone(),
                    path: r.path.clone(),
                    handler_name: r.handler_name.clone(),
                    request_schema: r.request_schema.clone(),
                    response_schema: r.response_schema.clone(),
                    parameter_schema: r.parameter_schema.clone(),
                    file_params: None,
                    is_async: true,       // PHP handlers are always async in our implementation
                    cors: r.cors.clone(), // Use route-specific CORS config
                    body_param_name: None,
                    handler_dependencies: Some(Vec::new()),
                }
            })
            .collect();

        build_router_with_handlers_and_config(routes, self.config.clone(), metadata)
    }
}

/// Parse an optional JSON schema string to Value.
fn parse_schema(schema: Option<String>) -> PhpResult<Option<serde_json::Value>> {
    if let Some(s) = schema {
        let value: serde_json::Value =
            serde_json::from_str(&s).map_err(|e| PhpException::default(format!("Invalid JSON schema: {}", e)))?;
        Ok(Some(value))
    } else {
        Ok(None)
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
            .unwrap_or_else(|e| {
                to_problem(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to build response: {}", e),
                )
                .expect("Failed to build error response")
            }));
    }

    // Try to extract Response - check if object has our expected methods
    if let Some(obj) = response.object().filter(|o| o.has_method("getBody")) {
        // Try to call getStatus method
        if let Ok(status_zval) = obj.try_call_method("getStatus", vec![]) {
            let status_code = status_zval.long().unwrap_or(200);
            let status = StatusCode::from_u16(status_code as u16).unwrap_or(StatusCode::OK);

            let mut builder = Response::builder().status(status);

            // Try to get headers
            let mut has_content_type = false;
            if let Ok(headers_zval) = obj.try_call_method("getHeaders", vec![])
                && let Some(arr) = headers_zval.array()
            {
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

            // Try to get body
            let body_zval = obj.try_call_method("getBody", vec![]).unwrap_or_else(|_| Zval::new());
            if let Some(generator) = body_zval.object()
                && generator.has_method("next")
            {
                // Streaming via generator
                let status_code = status.as_u16();
                let headers = builder
                    .headers_ref()
                    .map(|h| {
                        h.iter()
                            .filter_map(|(name, value)| value.to_str().ok().map(|v| (name.to_string(), v.to_string())))
                            .collect::<HashMap<_, _>>()
                    })
                    .unwrap_or_default();
                let cfg = crate::php::StreamingConfig { status_code, headers };
                return crate::php::register_generator(&body_zval, Some(cfg)).and_then(|(idx, config)| {
                    crate::php::create_streaming_response(idx, config)
                        .map(|r| r.into_response())
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
                });
            }

            let body_str = body_zval.string().map(|s| s.to_string()).unwrap_or_default();

            // Set content-type if not already set and we have a body
            if !has_content_type && !body_str.is_empty() {
                builder = builder.header("content-type", "application/json");
            }

            return builder
                .body(Body::from(body_str))
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to build response: {}", e),
                    )
                })
                .or_else(|(_, msg)| to_problem(StatusCode::INTERNAL_SERVER_ERROR, msg));
        }
    }

    // If it's a string, return as text/plain
    if let Some(s) = response.string() {
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "text/plain")
            .body(Body::from(s.to_string()))
            .unwrap_or_else(|e| {
                to_problem(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to build response: {}", e),
                )
                .expect("Failed to build error response")
            }));
    }

    // Try to convert to JSON
    let body_json = match zval_to_json(response) {
        Ok(val) => val,
        Err(e) => {
            return to_problem(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to convert response: {}", e),
            );
        }
    };

    let body_bytes = serde_json::to_vec(&body_json).unwrap_or_default();
    let mut builder = Response::builder().status(StatusCode::OK);
    builder = builder.header("content-type", "application/json");
    builder
        .body(Body::from(body_bytes))
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to build response: {}", e),
            )
        })
        .or_else(|(_, msg)| to_problem(StatusCode::INTERNAL_SERVER_ERROR, msg))
}

/// Build a structured ProblemDetails response with application/problem+json.
pub fn to_problem(status: StatusCode, detail: impl Into<String>) -> HandlerResult {
    let problem = ProblemDetails::new(
        ProblemDetails::TYPE_INTERNAL_SERVER_ERROR,
        "Internal Server Error",
        status,
    )
    .with_detail(detail);
    let body = serde_json::to_vec(&problem).unwrap_or_else(|_| b"{}".to_vec());
    Response::builder()
        .status(status)
        .header("content-type", CONTENT_TYPE_PROBLEM_JSON)
        .body(Body::from(body))
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to build error response: {}", e),
            )
        })
}

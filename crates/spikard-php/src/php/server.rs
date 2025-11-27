//! PHP HTTP server implementation.
//!
//! This module provides the `PhpServer` class that can be used from PHP
//! to create and run an HTTP server with Spikard's middleware stack.

use axum::body::Body;
use axum::http::{HeaderName, HeaderValue, Request, Response, StatusCode};
use ext_php_rs::boxed::ZBox;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendHashTable, Zval};
use spikard_http::{Handler, HandlerResult, RequestData, ServerConfig};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use super::zval_to_json;

/// A registered route with its handler.
struct RegisteredRoute {
    method: String,
    path: String,
    handler_name: String,
}

/// PHP-visible HTTP server class.
#[php_class]
#[php(name = "Spikard\\Server")]
pub struct PhpServer {
    routes: Vec<RegisteredRoute>,
    host: String,
    port: u16,
}

#[php_impl]
impl PhpServer {
    /// Create a new server instance.
    pub fn new(host: Option<String>, port: Option<i64>) -> Self {
        Self {
            routes: Vec::new(),
            host: host.unwrap_or_else(|| "127.0.0.1".to_string()),
            port: port.map(|p| p as u16).unwrap_or(8000),
        }
    }

    /// Register a GET route.
    #[php(name = "get")]
    pub fn register_get(&mut self, path: String, handler_name: String) {
        self.routes.push(RegisteredRoute {
            method: "GET".to_string(),
            path,
            handler_name,
        });
    }

    /// Register a POST route.
    #[php(name = "post")]
    pub fn register_post(&mut self, path: String, handler_name: String) {
        self.routes.push(RegisteredRoute {
            method: "POST".to_string(),
            path,
            handler_name,
        });
    }

    /// Register a PUT route.
    #[php(name = "put")]
    pub fn register_put(&mut self, path: String, handler_name: String) {
        self.routes.push(RegisteredRoute {
            method: "PUT".to_string(),
            path,
            handler_name,
        });
    }

    /// Register a PATCH route.
    #[php(name = "patch")]
    pub fn register_patch(&mut self, path: String, handler_name: String) {
        self.routes.push(RegisteredRoute {
            method: "PATCH".to_string(),
            path,
            handler_name,
        });
    }

    /// Register a DELETE route.
    #[php(name = "delete")]
    pub fn register_delete(&mut self, path: String, handler_name: String) {
        self.routes.push(RegisteredRoute {
            method: "DELETE".to_string(),
            path,
            handler_name,
        });
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
}

impl PhpServer {
    /// Get server configuration.
    #[allow(dead_code)]
    pub fn config(&self) -> ServerConfig {
        ServerConfig {
            host: self.host.clone(),
            port: self.port,
            ..Default::default()
        }
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

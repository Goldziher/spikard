//! HTTP server implementation using Tokio and Axum

use crate::handler::RequestData;
use crate::query_parser::parse_query_string_to_json;
use crate::{PythonHandler, Router, ServerConfig};
use axum::Router as AxumRouter;
use axum::body::Body;
use axum::extract::Path;
use axum::routing::{MethodRouter, get};
use http_body_util::BodyExt;
use pyo3::{Py, PyAny};
use serde_json::Value;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Validate Content-Type header and related requirements
fn validate_content_type(
    headers: &axum::http::HeaderMap,
    body_size: usize,
) -> Result<(), (axum::http::StatusCode, String)> {
    // Check Content-Type header if present
    #[allow(clippy::collapsible_if)]
    if let Some(content_type_header) = headers.get(axum::http::header::CONTENT_TYPE) {
        if let Ok(content_type_str) = content_type_header.to_str() {
            // Parse Content-Type to extract media type and parameters
            let parts: Vec<&str> = content_type_str.split(';').map(|s| s.trim()).collect();
            let media_type = parts[0].to_lowercase();

            // Validation 1: multipart/form-data MUST have boundary parameter
            if media_type == "multipart/form-data" {
                let has_boundary = parts.iter().skip(1).any(|part| part.starts_with("boundary="));
                if !has_boundary {
                    let error_body = serde_json::json!({
                        "error": "multipart/form-data requires 'boundary' parameter"
                    });
                    return Err((axum::http::StatusCode::BAD_REQUEST, error_body.to_string()));
                }
            }

            // Validation 2: JSON content type charset must be UTF-8 (or absent)
            if media_type == "application/json" {
                for part in parts.iter().skip(1) {
                    if part.starts_with("charset=") {
                        let charset = part.trim_start_matches("charset=").trim();
                        // Only UTF-8 is allowed (case-insensitive)
                        if !charset.eq_ignore_ascii_case("utf-8") && !charset.eq_ignore_ascii_case("utf8") {
                            let error_body = serde_json::json!({
                                "error": format!("Unsupported charset '{}' for JSON. Only UTF-8 is supported.", charset)
                            });
                            return Err((axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE, error_body.to_string()));
                        }
                    }
                }
            }
        }
    }

    // Validation 3: Content-Length must match actual body size
    #[allow(clippy::collapsible_if)]
    if let Some(content_length_header) = headers.get(axum::http::header::CONTENT_LENGTH) {
        if let Ok(content_length_str) = content_length_header.to_str() {
            if let Ok(declared_length) = content_length_str.parse::<usize>() {
                if declared_length != body_size {
                    let error_body = serde_json::json!({
                        "error": "Content-Length header does not match actual body size"
                    });
                    return Err((axum::http::StatusCode::BAD_REQUEST, error_body.to_string()));
                }
            }
        }
    }

    Ok(())
}

/// Extract and parse query parameters from request URI
fn extract_query_params(uri: &axum::http::Uri) -> Value {
    let query_string = uri.query().unwrap_or("");
    if query_string.is_empty() {
        Value::Object(serde_json::Map::new())
    } else {
        // parse_numbers=true: Auto-convert numeric strings to numbers (e.g., "123" → 123)
        // This is essential for array parameters like ?device_ids=1&device_ids=2 → [1, 2]
        parse_query_string_to_json(query_string.as_bytes(), true)
    }
}

/// Extract headers from request
fn extract_headers(headers: &axum::http::HeaderMap) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for (name, value) in headers.iter() {
        if let Ok(val_str) = value.to_str() {
            // Convert header name to lowercase for consistent access
            map.insert(name.as_str().to_lowercase(), val_str.to_string());
        }
    }
    map
}

/// Extract cookies from request headers
fn extract_cookies(headers: &axum::http::HeaderMap) -> HashMap<String, String> {
    let mut cookies = HashMap::new();

    // Look for Cookie header
    #[allow(clippy::collapsible_if)]
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            // Parse cookies using the cookie crate for RFC 6265 compliance and proper percent-decoding
            for cookie in cookie::Cookie::split_parse(cookie_str).flatten() {
                cookies.insert(cookie.name().to_string(), cookie.value().to_string());
            }
        }
    }

    cookies
}

/// HTTP Server
pub struct Server {
    config: ServerConfig,
    router: Router,
}

impl Server {
    /// Create a new server with configuration
    pub fn new(config: ServerConfig, router: Router) -> Self {
        Self { config, router }
    }

    /// Create a new server with Python handlers
    ///
    /// Routes are grouped by path before registration to support multiple HTTP methods
    /// for the same path (e.g., GET /data and POST /data). Axum requires that all methods
    /// for a path be merged into a single MethodRouter before calling `.route()`.
    pub fn with_python_handlers(
        _config: ServerConfig,
        routes: Vec<(crate::Route, Py<PyAny>)>,
    ) -> Result<AxumRouter, String> {
        let mut app = AxumRouter::new();

        // Group routes by path first to handle multiple methods for the same path.
        // This prevents duplicate route registration errors when the same path has
        // multiple HTTP methods (e.g., GET /api/data and POST /api/data).
        let mut routes_by_path: HashMap<String, Vec<(crate::Route, Py<PyAny>)>> = HashMap::new();
        for (route, handler_py) in routes {
            routes_by_path
                .entry(route.path.clone())
                .or_default()
                .push((route, handler_py));
        }

        // Sort paths alphabetically for consistent route registration order
        let mut sorted_paths: Vec<String> = routes_by_path.keys().cloned().collect();
        sorted_paths.sort();

        // Process each path with all its methods
        for path in sorted_paths {
            let route_handlers = routes_by_path.remove(&path).unwrap();

            // Group by method within this path (last handler wins for duplicates)
            // This handles multiple test fixtures testing the same route+method combination
            let mut handlers_by_method: HashMap<crate::Method, (crate::Route, Py<PyAny>)> = HashMap::new();
            for (route, handler_py) in route_handlers {
                // Last handler wins if multiple fixtures test the same route+method
                let method = route.method.clone();
                handlers_by_method.insert(method, (route, handler_py));
            }

            let mut combined_router: Option<MethodRouter> = None;

            for (_method, (route, handler_py)) in handlers_by_method {
                let handler = PythonHandler::new(
                    handler_py,
                    route.is_async,
                    route.request_validator.clone(),
                    route.response_validator.clone(),
                    route.parameter_validator.clone(),
                );

                // Create Axum route based on HTTP method
                // Extract all request data in Rust before calling Python
                let method_router: MethodRouter = match route.method.as_str() {
                    "GET" => axum::routing::get(
                        move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                            let _ = std::fs::write("/tmp/axum_route_called.log", "GET route handler called\n");
                            let query_params = extract_query_params(req.uri());
                            let headers = extract_headers(req.headers());
                            let cookies = extract_cookies(req.headers());
                            let _ = std::fs::write(
                                "/tmp/axum_query_params.log",
                                format!("query_params: {:?}\n", query_params),
                            );
                            let request_data = RequestData {
                                path_params: path_params.0,
                                query_params,
                                headers,
                                cookies,
                                body: None, // GET requests don't have a body
                            };
                            handler.call(req, request_data).await
                        },
                    ),
                    "POST" => axum::routing::post(
                        move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                            // Extract body for POST requests
                            let (parts, body) = req.into_parts();
                            let body_bytes = body
                                .collect()
                                .await
                                .map_err(|e| {
                                    (
                                        axum::http::StatusCode::BAD_REQUEST,
                                        format!("Failed to read body: {}", e),
                                    )
                                })?
                                .to_bytes();

                            // Validate Content-Type and Content-Length before processing
                            validate_content_type(&parts.headers, body_bytes.len())?;

                            let query_params = extract_query_params(&parts.uri);
                            let headers = extract_headers(&parts.headers);
                            let cookies = extract_cookies(&parts.headers);

                            let body_value = if !body_bytes.is_empty() {
                                serde_json::from_slice::<Value>(&body_bytes)
                                    .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, format!("Invalid JSON: {}", e)))?
                                    .into()
                            } else {
                                None
                            };

                            let request_data = RequestData {
                                path_params: path_params.0,
                                query_params,
                                headers,
                                cookies,
                                body: body_value,
                            };

                            let req = axum::extract::Request::from_parts(parts, Body::empty());
                            handler.call(req, request_data).await
                        },
                    ),
                    "PUT" => axum::routing::put(
                        move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                            let (parts, body) = req.into_parts();
                            let body_bytes = body
                                .collect()
                                .await
                                .map_err(|e| {
                                    (
                                        axum::http::StatusCode::BAD_REQUEST,
                                        format!("Failed to read body: {}", e),
                                    )
                                })?
                                .to_bytes();

                            // Validate Content-Type and Content-Length before processing
                            validate_content_type(&parts.headers, body_bytes.len())?;

                            let query_params = extract_query_params(&parts.uri);
                            let headers = extract_headers(&parts.headers);
                            let cookies = extract_cookies(&parts.headers);

                            let body_value = if !body_bytes.is_empty() {
                                serde_json::from_slice::<Value>(&body_bytes)
                                    .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, format!("Invalid JSON: {}", e)))?
                                    .into()
                            } else {
                                None
                            };

                            let request_data = RequestData {
                                path_params: path_params.0,
                                query_params,
                                headers,
                                cookies,
                                body: body_value,
                            };

                            let req = axum::extract::Request::from_parts(parts, Body::empty());
                            handler.call(req, request_data).await
                        },
                    ),
                    "PATCH" => axum::routing::patch(
                        move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                            let (parts, body) = req.into_parts();
                            let body_bytes = body
                                .collect()
                                .await
                                .map_err(|e| {
                                    (
                                        axum::http::StatusCode::BAD_REQUEST,
                                        format!("Failed to read body: {}", e),
                                    )
                                })?
                                .to_bytes();

                            // Validate Content-Type and Content-Length before processing
                            validate_content_type(&parts.headers, body_bytes.len())?;

                            let query_params = extract_query_params(&parts.uri);
                            let headers = extract_headers(&parts.headers);
                            let cookies = extract_cookies(&parts.headers);

                            let body_value = if !body_bytes.is_empty() {
                                serde_json::from_slice::<Value>(&body_bytes)
                                    .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, format!("Invalid JSON: {}", e)))?
                                    .into()
                            } else {
                                None
                            };

                            let request_data = RequestData {
                                path_params: path_params.0,
                                query_params,
                                headers,
                                cookies,
                                body: body_value,
                            };

                            let req = axum::extract::Request::from_parts(parts, Body::empty());
                            handler.call(req, request_data).await
                        },
                    ),
                    "DELETE" => axum::routing::delete(
                        move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                            let query_params = extract_query_params(req.uri());
                            let headers = extract_headers(req.headers());
                            let cookies = extract_cookies(req.headers());
                            let request_data = RequestData {
                                path_params: path_params.0,
                                query_params,
                                headers,
                                cookies,
                                body: None,
                            };
                            handler.call(req, request_data).await
                        },
                    ),
                    "HEAD" => axum::routing::head(
                        move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                            let query_params = extract_query_params(req.uri());
                            let headers = extract_headers(req.headers());
                            let cookies = extract_cookies(req.headers());
                            let request_data = RequestData {
                                path_params: path_params.0,
                                query_params,
                                headers,
                                cookies,
                                body: None,
                            };
                            handler.call(req, request_data).await
                        },
                    ),
                    "OPTIONS" => axum::routing::options(
                        move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                            let query_params = extract_query_params(req.uri());
                            let headers = extract_headers(req.headers());
                            let cookies = extract_cookies(req.headers());
                            let request_data = RequestData {
                                path_params: path_params.0,
                                query_params,
                                headers,
                                cookies,
                                body: None,
                            };
                            handler.call(req, request_data).await
                        },
                    ),
                    "TRACE" => axum::routing::trace(
                        move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                            let query_params = extract_query_params(req.uri());
                            let headers = extract_headers(req.headers());
                            let cookies = extract_cookies(req.headers());
                            let request_data = RequestData {
                                path_params: path_params.0,
                                query_params,
                                headers,
                                cookies,
                                body: None,
                            };
                            handler.call(req, request_data).await
                        },
                    ),
                    _ => return Err(format!("Unsupported HTTP method: {}", route.method.as_str())),
                };

                // Merge method routers for the same path
                combined_router = Some(match combined_router {
                    None => method_router,
                    Some(existing) => existing.merge(method_router),
                });

                tracing::info!("Registered route: {} {}", route.method.as_str(), path);
            }

            // Register the combined router for this path
            if let Some(router) = combined_router {
                // FastAPI and Axum both use {param} syntax for path parameters
                // No conversion needed - just register the route as-is
                app = app.route(&path, router);
            }
        }

        // Add middleware
        app = app.layer(axum::middleware::from_fn(
            crate::middleware::validate_content_type_middleware,
        ));
        app = app.layer(TraceLayer::new_for_http());

        Ok(app)
    }

    /// Initialize logging
    pub fn init_logging() {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "spikard=debug,tower_http=debug".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
    }

    /// Start the server
    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Starting server with {} routes", self.router.route_count());

        // Build Axum router
        let app = self.build_axum_router();

        // Bind to address
        let addr = format!("{}:{}", self.config.host, self.config.port);
        let socket_addr: SocketAddr = addr.parse()?;
        let listener = TcpListener::bind(socket_addr).await?;

        tracing::info!("Listening on http://{}", socket_addr);

        // Start server
        axum::serve(listener, app).await?;

        Ok(())
    }

    /// Build Axum router from our router
    fn build_axum_router(&self) -> AxumRouter {
        let mut app = AxumRouter::new();

        // Add health check endpoint
        app = app.route("/health", get(|| async { "OK" }));

        // TODO: Add routes from self.router
        // For now, we'll need Python FFI integration to call handlers

        // Add middleware
        app = app.layer(TraceLayer::new_for_http());

        app
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let config = ServerConfig::default();
        let router = Router::new();
        let _server = Server::new(config, router);

        // Test passes if we get here without panic
    }
}

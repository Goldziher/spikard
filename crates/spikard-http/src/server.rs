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
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE)
        && let Ok(cookie_str) = cookie_header.to_str()
    {
        // Parse cookie header: "name1=value1; name2=value2"
        for cookie_pair in cookie_str.split(';') {
            let cookie_pair = cookie_pair.trim();
            if let Some((name, value)) = cookie_pair.split_once('=') {
                cookies.insert(name.trim().to_string(), value.trim().to_string());
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
    pub fn with_python_handlers(
        _config: ServerConfig,
        routes: Vec<(crate::Route, Py<PyAny>)>,
    ) -> Result<AxumRouter, String> {
        let mut app = AxumRouter::new();

        // Add routes with Python handlers
        for (route, handler_py) in routes {
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
                        let query_params = extract_query_params(&parts.uri);
                        let headers = extract_headers(&parts.headers);
                        let cookies = extract_cookies(&parts.headers);
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
                        let query_params = extract_query_params(&parts.uri);
                        let headers = extract_headers(&parts.headers);
                        let cookies = extract_cookies(&parts.headers);
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
                        let query_params = extract_query_params(&parts.uri);
                        let headers = extract_headers(&parts.headers);
                        let cookies = extract_cookies(&parts.headers);
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
                _ => return Err(format!("Unsupported HTTP method: {}", route.method.as_str())),
            };

            // FastAPI and Axum both use {param} syntax for path parameters
            // No conversion needed - just register the route as-is
            app = app.route(&route.path, method_router);

            tracing::info!("Registered route: {} {}", route.method.as_str(), route.path);
        }

        // Add middleware
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

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

/// Extract raw query parameters as strings (no type conversion)
/// Used for validation error messages to show the actual input values
fn extract_raw_query_params(uri: &axum::http::Uri) -> HashMap<String, String> {
    let query_string = uri.query().unwrap_or("");
    if query_string.is_empty() {
        HashMap::new()
    } else {
        // Parse without number conversion to get raw string values
        // For arrays, we just take the first value since validation errors typically show one value
        crate::query_parser::parse_query_string(query_string.as_bytes(), '&')
            .into_iter()
            .fold(HashMap::new(), |mut acc, (k, v)| {
                acc.entry(k).or_insert(v);
                acc
            })
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
    if let Some(cookie_str) = headers.get(axum::http::header::COOKIE).and_then(|h| h.to_str().ok()) {
        // Parse cookies using the cookie crate for RFC 6265 compliance and proper percent-decoding
        for cookie in cookie::Cookie::split_parse(cookie_str).flatten() {
            cookies.insert(cookie.name().to_string(), cookie.value().to_string());
        }
    }

    cookies
}

/// Create RequestData from request parts (for requests without body)
fn create_request_data_without_body(
    uri: &axum::http::Uri,
    headers: &axum::http::HeaderMap,
    path_params: HashMap<String, String>,
) -> RequestData {
    RequestData {
        path_params,
        query_params: extract_query_params(uri),
        raw_query_params: extract_raw_query_params(uri),
        headers: extract_headers(headers),
        cookies: extract_cookies(headers),
        body: None,
    }
}

/// Create RequestData from request parts (for requests with body)
async fn create_request_data_with_body(
    parts: &axum::http::request::Parts,
    path_params: HashMap<String, String>,
    body: Body,
) -> Result<RequestData, (axum::http::StatusCode, String)> {
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

    // Note: Content-Type and Content-Length validation is handled by middleware

    let body_value = if !body_bytes.is_empty() {
        serde_json::from_slice::<Value>(&body_bytes)
            .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, format!("Invalid JSON: {}", e)))?
            .into()
    } else {
        None
    };

    Ok(RequestData {
        path_params,
        query_params: extract_query_params(&parts.uri),
        raw_query_params: extract_raw_query_params(&parts.uri),
        headers: extract_headers(&parts.headers),
        cookies: extract_cookies(&parts.headers),
        body: body_value,
    })
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

                // Check if route has path parameters
                let has_path_params = path.contains('{');

                // Create Axum route based on HTTP method
                // Extract all request data in Rust before calling Python
                // For routes without path parameters, don't try to extract Path
                let method_router: MethodRouter = match route.method.as_str() {
                    // Methods without body: GET, DELETE, HEAD, OPTIONS, TRACE
                    "GET" => {
                        if has_path_params {
                            axum::routing::get(
                                move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                                    let request_data =
                                        create_request_data_without_body(req.uri(), req.headers(), path_params.0);
                                    handler.call(req, request_data).await
                                },
                            )
                        } else {
                            axum::routing::get(move |req: axum::extract::Request| async move {
                                let request_data =
                                    create_request_data_without_body(req.uri(), req.headers(), HashMap::new());
                                handler.call(req, request_data).await
                            })
                        }
                    }
                    "DELETE" => {
                        if has_path_params {
                            axum::routing::delete(
                                move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                                    let request_data =
                                        create_request_data_without_body(req.uri(), req.headers(), path_params.0);
                                    handler.call(req, request_data).await
                                },
                            )
                        } else {
                            axum::routing::delete(move |req: axum::extract::Request| async move {
                                let request_data =
                                    create_request_data_without_body(req.uri(), req.headers(), HashMap::new());
                                handler.call(req, request_data).await
                            })
                        }
                    }
                    "HEAD" => {
                        if has_path_params {
                            axum::routing::head(
                                move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                                    let request_data =
                                        create_request_data_without_body(req.uri(), req.headers(), path_params.0);
                                    handler.call(req, request_data).await
                                },
                            )
                        } else {
                            axum::routing::head(move |req: axum::extract::Request| async move {
                                let request_data =
                                    create_request_data_without_body(req.uri(), req.headers(), HashMap::new());
                                handler.call(req, request_data).await
                            })
                        }
                    }
                    "OPTIONS" => {
                        if has_path_params {
                            axum::routing::options(
                                move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                                    let request_data =
                                        create_request_data_without_body(req.uri(), req.headers(), path_params.0);
                                    handler.call(req, request_data).await
                                },
                            )
                        } else {
                            axum::routing::options(move |req: axum::extract::Request| async move {
                                let request_data =
                                    create_request_data_without_body(req.uri(), req.headers(), HashMap::new());
                                handler.call(req, request_data).await
                            })
                        }
                    }
                    "TRACE" => {
                        if has_path_params {
                            axum::routing::trace(
                                move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                                    let request_data =
                                        create_request_data_without_body(req.uri(), req.headers(), path_params.0);
                                    handler.call(req, request_data).await
                                },
                            )
                        } else {
                            axum::routing::trace(move |req: axum::extract::Request| async move {
                                let request_data =
                                    create_request_data_without_body(req.uri(), req.headers(), HashMap::new());
                                handler.call(req, request_data).await
                            })
                        }
                    }
                    // Methods with body: POST, PUT, PATCH
                    "POST" => {
                        if has_path_params {
                            axum::routing::post(
                                move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                                    let (parts, body) = req.into_parts();
                                    let request_data =
                                        create_request_data_with_body(&parts, path_params.0, body).await?;
                                    let req = axum::extract::Request::from_parts(parts, Body::empty());
                                    handler.call(req, request_data).await
                                },
                            )
                        } else {
                            axum::routing::post(move |req: axum::extract::Request| async move {
                                let (parts, body) = req.into_parts();
                                let request_data = create_request_data_with_body(&parts, HashMap::new(), body).await?;
                                let req = axum::extract::Request::from_parts(parts, Body::empty());
                                handler.call(req, request_data).await
                            })
                        }
                    }
                    "PUT" => {
                        if has_path_params {
                            axum::routing::put(
                                move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                                    let (parts, body) = req.into_parts();
                                    let request_data =
                                        create_request_data_with_body(&parts, path_params.0, body).await?;
                                    let req = axum::extract::Request::from_parts(parts, Body::empty());
                                    handler.call(req, request_data).await
                                },
                            )
                        } else {
                            axum::routing::put(move |req: axum::extract::Request| async move {
                                let (parts, body) = req.into_parts();
                                let request_data = create_request_data_with_body(&parts, HashMap::new(), body).await?;
                                let req = axum::extract::Request::from_parts(parts, Body::empty());
                                handler.call(req, request_data).await
                            })
                        }
                    }
                    "PATCH" => {
                        if has_path_params {
                            axum::routing::patch(
                                move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                                    let (parts, body) = req.into_parts();
                                    let request_data =
                                        create_request_data_with_body(&parts, path_params.0, body).await?;
                                    let req = axum::extract::Request::from_parts(parts, Body::empty());
                                    handler.call(req, request_data).await
                                },
                            )
                        } else {
                            axum::routing::patch(move |req: axum::extract::Request| async move {
                                let (parts, body) = req.into_parts();
                                let request_data = create_request_data_with_body(&parts, HashMap::new(), body).await?;
                                let req = axum::extract::Request::from_parts(parts, Body::empty());
                                handler.call(req, request_data).await
                            })
                        }
                    }
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
                // Strip type hints from path for Axum compatibility
                // /items/{id:uuid} -> /items/{id}
                // /files/{path:path} -> /files/{*path}
                let axum_path = crate::type_hints::strip_type_hints(&path);
                app = app.route(&axum_path, router);
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

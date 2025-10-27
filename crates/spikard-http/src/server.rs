//! HTTP server implementation using Tokio and Axum

use crate::handler::RequestData;
use crate::{PythonHandler, Router, ServerConfig};
use axum::body::Body;
use axum::extract::{Path, Query};
use axum::routing::{get, MethodRouter};
use axum::Router as AxumRouter;
use bytes::Bytes;
use http_body_util::BodyExt;
use pyo3::{Py, PyAny};
use serde_json::Value;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
            );

            // Create Axum route based on HTTP method
            // Extract all request data in Rust before calling Python
            let method_router: MethodRouter = match route.method.as_str() {
                "GET" => axum::routing::get(move |path_params: Path<HashMap<String, String>>, query_params: Query<HashMap<String, String>>, req| async move {
                    let request_data = RequestData {
                        path_params: path_params.0,
                        query_params: query_params.0,
                        body: None, // GET requests don't have a body
                    };
                    handler.call(req, request_data).await
                }),
                "POST" => axum::routing::post(move |path_params: Path<HashMap<String, String>>, query_params: Query<HashMap<String, String>>, req: axum::extract::Request| async move {
                    // Extract body for POST requests
                    let (parts, body) = req.into_parts();
                    let body_bytes = body.collect().await
                        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, format!("Failed to read body: {}", e)))?
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
                        query_params: query_params.0,
                        body: body_value,
                    };

                    let req = axum::extract::Request::from_parts(parts, Body::empty());
                    handler.call(req, request_data).await
                }),
                "PUT" => axum::routing::put(move |path_params: Path<HashMap<String, String>>, query_params: Query<HashMap<String, String>>, req: axum::extract::Request| async move {
                    let (parts, body) = req.into_parts();
                    let body_bytes = body.collect().await
                        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, format!("Failed to read body: {}", e)))?
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
                        query_params: query_params.0,
                        body: body_value,
                    };

                    let req = axum::extract::Request::from_parts(parts, Body::empty());
                    handler.call(req, request_data).await
                }),
                "PATCH" => axum::routing::patch(move |path_params: Path<HashMap<String, String>>, query_params: Query<HashMap<String, String>>, req: axum::extract::Request| async move {
                    let (parts, body) = req.into_parts();
                    let body_bytes = body.collect().await
                        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, format!("Failed to read body: {}", e)))?
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
                        query_params: query_params.0,
                        body: body_value,
                    };

                    let req = axum::extract::Request::from_parts(parts, Body::empty());
                    handler.call(req, request_data).await
                }),
                "DELETE" => axum::routing::delete(move |path_params: Path<HashMap<String, String>>, query_params: Query<HashMap<String, String>>, req| async move {
                    let request_data = RequestData {
                        path_params: path_params.0,
                        query_params: query_params.0,
                        body: None,
                    };
                    handler.call(req, request_data).await
                }),
                _ => {
                    return Err(format!("Unsupported HTTP method: {}", route.method.as_str()))
                }
            };

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
        tracing::info!(
            "Starting server with {} routes",
            self.router.route_count()
        );

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
        let server = Server::new(config, router);

        // Just verify it constructs
        assert!(true);
    }
}

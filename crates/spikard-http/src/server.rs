//! HTTP server implementation using Tokio and Axum

use crate::{PythonHandler, Router, ServerConfig};
use axum::routing::{get, MethodRouter};
use axum::Router as AxumRouter;
use pyo3::{Py, PyAny};
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
        config: ServerConfig,
        routes: Vec<(crate::Route, Py<PyAny>)>,
    ) -> Result<AxumRouter, String> {
        let mut app = AxumRouter::new();

        // Add health check endpoint
        app = app.route("/health", get(|| async { "OK" }));

        // Add routes with Python handlers
        for (route, handler_py) in routes {
            let handler = PythonHandler::new(handler_py, route.is_async);

            // Create Axum route based on HTTP method
            let method_router: MethodRouter = match route.method.as_str() {
                "GET" => axum::routing::get(move |req| async move { handler.call(req).await }),
                "POST" => axum::routing::post(move |req| async move { handler.call(req).await }),
                "PUT" => axum::routing::put(move |req| async move { handler.call(req).await }),
                "PATCH" => axum::routing::patch(move |req| async move { handler.call(req).await }),
                "DELETE" => axum::routing::delete(move |req| async move { handler.call(req).await }),
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

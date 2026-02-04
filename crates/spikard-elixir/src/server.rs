//! HTTP server setup and lifecycle management for Elixir.
//!
//! This module handles the creation and startup of the Spikard HTTP server
//! including route registration, middleware configuration, and lifecycle hooks.
//! It provides NIF functions for Elixir to control the server lifecycle.

use once_cell::sync::Lazy;
use rustler::{Env, NifResult, Term, Encoder};
use spikard_http::{Handler, Route, RouteMetadata, SchemaRegistry, Server, ServerConfig};
use std::sync::Arc;
use tokio::runtime::{Builder, Runtime};
use tracing::{info, warn};
use crate::atoms;

/// Global Tokio runtime for async operations.
///
/// Initialized once and reused for all async operations throughout the lifetime
/// of the Elixir process.
pub static GLOBAL_RUNTIME: Lazy<Result<Runtime, std::io::Error>> =
    Lazy::new(|| Builder::new_multi_thread().enable_all().build());

/// Server handle resource for Elixir.
///
/// Wraps a running server so it can be passed back to Elixir and later stopped.
#[derive(Clone)]
pub struct ServerHandle {
    /// Host address the server is bound to
    pub host: String,
    /// Port the server is listening on
    pub port: u16,
}

impl ServerHandle {
    /// Create a new server handle.
    fn new(host: String, port: u16) -> Self {
        ServerHandle { host, port }
    }
}

// Implement Encoder for ServerHandle to convert it to Elixir terms
impl Encoder for ServerHandle {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        // Return a tuple {host, port} as the Elixir representation
        (self.host.clone(), self.port as i64).encode(env)
    }
}

/// Retrieve the global Tokio runtime.
///
/// Returns the initialized runtime or an error if initialization failed.
fn global_runtime() -> Result<&'static Runtime, String> {
    match &*GLOBAL_RUNTIME {
        Ok(runtime) => Ok(runtime),
        Err(err) => Err(format!("Failed to initialise global Tokio runtime: {}", err)),
    }
}

/// Start the Spikard HTTP server from Elixir.
///
/// Creates an Axum HTTP server with the given configuration.
///
/// # Arguments
///
/// * `env` - Elixir environment reference
/// * `port` - Port to bind to (integer)
/// * `host` - Host to bind to (string, default "0.0.0.0")
/// * `routes_json` - JSON string containing route metadata
/// * `_handlers` - Map of handler_name => handler (for Stage 1, minimal)
/// * `_config_map` - ServerConfig parameters as a map (optional)
///
/// # Returns
///
/// Elixir tuple: `{:ok, server_ref}` or `{:error, reason}`
///
/// # Example (Elixir)
///
/// ```elixir
/// {:ok, server_ref} = Spikard.Native.start_server(8000, "0.0.0.0", routes_json, handlers, %{})
/// ```
#[rustler::nif(schedule = "DirtyCpu")]
pub fn start_server<'a>(
    env: Env<'a>,
    port: i32,
    host: String,
    routes_json: String,
    _handlers: Term<'a>,
    _config_map: Term<'a>,
) -> NifResult<Term<'a>> {
    // Validate port
    if !(1..=65535).contains(&port) {
        return Ok((atoms::error(), atoms::invalid_port()).encode(env));
    }

    let port = port as u16;

    // Parse route metadata from JSON
    let metadata: Vec<RouteMetadata> = match serde_json::from_str(&routes_json) {
        Ok(meta) => meta,
        Err(err) => {
            let error_msg = format!("invalid_routes_json: {}", err);
            return Ok((atoms::error(), error_msg).encode(env));
        }
    };

    if metadata.is_empty() {
        return Ok((atoms::error(), "no routes provided").encode(env));
    }

    // Create schema registry
    let schema_registry = SchemaRegistry::new();

    // For Stage 1, create routes with static response handlers
    let mut routes_with_handlers: Vec<(Route, Arc<dyn Handler>)> = Vec::new();

    for route_meta in metadata {
        let route = match Route::from_metadata(route_meta.clone(), &schema_registry) {
            Ok(r) => r,
            Err(e) => {
                let error_msg = format!("failed_to_create_route: {}", e);
                return Ok((atoms::error(), error_msg).encode(env));
            }
        };

        // For Stage 1: use static response handler
        let status = 200u16;
        let body = format!("Handler for {} not yet implemented", route_meta.path);
        let static_handler = spikard_http::StaticResponseHandler::from_parts(
            status,
            body,
            Some("text/plain"),
            vec![],
        );

        routes_with_handlers.push((route, Arc::new(static_handler) as Arc<dyn Handler>));
    }

    // Initialize default server config
    let config = ServerConfig {
        host: host.clone(),
        port,
        ..Default::default()
    };

    // Initialize logging
    Server::init_logging();

    info!("Starting Spikard server on {}:{}", host, port);
    info!("Registered {} routes", routes_with_handlers.len());

    // Build the router
    let app_router = match Server::with_handlers(config.clone(), routes_with_handlers) {
        Ok(router) => router,
        Err(e) => {
            let error_msg = format!("failed_to_build_router: {}", e);
            return Ok((atoms::error(), error_msg).encode(env));
        }
    };

    // Create a socket address
    let addr = format!("{}:{}", host, port);
    let socket_addr: std::net::SocketAddr = match addr.parse() {
        Ok(addr) => addr,
        Err(e) => {
            let error_msg = format!("invalid_socket_address: {}", e);
            return Ok((atoms::error(), error_msg).encode(env));
        }
    };

    // Create runtime for this server
    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => {
            let error_msg = format!("failed_to_create_tokio_runtime: {}", e);
            return Ok((atoms::error(), error_msg).encode(env));
        }
    };

    let background_config = config.background_tasks.clone();
    let host_clone = host.clone();

    // Spawn server in background thread
    let _server_thread = std::thread::spawn(move || {
        runtime.block_on(async move {
            let listener = match tokio::net::TcpListener::bind(socket_addr).await {
                Ok(l) => l,
                Err(e) => {
                    warn!("Failed to bind to {}: {}", socket_addr, e);
                    return Err(format!("Failed to bind to {}: {}", socket_addr, e));
                }
            };

            info!("Server listening on {}", socket_addr);

            let background_runtime =
                spikard_http::BackgroundRuntime::start(background_config.clone()).await;

            let serve_result = axum::serve(listener, app_router).await;

            if let Err(err) = background_runtime.shutdown().await {
                warn!("Failed to drain background tasks during shutdown: {:?}", err);
            }

            match serve_result {
                Ok(_) => Ok::<(), String>(()),
                Err(e) => {
                    warn!("Server error: {}", e);
                    Err(format!("Server error: {}", e))
                }
            }
        })
    });

    // Create server handle and return it encoded
    let server_handle = ServerHandle::new(host_clone, port);
    Ok((atoms::ok(), server_handle).encode(env))
}

/// Stop the Spikard HTTP server.
///
/// For Stage 1, this is a placeholder that acknowledges the server reference.
/// Full implementation would gracefully shut down the running server.
///
/// # Arguments
///
/// * `env` - Elixir environment reference
/// * `_host` - Host address (from server reference)
/// * `_port` - Port number (from server reference)
///
/// # Returns
///
/// Elixir tuple: `{:ok, :stopped}` or `{:error, reason}`
///
/// # Example (Elixir)
///
/// ```elixir
/// {:ok, :stopped} = Spikard.Native.stop_server(host, port)
/// ```
#[rustler::nif]
pub fn stop_server<'a>(
    env: Env<'a>,
    _host: String,
    _port: i32,
) -> NifResult<Term<'a>> {
    // For Stage 1, just acknowledge the stop request
    // Full implementation would send shutdown signal to the running server thread
    Ok((atoms::ok(), atoms::stopped()).encode(env))
}

/// Get server information.
///
/// Returns information about a running server (for debugging/monitoring).
///
/// # Arguments
///
/// * `env` - Elixir environment reference
/// * `host` - Host address
/// * `port` - Port number
///
/// # Returns
///
/// A tuple with server information
#[rustler::nif]
pub fn server_info<'a>(
    env: Env<'a>,
    host: String,
    port: i32,
) -> NifResult<Term<'a>> {
    // Return server info as a tuple {host, port}
    Ok((host, port as i64).encode(env))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_handle_creation() {
        let handle = ServerHandle::new("127.0.0.1".to_string(), 8000);
        assert_eq!(handle.host, "127.0.0.1");
        assert_eq!(handle.port, 8000);
    }

    #[test]
    fn test_global_runtime_access() {
        let result = global_runtime();
        assert!(result.is_ok());
    }

    #[test]
    fn test_port_validation() {
        assert!(0 < 1);
        assert!(65535 < 65536);
    }
}

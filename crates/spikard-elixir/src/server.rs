//! HTTP server setup and lifecycle management for Elixir.
//!
//! This module handles the creation and startup of the Spikard HTTP server
//! including route registration, middleware configuration, and lifecycle hooks.
//! It provides NIF functions for Elixir to control the server lifecycle.

use crate::atoms;
use crate::error::struct_error;
use once_cell::sync::{Lazy, OnceCell};
use rustler::{Encoder, Env, MapIterator, NifResult, Term};
use spikard_http::{
    CompressionConfig, Handler, RateLimitConfig, Route, RouteMetadata, SchemaRegistry, Server, ServerConfig,
    StaticFilesConfig,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use tokio::runtime::{Builder, Runtime};
use tokio::sync::oneshot;
use tracing::{info, warn};

/// Global flag to track if logging has been initialized.
static LOGGING_INITIALIZED: OnceCell<()> = OnceCell::new();

/// Global Tokio runtime for async operations.
///
/// Initialized once and reused for all async operations throughout the lifetime
/// of the Elixir process.
pub static GLOBAL_RUNTIME: Lazy<Result<Runtime, std::io::Error>> =
    Lazy::new(|| Builder::new_multi_thread().enable_all().build());

/// Running server entry containing shutdown channel and thread handle.
struct RunningServer {
    /// Sender to signal shutdown
    shutdown_tx: oneshot::Sender<()>,
    /// Thread handle for the server
    thread_handle: JoinHandle<Result<(), String>>,
}

/// Global registry of running servers keyed by (host, port).
static SERVER_REGISTRY: Lazy<Mutex<HashMap<(String, u16), RunningServer>>> = Lazy::new(|| Mutex::new(HashMap::new()));

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

/// Extract ServerConfig from an Elixir config map.
///
/// Parses the Elixir map and extracts middleware configuration:
/// - compression: %{gzip: bool, brotli: bool, min_size: int, quality: int}
/// - rate_limit: %{per_second: int, burst: int, ip_based: bool}
/// - static_files: [%{directory: string, route_prefix: string, index_file: bool, cache_control: string}]
/// - enable_request_id: bool
/// - max_body_size: int or nil
/// - request_timeout: int or nil
/// - graceful_shutdown: bool
/// - shutdown_timeout: int
fn extract_server_config(host: String, port: u16, config_term: Term) -> ServerConfig {
    let mut config = ServerConfig {
        host,
        port,
        ..Default::default()
    };

    // Early return if not a map
    let iter = match MapIterator::new(config_term) {
        Some(it) => it,
        None => return config,
    };

    for (key, value) in iter {
        // Decode key - try string first, then atom
        let key_str: String = if let Ok(s) = key.decode::<String>() {
            s
        } else if let Ok(atom) = key.decode::<rustler::Atom>() {
            // Get atom name - atoms decode to their string representation
            format!("{:?}", atom).trim_start_matches(':').to_string()
        } else {
            continue;
        };

        match key_str.as_str() {
            "enable_request_id" => {
                if let Ok(v) = value.decode::<bool>() {
                    config.enable_request_id = v;
                }
            }
            "max_body_size" => {
                if let Ok(v) = value.decode::<i64>() {
                    config.max_body_size = Some(v as usize);
                }
            }
            "request_timeout" => {
                if let Ok(v) = value.decode::<i64>() {
                    config.request_timeout = Some(v as u64);
                }
            }
            "graceful_shutdown" => {
                if let Ok(v) = value.decode::<bool>() {
                    config.graceful_shutdown = v;
                }
            }
            "shutdown_timeout" => {
                if let Ok(v) = value.decode::<i64>() {
                    config.shutdown_timeout = v as u64;
                }
            }
            "workers" => {
                if let Ok(v) = value.decode::<i64>() {
                    config.workers = v as usize;
                }
            }
            "compression" => {
                if let Some(compression) = extract_compression_config(value) {
                    config.compression = Some(compression);
                }
            }
            "rate_limit" => {
                if let Some(rate_limit) = extract_rate_limit_config(value) {
                    config.rate_limit = Some(rate_limit);
                }
            }
            "static_files" => {
                if let Ok(static_files) = extract_static_files_config(value) {
                    config.static_files = static_files;
                }
            }
            _ => {
                // Ignore unknown keys
            }
        }
    }

    config
}

/// Extract test config without requiring host/port inputs.
pub(crate) fn extract_test_config(config_term: Term) -> ServerConfig {
    extract_server_config("127.0.0.1".to_string(), 0, config_term)
}

/// Extract CompressionConfig from an Elixir map.
fn extract_compression_config(term: Term) -> Option<CompressionConfig> {
    let iter = MapIterator::new(term)?;
    let mut config = CompressionConfig::default();
    let mut has_values = false;

    for (key, value) in iter {
        let key_str: String = if let Ok(s) = key.decode::<String>() {
            s
        } else if let Ok(atom) = key.decode::<rustler::Atom>() {
            format!("{:?}", atom).trim_start_matches(':').to_string()
        } else {
            continue;
        };

        match key_str.as_str() {
            "gzip" => {
                if let Ok(v) = value.decode::<bool>() {
                    config.gzip = v;
                    has_values = true;
                }
            }
            "brotli" => {
                if let Ok(v) = value.decode::<bool>() {
                    config.brotli = v;
                    has_values = true;
                }
            }
            "min_size" => {
                if let Ok(v) = value.decode::<i64>() {
                    config.min_size = v as usize;
                    has_values = true;
                }
            }
            "quality" => {
                if let Ok(v) = value.decode::<i64>() {
                    config.quality = v as u32;
                    has_values = true;
                }
            }
            _ => {}
        }
    }

    if has_values { Some(config) } else { None }
}

/// Extract RateLimitConfig from an Elixir map.
fn extract_rate_limit_config(term: Term) -> Option<RateLimitConfig> {
    let iter = MapIterator::new(term)?;
    let mut config = RateLimitConfig::default();
    let mut has_values = false;

    for (key, value) in iter {
        let key_str: String = if let Ok(s) = key.decode::<String>() {
            s
        } else if let Ok(atom) = key.decode::<rustler::Atom>() {
            format!("{:?}", atom).trim_start_matches(':').to_string()
        } else {
            continue;
        };

        match key_str.as_str() {
            "per_second" => {
                if let Ok(v) = value.decode::<i64>() {
                    config.per_second = v as u64;
                    has_values = true;
                }
            }
            "burst" => {
                if let Ok(v) = value.decode::<i64>() {
                    config.burst = v as u32;
                    has_values = true;
                }
            }
            "ip_based" => {
                if let Ok(v) = value.decode::<bool>() {
                    config.ip_based = v;
                    has_values = true;
                }
            }
            _ => {}
        }
    }

    if has_values { Some(config) } else { None }
}

/// Extract StaticFilesConfig array from an Elixir list.
fn extract_static_files_config(term: Term) -> Result<Vec<StaticFilesConfig>, String> {
    let mut configs = Vec::new();

    // Try to decode as a list of maps
    // If the decode fails, it might be nil or an empty list
    if let Ok(list) = term.decode::<Vec<Term>>() {
        for item in list {
            let iter = match MapIterator::new(item) {
                Some(it) => it,
                None => continue,
            };

            let mut directory = String::new();
            let mut route_prefix = String::new();
            let mut index_file = true;
            let mut cache_control = None;

            for (key, value) in iter {
                let key_str: String = if let Ok(s) = key.decode::<String>() {
                    s
                } else if let Ok(atom) = key.decode::<rustler::Atom>() {
                    format!("{:?}", atom).trim_start_matches(':').to_string()
                } else {
                    continue;
                };

                match key_str.as_str() {
                    "directory" => {
                        if let Ok(v) = value.decode::<String>() {
                            directory = v;
                        }
                    }
                    "route_prefix" => {
                        if let Ok(v) = value.decode::<String>() {
                            route_prefix = v;
                        }
                    }
                    "index_file" => {
                        if let Ok(v) = value.decode::<bool>() {
                            index_file = v;
                        }
                    }
                    "cache_control" => {
                        if let Ok(v) = value.decode::<String>() {
                            cache_control = Some(v);
                        }
                    }
                    _ => {}
                }
            }

            if !directory.is_empty() && !route_prefix.is_empty() {
                configs.push(StaticFilesConfig {
                    directory,
                    route_prefix,
                    index_file,
                    cache_control,
                });
            }
        }
    }

    Ok(configs)
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
/// * `handler_runner_pid` - PID of the HandlerRunner GenServer
/// * `config_map` - ServerConfig parameters as a map (optional)
///
/// # Returns
///
/// Elixir tuple: `{:ok, server_ref}` or `{:error, reason}`
///
/// # Example (Elixir)
///
/// ```elixir
/// {:ok, server_ref} = Spikard.Native.start_server(8000, "0.0.0.0", routes_json, handler_runner_pid, %{
///   compression: %{gzip: true, brotli: true},
///   rate_limit: %{per_second: 100, burst: 200}
/// })
/// ```
#[rustler::nif(schedule = "DirtyCpu")]
pub fn start_server<'a>(
    env: Env<'a>,
    port: i32,
    host: String,
    routes_json: String,
    handler_runner_pid: rustler::LocalPid,
    config_map: Term<'a>,
) -> NifResult<Term<'a>> {
    // Validate port
    if !(1..=65535).contains(&port) {
        return Ok(struct_error(
            env,
            atoms::invalid_port(),
            "Port must be between 1 and 65535",
        ));
    }

    let port = port as u16;

    // Parse route metadata from JSON
    let metadata: Vec<RouteMetadata> = match serde_json::from_str(&routes_json) {
        Ok(meta) => meta,
        Err(err) => {
            let error_msg = format!("Failed to parse routes JSON: {}", err);
            return Ok(struct_error(env, atoms::invalid_routes_json(), &error_msg));
        }
    };

    // Extract server config first to check if we have static_files or other config
    let config = extract_server_config(host.clone(), port, config_map);

    // Allow empty routes - middleware (like static_files) can serve content without routes
    // Allow empty routes if metadata is empty and no routes are defined

    // Create schema registry
    let schema_registry = SchemaRegistry::new();

    // Create routes with Elixir handlers
    let mut routes_with_handlers: Vec<(Route, Arc<dyn Handler>)> = Vec::new();

    for route_meta in metadata {
        let route = match Route::from_metadata(route_meta.clone(), &schema_registry) {
            Ok(r) => r,
            Err(e) => {
                let error_msg = format!("Failed to create route: {}", e);
                return Ok(struct_error(env, atoms::route_creation_failed(), &error_msg));
            }
        };

        // Create ElixirHandler with the handler runner PID
        let elixir_handler = match crate::handler::ElixirHandler::new(&route, handler_runner_pid) {
            Ok(h) => h,
            Err(e) => {
                let error_msg = format!("Failed to create handler for {}: {}", route_meta.path, e);
                return Ok(struct_error(env, atoms::route_creation_failed(), &error_msg));
            }
        };

        routes_with_handlers.push((route, Arc::new(elixir_handler) as Arc<dyn Handler>));
    }

    info!("Starting Spikard server on {}:{}", config.host, config.port);
    if config.compression.is_some() {
        info!("Compression enabled");
    }
    if config.rate_limit.is_some() {
        info!("Rate limiting enabled");
    }
    info!("Registered {} routes", routes_with_handlers.len());

    // Build the router
    let app_router = match Server::with_handlers(config.clone(), routes_with_handlers) {
        Ok(router) => router,
        Err(e) => {
            let error_msg = format!("Failed to build router: {}", e);
            return Ok(struct_error(env, atoms::router_build_failed(), &error_msg));
        }
    };

    // Create a socket address
    let addr = format!("{}:{}", host, port);
    let socket_addr: std::net::SocketAddr = match addr.parse() {
        Ok(addr) => addr,
        Err(e) => {
            let error_msg = format!("Invalid socket address '{}': {}", addr, e);
            return Ok(struct_error(env, atoms::invalid_socket_address(), &error_msg));
        }
    };

    // Create runtime for this server
    let runtime = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
        Ok(rt) => rt,
        Err(e) => {
            let error_msg = format!("Failed to create Tokio runtime: {}", e);
            return Ok(struct_error(env, atoms::runtime_error(), &error_msg));
        }
    };

    let background_config = config.background_tasks.clone();
    let host_clone = host.clone();
    let registry_key = (host.clone(), port);

    // Create shutdown channel
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    // Spawn server in background thread
    let server_thread = std::thread::spawn(move || {
        runtime.block_on(async move {
            let listener = match tokio::net::TcpListener::bind(socket_addr).await {
                Ok(l) => l,
                Err(e) => {
                    warn!("Failed to bind to {}: {}", socket_addr, e);
                    return Err(format!("Failed to bind to {}: {}", socket_addr, e));
                }
            };

            info!("Server listening on {}", socket_addr);

            let background_runtime = spikard_http::BackgroundRuntime::start(background_config.clone()).await;

            // Use graceful shutdown with the shutdown receiver
            let serve_result = axum::serve(listener, app_router)
                .with_graceful_shutdown(async {
                    let _ = shutdown_rx.await;
                    info!("Shutdown signal received, stopping server");
                })
                .await;

            if let Err(err) = background_runtime.shutdown().await {
                warn!("Failed to drain background tasks during shutdown: {:?}", err);
            }

            match serve_result {
                Ok(_) => {
                    info!("Server stopped gracefully");
                    Ok::<(), String>(())
                }
                Err(e) => {
                    warn!("Server error: {}", e);
                    Err(format!("Server error: {}", e))
                }
            }
        })
    });

    // Register the running server
    {
        let mut registry = SERVER_REGISTRY
            .lock()
            .map_err(|e| rustler::Error::Term(Box::new(format!("Failed to lock server registry: {}", e))))?;
        registry.insert(
            registry_key,
            RunningServer {
                shutdown_tx,
                thread_handle: server_thread,
            },
        );
    }

    // Create server handle and return it encoded
    let server_handle = ServerHandle::new(host_clone, port);
    Ok((atoms::ok(), server_handle).encode(env))
}

/// Stop the Spikard HTTP server.
///
/// Gracefully shuts down the running server by sending a shutdown signal
/// and waiting for the server thread to complete.
///
/// # Arguments
///
/// * `env` - Elixir environment reference
/// * `host` - Host address (from server reference)
/// * `port` - Port number (from server reference)
///
/// # Returns
///
/// Elixir tuple: `:ok` on success or `{:error, {:reason, message}}` on failure
///
/// # Example (Elixir)
///
/// ```elixir
/// :ok = Spikard.Native.stop_server(host, port)
/// ```
#[rustler::nif(schedule = "DirtyCpu")]
pub fn stop_server<'a>(env: Env<'a>, host: String, port: i32) -> NifResult<Term<'a>> {
    let port = port as u16;
    let registry_key = (host.clone(), port);

    // Remove the server from registry and get its handles
    let running_server = {
        let mut registry = match SERVER_REGISTRY.lock() {
            Ok(r) => r,
            Err(e) => {
                let error_msg = format!("Failed to lock server registry: {}", e);
                return Ok(struct_error(env, atoms::runtime_error(), &error_msg));
            }
        };
        registry.remove(&registry_key)
    };

    match running_server {
        Some(server) => {
            info!("Stopping server on {}:{}", host, port);

            // Send shutdown signal (ignore error if receiver already dropped)
            let _ = server.shutdown_tx.send(());

            // Wait for the server thread to complete with a timeout
            match server.thread_handle.join() {
                Ok(Ok(())) => {
                    info!("Server {}:{} stopped successfully", host, port);
                    Ok(atoms::ok().encode(env))
                }
                Ok(Err(e)) => {
                    let error_msg = format!("Server stopped with error: {}", e);
                    Ok(struct_error(env, atoms::runtime_error(), &error_msg))
                }
                Err(_) => {
                    let error_msg = "Server thread panicked during shutdown";
                    Ok(struct_error(env, atoms::runtime_error(), error_msg))
                }
            }
        }
        None => {
            // Server not found in registry - it may have already been stopped
            // Return :ok for idempotent behavior
            Ok(atoms::ok().encode(env))
        }
    }
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
/// A map with server information including running status
#[rustler::nif]
pub fn server_info<'a>(env: Env<'a>, host: String, port: i32) -> NifResult<Term<'a>> {
    let port_u16 = port as u16;
    let registry_key = (host.clone(), port_u16);

    // Check if server is in registry (running)
    let is_running = {
        let registry = match SERVER_REGISTRY.lock() {
            Ok(r) => r,
            Err(_) => {
                return Ok((
                    atoms::host(),
                    host,
                    atoms::port(),
                    port as i64,
                    atoms::error(),
                    "registry_locked",
                )
                    .encode(env));
            }
        };
        registry.contains_key(&registry_key)
    };

    // Return server info as a map-like tuple
    // {:ok, %{host: host, port: port, running: boolean}}
    let info = (
        (atoms::host(), host),
        (atoms::port(), port as i64),
        (
            rustler::Atom::from_str(env, "running").map_err(|e| rustler::Error::Term(Box::new(format!("{:?}", e))))?,
            is_running,
        ),
    );
    Ok((atoms::ok(), info).encode(env))
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
}

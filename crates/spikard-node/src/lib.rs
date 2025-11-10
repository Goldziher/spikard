//! Node.js bindings for Spikard HTTP framework
//!
//! This crate provides high-performance Node.js bindings using napi-rs.
//!
//! ## Architecture
//!
//! The binding architecture follows a clean separation:
//!
//! - **`run_server`**: Main entry point that starts the Axum HTTP server in a background thread
//! - **`NodeHandler`**: Implements `spikard_http::Handler` trait using ThreadsafeFunction
//! - **`TestClient`**: In-memory testing client that doesn't require a running server
//!
//! ## Thread Safety
//!
//! The implementation uses:
//! - ThreadsafeFunction to safely call JavaScript from Rust async tasks
//! - Dedicated Tokio runtime in a background thread to avoid blocking Node's event loop
//! - Arc-wrapped handlers for safe sharing across Axum routes
//!
//! ## Example
//!
//! ```typescript
//! import { Spikard, get } from '@spikard/node';
//!
//! const app = new Spikard();
//!
//! get('/')(async function root() {
//!   return { message: 'Hello, world!' };
//! });
//!
//! app.run({ port: 8000 });
//! ```

#![deny(clippy::all)]
#![warn(missing_docs)]

mod handler;
mod response;
mod test_client;

use napi::bindgen_prelude::*;
use napi_derive::napi;
use spikard_http::{RouteMetadata, Server, ServerConfig};
use std::sync::Arc;
use tracing::{error, info};

/// Start the Spikard HTTP server from Node.js
///
/// Creates an Axum HTTP server in a dedicated background thread with its own Tokio runtime.
/// This ensures the Node.js event loop remains free to process ThreadsafeFunction calls.
///
/// # Arguments
///
/// * `app` - Application object containing routes and handler functions
/// * `host` - Host address to bind to (default: "127.0.0.1")
/// * `port` - Port number to listen on (default: 8000)
///
/// # Returns
///
/// Returns `Ok(())` after the server thread is spawned. Note that this function
/// returns immediately - the server runs in the background.
///
/// # Errors
///
/// Returns an error if:
/// - Route metadata is invalid or missing required fields
/// - Handler functions cannot be converted to ThreadsafeFunctions
/// - Socket address is invalid
/// - Route creation fails
///
/// # Example
///
/// ```typescript
/// const app = {
///   routes: [{
///     method: 'GET',
///     path: '/',
///     handler_name: 'root',
///     is_async: true
///   }],
///   handlers: {
///     root: async () => ({ message: 'Hello' })
///   }
/// };
///
/// runServer(app, '0.0.0.0', 8000);
/// ```
#[napi]
pub fn run_server(_env: Env, app: Object, host: Option<String>, port: Option<u32>) -> Result<()> {
    let host = host.unwrap_or_else(|| "127.0.0.1".to_string());
    let port = port.unwrap_or(8000) as u16;

    // Extract routes from the app object
    let routes_array: Object = app
        .get_named_property("routes")
        .map_err(|e| Error::from_reason(format!("Failed to get routes from app: {}", e)))?;

    // Convert routes array to Vec<RouteMetadata>
    let routes_length = routes_array.get_array_length()?;
    let mut routes = Vec::new();

    for i in 0..routes_length {
        let route_obj: Object = routes_array.get_element(i)?;

        // Extract route metadata fields
        let method: String = route_obj.get_named_property("method")?;

        let path: String = route_obj.get_named_property("path")?;

        let handler_name: String = route_obj.get_named_property("handler_name")?;

        let is_async: bool = route_obj.get_named_property("is_async")?;

        let route_meta = RouteMetadata {
            method,
            path,
            handler_name,
            request_schema: None, // TODO: Extract from route
            response_schema: None,
            parameter_schema: None,
            file_params: None,
            is_async,
            cors: None,
        };

        routes.push(route_meta);
    }

    // Extract handlers map from app
    let handlers_obj: Object = app
        .get_named_property("handlers")
        .map_err(|e| Error::from_reason(format!("Failed to get handlers from app: {}", e)))?;

    // Build handler map by extracting JS functions and creating ThreadsafeFunctions
    let mut handler_map = std::collections::HashMap::new();

    for route in &routes {
        // Get the JS handler function from the handlers object
        let js_handler: Function<String, Promise<String>> = handlers_obj
            .get_named_property(&route.handler_name)
            .map_err(|e| Error::from_reason(format!("Failed to get handler '{}': {}", route.handler_name, e)))?;

        // Build ThreadsafeFunction following kreuzberg pattern
        let tsfn = js_handler
            .build_threadsafe_function()
            .build_callback(|ctx| Ok(vec![ctx.value]))
            .map_err(|e| {
                Error::from_reason(format!(
                    "Failed to build ThreadsafeFunction for '{}': {}",
                    route.handler_name, e
                ))
            })?;

        // Create NodeHandler with the ThreadsafeFunction
        let node_handler = handler::NodeHandler::new(route.handler_name.clone(), tsfn);
        handler_map.insert(
            route.handler_name.clone(),
            Arc::new(node_handler) as Arc<dyn spikard_http::Handler>,
        );
    }

    // Create server config
    let config = ServerConfig {
        host: host.clone(),
        port,
        ..Default::default()
    };

    // Create schema registry for validator deduplication
    let schema_registry = spikard_http::SchemaRegistry::new();

    // Build routes with compiled validators
    let routes_with_handlers: Vec<(spikard_http::Route, Arc<dyn spikard_http::Handler>)> = routes
        .into_iter()
        .map(|metadata| {
            let route = spikard_http::Route::from_metadata(metadata.clone(), &schema_registry)
                .map_err(|e| Error::from_reason(format!("Failed to create route: {}", e)))?;

            let handler = handler_map
                .get(&metadata.handler_name)
                .ok_or_else(|| Error::from_reason(format!("Handler not found: {}", metadata.handler_name)))?
                .clone();

            Ok::<_, Error>((route, handler))
        })
        .collect::<Result<Vec<_>>>()?;

    // Initialize logging
    Server::init_logging();

    info!("Starting Spikard server on {}:{}", host, port);
    info!("Registered {} routes", routes_with_handlers.len());

    // Build Axum router with handlers
    let app_router = Server::with_handlers(config.clone(), routes_with_handlers)
        .map_err(|e| Error::from_reason(format!("Failed to build router: {}", e)))?;

    // Start the server in a background thread with its own Tokio runtime
    // This keeps the Node.js event loop free to process ThreadsafeFunction calls
    let addr = format!("{}:{}", config.host, config.port);
    let socket_addr: std::net::SocketAddr = addr
        .parse()
        .map_err(|e| Error::from_reason(format!("Invalid socket address {}: {}", addr, e)))?;

    std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

        runtime.block_on(async move {
            let listener = tokio::net::TcpListener::bind(socket_addr)
                .await
                .unwrap_or_else(|_| panic!("Failed to bind to {}", socket_addr));

            info!("Server listening on {}", socket_addr);

            if let Err(e) = axum::serve(listener, app_router).await {
                error!("Server error: {}", e);
            }
        });
    });

    Ok(())
}

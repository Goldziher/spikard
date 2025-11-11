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
mod lifecycle;
mod response;
mod sse;
mod test_client;
mod websocket;

use napi::bindgen_prelude::*;
use napi_derive::napi;
use spikard_http::{RouteMetadata, Server, ServerConfig};
use std::sync::Arc;
use tracing::{error, info};

/// Extract ServerConfig from Node.js Object
///
/// Complete extraction of all middleware configurations following the Python pattern in spikard-py
fn extract_server_config(config: &Object) -> Result<ServerConfig> {
    use spikard_http::{
        ApiKeyConfig, CompressionConfig, ContactInfo, JwtConfig, LicenseInfo, OpenApiConfig, RateLimitConfig,
        ServerInfo, StaticFilesConfig,
    };
    use std::collections::HashMap;

    // Extract host (default: "127.0.0.1")
    let host = config.get::<String>("host")?.unwrap_or_else(|| "127.0.0.1".to_string());

    // Extract port (default: 8000)
    let port = config.get::<u32>("port")?.unwrap_or(8000) as u16;

    // Extract workers (default: 1)
    let workers = config.get::<u32>("workers")?.unwrap_or(1) as usize;

    // Extract enableRequestId (default: true)
    let enable_request_id = config.get::<bool>("enableRequestId")?.unwrap_or(true);

    // Extract optional maxBodySize
    let max_body_size = config.get::<u32>("maxBodySize")?.map(|v| v as usize);

    // Extract optional requestTimeout
    let request_timeout = config.get::<u32>("requestTimeout")?.map(|v| v as u64);

    // Extract gracefulShutdown (default: true)
    let graceful_shutdown = config.get::<bool>("gracefulShutdown")?.unwrap_or(true);

    // Extract shutdownTimeout (default: 30)
    let shutdown_timeout = config.get::<u32>("shutdownTimeout")?.unwrap_or(30) as u64;

    // Extract compression config
    let compression = config.get::<Object>("compression")?.and_then(|comp| {
        let gzip = comp.get::<bool>("gzip").ok()?.unwrap_or(true);
        let brotli = comp.get::<bool>("brotli").ok()?.unwrap_or(true);
        let min_size = comp.get::<u32>("minSize").ok()?.unwrap_or(1024) as usize;
        let quality = comp.get::<u32>("quality").ok()?.unwrap_or(6);

        Some(CompressionConfig {
            gzip,
            brotli,
            min_size,
            quality,
        })
    });

    // Extract rate limit config
    let rate_limit = config.get::<Object>("rateLimit")?.and_then(|rl| {
        let per_second = rl.get::<u32>("perSecond").ok()?? as u64;
        let burst = rl.get::<u32>("burst").ok()??;
        let ip_based = rl.get::<bool>("ipBased").ok()?.unwrap_or(true);
        Some(RateLimitConfig {
            per_second,
            burst,
            ip_based,
        })
    });

    // Extract JWT auth config
    let jwt_auth = config.get::<Object>("jwtAuth")?.and_then(|jwt| {
        let secret = jwt.get::<String>("secret").ok()??;
        let algorithm = jwt
            .get::<String>("algorithm")
            .ok()?
            .unwrap_or_else(|| "HS256".to_string());
        let audience: Option<Vec<String>> = jwt.get::<Vec<String>>("audience").ok()?;
        let issuer: Option<String> = jwt.get::<String>("issuer").ok()?;
        let leeway = jwt.get::<u32>("leeway").ok()?.unwrap_or(0) as u64;
        Some(JwtConfig {
            secret,
            algorithm,
            audience,
            issuer,
            leeway,
        })
    });

    // Extract API key auth config
    let api_key_auth = config.get::<Object>("apiKeyAuth")?.and_then(|api| {
        let keys: Vec<String> = api.get::<Vec<String>>("keys").ok()??;
        let header_name = api
            .get::<String>("headerName")
            .ok()?
            .unwrap_or_else(|| "X-API-Key".to_string());
        Some(ApiKeyConfig { keys, header_name })
    });

    // Extract static files config (array)
    let static_files = config
        .get::<Object>("staticFiles")?
        .and_then(|arr| {
            let length = arr.get_array_length().ok()?;
            let mut configs = Vec::new();
            for i in 0..length {
                let sf: Object = arr.get_element(i).ok()?;
                let directory = sf.get::<String>("directory").ok()??;
                let route_prefix = sf.get::<String>("routePrefix").ok()??;
                let index_file = sf.get::<bool>("indexFile").ok()?.unwrap_or(true);
                let cache_control: Option<String> = sf.get::<String>("cacheControl").ok()?;
                configs.push(StaticFilesConfig {
                    directory,
                    route_prefix,
                    index_file,
                    cache_control,
                });
            }
            Some(configs)
        })
        .unwrap_or_default();

    // Extract OpenAPI config
    let openapi = config.get::<Object>("openapi")?.and_then(|api| {
        let enabled = api.get::<bool>("enabled").ok()?.unwrap_or(false);
        let title = api.get::<String>("title").ok()?.unwrap_or_else(|| "API".to_string());
        let version = api
            .get::<String>("version")
            .ok()?
            .unwrap_or_else(|| "1.0.0".to_string());
        let description: Option<String> = api.get::<String>("description").ok()?;
        let swagger_ui_path = api
            .get::<String>("swaggerUiPath")
            .ok()?
            .unwrap_or_else(|| "/docs".to_string());
        let redoc_path = api
            .get::<String>("redocPath")
            .ok()?
            .unwrap_or_else(|| "/redoc".to_string());
        let openapi_json_path = api
            .get::<String>("openapiJsonPath")
            .ok()?
            .unwrap_or_else(|| "/openapi.json".to_string());

        // Extract contact info
        let contact = api.get::<Object>("contact").ok()?.and_then(|c| {
            let name: Option<String> = c.get::<String>("name").ok()?;
            let email: Option<String> = c.get::<String>("email").ok()?;
            let url: Option<String> = c.get::<String>("url").ok()?;
            Some(ContactInfo { name, email, url })
        });

        // Extract license info
        let license = api.get::<Object>("license").ok()?.and_then(|l| {
            let name = l.get::<String>("name").ok()??;
            let url: Option<String> = l.get::<String>("url").ok()?;
            Some(LicenseInfo { name, url })
        });

        // Extract servers (array)
        let servers = api
            .get::<Object>("servers")
            .ok()?
            .and_then(|arr| {
                let length = arr.get_array_length().ok()?;
                let mut server_list = Vec::new();
                for i in 0..length {
                    let s: Object = arr.get_element(i).ok()?;
                    let url = s.get::<String>("url").ok()??;
                    let description: Option<String> = s.get::<String>("description").ok()?;
                    server_list.push(ServerInfo { url, description });
                }
                Some(server_list)
            })
            .unwrap_or_default();

        // Extract security schemes (Record<string, SecuritySchemeInfo>)
        // For now, return empty map - security schemes will be auto-detected from middleware
        let security_schemes = HashMap::new();

        Some(OpenApiConfig {
            enabled,
            title,
            version,
            description,
            swagger_ui_path,
            redoc_path,
            openapi_json_path,
            contact,
            license,
            servers,
            security_schemes,
        })
    });

    Ok(ServerConfig {
        host,
        port,
        workers,
        enable_request_id,
        max_body_size,
        request_timeout,
        compression,
        rate_limit,
        jwt_auth,
        api_key_auth,
        static_files,
        graceful_shutdown,
        shutdown_timeout,
        openapi,
        lifecycle_hooks: None, // Will be set later in run_server
    })
}

/// Start the Spikard HTTP server from Node.js
///
/// Creates an Axum HTTP server in a dedicated background thread with its own Tokio runtime.
/// This ensures the Node.js event loop remains free to process ThreadsafeFunction calls.
///
/// # Arguments
///
/// * `app` - Application object containing routes and handler functions
/// * `config` - Optional ServerConfig with all middleware settings
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
/// import { Spikard, ServerConfig } from '@spikard/node';
///
/// const config: ServerConfig = {
///   host: '0.0.0.0',
///   port: 8000,
///   compression: { quality: 9 },
///   openapi: {
///     enabled: true,
///     title: 'My API',
///     version: '1.0.0'
///   }
/// };
///
/// const app = new Spikard();
/// app.run(config);
/// ```
#[napi]
pub fn run_server(_env: Env, app: Object, config: Option<Object>) -> Result<()> {
    // Extract config or use defaults
    let server_config = if let Some(cfg) = config {
        extract_server_config(&cfg)?
    } else {
        ServerConfig::default()
    };

    let host = server_config.host.clone();
    let port = server_config.port;

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

    // Extract lifecycle hooks from app if they exist
    let lifecycle_hooks = if let Ok(hooks_obj) = app.get_named_property::<Object>("lifecycleHooks") {
        let mut hooks = spikard_http::LifecycleHooks::new();

        // Helper function to extract and wrap hook functions
        let extract_hooks = |hooks_obj: &Object, hook_type: &str| -> Result<Vec<lifecycle::NodeLifecycleHook>> {
            let hook_array: Result<Object> = hooks_obj.get_named_property(hook_type);
            if let Ok(arr) = hook_array {
                let length = arr.get_array_length()?;
                let mut result = Vec::new();

                for i in 0..length {
                    let js_fn: Function<String, Promise<String>> = arr.get_element(i)?;
                    let name = format!("{}_{}", hook_type, i);

                    // Build ThreadsafeFunction for the hook
                    let tsfn = js_fn
                        .build_threadsafe_function()
                        .build_callback(|ctx| Ok(vec![ctx.value]))
                        .map_err(|e| {
                            Error::from_reason(format!("Failed to build ThreadsafeFunction for hook '{}': {}", name, e))
                        })?;

                    result.push(lifecycle::NodeLifecycleHook::new(name, tsfn));
                }

                Ok(result)
            } else {
                Ok(Vec::new())
            }
        };

        // Extract each hook type
        for hook in extract_hooks(&hooks_obj, "onRequest")? {
            hooks.add_on_request(std::sync::Arc::new(hook));
        }

        for hook in extract_hooks(&hooks_obj, "preValidation")? {
            hooks.add_pre_validation(std::sync::Arc::new(hook));
        }

        for hook in extract_hooks(&hooks_obj, "preHandler")? {
            hooks.add_pre_handler(std::sync::Arc::new(hook));
        }

        for hook in extract_hooks(&hooks_obj, "onResponse")? {
            hooks.add_on_response(std::sync::Arc::new(hook));
        }

        for hook in extract_hooks(&hooks_obj, "onError")? {
            hooks.add_on_error(std::sync::Arc::new(hook));
        }

        Some(hooks)
    } else {
        None
    };

    // Use the extracted config and set lifecycle hooks
    let mut server_config = server_config;
    server_config.lifecycle_hooks = lifecycle_hooks;

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
    let app_router = Server::with_handlers(server_config.clone(), routes_with_handlers)
        .map_err(|e| Error::from_reason(format!("Failed to build router: {}", e)))?;

    // Start the server in a background thread with its own Tokio runtime
    // This keeps the Node.js event loop free to process ThreadsafeFunction calls
    let addr = format!("{}:{}", server_config.host, server_config.port);
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

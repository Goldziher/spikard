//! HTTP server initialization and lifecycle management.
//!
//! This module handles starting the Spikard HTTP server with handlers,
//! middleware configuration, and lifecycle hooks.

use crate::RubyHandler;
use crate::config::server_config::extract_server_config;
use crate::di::build_dependency_container;
use axum::routing::get;
use magnus::prelude::*;
use magnus::{Error, RHash, Ruby, TryConvert, Value, r_hash::ForEach};
use spikard_http::{Handler, Route, RouteMetadata, SchemaRegistry, Server};
use std::sync::Arc;
use tokio::runtime::Runtime;
use tracing::{info, warn};

/// Helper function to run the server startup logic without the GVL.
///
/// This is called via `call_without_gvl!` to release the GVL before blocking on the async runtime.
/// This allows handlers to acquire the GVL during request processing.
async fn start_server_async(
    socket_addr: std::net::SocketAddr,
    app_router: axum::Router,
    background_config: spikard_http::BackgroundTaskConfig,
) -> Result<(), String> {
    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .map_err(|err| format!("Failed to bind to {socket_addr}: {err}"))?;

    info!("Server listening on {}", socket_addr);

    let background_runtime = spikard_http::BackgroundRuntime::start(background_config).await;
    crate::background::install_handle(background_runtime.handle());

    let serve_result = axum::serve(listener, app_router).await;

    crate::background::clear_handle();

    if let Err(err) = background_runtime.shutdown().await {
        warn!("Failed to drain background tasks during shutdown: {:?}", err);
    }

    serve_result.map_err(|e| format!("Server error: {e}"))?;
    Ok::<(), String>(())
}

/// Wrapper function for `call_without_gvl!` to start the server without the GVL.
fn start_server_without_gvl(
    runtime: &Runtime,
    socket_addr: std::net::SocketAddr,
    app_router: axum::Router,
    background_config: spikard_http::BackgroundTaskConfig,
) -> Result<(), String> {
    runtime.block_on(start_server_async(socket_addr, app_router, background_config))
}

/// Start the Spikard HTTP server from Ruby
///
/// Creates an Axum HTTP server in a dedicated background thread with its own Tokio runtime.
///
/// # Arguments
///
/// * `routes_json` - JSON string containing route metadata
/// * `handlers` - Ruby Hash mapping handler_name => Proc
/// * `config` - Ruby ServerConfig object with all middleware settings
/// * `hooks_value` - Lifecycle hooks
/// * `ws_handlers` - WebSocket handlers
/// * `sse_producers` - SSE producers
/// * `dependencies` - Dependency injection container
///
/// # Example (Ruby)
///
/// ```ruby
/// config = Spikard::ServerConfig.new(host: '0.0.0.0', port: 8000)
/// Spikard::Native.run_server(routes_json, handlers, config, hooks, ws, sse, deps)
/// ```
#[allow(clippy::too_many_arguments)]
pub fn run_server(
    ruby: &Ruby,
    routes_json: String,
    handlers: Value,
    config_value: Value,
    hooks_value: Value,
    ws_handlers: Value,
    sse_producers: Value,
    dependencies: Value,
) -> Result<(), Error> {
    let mut config = extract_server_config(ruby, config_value)?;

    let host = config.host.clone();
    let port = config.port;

    let metadata: Vec<RouteMetadata> = serde_json::from_str(&routes_json)
        .map_err(|err| Error::new(ruby.exception_arg_error(), format!("Invalid routes JSON: {}", err)))?;

    let handlers_hash = RHash::from_value(handlers).ok_or_else(|| {
        Error::new(
            ruby.exception_arg_error(),
            "handlers parameter must be a Hash of handler_name => Proc",
        )
    })?;

    let json_module = ruby
        .class_object()
        .funcall::<_, _, Value>("const_get", ("JSON",))
        .map_err(|err| Error::new(ruby.exception_name_error(), format!("JSON module not found: {}", err)))?;

    let schema_registry = SchemaRegistry::new();

    let mut routes_with_handlers: Vec<(Route, Arc<dyn Handler>)> = Vec::new();

    for route_meta in metadata {
        let route = Route::from_metadata(route_meta.clone(), &schema_registry)
            .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("Failed to create route: {}", e)))?;

        let handler_key = ruby.str_new(&route_meta.handler_name);
        let handler_value: Value = match handlers_hash.lookup(handler_key) {
            Ok(val) => val,
            Err(_) => {
                return Err(Error::new(
                    ruby.exception_arg_error(),
                    format!("Handler '{}' not found in handlers hash", route_meta.handler_name),
                ));
            }
        };

        let ruby_handler = RubyHandler::new_for_server(
            ruby,
            handler_value,
            route_meta.handler_name.clone(),
            json_module,
            &route,
        )?;

        routes_with_handlers.push((route, Arc::new(ruby_handler) as Arc<dyn Handler>));
    }

    let lifecycle_hooks = if let Ok(registry) = <&crate::NativeLifecycleRegistry>::try_convert(hooks_value) {
        Some(registry.take_hooks())
    } else if !hooks_value.is_nil() {
        let hooks_hash = RHash::from_value(hooks_value)
            .ok_or_else(|| Error::new(ruby.exception_arg_error(), "lifecycle_hooks parameter must be a Hash"))?;

        let mut hooks = spikard_http::LifecycleHooks::new();
        type RubyHookVec = Vec<
            Arc<
                dyn spikard_http::lifecycle::LifecycleHook<
                        axum::http::Request<axum::body::Body>,
                        axum::http::Response<axum::body::Body>,
                    >,
            >,
        >;

        let extract_hooks = |key: &str| -> Result<RubyHookVec, Error> {
            let key_sym = ruby.to_symbol(key);
            if let Some(hooks_array) = hooks_hash.get(key_sym)
                && !hooks_array.is_nil()
            {
                let array = magnus::RArray::from_value(hooks_array)
                    .ok_or_else(|| Error::new(ruby.exception_type_error(), format!("{} must be an Array", key)))?;

                let mut result = Vec::new();
                let len = array.len();
                for i in 0..len {
                    let hook_value: Value = array.entry(i as isize)?;
                    let name = format!("{}_{}", key, i);
                    let ruby_hook = crate::lifecycle::RubyLifecycleHook::new(name, hook_value);
                    result.push(Arc::new(ruby_hook)
                        as Arc<
                            dyn spikard_http::lifecycle::LifecycleHook<
                                    axum::http::Request<axum::body::Body>,
                                    axum::http::Response<axum::body::Body>,
                                >,
                        >);
                }
                return Ok(result);
            }
            Ok(Vec::new())
        };

        for hook in extract_hooks("on_request")? {
            hooks.add_on_request(hook);
        }

        for hook in extract_hooks("pre_validation")? {
            hooks.add_pre_validation(hook);
        }

        for hook in extract_hooks("pre_handler")? {
            hooks.add_pre_handler(hook);
        }

        for hook in extract_hooks("on_response")? {
            hooks.add_on_response(hook);
        }

        for hook in extract_hooks("on_error")? {
            hooks.add_on_error(hook);
        }

        Some(hooks)
    } else {
        None
    };

    config.lifecycle_hooks = lifecycle_hooks.map(Arc::new);

    #[cfg(feature = "di")]
    {
        if let Ok(registry) = <&crate::NativeDependencyRegistry>::try_convert(dependencies) {
            config.di_container = Some(Arc::new(registry.take_container()?));
        } else if !dependencies.is_nil() {
            match build_dependency_container(ruby, dependencies) {
                Ok(container) => {
                    config.di_container = Some(Arc::new(container));
                }
                Err(err) => {
                    return Err(Error::new(
                        ruby.exception_runtime_error(),
                        format!("Failed to build DI container: {}", err),
                    ));
                }
            }
        }
    }

    Server::init_logging();

    info!("Starting Spikard server on {}:{}", host, port);
    info!("Registered {} routes", routes_with_handlers.len());

    let mut app_router = Server::with_handlers(config.clone(), routes_with_handlers)
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("Failed to build router: {}", e)))?;

    let mut ws_endpoints = Vec::new();
    if !ws_handlers.is_nil() {
        let ws_hash = RHash::from_value(ws_handlers)
            .ok_or_else(|| Error::new(ruby.exception_arg_error(), "WebSocket handlers must be a Hash"))?;

        ws_hash.foreach(|path: String, factory: Value| -> Result<ForEach, Error> {
            if let Some(ws_state) = crate::websocket::create_websocket_state(ruby, factory)? {
                ws_endpoints.push((path, ws_state));
            }

            Ok(ForEach::Continue)
        })?;
    }

    let mut sse_endpoints = Vec::new();
    if !sse_producers.is_nil() {
        let sse_hash = RHash::from_value(sse_producers)
            .ok_or_else(|| Error::new(ruby.exception_arg_error(), "SSE producers must be a Hash"))?;

        sse_hash.foreach(|path: String, factory: Value| -> Result<ForEach, Error> {
            let producer_instance = factory.funcall::<_, _, Value>("call", ()).map_err(|e| {
                Error::new(
                    ruby.exception_runtime_error(),
                    format!("Failed to create SSE producer: {}", e),
                )
            })?;

            let sse_state = crate::sse::create_sse_state(ruby, producer_instance)?;

            sse_endpoints.push((path, sse_state));

            Ok(ForEach::Continue)
        })?;
    }

    for (path, ws_state) in ws_endpoints {
        info!("Registered WebSocket endpoint: {}", path);
        app_router = app_router.route(
            &path,
            get(spikard_http::websocket_handler::<crate::websocket::RubyWebSocketHandler>).with_state(ws_state),
        );
    }

    for (path, sse_state) in sse_endpoints {
        info!("Registered SSE endpoint: {}", path);
        app_router = app_router.route(
            &path,
            get(spikard_http::sse_handler::<crate::sse::RubySseEventProducer>).with_state(sse_state),
        );
    }

    let addr = format!("{}:{}", config.host, config.port);
    let socket_addr: std::net::SocketAddr = addr.parse().map_err(|e| {
        Error::new(
            ruby.exception_arg_error(),
            format!("Invalid socket address {}: {}", addr, e),
        )
    })?;

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("Failed to create Tokio runtime: {}", e),
            )
        })?;

    let background_config = config.background_tasks.clone();
    let runtime_ref = &runtime;

    // Release the GVL before blocking on the async runtime to allow handlers to acquire it during request processing
    let result = crate::call_without_gvl!(
        start_server_without_gvl,
        args: (runtime_ref, &Runtime, socket_addr, std::net::SocketAddr, app_router, axum::Router, background_config, spikard_http::BackgroundTaskConfig),
        return_type: Result<(), String>
    );

    result.map_err(|msg| Error::new(ruby.exception_runtime_error(), msg))?;

    Ok(())
}

/// Validate and normalize route metadata using the Rust RouteMetadata schema.
///
/// Parses the provided JSON, compiles schemas/parameter validators to ensure
/// correctness, and returns a canonical JSON string. This keeps Ruby-sourced
/// metadata aligned with the Rust core types.
pub fn normalize_route_metadata(_ruby: &Ruby, routes_json: String) -> Result<String, Error> {
    let registry = SchemaRegistry::new();
    let routes: Vec<RouteMetadata> = serde_json::from_str(&routes_json)
        .map_err(|err| Error::new(magnus::exception::arg_error(), format!("Invalid routes JSON: {err}")))?;

    for route in &routes {
        Route::from_metadata(route.clone(), &registry).map_err(|err| {
            Error::new(
                magnus::exception::runtime_error(),
                format!("Invalid route {} {}: {err}", route.method, route.path),
            )
        })?;
    }

    serde_json::to_string(&routes).map_err(|err| {
        Error::new(
            magnus::exception::runtime_error(),
            format!("Failed to serialise routes: {err}"),
        )
    })
}

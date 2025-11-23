#![allow(deprecated)]

//! Spikard Ruby bindings using Magnus FFI.
//!
//! This crate provides Ruby bindings for the Spikard HTTP toolkit, allowing
//! Ruby developers to build and test HTTP services with Rust performance.
//!
//! ## Modules
//!
//! - `test_client`: TestClient wrapper for integration testing
//! - `handler`: RubyHandler trait implementation
//! - `di`: Dependency injection bridge for Ruby types
//! - `config`: ServerConfig extraction from Ruby objects
//! - `conversion`: Ruby ↔ Rust type conversions
//! - `server`: HTTP server setup and lifecycle management
//! - `background`: Background task management
//! - `lifecycle`: Lifecycle hook implementations
//! - `sse`: Server-Sent Events support
//! - `test_sse`: SSE testing utilities
//! - `websocket`: WebSocket support
//! - `test_websocket`: WebSocket testing utilities

mod background;
mod config;
mod conversion;
mod di;
mod handler;
mod lifecycle;
mod server;
mod sse;
mod test_client;
mod test_sse;
mod test_websocket;
mod websocket;

use async_stream::stream;
use axum::body::Body;
use axum::http::{HeaderName, HeaderValue, Method, Request, Response, StatusCode};
use axum_test::{TestServer, TestServerConfig, Transport};
use bytes::Bytes;
use cookie::Cookie;
use magnus::prelude::*;
use magnus::value::{InnerValue, Opaque};
use magnus::{
    Error, Module, RArray, RHash, RString, Ruby, TryConvert, Value, function, gc::Marker, method, r_hash::ForEach,
};
use once_cell::sync::Lazy;
use serde_json::{Map as JsonMap, Value as JsonValue};
use spikard_http::ParameterValidator;
use spikard_http::problem::ProblemDetails;
use spikard_http::testing::{
    MultipartFilePart, SnapshotError, build_multipart_body, encode_urlencoded_body, snapshot_response,
};
use spikard_http::{Handler, HandlerResponse, HandlerResult, RequestData};
use spikard_http::{Route, RouteMetadata, SchemaValidator};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::pin::Pin;
use std::sync::Arc;
use tokio::runtime::{Builder, Runtime};

// Re-export for internal use and public API
pub use handler::RubyHandler;
pub use server::run_server;
pub use test_client::NativeTestClient;

/// Return the Spikard version.
fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Build dependency container from Ruby dependencies
///
/// Converts Ruby dependencies (values and factories) to Rust DependencyContainer
#[cfg(feature = "di")]
fn build_dependency_container(
    ruby: &Ruby,
    dependencies: Value,
) -> Result<spikard_core::di::DependencyContainer, Error> {
    use spikard_core::di::DependencyContainer;
    use std::sync::Arc;

    if dependencies.is_nil() {
        return Ok(DependencyContainer::new());
    }

    let mut container = DependencyContainer::new();
    let deps_hash = RHash::try_convert(dependencies)?;

    deps_hash.foreach(|key: String, value: Value| -> Result<ForEach, Error> {
        // Check if this is a factory (has a 'type' field set to :factory)
        if let Ok(dep_hash) = RHash::try_convert(value) {
            let dep_type: Option<String> = get_kw(ruby, dep_hash, "type").and_then(|v| String::try_convert(v).ok());

            match dep_type.as_deref() {
                Some("factory") => {
                    // Factory dependency
                    let factory = get_kw(ruby, dep_hash, "factory")
                        .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "Factory missing 'factory' key"))?;

                    let depends_on: Vec<String> = get_kw(ruby, dep_hash, "depends_on")
                        .and_then(|v| Vec::<String>::try_convert(v).ok())
                        .unwrap_or_default();

                    let singleton: bool = get_kw(ruby, dep_hash, "singleton")
                        .and_then(|v| bool::try_convert(v).ok())
                        .unwrap_or(false);

                    let cacheable: bool = get_kw(ruby, dep_hash, "cacheable")
                        .and_then(|v| bool::try_convert(v).ok())
                        .unwrap_or(true);

                    let factory_dep =
                        crate::di::RubyFactoryDependency::new(key.clone(), factory, depends_on, singleton, cacheable);

                    container.register(key.clone(), Arc::new(factory_dep)).map_err(|e| {
                        Error::new(
                            ruby.exception_runtime_error(),
                            format!("Failed to register factory '{}': {}", key, e),
                        )
                    })?;
                }
                Some("value") => {
                    // Value dependency
                    let value_data = get_kw(ruby, dep_hash, "value").ok_or_else(|| {
                        Error::new(ruby.exception_runtime_error(), "Value dependency missing 'value' key")
                    })?;

                    let value_dep = crate::di::RubyValueDependency::new(key.clone(), value_data);

                    container.register(key.clone(), Arc::new(value_dep)).map_err(|e| {
                        Error::new(
                            ruby.exception_runtime_error(),
                            format!("Failed to register value '{}': {}", key, e),
                        )
                    })?;
                }
                _ => {
                    return Err(Error::new(
                        ruby.exception_runtime_error(),
                        format!("Invalid dependency type for '{}'", key),
                    ));
                }
            }
        } else {
            // Treat as raw value
            let value_dep = crate::di::RubyValueDependency::new(key.clone(), value);
            container.register(key.clone(), Arc::new(value_dep)).map_err(|e| {
                Error::new(
                    ruby.exception_runtime_error(),
                    format!("Failed to register value '{}': {}", key, e),
                )
            })?;
        }

        Ok(ForEach::Continue)
    })?;

    Ok(container)
}

/// Helper to extract an optional string from a Ruby Hash
fn get_optional_string_from_hash(hash: RHash, key: &str) -> Result<Option<String>, Error> {
    match hash.get(String::from(key)) {
        Some(v) if !v.is_nil() => Ok(Some(String::try_convert(v)?)),
        _ => Ok(None),
    }
}

/// Helper to extract a required string from a Ruby Hash
fn get_required_string_from_hash(hash: RHash, key: &str, ruby: &Ruby) -> Result<String, Error> {
    let value = hash
        .get(String::from(key))
        .ok_or_else(|| Error::new(ruby.exception_arg_error(), format!("missing required key '{}'", key)))?;
    if value.is_nil() {
        return Err(Error::new(
            ruby.exception_arg_error(),
            format!("key '{}' cannot be nil", key),
        ));
    }
    String::try_convert(value)
}

fn extract_files(ruby: &Ruby, files_value: Value) -> Result<Vec<MultipartFilePart>, Error> {
    let files_hash = RHash::try_convert(files_value)?;

    let keys_array: RArray = files_hash.funcall("keys", ())?;
    let mut result = Vec::new();

    for i in 0..keys_array.len() {
        let key_val = keys_array.entry::<Value>(i as isize)?;
        let field_name = String::try_convert(key_val)?;
        let value = files_hash
            .get(key_val)
            .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "Failed to get hash value"))?;

        if let Some(outer_array) = RArray::from_value(value) {
            if outer_array.is_empty() {
                continue;
            }

            let first_elem = outer_array.entry::<Value>(0)?;

            if RArray::from_value(first_elem).is_some() {
                for j in 0..outer_array.len() {
                    let file_array = outer_array.entry::<Value>(j as isize)?;
                    let file_data = extract_single_file(ruby, &field_name, file_array)?;
                    result.push(file_data);
                }
            } else {
                let file_data = extract_single_file(ruby, &field_name, value)?;
                result.push(file_data);
            }
        }
    }

    Ok(result)
}

/// Extract a single file from Ruby array [filename, content, content_type (optional)]
fn extract_single_file(ruby: &Ruby, field_name: &str, array_value: Value) -> Result<MultipartFilePart, Error> {
    let array = RArray::from_value(array_value)
        .ok_or_else(|| Error::new(ruby.exception_arg_error(), "file must be an Array [filename, content]"))?;

    if array.len() < 2 {
        return Err(Error::new(
            ruby.exception_arg_error(),
            "file Array must have at least 2 elements: [filename, content]",
        ));
    }

    let filename: String = String::try_convert(array.shift()?)?;
    let content_str: String = String::try_convert(array.shift()?)?;
    let content = content_str.into_bytes();

    let content_type: Option<String> = if !array.is_empty() {
        String::try_convert(array.shift()?).ok()
    } else {
        None
    };

    Ok(MultipartFilePart {
        field_name: field_name.to_string(),
        filename,
        content,
        content_type,
    })
}

/// Extract ServerConfig from Ruby ServerConfig object
fn extract_server_config(ruby: &Ruby, config_value: Value) -> Result<spikard_http::ServerConfig, Error> {
    use spikard_http::{
        ApiKeyConfig, CompressionConfig, ContactInfo, JwtConfig, LicenseInfo, OpenApiConfig, RateLimitConfig,
        ServerInfo, StaticFilesConfig,
    };
    use std::collections::HashMap;

    let host: String = config_value.funcall("host", ())?;

    let port: u32 = config_value.funcall("port", ())?;

    let workers: usize = config_value.funcall("workers", ())?;

    let enable_request_id: bool = config_value.funcall("enable_request_id", ())?;

    let max_body_size_value: Value = config_value.funcall("max_body_size", ())?;
    let max_body_size = if max_body_size_value.is_nil() {
        None
    } else {
        Some(u64::try_convert(max_body_size_value)? as usize)
    };

    let request_timeout_value: Value = config_value.funcall("request_timeout", ())?;
    let request_timeout = if request_timeout_value.is_nil() {
        None
    } else {
        Some(u64::try_convert(request_timeout_value)?)
    };

    let graceful_shutdown: bool = config_value.funcall("graceful_shutdown", ())?;

    let shutdown_timeout: u64 = config_value.funcall("shutdown_timeout", ())?;

    let compression_value: Value = config_value.funcall("compression", ())?;
    let compression = if compression_value.is_nil() {
        None
    } else {
        let gzip: bool = compression_value.funcall("gzip", ())?;
        let brotli: bool = compression_value.funcall("brotli", ())?;
        let min_size: usize = compression_value.funcall("min_size", ())?;
        let quality: u32 = compression_value.funcall("quality", ())?;
        Some(CompressionConfig {
            gzip,
            brotli,
            min_size,
            quality,
        })
    };

    let rate_limit_value: Value = config_value.funcall("rate_limit", ())?;
    let rate_limit = if rate_limit_value.is_nil() {
        None
    } else {
        let per_second: u64 = rate_limit_value.funcall("per_second", ())?;
        let burst: u32 = rate_limit_value.funcall("burst", ())?;
        let ip_based: bool = rate_limit_value.funcall("ip_based", ())?;
        Some(RateLimitConfig {
            per_second,
            burst,
            ip_based,
        })
    };

    let jwt_auth_value: Value = config_value.funcall("jwt_auth", ())?;
    let jwt_auth = if jwt_auth_value.is_nil() {
        None
    } else {
        let secret: String = jwt_auth_value.funcall("secret", ())?;
        let algorithm: String = jwt_auth_value.funcall("algorithm", ())?;
        let audience_value: Value = jwt_auth_value.funcall("audience", ())?;
        let audience = if audience_value.is_nil() {
            None
        } else {
            Some(Vec::<String>::try_convert(audience_value)?)
        };
        let issuer_value: Value = jwt_auth_value.funcall("issuer", ())?;
        let issuer = if issuer_value.is_nil() {
            None
        } else {
            Some(String::try_convert(issuer_value)?)
        };
        let leeway: u64 = jwt_auth_value.funcall("leeway", ())?;
        Some(JwtConfig {
            secret,
            algorithm,
            audience,
            issuer,
            leeway,
        })
    };

    let api_key_auth_value: Value = config_value.funcall("api_key_auth", ())?;
    let api_key_auth = if api_key_auth_value.is_nil() {
        None
    } else {
        let keys: Vec<String> = api_key_auth_value.funcall("keys", ())?;
        let header_name: String = api_key_auth_value.funcall("header_name", ())?;
        Some(ApiKeyConfig { keys, header_name })
    };

    let static_files_value: Value = config_value.funcall("static_files", ())?;
    let static_files_array = RArray::from_value(static_files_value)
        .ok_or_else(|| Error::new(ruby.exception_type_error(), "static_files must be an Array"))?;

    let mut static_files = Vec::new();
    for i in 0..static_files_array.len() {
        let sf_value = static_files_array.entry::<Value>(i as isize)?;
        let directory: String = sf_value.funcall("directory", ())?;
        let route_prefix: String = sf_value.funcall("route_prefix", ())?;
        let index_file: bool = sf_value.funcall("index_file", ())?;
        let cache_control_value: Value = sf_value.funcall("cache_control", ())?;
        let cache_control = if cache_control_value.is_nil() {
            None
        } else {
            Some(String::try_convert(cache_control_value)?)
        };
        static_files.push(StaticFilesConfig {
            directory,
            route_prefix,
            index_file,
            cache_control,
        });
    }

    let openapi_value: Value = config_value.funcall("openapi", ())?;
    let openapi = if openapi_value.is_nil() {
        None
    } else {
        let enabled: bool = openapi_value.funcall("enabled", ())?;
        let title: String = openapi_value.funcall("title", ())?;
        let version: String = openapi_value.funcall("version", ())?;
        let description_value: Value = openapi_value.funcall("description", ())?;
        let description = if description_value.is_nil() {
            None
        } else {
            Some(String::try_convert(description_value)?)
        };
        let swagger_ui_path: String = openapi_value.funcall("swagger_ui_path", ())?;
        let redoc_path: String = openapi_value.funcall("redoc_path", ())?;
        let openapi_json_path: String = openapi_value.funcall("openapi_json_path", ())?;

        let contact_value: Value = openapi_value.funcall("contact", ())?;
        let contact = if contact_value.is_nil() {
            None
        } else if let Some(contact_hash) = RHash::from_value(contact_value) {
            let name = get_optional_string_from_hash(contact_hash, "name")?;
            let email = get_optional_string_from_hash(contact_hash, "email")?;
            let url = get_optional_string_from_hash(contact_hash, "url")?;
            Some(ContactInfo { name, email, url })
        } else {
            let name_value: Value = contact_value.funcall("name", ())?;
            let email_value: Value = contact_value.funcall("email", ())?;
            let url_value: Value = contact_value.funcall("url", ())?;
            Some(ContactInfo {
                name: if name_value.is_nil() {
                    None
                } else {
                    Some(String::try_convert(name_value)?)
                },
                email: if email_value.is_nil() {
                    None
                } else {
                    Some(String::try_convert(email_value)?)
                },
                url: if url_value.is_nil() {
                    None
                } else {
                    Some(String::try_convert(url_value)?)
                },
            })
        };

        let license_value: Value = openapi_value.funcall("license", ())?;
        let license = if license_value.is_nil() {
            None
        } else if let Some(license_hash) = RHash::from_value(license_value) {
            let name = get_required_string_from_hash(license_hash, "name", ruby)?;
            let url = get_optional_string_from_hash(license_hash, "url")?;
            Some(LicenseInfo { name, url })
        } else {
            let name: String = license_value.funcall("name", ())?;
            let url_value: Value = license_value.funcall("url", ())?;
            let url = if url_value.is_nil() {
                None
            } else {
                Some(String::try_convert(url_value)?)
            };
            Some(LicenseInfo { name, url })
        };

        let servers_value: Value = openapi_value.funcall("servers", ())?;
        let servers_array = RArray::from_value(servers_value)
            .ok_or_else(|| Error::new(ruby.exception_type_error(), "servers must be an Array"))?;

        let mut servers = Vec::new();
        for i in 0..servers_array.len() {
            let server_value = servers_array.entry::<Value>(i as isize)?;

            let (url, description) = if let Some(server_hash) = RHash::from_value(server_value) {
                let url = get_required_string_from_hash(server_hash, "url", ruby)?;
                let description = get_optional_string_from_hash(server_hash, "description")?;
                (url, description)
            } else {
                let url: String = server_value.funcall("url", ())?;
                let description_value: Value = server_value.funcall("description", ())?;
                let description = if description_value.is_nil() {
                    None
                } else {
                    Some(String::try_convert(description_value)?)
                };
                (url, description)
            };

            servers.push(ServerInfo { url, description });
        }

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
    };

    Ok(spikard_http::ServerConfig {
        host,
        port: port as u16,
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
        background_tasks: spikard_http::BackgroundTaskConfig::default(),
        openapi,
        lifecycle_hooks: None,
        di_container: None,
    })
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
fn run_server(
    ruby: &Ruby,
    routes_json: String,
    handlers: Value,
    config_value: Value,
    hooks_value: Value,
    ws_handlers: Value,
    sse_producers: Value,
    dependencies: Value,
) -> Result<(), Error> {
    use spikard_http::{SchemaRegistry, Server};
    use tracing::{error, info, warn};

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

    let mut routes_with_handlers: Vec<(Route, Arc<dyn spikard_http::Handler>)> = Vec::new();

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
            route_meta.method.clone(),
            route_meta.path.clone(),
            json_module,
            &route,
        )?;

        routes_with_handlers.push((route, Arc::new(ruby_handler) as Arc<dyn spikard_http::Handler>));
    }

    let lifecycle_hooks = if !hooks_value.is_nil() {
        let hooks_hash = RHash::from_value(hooks_value)
            .ok_or_else(|| Error::new(ruby.exception_arg_error(), "lifecycle_hooks parameter must be a Hash"))?;

        let mut hooks = spikard_http::LifecycleHooks::new();
        type RubyHookVec = Vec<Arc<dyn spikard_http::lifecycle::LifecycleHook<Request<Body>, Response<Body>>>>;

        let extract_hooks = |key: &str| -> Result<RubyHookVec, Error> {
            let key_sym = ruby.to_symbol(key);
            if let Some(hooks_array) = hooks_hash.get(key_sym)
                && !hooks_array.is_nil()
            {
                let array = RArray::from_value(hooks_array)
                    .ok_or_else(|| Error::new(ruby.exception_type_error(), format!("{} must be an Array", key)))?;

                let mut result = Vec::new();
                let len = array.len();
                for i in 0..len {
                    let hook_value: Value = array.entry(i as isize)?;
                    let name = format!("{}_{}", key, i);
                    let ruby_hook = lifecycle::RubyLifecycleHook::new(name, hook_value);
                    result.push(Arc::new(ruby_hook)
                        as Arc<
                            dyn spikard_http::lifecycle::LifecycleHook<Request<Body>, Response<Body>>,
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

    // Extract and register dependencies
    #[cfg(feature = "di")]
    {
        if !dependencies.is_nil() {
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
            let handler_instance = factory.funcall::<_, _, Value>("call", ()).map_err(|e| {
                Error::new(
                    ruby.exception_runtime_error(),
                    format!("Failed to create WebSocket handler: {}", e),
                )
            })?;

            let ws_state = crate::websocket::create_websocket_state(ruby, handler_instance)?;

            ws_endpoints.push((path, ws_state));

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

    use axum::routing::get;
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

    runtime.block_on(async move {
        let listener = tokio::net::TcpListener::bind(socket_addr)
            .await
            .unwrap_or_else(|_| panic!("Failed to bind to {}", socket_addr));

        info!("Server listening on {}", socket_addr);

        let background_runtime = spikard_http::BackgroundRuntime::start(background_config.clone()).await;
        crate::background::install_handle(background_runtime.handle());

        let serve_result = axum::serve(listener, app_router).await;

        crate::background::clear_handle();

        if let Err(err) = background_runtime.shutdown().await {
            warn!("Failed to drain background tasks during shutdown: {:?}", err);
        }

        if let Err(e) = serve_result {
            error!("Server error: {}", e);
        }
    });

    Ok(())
}

>>>>>>> 49b4c69c (feat(di): implement handler parameter extraction for Python/Node/Ruby bindings)
#[magnus::init]
pub fn init(ruby: &Ruby) -> Result<(), Error> {
    let spikard = ruby.define_module("Spikard")?;
    spikard.define_singleton_method("version", function!(version, 0))?;
    let native = match spikard.const_get("Native") {
        Ok(module) => module,
        Err(_) => spikard.define_module("Native")?,
    };

    native.define_singleton_method("run_server", function!(run_server, 7))?;
    native.define_singleton_method("background_run", function!(background::background_run, 1))?;

    let class = native.define_class("TestClient", ruby.class_object())?;
    class.define_alloc_func::<NativeTestClient>();
    class.define_method("initialize", method!(NativeTestClient::initialize, 5))?;
    class.define_method("request", method!(NativeTestClient::request, 3))?;
    class.define_method("websocket", method!(NativeTestClient::websocket, 1))?;
    class.define_method("sse", method!(NativeTestClient::sse, 1))?;
    class.define_method("close", method!(NativeTestClient::close, 0))?;

    let spikard_module = ruby.define_module("Spikard")?;
    test_websocket::init(ruby, &spikard_module)?;
    test_sse::init(ruby, &spikard_module)?;

    Ok(())
}

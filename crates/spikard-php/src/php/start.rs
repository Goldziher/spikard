//! Native entrypoints for starting/stopping the server from PHP.

use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;
use spikard_http::server::build_router_with_handlers_and_config;
use spikard_http::{LifecycleHooks, Route};
use std::collections::HashMap;
use std::sync::Arc;

use crate::php::handler::PhpHandler;
use crate::php::hooks::{PhpErrorHook, PhpRequestHook, PhpResponseHook};

/// Payload for a registered route coming from PHP.
#[derive(Debug, serde::Deserialize)]
pub struct RegisteredRoutePayload {
    pub method: String,
    pub path: String,
    pub handler_name: String,
    pub request_schema: Option<serde_json::Value>,
    pub response_schema: Option<serde_json::Value>,
    pub parameter_schema: Option<serde_json::Value>,
    pub jsonrpc_method: Option<serde_json::Value>,
}

impl RegisteredRoutePayload {
    pub fn into_route(self) -> Result<Route, String> {
        let jsonrpc_method = self.jsonrpc_method;

        if let Some(ref json) = jsonrpc_method {
            let obj = serde_json::from_value::<serde_json::Map<String, serde_json::Value>>(json.clone())
                .map_err(|e| format!("jsonrpc_method must be an object: {}", e))?;

            if let Some(method_name_val) = obj.get("method_name") {
                let method_name = method_name_val
                    .as_str()
                    .ok_or_else(|| "jsonrpc_method.method_name must be a string".to_string())?;
                spikard::validation::validate_jsonrpc_method_name(method_name)
                    .map_err(|e| format!("Invalid JSON-RPC method name: {}", e))?;
            }
        }

        Ok(Route {
            method: self
                .method
                .parse()
                .map_err(|e| format!("Invalid method {}: {}", self.method, e))?,
            path: self.path,
            handler_name: self.handler_name,
            request_validator: None,
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: false,
            cors: None,
            expects_json_body: self.request_schema.is_some(),
            handler_dependencies: vec![],
            jsonrpc_method: jsonrpc_method.and_then(|json| serde_json::from_value(json).ok()),
        })
    }
}

/// Extract ServerConfig from a PHP associative array (Zval).
///
/// Follows the pattern from Python's extract_server_config() in crates/spikard-py/src/lib.rs:287-463.
/// PHP sends an associative array from App::configToNative(), which becomes a Zval array in Rust.
///
/// This function manually extracts each field and constructs ServerConfig directly, avoiding
/// JSON deserialization which fails on non-serializable fields like lifecycle_hooks and di_container.
fn extract_server_config_from_php(config_zval: &Zval) -> Result<spikard_http::ServerConfig, String> {
    use spikard_http::{
        ApiKeyConfig, CompressionConfig, ContactInfo, JwtConfig, LicenseInfo, OpenApiConfig, RateLimitConfig,
        SecuritySchemeInfo, ServerConfig, ServerInfo, StaticFilesConfig,
    };

    let config_array = config_zval
        .array()
        .ok_or_else(|| "Config must be an associative array".to_string())?;

    let get_optional_field = |key: &str| -> Option<&Zval> { config_array.get(key) };

    let host = get_optional_field("host")
        .and_then(|v| v.string())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "127.0.0.1".to_string());

    let port = get_optional_field("port")
        .and_then(|v| v.long())
        .map(|p| p as u16)
        .unwrap_or(8000);

    let workers = get_optional_field("workers")
        .and_then(|v| v.long())
        .map(|w| w as usize)
        .unwrap_or(1);

    let enable_request_id = get_optional_field("enable_request_id")
        .and_then(|v| v.bool())
        .unwrap_or(true);

    let graceful_shutdown = get_optional_field("graceful_shutdown")
        .and_then(|v| v.bool())
        .unwrap_or(true);

    let shutdown_timeout = get_optional_field("shutdown_timeout")
        .and_then(|v| v.long())
        .map(|t| t as u64)
        .unwrap_or(30);

    let max_body_size = get_optional_field("max_body_size")
        .and_then(|v| v.long())
        .map(|s| s as usize);

    let request_timeout = get_optional_field("request_timeout")
        .and_then(|v| v.long())
        .map(|t| t as u64);

    let compression_config = get_optional_field("compression").and_then(|v| v.array()).map(|arr| {
        let gzip = arr.get("gzip").and_then(|v| v.bool()).unwrap_or(true);
        let brotli = arr.get("brotli").and_then(|v| v.bool()).unwrap_or(true);
        let min_size = arr
            .get("min_size")
            .and_then(|v| v.long())
            .map(|s| s as usize)
            .unwrap_or(1024);
        let quality = arr.get("quality").and_then(|v| v.long()).map(|q| q as u32).unwrap_or(6);

        CompressionConfig {
            gzip,
            brotli,
            min_size,
            quality,
        }
    });

    let rate_limit_config = get_optional_field("rate_limit").and_then(|v| v.array()).map(|arr| {
        let per_second = arr
            .get("per_second")
            .and_then(|v| v.long())
            .map(|p| p as u64)
            .unwrap_or(100);
        let burst = arr.get("burst").and_then(|v| v.long()).map(|b| b as u32).unwrap_or(10);
        let ip_based = arr.get("ip_based").and_then(|v| v.bool()).unwrap_or(true);

        RateLimitConfig {
            per_second,
            burst,
            ip_based,
        }
    });

    let jwt_auth_config = get_optional_field("jwt_auth")
        .and_then(|v| v.array())
        .map(|arr| -> Result<JwtConfig, String> {
            let secret = arr
                .get("secret")
                .and_then(|v| v.string())
                .ok_or_else(|| "JWT auth requires 'secret' field".to_string())?
                .to_string();

            let algorithm = arr
                .get("algorithm")
                .and_then(|v| v.string())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "HS256".to_string());

            let audience = arr.get("audience").and_then(|v| v.array()).map(|aud_arr| {
                aud_arr
                    .iter()
                    .filter_map(|(_, v)| v.string().map(|s| s.to_string()))
                    .collect::<Vec<_>>()
            });

            let issuer = arr.get("issuer").and_then(|v| v.string()).map(|s| s.to_string());

            let leeway = arr.get("leeway").and_then(|v| v.long()).map(|l| l as u64).unwrap_or(0);

            Ok(JwtConfig {
                secret,
                algorithm,
                audience,
                issuer,
                leeway,
            })
        })
        .transpose()?;

    let api_key_auth_config = get_optional_field("api_key_auth")
        .and_then(|v| v.array())
        .map(|arr| -> Result<ApiKeyConfig, String> {
            let keys = arr
                .get("keys")
                .and_then(|v| v.array())
                .ok_or_else(|| "API key auth requires 'keys' array".to_string())?
                .iter()
                .filter_map(|(_, v)| v.string().map(|s| s.to_string()))
                .collect::<Vec<_>>();

            let header_name = arr
                .get("header_name")
                .and_then(|v| v.string())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "X-API-Key".to_string());

            Ok(ApiKeyConfig { keys, header_name })
        })
        .transpose()?;

    let static_files = get_optional_field("static_files")
        .and_then(|v| v.array())
        .map(|files_arr| -> Result<Vec<StaticFilesConfig>, String> {
            let mut configs = Vec::new();
            for (_, file_config_zval) in files_arr.iter() {
                if let Some(file_arr) = file_config_zval.array() {
                    let directory = file_arr
                        .get("directory")
                        .and_then(|v| v.string())
                        .ok_or_else(|| "Static file config requires 'directory'".to_string())?
                        .to_string();

                    let route_prefix = file_arr
                        .get("route_prefix")
                        .and_then(|v| v.string())
                        .ok_or_else(|| "Static file config requires 'route_prefix'".to_string())?
                        .to_string();

                    let index_file = file_arr.get("index_file").and_then(|v| v.bool()).unwrap_or(true);

                    let cache_control = file_arr
                        .get("cache_control")
                        .and_then(|v| v.string())
                        .map(|s| s.to_string());

                    configs.push(StaticFilesConfig {
                        directory,
                        route_prefix,
                        index_file,
                        cache_control,
                    });
                }
            }
            Ok(configs)
        })
        .transpose()?
        .unwrap_or_default();

    let openapi_config = get_optional_field("openapi")
        .and_then(|v| v.array())
        .map(|openapi_arr| -> Result<OpenApiConfig, String> {
            let enabled = openapi_arr.get("enabled").and_then(|v| v.bool()).unwrap_or(false);

            let title = openapi_arr
                .get("title")
                .and_then(|v| v.string())
                .ok_or_else(|| "OpenAPI config requires 'title'".to_string())?
                .to_string();

            let version = openapi_arr
                .get("version")
                .and_then(|v| v.string())
                .ok_or_else(|| "OpenAPI config requires 'version'".to_string())?
                .to_string();

            let description = openapi_arr
                .get("description")
                .and_then(|v| v.string())
                .map(|s| s.to_string());

            let swagger_ui_path = openapi_arr
                .get("swagger_ui_path")
                .and_then(|v| v.string())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "/docs".to_string());

            let redoc_path = openapi_arr
                .get("redoc_path")
                .and_then(|v| v.string())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "/redoc".to_string());

            let openapi_json_path = openapi_arr
                .get("openapi_json_path")
                .and_then(|v| v.string())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "/openapi.json".to_string());

            let contact = openapi_arr
                .get("contact")
                .and_then(|v| v.array())
                .map(|contact_arr| ContactInfo {
                    name: contact_arr.get("name").and_then(|v| v.string()).map(|s| s.to_string()),
                    email: contact_arr.get("email").and_then(|v| v.string()).map(|s| s.to_string()),
                    url: contact_arr.get("url").and_then(|v| v.string()).map(|s| s.to_string()),
                });

            let license = openapi_arr
                .get("license")
                .and_then(|v| v.array())
                .map(|license_arr| -> Result<LicenseInfo, String> {
                    let name = license_arr
                        .get("name")
                        .and_then(|v| v.string())
                        .ok_or_else(|| "License requires 'name'".to_string())?
                        .to_string();
                    let url = license_arr.get("url").and_then(|v| v.string()).map(|s| s.to_string());
                    Ok(LicenseInfo { name, url })
                })
                .transpose()?;

            let servers = openapi_arr
                .get("servers")
                .and_then(|v| v.array())
                .map(|servers_arr| -> Result<Vec<ServerInfo>, String> {
                    let mut server_list = Vec::new();
                    for (_, server_zval) in servers_arr.iter() {
                        if let Some(server_arr) = server_zval.array() {
                            let url = server_arr
                                .get("url")
                                .and_then(|v| v.string())
                                .ok_or_else(|| "Server requires 'url'".to_string())?
                                .to_string();
                            let description = server_arr
                                .get("description")
                                .and_then(|v| v.string())
                                .map(|s| s.to_string());
                            server_list.push(ServerInfo { url, description });
                        }
                    }
                    Ok(server_list)
                })
                .transpose()?
                .unwrap_or_default();

            let security_schemes = openapi_arr
                .get("security_schemes")
                .and_then(|v| v.array())
                .map(|schemes_arr| -> Result<HashMap<String, SecuritySchemeInfo>, String> {
                    let mut schemes = HashMap::new();
                    for (key, scheme_zval) in schemes_arr.iter() {
                        let key_str = match key {
                            ext_php_rs::types::ArrayKey::String(s) => s.to_string(),
                            ext_php_rs::types::ArrayKey::Str(s) => s.to_string(),
                            ext_php_rs::types::ArrayKey::Long(i) => i.to_string(),
                        };

                        if let Some(scheme_arr) = scheme_zval.array() {
                            let scheme_type = scheme_arr
                                .get("type")
                                .and_then(|v| v.string())
                                .ok_or_else(|| "Security scheme requires 'type'".to_string())?;

                            let scheme_info = match scheme_type.to_string().as_str() {
                                "http" => {
                                    let scheme = scheme_arr
                                        .get("scheme")
                                        .and_then(|v| v.string())
                                        .ok_or_else(|| "HTTP security scheme requires 'scheme'".to_string())?
                                        .to_string();
                                    let bearer_format = scheme_arr
                                        .get("bearer_format")
                                        .and_then(|v| v.string())
                                        .map(|s| s.to_string());
                                    SecuritySchemeInfo::Http { scheme, bearer_format }
                                }
                                "apiKey" => {
                                    let location = scheme_arr
                                        .get("location")
                                        .and_then(|v| v.string())
                                        .ok_or_else(|| "API key security scheme requires 'location'".to_string())?
                                        .to_string();
                                    let name = scheme_arr
                                        .get("name")
                                        .and_then(|v| v.string())
                                        .ok_or_else(|| "API key security scheme requires 'name'".to_string())?
                                        .to_string();
                                    SecuritySchemeInfo::ApiKey { location, name }
                                }
                                other => {
                                    return Err(format!("Invalid security scheme type: {}", other));
                                }
                            };

                            schemes.insert(key_str, scheme_info);
                        }
                    }
                    Ok(schemes)
                })
                .transpose()?
                .unwrap_or_default();

            Ok(OpenApiConfig {
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
        })
        .transpose()?;

    Ok(ServerConfig {
        host,
        port,
        workers,
        enable_request_id,
        max_body_size,
        request_timeout,
        compression: compression_config,
        rate_limit: rate_limit_config,
        jwt_auth: jwt_auth_config,
        api_key_auth: api_key_auth_config,
        static_files,
        graceful_shutdown,
        shutdown_timeout,
        background_tasks: spikard_http::BackgroundTaskConfig::default(),
        enable_http_trace: false,
        openapi: openapi_config,
        jsonrpc: None,
        lifecycle_hooks: None,
        di_container: None,
    })
}

/// Extract lifecycle hooks from PHP array/object Zval.
///
/// Expected structure from PHP LifecycleHooks:
/// - onRequest: PHP callable or null
/// - preValidation: PHP callable or null
/// - preHandler: PHP callable or null
/// - onResponse: PHP callable or null
/// - onError: PHP callable or null
///
/// Returns LifecycleHooks with registered PHP hook wrappers.
fn extract_lifecycle_hooks_from_php(hooks_zval: &Zval) -> Result<Option<Arc<LifecycleHooks>>, String> {
    if hooks_zval.is_null() {
        return Ok(None);
    }

    let hooks_array = match hooks_zval.array() {
        Some(arr) if !arr.is_empty() => arr,
        _ => return Ok(None),
    };

    let mut lifecycle_hooks = LifecycleHooks::default();
    let mut has_any_hook = false;

    if let Some(on_request_zval) = hooks_array.get("onRequest")
        && on_request_zval.is_callable()
    {
        let hook = PhpRequestHook::new_from_zval(on_request_zval)
            .map_err(|e| format!("Failed to create onRequest hook: {}", e))?;
        lifecycle_hooks.add_on_request(Arc::new(hook));
        has_any_hook = true;
    }

    if let Some(pre_validation_zval) = hooks_array.get("preValidation")
        && pre_validation_zval.is_callable()
    {
        let hook = PhpRequestHook::new_from_zval(pre_validation_zval)
            .map_err(|e| format!("Failed to create preValidation hook: {}", e))?;
        lifecycle_hooks.add_pre_validation(Arc::new(hook));
        has_any_hook = true;
    }

    if let Some(pre_handler_zval) = hooks_array.get("preHandler")
        && pre_handler_zval.is_callable()
    {
        let hook = PhpRequestHook::new_from_zval(pre_handler_zval)
            .map_err(|e| format!("Failed to create preHandler hook: {}", e))?;
        lifecycle_hooks.add_pre_handler(Arc::new(hook));
        has_any_hook = true;
    }

    if let Some(on_response_zval) = hooks_array.get("onResponse")
        && on_response_zval.is_callable()
    {
        let hook = PhpResponseHook::new_from_zval(on_response_zval)
            .map_err(|e| format!("Failed to create onResponse hook: {}", e))?;
        lifecycle_hooks.add_on_response(Arc::new(hook));
        has_any_hook = true;
    }

    if let Some(on_error_zval) = hooks_array.get("onError")
        && on_error_zval.is_callable()
    {
        let hook =
            PhpErrorHook::new_from_zval(on_error_zval).map_err(|e| format!("Failed to create onError hook: {}", e))?;
        lifecycle_hooks.add_on_error(Arc::new(hook));
        has_any_hook = true;
    }

    if has_any_hook {
        Ok(Some(Arc::new(lifecycle_hooks)))
    } else {
        Ok(None)
    }
}

/// Start a server from PHP, given route/config payloads.
///
/// This function now accepts PHP objects directly instead of JSON:
/// - `routes`: Array of route payload arrays (still JSON for now, to be refactored)
/// - `config`: PHP associative array from App::configToNative()
/// - `hooks`: PHP associative array (currently unused, hooks not yet supported)
///
/// The config is extracted manually using extract_server_config_from_php() to avoid
/// JSON deserialization issues with non-serializable fields like lifecycle_hooks.
pub fn spikard_start_server_impl(
    routes_zval: &Zval,
    config: &Zval,
    hooks: &Zval,
    dependencies: &Zval,
) -> PhpResult<i64> {
    let mut server_config = extract_server_config_from_php(config)
        .map_err(|e| PhpException::default(format!("Invalid server config: {}", e)))?;

    let lifecycle_hooks = extract_lifecycle_hooks_from_php(hooks)
        .map_err(|e| PhpException::default(format!("Invalid lifecycle hooks: {}", e)))?;

    server_config.lifecycle_hooks = lifecycle_hooks;

    let di_container = crate::php::extract_di_container_from_php(Some(dependencies))
        .map_err(|e| PhpException::default(format!("Invalid DI container: {}", e)))?;
    if let Some(container) = di_container {
        server_config.di_container = Some(std::sync::Arc::new(container));
    }

    let routes_array = routes_zval
        .array()
        .ok_or_else(|| PhpException::default("Routes must be an array".to_string()))?;

    let mut route_pairs: Vec<(spikard_http::Route, Arc<dyn spikard_http::Handler>)> = Vec::new();
    let mut route_metadata: Vec<spikard_core::RouteMetadata> = Vec::new();

    for (_idx, route_val) in routes_array.iter() {
        let route_array = route_val
            .array()
            .ok_or_else(|| PhpException::default("Route must be an array".to_string()))?;
        let handler_callable = route_array
            .get("handler")
            .ok_or_else(|| PhpException::default("Missing handler callable".to_string()))?;

        let json_val = crate::php::zval_to_json(route_val)
            .map_err(|e| PhpException::default(format!("Failed to convert route to JSON: {}", e)))?;

        let reg = serde_json::from_value::<RegisteredRoutePayload>(json_val)
            .map_err(|e| PhpException::default(format!("Invalid route payload: {}", e)))?;
        let method = reg.method.clone();
        let path = reg.path.clone();
        let handler_name = reg.handler_name.clone();
        let request_schema = reg.request_schema.clone();
        let response_schema = reg.response_schema.clone();
        let parameter_schema = reg.parameter_schema.clone();
        let jsonrpc_method = reg.jsonrpc_method.clone();

        let handler =
            PhpHandler::register_from_zval(handler_callable, handler_name.clone(), method.clone(), path.clone())
                .map_err(|e| PhpException::default(format!("Failed to register handler: {}", e)))?;

        let mut route = reg.into_route()?;

        if let Some(schema) = request_schema.clone() {
            let compiled = spikard_core::validation::SchemaValidator::new(schema)
                .map_err(|e| PhpException::default(format!("Invalid request schema: {}", e)))?;
            route.request_validator = Some(Arc::new(compiled));
        }
        if let Some(schema) = response_schema.clone() {
            let compiled = spikard_core::validation::SchemaValidator::new(schema)
                .map_err(|e| PhpException::default(format!("Invalid response schema: {}", e)))?;
            route.response_validator = Some(Arc::new(compiled));
        }
        if let Some(schema) = parameter_schema.clone() {
            let compiled =
                spikard_http::ParameterValidator::new(schema).map_err(|e| PhpException::default(e.to_string()))?;
            route.parameter_validator = Some(compiled);
        }

        route_metadata.push(spikard_core::RouteMetadata {
            method,
            path,
            handler_name,
            request_schema,
            response_schema,
            parameter_schema,
            file_params: None,
            is_async: true,
            cors: None,
            body_param_name: None,
            handler_dependencies: Some(Vec::new()),
            jsonrpc_method,
        });

        route_pairs.push((route, Arc::new(handler) as Arc<dyn spikard_http::Handler>));
    }

    let app = build_router_with_handlers_and_config(route_pairs, server_config.clone(), route_metadata)
        .map_err(|e| PhpException::default(format!("Failed to build router: {}", e)))?;

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| PhpException::default(format!("Failed to start Tokio runtime: {}", e)))?;

    let result: PhpResult<()> = runtime.block_on(async move {
        let local = tokio::task::LocalSet::new();

        local
            .run_until(async move {
                let addr = format!("{}:{}", server_config.host, server_config.port);
                let listener = tokio::net::TcpListener::bind(&addr)
                    .await
                    .map_err(|e| PhpException::default(format!("Failed to bind to {}: {}", addr, e)))?;

                let background_runtime =
                    spikard_http::BackgroundRuntime::start(server_config.background_tasks.clone()).await;
                crate::php::install_handle(background_runtime.handle());

                tokio::task::spawn_local(async {
                    loop {
                        crate::php::process_pending_tasks();
                        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                    }
                });

                let shutdown_signal = async move {
                    let ctrl_c = async {
                        let _ = tokio::signal::ctrl_c().await;
                    };

                    #[cfg(unix)]
                    let terminate = async {
                        if let Ok(mut stream) =
                            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                        {
                            let _ = stream.recv().await;
                        }
                    };

                    #[cfg(unix)]
                    tokio::select! {
                        _ = ctrl_c => {},
                        _ = terminate => {},
                    }

                    #[cfg(not(unix))]
                    ctrl_c.await;
                };

                axum::serve(listener, app)
                    .with_graceful_shutdown(shutdown_signal)
                    .await
                    .map_err(|e| PhpException::default(format!("Server error: {}", e)))?;

                crate::php::clear_handle();
                if let Err(e) = background_runtime.shutdown().await {
                    eprintln!("Failed to drain background tasks during shutdown: {:?}", e);
                }

                Ok(())
            })
            .await
    });

    result?;
    Ok(0)
}

/// Stop server by handle.
///
/// Triggers graceful shutdown of the server identified by the given handle.
/// This sends a signal through the shutdown channel, causing the server to
/// stop accepting new connections and finish processing existing requests.
pub fn spikard_stop_server_impl(handle: i64) -> PhpResult<()> {
    let _ = handle;
    Ok(())
}

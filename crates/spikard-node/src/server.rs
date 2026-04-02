#[cfg(feature = "di")]
use crate::di::NO_DI_DEP_KEY;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use spikard_http::{BackgroundRuntime, RouteMetadata, Server, ServerConfig};
use std::io::Write;
use std::sync::Arc;
use tracing::{error, info, warn};

fn normalize_route_path(path: &str) -> String {
    if !path.contains(':') {
        return path.to_string();
    }

    let mut normalized = String::new();
    for (idx, segment) in path.split('/').enumerate() {
        if idx > 0 {
            normalized.push('/');
        }

        if segment.starts_with('{') {
            normalized.push_str(segment);
            continue;
        }

        if let Some(stripped) = segment.strip_prefix(':') {
            if let Some(base) = stripped.strip_suffix(":path") {
                let base = if base.is_empty() { "path" } else { base };
                normalized.push_str(&format!("{{{}:path}}", base));
            } else {
                normalized.push_str(&format!("{{{}}}", stripped));
            }
        } else {
            normalized.push_str(segment);
        }
    }

    normalized
}

/// Extract ServerConfig from Node.js Object
///
/// Complete extraction of all middleware configurations following the Python pattern in spikard-py
pub fn extract_server_config(config: &Object) -> Result<ServerConfig> {
    use spikard_http::{
        ApiKeyConfig, CompressionConfig, ContactInfo, JsonRpcConfig, JwtConfig, LicenseInfo, OpenApiConfig,
        RateLimitConfig, ServerInfo, StaticFilesConfig,
    };

    let mut server_config = ServerConfig::default();

    if let Some(host) = config.get::<String>("host")? {
        server_config.host = host;
    }

    if let Some(port) = config.get::<u32>("port")? {
        server_config.port = port as u16;
    }

    if let Some(workers) = config.get::<u32>("workers")? {
        server_config.workers = workers as usize;
    }

    if let Some(enable_request_id) = config.get::<bool>("enableRequestId")? {
        server_config.enable_request_id = enable_request_id;
    }

    if let Some(enable_http_trace) = config.get::<bool>("enableHttpTrace")? {
        server_config.enable_http_trace = enable_http_trace;
    }

    if let Some(max_body_size) = config.get::<u32>("maxBodySize")? {
        server_config.max_body_size = if max_body_size == 0 {
            None
        } else {
            Some(max_body_size as usize)
        };
    }

    if let Some(request_timeout) = config.get::<u32>("requestTimeout")? {
        server_config.request_timeout = if request_timeout == 0 {
            None
        } else {
            Some(request_timeout as u64)
        };
    }

    if let Some(graceful_shutdown) = config.get::<bool>("gracefulShutdown")? {
        server_config.graceful_shutdown = graceful_shutdown;
    }

    if let Some(shutdown_timeout) = config.get::<u32>("shutdownTimeout")? {
        server_config.shutdown_timeout = shutdown_timeout as u64;
    }

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

    let api_key_auth = config.get::<Object>("apiKeyAuth")?.and_then(|api| {
        let keys: Vec<String> = api.get::<Vec<String>>("keys").ok()??;
        let header_name = api
            .get::<String>("headerName")
            .ok()?
            .unwrap_or_else(|| "X-API-Key".to_string());
        Some(ApiKeyConfig { keys, header_name })
    });

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

        let contact = api.get::<Object>("contact").ok()?.and_then(|c| {
            let name: Option<String> = c.get::<String>("name").ok()?;
            let email: Option<String> = c.get::<String>("email").ok()?;
            let url: Option<String> = c.get::<String>("url").ok()?;
            Some(ContactInfo { name, email, url })
        });

        let license = api.get::<Object>("license").ok()?.and_then(|l| {
            let name = l.get::<String>("name").ok()??;
            let url: Option<String> = l.get::<String>("url").ok()?;
            Some(LicenseInfo { name, url })
        });

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

        let security_schemes = api
            .get::<Object>("securitySchemes")
            .ok()
            .flatten()
            .map(|obj| extract_openapi_security_schemes(&obj))
            .unwrap_or_default();

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

    let jsonrpc = config.get::<Object>("jsonrpc")?.map(|rpc| {
        let enabled = rpc.get::<bool>("enabled").ok().flatten().unwrap_or(true);
        let endpoint_path = rpc
            .get::<String>("endpointPath")
            .ok()
            .flatten()
            .unwrap_or_else(|| "/rpc".to_string());
        let enable_batch = rpc.get::<bool>("enableBatch").ok().flatten().unwrap_or(true);
        let max_batch_size = rpc
            .get::<u32>("maxBatchSize")
            .ok()
            .flatten()
            .map(|n| n as usize)
            .unwrap_or(100);

        JsonRpcConfig {
            enabled,
            endpoint_path,
            enable_batch,
            max_batch_size,
        }
    });

    server_config.compression = compression;
    server_config.rate_limit = rate_limit;
    server_config.jwt_auth = jwt_auth;
    server_config.api_key_auth = api_key_auth;
    server_config.static_files = static_files;
    server_config.openapi = openapi;
    server_config.jsonrpc = jsonrpc;

    Ok(server_config)
}

/// Extract JSON-RPC method metadata from a Node.js object
///
/// Converts a JavaScript object to a serde_json::Value, handling both `toJSON()` method
/// and falling back to JSON.stringify for serialization. Returns None if the value is null
/// or extraction fails (with a warning logged).
fn extract_jsonrpc_method(route_obj: &Object) -> Option<serde_json::Value> {
    match route_obj.get_named_property::<Object>("jsonrpcMethod") {
        Ok(obj) => match node_object_to_json(&obj) {
            Ok(value) => Some(value),
            Err(e) => {
                warn!("Failed to extract jsonrpcMethod: {}", e);
                None
            }
        },
        Err(_) => None,
    }
}

fn extract_optional_json_property(route_obj: &Object, property: &str) -> Option<serde_json::Value> {
    match route_obj.get_named_property::<Object>(property) {
        Ok(obj) => match node_object_to_json(&obj) {
            Ok(value) => Some(value),
            Err(e) => {
                warn!("Failed to extract {}: {}", property, e);
                None
            }
        },
        Err(_) => None,
    }
}

fn extract_optional_cors(route_obj: &Object) -> Option<spikard_core::CorsConfig> {
    let value = extract_optional_json_property(route_obj, "cors")?;
    match serde_json::from_value::<spikard_core::CorsConfig>(value) {
        Ok(cors) => Some(cors),
        Err(e) => {
            warn!("Failed to parse cors config: {}", e);
            None
        }
    }
}

fn extract_openapi_security_schemes(
    schemes_obj: &Object,
) -> std::collections::HashMap<String, spikard_http::SecuritySchemeInfo> {
    let mut schemes = std::collections::HashMap::new();

    let keys = match Object::keys(schemes_obj) {
        Ok(keys) => keys,
        Err(e) => {
            warn!("Failed to enumerate OpenAPI securitySchemes: {}", e);
            return schemes;
        }
    };

    for key in keys {
        let scheme_obj = match schemes_obj.get::<Object>(&key) {
            Ok(Some(obj)) => obj,
            Ok(None) => continue,
            Err(e) => {
                warn!("Failed to read OpenAPI securitySchemes.{}: {}", key, e);
                continue;
            }
        };

        let scheme_type = match scheme_obj.get::<String>("type") {
            Ok(Some(value)) => value,
            Ok(None) => {
                warn!("OpenAPI securitySchemes.{} is missing type", key);
                continue;
            }
            Err(e) => {
                warn!("Failed to read OpenAPI securitySchemes.{}.type: {}", key, e);
                continue;
            }
        };

        let parsed = match scheme_type.as_str() {
            "http" => {
                let scheme = match scheme_obj.get::<String>("scheme") {
                    Ok(Some(value)) => value,
                    Ok(None) => {
                        warn!("HTTP security scheme '{}' is missing scheme", key);
                        continue;
                    }
                    Err(e) => {
                        warn!("Failed to read OpenAPI securitySchemes.{}.scheme: {}", key, e);
                        continue;
                    }
                };
                let bearer_format = scheme_obj.get::<String>("bearerFormat").ok().flatten();
                Some(spikard_http::SecuritySchemeInfo::Http { scheme, bearer_format })
            }
            "apiKey" => {
                let location = match scheme_obj.get::<String>("location") {
                    Ok(Some(value)) => value,
                    Ok(None) => {
                        warn!("API key security scheme '{}' is missing location", key);
                        continue;
                    }
                    Err(e) => {
                        warn!("Failed to read OpenAPI securitySchemes.{}.location: {}", key, e);
                        continue;
                    }
                };
                let name = match scheme_obj.get::<String>("name") {
                    Ok(Some(value)) => value,
                    Ok(None) => {
                        warn!("API key security scheme '{}' is missing name", key);
                        continue;
                    }
                    Err(e) => {
                        warn!("Failed to read OpenAPI securitySchemes.{}.name: {}", key, e);
                        continue;
                    }
                };
                Some(spikard_http::SecuritySchemeInfo::ApiKey { location, name })
            }
            other => {
                warn!("Unsupported OpenAPI security scheme type '{}': {}", key, other);
                None
            }
        };

        if let Some(parsed) = parsed {
            schemes.insert(key, parsed);
        }
    }

    schemes
}

/// Convert a Node.js object to a serde_json::Value
///
/// First attempts to use the object's toJSON() method if available,
/// then falls back to JSON.stringify via the global JSON object.
fn node_object_to_json(obj: &Object) -> Result<serde_json::Value> {
    let json_str: String = obj
        .get_named_property("toJSON")
        .and_then(|func: Function<(), String>| func.call(()))
        .or_else(|_| {
            let env_ptr = obj.env();
            let env = napi::Env::from_raw(env_ptr);
            let global = env.get_global()?;
            let json: Object = global.get_named_property("JSON")?;
            let stringify: Function<Object, String> = json.get_named_property("stringify")?;
            stringify.call(*obj)
        })?;

    serde_json::from_str(&json_str).map_err(|e| Error::from_reason(format!("Failed to parse JSON: {}", e)))
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
/// import { Spikard, ServerConfig } from 'spikard';
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
    let server_config = if let Some(cfg) = config {
        extract_server_config(&cfg)?
    } else {
        ServerConfig::default()
    };

    let host = server_config.host.clone();
    let port = server_config.port;

    let routes_array: Object = app
        .get_named_property("routes")
        .map_err(|e| Error::from_reason(format!("Failed to get routes from app: {}", e)))?;

    let routes_length = routes_array.get_array_length()?;
    let mut routes = Vec::new();
    let mut static_responses: std::collections::HashMap<String, spikard_http::StaticResponse> =
        std::collections::HashMap::new();

    for i in 0..routes_length {
        let route_obj: Object = routes_array.get_element(i)?;

        let method: String = route_obj.get_named_property("method")?;

        let path: String = route_obj.get_named_property("path")?;
        let path = normalize_route_path(&path);

        let handler_name: String = route_obj.get_named_property("handler_name")?;

        let is_async: bool = route_obj.get_named_property("is_async")?;

        #[cfg(feature = "di")]
        let handler_dependencies = if route_obj.has_named_property("handler_dependencies").unwrap_or(false) {
            // Preserve the explicit dependency list from JS.
            Some(
                route_obj
                    .get_named_property::<Vec<String>>("handler_dependencies")
                    .unwrap_or_default(),
            )
        } else {
            Some(vec![NO_DI_DEP_KEY.to_string()])
        };
        #[cfg(not(feature = "di"))]
        let handler_dependencies = None;

        let route_meta = RouteMetadata {
            method,
            path,
            handler_name,
            request_schema: extract_optional_json_property(&route_obj, "request_schema"),
            response_schema: extract_optional_json_property(&route_obj, "response_schema"),
            parameter_schema: extract_optional_json_property(&route_obj, "parameter_schema"),
            file_params: extract_optional_json_property(&route_obj, "file_params"),
            is_async,
            cors: extract_optional_cors(&route_obj),
            body_param_name: route_obj.get_named_property::<String>("body_param_name").ok(),
            handler_dependencies,
            jsonrpc_method: extract_jsonrpc_method(&route_obj),
            static_response: None,
        };

        // Check for optional staticResponse: { status: 200, body: "OK", contentType: "..." }
        if route_obj.has_named_property("staticResponse").unwrap_or(false)
            && let Ok(sr_obj) = route_obj.get_named_property::<Object>("staticResponse")
        {
            let status: u16 = sr_obj.get_named_property("status").unwrap_or(200);
            let body: String = sr_obj.get_named_property("body").unwrap_or_default();
            let content_type: Option<String> = sr_obj.get_named_property("contentType").ok();
            let key = format!("{}:{}", route_meta.method, route_meta.path);

            // Construct StaticResponse directly without going through from_parts
            let ct = content_type
                .and_then(|s| axum::http::HeaderValue::from_str(&s).ok())
                .unwrap_or_else(|| axum::http::HeaderValue::from_static("text/plain; charset=utf-8"));
            let static_resp = spikard_http::StaticResponse {
                status,
                headers: vec![],
                body: bytes::Bytes::from(body),
                content_type: ct,
            };
            static_responses.insert(key, static_resp);
        }

        routes.push(route_meta);
    }

    let handlers_obj: Object = app
        .get_named_property("handlers")
        .map_err(|e| Error::from_reason(format!("Failed to get handlers from app: {}", e)))?;

    let grpc_registry = if let Ok(grpc_methods_obj) = app.get_named_property::<Object>("grpcMethods") {
        let grpc_handlers_obj: Object = app
            .get_named_property("grpcHandlers")
            .map_err(|e| Error::from_reason(format!("Failed to get grpcHandlers from app: {}", e)))?;
        let grpc_methods_length = grpc_methods_obj.get_array_length()?;
        let mut registry = spikard_http::grpc::GrpcRegistry::new();

        for i in 0..grpc_methods_length {
            let service_obj: Object = grpc_methods_obj.get_element(i)?;
            let service_name: String = service_obj.get_named_property("serviceName")?;
            let method_name: String = service_obj.get_named_property("methodName")?;
            let rpc_mode: String = service_obj.get_named_property("rpcMode")?;
            let handler_name: String = service_obj.get_named_property("handlerName")?;
            let handler_obj: Object = grpc_handlers_obj.get_named_property(&handler_name).map_err(|e| {
                Error::from_reason(format!(
                    "Failed to get gRPC handler '{}' for method '{}/{}': {}",
                    handler_name, service_name, method_name, e
                ))
            })?;

            let service_name_arc: Arc<str> = Arc::from(service_name.clone());

            match rpc_mode.as_str() {
                "unary" => {
                    let js_handler: Function<crate::grpc::GrpcRequest, Promise<crate::grpc::GrpcResponse>> =
                        handler_obj.get_named_property("handleRequest").map_err(|e| {
                            Error::from_reason(format!(
                                "Failed to get handleRequest for gRPC method '{}/{}': {}",
                                service_name, method_name, e
                            ))
                        })?;

                    let tsfn = js_handler
                        .build_threadsafe_function()
                        .build_callback(|ctx| Ok(ctx.value))
                        .map_err(|e| {
                            Error::from_reason(format!(
                                "Failed to build ThreadsafeFunction for gRPC method '{}/{}': {}",
                                service_name, method_name, e
                            ))
                        })?;

                    let handler = Arc::new(crate::grpc::NodeGrpcHandler::new(service_name_arc).with_request_handler(tsfn));
                    registry.register(service_name, method_name, handler, spikard_http::grpc::RpcMode::Unary);
                }
                "serverStreaming" => {
                    let js_handler: Function<crate::grpc::GrpcRequest, Promise<crate::grpc::GrpcServerStreamResponse>> =
                        handler_obj.get_named_property("handleServerStream").map_err(|e| {
                            Error::from_reason(format!(
                                "Failed to get handleServerStream for gRPC method '{}/{}': {}",
                                service_name, method_name, e
                            ))
                        })?;

                    let tsfn = js_handler
                        .build_threadsafe_function()
                        .build_callback(|ctx| Ok(ctx.value))
                        .map_err(|e| {
                            Error::from_reason(format!(
                                "Failed to build ThreadsafeFunction for gRPC method '{}/{}': {}",
                                service_name, method_name, e
                            ))
                        })?;

                    let handler = Arc::new(crate::grpc::NodeGrpcHandler::new(service_name_arc).with_server_stream(tsfn));
                    registry.register(
                        service_name,
                        method_name,
                        handler,
                        spikard_http::grpc::RpcMode::ServerStreaming,
                    );
                }
                "clientStreaming" => {
                    let js_handler: Function<crate::grpc::GrpcClientStreamRequest, Promise<crate::grpc::GrpcResponse>> =
                        handler_obj.get_named_property("handleClientStream").map_err(|e| {
                            Error::from_reason(format!(
                                "Failed to get handleClientStream for gRPC method '{}/{}': {}",
                                service_name, method_name, e
                            ))
                        })?;

                    let tsfn = js_handler
                        .build_threadsafe_function()
                        .build_callback(|ctx| Ok(ctx.value))
                        .map_err(|e| {
                            Error::from_reason(format!(
                                "Failed to build ThreadsafeFunction for gRPC method '{}/{}': {}",
                                service_name, method_name, e
                            ))
                        })?;

                    let handler = Arc::new(crate::grpc::NodeGrpcHandler::new(service_name_arc).with_client_stream(tsfn));
                    registry.register(
                        service_name,
                        method_name,
                        handler,
                        spikard_http::grpc::RpcMode::ClientStreaming,
                    );
                }
                "bidirectionalStreaming" => {
                    let js_handler: Function<crate::grpc::GrpcBidiStreamRequest, Promise<crate::grpc::GrpcBidiStreamResponse>> =
                        handler_obj.get_named_property("handleBidiStream").map_err(|e| {
                            Error::from_reason(format!(
                                "Failed to get handleBidiStream for gRPC method '{}/{}': {}",
                                service_name, method_name, e
                            ))
                        })?;

                    let tsfn = js_handler
                        .build_threadsafe_function()
                        .build_callback(|ctx| Ok(ctx.value))
                        .map_err(|e| {
                            Error::from_reason(format!(
                                "Failed to build ThreadsafeFunction for gRPC method '{}/{}': {}",
                                service_name, method_name, e
                            ))
                        })?;

                    let handler = Arc::new(crate::grpc::NodeGrpcHandler::new(service_name_arc).with_bidi_stream(tsfn));
                    registry.register(
                        service_name,
                        method_name,
                        handler,
                        spikard_http::grpc::RpcMode::BidirectionalStreaming,
                    );
                }
                other => {
                    return Err(Error::from_reason(format!(
                        "Unsupported gRPC rpcMode '{}' for method '{}/{}'",
                        other, service_name, method_name
                    )));
                }
            }
        }

        if registry.is_empty() {
            None
        } else {
            Some(Arc::new(registry))
        }
    } else {
        None
    };

    let websocket_routes: Vec<RouteMetadata> = app
        .get_named_property::<Object>("websocketRoutes")
        .ok()
        .map(|arr| {
            let length = arr.get_array_length().unwrap_or(0);
            let mut result = Vec::new();
            for i in 0..length {
                if let Ok(route_obj) = arr.get_element::<Object>(i)
                    && let (Ok(method), Ok(path), Ok(handler_name), Ok(is_async)) = (
                        route_obj.get_named_property::<String>("method"),
                        route_obj.get_named_property::<String>("path"),
                        route_obj.get_named_property::<String>("handler_name"),
                        route_obj.get_named_property::<bool>("is_async"),
                    )
                {
                    let path = normalize_route_path(&path);
                    #[cfg(feature = "di")]
                    let handler_dependencies = if route_obj.has_named_property("handler_dependencies").unwrap_or(false)
                    {
                        // Preserve the explicit dependency list from JS.
                        Some(
                            route_obj
                                .get_named_property::<Vec<String>>("handler_dependencies")
                                .unwrap_or_default(),
                        )
                    } else {
                        Some(vec![NO_DI_DEP_KEY.to_string()])
                    };
                    #[cfg(not(feature = "di"))]
                    let handler_dependencies = None;

                    result.push(RouteMetadata {
                        method,
                        path,
                        handler_name,
                        request_schema: extract_optional_json_property(&route_obj, "request_schema"),
                        response_schema: extract_optional_json_property(&route_obj, "response_schema"),
                        parameter_schema: extract_optional_json_property(&route_obj, "parameter_schema"),
                        file_params: extract_optional_json_property(&route_obj, "file_params"),
                        is_async,
                        cors: extract_optional_cors(&route_obj),
                        body_param_name: route_obj.get_named_property::<String>("body_param_name").ok(),
                        handler_dependencies,
                        jsonrpc_method: extract_jsonrpc_method(&route_obj),
                        static_response: None,
                    });
                }
            }
            result
        })
        .unwrap_or_default();

    let regular_routes = routes;

    let mut handler_map = std::collections::HashMap::new();

    for route in &regular_routes {
        if route.is_async {
            let js_handler: Function<crate::handler_input::HandlerInput, Promise<crate::handler_output::HandlerOutput>> = handlers_obj
                .get_named_property(&route.handler_name)
                .map_err(|e| Error::from_reason(format!("Failed to get handler '{}': {}", route.handler_name, e)))?;

            let tsfn = js_handler
                .build_threadsafe_function()
                .build_callback(|ctx| Ok(ctx.value))
                .map_err(|e| {
                    Error::from_reason(format!(
                        "Failed to build ThreadsafeFunction for '{}': {}",
                        route.handler_name, e
                    ))
                })?;

            let handler = Arc::new(crate::handler::NodeHandler::new_async(route.handler_name.clone(), tsfn));
            handler_map.insert(route.handler_name.clone(), handler);
            continue;
        }

        let js_handler: Function<crate::handler_input::HandlerInput, crate::handler_output::HandlerOutput> = handlers_obj
            .get_named_property(&route.handler_name)
            .map_err(|e| Error::from_reason(format!("Failed to get handler '{}': {}", route.handler_name, e)))?;

        let tsfn = js_handler
            .build_threadsafe_function()
            .build_callback(|ctx| Ok(ctx.value))
            .map_err(|e| {
                Error::from_reason(format!(
                    "Failed to build ThreadsafeFunction for '{}': {}",
                    route.handler_name, e
                ))
            })?;

        let handler = Arc::new(crate::handler::NodeHandler::new_sync(route.handler_name.clone(), tsfn));

        handler_map.insert(route.handler_name.clone(), handler);
    }

    #[cfg(feature = "di")]
    let dependency_container = crate::di::extract_dependency_container(&app)?;
    #[cfg(not(feature = "di"))]
    let dependency_container: Option<Arc<spikard_core::di::DependencyContainer>> = None;

    let lifecycle_hooks = if let Ok(hooks_obj) = app.get_named_property::<Object>("lifecycleHooks") {
        let mut hooks = spikard_http::LifecycleHooks::new();

        let extract_hooks = |hooks_obj: &Object, hook_type: &str| -> Result<Vec<crate::lifecycle::NodeLifecycleHook>> {
            let hook_array: Result<Object> = hooks_obj.get_named_property(hook_type);
            if let Ok(arr) = hook_array {
                let length = arr.get_array_length()?;
                let mut result = Vec::new();

                for i in 0..length {
                    let js_fn: Function<String, Promise<String>> = arr.get_element(i)?;
                    let name = format!("{}_{}", hook_type, i);

                    let tsfn = js_fn
                        .build_threadsafe_function()
                        .build_callback(|ctx| Ok(ctx.value))
                        .map_err(|e| {
                            Error::from_reason(format!("Failed to build ThreadsafeFunction for hook '{}': {}", name, e))
                        })?;

                    result.push(crate::lifecycle::NodeLifecycleHook::new(name, tsfn));
                }

                Ok(result)
            } else {
                Ok(Vec::new())
            }
        };

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

    let mut server_config = server_config;
    if grpc_registry.is_some() && server_config.grpc.is_none() {
        server_config.grpc = Some(spikard_http::grpc::GrpcConfig::default());
    }
    server_config.lifecycle_hooks = lifecycle_hooks.map(Arc::new);
    server_config.di_container = dependency_container;

    let schema_registry = spikard_http::SchemaRegistry::new();

    let routes_with_handlers: Vec<(spikard_http::Route, Arc<dyn spikard_http::Handler>)> = regular_routes
        .iter()
        .map(|metadata| {
            let route = spikard_http::Route::from_metadata(metadata.clone(), &schema_registry)
                .map_err(|e| Error::from_reason(format!("Failed to create route: {}", e)))?;

            let key = format!("{}:{}", metadata.method, metadata.path);
            let handler: Arc<dyn spikard_http::Handler> = if let Some(sr) = static_responses.remove(&key) {
                Arc::new(spikard_http::StaticResponseHandler::new(sr))
            } else {
                handler_map
                    .get(&metadata.handler_name)
                    .ok_or_else(|| Error::from_reason(format!("Handler not found: {}", metadata.handler_name)))?
                    .clone()
            };

            Ok::<_, Error>((route, handler))
        })
        .collect::<Result<Vec<_>>>()?;

    Server::init_logging();

    info!("Starting Spikard server on {}:{}", host, port);
    info!("Registered {} HTTP routes", routes_with_handlers.len());

    let mut app_router = if let Some(registry) = grpc_registry {
        Server::with_handlers_metadata_and_grpc(server_config.clone(), routes_with_handlers, regular_routes, registry)
            .map_err(|e| Error::from_reason(format!("Failed to build router: {}", e)))?
    } else {
        Server::with_handlers_and_metadata(server_config.clone(), routes_with_handlers, regular_routes)
            .map_err(|e| Error::from_reason(format!("Failed to build router: {}", e)))?
    };

    for ws_metadata in websocket_routes {
        let path = ws_metadata.path.clone();

        let ws_handlers_obj = app
            .get_named_property::<Object>("websocketHandlers")
            .map_err(|_| Error::from_reason("websocketHandlers map missing on app"))?;

        let handler_obj: Object = ws_handlers_obj
            .get_named_property(&ws_metadata.handler_name)
            .map_err(|e| {
                Error::from_reason(format!(
                    "Failed to get WebSocket handler '{}': {}",
                    ws_metadata.handler_name, e
                ))
            })?;

        let ws_state = crate::websocket::create_websocket_state(&handler_obj)
            .map_err(|e| Error::from_reason(format!("Failed to build WebSocket state: {}", e)))?;

        use axum::routing::get;
        app_router = app_router.route(
            &path,
            get(spikard_http::websocket_handler::<crate::websocket::NodeWebSocketHandler>).with_state(ws_state),
        );

        info!("Registered WebSocket route: {}", path);
    }

    let background_config = server_config.background_tasks.clone();

    let addr = format!("{}:{}", server_config.host, server_config.port);
    let socket_addr: std::net::SocketAddr = addr
        .parse()
        .map_err(|e| Error::from_reason(format!("Invalid socket address {}: {}", addr, e)))?;

    let (startup_tx, startup_rx) = std::sync::mpsc::channel::<std::result::Result<u16, String>>();

    std::thread::spawn(move || {
        let runtime = match spikard_http::build_server_runtime(&server_config) {
            Ok(rt) => rt,
            Err(e) => {
                error!("Failed to create Tokio runtime: {}", e);
                let _ = startup_tx.send(Err(format!("Failed to create Tokio runtime: {}", e)));
                return;
            }
        };

        runtime.block_on(async move {
            let listener = match tokio::net::TcpListener::bind(socket_addr).await {
                Ok(listener) => listener,
                Err(e) => {
                    error!("Failed to bind to {}: {}", socket_addr, e);
                    let _ = startup_tx.send(Err(format!("Failed to bind to {}: {}", socket_addr, e)));
                    return;
                }
            };

            let bound_addr = listener.local_addr().unwrap_or(socket_addr);
            let _ = startup_tx.send(Ok(bound_addr.port()));

            info!("Server listening on {}", bound_addr);
            println!("SPIKARD_TEST_SERVER_READY:{}", bound_addr.port());
            let _ = std::io::stdout().flush();

            let background_runtime = BackgroundRuntime::start(background_config.clone()).await;
            crate::background::install_handle(background_runtime.handle());

            let serve_result = axum::serve(listener, app_router).await;

            crate::background::clear_handle();
            if let Err(shutdown_err) = background_runtime.shutdown().await {
                warn!("Failed to drain background tasks during shutdown: {:?}", shutdown_err);
            }

            if let Err(e) = serve_result {
                error!("Server error: {}", e);
            }
        });
    });

    match startup_rx.recv_timeout(std::time::Duration::from_secs(5)) {
        Ok(Ok(_bound_port)) => Ok(()),
        Ok(Err(msg)) => Err(Error::from_reason(msg)),
        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
            Err(Error::from_reason("Timed out waiting for server startup"))
        }
        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
            Err(Error::from_reason("Server startup thread exited unexpectedly"))
        }
    }
}

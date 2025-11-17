//! HTTP server implementation using Tokio and Axum

use crate::handler_trait::{Handler, HandlerResult, RequestData};
use crate::parameters::ParameterValidator;
use crate::query_parser::parse_query_string_to_json;
use crate::validation::SchemaValidator;
use crate::{CorsConfig, ProblemDetails, Router, ServerConfig};
use axum::Router as AxumRouter;
use axum::body::Body;
use axum::extract::{DefaultBodyLimit, Path};
use axum::routing::{MethodRouter, get};
use http_body_util::BodyExt;
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::key_extractor::GlobalKeyExtractor;
use tower_http::compression::CompressionLayer;
use tower_http::compression::predicate::{NotForContentType, Predicate, SizeAbove};
use tower_http::request_id::{MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer};
use tower_http::sensitive_headers::SetSensitiveRequestHeadersLayer;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

/// Type alias for route handler pairs
type RouteHandlerPair = (crate::Route, Arc<dyn Handler>);

/// Wrapper that runs request/parameter validation before calling the user handler.
struct ValidatingHandler {
    inner: Arc<dyn Handler>,
    request_validator: Option<Arc<SchemaValidator>>,
    parameter_validator: Option<ParameterValidator>,
}

impl ValidatingHandler {
    fn new(inner: Arc<dyn Handler>, route: &crate::Route) -> Self {
        Self {
            inner,
            request_validator: route.request_validator.clone(),
            parameter_validator: route.parameter_validator.clone(),
        }
    }
}

impl Handler for ValidatingHandler {
    fn call(
        &self,
        req: axum::http::Request<Body>,
        request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        let inner = self.inner.clone();
        let request_validator = self.request_validator.clone();
        let parameter_validator = self.parameter_validator.clone();

        Box::pin(async move {
            if let Some(validator) = request_validator
                && let Err(errors) = validator.validate(&request_data.body)
            {
                let problem = ProblemDetails::from_validation_error(&errors);
                let body = problem.to_json().unwrap_or_else(|_| "{}".to_string());
                return Err((problem.status_code(), body));
            }

            if let Some(validator) = parameter_validator {
                let raw_query_strings: HashMap<String, String> = request_data
                    .raw_query_params
                    .iter()
                    .filter_map(|(k, v)| v.first().map(|value| (k.clone(), value.clone())))
                    .collect();

                if let Err(errors) = validator.validate_and_extract(
                    &request_data.query_params,
                    &raw_query_strings,
                    &request_data.path_params,
                    &request_data.headers,
                    &request_data.cookies,
                ) {
                    let problem = ProblemDetails::from_validation_error(&errors);
                    let body = problem.to_json().unwrap_or_else(|_| "{}".to_string());
                    return Err((problem.status_code(), body));
                }
            }

            inner.call(req, request_data).await
        })
    }
}

/// Request ID generator using UUIDs
#[derive(Clone, Default)]
struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _request: &axum::http::Request<B>) -> Option<RequestId> {
        let id = Uuid::new_v4().to_string().parse().ok()?;
        Some(RequestId::new(id))
    }
}

/// Graceful shutdown signal handler
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received SIGINT (Ctrl+C), starting graceful shutdown");
        },
        _ = terminate => {
            tracing::info!("Received SIGTERM, starting graceful shutdown");
        },
    }
}

/// Extract and parse query parameters from request URI
fn extract_query_params(uri: &axum::http::Uri) -> Value {
    let query_string = uri.query().unwrap_or("");
    if query_string.is_empty() {
        Value::Object(serde_json::Map::new())
    } else {
        // parse_numbers=true: Auto-convert numeric strings to numbers (e.g., "123" → 123)
        // This is essential for array parameters like ?device_ids=1&device_ids=2 → [1, 2]
        parse_query_string_to_json(query_string.as_bytes(), true)
    }
}

/// Extract raw query parameters as strings (no type conversion)
/// Used for validation error messages to show the actual input values
fn extract_raw_query_params(uri: &axum::http::Uri) -> HashMap<String, Vec<String>> {
    let query_string = uri.query().unwrap_or("");
    if query_string.is_empty() {
        HashMap::new()
    } else {
        // Parse without number conversion to get raw string values
        // Collect all values for each key (supports repeated params like ?a=1&a=2)
        crate::query_parser::parse_query_string(query_string.as_bytes(), '&')
            .into_iter()
            .fold(HashMap::new(), |mut acc, (k, v)| {
                acc.entry(k).or_insert_with(Vec::new).push(v);
                acc
            })
    }
}

/// Extract headers from request
fn extract_headers(headers: &axum::http::HeaderMap) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for (name, value) in headers.iter() {
        if let Ok(val_str) = value.to_str() {
            // Convert header name to lowercase for consistent access
            map.insert(name.as_str().to_lowercase(), val_str.to_string());
        }
    }
    map
}

/// Extract cookies from request headers
fn extract_cookies(headers: &axum::http::HeaderMap) -> HashMap<String, String> {
    let mut cookies = HashMap::new();

    // Look for Cookie header
    if let Some(cookie_str) = headers.get(axum::http::header::COOKIE).and_then(|h| h.to_str().ok()) {
        // Parse cookies using the cookie crate for RFC 6265 compliance and proper percent-decoding
        for cookie in cookie::Cookie::split_parse(cookie_str).flatten() {
            cookies.insert(cookie.name().to_string(), cookie.value().to_string());
        }
    }

    cookies
}

/// Create RequestData from request parts (for requests without body)
///
/// Wraps HashMaps in Arc to enable cheap cloning without duplicating data.
fn create_request_data_without_body(
    uri: &axum::http::Uri,
    method: &axum::http::Method,
    headers: &axum::http::HeaderMap,
    path_params: HashMap<String, String>,
) -> RequestData {
    RequestData {
        path_params: Arc::new(path_params),
        query_params: extract_query_params(uri),
        raw_query_params: Arc::new(extract_raw_query_params(uri)),
        headers: Arc::new(extract_headers(headers)),
        cookies: Arc::new(extract_cookies(headers)),
        body: Value::Null,
        method: method.as_str().to_string(),
        path: uri.path().to_string(),
    }
}

/// Create RequestData from request parts (for requests with body)
///
/// Wraps HashMaps in Arc to enable cheap cloning without duplicating data.
async fn create_request_data_with_body(
    parts: &axum::http::request::Parts,
    path_params: HashMap<String, String>,
    body: Body,
) -> Result<RequestData, (axum::http::StatusCode, String)> {
    let body_bytes = body
        .collect()
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::BAD_REQUEST,
                format!("Failed to read body: {}", e),
            )
        })?
        .to_bytes();

    // Note: Content-Type and Content-Length validation is handled by middleware

    let body_value = if !body_bytes.is_empty() {
        serde_json::from_slice::<Value>(&body_bytes)
            .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, format!("Invalid JSON: {}", e)))?
    } else {
        Value::Null
    };

    Ok(RequestData {
        path_params: Arc::new(path_params),
        query_params: extract_query_params(&parts.uri),
        raw_query_params: Arc::new(extract_raw_query_params(&parts.uri)),
        headers: Arc::new(extract_headers(&parts.headers)),
        cookies: Arc::new(extract_cookies(&parts.headers)),
        body: body_value,
        method: parts.method.as_str().to_string(),
        path: parts.uri.path().to_string(),
    })
}

/// Execute a handler with lifecycle hooks
///
/// This wraps the handler execution with lifecycle hooks at appropriate points:
/// 1. preValidation hooks (before handler, which does validation)
/// 2. preHandler hooks (after validation, before handler)
/// 3. Handler execution
/// 4. onResponse hooks (after successful handler execution)
/// 5. onError hooks (if handler or any hook fails)
async fn execute_with_lifecycle_hooks(
    req: axum::http::Request<Body>,
    request_data: RequestData,
    handler: Arc<dyn Handler>,
    hooks: Option<Arc<crate::LifecycleHooks>>,
) -> Result<axum::http::Response<Body>, (axum::http::StatusCode, String)> {
    use crate::lifecycle::HookResult;
    use axum::http::StatusCode;

    // If no hooks registered, fast path
    let Some(hooks) = hooks else {
        return handler.call(req, request_data).await;
    };

    // Fast path: if hooks are empty, skip hook execution
    if hooks.is_empty() {
        return handler.call(req, request_data).await;
    }

    // 1. preValidation hooks (before validation)
    let req = match hooks.execute_pre_validation(req).await {
        Ok(HookResult::Continue(r)) => r,
        Ok(HookResult::ShortCircuit(response)) => return Ok(response),
        Err(e) => {
            let error_response = axum::http::Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!(
                    "{{\"error\":\"preValidation hook failed: {}\"}}",
                    e
                )))
                .unwrap();

            // Execute onError hooks
            return match hooks.execute_on_error(error_response).await {
                Ok(resp) => Ok(resp),
                Err(_) => Ok(axum::http::Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("{\"error\":\"Hook execution failed\"}"))
                    .unwrap()),
            };
        }
    };

    // 2. preHandler hooks (after validation, before handler)
    // Note: Validation happens inside handler.call()
    let req = match hooks.execute_pre_handler(req).await {
        Ok(HookResult::Continue(r)) => r,
        Ok(HookResult::ShortCircuit(response)) => return Ok(response),
        Err(e) => {
            let error_response = axum::http::Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!("{{\"error\":\"preHandler hook failed: {}\"}}", e)))
                .unwrap();

            // Execute onError hooks
            return match hooks.execute_on_error(error_response).await {
                Ok(resp) => Ok(resp),
                Err(_) => Ok(axum::http::Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("{\"error\":\"Hook execution failed\"}"))
                    .unwrap()),
            };
        }
    };

    // 3. Execute handler
    let response = match handler.call(req, request_data).await {
        Ok(resp) => resp,
        Err((status, message)) => {
            // Handler failed - create error response and run onError hooks
            let error_response = axum::http::Response::builder()
                .status(status)
                .body(Body::from(message))
                .unwrap();

            return match hooks.execute_on_error(error_response).await {
                Ok(resp) => Ok(resp),
                Err(e) => Ok(axum::http::Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(format!("{{\"error\":\"onError hook failed: {}\"}}", e)))
                    .unwrap()),
            };
        }
    };

    // 4. onResponse hooks (after successful handler execution)
    match hooks.execute_on_response(response).await {
        Ok(resp) => Ok(resp),
        Err(e) => Ok(axum::http::Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(format!("{{\"error\":\"onResponse hook failed: {}\"}}", e)))
            .unwrap()),
    }
}

/// Build an Axum router from routes and foreign handlers
pub fn build_router_with_handlers(
    routes: Vec<(crate::Route, Arc<dyn Handler>)>,
    hooks: Option<Arc<crate::LifecycleHooks>>,
) -> Result<AxumRouter, String> {
    let mut app = AxumRouter::new();

    // Build route registry for middleware lookup
    let mut registry = HashMap::new();
    for (route, _) in &routes {
        let axum_path = crate::type_hints::strip_type_hints(&route.path);
        let axum_path = if axum_path.starts_with('/') {
            axum_path
        } else {
            format!("/{}", axum_path)
        };
        registry.insert(
            (route.method.as_str().to_string(), axum_path),
            crate::middleware::RouteInfo {
                expects_json_body: route.expects_json_body,
            },
        );
    }
    let route_registry: crate::middleware::RouteRegistry = Arc::new(registry);

    // Group routes by path to support multiple methods on same route
    let mut routes_by_path: HashMap<String, Vec<RouteHandlerPair>> = HashMap::new();
    for (route, handler) in routes {
        routes_by_path
            .entry(route.path.clone())
            .or_default()
            .push((route, handler));
    }

    let mut sorted_paths: Vec<String> = routes_by_path.keys().cloned().collect();
    sorted_paths.sort();

    for path in sorted_paths {
        let route_handlers = routes_by_path
            .remove(&path)
            .ok_or_else(|| format!("Missing handlers for path '{}'", path))?;

        let mut handlers_by_method: HashMap<crate::Method, (crate::Route, Arc<dyn Handler>)> = HashMap::new();
        for (route, handler) in route_handlers {
            let validating_handler = Arc::new(ValidatingHandler::new(handler, &route));
            handlers_by_method.insert(route.method.clone(), (route, validating_handler));
        }

        // Check if any route on this path has CORS config
        let cors_config: Option<CorsConfig> = handlers_by_method
            .values()
            .find_map(|(route, _)| route.cors.as_ref())
            .cloned();

        // Check if there's an explicit OPTIONS handler
        let has_options_handler = handlers_by_method.keys().any(|m| m.as_str() == "OPTIONS");

        let mut combined_router: Option<MethodRouter> = None;
        let has_path_params = path.contains('{');

        for (_method, (route, handler)) in handlers_by_method {
            let method_router: MethodRouter = match route.method.as_str() {
                "GET" => {
                    if has_path_params {
                        let handler = handler.clone();
                        let hooks = hooks.clone();
                        axum::routing::get(
                            move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                                let request_data = create_request_data_without_body(
                                    req.uri(),
                                    req.method(),
                                    req.headers(),
                                    path_params.0,
                                );
                                execute_with_lifecycle_hooks(req, request_data, handler, hooks).await
                            },
                        )
                    } else {
                        let handler = handler.clone();
                        let hooks = hooks.clone();
                        axum::routing::get(move |req: axum::extract::Request| async move {
                            let request_data = create_request_data_without_body(
                                req.uri(),
                                req.method(),
                                req.headers(),
                                HashMap::new(),
                            );
                            execute_with_lifecycle_hooks(req, request_data, handler, hooks).await
                        })
                    }
                }
                "DELETE" => {
                    if has_path_params {
                        let handler = handler.clone();
                        let hooks = hooks.clone();
                        axum::routing::delete(
                            move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                                let request_data = create_request_data_without_body(
                                    req.uri(),
                                    req.method(),
                                    req.headers(),
                                    path_params.0,
                                );
                                execute_with_lifecycle_hooks(req, request_data, handler, hooks).await
                            },
                        )
                    } else {
                        let handler = handler.clone();
                        let hooks = hooks.clone();
                        axum::routing::delete(move |req: axum::extract::Request| async move {
                            let request_data = create_request_data_without_body(
                                req.uri(),
                                req.method(),
                                req.headers(),
                                HashMap::new(),
                            );
                            execute_with_lifecycle_hooks(req, request_data, handler, hooks).await
                        })
                    }
                }
                "HEAD" => {
                    if has_path_params {
                        let handler = handler.clone();
                        let hooks = hooks.clone();
                        axum::routing::head(
                            move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                                let request_data = create_request_data_without_body(
                                    req.uri(),
                                    req.method(),
                                    req.headers(),
                                    path_params.0,
                                );
                                execute_with_lifecycle_hooks(req, request_data, handler, hooks).await
                            },
                        )
                    } else {
                        let handler = handler.clone();
                        let hooks = hooks.clone();
                        axum::routing::head(move |req: axum::extract::Request| async move {
                            let request_data = create_request_data_without_body(
                                req.uri(),
                                req.method(),
                                req.headers(),
                                HashMap::new(),
                            );
                            execute_with_lifecycle_hooks(req, request_data, handler, hooks).await
                        })
                    }
                }
                "OPTIONS" => {
                    // If this route has CORS config, use CORS preflight logic instead of handler
                    if let Some(ref cors_cfg) = route.cors {
                        let cors_config = cors_cfg.clone();
                        axum::routing::options(move |req: axum::extract::Request| async move {
                            crate::cors::handle_preflight(req.headers(), &cors_config).map_err(|e| *e)
                        })
                    } else if has_path_params {
                        let handler = handler.clone();
                        let hooks = hooks.clone();
                        axum::routing::options(
                            move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                                let request_data = create_request_data_without_body(
                                    req.uri(),
                                    req.method(),
                                    req.headers(),
                                    path_params.0,
                                );
                                execute_with_lifecycle_hooks(req, request_data, handler, hooks).await
                            },
                        )
                    } else {
                        let handler = handler.clone();
                        let hooks = hooks.clone();
                        axum::routing::options(move |req: axum::extract::Request| async move {
                            let request_data = create_request_data_without_body(
                                req.uri(),
                                req.method(),
                                req.headers(),
                                HashMap::new(),
                            );
                            execute_with_lifecycle_hooks(req, request_data, handler, hooks).await
                        })
                    }
                }
                "TRACE" => {
                    if has_path_params {
                        let handler = handler.clone();
                        let hooks = hooks.clone();
                        axum::routing::trace(
                            move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                                let request_data = create_request_data_without_body(
                                    req.uri(),
                                    req.method(),
                                    req.headers(),
                                    path_params.0,
                                );
                                execute_with_lifecycle_hooks(req, request_data, handler, hooks).await
                            },
                        )
                    } else {
                        let handler = handler.clone();
                        let hooks = hooks.clone();
                        axum::routing::trace(move |req: axum::extract::Request| async move {
                            let request_data = create_request_data_without_body(
                                req.uri(),
                                req.method(),
                                req.headers(),
                                HashMap::new(),
                            );
                            execute_with_lifecycle_hooks(req, request_data, handler, hooks).await
                        })
                    }
                }
                "POST" => {
                    if has_path_params {
                        let handler = handler.clone();
                        let hooks = hooks.clone();
                        axum::routing::post(
                            move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                                let (parts, body) = req.into_parts();
                                let request_data = create_request_data_with_body(&parts, path_params.0, body).await?;
                                let req = axum::extract::Request::from_parts(parts, Body::empty());
                                execute_with_lifecycle_hooks(req, request_data, handler, hooks).await
                            },
                        )
                    } else {
                        let handler = handler.clone();
                        let hooks = hooks.clone();
                        axum::routing::post(move |req: axum::extract::Request| async move {
                            let (parts, body) = req.into_parts();
                            let request_data = create_request_data_with_body(&parts, HashMap::new(), body).await?;
                            let req = axum::extract::Request::from_parts(parts, Body::empty());
                            execute_with_lifecycle_hooks(req, request_data, handler, hooks).await
                        })
                    }
                }
                "PUT" => {
                    if has_path_params {
                        let handler = handler.clone();
                        let hooks = hooks.clone();
                        axum::routing::put(
                            move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                                let (parts, body) = req.into_parts();
                                let request_data = create_request_data_with_body(&parts, path_params.0, body).await?;
                                let req = axum::extract::Request::from_parts(parts, Body::empty());
                                execute_with_lifecycle_hooks(req, request_data, handler, hooks).await
                            },
                        )
                    } else {
                        let handler = handler.clone();
                        let hooks = hooks.clone();
                        axum::routing::put(move |req: axum::extract::Request| async move {
                            let (parts, body) = req.into_parts();
                            let request_data = create_request_data_with_body(&parts, HashMap::new(), body).await?;
                            let req = axum::extract::Request::from_parts(parts, Body::empty());
                            execute_with_lifecycle_hooks(req, request_data, handler, hooks).await
                        })
                    }
                }
                "PATCH" => {
                    if has_path_params {
                        let handler = handler.clone();
                        let hooks = hooks.clone();
                        axum::routing::patch(
                            move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                                let (parts, body) = req.into_parts();
                                let request_data = create_request_data_with_body(&parts, path_params.0, body).await?;
                                let req = axum::extract::Request::from_parts(parts, Body::empty());
                                execute_with_lifecycle_hooks(req, request_data, handler, hooks).await
                            },
                        )
                    } else {
                        let handler = handler.clone();
                        let hooks = hooks.clone();
                        axum::routing::patch(move |req: axum::extract::Request| async move {
                            let (parts, body) = req.into_parts();
                            let request_data = create_request_data_with_body(&parts, HashMap::new(), body).await?;
                            let req = axum::extract::Request::from_parts(parts, Body::empty());
                            execute_with_lifecycle_hooks(req, request_data, handler, hooks).await
                        })
                    }
                }
                _ => return Err(format!("Unsupported HTTP method: {}", route.method.as_str())),
            };

            combined_router = Some(match combined_router {
                None => method_router,
                Some(existing) => existing.merge(method_router),
            });

            tracing::info!("Registered route: {} {}", route.method.as_str(), path);
        }

        // If CORS config exists but no explicit OPTIONS handler, auto-generate one
        if let Some(ref cors_cfg) = cors_config
            && !has_options_handler
        {
            let cors_config_clone: CorsConfig = cors_cfg.clone();
            let options_router = axum::routing::options(move |req: axum::extract::Request| async move {
                crate::cors::handle_preflight(req.headers(), &cors_config_clone).map_err(|e| *e)
            });

            combined_router = Some(match combined_router {
                None => options_router,
                Some(existing) => existing.merge(options_router),
            });

            tracing::info!("Auto-generated OPTIONS handler for CORS preflight: {}", path);
        }

        if let Some(router) = combined_router {
            let mut axum_path = crate::type_hints::strip_type_hints(&path);
            // Ensure path starts with / for Axum compatibility
            if !axum_path.starts_with('/') {
                axum_path = format!("/{}", axum_path);
            }
            app = app.route(&axum_path, router);
        }
    }

    app = app.layer(axum::middleware::from_fn(
        crate::middleware::validate_content_type_middleware,
    ));
    app = app.layer(TraceLayer::new_for_http());

    // Inject route registry as extension for middleware lookup
    // This must be added AFTER middleware so it runs FIRST (layers run in reverse order)
    app = app.layer(axum::Extension(route_registry));

    Ok(app)
}

/// Build router with handlers and apply middleware based on config
pub fn build_router_with_handlers_and_config(
    routes: Vec<RouteHandlerPair>,
    config: ServerConfig,
    route_metadata: Vec<crate::RouteMetadata>,
) -> Result<AxumRouter, String> {
    // Extract lifecycle hooks from config
    let hooks = config.lifecycle_hooks.clone().map(Arc::new);

    // Start with the basic router
    let mut app = build_router_with_handlers(routes, hooks)?;

    // Apply middleware layers directly (they're applied in reverse order)
    // Layer order: last added = first executed

    // 1. Sensitive headers (hide auth tokens from logs)
    app = app.layer(SetSensitiveRequestHeadersLayer::new([
        axum::http::header::AUTHORIZATION,
        axum::http::header::COOKIE,
    ]));

    // 2. Compression (should compress final responses)
    if let Some(ref compression) = config.compression {
        let mut compression_layer = CompressionLayer::new();
        if !compression.gzip {
            compression_layer = compression_layer.gzip(false);
        }
        if !compression.brotli {
            compression_layer = compression_layer.br(false);
        }

        // Respect configurable minimum compression size while preserving default predicate behavior.
        let min_threshold = compression.min_size.min(u16::MAX as usize) as u16;
        let predicate = SizeAbove::new(min_threshold)
            .and(NotForContentType::GRPC)
            .and(NotForContentType::IMAGES)
            .and(NotForContentType::SSE);
        let compression_layer = compression_layer.compress_when(predicate);

        app = app.layer(compression_layer);
    }

    // 3. Rate limiting (before other processing to reject early)
    if let Some(ref rate_limit) = config.rate_limit {
        if rate_limit.ip_based {
            let governor_conf = Arc::new(
                GovernorConfigBuilder::default()
                    .per_second(rate_limit.per_second)
                    .burst_size(rate_limit.burst)
                    .finish()
                    .ok_or_else(|| "Failed to create rate limiter".to_string())?,
            );
            app = app.layer(tower_governor::GovernorLayer::new(governor_conf));
        } else {
            let governor_conf = Arc::new(
                GovernorConfigBuilder::default()
                    .per_second(rate_limit.per_second)
                    .burst_size(rate_limit.burst)
                    .key_extractor(GlobalKeyExtractor)
                    .finish()
                    .ok_or_else(|| "Failed to create rate limiter".to_string())?,
            );
            app = app.layer(tower_governor::GovernorLayer::new(governor_conf));
        }
    }

    // 3a. JWT authentication (after rate limiting, before business logic)
    if let Some(ref jwt_config) = config.jwt_auth {
        let jwt_config_clone = jwt_config.clone();
        app = app.layer(axum::middleware::from_fn(move |headers, req, next| {
            crate::auth::jwt_auth_middleware(jwt_config_clone.clone(), headers, req, next)
        }));
    }

    // 3b. API key authentication (after rate limiting, before business logic)
    if let Some(ref api_key_config) = config.api_key_auth {
        let api_key_config_clone = api_key_config.clone();
        app = app.layer(axum::middleware::from_fn(move |headers, req, next| {
            crate::auth::api_key_auth_middleware(api_key_config_clone.clone(), headers, req, next)
        }));
    }

    // 4. Timeout layer (should wrap everything except request ID)
    if let Some(timeout_secs) = config.request_timeout {
        app = app.layer(TimeoutLayer::new(Duration::from_secs(timeout_secs)));
    }

    // 5. Request ID (outermost - should be first to execute)
    if config.enable_request_id {
        app = app
            .layer(PropagateRequestIdLayer::x_request_id())
            .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid));
    }

    // 6. Body size limit (applied directly as it's not a tower::Layer)
    if let Some(max_size) = config.max_body_size {
        app = app.layer(DefaultBodyLimit::max(max_size));
    } else {
        // Disable body limit if None (not recommended for production)
        app = app.layer(DefaultBodyLimit::disable());
    }

    // 7. Add static file serving routes
    for static_config in &config.static_files {
        let mut serve_dir = ServeDir::new(&static_config.directory);
        if static_config.index_file {
            serve_dir = serve_dir.append_index_html_on_directories(true);
        }

        let mut static_router = AxumRouter::new().fallback_service(serve_dir);
        if let Some(ref cache_control) = static_config.cache_control {
            let header_value = axum::http::HeaderValue::from_str(cache_control)
                .map_err(|e| format!("Invalid cache-control header: {}", e))?;
            static_router = static_router.layer(SetResponseHeaderLayer::overriding(
                axum::http::header::CACHE_CONTROL,
                header_value,
            ));
        }

        app = app.nest_service(&static_config.route_prefix, static_router);

        tracing::info!(
            "Serving static files from '{}' at '{}'",
            static_config.directory,
            static_config.route_prefix
        );
    }

    // 8. Add OpenAPI documentation routes (without authentication)
    if let Some(ref openapi_config) = config.openapi
        && openapi_config.enabled
    {
        use axum::response::{Html, Json};

        // Generate OpenAPI spec from routes with auto-detected security schemes
        let schema_registry = crate::SchemaRegistry::new();
        let openapi_spec =
            crate::openapi::generate_openapi_spec(&route_metadata, openapi_config, &schema_registry, Some(&config))
                .map_err(|e| format!("Failed to generate OpenAPI spec: {}", e))?;

        // Serialize to JSON once
        let spec_json =
            serde_json::to_string(&openapi_spec).map_err(|e| format!("Failed to serialize OpenAPI spec: {}", e))?;
        let spec_value = serde_json::from_str::<serde_json::Value>(&spec_json)
            .map_err(|e| format!("Failed to parse OpenAPI spec: {}", e))?;

        // OpenAPI JSON endpoint
        let openapi_json_path = openapi_config.openapi_json_path.clone();
        app = app.route(&openapi_json_path, get(move || async move { Json(spec_value) }));

        // Swagger UI endpoint
        let swagger_html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Swagger UI</title>
    <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist/swagger-ui.css">
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist/swagger-ui-bundle.js"></script>
    <script>
        SwaggerUIBundle({{
            url: '{}',
            dom_id: '#swagger-ui',
        }});
    </script>
</body>
</html>"#,
            openapi_json_path
        );
        let swagger_ui_path = openapi_config.swagger_ui_path.clone();
        app = app.route(&swagger_ui_path, get(move || async move { Html(swagger_html) }));

        // Redoc endpoint
        let redoc_html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Redoc</title>
</head>
<body>
    <redoc spec-url='{}'></redoc>
    <script src="https://cdn.redoc.ly/redoc/latest/bundles/redoc.standalone.js"></script>
</body>
</html>"#,
            openapi_json_path
        );
        let redoc_path = openapi_config.redoc_path.clone();
        app = app.route(&redoc_path, get(move || async move { Html(redoc_html) }));

        tracing::info!("OpenAPI documentation enabled at {}", openapi_json_path);
    }

    Ok(app)
}

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
    ///
    /// Build router with trait-based handlers
    /// Routes are grouped by path before registration to support multiple HTTP methods
    /// for the same path (e.g., GET /data and POST /data). Axum requires that all methods
    /// for a path be merged into a single MethodRouter before calling `.route()`.
    pub fn with_handlers(
        config: ServerConfig,
        routes: Vec<(crate::Route, Arc<dyn Handler>)>,
    ) -> Result<AxumRouter, String> {
        // Extract metadata from routes for backward compatibility
        let metadata: Vec<crate::RouteMetadata> = routes
            .iter()
            .map(|(route, _)| crate::RouteMetadata {
                method: route.method.to_string(),
                path: route.path.clone(),
                handler_name: route.handler_name.clone(),
                request_schema: None,   // Lost in compilation
                response_schema: None,  // Lost in compilation
                parameter_schema: None, // Lost in compilation
                file_params: route.file_params.clone(),
                is_async: route.is_async,
                cors: route.cors.clone(),
            })
            .collect();
        build_router_with_handlers_and_config(routes, config, metadata)
    }

    /// Create a new server with Python handlers and metadata for OpenAPI
    pub fn with_handlers_and_metadata(
        config: ServerConfig,
        routes: Vec<(crate::Route, Arc<dyn Handler>)>,
        metadata: Vec<crate::RouteMetadata>,
    ) -> Result<AxumRouter, String> {
        build_router_with_handlers_and_config(routes, config, metadata)
    }

    /// Run the server with the Axum router and config
    pub async fn run_with_config(app: AxumRouter, config: ServerConfig) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("{}:{}", config.host, config.port);
        let socket_addr: SocketAddr = addr.parse()?;
        let listener = TcpListener::bind(socket_addr).await?;

        tracing::info!("Listening on http://{}", socket_addr);

        if config.graceful_shutdown {
            axum::serve(listener, app)
                .with_graceful_shutdown(shutdown_signal())
                .await?;
        } else {
            axum::serve(listener, app).await?;
        }

        Ok(())
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
        tracing::info!("Starting server with {} routes", self.router.route_count());

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
        let _server = Server::new(config, router);

        // Test passes if we get here without panic
    }
}

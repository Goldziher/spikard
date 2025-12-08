//! HTTP server implementation using Tokio and Axum
//!
//! This module provides the main server builder and routing infrastructure, with
//! focused submodules for handler validation, request extraction, and lifecycle execution.

pub mod handler;
pub mod lifecycle_execution;
pub mod request_extraction;

use crate::handler_trait::Handler;
use crate::{CorsConfig, Router, ServerConfig};
use axum::Router as AxumRouter;
use axum::body::Body;
use axum::extract::{DefaultBodyLimit, Path};
use axum::http::StatusCode;
use axum::routing::{MethodRouter, get};
use spikard_core::type_hints;
use std::collections::HashMap;
use std::net::SocketAddr;
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

/// Extract required dependencies from route metadata
///
/// Placeholder implementation until routes can declare dependencies via metadata.
#[cfg(feature = "di")]
fn extract_handler_dependencies(route: &crate::Route) -> Vec<String> {
    route.handler_dependencies.clone()
}

/// Determines if a method typically has a request body
fn method_expects_body(method: &str) -> bool {
    matches!(method, "POST" | "PUT" | "PATCH")
}

/// Creates a method router for the given HTTP method
/// Handles both path parameters and non-path variants
fn create_method_router(
    method: &str,
    has_path_params: bool,
    handler: Arc<dyn Handler>,
    hooks: Option<Arc<crate::LifecycleHooks>>,
) -> axum::routing::MethodRouter {
    let expects_body = method_expects_body(method);

    if expects_body {
        // POST, PUT, PATCH - need to handle body
        if has_path_params {
            let handler_clone = handler.clone();
            let hooks_clone = hooks.clone();
            match method {
                "POST" => axum::routing::post(
                    move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                        let (parts, body) = req.into_parts();
                        let request_data =
                            request_extraction::create_request_data_with_body(&parts, path_params.0, body).await?;
                        let req = axum::extract::Request::from_parts(parts, Body::empty());
                        lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler_clone, hooks_clone)
                            .await
                    },
                ),
                "PUT" => axum::routing::put(
                    move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                        let (parts, body) = req.into_parts();
                        let request_data =
                            request_extraction::create_request_data_with_body(&parts, path_params.0, body).await?;
                        let req = axum::extract::Request::from_parts(parts, Body::empty());
                        lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler_clone, hooks_clone)
                            .await
                    },
                ),
                "PATCH" => axum::routing::patch(
                    move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                        let (parts, body) = req.into_parts();
                        let request_data =
                            request_extraction::create_request_data_with_body(&parts, path_params.0, body).await?;
                        let req = axum::extract::Request::from_parts(parts, Body::empty());
                        lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler_clone, hooks_clone)
                            .await
                    },
                ),
                _ => {
                    eprintln!(
                        "[spikard-router] unsupported HTTP method with path params: {} (defaulting to 405)",
                        method
                    );
                    MethodRouter::new()
                }
            }
        } else {
            let handler_clone = handler.clone();
            let hooks_clone = hooks.clone();
            match method {
                "POST" => axum::routing::post(move |req: axum::extract::Request| async move {
                    let (parts, body) = req.into_parts();
                    let request_data =
                        request_extraction::create_request_data_with_body(&parts, HashMap::new(), body).await?;
                    let req = axum::extract::Request::from_parts(parts, Body::empty());
                    lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler_clone, hooks_clone)
                        .await
                }),
                "PUT" => axum::routing::put(move |req: axum::extract::Request| async move {
                    let (parts, body) = req.into_parts();
                    let request_data =
                        request_extraction::create_request_data_with_body(&parts, HashMap::new(), body).await?;
                    let req = axum::extract::Request::from_parts(parts, Body::empty());
                    lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler_clone, hooks_clone)
                        .await
                }),
                "PATCH" => axum::routing::patch(move |req: axum::extract::Request| async move {
                    let (parts, body) = req.into_parts();
                    let request_data =
                        request_extraction::create_request_data_with_body(&parts, HashMap::new(), body).await?;
                    let req = axum::extract::Request::from_parts(parts, Body::empty());
                    lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler_clone, hooks_clone)
                        .await
                }),
                _ => {
                    eprintln!(
                        "[spikard-router] unsupported HTTP method without path params: {} (defaulting to 405)",
                        method
                    );
                    MethodRouter::new()
                }
            }
        }
    } else {
        // GET, DELETE, HEAD, TRACE - no body handling
        if has_path_params {
            let handler_clone = handler.clone();
            let hooks_clone = hooks.clone();
            match method {
                "GET" => axum::routing::get(
                    move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                        let request_data = request_extraction::create_request_data_without_body(
                            req.uri(),
                            req.method(),
                            req.headers(),
                            path_params.0,
                        );
                        lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler_clone, hooks_clone)
                            .await
                    },
                ),
                "DELETE" => axum::routing::delete(
                    move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                        let request_data = request_extraction::create_request_data_without_body(
                            req.uri(),
                            req.method(),
                            req.headers(),
                            path_params.0,
                        );
                        lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler_clone, hooks_clone)
                            .await
                    },
                ),
                "HEAD" => axum::routing::head(
                    move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                        let request_data = request_extraction::create_request_data_without_body(
                            req.uri(),
                            req.method(),
                            req.headers(),
                            path_params.0,
                        );
                        lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler_clone, hooks_clone)
                            .await
                    },
                ),
                "TRACE" => axum::routing::trace(
                    move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                        let request_data = request_extraction::create_request_data_without_body(
                            req.uri(),
                            req.method(),
                            req.headers(),
                            path_params.0,
                        );
                        lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler_clone, hooks_clone)
                            .await
                    },
                ),
                "OPTIONS" => axum::routing::options(
                    move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                        let request_data = request_extraction::create_request_data_without_body(
                            req.uri(),
                            req.method(),
                            req.headers(),
                            path_params.0,
                        );
                        lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler_clone, hooks_clone)
                            .await
                    },
                ),
                _ => {
                    eprintln!(
                        "[spikard-router] unsupported HTTP method with path params: {} (defaulting to 405)",
                        method
                    );
                    MethodRouter::new()
                }
            }
        } else {
            let handler_clone = handler.clone();
            let hooks_clone = hooks.clone();
            match method {
                "GET" => axum::routing::get(move |req: axum::extract::Request| async move {
                    let request_data = request_extraction::create_request_data_without_body(
                        req.uri(),
                        req.method(),
                        req.headers(),
                        HashMap::new(),
                    );
                    lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler_clone, hooks_clone)
                        .await
                }),
                "DELETE" => axum::routing::delete(move |req: axum::extract::Request| async move {
                    let request_data = request_extraction::create_request_data_without_body(
                        req.uri(),
                        req.method(),
                        req.headers(),
                        HashMap::new(),
                    );
                    lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler_clone, hooks_clone)
                        .await
                }),
                "HEAD" => axum::routing::head(move |req: axum::extract::Request| async move {
                    let request_data = request_extraction::create_request_data_without_body(
                        req.uri(),
                        req.method(),
                        req.headers(),
                        HashMap::new(),
                    );
                    lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler_clone, hooks_clone)
                        .await
                }),
                "TRACE" => axum::routing::trace(move |req: axum::extract::Request| async move {
                    let request_data = request_extraction::create_request_data_without_body(
                        req.uri(),
                        req.method(),
                        req.headers(),
                        HashMap::new(),
                    );
                    lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler_clone, hooks_clone)
                        .await
                }),
                "OPTIONS" => axum::routing::options(move |req: axum::extract::Request| async move {
                    let request_data = request_extraction::create_request_data_without_body(
                        req.uri(),
                        req.method(),
                        req.headers(),
                        HashMap::new(),
                    );
                    lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler_clone, hooks_clone)
                        .await
                }),
                _ => {
                    eprintln!(
                        "[spikard-router] unsupported HTTP method without path params: {} (defaulting to 405)",
                        method
                    );
                    MethodRouter::new()
                }
            }
        }
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

/// Build an Axum router from routes and foreign handlers
#[cfg(not(feature = "di"))]
pub fn build_router_with_handlers(
    routes: Vec<(crate::Route, Arc<dyn Handler>)>,
    hooks: Option<Arc<crate::LifecycleHooks>>,
) -> Result<AxumRouter, String> {
    build_router_with_handlers_inner(routes, hooks, None)
}

/// Build an Axum router from routes and foreign handlers with optional DI container
#[cfg(feature = "di")]
pub fn build_router_with_handlers(
    routes: Vec<(crate::Route, Arc<dyn Handler>)>,
    hooks: Option<Arc<crate::LifecycleHooks>>,
    di_container: Option<Arc<spikard_core::di::DependencyContainer>>,
) -> Result<AxumRouter, String> {
    build_router_with_handlers_inner(routes, hooks, di_container)
}

fn build_router_with_handlers_inner(
    routes: Vec<(crate::Route, Arc<dyn Handler>)>,
    hooks: Option<Arc<crate::LifecycleHooks>>,
    #[cfg(feature = "di")] di_container: Option<Arc<spikard_core::di::DependencyContainer>>,
    #[cfg(not(feature = "di"))] _di_container: Option<()>,
) -> Result<AxumRouter, String> {
    let mut app = AxumRouter::new();

    let mut registry = HashMap::new();
    for (route, _) in &routes {
        let axum_path = type_hints::strip_type_hints(&route.path);
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
            #[cfg(feature = "di")]
            let handler = if let Some(ref container) = di_container {
                let mut required_deps = extract_handler_dependencies(&route);
                if required_deps.is_empty() {
                    required_deps = container.keys();
                }

                if !required_deps.is_empty() {
                    Arc::new(crate::di_handler::DependencyInjectingHandler::new(
                        handler,
                        Arc::clone(container),
                        required_deps,
                    )) as Arc<dyn Handler>
                } else {
                    handler
                }
            } else {
                handler
            };

            let validating_handler = Arc::new(handler::ValidatingHandler::new(handler, &route));
            handlers_by_method.insert(route.method.clone(), (route, validating_handler));
        }

        let cors_config: Option<CorsConfig> = handlers_by_method
            .values()
            .find_map(|(route, _)| route.cors.as_ref())
            .cloned();

        let has_options_handler = handlers_by_method.keys().any(|m| m.as_str() == "OPTIONS");

        let mut combined_router: Option<MethodRouter> = None;
        let has_path_params = path.contains('{');

        for (_method, (route, handler)) in handlers_by_method {
            let method_router: MethodRouter = match route.method.as_str() {
                "OPTIONS" => {
                    if let Some(ref cors_cfg) = route.cors {
                        let cors_config = cors_cfg.clone();
                        axum::routing::options(move |req: axum::extract::Request| async move {
                            crate::cors::handle_preflight(req.headers(), &cors_config).map_err(|e| *e)
                        })
                    } else {
                        create_method_router(route.method.as_str(), has_path_params, handler, hooks.clone())
                    }
                }
                method => create_method_router(method, has_path_params, handler, hooks.clone()),
            };

            combined_router = Some(match combined_router {
                None => method_router,
                Some(existing) => existing.merge(method_router),
            });

            tracing::info!("Registered route: {} {}", route.method.as_str(), path);
        }

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
            let mut axum_path = type_hints::strip_type_hints(&path);
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

    app = app.layer(axum::Extension(route_registry));

    Ok(app)
}

/// Build router with handlers and apply middleware based on config
pub fn build_router_with_handlers_and_config(
    routes: Vec<RouteHandlerPair>,
    config: ServerConfig,
    route_metadata: Vec<crate::RouteMetadata>,
) -> Result<AxumRouter, String> {
    #[cfg(feature = "di")]
    if config.di_container.is_none() {
        eprintln!("[spikard-di] build_router: di_container is None");
    } else {
        eprintln!(
            "[spikard-di] build_router: di_container has keys: {:?}",
            config.di_container.as_ref().unwrap().keys()
        );
    }
    let hooks = config.lifecycle_hooks.clone();

    #[cfg(feature = "di")]
    let mut app = build_router_with_handlers(routes, hooks, config.di_container.clone())?;
    #[cfg(not(feature = "di"))]
    let mut app = build_router_with_handlers(routes, hooks)?;

    app = app.layer(SetSensitiveRequestHeadersLayer::new([
        axum::http::header::AUTHORIZATION,
        axum::http::header::COOKIE,
    ]));

    if let Some(ref compression) = config.compression {
        let mut compression_layer = CompressionLayer::new();
        if !compression.gzip {
            compression_layer = compression_layer.gzip(false);
        }
        if !compression.brotli {
            compression_layer = compression_layer.br(false);
        }

        let min_threshold = compression.min_size.min(u16::MAX as usize) as u16;
        let predicate = SizeAbove::new(min_threshold)
            .and(NotForContentType::GRPC)
            .and(NotForContentType::IMAGES)
            .and(NotForContentType::SSE);
        let compression_layer = compression_layer.compress_when(predicate);

        app = app.layer(compression_layer);
    }

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

    if let Some(ref jwt_config) = config.jwt_auth {
        let jwt_config_clone = jwt_config.clone();
        app = app.layer(axum::middleware::from_fn(move |headers, req, next| {
            crate::auth::jwt_auth_middleware(jwt_config_clone.clone(), headers, req, next)
        }));
    }

    if let Some(ref api_key_config) = config.api_key_auth {
        let api_key_config_clone = api_key_config.clone();
        app = app.layer(axum::middleware::from_fn(move |headers, req, next| {
            crate::auth::api_key_auth_middleware(api_key_config_clone.clone(), headers, req, next)
        }));
    }

    if let Some(timeout_secs) = config.request_timeout {
        app = app.layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(timeout_secs),
        ));
    }

    if config.enable_request_id {
        app = app
            .layer(PropagateRequestIdLayer::x_request_id())
            .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid));
    }

    if let Some(max_size) = config.max_body_size {
        app = app.layer(DefaultBodyLimit::max(max_size));
    } else {
        app = app.layer(DefaultBodyLimit::disable());
    }

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

    if let Some(ref openapi_config) = config.openapi
        && openapi_config.enabled
    {
        use axum::response::{Html, Json};

        let schema_registry = crate::SchemaRegistry::new();
        let openapi_spec =
            crate::openapi::generate_openapi_spec(&route_metadata, openapi_config, &schema_registry, Some(&config))
                .map_err(|e| format!("Failed to generate OpenAPI spec: {}", e))?;

        let spec_json =
            serde_json::to_string(&openapi_spec).map_err(|e| format!("Failed to serialize OpenAPI spec: {}", e))?;
        let spec_value = serde_json::from_str::<serde_json::Value>(&spec_json)
            .map_err(|e| format!("Failed to parse OpenAPI spec: {}", e))?;

        let openapi_json_path = openapi_config.openapi_json_path.clone();
        app = app.route(&openapi_json_path, get(move || async move { Json(spec_value) }));

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
        let metadata: Vec<crate::RouteMetadata> = routes
            .iter()
            .map(|(route, _)| {
                #[cfg(feature = "di")]
                {
                    crate::RouteMetadata {
                        method: route.method.to_string(),
                        path: route.path.clone(),
                        handler_name: route.handler_name.clone(),
                        request_schema: None,
                        response_schema: None,
                        parameter_schema: None,
                        file_params: route.file_params.clone(),
                        is_async: route.is_async,
                        cors: route.cors.clone(),
                        body_param_name: None,
                        handler_dependencies: Some(route.handler_dependencies.clone()),
                    }
                }
                #[cfg(not(feature = "di"))]
                {
                    crate::RouteMetadata {
                        method: route.method.to_string(),
                        path: route.path.clone(),
                        handler_name: route.handler_name.clone(),
                        request_schema: None,
                        response_schema: None,
                        parameter_schema: None,
                        file_params: route.file_params.clone(),
                        is_async: route.is_async,
                        cors: route.cors.clone(),
                        body_param_name: None,
                    }
                }
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

        let app = self.build_axum_router();

        let addr = format!("{}:{}", self.config.host, self.config.port);
        let socket_addr: SocketAddr = addr.parse()?;
        let listener = TcpListener::bind(socket_addr).await?;

        tracing::info!("Listening on http://{}", socket_addr);

        axum::serve(listener, app).await?;

        Ok(())
    }

    /// Build Axum router from our router
    fn build_axum_router(&self) -> AxumRouter {
        let mut app = AxumRouter::new();

        app = app.route("/health", get(|| async { "OK" }));

        // TODO: Add routes from self.router

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
    }
}

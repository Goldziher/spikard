//! HTTP server implementation using Tokio and Axum
//!
//! This module provides the main server builder and routing infrastructure, with
//! focused submodules for handler validation, request extraction, and lifecycle execution.

pub mod grpc_routing;
pub mod handler;
pub mod lifecycle_execution;
pub mod request_extraction;

use crate::handler_trait::{Handler, HandlerResult, RequestData};
use crate::{CorsConfig, Router, ServerConfig};
use axum::Router as AxumRouter;
use axum::body::Body;
use axum::extract::{DefaultBodyLimit, Path};
use axum::http::StatusCode;
use axum::routing::{MethodRouter, get, post};
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
fn method_expects_body(method: &crate::Method) -> bool {
    matches!(method, crate::Method::Post | crate::Method::Put | crate::Method::Patch)
}

fn looks_like_json(body: &str) -> bool {
    let trimmed = body.trim_start();
    trimmed.starts_with('{') || trimmed.starts_with('[')
}

fn error_to_response(status: StatusCode, body: String) -> axum::response::Response {
    let content_type = if looks_like_json(&body) {
        "application/json"
    } else {
        "text/plain; charset=utf-8"
    };

    axum::response::Response::builder()
        .status(status)
        .header(axum::http::header::CONTENT_TYPE, content_type)
        .body(Body::from(body))
        .unwrap_or_else(|_| {
            axum::response::Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")
                .body(Body::from("Failed to build error response"))
                .unwrap()
        })
}

fn handler_result_to_response(result: HandlerResult) -> axum::response::Response {
    match result {
        Ok(response) => response,
        Err((status, body)) => error_to_response(status, body),
    }
}

#[inline]
async fn call_with_optional_hooks(
    req: axum::http::Request<Body>,
    request_data: RequestData,
    handler: Arc<dyn Handler>,
    hooks: Option<Arc<crate::LifecycleHooks>>,
) -> HandlerResult {
    if hooks.as_ref().is_some_and(|h| !h.is_empty()) {
        lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler, hooks).await
    } else {
        handler.call(req, request_data).await
    }
}

/// Creates a method router for the given HTTP method.
/// Handles both path parameters and non-path variants.
fn create_method_router(
    method: crate::Method,
    has_path_params: bool,
    handler: Arc<dyn Handler>,
    hooks: Option<Arc<crate::LifecycleHooks>>,
    include_raw_query_params: bool,
    include_query_params_json: bool,
) -> axum::routing::MethodRouter {
    let expects_body = method_expects_body(&method);
    let include_headers = handler.wants_headers();
    let include_cookies = handler.wants_cookies();
    let without_body_options = request_extraction::WithoutBodyExtractionOptions {
        include_raw_query_params,
        include_query_params_json,
        include_headers,
        include_cookies,
    };

    if expects_body {
        if has_path_params {
            let handler_clone = handler.clone();
            let hooks_clone = hooks.clone();
            match method {
                crate::Method::Post => axum::routing::post(
                    move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                        let (parts, body) = req.into_parts();
                        let request_data = match request_extraction::create_request_data_with_body(
                            &parts,
                            path_params.0,
                            body,
                            include_raw_query_params,
                            include_query_params_json,
                            include_headers,
                            include_cookies,
                        )
                        .await
                        {
                            Ok(data) => data,
                            Err((status, body)) => return error_to_response(status, body),
                        };
                        let req = axum::extract::Request::from_parts(parts, Body::empty());
                        handler_result_to_response(
                            call_with_optional_hooks(req, request_data, handler_clone, hooks_clone).await,
                        )
                    },
                ),
                crate::Method::Put => axum::routing::put(
                    move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                        let (parts, body) = req.into_parts();
                        let request_data = match request_extraction::create_request_data_with_body(
                            &parts,
                            path_params.0,
                            body,
                            include_raw_query_params,
                            include_query_params_json,
                            include_headers,
                            include_cookies,
                        )
                        .await
                        {
                            Ok(data) => data,
                            Err((status, body)) => return error_to_response(status, body),
                        };
                        let req = axum::extract::Request::from_parts(parts, Body::empty());
                        handler_result_to_response(
                            call_with_optional_hooks(req, request_data, handler_clone, hooks_clone).await,
                        )
                    },
                ),
                crate::Method::Patch => axum::routing::patch(
                    move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                        let (parts, body) = req.into_parts();
                        let request_data = match request_extraction::create_request_data_with_body(
                            &parts,
                            path_params.0,
                            body,
                            include_raw_query_params,
                            include_query_params_json,
                            include_headers,
                            include_cookies,
                        )
                        .await
                        {
                            Ok(data) => data,
                            Err((status, body)) => return error_to_response(status, body),
                        };
                        let req = axum::extract::Request::from_parts(parts, Body::empty());
                        handler_result_to_response(
                            call_with_optional_hooks(req, request_data, handler_clone, hooks_clone).await,
                        )
                    },
                ),
                crate::Method::Get
                | crate::Method::Delete
                | crate::Method::Head
                | crate::Method::Options
                | crate::Method::Trace => MethodRouter::new(),
            }
        } else {
            let handler_clone = handler.clone();
            let hooks_clone = hooks.clone();
            match method {
                crate::Method::Post => axum::routing::post(move |req: axum::extract::Request| async move {
                    let (parts, body) = req.into_parts();
                    let request_data = match request_extraction::create_request_data_with_body(
                        &parts,
                        HashMap::new(),
                        body,
                        include_raw_query_params,
                        include_query_params_json,
                        include_headers,
                        include_cookies,
                    )
                    .await
                    {
                        Ok(data) => data,
                        Err((status, body)) => return error_to_response(status, body),
                    };
                    let req = axum::extract::Request::from_parts(parts, Body::empty());
                    handler_result_to_response(
                        call_with_optional_hooks(req, request_data, handler_clone, hooks_clone).await,
                    )
                }),
                crate::Method::Put => axum::routing::put(move |req: axum::extract::Request| async move {
                    let (parts, body) = req.into_parts();
                    let request_data = match request_extraction::create_request_data_with_body(
                        &parts,
                        HashMap::new(),
                        body,
                        include_raw_query_params,
                        include_query_params_json,
                        include_headers,
                        include_cookies,
                    )
                    .await
                    {
                        Ok(data) => data,
                        Err((status, body)) => return error_to_response(status, body),
                    };
                    let req = axum::extract::Request::from_parts(parts, Body::empty());
                    handler_result_to_response(
                        call_with_optional_hooks(req, request_data, handler_clone, hooks_clone).await,
                    )
                }),
                crate::Method::Patch => axum::routing::patch(move |req: axum::extract::Request| async move {
                    let (parts, body) = req.into_parts();
                    let request_data = match request_extraction::create_request_data_with_body(
                        &parts,
                        HashMap::new(),
                        body,
                        include_raw_query_params,
                        include_query_params_json,
                        include_headers,
                        include_cookies,
                    )
                    .await
                    {
                        Ok(data) => data,
                        Err((status, body)) => return error_to_response(status, body),
                    };
                    let req = axum::extract::Request::from_parts(parts, Body::empty());
                    handler_result_to_response(
                        call_with_optional_hooks(req, request_data, handler_clone, hooks_clone).await,
                    )
                }),
                crate::Method::Get
                | crate::Method::Delete
                | crate::Method::Head
                | crate::Method::Options
                | crate::Method::Trace => MethodRouter::new(),
            }
        }
    } else if has_path_params {
        let handler_clone = handler.clone();
        let hooks_clone = hooks.clone();
        match method {
            crate::Method::Get => axum::routing::get(
                move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                    let request_data = request_extraction::create_request_data_without_body(
                        req.uri(),
                        req.method(),
                        req.headers(),
                        path_params.0,
                        without_body_options,
                    );
                    handler_result_to_response(
                        call_with_optional_hooks(req, request_data, handler_clone, hooks_clone).await,
                    )
                },
            ),
            crate::Method::Delete => axum::routing::delete(
                move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                    let request_data = request_extraction::create_request_data_without_body(
                        req.uri(),
                        req.method(),
                        req.headers(),
                        path_params.0,
                        without_body_options,
                    );
                    handler_result_to_response(
                        call_with_optional_hooks(req, request_data, handler_clone, hooks_clone).await,
                    )
                },
            ),
            crate::Method::Head => axum::routing::head(
                move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                    let request_data = request_extraction::create_request_data_without_body(
                        req.uri(),
                        req.method(),
                        req.headers(),
                        path_params.0,
                        without_body_options,
                    );
                    handler_result_to_response(
                        call_with_optional_hooks(req, request_data, handler_clone, hooks_clone).await,
                    )
                },
            ),
            crate::Method::Trace => axum::routing::trace(
                move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                    let request_data = request_extraction::create_request_data_without_body(
                        req.uri(),
                        req.method(),
                        req.headers(),
                        path_params.0,
                        without_body_options,
                    );
                    handler_result_to_response(
                        call_with_optional_hooks(req, request_data, handler_clone, hooks_clone).await,
                    )
                },
            ),
            crate::Method::Options => axum::routing::options(
                move |path_params: Path<HashMap<String, String>>, req: axum::extract::Request| async move {
                    let request_data = request_extraction::create_request_data_without_body(
                        req.uri(),
                        req.method(),
                        req.headers(),
                        path_params.0,
                        without_body_options,
                    );
                    handler_result_to_response(
                        call_with_optional_hooks(req, request_data, handler_clone, hooks_clone).await,
                    )
                },
            ),
            crate::Method::Post | crate::Method::Put | crate::Method::Patch => MethodRouter::new(),
        }
    } else {
        let handler_clone = handler.clone();
        let hooks_clone = hooks.clone();
        match method {
            crate::Method::Get => axum::routing::get(move |req: axum::extract::Request| async move {
                let request_data = request_extraction::create_request_data_without_body(
                    req.uri(),
                    req.method(),
                    req.headers(),
                    HashMap::new(),
                    without_body_options,
                );
                handler_result_to_response(
                    call_with_optional_hooks(req, request_data, handler_clone, hooks_clone).await,
                )
            }),
            crate::Method::Delete => axum::routing::delete(move |req: axum::extract::Request| async move {
                let request_data = request_extraction::create_request_data_without_body(
                    req.uri(),
                    req.method(),
                    req.headers(),
                    HashMap::new(),
                    without_body_options,
                );
                handler_result_to_response(
                    call_with_optional_hooks(req, request_data, handler_clone, hooks_clone).await,
                )
            }),
            crate::Method::Head => axum::routing::head(move |req: axum::extract::Request| async move {
                let request_data = request_extraction::create_request_data_without_body(
                    req.uri(),
                    req.method(),
                    req.headers(),
                    HashMap::new(),
                    without_body_options,
                );
                handler_result_to_response(
                    call_with_optional_hooks(req, request_data, handler_clone, hooks_clone).await,
                )
            }),
            crate::Method::Trace => axum::routing::trace(move |req: axum::extract::Request| async move {
                let request_data = request_extraction::create_request_data_without_body(
                    req.uri(),
                    req.method(),
                    req.headers(),
                    HashMap::new(),
                    without_body_options,
                );
                handler_result_to_response(
                    call_with_optional_hooks(req, request_data, handler_clone, hooks_clone).await,
                )
            }),
            crate::Method::Options => axum::routing::options(move |req: axum::extract::Request| async move {
                let request_data = request_extraction::create_request_data_without_body(
                    req.uri(),
                    req.method(),
                    req.headers(),
                    HashMap::new(),
                    without_body_options,
                );
                handler_result_to_response(
                    call_with_optional_hooks(req, request_data, handler_clone, hooks_clone).await,
                )
            }),
            crate::Method::Post | crate::Method::Put | crate::Method::Patch => MethodRouter::new(),
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
///
/// Coverage: Tested via integration tests (Unix signal handling not easily unit testable)
#[cfg(not(tarpaulin_include))]
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
    build_router_with_handlers_inner(routes, hooks, None, true)
}

/// Build an Axum router from routes and foreign handlers with optional DI container
#[cfg(feature = "di")]
pub fn build_router_with_handlers(
    routes: Vec<(crate::Route, Arc<dyn Handler>)>,
    hooks: Option<Arc<crate::LifecycleHooks>>,
    di_container: Option<Arc<spikard_core::di::DependencyContainer>>,
) -> Result<AxumRouter, String> {
    build_router_with_handlers_inner(routes, hooks, di_container, true)
}

fn build_router_with_handlers_inner(
    routes: Vec<(crate::Route, Arc<dyn Handler>)>,
    hooks: Option<Arc<crate::LifecycleHooks>>,
    #[cfg(feature = "di")] di_container: Option<Arc<spikard_core::di::DependencyContainer>>,
    #[cfg(not(feature = "di"))] _di_container: Option<()>,
    enable_http_trace: bool,
) -> Result<AxumRouter, String> {
    let mut app = AxumRouter::new();

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
            let method = route.method.clone();
            let method_router: MethodRouter = match method {
                crate::Method::Options => {
                    if let Some(ref cors_cfg) = route.cors {
                        let cors_config = cors_cfg.clone();
                        axum::routing::options(move |req: axum::extract::Request| async move {
                            crate::cors::handle_preflight(req.headers(), &cors_config).map_err(|e| *e)
                        })
                    } else {
                        let include_raw_query_params = route.parameter_validator.is_some();
                        let include_query_params_json = !handler.prefers_parameter_extraction();
                        create_method_router(
                            method,
                            has_path_params,
                            handler,
                            hooks.clone(),
                            include_raw_query_params,
                            include_query_params_json,
                        )
                    }
                }
                method => {
                    let include_raw_query_params = route.parameter_validator.is_some();
                    let include_query_params_json = !handler.prefers_parameter_extraction();
                    create_method_router(
                        method,
                        has_path_params,
                        handler,
                        hooks.clone(),
                        include_raw_query_params,
                        include_query_params_json,
                    )
                }
            };

            let method_router = method_router.layer(axum::middleware::from_fn_with_state(
                crate::middleware::RouteInfo {
                    expects_json_body: route.expects_json_body,
                },
                crate::middleware::validate_content_type_middleware,
            ));

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

    if enable_http_trace {
        app = app.layer(TraceLayer::new_for_http());
    }

    Ok(app)
}

/// Build router with handlers and apply middleware based on config
pub fn build_router_with_handlers_and_config(
    routes: Vec<RouteHandlerPair>,
    config: ServerConfig,
    route_metadata: Vec<crate::RouteMetadata>,
) -> Result<AxumRouter, String> {
    #[cfg(feature = "di")]
    if let Some(di_container) = config.di_container.as_ref() {
        eprintln!(
            "[spikard-di] build_router: di_container has keys: {:?}",
            di_container.keys()
        );
    } else {
        eprintln!("[spikard-di] build_router: di_container is None");
    }
    let hooks = config.lifecycle_hooks.clone();

    let jsonrpc_registry = if let Some(ref jsonrpc_config) = config.jsonrpc {
        if jsonrpc_config.enabled {
            let registry = Arc::new(crate::jsonrpc::JsonRpcMethodRegistry::new());

            for (route, handler) in &routes {
                if let Some(ref jsonrpc_info) = route.jsonrpc_method {
                    let method_name = jsonrpc_info.method_name.clone();

                    let metadata = crate::jsonrpc::MethodMetadata::new(&method_name)
                        .with_params_schema(jsonrpc_info.params_schema.clone().unwrap_or(serde_json::json!({})))
                        .with_result_schema(jsonrpc_info.result_schema.clone().unwrap_or(serde_json::json!({})));

                    let metadata = if let Some(ref description) = jsonrpc_info.description {
                        metadata.with_description(description.clone())
                    } else {
                        metadata
                    };

                    let metadata = if jsonrpc_info.deprecated {
                        metadata.mark_deprecated()
                    } else {
                        metadata
                    };

                    let mut metadata = metadata;
                    for tag in &jsonrpc_info.tags {
                        metadata = metadata.with_tag(tag.clone());
                    }

                    if let Err(e) = registry.register(&method_name, Arc::clone(handler), metadata) {
                        tracing::warn!(
                            "Failed to register JSON-RPC method '{}' for route {}: {}",
                            method_name,
                            route.path,
                            e
                        );
                    } else {
                        tracing::debug!(
                            "Registered JSON-RPC method '{}' for route {} {} (handler: {})",
                            method_name,
                            route.method,
                            route.path,
                            route.handler_name
                        );
                    }
                }
            }

            Some(registry)
        } else {
            None
        }
    } else {
        None
    };

    #[cfg(feature = "di")]
    let mut app =
        build_router_with_handlers_inner(routes, hooks, config.di_container.clone(), config.enable_http_trace)?;
    #[cfg(not(feature = "di"))]
    let mut app = build_router_with_handlers_inner(routes, hooks, None, config.enable_http_trace)?;

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

    if let Some(ref jsonrpc_config) = config.jsonrpc
        && jsonrpc_config.enabled
        && let Some(registry) = jsonrpc_registry
    {
        let jsonrpc_router = Arc::new(crate::jsonrpc::JsonRpcRouter::new(
            registry,
            jsonrpc_config.enable_batch,
            jsonrpc_config.max_batch_size,
        ));

        let state = Arc::new(crate::jsonrpc::JsonRpcState { router: jsonrpc_router });

        let endpoint_path = jsonrpc_config.endpoint_path.clone();
        app = app.route(&endpoint_path, post(crate::jsonrpc::handle_jsonrpc).with_state(state));

        // TODO: Add per-method routes if enabled
        // TODO: Add WebSocket endpoint if enabled
        // TODO: Add SSE endpoint if enabled
        // TODO: Add OpenRPC spec endpoint if enabled

        tracing::info!("JSON-RPC endpoint enabled at {}", endpoint_path);
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
                        jsonrpc_method: route
                            .jsonrpc_method
                            .as_ref()
                            .map(|info| serde_json::to_value(info).unwrap_or(serde_json::json!(null))),
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
                        jsonrpc_method: route
                            .jsonrpc_method
                            .as_ref()
                            .map(|info| serde_json::to_value(info).unwrap_or(serde_json::json!(null))),
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
    ///
    /// Coverage: Production-only, tested via integration tests
    #[cfg(not(tarpaulin_include))]
    pub async fn run_with_config(app: AxumRouter, config: ServerConfig) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("{}:{}", config.host, config.port);
        let socket_addr: SocketAddr = addr.parse()?;
        let listener = TcpListener::bind(socket_addr).await?;

        tracing::info!("Listening on http://{}", socket_addr);

        if config.graceful_shutdown {
            axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
                .with_graceful_shutdown(shutdown_signal())
                .await?;
        } else {
            axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;
        }

        Ok(())
    }

    /// Initialize logging
    pub fn init_logging() {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "spikard=info,tower_http=info".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
    }

    /// Start the server
    ///
    /// Coverage: Production-only, tested via integration tests
    #[cfg(not(tarpaulin_include))]
    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Starting server with {} routes", self.router.route_count());

        let app = self.build_axum_router();

        let addr = format!("{}:{}", self.config.host, self.config.port);
        let socket_addr: SocketAddr = addr.parse()?;
        let listener = TcpListener::bind(socket_addr).await?;

        tracing::info!("Listening on http://{}", socket_addr);

        axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;

        Ok(())
    }

    /// Build Axum router from our router
    fn build_axum_router(&self) -> AxumRouter {
        let mut app = AxumRouter::new();

        app = app.route("/health", get(|| async { "OK" }));

        // TODO: Add routes from self.router

        if self.config.enable_http_trace {
            app = app.layer(TraceLayer::new_for_http());
        }

        app
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::pin::Pin;
    use std::sync::Arc;

    struct TestHandler;

    impl Handler for TestHandler {
        fn call(
            &self,
            _request: axum::http::Request<Body>,
            _request_data: crate::handler_trait::RequestData,
        ) -> Pin<Box<dyn std::future::Future<Output = crate::handler_trait::HandlerResult> + Send + '_>> {
            Box::pin(async { Ok(axum::http::Response::builder().status(200).body(Body::empty()).unwrap()) })
        }
    }

    fn build_test_route(path: &str, method: &str, handler_name: &str, expects_json_body: bool) -> crate::Route {
        use std::str::FromStr;
        crate::Route {
            path: path.to_string(),
            method: spikard_core::Method::from_str(method).expect("valid method"),
            handler_name: handler_name.to_string(),
            expects_json_body,
            cors: None,
            is_async: true,
            file_params: None,
            request_validator: None,
            response_validator: None,
            parameter_validator: None,
            jsonrpc_method: None,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
        }
    }

    fn build_test_route_with_cors(
        path: &str,
        method: &str,
        handler_name: &str,
        expects_json_body: bool,
        cors: crate::CorsConfig,
    ) -> crate::Route {
        use std::str::FromStr;
        crate::Route {
            path: path.to_string(),
            method: spikard_core::Method::from_str(method).expect("valid method"),
            handler_name: handler_name.to_string(),
            expects_json_body,
            cors: Some(cors),
            is_async: true,
            file_params: None,
            request_validator: None,
            response_validator: None,
            parameter_validator: None,
            jsonrpc_method: None,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
        }
    }

    #[test]
    fn test_method_expects_body_post() {
        assert!(method_expects_body(&crate::Method::Post));
    }

    #[test]
    fn test_method_expects_body_put() {
        assert!(method_expects_body(&crate::Method::Put));
    }

    #[test]
    fn test_method_expects_body_patch() {
        assert!(method_expects_body(&crate::Method::Patch));
    }

    #[test]
    fn test_method_expects_body_get() {
        assert!(!method_expects_body(&crate::Method::Get));
    }

    #[test]
    fn test_method_expects_body_delete() {
        assert!(!method_expects_body(&crate::Method::Delete));
    }

    #[test]
    fn test_method_expects_body_head() {
        assert!(!method_expects_body(&crate::Method::Head));
    }

    #[test]
    fn test_method_expects_body_options() {
        assert!(!method_expects_body(&crate::Method::Options));
    }

    #[test]
    fn test_method_expects_body_trace() {
        assert!(!method_expects_body(&crate::Method::Trace));
    }

    #[test]
    fn test_make_request_uuid_generates_valid_uuid() {
        let mut maker = MakeRequestUuid;
        let request = axum::http::Request::builder().body(Body::empty()).unwrap();

        let id = maker.make_request_id(&request);

        assert!(id.is_some());
        let id_val = id.unwrap();
        let id_str = id_val.header_value().to_str().expect("valid utf8");
        assert!(!id_str.is_empty());
        assert!(Uuid::parse_str(id_str).is_ok());
    }

    #[test]
    fn test_make_request_uuid_unique_per_call() {
        let mut maker = MakeRequestUuid;
        let request = axum::http::Request::builder().body(Body::empty()).unwrap();

        let id1 = maker.make_request_id(&request).unwrap();
        let id2 = maker.make_request_id(&request).unwrap();

        let id1_str = id1.header_value().to_str().expect("valid utf8");
        let id2_str = id2.header_value().to_str().expect("valid utf8");
        assert_ne!(id1_str, id2_str);
    }

    #[test]
    fn test_make_request_uuid_v4_format() {
        let mut maker = MakeRequestUuid;
        let request = axum::http::Request::builder().body(Body::empty()).unwrap();

        let id = maker.make_request_id(&request).unwrap();
        let id_str = id.header_value().to_str().expect("valid utf8");

        let uuid = Uuid::parse_str(id_str).expect("valid UUID");
        assert_eq!(uuid.get_version(), Some(uuid::Version::Random));
    }

    #[test]
    fn test_make_request_uuid_multiple_independent_makers() {
        let request = axum::http::Request::builder().body(Body::empty()).unwrap();

        let id1 = {
            let mut maker1 = MakeRequestUuid;
            maker1.make_request_id(&request).unwrap()
        };
        let id2 = {
            let mut maker2 = MakeRequestUuid;
            maker2.make_request_id(&request).unwrap()
        };

        let id1_str = id1.header_value().to_str().expect("valid utf8");
        let id2_str = id2.header_value().to_str().expect("valid utf8");
        assert_ne!(id1_str, id2_str);
    }

    #[test]
    fn test_make_request_uuid_clone_independence() {
        let mut maker1 = MakeRequestUuid;
        let mut maker2 = maker1.clone();
        let request = axum::http::Request::builder().body(Body::empty()).unwrap();

        let id1 = maker1.make_request_id(&request).unwrap();
        let id2 = maker2.make_request_id(&request).unwrap();

        let id1_str = id1.header_value().to_str().expect("valid utf8");
        let id2_str = id2.header_value().to_str().expect("valid utf8");
        assert_ne!(id1_str, id2_str);
    }

    #[test]
    fn test_server_creation() {
        let config = ServerConfig::default();
        let router = Router::new();
        let _server = Server::new(config, router);
    }

    #[test]
    fn test_server_creation_with_custom_host_port() {
        let mut config = ServerConfig::default();
        config.host = "0.0.0.0".to_string();
        config.port = 3000;

        let router = Router::new();
        let server = Server::new(config.clone(), router);

        assert_eq!(server.config.host, "0.0.0.0");
        assert_eq!(server.config.port, 3000);
    }

    #[test]
    fn test_server_config_default_values() {
        let config = ServerConfig::default();

        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8000);
        assert_eq!(config.workers, 1);
        assert!(!config.enable_request_id);
        assert!(config.max_body_size.is_some());
        assert!(config.request_timeout.is_none());
        assert!(config.graceful_shutdown);
    }

    #[test]
    fn test_server_config_builder_pattern() {
        let config = ServerConfig::builder().port(9000).host("0.0.0.0".to_string()).build();

        assert_eq!(config.port, 9000);
        assert_eq!(config.host, "0.0.0.0");
    }

    #[cfg(feature = "di")]
    fn build_router_for_tests(
        routes: Vec<(crate::Route, Arc<dyn Handler>)>,
        hooks: Option<Arc<crate::LifecycleHooks>>,
    ) -> Result<AxumRouter, String> {
        build_router_with_handlers(routes, hooks, None)
    }

    #[cfg(not(feature = "di"))]
    fn build_router_for_tests(
        routes: Vec<(crate::Route, Arc<dyn Handler>)>,
        hooks: Option<Arc<crate::LifecycleHooks>>,
    ) -> Result<AxumRouter, String> {
        build_router_with_handlers(routes, hooks)
    }

    #[test]
    fn test_route_registry_empty_routes() {
        let routes: Vec<(crate::Route, Arc<dyn Handler>)> = vec![];
        let _result = build_router_for_tests(routes, None);
    }

    #[test]
    fn test_route_registry_single_route() {
        let route = build_test_route("/test", "GET", "test_handler", false);

        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let routes = vec![(route, handler)];

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_path_normalization_without_leading_slash() {
        let route = build_test_route("api/users", "GET", "list_users", false);

        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let routes = vec![(route, handler)];

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_path_normalization_with_leading_slash() {
        let route = build_test_route("/api/users", "GET", "list_users", false);

        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let routes = vec![(route, handler)];

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_routes_same_path_different_methods() {
        let get_route = build_test_route("/users", "GET", "list_users", false);
        let post_route = build_test_route("/users", "POST", "create_user", true);

        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let routes = vec![(get_route, handler.clone()), (post_route, handler)];

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_different_routes() {
        let users_route = build_test_route("/users", "GET", "list_users", false);
        let posts_route = build_test_route("/posts", "GET", "list_posts", false);

        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let routes = vec![(users_route, handler.clone()), (posts_route, handler)];

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_with_single_path_parameter() {
        let route = build_test_route("/users/{id}", "GET", "get_user", false);

        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let routes = vec![(route, handler)];

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_with_multiple_path_parameters() {
        let route = build_test_route("/users/{user_id}/posts/{post_id}", "GET", "get_user_post", false);

        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let routes = vec![(route, handler)];

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_with_path_parameter_post_with_body() {
        let route = build_test_route("/users/{id}", "PUT", "update_user", true);

        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let routes = vec![(route, handler)];

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_with_path_parameter_delete() {
        let route = build_test_route("/users/{id}", "DELETE", "delete_user", false);

        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let routes = vec![(route, handler)];

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_post_method_with_body() {
        let route = build_test_route("/users", "POST", "create_user", true);

        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let routes = vec![(route, handler)];

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_put_method_with_body() {
        let route = build_test_route("/users/{id}", "PUT", "update_user", true);

        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let routes = vec![(route, handler)];

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_patch_method_with_body() {
        let route = build_test_route("/users/{id}", "PATCH", "patch_user", true);

        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let routes = vec![(route, handler)];

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_head_method() {
        let route = build_test_route("/users", "HEAD", "head_users", false);

        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let routes = vec![(route, handler)];

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_options_method() {
        let route = build_test_route("/users", "OPTIONS", "options_users", false);

        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let routes = vec![(route, handler)];

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_trace_method() {
        let route = build_test_route("/users", "TRACE", "trace_users", false);

        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let routes = vec![(route, handler)];

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_with_cors_config() {
        let cors_config = crate::CorsConfig {
            allowed_origins: vec!["https://example.com".to_string()],
            allowed_methods: vec!["GET".to_string(), "POST".to_string()],
            allowed_headers: vec!["Content-Type".to_string()],
            expose_headers: None,
            max_age: Some(3600),
            allow_credentials: Some(true),
        };

        let route = build_test_route_with_cors("/users", "GET", "list_users", false, cors_config);

        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let routes = vec![(route, handler)];

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_routes_with_cors_same_path() {
        let cors_config = crate::CorsConfig {
            allowed_origins: vec!["https://example.com".to_string()],
            allowed_methods: vec!["GET".to_string(), "POST".to_string()],
            allowed_headers: vec!["Content-Type".to_string()],
            expose_headers: None,
            max_age: Some(3600),
            allow_credentials: Some(true),
        };

        let get_route = build_test_route_with_cors("/users", "GET", "list_users", false, cors_config.clone());
        let post_route = build_test_route_with_cors("/users", "POST", "create_user", true, cors_config);

        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let routes = vec![(get_route, handler.clone()), (post_route, handler)];

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_routes_sorted_by_path() {
        let zebra_route = build_test_route("/zebra", "GET", "get_zebra", false);
        let alpha_route = build_test_route("/alpha", "GET", "get_alpha", false);

        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let routes = vec![(zebra_route, handler.clone()), (alpha_route, handler)];

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_routes_with_nested_paths() {
        let parent_route = build_test_route("/api", "GET", "get_api", false);
        let child_route = build_test_route("/api/users", "GET", "get_users", false);

        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let routes = vec![(parent_route, handler.clone()), (child_route, handler)];

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_routes_with_lifecycle_hooks() {
        let hooks = crate::LifecycleHooks::new();
        let hooks = Arc::new(hooks);

        let route = build_test_route("/users", "GET", "list_users", false);

        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let routes = vec![(route, handler)];

        let result = build_router_for_tests(routes, Some(hooks));
        assert!(result.is_ok());
    }

    #[test]
    fn test_routes_without_lifecycle_hooks() {
        let route = build_test_route("/users", "GET", "list_users", false);

        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let routes = vec![(route, handler)];

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_with_trailing_slash() {
        let route = build_test_route("/users/", "GET", "list_users", false);

        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let routes = vec![(route, handler)];

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_with_root_path() {
        let route = build_test_route("/", "GET", "root_handler", false);

        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let routes = vec![(route, handler)];

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_large_number_of_routes() {
        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let mut routes = vec![];

        for i in 0..50 {
            let route = build_test_route(&format!("/route{}", i), "GET", &format!("handler_{}", i), false);
            routes.push((route, handler.clone()));
        }

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_with_query_params_in_path_definition() {
        let route = build_test_route("/search", "GET", "search", false);

        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let routes = vec![(route, handler)];

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_all_http_methods_on_same_path() {
        let handler: Arc<dyn Handler> = Arc::new(TestHandler);
        let methods = vec!["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];

        let mut routes = vec![];
        for method in methods {
            let expects_body = matches!(method, "POST" | "PUT" | "PATCH");
            let route = build_test_route("/resource", method, &format!("handler_{}", method), expects_body);
            routes.push((route, handler.clone()));
        }

        let result = build_router_for_tests(routes, None);
        assert!(result.is_ok());
    }
}

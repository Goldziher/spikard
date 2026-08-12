//! HTTP server implementation using Tokio and Axum
//!
//! This module provides the main server builder and routing infrastructure, with
//! focused submodules for handler validation, request extraction, and lifecycle execution.

pub(crate) mod fast_router;
pub mod grpc_routing;
pub(crate) mod handler;
pub(crate) mod lifecycle_execution;
pub(crate) mod request_extraction;

use crate::handler_trait::{Handler, HandlerResult, RequestData};
use crate::{CorsConfig, ServerConfig};
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

#[derive(Clone)]
struct GrpcMiddlewareState {
    registry: Arc<crate::grpc::GrpcRegistry>,
    config: crate::grpc::GrpcConfig,
}

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

fn route_to_metadata(route: &crate::Route) -> crate::RouteMetadata {
    #[cfg(feature = "di")]
    {
        crate::RouteMetadata {
            method: route.method.to_string(),
            path: route.path.clone(),
            handler_name: route.handler_name.clone(),
            request_schema: route
                .request_validator
                .as_ref()
                .map(|validator| validator.schema().clone()),
            response_schema: route
                .response_validator
                .as_ref()
                .map(|validator| validator.schema().clone()),
            parameter_schema: route
                .parameter_validator
                .as_ref()
                .map(|validator| validator.schema().clone()),
            file_params: route.file_params.clone(),
            is_async: route.is_async,
            cors: route.cors.clone(),
            body_param_name: route.expects_json_body.then(|| "body".to_string()),
            handler_dependencies: Some(route.handler_dependencies.clone()),
            jsonrpc_method: route
                .jsonrpc_method
                .as_ref()
                .map(|info| serde_json::to_value(info).unwrap_or(serde_json::json!(null))),
            static_response: None,
            compression: route.compression.clone(),
            body_limit: route.body_limit,
            request_timeout_secs: route.request_timeout_secs,
            rate_limit: route.rate_limit.clone(),
            request_id: route.request_id,
            jwt_auth: route.jwt_auth.clone(),
            api_key_auth: route.api_key_auth.clone(),
            authorization: route.authorization.clone(),
            lifecycle_hooks: route.lifecycle_hooks.clone(),
            openrpc_spec: route.openrpc_spec.clone(),
        }
    }
    #[cfg(not(feature = "di"))]
    {
        crate::RouteMetadata {
            method: route.method.to_string(),
            path: route.path.clone(),
            handler_name: route.handler_name.clone(),
            request_schema: route
                .request_validator
                .as_ref()
                .map(|validator| validator.schema().clone()),
            response_schema: route
                .response_validator
                .as_ref()
                .map(|validator| validator.schema().clone()),
            parameter_schema: route
                .parameter_validator
                .as_ref()
                .map(|validator| validator.schema().clone()),
            file_params: route.file_params.clone(),
            is_async: route.is_async,
            cors: route.cors.clone(),
            body_param_name: route.expects_json_body.then(|| "body".to_string()),
            jsonrpc_method: route
                .jsonrpc_method
                .as_ref()
                .map(|info| serde_json::to_value(info).unwrap_or(serde_json::json!(null))),
            static_response: None,
            compression: route.compression.clone(),
            body_limit: route.body_limit,
            request_timeout_secs: route.request_timeout_secs,
            rate_limit: route.rate_limit.clone(),
            request_id: route.request_id,
            jwt_auth: route.jwt_auth.clone(),
            api_key_auth: route.api_key_auth.clone(),
            authorization: route.authorization.clone(),
            lifecycle_hooks: route.lifecycle_hooks.clone(),
            openrpc_spec: route.openrpc_spec.clone(),
        }
    }
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

async fn grpc_routing_middleware(
    axum::extract::State(state): axum::extract::State<GrpcMiddlewareState>,
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    if grpc_routing::is_grpc_request(&request) {
        return match grpc_routing::route_grpc_request(Arc::clone(&state.registry), &state.config, request).await {
            Ok(response) => response,
            Err((status, body)) => error_to_response(status, body),
        };
    }

    next.run(request).await
}

#[inline]
async fn call_with_optional_hooks(
    req: axum::http::Request<Body>,
    request_data: RequestData,
    handler: Arc<dyn Handler>,
    hooks: Option<Arc<crate::LifecycleHooks>>,
) -> HandlerResult {
    let request_data = if let Some(claims) = req.extensions().get::<crate::auth::Claims>() {
        let mut request_data = request_data;
        if let Ok(serialized_claims) = serde_json::to_string(claims) {
            let mut headers = (*request_data.headers).clone();
            headers.insert(crate::auth::INTERNAL_JWT_CLAIMS_HEADER.to_string(), serialized_claims);
            request_data.headers = Arc::new(headers);
        }
        request_data
    } else {
        request_data
    };

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
                | crate::Method::Connect
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
                | crate::Method::Connect
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
            crate::Method::Post | crate::Method::Put | crate::Method::Patch | crate::Method::Connect => {
                MethodRouter::new()
            }
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
            crate::Method::Post | crate::Method::Put | crate::Method::Patch | crate::Method::Connect => {
                MethodRouter::new()
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

/// Build a per-route JWT middleware config from `Route.jwt_auth`, or `None` when the route's JWT
/// requirement is explicitly disabled (`enabled: false`).
///
/// `spikard_core::http::JwtAuthConfig::secret` is optional because asymmetric algorithms verify
/// against a `public_key` instead, but `spikard_http::JwtConfig` — the type
/// `auth::jwt_auth_middleware` actually consumes — only supports a symmetric secret today. A
/// route configured for an asymmetric algorithm has no working enforcement path yet, so this is
/// a hard build error rather than a route that silently never checks its token: failing to start
/// is safer than failing open. ~keep
fn jwt_config_from_route(route_path: &str, cfg: &crate::JwtAuthConfig) -> Result<Option<crate::JwtConfig>, String> {
    if !cfg.enabled {
        return Ok(None);
    }

    let Some(secret) = cfg.secret.clone() else {
        return Err(format!(
            "route '{route_path}': JWT auth with algorithm '{}' has no `secret`; per-route JWT \
             enforcement only supports symmetric (secret-based) algorithms today, not `public_key`",
            cfg.algorithm
        ));
    };

    Ok(Some(crate::JwtConfig {
        secret,
        algorithm: cfg.algorithm.clone(),
        audience: cfg.audience.clone(),
        issuer: cfg.issuer.clone(),
        leeway: cfg.leeway,
    }))
}

/// Build a per-route API key middleware config from `Route.api_key_auth`, or `None` when the
/// route's API key requirement is explicitly disabled.
///
/// An empty `keys` list on an *enabled* config is a hard misconfiguration error, never an open
/// gate — see `ApiKeyAuthConfig::keys`'s doc comment in `spikard-core`. Enforcing it as an error
/// here (rather than deploying a route that rejects every request with 401) is what that
/// documented invariant requires of "a later enforcement phase". ~keep
fn api_key_config_from_route(
    route_path: &str,
    cfg: &crate::ApiKeyAuthConfig,
) -> Result<Option<crate::ApiKeyConfig>, String> {
    if !cfg.enabled {
        return Ok(None);
    }

    if cfg.keys.is_empty() {
        return Err(format!(
            "route '{route_path}': API key auth is enabled with an empty `keys` list, which would \
             accept no key at all; configure at least one valid key or disable the requirement"
        ));
    }

    Ok(Some(crate::ApiKeyConfig {
        keys: cfg.keys.clone(),
        header_name: cfg.header_name.clone(),
    }))
}

/// Resolve a route's named lifecycle hook selection against the server's registered
/// `LifecycleHooks` container.
///
/// Per-route `lifecycle_hooks` is opt-in, not a refinement of a server-wide default: a route with
/// no `lifecycle_hooks` config runs no hooks at all, even if hooks are registered on
/// `ServerConfig.lifecycle_hooks`. This replaces the old behavior where the same registered hook
/// set was threaded into *every* route unconditionally (`hooks.clone()` at every
/// `create_method_router` call site) — that made "hooks registered on the server" indistinguishable
/// from "hooks that run for this route," which is exactly the bug this per-route wiring fixes. A
/// named hook that isn't registered for the phase it's requested in is a hard build-time error,
/// never a silent skip: silently running fewer hooks than a route's config asks for is a
/// security-relevant footgun (e.g. a missing auth hook). ~keep
fn resolve_route_lifecycle_hooks(
    route_path: &str,
    config: &crate::LifecycleHooksConfig,
    registered: Option<&crate::LifecycleHooks>,
) -> Result<crate::LifecycleHooks, String> {
    use crate::LifecycleHookPhase;

    let mut resolved = crate::LifecycleHooks::default();

    let phases: [(LifecycleHookPhase, &[crate::LifecycleHookRef]); 5] = [
        (LifecycleHookPhase::OnRequest, &config.on_request),
        (LifecycleHookPhase::PreValidation, &config.pre_validation),
        (LifecycleHookPhase::PreHandler, &config.pre_handler),
        (LifecycleHookPhase::OnResponse, &config.on_response),
        (LifecycleHookPhase::OnError, &config.on_error),
    ];

    for (phase, refs) in phases {
        if refs.is_empty() {
            continue;
        }

        let Some(registered) = registered else {
            return Err(format!(
                "route '{route_path}': references lifecycle hook '{}' ({phase:?}), but no lifecycle hooks are registered on the server",
                refs[0].name
            ));
        };

        let mut ordered: Vec<&crate::LifecycleHookRef> = refs.iter().collect();
        ordered.sort_by_key(|hook_ref| hook_ref.order.unwrap_or(u32::MAX));

        for hook_ref in ordered {
            let hook = registered.find_in_phase(phase, &hook_ref.name).ok_or_else(|| {
                format!(
                    "route '{route_path}': references lifecycle hook '{}' ({phase:?}), but no hook with that name is registered for that phase",
                    hook_ref.name
                )
            })?;

            match phase {
                LifecycleHookPhase::OnRequest => resolved.add_on_request(hook),
                LifecycleHookPhase::PreValidation => resolved.add_pre_validation(hook),
                LifecycleHookPhase::PreHandler => resolved.add_pre_handler(hook),
                LifecycleHookPhase::OnResponse => resolved.add_on_response(hook),
                LifecycleHookPhase::OnError => resolved.add_on_error(hook),
            }
        }
    }

    Ok(resolved)
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
    let mut fast_router = fast_router::FastRouter::new();

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

        type RouteEntry = (crate::Route, Arc<dyn Handler>, Option<crate::StaticResponse>);
        let mut handlers_by_method: HashMap<crate::Method, RouteEntry> = HashMap::new();
        for (route, handler) in route_handlers {
            #[cfg(feature = "di")]
            let handler = if let Some(ref container) = di_container {
                let required_deps = extract_handler_dependencies(&route);
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

            let static_resp = handler.static_response();
            let validating_handler = Arc::new(handler::ValidatingHandler::new(handler, &route));
            handlers_by_method.insert(route.method.clone(), (route, validating_handler, static_resp));
        }

        let cors_config: Option<CorsConfig> = handlers_by_method
            .values()
            .find_map(|(route, _, _)| route.cors.as_ref())
            .cloned();

        let has_options_handler = handlers_by_method.keys().any(|m| m.as_str() == "OPTIONS");

        let mut combined_router: Option<MethodRouter> = None;
        let has_path_params = path.contains('{');

        for (_method, (route, handler, static_resp_opt)) in handlers_by_method {
            let method = route.method.clone();

            // NOTE: static routes also bypass CORS handling, content-type
            if let Some(static_resp) = static_resp_opt {
                let resp_status = static_resp.status;

                if !has_path_params {
                    let axum_path_for_fast = spikard_core::type_hints::strip_type_hints(&path);
                    let http_method: axum::http::Method = route.method.as_str().parse().map_err(|_| {
                        format!(
                            "Invalid HTTP method '{}' for static route {}",
                            route.method.as_str(),
                            path
                        )
                    })?;
                    fast_router.insert(http_method, &axum_path_for_fast, &static_resp);
                }

                let static_handler = move || {
                    let resp = static_resp.to_response();
                    async move { resp }
                };

                let method_router: MethodRouter = match method {
                    crate::Method::Get => axum::routing::get(static_handler),
                    crate::Method::Post => axum::routing::post(static_handler),
                    crate::Method::Put => axum::routing::put(static_handler),
                    crate::Method::Patch => axum::routing::patch(static_handler),
                    crate::Method::Delete => axum::routing::delete(static_handler),
                    crate::Method::Head => axum::routing::head(static_handler),
                    crate::Method::Options => axum::routing::options(static_handler),
                    crate::Method::Connect => MethodRouter::new(),
                    crate::Method::Trace => axum::routing::trace(static_handler),
                };

                combined_router = Some(match combined_router {
                    None => method_router,
                    Some(existing) => existing.merge(method_router),
                });

                tracing::info!(
                    "Registered static route: {} {} (status {})",
                    route.method.as_str(),
                    path,
                    resp_status,
                );
                continue;
            }

            // ~keep Per-route lifecycle hooks are opt-in: only routes that name hooks via
            // ~keep `route.lifecycle_hooks` get any hook execution, resolved against the
            // ~keep server's registered `LifecycleHooks` (see `resolve_route_lifecycle_hooks`).
            // ~keep A route with no `lifecycle_hooks` config runs no hooks, even if hooks are
            // ~keep registered on `ServerConfig.lifecycle_hooks` — this is a deliberate
            // ~keep replacement of the old blanket-apply-to-every-route behavior, not an
            // ~keep oversight.
            let route_hooks: Option<Arc<crate::LifecycleHooks>> = match route.lifecycle_hooks.as_ref() {
                Some(config) => Some(Arc::new(resolve_route_lifecycle_hooks(
                    &path,
                    config,
                    hooks.as_deref(),
                )?)),
                None => None,
            };

            // Preflight (OPTIONS + `route.cors`) is a full bypass of the normal handler pipeline
            // (see `crate::cors::handle_preflight`), so none of the auth/CORS-simple-request
            // layers below make sense on it: per the CORS spec, preflight must succeed without
            // credentials, and it already gets complete CORS handling from `handle_preflight`
            // itself. ~keep
            let is_preflight_bypass = matches!(method, crate::Method::Options) && route.cors.is_some();

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
                            route_hooks.clone(),
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
                        route_hooks.clone(),
                        include_raw_query_params,
                        include_query_params_json,
                    )
                }
            };

            let method_router = if matches!(
                route.method,
                crate::Method::Post | crate::Method::Put | crate::Method::Patch
            ) && (route.expects_json_body || route.file_params.is_some())
            {
                method_router.layer(axum::middleware::from_fn_with_state(
                    crate::middleware::RouteInfo {
                        expects_json_body: route.expects_json_body,
                    },
                    crate::middleware::validate_content_type_middleware,
                ))
            } else {
                method_router
            };

            let method_router = if let Some(max_bytes) = route.body_limit {
                method_router.layer(axum::middleware::from_fn_with_state(
                    crate::middleware::BodyLimitState { max_bytes },
                    crate::middleware::body_limit_middleware,
                ))
            } else {
                method_router
            };

            // ~keep Compression mirrors the global `CompressionLayer` construction exactly, but
            // ~keep is nested *inside* any global compression layer (that one wraps the whole
            // ~keep app, outside this per-route stack). Because tower-http's compression is a
            // ~keep no-op once a response already carries `Content-Encoding`, a per-route layer
            // ~keep can add or customize compression when the global layer doesn't apply (or use
            // ~keep different settings) — but it cannot force compression OFF for a route while a
            // ~keep global compression layer is active, since that outer layer still runs
            // ~keep afterwards regardless of what this inner layer did. Same tighten-only
            // ~keep relationship as body_limit/timeout, just expressed through layer/predicate
            // ~keep behavior instead of an explicit min() comparison.
            let method_router: MethodRouter = if let Some(ref compression) = route.compression {
                let mut compression_layer = CompressionLayer::new();
                if !compression.gzip {
                    compression_layer = compression_layer.gzip(false);
                }
                if !compression.brotli {
                    compression_layer = compression_layer.br(false);
                }

                let min_threshold = compression.min_size.min(u64::MAX as usize) as u64;
                let predicate = SizeAbove::new(min_threshold)
                    .and(NotForContentType::GRPC)
                    .and(NotForContentType::IMAGES)
                    .and(NotForContentType::SSE);
                let compression_layer = compression_layer.compress_when(predicate);

                method_router.layer(compression_layer)
            } else {
                method_router
            };

            // ~keep CORS has no server-global equivalent (`ServerConfig` has no `cors` field —
            // ~keep CORS is per-route by design), so there is no tighten/loosen precedence
            // ~keep question here: a route either enforces CORS or it doesn't. This covers the
            // ~keep simple-request path; preflight is handled separately above.
            let method_router: MethodRouter = if let Some(ref cors_cfg) = route.cors {
                if is_preflight_bypass {
                    method_router
                } else {
                    method_router.layer(axum::middleware::from_fn_with_state(
                        crate::cors::CorsSimpleRequestState {
                            config: cors_cfg.clone(),
                        },
                        crate::cors::cors_simple_request_middleware,
                    ))
                }
            } else {
                method_router
            };

            // ~keep Per-route rate limiting runs an independent counter from any global
            // ~keep `GovernorLayer`; when both are present, whichever hits its own limit first
            // ~keep returns 429 — an AND relationship identical in spirit to body_limit/timeout's
            // ~keep "tighter wins" nesting, just enforced via two independent buckets rather than
            // ~keep a single compared value.
            let method_router: MethodRouter = if let Some(ref rate_limit) = route.rate_limit {
                if rate_limit.ip_based {
                    let governor_conf = Arc::new(
                        GovernorConfigBuilder::default()
                            .per_second(rate_limit.per_second)
                            .burst_size(rate_limit.burst)
                            .finish()
                            .ok_or_else(|| format!("route '{}': failed to create rate limiter", path))?,
                    );
                    method_router.layer(tower_governor::GovernorLayer::new(governor_conf))
                } else {
                    let governor_conf = Arc::new(
                        GovernorConfigBuilder::default()
                            .per_second(rate_limit.per_second)
                            .burst_size(rate_limit.burst)
                            .key_extractor(GlobalKeyExtractor)
                            .finish()
                            .ok_or_else(|| format!("route '{}': failed to create rate limiter", path))?,
                    );
                    method_router.layer(tower_governor::GovernorLayer::new(governor_conf))
                }
            } else {
                method_router
            };

            // ~keep Authorization is layered *inside* (added before, i.e. closer to the
            // ~keep handler than) JWT auth below, so it runs after JWT auth has populated the
            // ~keep `Claims` request extension it reads. A route with `authorization` but no
            // ~keep populated claims (no JWT ran) fails closed with 403 rather than silently
            // ~keep allowing the request.
            let method_router: MethodRouter = if !is_preflight_bypass && let Some(ref authz_cfg) = route.authorization {
                let authz_cfg = authz_cfg.clone();
                method_router.layer(axum::middleware::from_fn(move |req, next| {
                    crate::auth::authorization_middleware(authz_cfg.clone(), req, next)
                }))
            } else {
                method_router
            };

            // ~keep Per-route JWT auth only protects routes that opt in; it adds a requirement
            // ~keep on top of whatever the global `jwt_auth` layer (if any) already enforces
            // ~keep outside this per-route stack, but a route without `jwt_auth` configured is
            // ~keep NOT protected by this layer even if other routes in the same router are —
            // ~keep that per-route independence is the entire point of this wiring pass.
            let method_router: MethodRouter = if !is_preflight_bypass
                && let Some(ref jwt_cfg) = route.jwt_auth
                && let Some(jwt_config) = jwt_config_from_route(&path, jwt_cfg)?
            {
                method_router.layer(axum::middleware::from_fn(move |headers, req, next| {
                    crate::auth::jwt_auth_middleware(jwt_config.clone(), headers, req, next)
                }))
            } else {
                method_router
            };

            let method_router: MethodRouter = if !is_preflight_bypass
                && let Some(ref api_key_cfg) = route.api_key_auth
                && let Some(api_key_config) = api_key_config_from_route(&path, api_key_cfg)?
            {
                method_router.layer(axum::middleware::from_fn(move |headers, req, next| {
                    crate::auth::api_key_auth_middleware(api_key_config.clone(), headers, req, next)
                }))
            } else {
                method_router
            };

            // ~keep `RequestIdConfig::enabled == Some(true)` lets a route opt into request-id
            // ~keep generation/propagation even when the server-global default is off — that
            // ~keep direction works cleanly because this layer sits inside (closer to the
            // ~keep handler than) any global request-id layer. `Some(false)` is accepted and
            // ~keep round-trips, but deliberately does NOT try to strip a header the global
            // ~keep layer would add: the global `PropagateRequestIdLayer` wraps this per-route
            // ~keep stack from the outside, so it runs *after* this layer on the response path
            // ~keep and would simply re-add whatever an inner "strip" removed. Forcing
            // ~keep request-id off in the presence of a global layer is not achievable with
            // ~keep this layer ordering, so we don't pretend to support it.
            let method_router: MethodRouter = if let Some(request_id_cfg) = route.request_id
                && request_id_cfg.enabled
            {
                method_router
                    .layer::<_, std::convert::Infallible>(PropagateRequestIdLayer::x_request_id())
                    .layer::<_, std::convert::Infallible>(SetRequestIdLayer::x_request_id(MakeRequestUuid))
            } else {
                method_router
            };

            let method_router = if let Some(timeout_secs) = route.request_timeout_secs {
                method_router.layer(TimeoutLayer::with_status_code(
                    StatusCode::REQUEST_TIMEOUT,
                    Duration::from_secs(timeout_secs),
                ))
            } else {
                method_router
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
            if axum_path != "/" && !axum_path.ends_with('/') && !axum_path.contains("{*") {
                app = app.route(&format!("{}/", axum_path), router.clone());
            }
            app = app.route(&axum_path, router);
        }
    }

    if enable_http_trace {
        app = app.layer(TraceLayer::new_for_http());
    }

    if fast_router.has_routes() {
        let fast_router = Arc::new(fast_router);
        app = app.layer(axum::middleware::from_fn(
            move |req: axum::extract::Request, next: axum::middleware::Next| {
                let fast_router = Arc::clone(&fast_router);
                async move {
                    if let Some(resp) = fast_router.lookup(req.method(), req.uri().path()) {
                        return resp;
                    }
                    next.run(req).await
                }
            },
        ));
    }

    Ok(app)
}

/// Build router with handlers and apply middleware based on config
pub fn build_router_with_handlers_and_config(
    routes: Vec<RouteHandlerPair>,
    config: ServerConfig,
    route_metadata: Vec<crate::RouteMetadata>,
) -> Result<AxumRouter, String> {
    build_router_with_handlers_and_config_and_grpc(routes, config, route_metadata, None)
}

/// Build router with handlers, config, and an optional gRPC service registry.
pub(crate) fn build_router_with_handlers_and_config_and_grpc(
    routes: Vec<RouteHandlerPair>,
    config: ServerConfig,
    route_metadata: Vec<crate::RouteMetadata>,
    grpc_registry: Option<Arc<crate::grpc::GrpcRegistry>>,
) -> Result<AxumRouter, String> {
    #[cfg(all(feature = "di", debug_assertions))]
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

    if let (Some(grpc_config), Some(registry)) = (config.grpc.clone(), grpc_registry)
        && !registry.is_empty()
    {
        let state = GrpcMiddlewareState {
            registry,
            config: grpc_config,
        };
        app = app.layer(axum::middleware::from_fn_with_state(state, grpc_routing_middleware));
    }

    if config.jwt_auth.is_some() || config.api_key_auth.is_some() {
        app = app.layer(SetSensitiveRequestHeadersLayer::new([
            axum::http::header::AUTHORIZATION,
            axum::http::header::COOKIE,
        ]));
    }

    if let Some(ref compression) = config.compression {
        let mut compression_layer = CompressionLayer::new();
        if !compression.gzip {
            compression_layer = compression_layer.gzip(false);
        }
        if !compression.brotli {
            compression_layer = compression_layer.br(false);
        }

        let min_threshold = compression.min_size.min(u64::MAX as usize) as u64;
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

    if let Some(ref asyncapi_config) = config.asyncapi
        && asyncapi_config.enabled
    {
        use crate::asyncapi::{AsyncApiState, handle_asyncapi_json, handle_asyncapi_parse, handle_asyncapi_validate};

        let registered_spec = asyncapi_config.spec.as_ref().map(|s| Arc::new(s.clone()));
        let state = AsyncApiState { registered_spec };

        app = app
            .route("/asyncapi/parse", post(handle_asyncapi_parse))
            .route("/asyncapi/validate", post(handle_asyncapi_validate))
            .route("/asyncapi.json", get(handle_asyncapi_json).with_state(state));

        tracing::info!("AsyncAPI endpoints enabled: POST /asyncapi/parse, POST /asyncapi/validate, GET /asyncapi.json");
    }

    if let Some(ref jsonrpc_config) = config.jsonrpc
        && jsonrpc_config.enabled
        && let Some(registry) = jsonrpc_registry
    {
        use axum::response::Json;

        let jsonrpc_router = Arc::new(crate::jsonrpc::JsonRpcRouter::new(
            Arc::clone(&registry),
            jsonrpc_config.enable_batch,
            jsonrpc_config.max_batch_size,
        ));

        let state = Arc::new(crate::jsonrpc::JsonRpcState { router: jsonrpc_router });

        let endpoint_path = jsonrpc_config.endpoint_path.clone();
        app = app.route(&endpoint_path, post(crate::jsonrpc::handle_jsonrpc).with_state(state));
        let openrpc_spec = crate::jsonrpc::generate_openrpc_spec(&registry, &config)?;
        app = app.route("/openrpc.json", get(move || async move { Json(openrpc_spec) }));

        tracing::info!("JSON-RPC endpoint enabled at {}", endpoint_path);
        tracing::info!("OpenRPC documentation enabled at /openrpc.json");
    }

    Ok(app)
}

/// HTTP Server
pub struct Server;

impl Server {
    /// Build a server router with runtime handlers.
    ///
    /// Build router with trait-based handlers
    /// Routes are grouped by path before registration to support multiple HTTP methods
    /// for the same path (e.g., GET /data and POST /data). Axum requires that all methods
    /// for a path be merged into a single MethodRouter before calling `.route()`.
    pub fn with_handlers(
        config: ServerConfig,
        routes: Vec<(crate::Route, Arc<dyn Handler>)>,
    ) -> Result<AxumRouter, String> {
        let metadata: Vec<crate::RouteMetadata> = routes.iter().map(|(route, _)| route_to_metadata(route)).collect();
        build_router_with_handlers_and_config(routes, config, metadata)
    }

    /// Build a server router with runtime handlers and explicit metadata for OpenAPI.
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
        // ~keep Honor SPIKARD_SERVER_PORT when set so e2e harnesses (and any deployment
        // ~keep that injects a port) can bind a runtime-chosen port without an explicit
        // ~keep config call. Falls back to the configured port (default 8000).
        let port = std::env::var("SPIKARD_SERVER_PORT")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(config.port);
        let addr = format!("{}:{}", config.host, port);
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
    ///
    /// This function is idempotent - calling it multiple times is safe.
    /// It uses `try_init()` instead of `init()` to avoid panics when the
    /// global subscriber has already been set (e.g., by a language runtime
    /// or a previous call).
    pub fn init_logging() {
        let _ = tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "spikard=info,tower_http=info".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .try_init();
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
            compression: None,
            body_limit: None,
            request_timeout_secs: None,
            rate_limit: None,
            request_id: None,
            jwt_auth: None,
            api_key_auth: None,
            authorization: None,
            lifecycle_hooks: None,
            openrpc_spec: None,
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
            compression: None,
            body_limit: None,
            request_timeout_secs: None,
            rate_limit: None,
            request_id: None,
            jwt_auth: None,
            api_key_auth: None,
            authorization: None,
            lifecycle_hooks: None,
            openrpc_spec: None,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
        }
    }

    /// `route_to_metadata()` must carry every per-route middleware field through, not just the
    /// ones that existed before this field set grew — a dropped field here silently disables
    /// per-route middleware for anything that rebuilds `RouteMetadata` from a live `Route` (e.g.
    /// OpenAPI spec generation). This round-trips `Route -> RouteMetadata -> Route` and asserts
    /// every new field survives both hops.
    #[test]
    fn should_round_trip_new_per_route_middleware_fields_through_route_to_metadata() {
        let mut route = build_test_route("/secure", "GET", "get_secure", false);
        route.rate_limit = Some(crate::RateLimitConfig {
            per_second: 5,
            burst: 10,
            ip_based: false,
        });
        route.request_id = Some(crate::RequestIdConfig { enabled: true });
        route.jwt_auth = Some(crate::JwtAuthConfig {
            enabled: true,
            secret: Some("s3cr3t".to_string()),
            public_key: None,
            algorithm: "HS256".to_string(),
            audience: None,
            issuer: None,
            leeway: 0,
        });
        route.api_key_auth = Some(crate::ApiKeyAuthConfig {
            enabled: true,
            keys: vec!["k1".to_string()],
            header_name: "X-API-Key".to_string(),
        });
        route.authorization = Some(crate::AuthorizationConfig {
            required_roles: vec!["admin".to_string()],
            ..Default::default()
        });
        route.lifecycle_hooks = Some(crate::LifecycleHooksConfig {
            on_request: vec![crate::LifecycleHookRef {
                name: "audit_log".to_string(),
                handler: "run_audit_log".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        });
        route.openrpc_spec = Some(serde_json::json!({ "name": "get.secure" }));

        // `RateLimitConfig` doesn't implement `PartialEq`, so compare its fields directly rather
        // than adding a derive to an existing shared type just for this assertion.
        let assert_rate_limit_matches = |rate_limit: &Option<crate::RateLimitConfig>| {
            let rate_limit = rate_limit.as_ref().expect("rate_limit survives the round trip");
            assert_eq!(rate_limit.per_second, 5);
            assert_eq!(rate_limit.burst, 10);
            assert!(!rate_limit.ip_based);
        };

        let metadata = route_to_metadata(&route);

        assert_rate_limit_matches(&metadata.rate_limit);
        assert_eq!(metadata.request_id, route.request_id);
        assert_eq!(metadata.jwt_auth, route.jwt_auth);
        assert_eq!(metadata.api_key_auth, route.api_key_auth);
        assert_eq!(metadata.authorization, route.authorization);
        assert_eq!(metadata.lifecycle_hooks, route.lifecycle_hooks);
        assert_eq!(metadata.openrpc_spec, route.openrpc_spec);

        let registry = spikard_core::SchemaRegistry::new();
        let rebuilt = crate::Route::from_metadata(metadata, &registry).expect("metadata round-trips");

        assert_rate_limit_matches(&rebuilt.rate_limit);
        assert_eq!(rebuilt.request_id, route.request_id);
        assert_eq!(rebuilt.jwt_auth, route.jwt_auth);
        assert_eq!(rebuilt.api_key_auth, route.api_key_auth);
        assert_eq!(rebuilt.authorization, route.authorization);
        assert_eq!(rebuilt.lifecycle_hooks, route.lifecycle_hooks);
        assert_eq!(rebuilt.openrpc_spec, route.openrpc_spec);
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
            ..Default::default()
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
            ..Default::default()
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

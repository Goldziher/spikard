//! High-level Rust API for Spikard.
//!
//! This crate provides the ergonomic `App` interface that mirrors the
//! batteries-included experience available in the Python/Node/Ruby bindings.
//! Routes, DTO schemas, and advanced transports (WebSocket/SSE) are all backed
//! by the shared `spikard-http` runtime, ensuring identical validation and
//! middleware behaviour across languages.

pub mod upload;
pub mod validation;

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use axum::Router as AxumRouter;
pub use axum::body::Body;
pub use axum::http::Request;
use axum::http::StatusCode;
use axum::routing::get as axum_get;
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
#[cfg(feature = "di")]
use spikard_core::di;
pub use spikard_graphql::{FullSchemaConfig, GraphQLRouteConfig, QueryMutationConfig, QueryOnlyConfig, SchemaConfig};
#[cfg(not(target_arch = "wasm32"))]
use spikard_http::server::Server;
pub use spikard_http::{
    ApiKeyConfig, BackgroundJobError, BackgroundJobMetadata, BackgroundTaskConfig, CompressionConfig, CorsConfig,
    GrpcConfig, JsonRpcConfig, JwtConfig, LifecycleHook, LifecycleHooks, LifecycleHooksBuilder, Method, OpenApiConfig,
    RateLimitConfig, Response, Route, RouteMetadata, ServerConfig, SseEvent, StaticFilesConfig,
    cors::{add_cors_headers, handle_preflight, validate_cors_request},
    handler_response::HandlerResponse,
    handler_trait::HandlerResult,
    lifecycle::{HookResult, request_hook, response_hook},
    sse::SseEventProducer,
    websocket::WebSocketHandler,
};
pub use spikard_http::{JsonRpcMethodInfo, ProblemDetails};
pub use spikard_http::{RequestData, handler_trait::Handler};
use spikard_http::{
    SchemaRegistry,
    sse::{SseState, sse_handler},
    websocket::{WebSocketState, websocket_handler},
};

pub use upload::UploadFile;

/// Convert a binding-side handler outcome into the framework's `HandlerResult`.
///
/// Binding handler bridges return `Result<Response, BoxError>` after invoking
/// the host-language callable; this adapter folds the result into the
/// axum-compatible `HandlerResult` shape the trait dispatch expects.
#[doc(hidden)]
pub fn handler_result_from_response(
    outcome: Result<Response, Box<dyn std::error::Error + Send + Sync>>,
) -> HandlerResult {
    use axum::http::{HeaderName, HeaderValue, StatusCode};
    match outcome {
        Ok(response) => {
            let status = StatusCode::from_u16(response.status_code).unwrap_or(StatusCode::OK);
            let body = match &response.content {
                Some(serde_json::Value::String(s)) => Body::from(s.clone()),
                Some(value) => Body::from(value.to_string()),
                None => Body::empty(),
            };
            let mut builder = axum::http::Response::builder().status(status);
            for (k, v) in &response.headers {
                if let (Ok(name), Ok(value)) = (HeaderName::try_from(k), HeaderValue::try_from(v)) {
                    builder = builder.header(name, value);
                }
            }
            builder
                .body(body)
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub mod testing;
pub use testing::{
    GraphQLSubscriptionSnapshot, ResponseSnapshot, SnapshotError, TestClient, WebSocketConnection, WebSocketMessage,
};

/// Spikard application builder.
pub struct App {
    config: ServerConfig,
    registry: SchemaRegistry,
    routes: Vec<(Route, Arc<dyn Handler>)>,
    metadata: Vec<RouteMetadata>,
    attached_routers: Vec<AxumRouter>,
}

impl App {
    /// Create a new application with the default server configuration.
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: ServerConfig::default(),
            registry: SchemaRegistry::new(),
            routes: Vec::new(),
            metadata: Vec::new(),
            attached_routers: Vec::new(),
        }
    }

    /// Set the server configuration.
    #[must_use]
    pub fn config(mut self, config: ServerConfig) -> Self {
        self.config = config;
        self
    }

    /// Register an `on_request` lifecycle hook (runs before validation and handler dispatch).
    pub fn on_request(
        &mut self,
        hook: Arc<dyn LifecycleHook<axum::http::Request<Body>, axum::http::Response<Body>>>,
    ) -> &mut Self {
        self.ensure_lifecycle_hooks().add_on_request(hook);
        self
    }

    /// Register a `pre_validation` lifecycle hook (runs after `on_request`, before validation).
    pub fn pre_validation(
        &mut self,
        hook: Arc<dyn LifecycleHook<axum::http::Request<Body>, axum::http::Response<Body>>>,
    ) -> &mut Self {
        self.ensure_lifecycle_hooks().add_pre_validation(hook);
        self
    }

    /// Register a `pre_handler` lifecycle hook (runs after validation, before the handler).
    pub fn pre_handler(
        &mut self,
        hook: Arc<dyn LifecycleHook<axum::http::Request<Body>, axum::http::Response<Body>>>,
    ) -> &mut Self {
        self.ensure_lifecycle_hooks().add_pre_handler(hook);
        self
    }

    /// Register an `on_response` lifecycle hook (runs after a successful handler response).
    pub fn on_response(
        &mut self,
        hook: Arc<dyn LifecycleHook<axum::http::Request<Body>, axum::http::Response<Body>>>,
    ) -> &mut Self {
        self.ensure_lifecycle_hooks().add_on_response(hook);
        self
    }

    /// Register an `on_error` lifecycle hook (runs when the handler returns an error).
    pub fn on_error(
        &mut self,
        hook: Arc<dyn LifecycleHook<axum::http::Request<Body>, axum::http::Response<Body>>>,
    ) -> &mut Self {
        self.ensure_lifecycle_hooks().add_on_error(hook);
        self
    }

    /// Retrieve a mutable reference to the `LifecycleHooks`, creating it if absent.
    /// Registration always happens before `run()`, so the Arc is never shared here.
    fn ensure_lifecycle_hooks(&mut self) -> &mut LifecycleHooks {
        if self.config.lifecycle_hooks.is_none() {
            self.config.lifecycle_hooks = Some(Arc::new(LifecycleHooks::new()));
        }
        // Fast path: we hold the only reference.
        if Arc::get_mut(self.config.lifecycle_hooks.as_mut().unwrap()).is_some() {
            return Arc::get_mut(self.config.lifecycle_hooks.as_mut().unwrap()).unwrap();
        }
        // Slow path: reclaim the inner value from the shared Arc so we can mutate it.
        let hooks = Arc::try_unwrap(self.config.lifecycle_hooks.take().unwrap())
            .unwrap_or_else(|_| panic!("lifecycle_hooks Arc is shared during App construction"));
        self.config.lifecycle_hooks = Some(Arc::new(hooks));
        Arc::get_mut(self.config.lifecycle_hooks.as_mut().unwrap()).unwrap()
    }

    /// Register a route using the provided builder and handler function.
    ///
    /// # Errors
    ///
    /// Returns an error if route construction fails or if the handler registration fails.
    pub fn route<H>(&mut self, builder: RouteBuilder, handler: H) -> std::result::Result<&mut Self, AppError>
    where
        H: IntoHandler + 'static,
    {
        let metadata = builder.into_metadata();
        let route = Route::from_metadata(metadata.clone(), &self.registry).map_err(AppError::Route)?;
        let handler = handler.into_handler();
        self.routes.push((route, handler));
        self.metadata.push(metadata);
        Ok(self)
    }

    /// Register a WebSocket handler for the specified path.
    pub fn websocket<H>(&mut self, path: impl Into<String>, handler: H) -> &mut Self
    where
        H: WebSocketHandler + Send + Sync + 'static,
    {
        let _ = self.websocket_with_schemas(path, handler, None, None);
        self
    }

    /// Register a WebSocket handler with optional message/response schemas.
    ///
    /// # Errors
    ///
    /// Returns an error if the WebSocket state construction fails.
    pub fn websocket_with_schemas<H>(
        &mut self,
        path: impl Into<String>,
        handler: H,
        message_schema: Option<serde_json::Value>,
        response_schema: Option<serde_json::Value>,
    ) -> std::result::Result<&mut Self, AppError>
    where
        H: WebSocketHandler + Send + Sync + 'static,
    {
        let state = if message_schema.is_some() || response_schema.is_some() {
            WebSocketState::with_schemas(handler, message_schema, response_schema).map_err(AppError::Route)?
        } else {
            WebSocketState::new(handler)
        };

        Ok(self.register_stateful_ws_route(path, state))
    }

    /// Register an SSE producer for the specified path.
    pub fn sse<P>(&mut self, path: impl Into<String>, producer: P) -> &mut Self
    where
        P: SseEventProducer + Send + Sync + 'static,
    {
        let _ = self.sse_with_schema(path, producer, None);
        self
    }

    /// Register an SSE producer with optional JSON schema.
    ///
    /// # Errors
    ///
    /// Returns an error if the SSE state construction fails.
    pub fn sse_with_schema<P>(
        &mut self,
        path: impl Into<String>,
        producer: P,
        event_schema: Option<serde_json::Value>,
    ) -> std::result::Result<&mut Self, AppError>
    where
        P: SseEventProducer + Send + Sync + 'static,
    {
        let state = if let Some(schema) = event_schema {
            SseState::with_schema(producer, Some(schema)).map_err(AppError::Route)?
        } else {
            SseState::new(producer)
        };

        Ok(self.register_stateful_sse_route(path, state))
    }

    /// Internal helper: register a WebSocket state with route normalization.
    fn register_stateful_ws_route<H: WebSocketHandler + Send + Sync + 'static>(
        &mut self,
        path: impl Into<String>,
        state: WebSocketState<H>,
    ) -> &mut Self {
        let path = normalize_path(path.into());
        let router = AxumRouter::new().route(&path, axum_get(websocket_handler::<H>).with_state(state));
        self.attached_routers.push(router);
        self
    }

    /// Internal helper: register an SSE state with route normalization.
    fn register_stateful_sse_route<P: SseEventProducer + Send + Sync + 'static>(
        &mut self,
        path: impl Into<String>,
        state: SseState<P>,
    ) -> &mut Self {
        let path = normalize_path(path.into());
        let router = AxumRouter::new().route(&path, axum_get(sse_handler::<P>).with_state(state));
        self.attached_routers.push(router);
        self
    }

    /// Attach an existing Axum router to this application, returning ownership.
    #[must_use]
    pub fn merge_axum_router(mut self, router: AxumRouter) -> Self {
        self.attached_routers.push(router);
        self
    }

    /// Attach an Axum router using a mutable reference for incremental configuration.
    pub fn attach_axum_router(&mut self, router: AxumRouter) -> &mut Self {
        self.attached_routers.push(router);
        self
    }

    /// Build the underlying Axum router.
    ///
    /// # Errors
    ///
    /// Returns an error if server or router construction fails.
    pub fn into_router(self) -> std::result::Result<axum::Router, AppError> {
        let Self {
            config,
            routes,
            metadata,
            attached_routers,
            ..
        } = self;
        let mut router = Server::with_handlers_and_metadata(config, routes, metadata).map_err(AppError::Server)?;
        for extra in attached_routers {
            router = router.merge(extra);
        }
        Ok(router)
    }

    /// Decompose the application into its Axum router and server configuration.
    ///
    /// This is the low-level escape hatch used by the C FFI layer to start the
    /// server on a background thread while retaining the bind address from the
    /// caller-supplied [`ServerConfig`].  Prefer [`App::run`] for normal use.
    ///
    /// # Errors
    ///
    /// Returns an error if router construction fails.
    pub fn into_router_and_config(self) -> std::result::Result<(axum::Router, ServerConfig), AppError> {
        let Self {
            config,
            routes,
            metadata,
            attached_routers,
            ..
        } = self;
        let mut router =
            Server::with_handlers_and_metadata(config.clone(), routes, metadata).map_err(AppError::Server)?;
        for extra in attached_routers {
            router = router.merge(extra);
        }
        Ok((router, config))
    }

    /// Run the HTTP server using the configured routes.
    ///
    /// # Errors
    ///
    /// Returns an error if server construction or execution fails.
    pub async fn run(self) -> std::result::Result<(), AppError> {
        let Self {
            config,
            routes,
            metadata,
            attached_routers,
            ..
        } = self;
        let mut router =
            Server::with_handlers_and_metadata(config.clone(), routes, metadata).map_err(AppError::Server)?;
        for extra in attached_routers {
            router = router.merge(extra);
        }
        Server::run_with_config(router, config)
            .await
            .map_err(|err| AppError::Server(err.to_string()))
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for defining a route.
#[derive(Clone)]
pub struct RouteBuilder {
    method: Method,
    path: String,
    handler_name: String,
    request_schema: Option<serde_json::Value>,
    response_schema: Option<serde_json::Value>,
    parameter_schema: Option<serde_json::Value>,
    file_params: Option<serde_json::Value>,
    cors: Option<CorsConfig>,
    compression: Option<CompressionConfig>,
    is_async: bool,
    #[cfg(feature = "di")]
    handler_dependencies: Option<Vec<String>>,
}

impl RouteBuilder {
    /// Create a new builder for the provided HTTP method and path.
    #[must_use]
    pub fn new(method: Method, path: impl Into<String>) -> Self {
        let path = path.into();
        let handler_name = default_handler_name(&method, &path);
        Self {
            method,
            path,
            handler_name,
            request_schema: None,
            response_schema: None,
            parameter_schema: None,
            file_params: None,
            cors: None,
            compression: None,
            is_async: true,
            #[cfg(feature = "di")]
            handler_dependencies: None,
        }
    }

    /// Assign an explicit handler name.
    #[must_use]
    pub fn handler_name(mut self, name: impl Into<String>) -> Self {
        self.handler_name = name.into();
        self
    }

    /// Attach a request body schema derived from the provided DTO type.
    #[must_use]
    pub fn request_body<T: JsonSchema>(mut self) -> Self {
        self.request_schema = Some(schema_for::<T>());
        self
    }

    /// Attach a response body schema derived from the provided DTO type.
    #[must_use]
    pub fn response_body<T: JsonSchema>(mut self) -> Self {
        self.response_schema = Some(schema_for::<T>());
        self
    }

    /// Attach request parameter schema derived from the provided DTO type.
    #[must_use]
    pub fn params<T: JsonSchema>(mut self) -> Self {
        self.parameter_schema = Some(schema_for::<T>());
        self
    }

    /// Provide a raw JSON schema for the request body.
    #[must_use]
    pub fn request_schema_json(mut self, schema: serde_json::Value) -> Self {
        self.request_schema = Some(schema);
        self
    }

    /// Provide a raw JSON schema for the response body.
    #[must_use]
    pub fn response_schema_json(mut self, schema: serde_json::Value) -> Self {
        self.response_schema = Some(schema);
        self
    }

    /// Provide a raw JSON schema for request parameters.
    #[must_use]
    pub fn params_schema_json(mut self, schema: serde_json::Value) -> Self {
        self.parameter_schema = Some(schema);
        self
    }

    /// Provide multipart file parameter configuration.
    #[must_use]
    pub fn file_params_json(mut self, schema: serde_json::Value) -> Self {
        self.file_params = Some(schema);
        self
    }

    /// Attach a CORS configuration for this route.
    #[must_use]
    pub fn cors(mut self, cors: CorsConfig) -> Self {
        self.cors = Some(cors);
        self
    }

    /// Attach a compression configuration for this route.
    #[must_use]
    pub const fn compression(mut self, compression: CompressionConfig) -> Self {
        self.compression = Some(compression);
        self
    }

    /// Mark the route as synchronous.
    #[must_use]
    pub const fn sync(mut self) -> Self {
        self.is_async = false;
        self
    }

    /// Declare the dependency keys that must be resolved before this handler runs.
    #[cfg(feature = "di")]
    #[must_use]
    pub fn handler_dependencies(mut self, dependencies: Vec<String>) -> Self {
        self.handler_dependencies = Some(dependencies);
        self
    }

    fn into_metadata(self) -> RouteMetadata {
        #[cfg(feature = "di")]
        {
            RouteMetadata {
                method: self.method.to_string(),
                path: self.path,
                handler_name: self.handler_name,
                request_schema: self.request_schema,
                response_schema: self.response_schema,
                parameter_schema: self.parameter_schema,
                file_params: self.file_params,
                is_async: self.is_async,
                cors: self.cors,
                compression: self.compression,
                body_param_name: None,
                handler_dependencies: self.handler_dependencies,
                jsonrpc_method: None,
                static_response: None,
            }
        }
        #[cfg(not(feature = "di"))]
        {
            RouteMetadata {
                method: self.method.to_string(),
                path: self.path,
                handler_name: self.handler_name,
                request_schema: self.request_schema,
                response_schema: self.response_schema,
                parameter_schema: self.parameter_schema,
                file_params: self.file_params,
                is_async: self.is_async,
                cors: self.cors,
                compression: self.compression,
                body_param_name: None,
                jsonrpc_method: None,
                static_response: None,
            }
        }
    }
}

macro_rules! http_method {
    (
        $(#[$meta:meta])*
        $name:ident,
        $method:expr
    ) => {
        $(#[$meta])*
        pub fn $name(path: impl Into<String>) -> RouteBuilder {
            RouteBuilder::new($method, path)
        }
    };
}

http_method!(
    /// Convenience helper for building a GET route.
    get,
    Method::Get
);

http_method!(
    /// Convenience helper for building a POST route.
    post,
    Method::Post
);

http_method!(
    /// Convenience helper for building a PUT route.
    put,
    Method::Put
);

http_method!(
    /// Convenience helper for building a PATCH route.
    patch,
    Method::Patch
);

http_method!(
    /// Convenience helper for building a DELETE route.
    delete,
    Method::Delete
);

fn default_handler_name(method: &Method, path: &str) -> String {
    let prefix = method.as_str().to_lowercase();
    let suffix = sanitize_identifier(path);
    format!("{prefix}_{suffix}")
}

fn sanitize_identifier(input: &str) -> String {
    let mut ident = input
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>();
    while ident.contains("__") {
        ident = ident.replace("__", "_");
    }
    ident.trim_matches('_').to_string()
}

fn schema_for<T: JsonSchema>() -> serde_json::Value {
    let root = schemars::schema_for!(T);
    match serde_json::to_value(root) {
        Ok(value) => value.get("schema").cloned().unwrap_or(value),
        Err(e) => {
            eprintln!("warning: failed to serialize schema: {e}, returning null");
            serde_json::Value::Null
        }
    }
}

fn normalize_path(path: String) -> String {
    if path.starts_with('/') {
        path
    } else {
        format!("/{path}")
    }
}

/// Error type for application builder operations.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// Route registration failed.
    #[error("Failed to register route: {0}")]
    Route(String),
    /// Server/router construction failed.
    #[error("Failed to build server: {0}")]
    Server(String),
    /// Failed to extract DTO from the request context.
    #[error("Failed to decode payload: {0}")]
    Decode(String),
}

impl From<AppError> for (StatusCode, String) {
    fn from(err: AppError) -> Self {
        match err {
            AppError::Route(msg) | AppError::Server(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Decode(msg) => (StatusCode::BAD_REQUEST, msg),
        }
    }
}

/// Wrapper around the raw request and validated metadata.
pub struct RequestContext {
    request: Request<Body>,
    data: RequestData,
}

impl RequestContext {
    const fn new(request: Request<Body>, data: RequestData) -> Self {
        Self { request, data }
    }

    /// Borrow the underlying HTTP request.
    #[must_use]
    pub const fn request(&self) -> &Request<Body> {
        &self.request
    }

    /// Deserialize the JSON request body into the provided type.
    ///
    /// # Errors
    ///
    /// Returns an error if the request body cannot be deserialized into the provided type.
    pub fn json<T: DeserializeOwned>(&self) -> std::result::Result<T, AppError> {
        if !self.data.body.is_null() {
            serde_json::from_value((*self.data.body).clone()).map_err(|err| AppError::Decode(err.to_string()))
        } else if let Some(raw_bytes) = &self.data.raw_body {
            serde_json::from_slice(raw_bytes).map_err(|err| AppError::Decode(err.to_string()))
        } else {
            serde_json::from_value((*self.data.body).clone()).map_err(|err| AppError::Decode(err.to_string()))
        }
    }

    /// Deserialize query parameters into the provided type.
    ///
    /// # Errors
    ///
    /// Returns an error if the query parameters cannot be deserialized into the provided type.
    pub fn query<T: DeserializeOwned>(&self) -> std::result::Result<T, AppError> {
        serde_json::from_value((*self.data.query_params).clone()).map_err(|err| AppError::Decode(err.to_string()))
    }

    /// Borrow the parsed query parameters as JSON.
    #[must_use]
    pub fn query_value(&self) -> &serde_json::Value {
        &self.data.query_params
    }

    /// Borrow the raw query parameter map (string inputs as received on the wire).
    #[must_use]
    pub fn raw_query_params(&self) -> &HashMap<String, Vec<String>> {
        &self.data.raw_query_params
    }

    /// Extract typed path parameters into the provided type.
    ///
    /// # Errors
    ///
    /// Returns an error if the path parameters cannot be serialized or deserialized.
    pub fn path<T: DeserializeOwned>(&self) -> std::result::Result<T, AppError> {
        let value = serde_json::to_value(&*self.data.path_params).map_err(|err| AppError::Decode(err.to_string()))?;
        serde_json::from_value(value).map_err(|err| AppError::Decode(err.to_string()))
    }

    /// Borrow the raw path parameter map.
    #[must_use]
    pub fn path_params(&self) -> &HashMap<String, String> {
        &self.data.path_params
    }

    /// Extract a raw path parameter by name.
    #[must_use]
    pub fn path_param(&self, name: &str) -> Option<&str> {
        self.data.path_params.get(name).map(String::as_str)
    }

    /// Return a header value (case-insensitive).
    #[must_use]
    pub fn header(&self, name: &str) -> Option<&str> {
        self.data.headers.get(&name.to_ascii_lowercase()).map(String::as_str)
    }

    /// Borrow the normalized headers map.
    #[must_use]
    pub fn headers_map(&self) -> &HashMap<String, String> {
        &self.data.headers
    }

    /// Return a cookie value.
    #[must_use]
    pub fn cookie(&self, name: &str) -> Option<&str> {
        self.data.cookies.get(name).map(String::as_str)
    }

    /// Borrow the cookies map.
    #[must_use]
    pub fn cookies_map(&self) -> &HashMap<String, String> {
        &self.data.cookies
    }

    /// Borrow the raw JSON request body.
    #[must_use]
    pub fn body_value(&self) -> &serde_json::Value {
        &self.data.body
    }

    /// Borrow resolved dependencies for this request (if DI is enabled).
    #[cfg(feature = "di")]
    #[must_use]
    pub fn dependencies(&self) -> Option<Arc<di::ResolvedDependencies>> {
        self.data.dependencies.as_ref().map(Arc::clone)
    }

    /// Return the HTTP method.
    #[must_use]
    pub fn method(&self) -> &str {
        &self.data.method
    }

    /// Return the request path.
    #[must_use]
    pub fn path_str(&self) -> &str {
        &self.data.path
    }
}

/// Convert user-facing handler functions into the low-level `Handler` trait.
pub trait IntoHandler {
    /// Convert this value into a shared request handler.
    fn into_handler(self) -> Arc<dyn Handler>;
}

impl<F, Fut> IntoHandler for F
where
    F: Send + Sync + 'static + Fn(RequestContext) -> Fut,
    Fut: Future<Output = HandlerResult> + Send + 'static,
{
    fn into_handler(self) -> Arc<dyn Handler> {
        Arc::new(FnHandler { inner: self })
    }
}

impl IntoHandler for Arc<dyn Handler> {
    fn into_handler(self) -> Arc<dyn Handler> {
        self
    }
}

struct FnHandler<F> {
    inner: F,
}

impl<F, Fut> Handler for FnHandler<F>
where
    F: Send + Sync + 'static + Fn(RequestContext) -> Fut,
    Fut: Future<Output = HandlerResult> + Send + 'static,
{
    fn call(&self, req: Request<Body>, data: RequestData) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        let ctx = RequestContext::new(req, data);
        Box::pin((self.inner)(ctx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Request, StatusCode};
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use tower::util::ServiceExt;

    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
    struct Greeting {
        message: String,
    }

    #[test]
    fn sanitize_identifier_handles_complex_path() {
        assert_eq!(
            sanitize_identifier("/api/v2/{resource}-{id}/action"),
            "api_v2_resource_id_action"
        );
    }

    #[test]
    fn normalize_path_adds_leading_slash() {
        assert_eq!(normalize_path("users".to_string()), "/users");
        assert_eq!(normalize_path("/users".to_string()), "/users");
    }

    #[test]
    fn default_handler_name_includes_method_prefix() {
        assert_eq!(default_handler_name(&Method::Get, "/items/{id}"), "get_items_id");
        assert_eq!(default_handler_name(&Method::Post, "items"), "post_items");
    }

    #[test]
    fn schema_for_returns_embedded_schema_object() {
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
        struct Payload {
            message: String,
        }

        let schema = schema_for::<Payload>();
        assert!(schema.is_object());
        assert_eq!(schema.get("type").and_then(|v| v.as_str()), Some("object"));
        assert!(schema.get("properties").is_some());
    }

    #[test]
    fn route_builder_sets_defaults_and_metadata() {
        let builder = post("items").sync();
        let meta = builder.into_metadata();
        assert_eq!(meta.method, "POST");
        assert_eq!(meta.path, "items");
        assert_eq!(meta.handler_name, "post_items");
        assert!(!meta.is_async);
        assert!(meta.request_schema.is_none());
        assert!(meta.response_schema.is_none());
    }

    #[test]
    fn app_error_maps_to_status_code_and_message() {
        let (status, msg): (StatusCode, String) = AppError::Decode("bad json".to_string()).into();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(msg, "bad json");
    }

    #[tokio::test]
    async fn registers_route_with_schema() {
        let mut app = App::new();
        app.route(
            post("/hello").request_body::<Greeting>().response_body::<Greeting>(),
            |ctx: RequestContext| async move {
                let body: Greeting = ctx.json()?;
                let response = serde_json::to_value(body).unwrap();
                Ok(axum::http::Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(Body::from(response.to_string()))
                    .unwrap())
            },
        )
        .unwrap();

        assert_eq!(app.metadata.len(), 1);
        let meta = &app.metadata[0];
        assert!(meta.request_schema.is_some());
        assert!(meta.response_schema.is_some());
        assert!(meta.parameter_schema.is_none());
    }

    #[test]
    fn request_context_extracts_and_accesses_all_fields() {
        let mut headers = std::collections::HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        headers.insert("authorization".to_string(), "Bearer token123".to_string());

        let mut cookies = std::collections::HashMap::new();
        cookies.insert("session_id".to_string(), "abc123".to_string());

        let mut path_params = std::collections::HashMap::new();
        path_params.insert("id".to_string(), "123".to_string());

        let request = Request::builder()
            .uri("http://localhost/users/123")
            .body(Body::empty())
            .unwrap();

        let data = RequestData {
            method: "POST".to_string(),
            path: "/users/{id}".to_string(),
            headers: std::sync::Arc::new(headers),
            cookies: std::sync::Arc::new(cookies),
            query_params: std::sync::Arc::new(serde_json::Value::Object(serde_json::Map::new())),
            validated_params: None,
            raw_query_params: std::sync::Arc::new(HashMap::new()),
            path_params: std::sync::Arc::new(path_params),
            body: std::sync::Arc::new(serde_json::Value::Null),
            raw_body: None,
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let ctx = RequestContext::new(request, data);

        assert_eq!(ctx.header("content-type"), Some("application/json"));
        assert_eq!(ctx.header("Content-Type"), Some("application/json"));
        assert_eq!(ctx.header("authorization"), Some("Bearer token123"));

        assert_eq!(ctx.cookie("session_id"), Some("abc123"));
        assert_eq!(ctx.cookie("nonexistent"), None);

        assert_eq!(ctx.path_param("id"), Some("123"));
        assert_eq!(ctx.path_param("missing"), None);

        assert_eq!(ctx.method(), "POST");
        assert_eq!(ctx.path_str(), "/users/{id}");
    }

    struct EchoWebSocket;

    impl WebSocketHandler for EchoWebSocket {
        async fn handle_message(&self, message: serde_json::Value) -> Option<serde_json::Value> {
            Some(message)
        }
    }

    #[tokio::test]
    async fn websocket_routes_are_registered() {
        let mut app = App::new();
        app.websocket("/ws", EchoWebSocket);
        let router = app.into_router().unwrap();
        let request = Request::builder()
            .uri("http://localhost/ws")
            .header("connection", "Upgrade")
            .header("upgrade", "websocket")
            .header("sec-websocket-version", "13")
            .header("sec-websocket-key", "dGhlIHNhbXBsZSBub25jZQ==")
            .body(Body::empty())
            .unwrap();
        let response = router.oneshot(request).await.unwrap();
        assert!(
            response.status() == StatusCode::SWITCHING_PROTOCOLS || response.status() == StatusCode::UPGRADE_REQUIRED
        );
    }

    struct DummyProducer;

    impl SseEventProducer for DummyProducer {
        async fn next_event(&self) -> Option<SseEvent> {
            Some(SseEvent::new(json!({
                "message": "hello"
            })))
        }
    }

    #[tokio::test]
    async fn sse_routes_are_registered() {
        let mut app = App::new();
        app.sse("/events", DummyProducer);
        let router = app.into_router().unwrap();
        let request = Request::builder()
            .uri("http://localhost/events")
            .body(Body::empty())
            .unwrap();
        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}

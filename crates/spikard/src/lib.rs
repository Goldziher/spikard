//! High-level Rust API for Spikard.
//!
//! This crate provides the ergonomic `App` interface that mirrors the
//! batteries-included experience available in the Python/Node/Ruby bindings.
//! Routes, DTO schemas, and advanced transports (WebSocket/SSE) are all backed
//! by the shared `spikard-http` runtime, ensuring identical validation and
//! middleware behaviour across languages.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use axum::http::{Request, StatusCode};
use axum::routing::get as axum_get;
use axum::{Router as AxumRouter, body::Body};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde_json::Value;
pub use spikard_http::{
    CompressionConfig, CorsConfig, LifecycleHook, LifecycleHooks, LifecycleHooksBuilder, Method, RateLimitConfig,
    ServerConfig, StaticFilesConfig,
    cors::{add_cors_headers, handle_preflight, validate_cors_request},
    handler_response::HandlerResponse,
    handler_trait::HandlerResult,
    lifecycle::{HookResult, request_hook, response_hook},
    sse::{SseEvent, SseEventProducer},
    websocket::WebSocketHandler,
};
use spikard_http::{
    Route, RouteMetadata, SchemaRegistry, Server,
    handler_trait::{Handler, RequestData},
    sse::{SseState, sse_handler},
    websocket::{WebSocketState, websocket_handler},
};

pub mod testing {
    use super::{App, AppError};
    use axum::Router as AxumRouter;
    use axum::body::Body;
    use axum::http::Request;
    use axum_test::{TestServer as AxumTestServer, TestServerConfig, Transport};
    pub use spikard_http::testing::{
        MultipartFilePart, ResponseSnapshot, SnapshotError, SseEvent, SseStream, WebSocketConnection, WebSocketMessage,
        build_multipart_body, encode_urlencoded_body,
    };

    /// Spikard-native test server wrapper that hides the Axum test harness.
    ///
    /// Tests can build an `App`, convert it into a `TestServer`, and then issue
    /// HTTP/SSE/WebSocket requests without touching `axum-test` directly.
    pub struct TestServer {
        mock_server: AxumTestServer,
        http_server: AxumTestServer,
    }

    impl TestServer {
        /// Build a test server from an `App`.
        pub fn from_app(app: App) -> Result<Self, AppError> {
            let router = app.into_router()?;
            Self::from_router(router)
        }

        /// Build a test server from an Axum router.
        pub fn from_router(router: AxumRouter) -> Result<Self, AppError> {
            let mock_server = AxumTestServer::new(router.clone()).map_err(|err| AppError::Server(err.to_string()))?;
            let config = TestServerConfig {
                transport: Some(Transport::HttpRandomPort),
                ..Default::default()
            };
            let http_server =
                AxumTestServer::new_with_config(router, config).map_err(|err| AppError::Server(err.to_string()))?;
            Ok(Self {
                mock_server,
                http_server,
            })
        }

        /// Execute an HTTP request and return a snapshot of the response.
        pub async fn call(&self, request: Request<Body>) -> Result<ResponseSnapshot, SnapshotError> {
            let response = spikard_http::testing::call_test_server(&self.mock_server, request).await;
            spikard_http::testing::snapshot_response(response).await
        }

        /// Open a WebSocket connection for the provided path.
        pub async fn connect_websocket(&self, path: &str) -> WebSocketConnection {
            spikard_http::testing::connect_websocket(&self.http_server, path).await
        }
    }
}

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
    pub fn config(mut self, config: ServerConfig) -> Self {
        self.config = config;
        self
    }

    /// Register a route using the provided builder and handler function.
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

        let path = normalize_path(path.into());
        let router = AxumRouter::new().route(&path, axum_get(websocket_handler::<H>).with_state(state));
        self.attached_routers.push(router);
        Ok(self)
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

        let path = normalize_path(path.into());
        let router = AxumRouter::new().route(&path, axum_get(sse_handler::<P>).with_state(state));
        self.attached_routers.push(router);
        Ok(self)
    }

    /// Attach an existing Axum router to this application, returning ownership.
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
    pub fn into_router(self) -> std::result::Result<axum::Router, AppError> {
        let App {
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

    /// Run the HTTP server using the configured routes.
    pub async fn run(self) -> std::result::Result<(), AppError> {
        let App {
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
pub struct RouteBuilder {
    method: Method,
    path: String,
    handler_name: String,
    request_schema: Option<Value>,
    response_schema: Option<Value>,
    parameter_schema: Option<Value>,
    file_params: Option<Value>,
    cors: Option<CorsConfig>,
    is_async: bool,
}

impl RouteBuilder {
    /// Create a new builder for the provided HTTP method and path.
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
            is_async: true,
        }
    }

    /// Assign an explicit handler name.
    pub fn handler_name(mut self, name: impl Into<String>) -> Self {
        self.handler_name = name.into();
        self
    }

    /// Attach a request body schema derived from the provided DTO type.
    pub fn request_body<T: JsonSchema>(mut self) -> Self {
        self.request_schema = Some(schema_for::<T>());
        self
    }

    /// Attach a response body schema derived from the provided DTO type.
    pub fn response_body<T: JsonSchema>(mut self) -> Self {
        self.response_schema = Some(schema_for::<T>());
        self
    }

    /// Attach request parameter schema derived from the provided DTO type.
    pub fn params<T: JsonSchema>(mut self) -> Self {
        self.parameter_schema = Some(schema_for::<T>());
        self
    }

    /// Provide a raw JSON schema for the request body.
    pub fn request_schema_json(mut self, schema: Value) -> Self {
        self.request_schema = Some(schema);
        self
    }

    /// Provide a raw JSON schema for the response body.
    pub fn response_schema_json(mut self, schema: Value) -> Self {
        self.response_schema = Some(schema);
        self
    }

    /// Provide a raw JSON schema for request parameters.
    pub fn params_schema_json(mut self, schema: Value) -> Self {
        self.parameter_schema = Some(schema);
        self
    }

    /// Provide multipart file parameter configuration.
    pub fn file_params_json(mut self, schema: Value) -> Self {
        self.file_params = Some(schema);
        self
    }

    /// Attach a CORS configuration for this route.
    pub fn cors(mut self, cors: CorsConfig) -> Self {
        self.cors = Some(cors);
        self
    }

    /// Mark the route as synchronous.
    pub fn sync(mut self) -> Self {
        self.is_async = false;
        self
    }

    fn into_metadata(self) -> RouteMetadata {
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
        }
    }
}

/// Convenience helper for building a GET route.
pub fn get(path: impl Into<String>) -> RouteBuilder {
    RouteBuilder::new(Method::Get, path)
}

/// Convenience helper for building a POST route.
pub fn post(path: impl Into<String>) -> RouteBuilder {
    RouteBuilder::new(Method::Post, path)
}

/// Convenience helper for building a PUT route.
pub fn put(path: impl Into<String>) -> RouteBuilder {
    RouteBuilder::new(Method::Put, path)
}

/// Convenience helper for building a PATCH route.
pub fn patch(path: impl Into<String>) -> RouteBuilder {
    RouteBuilder::new(Method::Patch, path)
}

/// Convenience helper for building a DELETE route.
pub fn delete(path: impl Into<String>) -> RouteBuilder {
    RouteBuilder::new(Method::Delete, path)
}

fn default_handler_name(method: &Method, path: &str) -> String {
    let prefix = method.as_str().to_lowercase();
    let suffix = sanitize_identifier(path);
    format!("{}_{}", prefix, suffix)
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

fn schema_for<T: JsonSchema>() -> Value {
    let root = schemars::schema_for!(T);
    let value = serde_json::to_value(root).expect("Schema serialization to succeed");
    value.get("schema").cloned().unwrap_or(value)
}

fn normalize_path(path: String) -> String {
    if path.starts_with('/') {
        path
    } else {
        format!("/{}", path)
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
            AppError::Route(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Server(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
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
    fn new(request: Request<Body>, data: RequestData) -> Self {
        Self { request, data }
    }

    /// Borrow the underlying HTTP request.
    pub fn request(&self) -> &Request<Body> {
        &self.request
    }

    /// Deserialize the JSON request body into the provided type.
    pub fn json<T: DeserializeOwned>(&self) -> std::result::Result<T, AppError> {
        serde_json::from_value(self.data.body.clone()).map_err(|err| AppError::Decode(err.to_string()))
    }

    /// Deserialize query parameters into the provided type.
    pub fn query<T: DeserializeOwned>(&self) -> std::result::Result<T, AppError> {
        serde_json::from_value(self.data.query_params.clone()).map_err(|err| AppError::Decode(err.to_string()))
    }

    /// Borrow the parsed query parameters as JSON.
    pub fn query_value(&self) -> &Value {
        &self.data.query_params
    }

    /// Borrow the raw query parameter map (string inputs as received on the wire).
    pub fn raw_query_params(&self) -> &HashMap<String, Vec<String>> {
        &self.data.raw_query_params
    }

    /// Extract typed path parameters into the provided type.
    pub fn path<T: DeserializeOwned>(&self) -> std::result::Result<T, AppError> {
        let value = serde_json::to_value(&*self.data.path_params).map_err(|err| AppError::Decode(err.to_string()))?;
        serde_json::from_value(value).map_err(|err| AppError::Decode(err.to_string()))
    }

    /// Borrow the raw path parameter map.
    pub fn path_params(&self) -> &HashMap<String, String> {
        &self.data.path_params
    }

    /// Extract a raw path parameter by name.
    pub fn path_param(&self, name: &str) -> Option<&str> {
        self.data.path_params.get(name).map(|s| s.as_str())
    }

    /// Return a header value (case-insensitive).
    pub fn header(&self, name: &str) -> Option<&str> {
        self.data.headers.get(&name.to_ascii_lowercase()).map(|s| s.as_str())
    }

    /// Borrow the normalized headers map.
    pub fn headers_map(&self) -> &HashMap<String, String> {
        &self.data.headers
    }

    /// Return a cookie value.
    pub fn cookie(&self, name: &str) -> Option<&str> {
        self.data.cookies.get(name).map(|s| s.as_str())
    }

    /// Borrow the cookies map.
    pub fn cookies_map(&self) -> &HashMap<String, String> {
        &self.data.cookies
    }

    /// Borrow the raw JSON request body.
    pub fn body_value(&self) -> &Value {
        &self.data.body
    }

    /// Return the HTTP method.
    pub fn method(&self) -> &str {
        &self.data.method
    }

    /// Return the request path.
    pub fn path_str(&self) -> &str {
        &self.data.path
    }
}

/// Convert user-facing handler functions into the low-level `Handler` trait.
pub trait IntoHandler {
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

    struct EchoWebSocket;

    impl WebSocketHandler for EchoWebSocket {
        fn handle_message(&self, message: serde_json::Value) -> impl Future<Output = Option<serde_json::Value>> + Send {
            async move { Some(message) }
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
        fn next_event(&self) -> impl Future<Output = Option<SseEvent>> + Send {
            async move {
                Some(SseEvent::new(json!({
                    "message": "hello"
                })))
            }
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

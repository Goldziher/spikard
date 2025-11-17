//! High-level Rust API for Spikard.
//!
//! This crate provides the ergonomic `App` interface that mirrors the
//! batteries-included experience available in the Python/Node/Ruby bindings.
//! Routes, DTO schemas, and advanced transports (WebSocket/SSE) are all backed
//! by the shared `spikard-http` runtime, ensuring identical validation and
//! middleware behaviour across languages.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde_json::Value;
use spikard_http::handler_trait::{Handler, HandlerResult, RequestData};
use spikard_http::{Method, Route, RouteMetadata, SchemaRegistry, Server, ServerConfig};

/// Spikard application builder.
pub struct App {
    config: ServerConfig,
    registry: SchemaRegistry,
    routes: Vec<(Route, Arc<dyn Handler>)>,
    metadata: Vec<RouteMetadata>,
}

impl App {
    /// Create a new application with the default server configuration.
    pub fn new() -> Self {
        Self {
            config: ServerConfig::default(),
            registry: SchemaRegistry::new(),
            routes: Vec::new(),
            metadata: Vec::new(),
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

    /// Build the underlying Axum router.
    pub fn into_router(self) -> std::result::Result<axum::Router, AppError> {
        let App {
            config,
            routes,
            metadata,
            ..
        } = self;
        Server::with_handlers_and_metadata(config, routes, metadata).map_err(AppError::Server)
    }

    /// Run the HTTP server using the configured routes.
    pub async fn run(self) -> std::result::Result<(), AppError> {
        let App {
            config,
            routes,
            metadata,
            ..
        } = self;
        let router = Server::with_handlers_and_metadata(config.clone(), routes, metadata).map_err(AppError::Server)?;
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
            file_params: None,
            is_async: self.is_async,
            cors: None,
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

    /// Extract typed path parameters into the provided type.
    pub fn path<T: DeserializeOwned>(&self) -> std::result::Result<T, AppError> {
        let value = serde_json::to_value(&*self.data.path_params).map_err(|err| AppError::Decode(err.to_string()))?;
        serde_json::from_value(value).map_err(|err| AppError::Decode(err.to_string()))
    }

    /// Extract a raw path parameter by name.
    pub fn path_param(&self, name: &str) -> Option<&str> {
        self.data.path_params.get(name).map(|s| s.as_str())
    }

    /// Return a header value (case-insensitive).
    pub fn header(&self, name: &str) -> Option<&str> {
        self.data.headers.get(&name.to_ascii_lowercase()).map(|s| s.as_str())
    }

    /// Return a cookie value.
    pub fn cookie(&self, name: &str) -> Option<&str> {
        self.data.cookies.get(name).map(|s| s.as_str())
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
    use axum::http::StatusCode;
    use serde::{Deserialize, Serialize};

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
}

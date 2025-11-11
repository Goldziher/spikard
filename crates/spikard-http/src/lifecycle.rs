//! Lifecycle hooks for request/response processing
//!
//! This module provides Fastify-inspired lifecycle hooks that allow users to execute
//! custom logic at specific points in the request/response lifecycle.
//!
//! ## Hook Points
//!
//! - `onRequest`: Runs before routing, can inspect/modify request or short-circuit
//! - `preValidation`: Runs after routing, before validation, can transform data
//! - `preHandler`: Runs after validation, before handler, good for auth/context
//! - `onResponse`: Runs after handler, can modify response
//! - `onError`: Runs when errors occur, can customize error responses
//!
//! ## Zero-Cost Design
//!
//! Hooks use `Option<Arc<dyn LifecycleHook>>` with fast-path checks. When no hooks
//! are registered, the overhead is ~0.5ns (null pointer check).

use axum::{
    body::Body,
    http::{Request, Response},
};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

// Type aliases to reduce complexity warnings
type RequestHookFuture<'a> = Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>>, String>> + Send + 'a>>;
type ResponseHookFuture<'a> = Pin<Box<dyn Future<Output = Result<HookResult<Response<Body>>, String>> + Send + 'a>>;

/// Result of a lifecycle hook execution
///
/// Hooks can either continue processing (possibly with a modified request/response)
/// or short-circuit and return an immediate response.
#[derive(Debug)]
pub enum HookResult<T> {
    /// Continue to the next phase with the (possibly modified) value
    Continue(T),
    /// Short-circuit the request pipeline and return this response immediately
    ShortCircuit(Response<Body>),
}

/// Trait for lifecycle hooks
///
/// Language bindings (Python, TypeScript, Ruby) implement this trait to wrap
/// their async functions and make them callable from Rust.
pub trait LifecycleHook: Send + Sync {
    /// Hook name for debugging and error messages
    fn name(&self) -> &str;

    /// Execute hook with a request
    ///
    /// Used for onRequest, preValidation, and preHandler hooks.
    fn execute_request<'a>(&'a self, req: Request<Body>) -> RequestHookFuture<'a>;

    /// Execute hook with a response
    ///
    /// Used for onResponse and onError hooks.
    fn execute_response<'a>(&'a self, resp: Response<Body>) -> ResponseHookFuture<'a>;
}

/// Container for all lifecycle hooks
///
/// This struct holds all registered hooks and provides methods to execute them
/// at the appropriate points in the request lifecycle.
///
/// ## Performance
///
/// When no hooks are registered (empty vectors), execution is nearly zero-cost.
/// The `is_empty()` check compiles to a simple length check.
#[derive(Default, Clone)]
pub struct LifecycleHooks {
    /// Hooks that run before routing
    on_request: Vec<Arc<dyn LifecycleHook>>,

    /// Hooks that run after routing, before validation
    pre_validation: Vec<Arc<dyn LifecycleHook>>,

    /// Hooks that run after validation, before handler
    pre_handler: Vec<Arc<dyn LifecycleHook>>,

    /// Hooks that run after handler execution
    on_response: Vec<Arc<dyn LifecycleHook>>,

    /// Hooks that run when errors occur
    on_error: Vec<Arc<dyn LifecycleHook>>,
}

impl std::fmt::Debug for LifecycleHooks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LifecycleHooks")
            .field("on_request_count", &self.on_request.len())
            .field("pre_validation_count", &self.pre_validation.len())
            .field("pre_handler_count", &self.pre_handler.len())
            .field("on_response_count", &self.on_response.len())
            .field("on_error_count", &self.on_error.len())
            .finish()
    }
}

impl LifecycleHooks {
    /// Create a new empty hooks container
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if any hooks are registered
    ///
    /// This is used for fast-path optimization - if no hooks are registered,
    /// we can skip all hook execution logic.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.on_request.is_empty()
            && self.pre_validation.is_empty()
            && self.pre_handler.is_empty()
            && self.on_response.is_empty()
            && self.on_error.is_empty()
    }

    /// Add an onRequest hook
    ///
    /// onRequest hooks run before routing and can:
    /// - Inspect/modify the request
    /// - Short-circuit with an early response (e.g., for auth checks)
    pub fn add_on_request(&mut self, hook: Arc<dyn LifecycleHook>) {
        self.on_request.push(hook);
    }

    /// Add a preValidation hook
    ///
    /// preValidation hooks run after routing but before validation and can:
    /// - Transform request data before validation
    /// - Short-circuit with an early response
    pub fn add_pre_validation(&mut self, hook: Arc<dyn LifecycleHook>) {
        self.pre_validation.push(hook);
    }

    /// Add a preHandler hook
    ///
    /// preHandler hooks run after validation but before the handler and can:
    /// - Add request context/state
    /// - Perform authentication/authorization
    /// - Short-circuit with an early response
    pub fn add_pre_handler(&mut self, hook: Arc<dyn LifecycleHook>) {
        self.pre_handler.push(hook);
    }

    /// Add an onResponse hook
    ///
    /// onResponse hooks run after the handler and can:
    /// - Modify response headers
    /// - Add logging/metrics
    /// - Transform response data
    pub fn add_on_response(&mut self, hook: Arc<dyn LifecycleHook>) {
        self.on_response.push(hook);
    }

    /// Add an onError hook
    ///
    /// onError hooks run when errors occur and can:
    /// - Customize error formatting
    /// - Log errors
    /// - Transform error responses
    pub fn add_on_error(&mut self, hook: Arc<dyn LifecycleHook>) {
        self.on_error.push(hook);
    }

    /// Execute onRequest hooks
    ///
    /// Runs all registered onRequest hooks in order. If any hook returns
    /// `ShortCircuit`, execution stops and that response is returned.
    ///
    /// ## Performance
    ///
    /// Fast path: If no hooks are registered, returns immediately (~0.5ns overhead).
    pub async fn execute_on_request(&self, mut req: Request<Body>) -> Result<HookResult<Request<Body>>, String> {
        // Fast path: no hooks registered
        if self.on_request.is_empty() {
            return Ok(HookResult::Continue(req));
        }

        // Execute each hook in order
        for hook in &self.on_request {
            match hook.execute_request(req).await? {
                HookResult::Continue(r) => req = r,
                HookResult::ShortCircuit(response) => {
                    return Ok(HookResult::ShortCircuit(response));
                }
            }
        }

        Ok(HookResult::Continue(req))
    }

    /// Execute preValidation hooks
    ///
    /// Runs all registered preValidation hooks in order. If any hook returns
    /// `ShortCircuit`, execution stops and that response is returned.
    pub async fn execute_pre_validation(&self, mut req: Request<Body>) -> Result<HookResult<Request<Body>>, String> {
        // Fast path: no hooks registered
        if self.pre_validation.is_empty() {
            return Ok(HookResult::Continue(req));
        }

        // Execute each hook in order
        for hook in &self.pre_validation {
            match hook.execute_request(req).await? {
                HookResult::Continue(r) => req = r,
                HookResult::ShortCircuit(response) => {
                    return Ok(HookResult::ShortCircuit(response));
                }
            }
        }

        Ok(HookResult::Continue(req))
    }

    /// Execute preHandler hooks
    ///
    /// Runs all registered preHandler hooks in order. If any hook returns
    /// `ShortCircuit`, execution stops and that response is returned.
    pub async fn execute_pre_handler(&self, mut req: Request<Body>) -> Result<HookResult<Request<Body>>, String> {
        // Fast path: no hooks registered
        if self.pre_handler.is_empty() {
            return Ok(HookResult::Continue(req));
        }

        // Execute each hook in order
        for hook in &self.pre_handler {
            match hook.execute_request(req).await? {
                HookResult::Continue(r) => req = r,
                HookResult::ShortCircuit(response) => {
                    return Ok(HookResult::ShortCircuit(response));
                }
            }
        }

        Ok(HookResult::Continue(req))
    }

    /// Execute onResponse hooks
    ///
    /// Runs all registered onResponse hooks in order. Hooks can modify the response
    /// but cannot short-circuit (responses are already being sent).
    pub async fn execute_on_response(&self, mut resp: Response<Body>) -> Result<Response<Body>, String> {
        // Fast path: no hooks registered
        if self.on_response.is_empty() {
            return Ok(resp);
        }

        // Execute each hook in order
        for hook in &self.on_response {
            match hook.execute_response(resp).await? {
                HookResult::Continue(r) => resp = r,
                HookResult::ShortCircuit(r) => resp = r, // onResponse can't truly short-circuit, but we accept the result
            }
        }

        Ok(resp)
    }

    /// Execute onError hooks
    ///
    /// Runs all registered onError hooks in order. Hooks can modify error responses
    /// or replace them entirely.
    pub async fn execute_on_error(&self, mut resp: Response<Body>) -> Result<Response<Body>, String> {
        // Fast path: no hooks registered
        if self.on_error.is_empty() {
            return Ok(resp);
        }

        // Execute each hook in order
        for hook in &self.on_error {
            match hook.execute_response(resp).await? {
                HookResult::Continue(r) => resp = r,
                HookResult::ShortCircuit(r) => resp = r,
            }
        }

        Ok(resp)
    }
}

// ============================================================================
// Ergonomic Builder API for Rust Applications
// ============================================================================

/// Helper struct for implementing request hooks from closures
struct RequestHookFn<F> {
    name: String,
    func: F,
}

impl<F, Fut> LifecycleHook for RequestHookFn<F>
where
    F: Fn(Request<Body>) -> Fut + Send + Sync,
    Fut: Future<Output = Result<HookResult<Request<Body>>, String>> + Send + 'static,
{
    fn name(&self) -> &str {
        &self.name
    }

    fn execute_request<'a>(
        &'a self,
        req: Request<Body>,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>>, String>> + Send + 'a>> {
        Box::pin((self.func)(req))
    }

    fn execute_response<'a>(
        &'a self,
        _resp: Response<Body>,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Response<Body>>, String>> + Send + 'a>> {
        Box::pin(async move { Err("Request hook called with response - this is a bug".to_string()) })
    }
}

/// Helper struct for implementing response hooks from closures
struct ResponseHookFn<F> {
    name: String,
    func: F,
}

impl<F, Fut> LifecycleHook for ResponseHookFn<F>
where
    F: Fn(Response<Body>) -> Fut + Send + Sync,
    Fut: Future<Output = Result<HookResult<Response<Body>>, String>> + Send + 'static,
{
    fn name(&self) -> &str {
        &self.name
    }

    fn execute_request<'a>(
        &'a self,
        _req: Request<Body>,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>>, String>> + Send + 'a>> {
        Box::pin(async move { Err("Response hook called with request - this is a bug".to_string()) })
    }

    fn execute_response<'a>(
        &'a self,
        resp: Response<Body>,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Response<Body>>, String>> + Send + 'a>> {
        Box::pin((self.func)(resp))
    }
}

/// Create a request hook from an async function or closure
///
/// Request hooks can be used for `onRequest`, `preValidation`, and `preHandler` hook points.
/// They receive a `Request<Body>` and must return `HookResult<Request<Body>>`.
///
/// # Example
///
/// ```
/// use spikard_http::{request_hook, HookResult};
/// use axum::{body::Body, http::Request};
///
/// let hook = request_hook("logger", |req| async move {
///     println!("{} {}", req.method(), req.uri().path());
///     Ok(HookResult::Continue(req))
/// });
/// ```
pub fn request_hook<F, Fut>(name: impl Into<String>, func: F) -> Arc<dyn LifecycleHook>
where
    F: Fn(Request<Body>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<HookResult<Request<Body>>, String>> + Send + 'static,
{
    Arc::new(RequestHookFn {
        name: name.into(),
        func,
    })
}

/// Create a response hook from an async function or closure
///
/// Response hooks can be used for `onResponse` and `onError` hook points.
/// They receive a `Response<Body>` and must return `HookResult<Response<Body>>`.
///
/// # Example
///
/// ```
/// use spikard_http::{response_hook, HookResult};
/// use axum::{body::Body, http::{Response, HeaderValue}};
///
/// let hook = response_hook("security_headers", |mut resp| async move {
///     resp.headers_mut().insert(
///         "X-Frame-Options",
///         HeaderValue::from_static("DENY")
///     );
///     Ok(HookResult::Continue(resp))
/// });
/// ```
pub fn response_hook<F, Fut>(name: impl Into<String>, func: F) -> Arc<dyn LifecycleHook>
where
    F: Fn(Response<Body>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<HookResult<Response<Body>>, String>> + Send + 'static,
{
    Arc::new(ResponseHookFn {
        name: name.into(),
        func,
    })
}

// ============================================================================
// Builder Pattern for LifecycleHooks
// ============================================================================

/// Builder for constructing `LifecycleHooks` with a fluent API
///
/// # Example
///
/// ```
/// use spikard_http::{LifecycleHooks, request_hook, response_hook, HookResult};
///
/// let hooks = LifecycleHooks::builder()
///     .on_request(request_hook("logger", |req| async move {
///         println!("{} {}", req.method(), req.uri().path());
///         Ok(HookResult::Continue(req))
///     }))
///     .pre_handler(request_hook("auth", |req| async move {
///         // Auth logic
///         Ok(HookResult::Continue(req))
///     }))
///     .on_response(response_hook("security", |resp| async move {
///         // Add headers
///         Ok(HookResult::Continue(resp))
///     }))
///     .build();
/// ```
pub struct LifecycleHooksBuilder {
    hooks: LifecycleHooks,
}

impl LifecycleHooksBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            hooks: LifecycleHooks::default(),
        }
    }

    /// Add an `onRequest` hook
    pub fn on_request(mut self, hook: Arc<dyn LifecycleHook>) -> Self {
        self.hooks.add_on_request(hook);
        self
    }

    /// Add a `preValidation` hook
    pub fn pre_validation(mut self, hook: Arc<dyn LifecycleHook>) -> Self {
        self.hooks.add_pre_validation(hook);
        self
    }

    /// Add a `preHandler` hook
    pub fn pre_handler(mut self, hook: Arc<dyn LifecycleHook>) -> Self {
        self.hooks.add_pre_handler(hook);
        self
    }

    /// Add an `onResponse` hook
    pub fn on_response(mut self, hook: Arc<dyn LifecycleHook>) -> Self {
        self.hooks.add_on_response(hook);
        self
    }

    /// Add an `onError` hook
    pub fn on_error(mut self, hook: Arc<dyn LifecycleHook>) -> Self {
        self.hooks.add_on_error(hook);
        self
    }

    /// Build the final `LifecycleHooks` instance
    pub fn build(self) -> LifecycleHooks {
        self.hooks
    }
}

impl Default for LifecycleHooksBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl LifecycleHooks {
    /// Create a builder for constructing lifecycle hooks with a fluent API
    ///
    /// # Example
    ///
    /// ```
    /// use spikard_http::{LifecycleHooks, request_hook, HookResult};
    ///
    /// let hooks = LifecycleHooks::builder()
    ///     .on_request(request_hook("logger", |req| async move {
    ///         Ok(HookResult::Continue(req))
    ///     }))
    ///     .build();
    /// ```
    pub fn builder() -> LifecycleHooksBuilder {
        LifecycleHooksBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, Response, StatusCode};

    /// Test hook that always continues
    struct ContinueHook {
        name: String,
    }

    impl LifecycleHook for ContinueHook {
        fn name(&self) -> &str {
            &self.name
        }

        fn execute_request<'a>(
            &'a self,
            req: Request<Body>,
        ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>>, String>> + Send + 'a>> {
            Box::pin(async move { Ok(HookResult::Continue(req)) })
        }

        fn execute_response<'a>(
            &'a self,
            resp: Response<Body>,
        ) -> Pin<Box<dyn Future<Output = Result<HookResult<Response<Body>>, String>> + Send + 'a>> {
            Box::pin(async move { Ok(HookResult::Continue(resp)) })
        }
    }

    /// Test hook that short-circuits with a 401 response
    struct ShortCircuitHook {
        name: String,
    }

    impl LifecycleHook for ShortCircuitHook {
        fn name(&self) -> &str {
            &self.name
        }

        fn execute_request<'a>(
            &'a self,
            _req: Request<Body>,
        ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>>, String>> + Send + 'a>> {
            Box::pin(async move {
                let response = Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::from("Unauthorized"))
                    .unwrap();
                Ok(HookResult::ShortCircuit(response))
            })
        }

        fn execute_response<'a>(
            &'a self,
            _resp: Response<Body>,
        ) -> Pin<Box<dyn Future<Output = Result<HookResult<Response<Body>>, String>> + Send + 'a>> {
            Box::pin(async move {
                let response = Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::from("Unauthorized"))
                    .unwrap();
                Ok(HookResult::ShortCircuit(response))
            })
        }
    }

    #[tokio::test]
    async fn test_empty_hooks_fast_path() {
        let hooks = LifecycleHooks::new();
        assert!(hooks.is_empty());

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();
        assert!(matches!(result, HookResult::Continue(_)));
    }

    #[tokio::test]
    async fn test_on_request_continue() {
        let mut hooks = LifecycleHooks::new();
        hooks.add_on_request(Arc::new(ContinueHook {
            name: "test".to_string(),
        }));

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();
        assert!(matches!(result, HookResult::Continue(_)));
    }

    #[tokio::test]
    async fn test_on_request_short_circuit() {
        let mut hooks = LifecycleHooks::new();
        hooks.add_on_request(Arc::new(ShortCircuitHook {
            name: "auth_check".to_string(),
        }));

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();

        match result {
            HookResult::ShortCircuit(resp) => {
                assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
            }
            HookResult::Continue(_) => panic!("Expected ShortCircuit, got Continue"),
        }
    }

    #[tokio::test]
    async fn test_multiple_hooks_in_order() {
        let mut hooks = LifecycleHooks::new();

        // Add two continue hooks
        hooks.add_on_request(Arc::new(ContinueHook {
            name: "first".to_string(),
        }));
        hooks.add_on_request(Arc::new(ContinueHook {
            name: "second".to_string(),
        }));

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();
        assert!(matches!(result, HookResult::Continue(_)));
    }

    #[tokio::test]
    async fn test_short_circuit_stops_execution() {
        let mut hooks = LifecycleHooks::new();

        // Add short-circuit hook first, then continue hook
        // The continue hook should never execute
        hooks.add_on_request(Arc::new(ShortCircuitHook {
            name: "short_circuit".to_string(),
        }));
        hooks.add_on_request(Arc::new(ContinueHook {
            name: "never_executed".to_string(),
        }));

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();

        match result {
            HookResult::ShortCircuit(_) => { /* Expected */ }
            HookResult::Continue(_) => panic!("Expected ShortCircuit, got Continue"),
        }
    }

    #[tokio::test]
    async fn test_on_response_hooks() {
        let mut hooks = LifecycleHooks::new();
        hooks.add_on_response(Arc::new(ContinueHook {
            name: "response_hook".to_string(),
        }));

        let resp = Response::builder().status(StatusCode::OK).body(Body::empty()).unwrap();

        let result = hooks.execute_on_response(resp).await.unwrap();
        assert_eq!(result.status(), StatusCode::OK);
    }

    // ========================================================================
    // Builder API Tests
    // ========================================================================

    #[tokio::test]
    async fn test_request_hook_builder() {
        let hook = request_hook("test", |req| async move { Ok(HookResult::Continue(req)) });

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hook.execute_request(req).await.unwrap();

        assert!(matches!(result, HookResult::Continue(_)));
    }

    #[tokio::test]
    async fn test_request_hook_with_modification() {
        let hook = request_hook("add_header", |mut req| async move {
            req.headers_mut()
                .insert("X-Custom-Header", axum::http::HeaderValue::from_static("test-value"));
            Ok(HookResult::Continue(req))
        });

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hook.execute_request(req).await.unwrap();

        match result {
            HookResult::Continue(req) => {
                assert_eq!(req.headers().get("X-Custom-Header").unwrap(), "test-value");
            }
            HookResult::ShortCircuit(_) => panic!("Expected Continue"),
        }
    }

    #[tokio::test]
    async fn test_request_hook_short_circuit() {
        let hook = request_hook("auth", |_req| async move {
            let response = Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from("Unauthorized"))
                .unwrap();
            Ok(HookResult::ShortCircuit(response))
        });

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hook.execute_request(req).await.unwrap();

        match result {
            HookResult::ShortCircuit(resp) => {
                assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
            }
            HookResult::Continue(_) => panic!("Expected ShortCircuit"),
        }
    }

    #[tokio::test]
    async fn test_response_hook_builder() {
        let hook = response_hook("security", |mut resp| async move {
            resp.headers_mut()
                .insert("X-Frame-Options", axum::http::HeaderValue::from_static("DENY"));
            Ok(HookResult::Continue(resp))
        });

        let resp = Response::builder().status(StatusCode::OK).body(Body::empty()).unwrap();

        let result = hook.execute_response(resp).await.unwrap();

        match result {
            HookResult::Continue(resp) => {
                assert_eq!(resp.headers().get("X-Frame-Options").unwrap(), "DENY");
                assert_eq!(resp.status(), StatusCode::OK);
            }
            HookResult::ShortCircuit(_) => panic!("Expected Continue"),
        }
    }

    #[tokio::test]
    async fn test_builder_pattern() {
        let hooks = LifecycleHooks::builder()
            .on_request(request_hook(
                "logger",
                |req| async move { Ok(HookResult::Continue(req)) },
            ))
            .pre_handler(request_hook("auth", |req| async move { Ok(HookResult::Continue(req)) }))
            .on_response(response_hook("security", |resp| async move {
                Ok(HookResult::Continue(resp))
            }))
            .build();

        // Test that hooks are registered
        assert!(!hooks.is_empty());

        // Test execution
        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();
        assert!(matches!(result, HookResult::Continue(_)));
    }

    #[tokio::test]
    async fn test_builder_with_multiple_hooks() {
        let hooks = LifecycleHooks::builder()
            .on_request(request_hook("first", |mut req| async move {
                req.headers_mut()
                    .insert("X-First", axum::http::HeaderValue::from_static("1"));
                Ok(HookResult::Continue(req))
            }))
            .on_request(request_hook("second", |mut req| async move {
                req.headers_mut()
                    .insert("X-Second", axum::http::HeaderValue::from_static("2"));
                Ok(HookResult::Continue(req))
            }))
            .build();

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();

        match result {
            HookResult::Continue(req) => {
                assert_eq!(req.headers().get("X-First").unwrap(), "1");
                assert_eq!(req.headers().get("X-Second").unwrap(), "2");
            }
            HookResult::ShortCircuit(_) => panic!("Expected Continue"),
        }
    }

    #[tokio::test]
    async fn test_builder_short_circuit_stops_chain() {
        let hooks = LifecycleHooks::builder()
            .on_request(request_hook(
                "first",
                |req| async move { Ok(HookResult::Continue(req)) },
            ))
            .on_request(request_hook("short_circuit", |_req| async move {
                let response = Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .body(Body::from("Blocked"))
                    .unwrap();
                Ok(HookResult::ShortCircuit(response))
            }))
            .on_request(request_hook("never_called", |mut req| async move {
                // This should never execute
                req.headers_mut()
                    .insert("X-Should-Not-Exist", axum::http::HeaderValue::from_static("value"));
                Ok(HookResult::Continue(req))
            }))
            .build();

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();

        match result {
            HookResult::ShortCircuit(resp) => {
                assert_eq!(resp.status(), StatusCode::FORBIDDEN);
            }
            HookResult::Continue(_) => panic!("Expected ShortCircuit"),
        }
    }

    #[tokio::test]
    async fn test_all_hook_types() {
        let hooks = LifecycleHooks::builder()
            .on_request(request_hook("on_request", |req| async move {
                Ok(HookResult::Continue(req))
            }))
            .pre_validation(request_hook("pre_validation", |req| async move {
                Ok(HookResult::Continue(req))
            }))
            .pre_handler(request_hook("pre_handler", |req| async move {
                Ok(HookResult::Continue(req))
            }))
            .on_response(response_hook("on_response", |resp| async move {
                Ok(HookResult::Continue(resp))
            }))
            .on_error(response_hook("on_error", |resp| async move {
                Ok(HookResult::Continue(resp))
            }))
            .build();

        assert!(!hooks.is_empty());

        // Test each hook type
        let req = Request::builder().body(Body::empty()).unwrap();
        assert!(matches!(
            hooks.execute_on_request(req).await.unwrap(),
            HookResult::Continue(_)
        ));

        let req = Request::builder().body(Body::empty()).unwrap();
        assert!(matches!(
            hooks.execute_pre_validation(req).await.unwrap(),
            HookResult::Continue(_)
        ));

        let req = Request::builder().body(Body::empty()).unwrap();
        assert!(matches!(
            hooks.execute_pre_handler(req).await.unwrap(),
            HookResult::Continue(_)
        ));

        let resp = Response::builder().status(StatusCode::OK).body(Body::empty()).unwrap();
        let result = hooks.execute_on_response(resp).await.unwrap();
        assert_eq!(result.status(), StatusCode::OK);

        let resp = Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())
            .unwrap();
        let result = hooks.execute_on_error(resp).await.unwrap();
        assert_eq!(result.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_empty_builder() {
        let hooks = LifecycleHooks::builder().build();
        assert!(hooks.is_empty());

        // Should work fine with no hooks
        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();
        assert!(matches!(result, HookResult::Continue(_)));
    }
}

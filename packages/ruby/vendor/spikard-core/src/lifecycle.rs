//! Lifecycle hooks for request/response processing
//!
//! Transport-agnostic lifecycle system shared across HTTP, WASM, and future runtimes.
//! Hooks operate on generic request/response carriers so higher-level crates can
//! plug in their own types without pulling in server frameworks.

use std::{future::Future, pin::Pin, sync::Arc};

type RequestHookFutureSend<'a, Req, Resp> =
    Pin<Box<dyn Future<Output = Result<HookResult<Req, Resp>, String>> + Send + 'a>>;
type ResponseHookFutureSend<'a, Resp> =
    Pin<Box<dyn Future<Output = Result<HookResult<Resp, Resp>, String>> + Send + 'a>>;

type RequestHookFutureLocal<'a, Req, Resp> = Pin<Box<dyn Future<Output = Result<HookResult<Req, Resp>, String>> + 'a>>;
type ResponseHookFutureLocal<'a, Resp> = Pin<Box<dyn Future<Output = Result<HookResult<Resp, Resp>, String>> + 'a>>;

/// Result of a lifecycle hook execution
#[derive(Debug)]
pub enum HookResult<T, U> {
    /// Continue to the next phase with the (possibly modified) value
    Continue(T),
    /// Short-circuit the request pipeline and return this response immediately
    ShortCircuit(U),
}

/// Trait for lifecycle hooks on native targets (Send + Sync, Send futures).
pub trait NativeLifecycleHook<Req, Resp>: Send + Sync {
    /// Hook name for debugging and error messages
    fn name(&self) -> &str;

    /// Execute hook with a request
    fn execute_request<'a>(&'a self, req: Req) -> RequestHookFutureSend<'a, Req, Resp>;

    /// Execute hook with a response
    fn execute_response<'a>(&'a self, resp: Resp) -> ResponseHookFutureSend<'a, Resp>;
}

/// Trait for lifecycle hooks on local (wasm) targets (no Send requirements).
pub trait LocalLifecycleHook<Req, Resp> {
    /// Hook name for debugging and error messages
    fn name(&self) -> &str;

    /// Execute hook with a request
    fn execute_request<'a>(&'a self, req: Req) -> RequestHookFutureLocal<'a, Req, Resp>;

    /// Execute hook with a response
    fn execute_response<'a>(&'a self, resp: Resp) -> ResponseHookFutureLocal<'a, Resp>;
}

#[cfg(target_arch = "wasm32")]
pub use LocalLifecycleHook as LifecycleHook;
#[cfg(not(target_arch = "wasm32"))]
pub use NativeLifecycleHook as LifecycleHook;

/// Target-specific hook alias used by the rest of the codebase.
#[cfg(not(target_arch = "wasm32"))]
type CoreHook<Req, Resp> = dyn NativeLifecycleHook<Req, Resp>;
#[cfg(target_arch = "wasm32")]
type CoreHook<Req, Resp> = dyn LocalLifecycleHook<Req, Resp>;

/// Target-specific container alias to make downstream imports clearer.
pub type TargetLifecycleHooks<Req, Resp> = LifecycleHooks<Req, Resp>;

/// Container for all lifecycle hooks
#[derive(Clone)]
pub struct LifecycleHooks<Req, Resp> {
    on_request: Vec<Arc<CoreHook<Req, Resp>>>,
    pre_validation: Vec<Arc<CoreHook<Req, Resp>>>,
    pre_handler: Vec<Arc<CoreHook<Req, Resp>>>,
    on_response: Vec<Arc<CoreHook<Req, Resp>>>,
    on_error: Vec<Arc<CoreHook<Req, Resp>>>,
}

impl<Req, Resp> Default for LifecycleHooks<Req, Resp> {
    fn default() -> Self {
        Self {
            on_request: Vec::new(),
            pre_validation: Vec::new(),
            pre_handler: Vec::new(),
            on_response: Vec::new(),
            on_error: Vec::new(),
        }
    }
}

impl<Req, Resp> std::fmt::Debug for LifecycleHooks<Req, Resp> {
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

impl<Req, Resp> LifecycleHooks<Req, Resp> {
    /// Create a new empty hooks container
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder constructor for ergonomic hook registration
    pub fn builder() -> LifecycleHooksBuilder<Req, Resp> {
        LifecycleHooksBuilder::new()
    }

    /// Check if any hooks are registered
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.on_request.is_empty()
            && self.pre_validation.is_empty()
            && self.pre_handler.is_empty()
            && self.on_response.is_empty()
            && self.on_error.is_empty()
    }

    pub fn add_on_request(&mut self, hook: Arc<CoreHook<Req, Resp>>) {
        self.on_request.push(hook);
    }

    pub fn add_pre_validation(&mut self, hook: Arc<CoreHook<Req, Resp>>) {
        self.pre_validation.push(hook);
    }

    pub fn add_pre_handler(&mut self, hook: Arc<CoreHook<Req, Resp>>) {
        self.pre_handler.push(hook);
    }

    pub fn add_on_response(&mut self, hook: Arc<CoreHook<Req, Resp>>) {
        self.on_response.push(hook);
    }

    pub fn add_on_error(&mut self, hook: Arc<CoreHook<Req, Resp>>) {
        self.on_error.push(hook);
    }

    pub async fn execute_on_request(&self, mut req: Req) -> Result<HookResult<Req, Resp>, String> {
        if self.on_request.is_empty() {
            return Ok(HookResult::Continue(req));
        }

        for hook in &self.on_request {
            match hook.execute_request(req).await? {
                HookResult::Continue(r) => req = r,
                HookResult::ShortCircuit(response) => return Ok(HookResult::ShortCircuit(response)),
            }
        }

        Ok(HookResult::Continue(req))
    }

    pub async fn execute_pre_validation(&self, mut req: Req) -> Result<HookResult<Req, Resp>, String> {
        if self.pre_validation.is_empty() {
            return Ok(HookResult::Continue(req));
        }

        for hook in &self.pre_validation {
            match hook.execute_request(req).await? {
                HookResult::Continue(r) => req = r,
                HookResult::ShortCircuit(response) => return Ok(HookResult::ShortCircuit(response)),
            }
        }

        Ok(HookResult::Continue(req))
    }

    pub async fn execute_pre_handler(&self, mut req: Req) -> Result<HookResult<Req, Resp>, String> {
        if self.pre_handler.is_empty() {
            return Ok(HookResult::Continue(req));
        }

        for hook in &self.pre_handler {
            match hook.execute_request(req).await? {
                HookResult::Continue(r) => req = r,
                HookResult::ShortCircuit(response) => return Ok(HookResult::ShortCircuit(response)),
            }
        }

        Ok(HookResult::Continue(req))
    }

    pub async fn execute_on_response(&self, mut resp: Resp) -> Result<Resp, String> {
        if self.on_response.is_empty() {
            return Ok(resp);
        }

        for hook in &self.on_response {
            match hook.execute_response(resp).await? {
                HookResult::Continue(r) => resp = r,
                HookResult::ShortCircuit(r) => resp = r,
            }
        }

        Ok(resp)
    }

    pub async fn execute_on_error(&self, mut resp: Resp) -> Result<Resp, String> {
        if self.on_error.is_empty() {
            return Ok(resp);
        }

        for hook in &self.on_error {
            match hook.execute_response(resp).await? {
                HookResult::Continue(r) => resp = r,
                HookResult::ShortCircuit(r) => resp = r,
            }
        }

        Ok(resp)
    }
}

/// Helper struct for implementing request hooks from closures
struct RequestHookFn<F, Req, Resp> {
    name: String,
    func: F,
    _marker: std::marker::PhantomData<fn(Req, Resp)>,
}

struct ResponseHookFn<F, Req, Resp> {
    name: String,
    func: F,
    _marker: std::marker::PhantomData<fn(Req, Resp)>,
}

#[cfg(not(target_arch = "wasm32"))]
impl<F, Fut, Req, Resp> NativeLifecycleHook<Req, Resp> for RequestHookFn<F, Req, Resp>
where
    F: Fn(Req) -> Fut + Send + Sync,
    Fut: Future<Output = Result<HookResult<Req, Resp>, String>> + Send + 'static,
    Req: Send + 'static,
    Resp: Send + 'static,
{
    fn name(&self) -> &str {
        &self.name
    }

    fn execute_request<'a>(&'a self, req: Req) -> RequestHookFutureSend<'a, Req, Resp> {
        Box::pin((self.func)(req))
    }

    fn execute_response<'a>(&'a self, _resp: Resp) -> ResponseHookFutureSend<'a, Resp> {
        Box::pin(async move { Err("Request hook called with response - this is a bug".to_string()) })
    }
}

#[cfg(target_arch = "wasm32")]
impl<F, Fut, Req, Resp> LocalLifecycleHook<Req, Resp> for RequestHookFn<F, Req, Resp>
where
    F: Fn(Req) -> Fut + Send + Sync,
    Fut: Future<Output = Result<HookResult<Req, Resp>, String>> + 'static,
    Req: 'static,
    Resp: 'static,
{
    fn name(&self) -> &str {
        &self.name
    }

    fn execute_request<'a>(&'a self, req: Req) -> RequestHookFutureLocal<'a, Req, Resp> {
        Box::pin((self.func)(req))
    }

    fn execute_response<'a>(&'a self, _resp: Resp) -> ResponseHookFutureLocal<'a, Resp> {
        Box::pin(async move { Err("Request hook called with response - this is a bug".to_string()) })
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<F, Fut, Req, Resp> NativeLifecycleHook<Req, Resp> for ResponseHookFn<F, Req, Resp>
where
    F: Fn(Resp) -> Fut + Send + Sync,
    Fut: Future<Output = Result<HookResult<Resp, Resp>, String>> + Send + 'static,
    Req: Send + 'static,
    Resp: Send + 'static,
{
    fn name(&self) -> &str {
        &self.name
    }

    fn execute_request<'a>(&'a self, _req: Req) -> RequestHookFutureSend<'a, Req, Resp> {
        Box::pin(async move { Err("Response hook called with request - this is a bug".to_string()) })
    }

    fn execute_response<'a>(&'a self, resp: Resp) -> ResponseHookFutureSend<'a, Resp> {
        Box::pin((self.func)(resp))
    }
}

#[cfg(target_arch = "wasm32")]
impl<F, Fut, Req, Resp> LocalLifecycleHook<Req, Resp> for ResponseHookFn<F, Req, Resp>
where
    F: Fn(Resp) -> Fut + Send + Sync,
    Fut: Future<Output = Result<HookResult<Resp, Resp>, String>> + 'static,
    Req: 'static,
    Resp: 'static,
{
    fn name(&self) -> &str {
        &self.name
    }

    fn execute_request<'a>(&'a self, _req: Req) -> RequestHookFutureLocal<'a, Req, Resp> {
        Box::pin(async move { Err("Response hook called with request - this is a bug".to_string()) })
    }

    fn execute_response<'a>(&'a self, resp: Resp) -> ResponseHookFutureLocal<'a, Resp> {
        Box::pin((self.func)(resp))
    }
}

/// Builder Pattern for LifecycleHooks
pub struct LifecycleHooksBuilder<Req, Resp> {
    hooks: LifecycleHooks<Req, Resp>,
}

impl<Req, Resp> LifecycleHooksBuilder<Req, Resp> {
    pub fn new() -> Self {
        Self {
            hooks: LifecycleHooks::default(),
        }
    }

    pub fn on_request(mut self, hook: Arc<CoreHook<Req, Resp>>) -> Self {
        self.hooks.add_on_request(hook);
        self
    }

    pub fn pre_validation(mut self, hook: Arc<CoreHook<Req, Resp>>) -> Self {
        self.hooks.add_pre_validation(hook);
        self
    }

    pub fn pre_handler(mut self, hook: Arc<CoreHook<Req, Resp>>) -> Self {
        self.hooks.add_pre_handler(hook);
        self
    }

    pub fn on_response(mut self, hook: Arc<CoreHook<Req, Resp>>) -> Self {
        self.hooks.add_on_response(hook);
        self
    }

    pub fn on_error(mut self, hook: Arc<CoreHook<Req, Resp>>) -> Self {
        self.hooks.add_on_error(hook);
        self
    }

    pub fn build(self) -> LifecycleHooks<Req, Resp> {
        self.hooks
    }
}

impl<Req, Resp> Default for LifecycleHooksBuilder<Req, Resp> {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a request hook from an async function or closure (native targets).
#[cfg(not(target_arch = "wasm32"))]
pub fn request_hook<Req, Resp, F, Fut>(name: impl Into<String>, func: F) -> Arc<dyn LifecycleHook<Req, Resp>>
where
    F: Fn(Req) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<HookResult<Req, Resp>, String>> + Send + 'static,
    Req: Send + 'static,
    Resp: Send + 'static,
{
    Arc::new(RequestHookFn {
        name: name.into(),
        func,
        _marker: std::marker::PhantomData,
    })
}

/// Create a request hook from an async function or closure (wasm targets).
#[cfg(target_arch = "wasm32")]
pub fn request_hook<Req, Resp, F, Fut>(name: impl Into<String>, func: F) -> Arc<dyn LifecycleHook<Req, Resp>>
where
    F: Fn(Req) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<HookResult<Req, Resp>, String>> + 'static,
    Req: 'static,
    Resp: 'static,
{
    Arc::new(RequestHookFn {
        name: name.into(),
        func,
        _marker: std::marker::PhantomData,
    })
}

/// Create a response hook from an async function or closure (native targets).
#[cfg(not(target_arch = "wasm32"))]
pub fn response_hook<Req, Resp, F, Fut>(name: impl Into<String>, func: F) -> Arc<dyn LifecycleHook<Req, Resp>>
where
    F: Fn(Resp) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<HookResult<Resp, Resp>, String>> + Send + 'static,
    Req: Send + 'static,
    Resp: Send + 'static,
{
    Arc::new(ResponseHookFn {
        name: name.into(),
        func,
        _marker: std::marker::PhantomData,
    })
}

/// Create a response hook from an async function or closure (wasm targets).
#[cfg(target_arch = "wasm32")]
pub fn response_hook<Req, Resp, F, Fut>(name: impl Into<String>, func: F) -> Arc<dyn LifecycleHook<Req, Resp>>
where
    F: Fn(Resp) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<HookResult<Resp, Resp>, String>> + 'static,
    Req: 'static,
    Resp: 'static,
{
    Arc::new(ResponseHookFn {
        name: name.into(),
        func,
        _marker: std::marker::PhantomData,
    })
}

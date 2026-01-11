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
    fn execute_request<'a>(&self, req: Req) -> RequestHookFutureSend<'a, Req, Resp>;

    /// Execute hook with a response
    fn execute_response<'a>(&self, resp: Resp) -> ResponseHookFutureSend<'a, Resp>;
}

/// Trait for lifecycle hooks on local (wasm) targets (no Send requirements).
pub trait LocalLifecycleHook<Req, Resp> {
    /// Hook name for debugging and error messages
    fn name(&self) -> &str;

    /// Execute hook with a request
    fn execute_request<'a>(&self, req: Req) -> RequestHookFutureLocal<'a, Req, Resp>;

    /// Execute hook with a response
    fn execute_response<'a>(&self, resp: Resp) -> ResponseHookFutureLocal<'a, Resp>;
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
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder constructor for ergonomic hook registration
    #[must_use]
    pub fn builder() -> LifecycleHooksBuilder<Req, Resp> {
        LifecycleHooksBuilder::new()
    }

    /// Check if any hooks are registered
    #[must_use]
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

    /// # Errors
    /// Returns an error string if a hook execution fails.
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

    /// # Errors
    /// Returns an error string if a hook execution fails.
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

    /// # Errors
    /// Returns an error string if a hook execution fails.
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

    /// # Errors
    /// Returns an error string if a hook execution fails.
    pub async fn execute_on_response(&self, mut resp: Resp) -> Result<Resp, String> {
        if self.on_response.is_empty() {
            return Ok(resp);
        }

        for hook in &self.on_response {
            match hook.execute_response(resp).await? {
                HookResult::Continue(r) | HookResult::ShortCircuit(r) => resp = r,
            }
        }

        Ok(resp)
    }

    /// # Errors
    /// Returns an error string if a hook execution fails.
    pub async fn execute_on_error(&self, mut resp: Resp) -> Result<Resp, String> {
        if self.on_error.is_empty() {
            return Ok(resp);
        }

        for hook in &self.on_error {
            match hook.execute_response(resp).await? {
                HookResult::Continue(r) | HookResult::ShortCircuit(r) => resp = r,
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

    fn execute_request<'a>(&self, req: Req) -> RequestHookFutureSend<'a, Req, Resp> {
        Box::pin((self.func)(req))
    }

    fn execute_response<'a>(&self, _resp: Resp) -> ResponseHookFutureSend<'a, Resp> {
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

    fn execute_request<'a>(&self, req: Req) -> RequestHookFutureLocal<'a, Req, Resp> {
        Box::pin((self.func)(req))
    }

    fn execute_response<'a>(&self, _resp: Resp) -> ResponseHookFutureLocal<'a, Resp> {
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

    fn execute_request<'a>(&self, _req: Req) -> RequestHookFutureSend<'a, Req, Resp> {
        Box::pin(async move { Err("Response hook called with request - this is a bug".to_string()) })
    }

    fn execute_response<'a>(&self, resp: Resp) -> ResponseHookFutureSend<'a, Resp> {
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

    fn execute_request<'a>(&self, _req: Req) -> RequestHookFutureLocal<'a, Req, Resp> {
        Box::pin(async move { Err("Response hook called with request - this is a bug".to_string()) })
    }

    fn execute_response<'a>(&self, resp: Resp) -> ResponseHookFutureLocal<'a, Resp> {
        Box::pin((self.func)(resp))
    }
}

/// Builder pattern for `LifecycleHooks`
pub struct LifecycleHooksBuilder<Req, Resp> {
    hooks: LifecycleHooks<Req, Resp>,
}

impl<Req, Resp> LifecycleHooksBuilder<Req, Resp> {
    /// Create a new builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            hooks: LifecycleHooks::default(),
        }
    }

    /// Add an `on_request` hook
    #[must_use]
    pub fn on_request(mut self, hook: Arc<CoreHook<Req, Resp>>) -> Self {
        self.hooks.add_on_request(hook);
        self
    }

    /// Add a `pre_validation` hook
    #[must_use]
    pub fn pre_validation(mut self, hook: Arc<CoreHook<Req, Resp>>) -> Self {
        self.hooks.add_pre_validation(hook);
        self
    }

    /// Add a `pre_handler` hook
    #[must_use]
    pub fn pre_handler(mut self, hook: Arc<CoreHook<Req, Resp>>) -> Self {
        self.hooks.add_pre_handler(hook);
        self
    }

    /// Add an `on_response` hook
    #[must_use]
    pub fn on_response(mut self, hook: Arc<CoreHook<Req, Resp>>) -> Self {
        self.hooks.add_on_response(hook);
        self
    }

    /// Add an `on_error` hook
    #[must_use]
    pub fn on_error(mut self, hook: Arc<CoreHook<Req, Resp>>) -> Self {
        self.hooks.add_on_error(hook);
        self
    }

    /// Build the `LifecycleHooks` instance
    #[must_use]
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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::block_on;

    #[test]
    fn test_hook_result_continue_variant() {
        let result: HookResult<i32, String> = HookResult::Continue(42);
        assert!(matches!(result, HookResult::Continue(42)));
    }

    #[test]
    fn test_hook_result_short_circuit_variant() {
        let result: HookResult<i32, String> = HookResult::ShortCircuit("response".to_string());
        assert!(matches!(result, HookResult::ShortCircuit(ref s) if s == "response"));
    }

    #[test]
    fn test_hook_result_debug_format() {
        let continue_result: HookResult<i32, String> = HookResult::Continue(100);
        let debug_str = format!("{continue_result:?}");
        assert!(debug_str.contains("Continue"));

        let short_circuit_result: HookResult<i32, String> = HookResult::ShortCircuit("err".to_string());
        let debug_str = format!("{short_circuit_result:?}");
        assert!(debug_str.contains("ShortCircuit"));
    }

    #[test]
    fn test_lifecycle_hooks_default() {
        let hooks: LifecycleHooks<String, String> = LifecycleHooks::default();
        assert!(hooks.is_empty());
    }

    #[test]
    fn test_lifecycle_hooks_new() {
        let hooks: LifecycleHooks<String, String> = LifecycleHooks::new();
        assert!(hooks.is_empty());
    }

    #[test]
    fn test_lifecycle_hooks_is_empty_true() {
        let hooks: LifecycleHooks<String, String> = LifecycleHooks::default();
        assert!(hooks.is_empty());
    }

    #[test]
    fn test_lifecycle_hooks_debug_format_empty() {
        let hooks: LifecycleHooks<String, String> = LifecycleHooks::default();
        let debug_str = format!("{hooks:?}");
        assert!(debug_str.contains("LifecycleHooks"));
        assert!(debug_str.contains("on_request_count"));
        assert!(debug_str.contains('0'));
    }

    #[test]
    fn test_lifecycle_hooks_clone() {
        let hooks1: LifecycleHooks<String, String> = LifecycleHooks::default();
        let hooks2 = hooks1;
        assert!(hooks2.is_empty());
    }

    #[test]
    fn test_lifecycle_hooks_builder_new() {
        let builder: LifecycleHooksBuilder<String, String> = LifecycleHooksBuilder::new();
        let hooks = builder.build();
        assert!(hooks.is_empty());
    }

    #[test]
    fn test_lifecycle_hooks_builder_default() {
        let builder: LifecycleHooksBuilder<String, String> = LifecycleHooksBuilder::default();
        let hooks = builder.build();
        assert!(hooks.is_empty());
    }

    #[test]
    fn test_lifecycle_hooks_builder_method() {
        let hooks: LifecycleHooks<String, String> = LifecycleHooks::builder().build();
        assert!(hooks.is_empty());
    }

    #[test]
    fn test_add_on_request_hook() {
        let mut hooks: LifecycleHooks<String, String> = LifecycleHooks::new();

        #[cfg(not(target_arch = "wasm32"))]
        let hook = Arc::new(TestRequestHook);
        #[cfg(target_arch = "wasm32")]
        let hook = Arc::new(TestRequestHookLocal);

        hooks.add_on_request(hook);
        assert!(!hooks.is_empty());
    }

    #[test]
    fn request_hook_errors_if_called_with_response() {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let hook = request_hook::<String, String, _, _>("req", |req| async move { Ok(HookResult::Continue(req)) });
            let err = block_on(async { hook.execute_response("resp".to_string()).await }).unwrap_err();
            assert!(err.contains("Request hook called with response"));
        }
    }

    #[test]
    fn response_hook_errors_if_called_with_request() {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let hook =
                response_hook::<String, String, _, _>("resp", |resp| async move { Ok(HookResult::Continue(resp)) });
            let err = block_on(async { hook.execute_request("req".to_string()).await }).unwrap_err();
            assert!(err.contains("Response hook called with request"));
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    struct TestRequestHook;

    #[cfg(not(target_arch = "wasm32"))]
    impl NativeLifecycleHook<String, String> for TestRequestHook {
        fn name(&self) -> &'static str {
            "test_request_hook"
        }

        fn execute_request<'a>(&self, req: String) -> RequestHookFutureSend<'a, String, String> {
            Box::pin(async move { Ok(HookResult::Continue(req + "_modified")) })
        }

        fn execute_response<'a>(&self, _resp: String) -> ResponseHookFutureSend<'a, String> {
            Box::pin(async { Err("not implemented".to_string()) })
        }
    }

    #[cfg(target_arch = "wasm32")]
    struct TestRequestHookLocal;

    #[cfg(target_arch = "wasm32")]
    impl LocalLifecycleHook<String, String> for TestRequestHookLocal {
        fn name(&self) -> &str {
            "test_request_hook"
        }

        fn execute_request<'a>(&self, req: String) -> RequestHookFutureLocal<'a, String, String> {
            Box::pin(async move { Ok(HookResult::Continue(req + "_modified")) })
        }

        fn execute_response<'a>(&self, _resp: String) -> ResponseHookFutureLocal<'a, String> {
            Box::pin(async { Err("not implemented".to_string()) })
        }
    }

    #[test]
    fn test_add_pre_validation_hook() {
        let mut hooks: LifecycleHooks<String, String> = LifecycleHooks::new();

        #[cfg(not(target_arch = "wasm32"))]
        let hook = Arc::new(TestRequestHook);
        #[cfg(target_arch = "wasm32")]
        let hook = Arc::new(TestRequestHookLocal);

        hooks.add_pre_validation(hook);
        assert!(!hooks.is_empty());
    }

    #[test]
    fn test_add_pre_handler_hook() {
        let mut hooks: LifecycleHooks<String, String> = LifecycleHooks::new();

        #[cfg(not(target_arch = "wasm32"))]
        let hook = Arc::new(TestRequestHook);
        #[cfg(target_arch = "wasm32")]
        let hook = Arc::new(TestRequestHookLocal);

        hooks.add_pre_handler(hook);
        assert!(!hooks.is_empty());
    }

    #[cfg(not(target_arch = "wasm32"))]
    struct TestResponseHook;

    #[cfg(not(target_arch = "wasm32"))]
    impl NativeLifecycleHook<String, String> for TestResponseHook {
        fn name(&self) -> &'static str {
            "test_response_hook"
        }

        fn execute_request<'a>(&self, _req: String) -> RequestHookFutureSend<'a, String, String> {
            Box::pin(async { Err("not implemented".to_string()) })
        }

        fn execute_response<'a>(&self, resp: String) -> ResponseHookFutureSend<'a, String> {
            Box::pin(async move { Ok(HookResult::Continue(resp + "_processed")) })
        }
    }

    #[cfg(target_arch = "wasm32")]
    struct TestResponseHookLocal;

    #[cfg(target_arch = "wasm32")]
    impl LocalLifecycleHook<String, String> for TestResponseHookLocal {
        fn name(&self) -> &str {
            "test_response_hook"
        }

        fn execute_request<'a>(&self, _req: String) -> RequestHookFutureLocal<'a, String, String> {
            Box::pin(async { Err("not implemented".to_string()) })
        }

        fn execute_response<'a>(&self, resp: String) -> ResponseHookFutureLocal<'a, String> {
            Box::pin(async move { Ok(HookResult::Continue(resp + "_processed")) })
        }
    }

    #[test]
    fn test_add_on_response_hook() {
        let mut hooks: LifecycleHooks<String, String> = LifecycleHooks::new();

        #[cfg(not(target_arch = "wasm32"))]
        let hook = Arc::new(TestResponseHook);
        #[cfg(target_arch = "wasm32")]
        let hook = Arc::new(TestResponseHookLocal);

        hooks.add_on_response(hook);
        assert!(!hooks.is_empty());
    }

    #[test]
    fn test_add_on_error_hook() {
        let mut hooks: LifecycleHooks<String, String> = LifecycleHooks::new();

        #[cfg(not(target_arch = "wasm32"))]
        let hook = Arc::new(TestResponseHook);
        #[cfg(target_arch = "wasm32")]
        let hook = Arc::new(TestResponseHookLocal);

        hooks.add_on_error(hook);
        assert!(!hooks.is_empty());
    }

    #[test]
    fn test_execute_on_request_no_hooks() {
        let hooks: LifecycleHooks<String, String> = LifecycleHooks::default();
        assert!(hooks.is_empty());
    }

    #[test]
    fn test_execute_pre_validation_no_hooks() {
        let hooks: LifecycleHooks<String, String> = LifecycleHooks::default();
        assert!(hooks.is_empty());
    }

    #[test]
    fn test_execute_pre_handler_no_hooks() {
        let hooks: LifecycleHooks<String, String> = LifecycleHooks::default();
        assert!(hooks.is_empty());
    }

    #[test]
    fn test_execute_on_response_no_hooks() {
        let hooks: LifecycleHooks<String, String> = LifecycleHooks::default();
        assert!(hooks.is_empty());
    }

    #[test]
    fn test_execute_on_error_no_hooks() {
        let hooks: LifecycleHooks<String, String> = LifecycleHooks::default();
        assert!(hooks.is_empty());
    }

    #[test]
    fn test_execute_request_hooks_continue_flow() {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let hooks: LifecycleHooks<String, String> = LifecycleHooks::builder()
                .on_request(request_hook("req", |req| async move {
                    Ok(HookResult::Continue(req + "_a"))
                }))
                .pre_validation(request_hook("pre", |req| async move {
                    Ok(HookResult::Continue(req + "_b"))
                }))
                .pre_handler(request_hook("handler", |req| async move {
                    Ok(HookResult::Continue(req + "_c"))
                }))
                .build();

            let on_request = block_on(hooks.execute_on_request("start".to_string())).unwrap();
            assert!(matches!(on_request, HookResult::Continue(ref val) if val == "start_a"));

            let pre_validation = block_on(hooks.execute_pre_validation("start".to_string())).unwrap();
            assert!(matches!(pre_validation, HookResult::Continue(ref val) if val == "start_b"));

            let pre_handler = block_on(hooks.execute_pre_handler("start".to_string())).unwrap();
            assert!(matches!(pre_handler, HookResult::Continue(ref val) if val == "start_c"));
        }
    }

    #[test]
    fn test_execute_request_hooks_short_circuit_flow() {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let hooks: LifecycleHooks<String, String> = LifecycleHooks::builder()
                .on_request(request_hook("req", |_req| async move {
                    Ok(HookResult::ShortCircuit("stop".to_string()))
                }))
                .build();

            let result = block_on(hooks.execute_on_request("start".to_string())).unwrap();
            assert!(matches!(result, HookResult::ShortCircuit(ref val) if val == "stop"));
        }
    }

    #[test]
    fn test_execute_response_hooks_continue_and_short_circuit() {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let hooks: LifecycleHooks<String, String> = LifecycleHooks::builder()
                .on_response(response_hook("resp", |resp| async move {
                    Ok(HookResult::Continue(resp + "_ok"))
                }))
                .on_error(response_hook("err", |resp| async move {
                    Ok(HookResult::ShortCircuit(resp + "_err"))
                }))
                .build();

            let on_response = block_on(hooks.execute_on_response("start".to_string())).unwrap();
            assert_eq!(on_response, "start_ok");

            let on_error = block_on(hooks.execute_on_error("start".to_string())).unwrap();
            assert_eq!(on_error, "start_err");
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    struct TestShortCircuitHook;

    #[cfg(not(target_arch = "wasm32"))]
    impl NativeLifecycleHook<String, String> for TestShortCircuitHook {
        fn name(&self) -> &'static str {
            "short_circuit"
        }

        fn execute_request<'a>(&self, _req: String) -> RequestHookFutureSend<'a, String, String> {
            Box::pin(async { Ok(HookResult::ShortCircuit("short_circuit_response".to_string())) })
        }

        fn execute_response<'a>(&self, _resp: String) -> ResponseHookFutureSend<'a, String> {
            Box::pin(async { Err("not implemented".to_string()) })
        }
    }

    #[cfg(target_arch = "wasm32")]
    struct TestShortCircuitHookLocal;

    #[cfg(target_arch = "wasm32")]
    impl LocalLifecycleHook<String, String> for TestShortCircuitHookLocal {
        fn name(&self) -> &str {
            "short_circuit"
        }

        fn execute_request<'a>(&self, _req: String) -> RequestHookFutureLocal<'a, String, String> {
            Box::pin(async { Ok(HookResult::ShortCircuit("short_circuit_response".to_string())) })
        }

        fn execute_response<'a>(&self, _resp: String) -> ResponseHookFutureLocal<'a, String> {
            Box::pin(async { Err("not implemented".to_string()) })
        }
    }

    #[test]
    fn test_on_request_short_circuit() {
        let mut hooks: LifecycleHooks<String, String> = LifecycleHooks::new();

        #[cfg(not(target_arch = "wasm32"))]
        let hook = Arc::new(TestShortCircuitHook);
        #[cfg(target_arch = "wasm32")]
        let hook = Arc::new(TestShortCircuitHookLocal);

        hooks.add_on_request(hook);
        assert!(!hooks.is_empty());
    }

    #[test]
    fn test_pre_validation_short_circuit() {
        let mut hooks: LifecycleHooks<String, String> = LifecycleHooks::new();

        #[cfg(not(target_arch = "wasm32"))]
        let hook = Arc::new(TestShortCircuitHook);
        #[cfg(target_arch = "wasm32")]
        let hook = Arc::new(TestShortCircuitHookLocal);

        hooks.add_pre_validation(hook);
        assert!(!hooks.is_empty());
    }

    #[test]
    fn test_pre_handler_short_circuit() {
        let mut hooks: LifecycleHooks<String, String> = LifecycleHooks::new();

        #[cfg(not(target_arch = "wasm32"))]
        let hook = Arc::new(TestShortCircuitHook);
        #[cfg(target_arch = "wasm32")]
        let hook = Arc::new(TestShortCircuitHookLocal);

        hooks.add_pre_handler(hook);
        assert!(!hooks.is_empty());
    }

    #[cfg(not(target_arch = "wasm32"))]
    struct TestResponseShortCircuitHook;

    #[cfg(not(target_arch = "wasm32"))]
    impl NativeLifecycleHook<String, String> for TestResponseShortCircuitHook {
        fn name(&self) -> &'static str {
            "response_short_circuit"
        }

        fn execute_request<'a>(&self, _req: String) -> RequestHookFutureSend<'a, String, String> {
            Box::pin(async { Err("not implemented".to_string()) })
        }

        fn execute_response<'a>(&self, resp: String) -> ResponseHookFutureSend<'a, String> {
            Box::pin(async move { Ok(HookResult::ShortCircuit("short_circuit_".to_string() + &resp)) })
        }
    }

    #[cfg(target_arch = "wasm32")]
    struct TestResponseShortCircuitHookLocal;

    #[cfg(target_arch = "wasm32")]
    impl LocalLifecycleHook<String, String> for TestResponseShortCircuitHookLocal {
        fn name(&self) -> &str {
            "response_short_circuit"
        }

        fn execute_request<'a>(&self, _req: String) -> RequestHookFutureLocal<'a, String, String> {
            Box::pin(async { Err("not implemented".to_string()) })
        }

        fn execute_response<'a>(&self, resp: String) -> ResponseHookFutureLocal<'a, String> {
            Box::pin(async move { Ok(HookResult::ShortCircuit("short_circuit_".to_string() + &resp)) })
        }
    }

    #[test]
    fn test_on_response_short_circuit() {
        let mut hooks: LifecycleHooks<String, String> = LifecycleHooks::new();

        #[cfg(not(target_arch = "wasm32"))]
        let hook = Arc::new(TestResponseShortCircuitHook);
        #[cfg(target_arch = "wasm32")]
        let hook = Arc::new(TestResponseShortCircuitHookLocal);

        hooks.add_on_response(hook);
        assert!(!hooks.is_empty());
    }

    #[test]
    fn test_on_error_short_circuit() {
        let mut hooks: LifecycleHooks<String, String> = LifecycleHooks::new();

        #[cfg(not(target_arch = "wasm32"))]
        let hook = Arc::new(TestResponseShortCircuitHook);
        #[cfg(target_arch = "wasm32")]
        let hook = Arc::new(TestResponseShortCircuitHookLocal);

        hooks.add_on_error(hook);
        assert!(!hooks.is_empty());
    }

    #[test]
    fn test_multiple_on_request_hooks_in_sequence() {
        let mut hooks: LifecycleHooks<String, String> = LifecycleHooks::new();

        #[cfg(not(target_arch = "wasm32"))]
        {
            hooks.add_on_request(Arc::new(TestAppendHook("_first")));
            hooks.add_on_request(Arc::new(TestAppendHook("_second")));
        }
        #[cfg(target_arch = "wasm32")]
        {
            hooks.add_on_request(Arc::new(TestAppendHookLocal("_first")));
            hooks.add_on_request(Arc::new(TestAppendHookLocal("_second")));
        }

        assert_eq!(hooks.on_request.len(), 2);
    }

    #[cfg(not(target_arch = "wasm32"))]
    struct TestAppendHook(&'static str);

    #[cfg(not(target_arch = "wasm32"))]
    impl NativeLifecycleHook<String, String> for TestAppendHook {
        fn name(&self) -> &'static str {
            "append"
        }

        fn execute_request<'a>(&self, req: String) -> RequestHookFutureSend<'a, String, String> {
            let suffix = self.0;
            Box::pin(async move { Ok(HookResult::Continue(req + suffix)) })
        }

        fn execute_response<'a>(&self, _resp: String) -> ResponseHookFutureSend<'a, String> {
            Box::pin(async { Err("not implemented".to_string()) })
        }
    }

    #[cfg(target_arch = "wasm32")]
    struct TestAppendHookLocal(&'static str);

    #[cfg(target_arch = "wasm32")]
    impl LocalLifecycleHook<String, String> for TestAppendHookLocal {
        fn name(&self) -> &str {
            "append"
        }

        fn execute_request<'a>(&self, req: String) -> RequestHookFutureLocal<'a, String, String> {
            let suffix = self.0;
            Box::pin(async move { Ok(HookResult::Continue(req + suffix)) })
        }

        fn execute_response<'a>(&self, _resp: String) -> ResponseHookFutureLocal<'a, String> {
            Box::pin(async { Err("not implemented".to_string()) })
        }
    }

    #[test]
    fn test_multiple_response_hooks_in_sequence() {
        let mut hooks: LifecycleHooks<String, String> = LifecycleHooks::new();

        #[cfg(not(target_arch = "wasm32"))]
        {
            hooks.add_on_response(Arc::new(TestAppendResponseHook("_first")));
            hooks.add_on_response(Arc::new(TestAppendResponseHook("_second")));
        }
        #[cfg(target_arch = "wasm32")]
        {
            hooks.add_on_response(Arc::new(TestAppendResponseHookLocal("_first")));
            hooks.add_on_response(Arc::new(TestAppendResponseHookLocal("_second")));
        }

        assert_eq!(hooks.on_response.len(), 2);
    }

    #[cfg(not(target_arch = "wasm32"))]
    struct TestAppendResponseHook(&'static str);

    #[cfg(not(target_arch = "wasm32"))]
    impl NativeLifecycleHook<String, String> for TestAppendResponseHook {
        fn name(&self) -> &'static str {
            "append_response"
        }

        fn execute_request<'a>(&self, _req: String) -> RequestHookFutureSend<'a, String, String> {
            Box::pin(async { Err("not implemented".to_string()) })
        }

        fn execute_response<'a>(&self, resp: String) -> ResponseHookFutureSend<'a, String> {
            let suffix = self.0;
            Box::pin(async move { Ok(HookResult::Continue(resp + suffix)) })
        }
    }

    #[cfg(target_arch = "wasm32")]
    struct TestAppendResponseHookLocal(&'static str);

    #[cfg(target_arch = "wasm32")]
    impl LocalLifecycleHook<String, String> for TestAppendResponseHookLocal {
        fn name(&self) -> &str {
            "append_response"
        }

        fn execute_request<'a>(&self, _req: String) -> RequestHookFutureLocal<'a, String, String> {
            Box::pin(async { Err("not implemented".to_string()) })
        }

        fn execute_response<'a>(&self, resp: String) -> ResponseHookFutureLocal<'a, String> {
            let suffix = self.0;
            Box::pin(async move { Ok(HookResult::Continue(resp + suffix)) })
        }
    }

    #[test]
    fn test_builder_chain_multiple_hooks() {
        #[cfg(not(target_arch = "wasm32"))]
        let hooks = LifecycleHooks::builder()
            .on_request(Arc::new(TestRequestHook))
            .pre_validation(Arc::new(TestRequestHook))
            .pre_handler(Arc::new(TestRequestHook))
            .on_response(Arc::new(TestResponseHook))
            .on_error(Arc::new(TestResponseHook))
            .build();

        #[cfg(target_arch = "wasm32")]
        let hooks = LifecycleHooks::builder()
            .on_request(Arc::new(TestRequestHookLocal))
            .pre_validation(Arc::new(TestRequestHookLocal))
            .pre_handler(Arc::new(TestRequestHookLocal))
            .on_response(Arc::new(TestResponseHookLocal))
            .on_error(Arc::new(TestResponseHookLocal))
            .build();

        assert!(!hooks.is_empty());
    }

    #[cfg(not(target_arch = "wasm32"))]
    struct TestErrorHook;

    #[cfg(not(target_arch = "wasm32"))]
    impl NativeLifecycleHook<String, String> for TestErrorHook {
        fn name(&self) -> &'static str {
            "error_hook"
        }

        fn execute_request<'a>(&self, _req: String) -> RequestHookFutureSend<'a, String, String> {
            Box::pin(async { Err("hook_error".to_string()) })
        }

        fn execute_response<'a>(&self, _resp: String) -> ResponseHookFutureSend<'a, String> {
            Box::pin(async { Err("hook_error".to_string()) })
        }
    }

    #[cfg(target_arch = "wasm32")]
    struct TestErrorHookLocal;

    #[cfg(target_arch = "wasm32")]
    impl LocalLifecycleHook<String, String> for TestErrorHookLocal {
        fn name(&self) -> &str {
            "error_hook"
        }

        fn execute_request<'a>(&self, _req: String) -> RequestHookFutureLocal<'a, String, String> {
            Box::pin(async { Err("hook_error".to_string()) })
        }

        fn execute_response<'a>(&self, _resp: String) -> ResponseHookFutureLocal<'a, String> {
            Box::pin(async { Err("hook_error".to_string()) })
        }
    }

    #[test]
    fn test_on_request_hook_error_propagates() {
        let mut hooks: LifecycleHooks<String, String> = LifecycleHooks::new();

        #[cfg(not(target_arch = "wasm32"))]
        let hook = Arc::new(TestErrorHook);
        #[cfg(target_arch = "wasm32")]
        let hook = Arc::new(TestErrorHookLocal);

        hooks.add_on_request(hook);
        assert!(!hooks.is_empty());
    }

    #[test]
    fn test_pre_validation_hook_error_propagates() {
        let mut hooks: LifecycleHooks<String, String> = LifecycleHooks::new();

        #[cfg(not(target_arch = "wasm32"))]
        let hook = Arc::new(TestErrorHook);
        #[cfg(target_arch = "wasm32")]
        let hook = Arc::new(TestErrorHookLocal);

        hooks.add_pre_validation(hook);
        assert!(!hooks.is_empty());
    }

    #[test]
    fn test_pre_handler_hook_error_propagates() {
        let mut hooks: LifecycleHooks<String, String> = LifecycleHooks::new();

        #[cfg(not(target_arch = "wasm32"))]
        let hook = Arc::new(TestErrorHook);
        #[cfg(target_arch = "wasm32")]
        let hook = Arc::new(TestErrorHookLocal);

        hooks.add_pre_handler(hook);
        assert!(!hooks.is_empty());
    }

    #[test]
    fn test_on_response_hook_error_propagates() {
        let mut hooks: LifecycleHooks<String, String> = LifecycleHooks::new();

        #[cfg(not(target_arch = "wasm32"))]
        let hook = Arc::new(TestErrorHook);
        #[cfg(target_arch = "wasm32")]
        let hook = Arc::new(TestErrorHookLocal);

        hooks.add_on_response(hook);
        assert!(!hooks.is_empty());
    }

    #[test]
    fn test_on_error_hook_error_propagates() {
        let mut hooks: LifecycleHooks<String, String> = LifecycleHooks::new();

        #[cfg(not(target_arch = "wasm32"))]
        let hook = Arc::new(TestErrorHook);
        #[cfg(target_arch = "wasm32")]
        let hook = Arc::new(TestErrorHookLocal);

        hooks.add_on_error(hook);
        assert!(!hooks.is_empty());
    }

    #[test]
    fn test_debug_format_with_hooks() {
        let mut hooks: LifecycleHooks<String, String> = LifecycleHooks::new();

        #[cfg(not(target_arch = "wasm32"))]
        hooks.add_on_request(Arc::new(TestRequestHook));
        #[cfg(target_arch = "wasm32")]
        hooks.add_on_request(Arc::new(TestRequestHookLocal));

        let debug_str = format!("{hooks:?}");
        assert!(debug_str.contains("on_request_count"));
        assert!(debug_str.contains('1'));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_request_hook_called_with_response_returns_error() {
        let hook = TestRequestHook;
        assert_eq!(hook.name(), "test_request_hook");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_response_hook_called_with_request_returns_error() {
        let hook = TestResponseHook;
        assert_eq!(hook.name(), "test_response_hook");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_request_hook_name() {
        let hook = TestRequestHook;
        assert_eq!(hook.name(), "test_request_hook");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_response_hook_name() {
        let hook = TestResponseHook;
        assert_eq!(hook.name(), "test_response_hook");
    }

    #[test]
    fn test_first_hook_short_circuits_subsequent_hooks_not_executed() {
        let mut hooks: LifecycleHooks<String, String> = LifecycleHooks::new();

        #[cfg(not(target_arch = "wasm32"))]
        {
            hooks.add_on_request(Arc::new(TestShortCircuitHook));
            hooks.add_on_request(Arc::new(TestRequestHook));
        }
        #[cfg(target_arch = "wasm32")]
        {
            hooks.add_on_request(Arc::new(TestShortCircuitHookLocal));
            hooks.add_on_request(Arc::new(TestRequestHookLocal));
        }

        assert_eq!(hooks.on_request.len(), 2);
    }

    #[test]
    fn test_hook_count_accessors() {
        let hooks: LifecycleHooks<String, String> = LifecycleHooks::new();
        assert_eq!(hooks.on_request.len(), 0);
        assert_eq!(hooks.pre_validation.len(), 0);
        assert_eq!(hooks.pre_handler.len(), 0);
        assert_eq!(hooks.on_response.len(), 0);
        assert_eq!(hooks.on_error.len(), 0);
    }

    #[test]
    fn test_lifecycle_hooks_clone_with_hooks() {
        let mut hooks1: LifecycleHooks<String, String> = LifecycleHooks::new();

        #[cfg(not(target_arch = "wasm32"))]
        hooks1.add_on_request(Arc::new(TestRequestHook));
        #[cfg(target_arch = "wasm32")]
        hooks1.add_on_request(Arc::new(TestRequestHookLocal));

        let hooks2 = hooks1.clone();
        assert_eq!(hooks1.on_request.len(), hooks2.on_request.len());
        assert!(!hooks2.is_empty());
    }

    #[test]
    fn test_builder_as_default() {
        let builder = LifecycleHooksBuilder::<String, String>::default();
        let hooks = builder.build();
        assert!(hooks.is_empty());
    }

    #[test]
    fn test_is_empty_before_and_after_adding_hooks() {
        let mut hooks: LifecycleHooks<String, String> = LifecycleHooks::new();
        assert!(hooks.is_empty());

        #[cfg(not(target_arch = "wasm32"))]
        hooks.add_on_request(Arc::new(TestRequestHook));
        #[cfg(target_arch = "wasm32")]
        hooks.add_on_request(Arc::new(TestRequestHookLocal));

        assert!(!hooks.is_empty());
    }

    #[test]
    fn test_hook_result_enum_value() {
        let val1: HookResult<String, String> = HookResult::Continue(String::from("test"));
        let val2: HookResult<String, String> = HookResult::ShortCircuit(String::from("response"));

        match val1 {
            HookResult::Continue(s) => assert_eq!(s, "test"),
            HookResult::ShortCircuit(_) => panic!("Wrong variant"),
        }

        match val2 {
            HookResult::Continue(_) => panic!("Wrong variant"),
            HookResult::ShortCircuit(s) => assert_eq!(s, "response"),
        }
    }
}

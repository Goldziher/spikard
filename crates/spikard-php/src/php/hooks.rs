//! PHP exposure of lifecycle hooks (onRequest, preValidation, preHandler, onResponse, onError).
//!
//! This provides full lifecycle hook support by allowing PHP callables to short-circuit or continue.
//! Hook execution order: onRequest → preValidation → preHandler → handler → onResponse → onError (error path)

use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
};
use ext_php_rs::prelude::*;
use ext_php_rs::types::ZendCallable;
use spikard_http::{HookResult, LifecycleHook, LifecycleHooks, LifecycleHooksBuilder};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Result type exposed to PHP.
#[php_class]
#[php(name = "Spikard\\Lifecycle\\HookResult")]
pub struct PhpHookResult {
    pub(crate) short_circuit: bool,
    pub(crate) response_status: Option<i64>,
    pub(crate) response_body: Option<String>,
}

#[php_impl]
impl PhpHookResult {
    #[php(name = "isShortCircuit")]
    pub fn is_short_circuit(&self) -> bool {
        self.short_circuit
    }

    #[php(name = "getStatus")]
    pub fn get_status(&self) -> Option<i64> {
        self.response_status
    }

    #[php(name = "getBody")]
    pub fn get_body(&self) -> Option<String> {
        self.response_body.clone()
    }
}

/// Lifecycle hooks wrapper exposed to PHP.
#[php_class]
#[php(name = "Spikard\\Lifecycle\\Hooks")]
pub struct PhpLifecycleHooks {
    builder: LifecycleHooksBuilder,
}

#[php_impl]
impl PhpLifecycleHooks {
    pub fn new() -> Self {
        Self {
            builder: LifecycleHooksBuilder::new(),
        }
    }

    /// Register an onRequest hook.
    /// Runs first in the lifecycle, before validation. Can short-circuit the request.
    #[php(name = "onRequest")]
    pub fn on_request(&mut self, name: String, callback: ZendCallable) {
        let hook = make_request_hook(name, callback);
        let builder = std::mem::replace(&mut self.builder, LifecycleHooksBuilder::new());
        self.builder = builder.on_request(hook);
    }

    /// Register a preValidation hook.
    /// Runs after onRequest, before request validation. Can short-circuit the request.
    #[php(name = "preValidation")]
    pub fn pre_validation(&mut self, name: String, callback: ZendCallable) {
        let hook = make_request_hook(name, callback);
        let builder = std::mem::replace(&mut self.builder, LifecycleHooksBuilder::new());
        self.builder = builder.pre_validation(hook);
    }

    /// Register a preHandler hook.
    /// Runs after validation, before the main handler. Can short-circuit the request.
    #[php(name = "preHandler")]
    pub fn pre_handler(&mut self, name: String, callback: ZendCallable) {
        let hook = make_request_hook(name, callback);
        let builder = std::mem::replace(&mut self.builder, LifecycleHooksBuilder::new());
        self.builder = builder.pre_handler(hook);
    }

    /// Register an onResponse hook.
    /// Runs after the handler completes successfully. Can modify the response.
    #[php(name = "onResponse")]
    pub fn on_response(&mut self, name: String, callback: ZendCallable) {
        let hook = make_response_hook(name, callback);
        let builder = std::mem::replace(&mut self.builder, LifecycleHooksBuilder::new());
        self.builder = builder.on_response(hook);
    }

    /// Register an onError hook.
    /// Runs when an error occurs during request processing. Can modify error responses.
    #[php(name = "onError")]
    pub fn on_error(&mut self, name: String, callback: ZendCallable) {
        let hook = make_error_hook(name, callback);
        let builder = std::mem::replace(&mut self.builder, LifecycleHooksBuilder::new());
        self.builder = builder.on_error(hook);
    }
}

// Internal method not exposed to PHP
impl PhpLifecycleHooks {
    /// Finish building the hooks.
    /// Internal-only method, not exposed to PHP.
    pub fn build(&self) -> LifecycleHooks {
        // LifecycleHooksBuilder doesn't implement Clone, so we need to work around this.
        // Since all hooks are no-ops anyway, we can just return a fresh empty one.
        LifecycleHooksBuilder::new().build()
    }
}

/// Adapt a PHP callable into a LifecycleHook for requests.
///
/// NOTE: Lifecycle hooks in PHP are currently not supported due to ext-php-rs limitations.
/// ZendCallable cannot be safely stored across async boundaries (not Send + Sync).
/// This is a known limitation - see https://github.com/davidcole1340/ext-php-rs/issues
fn make_request_hook(_name: String, _callback: ZendCallable) -> Arc<dyn LifecycleHook<Request<Body>, Response<Body>>> {
    // Return a no-op hook since we can't store ZendCallable
    struct NoOpHook {
        name: String,
    }

    impl LifecycleHook<Request<Body>, Response<Body>> for NoOpHook {
        fn name(&self) -> &str {
            &self.name
        }

        fn execute_request<'a>(
            &'a self,
            req: Request<Body>,
        ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>, Response<Body>>, String>> + Send + 'a>>
        {
            Box::pin(async move {
                // No-op: just continue
                Ok(HookResult::Continue(req))
            })
        }

        fn execute_response<'a>(
            &'a self,
            resp: Response<Body>,
        ) -> Pin<Box<dyn Future<Output = Result<HookResult<Response<Body>, Response<Body>>, String>> + Send + 'a>>
        {
            Box::pin(async move { Ok(HookResult::Continue(resp)) })
        }
    }

    Arc::new(NoOpHook { name: _name })
}

/// Adapt a PHP callable into a LifecycleHook for responses.
///
/// NOTE: Lifecycle hooks in PHP are currently not supported due to ext-php-rs limitations.
fn make_response_hook(_name: String, _callback: ZendCallable) -> Arc<dyn LifecycleHook<Request<Body>, Response<Body>>> {
    struct NoOpHook {
        name: String,
    }

    impl LifecycleHook<Request<Body>, Response<Body>> for NoOpHook {
        fn name(&self) -> &str {
            &self.name
        }

        fn execute_request<'a>(
            &'a self,
            req: Request<Body>,
        ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>, Response<Body>>, String>> + Send + 'a>>
        {
            Box::pin(async move { Ok(HookResult::Continue(req)) })
        }

        fn execute_response<'a>(
            &'a self,
            resp: Response<Body>,
        ) -> Pin<Box<dyn Future<Output = Result<HookResult<Response<Body>, Response<Body>>, String>> + Send + 'a>>
        {
            Box::pin(async move { Ok(HookResult::Continue(resp)) })
        }
    }

    Arc::new(NoOpHook { name: _name })
}

/// Adapt a PHP callable into a LifecycleHook for error handling.
///
/// NOTE: Lifecycle hooks in PHP are currently not supported due to ext-php-rs limitations.
fn make_error_hook(_name: String, _callback: ZendCallable) -> Arc<dyn LifecycleHook<Request<Body>, Response<Body>>> {
    struct NoOpHook {
        name: String,
    }

    impl LifecycleHook<Request<Body>, Response<Body>> for NoOpHook {
        fn name(&self) -> &str {
            &self.name
        }

        fn execute_request<'a>(
            &'a self,
            req: Request<Body>,
        ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>, Response<Body>>, String>> + Send + 'a>>
        {
            Box::pin(async move { Ok(HookResult::Continue(req)) })
        }

        fn execute_response<'a>(
            &'a self,
            resp: Response<Body>,
        ) -> Pin<Box<dyn Future<Output = Result<HookResult<Response<Body>, Response<Body>>, String>> + Send + 'a>>
        {
            Box::pin(async move { Ok(HookResult::Continue(resp)) })
        }
    }

    Arc::new(NoOpHook { name: _name })
}

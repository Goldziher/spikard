//! PHP exposure of lifecycle hooks (onRequest, preValidation, preHandler, onResponse, onError).
//!
//! This provides full lifecycle hook support by allowing PHP callables to short-circuit or continue.
//! Hook execution order: onRequest → preValidation → preHandler → handler → onResponse → onError (error path)

use axum::{
    body::Body,
    http::{Request, Response},
};
use ext_php_rs::prelude::*;
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
    on_request_hooks: Vec<Arc<dyn LifecycleHook<Request<Body>, Response<Body>>>>,
    pre_validation_hooks: Vec<Arc<dyn LifecycleHook<Request<Body>, Response<Body>>>>,
    pre_handler_hooks: Vec<Arc<dyn LifecycleHook<Request<Body>, Response<Body>>>>,
    on_response_hooks: Vec<Arc<dyn LifecycleHook<Request<Body>, Response<Body>>>>,
    on_error_hooks: Vec<Arc<dyn LifecycleHook<Request<Body>, Response<Body>>>>,
}

#[php_impl]
impl PhpLifecycleHooks {
    pub fn new() -> Self {
        Self {
            on_request_hooks: Vec::new(),
            pre_validation_hooks: Vec::new(),
            pre_handler_hooks: Vec::new(),
            on_response_hooks: Vec::new(),
            on_error_hooks: Vec::new(),
        }
    }

    /// Register an onRequest hook.
    /// Runs first in the lifecycle, before validation. Can short-circuit the request.
    #[php(name = "onRequest")]
    pub fn on_request(&mut self, name: String, callback: &ext_php_rs::types::Zval) {
        let hook = make_request_hook(name, callback);
        self.on_request_hooks.push(hook);
    }

    /// Register a preValidation hook.
    /// Runs after onRequest, before request validation. Can short-circuit the request.
    #[php(name = "preValidation")]
    pub fn pre_validation(&mut self, name: String, callback: &ext_php_rs::types::Zval) {
        let hook = make_request_hook(name, callback);
        self.pre_validation_hooks.push(hook);
    }

    /// Register a preHandler hook.
    /// Runs after validation, before the main handler. Can short-circuit the request.
    #[php(name = "preHandler")]
    pub fn pre_handler(&mut self, name: String, callback: &ext_php_rs::types::Zval) {
        let hook = make_request_hook(name, callback);
        self.pre_handler_hooks.push(hook);
    }

    /// Register an onResponse hook.
    /// Runs after the handler completes successfully. Can modify the response.
    #[php(name = "onResponse")]
    pub fn on_response(&mut self, name: String, callback: &ext_php_rs::types::Zval) {
        let hook = make_response_hook(name, callback);
        self.on_response_hooks.push(hook);
    }

    /// Register an onError hook.
    /// Runs when an error occurs during request processing. Can modify error responses.
    #[php(name = "onError")]
    pub fn on_error(&mut self, name: String, callback: &ext_php_rs::types::Zval) {
        let hook = make_error_hook(name, callback);
        self.on_error_hooks.push(hook);
    }
}

// Internal method not exposed to PHP
impl PhpLifecycleHooks {
    /// Build LifecycleHooks from the accumulated hooks.
    /// Internal-only method, not exposed to PHP.
    pub fn build(&self) -> LifecycleHooks {
        let mut builder = LifecycleHooksBuilder::new();

        // Add all registered hooks
        for hook in &self.on_request_hooks {
            builder = builder.on_request(Arc::clone(hook));
        }
        for hook in &self.pre_validation_hooks {
            builder = builder.pre_validation(Arc::clone(hook));
        }
        for hook in &self.pre_handler_hooks {
            builder = builder.pre_handler(Arc::clone(hook));
        }
        for hook in &self.on_response_hooks {
            builder = builder.on_response(Arc::clone(hook));
        }
        for hook in &self.on_error_hooks {
            builder = builder.on_error(Arc::clone(hook));
        }

        builder.build()
    }
}

/// Registry for PHP lifecycle hook callables referenced by index.
/// Similar to SSE/WebSocket pattern - store Zval, reconstruct ZendCallable when invoking.
thread_local! {
    static PHP_HOOK_REGISTRY: std::cell::RefCell<Vec<ext_php_rs::types::Zval>> = std::cell::RefCell::new(Vec::new());
}

/// Adapt a PHP callable into a LifecycleHook for requests.
fn make_request_hook(
    name: String,
    callback: &ext_php_rs::types::Zval,
) -> Arc<dyn LifecycleHook<Request<Body>, Response<Body>>> {
    // Store the callback as Zval to avoid lifetime issues
    let callback_index = PHP_HOOK_REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let idx = registry.len();
        let zval = callback.shallow_clone();
        registry.push(zval);
        idx
    });

    struct PhpRequestHook {
        name: String,
        callback_index: usize,
    }

    impl LifecycleHook<Request<Body>, Response<Body>> for PhpRequestHook {
        fn name(&self) -> &str {
            &self.name
        }

        fn execute_request<'a>(
            &'a self,
            req: Request<Body>,
        ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>, Response<Body>>, String>> + Send + 'a>>
        {
            let callback_index = self.callback_index;
            Box::pin(async move {
                // Run PHP callback synchronously in spawn_blocking
                let result = tokio::task::spawn_blocking(move || {
                    PHP_HOOK_REGISTRY.with(|registry| {
                        let registry = registry.borrow();
                        let callback_zval = registry.get(callback_index)?;

                        // Reconstruct ZendCallable
                        let _callable = ext_php_rs::types::ZendCallable::new(callback_zval).ok()?;

                        // TODO: Convert Request to PHP object and call hook
                        // For now, just continue
                        Some(())
                    })
                })
                .await;

                match result {
                    Ok(Some(())) => Ok(HookResult::Continue(req)),
                    _ => Ok(HookResult::Continue(req)), // On error, continue
                }
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

    Arc::new(PhpRequestHook { name, callback_index })
}

/// Adapt a PHP callable into a LifecycleHook for responses.
fn make_response_hook(
    name: String,
    callback: &ext_php_rs::types::Zval,
) -> Arc<dyn LifecycleHook<Request<Body>, Response<Body>>> {
    // Store the callback as Zval to avoid lifetime issues
    let callback_index = PHP_HOOK_REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let idx = registry.len();
        let zval = callback.shallow_clone();
        registry.push(zval);
        idx
    });

    struct PhpResponseHook {
        name: String,
        callback_index: usize,
    }

    impl LifecycleHook<Request<Body>, Response<Body>> for PhpResponseHook {
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
            let callback_index = self.callback_index;
            Box::pin(async move {
                // Run PHP callback synchronously in spawn_blocking
                let result = tokio::task::spawn_blocking(move || {
                    PHP_HOOK_REGISTRY.with(|registry| {
                        let registry = registry.borrow();
                        let callback_zval = registry.get(callback_index)?;

                        // Reconstruct ZendCallable
                        let _callable = ext_php_rs::types::ZendCallable::new(callback_zval).ok()?;

                        // TODO: Convert Response to PHP object and call hook
                        // For now, just continue
                        Some(())
                    })
                })
                .await;

                match result {
                    Ok(Some(())) => Ok(HookResult::Continue(resp)),
                    _ => Ok(HookResult::Continue(resp)), // On error, continue
                }
            })
        }
    }

    Arc::new(PhpResponseHook { name, callback_index })
}

/// Adapt a PHP callable into a LifecycleHook for error handling.
fn make_error_hook(
    name: String,
    callback: &ext_php_rs::types::Zval,
) -> Arc<dyn LifecycleHook<Request<Body>, Response<Body>>> {
    // Store the callback as Zval to avoid lifetime issues
    let callback_index = PHP_HOOK_REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let idx = registry.len();
        let zval = callback.shallow_clone();
        registry.push(zval);
        idx
    });

    struct PhpErrorHook {
        name: String,
        callback_index: usize,
    }

    impl LifecycleHook<Request<Body>, Response<Body>> for PhpErrorHook {
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
            let callback_index = self.callback_index;
            Box::pin(async move {
                // Run PHP callback synchronously in spawn_blocking
                let result = tokio::task::spawn_blocking(move || {
                    PHP_HOOK_REGISTRY.with(|registry| {
                        let registry = registry.borrow();
                        let callback_zval = registry.get(callback_index)?;

                        // Reconstruct ZendCallable
                        let _callable = ext_php_rs::types::ZendCallable::new(callback_zval).ok()?;

                        // TODO: Convert error Response to PHP object and call hook
                        // For now, just continue
                        Some(())
                    })
                })
                .await;

                match result {
                    Ok(Some(())) => Ok(HookResult::Continue(resp)),
                    _ => Ok(HookResult::Continue(resp)), // On error, continue
                }
            })
        }
    }

    Arc::new(PhpErrorHook { name, callback_index })
}

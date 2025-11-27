//! PHP exposure of lifecycle hooks (onRequest, onResponse).
//!
//! This provides minimal parity by allowing PHP callables to short-circuit or continue.

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
    #[php(name = "onRequest")]
    pub fn on_request(&mut self, name: String, callback: ZendCallable) {
        let hook = make_request_hook(name, callback);
        self.builder = self.builder.on_request(hook);
    }

    /// Register an onResponse hook.
    #[php(name = "onResponse")]
    pub fn on_response(&mut self, name: String, callback: ZendCallable) {
        let hook = make_response_hook(name, callback);
        self.builder = self.builder.on_response(hook);
    }

    /// Finish building the hooks.
    #[php(name = "build")]
    pub fn build(&self) -> LifecycleHooks {
        self.builder.clone().build()
    }
}

/// Adapt a PHP callable into a LifecycleHook for requests.
fn make_request_hook(name: String, callback: ZendCallable) -> Arc<dyn LifecycleHook<Request<Body>, Response<Body>>> {
    struct Hook {
        name: String,
        callback: ZendCallable,
    }

    impl LifecycleHook<Request<Body>, Response<Body>> for Hook {
        fn name(&self) -> &str {
            &self.name
        }

        fn execute_request<'a>(
            &'a self,
            req: Request<Body>,
        ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>, Response<Body>>, String>> + Send + 'a>>
        {
            // Convert request to minimal map for PHP hook. Keep it simple: method + path.
            let method = req.method().to_string();
            let path = req.uri().path().to_string();

            // Build args: [method, path]
            let args: Vec<ext_php_rs::types::Zval> = vec![method.into(), path.into()];

            let result = self
                .callback
                .try_call(args.iter().collect())
                .map_err(|e| format!("PHP hook failed: {:?}", e));

            Box::pin(async move {
                let z = result?;
                // If hook returns null/false => Continue
                if z.is_null() || (z.bool().unwrap_or(false) == false && z.long().is_none()) {
                    return Ok(HookResult::Continue(req));
                }
                // If hook returns int => short-circuit with that status
                if let Some(code) = z.long() {
                    let status = StatusCode::from_u16(code as u16).unwrap_or(StatusCode::OK);
                    let resp = Response::builder()
                        .status(status)
                        .body(Body::empty())
                        .map_err(|e| format!("Failed to build response: {}", e))?;
                    return Ok(HookResult::ShortCircuit(resp));
                }
                // If hook returns string => short-circuit 200 with body
                if let Some(body) = z.string() {
                    let resp = Response::builder()
                        .status(StatusCode::OK)
                        .body(Body::from(body.to_string()))
                        .map_err(|e| format!("Failed to build response: {}", e))?;
                    return Ok(HookResult::ShortCircuit(resp));
                }
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

    Arc::new(Hook { name, callback })
}

/// Adapt a PHP callable into a LifecycleHook for responses.
fn make_response_hook(name: String, callback: ZendCallable) -> Arc<dyn LifecycleHook<Request<Body>, Response<Body>>> {
    struct Hook {
        name: String,
        callback: ZendCallable,
    }

    impl LifecycleHook<Request<Body>, Response<Body>> for Hook {
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
            let status = resp.status();
            let body_bytes = hyper::body::to_bytes(resp.into_body())
                .await
                .map_err(|e| format!("Failed to read response body: {}", e))?;
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap_or_default();

            let args: Vec<ext_php_rs::types::Zval> = vec![status.as_u16().into(), body_str.into()];

            let result = self
                .callback
                .try_call(args.iter().collect())
                .map_err(|e| format!("PHP hook failed: {:?}", e));

            Box::pin(async move {
                let z = result?;
                // If hook returns null/false => Continue
                if z.is_null() || (z.bool().unwrap_or(false) == false && z.long().is_none()) {
                    return Ok(HookResult::Continue(
                        Response::builder()
                            .status(status)
                            .body(Body::from(body_str))
                            .map_err(|e| format!("Failed to rebuild response: {}", e))?,
                    ));
                }
                // If hook returns int => short-circuit with that status
                if let Some(code) = z.long() {
                    let status_override = StatusCode::from_u16(code as u16).unwrap_or(StatusCode::OK);
                    let resp = Response::builder()
                        .status(status_override)
                        .body(Body::from(body_str.clone()))
                        .map_err(|e| format!("Failed to build response: {}", e))?;
                    return Ok(HookResult::ShortCircuit(resp));
                }
                // If hook returns string => short-circuit 200 with new body
                if let Some(body) = z.string() {
                    let resp = Response::builder()
                        .status(StatusCode::OK)
                        .body(Body::from(body.to_string()))
                        .map_err(|e| format!("Failed to build response: {}", e))?;
                    return Ok(HookResult::ShortCircuit(resp));
                }
                Ok(HookResult::Continue(
                    Response::builder()
                        .status(status)
                        .body(Body::from(body_str))
                        .map_err(|e| format!("Failed to rebuild response: {}", e))?,
                ))
            })
        }
    }

    Arc::new(Hook { name, callback })
}

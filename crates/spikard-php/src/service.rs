#![allow(clippy::too_many_arguments, clippy::unused_async)]

use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendCallable, Zval};
use std::panic::AssertUnwindSafe;
use std::sync::Arc;

thread_local! {
    static PHP_HANDLER_REGISTRY: std::cell::RefCell<Vec<ZendCallable<'static>>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

/// Generated ext-php-rs bridge for the `Handler` contract.
///
/// Wraps a PHP callable (stored as an index in a thread-local registry)
/// so it can be used as `Arc<dyn Handler>` from Rust async code.
/// Dispatch blocks on the Tokio runtime (PHP is single-threaded per request).
pub struct PhpHandlerBridge {
    handler_index: usize,
}

impl PhpHandlerBridge {
    /// Create a bridge from a handler index.
    pub fn new(handler_index: usize) -> Self {
        Self { handler_index }
    }
}

// SAFETY: The bridge holds only a usize (immutable, Copy).
// PHP handler registry lookup is thread-safe via thread-local RefCell.
impl Send for PhpHandlerBridge {}
impl Sync for PhpHandlerBridge {}

impl spikard::Handler for PhpHandlerBridge {
    fn call(
        &self,
        _request: spikard::Request<spikard::Body>,
        request_data: spikard::RequestData,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard::HandlerResult> + Send + '_>> {
        Box::pin(async move {
            // Invoke the PHP callable synchronously (blocking)
            let outcome: Result<spikard::Response, Box<dyn std::error::Error + Send + Sync>> = (async {
                // Serialize the request to JSON for PHP roundtrip
                let req_json = serde_json::to_string(&request_data)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

                let raw_result = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    PHP_HANDLER_REGISTRY.with(|registry| -> Result<String, String> {
                        let registry = registry.borrow();
                        let Some(callable) = registry.get(self.handler_index) else {
                            return Err(format!("Handler not found at index {}", self.handler_index));
                        };

                        // Deserialize JSON request into PHP object
                        let req_obj =
                            serde_json::from_str::<serde_json::Value>(&req_json).map_err(|e| e.to_string())?;
                        let req_zval = serde_json::json!(req_obj).into();

                        // Invoke the callable
                        let resp_zval = callable
                            .try_call(vec![&req_zval])
                            .map_err(|e| format!("PHP callable invocation failed: {:?}", e))?;

                        // Serialize response back to JSON
                        Ok(serde_json::to_string(&resp_zval).unwrap_or_else(|_| "{}".to_string()))
                    })
                }))
                .map_err(|_| {
                    Box::new(std::io::Error::new(std::io::ErrorKind::Other, "PHP handler panicked"))
                        as Box<dyn std::error::Error + Send + Sync>
                })?
                .map_err(|e| {
                    Box::new(std::io::Error::new(std::io::ErrorKind::Other, e))
                        as Box<dyn std::error::Error + Send + Sync>
                })?;

                // Deserialize the JSON result back into the wire response DTO.
                let response: spikard::Response = serde_json::from_str(&raw_result)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                Ok(response)
            })
            .await;

            spikard::handler_result_from_response(outcome)
        })
    }
}
/// Drive `spikard::App::run` from PHP.
///
/// Each entry in `registrations` is an array of `[method_name, metadata_array, callable]`
/// produced by the PHP service class.
#[php_function]
pub fn app_run(registrations: &Bound<'_, Zval>) -> PhpResult<()> {
    let mut owner = spikard::App::new();
    // Register all handlers with the owner
    if let Ok(reg_arr) = registrations.try_into::<Vec<Zval>>() {
        for entry in reg_arr {
            if let Ok(tuple) = entry.try_into::<Vec<Zval>>() {
                if tuple.len() < 3 {
                    return Err(PhpException::default("Invalid registration tuple length".into()));
                }
                let method_name: String = tuple[0].try_into()?;
                let callable = tuple[2].clone();

                match method_name.as_str() {
                    "route" => {
                        let handler_index = PHP_HANDLER_REGISTRY.with(|registry| {
                            let mut registry = registry.borrow_mut();
                            let idx = registry.len();
                            // Convert Zval to ZendCallable
                            if let Ok(zen_callable) = ZendCallable::new_owned(callable.clone()) {
                                registry.push(zen_callable);
                                idx
                            } else {
                                usize::MAX
                            }
                        });
                        if handler_index == usize::MAX {
                            return Err(PhpException::default("Failed to register callable".into()));
                        }

                        let bridge = PhpHandlerBridge::new(handler_index);
                        let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                        let meta: Vec<Zval> = tuple[1].clone().try_into()?;
                        let builder: spikard::RouteBuilder = meta
                            .get(0)
                            .ok_or_else(|| PhpException::default("Missing metadata at index 0".into()))?
                            .try_into()?;
                        owner
                            .route(builder, handler)
                            .map_err(|e| PhpException::default(e.to_string()))?;
                    }
                    _ => {
                        return Err(PhpException::default(format!(
                            "unknown registration method: {method_name}"
                        )));
                    }
                }
            }
        }
    }

    tokio::runtime::Handle::current()
        .block_on(owner.run())
        .map_err(|e| PhpException::default(e.to_string()))?;
    Ok(())
}

/// Drive `spikard::App::into_router` from PHP.
///
/// Each entry in `registrations` is an array of `[method_name, metadata_array, callable]`
/// produced by the PHP service class.
#[php_function]
pub fn app_into_router(registrations: &Bound<'_, Zval>) -> PhpResult<()> {
    let mut owner = spikard::App::new();
    // Register all handlers with the owner
    if let Ok(reg_arr) = registrations.try_into::<Vec<Zval>>() {
        for entry in reg_arr {
            if let Ok(tuple) = entry.try_into::<Vec<Zval>>() {
                if tuple.len() < 3 {
                    return Err(PhpException::default("Invalid registration tuple length".into()));
                }
                let method_name: String = tuple[0].try_into()?;
                let callable = tuple[2].clone();

                match method_name.as_str() {
                    "route" => {
                        let handler_index = PHP_HANDLER_REGISTRY.with(|registry| {
                            let mut registry = registry.borrow_mut();
                            let idx = registry.len();
                            // Convert Zval to ZendCallable
                            if let Ok(zen_callable) = ZendCallable::new_owned(callable.clone()) {
                                registry.push(zen_callable);
                                idx
                            } else {
                                usize::MAX
                            }
                        });
                        if handler_index == usize::MAX {
                            return Err(PhpException::default("Failed to register callable".into()));
                        }

                        let bridge = PhpHandlerBridge::new(handler_index);
                        let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                        let meta: Vec<Zval> = tuple[1].clone().try_into()?;
                        let builder: spikard::RouteBuilder = meta
                            .get(0)
                            .ok_or_else(|| PhpException::default("Missing metadata at index 0".into()))?
                            .try_into()?;
                        owner
                            .route(builder, handler)
                            .map_err(|e| PhpException::default(e.to_string()))?;
                    }
                    _ => {
                        return Err(PhpException::default(format!(
                            "unknown registration method: {method_name}"
                        )));
                    }
                }
            }
        }
    }

    owner.into_router().map_err(|e| PhpException::default(e.to_string()))?;
    Ok(())
}

//! Native entrypoints for starting/stopping the server from PHP.

use ext_php_rs::prelude::*;
use spikard_http::server::build_router_with_handlers_and_config;
use spikard_http::{LifecycleHooks, Route};
use std::sync::Arc;

use crate::php::handler::PhpHandler;

/// Payload for a registered route coming from PHP.
#[derive(Debug, serde::Deserialize)]
pub struct RegisteredRoutePayload {
    pub method: String,
    pub path: String,
    pub handler_name: String,
    #[serde(skip)]
    pub handler: Option<ext_php_rs::types::ZendCallable>,
    pub request_schema: Option<serde_json::Value>,
    pub response_schema: Option<serde_json::Value>,
    pub parameter_schema: Option<serde_json::Value>,
}

impl RegisteredRoutePayload {
    pub fn into_route(self) -> Result<Route, String> {
        Ok(Route {
            method: self
                .method
                .parse()
                .map_err(|e| format!("Invalid method {}: {}", self.method, e))?,
            path: self.path,
            handler_name: self.handler_name,
            request_validator: None,
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: false,
            cors: None,
            expects_json_body: self.request_schema.is_some(),
            #[cfg(feature = "di")]
            handler_dependencies: Vec::new(),
        })
    }
}

/// Start a server from PHP, given route/config payloads.
#[php_function]
#[php(name = "spikard_start_server")]
pub fn spikard_start_server(
    routes: Vec<serde_json::Value>,
    config: serde_json::Value,
    hooks: serde_json::Value,
) -> PhpResult<u64> {
    // Deserialize config and hooks into ServerConfig/LifecycleHooks
    let server_config: spikard_http::ServerConfig =
        serde_json::from_value(config).map_err(|e| PhpException::default(format!("Invalid server config: {}", e)))?;

    // Rehydrate hooks (optional)
    let lifecycle_hooks = serde_json::from_value::<Option<LifecycleHooks>>(hooks)
        .map_err(|e| PhpException::default(format!("Invalid lifecycle hooks: {}", e)))?;

    // Rebuild routes with handlers
    let mut route_pairs: Vec<(spikard_http::Route, Arc<dyn spikard_http::Handler>)> = Vec::new();
    for route_val in routes {
        let reg = serde_json::from_value::<RegisteredRoutePayload>(route_val)
            .map_err(|e| PhpException::default(format!("Invalid route payload: {}", e)))?;

        // Handler is provided via separate array in PHP (handler index not used here)
        let handler_callable = reg
            .handler
            .ok_or_else(|| PhpException::default("Missing handler callable"))?;
        let handler = PhpHandler::register(
            handler_callable,
            reg.handler_name.clone(),
            reg.method.clone(),
            reg.path.clone(),
        );

        let mut route = reg.into_route()?;

        // Apply schemas if provided
        if let Some(schema) = reg.request_schema {
            let compiled = spikard_core::validation::SchemaValidator::new(schema)
                .map_err(|e| PhpException::default(format!("Invalid request schema: {}", e)))?;
            route.request_validator = Some(Arc::new(compiled));
        }
        if let Some(schema) = reg.response_schema {
            let compiled = spikard_core::validation::SchemaValidator::new(schema)
                .map_err(|e| PhpException::default(format!("Invalid response schema: {}", e)))?;
            route.response_validator = Some(Arc::new(compiled));
        }
        if let Some(schema) = reg.parameter_schema {
            let compiled =
                spikard_http::ParameterValidator::new(schema).map_err(|e| PhpException::default(format!("{}", e)))?;
            route.parameter_validator = Some(compiled);
        }

        route_pairs.push((route, Arc::new(handler) as Arc<dyn spikard_http::Handler>));
    }

    let hooks_arc = lifecycle_hooks.as_ref().map(Arc::new);
    let app = build_router_with_handlers_and_config(route_pairs, server_config.clone(), hooks_arc)
        .map_err(|e| PhpException::default(format!("Failed to build router: {}", e)))?;

    // Spawn server in background
    let handle = std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
        rt.block_on(async move {
            if let Err(e) = spikard_http::Server::run_with_config(app, server_config).await {
                eprintln!("Server error: {e}");
            }
        });
    });

    // Return a fake id; real shutdown tracking TBD.
    let id = handle.thread().id().as_u64().unwrap_or(0);
    std::mem::forget(handle);
    Ok(id)
}

/// Stop server placeholder (no-op).
#[php_function]
#[php(name = "spikard_stop_server")]
pub fn spikard_stop_server(_handle: u64) -> PhpResult<()> {
    // TODO: Implement graceful shutdown tracking handles.
    Ok(())
}

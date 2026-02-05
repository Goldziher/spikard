//! Elixir lifecycle hooks implementation.
//!
//! This module provides `ElixirLifecycleHook` which implements the `LifecycleHook` trait
//! for executing Elixir hook functions at various points in the request lifecycle.
//! Uses message passing via OwnedEnv to communicate with Elixir GenServer.

#![allow(dead_code)]
#![deny(clippy::unwrap_used)]

use axum::body::Body;
use axum::http::{HeaderName, HeaderValue, Request, Response, StatusCode};
use once_cell::sync::Lazy;
use rustler::{Encoder, Env, LocalPid, NifResult, OwnedEnv, Term};
use serde_json::{json, Value as JsonValue};
use spikard_http::lifecycle::{HookResult, LifecycleHook};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::oneshot;
use tracing::{debug, warn};

use crate::atoms;
use crate::conversion::json_to_elixir;

/// Hook type enum for identifying which hook phase is being executed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookType {
    OnRequest,
    PreValidation,
    PreHandler,
    OnResponse,
    OnError,
}

impl HookType {
    /// Convert to Elixir atom name.
    pub fn to_atom_str(&self) -> &'static str {
        match self {
            HookType::OnRequest => "on_request",
            HookType::PreValidation => "pre_validation",
            HookType::PreHandler => "pre_handler",
            HookType::OnResponse => "on_response",
            HookType::OnError => "on_error",
        }
    }
}

/// Hook execution result from Elixir.
#[derive(Debug, Clone)]
pub enum ElixirHookResult {
    /// Continue with potentially modified context/response
    Continue(JsonValue),
    /// Short-circuit with early response
    ShortCircuit(JsonValue),
}

/// Global map of pending hook requests keyed by unique request ID.
/// Used to deliver responses from Elixir back to waiting Rust hooks.
static PENDING_HOOK_REQUESTS: Lazy<Mutex<HashMap<u64, oneshot::Sender<ElixirHookResult>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Counter for generating unique hook request IDs.
static HOOK_REQUEST_ID_COUNTER: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));

/// Generate a unique hook request ID.
fn next_hook_request_id() -> u64 {
    let mut counter = HOOK_REQUEST_ID_COUNTER.lock().unwrap_or_else(|e| e.into_inner());
    *counter = counter.wrapping_add(1);
    *counter
}

/// Register a pending hook request and return its ID.
fn register_pending_hook_request(sender: oneshot::Sender<ElixirHookResult>) -> u64 {
    let id = next_hook_request_id();
    let mut pending = PENDING_HOOK_REQUESTS.lock().unwrap_or_else(|e| e.into_inner());
    pending.insert(id, sender);
    id
}

/// Deliver a hook response to a pending request by its ID.
/// Returns true if the request was found and response delivered.
pub fn deliver_hook_response(request_id: u64, result: ElixirHookResult) -> bool {
    let sender = {
        let mut pending = PENDING_HOOK_REQUESTS.lock().unwrap_or_else(|e| e.into_inner());
        pending.remove(&request_id)
    };

    match sender {
        Some(tx) => tx.send(result).is_ok(),
        None => {
            warn!("No pending hook request found for ID {}", request_id);
            false
        }
    }
}

/// NIF to deliver a hook response from Elixir back to the waiting Rust hook.
///
/// Called by the HandlerRunner GenServer after processing a hook.
///
/// # Arguments
///
/// * `request_id` - The unique ID of the hook request
/// * `result_type` - Atom :continue or :short_circuit
/// * `payload` - The hook result payload (context or response)
#[rustler::nif(name = "deliver_hook_response")]
pub fn deliver_hook_response_nif<'a>(
    env: Env<'a>,
    request_id: u64,
    result_type: Term<'a>,
    payload: Term<'a>,
) -> NifResult<Term<'a>> {
    // Convert payload to JSON
    let payload_json = crate::conversion::elixir_to_json(env, payload)?;

    // Determine result type
    let result = if result_type.decode::<rustler::Atom>().map(|a| a == atoms::continue_()).unwrap_or(false) {
        ElixirHookResult::Continue(payload_json)
    } else if result_type.decode::<rustler::Atom>().map(|a| a == atoms::short_circuit()).unwrap_or(false) {
        ElixirHookResult::ShortCircuit(payload_json)
    } else {
        // Default to continue
        ElixirHookResult::Continue(payload_json)
    };

    if deliver_hook_response(request_id, result) {
        Ok(atoms::ok().encode(env))
    } else {
        Ok((atoms::error(), atoms::not_implemented()).encode(env))
    }
}

/// Elixir lifecycle hook that delegates execution to an Elixir process.
///
/// This struct implements the `LifecycleHook` trait and uses message passing
/// to invoke Elixir hook functions via a GenServer.
pub struct ElixirLifecycleHook {
    /// PID of the handler runner GenServer that manages hooks
    handler_runner_pid: Arc<LocalPid>,
    /// Type of hook (on_request, pre_handler, etc.)
    hook_type: HookType,
    /// Index of this hook in the hooks list (for ordering)
    hook_index: usize,
    /// Name for debugging/tracing
    name: String,
    /// Timeout for hook execution in milliseconds
    timeout_ms: u64,
}

/// Default timeout for hook execution (30 seconds).
const DEFAULT_HOOK_TIMEOUT_MS: u64 = 30_000;

impl ElixirLifecycleHook {
    /// Create a new Elixir lifecycle hook.
    ///
    /// # Arguments
    ///
    /// * `handler_runner_pid` - PID of the HandlerRunner GenServer
    /// * `hook_type` - The type of hook (on_request, pre_handler, etc.)
    /// * `hook_index` - Index of this hook in the hooks list
    pub fn new(
        handler_runner_pid: LocalPid,
        hook_type: HookType,
        hook_index: usize,
    ) -> Self {
        Self {
            handler_runner_pid: Arc::new(handler_runner_pid),
            hook_type,
            hook_index,
            name: format!("elixir_{}_hook_{}", hook_type.to_atom_str(), hook_index),
            timeout_ms: DEFAULT_HOOK_TIMEOUT_MS,
        }
    }

    /// Create a new Elixir lifecycle hook with custom timeout.
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }

    /// Execute a request-phase hook (on_request, pre_validation, pre_handler).
    async fn execute_request_hook(
        &self,
        req: Request<Body>,
    ) -> Result<HookResult<Request<Body>, Response<Body>>, String> {
        // Convert request to JSON context
        let context = request_to_hook_context(&req);

        // Create response channel
        let (tx, rx) = oneshot::channel();
        let request_id = register_pending_hook_request(tx);

        debug!(
            "Sending hook request {} for {} hook index {}",
            request_id,
            self.hook_type.to_atom_str(),
            self.hook_index
        );

        // Send request to HandlerRunner via OwnedEnv
        let pid = *self.handler_runner_pid;
        let hook_type = self.hook_type;
        let hook_index = self.hook_index;
        let send_result = send_hook_request(pid, request_id, hook_type, hook_index, &context);

        if let Err(e) = send_result {
            warn!("Failed to send hook request to HandlerRunner: {}", e);
            return Ok(HookResult::Continue(req));
        }

        // Wait for response with timeout
        let timeout = Duration::from_millis(self.timeout_ms);
        let result = match tokio::time::timeout(timeout, rx).await {
            Ok(Ok(result)) => result,
            Ok(Err(_)) => {
                warn!("Hook response channel closed unexpectedly for request {}", request_id);
                return Ok(HookResult::Continue(req));
            }
            Err(_) => {
                warn!("Hook request timed out for request {}", request_id);
                return Ok(HookResult::Continue(req));
            }
        };

        // Interpret the result
        match result {
            ElixirHookResult::Continue(context_json) => {
                // Reconstruct request with potentially modified context
                let modified_req = apply_context_to_request(req, &context_json);
                Ok(HookResult::Continue(modified_req))
            }
            ElixirHookResult::ShortCircuit(response_json) => {
                let response = json_to_response(&response_json)?;
                Ok(HookResult::ShortCircuit(response))
            }
        }
    }

    /// Execute a response-phase hook (on_response, on_error).
    async fn execute_response_hook(
        &self,
        resp: Response<Body>,
    ) -> Result<HookResult<Response<Body>, Response<Body>>, String> {
        // Convert response to JSON context
        let context = response_to_hook_context(&resp);

        // Create response channel
        let (tx, rx) = oneshot::channel();
        let request_id = register_pending_hook_request(tx);

        debug!(
            "Sending response hook request {} for {} hook index {}",
            request_id,
            self.hook_type.to_atom_str(),
            self.hook_index
        );

        // Send request to HandlerRunner via OwnedEnv
        let pid = *self.handler_runner_pid;
        let hook_type = self.hook_type;
        let hook_index = self.hook_index;
        let send_result = send_hook_request(pid, request_id, hook_type, hook_index, &context);

        if let Err(e) = send_result {
            warn!("Failed to send response hook request to HandlerRunner: {}", e);
            return Ok(HookResult::Continue(resp));
        }

        // Wait for response with timeout
        let timeout = Duration::from_millis(self.timeout_ms);
        let result = match tokio::time::timeout(timeout, rx).await {
            Ok(Ok(result)) => result,
            Ok(Err(_)) => {
                warn!("Response hook channel closed unexpectedly for request {}", request_id);
                return Ok(HookResult::Continue(resp));
            }
            Err(_) => {
                warn!("Response hook request timed out for request {}", request_id);
                return Ok(HookResult::Continue(resp));
            }
        };

        // Interpret the result
        match result {
            ElixirHookResult::Continue(response_json) | ElixirHookResult::ShortCircuit(response_json) => {
                // For response hooks, both continue and short_circuit result in the response
                let modified_resp = json_to_response(&response_json)?;
                Ok(HookResult::Continue(modified_resp))
            }
        }
    }
}

impl LifecycleHook<Request<Body>, Response<Body>> for ElixirLifecycleHook {
    fn name(&self) -> &str {
        &self.name
    }

    fn execute_request<'a>(
        &self,
        req: Request<Body>,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>, Response<Body>>, String>> + Send + 'a>>
    {
        let pid = Arc::clone(&self.handler_runner_pid);
        let hook_type = self.hook_type;
        let hook_index = self.hook_index;
        let timeout_ms = self.timeout_ms;

        Box::pin(async move {
            let hook = ElixirLifecycleHook {
                handler_runner_pid: pid,
                hook_type,
                hook_index,
                name: format!("elixir_{}_hook_{}", hook_type.to_atom_str(), hook_index),
                timeout_ms,
            };
            hook.execute_request_hook(req).await
        })
    }

    fn execute_response<'a>(
        &self,
        resp: Response<Body>,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Response<Body>, Response<Body>>, String>> + Send + 'a>>
    {
        let pid = Arc::clone(&self.handler_runner_pid);
        let hook_type = self.hook_type;
        let hook_index = self.hook_index;
        let timeout_ms = self.timeout_ms;

        Box::pin(async move {
            let hook = ElixirLifecycleHook {
                handler_runner_pid: pid,
                hook_type,
                hook_index,
                name: format!("elixir_{}_hook_{}", hook_type.to_atom_str(), hook_index),
                timeout_ms,
            };
            hook.execute_response_hook(resp).await
        })
    }
}

/// Send a hook request to the Elixir HandlerRunner via OwnedEnv.
///
/// Message format: {:hook_execute, hook_type, hook_index, request_id, context}
fn send_hook_request(
    handler_runner_pid: LocalPid,
    request_id: u64,
    hook_type: HookType,
    hook_index: usize,
    context: &JsonValue,
) -> Result<(), String> {
    let context = context.clone();

    let mut owned_env = OwnedEnv::new();

    let send_result = owned_env.send_and_clear(&handler_runner_pid, |env| {
        // Build the hook_execute atom
        let hook_execute_atom = atoms::hook_execute();

        // Build hook type atom
        let hook_type_atom = match hook_type {
            HookType::OnRequest => atoms::on_request(),
            HookType::PreValidation => atoms::pre_validation(),
            HookType::PreHandler => atoms::pre_handler(),
            HookType::OnResponse => atoms::on_response(),
            HookType::OnError => atoms::on_error(),
        };

        let hook_index_term = hook_index.encode(env);
        let request_id_term = request_id.encode(env);

        // Convert context to Elixir term
        let context_term = match json_to_elixir(env, &context) {
            Ok(t) => t,
            Err(e) => {
                warn!("Failed to convert hook context to Elixir: {:?}", e);
                return atoms::error().encode(env);
            }
        };

        // Build message tuple: {:hook_execute, hook_type, hook_index, request_id, context}
        (hook_execute_atom, hook_type_atom, hook_index_term, request_id_term, context_term).encode(env)
    });

    match send_result {
        Ok(()) => {
            debug!("Successfully sent hook request {}", request_id);
            Ok(())
        }
        Err(e) => {
            warn!("Failed to send hook request: {:?}", e);
            Err(format!("Send error: {:?}", e))
        }
    }
}

/// Convert a Request to a hook context JSON value.
fn request_to_hook_context(req: &Request<Body>) -> JsonValue {
    let headers: HashMap<String, String> = req
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string().to_lowercase(), v.to_str().unwrap_or("").to_string()))
        .collect();

    json!({
        "method": req.method().to_string(),
        "path": req.uri().path().to_string(),
        "query": req.uri().query().unwrap_or(""),
        "headers": headers,
    })
}

/// Convert a Response to a hook context JSON value.
fn response_to_hook_context(resp: &Response<Body>) -> JsonValue {
    let headers: HashMap<String, String> = resp
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string().to_lowercase(), v.to_str().unwrap_or("").to_string()))
        .collect();

    json!({
        "status": resp.status().as_u16(),
        "headers": headers,
        // Note: We can't easily access the body here as it's a stream
        // The body will be passed separately or reconstructed
    })
}

/// Apply a modified context back to a request.
fn apply_context_to_request(mut req: Request<Body>, context: &JsonValue) -> Request<Body> {
    // Apply modified headers if present
    if let Some(headers_obj) = context.get("headers").and_then(|v| v.as_object()) {
        for (key, value) in headers_obj {
            if let Some(value_str) = value.as_str() {
                if let (Ok(name), Ok(val)) = (
                    HeaderName::from_bytes(key.as_bytes()),
                    HeaderValue::from_str(value_str),
                ) {
                    req.headers_mut().insert(name, val);
                }
            }
        }
    }

    req
}

/// Convert a JSON response representation to an HTTP Response.
fn json_to_response(json: &JsonValue) -> Result<Response<Body>, String> {
    let status = json
        .get("status")
        .and_then(|v| v.as_u64())
        .unwrap_or(200) as u16;

    let status_code = StatusCode::from_u16(status).unwrap_or(StatusCode::OK);

    let mut builder = Response::builder().status(status_code);

    // Add headers
    if let Some(headers_obj) = json.get("headers").and_then(|v| v.as_object()) {
        for (key, value) in headers_obj {
            if let Some(value_str) = value.as_str() {
                builder = builder.header(key.as_str(), value_str);
            }
        }
    }

    // Build body
    let body = if let Some(body_val) = json.get("body") {
        if let Some(s) = body_val.as_str() {
            Body::from(s.to_string())
        } else {
            Body::from(serde_json::to_vec(body_val).unwrap_or_default())
        }
    } else {
        Body::empty()
    };

    // Ensure content-type is set for JSON bodies
    if json.get("body").is_some() && !json.get("headers").and_then(|h| h.get("content-type")).is_some() {
        builder = builder.header("content-type", "application/json");
    }

    builder.body(body).map_err(|e| format!("Failed to build response: {}", e))
}

/// Create lifecycle hooks from Elixir configuration.
///
/// Returns a LifecycleHooks struct populated with ElixirLifecycleHook instances.
pub fn create_lifecycle_hooks(
    handler_runner_pid: LocalPid,
    hook_counts: HookCounts,
) -> spikard_http::lifecycle::LifecycleHooks {
    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    // Add on_request hooks
    for i in 0..hook_counts.on_request {
        let hook = ElixirLifecycleHook::new(handler_runner_pid, HookType::OnRequest, i);
        hooks.add_on_request(Arc::new(hook));
    }

    // Add pre_validation hooks
    for i in 0..hook_counts.pre_validation {
        let hook = ElixirLifecycleHook::new(handler_runner_pid, HookType::PreValidation, i);
        hooks.add_pre_validation(Arc::new(hook));
    }

    // Add pre_handler hooks
    for i in 0..hook_counts.pre_handler {
        let hook = ElixirLifecycleHook::new(handler_runner_pid, HookType::PreHandler, i);
        hooks.add_pre_handler(Arc::new(hook));
    }

    // Add on_response hooks
    for i in 0..hook_counts.on_response {
        let hook = ElixirLifecycleHook::new(handler_runner_pid, HookType::OnResponse, i);
        hooks.add_on_response(Arc::new(hook));
    }

    // Add on_error hooks
    for i in 0..hook_counts.on_error {
        let hook = ElixirLifecycleHook::new(handler_runner_pid, HookType::OnError, i);
        hooks.add_on_error(Arc::new(hook));
    }

    hooks
}

/// Counts of hooks for each lifecycle phase.
#[derive(Debug, Clone, Default)]
pub struct HookCounts {
    pub on_request: usize,
    pub pre_validation: usize,
    pub pre_handler: usize,
    pub on_response: usize,
    pub on_error: usize,
}

impl HookCounts {
    pub fn is_empty(&self) -> bool {
        self.on_request == 0
            && self.pre_validation == 0
            && self.pre_handler == 0
            && self.on_response == 0
            && self.on_error == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hook_type_to_atom_str() {
        assert_eq!(HookType::OnRequest.to_atom_str(), "on_request");
        assert_eq!(HookType::PreValidation.to_atom_str(), "pre_validation");
        assert_eq!(HookType::PreHandler.to_atom_str(), "pre_handler");
        assert_eq!(HookType::OnResponse.to_atom_str(), "on_response");
        assert_eq!(HookType::OnError.to_atom_str(), "on_error");
    }

    #[test]
    fn test_hook_counts_is_empty() {
        let counts = HookCounts::default();
        assert!(counts.is_empty());

        let counts = HookCounts { on_request: 1, ..Default::default() };
        assert!(!counts.is_empty());
    }

    #[test]
    fn test_request_to_hook_context() {
        let req = Request::builder()
            .method("POST")
            .uri("/test?foo=bar")
            .header("content-type", "application/json")
            .body(Body::empty())
            .expect("Failed to build request");

        let context = request_to_hook_context(&req);

        assert_eq!(context["method"], "POST");
        assert_eq!(context["path"], "/test");
        assert_eq!(context["query"], "foo=bar");
        assert_eq!(context["headers"]["content-type"], "application/json");
    }

    #[test]
    fn test_json_to_response() {
        let json = json!({
            "status": 201,
            "headers": {"x-custom": "value"},
            "body": {"id": 123}
        });

        let resp = json_to_response(&json).expect("Failed to create response");

        assert_eq!(resp.status(), StatusCode::CREATED);
        assert_eq!(resp.headers().get("x-custom").map(|v| v.to_str().ok()), Some(Some("value")));
    }
}

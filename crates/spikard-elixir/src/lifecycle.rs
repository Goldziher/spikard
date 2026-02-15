//! Lifecycle hook integration for Elixir bindings.
//!
//! Spikard executes lifecycle hooks in Rust (spikard-http). For Elixir, the hook
//! functions live in the `Spikard.HandlerRunner` process; Rust sends
//! `{:hook_execute, hook_type, hook_index, request_id, context}` messages to that
//! process and waits for `deliver_hook_response/3` to be called.

#![allow(dead_code)]
#![deny(clippy::unwrap_used)]

use axum::body::Body;
use axum::http::{HeaderName, HeaderValue, Request, Response, StatusCode};
use once_cell::sync::Lazy;
use rustler::{Encoder, Env, LocalPid, NifResult, OwnedEnv, Term};
use serde_json::{Value as JsonValue, json};
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use tokio::sync::oneshot;
use tracing::warn;

use crate::atoms;
use crate::conversion::{elixir_to_json, json_to_elixir};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HookResultType {
    Continue,
    ShortCircuit,
    Error,
}

#[derive(Debug, Clone)]
struct HookResponse {
    kind: HookResultType,
    payload: JsonValue,
}

static HOOK_REQUEST_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

static PENDING_HOOK_REQUESTS: Lazy<Mutex<HashMap<u64, oneshot::Sender<HookResponse>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

fn next_hook_request_id() -> u64 {
    HOOK_REQUEST_ID_COUNTER.fetch_add(1, Ordering::Relaxed).wrapping_add(1)
}

fn register_pending_hook(sender: oneshot::Sender<HookResponse>) -> u64 {
    let id = next_hook_request_id();
    let mut pending = PENDING_HOOK_REQUESTS.lock().unwrap_or_else(|e| e.into_inner());
    pending.insert(id, sender);
    id
}

fn deliver_hook_response_inner(request_id: u64, response: HookResponse) -> bool {
    let sender = {
        let mut pending = PENDING_HOOK_REQUESTS.lock().unwrap_or_else(|e| e.into_inner());
        pending.remove(&request_id)
    };

    if let Some(tx) = sender {
        tx.send(response).is_ok()
    } else {
        warn!("No pending hook request found for ID {}", request_id);
        false
    }
}

#[rustler::nif]
pub fn deliver_hook_response<'a>(
    env: Env<'a>,
    request_id: u64,
    result_type: Term<'a>,
    payload: Term<'a>,
) -> NifResult<Term<'a>> {
    let atom = result_type.decode::<rustler::Atom>();
    let kind = match atom {
        Ok(a) if a == atoms::continue_() => HookResultType::Continue,
        Ok(a) if a == atoms::short_circuit() => HookResultType::ShortCircuit,
        Ok(a) if a == atoms::error() => HookResultType::Error,
        _ => HookResultType::Error,
    };

    let payload_json = elixir_to_json(env, payload)?;
    let response = HookResponse {
        kind,
        payload: payload_json,
    };

    if deliver_hook_response_inner(request_id, response) {
        Ok(atoms::ok().encode(env))
    } else {
        Ok((atoms::error(), atoms::not_implemented()).encode(env))
    }
}

#[derive(Debug, Clone, Default)]
pub struct LifecycleHookCounts {
    pub on_request: usize,
    pub pre_validation: usize,
    pub pre_handler: usize,
    pub on_response: usize,
    pub on_error: usize,
}

fn request_context_from_req(req: &Request<Body>) -> JsonValue {
    let headers: HashMap<String, String> = req
        .headers()
        .iter()
        .filter_map(|(name, value)| {
            value
                .to_str()
                .ok()
                .map(|v| (name.as_str().to_ascii_lowercase(), v.to_string()))
        })
        .collect();

    json!({
        "method": req.method().as_str(),
        "path": req.uri().path(),
        "query": req.uri().query().unwrap_or(""),
        "headers": headers,
    })
}

fn response_context_from_resp(resp: &Response<Body>) -> JsonValue {
    let headers: HashMap<String, String> = resp
        .headers()
        .iter()
        .filter_map(|(name, value)| {
            value
                .to_str()
                .ok()
                .map(|v| (name.as_str().to_ascii_lowercase(), v.to_string()))
        })
        .collect();

    json!({
        "status_code": resp.status().as_u16(),
        "headers": headers,
    })
}

fn apply_headers(headers: &mut axum::http::HeaderMap, updates: &JsonValue) {
    let Some(map) = updates.as_object() else { return };
    for (name, value) in map {
        let Some(value) = value.as_str() else { continue };
        let Ok(header_name) = HeaderName::from_bytes(name.as_bytes()) else {
            continue;
        };
        let Ok(header_value) = HeaderValue::from_str(value) else {
            continue;
        };
        headers.insert(header_name, header_value);
    }
}

fn hook_short_circuit_response(payload: &JsonValue) -> Result<Response<Body>, String> {
    let status = payload
        .get("status")
        .and_then(|v| v.as_u64())
        .or_else(|| payload.get("status_code").and_then(|v| v.as_u64()))
        .unwrap_or(200);

    let mut builder = Response::builder()
        .status(StatusCode::from_u16(status as u16).map_err(|e| format!("Invalid status code {status}: {e}"))?);

    if let Some(headers) = payload.get("headers").and_then(|h| h.as_object()) {
        for (name, value) in headers {
            if let Some(value) = value.as_str() {
                if let Ok(header_name) = HeaderName::from_bytes(name.as_bytes()) {
                    if let Ok(header_value) = HeaderValue::from_str(value) {
                        builder = builder.header(header_name, header_value);
                    }
                }
            }
        }
    }

    let body_value = payload.get("body").cloned();
    let body_bytes = match body_value {
        None | Some(JsonValue::Null) => Vec::new(),
        Some(JsonValue::String(s)) => s.into_bytes(),
        Some(v) => serde_json::to_vec(&v).map_err(|e| format!("Failed to serialize hook body: {e}"))?,
    };

    builder
        .body(Body::from(body_bytes))
        .map_err(|e| format!("Failed to build hook response: {e}"))
}

fn send_hook_execute(
    handler_runner_pid: LocalPid,
    hook_type: &str,
    hook_index: usize,
    request_id: u64,
    context: &JsonValue,
) -> Result<(), String> {
    let hook_type = hook_type.to_string();
    let context = context.clone();

    let owned_env = OwnedEnv::new();
    let direct_send = owned_env.run(|env| {
        let hook_execute_atom = rustler::Atom::from_str(env, "hook_execute")
            .map_err(|_| "Failed to create hook_execute atom".to_string())?;
        let hook_type_atom =
            rustler::Atom::from_str(env, &hook_type).map_err(|_| "Failed to create hook_type atom".to_string())?;

        let msg = (
            hook_execute_atom,
            hook_type_atom,
            hook_index as u64,
            request_id,
            json_to_elixir(env, &context).map_err(|e| format!("Failed to convert hook context: {e:?}"))?,
        )
            .encode(env);

        env.send(&handler_runner_pid, msg)
            .map_err(|_| "Failed to send hook_execute message".to_string())
    });

    if direct_send.is_ok() {
        return Ok(());
    }

    std::thread::spawn(move || {
        let owned_env = OwnedEnv::new();
        owned_env.run(|env| {
            let hook_execute_atom = rustler::Atom::from_str(env, "hook_execute")
                .map_err(|_| "Failed to create hook_execute atom".to_string())?;
            let hook_type_atom =
                rustler::Atom::from_str(env, &hook_type).map_err(|_| "Failed to create hook_type atom".to_string())?;

            let msg = (
                hook_execute_atom,
                hook_type_atom,
                hook_index as u64,
                request_id,
                json_to_elixir(env, &context).map_err(|e| format!("Failed to convert hook context: {e:?}"))?,
            )
                .encode(env);

            env.send(&handler_runner_pid, msg)
                .map_err(|_| "Failed to send hook_execute message".to_string())
        })
    })
    .join()
    .map_err(|_| "Failed to send hook_execute message".to_string())?
}

async fn execute_request_hook(
    handler_runner_pid: LocalPid,
    hook_type: &'static str,
    hook_index: usize,
    req: Request<Body>,
) -> Result<spikard_http::HookResult<Request<Body>, Response<Body>>, String> {
    let context = request_context_from_req(&req);

    let (tx, rx) = oneshot::channel();
    let request_id = register_pending_hook(tx);

    send_hook_execute(handler_runner_pid, hook_type, hook_index, request_id, &context)?;

    let timeout = Duration::from_secs(5);
    let response = match tokio::time::timeout(timeout, rx).await {
        Ok(Ok(r)) => r,
        Ok(Err(_)) => return Err("Hook response channel closed unexpectedly".to_string()),
        Err(_) => return Err("Hook execution timed out".to_string()),
    };

    match response.kind {
        HookResultType::Continue => {
            let (mut parts, body) = req.into_parts();
            if let Some(headers) = response.payload.get("headers") {
                apply_headers(&mut parts.headers, headers);
            }
            Ok(spikard_http::HookResult::Continue(Request::from_parts(parts, body)))
        }
        HookResultType::ShortCircuit => {
            let resp = hook_short_circuit_response(&response.payload)?;
            Ok(spikard_http::HookResult::ShortCircuit(resp))
        }
        HookResultType::Error => Err("Hook execution failed".to_string()),
    }
}

async fn execute_response_hook(
    handler_runner_pid: LocalPid,
    hook_type: &'static str,
    hook_index: usize,
    resp: Response<Body>,
) -> Result<spikard_http::HookResult<Response<Body>, Response<Body>>, String> {
    let context = response_context_from_resp(&resp);

    let (tx, rx) = oneshot::channel();
    let request_id = register_pending_hook(tx);

    send_hook_execute(handler_runner_pid, hook_type, hook_index, request_id, &context)?;

    let timeout = Duration::from_secs(5);
    let response = match tokio::time::timeout(timeout, rx).await {
        Ok(Ok(r)) => r,
        Ok(Err(_)) => return Err("Hook response channel closed unexpectedly".to_string()),
        Err(_) => return Err("Hook execution timed out".to_string()),
    };

    match response.kind {
        HookResultType::Continue => {
            let (mut parts, body) = resp.into_parts();
            if let Some(headers) = response.payload.get("headers") {
                apply_headers(&mut parts.headers, headers);
            }
            if let Some(status_code) = response.payload.get("status_code").and_then(|v| v.as_u64()) {
                if let Ok(status) = StatusCode::from_u16(status_code as u16) {
                    parts.status = status;
                }
            }
            Ok(spikard_http::HookResult::Continue(Response::from_parts(parts, body)))
        }
        HookResultType::ShortCircuit => {
            let resp = hook_short_circuit_response(&response.payload)?;
            Ok(spikard_http::HookResult::ShortCircuit(resp))
        }
        HookResultType::Error => Err("Hook execution failed".to_string()),
    }
}

pub fn build_lifecycle_hooks(
    handler_runner_pid: LocalPid,
    counts: &LifecycleHookCounts,
) -> spikard_http::LifecycleHooks {
    let mut hooks = spikard_http::LifecycleHooks::new();

    for idx in 0..counts.on_request {
        let pid = handler_runner_pid;
        hooks.add_on_request(spikard_http::request_hook(
            format!("on_request_hook_{idx}"),
            move |req| execute_request_hook(pid, "on_request", idx, req),
        ));
    }
    for idx in 0..counts.pre_validation {
        let pid = handler_runner_pid;
        hooks.add_pre_validation(spikard_http::request_hook(
            format!("pre_validation_hook_{idx}"),
            move |req| execute_request_hook(pid, "pre_validation", idx, req),
        ));
    }
    for idx in 0..counts.pre_handler {
        let pid = handler_runner_pid;
        hooks.add_pre_handler(spikard_http::request_hook(
            format!("pre_handler_hook_{idx}"),
            move |req| execute_request_hook(pid, "pre_handler", idx, req),
        ));
    }
    for idx in 0..counts.on_response {
        let pid = handler_runner_pid;
        hooks.add_on_response(spikard_http::response_hook(
            format!("on_response_hook_{idx}"),
            move |resp| execute_response_hook(pid, "on_response", idx, resp),
        ));
    }
    for idx in 0..counts.on_error {
        let pid = handler_runner_pid;
        hooks.add_on_error(spikard_http::response_hook(
            format!("on_error_hook_{idx}"),
            move |resp| execute_response_hook(pid, "on_error", idx, resp),
        ));
    }

    hooks
}

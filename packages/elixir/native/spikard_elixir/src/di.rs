//! Elixir dependency injection implementation.
//!
//! This module provides support for factory-based dependency injection in Elixir handlers.
//! Factories are functions that create instances of dependencies based on the current request.
//! This module handles the async request/response pattern for invoking Elixir factories
//! and delivering their results back to waiting handlers.
//!
//! # Architecture
//!
//! The DI factory resolution follows the same pattern as handlers and lifecycle hooks:
//!
//! 1. When a factory needs to be resolved, Rust creates a oneshot channel and registers
//!    a pending request with a unique ID
//! 2. Rust sends a message to the HandlerRunner GenServer requesting factory resolution
//! 3. Elixir invokes the factory function with the request context
//! 4. Elixir calls `deliver_factory_response/3` with the factory result
//! 5. Rust converts the Elixir term to JSON and delivers it to the waiting channel
//! 6. The waiting handler receives the resolved dependency and continues processing

#![allow(dead_code)]
#![deny(clippy::unwrap_used)]

use once_cell::sync::Lazy;
use rustler::{Encoder, Env, LocalPid, NifResult, OwnedEnv, Term};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::sync::oneshot;
use tracing::{debug, warn};

use crate::atoms;

/// Global map of pending factory requests keyed by unique request ID.
/// Used to deliver responses from Elixir factories back to waiting Rust code.
static PENDING_FACTORY_REQUESTS: Lazy<Mutex<HashMap<u64, oneshot::Sender<JsonValue>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Counter for generating unique factory request IDs.
static FACTORY_REQUEST_ID_COUNTER: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));

/// Generate a unique factory request ID.
fn next_factory_request_id() -> u64 {
    let mut counter = FACTORY_REQUEST_ID_COUNTER.lock().unwrap_or_else(|e| e.into_inner());
    *counter = counter.wrapping_add(1);
    *counter
}

/// Register a pending factory request and return its ID.
fn register_pending_factory_request(sender: oneshot::Sender<JsonValue>) -> u64 {
    let id = next_factory_request_id();
    let mut pending = PENDING_FACTORY_REQUESTS.lock().unwrap_or_else(|e| e.into_inner());
    pending.insert(id, sender);
    id
}

/// Deliver a factory response to a pending request by its ID.
/// Returns true if the request was found and response delivered.
fn deliver_factory_response_impl(request_id: u64, response: JsonValue) -> bool {
    let sender = {
        let mut pending = PENDING_FACTORY_REQUESTS.lock().unwrap_or_else(|e| e.into_inner());
        pending.remove(&request_id)
    };

    match sender {
        Some(tx) => {
            tx.send(response).is_ok()
        }
        None => {
            warn!("No pending factory request found for ID {}", request_id);
            false
        }
    }
}

/// NIF to deliver a factory response from Elixir back to the waiting Rust code.
///
/// Called by the HandlerRunner GenServer after executing a factory function.
///
/// # Arguments
///
/// * `request_id` - The unique ID of the factory request
/// * `result` - The Elixir term representing the resolved dependency (converted to JSON)
///
/// # Returns
///
/// * `:ok` - If the response was successfully delivered to the pending request
/// * `{:error, :not_implemented}` - If no pending request was found for the given ID
#[rustler::nif]
pub fn deliver_factory_response<'a>(
    env: Env<'a>,
    request_id: u64,
    result: Term<'a>,
) -> NifResult<Term<'a>> {
    debug!("deliver_factory_response called with request_id={}", request_id);

    // Convert Elixir term to JSON
    let result_json = crate::conversion::elixir_to_json(env, result)?;

    if deliver_factory_response_impl(request_id, result_json) {
        debug!("Successfully delivered factory response for request {}", request_id);
        Ok(atoms::ok().encode(env))
    } else {
        warn!("Failed to deliver factory response: no pending request for ID {}", request_id);
        Ok((atoms::error(), atoms::not_implemented()).encode(env))
    }
}

/// Wait for a factory resolution to complete.
///
/// This is an internal function that should be called from handlers that need to
/// resolve a factory dependency asynchronously.
///
/// # Arguments
///
/// * `handler_runner_pid` - The PID of the HandlerRunner GenServer
/// * `factory_name` - The name of the factory function to invoke
/// * `context` - JSON context data to pass to the factory
/// * `timeout_ms` - Timeout in milliseconds for the factory call
///
/// # Returns
///
/// * `Ok(JsonValue)` - The resolved dependency as JSON
/// * `Err(String)` - An error message if the factory resolution failed or timed out
pub async fn resolve_factory(
    handler_runner_pid: LocalPid,
    factory_name: &str,
    context: &JsonValue,
    timeout_ms: u64,
) -> Result<JsonValue, String> {
    debug!("Resolving factory: {}", factory_name);

    // Create response channel
    let (tx, rx) = oneshot::channel();
    let request_id = register_pending_factory_request(tx);

    debug!(
        "Registered pending factory request {} for factory '{}'",
        request_id, factory_name
    );

    // Send request to HandlerRunner via OwnedEnv
    let factory_name = factory_name.to_string();
    let context = context.clone();
    let send_result = send_factory_request(handler_runner_pid, request_id, &factory_name, &context);

    if let Err(e) = send_result {
        warn!("Failed to send factory request to HandlerRunner: {}", e);
        return Err(format!("Failed to send factory request: {}", e));
    }

    // Wait for response with timeout
    let timeout = std::time::Duration::from_millis(timeout_ms);
    debug!(
        "Waiting for factory response with timeout {:?} for request {}",
        timeout, request_id
    );

    match tokio::time::timeout(timeout, rx).await {
        Ok(Ok(json)) => {
            debug!("Received factory response for request {}", request_id);
            Ok(json)
        }
        Ok(Err(_)) => {
            warn!("Factory response channel closed unexpectedly for request {}", request_id);
            Err("Factory response channel closed unexpectedly".to_string())
        }
        Err(_) => {
            warn!("Factory request timed out for request {}", request_id);
            Err("Factory request timed out".to_string())
        }
    }
}

/// Send a factory request to the Elixir HandlerRunner via OwnedEnv.
///
/// This function creates an OwnedEnv, builds a message tuple, and sends it
/// to the HandlerRunner GenServer. The message format is:
/// `{:factory_resolve, request_id, factory_name, context}`
fn send_factory_request(
    handler_runner_pid: LocalPid,
    request_id: u64,
    factory_name: &str,
    context: &JsonValue,
) -> Result<(), String> {
    let context = context.clone();

    let mut owned_env = OwnedEnv::new();

    let send_result = owned_env.send_and_clear(&handler_runner_pid, |env| {
        // Build the factory_resolve atom
        let factory_resolve_atom = match rustler::Atom::from_str(env, "factory_resolve") {
            Ok(a) => a,
            Err(_) => {
                warn!("Failed to create factory_resolve atom");
                return atoms::error().encode(env);
            }
        };

        let request_id_term = request_id.encode(env);
        let factory_name_term = factory_name.encode(env);

        // Convert context to Elixir term
        let context_term = match crate::conversion::json_to_elixir(env, &context) {
            Ok(t) => t,
            Err(e) => {
                warn!("Failed to convert factory context to Elixir: {:?}", e);
                return atoms::error().encode(env);
            }
        };

        // Build message tuple: {:factory_resolve, request_id, factory_name, context}
        (factory_resolve_atom, request_id_term, factory_name_term, context_term).encode(env)
    });

    match send_result {
        Ok(()) => {
            debug!("Successfully sent factory request {} to HandlerRunner", request_id);
            Ok(())
        }
        Err(e) => {
            warn!("Failed to send factory request: {:?}", e);
            Err(format!("Send error: {:?}", e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_request_id_increment() {
        let id1 = next_factory_request_id();
        let id2 = next_factory_request_id();
        assert!(id2 > id1);
    }

    #[test]
    fn test_register_pending_factory_request() {
        let (tx, _rx) = oneshot::channel();
        let id = register_pending_factory_request(tx);
        assert!(id > 0);
    }

    #[test]
    fn test_deliver_factory_response_not_found() {
        // Try to deliver to a non-existent request
        let result = deliver_factory_response_impl(99999, JsonValue::Null);
        assert!(!result, "Should fail to deliver to non-existent request");
    }

    #[test]
    fn test_deliver_factory_response_success() {
        let (tx, mut rx) = oneshot::channel();
        let id = register_pending_factory_request(tx);

        let response = serde_json::json!({"key": "value"});
        let delivered = deliver_factory_response_impl(id, response.clone());

        assert!(delivered, "Should successfully deliver response");
        assert_eq!(rx.try_recv().ok(), Some(response));
    }
}

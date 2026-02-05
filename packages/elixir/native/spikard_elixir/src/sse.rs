//! Elixir SSE (Server-Sent Events) support.
//!
//! This module provides Server-Sent Events (SSE) support for Elixir producers.
//! SSE producers are modules that generate events to stream to clients using the
//! text/event-stream protocol.

#![allow(dead_code)]
#![deny(clippy::unwrap_used)]

use axum::body::Body;
use axum::http::{HeaderName, HeaderValue, Response, StatusCode};
use once_cell::sync::Lazy;
use rustler::{Encoder, Env, LocalPid, NifResult, OwnedEnv, Term};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::oneshot;
use tracing::{debug, warn};

use crate::atoms;
use spikard_http::{Handler, HandlerResult, RequestData};

/// Global map of pending SSE next_event requests keyed by unique reference ID.
/// Used to deliver the next event from Elixir producer back to waiting Rust code.
static PENDING_SSE_REQUESTS: Lazy<Mutex<HashMap<u64, oneshot::Sender<SseEventResult>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Counter for generating unique SSE request IDs.
static SSE_REQUEST_ID_COUNTER: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));

/// Result from an SSE producer's next_event callback.
#[derive(Debug, Clone)]
pub struct SseEventResult {
    pub data: Option<JsonValue>,
    pub event: Option<String>,
    pub id: Option<String>,
    pub done: bool,
}

/// Generate a unique SSE request ID.
fn next_sse_request_id() -> u64 {
    let mut counter = SSE_REQUEST_ID_COUNTER.lock().unwrap_or_else(|e| e.into_inner());
    *counter = counter.wrapping_add(1);
    *counter
}

/// Register a pending SSE request and return its ID.
fn register_pending_sse_request(sender: oneshot::Sender<SseEventResult>) -> u64 {
    let id = next_sse_request_id();
    let mut pending = PENDING_SSE_REQUESTS.lock().unwrap_or_else(|e| e.into_inner());
    pending.insert(id, sender);
    id
}

/// Deliver an SSE event result to a pending request by its ID.
pub fn deliver_sse_event(request_id: u64, event_result: SseEventResult) -> bool {
    let sender = {
        let mut pending = PENDING_SSE_REQUESTS.lock().unwrap_or_else(|e| e.into_inner());
        pending.remove(&request_id)
    };

    match sender {
        Some(tx) => tx.send(event_result).is_ok(),
        None => {
            warn!("No pending SSE request found for ID {}", request_id);
            false
        }
    }
}

/// NIF to deliver an SSE event result from Elixir producer back to the waiting Rust code.
///
/// Called by the ProducerServer after getting the next event from the producer.
#[rustler::nif]
pub fn deliver_sse_event_result<'a>(
    env: Env<'a>,
    request_id: u64,
    event_term: Term<'a>,
) -> NifResult<Term<'a>> {
    let event_result = parse_sse_event_term(env, event_term)?;

    if deliver_sse_event(request_id, event_result) {
        Ok(atoms::ok().encode(env))
    } else {
        Ok((atoms::error(), atoms::not_implemented()).encode(env))
    }
}

/// Parse an Elixir term representing an SSE event into SseEventResult.
fn parse_sse_event_term(_env: Env, term: Term) -> NifResult<SseEventResult> {
    // The term should be either:
    // - :done (atom)
    // - %{data: ..., event: ..., id: ...} (map)

    if term.is_atom() {
        // Check if it's the :done atom
        if let Ok(atom_str) = term.atom_to_string() {
            if atom_str == "done" {
                return Ok(SseEventResult {
                    data: None,
                    event: None,
                    id: None,
                    done: true,
                });
            }
        }
    }

    // Otherwise try to decode as a map with event data
    if let Ok(map) = term.decode::<HashMap<String, Term>>() {
        let data = map
            .get("data")
            .and_then(|t| {
                // Try to convert term to JSON
                if let Ok(json_str) = t.decode::<String>() {
                    serde_json::from_str(&json_str).ok()
                } else {
                    None
                }
            });

        let event = map
            .get("event")
            .and_then(|t| t.decode::<String>().ok());

        let id = map.get("id").and_then(|t| t.decode::<String>().ok());

        return Ok(SseEventResult {
            data,
            event,
            id,
            done: false,
        });
    }

    Err(rustler::Error::Atom("invalid_sse_event_format"))
}

/// Inner state of an Elixir SSE producer handler.
pub struct ElixirSseProducerInner {
    /// PID of the ProducerServer GenServer that manages event generation
    pub producer_server_pid: LocalPid,
    /// Name of the producer module
    pub producer_name: String,
    /// HTTP path for this SSE endpoint
    pub path: String,
    /// Timeout for producer calls in milliseconds
    pub timeout_ms: u64,
}

/// Wrapper around an Elixir SSE producer that implements the Handler trait.
///
/// When the handler is called, it streams events from the Elixir producer.
#[derive(Clone)]
pub struct ElixirSseProducer {
    pub inner: Arc<ElixirSseProducerInner>,
}

/// Default timeout for SSE producer calls (30 seconds).
const DEFAULT_SSE_TIMEOUT_MS: u64 = 30_000;

impl ElixirSseProducer {
    /// Create a new ElixirSseProducer.
    pub fn new(producer_server_pid: LocalPid, producer_name: String, path: String) -> Self {
        Self {
            inner: Arc::new(ElixirSseProducerInner {
                producer_server_pid,
                producer_name,
                path,
                timeout_ms: DEFAULT_SSE_TIMEOUT_MS,
            }),
        }
    }

    /// Create a new producer with custom timeout.
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        if let Some(inner) = Arc::get_mut(&mut self.inner) {
            inner.timeout_ms = timeout_ms;
        }
        self
    }

    /// Handle an SSE request by streaming events from the producer.
    pub async fn handle(&self, _request_data: RequestData) -> HandlerResult {
        debug!("[SSE] handle() called for path {}", self.inner.path);

        // For now, return a simple SSE response with the proper headers
        // The actual event streaming is handled at a higher level in the framework
        let response = Response::builder()
            .status(StatusCode::OK)
            .header(
                HeaderName::from_static("content-type"),
                HeaderValue::from_static("text/event-stream"),
            )
            .header(
                HeaderName::from_static("cache-control"),
                HeaderValue::from_static("no-cache"),
            )
            .header(
                HeaderName::from_static("connection"),
                HeaderValue::from_static("keep-alive"),
            )
            .header(
                HeaderName::from_static("access-control-allow-origin"),
                HeaderValue::from_static("*"),
            )
            .body(Body::empty())
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to build response: {}", e)))?;

        Ok(response)
    }

    /// Get the next event from the producer.
    ///
    /// Sends a message to the ProducerServer and waits for the next event.
    pub async fn next_event(&self) -> Result<Option<SseEventResult>, String> {
        debug!("[SSE] next_event() called");

        // Create response channel
        let (tx, rx) = oneshot::channel();
        let request_id = register_pending_sse_request(tx);

        // Send next_event request to ProducerServer via OwnedEnv
        let producer_server_pid = self.inner.producer_server_pid;

        let send_result = send_sse_next_event_request(producer_server_pid, request_id);

        if let Err(e) = send_result {
            warn!("Failed to send SSE next_event request: {}", e);
            return Err(e);
        }

        // Wait for response with timeout
        let timeout = Duration::from_millis(self.inner.timeout_ms);
        debug!("[SSE] Waiting for next_event response with timeout {:?}", timeout);

        match tokio::time::timeout(timeout, rx).await {
            Ok(Ok(result)) => {
                debug!("[SSE] Received event result: done={}", result.done);
                Ok(Some(result))
            }
            Ok(Err(_)) => {
                warn!("SSE event result channel closed unexpectedly");
                Err("Event result channel closed".to_string())
            }
            Err(_) => {
                warn!("SSE next_event request timed out");
                Err("Producer request timed out".to_string())
            }
        }
    }
}

impl Handler for ElixirSseProducer {
    fn call(
        &self,
        _req: axum::http::Request<Body>,
        request_data: RequestData,
    ) -> Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send + '_>> {
        debug!("[SSE] call() invoked");
        let handler = self.clone();
        Box::pin(async move { handler.handle(request_data).await })
    }
}

/// Send an SSE next_event request to the Elixir ProducerServer via OwnedEnv.
fn send_sse_next_event_request(
    producer_server_pid: LocalPid,
    request_id: u64,
) -> Result<(), String> {
    let mut owned_env = OwnedEnv::new();

    let send_result = owned_env.send_and_clear(&producer_server_pid, |env| {
        // Build the request message
        // Format: {:sse_next, request_id}
        let request_atom = match rustler::Atom::from_str(env, "sse_next") {
            Ok(a) => a,
            Err(_) => {
                warn!("Failed to create sse_next atom");
                return atoms::error().encode(env);
            }
        };

        let request_id_term = request_id.encode(env);
        debug!("Sending sse_next message to producer server for request {}", request_id);

        // Build and return the message tuple
        (request_atom, request_id_term).encode(env)
    });

    match send_result {
        Ok(()) => {
            debug!("Successfully sent sse_next message for request {}", request_id);
            Ok(())
        }
        Err(e) => {
            warn!("Failed to send to producer server: {:?}", e);
            Err(format!("Send error: {:?}", e))
        }
    }
}

/// Get the next event from the producer.
/// This is a placeholder for testing; actual implementation would integrate with ProducerServer.
pub async fn _next_event_placeholder() -> Option<SseEventResult> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sse_event_result_done() {
        let result = SseEventResult {
            data: None,
            event: None,
            id: None,
            done: true,
        };
        assert!(result.done);
    }

    #[test]
    fn test_sse_event_result_with_data() {
        let result = SseEventResult {
            data: Some(json!({"hello": "world"})),
            event: Some("message".to_string()),
            id: Some("123".to_string()),
            done: false,
        };
        assert!(!result.done);
        assert!(result.event.is_some());
        assert!(result.id.is_some());
    }
}

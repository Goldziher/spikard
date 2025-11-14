//! Node.js SSE producer bindings

use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunction;
use serde_json::Value;
use spikard_http::{SseEvent, SseEventProducer};
use std::sync::Arc;
use tracing::{debug, error};

/// Type alias for Node.js Promise-returning ThreadsafeFunction
#[allow(dead_code)]
type NodeTsfn = ThreadsafeFunction<String, Promise<String>, Vec<String>, napi::Status, false>;

/// Node.js implementation of SseEventProducer
#[allow(dead_code)]
pub struct NodeSseEventProducer {
    /// Producer name for debugging
    name: String,
    /// ThreadsafeFunction to call JavaScript next_event method
    next_event_tsfn: Arc<NodeTsfn>,
    /// ThreadsafeFunction to call JavaScript on_connect method
    on_connect_tsfn: Option<Arc<NodeTsfn>>,
    /// ThreadsafeFunction to call JavaScript on_disconnect method
    on_disconnect_tsfn: Option<Arc<NodeTsfn>>,
}

impl NodeSseEventProducer {
    /// Create a new Node.js SSE event producer
    #[allow(dead_code)]
    pub fn new(
        name: String,
        next_event_tsfn: NodeTsfn,
        on_connect_tsfn: Option<NodeTsfn>,
        on_disconnect_tsfn: Option<NodeTsfn>,
    ) -> Self {
        Self {
            name,
            next_event_tsfn: Arc::new(next_event_tsfn),
            on_connect_tsfn: on_connect_tsfn.map(Arc::new),
            on_disconnect_tsfn: on_disconnect_tsfn.map(Arc::new),
        }
    }
}

impl SseEventProducer for NodeSseEventProducer {
    async fn next_event(&self) -> Option<SseEvent> {
        debug!("Node.js SSE producer '{}': next_event", self.name);

        // Call JavaScript function via ThreadsafeFunction
        let func = Arc::clone(&self.next_event_tsfn);
        let json_output = match func.call_async("{}".to_string()).await {
            Ok(promise) => match promise.await {
                Ok(result) => result,
                Err(e) => {
                    error!("JavaScript promise failed in next_event: {}", e);
                    return None;
                }
            },
            Err(e) => {
                error!("Failed to call JavaScript next_event: {}", e);
                return None;
            }
        };

        // Check for null response (end of stream)
        if json_output == "null" || json_output.is_empty() {
            debug!("Node.js SSE producer: received null, ending stream");
            return None;
        }

        // Parse the JSON response from JavaScript
        // Expected format: { data: any, event_type?: string, id?: string, retry?: number }
        match serde_json::from_str::<Value>(&json_output) {
            Ok(value) => {
                // Extract event fields
                let data = value.get("data").cloned().unwrap_or(Value::Null);

                let event_type = value
                    .get("event_type")
                    .or_else(|| value.get("eventType"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                let id = value.get("id").and_then(|v| v.as_str()).map(|s| s.to_string());

                let retry = value.get("retry").and_then(|v| v.as_u64());

                // Create Rust SseEvent
                let mut event = if let Some(et) = event_type {
                    SseEvent::with_type(et, data)
                } else {
                    SseEvent::new(data)
                };

                if let Some(id_str) = id {
                    event = event.with_id(id_str);
                }

                if let Some(retry_ms) = retry {
                    event = event.with_retry(retry_ms);
                }

                Some(event)
            }
            Err(e) => {
                error!("Failed to parse JavaScript SSE event: {}", e);
                None
            }
        }
    }

    async fn on_connect(&self) {
        debug!("Node.js SSE producer '{}': on_connect", self.name);

        if let Some(func) = &self.on_connect_tsfn {
            let func = Arc::clone(func);
            let _ = func.call_async("{}".to_string()).await;
            debug!("Node.js SSE producer '{}': on_connect completed", self.name);
        }
    }

    async fn on_disconnect(&self) {
        debug!("Node.js SSE producer '{}': on_disconnect", self.name);

        if let Some(func) = &self.on_disconnect_tsfn {
            let func = Arc::clone(func);
            let _ = func.call_async("{}".to_string()).await;
            debug!("Node.js SSE producer '{}': on_disconnect completed", self.name);
        }
    }
}

/// Convert Node.js object to JSON Value
fn node_object_to_json(obj: &Object) -> Result<serde_json::Value> {
    let json_str: String = obj
        .get_named_property("toJSON")
        .and_then(|func: Function<(), String>| func.call(()))
        .or_else(|_| {
            // Fallback: use JSON.stringify
            let env_ptr = obj.env();
            let env = napi::Env::from_raw(env_ptr);
            let global = env.get_global()?;
            let json: Object = global.get_named_property("JSON")?;
            let stringify: Function<Object, String> = json.get_named_property("stringify")?;
            stringify.call(*obj)
        })?;

    serde_json::from_str(&json_str).map_err(|e| napi::Error::from_reason(format!("Failed to parse JSON: {}", e)))
}

/// Create SseState from Node.js producer factory
///
/// This function is designed to be called from JavaScript to register SSE producers.
#[allow(dead_code)]
pub fn create_sse_state(producer_instance: &Object) -> Result<spikard_http::SseState<NodeSseEventProducer>> {
    // Extract the nextEvent function
    let next_event_fn: Function<String, Promise<String>> = producer_instance.get_named_property("nextEvent")?;

    // Build ThreadsafeFunction for next_event
    let next_event_tsfn = next_event_fn
        .build_threadsafe_function()
        .build_callback(|ctx| Ok(vec![ctx.value]))?;

    // Extract optional onConnect function
    let on_connect_tsfn = producer_instance
        .get_named_property::<Function<String, Promise<String>>>("onConnect")
        .ok()
        .and_then(|func| {
            func.build_threadsafe_function()
                .build_callback(|ctx| Ok(vec![ctx.value]))
                .ok()
        });

    // Extract optional onDisconnect function
    let on_disconnect_tsfn = producer_instance
        .get_named_property::<Function<String, Promise<String>>>("onDisconnect")
        .ok()
        .and_then(|func| {
            func.build_threadsafe_function()
                .build_callback(|ctx| Ok(vec![ctx.value]))
                .ok()
        });

    // Extract event schema if available
    let event_schema = producer_instance
        .get_named_property::<Object>("_eventSchema")
        .ok()
        .and_then(|obj| node_object_to_json(&obj).ok());

    // Create Node SSE producer
    let node_producer = NodeSseEventProducer::new(
        "SseEventProducer".to_string(),
        next_event_tsfn,
        on_connect_tsfn,
        on_disconnect_tsfn,
    );

    // Create and return SSE state with schema
    if event_schema.is_some() {
        spikard_http::SseState::with_schema(node_producer, event_schema).map_err(napi::Error::from_reason)
    } else {
        Ok(spikard_http::SseState::new(node_producer))
    }
}

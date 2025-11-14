//! Server-Sent Events (SSE) support for Spikard
//!
//! Provides SSE streaming with event generation and lifecycle management.

use axum::{
    extract::State,
    response::{
        IntoResponse,
        sse::{Event, KeepAlive, Sse},
    },
};
use futures_util::stream;
use serde_json::Value;
use std::{convert::Infallible, sync::Arc, time::Duration};
use tracing::{debug, error, info};

/// SSE event producer trait
///
/// Implement this trait to generate Server-Sent Events for your application.
pub trait SseEventProducer: Send + Sync {
    /// Generate the next event
    ///
    /// Returns `Some(event)` when an event is ready, or `None` when the stream should end.
    /// This method is called repeatedly to produce the event stream.
    fn next_event(&self) -> impl std::future::Future<Output = Option<SseEvent>> + Send;

    /// Called when a client connects to the SSE endpoint
    fn on_connect(&self) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }

    /// Called when a client disconnects from the SSE endpoint
    fn on_disconnect(&self) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }
}

/// An individual SSE event
#[derive(Debug, Clone)]
pub struct SseEvent {
    /// Event type (optional)
    pub event_type: Option<String>,
    /// Event data (JSON value)
    pub data: Value,
    /// Event ID (optional, for client-side reconnection)
    pub id: Option<String>,
    /// Retry timeout in milliseconds (optional)
    pub retry: Option<u64>,
}

impl SseEvent {
    /// Create a new SSE event with data only
    pub fn new(data: Value) -> Self {
        Self {
            event_type: None,
            data,
            id: None,
            retry: None,
        }
    }

    /// Create a new SSE event with an event type and data
    pub fn with_type(event_type: impl Into<String>, data: Value) -> Self {
        Self {
            event_type: Some(event_type.into()),
            data,
            id: None,
            retry: None,
        }
    }

    /// Set the event ID for client-side reconnection support
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the retry timeout for client reconnection
    pub fn with_retry(mut self, retry_ms: u64) -> Self {
        self.retry = Some(retry_ms);
        self
    }

    /// Convert to Axum's SSE Event
    fn into_axum_event(self) -> Result<Event, serde_json::Error> {
        let json_data = serde_json::to_string(&self.data)?;

        let mut event = Event::default().data(json_data);

        if let Some(event_type) = self.event_type {
            event = event.event(event_type);
        }

        if let Some(id) = self.id {
            event = event.id(id);
        }

        if let Some(retry) = self.retry {
            event = event.retry(Duration::from_millis(retry));
        }

        Ok(event)
    }
}

/// SSE state shared across connections
pub struct SseState<P: SseEventProducer> {
    producer: Arc<P>,
    /// Optional JSON Schema for validating outgoing events
    event_schema: Option<Arc<jsonschema::Validator>>,
}

impl<P: SseEventProducer> Clone for SseState<P> {
    fn clone(&self) -> Self {
        Self {
            producer: Arc::clone(&self.producer),
            event_schema: self.event_schema.clone(),
        }
    }
}

impl<P: SseEventProducer + 'static> SseState<P> {
    /// Create new SSE state with an event producer
    pub fn new(producer: P) -> Self {
        Self {
            producer: Arc::new(producer),
            event_schema: None,
        }
    }

    /// Create new SSE state with an event producer and event schema
    pub fn with_schema(producer: P, event_schema: Option<serde_json::Value>) -> Result<Self, String> {
        let event_validator = if let Some(schema) = event_schema {
            Some(Arc::new(
                jsonschema::validator_for(&schema).map_err(|e| format!("Invalid event schema: {}", e))?,
            ))
        } else {
            None
        };

        Ok(Self {
            producer: Arc::new(producer),
            event_schema: event_validator,
        })
    }
}

/// SSE endpoint handler
///
/// This is the main entry point for SSE connections.
/// Use this as an Axum route handler.
pub async fn sse_handler<P: SseEventProducer + 'static>(State(state): State<SseState<P>>) -> impl IntoResponse {
    info!("SSE client connected");

    // Notify producer of connection
    state.producer.on_connect().await;

    // Create event stream
    let producer = Arc::clone(&state.producer);
    let event_schema = state.event_schema.clone();
    let stream = stream::unfold((producer, event_schema), |(producer, event_schema)| async move {
        match producer.next_event().await {
            Some(sse_event) => {
                debug!("Sending SSE event: {:?}", sse_event.event_type);

                // Validate event data if schema is provided
                if let Some(validator) = &event_schema
                    && !validator.is_valid(&sse_event.data)
                {
                    error!("SSE event validation failed");
                    // Skip this event and continue to the next one
                    return Some((
                        Ok::<_, Infallible>(Event::default().data("validation_error")),
                        (producer, event_schema),
                    ));
                }

                match sse_event.into_axum_event() {
                    Ok(event) => Some((Ok::<_, Infallible>(event), (producer, event_schema))),
                    Err(e) => {
                        error!("Failed to serialize SSE event: {}", e);
                        None
                    }
                }
            }
            None => {
                info!("SSE stream ended");
                None
            }
        }
    });

    // Convert to SSE response with keep-alive
    let sse_response =
        Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15)).text("keep-alive"));

    // Return the SSE response
    sse_response.into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestProducer {
        count: std::sync::atomic::AtomicUsize,
    }

    impl SseEventProducer for TestProducer {
        async fn next_event(&self) -> Option<SseEvent> {
            let count = self.count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if count < 3 {
                Some(SseEvent::new(serde_json::json!({
                    "message": format!("Event {}", count)
                })))
            } else {
                None
            }
        }
    }

    #[test]
    fn test_sse_event_creation() {
        let event = SseEvent::new(serde_json::json!({"test": "data"}));
        assert!(event.event_type.is_none());
        assert!(event.id.is_none());
        assert!(event.retry.is_none());

        let event = SseEvent::with_type("notification", serde_json::json!({"test": "data"}))
            .with_id("123")
            .with_retry(5000);
        assert_eq!(event.event_type, Some("notification".to_string()));
        assert_eq!(event.id, Some("123".to_string()));
        assert_eq!(event.retry, Some(5000));
    }

    #[test]
    fn test_sse_state_creation() {
        let producer = TestProducer {
            count: std::sync::atomic::AtomicUsize::new(0),
        };
        let state = SseState::new(producer);
        let cloned = state.clone();
        // Verify state can be cloned
        assert!(Arc::ptr_eq(&state.producer, &cloned.producer));
    }
}

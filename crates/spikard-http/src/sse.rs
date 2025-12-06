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
/// Implement this trait to create custom Server-Sent Event (SSE) producers for your application.
/// The producer generates events that are streamed to connected clients.
///
/// # Understanding SSE
///
/// Server-Sent Events (SSE) provide one-way communication from server to client over HTTP.
/// Unlike WebSocket, SSE uses standard HTTP and automatically handles reconnection.
/// Use SSE when you need to push data to clients without bidirectional communication.
///
/// # Implementing the Trait
///
/// You must implement the `next_event` method to generate events. The `on_connect` and
/// `on_disconnect` methods are optional lifecycle hooks.
///
/// # Example
///
/// ```ignore
/// use spikard_http::sse::{SseEventProducer, SseEvent};
/// use serde_json::json;
/// use std::time::Duration;
/// use tokio::time::sleep;
///
/// struct CounterProducer {
///     limit: usize,
/// }
///
/// #[async_trait]
/// impl SseEventProducer for CounterProducer {
///     async fn next_event(&self) -> Option<SseEvent> {
///         static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
///
///         let count = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
///         if count < self.limit {
///             Some(SseEvent::new(json!({"count": count})))
///         } else {
///             None
///         }
///     }
///
///     async fn on_connect(&self) {
///         println!("Client connected");
///     }
///
///     async fn on_disconnect(&self) {
///         println!("Client disconnected");
///     }
/// }
/// ```
pub trait SseEventProducer: Send + Sync {
    /// Generate the next event
    ///
    /// Called repeatedly to produce the event stream. Should return `Some(event)` when
    /// an event is ready to send, or `None` when the stream should end.
    ///
    /// # Returns
    /// * `Some(event)` - Event to send to the client
    /// * `None` - Stream complete, connection will close
    fn next_event(&self) -> impl std::future::Future<Output = Option<SseEvent>> + Send;

    /// Called when a client connects to the SSE endpoint
    ///
    /// Optional lifecycle hook invoked when a new SSE connection is established.
    /// Default implementation does nothing.
    fn on_connect(&self) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }

    /// Called when a client disconnects from the SSE endpoint
    ///
    /// Optional lifecycle hook invoked when an SSE connection is closed (either by the
    /// client or the stream ending). Default implementation does nothing.
    fn on_disconnect(&self) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }
}

/// An individual SSE event
///
/// Represents a single Server-Sent Event to be sent to a connected client.
/// Events can have an optional type, ID, and retry timeout for advanced scenarios.
///
/// # Fields
///
/// * `event_type` - Optional event type string (used for client-side event filtering)
/// * `data` - JSON data payload to send to the client
/// * `id` - Optional event ID (clients can use this to resume after disconnect)
/// * `retry` - Optional retry timeout in milliseconds (tells client when to reconnect)
///
/// # SSE Format
///
/// Events are serialized to the following text format:
/// ```text
/// event: event_type
/// data: {"json":"value"}
/// id: event-123
/// retry: 3000
/// ```
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
    ///
    /// Creates a minimal event with just the data payload. Use builder methods
    /// to add optional fields.
    ///
    /// # Arguments
    /// * `data` - JSON value to send to the client
    ///
    /// # Example
    ///
    /// ```ignore
    /// use serde_json::json;
    /// use spikard_http::sse::SseEvent;
    ///
    /// let event = SseEvent::new(json!({"status": "connected"}));
    /// ```
    pub fn new(data: Value) -> Self {
        Self {
            event_type: None,
            data,
            id: None,
            retry: None,
        }
    }

    /// Create a new SSE event with an event type and data
    ///
    /// Creates an event with a type field. Clients can filter events by type
    /// in their event listener.
    ///
    /// # Arguments
    /// * `event_type` - String identifying the event type (e.g., "update", "error")
    /// * `data` - JSON value to send to the client
    ///
    /// # Example
    ///
    /// ```ignore
    /// use serde_json::json;
    /// use spikard_http::sse::SseEvent;
    ///
    /// let event = SseEvent::with_type("update", json!({"count": 42}));
    /// // Client can listen with: eventSource.addEventListener("update", ...)
    /// ```
    pub fn with_type(event_type: impl Into<String>, data: Value) -> Self {
        Self {
            event_type: Some(event_type.into()),
            data,
            id: None,
            retry: None,
        }
    }

    /// Set the event ID for client-side reconnection support
    ///
    /// Sets an ID that clients can use to resume from this point if they disconnect.
    /// The client sends this ID back in the `Last-Event-ID` header when reconnecting.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this event
    ///
    /// # Example
    ///
    /// ```ignore
    /// use serde_json::json;
    /// use spikard_http::sse::SseEvent;
    ///
    /// let event = SseEvent::new(json!({"count": 1}))
    ///     .with_id("event-1");
    /// ```
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the retry timeout for client reconnection
    ///
    /// Sets the time in milliseconds clients should wait before attempting to reconnect
    /// if the connection is lost. The client browser will automatically handle reconnection.
    ///
    /// # Arguments
    /// * `retry_ms` - Retry timeout in milliseconds
    ///
    /// # Example
    ///
    /// ```ignore
    /// use serde_json::json;
    /// use spikard_http::sse::SseEvent;
    ///
    /// let event = SseEvent::new(json!({"data": "value"}))
    ///     .with_retry(5000); // Reconnect after 5 seconds
    /// ```
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
///
/// Contains the event producer and optional JSON schema for validating
/// events. This state is shared among all connections to the same SSE endpoint.
pub struct SseState<P: SseEventProducer> {
    /// The event producer implementation
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
    ///
    /// Creates a new state without event validation schema.
    /// Events are not validated.
    ///
    /// # Arguments
    /// * `producer` - The event producer implementation
    ///
    /// # Example
    ///
    /// ```ignore
    /// let state = SseState::new(MyProducer);
    /// ```
    pub fn new(producer: P) -> Self {
        Self {
            producer: Arc::new(producer),
            event_schema: None,
        }
    }

    /// Create new SSE state with an event producer and optional event schema
    ///
    /// Creates a new state with optional JSON schema for validating outgoing events.
    /// If a schema is provided and an event fails validation, it is silently dropped.
    ///
    /// # Arguments
    /// * `producer` - The event producer implementation
    /// * `event_schema` - Optional JSON schema for validating events
    ///
    /// # Returns
    /// * `Ok(state)` - Successfully created state
    /// * `Err(msg)` - Invalid schema provided
    ///
    /// # Example
    ///
    /// ```ignore
    /// use serde_json::json;
    ///
    /// let event_schema = json!({
    ///     "type": "object",
    ///     "properties": {
    ///         "count": {"type": "integer"}
    ///     }
    /// });
    ///
    /// let state = SseState::with_schema(MyProducer, Some(event_schema))?;
    /// ```
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
/// This is the main entry point for SSE connections. Use this as an Axum route
/// handler by passing it to an Axum router's `.route()` method with `get()`.
///
/// The handler establishes a connection and streams events from the producer to
/// the client using the Server-Sent Events protocol (text/event-stream).
///
/// # Arguments
/// * `State(state)` - Application state containing the event producer and optional schema
///
/// # Returns
/// A streaming response with the `text/event-stream` content type
///
/// # Example
///
/// ```ignore
/// use axum::{Router, routing::get, extract::State};
///
/// let state = SseState::new(MyProducer);
/// let router = Router::new()
///     .route("/events", get(sse_handler::<MyProducer>))
///     .with_state(state);
///
/// // Client usage:
/// // const eventSource = new EventSource('/events');
/// // eventSource.onmessage = (e) => console.log(e.data);
/// ```
pub async fn sse_handler<P: SseEventProducer + 'static>(State(state): State<SseState<P>>) -> impl IntoResponse {
    info!("SSE client connected");

    state.producer.on_connect().await;

    let producer = Arc::clone(&state.producer);
    let event_schema = state.event_schema.clone();
    let stream = stream::unfold((producer, event_schema), |(producer, event_schema)| async move {
        match producer.next_event().await {
            Some(sse_event) => {
                debug!("Sending SSE event: {:?}", sse_event.event_type);

                if let Some(validator) = &event_schema
                    && !validator.is_valid(&sse_event.data)
                {
                    error!("SSE event validation failed");
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

    let sse_response =
        Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15)).text("keep-alive"));

    sse_response.into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    // ============================================================================
    // Test Producers
    // ============================================================================

    struct TestProducer {
        count: AtomicUsize,
    }

    impl SseEventProducer for TestProducer {
        async fn next_event(&self) -> Option<SseEvent> {
            let count = self.count.fetch_add(1, Ordering::Relaxed);
            if count < 3 {
                Some(SseEvent::new(serde_json::json!({
                    "message": format!("Event {}", count)
                })))
            } else {
                None
            }
        }
    }

    /// Producer that tracks connect/disconnect lifecycle
    struct LifecycleProducer {
        connect_count: Arc<AtomicUsize>,
        disconnect_count: Arc<AtomicUsize>,
        event_count: AtomicUsize,
    }

    impl LifecycleProducer {
        fn new(connect: Arc<AtomicUsize>, disconnect: Arc<AtomicUsize>) -> Self {
            Self {
                connect_count: connect,
                disconnect_count: disconnect,
                event_count: AtomicUsize::new(0),
            }
        }
    }

    impl SseEventProducer for LifecycleProducer {
        async fn next_event(&self) -> Option<SseEvent> {
            let idx: usize = self.event_count.fetch_add(1, Ordering::Relaxed);
            if idx < 2 {
                Some(SseEvent::new(serde_json::json!({"event": idx})))
            } else {
                None
            }
        }

        async fn on_connect(&self) {
            self.connect_count.fetch_add(1, Ordering::Relaxed);
        }

        async fn on_disconnect(&self) {
            self.disconnect_count.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Producer for multiline event testing
    struct MultilineProducer {
        sent: AtomicBool,
    }

    impl SseEventProducer for MultilineProducer {
        async fn next_event(&self) -> Option<SseEvent> {
            let was_sent: bool = self.sent.swap(true, Ordering::Relaxed);
            if !was_sent {
                Some(SseEvent::new(serde_json::json!({
                    "text": "line1\nline2\nline3"
                })))
            } else {
                None
            }
        }
    }

    /// Producer for special characters testing
    struct SpecialCharsProducer {
        sent: AtomicBool,
    }

    impl SseEventProducer for SpecialCharsProducer {
        async fn next_event(&self) -> Option<SseEvent> {
            let was_sent: bool = self.sent.swap(true, Ordering::Relaxed);
            if !was_sent {
                Some(SseEvent::new(serde_json::json!({
                    "data": "special: \"quotes\", \\ backslash, \t tab, \r\n crlf"
                })))
            } else {
                None
            }
        }
    }

    /// Producer for large payload testing
    struct LargePayloadProducer {
        sent: AtomicBool,
    }

    impl SseEventProducer for LargePayloadProducer {
        async fn next_event(&self) -> Option<SseEvent> {
            let was_sent: bool = self.sent.swap(true, Ordering::Relaxed);
            if !was_sent {
                // Create a 100KB payload
                let large_string: String = "x".repeat(100_000);
                Some(SseEvent::new(serde_json::json!({
                    "payload": large_string
                })))
            } else {
                None
            }
        }
    }

    /// Producer that sends many events rapidly
    struct RapidEventProducer {
        event_count: usize,
        current: AtomicUsize,
    }

    impl RapidEventProducer {
        fn new(count: usize) -> Self {
            Self {
                event_count: count,
                current: AtomicUsize::new(0),
            }
        }
    }

    impl SseEventProducer for RapidEventProducer {
        async fn next_event(&self) -> Option<SseEvent> {
            let idx: usize = self.current.fetch_add(1, Ordering::Relaxed);
            if idx < self.event_count {
                Some(SseEvent::new(serde_json::json!({
                    "id": idx,
                    "data": format!("event_{}", idx)
                })))
            } else {
                None
            }
        }
    }

    /// Producer with all event fields populated
    struct FullFieldProducer {
        sent: AtomicBool,
    }

    impl SseEventProducer for FullFieldProducer {
        async fn next_event(&self) -> Option<SseEvent> {
            let was_sent: bool = self.sent.swap(true, Ordering::Relaxed);
            if !was_sent {
                Some(
                    SseEvent::with_type(
                        "counter_update",
                        serde_json::json!({
                            "count": 42,
                            "status": "active"
                        }),
                    )
                    .with_id("event-123")
                    .with_retry(5000),
                )
            } else {
                None
            }
        }
    }

    /// Producer that ends immediately (keep-alive test)
    struct NoEventProducer;

    impl SseEventProducer for NoEventProducer {
        async fn next_event(&self) -> Option<SseEvent> {
            None
        }
    }

    // ============================================================================
    // Tests: Event Creation & Formatting
    // ============================================================================

    #[test]
    fn test_sse_event_creation_minimal() {
        let event: SseEvent = SseEvent::new(serde_json::json!({"test": "data"}));
        assert!(event.event_type.is_none());
        assert!(event.id.is_none());
        assert!(event.retry.is_none());
    }

    #[test]
    fn test_sse_event_with_all_fields() {
        let event: SseEvent = SseEvent::with_type("update", serde_json::json!({"count": 42}))
            .with_id("event-001")
            .with_retry(3000);

        assert_eq!(event.event_type, Some("update".to_string()));
        assert_eq!(event.id, Some("event-001".to_string()));
        assert_eq!(event.retry, Some(3000));
    }

    #[test]
    fn test_sse_event_builder_pattern() {
        let event: SseEvent = SseEvent::with_type("notification", serde_json::json!({"text": "hello"}))
            .with_id("notif-456")
            .with_retry(5000);

        assert_eq!(event.event_type, Some("notification".to_string()));
        assert_eq!(event.id, Some("notif-456".to_string()));
        assert_eq!(event.retry, Some(5000));
    }

    #[test]
    fn test_sse_event_multiline_data() {
        let event: SseEvent = SseEvent::new(serde_json::json!({
            "text": "line1\nline2\nline3"
        }));

        assert!(event.data.is_object());
        let text: Option<&str> = event.data.get("text").and_then(|v| v.as_str());
        assert_eq!(text, Some("line1\nline2\nline3"));
    }

    #[test]
    fn test_sse_event_special_characters() {
        let event: SseEvent = SseEvent::new(serde_json::json!({
            "data": "special: \"quotes\", \\ backslash"
        }));

        assert!(event.data.is_object());
    }

    #[test]
    fn test_sse_event_large_payload() {
        let large_string: String = "x".repeat(100_000);
        let event: SseEvent = SseEvent::new(serde_json::json!({
            "payload": large_string.clone()
        }));

        let payload_field: Option<&str> = event.data.get("payload").and_then(|v| v.as_str());
        assert_eq!(payload_field.map(|s| s.len()), Some(100_000));
    }

    #[test]
    fn test_sse_event_into_axum_event_conversion() {
        let event: SseEvent = SseEvent::new(serde_json::json!({"msg": "test"}));
        let axum_event: Result<axum::response::sse::Event, serde_json::Error> = event.into_axum_event();
        assert!(axum_event.is_ok());
    }

    #[test]
    fn test_sse_event_into_axum_with_all_fields() {
        let event: SseEvent = SseEvent::with_type("event", serde_json::json!({"id": 1}))
            .with_id("123")
            .with_retry(5000);

        let axum_event: Result<axum::response::sse::Event, serde_json::Error> = event.into_axum_event();
        assert!(axum_event.is_ok());
    }

    // ============================================================================
    // Tests: Stream Lifecycle
    // ============================================================================

    #[test]
    fn test_sse_state_creation() {
        let producer: TestProducer = TestProducer {
            count: AtomicUsize::new(0),
        };
        let state: SseState<TestProducer> = SseState::new(producer);
        let cloned: SseState<TestProducer> = state.clone();
        assert!(Arc::ptr_eq(&state.producer, &cloned.producer));
    }

    #[test]
    fn test_sse_state_with_schema_valid() {
        let producer: TestProducer = TestProducer {
            count: AtomicUsize::new(0),
        };
        let schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {
                "message": {"type": "string"}
            }
        });

        let result: Result<SseState<TestProducer>, String> = SseState::with_schema(producer, Some(schema));
        assert!(result.is_ok());
    }

    #[test]
    fn test_sse_state_with_invalid_schema() {
        let producer: TestProducer = TestProducer {
            count: AtomicUsize::new(0),
        };
        let invalid_schema: serde_json::Value = serde_json::json!({
            "type": "not-a-valid-type"
        });

        let result: Result<SseState<TestProducer>, String> = SseState::with_schema(producer, Some(invalid_schema));
        assert!(result.is_err());
    }

    #[test]
    fn test_sse_state_with_schema_none() {
        let producer: TestProducer = TestProducer {
            count: AtomicUsize::new(0),
        };
        let result: Result<SseState<TestProducer>, String> = SseState::with_schema(producer, None);
        assert!(result.is_ok());
    }

    // ============================================================================
    // Tests: Connection Lifecycle Hooks
    // ============================================================================

    #[tokio::test]
    async fn test_sse_lifecycle_on_connect_called() {
        let connect_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
        let disconnect_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));

        let producer: LifecycleProducer =
            LifecycleProducer::new(Arc::clone(&connect_count), Arc::clone(&disconnect_count));

        producer.on_connect().await;
        assert_eq!(connect_count.load(Ordering::Relaxed), 1);
    }

    #[tokio::test]
    async fn test_sse_lifecycle_on_disconnect_called() {
        let connect_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
        let disconnect_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));

        let producer: LifecycleProducer =
            LifecycleProducer::new(Arc::clone(&connect_count), Arc::clone(&disconnect_count));

        producer.on_disconnect().await;
        assert_eq!(disconnect_count.load(Ordering::Relaxed), 1);
    }

    // ============================================================================
    // Tests: Event Ordering & Delivery
    // ============================================================================

    #[tokio::test]
    async fn test_sse_event_ordering_preserved() {
        let producer: RapidEventProducer = RapidEventProducer::new(10);

        let mut last_idx: i32 = -1;
        for _ in 0..10 {
            if let Some(event) = producer.next_event().await {
                if let Some(id) = event.data.get("id").and_then(|v| v.as_i64()) {
                    assert!(id as i32 > last_idx, "Event ordering violated");
                    last_idx = id as i32;
                }
            }
        }
    }

    #[tokio::test]
    async fn test_sse_rapid_event_sending() {
        let producer: RapidEventProducer = RapidEventProducer::new(100);

        let mut count: usize = 0;
        loop {
            match producer.next_event().await {
                Some(_event) => count += 1,
                None => break,
            }
        }

        assert_eq!(count, 100);
    }

    // ============================================================================
    // Tests: Edge Cases & Error Handling
    // ============================================================================

    #[test]
    fn test_sse_event_with_empty_data_object() {
        let event: SseEvent = SseEvent::new(serde_json::json!({}));
        assert!(event.data.is_object());
    }

    #[test]
    fn test_sse_event_with_nested_data() {
        let event: SseEvent = SseEvent::new(serde_json::json!({
            "nested": {
                "deep": {
                    "value": "found"
                }
            }
        }));

        let deep_value: Option<&str> = event
            .data
            .get("nested")
            .and_then(|v| v.get("deep"))
            .and_then(|v| v.get("value"))
            .and_then(|v| v.as_str());

        assert_eq!(deep_value, Some("found"));
    }

    #[tokio::test]
    async fn test_sse_producer_stream_ends_cleanly() {
        let producer: NoEventProducer = NoEventProducer;

        let event1: Option<SseEvent> = producer.next_event().await;
        assert!(event1.is_none());

        let event2: Option<SseEvent> = producer.next_event().await;
        assert!(event2.is_none());
    }

    #[test]
    fn test_sse_event_clone() {
        let original: SseEvent = SseEvent::with_type("test", serde_json::json!({"data": "test"}))
            .with_id("id-1")
            .with_retry(2000);

        let cloned: SseEvent = original.clone();

        assert_eq!(cloned.event_type, original.event_type);
        assert_eq!(cloned.id, original.id);
        assert_eq!(cloned.retry, original.retry);
        assert_eq!(cloned.data, original.data);
    }

    #[test]
    fn test_sse_event_debug_impl() {
        let event: SseEvent = SseEvent::new(serde_json::json!({"msg": "debug"}));
        let debug_str: String = format!("{:?}", event);
        assert!(debug_str.contains("SseEvent"));
    }

    // ============================================================================
    // Tests: Concurrent Stream Handling
    // ============================================================================

    #[tokio::test]
    async fn test_sse_multiple_producers_independent() {
        let producer1: TestProducer = TestProducer {
            count: AtomicUsize::new(0),
        };
        let producer2: TestProducer = TestProducer {
            count: AtomicUsize::new(0),
        };

        let _event1: Option<SseEvent> = producer1.next_event().await;
        let _event2: Option<SseEvent> = producer2.next_event().await;

        let count1: usize = producer1.count.load(Ordering::Relaxed);
        let count2: usize = producer2.count.load(Ordering::Relaxed);

        assert_eq!(count1, 1);
        assert_eq!(count2, 1);
    }

    // ============================================================================
    // Tests: Validation & Schema
    // ============================================================================

    #[test]
    fn test_sse_state_cloning_preserves_schema() {
        let producer: TestProducer = TestProducer {
            count: AtomicUsize::new(0),
        };
        let schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {
                "message": {"type": "string"}
            }
        });

        let state: SseState<TestProducer> =
            SseState::with_schema(producer, Some(schema)).expect("schema should be valid");
        let cloned: SseState<TestProducer> = state.clone();

        assert!(Arc::ptr_eq(&state.producer, &cloned.producer));
        match (&state.event_schema, &cloned.event_schema) {
            (Some(s1), Some(s2)) => {
                assert!(Arc::ptr_eq(s1, s2));
            }
            _ => panic!("Schema should be preserved in clone"),
        }
    }

    // ============================================================================
    // Tests: Payload & Data Integrity
    // ============================================================================

    #[tokio::test]
    async fn test_sse_large_payload_integrity() {
        let producer: LargePayloadProducer = LargePayloadProducer {
            sent: AtomicBool::new(false),
        };

        let event: Option<SseEvent> = producer.next_event().await;
        assert!(event.is_some());

        if let Some(evt) = event {
            let payload: Option<&str> = evt.data.get("payload").and_then(|v| v.as_str());
            assert_eq!(payload.map(|s| s.len()), Some(100_000));
        }
    }

    #[tokio::test]
    async fn test_sse_multiline_data_preservation() {
        let producer: MultilineProducer = MultilineProducer {
            sent: AtomicBool::new(false),
        };

        let event: Option<SseEvent> = producer.next_event().await;
        assert!(event.is_some());

        if let Some(evt) = event {
            let text: Option<&str> = evt.data.get("text").and_then(|v| v.as_str());
            assert_eq!(text, Some("line1\nline2\nline3"));
        }
    }

    #[tokio::test]
    async fn test_sse_special_chars_in_payload() {
        let producer: SpecialCharsProducer = SpecialCharsProducer {
            sent: AtomicBool::new(false),
        };

        let event: Option<SseEvent> = producer.next_event().await;
        assert!(event.is_some());

        if let Some(evt) = event {
            let data: Option<&str> = evt.data.get("data").and_then(|v| v.as_str());
            assert!(data.is_some());
            assert!(data.unwrap().contains("quotes"));
        }
    }

    #[tokio::test]
    async fn test_sse_full_event_fields_together() {
        let producer: FullFieldProducer = FullFieldProducer {
            sent: AtomicBool::new(false),
        };

        let event: Option<SseEvent> = producer.next_event().await;
        assert!(event.is_some());

        if let Some(evt) = event {
            assert_eq!(evt.event_type, Some("counter_update".to_string()));
            assert_eq!(evt.id, Some("event-123".to_string()));
            assert_eq!(evt.retry, Some(5000));
            assert_eq!(evt.data.get("count").and_then(|v| v.as_i64()), Some(42));
        }
    }
}

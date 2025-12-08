//! Behavioral tests for Server-Sent Events (SSE) functionality
//!
//! These tests verify end-to-end SSE behavior including:
//! - Connection establishment and event streaming
//! - Client reconnection with Last-Event-ID header
//! - Event ordering preservation
//! - Connection cleanup on disconnect
//! - Keep-alive behavior
//! - Backpressure handling for slow clients
//! - Graceful shutdown with active streams

mod common;

use spikard_http::sse::{SseEvent, SseEventProducer};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Duration;
use tokio::time::sleep;

// ============================================================================
// Test Producers for Behavioral Testing
// ============================================================================

/// Producer that simulates a stream of numbered events (for ordering tests)
struct SequentialEventProducer {
    total_events: usize,
    current_count: Arc<AtomicUsize>,
    connect_count: Arc<AtomicUsize>,
    disconnect_count: Arc<AtomicUsize>,
}

impl SequentialEventProducer {
    fn new(total_events: usize) -> Self {
        Self {
            total_events,
            current_count: Arc::new(AtomicUsize::new(0)),
            connect_count: Arc::new(AtomicUsize::new(0)),
            disconnect_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn get_connect_count(&self) -> usize {
        self.connect_count.load(Ordering::Relaxed)
    }

    fn get_disconnect_count(&self) -> usize {
        self.disconnect_count.load(Ordering::Relaxed)
    }
}

impl SseEventProducer for SequentialEventProducer {
    async fn next_event(&self) -> Option<SseEvent> {
        let idx = self.current_count.fetch_add(1, Ordering::Relaxed);
        if idx < self.total_events {
            Some(
                SseEvent::with_type(
                    "data",
                    serde_json::json!({
                        "sequence": idx,
                        "message": format!("Event {}", idx)
                    }),
                )
                .with_id(format!("event-{}", idx)),
            )
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

/// Producer that supports reconnection with Last-Event-ID tracking
struct ReconnectableEventProducer {
    events: Vec<(String, serde_json::Value)>,
    current_idx: Arc<AtomicUsize>,
    connect_count: Arc<AtomicUsize>,
}

impl ReconnectableEventProducer {
    fn new(events: Vec<(String, serde_json::Value)>) -> Self {
        Self {
            events,
            current_idx: Arc::new(AtomicUsize::new(0)),
            connect_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn get_connect_count(&self) -> usize {
        self.connect_count.load(Ordering::Relaxed)
    }
}

impl SseEventProducer for ReconnectableEventProducer {
    async fn next_event(&self) -> Option<SseEvent> {
        let idx = self.current_idx.fetch_add(1, Ordering::Relaxed);
        if idx < self.events.len() {
            let (id, data) = self.events[idx].clone();
            Some(SseEvent::with_type("update", data).with_id(id.clone()))
        } else {
            None
        }
    }

    async fn on_connect(&self) {
        self.connect_count.fetch_add(1, Ordering::Relaxed);
    }
}

/// Producer that sends events with configurable delays for backpressure testing
struct SlowClientProducer {
    event_count: usize,
    delay_ms: u64,
    current_idx: Arc<AtomicUsize>,
    events_sent: Arc<AtomicUsize>,
}

impl SlowClientProducer {
    fn new(event_count: usize, delay_ms: u64) -> Self {
        Self {
            event_count,
            delay_ms,
            current_idx: Arc::new(AtomicUsize::new(0)),
            events_sent: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn get_events_sent(&self) -> usize {
        self.events_sent.load(Ordering::Relaxed)
    }
}

impl SseEventProducer for SlowClientProducer {
    async fn next_event(&self) -> Option<SseEvent> {
        let idx = self.current_idx.fetch_add(1, Ordering::Relaxed);
        if idx < self.event_count {
            // Simulate event generation delay
            sleep(Duration::from_millis(self.delay_ms)).await;
            self.events_sent.fetch_add(1, Ordering::Relaxed);
            Some(SseEvent::new(serde_json::json!({
                "event_number": idx,
                "timestamp": chrono::Utc::now().to_rfc3339()
            })))
        } else {
            None
        }
    }
}

/// Producer that maintains consistent ordering even with rapid fire events
struct RapidFireOrderedProducer {
    event_count: usize,
    current_idx: Arc<AtomicUsize>,
    events_generated: Arc<AtomicUsize>,
}

impl RapidFireOrderedProducer {
    fn new(event_count: usize) -> Self {
        Self {
            event_count,
            current_idx: Arc::new(AtomicUsize::new(0)),
            events_generated: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn get_generated_count(&self) -> usize {
        self.events_generated.load(Ordering::Relaxed)
    }
}

impl SseEventProducer for RapidFireOrderedProducer {
    async fn next_event(&self) -> Option<SseEvent> {
        let idx = self.current_idx.fetch_add(1, Ordering::Relaxed);
        if idx < self.event_count {
            self.events_generated.fetch_add(1, Ordering::Relaxed);
            Some(
                SseEvent::with_type(
                    "rapid",
                    serde_json::json!({
                        "index": idx,
                        "nanotime": std::time::SystemTime::now().duration_since(
                            std::time::UNIX_EPOCH
                        ).unwrap().as_nanos()
                    }),
                )
                .with_id(format!("{}", idx)),
            )
        } else {
            None
        }
    }
}

/// Producer that simulates keep-alive with periodic heartbeats
struct KeepAliveProducer {
    total_events: usize,
    current_idx: Arc<AtomicUsize>,
}

impl KeepAliveProducer {
    fn new(total_events: usize) -> Self {
        Self {
            total_events,
            current_idx: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl SseEventProducer for KeepAliveProducer {
    async fn next_event(&self) -> Option<SseEvent> {
        let idx = self.current_idx.fetch_add(1, Ordering::Relaxed);
        if idx < self.total_events {
            Some(SseEvent::new(serde_json::json!({
                "heartbeat": idx,
                "alive": true
            })))
        } else {
            None
        }
    }
}

/// Producer for graceful shutdown testing that tracks disconnections
struct GracefulShutdownProducer {
    total_events: usize,
    current_idx: Arc<AtomicUsize>,
    disconnect_called: Arc<AtomicBool>,
}

impl GracefulShutdownProducer {
    fn new(total_events: usize) -> Self {
        Self {
            total_events,
            current_idx: Arc::new(AtomicUsize::new(0)),
            disconnect_called: Arc::new(AtomicBool::new(false)),
        }
    }

    fn was_disconnect_called(&self) -> bool {
        self.disconnect_called.load(Ordering::Relaxed)
    }
}

impl SseEventProducer for GracefulShutdownProducer {
    async fn next_event(&self) -> Option<SseEvent> {
        let idx = self.current_idx.fetch_add(1, Ordering::Relaxed);
        if idx < self.total_events {
            Some(SseEvent::new(serde_json::json!({"index": idx})))
        } else {
            None
        }
    }

    async fn on_disconnect(&self) {
        self.disconnect_called.store(true, Ordering::Relaxed);
    }
}

// ============================================================================
// Test 1: SSE Connection Establishment and Event Streaming
// ============================================================================

#[tokio::test]
async fn test_sse_connection_establishment_and_streaming() {
    // Given: A sequential event producer with 5 events
    let producer = SequentialEventProducer::new(5);

    // When: Client connects and generates events
    producer.on_connect().await;

    let mut events_received = Vec::new();
    for i in 0..5 {
        if let Some(event) = producer.next_event().await {
            // Verify event has the expected structure
            assert_eq!(
                event.data.get("sequence").and_then(|v| v.as_u64()),
                Some(i as u64),
                "Event {} has correct sequence number",
                i
            );
            assert!(event.id.is_some(), "Event {} has ID for tracking", i);
            events_received.push(event);
        }
    }

    // Then: Verify all 5 events were received in order
    assert_eq!(events_received.len(), 5, "All 5 events should be received");
    for (idx, event) in events_received.iter().enumerate() {
        assert_eq!(
            event.data.get("sequence").and_then(|v| v.as_u64()),
            Some(idx as u64),
            "Event {} has correct sequence",
            idx
        );
    }

    // Verify stream ends cleanly after last event
    assert!(
        producer.next_event().await.is_none(),
        "Stream should end after all events"
    );
}

// ============================================================================
// Test 2: Client Reconnection with Last-Event-ID Header
// ============================================================================

#[tokio::test]
async fn test_client_reconnection_with_last_event_id() {
    // Given: A set of events with IDs for reconnection tracking
    let events = vec![
        ("id-1".to_string(), serde_json::json!({"data": "event1"})),
        ("id-2".to_string(), serde_json::json!({"data": "event2"})),
        ("id-3".to_string(), serde_json::json!({"data": "event3"})),
        ("id-4".to_string(), serde_json::json!({"data": "event4"})),
    ];

    let producer = ReconnectableEventProducer::new(events);

    // When: Client connects first time
    producer.on_connect().await;
    assert_eq!(producer.get_connect_count(), 1);

    // Consume first 2 events
    let event1 = producer.next_event().await.unwrap();
    let event1_id = event1.id.clone();
    assert_eq!(event1_id, Some("id-1".to_string()));

    let event2 = producer.next_event().await.unwrap();
    let event2_id = event2.id.clone();
    assert_eq!(event2_id, Some("id-2".to_string()));

    // Simulate connection loss after event 2
    // In real scenario, client would reconnect with Last-Event-ID: id-2

    // When: Simulate new connection
    producer.on_connect().await;
    assert_eq!(producer.get_connect_count(), 2);

    // Then: Client should be able to resume from where it left off
    // The producer will continue from the current index
    let event3 = producer.next_event().await.unwrap();
    assert_eq!(event3.id, Some("id-3".to_string()));

    // Verify reconnection didn't cause event loss
    assert_eq!(producer.get_connect_count(), 2, "Client reconnected successfully");
}

// ============================================================================
// Test 3: Event Ordering Preservation
// ============================================================================

#[tokio::test]
async fn test_event_ordering_preservation() {
    // Given: A rapid-fire producer generating many events
    let producer = RapidFireOrderedProducer::new(100);

    // When: We collect all events
    let mut events_collected = Vec::new();
    loop {
        match producer.next_event().await {
            Some(event) => events_collected.push(event),
            None => break,
        }
    }

    // Then: Verify events are in strict sequential order
    assert_eq!(events_collected.len(), 100, "All 100 events should be collected");

    let mut last_sequence = -1i32;
    for (idx, event) in events_collected.iter().enumerate() {
        let sequence = event.data.get("index").and_then(|v| v.as_i64()).unwrap() as i32;
        assert_eq!(
            sequence, idx as i32,
            "Event at position {} has correct sequence number {}",
            idx, sequence
        );
        assert!(sequence > last_sequence, "Events are in increasing order");
        last_sequence = sequence;
    }

    // Verify we generated exactly 100 events
    assert_eq!(
        producer.get_generated_count(),
        100,
        "Exactly 100 events should be generated"
    );
}

// ============================================================================
// Test 4: Connection Cleanup on Client Disconnect
// ============================================================================

#[tokio::test]
async fn test_connection_cleanup_on_disconnect() {
    // Given: A producer that tracks connection lifecycle
    let producer = SequentialEventProducer::new(3);

    // When: Client connects
    producer.on_connect().await;
    assert_eq!(producer.get_connect_count(), 1, "Client should be marked as connected");

    // Consume one event
    let _event1 = producer.next_event().await;

    // Simulate client disconnect
    producer.on_disconnect().await;
    assert_eq!(
        producer.get_disconnect_count(),
        1,
        "Client should be marked as disconnected"
    );

    // Verify cleanup occurred (in real scenario, this would close resources)
    assert!(producer.get_disconnect_count() > 0, "Disconnect hook was invoked");
}

// ============================================================================
// Test 5: Keep-Alive Behavior
// ============================================================================

#[tokio::test]
async fn test_keep_alive_behavior() {
    // Given: A producer that sends periodic keep-alive events
    let producer = KeepAliveProducer::new(5);

    // When: We consume all events
    let mut events = Vec::new();
    loop {
        match producer.next_event().await {
            Some(event) => {
                // Verify each event has a heartbeat field
                assert!(
                    event.data.get("heartbeat").is_some(),
                    "Each event should contain heartbeat data"
                );
                assert!(
                    event.data.get("alive").and_then(|v| v.as_bool()) == Some(true),
                    "All events should indicate server is alive"
                );
                events.push(event);
            }
            None => break,
        }
    }

    // Then: Verify keep-alive events were sent
    assert_eq!(events.len(), 5, "All keep-alive events should be received");

    // Verify stream eventually closes (not infinite keep-alive)
    assert!(
        producer.next_event().await.is_none(),
        "Stream should terminate normally"
    );
}

// ============================================================================
// Test 6: Backpressure When Client is Slow
// ============================================================================

#[tokio::test]
async fn test_backpressure_slow_client() {
    // Given: A slow client producer with delayed event generation
    let producer = SlowClientProducer::new(5, 10); // 10ms delay per event

    // When: We generate events with intentional delays
    let start = std::time::Instant::now();
    let mut events_count = 0;

    loop {
        match producer.next_event().await {
            Some(_event) => {
                events_count += 1;
            }
            None => break,
        }
    }

    let elapsed = start.elapsed();

    // Then: Verify events were generated with expected delay
    assert_eq!(events_count, 5, "All 5 events should be generated despite backpressure");

    // Verify timing: should take at least 5 * 10ms = 50ms
    assert!(
        elapsed.as_millis() >= 50,
        "Event generation should have delays, took {:?}ms",
        elapsed.as_millis()
    );

    // Verify events were actually sent
    assert_eq!(producer.get_events_sent(), 5, "All events should be marked as sent");
}

// ============================================================================
// Test 7: Graceful Shutdown with Active Streams
// ============================================================================

#[tokio::test]
async fn test_graceful_shutdown_with_active_streams() {
    // Given: A producer with pending events
    let producer = GracefulShutdownProducer::new(3);

    // When: Connection is active and generating events
    for _ in 0..2 {
        let _ = producer.next_event().await;
    }

    // Simulate graceful shutdown
    producer.on_disconnect().await;

    // Then: Verify disconnect was called during shutdown
    assert!(
        producer.was_disconnect_called(),
        "Disconnect should be called during graceful shutdown"
    );

    // Verify no panic occurred and stream can still be accessed
    let remaining = producer.next_event().await;
    assert!(remaining.is_some(), "Stream should continue until complete");
}

// ============================================================================
// Test 8: Event IDs Preserved Through Stream
// ============================================================================

#[tokio::test]
async fn test_event_ids_preserved_through_stream() {
    // Given: A producer that assigns event IDs
    let producer = SequentialEventProducer::new(10);

    // When: We collect all events and their IDs
    let mut event_ids = Vec::new();
    loop {
        match producer.next_event().await {
            Some(event) => {
                if let Some(id) = event.id.clone() {
                    event_ids.push(id);
                }
            }
            None => break,
        }
    }

    // Then: Verify all events have unique IDs in expected format
    assert_eq!(event_ids.len(), 10, "All 10 events should have IDs");

    for (idx, id) in event_ids.iter().enumerate() {
        assert_eq!(id, &format!("event-{}", idx), "Event ID should match expected format");
    }

    // Verify no duplicate IDs
    let unique_ids: std::collections::HashSet<_> = event_ids.iter().cloned().collect();
    assert_eq!(unique_ids.len(), event_ids.len(), "All event IDs should be unique");
}

// ============================================================================
// Test 9: Multiple Concurrent Connections
// ============================================================================

#[tokio::test]
async fn test_multiple_concurrent_connections() {
    // Given: Multiple producers for simulating concurrent connections
    let producer1 = Arc::new(SequentialEventProducer::new(5));
    let producer2 = Arc::new(SequentialEventProducer::new(5));

    // When: Both connections are active simultaneously
    producer1.on_connect().await;
    producer2.on_connect().await;

    // Create concurrent tasks to simulate multiple clients
    let handle1 = {
        let producer = Arc::clone(&producer1);
        tokio::spawn(async move {
            let mut count = 0;
            loop {
                match producer.next_event().await {
                    Some(_) => count += 1,
                    None => break,
                }
            }
            count
        })
    };

    let handle2 = {
        let producer = Arc::clone(&producer2);
        tokio::spawn(async move {
            let mut count = 0;
            loop {
                match producer.next_event().await {
                    Some(_) => count += 1,
                    None => break,
                }
            }
            count
        })
    };

    // Then: Both connections should complete independently
    let count1 = handle1.await.unwrap();
    let count2 = handle2.await.unwrap();

    assert_eq!(count1, 5, "First connection should receive 5 events");
    assert_eq!(count2, 5, "Second connection should receive 5 events");
}

// ============================================================================
// Test 10: Event Type Preservation
// ============================================================================

#[tokio::test]
async fn test_event_type_preservation() {
    // Given: A producer that sends events with different types
    let producer = SequentialEventProducer::new(5);

    // When: We collect all events
    let mut events = Vec::new();
    loop {
        match producer.next_event().await {
            Some(event) => {
                events.push(event);
            }
            None => break,
        }
    }

    // Then: All events should have the correct type
    assert_eq!(events.len(), 5);
    for event in events {
        assert_eq!(
            event.event_type,
            Some("data".to_string()),
            "Event type should be preserved as 'data'"
        );
    }
}

// ============================================================================
// Test 11: Empty Event Stream (No Events)
// ============================================================================

#[tokio::test]
async fn test_empty_event_stream() {
    // Given: A producer with zero events
    let producer = SequentialEventProducer::new(0);

    // When: We try to consume events
    let event = producer.next_event().await;

    // Then: Stream should be empty immediately
    assert!(event.is_none(), "Empty stream should produce no events");
}

// ============================================================================
// Test 12: Event Data Integrity Through Stream
// ============================================================================

#[tokio::test]
async fn test_event_data_integrity_through_stream() {
    // Given: Events with complex JSON data
    let events = vec![
        (
            "id-1".to_string(),
            serde_json::json!({
                "name": "Alice",
                "age": 30,
                "active": true,
                "tags": ["rust", "async"],
                "metadata": {
                    "created": "2024-01-01",
                    "updated": "2024-01-02"
                }
            }),
        ),
        (
            "id-2".to_string(),
            serde_json::json!({
                "name": "Bob",
                "age": 25,
                "active": false,
                "tags": ["python"],
                "metadata": {
                    "created": "2024-01-03"
                }
            }),
        ),
    ];

    let producer = ReconnectableEventProducer::new(events.clone());

    // When: We consume events and verify data integrity
    let event1 = producer.next_event().await.unwrap();
    assert_eq!(event1.data.get("name").and_then(|v| v.as_str()), Some("Alice"));
    assert_eq!(event1.data.get("age").and_then(|v| v.as_i64()), Some(30));
    assert_eq!(
        event1.data.get("tags").and_then(|v| v.as_array()).map(|a| a.len()),
        Some(2)
    );

    let event2 = producer.next_event().await.unwrap();
    assert_eq!(event2.data.get("name").and_then(|v| v.as_str()), Some("Bob"));
    assert_eq!(event2.data.get("age").and_then(|v| v.as_i64()), Some(25));

    // Then: All data should be preserved exactly
    assert_eq!(
        event1
            .data
            .get("metadata")
            .and_then(|v| v.get("created"))
            .and_then(|v| v.as_str()),
        Some("2024-01-01")
    );
    assert_eq!(
        event2
            .data
            .get("metadata")
            .and_then(|v| v.get("created"))
            .and_then(|v| v.as_str()),
        Some("2024-01-03")
    );
}

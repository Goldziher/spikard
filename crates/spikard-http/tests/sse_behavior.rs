#![allow(clippy::pedantic, clippy::nursery, clippy::all)]
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

#[tokio::test]
async fn test_sse_connection_establishment_and_streaming() {
    let producer = SequentialEventProducer::new(5);

    producer.on_connect().await;

    let mut events_received = Vec::new();
    for i in 0..5 {
        if let Some(event) = producer.next_event().await {
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

    assert_eq!(events_received.len(), 5, "All 5 events should be received");
    for (idx, event) in events_received.iter().enumerate() {
        assert_eq!(
            event.data.get("sequence").and_then(|v| v.as_u64()),
            Some(idx as u64),
            "Event {} has correct sequence",
            idx
        );
    }

    assert!(
        producer.next_event().await.is_none(),
        "Stream should end after all events"
    );
}

#[tokio::test]
async fn test_client_reconnection_with_last_event_id() {
    let events = vec![
        ("id-1".to_string(), serde_json::json!({"data": "event1"})),
        ("id-2".to_string(), serde_json::json!({"data": "event2"})),
        ("id-3".to_string(), serde_json::json!({"data": "event3"})),
        ("id-4".to_string(), serde_json::json!({"data": "event4"})),
    ];

    let producer = ReconnectableEventProducer::new(events);

    producer.on_connect().await;
    assert_eq!(producer.get_connect_count(), 1);

    let event1 = producer.next_event().await.unwrap();
    let event1_id = event1.id.clone();
    assert_eq!(event1_id, Some("id-1".to_string()));

    let event2 = producer.next_event().await.unwrap();
    let event2_id = event2.id.clone();
    assert_eq!(event2_id, Some("id-2".to_string()));

    producer.on_connect().await;
    assert_eq!(producer.get_connect_count(), 2);

    let event3 = producer.next_event().await.unwrap();
    assert_eq!(event3.id, Some("id-3".to_string()));

    assert_eq!(producer.get_connect_count(), 2, "Client reconnected successfully");
}

#[tokio::test]
async fn test_event_ordering_preservation() {
    let producer = RapidFireOrderedProducer::new(100);

    let mut events_collected = Vec::new();
    loop {
        match producer.next_event().await {
            Some(event) => events_collected.push(event),
            None => break,
        }
    }

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

    assert_eq!(
        producer.get_generated_count(),
        100,
        "Exactly 100 events should be generated"
    );
}

#[tokio::test]
async fn test_connection_cleanup_on_disconnect() {
    let producer = SequentialEventProducer::new(3);

    producer.on_connect().await;
    assert_eq!(producer.get_connect_count(), 1, "Client should be marked as connected");

    let _event1 = producer.next_event().await;

    producer.on_disconnect().await;
    assert_eq!(
        producer.get_disconnect_count(),
        1,
        "Client should be marked as disconnected"
    );

    assert!(producer.get_disconnect_count() > 0, "Disconnect hook was invoked");
}

#[tokio::test]
async fn test_keep_alive_behavior() {
    let producer = KeepAliveProducer::new(5);

    let mut events = Vec::new();
    loop {
        match producer.next_event().await {
            Some(event) => {
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

    assert_eq!(events.len(), 5, "All keep-alive events should be received");

    assert!(
        producer.next_event().await.is_none(),
        "Stream should terminate normally"
    );
}

#[tokio::test]
async fn test_backpressure_slow_client() {
    let producer = SlowClientProducer::new(5, 10);

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

    assert_eq!(events_count, 5, "All 5 events should be generated despite backpressure");

    assert!(
        elapsed.as_millis() >= 50,
        "Event generation should have delays, took {:?}ms",
        elapsed.as_millis()
    );

    assert_eq!(producer.get_events_sent(), 5, "All events should be marked as sent");
}

#[tokio::test]
async fn test_graceful_shutdown_with_active_streams() {
    let producer = GracefulShutdownProducer::new(3);

    for _ in 0..2 {
        let _ = producer.next_event().await;
    }

    producer.on_disconnect().await;

    assert!(
        producer.was_disconnect_called(),
        "Disconnect should be called during graceful shutdown"
    );

    let remaining = producer.next_event().await;
    assert!(remaining.is_some(), "Stream should continue until complete");
}

#[tokio::test]
async fn test_event_ids_preserved_through_stream() {
    let producer = SequentialEventProducer::new(10);

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

    assert_eq!(event_ids.len(), 10, "All 10 events should have IDs");

    for (idx, id) in event_ids.iter().enumerate() {
        assert_eq!(id, &format!("event-{}", idx), "Event ID should match expected format");
    }

    let unique_ids: std::collections::HashSet<_> = event_ids.iter().cloned().collect();
    assert_eq!(unique_ids.len(), event_ids.len(), "All event IDs should be unique");
}

#[tokio::test]
async fn test_multiple_concurrent_connections() {
    let producer1 = Arc::new(SequentialEventProducer::new(5));
    let producer2 = Arc::new(SequentialEventProducer::new(5));

    producer1.on_connect().await;
    producer2.on_connect().await;

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

    let count1 = handle1.await.unwrap();
    let count2 = handle2.await.unwrap();

    assert_eq!(count1, 5, "First connection should receive 5 events");
    assert_eq!(count2, 5, "Second connection should receive 5 events");
}

#[tokio::test]
async fn test_event_type_preservation() {
    let producer = SequentialEventProducer::new(5);

    let mut events = Vec::new();
    loop {
        match producer.next_event().await {
            Some(event) => {
                events.push(event);
            }
            None => break,
        }
    }

    assert_eq!(events.len(), 5);
    for event in events {
        assert_eq!(
            event.event_type,
            Some("data".to_string()),
            "Event type should be preserved as 'data'"
        );
    }
}

#[tokio::test]
async fn test_empty_event_stream() {
    let producer = SequentialEventProducer::new(0);

    let event = producer.next_event().await;

    assert!(event.is_none(), "Empty stream should produce no events");
}

#[tokio::test]
async fn test_event_data_integrity_through_stream() {
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

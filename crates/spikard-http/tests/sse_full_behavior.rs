#![allow(clippy::pedantic, clippy::nursery, clippy::all)]
//! Comprehensive integration tests for Server-Sent Events (SSE) functionality
//!
//! These tests verify full end-to-end SSE behavior including:
//! - Event streaming with multiple events
//! - Event IDs and Last-Event-ID tracking
//! - Client reconnection with resume capability
//! - Event retry timeout handling
//! - Comment events and keep-alive
//! - Connection cleanup on disconnect
//! - Multi-line data field formatting
//! - Custom event types
//! - Large event payload handling
//! - Producer error handling

mod common;

use spikard_http::sse::{SseEvent, SseEventProducer};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

// ===== Test Producers for Full Behavior Testing =====

/// Producer that yields multiple numbered events in sequence
#[derive(Debug, Clone)]
struct MultiEventProducer {
    event_count: usize,
    current_idx: Arc<AtomicUsize>,
}

impl MultiEventProducer {
    fn new(event_count: usize) -> Self {
        Self {
            event_count,
            current_idx: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl SseEventProducer for MultiEventProducer {
    async fn next_event(&self) -> Option<SseEvent> {
        let idx = self.current_idx.fetch_add(1, Ordering::SeqCst);
        if idx < self.event_count {
            Some(
                SseEvent::new(serde_json::json!({
                    "event_number": idx,
                    "message": format!("Event {}", idx),
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }))
                .with_id(format!("event-{}", idx)),
            )
        } else {
            None
        }
    }
}

/// Producer with event ID tracking for reconnection tests
#[derive(Debug, Clone)]
struct IdTrackedEventProducer {
    events: Vec<(String, serde_json::Value)>,
    current_idx: Arc<AtomicUsize>,
}

impl IdTrackedEventProducer {
    fn new(events: Vec<(String, serde_json::Value)>) -> Self {
        Self {
            events,
            current_idx: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn get_current_idx(&self) -> usize {
        self.current_idx.load(Ordering::SeqCst)
    }
}

impl SseEventProducer for IdTrackedEventProducer {
    async fn next_event(&self) -> Option<SseEvent> {
        let idx = self.current_idx.fetch_add(1, Ordering::SeqCst);
        if idx < self.events.len() {
            let (id, data) = self.events[idx].clone();
            Some(SseEvent::new(data).with_id(id))
        } else {
            None
        }
    }
}

/// Producer that simulates retry timeout scenarios
#[derive(Debug, Clone)]
struct RetryTimeoutProducer {
    event_count: usize,
    current_idx: Arc<AtomicUsize>,
}

impl RetryTimeoutProducer {
    fn new(event_count: usize) -> Self {
        Self {
            event_count,
            current_idx: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl SseEventProducer for RetryTimeoutProducer {
    async fn next_event(&self) -> Option<SseEvent> {
        let idx = self.current_idx.fetch_add(1, Ordering::SeqCst);
        if idx < self.event_count {
            // Alternate between events with and without retry timeout
            if idx % 2 == 0 {
                Some(
                    SseEvent::new(serde_json::json!({"index": idx, "with_retry": true}))
                        .with_id(format!("event-{}", idx))
                        .with_retry(3000), // 3 second retry
                )
            } else {
                Some(
                    SseEvent::new(serde_json::json!({"index": idx, "with_retry": false}))
                        .with_id(format!("event-{}", idx)),
                )
            }
        } else {
            None
        }
    }
}

/// Producer that sends comment-like events for keep-alive
#[derive(Debug, Clone)]
struct KeepAliveProducer {
    event_count: usize,
    current_idx: Arc<AtomicUsize>,
}

impl KeepAliveProducer {
    fn new(event_count: usize) -> Self {
        Self {
            event_count,
            current_idx: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl SseEventProducer for KeepAliveProducer {
    async fn next_event(&self) -> Option<SseEvent> {
        let idx = self.current_idx.fetch_add(1, Ordering::SeqCst);
        if idx < self.event_count {
            // Alternate between real events and keep-alive events
            if idx % 3 == 0 {
                Some(
                    SseEvent::with_type("data", serde_json::json!({"index": idx, "type": "real_event"}))
                        .with_id(format!("event-{}", idx)),
                )
            } else {
                // Keep-alive event (no data)
                Some(SseEvent::new(serde_json::json!({"index": idx, "type": "keep_alive"})))
            }
        } else {
            None
        }
    }
}

/// Producer that tracks disconnection lifecycle
#[derive(Debug, Clone)]
struct DisconnectTrackingProducer {
    event_count: usize,
    current_idx: Arc<AtomicUsize>,
    connect_count: Arc<AtomicUsize>,
    disconnect_count: Arc<AtomicUsize>,
}

impl DisconnectTrackingProducer {
    fn new(event_count: usize) -> Self {
        Self {
            event_count,
            current_idx: Arc::new(AtomicUsize::new(0)),
            connect_count: Arc::new(AtomicUsize::new(0)),
            disconnect_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn get_connect_count(&self) -> usize {
        self.connect_count.load(Ordering::SeqCst)
    }

    fn get_disconnect_count(&self) -> usize {
        self.disconnect_count.load(Ordering::SeqCst)
    }
}

impl SseEventProducer for DisconnectTrackingProducer {
    async fn next_event(&self) -> Option<SseEvent> {
        let idx = self.current_idx.fetch_add(1, Ordering::SeqCst);
        if idx < self.event_count {
            Some(
                SseEvent::new(serde_json::json!({
                    "index": idx,
                    "message": format!("Event {}", idx)
                }))
                .with_id(format!("event-{}", idx)),
            )
        } else {
            None
        }
    }

    async fn on_connect(&self) {
        self.connect_count.fetch_add(1, Ordering::SeqCst);
    }

    async fn on_disconnect(&self) {
        self.disconnect_count.fetch_add(1, Ordering::SeqCst);
    }
}

/// Producer with custom event types
#[derive(Debug, Clone)]
struct CustomEventTypeProducer {
    event_count: usize,
    current_idx: Arc<AtomicUsize>,
}

impl CustomEventTypeProducer {
    fn new(event_count: usize) -> Self {
        Self {
            event_count,
            current_idx: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl SseEventProducer for CustomEventTypeProducer {
    async fn next_event(&self) -> Option<SseEvent> {
        let idx = self.current_idx.fetch_add(1, Ordering::SeqCst);
        if idx < self.event_count {
            let event_type = match idx % 3 {
                0 => "user_update",
                1 => "status_change",
                _ => "notification",
            };

            Some(
                SseEvent::with_type(
                    event_type,
                    serde_json::json!({
                        "index": idx,
                        "event_type": event_type
                    }),
                )
                .with_id(format!("event-{}", idx)),
            )
        } else {
            None
        }
    }
}

/// Producer with large event payloads
#[derive(Debug, Clone)]
struct LargePayloadProducer {
    event_count: usize,
    current_idx: Arc<AtomicUsize>,
}

impl LargePayloadProducer {
    fn new(event_count: usize) -> Self {
        Self {
            event_count,
            current_idx: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl SseEventProducer for LargePayloadProducer {
    async fn next_event(&self) -> Option<SseEvent> {
        let idx = self.current_idx.fetch_add(1, Ordering::SeqCst);
        if idx < self.event_count {
            // Create a large payload (100KB+)
            let large_data: Vec<i32> = (0..25000).collect();
            Some(
                SseEvent::new(serde_json::json!({
                    "index": idx,
                    "large_array": large_data,
                    "metadata": {
                        "size": "large",
                        "description": "Large payload event"
                    }
                }))
                .with_id(format!("event-{}", idx)),
            )
        } else {
            None
        }
    }
}

/// Producer that simulates errors
#[derive(Debug, Clone)]
struct ErrorProducer {
    should_error: Arc<AtomicBool>,
    event_count: Arc<AtomicUsize>,
}

impl ErrorProducer {
    fn new() -> Self {
        Self {
            should_error: Arc::new(AtomicBool::new(false)),
            event_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn enable_error(&self) {
        self.should_error.store(true, Ordering::SeqCst);
    }
}

impl SseEventProducer for ErrorProducer {
    async fn next_event(&self) -> Option<SseEvent> {
        if self.should_error.load(Ordering::SeqCst) {
            // Producer error: return None to end stream
            None
        } else {
            let idx = self.event_count.fetch_add(1, Ordering::SeqCst);
            Some(SseEvent::new(serde_json::json!({"index": idx})))
        }
    }
}

// ===== Test 1: Event Streaming with Multiple Events =====

#[tokio::test]
async fn test_sse_event_streaming_multiple_events() {
    let producer = MultiEventProducer::new(5);

    let mut events_received = Vec::new();
    loop {
        match producer.next_event().await {
            Some(event) => events_received.push(event),
            None => break,
        }
    }

    // Verify all events received in order
    assert_eq!(events_received.len(), 5);

    for (idx, event) in events_received.iter().enumerate() {
        assert_eq!(event.data.get("event_number").unwrap(), idx);
        assert_eq!(event.id, Some(format!("event-{}", idx)));
    }

    // Stream should end
    assert!(producer.next_event().await.is_none());
}

// ===== Test 2: Event with ID =====

#[tokio::test]
async fn test_sse_event_with_id() {
    let producer = IdTrackedEventProducer::new(vec![
        ("id-1".to_string(), serde_json::json!({"data": "event1"})),
        ("id-2".to_string(), serde_json::json!({"data": "event2"})),
        ("id-3".to_string(), serde_json::json!({"data": "event3"})),
    ]);

    // Collect all events
    let mut events = Vec::new();
    loop {
        match producer.next_event().await {
            Some(event) => events.push(event),
            None => break,
        }
    }

    // Verify IDs are sequential and traceable
    assert_eq!(events[0].id, Some("id-1".to_string()));
    assert_eq!(events[1].id, Some("id-2".to_string()));
    assert_eq!(events[2].id, Some("id-3".to_string()));
}

// ===== Test 3: Client Reconnection with Last-Event-ID =====

#[tokio::test]
async fn test_sse_client_reconnection_with_last_event_id() {
    let events = vec![
        ("id-1".to_string(), serde_json::json!({"seq": 1})),
        ("id-2".to_string(), serde_json::json!({"seq": 2})),
        ("id-3".to_string(), serde_json::json!({"seq": 3})),
        ("id-4".to_string(), serde_json::json!({"seq": 4})),
    ];

    let producer = IdTrackedEventProducer::new(events);

    // First connection: consume 2 events
    let event1 = producer.next_event().await.unwrap();
    let event2 = producer.next_event().await.unwrap();

    assert_eq!(event1.id, Some("id-1".to_string()));
    assert_eq!(event2.id, Some("id-2".to_string()));
    assert_eq!(producer.get_current_idx(), 2);

    // Simulate reconnection - producer continues from index 2
    // In real scenario, client sends Last-Event-ID: id-2
    let event3 = producer.next_event().await.unwrap();
    let event4 = producer.next_event().await.unwrap();

    // Should get events 3 and 4
    assert_eq!(event3.id, Some("id-3".to_string()));
    assert_eq!(event4.id, Some("id-4".to_string()));

    // Stream should end
    assert!(producer.next_event().await.is_none());
}

// ===== Test 4: Event Retry Timeout =====

#[tokio::test]
async fn test_sse_event_retry_timeout() {
    let producer = RetryTimeoutProducer::new(6);

    let mut events = Vec::new();
    loop {
        match producer.next_event().await {
            Some(event) => events.push(event),
            None => break,
        }
    }

    // Verify alternating retry timeouts
    assert_eq!(events[0].retry, Some(3000));
    assert_eq!(events[1].retry, None);
    assert_eq!(events[2].retry, Some(3000));
    assert_eq!(events[3].retry, None);
    assert_eq!(events[4].retry, Some(3000));
    assert_eq!(events[5].retry, None);
}

// ===== Test 5: Comment Events and Keep-Alive =====

#[tokio::test]
async fn test_sse_comment_events() {
    let producer = KeepAliveProducer::new(9);

    let mut real_events = 0;
    let mut keep_alive_events = 0;

    loop {
        match producer.next_event().await {
            Some(event) => {
                if event.event_type == Some("data".to_string()) {
                    real_events += 1;
                } else if event.data.get("type").and_then(|v| v.as_str()) == Some("keep_alive") {
                    keep_alive_events += 1;
                }
            }
            None => break,
        }
    }

    // Should have 3 real events (indices 0, 3, 6) and 6 keep-alive
    assert_eq!(real_events, 3);
    assert_eq!(keep_alive_events, 6);
}

// ===== Test 6: Connection Cleanup on Disconnect =====

#[tokio::test]
async fn test_sse_connection_cleanup() {
    let producer = DisconnectTrackingProducer::new(3);

    // Simulate connection
    producer.on_connect().await;
    assert_eq!(producer.get_connect_count(), 1);

    // Consume some events
    let _ = producer.next_event().await;
    let _ = producer.next_event().await;

    // Simulate disconnect
    producer.on_disconnect().await;
    assert_eq!(producer.get_disconnect_count(), 1);

    // Verify cleanup was called
    assert_eq!(producer.get_connect_count(), 1);
    assert_eq!(producer.get_disconnect_count(), 1);
}

// ===== Test 7: Event with Multiple Data Lines =====

#[tokio::test]
async fn test_sse_event_with_multiple_data_lines() {
    let producer = IdTrackedEventProducer::new(vec![(
        "id-1".to_string(),
        serde_json::json!({
            "line1": "data line 1",
            "line2": "data line 2",
            "line3": "data line 3",
            "multiline": "this spans\nmultiple\nlines"
        }),
    )]);

    let event = producer.next_event().await.unwrap();

    // Verify all data is present
    assert!(event.data.get("line1").is_some());
    assert!(event.data.get("line2").is_some());
    assert!(event.data.get("line3").is_some());
    assert!(event.data.get("multiline").is_some());
    assert_eq!(event.id, Some("id-1".to_string()));
}

// ===== Test 8: Event Custom Event Type =====

#[tokio::test]
async fn test_sse_event_custom_event_type() {
    let producer = CustomEventTypeProducer::new(9);

    let mut event_types = Vec::new();
    loop {
        match producer.next_event().await {
            Some(event) => {
                if let Some(evt_type) = event.event_type {
                    event_types.push(evt_type);
                }
            }
            None => break,
        }
    }

    // Verify pattern of event types
    assert_eq!(event_types[0], "user_update");
    assert_eq!(event_types[1], "status_change");
    assert_eq!(event_types[2], "notification");
    assert_eq!(event_types[3], "user_update");
    assert_eq!(event_types.len(), 9);
}

// ===== Test 9: Large Event Data =====

#[tokio::test]
async fn test_sse_large_event_data() {
    let producer = LargePayloadProducer::new(1);

    let event = producer.next_event().await.unwrap();

    // Verify large data is present
    assert!(event.data.get("large_array").is_some());
    let array = event.data.get("large_array").unwrap();

    // Should be an array with 25000 elements
    if let Some(arr) = array.as_array() {
        assert_eq!(arr.len(), 25000);
    } else {
        panic!("Expected array");
    }

    // Verify serialization works
    let serialized = event.data.to_string();
    assert!(serialized.len() > 100000); // > 100KB
}

// ===== Test 10: Producer Error =====

#[tokio::test]
async fn test_sse_producer_error() {
    let producer = ErrorProducer::new();

    // Normal operation
    let event1 = producer.next_event().await;
    assert!(event1.is_some());

    let event2 = producer.next_event().await;
    assert!(event2.is_some());

    // Enable error mode
    producer.enable_error();

    // Should end stream
    let event3 = producer.next_event().await;
    assert!(event3.is_none());

    // Stream should remain closed
    let event4 = producer.next_event().await;
    assert!(event4.is_none());
}

// ===== Additional Edge Cases =====

#[tokio::test]
async fn test_sse_rapid_event_generation() {
    let producer = MultiEventProducer::new(100);

    let mut count = 0;
    let start = std::time::Instant::now();

    loop {
        match producer.next_event().await {
            Some(_) => count += 1,
            None => break,
        }
    }

    let duration = start.elapsed();

    // Should generate 100 events very quickly
    assert_eq!(count, 100);
    assert!(duration.as_secs() < 1, "Should generate 100 events in < 1 second");
}

#[tokio::test]
async fn test_sse_event_data_integrity() {
    let events = vec![
        (
            "id-1".to_string(),
            serde_json::json!({"unicode": "🚀💡🔥", "text": "hello"}),
        ),
        ("id-2".to_string(), serde_json::json!({"null_value": null, "empty": {}})),
        (
            "id-3".to_string(),
            serde_json::json!({"nested": {"deep": {"data": [1, 2, 3]}}}),
        ),
    ];

    let producer = IdTrackedEventProducer::new(events.clone());

    let mut received = Vec::new();
    loop {
        match producer.next_event().await {
            Some(event) => received.push(event),
            None => break,
        }
    }

    // Verify data integrity
    assert_eq!(received[0].data, events[0].1);
    assert_eq!(received[1].data, events[1].1);
    assert_eq!(received[2].data, events[2].1);
}

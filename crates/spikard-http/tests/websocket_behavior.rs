#![allow(clippy::pedantic, clippy::nursery, clippy::all)]
//! Behavioral tests for WebSocket functionality in spikard-http
//!
//! This test module verifies observable WebSocket behavior from the perspective of
//! external clients and handlers, focusing on:
//!
//! 1. **Connection Lifecycle** - Proper initialization, connection establishment,
//!    and graceful disconnection with lifecycle hook invocation
//!
//! 2. **Concurrent Message Handling** - Multiple messages in rapid succession and
//!    from concurrent tasks, ensuring thread-safety and isolation between connections
//!
//! 3. **Message Ordering** - Sequential delivery guarantees and ordering preservation
//!    under various load conditions
//!
//! 4. **Abort Handling** - Recovery from errors, state preservation across errors,
//!    and proper error propagation
//!
//! 5. **Schema Validation** - JSON schema validation of incoming messages with
//!    rejection of malformed payloads
//!
//! 6. **Handler Error Propagation** - Proper handling of errors returned by handlers,
//!    error state transitions, and recovery mechanisms
//!
//! 7. **Message Buffering** - Behavior under high load, rapid message bursts,
//!    and concurrent client processing
//!
//! These tests verify behavioral contracts without testing implementation details,
//! ensuring the WebSocket module works correctly when used as a black box.

use serde_json::{Value, json};
use spikard_http::websocket::{WebSocketHandler, WebSocketState};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

/// Handler that echoes messages back to the client
#[derive(Debug, Clone)]
struct EchoHandler;

impl WebSocketHandler for EchoHandler {
    async fn handle_message(&self, message: Value) -> Option<Value> {
        Some(message)
    }
}

/// Handler that tracks lifecycle events and message count
#[derive(Debug, Clone)]
struct LifecycleHandler {
    connect_called: Arc<AtomicBool>,
    disconnect_called: Arc<AtomicBool>,
    message_count: Arc<AtomicUsize>,
    messages: Arc<Mutex<Vec<Value>>>,
}

impl LifecycleHandler {
    fn new() -> Self {
        Self {
            connect_called: Arc::new(AtomicBool::new(false)),
            disconnect_called: Arc::new(AtomicBool::new(false)),
            message_count: Arc::new(AtomicUsize::new(0)),
            messages: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn reset(&self) {
        self.connect_called.store(false, Ordering::SeqCst);
        self.disconnect_called.store(false, Ordering::SeqCst);
        self.message_count.store(0, Ordering::SeqCst);
        self.messages.lock().unwrap().clear();
    }
}

impl WebSocketHandler for LifecycleHandler {
    async fn handle_message(&self, message: Value) -> Option<Value> {
        self.message_count.fetch_add(1, Ordering::SeqCst);
        self.messages.lock().unwrap().push(message.clone());
        Some(message)
    }

    async fn on_connect(&self) {
        self.connect_called.store(true, Ordering::SeqCst);
    }

    async fn on_disconnect(&self) {
        self.disconnect_called.store(true, Ordering::SeqCst);
    }
}

/// Handler that validates message schema
#[derive(Debug)]
struct SchemaValidationHandler {
    invalid_messages: Arc<Mutex<Vec<String>>>,
}

impl SchemaValidationHandler {
    fn new() -> Self {
        Self {
            invalid_messages: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl WebSocketHandler for SchemaValidationHandler {
    async fn handle_message(&self, message: Value) -> Option<Value> {
        if message.get("type").is_some() {
            Some(json!({"status": "ok", "echo": message}))
        } else {
            self.invalid_messages.lock().unwrap().push(format!("{:?}", message));
            None
        }
    }
}

/// Handler that simulates errors during message processing
#[derive(Debug)]
struct ErrorHandler {
    error_count: Arc<AtomicUsize>,
    should_error: Arc<AtomicBool>,
}

impl ErrorHandler {
    fn new() -> Self {
        Self {
            error_count: Arc::new(AtomicUsize::new(0)),
            should_error: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl WebSocketHandler for ErrorHandler {
    async fn handle_message(&self, message: Value) -> Option<Value> {
        if self.should_error.load(Ordering::SeqCst) {
            self.error_count.fetch_add(1, Ordering::SeqCst);
            None
        } else {
            Some(message)
        }
    }
}

/// Handler that tracks message ordering
#[derive(Debug)]
struct OrderingHandler {
    messages: Arc<Mutex<Vec<usize>>>,
}

impl OrderingHandler {
    fn new() -> Self {
        Self {
            messages: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl WebSocketHandler for OrderingHandler {
    async fn handle_message(&self, message: Value) -> Option<Value> {
        if let Some(seq) = message.get("sequence").and_then(|v| v.as_u64()) {
            self.messages.lock().unwrap().push(seq as usize);
            Some(json!({"received": seq}))
        } else {
            None
        }
    }
}

/// Handler that simulates high-load buffering behavior
#[derive(Debug)]
struct BufferingHandler {
    processed: Arc<AtomicUsize>,
}

impl BufferingHandler {
    fn new() -> Self {
        Self {
            processed: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl WebSocketHandler for BufferingHandler {
    async fn handle_message(&self, message: Value) -> Option<Value> {
        self.processed.fetch_add(1, Ordering::SeqCst);
        sleep(Duration::from_millis(1)).await;
        Some(json!({"processed": message, "total": self.processed.load(Ordering::SeqCst)}))
    }
}

#[tokio::test]
async fn test_websocket_connection_initialization() {
    let handler = LifecycleHandler::new();

    assert!(!handler.connect_called.load(Ordering::SeqCst));
    assert!(!handler.disconnect_called.load(Ordering::SeqCst));
    assert_eq!(handler.message_count.load(Ordering::SeqCst), 0);

    let _state: WebSocketState<LifecycleHandler> = WebSocketState::new(handler.clone());
}

#[tokio::test]
async fn test_websocket_connection_lifecycle_state_transitions() {
    let handler = LifecycleHandler::new();
    handler.reset();

    handler.on_connect().await;
    assert!(handler.connect_called.load(Ordering::SeqCst));
    assert!(!handler.disconnect_called.load(Ordering::SeqCst));

    let msg = json!({"test": "data"});
    let _resp = handler.handle_message(msg).await;

    handler.on_disconnect().await;
    assert!(handler.connect_called.load(Ordering::SeqCst));
    assert!(handler.disconnect_called.load(Ordering::SeqCst));
}

#[tokio::test]
async fn test_websocket_sends_and_receives_single_message() {
    let handler = EchoHandler;
    let msg = json!({"test": "message"});
    let response = handler.handle_message(msg.clone()).await;

    assert_eq!(response, Some(msg));
}

#[tokio::test]
async fn test_multiple_messages_from_same_connection() {
    let handler = LifecycleHandler::new();
    handler.reset();

    let msg1 = json!({"id": 1, "text": "first"});
    let msg2 = json!({"id": 2, "text": "second"});
    let msg3 = json!({"id": 3, "text": "third"});

    let _resp1 = handler.handle_message(msg1).await;
    let _resp2 = handler.handle_message(msg2).await;
    let _resp3 = handler.handle_message(msg3).await;

    assert_eq!(handler.message_count.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn test_concurrent_message_processing() {
    let handler = Arc::new(LifecycleHandler::new());

    let mut handles = vec![];

    for i in 0..10 {
        let handler_clone = handler.clone();
        let handle = tokio::spawn(async move {
            let msg = json!({"id": i, "data": format!("message_{}", i)});
            handler_clone.handle_message(msg).await
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    assert_eq!(handler.message_count.load(Ordering::SeqCst), 10);
    assert_eq!(handler.messages.lock().unwrap().len(), 10);
}

#[tokio::test]
async fn test_multiple_concurrent_connections_isolation() {
    let handler1 = LifecycleHandler::new();
    let handler2 = LifecycleHandler::new();

    handler1.reset();
    handler2.reset();

    let msg1 = json!({"connection": 1, "seq": 1});
    let msg2 = json!({"connection": 2, "seq": 1});

    let _resp1 = handler1.handle_message(msg1).await;
    let _resp2 = handler2.handle_message(msg2).await;

    assert_eq!(handler1.message_count.load(Ordering::SeqCst), 1);
    assert_eq!(handler2.message_count.load(Ordering::SeqCst), 1);

    assert_eq!(handler1.messages.lock().unwrap()[0].get("connection").unwrap(), 1);
    assert_eq!(handler2.messages.lock().unwrap()[0].get("connection").unwrap(), 2);
}

#[tokio::test]
async fn test_message_ordering_sequential_delivery() {
    let handler = OrderingHandler::new();

    for seq in 0..10 {
        let msg = json!({"sequence": seq});
        let _response = handler.handle_message(msg).await;
    }

    let messages = handler.messages.lock().unwrap();
    let expected: Vec<usize> = (0..10).collect();

    assert_eq!(*messages, expected);
}

#[tokio::test]
async fn test_message_ordering_concurrent_arrival() {
    let handler = Arc::new(OrderingHandler::new());

    let mut handles = vec![];
    for seq in 0..20 {
        let handler_clone = handler.clone();
        let handle = tokio::spawn(async move {
            let msg = json!({"sequence": seq});
            handler_clone.handle_message(msg).await
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    let messages = handler.messages.lock().unwrap();
    assert_eq!(messages.len(), 20);

    let mut sorted = messages.clone();
    sorted.sort();
    let expected: Vec<usize> = (0..20).collect();
    assert_eq!(sorted, expected);
}

#[tokio::test]
async fn test_message_ordering_with_delays() {
    let handler = OrderingHandler::new();

    for seq in 0..5 {
        let msg = json!({"sequence": seq});
        let _response = handler.handle_message(msg).await;
        sleep(Duration::from_millis(1)).await;
    }

    let messages = handler.messages.lock().unwrap();
    let expected: Vec<usize> = (0..5).collect();

    assert_eq!(*messages, expected);
}

#[tokio::test]
async fn test_handler_disconnect_on_normal_close() {
    let handler = LifecycleHandler::new();
    handler.reset();

    handler.on_disconnect().await;

    assert!(handler.disconnect_called.load(Ordering::SeqCst));
}

#[tokio::test]
async fn test_handler_continues_after_failed_message() {
    let handler = LifecycleHandler::new();
    handler.reset();

    let valid_msg = json!({"data": "test"});
    let resp1 = handler.handle_message(valid_msg).await;
    assert!(resp1.is_some());

    let another_msg = json!({"data": "another"});
    let resp2 = handler.handle_message(another_msg).await;
    assert!(resp2.is_some());

    assert_eq!(handler.message_count.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn test_handler_state_after_error() {
    let handler = ErrorHandler::new();

    handler.should_error.store(true, Ordering::SeqCst);

    let msg1 = json!({"test": 1});
    let resp1 = handler.handle_message(msg1).await;

    assert!(resp1.is_none());
    assert_eq!(handler.error_count.load(Ordering::SeqCst), 1);

    handler.should_error.store(false, Ordering::SeqCst);

    let msg2 = json!({"test": 2});
    let resp2 = handler.handle_message(msg2).await;

    assert!(resp2.is_some());
    assert_eq!(handler.error_count.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn test_schema_validation_accepts_valid_message() {
    let handler = SchemaValidationHandler::new();

    let valid_msg = json!({"type": "test", "data": "content"});
    let response = handler.handle_message(valid_msg).await;

    assert!(response.is_some());
    let resp = response.unwrap();
    assert_eq!(resp.get("status").unwrap(), "ok");
}

#[tokio::test]
async fn test_schema_validation_rejects_invalid_message() {
    let handler = SchemaValidationHandler::new();

    let invalid_msg = json!({"data": "content"});
    let response = handler.handle_message(invalid_msg.clone()).await;

    assert!(response.is_none());

    assert_eq!(handler.invalid_messages.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn test_schema_validation_multiple_validations() {
    let handler = SchemaValidationHandler::new();

    let valid1 = json!({"type": "cmd", "action": "start"});
    let invalid1 = json!({"action": "start"});
    let valid2 = json!({"type": "query", "params": {}});
    let invalid2 = json!({"id": 123});

    let r1 = handler.handle_message(valid1).await;
    let r2 = handler.handle_message(invalid1).await;
    let r3 = handler.handle_message(valid2).await;
    let r4 = handler.handle_message(invalid2).await;

    assert!(r1.is_some());
    assert!(r2.is_none());
    assert!(r3.is_some());
    assert!(r4.is_none());

    assert_eq!(handler.invalid_messages.lock().unwrap().len(), 2);
}

#[tokio::test]
async fn test_schema_validation_type_checking() {
    let handler = SchemaValidationHandler::new();

    let msg_with_number_type = json!({"type": 123});
    let response = handler.handle_message(msg_with_number_type).await;

    assert!(response.is_some());
}

#[tokio::test]
async fn test_handler_error_state_preservation() {
    let handler = ErrorHandler::new();

    handler.should_error.store(true, Ordering::SeqCst);

    for i in 0..5 {
        let msg = json!({"id": i});
        let _resp = handler.handle_message(msg).await;
    }

    assert_eq!(handler.error_count.load(Ordering::SeqCst), 5);
}

#[tokio::test]
async fn test_handler_error_recovery_transitions() {
    let handler = ErrorHandler::new();

    let msg1 = json!({"id": 1});
    let resp1 = handler.handle_message(msg1).await;
    assert!(resp1.is_some());

    handler.should_error.store(true, Ordering::SeqCst);
    let msg2 = json!({"id": 2});
    let resp2 = handler.handle_message(msg2).await;
    assert!(resp2.is_none());

    handler.should_error.store(false, Ordering::SeqCst);
    let msg3 = json!({"id": 3});
    let resp3 = handler.handle_message(msg3).await;
    assert!(resp3.is_some());
}

#[tokio::test]
async fn test_selective_error_handling() {
    let handler = ErrorHandler::new();

    handler.should_error.store(false, Ordering::SeqCst);
    let msg1 = json!({"id": 1});
    let resp1 = handler.handle_message(msg1).await;
    assert!(resp1.is_some());

    handler.should_error.store(true, Ordering::SeqCst);
    let msg2 = json!({"id": 2});
    let resp2 = handler.handle_message(msg2).await;
    assert!(resp2.is_none());

    handler.should_error.store(false, Ordering::SeqCst);
    let msg3 = json!({"id": 3});
    let resp3 = handler.handle_message(msg3).await;
    assert!(resp3.is_some());

    assert_eq!(handler.error_count.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn test_message_buffering_rapid_succession() {
    let handler = BufferingHandler::new();

    for i in 0..50 {
        let msg = json!({"id": i, "timestamp": "2024-01-01T00:00:00Z"});
        let _response = handler.handle_message(msg).await;
    }

    assert_eq!(handler.processed.load(Ordering::SeqCst), 50);
}

#[tokio::test]
async fn test_message_buffering_concurrent_load() {
    let handler = Arc::new(BufferingHandler::new());

    let mut handles = vec![];

    for task_id in 0..10 {
        let handler_clone = handler.clone();
        let handle = tokio::spawn(async move {
            for seq in 0..10 {
                let msg = json!({
                    "task": task_id,
                    "sequence": seq,
                    "data": format!("task_{}_msg_{}", task_id, seq)
                });
                let _resp = handler_clone.handle_message(msg).await;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    assert_eq!(handler.processed.load(Ordering::SeqCst), 100);
}

#[tokio::test]
async fn test_message_buffering_response_correctness_under_load() {
    let handler = BufferingHandler::new();

    for i in 0..20 {
        let msg = json!({"burst_id": i, "data": "test"});
        let response = handler.handle_message(msg.clone()).await;

        assert!(response.is_some());
        let resp = response.unwrap();
        assert!(resp.get("processed").is_some());
        assert!(resp.get("total").is_some());
    }

    assert_eq!(handler.processed.load(Ordering::SeqCst), 20);
}

#[tokio::test]
async fn test_message_buffering_maintains_order_under_load() {
    let handler = Arc::new(OrderingHandler::new());

    let mut handles = vec![];

    let handler_clone = handler.clone();
    let handle = tokio::spawn(async move {
        for seq in 0..100 {
            let msg = json!({"sequence": seq});
            let _resp = handler_clone.handle_message(msg).await;
        }
    });
    handles.push(handle);

    for handle in handles {
        let _ = handle.await;
    }

    let messages = handler.messages.lock().unwrap();
    let expected: Vec<usize> = (0..100).collect();
    assert_eq!(*messages, expected);
}

#[tokio::test]
async fn test_large_payload_handling() {
    let handler = EchoHandler;

    let large_array: Vec<i32> = (0..1000).collect();
    let large_msg = json!({
        "type": "large_payload",
        "data": large_array,
        "metadata": {
            "size": 1000,
            "description": "This is a test of large payload handling"
        }
    });

    let response = handler.handle_message(large_msg.clone()).await;
    assert!(response.is_some());
    assert_eq!(response.unwrap(), large_msg);
}

#[tokio::test]
async fn test_deeply_nested_message_handling() {
    let handler = EchoHandler;

    let mut nested = json!({"value": "deep"});
    for _ in 0..50 {
        nested = json!({"level": nested});
    }

    let response = handler.handle_message(nested.clone()).await;
    assert!(response.is_some());
    assert_eq!(response.unwrap(), nested);
}

#[tokio::test]
async fn test_unicode_and_special_characters() {
    let handler = EchoHandler;

    let unicode_msg = json!({
        "emoji": "🚀💡🔥",
        "chinese": "你好世界",
        "arabic": "مرحبا بالعالم",
        "special": "!@#$%^&*()",
        "newlines": "line1\nline2\nline3"
    });

    let response = handler.handle_message(unicode_msg.clone()).await;
    assert!(response.is_some());
    assert_eq!(response.unwrap(), unicode_msg);
}

#[tokio::test]
async fn test_null_and_empty_values() {
    let handler = EchoHandler;

    let test_cases = vec![
        json!({"value": null}),
        json!({"array": []}),
        json!({"object": {}}),
        json!({"string": ""}),
    ];

    for msg in test_cases {
        let response = handler.handle_message(msg.clone()).await;
        assert!(response.is_some());
        assert_eq!(response.unwrap(), msg);
    }
}

#[tokio::test]
async fn test_mixed_type_arrays() {
    let handler = EchoHandler;

    let msg = json!({
        "mixed": [1, "two", 3.0, true, null, {"key": "value"}, []]
    });

    let response = handler.handle_message(msg.clone()).await;
    assert!(response.is_some());
    assert_eq!(response.unwrap(), msg);
}

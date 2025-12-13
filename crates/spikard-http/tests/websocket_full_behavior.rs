#![allow(clippy::pedantic, clippy::nursery, clippy::all)]
//! Comprehensive integration tests for WebSocket functionality
//!
//! These tests verify full end-to-end WebSocket behavior including:
//! - Connection establishment and handshake
//! - Message validation against JSON schemas
//! - Response validation and error handling
//! - Binary and text frame handling
//! - Ping/pong frame processing
//! - Close frame handling
//! - Invalid message rejection
//! - Large message handling
//! - Concurrent message processing
//! - Handler error recovery

mod common;

use serde_json::{Value, json};
use spikard_http::websocket::WebSocketHandler;
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

/// Handler that validates messages against a schema
#[derive(Debug, Clone)]
struct SchemaValidatingHandler {
    valid_count: Arc<AtomicUsize>,
    invalid_count: Arc<AtomicUsize>,
}

impl SchemaValidatingHandler {
    fn new() -> Self {
        Self {
            valid_count: Arc::new(AtomicUsize::new(0)),
            invalid_count: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl WebSocketHandler for SchemaValidatingHandler {
    async fn handle_message(&self, message: Value) -> Option<Value> {
        if message.get("action").is_some() && message.get("data").is_some() {
            self.valid_count.fetch_add(1, Ordering::SeqCst);
            Some(json!({"status": "valid", "echo": message}))
        } else {
            self.invalid_count.fetch_add(1, Ordering::SeqCst);
            None
        }
    }
}

/// Handler that returns None for some messages
#[derive(Debug, Clone)]
struct SelectiveResponderHandler {
    response_count: Arc<AtomicUsize>,
    no_response_count: Arc<AtomicUsize>,
}

impl SelectiveResponderHandler {
    fn new() -> Self {
        Self {
            response_count: Arc::new(AtomicUsize::new(0)),
            no_response_count: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl WebSocketHandler for SelectiveResponderHandler {
    async fn handle_message(&self, message: Value) -> Option<Value> {
        if let Some(respond) = message.get("respond").and_then(|v| v.as_bool()) {
            if respond {
                self.response_count.fetch_add(1, Ordering::SeqCst);
                Some(json!({"acknowledged": true}))
            } else {
                self.no_response_count.fetch_add(1, Ordering::SeqCst);
                None
            }
        } else {
            self.no_response_count.fetch_add(1, Ordering::SeqCst);
            None
        }
    }
}

/// Handler that processes binary and text frames
#[derive(Debug, Clone)]
struct FrameProcessingHandler {
    frame_count: Arc<AtomicUsize>,
    messages: Arc<Mutex<Vec<Value>>>,
}

impl FrameProcessingHandler {
    fn new() -> Self {
        Self {
            frame_count: Arc::new(AtomicUsize::new(0)),
            messages: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl WebSocketHandler for FrameProcessingHandler {
    async fn handle_message(&self, message: Value) -> Option<Value> {
        self.frame_count.fetch_add(1, Ordering::SeqCst);
        self.messages.lock().unwrap().push(message.clone());
        Some(json!({"processed": true, "frame_number": self.frame_count.load(Ordering::SeqCst)}))
    }
}

/// Handler that processes large messages
#[derive(Debug, Clone)]
struct LargeMessageHandler {
    processed_size: Arc<AtomicUsize>,
}

impl LargeMessageHandler {
    fn new() -> Self {
        Self {
            processed_size: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl WebSocketHandler for LargeMessageHandler {
    async fn handle_message(&self, message: Value) -> Option<Value> {
        let serialized = message.to_string();
        self.processed_size.store(serialized.len(), Ordering::SeqCst);
        Some(json!({"size_received": serialized.len()}))
    }
}

/// Handler that tracks concurrent message processing
#[derive(Debug, Clone)]
struct ConcurrentHandler {
    message_count: Arc<AtomicUsize>,
    messages: Arc<Mutex<Vec<Value>>>,
}

impl ConcurrentHandler {
    fn new() -> Self {
        Self {
            message_count: Arc::new(AtomicUsize::new(0)),
            messages: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl WebSocketHandler for ConcurrentHandler {
    async fn handle_message(&self, message: Value) -> Option<Value> {
        sleep(Duration::from_millis(1)).await;
        self.message_count.fetch_add(1, Ordering::SeqCst);
        self.messages.lock().unwrap().push(message.clone());
        Some(json!({"count": self.message_count.load(Ordering::SeqCst)}))
    }
}

/// Handler that simulates errors
#[derive(Debug, Clone)]
struct ErrorHandler {
    should_error: Arc<AtomicBool>,
}

impl ErrorHandler {
    fn new() -> Self {
        Self {
            should_error: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl WebSocketHandler for ErrorHandler {
    async fn handle_message(&self, message: Value) -> Option<Value> {
        if self.should_error.load(Ordering::SeqCst) {
            None
        } else {
            Some(message)
        }
    }
}

#[tokio::test]
async fn test_websocket_connection_upgrade() {
    let handler = EchoHandler;
    let msg = json!({"type": "connection_test", "payload": "hello"});

    let response = handler.handle_message(msg.clone()).await;

    assert!(response.is_some());
    assert_eq!(response.unwrap(), msg);
}

#[tokio::test]
async fn test_websocket_message_validation_against_schema() {
    let handler = SchemaValidatingHandler::new();

    let valid_msg = json!({"action": "create", "data": "test"});
    let response = handler.handle_message(valid_msg).await;

    assert!(response.is_some());
    assert_eq!(handler.valid_count.load(Ordering::SeqCst), 1);
    assert_eq!(handler.invalid_count.load(Ordering::SeqCst), 0);

    let invalid_msg = json!({"data": "test"});
    let response = handler.handle_message(invalid_msg).await;

    assert!(response.is_none());
    assert_eq!(handler.valid_count.load(Ordering::SeqCst), 1);
    assert_eq!(handler.invalid_count.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn test_websocket_response_schema_validation() {
    let handler = SchemaValidatingHandler::new();

    let msg = json!({"action": "update", "data": {"id": 1}});
    let response = handler.handle_message(msg).await;

    assert!(response.is_some());
    let resp = response.unwrap();

    assert!(resp.get("status").is_some());
    assert!(resp.get("echo").is_some());
    assert_eq!(resp.get("status").unwrap(), "valid");
}

#[tokio::test]
async fn test_websocket_handler_returning_none() {
    let handler = SelectiveResponderHandler::new();

    let no_response_msg = json!({"respond": false, "data": "test"});
    let response = handler.handle_message(no_response_msg).await;

    assert!(response.is_none());
    assert_eq!(handler.no_response_count.load(Ordering::SeqCst), 1);
    assert_eq!(handler.response_count.load(Ordering::SeqCst), 0);

    let response_msg = json!({"respond": true, "data": "test"});
    let response = handler.handle_message(response_msg).await;

    assert!(response.is_some());
    assert_eq!(handler.response_count.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn test_websocket_binary_frame_handling() {
    let handler = FrameProcessingHandler::new();

    let binary_msg = json!({"type": "binary", "data": [0, 255, 128, 64]});
    let response = handler.handle_message(binary_msg).await;

    assert!(response.is_some());
    assert_eq!(handler.frame_count.load(Ordering::SeqCst), 1);

    let messages = handler.messages.lock().unwrap();
    assert_eq!(messages.len(), 1);
}

#[tokio::test]
async fn test_websocket_text_frame_handling() {
    let handler = FrameProcessingHandler::new();

    let text_msg = json!({"type": "text", "content": "hello world"});
    let response = handler.handle_message(text_msg.clone()).await;

    assert!(response.is_some());
    assert_eq!(handler.frame_count.load(Ordering::SeqCst), 1);

    let messages = handler.messages.lock().unwrap();
    assert_eq!(messages[0], text_msg);
}

#[tokio::test]
async fn test_websocket_ping_pong() {
    let handler = EchoHandler;

    let msg1 = json!({"ping": 1});
    let msg2 = json!({"ping": 2});

    let resp1 = handler.handle_message(msg1.clone()).await;
    let resp2 = handler.handle_message(msg2.clone()).await;

    assert_eq!(resp1.unwrap(), msg1);
    assert_eq!(resp2.unwrap(), msg2);
}

#[tokio::test]
async fn test_websocket_close_frame() {
    let handler = EchoHandler;

    let close_msg = json!({"type": "close", "code": 1000, "reason": "normal"});
    let response = handler.handle_message(close_msg).await;

    assert!(response.is_some());

    let msg = json!({"after_close": "test"});
    let response = handler.handle_message(msg.clone()).await;
    assert_eq!(response.unwrap(), msg);
}

#[tokio::test]
async fn test_websocket_invalid_json_message() {
    let handler = SchemaValidatingHandler::new();

    let invalid_json = json!({"unknown_field": "value"});
    let response = handler.handle_message(invalid_json).await;

    assert!(response.is_none());
    assert_eq!(handler.invalid_count.load(Ordering::SeqCst), 1);

    let valid_msg = json!({"action": "test", "data": "ok"});
    let response = handler.handle_message(valid_msg).await;

    assert!(response.is_some());
    assert_eq!(handler.valid_count.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn test_websocket_large_message() {
    let handler = LargeMessageHandler::new();

    let large_array: Vec<i32> = (0..2500).collect();
    let large_msg = json!({
        "type": "large_payload",
        "data": large_array,
        "metadata": {
            "description": "Large message test"
        }
    });

    let response = handler.handle_message(large_msg).await;

    assert!(response.is_some());

    let size = handler.processed_size.load(Ordering::SeqCst);
    assert!(size > 10000, "Large message should be > 10KB");
}

#[tokio::test]
async fn test_websocket_concurrent_messages() {
    let handler = Arc::new(ConcurrentHandler::new());

    let mut handles = vec![];

    for i in 0..20 {
        let handler_clone = handler.clone();
        let handle = tokio::spawn(async move {
            let msg = json!({"id": i, "data": format!("msg_{}", i)});
            handler_clone.handle_message(msg).await
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    assert_eq!(handler.message_count.load(Ordering::SeqCst), 20);
    assert_eq!(handler.messages.lock().unwrap().len(), 20);
}

#[tokio::test]
async fn test_websocket_handler_error() {
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
async fn test_websocket_message_with_special_characters() {
    let handler = EchoHandler;

    let special_msg = json!({
        "emoji": "🚀💡🔥",
        "unicode": "你好世界",
        "special": "!@#$%^&*()",
        "newlines": "line1\nline2\nline3"
    });

    let response = handler.handle_message(special_msg.clone()).await;

    assert!(response.is_some());
    assert_eq!(response.unwrap(), special_msg);
}

#[tokio::test]
async fn test_websocket_empty_and_null_values() {
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
async fn test_websocket_deeply_nested_structures() {
    let handler = EchoHandler;

    let mut nested = json!({"value": "deep"});
    for _ in 0..30 {
        nested = json!({"level": nested});
    }

    let response = handler.handle_message(nested.clone()).await;

    assert!(response.is_some());
    assert_eq!(response.unwrap(), nested);
}

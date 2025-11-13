//! Tests for the WebSocket test client functionality.
//!
//! These tests verify that the WebSocket test client works correctly,
//! including sending/receiving messages, JSON handling, and connection lifecycle.

use axum::{
    Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    routing::get,
};
use axum_test::{TestServer, TestServerConfig, Transport};
use serde_json::json;
use spikard_http::testing::{WebSocketMessage, connect_websocket};

/// Create test server config with HTTP transport (required for WebSockets)
fn ws_test_config() -> TestServerConfig {
    TestServerConfig {
        transport: Some(Transport::HttpRandomPort),
        ..TestServerConfig::default()
    }
}

/// Helper to create a simple echo WebSocket server
fn echo_app() -> Router {
    Router::new().route(
        "/echo",
        get(|ws: WebSocketUpgrade| async move { ws.on_upgrade(echo_handler) }),
    )
}

async fn echo_handler(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                if socket.send(Message::Text(text)).await.is_err() {
                    break;
                }
            }
            Message::Binary(data) => {
                if socket.send(Message::Binary(data)).await.is_err() {
                    break;
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }
}

/// Helper to create a JSON-echo WebSocket server
fn json_echo_app() -> Router {
    Router::new().route(
        "/json-echo",
        get(|ws: WebSocketUpgrade| async move { ws.on_upgrade(json_echo_handler) }),
    )
}

async fn json_echo_handler(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            // Parse JSON, add "echoed": true, and send back
            if let Ok(mut value) = serde_json::from_str::<serde_json::Value>(&text) {
                if let Some(obj) = value.as_object_mut() {
                    obj.insert("echoed".to_string(), json!(true));
                }
                let response = serde_json::to_string(&value).unwrap();
                if socket.send(Message::Text(response.into())).await.is_err() {
                    break;
                }
            }
        }
    }
}

/// Helper to create a multi-message WebSocket server
fn multi_message_app() -> Router {
    Router::new().route(
        "/multi",
        get(|ws: WebSocketUpgrade| async move { ws.on_upgrade(multi_message_handler) }),
    )
}

async fn multi_message_handler(mut socket: WebSocket) {
    // Wait for client to connect, then send multiple messages
    if let Some(Ok(Message::Text(text))) = socket.recv().await {
        if text == "start" {
            for i in 1..=5 {
                let msg = json!({"sequence": i, "message": format!("Message {}", i)});
                if socket
                    .send(Message::Text(serde_json::to_string(&msg).unwrap().into()))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        }
    }
}

#[tokio::test]
async fn test_websocket_connect() {
    let server = TestServer::new_with_config(echo_app(), ws_test_config()).unwrap();
    let ws = connect_websocket(&server, "/echo").await;
    ws.close().await;
}

#[tokio::test]
async fn test_websocket_send_receive_text() {
    let server = TestServer::new_with_config(echo_app(), ws_test_config()).unwrap();
    let mut ws = connect_websocket(&server, "/echo").await;

    ws.send_text("Hello, WebSocket!").await;
    let response = ws.receive_text().await;

    assert_eq!(response, "Hello, WebSocket!");
    ws.close().await;
}

#[tokio::test]
async fn test_websocket_send_receive_json() {
    let server = TestServer::new_with_config(json_echo_app(), ws_test_config()).unwrap();
    let mut ws = connect_websocket(&server, "/json-echo").await;

    let message = json!({"type": "greeting", "text": "Hello"});
    ws.send_json(&message).await;

    let response: serde_json::Value = ws.receive_json().await;

    assert_eq!(response["type"], "greeting");
    assert_eq!(response["text"], "Hello");
    assert_eq!(response["echoed"], true);

    ws.close().await;
}

#[tokio::test]
async fn test_websocket_receive_message_text() {
    let server = TestServer::new_with_config(echo_app(), ws_test_config()).unwrap();
    let mut ws = connect_websocket(&server, "/echo").await;

    ws.send_text("Test message").await;
    let msg = ws.receive_message().await;

    match msg {
        WebSocketMessage::Text(text) => {
            assert_eq!(text, "Test message");
        }
        _ => panic!("Expected text message"),
    }

    ws.close().await;
}

#[tokio::test]
async fn test_websocket_message_as_json() {
    let server = TestServer::new_with_config(echo_app(), ws_test_config()).unwrap();
    let mut ws = connect_websocket(&server, "/echo").await;

    let json_str = r#"{"status":"ok","count":42}"#;
    ws.send_text(json_str).await;
    let msg = ws.receive_message().await;

    let json_value = msg.as_json().expect("Should parse as JSON");
    assert_eq!(json_value["status"], "ok");
    assert_eq!(json_value["count"], 42);

    ws.close().await;
}

#[tokio::test]
async fn test_websocket_multiple_messages() {
    let server = TestServer::new_with_config(multi_message_app(), ws_test_config()).unwrap();
    let mut ws = connect_websocket(&server, "/multi").await;

    ws.send_text("start").await;

    // Receive 5 messages
    for i in 1..=5 {
        let msg: serde_json::Value = ws.receive_json().await;
        assert_eq!(msg["sequence"], i);
        assert_eq!(msg["message"], format!("Message {}", i));
    }

    ws.close().await;
}

#[tokio::test]
async fn test_websocket_send_receive_empty_string() {
    let server = TestServer::new_with_config(echo_app(), ws_test_config()).unwrap();
    let mut ws = connect_websocket(&server, "/echo").await;

    ws.send_text("").await;
    let response = ws.receive_text().await;

    assert_eq!(response, "");
    ws.close().await;
}

#[tokio::test]
async fn test_websocket_send_receive_unicode() {
    let server = TestServer::new_with_config(echo_app(), ws_test_config()).unwrap();
    let mut ws = connect_websocket(&server, "/echo").await;

    let unicode_text = "Hello 世界 🌍 مرحبا";
    ws.send_text(unicode_text).await;
    let response = ws.receive_text().await;

    assert_eq!(response, unicode_text);
    ws.close().await;
}

#[tokio::test]
async fn test_websocket_json_with_special_characters() {
    let server = TestServer::new_with_config(json_echo_app(), ws_test_config()).unwrap();
    let mut ws = connect_websocket(&server, "/json-echo").await;

    let message = json!({
        "text": "Hello \"world\" with 'quotes'",
        "newlines": "Line 1\nLine 2\nLine 3",
        "unicode": "世界 🌍"
    });

    ws.send_json(&message).await;
    let response: serde_json::Value = ws.receive_json().await;

    assert_eq!(response["text"], "Hello \"world\" with 'quotes'");
    assert_eq!(response["newlines"], "Line 1\nLine 2\nLine 3");
    assert_eq!(response["unicode"], "世界 🌍");
    assert_eq!(response["echoed"], true);

    ws.close().await;
}

#[tokio::test]
async fn test_websocket_message_type_detection() {
    let server = TestServer::new_with_config(echo_app(), ws_test_config()).unwrap();
    let mut ws = connect_websocket(&server, "/echo").await;

    ws.send_text("text message").await;
    let msg = ws.receive_message().await;

    assert!(msg.as_text().is_some());
    assert!(msg.as_binary().is_none());
    assert!(!msg.is_close());

    ws.close().await;
}

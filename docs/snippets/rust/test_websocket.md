```rust
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};
use axum_test::TestServer;
use futures::{SinkExt, StreamExt};

async fn echo_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.next().await {
        if let Message::Text(text) = msg {
            if socket.send(Message::Text(text)).await.is_err() {
                break;
            }
        }
    }
}

#[tokio::test]
async fn test_websocket_echo() {
    let app = Router::new().route("/echo", get(echo_handler));
    let server = TestServer::new(app).expect("failed to create test server");

    let mut ws = server.into_websocket("/echo").await;

    // Send text message
    ws.send(Message::Text("Hello".into())).await.expect("failed to send message");
    let response = ws.next().await.expect("failed to receive message").expect("received empty message");
    assert_eq!(response, Message::Text("Hello".into()));

    // Send JSON message
    let json_msg = serde_json::json!({"type": "ping"}).to_string();
    ws.send(Message::Text(json_msg.clone())).await.expect("failed to send message");
    let response = ws.next().await.expect("failed to receive message").expect("received empty message");
    assert_eq!(response, Message::Text(json_msg));
}
```

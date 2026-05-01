use axum::routing::get;
use serde_json::Value;
use spikard_http::testing::{WebSocketMessage, connect_websocket};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::time::{Duration, timeout};

/// Simple echo handler that also tracks disconnect.
struct EchoHandler {
    disconnected: Arc<AtomicBool>,
}

impl spikard_http::WebSocketHandler for EchoHandler {
    async fn handle_message(&self, message: Value) -> Option<Value> {
        Some(message)
    }

    fn on_disconnect(&self) -> impl std::future::Future<Output = ()> + Send {
        let flag = Arc::clone(&self.disconnected);
        async move {
            flag.store(true, Ordering::SeqCst);
        }
    }
}

/// Build a test server with a basic echo WebSocket handler at "/ws".
fn echo_server(disconnected: Arc<AtomicBool>) -> axum_test::TestServer {
    let state = spikard_http::WebSocketState::new(EchoHandler { disconnected });
    let app = axum::Router::new()
        .route("/ws", get(spikard_http::websocket_handler::<EchoHandler>))
        .with_state(state);

    axum_test::TestServer::new_with_config(
        app,
        axum_test::TestServerConfig {
            transport: Some(axum_test::Transport::HttpRandomPort),
            ..axum_test::TestServerConfig::default()
        },
    )
}

/// Test: `close_with(1001, Some("going away"))` triggers on_disconnect.
#[tokio::test]
async fn close_with_going_away_triggers_on_disconnect() {
    let disconnected = Arc::new(AtomicBool::new(false));
    let server = echo_server(Arc::clone(&disconnected));

    let conn = connect_websocket(&server, "/ws").await;
    conn.close_with(1001, Some("going away".to_string()))
        .await
        .expect("close_with should not fail");

    // Give the server time to process the close and call on_disconnect.
    let result = timeout(Duration::from_secs(2), async {
        loop {
            if disconnected.load(Ordering::SeqCst) {
                return;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await;

    assert!(result.is_ok(), "on_disconnect was not called after close_with(1001)");
}

/// Test: default `close()` sends code 1000 (Normal Closure) and triggers disconnect.
#[tokio::test]
async fn close_default_triggers_on_disconnect() {
    let disconnected = Arc::new(AtomicBool::new(false));
    let server = echo_server(Arc::clone(&disconnected));

    let conn = connect_websocket(&server, "/ws").await;
    conn.close().await.expect("close should not fail");

    let result = timeout(Duration::from_secs(2), async {
        loop {
            if disconnected.load(Ordering::SeqCst) {
                return;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await;

    assert!(result.is_ok(), "on_disconnect was not called after close()");
}

/// Test: `Message::Close(None)` from server maps to client-side `Close { code: 1005, reason: None }`.
#[tokio::test]
async fn server_close_without_frame_maps_to_code_1005() {
    // Create a server that immediately sends Close(None) after upgrading.
    let app = axum::Router::new().route(
        "/ws",
        get(|ws: axum::extract::ws::WebSocketUpgrade| async move {
            ws.on_upgrade(|mut socket| async move {
                // Send a close frame with no code/reason.
                let _ = socket.send(axum::extract::ws::Message::Close(None)).await;
            })
        }),
    );

    let server = axum_test::TestServer::new_with_config(
        app,
        axum_test::TestServerConfig {
            transport: Some(axum_test::Transport::HttpRandomPort),
            ..axum_test::TestServerConfig::default()
        },
    );

    let mut conn = connect_websocket(&server, "/ws").await;
    let msg = conn.receive_message().await;

    assert!(msg.is_close(), "expected close message");
    assert_eq!(msg.close_code(), Some(1005), "Close(None) should map to code 1005");
    assert_eq!(msg.close_reason(), None, "Close(None) should have no reason");
}

/// Test: server close with code 1000 and reason arrives correctly.
#[tokio::test]
async fn server_close_with_code_1000_round_trips() {
    use axum::extract::ws::{CloseFrame, Utf8Bytes};

    let app = axum::Router::new().route(
        "/ws",
        get(|ws: axum::extract::ws::WebSocketUpgrade| async move {
            ws.on_upgrade(|mut socket| async move {
                let _ = socket
                    .send(axum::extract::ws::Message::Close(Some(CloseFrame {
                        code: 1000,
                        reason: Utf8Bytes::from("bye"),
                    })))
                    .await;
            })
        }),
    );

    let server = axum_test::TestServer::new_with_config(
        app,
        axum_test::TestServerConfig {
            transport: Some(axum_test::Transport::HttpRandomPort),
            ..axum_test::TestServerConfig::default()
        },
    );

    let mut conn = connect_websocket(&server, "/ws").await;
    let msg = conn.receive_message().await;

    assert!(msg.is_close(), "expected close message");
    assert_eq!(msg.close_code(), Some(1000));
    assert_eq!(msg.close_reason(), Some("bye"));
}

/// Test: `WebSocketMessage` helpers work correctly.
#[test]
fn websocket_message_close_code_and_reason_helpers() {
    let close_with_reason = WebSocketMessage::Close {
        code: 1001,
        reason: Some("going away".to_string()),
    };
    assert!(close_with_reason.is_close());
    assert_eq!(close_with_reason.close_code(), Some(1001));
    assert_eq!(close_with_reason.close_reason(), Some("going away"));
    assert!(close_with_reason.as_text().is_none());

    let close_no_reason = WebSocketMessage::Close { code: 1005, reason: None };
    assert!(close_no_reason.is_close());
    assert_eq!(close_no_reason.close_code(), Some(1005));
    assert_eq!(close_no_reason.close_reason(), None);

    let text_msg = WebSocketMessage::Text("hello".to_string());
    assert!(!text_msg.is_close());
    assert_eq!(text_msg.close_code(), None);
    assert_eq!(text_msg.close_reason(), None);
}

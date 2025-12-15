use axum::routing::get;
use serde_json::Value;
use spikard_http::{WebSocketHandler, WebSocketState, websocket_handler};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::time::{Duration, timeout};

#[derive(Debug)]
struct CountingEchoHandler {
    connects: Arc<AtomicUsize>,
    disconnects: Arc<AtomicUsize>,
}

impl CountingEchoHandler {
    const fn new(connects: Arc<AtomicUsize>, disconnects: Arc<AtomicUsize>) -> Self {
        Self { connects, disconnects }
    }
}

impl WebSocketHandler for CountingEchoHandler {
    async fn handle_message(&self, message: Value) -> Option<Value> {
        Some(message)
    }

    fn on_connect(&self) -> impl std::future::Future<Output = ()> + Send {
        let connects = Arc::clone(&self.connects);
        async move {
            connects.fetch_add(1, Ordering::SeqCst);
        }
    }

    fn on_disconnect(&self) -> impl std::future::Future<Output = ()> + Send {
        let disconnects = Arc::clone(&self.disconnects);
        async move {
            disconnects.fetch_add(1, Ordering::SeqCst);
        }
    }
}

#[tokio::test]
async fn websocket_handles_json_validation_invalid_json_binary_and_close() {
    let connects = Arc::new(AtomicUsize::new(0));
    let disconnects = Arc::new(AtomicUsize::new(0));

    let message_schema = serde_json::json!({
        "type": "object",
        "properties": {
            "ok": { "type": "boolean" }
        },
        "required": ["ok"]
    });

    let state = WebSocketState::with_schemas(
        CountingEchoHandler::new(Arc::clone(&connects), Arc::clone(&disconnects)),
        Some(message_schema),
        None,
    )
    .expect("state");

    let app = axum::Router::new()
        .route("/ws", get(websocket_handler::<CountingEchoHandler>))
        .with_state(state);

    let server = axum_test::TestServer::new_with_config(
        app,
        axum_test::TestServerConfig {
            transport: Some(axum_test::Transport::HttpRandomPort),
            ..axum_test::TestServerConfig::default()
        },
    )
    .expect("server");
    let mut socket = server.get_websocket("/ws").await.into_websocket().await;

    assert_eq!(connects.load(Ordering::SeqCst), 1);

    socket.send_json(&serde_json::json!({"ok": true})).await;
    let echoed: Value = socket.receive_json().await;
    assert_eq!(echoed, serde_json::json!({"ok": true}));

    socket.send_text("{").await;
    let invalid_json: Value = socket.receive_json().await;
    assert_eq!(invalid_json["type"], "error");
    assert_eq!(invalid_json["message"], "Invalid JSON");

    socket.send_json(&serde_json::json!({"nope": true})).await;
    let validation_error: Value = socket.receive_json().await;
    assert_eq!(validation_error["error"], "Message validation failed");

    socket
        .send_message(axum_test::WsMessage::Binary(bytes::Bytes::from_static(b"bin")))
        .await;
    let received = socket.receive_bytes().await;
    assert_eq!(&received[..], b"bin");

    socket
        .send_message(axum_test::WsMessage::Ping(bytes::Bytes::from_static(b"ping")))
        .await;
    let pong = socket.receive_message().await;
    assert!(matches!(pong, axum_test::WsMessage::Pong(_)));

    socket.close().await;

    timeout(Duration::from_secs(1), async {
        loop {
            if disconnects.load(Ordering::SeqCst) == 1 {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("disconnect");
}

#[derive(Debug)]
struct AlwaysObjectHandler;

impl WebSocketHandler for AlwaysObjectHandler {
    async fn handle_message(&self, _message: Value) -> Option<Value> {
        Some(serde_json::json!({"response": "object"}))
    }
}

#[tokio::test]
async fn websocket_suppresses_responses_that_fail_response_schema_validation() {
    let response_schema = serde_json::json!({ "type": "string" });
    let state = WebSocketState::with_schemas(AlwaysObjectHandler, None, Some(response_schema)).expect("state");

    let app = axum::Router::new()
        .route("/ws", get(websocket_handler::<AlwaysObjectHandler>))
        .with_state(state);

    let server = axum_test::TestServer::new_with_config(
        app,
        axum_test::TestServerConfig {
            transport: Some(axum_test::Transport::HttpRandomPort),
            ..axum_test::TestServerConfig::default()
        },
    )
    .expect("server");
    let mut socket = server.get_websocket("/ws").await.into_websocket().await;

    socket.send_json(&serde_json::json!({"any": "thing"})).await;

    let no_response = timeout(Duration::from_millis(100), socket.receive_message()).await;
    assert!(
        no_response.is_err(),
        "response should be suppressed by schema validation"
    );

    socket.close().await;
}

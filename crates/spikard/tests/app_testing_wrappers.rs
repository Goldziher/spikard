use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::routing::get;
use serde_json::Value;
use spikard::testing::{SseStream, TestServer, WebSocketMessage};
use spikard::{App, get as spikard_get};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
struct EchoWs;

impl spikard::WebSocketHandler for EchoWs {
    async fn handle_message(&self, message: Value) -> Option<Value> {
        Some(message)
    }
}

#[derive(Debug)]
struct CounterSse {
    count: Arc<AtomicUsize>,
    limit: usize,
}

impl CounterSse {
    fn new(limit: usize) -> Self {
        Self {
            count: Arc::new(AtomicUsize::new(0)),
            limit,
        }
    }
}

impl spikard::SseEventProducer for CounterSse {
    fn next_event(&self) -> impl std::future::Future<Output = Option<spikard::SseEvent>> + Send {
        let count = Arc::clone(&self.count);
        let limit = self.limit;
        async move {
            let current = count.fetch_add(1, Ordering::SeqCst);
            if current >= limit {
                None
            } else {
                Some(spikard::SseEvent::new(serde_json::json!({"count": current})))
            }
        }
    }
}

#[tokio::test]
async fn app_testing_server_supports_http_sse_and_websocket_helpers() {
    let mut app = App::new();

    app.route(spikard_get("hello"), |_ctx| async move {
        Ok(axum::http::Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("ok"))
            .expect("response"))
    })
    .expect("route");

    app.sse("events", CounterSse::new(2));
    app.websocket("ws", EchoWs);

    let router = app
        .merge_axum_router(axum::Router::new().route("/extra", get(|| async { "extra" })))
        .into_router()
        .expect("router");

    let server = TestServer::from_router(router).expect("server");

    let snapshot = server
        .call(Request::builder().uri("/hello").body(Body::empty()).expect("req"))
        .await
        .expect("snapshot");
    assert_eq!(snapshot.status, 200);
    assert_eq!(snapshot.text().expect("text"), "ok");

    let sse_snapshot = server
        .call(Request::builder().uri("/events").body(Body::empty()).expect("req"))
        .await
        .expect("snapshot");
    let stream = SseStream::from_response(&sse_snapshot).expect("sse");
    assert_eq!(stream.events().len(), 2);
    assert_eq!(stream.events_as_json().expect("json")[0]["count"], 0);

    let extra = server
        .call(Request::builder().uri("/extra").body(Body::empty()).expect("req"))
        .await
        .expect("snapshot");
    assert_eq!(extra.text().expect("text"), "extra");

    let mut ws = server.connect_websocket("/ws").await;
    ws.send_json(&serde_json::json!({"hello": "ws"})).await;
    let msg = ws.receive_message().await;
    assert_eq!(msg.as_json().expect("json")["hello"], "ws");
    assert!(!msg.is_close());
    assert!(matches!(msg, WebSocketMessage::Text(_)));
    ws.close().await;
}

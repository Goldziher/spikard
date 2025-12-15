use axum::{Router, routing::get};
use http_body_util::BodyExt;
use serde_json::json;
use spikard_http::sse::{SseEvent, SseEventProducer, SseState, sse_handler};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::time::timeout;
use tower::ServiceExt;

struct CountingProducer {
    connect: Arc<AtomicUsize>,
    disconnect: Arc<AtomicUsize>,
    remaining: AtomicUsize,
}

impl CountingProducer {
    const fn new(connect: Arc<AtomicUsize>, disconnect: Arc<AtomicUsize>, events: usize) -> Self {
        Self {
            connect,
            disconnect,
            remaining: AtomicUsize::new(events),
        }
    }
}

impl SseEventProducer for CountingProducer {
    async fn next_event(&self) -> Option<SseEvent> {
        let prev = self.remaining.fetch_sub(1, Ordering::Relaxed);
        (prev > 0).then(|| SseEvent::new(json!({"message": "ok"})))
    }

    async fn on_connect(&self) {
        self.connect.fetch_add(1, Ordering::Relaxed);
    }

    async fn on_disconnect(&self) {
        self.disconnect.fetch_add(1, Ordering::Relaxed);
    }
}

#[tokio::test]
async fn sse_handler_invokes_connect_and_disconnect_per_request() {
    let connect = Arc::new(AtomicUsize::new(0));
    let disconnect = Arc::new(AtomicUsize::new(0));

    let state = SseState::new(CountingProducer::new(Arc::clone(&connect), Arc::clone(&disconnect), 1));
    let app = Router::new()
        .route("/events", get(sse_handler::<CountingProducer>))
        .with_state(state);

    let response = app
        .oneshot(
            axum::http::Request::builder()
                .uri("/events")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let bytes = timeout(std::time::Duration::from_secs(5), response.into_body().collect())
        .await
        .expect("response body collection timed out")
        .unwrap()
        .to_bytes();
    let body = String::from_utf8_lossy(&bytes);

    assert!(body.contains("data:"), "expected SSE data frame, got: {body}");
    assert_eq!(connect.load(Ordering::Relaxed), 1);
    assert_eq!(disconnect.load(Ordering::Relaxed), 1);
}

#[tokio::test]
async fn sse_handler_emits_validation_error_when_schema_rejects_event() {
    use std::sync::atomic::AtomicBool;

    struct InvalidEventProducer {
        sent: AtomicBool,
    }

    impl SseEventProducer for InvalidEventProducer {
        async fn next_event(&self) -> Option<SseEvent> {
            if self.sent.swap(true, Ordering::Relaxed) {
                None
            } else {
                Some(SseEvent::new(json!({"count": "not-an-integer"})))
            }
        }
    }

    let schema = json!({
        "type": "object",
        "properties": {
            "count": {"type": "integer"}
        },
        "required": ["count"]
    });
    let state = SseState::with_schema(
        InvalidEventProducer {
            sent: AtomicBool::new(false),
        },
        Some(schema),
    )
    .expect("valid schema");
    let app = Router::new()
        .route("/events", get(sse_handler::<InvalidEventProducer>))
        .with_state(state);

    let response = app
        .oneshot(
            axum::http::Request::builder()
                .uri("/events")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let bytes = timeout(std::time::Duration::from_secs(5), response.into_body().collect())
        .await
        .expect("response body collection timed out")
        .unwrap()
        .to_bytes();
    let body = String::from_utf8_lossy(&bytes);

    assert!(
        body.contains("validation_error"),
        "expected validation_error frame, got: {body}"
    );
}

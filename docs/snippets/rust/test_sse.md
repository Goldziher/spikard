```rust
use axum::{
    response::sse::{Event, Sse},
    routing::get,
    Router,
};
use axum_test::TestServer;
use futures::stream::{self, Stream};
use serde_json::json;
use std::convert::Infallible;

fn notifications() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::iter(0..3).map(|i| {
        Ok(Event::default().data(json!({"count": i}).to_string()))
    });
    Sse::new(stream)
}

#[tokio::test]
async fn test_sse_stream() {
    let app = Router::new().route("/notifications", get(notifications));
    let server = TestServer::new(app).unwrap();

    let response = server.get("/notifications").await;
    assert_eq!(response.status_code(), 200);

    // Verify content type
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert!(content_type.contains("text/event-stream"));

    // Parse SSE events from response body
    let body = response.text();
    let events: Vec<&str> = body
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .collect();

    assert_eq!(events.len(), 3);
    assert!(events[0].contains(r#"{"count":0}"#));
    assert!(events[2].contains(r#"{"count":2}"#));
}
```

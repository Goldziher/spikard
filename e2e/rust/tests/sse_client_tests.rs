//! Tests for the SSE test client functionality.
//!
//! These tests verify that the SSE test client correctly parses
//! Server-Sent Events streams and extracts JSON data.

use axum::{Router, response::Sse, response::sse::Event, routing::get};
use axum_test::TestServer;
use futures_util::stream::{self, Stream};
use serde_json::json;
use spikard_http::testing::{SseStream, snapshot_response};
use std::convert::Infallible;

/// Helper to create a simple SSE server that sends a few events
fn simple_sse_app() -> Router {
    Router::new().route("/events", get(simple_sse_handler))
}

async fn simple_sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::iter(vec![
        Ok(Event::default().data("event 1")),
        Ok(Event::default().data("event 2")),
        Ok(Event::default().data("event 3")),
    ]);

    Sse::new(stream)
}

/// Helper to create a JSON SSE server
fn json_sse_app() -> Router {
    Router::new().route("/json-events", get(json_sse_handler))
}

async fn json_sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let events = vec![
        json!({"type": "message", "id": 1, "text": "First message"}),
        json!({"type": "message", "id": 2, "text": "Second message"}),
        json!({"type": "message", "id": 3, "text": "Third message"}),
    ];

    let stream = stream::iter(
        events
            .into_iter()
            .map(|event| Ok(Event::default().data(serde_json::to_string(&event).unwrap()))),
    );

    Sse::new(stream)
}

/// Helper to create an SSE server with complex JSON
fn complex_json_sse_app() -> Router {
    Router::new().route("/complex", get(complex_json_sse_handler))
}

async fn complex_json_sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let events = vec![
        json!({
            "type": "user_joined",
            "user": {"id": 123, "name": "Alice"},
            "timestamp": "2024-01-15T10:30:00Z"
        }),
        json!({
            "type": "message",
            "user": {"id": 123, "name": "Alice"},
            "text": "Hello everyone!",
            "metadata": {"edited": false, "reactions": []}
        }),
    ];

    let stream = stream::iter(
        events
            .into_iter()
            .map(|event| Ok(Event::default().data(serde_json::to_string(&event).unwrap()))),
    );

    Sse::new(stream)
}

/// Helper to create an SSE server with empty events
fn empty_events_sse_app() -> Router {
    Router::new().route("/empty", get(empty_events_sse_handler))
}

async fn empty_events_sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::iter(vec![
        Ok(Event::default().data("")),
        Ok(Event::default().data("non-empty")),
        Ok(Event::default().data("")),
    ]);

    Sse::new(stream)
}

#[tokio::test]
async fn test_sse_parse_simple_events() {
    let server = TestServer::new(simple_sse_app()).unwrap();
    let axum_response = server.get("/events").await;
    let snapshot = snapshot_response(axum_response).await.unwrap();

    let sse = SseStream::from_response(&snapshot).expect("Should parse SSE");
    let events = sse.events();

    assert_eq!(events.len(), 3);
    assert_eq!(events[0].data, "event 1");
    assert_eq!(events[1].data, "event 2");
    assert_eq!(events[2].data, "event 3");
}

#[tokio::test]
async fn test_sse_parse_json_events() {
    let server = TestServer::new(json_sse_app()).unwrap();
    let axum_response = server.get("/json-events").await;
    let snapshot = snapshot_response(axum_response).await.unwrap();

    let sse = SseStream::from_response(&snapshot).expect("Should parse SSE");
    let json_events = sse.events_as_json().expect("Should parse all as JSON");

    assert_eq!(json_events.len(), 3);
    assert_eq!(json_events[0]["type"], "message");
    assert_eq!(json_events[0]["id"], 1);
    assert_eq!(json_events[0]["text"], "First message");

    assert_eq!(json_events[1]["id"], 2);
    assert_eq!(json_events[2]["id"], 3);
}

#[tokio::test]
async fn test_sse_parse_complex_json() {
    let server = TestServer::new(complex_json_sse_app()).unwrap();
    let axum_response = server.get("/complex").await;
    let snapshot = snapshot_response(axum_response).await.unwrap();

    let sse = SseStream::from_response(&snapshot).expect("Should parse SSE");
    let json_events = sse.events_as_json().expect("Should parse all as JSON");

    assert_eq!(json_events.len(), 2);

    // First event - user joined
    assert_eq!(json_events[0]["type"], "user_joined");
    assert_eq!(json_events[0]["user"]["id"], 123);
    assert_eq!(json_events[0]["user"]["name"], "Alice");

    // Second event - message
    assert_eq!(json_events[1]["type"], "message");
    assert_eq!(json_events[1]["text"], "Hello everyone!");
    assert!(json_events[1]["metadata"]["reactions"].is_array());
}

#[tokio::test]
async fn test_sse_individual_event_json_parsing() {
    let server = TestServer::new(json_sse_app()).unwrap();
    let axum_response = server.get("/json-events").await;
    let snapshot = snapshot_response(axum_response).await.unwrap();

    let sse = SseStream::from_response(&snapshot).expect("Should parse SSE");
    let events = sse.events();

    // Parse each event individually
    for (i, event) in events.iter().enumerate() {
        let json = event.as_json().expect("Should parse as JSON");
        assert_eq!(json["id"], i + 1);
    }
}

#[tokio::test]
async fn test_sse_access_raw_body() {
    let server = TestServer::new(simple_sse_app()).unwrap();
    let axum_response = server.get("/events").await;
    let snapshot = snapshot_response(axum_response).await.unwrap();

    let sse = SseStream::from_response(&snapshot).expect("Should parse SSE");
    let body = sse.body();

    assert!(body.contains("data:"));
    assert!(body.contains("event 1"));
    assert!(body.contains("event 2"));
    assert!(body.contains("event 3"));
}

#[tokio::test]
async fn test_sse_empty_events() {
    let server = TestServer::new(empty_events_sse_app()).unwrap();
    let axum_response = server.get("/empty").await;
    let snapshot = snapshot_response(axum_response).await.unwrap();

    let sse = SseStream::from_response(&snapshot).expect("Should parse SSE");
    let events = sse.events();

    // Note: axum's SSE implementation filters out empty data events,
    // which is correct according to the SSE spec. So we only get the non-empty event.
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].data, "non-empty");
}

#[tokio::test]
async fn test_sse_event_order_preserved() {
    let server = TestServer::new(json_sse_app()).unwrap();
    let axum_response = server.get("/json-events").await;
    let snapshot = snapshot_response(axum_response).await.unwrap();

    let sse = SseStream::from_response(&snapshot).expect("Should parse SSE");
    let json_events = sse.events_as_json().expect("Should parse all as JSON");

    // Verify events are in order
    for (i, event) in json_events.iter().enumerate() {
        assert_eq!(event["id"], i + 1);
    }
}

#[tokio::test]
async fn test_sse_json_with_special_characters() {
    let app = Router::new().route(
        "/special",
        get(|| async {
            let events = vec![json!({
                "text": "Hello \"world\" with 'quotes'",
                "newlines": "Line 1\nLine 2",
                "unicode": "世界 🌍"
            })];

            let stream = stream::iter(
                events
                    .into_iter()
                    .map(|event| Ok::<_, Infallible>(Event::default().data(serde_json::to_string(&event).unwrap()))),
            );

            Sse::new(stream)
        }),
    );

    let server = TestServer::new(app).unwrap();
    let axum_response = server.get("/special").await;
    let snapshot = snapshot_response(axum_response).await.unwrap();

    let sse = SseStream::from_response(&snapshot).expect("Should parse SSE");
    let json_events = sse.events_as_json().expect("Should parse all as JSON");

    assert_eq!(json_events[0]["text"], "Hello \"world\" with 'quotes'");
    assert_eq!(json_events[0]["newlines"], "Line 1\nLine 2");
    assert_eq!(json_events[0]["unicode"], "世界 🌍");
}

#[tokio::test]
async fn test_sse_malformed_json_error() {
    let app = Router::new().route(
        "/malformed",
        get(|| async {
            let stream = stream::iter(vec![Ok::<_, Infallible>(Event::default().data("{invalid json}"))]);
            Sse::new(stream)
        }),
    );

    let server = TestServer::new(app).unwrap();
    let axum_response = server.get("/malformed").await;
    let snapshot = snapshot_response(axum_response).await.unwrap();

    let sse = SseStream::from_response(&snapshot).expect("Should parse SSE");

    // Should fail to parse as JSON
    let result = sse.events_as_json();
    assert!(result.is_err());
}

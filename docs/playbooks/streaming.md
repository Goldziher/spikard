# Streaming & Real-Time

Spikard supports streaming responses, Server-Sent Events (SSE), and WebSocket handlers with production-ready patterns.

## Streaming response (Rust)

SSE and WebSocket support are currently available in Rust's App API:

=== "Rust"

    ```rust
    use spikard::{App, SseEvent, SseEventProducer};

    struct MyProducer;

    impl SseEventProducer for MyProducer {
        async fn next_event(&self) -> Option<SseEvent> {
            // Generate and return an event
            Some(SseEvent::new(serde_json::json!({"data": "value"})))
        }
    }

    let mut app = App::new();
    app.sse("/events", MyProducer)?;
    app.run().await?;
    ```

Language binding support for SSE and WebSocket registration is in development.

## WebSocket Handler (Rust)

```rust
use spikard::{App, WebSocketHandler};
use serde_json::json;

struct EchoHandler;

impl WebSocketHandler for EchoHandler {
    async fn handle_message(&self, message: serde_json::Value) -> Option<serde_json::Value> {
        // Echo back the message
        Some(message)
    }
}

let mut app = App::new();
app.websocket("/ws", EchoHandler)?;
app.run().await?;
```

## Server-Sent Events (SSE) Handler (Rust)

```rust
use spikard::{App, SseEvent, SseEventProducer};
use tokio::time::sleep;
use std::time::Duration;

struct TimerProducer;

impl SseEventProducer for TimerProducer {
    async fn next_event(&self) -> Option<SseEvent> {
        sleep(Duration::from_secs(1)).await;
        Some(SseEvent::new(
            serde_json::json!({"timestamp": chrono::Utc::now()})
        ))
    }
}

let mut app = App::new();
app.sse("/events", TimerProducer)?;
app.run().await?;
```

## Testing Streaming Handlers

Test SSE and WebSocket handlers using the TestClient:

```rust
#[tokio::test]
async fn test_sse_stream() {
    let mut app = App::new();
    app.sse("/events", MyProducer)?;
    let router = app.into_router()?;

    let client = TestClient::with_router(router);
    
    // Test SSE connection
    let response = client.get("/events").await;
    assert_eq!(response.status_code, 200);
    assert!(response.headers.contains_key("content-type"));
}

#[tokio::test]
async fn test_websocket_handler() {
    let mut app = App::new();
    app.websocket("/ws", EchoHandler)?;
    let router = app.into_router()?;

    let client = TestClient::with_router(router);
    
    // Test WebSocket connection (upgrade handshake)
    let response = client.get("/ws").await;
    // Should get 101 Switching Protocols or similar
    assert!(response.status_code >= 100 && response.status_code < 200);
}
```

## Best practices

- Set appropriate content types (`application/x-ndjson`, `text/event-stream`)
- Handle client disconnects gracefully; stop producing when the connection closes
- Implement keepalive for long-lived connections to prevent proxy timeouts
- Use exponential backoff for retry logic
- Track active connections and clean up resources in finally blocks
- Add authentication middleware before handler execution
- Test disconnect scenarios and ensure proper cleanup
- Monitor memory usage with slow consumers and implement backpressure

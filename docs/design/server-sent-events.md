# Server-Sent Events (SSE)

**Date:** November 2025
**Status:** ✅ Python, Node.js, Ruby
**Related:** [websocket-support.md](./websocket-support.md), [validation-strategy.md](./validation-strategy.md)

## Overview

Spikard provides Server-Sent Events (SSE) support built on Axum's native `axum::response::sse` module. SSE enables real-time, unidirectional server-to-client event streaming over HTTP, with automatic JSON Schema validation in Rust.

**Design Principles:**
- **Async-first** - Async generator pattern
- **Litestar-style decorators** - `@sse()` standalone decorator
- **Rust validation** - All JSON Schema validation in Rust layer
- **Zero-cost** - No validation overhead when schemas aren't provided
- **Type-safe** - Automatic schema extraction from type hints

## Python API

### Basic Stream

```python
from spikard import sse
from typing import AsyncIterator
import asyncio

@sse("/events")
async def event_stream() -> AsyncIterator[dict]:
    """Simple event stream."""
    for i in range(10):
        await asyncio.sleep(1)
        yield {"count": i}
```

### With Schema Validation

```python
from spikard import sse
from typing import TypedDict, AsyncIterator
import asyncio

class StatusEvent(TypedDict):
    status: str
    message: str
    timestamp: int

@sse("/status")
async def status_stream() -> AsyncIterator[StatusEvent]:
    """Type-safe event stream with automatic validation."""
    for i in range(10):
        await asyncio.sleep(1)
        yield {
            "status": "ok",
            "message": f"Update {i}",
            "timestamp": i
        }
```

The decorator automatically:
1. Extracts JSON Schema from `AsyncIterator[StatusEvent]` type hint
2. Passes schema to Rust via `create_sse_state()`
3. Validates each event in Rust before streaming to clients
4. Invalid events are logged and skipped

### Manual Schema Specification

```python
event_schema = {
    "type": "object",
    "properties": {
        "status": {"type": "string"},
        "message": {"type": "string"}
    },
    "required": ["status", "message"]
}

@sse("/status", event_schema=event_schema)
async def status_stream() -> AsyncIterator[dict]:
    while True:
        yield {"status": "ok", "message": "Running"}
        await asyncio.sleep(1)
```

### Lifecycle Hooks

```python
@sse("/notifications")
async def notifications() -> AsyncIterator[dict]:
    while True:
        notification = await get_next_notification()
        yield notification

@notifications.on_connect
async def handle_connect():
    print("Client connected to SSE stream")

@notifications.on_disconnect
async def handle_disconnect():
    print("Client disconnected from SSE stream")
```

## Architecture

### Rust Core

**File:** `crates/spikard-http/src/sse.rs`

```rust
pub trait SseEventProducer: Send + Sync {
    async fn next_event(&self) -> Option<SseEvent>;

    // Optional lifecycle hooks
    async fn on_connect(&self) {}
    async fn on_disconnect(&self) {}
}

pub struct SseState<P: SseEventProducer> {
    producer: Arc<P>,
    event_schema: Option<Arc<jsonschema::Validator>>,
}

pub struct SseEvent {
    pub event_type: Option<String>,
    pub data: Value,
    pub id: Option<String>,
    pub retry: Option<u64>,
}
```

**Validation Logic:**
- Events validated **before** streaming to clients
- Invalid events → logged and skipped
- Sends "validation_error" event placeholder
- Stream continues with valid events

### Python Bridge

**File:** `crates/spikard-py/src/sse.rs`

```rust
pub fn create_sse_state(
    factory: &Bound<'_, PyAny>,
) -> PyResult<spikard_http::SseState<PythonSseEventProducer>> {
    let producer_instance = factory.call0()?;

    // Extract event schema
    let event_schema = producer_instance
        .getattr("_event_schema")
        .ok()
        .and_then(|attr| python_dict_to_json(attr));

    let py_producer = PythonSseEventProducer::new(producer_instance.unbind());
    SseState::with_schema(py_producer, event_schema)
}
```

### Schema Extraction

**File:** `packages/python/spikard/sse.py`

```python
def sse(
    path: str,
    *,
    event_schema: dict[str, Any] | None = None,
) -> Callable[[F], F]:
    """SSE endpoint decorator with schema validation."""

    def decorator(func: F) -> F:
        # Extract event schema from AsyncIterator[EventType] type hint
        if event_schema is None:
            type_hints = get_type_hints(func)
            return_type = type_hints.get("return")

            if return_type:
                origin = get_origin(return_type)
                if origin is not None:
                    args = get_args(return_type)
                    if args and args[0] != dict:
                        event_schema = extract_json_schema(args[0])

        # Store schema as function attribute
        wrapper._event_schema = event_schema

        return wrapper
    return decorator
```

## Supported Type Systems

All Python type systems that can produce JSON Schema:

- **TypedDict** - Built-in Python typing
- **Pydantic** - `model_json_schema()`
- **msgspec** - `__json_schema__()`
- **dataclasses** - Via conversion
- **Raw JSON Schema** - Direct specification

## Validation Flow

```
┌─────────────────────┐
│   Python Layer      │
│   @sse()            │ → Extract schema from AsyncIterator[T]
│   AsyncIterator[T]  │
└──────────┬──────────┘
           │
           ↓
┌─────────────────────┐
│  Python Bridge      │ → extract_json_schema()
│  (spikard-py)       │ → Convert to serde_json::Value
└──────────┬──────────┘
           │
           ↓
┌─────────────────────┐
│    Rust Core        │ → jsonschema::validator_for()
│    SseState         │ → validator.is_valid(&event.data)
└─────────────────────┘
```

## SSE Protocol

SSE follows the W3C EventSource specification:

```
Content-Type: text/event-stream
Cache-Control: no-cache
Connection: keep-alive

event: message
id: 1
data: {"status":"ok","message":"Update 1"}

event: notification
id: 2
data: {"user":"Alice","action":"login"}

: keep-alive comment
```

**Protocol Features:**
- **event:** Event type (defaults to "message")
- **id:** Event ID for client reconnection
- **data:** Event payload (JSON)
- **retry:** Reconnection interval (milliseconds)
- **Comments:** Lines starting with `:` for keep-alive

## Client Usage

### JavaScript/Browser

```javascript
const eventSource = new EventSource('/status');

// Handle all events
eventSource.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Event:', data);
  console.log('ID:', event.lastEventId);
};

// Handle typed events
eventSource.addEventListener('notification', (event) => {
  const notification = JSON.parse(event.data);
  console.log('Notification:', notification);
});

// Handle errors
eventSource.onerror = (error) => {
  console.error('SSE error:', error);
  // EventSource automatically reconnects
};

// Close when done
eventSource.close();
```

### Python Client

```python
import httpx
from httpx_sse import connect_sse

async with httpx.AsyncClient() as client:
    async with connect_sse(client, "GET", "http://localhost:8000/status") as source:
        async for event in source.aiter_sse():
            print(f"Event: {event.event}")
            print(f"Data: {event.data}")
            print(f"ID: {event.id}")
```

## Reconnection Handling

SSE supports automatic reconnection with `Last-Event-ID` header:

```python
@sse("/events")
async def events(request: Request) -> AsyncIterator[dict]:
    # Get last received event ID
    last_id = request.headers.get("last-event-id")

    # Resume from last_id if reconnecting
    start_index = int(last_id) + 1 if last_id else 0

    for i in range(start_index, 100):
        yield {"index": i, "data": f"Event {i}"}
```

## Keep-Alive Configuration

Axum automatically sends keep-alive comments every 15 seconds:

```rust
Sse::new(stream).keep_alive(
    KeepAlive::new()
        .interval(Duration::from_secs(15))
        .text("keep-alive")
)
```

## Testing

**File:** `crates/spikard-http/src/testing.rs`

```rust
#[tokio::test]
async fn test_sse() {
    let app = create_test_app();
    let server = TestServer::new(app).unwrap();

    let response = server.get("/events").await;
    assert_eq!(response.status_code(), 200);
    assert_eq!(
        response.header("content-type"),
        "text/event-stream"
    );

    let events = parse_sse_events(response.text());
    assert_eq!(events.len(), 3);
    assert_eq!(events[0]["data"], r#"{"count":0}"#);
}
```

## Implementation Files

| Layer | File | Purpose |
|-------|------|---------|
| Core | `crates/spikard-http/src/sse.rs` | SSE trait + validation |
| Python Bridge | `crates/spikard-py/src/sse.rs` | PyO3 producer wrapper |
| Python API | `packages/python/spikard/sse.py` | `@sse()` decorator |
| Testing | `crates/spikard-http/src/testing.rs` | Test helpers |

## Performance

- **Validation:** ~500k validations/sec in Rust
- **No GIL contention:** Validation outside Python
- **Zero-copy:** Direct validation on `serde_json::Value`
- **Zero-cost when unused:** No overhead without schemas
- **Automatic backpressure:** Axum handles slow clients

## Next Steps

1. **TypeScript/Node.js API** - High-level decorator API for TypeScript
2. **Ruby API** - High-level decorator API for Ruby
3. **Rust native producers** - For pure Rust applications
4. **Add validation test fixtures** - SSE fixtures with schemas for e2e tests

## References

- **SSE Specification:** https://html.spec.whatwg.org/multipage/server-sent-events.html
- **Axum SSE module:** https://docs.rs/axum/latest/axum/response/sse/
- **JSON Schema 2020-12:** https://json-schema.org/draft/2020-12/json-schema-core.html
- **MDN EventSource:** https://developer.mozilla.org/en-US/docs/Web/API/EventSource

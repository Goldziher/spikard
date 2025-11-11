# Server-Sent Events (SSE) Implementation Guide

**Date:** November 2025
**Status:** 🟡 Design Complete, Implementation Pending
**Related Docs:** [streaming-responses.md](./streaming-responses.md), [architecture.md](./architecture.md)

## Executive Summary

This document provides a concrete implementation plan for Server-Sent Events (SSE) in Spikard, leveraging Axum's native `axum::response::sse` module. SSE enables real-time, unidirectional server-to-client event streaming over HTTP, ideal for live updates, notifications, and event feeds.

**Key Design Principles:**
- ✅ Leverage Axum's `axum::response::sse` module for protocol compliance
- ✅ Support event types, IDs, and retry configuration per SSE spec
- ✅ Automatic keep-alive ping handling
- ✅ Automatic reconnection support with Last-Event-ID
- ✅ Idiomatic async generator/iterator APIs in all language bindings

## Overview

### Goals

1. Enable handlers to return SSE event streams
2. Support all SSE features: event types, IDs, retry intervals
3. Provide automatic keep-alive to prevent connection timeouts
4. Handle client reconnection with Last-Event-ID header
5. Provide idiomatic APIs for Python, TypeScript, and Ruby
6. Integrate seamlessly with existing Handler trait

### Non-Goals

- WebSocket support (covered in separate design doc)
- Bidirectional communication (SSE is unidirectional)
- Binary event data (SSE is text-only)
- Custom SSE protocol extensions

## SSE Protocol Overview

Server-Sent Events is a W3C standard for real-time event streaming:

```
HTTP/1.1 200 OK
Content-Type: text/event-stream
Cache-Control: no-cache
Connection: keep-alive

event: message
id: 1
data: Hello, World!

event: update
id: 2
data: {"user": "Alice", "action": "login"}

: keep-alive comment

event: notification
id: 3
data: System maintenance in 5 minutes
```

**Protocol Features:**
- **Event field:** Optional event type (defaults to "message")
- **ID field:** Optional event ID for reconnection
- **Data field:** Event payload (can span multiple lines)
- **Retry field:** Milliseconds before client reconnects
- **Comments:** Lines starting with `:` are ignored (used for keep-alive)

## Axum's SSE Approach

Axum provides first-class SSE support via the `axum::response::sse` module:

```rust
use axum::{
    response::sse::{Event, KeepAlive, Sse},
    routing::get,
    Router,
};
use futures::stream::{self, Stream};
use std::time::Duration;
use std::convert::Infallible;

async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::iter(vec![
        Event::default().data("event 1"),
        Event::default()
            .event("custom")
            .id("42")
            .data("event 2"),
    ]);

    Sse::new(stream)
        .keep_alive(
            KeepAlive::new()
                .interval(Duration::from_secs(15))
                .text("keep-alive-text")
        )
}
```

**Key Types:**
- **`Sse<S>`:** Wrapper that sets headers and formats events
- **`Event`:** Builder for SSE events with `data()`, `event()`, `id()`, `retry()` methods
- **`KeepAlive`:** Configuration for automatic keep-alive comments
- Stream produces `Result<Event, E>` where `E: StdError`

## Implementation Strategy

### Phase 1: Rust Core (1-2 days)

**File:** `crates/spikard-http/src/sse.rs` (new)

```rust
use axum::response::sse::{Event as AxumEvent, KeepAlive as AxumKeepAlive, Sse as AxumSse};
use futures::stream::Stream;
use serde::Serialize;
use std::time::Duration;

/// SSE event builder
#[derive(Debug, Clone, Default)]
pub struct SseEvent {
    /// Event type (defaults to "message")
    event: Option<String>,
    /// Event ID for reconnection
    id: Option<String>,
    /// Event data
    data: String,
    /// Retry interval in milliseconds
    retry: Option<Duration>,
}

impl SseEvent {
    /// Create new SSE event
    pub fn new() -> Self {
        Self::default()
    }

    /// Create event from data
    pub fn data(data: impl Into<String>) -> Self {
        Self {
            data: data.into(),
            ..Default::default()
        }
    }

    /// Create JSON event from serializable data
    pub fn json<T: Serialize>(data: &T) -> Result<Self, serde_json::Error> {
        Ok(Self {
            data: serde_json::to_string(data)?,
            ..Default::default()
        })
    }

    /// Set event type
    pub fn with_event(mut self, event: impl Into<String>) -> Self {
        self.event = Some(event.into());
        self
    }

    /// Set event ID
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set retry interval
    pub fn with_retry(mut self, retry: Duration) -> Self {
        self.retry = Some(retry);
        self
    }

    /// Convert to Axum Event
    pub fn into_axum_event(self) -> AxumEvent {
        let mut event = AxumEvent::default().data(self.data);

        if let Some(event_type) = self.event {
            event = event.event(event_type);
        }

        if let Some(id) = self.id {
            event = event.id(id);
        }

        if let Some(retry) = self.retry {
            event = event.retry(retry);
        }

        event
    }
}

/// SSE keep-alive configuration
#[derive(Debug, Clone)]
pub struct SseKeepAlive {
    /// Interval between keep-alive comments
    interval: Duration,
    /// Optional text for keep-alive comments
    text: Option<String>,
}

impl SseKeepAlive {
    /// Create new keep-alive config with interval
    pub fn new(interval: Duration) -> Self {
        Self {
            interval,
            text: None,
        }
    }

    /// Set keep-alive text
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Convert to Axum KeepAlive
    pub fn into_axum_keepalive(self) -> AxumKeepAlive {
        let mut keep_alive = AxumKeepAlive::new().interval(self.interval);

        if let Some(text) = self.text {
            keep_alive = keep_alive.text(text);
        }

        keep_alive
    }
}

impl Default for SseKeepAlive {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(15),
            text: None,
        }
    }
}

/// SSE response handler result
pub struct SseResponse {
    /// Stream of SSE events
    stream: Pin<Box<dyn Stream<Item = Result<SseEvent, BoxError>> + Send>>,
    /// Keep-alive configuration
    keep_alive: Option<SseKeepAlive>,
}

impl SseResponse {
    /// Create SSE response from event stream
    pub fn new<S, E>(stream: S) -> Self
    where
        S: Stream<Item = Result<SseEvent, E>> + Send + 'static,
        E: Into<BoxError>,
    {
        Self {
            stream: Box::pin(stream.map(|r| r.map_err(Into::into))),
            keep_alive: Some(SseKeepAlive::default()),
        }
    }

    /// Set keep-alive configuration
    pub fn with_keep_alive(mut self, keep_alive: SseKeepAlive) -> Self {
        self.keep_alive = Some(keep_alive);
        self
    }

    /// Disable keep-alive
    pub fn without_keep_alive(mut self) -> Self {
        self.keep_alive = None;
        self
    }

    /// Convert to Axum SSE response
    pub fn into_axum_response(self) -> AxumSse<impl Stream<Item = Result<AxumEvent, BoxError>>> {
        let stream = self.stream.map(|result| {
            result.map(|event| event.into_axum_event())
        });

        let mut sse = AxumSse::new(stream);

        if let Some(keep_alive) = self.keep_alive {
            sse = sse.keep_alive(keep_alive.into_axum_keepalive());
        }

        sse
    }
}
```

**File:** `crates/spikard-http/src/handler.rs` (update)

Add SSE support to HandlerResponse:

```rust
pub enum HandlerResponse {
    Response(Response<Body>),
    Stream { /* ... */ },
    /// Server-Sent Events response
    Sse(SseResponse),
}

impl HandlerResponse {
    pub fn into_response(self) -> Response<Body> {
        match self {
            HandlerResponse::Response(r) => r,
            HandlerResponse::Stream { stream, status, headers } => { /* ... */ }
            HandlerResponse::Sse(sse) => {
                sse.into_axum_response().into_response()
            }
        }
    }
}
```

### Phase 2: Python Bindings (1-2 days)

**File:** `crates/spikard-py/src/sse.rs` (new)

```rust
use pyo3::prelude::*;
use crate::sse::{SseEvent as RustSseEvent, SseKeepAlive as RustKeepAlive};

#[pyclass]
#[derive(Clone)]
pub struct SseEvent {
    #[pyo3(get, set)]
    pub data: String,

    #[pyo3(get, set)]
    pub event: Option<String>,

    #[pyo3(get, set)]
    pub id: Option<String>,

    #[pyo3(get, set)]
    pub retry: Option<u64>, // milliseconds
}

#[pymethods]
impl SseEvent {
    #[new]
    #[pyo3(signature = (data, *, event=None, id=None, retry=None))]
    fn new(
        data: String,
        event: Option<String>,
        id: Option<String>,
        retry: Option<u64>,
    ) -> Self {
        Self { data, event, id, retry }
    }

    /// Create event from JSON-serializable data
    #[staticmethod]
    fn json(data: &Bound<'_, PyAny>) -> PyResult<Self> {
        let json_str = Python::with_gil(|py| {
            let json_module = py.import("json")?;
            let json_str: String = json_module
                .call_method1("dumps", (data,))?
                .extract()?;
            Ok::<_, PyErr>(json_str)
        })?;

        Ok(Self {
            data: json_str,
            event: None,
            id: None,
            retry: None,
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "SseEvent(data={:?}, event={:?}, id={:?}, retry={:?})",
            self.data, self.event, self.id, self.retry
        )
    }
}

impl SseEvent {
    pub fn to_rust_event(&self) -> RustSseEvent {
        let mut event = RustSseEvent::data(&self.data);

        if let Some(ref event_type) = self.event {
            event = event.with_event(event_type);
        }

        if let Some(ref id) = self.id {
            event = event.with_id(id);
        }

        if let Some(retry_ms) = self.retry {
            event = event.with_retry(Duration::from_millis(retry_ms));
        }

        event
    }
}

#[pyclass]
pub struct SseResponse {
    /// Python async generator yielding SseEvent objects
    stream: Py<PyAny>,

    /// Keep-alive interval in seconds
    #[pyo3(get, set)]
    keep_alive_interval: Option<f64>,

    /// Keep-alive text
    #[pyo3(get, set)]
    keep_alive_text: Option<String>,
}

#[pymethods]
impl SseResponse {
    #[new]
    #[pyo3(signature = (stream, *, keep_alive_interval=15.0, keep_alive_text=None))]
    fn new(
        stream: Py<PyAny>,
        keep_alive_interval: Option<f64>,
        keep_alive_text: Option<String>,
    ) -> Self {
        Self {
            stream,
            keep_alive_interval,
            keep_alive_text,
        }
    }
}

impl SseResponse {
    pub fn to_rust_stream(&self) -> impl Stream<Item = Result<RustSseEvent, BoxError>> + Send + 'static {
        let stream = self.stream.clone();

        async_stream::stream! {
            loop {
                // Get next event from Python async iterator
                let event = Python::with_gil(|py| -> PyResult<Option<SseEvent>> {
                    let awaitable = stream.call_method0(py, "__anext__")?;
                    let future = pyo3_async_runtimes::tokio::into_future(awaitable)?;
                    Ok(Some(future))
                })?;

                if let Some(future) = event {
                    match future.await {
                        Ok(py_event) => {
                            let sse_event = Python::with_gil(|py| {
                                py_event.extract::<SseEvent>(py)
                            })?;
                            yield Ok(sse_event.to_rust_event());
                        }
                        Err(e) => {
                            // Check for StopAsyncIteration
                            if Python::with_gil(|py| {
                                e.is_instance_of::<pyo3::exceptions::PyStopAsyncIteration>(py)
                            }) {
                                break;
                            }
                            yield Err(e.into());
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        }
    }

    pub fn to_handler_response(&self) -> PyResult<HandlerResponse> {
        let stream = self.to_rust_stream();
        let mut response = SseResponse::new(stream);

        if let Some(interval_secs) = self.keep_alive_interval {
            let keep_alive = RustKeepAlive::new(Duration::from_secs_f64(interval_secs));
            let keep_alive = if let Some(ref text) = self.keep_alive_text {
                keep_alive.with_text(text)
            } else {
                keep_alive
            };
            response = response.with_keep_alive(keep_alive);
        } else {
            response = response.without_keep_alive();
        }

        Ok(HandlerResponse::Sse(response))
    }
}
```

**File:** `packages/python/spikard/sse.py` (new)

```python
from typing import AsyncIterator, Any
from dataclasses import dataclass

@dataclass
class SseEvent:
    """Server-Sent Event

    Represents a single SSE event with optional type, ID, and retry.

    Attributes:
        data: Event data (string or will be JSON-serialized)
        event: Optional event type (defaults to "message")
        id: Optional event ID for reconnection
        retry: Optional retry interval in milliseconds

    Examples:
        Simple text event:
        ```python
        event = SseEvent(data="Hello, World!")
        ```

        Typed event with ID:
        ```python
        event = SseEvent(
            data='{"user": "Alice"}',
            event="user_login",
            id="123"
        )
        ```

        Event with retry:
        ```python
        event = SseEvent(
            data="Reconnect soon",
            retry=5000  # 5 seconds
        )
        ```
    """
    data: str
    event: str | None = None
    id: str | None = None
    retry: int | None = None

    @staticmethod
    def json(data: Any, **kwargs) -> "SseEvent":
        """Create event from JSON-serializable data

        Args:
            data: Any JSON-serializable object
            **kwargs: Additional SseEvent fields (event, id, retry)

        Returns:
            SseEvent with JSON-serialized data
        """
        import json
        return SseEvent(data=json.dumps(data), **kwargs)


class SseResponse:
    """Server-Sent Events response

    Accepts an async generator that yields SseEvent objects.
    Automatically handles keep-alive and reconnection.

    Args:
        stream: Async generator yielding SseEvent objects
        keep_alive_interval: Seconds between keep-alive pings (default 15)
        keep_alive_text: Optional text for keep-alive comments

    Examples:
        Real-time notifications:
        ```python
        @app.get("/notifications")
        async def notifications():
            async def event_stream():
                while True:
                    # Wait for new notification
                    notification = await notification_queue.get()

                    yield SseEvent.json(
                        notification,
                        event="notification",
                        id=notification["id"]
                    )

            return SseResponse(event_stream())
        ```

        Live counter:
        ```python
        @app.get("/counter")
        async def counter():
            async def count_stream():
                for i in range(100):
                    await asyncio.sleep(1)
                    yield SseEvent(
                        data=str(i),
                        event="count",
                        id=str(i)
                    )

            return SseResponse(count_stream(), keep_alive_interval=5)
        ```

        Reconnection handling:
        ```python
        @app.get("/events")
        async def events(request: Request):
            # Get last received event ID
            last_id = request.headers.get("last-event-id")

            async def event_stream():
                # Resume from last_id if reconnecting
                events = await get_events_since(last_id)

                for event in events:
                    yield SseEvent.json(
                        event,
                        id=event["id"],
                        event="update"
                    )

            return SseResponse(event_stream())
        ```
    """

    def __init__(
        self,
        stream: AsyncIterator[SseEvent],
        *,
        keep_alive_interval: float = 15.0,
        keep_alive_text: str | None = None,
    ):
        self.stream = stream
        self.keep_alive_interval = keep_alive_interval
        self.keep_alive_text = keep_alive_text
```

**Usage Example:**

```python
from spikard import Spikard, Request
from spikard.sse import SseEvent, SseResponse
import asyncio

app = Spikard()

@app.get("/events")
async def events():
    """Real-time event stream"""
    async def event_stream():
        for i in range(10):
            await asyncio.sleep(1)
            yield SseEvent(
                data=f"Event {i}",
                event="counter",
                id=str(i)
            )

    return SseResponse(event_stream())

@app.get("/notifications")
async def notifications(request: Request):
    """Notification feed with reconnection support"""
    # Handle reconnection
    last_id = request.headers.get("last-event-id")

    async def notification_stream():
        # Get notifications since last_id (if reconnecting)
        notifications = await get_notifications_since(last_id)

        for notification in notifications:
            yield SseEvent.json(
                notification,
                event="notification",
                id=notification["id"]
            )

    return SseResponse(notification_stream())

@app.get("/stock-prices")
async def stock_prices():
    """Live stock price updates"""
    async def price_stream():
        while True:
            prices = await fetch_current_prices()
            yield SseEvent.json(
                prices,
                event="prices",
                id=str(int(time.time() * 1000))
            )
            await asyncio.sleep(5)

    return SseResponse(
        price_stream(),
        keep_alive_interval=30  # 30 second keep-alive
    )
```

### Phase 3: TypeScript Bindings (1-2 days)

**Usage Example:**

```typescript
import { Spikard, Request, SseEvent, SseResponse } from '@spikard/node';

const app = new Spikard();

app.get('/events', async () => {
  async function* eventStream() {
    for (let i = 0; i < 10; i++) {
      await new Promise(resolve => setTimeout(resolve, 1000));
      yield new SseEvent({
        data: `Event ${i}`,
        event: 'counter',
        id: String(i)
      });
    }
  }

  return new SseResponse(eventStream());
});

app.get('/notifications', async (request: Request) => {
  const lastId = request.headers['last-event-id'];

  async function* notificationStream() {
    const notifications = await getNotificationsSince(lastId);

    for (const notification of notifications) {
      yield SseEvent.json(notification, {
        event: 'notification',
        id: notification.id
      });
    }
  }

  return new SseResponse(notificationStream(), {
    keepAliveInterval: 15000
  });
});
```

### Phase 4: Ruby Bindings (1-2 days)

**Usage Example:**

```ruby
require 'spikard'

app = Spikard::App.new

app.get('/events', handler_name: 'events') do
  Enumerator.new do |yielder|
    10.times do |i|
      sleep 1
      yielder << Spikard::SseEvent.new(
        data: "Event #{i}",
        event: 'counter',
        id: i.to_s
      )
    end
  end
end

app.get('/notifications', handler_name: 'notifications') do |request|
  last_id = request.headers['last-event-id']

  Spikard::SseResponse.new(
    Enumerator.new do |yielder|
      notifications = get_notifications_since(last_id)
      notifications.each do |notification|
        yielder << Spikard::SseEvent.json(
          notification,
          event: 'notification',
          id: notification[:id]
        )
      end
    end,
    keep_alive_interval: 15.0
  )
end
```

## Testing Strategy

### Integration Tests (Python)

**File:** `packages/python/tests/test_sse.py`

```python
import pytest
from spikard import Spikard, Request
from spikard.sse import SseEvent, SseResponse
import asyncio

@pytest.mark.asyncio
async def test_sse_basic():
    """Test basic SSE event stream"""
    app = Spikard()

    @app.get("/events")
    async def events():
        async def stream():
            for i in range(3):
                yield SseEvent(data=f"event{i}", id=str(i))

        return SseResponse(stream())

    client = app.test_client()
    response = await client.get("/events")

    assert response.status_code == 200
    assert response.headers["content-type"] == "text/event-stream"
    assert response.headers["cache-control"] == "no-cache"

    # Parse SSE events
    events = parse_sse_events(response.text)
    assert len(events) == 3
    assert events[0]["data"] == "event0"
    assert events[0]["id"] == "0"

@pytest.mark.asyncio
async def test_sse_json_events():
    """Test SSE with JSON data"""
    app = Spikard()

    @app.get("/json-events")
    async def json_events():
        async def stream():
            yield SseEvent.json({"count": 1}, event="update")
            yield SseEvent.json({"count": 2}, event="update")

        return SseResponse(stream())

    client = app.test_client()
    response = await client.get("/json-events")

    events = parse_sse_events(response.text)
    assert len(events) == 2
    assert json.loads(events[0]["data"]) == {"count": 1}
    assert events[0]["event"] == "update"

@pytest.mark.asyncio
async def test_sse_reconnection():
    """Test SSE reconnection with Last-Event-ID"""
    app = Spikard()

    @app.get("/events")
    async def events(request: Request):
        last_id = request.headers.get("last-event-id")
        start = int(last_id) + 1 if last_id else 0

        async def stream():
            for i in range(start, start + 3):
                yield SseEvent(data=f"event{i}", id=str(i))

        return SseResponse(stream())

    client = app.test_client()

    # Initial connection
    response = await client.get("/events")
    events = parse_sse_events(response.text)
    assert events[0]["id"] == "0"

    # Reconnect with Last-Event-ID
    response = await client.get(
        "/events",
        headers={"last-event-id": "2"}
    )
    events = parse_sse_events(response.text)
    assert events[0]["id"] == "3"  # Resumed from ID 2
```

## Client Usage

### JavaScript/Browser Client

```javascript
const eventSource = new EventSource('/events');

// Handle default "message" events
eventSource.onmessage = (event) => {
  console.log('Message:', event.data);
  console.log('ID:', event.lastEventId);
};

// Handle typed events
eventSource.addEventListener('notification', (event) => {
  const notification = JSON.parse(event.data);
  console.log('Notification:', notification);
});

// Handle errors and reconnection
eventSource.onerror = (error) => {
  console.error('SSE error:', error);
  // EventSource automatically reconnects
};

// Close connection when done
// eventSource.close();
```

### Python Client (httpx-sse)

```python
import httpx
from httpx_sse import connect_sse

async with httpx.AsyncClient() as client:
    async with connect_sse(client, "GET", "http://localhost:8000/events") as event_source:
        async for event in event_source.aiter_sse():
            print(f"Event: {event.event}")
            print(f"Data: {event.data}")
            print(f"ID: {event.id}")
```

## Performance Considerations

### Connection Limits

- Each SSE connection holds an open HTTP connection
- Typical browser limit: 6 connections per domain
- Use HTTP/2 for better connection multiplexing
- Consider connection pooling and load balancing

### Keep-Alive Tuning

| Interval | Use Case |
|----------|----------|
| 5-10s | Low-latency, frequent updates |
| 15-30s | Standard real-time updates (default) |
| 60s+ | Infrequent updates, battery-sensitive |

### Memory Management

- Axum automatically handles backpressure
- Slow clients won't block event generation
- Consider buffer limits for high-volume streams

## Implementation Checklist

### Rust Core
- [ ] Create `crates/spikard-http/src/sse.rs`
- [ ] Define `SseEvent` builder
- [ ] Define `SseKeepAlive` config
- [ ] Define `SseResponse` wrapper
- [ ] Add SSE variant to `HandlerResponse`
- [ ] Write unit tests
- [ ] Add integration tests

### Python Bindings
- [ ] Create `crates/spikard-py/src/sse.rs`
- [ ] Implement `SseEvent` class
- [ ] Implement `SseResponse` class
- [ ] Convert Python async generator to Rust stream
- [ ] Create `packages/python/spikard/sse.py` API
- [ ] Add type hints and docstrings
- [ ] Write integration tests
- [ ] Add examples

### TypeScript Bindings
- [ ] Create `crates/spikard-node/src/sse.rs`
- [ ] Implement SSE classes
- [ ] Add TypeScript types
- [ ] Write integration tests
- [ ] Add examples

### Ruby Bindings
- [ ] Create `crates/spikard-rb/src/sse.rs`
- [ ] Implement SSE classes
- [ ] Write tests
- [ ] Add examples

### Documentation
- [ ] Add SSE section to README
- [ ] Create user guide with examples
- [ ] Document browser client usage
- [ ] Document reconnection handling
- [ ] Add performance tuning guide

## References

- **SSE Specification:** https://html.spec.whatwg.org/multipage/server-sent-events.html
- **Axum SSE module:** https://docs.rs/axum/latest/axum/response/sse/
- **MDN EventSource:** https://developer.mozilla.org/en-US/docs/Web/API/EventSource
- **httpx-sse (Python client):** https://github.com/florimondmanca/httpx-sse
- **EventSource polyfill:** https://github.com/Yaffle/EventSource

---

**Key Takeaway:** Server-Sent Events provide a standardized protocol for real-time server-to-client streaming, with Axum's native support ensuring spec compliance, automatic keep-alive, and reconnection handling through idiomatic async generator/iterator APIs.

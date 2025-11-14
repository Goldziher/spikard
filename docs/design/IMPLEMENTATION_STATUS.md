# WebSocket & SSE Implementation Status

**Date:** November 2025
**Status:** ✅ Rust Core + Python/Node.js/Ruby Bindings Complete

## What We Built

### Core Architecture

We implemented a simplified, pragmatic approach for WebSocket and SSE support that prioritizes:
1. **Async-first** - All handlers are async
2. **Litestar-style decorators** - `@websocket()` and `@sse()`, not `@app.method()`
3. **JSON Schema validation in Rust** - All heavy lifting in the Rust layer
4. **Type hint extraction** - Automatic schema extraction from Python type hints
5. **Zero-cost when unused** - No validation overhead without schemas

### WebSocket Implementation

#### Rust Core (`crates/spikard-http/src/websocket.rs`)

```rust
pub trait WebSocketHandler: Send + Sync {
    async fn handle_message(&self, message: Value) -> Option<Value>;

    // Optional lifecycle hooks with default implementations
    async fn on_connect(&self) {}
    async fn on_disconnect(&self) {}
}

pub struct WebSocketState<H: WebSocketHandler> {
    handler: Arc<H>,
    message_schema: Option<Arc<jsonschema::Validator>>,
    response_schema: Option<Arc<jsonschema::Validator>>,
}

impl<H: WebSocketHandler + 'static> WebSocketState<H> {
    pub fn with_schemas(
        handler: H,
        message_schema: Option<serde_json::Value>,
        response_schema: Option<serde_json::Value>,
    ) -> Result<Self, String> {
        // Creates validators from schemas
    }
}
```

**Validation Logic:**
- Incoming messages validated **before** handler processing
- Invalid messages → error response sent to client
- Outgoing responses validated **before** sending
- Invalid responses → logged and dropped (connection continues)

#### Python API (`packages/python/spikard/websocket.py`)

```python
from spikard import websocket
from typing import TypedDict

class ChatMessage(TypedDict):
    text: str
    user: str

@websocket("/chat")
async def chat_handler(message: ChatMessage) -> dict:
    """WebSocket handler with automatic schema validation."""
    return {"echo": message["text"], "from": message["user"]}
```

**Features:**
- `@websocket(path, *, message_schema=None, response_schema=None)`
- Automatic schema extraction from type hints
- Schemas passed to Rust via `create_websocket_state()`
- Supports all Python type systems: TypedDict, dataclass, Pydantic, msgspec

### SSE Implementation

#### Rust Core (`crates/spikard-http/src/sse.rs`)

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

impl<P: SseEventProducer + 'static> SseState<P> {
    pub fn with_schema(
        producer: P,
        event_schema: Option<serde_json::Value>,
    ) -> Result<Self, String> {
        // Creates validator from schema
    }
}
```

**Validation Logic:**
- Events validated **before** streaming to clients
- Invalid events → logged and skipped (sends "validation_error" event)
- Stream continues with valid events

#### Python API (`packages/python/spikard/sse.py`)

```python
from spikard import sse
from typing import TypedDict, AsyncIterator

class StatusEvent(TypedDict):
    status: str
    message: str
    timestamp: int

@sse("/status")
async def status_stream() -> AsyncIterator[StatusEvent]:
    """SSE handler with automatic schema validation."""
    for i in range(10):
        await asyncio.sleep(1)
        yield {"status": "ok", "message": f"Update {i}", "timestamp": i}
```

**Features:**
- `@sse(path, *, event_schema=None)`
- Automatic schema extraction from `AsyncIterator[EventType]` hints
- Schemas passed to Rust via `create_sse_state()`
- Supports all Python type systems

### Schema Validation Flow

```
┌─────────────────┐
│  Python Layer   │
│  Type Hints     │ → TypedDict, dataclass, Pydantic, msgspec
│  @websocket()   │
│  @sse()         │
└────────┬────────┘
         │ Schema Extraction
         ↓
┌─────────────────┐
│ Python Bridge   │ → extract_json_schema() from type hints
│ (spikard-py)    │ → Convert Python dict → serde_json::Value
└────────┬────────┘
         │ Pass JSON Schema
         ↓
┌─────────────────┐
│   Rust Core     │
│ WebSocketState  │ → jsonschema::validator_for()
│ SseState        │ → validator.is_valid(&json_value)
└─────────────────┘
```

**Key Design Decision:** All validation happens in Rust using `jsonschema` crate for:
- **Performance** - No Python GIL contention
- **Consistency** - Same validation across Python/Node/Ruby
- **Zero-copy** - Direct validation of Rust `serde_json::Value`

## Implementation Details

### Python Bridge Layer

**WebSocket** (`crates/spikard-py/src/websocket.rs`):
```rust
pub fn create_websocket_state(
    factory: &Bound<'_, PyAny>,
) -> PyResult<spikard_http::WebSocketState<PythonWebSocketHandler>> {
    let handler_instance = factory.call0()?;

    // Extract schemas from Python handler attributes
    let message_schema = handler_instance.getattr("_message_schema").ok()
        .and_then(|attr| python_dict_to_json(attr));
    let response_schema = handler_instance.getattr("_response_schema").ok()
        .and_then(|attr| python_dict_to_json(attr));

    // Create handler and state with schemas
    let py_handler = PythonWebSocketHandler::new(handler_instance.unbind());
    WebSocketState::with_schemas(py_handler, message_schema, response_schema)
}
```

**SSE** (`crates/spikard-py/src/sse.rs`):
```rust
pub fn create_sse_state(
    factory: &Bound<'_, PyAny>,
) -> PyResult<spikard_http::SseState<PythonSseEventProducer>> {
    let producer_instance = factory.call0()?;

    // Extract event schema
    let event_schema = producer_instance.getattr("_event_schema").ok()
        .and_then(|attr| python_dict_to_json(attr));

    let py_producer = PythonSseEventProducer::new(producer_instance.unbind());
    SseState::with_schema(py_producer, event_schema)
}
```

### Test Infrastructure

**WebSocket Testing** (`crates/spikard-http/src/testing.rs`):
- Uses `axum-test` with `Transport::HttpRandomPort` for real WebSocket upgrade
- Multi-threaded Tokio runtime required for HTTP transport background tasks
- `WebSocketConnection` wrapper with `send_text()`, `send_json()`, `receive_json()` methods

**SSE Testing** (`crates/spikard-http/src/testing.rs`):
- HTTP GET request to SSE endpoint
- Stream reading with timeout support
- Event parsing and JSON deserialization

## What Differs from Original Design Docs

### Simplified WebSocket API
**Original Design** (websocket-support.md):
- Multiple lifecycle hooks: `on_connect`, `on_message`, `on_close`, `on_error`
- Class-based handlers
- Complex connection management

**Actual Implementation:**
- Single `handle_message()` method (with optional lifecycle hooks)
- Decorator-based handlers
- Simplified connection lifecycle

**Rationale:** Keep it simple. Most use cases only need message handling. Lifecycle hooks available as opt-in.

### Litestar-Style Decorators
**Original Design:**
- `@app.websocket()` pattern

**Actual Implementation:**
- `@websocket()` standalone decorator
- Uses `Spikard.current_instance` pattern

**Rationale:** Matches Litestar/Starlette conventions. Cleaner API for most cases.

### Async-First Philosophy
**Original Design:**
- Support both sync and async handlers

**Actual Implementation:**
- **Async-only** handlers

**Rationale:** Consistency with HTTP handlers. Python async is mature enough.

### Schema Validation Addition
**Original Design:**
- No mention of validation

**Actual Implementation:**
- Full JSON Schema validation in Rust
- Automatic extraction from type hints
- Zero-cost when not used

**Rationale:** Type safety and validation are core Spikard features. Extend to WebSocket/SSE.

## Testing Status

### Python
✅ WebSocket handler registration
✅ WebSocket message exchange
✅ SSE event streaming
✅ Schema extraction from type hints
✅ Multi-threaded runtime for test client
✅ Rust layer schema validation
🟡 E2E validation tests (need fixtures with schemas)

### Node.js
✅ Rust bindings with schema support
✅ ThreadsafeFunction integration
✅ Schema extraction from handler attributes
🟡 TypeScript decorator API (pending)
🟡 E2E validation tests (pending)

### Ruby
✅ Rust bindings with schema support
✅ Magnus integration with GVL safety
✅ Schema extraction from instance variables
🟡 Ruby decorator API (pending)
🟡 E2E validation tests (pending)

### Rust Native
🟡 Native Rust handlers (for pure Rust applications)

## Next Steps

1. **Add validation test fixtures** - Create WebSocket/SSE fixtures with schemas for e2e testing
2. **TypeScript decorator API** - High-level `@websocket()` and `@sse()` decorators for Node.js
3. **Ruby decorator API** - High-level decorator methods for Ruby
4. **Rust native handlers** - For pure Rust applications (no FFI)
5. **AsyncAPI generation** - Generate AsyncAPI 3.0 specs from WebSocket/SSE handlers

## Implementation Files

### Rust Core
- WebSocket trait: `crates/spikard-http/src/websocket.rs`
- SSE trait: `crates/spikard-http/src/sse.rs`

### Python Bindings
- WebSocket bridge: `crates/spikard-py/src/websocket.rs`
- SSE bridge: `crates/spikard-py/src/sse.rs`
- WebSocket API: `packages/python/spikard/websocket.py`
- SSE API: `packages/python/spikard/sse.py`

### Node.js Bindings
- WebSocket bridge: `crates/spikard-node/src/websocket.rs` ✅ Schema support added
- SSE bridge: `crates/spikard-node/src/sse.rs` ✅ Schema support added

### Ruby Bindings
- WebSocket bridge: `crates/spikard-rb/src/websocket.rs` ✅ Schema support added
- SSE bridge: `crates/spikard-rb/src/sse.rs` ✅ Schema support added

### Test Infrastructure
- Python test client: `crates/spikard-py/src/test_websocket.rs`
- Node test client: `crates/spikard-node/src/test_websocket.rs`
- Ruby test client: `crates/spikard-rb/src/test_websocket.rs`
- Test fixtures: `testing_data/{websockets,sse}/*.json`

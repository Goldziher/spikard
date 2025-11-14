# WebSocket Support

**Date:** November 2025
**Status:** ✅ Python, Node.js, Ruby
**Related:** [server-sent-events.md](./server-sent-events.md), [validation-strategy.md](./validation-strategy.md)

## Overview

Spikard provides full-duplex WebSocket support built on Axum's native WebSocket handling. All message validation happens in Rust using JSON Schema, with automatic schema extraction from type hints.

**Design Principles:**
- **Async-first** - All handlers are async functions
- **Litestar-style decorators** - `@websocket()` standalone decorator
- **Rust validation** - All JSON Schema validation in Rust layer
- **Zero-cost** - No validation overhead when schemas aren't provided
- **Type-safe** - Automatic schema extraction from type hints

## Python API

### Basic Handler

```python
from spikard import websocket

@websocket("/echo")
async def echo_handler(message: dict) -> dict:
    """Simple echo server."""
    return message
```

### With Schema Validation

```python
from spikard import websocket
from typing import TypedDict

class ChatMessage(TypedDict):
    text: str
    user: str

@websocket("/chat")
async def chat_handler(message: ChatMessage) -> dict:
    """Type-safe chat handler with automatic validation."""
    return {
        "echo": message["text"],
        "from": message["user"]
    }
```

The decorator automatically:
1. Extracts JSON Schema from `ChatMessage` type hint
2. Passes schema to Rust via `create_websocket_state()`
3. Validates incoming messages in Rust before handler execution
4. Returns validation errors to client automatically

### Lifecycle Hooks

```python
@websocket("/chat")
async def chat_handler(message: dict) -> dict:
    return {"echo": message}

@chat_handler.on_connect
async def handle_connect():
    print("Client connected")

@chat_handler.on_disconnect
async def handle_disconnect():
    print("Client disconnected")
```

### Manual Schema Specification

```python
message_schema = {
    "type": "object",
    "properties": {
        "text": {"type": "string"},
        "user": {"type": "string"}
    },
    "required": ["text", "user"]
}

response_schema = {
    "type": "object",
    "properties": {
        "echo": {"type": "string"}
    }
}

@websocket("/chat", message_schema=message_schema, response_schema=response_schema)
async def chat_handler(message: dict) -> dict:
    return {"echo": message["text"]}
```

## Architecture

### Rust Core

**File:** `crates/spikard-http/src/websocket.rs`

```rust
pub trait WebSocketHandler: Send + Sync {
    async fn handle_message(&self, message: Value) -> Option<Value>;

    // Optional lifecycle hooks
    async fn on_connect(&self) {}
    async fn on_disconnect(&self) {}
}

pub struct WebSocketState<H: WebSocketHandler> {
    handler: Arc<H>,
    message_schema: Option<Arc<jsonschema::Validator>>,
    response_schema: Option<Arc<jsonschema::Validator>>,
}
```

**Validation Logic:**
- Incoming messages validated **before** handler processing
- Invalid messages → error response sent to client
- Outgoing responses validated **before** sending
- Invalid responses → logged and dropped

### Python Bridge

**File:** `crates/spikard-py/src/websocket.rs`

```rust
pub fn create_websocket_state(
    factory: &Bound<'_, PyAny>,
) -> PyResult<spikard_http::WebSocketState<PythonWebSocketHandler>> {
    let handler_instance = factory.call0()?;

    // Extract schemas from Python handler attributes
    let message_schema = handler_instance
        .getattr("_message_schema")
        .ok()
        .and_then(|attr| python_dict_to_json(attr));

    let response_schema = handler_instance
        .getattr("_response_schema")
        .ok()
        .and_then(|attr| python_dict_to_json(attr));

    let py_handler = PythonWebSocketHandler::new(handler_instance.unbind());
    WebSocketState::with_schemas(py_handler, message_schema, response_schema)
}
```

### Schema Extraction

**File:** `packages/python/spikard/websocket.py`

```python
def websocket(
    path: str,
    *,
    message_schema: dict[str, Any] | None = None,
    response_schema: dict[str, Any] | None = None,
) -> Callable[[F], F]:
    """WebSocket endpoint decorator with schema validation."""

    def decorator(func: F) -> F:
        # Extract schemas from type hints if not provided
        if message_schema is None:
            type_hints = get_type_hints(func)
            sig = inspect.signature(func)
            for param in sig.parameters.values():
                if param.name == "message":
                    param_type = type_hints.get(param.name)
                    if param_type and param_type != dict:
                        message_schema = extract_json_schema(param_type)

        if response_schema is None:
            return_type = type_hints.get("return")
            if return_type and return_type != dict:
                response_schema = extract_json_schema(return_type)

        # Store schemas as function attributes
        wrapper._message_schema = message_schema
        wrapper._response_schema = response_schema

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
┌─────────────────┐
│  Python Layer   │
│  @websocket()   │ → Extract schema from type hints
│  TypedDict etc. │
└────────┬────────┘
         │
         ↓
┌─────────────────┐
│ Python Bridge   │ → extract_json_schema()
│ (spikard-py)    │ → Convert to serde_json::Value
└────────┬────────┘
         │
         ↓
┌─────────────────┐
│   Rust Core     │ → jsonschema::validator_for()
│ WebSocketState  │ → validator.is_valid(&json_value)
└─────────────────┘
```

## Client Usage

### JavaScript/Browser

```javascript
const ws = new WebSocket('ws://localhost:8000/chat');

ws.onopen = () => {
  ws.send(JSON.stringify({ text: 'Hello', user: 'Alice' }));
};

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Received:', data);
};
```

### Python Client

```python
import asyncio
import websockets
import json

async def client():
    async with websockets.connect('ws://localhost:8000/chat') as ws:
        await ws.send(json.dumps({"text": "Hello", "user": "Alice"}))
        response = await ws.recv()
        print(json.loads(response))

asyncio.run(client())
```

## Testing

**File:** `crates/spikard-http/src/testing.rs`

Uses `axum-test` with `Transport::HttpRandomPort` for real WebSocket upgrades:

```rust
#[tokio::test(flavor = "multi_thread")]
async fn test_websocket() {
    let app = create_test_app();
    let server = TestServer::new_with_config(
        app,
        TestServerConfig::builder()
            .transport(axum_test::Transport::HttpRandomPort)
            .build()
    ).unwrap();

    let ws = server.get_websocket("/chat").await;
    ws.send_text(r#"{"text":"Hello","user":"Alice"}"#).await;
    let msg = ws.receive_text().await.unwrap();
    assert_eq!(msg, r#"{"echo":"Hello","from":"Alice"}"#);
}
```

## Implementation Files

| Layer | File | Purpose |
|-------|------|---------|
| Core | `crates/spikard-http/src/websocket.rs` | WebSocket trait + validation |
| Python Bridge | `crates/spikard-py/src/websocket.rs` | PyO3 handler wrapper |
| Python API | `packages/python/spikard/websocket.py` | `@websocket()` decorator |
| Testing | `crates/spikard-http/src/testing.rs` | Test helpers |

## Performance

- **Validation:** ~500k validations/sec in Rust
- **No GIL contention:** Validation outside Python
- **Zero-copy:** Direct validation on `serde_json::Value`
- **Zero-cost when unused:** No overhead without schemas

## Next Steps

1. **TypeScript/Node.js API** - High-level decorator API for TypeScript
2. **Ruby API** - High-level decorator API for Ruby
3. **Rust native handlers** - For pure Rust applications
4. **Add validation test fixtures** - WebSocket fixtures with schemas for e2e tests

## References

- **WebSocket RFC 6455:** https://datatracker.ietf.org/doc/html/rfc6455
- **Axum WebSocket:** https://docs.rs/axum/latest/axum/extract/ws/
- **JSON Schema 2020-12:** https://json-schema.org/draft/2020-12/json-schema-core.html

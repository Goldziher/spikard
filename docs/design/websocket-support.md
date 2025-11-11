# WebSocket Support Implementation Guide

**Date:** November 2025
**Status:** 🟡 Design Complete, Implementation Pending
**Related Docs:** [streaming-responses.md](./streaming-responses.md), [server-sent-events.md](./server-sent-events.md), [architecture.md](./architecture.md)

## Executive Summary

This document provides a concrete implementation plan for WebSocket support in Spikard, leveraging Axum's `WebSocketUpgrade` extractor and `axum::extract::ws` module. WebSockets enable full-duplex, bidirectional communication between client and server, ideal for chat applications, collaborative editing, gaming, and real-time data synchronization.

**Key Design Principles:**
- ✅ Leverage Axum's `WebSocketUpgrade` for protocol handshake
- ✅ Support text and binary messages
- ✅ Automatic ping/pong keep-alive
- ✅ Clean connection lifecycle management (open, message, close, error)
- ✅ Idiomatic callback/handler APIs in all language bindings

## Overview

### Goals

1. Enable handlers to upgrade HTTP connections to WebSocket
2. Support bidirectional message exchange (text and binary)
3. Provide lifecycle hooks: onConnect, onMessage, onClose, onError
4. Handle ping/pong automatically for connection health
5. Provide idiomatic APIs for Python, TypeScript, and Ruby
6. Support both low-level (per-message) and high-level (stream) patterns

### Non-Goals

- Custom WebSocket extensions (compression, per-message deflate handled by Axum)
- Application-level protocols (STOMP, WAMP) - users can build on top
- Load balancing/clustering (handled at infrastructure level)
- Automatic reconnection (client responsibility)

## WebSocket Protocol Overview

WebSocket (RFC 6455) provides full-duplex communication over a single TCP connection:

1. **Handshake:** HTTP Upgrade request/response
2. **Framing:** Messages sent as frames with opcodes (text, binary, ping, pong, close)
3. **Keep-alive:** Ping/pong frames maintain connection
4. **Close:** Graceful shutdown with close frame and status code

## Axum's WebSocket Approach

Axum provides WebSocket support via the `axum::extract::ws` module:

```rust
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    // Receive messages
    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("Received text: {}", text);
                // Echo back
                if socket.send(Message::Text(text)).await.is_err() {
                    break;
                }
            }
            Ok(Message::Binary(data)) => {
                println!("Received binary: {} bytes", data.len());
            }
            Ok(Message::Close(_)) => {
                println!("Client closed connection");
                break;
            }
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
```

**Key Types:**
- **`WebSocketUpgrade`:** Extractor that performs protocol upgrade
- **`WebSocket`:** Bidirectional stream of messages (implements `Stream + Sink`)
- **`Message`:** Enum for Text, Binary, Ping, Pong, Close frames
- **`.on_upgrade(handler)`:** Spawns async task for socket handling

## Implementation Strategy

### Phase 1: Rust Core (2-3 days)

**File:** `crates/spikard-http/src/websocket.rs` (new)

```rust
use axum::{
    extract::ws::{CloseFrame, Message as AxumMessage, WebSocket as AxumWebSocket},
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;

/// WebSocket message types
#[derive(Debug, Clone)]
pub enum WebSocketMessage {
    /// Text message (UTF-8 string)
    Text(String),
    /// Binary message
    Binary(Vec<u8>),
    /// Close frame with optional code and reason
    Close(Option<WebSocketCloseFrame>),
}

/// WebSocket close frame
#[derive(Debug, Clone)]
pub struct WebSocketCloseFrame {
    pub code: u16,
    pub reason: String,
}

impl From<AxumMessage> for WebSocketMessage {
    fn from(msg: AxumMessage) -> Self {
        match msg {
            AxumMessage::Text(text) => WebSocketMessage::Text(text),
            AxumMessage::Binary(data) => WebSocketMessage::Binary(data.to_vec()),
            AxumMessage::Close(close) => {
                let frame = close.map(|cf| WebSocketCloseFrame {
                    code: cf.code,
                    reason: cf.reason.to_string(),
                });
                WebSocketMessage::Close(frame)
            }
            _ => unreachable!("Ping/Pong handled automatically"),
        }
    }
}

impl From<WebSocketMessage> for AxumMessage {
    fn from(msg: WebSocketMessage) -> Self {
        match msg {
            WebSocketMessage::Text(text) => AxumMessage::Text(text),
            WebSocketMessage::Binary(data) => AxumMessage::Binary(data.into()),
            WebSocketMessage::Close(frame) => {
                let close_frame = frame.map(|f| CloseFrame {
                    code: f.code,
                    reason: f.reason.into(),
                });
                AxumMessage::Close(close_frame)
            }
        }
    }
}

/// WebSocket connection handler
///
/// Trait implemented by language bindings to handle WebSocket lifecycle
#[async_trait::async_trait]
pub trait WebSocketHandler: Send + Sync {
    /// Called when WebSocket connection is established
    async fn on_connect(&self) -> Result<(), String> {
        Ok(())
    }

    /// Called when a message is received
    async fn on_message(&self, message: WebSocketMessage) -> Result<Option<WebSocketMessage>, String>;

    /// Called when connection is closed
    async fn on_close(&self, frame: Option<WebSocketCloseFrame>) -> Result<(), String> {
        Ok(())
    }

    /// Called when an error occurs
    async fn on_error(&self, error: String) -> Result<(), String> {
        Ok(())
    }
}

/// WebSocket response that upgrades the connection
pub struct WebSocketResponse {
    handler: Arc<dyn WebSocketHandler>,
}

impl WebSocketResponse {
    /// Create new WebSocket response with handler
    pub fn new(handler: Arc<dyn WebSocketHandler>) -> Self {
        Self { handler }
    }

    /// Upgrade the connection and handle messages
    pub async fn upgrade(self, ws: AxumWebSocket) {
        let handler = self.handler;

        // Call on_connect
        if let Err(e) = handler.on_connect().await {
            eprintln!("WebSocket on_connect error: {}", e);
            return;
        }

        // Split socket into sender and receiver
        let (mut sender, mut receiver) = ws.split();

        // Message handling loop
        while let Some(result) = receiver.next().await {
            match result {
                Ok(msg) => {
                    // Skip ping/pong (handled automatically by Axum)
                    if matches!(msg, AxumMessage::Ping(_) | AxumMessage::Pong(_)) {
                        continue;
                    }

                    let ws_msg = WebSocketMessage::from(msg);

                    // Handle close frame
                    if let WebSocketMessage::Close(frame) = &ws_msg {
                        if let Err(e) = handler.on_close(frame.clone()).await {
                            eprintln!("WebSocket on_close error: {}", e);
                        }
                        break;
                    }

                    // Call on_message handler
                    match handler.on_message(ws_msg).await {
                        Ok(Some(response)) => {
                            // Send response message
                            let axum_msg = AxumMessage::from(response);
                            if let Err(e) = sender.send(axum_msg).await {
                                eprintln!("Failed to send WebSocket message: {}", e);
                                let _ = handler.on_error(e.to_string()).await;
                                break;
                            }
                        }
                        Ok(None) => {
                            // No response to send
                        }
                        Err(e) => {
                            eprintln!("WebSocket on_message error: {}", e);
                            let _ = handler.on_error(e.clone()).await;
                            // Optionally close connection on error
                            let _ = sender.send(AxumMessage::Close(Some(CloseFrame {
                                code: 1011, // Internal error
                                reason: e.into(),
                            }))).await;
                            break;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("WebSocket receive error: {}", e);
                    let _ = handler.on_error(e.to_string()).await;
                    break;
                }
            }
        }

        // Connection closed
        let _ = handler.on_close(None).await;
    }
}

/// WebSocket connection wrapper for sending messages
///
/// Passed to language bindings to allow sending messages from handlers
pub struct WebSocketConnection {
    sender: Arc<Mutex<SplitSink<AxumWebSocket, AxumMessage>>>,
}

impl WebSocketConnection {
    /// Send a message to the client
    pub async fn send(&self, message: WebSocketMessage) -> Result<(), String> {
        let axum_msg = AxumMessage::from(message);
        self.sender
            .lock()
            .await
            .send(axum_msg)
            .await
            .map_err(|e| e.to_string())
    }

    /// Close the connection
    pub async fn close(&self, code: u16, reason: String) -> Result<(), String> {
        let close_frame = CloseFrame {
            code,
            reason: reason.into(),
        };
        self.sender
            .lock()
            .await
            .send(AxumMessage::Close(Some(close_frame)))
            .await
            .map_err(|e| e.to_string())
    }
}
```

**File:** `crates/spikard-http/src/handler.rs` (update)

Add WebSocket support to HandlerResponse:

```rust
pub enum HandlerResponse {
    Response(Response<Body>),
    Stream { /* ... */ },
    Sse(SseResponse),
    /// WebSocket upgrade response
    WebSocket(WebSocketResponse),
}

impl HandlerResponse {
    pub fn into_response(self, ws_upgrade: Option<WebSocketUpgrade>) -> Response<Body> {
        match self {
            HandlerResponse::Response(r) => r,
            HandlerResponse::Stream { stream, status, headers } => { /* ... */ }
            HandlerResponse::Sse(sse) => sse.into_axum_response().into_response(),
            HandlerResponse::WebSocket(ws) => {
                // WebSocket requires the WebSocketUpgrade extractor
                if let Some(upgrade) = ws_upgrade {
                    upgrade.on_upgrade(move |socket| ws.upgrade(socket))
                } else {
                    // Return error if upgrade not available
                    Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("WebSocket upgrade required"))
                        .unwrap()
                }
            }
        }
    }
}
```

### Phase 2: Python Bindings (2-3 days)

**File:** `crates/spikard-py/src/websocket.rs` (new)

```rust
use pyo3::prelude::*;
use crate::websocket::{
    WebSocketHandler as RustWebSocketHandler,
    WebSocketMessage as RustMessage,
    WebSocketCloseFrame as RustCloseFrame,
};

#[pyclass]
#[derive(Clone)]
pub struct WebSocketMessage {
    inner: RustMessage,
}

#[pymethods]
impl WebSocketMessage {
    #[staticmethod]
    fn text(data: String) -> Self {
        Self {
            inner: RustMessage::Text(data),
        }
    }

    #[staticmethod]
    fn binary(data: Vec<u8>) -> Self {
        Self {
            inner: RustMessage::Binary(data),
        }
    }

    #[staticmethod]
    fn close(code: u16, reason: String) -> Self {
        Self {
            inner: RustMessage::Close(Some(RustCloseFrame { code, reason })),
        }
    }

    fn is_text(&self) -> bool {
        matches!(self.inner, RustMessage::Text(_))
    }

    fn is_binary(&self) -> bool {
        matches!(self.inner, RustMessage::Binary(_))
    }

    fn as_text(&self) -> Option<String> {
        if let RustMessage::Text(ref text) = self.inner {
            Some(text.clone())
        } else {
            None
        }
    }

    fn as_binary(&self) -> Option<Vec<u8>> {
        if let RustMessage::Binary(ref data) = self.inner {
            Some(data.clone())
        } else {
            None
        }
    }
}

/// Python WebSocket handler wrapper
pub struct PythonWebSocketHandler {
    /// Python callbacks
    on_connect: Option<Py<PyAny>>,
    on_message: Py<PyAny>,
    on_close: Option<Py<PyAny>>,
    on_error: Option<Py<PyAny>>,
}

#[async_trait::async_trait]
impl RustWebSocketHandler for PythonWebSocketHandler {
    async fn on_connect(&self) -> Result<(), String> {
        if let Some(ref callback) = self.on_connect {
            Python::with_gil(|py| {
                let result = callback.call0(py)?;
                // If it's a coroutine, await it
                if result.bind(py).hasattr("__await__")? {
                    let future = pyo3_async_runtimes::tokio::into_future(result)?;
                    Ok(future)
                } else {
                    Ok(async { Ok(()) }.boxed())
                }
            }).and_then(|future| future.await)
            .map_err(|e: PyErr| e.to_string())?;
        }
        Ok(())
    }

    async fn on_message(&self, message: RustMessage) -> Result<Option<RustMessage>, String> {
        let py_message = Python::with_gil(|py| {
            Py::new(py, WebSocketMessage { inner: message })
        }).map_err(|e: PyErr| e.to_string())?;

        let result = Python::with_gil(|py| {
            let result = self.on_message.call1(py, (py_message,))?;

            // If it's a coroutine, await it
            if result.bind(py).hasattr("__await__")? {
                pyo3_async_runtimes::tokio::into_future(result)
            } else {
                Ok(async move { Ok(result) }.boxed())
            }
        }).map_err(|e: PyErr| e.to_string())?;

        let py_response = result.await.map_err(|e: PyErr| e.to_string())?;

        // Extract response message (if any)
        Python::with_gil(|py| {
            if py_response.bind(py).is_none() {
                return Ok(None);
            }

            let msg: WebSocketMessage = py_response.extract(py)
                .map_err(|e: PyErr| e.to_string())?;
            Ok(Some(msg.inner))
        })
    }

    async fn on_close(&self, frame: Option<RustCloseFrame>) -> Result<(), String> {
        if let Some(ref callback) = self.on_close {
            Python::with_gil(|py| {
                let py_frame = frame.map(|f| (f.code, f.reason));
                let result = callback.call1(py, (py_frame,))?;

                if result.bind(py).hasattr("__await__")? {
                    pyo3_async_runtimes::tokio::into_future(result)
                } else {
                    Ok(async { Ok(()) }.boxed())
                }
            }).and_then(|future| future.await)
            .map_err(|e: PyErr| e.to_string())?;
        }
        Ok(())
    }

    async fn on_error(&self, error: String) -> Result<(), String> {
        if let Some(ref callback) = self.on_error {
            Python::with_gil(|py| {
                let result = callback.call1(py, (error,))?;

                if result.bind(py).hasattr("__await__")? {
                    pyo3_async_runtimes::tokio::into_future(result)
                } else {
                    Ok(async { Ok(()) }.boxed())
                }
            }).and_then(|future| future.await)
            .map_err(|e: PyErr| e.to_string())?;
        }
        Ok(())
    }
}
```

**File:** `packages/python/spikard/websocket.py` (new)

```python
from typing import Callable, Awaitable, Optional
from dataclasses import dataclass

@dataclass
class WebSocketMessage:
    """WebSocket message

    Can be text or binary data.
    """

    @staticmethod
    def text(data: str) -> "WebSocketMessage":
        """Create text message"""
        ...

    @staticmethod
    def binary(data: bytes) -> "WebSocketMessage":
        """Create binary message"""
        ...

    @staticmethod
    def close(code: int = 1000, reason: str = "") -> "WebSocketMessage":
        """Create close frame"""
        ...

    def is_text(self) -> bool:
        """Check if message is text"""
        ...

    def is_binary(self) -> bool:
        """Check if message is binary"""
        ...

    def as_text(self) -> Optional[str]:
        """Get text content (if text message)"""
        ...

    def as_binary(self) -> Optional[bytes]:
        """Get binary content (if binary message)"""
        ...


class WebSocketResponse:
    """WebSocket handler response

    Defines callbacks for WebSocket lifecycle events.

    Args:
        on_message: Handler for incoming messages (required)
        on_connect: Optional handler called when connection opens
        on_close: Optional handler called when connection closes
        on_error: Optional handler called on errors

    Examples:
        Echo server:
        ```python
        @app.websocket("/ws")
        async def websocket_echo(request: Request):
            async def on_message(msg: WebSocketMessage):
                # Echo back the same message
                return msg

            return WebSocketResponse(on_message=on_message)
        ```

        Chat server:
        ```python
        @app.websocket("/chat")
        async def websocket_chat(request: Request):
            user_id = request.query_params.get("user_id")

            async def on_connect():
                await chat_rooms.add_user(user_id)
                print(f"User {user_id} connected")

            async def on_message(msg: WebSocketMessage):
                if msg.is_text():
                    text = msg.as_text()
                    await chat_rooms.broadcast(user_id, text)
                # Don't return anything (no direct response)
                return None

            async def on_close(frame):
                await chat_rooms.remove_user(user_id)
                print(f"User {user_id} disconnected")

            return WebSocketResponse(
                on_connect=on_connect,
                on_message=on_message,
                on_close=on_close,
            )
        ```

        Binary protocol:
        ```python
        @app.websocket("/data")
        async def websocket_binary(request: Request):
            async def on_message(msg: WebSocketMessage):
                if msg.is_binary():
                    data = msg.as_binary()
                    # Process binary protocol
                    response_data = process_protocol(data)
                    return WebSocketMessage.binary(response_data)
                return None

            return WebSocketResponse(on_message=on_message)
        ```
    """

    def __init__(
        self,
        *,
        on_message: Callable[[WebSocketMessage], Awaitable[Optional[WebSocketMessage]]],
        on_connect: Optional[Callable[[], Awaitable[None]]] = None,
        on_close: Optional[Callable[[Optional[tuple[int, str]]], Awaitable[None]]] = None,
        on_error: Optional[Callable[[str], Awaitable[None]]] = None,
    ):
        self.on_message = on_message
        self.on_connect = on_connect
        self.on_close = on_close
        self.on_error = on_error
```

**Usage Example:**

```python
from spikard import Spikard, Request
from spikard.websocket import WebSocketMessage, WebSocketResponse
import json

app = Spikard()

# Simple echo server
@app.websocket("/echo")
async def echo(request: Request):
    async def on_message(msg: WebSocketMessage):
        return msg  # Echo back

    return WebSocketResponse(on_message=on_message)

# Chat room
connected_clients = {}

@app.websocket("/chat")
async def chat(request: Request):
    user_id = request.query_params.get("user_id", "anonymous")

    async def on_connect():
        connected_clients[user_id] = True
        # Send welcome message
        await broadcast(f"{user_id} joined the chat")

    async def on_message(msg: WebSocketMessage):
        if msg.is_text():
            text = msg.as_text()
            await broadcast(f"{user_id}: {text}")
        return None  # No direct response

    async def on_close(frame):
        del connected_clients[user_id]
        await broadcast(f"{user_id} left the chat")

    async def on_error(error):
        print(f"Error for {user_id}: {error}")

    return WebSocketResponse(
        on_connect=on_connect,
        on_message=on_message,
        on_close=on_close,
        on_error=on_error,
    )

async def broadcast(message: str):
    """Broadcast message to all connected clients"""
    # Implementation would use a shared message queue
    pass

# JSON-RPC over WebSocket
@app.websocket("/rpc")
async def json_rpc(request: Request):
    async def on_message(msg: WebSocketMessage):
        if not msg.is_text():
            return WebSocketMessage.text(
                json.dumps({"error": "Only text messages supported"})
            )

        try:
            data = json.loads(msg.as_text())
            method = data.get("method")
            params = data.get("params", {})
            request_id = data.get("id")

            # Dispatch RPC method
            result = await handle_rpc_method(method, params)

            response = {
                "jsonrpc": "2.0",
                "result": result,
                "id": request_id,
            }
            return WebSocketMessage.text(json.dumps(response))

        except Exception as e:
            error_response = {
                "jsonrpc": "2.0",
                "error": {"code": -32603, "message": str(e)},
                "id": None,
            }
            return WebSocketMessage.text(json.dumps(error_response))

    return WebSocketResponse(on_message=on_message)
```

### Phase 3: TypeScript Bindings (2-3 days)

**Usage Example:**

```typescript
import { Spikard, Request, WebSocketMessage, WebSocketResponse } from '@spikard/node';

const app = new Spikard();

// Echo server
app.websocket('/echo', async (request: Request) => {
  return new WebSocketResponse({
    onMessage: async (msg: WebSocketMessage) => {
      return msg; // Echo back
    }
  });
});

// Chat room
const connectedClients = new Map<string, boolean>();

app.websocket('/chat', async (request: Request) => {
  const userId = request.queryParams.get('user_id') || 'anonymous';

  return new WebSocketResponse({
    onConnect: async () => {
      connectedClients.set(userId, true);
      await broadcast(`${userId} joined the chat`);
    },

    onMessage: async (msg: WebSocketMessage) => {
      if (msg.isText()) {
        const text = msg.asText();
        await broadcast(`${userId}: ${text}`);
      }
      return null; // No direct response
    },

    onClose: async (frame) => {
      connectedClients.delete(userId);
      await broadcast(`${userId} left the chat`);
    },

    onError: async (error: string) => {
      console.error(`Error for ${userId}:`, error);
    }
  });
});

async function broadcast(message: string) {
  // Broadcast to all connected clients
}
```

### Phase 4: Ruby Bindings (2-3 days)

**Usage Example:**

```ruby
require 'spikard'
require 'json'

app = Spikard::App.new

# Echo server
app.websocket('/echo', handler_name: 'echo') do
  Spikard::WebSocketResponse.new(
    on_message: lambda do |msg|
      msg # Echo back
    end
  )
end

# Chat room
$connected_clients = {}

app.websocket('/chat', handler_name: 'chat') do |request|
  user_id = request.query_params['user_id'] || 'anonymous'

  Spikard::WebSocketResponse.new(
    on_connect: lambda do
      $connected_clients[user_id] = true
      broadcast("#{user_id} joined the chat")
    end,

    on_message: lambda do |msg|
      if msg.text?
        text = msg.as_text
        broadcast("#{user_id}: #{text}")
      end
      nil # No direct response
    end,

    on_close: lambda do |frame|
      $connected_clients.delete(user_id)
      broadcast("#{user_id} left the chat")
    end
  )
end
```

## Testing Strategy

### Integration Tests (Python)

**File:** `packages/python/tests/test_websocket.py`

```python
import pytest
from spikard import Spikard, Request
from spikard.websocket import WebSocketMessage, WebSocketResponse
import websockets
import json

@pytest.mark.asyncio
async def test_websocket_echo():
    """Test basic WebSocket echo"""
    app = Spikard()

    @app.websocket("/echo")
    async def echo(request: Request):
        async def on_message(msg: WebSocketMessage):
            return msg

        return WebSocketResponse(on_message=on_message)

    # Start server
    async with app.test_server() as server:
        async with websockets.connect(f"ws://{server.host}:{server.port}/echo") as ws:
            # Send text message
            await ws.send("Hello, WebSocket!")
            response = await ws.recv()
            assert response == "Hello, WebSocket!"

            # Send binary message
            await ws.send(b"\x00\x01\x02\x03")
            response = await ws.recv()
            assert response == b"\x00\x01\x02\x03"

@pytest.mark.asyncio
async def test_websocket_lifecycle():
    """Test WebSocket lifecycle hooks"""
    connect_called = False
    close_called = False

    app = Spikard()

    @app.websocket("/lifecycle")
    async def lifecycle(request: Request):
        async def on_connect():
            nonlocal connect_called
            connect_called = True

        async def on_message(msg: WebSocketMessage):
            return WebSocketMessage.text("pong")

        async def on_close(frame):
            nonlocal close_called
            close_called = True

        return WebSocketResponse(
            on_connect=on_connect,
            on_message=on_message,
            on_close=on_close,
        )

    async with app.test_server() as server:
        async with websockets.connect(f"ws://{server.host}:{server.port}/lifecycle") as ws:
            assert connect_called

            await ws.send("ping")
            response = await ws.recv()
            assert response == "pong"

        # Connection closed
        assert close_called

@pytest.mark.asyncio
async def test_websocket_json_rpc():
    """Test JSON-RPC over WebSocket"""
    app = Spikard()

    @app.websocket("/rpc")
    async def rpc(request: Request):
        async def on_message(msg: WebSocketMessage):
            if not msg.is_text():
                return WebSocketMessage.text(
                    json.dumps({"error": "Text only"})
                )

            data = json.loads(msg.as_text())
            method = data["method"]

            # Simple calculator RPC
            if method == "add":
                result = data["params"]["a"] + data["params"]["b"]
            elif method == "multiply":
                result = data["params"]["a"] * data["params"]["b"]
            else:
                result = {"error": "Unknown method"}

            response = {
                "jsonrpc": "2.0",
                "result": result,
                "id": data["id"],
            }
            return WebSocketMessage.text(json.dumps(response))

        return WebSocketResponse(on_message=on_message)

    async with app.test_server() as server:
        async with websockets.connect(f"ws://{server.host}:{server.port}/rpc") as ws:
            # Call add method
            await ws.send(json.dumps({
                "jsonrpc": "2.0",
                "method": "add",
                "params": {"a": 5, "b": 3},
                "id": 1,
            }))

            response = json.loads(await ws.recv())
            assert response["result"] == 8
            assert response["id"] == 1
```

## Client Usage

### JavaScript/Browser Client

```javascript
const ws = new WebSocket('ws://localhost:8000/chat?user_id=alice');

ws.onopen = () => {
  console.log('Connected');
  ws.send('Hello, everyone!');
};

ws.onmessage = (event) => {
  console.log('Received:', event.data);
};

ws.onerror = (error) => {
  console.error('WebSocket error:', error);
};

ws.onclose = (event) => {
  console.log('Disconnected:', event.code, event.reason);
};

// Send JSON
ws.send(JSON.stringify({ type: 'message', content: 'Hello' }));

// Close connection
ws.close(1000, 'Normal closure');
```

### Python Client (websockets)

```python
import asyncio
import websockets

async def client():
    async with websockets.connect('ws://localhost:8000/echo') as ws:
        await ws.send('Hello, server!')
        response = await ws.recv()
        print(f'Received: {response}')

asyncio.run(client())
```

## Performance Considerations

### Connection Limits

- Each WebSocket is a persistent connection
- Plan for connection pooling and horizontal scaling
- Use load balancers with WebSocket support (sticky sessions)

### Message Size

- Large messages should be chunked at application level
- Consider using compression for text messages
- Binary messages are more efficient than JSON for large data

### Keep-Alive

- Axum handles ping/pong automatically
- Configure timeouts at infrastructure level
- Implement application-level heartbeat if needed

## Implementation Checklist

### Rust Core
- [ ] Create `crates/spikard-http/src/websocket.rs`
- [ ] Define `WebSocketMessage` enum
- [ ] Define `WebSocketHandler` trait
- [ ] Define `WebSocketResponse` wrapper
- [ ] Add WebSocket variant to `HandlerResponse`
- [ ] Implement message loop with lifecycle hooks
- [ ] Write unit tests
- [ ] Add integration tests

### Python Bindings
- [ ] Create `crates/spikard-py/src/websocket.rs`
- [ ] Implement `WebSocketMessage` class
- [ ] Implement `WebSocketResponse` class
- [ ] Implement `PythonWebSocketHandler` trait
- [ ] Create `packages/python/spikard/websocket.py` API
- [ ] Add type hints and docstrings
- [ ] Write integration tests
- [ ] Add examples (echo, chat, JSON-RPC)

### TypeScript Bindings
- [ ] Create `crates/spikard-node/src/websocket.rs`
- [ ] Implement WebSocket classes
- [ ] Add TypeScript types
- [ ] Write integration tests
- [ ] Add examples

### Ruby Bindings
- [ ] Create `crates/spikard-rb/src/websocket.rs`
- [ ] Implement WebSocket classes
- [ ] Write tests
- [ ] Add examples

### Documentation
- [ ] Add WebSocket section to README
- [ ] Create user guide with examples
- [ ] Document client usage (browser, Python, Node.js)
- [ ] Add chat application tutorial
- [ ] Document security considerations (CORS, authentication)

## Security Considerations

### Authentication

- Authenticate WebSocket connections in `on_connect` hook
- Use query parameters or headers for tokens
- Close connection immediately if authentication fails

```python
async def on_connect():
    token = request.query_params.get("token")
    if not await verify_token(token):
        raise AuthenticationError("Invalid token")
```

### Rate Limiting

- Implement per-connection rate limiting in `on_message`
- Track message frequency and size
- Close connection if limits exceeded

### Input Validation

- Always validate and sanitize WebSocket messages
- Treat WebSocket input like HTTP request body
- Use schema validation for JSON messages

## References

- **WebSocket RFC 6455:** https://datatracker.ietf.org/doc/html/rfc6455
- **Axum WebSocket:** https://docs.rs/axum/latest/axum/extract/ws/
- **MDN WebSocket API:** https://developer.mozilla.org/en-US/docs/Web/API/WebSocket
- **websockets (Python client):** https://websockets.readthedocs.io/
- **ws (Node.js library):** https://github.com/websockets/ws

---

**Key Takeaway:** WebSocket support enables full-duplex, bidirectional communication by leveraging Axum's native protocol upgrade and message handling, with lifecycle hooks (onConnect, onMessage, onClose, onError) providing clean integration points for Python, TypeScript, and Ruby applications.

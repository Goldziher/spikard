# Streaming & Real-Time

Serve streaming responses (chunked, NDJSON), SSE, or WebSockets with production-ready patterns.

## Streaming response

=== "Python"

    --8<-- "snippets/python/streaming.md"

=== "TypeScript"

    --8<-- "snippets/typescript/streaming.md"

=== "Ruby"

    --8<-- "snippets/ruby/streaming.md"

=== "PHP"

    --8<-- "snippets/php/streaming.md"

=== "Rust"

    --8<-- "snippets/rust/streaming.md"

## Production patterns

### Client disconnect handling

Detect and handle client disconnections to avoid resource leaks:

```python
import asyncio
from spikard import Spikard, SseEvent, sse

app = Spikard()

@sse("/events")
async def events(request):
    connected_clients = 0
    try:
        connected_clients += 1
        app.logger.info(f"Client connected. Total: {connected_clients}")

        while True:
            # Check if client disconnected
            if await request.is_disconnected():
                app.logger.info("Client disconnected gracefully")
                break

            # Generate and send event
            yield SseEvent(data={"timestamp": time.time()})
            await asyncio.sleep(1)

    except asyncio.CancelledError:
        app.logger.info("Client connection cancelled")
        raise
    finally:
        connected_clients -= 1
        app.logger.info(f"Cleanup complete. Remaining: {connected_clients}")
```

### Backpressure handling

Manage slow consumers to prevent memory buildup:

```python
import asyncio
from collections import deque
from spikard import Spikard, SseEvent, sse

app = Spikard()

# Shared event queue with max size
event_queue = deque(maxlen=100)

async def event_producer():
    """Background task producing events"""
    counter = 0
    while True:
        event_queue.append({"id": counter, "data": "event data"})
        counter += 1
        await asyncio.sleep(0.1)

@sse("/events")
async def events(request):
    last_id = 0

    try:
        while True:
            if await request.is_disconnected():
                break

            # Only send if queue has new events
            if len(event_queue) > 0:
                event = event_queue[-1]
                if event["id"] > last_id:
                    yield SseEvent(data=event["data"], id=str(event["id"]))
                    last_id = event["id"]

            await asyncio.sleep(0.5)  # Throttle to prevent overwhelming client

    except asyncio.CancelledError:
        pass
```

### Error handling and recovery

Handle errors gracefully with retry logic:

```python
import asyncio
from spikard import Spikard, SseEvent, sse

app = Spikard()

@sse("/events")
async def events(request):
    retry_count = 0
    max_retries = 3

    try:
        while True:
            if await request.is_disconnected():
                break

            try:
                # Simulate fetching data that might fail
                data = await fetch_data()
                retry_count = 0  # Reset on success

                yield SseEvent(
                    data=data,
                    retry=1000,  # Client should retry after 1s
                    event="update"
                )

            except Exception as e:
                retry_count += 1
                app.logger.error(f"Error fetching data: {e}")

                if retry_count >= max_retries:
                    yield SseEvent(
                        data={"error": "Service temporarily unavailable"},
                        event="error"
                    )
                    break

                await asyncio.sleep(2 ** retry_count)  # Exponential backoff

            await asyncio.sleep(1)

    except asyncio.CancelledError:
        app.logger.info("Stream cancelled")
    finally:
        app.logger.info("Stream cleanup complete")

## WebSockets

=== "Python"

    --8<-- "snippets/python/websocket.md"

=== "TypeScript"

    --8<-- "snippets/typescript/websocket.md"

=== "Ruby"

    --8<-- "snippets/ruby/websocket.md"

=== "PHP"

    --8<-- "snippets/php/websocket.md"

=== "Rust"

    --8<-- "snippets/rust/websocket.md"

### WebSocket connection lifecycle

Production WebSocket handler with complete lifecycle management:

```python
from spikard import Spikard, websocket

app = Spikard()

@websocket("/ws")
async def chat(message: dict) -> dict | None:
    # Simple protocol: respond to pings, otherwise echo.
    if message.get("type") == "ping":
        return {"type": "pong"}
    return {"type": "echo", "data": message}


def _on_connect() -> None:
    app.logger.info("WebSocket client connected")


def _on_disconnect() -> None:
    app.logger.info("WebSocket client disconnected")


# Optional lifecycle hooks.
chat.on_connect = _on_connect
chat.on_disconnect = _on_disconnect
```

## Server-Sent Events (SSE)

=== "Python"

    --8<-- "snippets/python/sse.md"

=== "TypeScript"

    --8<-- "snippets/typescript/sse.md"

=== "Ruby"

    --8<-- "snippets/ruby/sse.md"

=== "PHP"

    --8<-- "snippets/php/sse.md"

=== "Rust"

    --8<-- "snippets/rust/sse.md"

### Production SSE with keepalive

SSE handler with keepalive, reconnection logic, and error handling:

```python
import asyncio
import time
from spikard import Spikard, SseEvent, sse

app = Spikard()

@sse("/events")
async def events(request):
    client_id = request.headers.get("X-Client-ID", "unknown")
    last_event_id = request.headers.get("Last-Event-ID", "0")

    app.logger.info(f"Client {client_id} connected, last_event_id: {last_event_id}")

    try:
        event_id = int(last_event_id)
    except ValueError:
        event_id = 0

    keepalive_interval = 15  # Send keepalive every 15s
    last_keepalive = time.time()

    try:
        while True:
            # Check for client disconnect
            if await request.is_disconnected():
                app.logger.info(f"Client {client_id} disconnected")
                break

            current_time = time.time()

            # Send keepalive comment to prevent timeout
            if current_time - last_keepalive > keepalive_interval:
                yield SseEvent(comment="keepalive")
                last_keepalive = current_time

            # Fetch and send data
            try:
                data = await fetch_event_data()
                event_id += 1

                yield SseEvent(
                    data=data,
                    id=str(event_id),
                    event="update",
                    retry=3000  # Client should retry after 3s on disconnect
                )

                last_keepalive = current_time

            except Exception as e:
                app.logger.error(f"Error fetching data: {e}")
                yield SseEvent(
                    data={"error": "Temporary error, retrying..."},
                    event="error"
                )

            await asyncio.sleep(1)

    except asyncio.CancelledError:
        app.logger.info(f"Client {client_id} stream cancelled")
        raise
    finally:
        app.logger.info(f"Client {client_id} cleanup complete")

async def fetch_event_data():
    """Simulate fetching data from a source"""
    return {"timestamp": time.time(), "value": "data"}
```

## Testing streaming handlers

### Testing SSE endpoints

```python
import pytest
from spikard.testing import TestClient

@pytest.mark.asyncio
async def test_sse_stream():
    async with TestClient(app) as client:
        # Connect to SSE endpoint
        async with client.stream("GET", "/events") as response:
            assert response.status_code == 200
            assert response.headers["content-type"] == "text/event-stream"

            # Read first few events
            events = []
            async for line in response.aiter_lines():
                if line.startswith("data:"):
                    events.append(line)
                if len(events) >= 3:
                    break

            assert len(events) == 3

@pytest.mark.asyncio
async def test_sse_reconnection():
    """Test SSE reconnection with Last-Event-ID"""
    async with TestClient(app) as client:
        headers = {"Last-Event-ID": "5"}
        async with client.stream("GET", "/events", headers=headers) as response:
            # Should resume from event ID 5
            assert response.status_code == 200
```

### Testing WebSocket handlers

```python
import pytest
from spikard.testing import TestClient

@pytest.mark.asyncio
async def test_websocket_echo():
    async with TestClient(app) as client:
        async with client.websocket_connect("/ws") as websocket:
            # Test echo
            await websocket.send_json({"type": "echo", "message": "hello"})
            response = await websocket.receive_json()
            assert response["type"] == "echo"

            # Test ping/pong
            await websocket.send_json({"type": "ping"})
            response = await websocket.receive_json()
            assert response["type"] == "pong"

@pytest.mark.asyncio
async def test_websocket_disconnect():
    """Test proper cleanup on disconnect"""
    async with TestClient(app) as client:
        async with client.websocket_connect("/ws") as websocket:
            # Get initial connection count
            await websocket.send_json({"type": "ping"})
            await websocket.receive_json()

        # Connection should be cleaned up after context exit
        # Verify by checking active_connections count in logs
```

### Testing streaming responses

```python
import pytest
from spikard.testing import TestClient

@pytest.mark.asyncio
async def test_streaming_response():
    async with TestClient(app) as client:
        async with client.stream("GET", "/stream") as response:
            assert response.status_code == 200

            chunks = []
            async for chunk in response.aiter_bytes():
                chunks.append(chunk)
                if len(chunks) >= 5:
                    break

            assert len(chunks) == 5
            assert all(len(chunk) > 0 for chunk in chunks)
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

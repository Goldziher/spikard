# Streaming & Real-Time

Serve streaming responses (chunked, NDJSON), SSE, or WebSockets.

## Streaming response

=== "Python"

    --8<-- "snippets/python/streaming.md"

=== "TypeScript"

    --8<-- "snippets/typescript/streaming.md"

=== "Ruby"

    --8<-- "snippets/ruby/streaming.md"

=== "Rust"

    --8<-- "snippets/rust/streaming.md"

## Tips
- Set appropriate content types (`application/x-ndjson`, `text/event-stream`).
- Handle client disconnects gracefully; stop producing when the connection closes.
- For WebSockets/SSE, add auth middleware before handler execution.

## WebSockets

=== "Python"

    --8<-- "snippets/python/websocket.md"

=== "Rust"

    --8<-- "snippets/rust/websocket.md"

??? note "TypeScript"
    WebSocket support exists in the runtime but the public API is still being finalized. Use HTTP/SSE for now.

=== "Ruby"

    --8<-- "snippets/ruby/websocket.md"

## Server-Sent Events (SSE)

=== "Python"

    --8<-- "snippets/python/sse.md"

=== "TypeScript"

    --8<-- "snippets/typescript/sse.md"

=== "Ruby"

    --8<-- "snippets/ruby/sse.md"

=== "Rust"

    --8<-- "snippets/rust/sse.md"

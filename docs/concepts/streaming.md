# Streaming & Async IO

Spikard treats streaming and real-time protocols as first-class citizens so the same APIs work for HTTP, WebSocket, and SSE flows.

## Capabilities
- **WebSockets**: bidirectional handlers with typed messages and backpressure-aware send/receive loops.
- **Server-Sent Events**: push event streams with graceful client disconnect handling.
- **Chunked responses**: stream files or long-running computations without buffering everything in memory.

## Concurrency Model
- Built on Tokio with cooperative scheduling
- Binding bridges expose async primitives that map to the language runtime (e.g., Python event loop thread, Node async iterators)
- Cancellation and shutdown signals propagate through middleware and handlers

## Validation for Streaming
- Envelope/message schemas validated per message when configured
- Connection-level auth and capability checks enforced in middleware before the handler executes

More details and decisions live in [ADR 0006](../adr/0006-streaming-and-async-protocols.md).

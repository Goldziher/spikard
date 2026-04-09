---
name: websocket-sse-support
priority: high
---

HTTP server must support WebSocket upgrades via Handler trait. SSE requires streaming
response bodies. Both require proper cleanup on connection termination.
Tests: websocket_integration.rs, sse_behavior.rs in /crates/spikard-http/tests/

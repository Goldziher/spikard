# Transport Edge Cases

Spikard’s HTTP runtime is designed to behave consistently across all language bindings, even when clients send “rough” traffic. This document captures the transport scenarios we explicitly test, how the runtime behaves, and the recommended ways to exercise each path.

## Request/Response Validation

### Content-Length mismatches
- **Behaviour**: Requests where the declared `Content-Length` does not match the body bytes never reach user code. The HTTP parser rejects them with a `400 Bad Request`.
- **Testing**: `e2e/python/tests/test_content_types.py::test_20_content_length_mismatch` covers this case.
- **Notes**: Because rejection happens inside the HTTP parser (hyper/h11), bindings don’t receive a hook. Log output comes from the Rust layer; handlers should not attempt to recover.

### Range / Partial Content
- **Behaviour**: Range requests (`Range: bytes=...`) return `206 Partial Content` when the static asset runner can satisfy the range; unsupported ranges receive `416`.
- **Testing**: `e2e/python/tests/test_static_files.py::test_partial_content` and the matching Node/Ruby suites.
- **Notes**: Ensure static assets include the right `Accept-Ranges` header; pre-compressed assets are not sliced.

### HEAD requests & trailing newlines
- **Behaviour**: HEAD responses share the GET handler but automatically drop the body while preserving headers/content-length. Static file fixtures were updated to keep trailing newlines so snapshots are stable.
- **Testing**: `e2e/python/tests/test_static_files.py::test_static_head_*`.

## Streaming Protocols

### Server-Sent Events (SSE)
- **Behaviour**: SSE endpoints register as long-lived GET routes that yield `text/event-stream` with `cache-control: no-cache`. The runtime keeps connections alive until the handler stops yielding.
- **Testing**:
  - AsyncAPI fixtures in `testing_data/sse/` are used to autogenerate language-specific SSE tests via `spikard generate asyncapi`.
  - New benchmarks (`benchmark-harness stream ...`) measure connection counts/event throughput.
- **How to benchmark**:
  ```bash
  # Build/run the harness against a generated Python SSE app
  cargo build --release --manifest-path tools/benchmark-harness/Cargo.toml
  ./target/release/benchmark-harness stream \
    --framework spikard-python \
    --app-dir tools/benchmark-harness/apps/spikard-python \
    --fixture testing_data/sse/statusUpdate.json \
    --connections 25 --duration 20 --warmup 5
  ```

### WebSockets
- **Behaviour**: WebSocket routes share the HTTP listener but upgrade connections and keep them pinned to the language binding’s event loop. Each language’s AsyncAPI handler scaffolding now includes WebSocket hooks.
- **Testing**:
  - AsyncAPI fixtures in `testing_data/websockets/` feed e2e suites.
  - The streaming benchmark uses `tokio-tungstenite` clients to open concurrent connections, send example payloads, and capture round-trip latency.
- **How to benchmark**:
  ```bash
  ./target/release/benchmark-harness stream \
    --framework spikard-node \
    --app-dir tools/benchmark-harness/apps/spikard-node \
    --fixture testing_data/websockets/chatMessage.json \
    --connections 50 --duration 30
  ```

### Disconnects & Timeouts
- SSE/WS handlers should expect disconnect notifications; the Rust runtime propagates cancellations (`AppError::Server`). All bindings bubble the signal to the handler future, so dropping the connection simply cancels the async task.
- Transport timeouts (e.g., client closes before SSE event flush) are recorded in streaming benchmarks as `errors`.

## Replaying Edge Cases

| Scenario | Command |
| --- | --- |
| Content-Length mismatch | `uv run pytest e2e/python/tests/test_content_types.py::test_20_content_length_mismatch` |
| HTTP Range requests | `task test:e2e:python -- test_static_files.py::test_partial_content` |
| SSE handler scaffolding | `spikard generate asyncapi testing_data/streaming/notifications.yaml --lang python --output app.py` |
| WebSocket benchmark | `benchmark-harness stream --framework spikard-python --fixture testing_data/websockets/chatMessage.json --connections 40` |

Keeping these behaviours documented helps us reason about regressions and ensures every language binding (Python/Node/Ruby/Rust/PHP) exposes the same guarantees. When new transport features are introduced (WebTransport, HTTP/3, etc.) they should be added here with explicit reproduction steps.

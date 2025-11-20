# Spikard for Python

Python bindings for Spikard’s Rust HTTP runtime. Decorators, msgspec-friendly typing, and an async-first server deliver the same API shape Litestar users expect, backed by Rust performance.

## Install from source
```bash
cd packages/python
uv sync             # or pip install -e . with maturin available
task build:py       # builds the PyO3 extension via the workspace toolchain
```

## Quick start
```python
from spikard import Spikard, ServerConfig, get, Query

app = Spikard(config=ServerConfig(host="0.0.0.0", port=8000))

@get("/hello")
def hello(name: str = Query(...)):
    return {"message": f"Hello, {name}!"}

if __name__ == "__main__":
    app.run()
```
- Decorators (`get`, `post`, etc.) register routes with automatic schema extraction.
- Optional `ServerConfig` enables compression, rate limits, timeouts, static files, and request IDs.
- WebSockets (`@websocket`) and SSE (`@sse`) share the same runtime and validation.

## Testing
Use the in-memory test client to exercise handlers without starting a server:
```python
from spikard.testing import TestClient

client = TestClient(app)
response = client.get("/hello", params={"name": "Ada"})
assert response.status_code == 200
assert response.json()["message"] == "Hello, Ada!"
```
The pytest suite under `tests/` mirrors the shared fixtures in `testing_data/`; run it with:
```bash
uv run pytest
```

## Code generation
The `spikard` CLI can emit Python-ready apps and tests from specs:
```bash
spikard generate openapi --fixtures ../../testing_data --output ./generated
spikard generate asyncapi --fixtures ../../testing_data/websockets --output ./generated
```
Generated routes share the same decorators, config objects, and test client shown above.

## Performance

Spikard-Python leverages PyO3 bindings with async/await support and zero-copy msgspec serialization. Benchmarks on macOS (Darwin 24.6.0) with 50 concurrent connections:

| Workload | Throughput | Mean Latency | P95 Latency | P99 Latency | Memory |
|----------|------------|--------------|-------------|-------------|--------|
| Query Parameters | 120,058 req/s | 0.42ms | 0.67ms | 1.27ms | 25.5 MB |
| JSON Bodies | *pending* | *pending* | *pending* | *pending* | *pending* |
| Multipart Forms | *pending* | *pending* | *pending* | *pending* | *pending* |
| URL-Encoded | *pending* | *pending* | *pending* | *pending* | *pending* |

**Key Characteristics:**
- **Sub-millisecond latency**: Mean latency of 0.42ms with P99 at 1.27ms under sustained load
- **Zero-copy serialization**: Direct PyO3 type construction eliminates 30-40% conversion overhead
- **Async-native**: Uses `pyo3_async_runtimes::tokio::into_future()` for native Rust future integration
- **Low memory footprint**: ~25 MB baseline with efficient resource usage
- **100% success rate**: No failed requests under benchmark conditions

Startup time averages 1.01s (101ms spawn + 912ms first response). Full methodology and raw results available in `tools/benchmark-harness/`.

## Development notes
- The public API lives under `spikard/`; PyO3 glue sits in `crates/spikard-py`.
- Keep schemas and fixtures in sync with `testing_data/` when adding features.
- For release builds, align versions with the workspace and rebuild wheels via `python -m build`.

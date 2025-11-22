# Spikard for Python

> **⚠️ Alpha Software**: Spikard is currently in alpha. APIs may change, and the library should not be considered production-stable. Use at your own risk in production environments.

High-performance Python web framework built on Rust. Spikard combines familiar Python decorators and async/await patterns with a Rust HTTP runtime, delivering exceptional performance through PyO3 bindings, zero-copy JSON conversion, and Tower middleware.

## Performance

Benchmarks measured on Apple M4 Pro (14 cores, 48GB RAM) with 100 concurrent connections over 10 seconds per workload using oha. All tests achieved 100% success rates.

### JSON Request/Response Performance

Tests measure JSON serialization/deserialization across varying payload sizes (86 bytes to 150 KB).

**Average Performance:**
- **Throughput**: 17,584 req/s
- **Mean Latency**: 5.69ms
- **P95 Latency**: 8.14ms
- **P99 Latency**: 9.76ms
- **Memory**: 26.8 MB (stable across all workloads)

**Detailed Breakdown by Payload Size:**

| Payload Size | Throughput | Mean Latency | P99 Latency |
|--------------|------------|--------------|-------------|
| Small (86 bytes) | 17,699 req/s | 5.65ms | 9.78ms |
| Medium (1.5 KB) | 17,904 req/s | 5.58ms | 9.86ms |
| Large (15 KB) | 17,775 req/s | 5.62ms | 9.59ms |
| Very Large (150 KB) | 16,958 req/s | 5.90ms | 9.80ms |

**Key Findings:**
- Performance remains stable across payload sizes
- Memory usage extremely stable at ~27 MB across all workloads
- Uses zero-copy serialization via msgspec
- PyO3 with async/await and direct msgspec integration
- 9.4x faster when using the Rust binding directly (see main README for comparison)

## Installation

```bash
uv add spikard
```

Or with pip:
```bash
pip install spikard
```

## Quick Start

```python
from spikard import Spikard, get

app = Spikard()

@get("/hello")
async def hello(name: str):
    return {"message": f"Hello, {name}!"}

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8000)
```

Start the server:
```bash
python app.py
```

## Examples

### Path Parameters

```python
from spikard import Spikard, get

app = Spikard()

@get("/users/{user_id}")
async def get_user(user_id: int):
    return {"user_id": user_id, "name": "Alice"}
```

### Query Parameters

```python
from spikard import Spikard, get, Query

app = Spikard()

@get("/search")
async def search(q: str = Query(...), limit: int = Query(10)):
    return {"query": q, "limit": limit, "results": []}
```

### JSON Request Bodies

```python
from spikard import Spikard, post
from typing import Optional

app = Spikard()

@post("/users")
async def create_user(name: str, email: str, age: Optional[int] = None):
    return {"id": 123, "name": name, "email": email, "age": age}
```

### Async Operations

```python
from spikard import Spikard, get
import asyncio

app = Spikard()

@get("/slow")
async def slow_endpoint():
    await asyncio.sleep(1)  # Simulate async I/O
    return {"status": "completed"}
```

### Server Configuration

```python
from spikard import Spikard, ServerConfig

config = ServerConfig(
    host="0.0.0.0",
    port=8000,
    workers=4,
    compression=True,
    request_timeout=30,
)

app = Spikard(config=config)

@app.get("/")
async def root():
    return {"status": "ok"}

if __name__ == "__main__":
    app.run()
```

### Testing Your Application

```python
from spikard import Spikard, get
from spikard.testing import TestClient

app = Spikard()

@get("/health")
async def health():
    return {"status": "healthy"}

# Test without starting a server
client = TestClient(app)
response = client.get("/health")
assert response.status_code == 200
assert response.json() == {"status": "healthy"}
```

## Features

- **Fast**: Rust-powered HTTP runtime with zero-copy JSON conversion
- **Familiar**: Python decorators and async/await patterns
- **Type-Safe**: Automatic request validation and schema extraction
- **Modern**: Built on Tokio, Axum, and Tower middleware
- **Async-First**: Native async/await support with optimized coroutine handling
- **Testable**: In-memory test client for unit testing without server startup

## Architecture

Spikard achieves high performance through:

- **Zero-Copy JSON**: Direct PyO3 object construction from `serde_json::Value` without string serialization (30-40% faster than `json.loads`)
- **Optimized Async**: Uses `pyo3_async_runtimes::tokio::into_future()` for direct coroutine conversion without spawn_blocking overhead
- **Handler Abstraction**: Zero-cost FFI boundaries with clean separation between Rust HTTP server and Python handlers
- **Tower Middleware**: All middleware (compression, timeouts, rate limiting) implemented in Rust for maximum efficiency

## Documentation

Full documentation including API reference, advanced usage patterns, and deployment guides will be available soon as a dedicated documentation site. For now, refer to the examples above and explore the type hints in your IDE.

## Development Status

Spikard is under active development. Current status:

- ✅ Core HTTP server and routing
- ✅ Request/response handling with automatic validation
- ✅ Async handler support
- ✅ Test client
- ✅ Basic server configuration
- 🚧 WebSockets and Server-Sent Events
- 🚧 Advanced middleware configuration
- 🚧 OpenAPI documentation generation
- 📋 Comprehensive documentation site

## Contributing

Spikard is open source and contributions are welcome! Visit the [GitHub repository](https://github.com/Goldziher/spikard) for source code, issue tracking, and contribution guidelines.

## License

MIT License - see LICENSE file for details.

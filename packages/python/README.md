# Spikard Python

High-performance Python web framework backed by a Rust core. Build REST APIs, WebSockets, and SSE services with FastAPI/Litestar-style decorators powered by Tokio, Hyper, and Tower middleware.

[![PyPI](https://img.shields.io/pypi/v/spikard.svg?logo=python&logoColor=white)](https://pypi.org/project/spikard/)
[![Docs](https://img.shields.io/badge/docs-spikard.dev-58FBDA)](https://spikard.dev)
[![Python](https://img.shields.io/pypi/pyversions/spikard.svg?logo=python&logoColor=white)](https://pypi.org/project/spikard/)
[![codecov](https://codecov.io/gh/Goldziher/spikard/graph/badge.svg?token=H4ZXDZ4A69)](https://codecov.io/gh/Goldziher/spikard)

## Installation

```bash
pip install spikard
```

Pre-built wheels available for macOS, Linux, Windows. Building from source requires Rust 1.75+.

**Development:**
```bash
cd packages/python
uv sync
```

**Requirements:** Python 3.10+

## Quick Start

```python
from spikard import Spikard
from msgspec import Struct

class User(Struct):
    id: int
    name: str
    email: str

app = Spikard()

@app.get("/users/{user_id}")
async def get_user(user_id: int) -> User:
    return User(id=user_id, name="Alice", email="alice@example.com")

@app.post("/users")
async def create_user(user: User) -> User:
    # Automatic validation via msgspec
    return user

if __name__ == "__main__":
    app.run(port=8000)
```

## Features

- **Multiple route styles:** FastAPI-style (`@app.get()`) or Litestar-style (`@get()`)
- **Automatic validation:** msgspec (default), Pydantic v2, dataclasses, TypedDict, NamedTuple
- **Request/response streaming:** WebSockets, Server-Sent Events, multipart uploads
- **Middleware stack:** Compression, rate limiting, request IDs, authentication, CORS
- **Async-first:** Full async/await with `pyo3_async_runtimes`
- **OpenAPI generation:** Automatic type introspection and documentation
- **Dependency injection:** Configurable container with singleton and factory support

## Core Concepts

**Route Decorators:**
```python
from spikard import Spikard, get, post

app = Spikard()

@app.get("/users/{user_id}")
async def get_user(user_id: int):
    return {"id": user_id}

@post("/users")  # Standalone decorator style
async def create_user(user: User):
    return user
```

**Validation with msgspec (recommended):**
```python
from msgspec import Struct

class User(Struct):
    name: str
    email: str

@app.post("/users")
async def create_user(user: User):
    return user  # Automatic validation
```

**Dependency Injection:**
```python
from spikard.di import Provide

class DatabasePool:
    async def fetch(self, sql: str) -> list: ...

app.provide(DatabasePool, Provide(create_pool, singleton=True))

@app.get("/data")
async def get_data(pool: DatabasePool) -> dict:
    return {"data": await pool.fetch("SELECT * FROM items")}
```

**WebSockets:**
```python
from spikard import websocket

@websocket("/ws")
async def chat(message: dict) -> dict | None:
    return {"echo": message}
```

**Server-Sent Events:**
```python
from spikard import sse

@sse("/events")
async def stream():
    for i in range(10):
        yield {"count": i}
```

**Lifecycle Hooks:**
```python
@app.pre_validation
async def check_auth(request):
    if not request.headers.get("authorization"):
        return Response({"error": "Unauthorized"}, 401)
    return request
```

## Configuration

```python
from spikard import Spikard, ServerConfig, CompressionConfig, RateLimitConfig, JwtConfig

config = ServerConfig(
    host="0.0.0.0",
    port=8080,
    workers=4,
    compression=CompressionConfig(gzip=True, brotli=True),
    rate_limit=RateLimitConfig(per_second=100, burst=200),
    jwt=JwtConfig(secret="key", algorithm="HS256")
)

app = Spikard(config=config)
```

See the [Configuration Guide](../../docs/python-configuration.md) for all options.

## Performance

Benchmarked across 34 workloads at 100 concurrency ([methodology](../../docs/benchmarks/methodology.md)):

| Framework | Avg RPS | P50 (ms) | P99 (ms) |
|-----------|--------:|----------:|----------:|
| **spikard** | 12,623 | 5.55 | 38.39 |
| litestar | 8,032 | 14.62 | 19.18 |
| fastapi | 6,418 | 16.43 | 21.72 |
| robyn | 6,012 | 16.85 | 24.18 |

Spikard is **1.6x faster** than Litestar, **2.0x faster** than FastAPI, and **2.1x faster** than Robyn (also Rust-backed).

Key optimizations:
- Zero-copy PyO3 type conversion (no JSON round-trips)
- Rust-powered HTTP server (Tokio + Hyper)
- GIL-friendly async design with `pyo3_async_runtimes`

## Testing
```python
from spikard.testing import TestClient

async def test_users():
    async with TestClient(app) as client:
        response = await client.get("/users/123")
        assert response.status_code == 200
        assert response.json()["user_id"] == 123
```

`TestClient` uses in-process Rust testing for speed. `LiveTestClient` starts a real subprocess server for WebSocket/SSE tests.

See [Testing Guide](../../docs/python-testing.md) for WebSocket and SSE testing.
## Examples

Runnable examples with dependency injection and database integration:
- [Python examples](../../examples/python/)
- [GraphQL schema support](../../examples/schemas/chat-service.asyncapi.yaml)
- [OpenAPI code generation](../../examples/schemas/todo-api.openapi.yaml)

See [examples/README.md](../../examples/README.md) for all languages and code generation.

## Documentation

- [Main README](../../README.md)
- [Contributing Guide](../../CONTRIBUTING.md)
- [Architecture Decision Records](../../docs/adr/)

## Other Languages

- **Rust:** [Crates.io](https://crates.io/crates/spikard)
- **TypeScript:** [npm (@spikard/node)](https://www.npmjs.com/package/@spikard/node)
- **Ruby:** [RubyGems](https://rubygems.org/gems/spikard)
- **PHP:** [Packagist](https://packagist.org/packages/spikard/spikard)

## License

MIT

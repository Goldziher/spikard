# Spikard Python

High-performance Python web framework backed by a Rust core. Build REST APIs, WebSockets, and SSE services with FastAPI/Litestar-style decorators powered by Tokio, Hyper, and Tower middleware.

[![Documentation](https://img.shields.io/badge/docs-spikard.dev-blue)](https://spikard.dev)
[![Crates.io](https://img.shields.io/crates/v/spikard.svg?color=blue)](https://crates.io/crates/spikard)
[![PyPI](https://img.shields.io/pypi/v/spikard.svg?color=blue)](https://pypi.org/project/spikard/)
[![npm](https://img.shields.io/npm/v/@spikard/node.svg?color=blue)](https://www.npmjs.com/package/@spikard/node)
[![Gem](https://img.shields.io/gem/v/spikard.svg?color=blue)](https://rubygems.org/gems/spikard)
[![Packagist](https://img.shields.io/packagist/v/spikard/spikard.svg?color=blue)](https://packagist.org/packages/spikard/spikard)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](../../LICENSE)

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
# Note: standalone decorators require app.include_router(get_default_router()) before app.run()
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

def create_pool() -> DatabasePool:
    return DatabasePool()

app.provide(DatabasePool, Provide(create_pool, singleton=True))

@app.get("/data")
async def get_data(pool: DatabasePool) -> dict:
    return {"data": await pool.fetch("SELECT * FROM items")}
```

**WebSockets:**
```python
from spikard import websocket

@app.websocket("/ws")
def chat_endpoint():
    async def handler(message: dict) -> dict | None:
        return {"echo": message}
    return handler
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

See the documentation for all options.

## Performance

Benchmarked across 34 workloads at 100 concurrency ([methodology](../../docs/benchmarks/methodology.md)):

| Framework | Avg RPS | P50 (ms) | P99 (ms) |
|-----------|--------:|----------:|----------:|
| **spikard** | 12,623 | 5.55 | 38.39 |
| litestar | 8,032 | 14.62 | 19.18 |
| fastapi | 6,418 | 16.43 | 21.72 |
| robyn | 6,012 | 16.85 | 24.18 |

Spikard is **1.6x faster** than Litestar (throughput-based; see full results for latency breakdown), **2.0x faster** than FastAPI, and **2.1x faster** than Robyn (also Rust-backed).

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
        assert response.json()["id"] == 123
```

`TestClient` uses in-process Rust testing for speed. `LiveTestClient` starts a real subprocess server for WebSocket/SSE tests.

See the main documentation for WebSocket and SSE testing.
## Examples

Runnable examples with dependency injection and database integration:
- [Python examples](../../examples/python/)
- [GraphQL schema support](../../examples/schemas/chat-service.asyncapi.yaml)
- [OpenAPI code generation](../../examples/schemas/todo-api.openapi.yaml)

See [examples/README.md](../../examples/README.md) for all languages and code generation.

## Documentation

Full documentation at [spikard.dev](https://spikard.dev). See also [CONTRIBUTING.md](../../CONTRIBUTING.md).

## Other Languages

- **Rust:** [Crates.io](https://crates.io/crates/spikard)
- **TypeScript:** [npm (@spikard/node)](https://www.npmjs.com/package/@spikard/node)
- **Ruby:** [RubyGems](https://rubygems.org/gems/spikard)
- **PHP:** [Packagist](https://packagist.org/packages/spikard/spikard)

## License

MIT

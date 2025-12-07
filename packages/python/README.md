# Spikard Python

High-performance Python web framework with a Rust core. Build REST APIs, WebSockets, and SSE services with FastAPI/Litestar-style decorators backed by Axum and Tower-HTTP.

## Badges

[![Documentation](https://img.shields.io/badge/docs-spikard.dev-58FBDA)](https://spikard.dev)
[![PyPI](https://img.shields.io/pypi/v/spikard.svg?logo=python&logoColor=white)](https://pypi.org/project/spikard/)
[![Downloads](https://img.shields.io/pypi/dm/spikard.svg)](https://pypi.org/project/spikard/)
[![Python](https://img.shields.io/pypi/pyversions/spikard.svg?logo=python&logoColor=white)](https://pypi.org/project/spikard/)
[![codecov](https://codecov.io/gh/Goldziher/spikard/graph/badge.svg?token=H4ZXDZ4A69)](https://codecov.io/gh/Goldziher/spikard)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/spikard.svg?logo=rust)](https://crates.io/crates/spikard)
[![npm](https://img.shields.io/npm/v/spikard.svg?logo=npm)](https://www.npmjs.com/package/spikard)
[![RubyGems](https://img.shields.io/gem/v/spikard.svg?logo=ruby&logoColor=white)](https://rubygems.org/gems/spikard)
[![Packagist](https://img.shields.io/packagist/v/spikard/spikard.svg?logo=php&logoColor=white)](https://packagist.org/packages/spikard/spikard)
[![Discord](https://img.shields.io/discord/1234567890?logo=discord&logoColor=white&label=community)](https://discord.gg/pXxagNK2zN)

## Installation

**Via pip:**

```bash
pip install spikard
```

Pre-built wheels are available for macOS, Linux, and Windows. If a wheel isn't available for your platform, pip will build from source (requires Rust toolchain).

**From source (development):**

```bash
cd packages/python
uv sync
# or
pip install -e .
```

**Requirements:**
- Python 3.10+
- Rust toolchain (only required for building from source)

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

## Core Features

### Route Registration

Spikard supports both **FastAPI-style** (instance decorators) and **Litestar-style** (standalone decorators) patterns.

**FastAPI-style (instance decorators):**

```python
from spikard import Spikard

app = Spikard()

@app.get("/users")
async def list_users():
    return {"users": []}

@app.post("/users")
async def create_user(user: User):
    return user
```

**Litestar-style (standalone decorators):**

```python
from spikard import Spikard, get, post, put, patch, delete

app = Spikard()

@get("/users")
async def list_users():
    return {"users": []}

@post("/users")
async def create_user(user: User):
    return user

@put("/users/{user_id}")
async def update_user(user_id: int, user: User):
    return user

@patch("/users/{user_id}")
async def patch_user(user_id: int, updates: dict):
    return updates

@delete("/users/{user_id}")
async def delete_user(user_id: int):
    return {"deleted": True}
```

**All HTTP methods supported:**
- `@app.get()` / `@get()` - GET requests
- `@app.post()` / `@post()` - POST requests
- `@app.put()` / `@put()` - PUT requests
- `@app.patch()` / `@patch()` - PATCH requests
- `@app.delete()` / `@delete()` - DELETE requests
- `@app.head()` / `@head()` - HEAD requests
- `@app.options()` / `@options()` - OPTIONS requests
- `@app.trace()` / `@trace()` - TRACE requests

### Path Parameters

```python
@app.get("/users/{user_id}")
async def get_user(user_id: int):
    return {"id": user_id}

@app.get("/posts/{post_id}/comments/{comment_id}")
async def get_comment(post_id: int, comment_id: int):
    return {"post_id": post_id, "comment_id": comment_id}
```

### Query Parameters

```python
from spikard import Query

@app.get("/search")
async def search(
    q: str,
    limit: int = Query(default=10),
    offset: int = Query(default=0)
):
    return {"query": q, "limit": limit, "offset": offset}
```

### Request Body Validation

Spikard supports multiple validation libraries. **msgspec is the default and recommended** for best performance.

**With msgspec.Struct (recommended - fastest):**

```python
from msgspec import Struct

class CreatePost(Struct):
    title: str
    content: str
    tags: list[str] = []

@app.post("/posts")
async def create_post(post: CreatePost):
    return {"title": post.title, "tag_count": len(post.tags)}
```

**Supported validation libraries:**
- **msgspec.Struct** (default, zero-copy, fastest)
- **Pydantic v2** BaseModel
- **dataclasses**

### Dependency Injection

Register values or factories and inject by parameter name:

```python
from spikard import Spikard
from spikard.di import Provide

app = Spikard()
app.provide("config", {"db_url": "postgresql://localhost/app"})
app.provide("db", Provide(lambda config: f"pool({config['db_url']})", depends_on=["config"], singleton=True))

@app.get("/stats")
async def stats(config: dict[str, str], db: str):
    return {"db": db, "env": config["db_url"]}
```
- **TypedDict**
- **NamedTuple**
- **attrs** classes

**With Pydantic v2:**

```python
from pydantic import BaseModel, EmailStr

class User(BaseModel):
    name: str
    email: EmailStr

@app.post("/users")
async def create_user(user: User):
    return user.model_dump()
```

**With dataclasses:**

```python
from dataclasses import dataclass

@dataclass
class Product:
    name: str
    price: float

@app.post("/products")
async def create_product(product: Product):
    return product
```

**With plain JSON Schema dict:**

```python
user_schema = {
    "type": "object",
    "properties": {
        "name": {"type": "string"},
        "email": {"type": "string", "format": "email"}
    },
    "required": ["name", "email"]
}

@app.post("/users", request_schema=user_schema)
async def create_user(user: dict):
    # user is validated against schema
    return user
```

### File Uploads

```python
from spikard import UploadFile

@app.post("/upload")
async def upload_file(file: UploadFile):
    # Use aread() for async file reading
    content = await file.aread()
    return {
        "filename": file.filename,
        "size": file.size,
        "content_type": file.content_type,
        "content_length": len(content)
    }
```

### Custom Responses

```python
from spikard import Response

@app.post("/users")
async def create_user(user: User) -> Response:
    return Response(
        content=user,
        status_code=201,
        headers={"X-Custom": "value"}
    )
```

### Streaming Responses

```python
from spikard import StreamingResponse

async def generate_data():
    for i in range(10):
        yield f"data: {i}\n".encode()

@app.get("/stream")
async def stream():
    return StreamingResponse(generate_data())
```

## Configuration

```python
from spikard import Spikard, ServerConfig, CompressionConfig, RateLimitConfig

config = ServerConfig(
    host="0.0.0.0",
    port=8080,
    workers=4,
    enable_request_id=True,
    max_body_size=10 * 1024 * 1024,  # 10 MB
    request_timeout=30,
    compression=CompressionConfig(
        gzip=True,
        brotli=True,
        quality=6
    ),
    rate_limit=RateLimitConfig(
        per_second=100,
        burst=200
    )
)

app = Spikard(config=config)
```

### Middleware Configuration

**Compression:**

```python
from spikard import CompressionConfig

compression = CompressionConfig(
    gzip=True,          # Enable gzip
    brotli=True,        # Enable brotli
    min_size=1024,      # Min bytes to compress
    quality=6           # 0-11 for brotli, 0-9 for gzip
)
```

**Rate Limiting:**

```python
from spikard import RateLimitConfig

rate_limit = RateLimitConfig(
    per_second=100,     # Max requests per second
    burst=200,          # Burst allowance
    ip_based=True       # Per-IP rate limiting
)
```

**JWT Authentication:**

```python
from spikard import JwtConfig

jwt = JwtConfig(
    secret="your-secret-key",
    algorithm="HS256",  # HS256, HS384, HS512, RS256, etc.
    audience=["api.example.com"],
    issuer="auth.example.com",
    leeway=30  # seconds
)
```

**Static Files:**

```python
from spikard import StaticFilesConfig

static = StaticFilesConfig(
    directory="./public",
    route_prefix="/static",
    index_file=True,
    cache_control="public, max-age=3600"
)

config = ServerConfig(static_files=[static])
```

**OpenAPI Documentation:**

```python
from spikard.config import OpenApiConfig

openapi = OpenApiConfig(
    enabled=True,
    title="My API",
    version="1.0.0",
    description="API documentation",
    swagger_ui_path="/docs",
    redoc_path="/redoc"
)

config = ServerConfig(openapi=openapi)
```

## Lifecycle Hooks

```python
@app.on_request
async def log_request(request):
    print(f"{request.method} {request.path}")
    return request  # Must return request to continue

@app.pre_validation
async def check_auth(request):
    token = request.headers.get("Authorization")
    if not token:
        return Response({"error": "Unauthorized"}, status_code=401)
    return request

@app.pre_handler
async def rate_check(request):
    # Additional checks before handler
    return request

@app.on_response
async def add_headers(response):
    response.headers["X-Frame-Options"] = "DENY"
    return response

@app.on_error
async def log_error(response):
    print(f"Error: {response.status_code}")
    return response
```

## WebSockets

```python
from spikard import Spikard, websocket

app = Spikard()

@websocket("/ws")
async def chat_handler(message: dict) -> dict | None:
    """Handle incoming WebSocket messages."""
    print(f"Received: {message}")
    # Echo back the message
    return {"echo": message}
```

## Server-Sent Events (SSE)

```python
from spikard import Spikard, sse
import asyncio

app = Spikard()

@sse("/events")
async def event_stream():
    """Stream events to connected clients."""
    for i in range(10):
        await asyncio.sleep(1)
        yield {
            "count": i,
            "message": f"Event {i}",
            "timestamp": i
        }
```

## Background Tasks

```python
from spikard import background

async def heavy_processing(data: dict):
    """Async function that runs after response is sent."""
    # Heavy processing here
    pass

@app.post("/process")
async def process_data(data: dict):
    # Schedule async function to run in background
    background.run(heavy_processing(data))
    return {"status": "processing"}
```

## Testing

```python
from spikard import TestClient
import pytest

@pytest.fixture
def client():
    return TestClient(app)

@pytest.mark.asyncio
async def test_get_user(client):
    response = await client.get("/users/123")
    assert response.status_code == 200
    data = response.json()
    assert data["id"] == 123

@pytest.mark.asyncio
async def test_create_user(client):
    response = await client.post("/users", json={
        "name": "Alice",
        "email": "alice@example.com"
    })
    assert response.status_code == 201
```

### WebSocket Testing

```python
import json

@pytest.mark.asyncio
async def test_websocket(client):
    async with client.websocket("/ws") as ws:
        await ws.send(json.dumps({"message": "hello"}))
        response_text = await ws.recv()
        response = json.loads(response_text)
        assert response["echo"]["message"] == "hello"
```

### SSE Testing

```python
@pytest.mark.asyncio
async def test_sse(client):
    async with client.sse("/events") as sse:
        events = []
        async for event in sse:
            events.append(event.data)
            if len(events) >= 3:
                break
        assert len(events) == 3
```

## Type Support

Spikard automatically extracts JSON schemas from:

- **msgspec.Struct** (recommended, fastest)
- **Pydantic v2 BaseModel**
- **dataclasses**
- **TypedDict**
- **NamedTuple**

All compile to JSON Schema for validation and OpenAPI generation.

## Performance

### vs FastAPI
Comprehensive comparison across 18 real-world workloads on Apple M4 Pro (100 concurrent connections):

| Framework | Avg Throughput | Mean Latency | Difference |
|-----------|----------------|--------------|------------|
| **Spikard** | **35,779 req/s** | **7.44ms** | baseline |
| FastAPI | 12,776 req/s | 7.90ms | **-64% slower** |

**Spikard is 2.8x faster than FastAPI** with statistically significant improvements (p < 0.05).

### Why Spikard is Faster

**Rust-Powered Core:**
- HTTP server built on Tokio and Hyper
- Tower middleware for zero-overhead routing
- No Python GIL contention for HTTP layer

**Zero-Copy Optimizations:**
- Direct PyO3 type construction (no JSON string serialization)
- Eliminates 30-40% conversion overhead vs serialize-then-parse
- msgspec integration for validation without extra allocations

**Async-First Design:**
- `pyo3_async_runtimes` for efficient coroutine handling
- Single event loop initialization per worker
- GIL release before awaiting Rust futures

## Running the Server

```python
# Development
app.run(host="127.0.0.1", port=8000)

# Production with multiple workers
config = ServerConfig(
    host="0.0.0.0",
    port=8080,
    workers=4
)
app.run(config=config)
```

## Examples

The [examples directory](../../examples/) contains comprehensive demonstrations:

**Python-specific examples:**
- [Dependency Injection](../../examples/di/python_basic.py) - Basic DI patterns
- [Database Integration](../../examples/di/python_database.py) - DI with database pools
- Additional examples in [examples/](../../examples/)

**API Schemas** (language-agnostic, can be used with code generation):
- [Todo API](../../examples/schemas/todo-api.openapi.yaml) - REST CRUD with validation
- [File Service](../../examples/schemas/file-service.openapi.yaml) - File uploads/downloads
- [Auth Service](../../examples/schemas/auth-service.openapi.yaml) - JWT, API keys, OAuth
- [Chat Service](../../examples/schemas/chat-service.asyncapi.yaml) - WebSocket messaging
- [Event Streams](../../examples/schemas/events-stream.asyncapi.yaml) - SSE streaming

See [examples/README.md](../../examples/README.md) for code generation instructions.

## Documentation

- [Main Project README](../../README.md)
- [Contributing Guide](../../CONTRIBUTING.md)
- [Architecture Decision Records](../../docs/adr/)
- [Python Examples](../../examples/python/)
- [GitHub Discussions](https://github.com/Goldziher/spikard/discussions)

## Ecosystem Links

- Rust: [Crates.io](https://crates.io/crates/spikard)
- Node.js: [npm (@spikard/node)](https://www.npmjs.com/package/@spikard/node)
- Ruby: [RubyGems](https://rubygems.org/gems/spikard)
- PHP: [Packagist](https://packagist.org/packages/spikard/spikard)
- WebAssembly: [npm (@spikard/wasm)](https://www.npmjs.com/package/@spikard/wasm)

## License

MIT

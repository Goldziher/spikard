# Spikard Python

High-performance Python web framework with a Rust core. Build REST APIs, WebSockets, and SSE services with FastAPI/Litestar-style decorators backed by Axum and Tower-HTTP.

## Installation

**From source (currently):**

```bash
cd packages/python
uv sync
# or
pip install -e .
```

**Requirements:**
- Python 3.11+
- Rust toolchain (for building from source)

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
    content = file.read()
    return {
        "filename": file.filename,
        "size": file.size,
        "content_type": file.content_type
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
from spikard import OpenApiConfig

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
from spikard import websocket
from typing import Any

@app.websocket("/ws")
class ChatHandler:
    async def on_connect(self):
        print("Client connected")

    async def handle_message(self, message: dict[str, Any]) -> dict[str, Any] | None:
        # Echo back the message
        return {"echo": message}

    async def on_disconnect(self):
        print("Client disconnected")
```

## Server-Sent Events (SSE)

```python
from spikard import sse, SseEvent

@app.sse("/events")
class EventProducer:
    def __init__(self):
        self.count = 0

    async def on_connect(self):
        print("SSE client connected")

    async def next_event(self) -> SseEvent | None:
        if self.count >= 10:
            return None  # End stream

        event = SseEvent(
            data={"count": self.count},
            event_type="update",
            id=str(self.count),
            retry_ms=3000
        )
        self.count += 1
        return event

    async def on_disconnect(self):
        print("SSE client disconnected")
```

## Background Tasks

```python
from spikard import background

@app.post("/process")
async def process_data(data: dict):
    background.run(lambda: heavy_processing(data))
    return {"status": "processing"}

def heavy_processing(data):
    # Runs after response is sent
    pass
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
@pytest.mark.asyncio
async def test_websocket(client):
    async with client.websocket("/ws") as ws:
        await ws.send_json({"message": "hello"})
        response = await ws.receive_json()
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

Python bindings use:
- **PyO3** for zero-copy FFI
- **msgspec** for ultra-fast serialization
- **pyo3_async_runtimes** for native async/await
- Direct Python object construction (no JSON round-trip)

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

See `/examples/python/` for more examples including:
- REST APIs with validation
- WebSocket chat
- SSE notifications
- File uploads
- Authentication
- Background tasks

## Documentation

- [Main Project README](../../README.md)
- [Contributing Guide](../../CONTRIBUTING.md)
- [Architecture Decision Records](../../docs/adr/)

## License

MIT

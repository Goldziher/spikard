# Python Binding

Spikard's Python binding uses PyO3 with msgspec-first validation. Decorators feel like FastAPI/Litestar while the Rust core handles routing, middleware, and streaming.

## Quickstart

```python
from spikard import Spikard
from msgspec import Struct

class User(Struct):
    id: int
    name: str

app = Spikard()

@app.get("/users/{id:int}")
async def get_user(id: int) -> User:
    return User(id=id, name="Alice")

if __name__ == "__main__":
    app.run(port=8000)
```

## Router

Use `Router` for modular route organization:

```python
from spikard.routing import Router

users = Router(prefix="/users")

@users.get("/{user_id}")
async def get_user(user_id: int) -> dict:
    return {"id": user_id}

app.include_router(users)
```

## Validation
- **msgspec (default)**: fastest; use `Struct` types for request/response validation.
- **Pydantic v2 / dataclasses / TypedDict / attrs**: auto-detected when used as handler params.

```python
from spikard import Body
from msgspec import Struct

class Payment(Struct):
    id: str
    amount: float

@app.post("/payments")
async def create_payment(payment: Body[Payment]) -> Payment:
    return payment
```

## Dependency Injection

Type-based injection (recommended):

```python
from spikard.di import Provide

class DatabasePool:
    pass

app.provide(DatabasePool, Provide(create_pool, singleton=True))

@app.get("/data")
async def get_data(pool: DatabasePool) -> dict:
    return {"status": "ok"}
```

## Lifecycle hooks

```python
@app.on_request
async def logging_hook(request: dict[str, object]):
    print(f"{request['method']} {request['path']}")
    return request
```

## Async Server

```python
import asyncio

async def main():
    await app.serve(host="0.0.0.0", port=8080)

asyncio.run(main())
```

## Testing

```python
from spikard.testing import TestClient

async def test_endpoint():
    async with TestClient(app) as client:
        response = await client.get("/data")
        assert response.status_code == 200
```

## Requests & Responses

### Request Parameters

Use parameter decorators to extract and validate request data:

=== "Query Parameters"
    ```python
    from spikard import Query

    @app.get("/search")
    async def search(q: Query[str], limit: Query[int] = Query(default=10)):
        return {"query": q, "limit": limit}
    ```

=== "Path Parameters"
    ```python
    from spikard import Path

    @app.get("/users/{user_id}")
    async def get_user(user_id: Path[int]):
        return {"id": user_id}
    ```

=== "Headers"
    ```python
    from spikard import Header

    @app.get("/info")
    async def get_info(x_token: Header[str]):
        return {"token": x_token}
    ```

=== "Cookies"
    ```python
    from spikard import Cookie

    @app.get("/profile")
    async def get_profile(session_id: Cookie[str]):
        return {"session": session_id}
    ```

### Response Types

Return typed responses or use Response/StreamingResponse for custom control:

=== "Typed Response"
    ```python
    from msgspec import Struct

    class User(Struct):
        id: int
        name: str

    @app.get("/users/{id}")
    async def get_user(id: int) -> User:
        return User(id=id, name="Alice")
    ```

=== "Response"
    ```python
    from spikard import Response

    @app.get("/download")
    async def download():
        return Response(
            content=b"file data",
            media_type="application/octet-stream",
            headers={"Content-Disposition": 'attachment; filename="file.bin"'}
        )
    ```

=== "StreamingResponse"
    ```python
    from spikard import StreamingResponse

    async def generate():
        for i in range(100):
            yield b"chunk %d\n" % i

    @app.get("/stream")
    async def stream_data():
        return StreamingResponse(generate(), media_type="text/plain")
    ```

## Configuration

Control server behavior with ServerConfig:

=== "Basic Config"
    ```python
    from spikard import Spikard, ServerConfig

    config = ServerConfig(
        host="0.0.0.0",
        port=8000,
        workers=4,
    )

    app = Spikard(config=config)
    ```

=== "Compression"
    ```python
    from spikard import ServerConfig
    from spikard.config import CompressionConfig

    config = ServerConfig(
        compression=CompressionConfig(
            gzip=True,
            brotli=True,
            min_size=2048,  # Only compress >= 2KB
            quality=9,  # Max compression
        )
    )
    ```

=== "Rate Limiting"
    ```python
    from spikard import ServerConfig
    from spikard.config import RateLimitConfig

    config = ServerConfig(
        rate_limit=RateLimitConfig(
            per_second=100,
            burst=200,
            ip_based=True,
        )
    )
    ```

=== "JWT Authentication"
    ```python
    from spikard import ServerConfig
    from spikard.config import JwtConfig

    config = ServerConfig(
        jwt_auth=JwtConfig(
            secret="your-secret-key",
            algorithm="HS256",
            audience=["api.example.com"],
            issuer="auth.example.com",
            leeway=10,  # 10 seconds tolerance
        )
    )
    ```

=== "API Key Authentication"
    ```python
    from spikard import ServerConfig
    from spikard.config import ApiKeyConfig

    config = ServerConfig(
        api_key_auth=ApiKeyConfig(
            keys=["secret-key-1", "secret-key-2"],
            header_name="X-API-Key",
        )
    )
    ```

=== "Static Files"
    ```python
    from spikard import ServerConfig
    from spikard.config import StaticFilesConfig

    config = ServerConfig(
        static_files=[
            StaticFilesConfig(
                directory="./public",
                route_prefix="/static",
                cache_control="public, max-age=3600",
            ),
        ]
    )
    ```

=== "OpenAPI Documentation"
    ```python
    from spikard import ServerConfig
    from spikard.config import (
        OpenApiConfig,
        ServerInfo,
        ContactInfo,
        LicenseInfo,
    )

    config = ServerConfig(
        openapi=OpenApiConfig(
            enabled=True,
            title="My API",
            version="1.0.0",
            description="API documentation",
            servers=[
                ServerInfo(url="https://api.example.com", description="Production"),
                ServerInfo(url="http://localhost:8000", description="Development"),
            ],
            contact=ContactInfo(
                name="API Support",
                email="support@example.com",
            ),
            license=LicenseInfo(
                name="MIT",
                url="https://opensource.org/licenses/MIT",
            ),
        )
    )
    ```

## File Uploads

Accept multipart file uploads with UploadFile:

```python
from dataclasses import dataclass
from spikard import Spikard
from spikard.datastructures import UploadFile

app = Spikard()

@dataclass
class FileUpload:
    file: UploadFile
    description: str

@app.post("/upload")
async def upload_file(body: FileUpload):
    content = body.file.read()  # or await body.file.aread()
    return {
        "filename": body.file.filename,
        "size": body.file.size,
        "content_type": body.file.content_type,
        "description": body.body.description,
    }
```

UploadFile supports both sync and async operations:
- `read(size=-1)` / `aread(size=-1)` - Read file contents
- `write(data)` / `awrite(data)` - Write data
- `seek(offset, whence=0)` / `aseek(offset, whence=0)` - Seek to position
- `close()` / `aclose()` - Close file
- `as_bytes_io()` - Get BytesIO object
- `rolled_to_disk` - Check if spooled to disk (files > 1MB by default)

## WebSocket Support

Define WebSocket handlers with the @websocket decorator:

```python
from spikard import Spikard, websocket

app = Spikard()

@app.websocket("/chat")
async def chat_handler(message: dict) -> dict | None:
    return {"echo": message}
```

WebSocket handlers receive JSON messages and can return dicts to send as responses:
- The handler is called with the parsed JSON `message`
- Return a `dict` to send a JSON response, or `None` to send nothing

## Server-Sent Events (SSE)

Stream events to clients with the @sse decorator:

```python
from spikard import Spikard, sse
import asyncio

app = Spikard()

@app.sse("/notifications")
async def notifications():
    for i in range(10):
        await asyncio.sleep(1)
        yield {"message": f"Notification {i}", "count": i}
```

SSE handlers are async generators that yield event dicts. Each dict is sent as a Server-Sent Event with JSON serialization.

## gRPC Support

Implement gRPC handlers for protobuf services:

```python
from spikard.grpc import GrpcHandler, GrpcRequest, GrpcResponse
import user_pb2

class UserServiceHandler(GrpcHandler):
    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        # Deserialize protobuf
        req = user_pb2.GetUserRequest()
        req.ParseFromString(request.payload)

        # Process
        user = user_pb2.User(id=req.id, name="John Doe")

        # Serialize and return
        return GrpcResponse(payload=user.SerializeToString())

    async def handle_server_stream(self, request: GrpcRequest):
        # Server streaming RPC
        req = user_pb2.StreamRequest()
        req.ParseFromString(request.payload)

        for item in get_items(req):
            response = user_pb2.StreamResponse(data=item)
            yield GrpcResponse(payload=response.SerializeToString())

    async def handle_client_stream(self, request_stream):
        # Client streaming RPC
        items = []
        async for request in request_stream:
            req = user_pb2.StreamItem()
            req.ParseFromString(request.payload)
            items.append(req)

        result = aggregate_items(items)
        response = user_pb2.AggregateResponse(data=result)
        return GrpcResponse(payload=response.SerializeToString())

    async def handle_bidi_stream(self, request_stream):
        # Bidirectional streaming RPC
        async for request in request_stream:
            req = user_pb2.BidiRequest()
            req.ParseFromString(request.payload)

            response_data = await process_bidi_item(req)
            response = user_pb2.BidiResponse(data=response_data)
            yield GrpcResponse(payload=response.SerializeToString())
```

GrpcRequest attributes:
- `payload` - Serialized protobuf bytes
- `method_name` - Name of the method called
- `service_name` - Name of the service
- `metadata` - Request metadata dict

GrpcResponse attributes:
- `payload` - Serialized protobuf response bytes
- `metadata` - Optional response metadata dict

## Testing

### TestClient (In-Process)

Fast, in-process testing using Rust directly:

```python
from spikard.testing import TestClient

async def test_get_user():
    async with TestClient(app) as client:
        response = await client.get("/users/1")
        assert response.status_code == 200
        assert response.json() == {"id": 1, "name": "Alice"}
```

TestResponse methods:
- `status_code` - HTTP status code
- `headers` - Response headers dict
- `bytes()` - Response body as bytes
- `text()` - Response body as text
- `json()` - Response body parsed as JSON
- `assert_status(code)` - Assert status code (chainable)
- `assert_status_ok()` - Assert status is 200 (chainable)

### LiveTestClient (Real Server)

Start a real server for specialized testing:

```python
from spikard.testing import LiveTestClient

async def test_with_real_server():
    async with LiveTestClient(app) as client:
        response = await client.get("/users/1")
        assert response.status_code == 200
```

LiveTestClient starts a real server in a subprocess, useful for testing server behavior, port binding, and signal handling.

## Deployment

- Local: `python app.py` or `await app.serve()`.
- Production: build with the binding and set `SPIKARD_PORT`/`SPIKARD_HOST` via env.

## Event Loop Integration

The binding uses `pyo3_async_runtimes` to convert Python coroutines directly to Tokio futures, eliminating the overhead of a dedicated event loop thread. See [Python architecture](../python/architecture.md).

## Troubleshooting

- Ensure Python 3.10+ and Rust toolchain are installed.
- If you see import errors, rebuild with `maturin develop` or `task build:py`.

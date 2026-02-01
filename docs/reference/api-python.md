# Python API Reference

The Python binding exposes the Rust runtime through a thin, Pythonic surface.

## Package
- Install: `pip install spikard`
- Entry module: `spikard`

## Core Types
- `Spikard` - register routes and run the server
- `Router` - modular route collection with prefix support
- Route decorators (`@app.get`, `@app.post`, ...) - built on msgspec type hints
- Lifecycle hooks (`on_request`, `pre_validation`, `pre_handler`, `on_response`, `on_error`)
- Validation powered by msgspec (with optional Pydantic/attrs/dataclass detection)
- Dependency injection via `app.provide` and `spikard.di.Provide`
- `TestClient` / `LiveTestClient` for testing

## Routing
```python
from spikard import Spikard

app = Spikard()

@app.get("/health")
async def health() -> dict:
    return {"status": "ok"}
```

## Router
```python
from spikard.routing import Router

api = Router(prefix="/api")

@api.get("/health")
async def health() -> dict:
    return {"ok": True}

app.include_router(api)
```

## Dependency Injection

Type-based (recommended):
```python
from spikard.di import Provide

class DatabasePool:
    pass

app.provide(DatabasePool, Provide(create_pool, singleton=True))

@app.get("/data")
async def handler(pool: DatabasePool) -> dict:
    return {"status": "ok"}
```

String-based (legacy):
```python
app.provide("config", {"db_url": "postgresql://localhost/app"})
```

## Typed Request Parameters
```python
from spikard import Body, Query, Header
from msgspec import Struct

class CreateUser(Struct):
    name: str
    email: str

@app.post("/users")
async def create_user(user: Body[CreateUser], verbose: Query[bool] = Query(default=False)) -> dict:
    return {"name": user.name}
```

## Testing
```python
from spikard.testing import TestClient

async def test_health():
    async with TestClient(app) as client:
        response = await client.get("/health")
        assert response.status_code == 200
```

## Async Server
```python
async def main():
    await app.serve(host="0.0.0.0", port=8080)
```

## Middleware
Use lifecycle hooks for cross-cutting behavior:
```python
@app.on_request
async def log_request(request):
    print(f"{request['method']} {request['path']}")
    return request
```

## Validation
Use msgspec `Struct` types for automatic request/response validation. Pydantic/dataclasses can be detected when enabled.

## Event Loop
The binding uses `pyo3_async_runtimes` to convert Python coroutines directly to Tokio futures, eliminating the overhead of a dedicated event loop thread. See [Python architecture](../python/architecture.md).

More detailed, generated docs can be added with `mkdocstrings` once docstrings are in place.

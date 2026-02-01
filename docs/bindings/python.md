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
- Typed params: `Query`, `Path`, `Header`, `Cookie`, `Body` helpers available.
- Bodies: `Body[msgspec.Struct]` (recommended) or other supported types.
- Responses: return Python objects/Structs; the runtime serializes.

## Deployment
- Local: `python app.py` or `await app.serve()`.
- Production: build with the binding and set `SPIKARD_PORT`/`SPIKARD_HOST` via env.

## Troubleshooting
- Ensure Python 3.10+ and Rust toolchain are installed.
- If you see import errors, rebuild with `maturin develop` or `task build:py`.

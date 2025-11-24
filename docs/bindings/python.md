# Python Binding

Spikard’s Python binding uses PyO3 with msgspec-first validation. Decorators feel like FastAPI/Litestar while the Rust core handles routing, middleware, and streaming.

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

## Validation
- **msgspec (default)**: fastest; use `Struct` types for request/response validation.
- **Pydantic v2 / dataclasses / TypedDict / attrs**: auto-detected when used as handler params.

```python
from msgspec import Struct

class Payment(Struct):
    id: str
    amount: float

@app.post("/payments")
async def create_payment(payment: Payment) -> Payment:
    return payment
```

## Middleware

```python
def logging_middleware(ctx, next_fn):
    print(f"{ctx.method} {ctx.path}")
    return next_fn()

app.use(logging_middleware)
```

## Requests & Responses
- Typed params: `Query`, `Path`, `Header`, `Cookie` helpers available.
- Bodies: `msgspec.Struct` (recommended) or other supported types.
- Responses: return Python objects/Structs; the runtime serializes.

## Deployment
- Local: `spikard run app.py` or `python app.py`.
- Production: build with the CLI or embed routes in Rust; set `SPIKARD_PORT`/`SPIKARD_HOST` via env.

## Troubleshooting
- Ensure Python 3.11+ and Rust toolchain are installed.
- If you see GIL-related errors, upgrade `spikard` and re-run `uv sync`; the binding uses a dedicated asyncio loop.

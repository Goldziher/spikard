# Python API Reference

The Python binding mirrors the Rust runtime while exposing Pythonic ergonomics via PyO3.

## Package
- Install: `pip install spikard`
- Entry module: `spikard`

## Core Types
- `App` – register routes, middleware, and error handlers
- `Context` – access request data (path params, query, headers, cookies, body)
- `Response` helpers – JSON responses, redirects, streams
- Validation helpers powered by msgspec (with optional Pydantic detection)

## Routing
```python
from spikard import App

app = App()

@app.get("/health")
async def health() -> dict:
    return {"status": "ok"}
```

## Middleware
```python
def logging_middleware(ctx, next_fn):
    print(f"Handling {ctx.method} {ctx.path}")
    return next_fn()

app.use(logging_middleware)
```

## Validation
Use msgspec `Struct` types for automatic request/response validation. Pydantic/dataclasses can be detected when enabled.

## Event Loop
The binding runs handlers on a dedicated Python event loop thread while the Rust runtime stays async (see [Python architecture](../python/architecture.md)).

More detailed, generated docs can be added with `mkdocstrings` once docstrings are in place.

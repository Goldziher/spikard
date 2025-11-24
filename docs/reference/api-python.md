# Python API Reference

The Python binding exposes the Rust runtime through a thin, Pythonic surface.

## Package
- Install: `pip install spikard`
- Entry module: `spikard`

## Core Types
- `Spikard` – register routes and run the server
- Route decorators (`@app.get`, `@app.post`, …) – built on msgspec type hints
- Lifecycle hooks (`on_request`, `pre_validation`, `pre_handler`, `on_response`, `on_error`)
- Validation powered by msgspec (with optional Pydantic/attrs/dataclass detection)
- Dependency injection via `app.provide` and `spikard.di.Provide`

## Routing
```python
from spikard import Spikard

app = Spikard()

@app.get("/health")
async def health() -> dict:
    return {"status": "ok"}
```

## Dependency Injection
```python
from spikard.di import Provide

app.provide("config", {"db_url": "postgresql://localhost/app"})
app.provide("db_pool", Provide(lambda config: f"pool({config['db_url']})", depends_on=["config"], singleton=True))
```

## Middleware
Use lifecycle hooks for cross-cutting behavior:
```python
from spikard import Spikard

app = Spikard()

@app.on_request
async def log_request(request):
    print(f"{request['method']} {request['path']}")
    return request
```

## Validation
Use msgspec `Struct` types for automatic request/response validation. Pydantic/dataclasses can be detected when enabled.

## Event Loop
The binding runs handlers on a dedicated Python event loop thread while the Rust runtime stays async (see [Python architecture](../python/architecture.md)).

More detailed, generated docs can be added with `mkdocstrings` once docstrings are in place.

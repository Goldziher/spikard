# spikard (Python)

User-facing Python package that wraps the `_spikard` PyO3 bindings and offers an ergonomic API for declaring routes, validators, and typed payloads.

## Layout
- `spikard/app.py` exposes the application builder used by `spikard-cli` to discover routes.
- `spikard/routing.py`, `params.py`, and `types.py` capture msgspec-backed request/response schemas.
- Tests under `tests/` exercise fixture-driven scenarios shared with the Rust HTTP runtime.

## Development
- Manage the environment with `uv sync` (see `pyproject.toml` and `uv.lock`).
- Run the Python suite via `uv run pytest`.
- Rebuild the native module by executing `task build:py` when the underlying Rust bindings change.

## Testing helpers
The `spikard.testing` module provides two complementary clients:

- `TestClient` (default) spins up the app in a subprocess and issues real HTTP
  requests via `httpx`. Use this when you want to exercise the full TCP stack
  (e.g. Range requests, timeouts, or load balancer behavior).
- `InProcessTestClient` routes requests through the same Rust `axum-test`
  harness used by the Ruby/Node bindings. It is ideal for transport-sensitive
  fixtures where the subprocess client would short-circuit (such as deliberately
  broken `Content-Length` headers).

```python
from spikard.testing import InProcessTestClient, TestClient

async def test_ok():
    async with TestClient(app) as client:
        resp = await client.get("/users/42")
        assert resp.status_code == 200

async def test_transport_edge_case():
    async with InProcessTestClient(app) as client:
        resp = await client.post("/data", headers={"Content-Length": "100"}, json={"value": "short"})
        assert resp.status_code == 400
```

The two clients can coexist inside the same test suite—pick the one that best
matches the scenario you need to reproduce.

## Publishing
- Wheels rely on the Hatchling backend; invoke `python -m build` (or `hatch build` if Hatch is installed) once the package is ready for distribution. Keep versions aligned with the workspace release cadence.

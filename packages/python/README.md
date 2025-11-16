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

## Publishing
- Wheels rely on the Hatchling backend; invoke `python -m build` (or `hatch build` if Hatch is installed) once the package is ready for distribution. Keep versions aligned with the workspace release cadence.

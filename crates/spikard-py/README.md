# spikard-py

PyO3 bindings that expose the Rust HTTP runtime to Python applications under the module name `_spikard`.

## Build & Installation
- Use `uv run maturin develop` for editable installs during development (`task build:py` runs the release variant).
- The resulting wheel re-exports route extraction helpers consumed by `spikard-cli` and the Python package in `packages/python`.

## Development Notes
- Rust code lives in `src/lib.rs` and must stay panic-free; convert errors into `PyErr` variants before crossing the FFI boundary.
- Keep feature flags aligned with the `pyproject.toml` configuration (`extension-module` is enabled by default).
- Validate changes with the Python integration tests: `uv run pytest tests/`.

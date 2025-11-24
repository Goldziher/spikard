# CLI Usage

`spikard-cli` boots the Rust HTTP server and wires in language bindings. It also drives code generation for DTOs and handlers.

## Install
```bash
cargo install spikard-cli
```

## Run an App
Point the CLI at your binding entrypoint:
```bash
spikard run path/to/app.py --host 0.0.0.0 --port 8000
```
- Serves routes provided by the Python binding (similar commands for other bindings)
- Auto-reload and multi-worker flags are parsed today and will be enabled incrementally

## Generate Code
Use the CLI to generate DTOs/handlers from contracts (OpenAPI/AsyncAPI):
```bash
spikard generate openapi ./openapi.yaml --lang python --output ./generated
spikard generate openapi ./openapi.yaml --lang typescript --output ./generated
```
Generated code stays aligned with the runtime schemas and fixture-based tests.

## Development Notes
- Build locally with `cargo build -p spikard-cli` or `task build:cli`.
- End-to-end scenarios run via `cargo run --package spikard-cli -- run examples/app.py`.
- The CLI inherits workspace lint/format settings; run `cargo fmt`/`cargo clippy` before publishing.

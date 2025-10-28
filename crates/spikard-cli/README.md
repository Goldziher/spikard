# spikard-cli

CLI entrypoint that boots the Rust HTTP server, loads Python apps through `_spikard`, and serves routes extracted from the binding layer.

## Usage
- `spikard run path/to/app.py --host 0.0.0.0 --port 8000` spins up Axum with handlers provided by the Python package.
- Auto-reload and multi-worker flags are parsed today but still print warnings until implemented.

## Development
- Compile with `cargo build -p spikard-cli` or `task build:cli`.
- Run end-to-end scenarios with `cargo run --package spikard-cli -- run examples/app.py`.
- Apply `cargo fmt`/`cargo clippy` before publishing; the crate inherits workspace lint settings.

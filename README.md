# Spikard

A multi-language package built with Rust, targeting Python, Node.js, and WebAssembly.

## Structure

- `crates/spikard` - Core Rust library
- `crates/spikard-cli` - Command-line interface
- `crates/spikard-http` - HTTP server
- `crates/spikard-py` - Python bindings (PyO3)
- `crates/spikard-node` - Node.js bindings (napi-rs)
- `crates/spikard-wasm` - WebAssembly bindings (wasm-bindgen)
- `packages/python/tests` - Fixture-driven integration tests backed by `testing_data/`

## Development

### Prerequisites

- Rust (2024 edition)
- Python 3.10+
- Node.js 18+
- pnpm
- uv (Python package manager)
- Task (task runner)

### Setup

```bash
task setup
```

### Building

Common build targets are exposed via the Taskfile:

```bash
task build:rust   # Build the Rust workspace
task build:py     # Build the PyO3 bindings
task build:node   # Build the Node.js bindings
task build:wasm   # Build the WASM bindings
task build:js     # Build all JavaScript/TypeScript packages
```

### Testing

```bash
task test        # Run Rust and Python suites with CI parity
task test:rust   # Rust-only checks
task test:python # Pytest suite (uses PYTHONPATH=. under the hood)
```

### Running

```bash
# Run CLI
task run:cli

# Run HTTP server
task run:http
```

## Taskfile quick reference

All automation lives in the root `Taskfile.yaml` and runs from the repository root:

- `task setup` – install toolchains and build the Python bindings once.
- `task build` – composite build for Rust, Python, and JavaScript targets.
- `task test` – execute Rust and Python tests just like CI.
- `task lint` – format and lint across languages (`cargo fmt`, `clippy`, `ruff`, etc.).

Custom commands automatically set `PYTHONPATH=.`, so the tasks can be copied directly into your shell without extra environment setup.

## License

MIT

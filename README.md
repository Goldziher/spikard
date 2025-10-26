# Spikard

A multi-language package built with Rust, targeting Python, Node.js, and WebAssembly.

## Structure

- `crates/spikard` - Core Rust library
- `crates/spikard-cli` - Command-line interface
- `crates/spikard-http` - HTTP server
- `crates/spikard-py` - Python bindings (PyO3)
- `crates/spikard-node` - Node.js bindings (napi-rs)
- `crates/spikard-wasm` - WebAssembly bindings (wasm-bindgen)

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

```bash
# Build Rust workspace
task build:rust

# Build Python bindings
task build:py

# Build Node.js bindings
task build:node

# Build WASM bindings
task build:wasm

# Build all JavaScript/TypeScript packages
task build:js
```

### Testing

```bash
# Run all tests
task test

# Run Rust tests only
task test:rust

# Run Python tests only
task test:python
```

### Running

```bash
# Run CLI
task run:cli

# Run HTTP server
task run:http
```

## License

MIT

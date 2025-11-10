# Spikard

A multi-language package built with Rust, targeting Python, Node.js, Ruby, and WebAssembly.

## Features

### Core HTTP Framework
- [x] Request/Response handling (path, query, headers, cookies, body)
- [x] JSON Schema validation (Draft 2020-12)
- [x] Format validation (UUID, date, datetime, email, URI, IPv4/IPv6)
- [x] CORS support (preflight, origin/method/header validation)
- [x] Multipart/form-data handling
- [x] URL-encoded form handling
- [x] RFC 9457 Problem Details error responses
- [x] Type hints in routes (`/items/{id:uuid}`, `/users/{id:int}`)

### Middleware & Performance
- [x] Request ID generation (UUID-based, X-Request-ID)
- [x] Response compression (gzip, brotli)
- [x] Request timeouts (configurable)
- [x] Body size limits (configurable max size)
- [x] Rate limiting (IP-based, configurable)
- [x] Graceful shutdown (SIGTERM/SIGINT)
- [x] Static file serving (with cache-control)
- [x] Sensitive header hiding (Authorization, Cookie)
- [ ] JWT authentication middleware
- [ ] API Key authentication middleware

### Advanced Features
- [ ] OpenAPI 3.1.0 generation
- [ ] Swagger UI integration
- [ ] Redoc integration
- [ ] WebSocket support
- [ ] Server-Sent Events (SSE)
- [ ] Streaming responses
- [ ] Background tasks
- [ ] Test client

### Language Bindings
- [x] Python (PyO3) - Full support
- [x] Node.js (napi-rs) - Full support
- [x] Ruby (Magnus) - Full support
- [x] WebAssembly (wasm-bindgen) - Basic support
- [ ] Python: Typed config forwarding
- [ ] Node.js: Typed config forwarding
- [ ] Ruby: Typed config forwarding

### CLI & Code Generation
- [x] OpenAPI to handler generation
- [x] Multi-language code generation (Python, Node, Ruby, Rust)
- [x] Fixture-based testing
- [ ] AsyncAPI support (WebSocket generation)

### Testing & Benchmarking
- [x] Fixture-driven integration tests
- [x] Python e2e tests
- [x] Node.js e2e tests
- [x] Ruby e2e tests
- [x] Benchmark harness
- [x] Performance benchmarks (Python, Node, Ruby)
- [ ] WebSocket benchmarks
- [ ] SSE benchmarks

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

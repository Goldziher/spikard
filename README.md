# Spikard

Rust-powered, multi-language web toolkit. Spikard’s first pillar is an HTTP framework that speaks REST, WebSockets, SSE, and streaming—with the same developer experience translated into Python, Node/Bun, Ruby, and WebAssembly.

## Principles
- **Correctness first**: follow IETF specs and emerging drafts for HTTP, CORS, cookies, validation, and problem details.
- **Performance by default**: built on high-performance Rust libraries (axum, hyper, tower) with zero-copy bindings where possible.
- **Code generation is table stakes**: the CLI generates apps and tests from OpenAPI and AsyncAPI today, with JSON-RPC and protobuf on deck.
- **Simplicity & parity**: one comparative API, adapted to each language’s idioms without re-inventing features differently per ecosystem.

## Inspirations
- Python: Litestar (authored here) for declarative routing, typing, and validation.
- Node/Bun: Fastify’s opt-in performance and plugin mentality.
- Ruby: block-friendly routing and Rack-style ergonomics.

## Architecture
- **Core runtime (Rust)**: `crates/spikard-core` for shared types, `crates/spikard-http` for the HTTP server and middleware, plus streaming, WebSocket, and SSE support.
- **Bindings**: PyO3 (`crates/spikard-py`), napi-rs (`crates/spikard-node`), Magnus (`crates/spikard-rb`), wasm-bindgen (`crates/spikard-wasm`). Language packages live under `packages/`.
- **CLI & codegen**: `crates/spikard-cli` and `crates/spikard-codegen` drive `spikard generate openapi|asyncapi …` and fixture-aware scaffolding.
- **Fixtures & e2e**: `testing_data/` is the single source of truth; e2e suites in `e2e/{python,node,wasm,ruby,rust}` exercise the same scenarios across bindings.

## What’s Included
- REST routing with typed params (`/users/{id:uuid}`), JSON/schema validation, and RFC 9457 problem details.
- Middleware: compression (gzip/brotli), rate limiting, request timeouts, body size limits, request IDs, static files, and CORS.
- Streaming: chunked responses, SSE producers, WebSocket handlers.
- Lifecycle hooks: on-request, pre-validation, pre-handler, on-response, on-error.
- Test clients: in-memory clients for Python/Node/Ruby/WASM reuse the Rust request pipeline.

## Code Generation
Spikard treats spec-driven workflows as first-class:
- **OpenAPI → routes**: `spikard generate openapi --fixtures testing_data --output …`
- **AsyncAPI → streaming**: `spikard generate asyncapi --fixtures testing_data/websockets --output …`
- **Planned**: JSON-RPC and protobuf pipelines mirror the same surface.

All generators rely on the same validators and schemas used by the e2e suites, keeping contracts aligned across languages.

## Language Targets
- **Python** (`packages/python`): decorators, dataclass/msgspec-friendly typing, async-first server, and in-memory test client.
- **Node/Bun** (`packages/node`): TypeScript-native API, typed config and routing helpers, napi-backed server, Vitest-ready test client.
- **Ruby** (`packages/ruby`): Sinatra-like blocks for routes, Magnus-backed runtime, Rack-friendly responses, Minitest/RSpec test client.
- **WebAssembly** (`packages/wasm`): edge-friendly TypeScript build with the same routing primitives and test client; fetch-handler helpers for workers.

## Getting Started (from source)
1) Install Rust toolchain, pnpm, bundler, and a Python 3.11+ environment.
2) Bootstrap deps (`pnpm install`, `bundle install` in `packages/ruby`, `uv sync` in `packages/python` if you use uv).
3) Build the bindings you need (e.g., `task build:py`, `task build:node`, `task build:ruby`, or `cargo build -p spikard-node`).
4) Run tests for a target (`task test:python`, `task test:node`, `task test:ruby`, `task test:wasm`, `cargo test -p spikard-http`). See `Taskfile.yaml` for shortcuts.

## Repository Layout
- `crates/` — Rust core, HTTP runtime, CLI, and bindings.
- `packages/` — Published language packages (Python/Node/Ruby/WASM).
- `e2e/` — Cross-language end-to-end suites generated from fixtures.
- `testing_data/` — Canonical fixtures and schemas powering validation, generators, and e2e tests.
- `tools/` — Generators, benchmark harness, and dev utilities.

## Contributing
- Keep fixtures authoritative: add/update scenarios under `testing_data/` and reuse them in e2e suites.
- Prefer codegen paths when adding routes or protocols; keep CLI surfaces consistent across bindings.
- Target correctness first—favor explicit validation and standards alignment even when optimizing hot paths.

## Performance

Benchmarks measured on macOS (Darwin 24.6.0) with 50 concurrent connections over 10 seconds. All tests achieved 100% success rates.

### Overall Performance Summary

| Binding | Throughput | Mean Latency | P95 Latency | P99 Latency | Memory | Startup |
|---------|------------|--------------|-------------|-------------|--------|---------|
| **Rust** | 165,228 req/s | 0.30ms | 0.36ms | 0.45ms | 17.4 MB | 1.01s |
| **Python** | 120,058 req/s | 0.42ms | 0.67ms | 1.27ms | 25.5 MB | 1.01s |
| Node | *pending* | *pending* | *pending* | *pending* | *pending* | *pending* |
| Ruby | *pending* | *pending* | *pending* | *pending* | *pending* | *pending* |
| WASM | *pending* | *pending* | *pending* | *pending* | *pending* | *pending* |

### Performance by Workload Type

#### JSON Request/Response
| Binding | Throughput | Mean Latency | P99 Latency |
|---------|------------|--------------|-------------|
| **Rust** | 160,989 req/s | 0.31ms | 0.45ms |
| **Python** | 5,796 req/s | 8.63ms | 15.32ms |
| Node | *pending* | *pending* | *pending* |
| Ruby | *pending* | *pending* | *pending* |

#### Multipart Form Data
| Binding | Throughput | Mean Latency | P99 Latency |
|---------|------------|--------------|-------------|
| Rust | *pending* | *pending* | *pending* |
| Python | *pending* | *pending* | *pending* |
| Node | *pending* | *pending* | *pending* |
| Ruby | *pending* | *pending* | *pending* |

#### URL-Encoded Forms
| Binding | Throughput | Mean Latency | P99 Latency |
|---------|------------|--------------|-------------|
| Rust | *pending* | *pending* | *pending* |
| Python | *pending* | *pending* | *pending* |
| Node | *pending* | *pending* | *pending* |
| Ruby | *pending* | *pending* | *pending* |

#### Query Parameters
| Binding | Throughput | Mean Latency | P99 Latency |
|---------|------------|--------------|-------------|
| **Rust** | 165,228 req/s | 0.30ms | 0.45ms |
| **Python** | 120,058 req/s | 0.42ms | 1.27ms |
| Node | *pending* | *pending* | *pending* |
| Ruby | *pending* | *pending* | *pending* |

**Notes:**
- Query parameters: Rust shows ~38% higher throughput than Python (165K vs 120K req/s)
- JSON bodies: Rust shows ~28× higher throughput than Python (161K vs 5.8K req/s)
- Python binding uses PyO3 with async/await and zero-copy msgspec serialization
- Memory footprint remains low (<30 MB) across all bindings
- Full benchmark methodology and raw results: `tools/benchmark-harness/results/`

## Roadmap Highlights
- JSON-RPC and protobuf generators alongside OpenAPI/AsyncAPI.
- Additional edge adapters and streaming benchmarks.
- More language parity work as new middleware lands in `crates/spikard-http`.

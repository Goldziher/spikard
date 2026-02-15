# Spikard Elixir Examples

Practical examples demonstrating Spikard's Elixir bindings with Rustler NIFs.

## Prerequisites

- Elixir 1.18+ with OTP 27+
- Rust toolchain (for NIF compilation)

## Running Examples

Each example is a standalone Elixir script:

```bash
cd examples/elixir

# Basic server
elixir 01_basic_server.exs

# Validation and request handling
elixir 02_validation.exs

# Streaming and SSE
elixir 03_streaming.exs

# WebSocket and SSE real-time
elixir 04_websocket_sse.exs

# Lifecycle hooks (auth, logging)
elixir 05_lifecycle_hooks.exs
```

## Examples

| File | Description | Port |
|------|-------------|------|
| `01_basic_server.exs` | Hello world, health check, echo | 8000 |
| `02_validation.exs` | Path params, query params, body validation | 8001 |
| `03_streaming.exs` | Streaming responses, SSE producer | 8002 |
| `04_websocket_sse.exs` | WebSocket chat, SSE notifications | 8003 |
| `05_lifecycle_hooks.exs` | Auth hooks, logging, security headers | 8004 |

## Testing

```bash
# From repository root
task test:elixir

# E2E tests
task test:e2e:elixir
```

## Learn More

- [Elixir Binding Documentation](../../docs/bindings/elixir.md)
- [Package README](../../packages/elixir/README.md)
- [HexDocs](https://hexdocs.pm/spikard)

# Spikard Documentation

Spikard is a polyglot API toolkit with a Rust core and first-class bindings for Python, TypeScript/Node, Ruby, and Rust. It gives every language the same router, middleware stack, validation engine, and streaming support so teams can build HTTP, JSON-RPC, or Protobuf services without re-learning framework conventions per runtime.

## What You Can Build

- **Consistent APIs across languages** – idiomatic bindings, shared routing/validation semantics, and compatible middleware in every language.
- **Protocol-agnostic services** – REST today with JSON-RPC and protobuf generation on deck, backed by a shared schema/validation layer.
- **Strong typing end-to-end** – JSON Schema + language-native types (msgspec, Zod, RBS) enforced in both request parsing and responses.
- **Real-time and streaming** – WebSocket and SSE support with a unified async model across bindings.
- **Code generation** – CLI-driven DTO/handler generation from OpenAPI/AsyncAPI keeps contract tests and bindings aligned.

## Documentation Map

- **[Getting Started](getting-started/quickstart.md)** – First route in each language and the minimum runtime wiring.
- **[Installation](getting-started/installation.md)** – How to install the Rust core, bindings, and CLI.
- **[Guides](guides/routing.md)** – Routing, requests/responses, validation, middleware, deployment, and benchmarks.
- **[Concepts](concepts/architecture.md)** – Architecture, runtime model, validation, middleware, and streaming internals.
- **[Reference](reference/api-python.md)** – Language APIs, configuration surface, types, and error semantics.
- **[CLI](cli/usage.md)** – Running the HTTP server and invoking generators from `spikard-cli`.
- **[ADRs](adr/README.md)** – Design history and reasoning behind the runtime.

## Supported Bindings

| Binding / Interface | Package | Docs |
|--------------------|---------|------|
| Python             | `pip install spikard` | [Python API Reference](reference/api-python.md) |
| TypeScript/Node.js | `npm install spikard` | [TypeScript API Reference](reference/api-typescript.md) |
| Ruby               | `gem install spikard` | [Ruby API Reference](reference/api-ruby.md) |
| Rust               | `cargo add spikard` | [Rust API Reference](reference/api-rust.md) |
| CLI                | `cargo install spikard-cli` | [CLI Usage](cli/usage.md) |

## Getting Help

- **Questions / bugs**: open an issue at [github.com/Goldziher/spikard](https://github.com/Goldziher/spikard).
- **Chat**: join the community Discord (`https://discord.gg/pXxagNK2zN`).
- **Contributing**: see [Contributing](contributing.md) for coding standards, environment setup, and testing instructions.

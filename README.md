<!-- GENERATED FILE — DO NOT EDIT DIRECTLY. Run: task readme:generate -->

# Spikard

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <a href="https://spikard.dev">
    <img src="https://img.shields.io/badge/docs-spikard.dev-007ec6" alt="Documentation">
  </a>
  <a href="https://crates.io/crates/spikard">
    <img src="https://img.shields.io/crates/v/spikard.svg?color=007ec6" alt="Crates.io">
  </a>
  <a href="https://pypi.org/project/spikard/">
    <img src="https://img.shields.io/pypi/v/spikard.svg?color=007ec6" alt="PyPI">
  </a>
  <a href="https://www.npmjs.com/package/@spikard/node">
    <img src="https://img.shields.io/npm/v/@spikard/node.svg?color=007ec6" alt="npm">
  </a>
  <a href="https://rubygems.org/gems/spikard">
    <img src="https://img.shields.io/gem/v/spikard.svg?color=007ec6" alt="RubyGems">
  </a>
  <a href="https://packagist.org/packages/spikard/spikard">
    <img src="https://img.shields.io/packagist/v/spikard/spikard.svg?color=007ec6" alt="Packagist">
  </a>
  <a href="https://hex.pm/packages/spikard">
    <img src="https://img.shields.io/hexpm/v/spikard.svg?color=007ec6" alt="Hex.pm">
  </a>
  <a href="https://github.com/Goldziher/spikard/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-007ec6" alt="License">
  </a>
</div>

Rust-centric, codegen-first framework for building typed web services and generating idiomatic server scaffolds for Python, TypeScript, Rust, Ruby, PHP, and Elixir.

## Features
- **Codegen-first CLI** for OpenAPI 3.1, AsyncAPI 3.0, OpenRPC/JSON-RPC, GraphQL, and Protobuf/gRPC
- **Agent-friendly MCP server** with CLI-equivalent project init and code generation tools
- **Real starter projects** via `spikard init` for Python, TypeScript, Rust, Ruby, PHP, and Elixir
- **Shared Rust runtime** with Tower HTTP middleware, WebSockets, SSE, and gRPC support
- **Quality-validated output** with formatter, syntax, and lint/type checks for generated code

## Supported Languages

| Language | Runtime / Toolchain | Package Manager |
|----------|---------------------|-----------------|
| Rust | 1.85+ | cargo |
| Python | 3.10+ | pip / uv |
| TypeScript | 5.x | npm / pnpm / yarn |
| Ruby | 3.2+ | gem / bundler |
| PHP | 8.2+ | composer |
| Elixir | 1.18+ / OTP 27+ | mix / hex |

## Installation

```bash
cargo install spikard-cli
```

## Quick Start

Install the CLI, scaffold a real project, and generate typed handlers:

```bash
spikard init my_api --lang python --dir .
cd my_api
uv sync
uv run python -m my_api.app
```

Generate code from a schema with the current command surface:

```bash
# OpenAPI 3.1
spikard generate openapi examples/schemas/auth-service.openapi.yaml --lang python --output ./generated.py

# GraphQL
spikard generate graphql examples/schemas/social.graphql --lang typescript --output ./src/generated.ts

# Protobuf / gRPC
spikard generate protobuf examples/schemas/user-service.proto --lang rust --output ./src/generated.rs
```

## MCP

The CLI ships with an MCP server by default, exposing the same init and code generation surface over stdio:

```bash
spikard mcp
```

That MCP surface includes `init_project`, `generate_openapi`, `generate_asyncapi_handlers`, `generate_jsonrpc`, `generate_graphql`, `generate_protobuf`, `generate_asyncapi_fixtures`, `generate_asyncapi_test_app`, `generate_asyncapi_bundle`, `validate_asyncapi`, and `get_features`.

For HTTP transport, build or install with the `mcp-http` feature and run:

```bash
cargo run -p spikard-cli --features mcp-http -- mcp --transport http --host 127.0.0.1 --port 3001
```

## Code Generation Support

| Format | Generated Targets |
|--------|-------------------|
| OpenAPI 3.1 | Python, TypeScript, Rust, Ruby, PHP, Elixir |
| AsyncAPI 3.0 | Python, TypeScript, Rust, Ruby, PHP, Elixir |
| OpenRPC / JSON-RPC | Python, TypeScript, Rust, Ruby, PHP, Elixir |
| GraphQL SDL / introspection | Python, TypeScript, Rust, Ruby, PHP, Elixir |
| Protobuf / gRPC | Python, TypeScript, Rust, Ruby, PHP, Elixir |

All generated output is validated before write-out, and `spikard init` starter projects now produce real, idiomatic project layouts for every supported binding.

## Performance

Benchmarked across 34 workloads at 100 concurrency ([methodology](docs/benchmarks/methodology.md)):

| Framework | Avg RPS | P50 (ms) | P99 (ms) |
|-----------|--------:|----------:|----------:|
| **spikard-rust** | 64,516 | 1.43 | 3.93 |
| spikard-bun | 49,460 | 2.18 | 4.21 |
| spikard-node | 46,160 | 2.18 | 3.35 |
| spikard-php | 16,942 | 5.82 | 9.1 |
| spikard-python | 12,623 | 5.55 | 38.39 |
| spikard-ruby | 7,151 | 14.62 | 18.98 |

Spikard is **benchmarked against mature framework baselines across the supported bindings**.

Key optimizations:
- **Thin bindings** over a shared Rust core
- **Zero-copy FFI paths** where the binding supports them
- **Template-driven code generation** validated before files are written

## Development

Use the task runner to keep the workspace aligned with CI:

```bash
task setup
task build
task test
task lint
task readme:validate
```

The authoritative README set is template-driven from `scripts/readme_config.yaml`, `scripts/readme_templates/`, and `scripts/readme_content/`.

## Related READMEs

- [CLI](crates/spikard-cli/README.md) - init, generate, testing, validate-asyncapi, and MCP
- [Rust crate](crates/spikard/README.md) - runtime API and tower-based HTTP stack
- [Python package](packages/python/README.md) - PyO3 bindings and Python-first usage
- [Node package](packages/node/README.md) - napi-rs bindings for TypeScript and Node.js
- [Ruby package](packages/ruby/README.md) - Magnus bindings and Ruby DSL
- [PHP package](packages/php/README.md) - ext-php-rs bindings and PHP framework API
- [Elixir package](packages/elixir/README.md) - Rustler bindings and Elixir router API

## Documentation

- [spikard.dev](https://spikard.dev)
- [Examples](examples/README.md)
- [Schema fixtures](examples/schemas/README.md)

## License

MIT - See [LICENSE](LICENSE) for details

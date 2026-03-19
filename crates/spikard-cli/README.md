<!-- GENERATED FILE — DO NOT EDIT DIRECTLY. Run: task readme:generate -->

# spikard-cli

CLI and MCP server for Spikard applications: project scaffolding, code generation, AsyncAPI fixture/test-app helpers, and schema validation.

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

## Features
- **Project scaffolding** with `spikard init`
- **CLI-equivalent MCP server** via `spikard mcp`
- **Code generation** from OpenAPI, GraphQL, gRPC/Protobuf, AsyncAPI, and OpenRPC
- **AsyncAPI testing helpers** for fixtures, test apps, and full bundles
- **Schema validation** for AsyncAPI specifications
- **Multi-language targets**: Python, TypeScript, Rust, Ruby, PHP, Elixir

## Installation

```bash
cargo install spikard-cli
```

## Quick Start

### init

Create a new Spikard project:

```bash
spikard init my-project --lang python --dir .
```

Supported languages: `python`, `typescript`, `rust`, `ruby`, `php`, `elixir`

### generate

Generate code from API specifications:

```bash
# OpenAPI 3.1
spikard generate openapi ./openapi.yaml --lang python --output ./generated.py

# AsyncAPI 3.0
spikard generate asyncapi ./asyncapi.yaml --lang elixir --output ./lib/generated.ex

# GraphQL SDL / introspection
spikard generate graphql ./schema.graphql --lang typescript --output ./src/generated.ts

# OpenRPC / JSON-RPC
spikard generate jsonrpc ./openrpc.json --lang ruby --output ./generated.rb

# Protobuf / gRPC
spikard generate protobuf ./service.proto --lang rust --output ./src/generated.rs

# PHP DTOs
spikard generate php-dto --output ./src/Generated
```

Supported target languages: `python`, `typescript`, `rust`, `ruby`, `php`, `elixir`

### validate-asyncapi

Validate AsyncAPI specifications:

```bash
spikard validate-asyncapi ./asyncapi.json
```

### testing asyncapi

Generate AsyncAPI fixtures, language-specific test apps, or a full bundle:

```bash
spikard testing asyncapi fixtures ./chat.asyncapi.yaml --output ./testing_data
spikard testing asyncapi test-app ./chat.asyncapi.yaml --lang elixir --output ./e2e/elixir
spikard testing asyncapi all ./chat.asyncapi.yaml --output ./generated
```

### mcp

Expose the same init and code generation surface over MCP stdio:

```bash
spikard mcp
```

For streamable HTTP transport, build with `--features mcp-http` and run:

```bash
cargo run -p spikard-cli --features mcp-http -- mcp --transport http --host 127.0.0.1 --port 3001
```

## Documentation

- [Main Project README](../../README.md)
- [Full API Documentation](https://docs.rs/spikard-cli)
- [spikard.dev](https://spikard.dev)

## License

MIT - See [LICENSE](../../LICENSE) for details

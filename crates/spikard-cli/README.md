<!-- GENERATED FILE — DO NOT EDIT DIRECTLY. Run: task readme:generate -->

# spikard-cli

CLI for Spikard applications: code generation, project scaffolding, and schema validation.

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
- **Code generation** from OpenAPI, GraphQL, gRPC/Protobuf, AsyncAPI, OpenRPC
- **Schema validation** for AsyncAPI specifications
- **Multi-language targets**: Python, TypeScript, Rust, Ruby, PHP

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

Supported languages: `python`, `typescript`, `rust`, `ruby`, `php`

### generate

Generate code from API specifications:

```bash
# OpenAPI 3.0+
spikard generate openapi ./openapi.json --lang python --output ./generated

# AsyncAPI 2.0+
spikard generate asyncapi ./asyncapi.json --lang python --output ./generated

# GraphQL
spikard generate graphql ./schema.graphql --lang python --output ./generated

# JSON-RPC 2.0
spikard generate jsonrpc ./openrpc.json --lang python --output ./generated

# PHP DTOs
spikard generate php-dto ./openapi.json --output ./src/Generated
```

Supported target languages: `python`, `typescript`, `rust`, `ruby`, `php`

### validate-asyncapi

Validate AsyncAPI specifications:

```bash
spikard validate-asyncapi ./asyncapi.json
```

## Documentation

- [Main Project README](../../README.md)
- [Full API Documentation](https://docs.rs/spikard-cli)
- [spikard.dev](https://spikard.dev)

## License

MIT - See [LICENSE](LICENSE) for details

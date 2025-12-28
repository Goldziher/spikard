# spikard-cli

CLI for Spikard applications: code generation, project scaffolding, and schema validation.

[![Crates.io](https://img.shields.io/crates/v/spikard-cli.svg)](https://crates.io/crates/spikard-cli)
[![Documentation](https://docs.rs/spikard-cli/badge.svg)](https://docs.rs/spikard-cli)

## Installation

```bash
cargo install spikard-cli
```

## Commands

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

## Examples

See `examples/` directory in the main repository for runnable demonstrations across all languages.

## Documentation

- [Main Project README](../../README.md)
- [Architecture Decision Records](../../docs/adr/)
- [API Documentation](https://docs.rs/spikard-cli)

## License

MIT

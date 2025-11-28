# spikard-codegen

Code generation and configuration management for Spikard applications.

## Status & Badges

[![Crates.io](https://img.shields.io/crates/v/spikard-codegen.svg)](https://crates.io/crates/spikard-codegen)
[![Downloads](https://img.shields.io/crates/d/spikard-codegen.svg)](https://crates.io/crates/spikard-codegen)
[![Documentation](https://docs.rs/spikard-codegen/badge.svg)](https://docs.rs/spikard-codegen)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Overview

`spikard-codegen` enables declarative API definition via YAML/JSON specifications
and generates type-safe, idiomatic code for Python, TypeScript, Rust, Ruby, and PHP.
Integration with OpenAPI 3.1 and AsyncAPI 2.0+ specifications ensures compatibility with
existing API design workflows.

## Features

- **Multi-Language Support** - Generate code for Python, TypeScript, Rust, Ruby, and PHP
- **OpenAPI 3.1 Integration** - Import/export OpenAPI 3.1 specifications
- **AsyncAPI 2.0+ Support** - For async/event-driven API definitions
- **Type-Safe Code Generation** - Full typing for all target languages
- **JSON Schema Validation** - Built-in validation against JSON schemas
- **Multi-Protocol Support** - HTTP, gRPC, queues (SQS/Kafka), CloudEvents
- **Cross-Language Consistency** - Same configuration generates idiomatic code for all targets
- **Template Customization** - Extend generation with custom Handlebars/Tera templates

## Installation

```toml
[dependencies]
spikard-codegen = "0.2.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Features

```toml
[dependencies]
spikard-codegen = { version = "0.2.0", features = ["full"] }
```

- `python` - Python code generation
- `typescript` - TypeScript code generation
- `rust` - Rust code generation
- `openapi` - OpenAPI spec handling
- `full` - All generators and features

## Usage

```rust
use spikard_codegen::{Generator, Target};
use std::path::Path;

// Load configuration
let generator = Generator::from_file(Path::new("spikard.yaml"))?;

// Validate
generator.validate()?;

// Generate Python code
generator.generate(Target::Python, Path::new("src/generated"))?;

// Generate OpenAPI spec
let openapi = generator.generate_openapi()?;
```

## Configuration Format

See `docs/adr/0004-code-generation.md` for the current generation approach and complete specification.

### Example

```yaml
version: "1.0"
name: "my-service"

http:
  routes:
    - path: "/users"
      method: POST
      handler: "handlers.create_user"
      request:
        body:
          type: object
          required: [name, email]
          properties:
            name: { type: string }
            email: { type: string, format: email }
      response:
        status: 201
        body: { $ref: "#/schemas/User" }

schemas:
  User:
    type: object
    properties:
      id: { type: string, format: uuid }
      name: { type: string }
      email: { type: string, format: email }
```

## CLI Integration

This crate is used by `spikard-cli` to provide the `generate` command:

```bash
spikard generate --config spikard.yaml --target python --out src/generated/
```

## Supported Targets

### HTTP Endpoints

- **Python** - Fast API compatible, type-hinted async handlers
- **TypeScript** - Express/Fastify compatible, full typing
- **Rust** - Native Axum handlers, zero-cost abstractions
- **Ruby** - Sinatra/Rails compatible, RBS-typed
- **PHP** - PSR-7 compliant, strict typing

### Event-Driven (Experimental)

- **Kafka** - Event producer/consumer stubs
- **SQS** - AWS Lambda integration
- **CloudEvents** - Standard event format handlers

## CLI Integration

Use `spikard-cli` to access code generation:

```bash
spikard generate --config spikard.yaml --language python --output src/generated/
spikard generate --from openapi.json --language typescript --output src/generated/
```

## Custom Templates

Extend code generation with custom Handlebars templates:

```rust
use spikard_codegen::TemplateEngine;

let engine = TemplateEngine::new();
engine.register_template("my_handler.hbs", my_template)?;

generator.with_template_engine(engine).generate()?;
```

## Examples

See `examples/` for complete code generation examples:

- `examples/http_api/` - Complete OpenAPI-based API generation
- `examples/async_service/` - Event-driven service generation
- `examples/multi_target/` - Single spec generating code for multiple languages

## Development Status

- [x] Configuration parsing (YAML/JSON)
- [x] Configuration validation
- [x] Intermediate representation
- [x] OpenAPI 3.1 integration
- [ ] Python code generation
- [ ] TypeScript code generation
- [ ] Rust code generation
- [ ] gRPC support
- [ ] Queue consumers/producers
- [ ] CloudEvents handlers

## Related Projects

- [spikard-cli](../spikard-cli/README.md) - Command-line interface
- [spikard](../spikard/README.md) - Main HTTP framework
- [spikard-http](../spikard-http/README.md) - HTTP server

## Documentation

- [Main Project README](../../README.md)
- [Code Generation ADR](../../docs/adr/0004-code-generation.md)
- [API Documentation](https://docs.rs/spikard-codegen)
- [OpenAPI Specification](https://spec.openapis.org/oas/v3.1.0)

## License

MIT

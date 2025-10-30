# spikard-codegen

Code generation and configuration management for Spikard.

## Overview

`spikard-codegen` enables declarative server definition via YAML/JSON configuration files
and generates idiomatic code for Python, TypeScript, and Rust.

## Features

- **Multi-Protocol Support**: HTTP, gRPC, queues (SQS/Kafka), CloudEvents
- **Type-Safe Code Generation**: Fully typed code for all targets
- **OpenAPI Integration**: Generate OpenAPI 3.1 specs from configuration
- **Schema Validation**: JSON Schema-based validation
- **Cross-Language Consistency**: Same config generates code for all targets

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

See [docs/design/09-unified-config-format.md](../../docs/design/09-unified-config-format.md)
for the complete specification.

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

## Development Status

- [x] Configuration parsing (YAML/JSON)
- [x] Configuration validation
- [x] Intermediate representation
- [ ] Python code generation
- [ ] TypeScript code generation
- [ ] Rust code generation
- [ ] OpenAPI 3.1 generation
- [ ] gRPC support
- [ ] Queue consumers/producers
- [ ] CloudEvents handlers

## License

MIT

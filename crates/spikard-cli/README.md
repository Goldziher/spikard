# spikard-cli

Command-line interface for building, validating, and running Spikard applications.

## Status & Badges

[![Crates.io](https://img.shields.io/crates/v/spikard-cli.svg)](https://crates.io/crates/spikard-cli)
[![Downloads](https://img.shields.io/crates/d/spikard-cli.svg)](https://crates.io/crates/spikard-cli)
[![Documentation](https://docs.rs/spikard-cli/badge.svg)](https://docs.rs/spikard-cli)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Features

- **Run applications** - Boot the Rust HTTP server with Python/Node/Ruby/PHP application handlers
- **Code generation** - Generate code from OpenAPI and AsyncAPI specifications
- **Validation** - Validate application structure and configuration
- **Schema inspection** - Inspect JSON Schema and OpenAPI definitions
- **Hot reload** - Development mode with automatic reloading (Python)
- **Multi-language support** - Run applications written in Python, Node.js, Ruby, PHP, and more

## Installation

### Binary (Recommended)

```bash
cargo install spikard-cli
```

### From Source

```bash
git clone https://github.com/Goldziher/spikard.git
cd spikard
cargo build -p spikard-cli --release
```

## Quick Start

### Running a Python Application

```bash
spikard run path/to/app.py --host 0.0.0.0 --port 8000
```

The CLI boots Axum with handlers provided by the Python application via the `spikard` package.

### Available Commands

```bash
# Run an application
spikard run <app-path> [OPTIONS]

# Validate an application
spikard validate <app-path>

# Generate code from specification
spikard generate [OPTIONS] <spec-path> [--output <OUTPUT_DIR>]

# Display help
spikard --help
```

## Options

### Run Command

- `--host <HOST>` - Server host (default: 127.0.0.1)
- `--port <PORT>` - Server port (default: 8000)
- `--workers <WORKERS>` - Number of worker threads
- `--reload` - Enable auto-reload in development (Python)
- `--config <CONFIG>` - Path to configuration file

### Generate Command

- `--language <LANGUAGE>` - Target language (python, typescript, rust, etc.)
- `--output <OUTPUT>` - Output directory
- `--schemas` - Include schema definitions

## Configuration

Create a `spikard.yml` file in your project root:

```yaml
name: my-api
version: 1.0.0
language: python
port: 8000

routes:
  - path: /api/users
    methods: [GET, POST]
    description: User management endpoints

validation:
  strict: true
  schemas:
    - path: ./schemas
```

## Development

- Compile with `cargo build -p spikard-cli` or `task build:cli`
- Run e2e tests with `cargo run --package spikard-cli -- run examples/app.py`
- Apply `cargo fmt`/`cargo clippy` before publishing (inherited from workspace)

## Subcommand Details

### run

Boots the Rust HTTP server and loads application handlers from the specified language binding:

```bash
spikard run ./app.py --port 8080 --reload
```

Environment variables:
- `SPIKARD_LOG` - Set log level (debug, info, warn, error)
- `SPIKARD_RELOAD` - Enable auto-reload

### validate

Validates the application structure, routes, and configuration:

```bash
spikard validate ./app.py
```

Checks:
- Valid route definitions
- Schema correctness
- Handler accessibility
- Configuration validity

### generate

Generates code from OpenAPI or AsyncAPI specifications:

```bash
spikard generate ./openapi.json --language python --output ./generated
```

Supports:
- OpenAPI 3.0+ specifications
- AsyncAPI 2.0+ specifications
- Type-safe code generation for all bindings
- Custom templates

## Related Crates

- [spikard](../spikard/README.md) - High-level HTTP framework
- [spikard-http](../spikard-http/README.md) - HTTP server
- [spikard-codegen](../spikard-codegen/README.md) - Code generation tools

## Documentation

- [Main Project README](../../README.md)
- [CLI Guide](../../docs/cli-guide.md)
- [API Documentation](https://docs.rs/spikard-cli)

## License

MIT

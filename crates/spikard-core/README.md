<!-- GENERATED FILE — DO NOT EDIT DIRECTLY. Run: task readme:generate -->

# spikard-core

Shared transport-agnostic primitives and types for building Spikard runtimes across multiple languages and frameworks.

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
- **Transport-agnostic** request/response models
- **JSON Schema validation** via jsonschema crate
- **Gzip and Brotli** compression/decompression
- **RFC-compliant** header and cookie parsing
- **Query string** parsing and URL handling
- **Zero-copy design** with minimal allocations
- **Strongly-typed** request and response structures

## Installation

```toml
[dependencies]
spikard-core = "0.12.0"
```

## Quick Start

### Request/Response Handling

```rust
use spikard_core::{Request, Response};
use std::collections::HashMap;

// Create a request
let mut request = Request::new(
    "GET".to_string(),
    "/api/users".to_string(),
);

// Add headers
request.headers_mut().insert(
    "Authorization".to_string(),
    "Bearer token123".to_string(),
);

// Add query parameters
let mut query = HashMap::new();
query.insert("filter".to_string(), "active".to_string());
request.set_query_params(query);

// Create a response
let mut response = Response::new(200);
response.set_body(r#"{"users": []}"#.as_bytes().to_vec());
```

### Schema Validation

```rust
use spikard_core::validation::ValidateBody;
use serde_json::json;

let schema = json!({
    "type": "object",
    "properties": {
        "name": { "type": "string" },
        "email": { "type": "string", "format": "email" }
    },
    "required": ["name", "email"]
});

let body = json!({
    "name": "Alice",
    "email": "alice@example.com"
});

// Validate body against schema
validate_body(&body, &schema)?;
```

## Core Types

- `Request` - HTTP request model with headers, cookies, body, and path parameters
- `Response` - HTTP response model with status, headers, and body
- `HandlerResult` - Standard result type for handlers
- `ValidationError` - Structured validation errors with field-level details
- `RequestContext` - Request execution context with metadata
- `RouteConfig` - Route configuration with validation schemas

## Architecture

`spikard-core` sits at the foundation of the Spikard architecture:

```
┌─────────────────────────────────────┐
│  Language Bindings                  │
│  (Python, Node, Ruby, PHP, Elixir)  │
└──────────────┬──────────────────────┘
               │ implements
┌──────────────▼──────────────────────┐
│  spikard-http (Axum Runtime)        │
└──────────────┬──────────────────────┘
               │ uses
┌──────────────▼──────────────────────┐
│  spikard-core (Primitives)          │
└─────────────────────────────────────┘
```

All language bindings depend on `spikard-core` to ensure consistent request/response handling across platforms.

## Related Crates

- [spikard](../spikard/README.md) - High-level HTTP framework
- [spikard-http](../spikard-http/README.md) - HTTP server implementation

## Documentation

- [Main Project README](../../README.md)
- [Full API Documentation](https://docs.rs/spikard-core)
- [spikard.dev](https://spikard.dev)

## License

MIT - See [LICENSE](../../LICENSE) for details

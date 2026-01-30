# spikard-core

Shared transport-agnostic primitives and types for building Spikard runtimes across multiple languages and frameworks.

## Status & Badges

[![Crates.io](https://img.shields.io/crates/v/spikard-core.svg)](https://crates.io/crates/spikard-core)
[![Downloads](https://img.shields.io/crates/d/spikard-core.svg)](https://crates.io/crates/spikard-core)
[![Documentation](https://docs.rs/spikard-core/badge.svg)](https://docs.rs/spikard-core)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Overview

`spikard-core` provides the foundational types and traits that enable Spikard to work across multiple language bindings:

- **Request/Response models** - HTTP-agnostic request and response types
- **Validation primitives** - JSON Schema validation and header/cookie checking
- **Middleware interfaces** - Traits for composable middleware stacks
- **Serialization support** - Efficient serialization via serde
- **Error handling** - Structured error types for cross-language error translation

## Features

- **Transport-agnostic** - Works with Axum, Hyper, or any HTTP framework
- **Schema validation** - Built-in JSON Schema support via jsonschema crate
- **Compression** - Gzip and Brotli compression/decompression
- **Header/Cookie handling** - RFC-compliant header and cookie parsing
- **URL encoding** - Query string parsing and URL handling
- **Zero-copy design** - Efficient memory usage with minimal allocations
- **Type safety** - Strongly-typed request and response structures

## Installation

```toml
[dependencies]
spikard-core = "0.10.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Optional Features

```toml
[dependencies]
spikard-core = { version = "0.10.0", features = ["di"] }
```

- `di` - Enables dependency injection support with Tokio async runtime

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

### Compression Support

```rust
use spikard_core::compression;

let original = b"This is a long string that will be compressed";

// Gzip compression
let compressed = compression::gzip_encode(original)?;
let decompressed = compression::gzip_decode(&compressed)?;

// Brotli compression
let compressed = compression::brotli_encode(original)?;
let decompressed = compression::brotli_decode(&compressed)?;
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
│  (Python, Node, Ruby, PHP, WASM)    │
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

## Documentation

- [Main Project README](../../README.md)
- [Full API Documentation](https://docs.rs/spikard-core)
- [Architecture Decision Records](../../docs/adr/)

## Related Crates

- [spikard](../spikard/README.md) - High-level HTTP framework
- [spikard-http](../spikard-http/README.md) - HTTP server implementation
- [spikard-py](../spikard-py/README.md) - Python bindings
- [spikard-node](../spikard-node/README.md) - Node.js bindings

## License

MIT

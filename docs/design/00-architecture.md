# Spikard Architecture

## Overview

Spikard is a modular HTTP toolkit with a **Rust core** and high-level bindings for **Python** and **TypeScript/Node.js**. The design philosophy is inspired by Litestar, emphasizing modularity, type safety, and performance.

## Core Principles

1. **Performance-critical operations in Rust** - HTTP parsing, routing, validation, middleware pipeline
2. **Developer experience in high-level languages** - Natural decorators, type hints, dependency injection
3. **Leverage existing ecosystems** - Don't reinvent Pydantic, Zod, etc.
4. **Zero-copy where possible** - Minimize data crossing language boundaries
5. **Sync + Async unified** - Both paradigms supported transparently

## Layered Architecture

```
┌─────────────────────────────────────────────────┐
│  Application Layer                              │
│  - App config, lifecycle hooks, plugins         │
├─────────────────────────────────────────────────┤
│  Controller Layer                               │
│  - Route groups, handlers, decorators           │
├─────────────────────────────────────────────────┤
│  Middleware Layer                               │
│  - CORS, Auth, Logging, Compression, etc.       │
├─────────────────────────────────────────────────┤
│  Guard Layer                                    │
│  - Authorization, validation gates              │
├─────────────────────────────────────────────────┤
│  Dependency Injection Layer                     │
│  - Providers, scopes (request/app/singleton)    │
├─────────────────────────────────────────────────┤
│  Rust Core Layer                                │
│  - Router, HTTP parser, Request/Response        │
│  - Validation, Parameter extraction             │
└─────────────────────────────────────────────────┘
```

## Request Processing Flow

```
HTTP Request
    ↓
[Rust] Parse HTTP (headers, path, query, body)
    ↓
[Rust] Route Matching (fast radix tree)
    ↓
[Rust] Extract & Validate Path Parameters
    ↓
[Rust] Extract & Validate Query Parameters
    ↓
[Rust] Extract Header Parameters
    ↓
[Rust] Validate Request Body (JSON Schema)
    ↓
[Rust] Execute Middleware Pipeline
    ↓
[Python/TS] Call Handler with Validated Data
    ↓
[Rust] Validate Response (optional)
    ↓
[Rust] Serialize Response
    ↓
HTTP Response
```

## Component Responsibilities

### Rust Core
- **HTTP server** - Based on Tokio/Axum ecosystem
- **Router** - Radix tree-based path matching
- **Request/Response** - Zero-copy where possible
- **Validation** - JSON Schema validation via `jsonschema-rs`
- **Parameter extraction** - Path, query, header parsing and coercion
- **Middleware pipeline** - Ordered execution of middleware
- **Static file serving** - With caching and compression
- **WebSocket support** - Bidirectional communication

### Python Bindings (PyO3)
- **Decorator API** - `@get`, `@post`, etc.
- **Controllers** - Grouping related routes
- **Dependency Injection** - Provider pattern with scopes
- **Guards** - Authorization logic
- **Schema extraction** - From Pydantic, msgspec, dataclasses
- **Handler execution** - Call Python async/sync functions
- **Plugin system** - Lifecycle hooks and extensions

### TypeScript Bindings (NAPI-RS)
- **Decorator API** - `@Get()`, `@Post()`, etc.
- **Controllers** - Class-based route grouping
- **Dependency Injection** - Constructor injection
- **Guards** - Authorization via interfaces
- **Schema extraction** - From Zod, class-validator, etc.
- **Handler execution** - Call TypeScript async/sync methods
- **Plugin system** - Lifecycle hooks and extensions

## Key Design Decisions

### 1. JSON Schema as Interchange Format
- **Don't reinvent validation** - Use existing ecosystems (Pydantic, Zod)
- **Well-defined standard** - JSON Schema Draft 7/2019/2020
- **OpenAPI compatibility** - Free OpenAPI generation
- **Flexibility** - Support any library that outputs JSON Schema

### 2. Simple Parameter System
- **No complex type system** - Path/query/header params are simple scalars
- **Type coercion in Rust** - `str`, `int`, `float`, `bool`, `UUID`
- **Clear defaults** - Explicit default values in function signatures
- **Array support** - For repeated query parameters

### 3. Modular Plugin Architecture
- **Composable middleware** - Functions or classes
- **Dependency providers** - Scoped dependency injection
- **Lifecycle hooks** - `on_startup`, `on_shutdown`, etc.
- **Easy extension** - No core modification needed

### 4. Performance Optimizations
- **Route compilation** - Routes registered once at startup
- **Schema compilation** - JSON Schema validators pre-compiled
- **Zero-copy validation** - Body bytes stay in Rust
- **Streaming support** - For large request/response bodies
- **Static file caching** - In-memory cache with ETag support

## Target Use Cases

1. **High-performance APIs** - REST, GraphQL, RPC
2. **Microservices** - Internal service communication
3. **Web applications** - Server-side rendering + API
4. **WebSocket services** - Real-time communication
5. **File servers** - Static assets with caching
6. **Proxy/Gateway** - Request routing and transformation

## Non-Goals

- **ORM/Database integration** - Use existing tools (SQLAlchemy, Prisma)
- **Template engines** - Use Jinja2, Handlebars, etc.
- **Frontend framework** - Focus on backend/API
- **Message queues** - Use RabbitMQ, Kafka, etc.
- **Task scheduling** - Use Celery, Bull, etc.

Spikard focuses on **HTTP handling** and **request/response processing**, integrating well with existing tools for other concerns.

# Benchmarked Frameworks

This document catalogs all frameworks included in the Spikard benchmark suite, organized by language ecosystem.

## Framework Categories

### Spikard Variants

Core Spikard implementations across different language bindings.

| Framework | Language | Runtime | Validation | Description |
|-----------|----------|---------|------------|-------------|
| `spikard-rust` | Rust | Native | Tower middleware | Pure Rust implementation, baseline performance |
| `spikard-python` | Python | CPython 3.12+ | msgspec | PyO3 bindings with msgspec validation |
| `spikard-node` | TypeScript | Node 20+ | Zod | napi-rs bindings with optional Zod validation |
| `spikard-ruby` | Ruby | Ruby 3.2-4.x | RBS | magnus bindings with RBS type definitions |
| `spikard-php` | PHP | PHP 8.2+ | Native types | ext-php-rs bindings with typed properties |

### Python Ecosystem

Python frameworks with ASGI/WSGI servers.

| Framework | Version | Server | Validation | Description |
|-----------|---------|--------|------------|-------------|
| `fastapi` | 0.115+ | Uvicorn | Pydantic v2 | Popular async framework with OpenAPI generation |
| `fastapi-granian` | 0.115+ | Granian | Pydantic v2 | FastAPI with Rust-based Granian ASGI server |
| `litestar` | 2.x | Uvicorn | msgspec | Performance-focused FastAPI alternative |
| `litestar-granian` | 2.x | Granian | msgspec | Litestar with Granian server |
| `robyn` | 0.x | Native Rust | None | Rust-based Python framework (similar architecture to Spikard) |

#### Raw Variants (No Validation)

| Framework | Description |
|-----------|-------------|
| `fastapi-raw` | FastAPI with Pydantic validation disabled |
| `fastapi-granian-raw` | FastAPI + Granian without validation |
| `litestar-raw` | Litestar with msgspec validation disabled |
| `litestar-granian-raw` | Litestar + Granian without validation |
| `spikard-raw` | Spikard Python with msgspec validation disabled |

**Purpose**: Measure validation overhead by comparing validated vs raw variants.

### Node.js Ecosystem

JavaScript/TypeScript frameworks on Node.js, Bun, and Deno runtimes.

| Framework | Runtime | Validation | Description |
|-----------|---------|------------|-------------|
| `express` | Node 20+ | Zod | Traditional Node.js framework, most popular |
| `express-raw` | Node 20+ | None | Express without validation |
| `fastify` | Node 20+ | Ajv (JSON Schema) | High-performance Node framework |
| `fastify-raw` | Node 20+ | None | Fastify without validation |
| `hono` | Node/Bun/Deno | Zod | Ultra-lightweight framework, multi-runtime |
| `hono-raw` | Node/Bun/Deno | None | Hono without validation |
| `elysia` | Bun 1.x | TypeBox | Bun-optimized framework with first-class TypeScript |
| `morojs` | Node 20+ | None | Minimalist routing framework |

### Ruby Ecosystem

Ruby frameworks with Rack-compatible servers.

| Framework | Server | Validation | Description |
|-----------|--------|------------|-------------|
| `roda` | Puma | Dry-validation | Lightweight routing tree framework |
| `roda-raw` | Puma | None | Roda without validation |
| `hanami-api` | Puma | Dry-validation | Modern Ruby framework from Hanami ecosystem |
| `hanami-api-raw` | Puma | None | Hanami API without validation |

### PHP Ecosystem

PHP frameworks using traditional and modern architectures.

| Framework | Version | Validation | Description |
|-----------|---------|------------|-------------|
| `phalcon` | 5.x | Built-in | C-extension framework, compiled for performance |
| `trongate` | Latest | Manual | Lightweight MVC framework |

### Baseline Comparisons

Reference implementations for performance comparison.

| Framework | Language | Description |
|-----------|----------|-------------|
| `axum-baseline` | Rust | Pure Rust Axum framework, theoretical maximum performance |

## Framework Details

### Python

#### FastAPI

**Version**: 0.115+
**Validation**: Pydantic v2
**Server**: Uvicorn (default) or Granian

The most popular async Python framework. Generates OpenAPI schemas automatically. Uses Pydantic for request/response validation with runtime type checking.

**Key features**:
- Automatic OpenAPI documentation
- Type hints for editor support
- Dependency injection system
- OAuth2 and JWT support

**Performance characteristics**:
- ~40% validation overhead (Pydantic)
- Good async performance with Uvicorn
- 15-25% improvement with Granian server
- Slower than Litestar due to heavier validation

#### Litestar

**Version**: 2.x
**Validation**: msgspec
**Server**: Uvicorn (default) or Granian

FastAPI-inspired framework optimized for performance. Uses msgspec instead of Pydantic for 5x faster validation.

**Key features**:
- OpenAPI generation
- Dependency injection
- Plugin system
- msgspec for fast serialization

**Performance characteristics**:
- ~15% validation overhead (msgspec)
- Faster than FastAPI in all workloads
- Excellent JSON performance
- 10-20% improvement with Granian

#### Robyn

**Version**: 0.x
**Validation**: None
**Server**: Native Rust (actix-web based)

Experimental framework with Rust core and Python bindings. Similar architecture to Spikard but less mature.

**Key features**:
- Rust-based HTTP server
- Minimal Python overhead
- Simple decorator-based routing

**Performance characteristics**:
- Very high RPS (~2-3x FastAPI)
- No built-in validation
- Limited middleware ecosystem
- Immature error handling

### Node.js

#### Express

**Version**: 4.x
**Validation**: Zod (when added)
**Server**: Node HTTP module

The most widely-used Node.js framework. Mature ecosystem with thousands of middleware packages.

**Key features**:
- Massive ecosystem
- Flexible routing
- Template engine support
- Simple API

**Performance characteristics**:
- Moderate performance
- ~25% validation overhead with Zod
- Single-threaded (typical Node limitation)
- Well-optimized for I/O-bound tasks

#### Fastify

**Version**: 4.x
**Validation**: Ajv (JSON Schema)
**Server**: Node HTTP module

Performance-focused Express alternative. Uses JSON Schema for validation with pre-compiled validators.

**Key features**:
- Schema-based validation (Ajv)
- Plugin architecture
- Request/response lifecycle hooks
- Logging built-in (pino)

**Performance characteristics**:
- 2-3x faster than Express
- ~10% validation overhead (compiled schemas)
- Optimized JSON serialization (fast-json-stringify)
- Lower memory usage

#### Hono

**Version**: 4.x
**Validation**: Zod
**Server**: Node/Bun/Deno/Cloudflare Workers

Ultra-lightweight framework designed for edge runtimes. Runs on Node, Bun, Deno, and Cloudflare Workers.

**Key features**:
- Multi-runtime support
- Tiny bundle size (<13KB)
- Edge-optimized
- Middleware system

**Performance characteristics**:
- Comparable to Fastify on Node
- 20-30% faster on Bun runtime
- Low memory footprint
- Excellent cold start times

#### Elysia

**Version**: 1.x
**Validation**: TypeBox
**Server**: Bun native

Framework designed specifically for Bun. Takes advantage of Bun's JavaScriptCore engine and fast I/O.

**Key features**:
- Bun-optimized
- End-to-end type safety
- Eden Treaty (type-safe client)
- Plugin system

**Performance characteristics**:
- 3-5x faster than Express
- Fastest Node.js ecosystem framework
- Requires Bun runtime
- Very low latency

### Ruby

#### Roda

**Version**: 3.x
**Validation**: Dry-validation
**Server**: Puma

Lightweight routing tree framework. Emphasizes simplicity and plugin-based architecture.

**Key features**:
- Routing tree (efficient matching)
- Plugin system
- Minimal dependencies
- Flexible architecture

**Performance characteristics**:
- Fast for Ruby ecosystem
- ~15% validation overhead
- Scales well with request complexity
- Lower memory than Rails

#### Hanami API

**Version**: 2.x
**Validation**: Dry-validation
**Server**: Puma

Modern Ruby framework focused on HTTP APIs. Part of the larger Hanami ecosystem.

**Key features**:
- Dry-rb integration
- Action-based architecture
- Immutable objects
- Clean separation of concerns

**Performance characteristics**:
- Similar to Roda
- Slightly higher memory usage
- Good JSON performance
- Well-suited for APIs

### PHP

#### Phalcon

**Version**: 5.x
**Validation**: Built-in validators
**Server**: PHP-FPM or Swoole

High-performance PHP framework implemented as C extension. Compiled for maximum speed.

**Key features**:
- C extension (compiled code)
- Full-stack framework
- ORM built-in
- Volt template engine

**Performance characteristics**:
- Fastest PHP framework
- 5-10x faster than Laravel
- Low memory overhead
- Requires C extension installation

#### Trongate

**Version**: Latest
**Validation**: Manual
**Server**: Apache/Nginx

Lightweight MVC framework with minimal dependencies.

**Key features**:
- No composer dependencies
- Rapid development focus
- Built-in code generator
- Simple architecture

**Performance characteristics**:
- Moderate performance
- Low resource usage
- Traditional PHP architecture

### Rust

#### Axum

**Version**: 0.7+
**Validation**: Tower middleware
**Server**: Tokio runtime

Modern Rust web framework built on Tower middleware and Hyper.

**Key features**:
- Type-safe extractors
- Tower middleware ecosystem
- Excellent compile-time guarantees
- First-class async support

**Performance characteristics**:
- 100k+ RPS baseline
- Minimal runtime overhead
- Zero-cost abstractions
- Sub-millisecond latencies

## Validation Overhead Summary

Framework pairs showing validation cost:

| Framework | Raw RPS | Validated RPS | Overhead |
|-----------|---------|---------------|----------|
| FastAPI (Pydantic) | 35,000 | 21,000 | ~40% |
| Litestar (msgspec) | 45,000 | 38,000 | ~15% |
| Spikard (msgspec) | 50,000 | 42,000 | ~16% |
| Express (Zod) | 25,000 | 18,000 | ~28% |
| Fastify (Ajv) | 60,000 | 54,000 | ~10% |
| Roda (Dry-validation) | 15,000 | 12,500 | ~17% |

**Key insight**: msgspec (Litestar, Spikard) and compiled schemas (Fastify Ajv) have the lowest validation overhead.

## Framework Selection Guide

### Choose FastAPI if you need:
- Automatic OpenAPI documentation
- Large ecosystem and community
- Mature production deployments
- Type safety with Pydantic

### Choose Litestar if you need:
- FastAPI-like DX with better performance
- msgspec's speed advantage
- Modern async patterns
- Plugin extensibility

### Choose Spikard if you need:
- Maximum Python performance
- Rust-level safety with Python DX
- Multi-language consistency (also using Spikard in Node/Ruby)
- Minimal validation overhead

### Choose Express if you need:
- Maximum compatibility and ecosystem
- Stable, well-understood patterns
- Extensive middleware options
- Traditional Node.js development

### Choose Fastify if you need:
- High-performance Node.js
- Schema-based validation
- Built-in logging and lifecycle hooks
- Production-grade plugin system

### Choose Hono if you need:
- Edge deployment (Cloudflare Workers)
- Multi-runtime support
- Minimal bundle size
- Modern patterns with portability

### Choose Elysia if you need:
- Maximum Node.js ecosystem performance
- Bun runtime advantages
- End-to-end type safety
- Cutting-edge features

## Adding New Frameworks

To add a framework to the benchmark suite:

1. Create app directory: `tools/benchmark-harness/apps/{framework-name}/`
2. Implement all workload endpoints from `tools/benchmark-harness/src/schema/workload.rs`
3. Match request/response schemas exactly
4. Use framework's recommended production configuration
5. Document framework version and dependencies
6. Create `-raw` variant if validation can be disabled
7. Add entry to `apps/README.md`
8. Test with: `benchmark-harness run --framework {framework-name}`

See existing apps for implementation examples.

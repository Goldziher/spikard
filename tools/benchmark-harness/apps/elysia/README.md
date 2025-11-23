# Elysia Benchmark Server

High-performance benchmark server built with [Elysia](https://elysiajs.com/) on Bun runtime.

## Overview

Elysia is a TypeScript framework built specifically for Bun, offering:
- **Built-in validation**: TypeBox-based schema validation with excellent type inference
- **High performance**: Leverages Bun's fast JavaScript runtime
- **Type safety**: First-class TypeScript support with automatic type inference
- **Minimal overhead**: Zero-cost abstractions for routing and handlers

## Prerequisites

This benchmark app requires **Bun runtime** (not Node.js):

```bash
# Install Bun (macOS/Linux)
curl -fsSL https://bun.sh/install | bash

# Or via npm (requires Node.js first)
npm install -g bun

# Verify installation
bun --version
```

## Installation

```bash
# Install dependencies
bun install
```

## Running

```bash
# Default port (8000)
bun run server.ts

# Custom port
bun run server.ts 3000
```

## Endpoints

### JSON Body Workloads
- `POST /json/small` - Small payload (~100 bytes)
- `POST /json/medium` - Medium payload (~1KB)
- `POST /json/large` - Large payload (~10KB)
- `POST /json/very-large` - Very large payload (~100KB)

### Multipart Form Workloads
- `POST /multipart/small` - Small file upload (~1KB)
- `POST /multipart/medium` - Medium file upload (~10KB)
- `POST /multipart/large` - Large file upload (~100KB)

### URL Encoded Form Workloads
- `POST /urlencoded/simple` - Simple form (few fields)
- `POST /urlencoded/complex` - Complex form (many fields)

### Path Parameter Workloads
- `GET /path/simple/:id` - Single path parameter
- `GET /path/multiple/:user_id/:post_id` - Multiple path parameters
- `GET /path/deep/:org/:team/:project/:resource/:id` - Deep path nesting
- `GET /path/int/:id` - Integer path parameter parsing
- `GET /path/uuid/:uuid` - UUID path parameter
- `GET /path/date/:date` - Date path parameter

### Query Parameter Workloads
- `GET /query/few` - Few query parameters (1-2)
- `GET /query/medium` - Medium query parameters (3-5)
- `GET /query/many` - Many query parameters (6-10)

### Health Check
- `GET /health` - Health check endpoint
- `GET /` - Root health check

## Implementation Notes

### Validation
Elysia uses TypeBox for schema validation, which is:
- **Fast**: Compiled validation at runtime
- **Type-safe**: Full TypeScript inference from schemas
- **Built-in**: No external validation library needed

### Performance Characteristics
- **Runtime**: Bun (not V8) - optimized for TypeScript
- **Routing**: Radix tree-based routing for O(log n) lookups
- **Serialization**: Bun's optimized JSON serialization
- **No middleware overhead**: Direct handler invocation

### Comparison to Other Frameworks
- **vs Fastify**: Similar routing performance, but Bun runtime advantages
- **vs Express**: Significantly faster due to Bun and modern architecture
- **vs Hono**: Similar design philosophy, both support multiple runtimes

## Benchmarking

This server is designed to be benchmarked using the standard harness:

```bash
# From benchmark harness directory
python benchmark.py --framework elysia --workload json/small --duration 30
```

## Resources

- [Elysia Documentation](https://elysiajs.com/introduction.html)
- [Bun Runtime](https://bun.sh)
- [TypeBox Validation](https://github.com/sinclairzx81/typebox)

# Fastify RAW Benchmark Server

This is a **raw performance** Fastify server implementation with **NO validation**.

## Purpose

Measures Fastify's baseline HTTP performance without any validation overhead. This provides a performance ceiling to compare against the validated Fastify implementation.

## Key Differences from Regular Fastify

- **No validation schemas** - No JSON schema or Zod validation
- **Direct body echo** - Request bodies are returned as-is without parsing/validation
- **Minimal processing** - Only essential HTTP handling and routing
- **No logging** - `logger: false` for maximum performance

## Implementation Details

- **Framework**: Fastify 5.6.2
- **Language**: JavaScript (Node.js)
- **Validation**: None (accepts any input)
- **Port**: Command line arg or default 8000
- **Plugins**: @fastify/formbody (for URL-encoded forms only)

## Endpoints

All 18 workload endpoints are implemented:

### JSON Bodies (4 endpoints)
- `POST /json/small` - No validation
- `POST /json/medium` - No validation
- `POST /json/large` - No validation
- `POST /json/very-large` - No validation

### Multipart Forms (3 endpoints)
- `POST /multipart/small` - Returns mock response
- `POST /multipart/medium` - Returns mock response
- `POST /multipart/large` - Returns mock response

### URL Encoded (2 endpoints)
- `POST /urlencoded/simple` - No validation
- `POST /urlencoded/complex` - No validation

### Path Parameters (6 endpoints)
- `GET /path/simple/:id` - No validation
- `GET /path/multiple/:user_id/:post_id` - No validation
- `GET /path/deep/:org/:team/:project/:resource/:id` - No validation
- `GET /path/int/:id` - Basic parseInt only
- `GET /path/uuid/:uuid` - No validation
- `GET /path/date/:date` - No validation

### Query Parameters (3 endpoints)
- `GET /query/few` - No validation
- `GET /query/medium` - No validation
- `GET /query/many` - No validation

### Health Check
- `GET /health` - Returns `{ status: 'ok' }`
- `GET /` - Returns `{ status: 'ok' }`

## Running

```bash
# Install dependencies
pnpm install

# Start server on default port 8000
pnpm start

# Start server on custom port
pnpm start 3000
```

## Performance Comparison

This server helps answer: "How much does validation cost?"

By comparing fastify-raw vs regular fastify, we can quantify the overhead of:
- JSON schema validation
- Type coercion and parsing
- Error handling for invalid input

## Benchmark Usage

```bash
# From benchmark-harness directory
cargo run -- profile --target fastify-raw --workload json-bodies --duration 30
```

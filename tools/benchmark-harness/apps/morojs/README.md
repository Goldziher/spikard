# MoroJS Benchmark Server

MoroJS benchmark implementation for the Spikard performance comparison suite.

## About MoroJS

MoroJS is a TypeScript-first web framework built on top of uWebSockets.js, designed for maximum performance while maintaining developer experience. It combines the speed of native C++ bindings with modern TypeScript patterns.

**Key Features:**
- Built on uWebSockets.js for native performance
- TypeScript-first design with full type safety
- Zod integration for runtime validation
- Minimal overhead and zero-copy where possible
- Modern async/await patterns

## Implementation Details

This benchmark server implements all 18 standard workload endpoints:

### JSON Body Endpoints (4)
- POST `/json/small` - Small JSON payload (~100 bytes)
- POST `/json/medium` - Medium JSON payload (~1KB)
- POST `/json/large` - Large JSON payload (~10KB)
- POST `/json/very-large` - Very large JSON payload (~100KB)

### Multipart Form Endpoints (3)
- POST `/multipart/small` - Mock multipart upload (~1KB)
- POST `/multipart/medium` - Mock multipart upload (~10KB)
- POST `/multipart/large` - Mock multipart upload (~100KB)

### URL Encoded Form Endpoints (2)
- POST `/urlencoded/simple` - Simple form data
- POST `/urlencoded/complex` - Complex form data

### Path Parameter Endpoints (6)
- GET `/path/simple/:id` - Single path parameter
- GET `/path/multiple/:user_id/:post_id` - Multiple parameters
- GET `/path/deep/:org/:team/:project/:resource/:id` - Deep nested paths
- GET `/path/int/:id` - Integer path parameter
- GET `/path/uuid/:uuid` - UUID path parameter
- GET `/path/date/:date` - Date path parameter

### Query Parameter Endpoints (3)
- GET `/query/few` - Few query parameters (1-2)
- GET `/query/medium` - Medium query parameters (3-5)
- GET `/query/many` - Many query parameters (6-10)

### Health Check Endpoints (2)
- GET `/health` - Health check endpoint
- GET `/` - Root health check

## Validation Strategy

Uses Zod schemas matching the Fastify reference implementation:
- Simple type validation (no email format constraints)
- Nested object validation for addresses and items
- Array validation for collections
- Integer and number type enforcement

## Usage

### Install Dependencies
```bash
npm install
# or
pnpm install
```

### Start Server
```bash
npm start [port]
# or
pnpm start [port]
```

Default port: 8000

### Examples
```bash
# Start on default port 8000
npm start

# Start on custom port
npm start 3000
```

## Performance Characteristics

MoroJS leverages uWebSockets.js for performance:
- Zero-copy HTTP parsing via native bindings
- Minimal JavaScript overhead
- Direct memory access where possible
- Efficient routing through radix tree
- Built-in HTTP/1.1 pipelining support

## Dependencies

- `@morojs/moro`: Core framework (~0.3.0)
- `zod`: Runtime validation (~3.23.8)
- `tsx`: TypeScript execution for development
- `typescript`: TypeScript compiler

## Development

```bash
# Development mode with auto-reload
npm run dev

# Type checking
npx tsc --noEmit
```

## Architecture

```
server.ts
├── Schema Definitions (Zod)
│   ├── SmallPayloadSchema
│   ├── MediumPayloadSchema
│   ├── LargePayloadSchema
│   └── VeryLargePayloadSchema
├── Route Handlers
│   ├── JSON body endpoints (with validation)
│   ├── Multipart endpoints (mock responses)
│   ├── URL encoded endpoints
│   ├── Path parameter endpoints
│   ├── Query parameter endpoints
│   └── Health check endpoints
└── Server startup with configurable port
```

## Notes

- All validation schemas match Fastify reference exactly
- No extra validation constraints (e.g., no email format, no positive/nonnegative)
- Multipart endpoints return mock responses for consistent comparison
- Query parameter endpoints echo back all query params
- Path parameter endpoints return extracted params as JSON

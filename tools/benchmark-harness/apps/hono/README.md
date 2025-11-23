# Hono Benchmark Server

Hono benchmark server implementation with Zod validation for workload comparison.

## Overview

This server implements all 18 benchmark workload endpoints using:
- **Hono** (v4.10+) - Ultra-fast web framework designed for edge/serverless
- **Zod** (v3.23+) - TypeScript-first schema validation
- **@hono/zod-validator** (v0.7+) - Official Hono middleware for Zod validation
- **@hono/node-server** (v1.19+) - Node.js adapter for Hono

## Features

- Complete TypeScript implementation with strict typing
- Zod schema validation matching Python Pydantic models
- All 18 workload endpoints:
  - 4 JSON body endpoints (small, medium, large, very-large)
  - 3 multipart form endpoints
  - 2 URL-encoded form endpoints
  - 6 path parameter endpoints
  - 3 query parameter endpoints

## Installation

```bash
npm install
```

## Running

### TypeScript (recommended)
```bash
npm start [port]
# or
tsx server.ts [port]
```

### JavaScript
```bash
npm run start:js [port]
# or
node server.js [port]
```

Default port: 8000

## Validation Schemas

All POST endpoints use Zod schemas for validation, matching the Python Pydantic models:

### SmallPayload (~100 bytes)
```typescript
{
  name: string
  description: string
  price: number
  tax?: number | null
}
```

### MediumPayload (~1KB)
```typescript
{
  name: string
  email: string (email format)
  age: number (positive integer)
  address: {
    street: string
    city: string
    state: string
    zip_code: string
  }
  tags: string[]
}
```

### LargePayload (~10KB)
```typescript
{
  user_id: string
  name: string
  email: string (email format)
  items: Array<{
    id: string
    name: string
    price: number
    quantity: number (non-negative integer)
  }>
  metadata: Record<string, any>
}
```

### VeryLargePayload (~100KB)
```typescript
{
  batch_id: string
  records: Array<Record<string, any>>
  summary: Record<string, any>
}
```

## API Endpoints

### Health Check
- `GET /` - Root endpoint
- `GET /health` - Health check

### JSON Body Workloads
- `POST /json/small` - Small JSON (~100 bytes)
- `POST /json/medium` - Medium JSON (~1KB)
- `POST /json/large` - Large JSON (~10KB)
- `POST /json/very-large` - Very large JSON (~100KB)

### Multipart Form Workloads
- `POST /multipart/small` - Small multipart (~1KB)
- `POST /multipart/medium` - Medium multipart (~10KB)
- `POST /multipart/large` - Large multipart (~100KB)

### URL-Encoded Form Workloads
- `POST /urlencoded/simple` - Simple URL-encoded form
- `POST /urlencoded/complex` - Complex URL-encoded form

### Path Parameter Workloads
- `GET /path/simple/:id` - Single path parameter
- `GET /path/multiple/:user_id/:post_id` - Multiple path parameters
- `GET /path/deep/:org/:team/:project/:resource/:id` - Deep nested parameters
- `GET /path/int/:id` - Integer path parameter
- `GET /path/uuid/:uuid` - UUID path parameter
- `GET /path/date/:date` - Date path parameter

### Query Parameter Workloads
- `GET /query/few` - Few query parameters (1-2)
- `GET /query/medium` - Medium query parameters (3-5)
- `GET /query/many` - Many query parameters (6-10)

## Testing

```bash
# Test health endpoint
curl http://localhost:8000/health

# Test JSON validation
curl -X POST http://localhost:8000/json/small \
  -H "Content-Type: application/json" \
  -d '{"name":"Test","description":"Item","price":9.99,"tax":0.99}'

# Test path parameters
curl http://localhost:8000/path/simple/123

# Test query parameters
curl "http://localhost:8000/query/few?search=test&limit=10"
```

## Architecture

- **Framework**: Hono - Edge-first framework with Node.js adapter
- **Validation**: Zod via @hono/zod-validator middleware
- **Runtime**: Node.js 18+ via @hono/node-server
- **Language**: TypeScript with strict mode enabled

## Performance Notes

- Zod validation happens via middleware before handler execution
- Invalid requests return structured error responses with Zod error details
- Uses `zValidator('json', schema)` pattern for type-safe validation
- Validated data accessed via `c.req.valid('json')`

## Dependencies

```json
{
  "dependencies": {
    "hono": "^4.10.0",
    "@hono/node-server": "^1.19.6",
    "@hono/zod-validator": "^0.7.4",
    "zod": "^3.23.8"
  },
  "devDependencies": {
    "tsx": "^4.19.2",
    "typescript": "^5.7.2",
    "@types/node": "^22.10.1"
  }
}
```

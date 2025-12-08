# Spikard Node.js Simple Example

A minimal Spikard Node.js example demonstrating basic HTTP routing with GET and POST handlers.

## Features

- **Basic routing**: GET and POST routes
- **Path parameters**: Extract variables from URL paths
- **JSON responses**: Automatic JSON serialization
- **Request body parsing**: Handle POST request bodies
- **Health checks**: Standard health endpoint pattern

## Prerequisites

Node.js 18+ with TypeScript support

## Installation

```bash
cd examples/node-simple
pnpm install
```

## Running the Server

```bash
pnpm dev
```

Or using tsx directly:

```bash
npx tsx src/server.ts
```

The server will start on `http://0.0.0.0:8000`

## Testing the Routes

### Root endpoint
```bash
curl http://localhost:8000/
```

**Response:**
```json
{
  "message": "Hello from Spikard Node!",
  "timestamp": "2025-12-08T22:00:00.000Z"
}
```

### Health check
```bash
curl http://localhost:8000/health
```

**Response:**
```json
{
  "status": "healthy",
  "uptime": 123.456
}
```

### Get user by ID
```bash
curl http://localhost:8000/users/42
```

**Response:**
```json
{
  "user_id": "42",
  "name": "Test User"
}
```

### Echo endpoint
```bash
curl -X POST http://localhost:8000/echo \
  -H "Content-Type: application/json" \
  -d '{"message": "hello"}'
```

**Response:**
```json
{
  "echoed": true,
  "body": {
    "message": "hello"
  },
  "receivedAt": "2025-12-08T22:00:00.000Z"
}
```

## Code Structure

```
node-simple/
├── src/
│   └── server.ts         # Main application with route handlers
├── package.json          # Dependencies and scripts
├── tsconfig.json         # TypeScript configuration
└── README.md             # This file
```

## Next Steps

For more advanced examples, see:
- [`examples/node/`](../node/) - Progressive tutorial examples (validation, streaming, WebSockets, lifecycle hooks)
- [`packages/node/README.md`](../../packages/node/README.md) - Full Node.js binding documentation
- [`docs/adr/`](../../docs/adr/) - Architecture Decision Records

## Key Concepts

### Route Definition

Routes are defined using helper functions that wrap handler functions:

```typescript
get("/path")(async function handlerName(req) {
  return { data: "response" };
});
```

### Path Parameters

Extract variables from URL paths using `:param` syntax:

```typescript
get("/users/:id")(async function getUserById(req) {
  const userId = req.path_params?.id;
  return { user_id: userId };
});
```

### Request Body

Access parsed JSON bodies in POST handlers:

```typescript
post("/echo")(async function handleEcho(req) {
  return { echoed: req.body };
});
```

### Response Types

Handlers can return:
- Plain objects (automatically serialized to JSON)
- Strings (returned as text/plain)
- Response objects with custom status codes and headers

## Documentation

- **Full API**: [packages/node/README.md](../../packages/node/README.md)
- **Examples**: [examples/node/](../node/)
- **Issues**: https://github.com/Goldziher/spikard/issues

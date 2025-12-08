# Spikard WASM Deno Example

This example demonstrates how to use the Spikard WASM framework with the Deno runtime.

## Overview

Spikard WASM is a TypeScript/JavaScript HTTP framework that runs on WebAssembly. This example shows how to:

- Use Spikard WASM with Deno's native TypeScript support
- Define HTTP handlers with strict TypeScript types
- Create routes and handle JSON requests/responses
- Apply middleware for common HTTP concerns (CORS, compression, request IDs)
- Use Deno's native `Deno.serve()` API

## Prerequisites

- [Deno](https://deno.com) 1.40+
- Node.js 18+ (for pnpm workspace resolution)
- pnpm 8+ (for workspace package management)

## Project Structure

```
wasm-deno/
├── src/
│   └── main.ts          # Server implementation with 5 routes
├── deno.json            # Deno configuration with import map
├── tsconfig.json        # TypeScript compiler with strict settings
├── package.json         # Package metadata (workspace reference)
└── README.md            # This file
```

## Running the Server

### Development Mode (with auto-reload)

```bash
# From the workspace root
pnpm --filter=@examples/wasm-deno dev

# Or directly with Deno
deno run --allow-net --allow-read --watch src/main.ts
```

### Production Mode

```bash
# From the workspace root
pnpm --filter=@examples/wasm-deno start

# Or directly with Deno
deno run --allow-net --allow-read src/main.ts
```

The server will start on `http://0.0.0.0:8000`.

## Available Routes

### GET /

Returns a welcome message with available routes.

```bash
curl http://localhost:8000/
```

Response:

```json
{
  "success": true,
  "data": {
    "message": "Welcome to Spikard WASM on Deno!",
    "routes": ["GET /", "GET /api/data", "POST /api/echo", "GET /health"]
  },
  "timestamp": "2025-12-08T12:34:56.789Z"
}
```

### GET /api/data

Returns sample API data with a random ID.

```bash
curl http://localhost:8000/api/data
```

Response:

```json
{
  "success": true,
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "spikard-deno",
    "version": "0.3.7",
    "timestamp": "2025-12-08T12:34:56.789Z"
  },
  "timestamp": "2025-12-08T12:34:56.789Z"
}
```

### GET /health

Health check endpoint with status and operational checks.

```bash
curl http://localhost:8000/health
```

Response:

```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "checks": {
      "wasm": "operational",
      "deno": "operational"
    }
  },
  "timestamp": "2025-12-08T12:34:56.789Z"
}
```

### POST /api/echo

Echo back the JSON payload with received timestamp.

```bash
curl -X POST http://localhost:8000/api/echo \
  -H "Content-Type: application/json" \
  -d '{"message": "hello world"}'
```

Response:

```json
{
  "success": true,
  "data": {
    "message": "hello world",
    "received_at": "2025-12-08T12:34:56.789Z"
  },
  "timestamp": "2025-12-08T12:34:56.789Z"
}
```

## Implementation Details

### TypeScript Types

All handlers use strict TypeScript with the following types:

- `ApiResponse<T>`: Standard response wrapper with success flag, data, error, and timestamp
- `DataObject`: API data structure with id, name, version, and timestamp
- `EchoObject`: Echo response with message and received_at timestamp
- All properties are marked `readonly` for immutability

### Response Format

All endpoints return a consistent JSON structure:

```typescript
interface ApiResponse<T = unknown> {
  readonly success: boolean;
  readonly data?: T;
  readonly error?: string;
  readonly timestamp: string;
}
```

- `success`: Indicates if the request succeeded
- `data`: Response payload (varies by endpoint)
- `error`: Error message (only present if success is false)
- `timestamp`: ISO 8601 formatted timestamp of response creation

### Error Handling

- Invalid JSON in POST body returns 400 Bad Request with error message
- Missing required fields returns 400 Bad Request with validation message
- Unknown routes return 404 Not Found
- All error responses follow the standard `ApiResponse` format
- All responses include `application/json` content-type header

### Deno Permissions

The example requires:

- `--allow-net`: For HTTP server binding
- `--allow-read`: For source code reading (watch mode, optional)

## Configuration

### Import Map (deno.json)

The Deno configuration uses an import map to resolve `@spikard/wasm`:

```json
{
  "imports": {
    "@spikard/wasm": "npm:@spikard/wasm@0.3.7"
  }
}
```

This allows using npm packages directly in Deno code via the `npm:` scheme.

### TypeScript Configuration

The `tsconfig.json` enforces strict type checking:

- `strict`: true - Enables all strict type checking options
- `noUncheckedIndexedAccess`: true - Requires bounds checking for indexed access
- `exactOptionalPropertyTypes`: true - Strict optional property checking
- `noUnusedLocals`: true - Flags unused variables
- `noUnusedParameters`: true - Flags unused function parameters

## Development Tips

### Type Checking with Deno

Deno has built-in type checking. Check types without running:

```bash
deno check src/main.ts
```

### Formatting with Deno

Format code according to Deno standards:

```bash
deno fmt src/
```

### Linting with Deno

Check code quality:

```bash
deno lint src/
```

### Debugging

Run with `--inspect-brk` flag for debugging:

```bash
deno run --inspect-brk --allow-net src/main.ts
```

## Integration with pnpm Workspace

This example integrates with the monorepo's pnpm workspace. The `@spikard/wasm` dependency is resolved from the workspace using `"workspace:*"`, which ensures you're using the local development version rather than a published npm package.

When updating the workspace, run:

```bash
pnpm install
```

## Learning Resources

- [Spikard WASM Documentation](../../packages/wasm/README.md)
- [Deno Manual](https://deno.land/manual)
- [WASM JavaScript API](https://developer.mozilla.org/en-US/docs/WebAssembly/JavaScript_interface)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)

## License

MIT

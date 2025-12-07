# Spikard Node.js Examples

This directory contains runnable examples demonstrating Spikard Node.js bindings features.

## Prerequisites

1. Node.js 18+ with TypeScript support
2. Spikard Node.js package installed:
   ```bash
   cd packages/node
   pnpm install
   ```

## Running Examples

Each example is a standalone TypeScript file that can be run with `tsx` or compiled and run with Node:

### Using tsx (recommended for development)

```bash
npx tsx examples/node/01-basic-server.ts
```

### Using Node directly

```bash
# First compile TypeScript
npx tsc examples/node/01-basic-server.ts

# Then run
node examples/node/01-basic-server.js
```

Then visit `http://127.0.0.1:8000` in your browser or use curl.

## Examples

### 01. Basic Server (`01-basic-server.ts`)

The simplest possible Spikard application with two routes.

**Features:**
- Basic server configuration
- Simple GET routes
- Response helpers for text and JSON

**Try:**
```bash
npx tsx examples/node/01-basic-server.ts

# In another terminal:
curl http://127.0.0.1:8000
curl http://127.0.0.1:8000/health
```

---

### 02. Validation (`02-validation.ts`)

REST API with JSON request body validation, query parameters, and path parameters.

**Features:**
- JSON request/response handling
- Query parameter parsing
- Path parameters extraction
- Input validation with structured error responses
- HTTP status codes (201, 400, 404)

**Try:**
```bash
npx tsx examples/node/02-validation.ts

# In another terminal:
# List all users
curl http://127.0.0.1:8000/users

# Filter by name
curl 'http://127.0.0.1:8000/users?name=Alice'

# Get specific user
curl http://127.0.0.1:8000/users/1

# Create a user
curl -X POST http://127.0.0.1:8000/users \
  -H 'Content-Type: application/json' \
  -d '{"name":"Charlie","email":"charlie@example.com"}'
```

---

### 03. Streaming (`03-streaming.ts`)

Real-time server-to-client streaming for large datasets and Server-Sent Events (SSE).

**Features:**
- Streaming large datasets efficiently
- Server-Sent Events (SSE) for real-time updates
- Different streaming formats (NDJSON, CSV, SSE)
- Browser-based HTML demo page
- Generator-based streaming

**Try:**
```bash
npx tsx examples/node/03-streaming.ts

# Open in browser: http://127.0.0.1:8000
# Or use curl to see raw streams:

# Stream numbers as newline-delimited JSON
curl http://127.0.0.1:8000/stream/numbers?count=10

# Stream events as SSE (runs for 10 seconds)
curl http://127.0.0.1:8000/stream/events?duration=10

# Stream CSV data
curl http://127.0.0.1:8000/stream/csv
```

---

### 04. WebSocket & SSE (`04-websocket-sse.ts`)

Bidirectional WebSocket communication and advanced SSE patterns.

**Features:**
- WebSocket chat example
- WebSocket notifications stream
- SSE-based history replay
- SSE metrics stream
- Browser-based interactive demo

**Try:**
```bash
npx tsx examples/node/04-websocket-sse.ts

# Open in browser: http://127.0.0.1:8000
# Test chat, notifications, and metrics streams interactively
```

---

### 05. Lifecycle Hooks (`05-lifecycle-hooks.ts`)

Demonstrate lifecycle hooks for logging, authentication, and response transformation.

**Features:**
- Request/response logging
- Authentication with Bearer tokens
- Request ID tracking
- Custom error responses
- Authorization checks
- Response header manipulation
- Hook short-circuiting

**Try:**
```bash
npx tsx examples/node/05-lifecycle-hooks.ts

# Open in browser: http://127.0.0.1:8000
# Test public, protected, and admin endpoints

# Or use curl with Bearer tokens:

# Public endpoint (no auth)
curl http://127.0.0.1:8000/public

# Protected endpoint (requires token)
curl -H "Authorization: Bearer alice:secret" http://127.0.0.1:8000/protected

# Admin endpoint (alice only)
curl -H "Authorization: Bearer alice:secret" http://127.0.0.1:8000/admin/stats

# Try unauthorized:
curl http://127.0.0.1:8000/protected  # 401
curl -H "Authorization: Bearer bob:secret" http://127.0.0.1:8000/admin/stats  # 403
```

---

## Feature Status

| Feature | Status | Example |
|---------|--------|---------|
| Basic routing | ✅ Complete | 01, 02 |
| JSON responses | ✅ Complete | 02 |
| Query parameters | ✅ Complete | 02, 03 |
| Path parameters | ✅ Complete | 02 |
| Request validation | ✅ Complete | 02 |
| Streaming responses | ✅ Complete | 03 |
| Server-Sent Events | ✅ Complete | 03, 04 |
| WebSockets | ✅ Complete | 04 |
| Lifecycle hooks | ✅ Complete | 05 |
| Request/response logging | ✅ Complete | 05 |
| Authentication | ✅ Complete | 05 |
| Authorization | ✅ Complete | 05 |
| Error handling | ✅ Complete | 02, 05 |

## Next Steps

For more advanced examples, check:
- `packages/node/src/` - Full TypeScript source code
- `docs/adr/` - Architecture Decision Records
- `packages/node/README.md` - Complete Node.js binding documentation
- `testing_data/` - Test fixtures and validation schemas

## Common Patterns

### Error Handling

Return structured error responses:

```typescript
return {
	status: 400,
	body: {
		error: "Invalid input",
		code: "validation_error",
		details: { field: "email" },
	},
};
```

### Authentication

Use lifecycle hooks to check Bearer tokens:

```typescript
pre_handler: async (req) => {
	const token = req.headers?.["authorization"]?.split(" ")[1];
	if (!token || !isValidToken(token)) {
		return { status: 401, body: { error: "Unauthorized" } };
	}
	return req;
}
```

### Streaming Responses

Use TypeScript generators with `StreamingResponse`:

```typescript
async function* generateData() {
	for (let i = 0; i < 1000; i++) {
		yield { item: i };
		await new Promise(r => setTimeout(r, 10));
	}
}

return new StreamingResponse(generateData());
```

## Need Help?

- Documentation: `packages/node/README.md`
- Tests: `packages/node/src/testing.spec.ts`
- Issues: https://github.com/Goldziher/spikard/issues

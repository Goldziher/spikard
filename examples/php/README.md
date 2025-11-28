# Spikard PHP Examples

This directory contains runnable examples demonstrating Spikard PHP bindings features.

## Prerequisites

1. PHP 8.2+ with the Spikard extension loaded
2. Composer dependencies installed:
   ```bash
   cd packages/php
   composer install
   ```

## Running Examples

Each example is a standalone PHP script that can be run directly:

```bash
php examples/php/01-hello-world.php
```

Then visit `http://127.0.0.1:8000` in your browser or use curl:

```bash
curl http://127.0.0.1:8000
```

## Examples

### 01. Hello World (`01-hello-world.php`)
The simplest possible Spikard application. Single route returning plain text.

**Features:**
- Basic server configuration
- Simple GET route
- Text response

**Try:**
```bash
php 01-hello-world.php
curl http://127.0.0.1:8000
```

---

### 02. JSON API (`02-json-api.php`)
REST API with JSON request/response handling.

**Features:**
- JSON responses
- JSON request body parsing
- Multiple HTTP methods (GET, POST)
- Input validation

**Try:**
```bash
php 02-json-api.php

# GET all users
curl http://127.0.0.1:8000/users

# Create a user
curl -X POST http://127.0.0.1:8000/users \
  -H 'Content-Type: application/json' \
  -d '{"name":"Charlie","email":"charlie@example.com"}'
```

---

### 03. Background Tasks (`03-background-tasks.php`)
Offload slow work to background tasks without blocking responses.

**Features:**
- `BackgroundTask::run()` for async work
- Fire-and-forget execution
- Immediate HTTP responses
- Graceful shutdown

**Try:**
```bash
php 03-background-tasks.php

# Create a user (returns immediately while email sends in background)
curl -X POST http://127.0.0.1:8000/users \
  -H 'Content-Type: application/json' \
  -d '{"name":"Alice","email":"alice@example.com"}'

# Check server logs to see background tasks executing
```

---

### 04. Server-Sent Events (`04-streaming-sse.php`)
Real-time server-to-client streaming using SSE.

**Features:**
- `StreamingResponse::sse()` helper
- PHP Generator for event streaming
- Automatic SSE formatting
- Browser EventSource client

**Try:**
```bash
php 04-streaming-sse.php

# Open in browser to see live updates
open http://127.0.0.1:8000

# Or use curl to see raw SSE stream
curl http://127.0.0.1:8000/events
```

---

### 05. Dependency Injection (`05-dependency-injection.php`)
Register and resolve dependencies using the DI container.

**Features:**
- Value dependencies (singletons)
- Factory dependencies with `Provide`
- Dependency resolution with `dependsOn`
- DI container configuration

**Try:**
```bash
php 05-dependency-injection.php
curl http://127.0.0.1:8000/users

# Check logs to see DI container initializing dependencies
```

**Note:** P0.2 (DI system) is complete. P1.4 (automatic parameter injection) is planned.

---

## Feature Status

| Feature | Status | Example |
|---------|--------|---------|
| Basic routing | ✅ Complete | 01, 02 |
| JSON responses | ✅ Complete | 02 |
| Background tasks | ✅ Complete (P0.1) | 03 |
| Streaming responses | ✅ Complete (P0.3) | 04 |
| Server-Sent Events | ✅ Complete (P0.3) | 04 |
| Dependency injection | ✅ Complete (P0.2) | 05 |
| Parameter extraction | ⏳ Planned (P1.4) | - |
| WebSockets | ✅ Available | - |
| File uploads | ✅ Available | - |
| Middleware | ✅ Available | - |

## More Examples

Additional examples planned:
- Path parameters and routing
- Query parameters
- Headers and cookies
- File uploads
- WebSocket chat
- Authentication middleware
- CORS configuration
- Rate limiting
- Lifecycle hooks

## Need Help?

- Documentation: `packages/php/README.md`
- Tests: `e2e/php/`
- Issues: https://github.com/Goldziher/spikard/issues

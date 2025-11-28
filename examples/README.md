# Spikard Examples

Comprehensive examples demonstrating Spikard capabilities across multiple languages and patterns.

## Quick Start

### Explore Schemas

Start with the API specifications:

```bash
# View all schemas
ls schemas/

# Specifications available:
# - todo-api.openapi.yaml (REST CRUD with validation)
# - file-service.openapi.yaml (multipart uploads, streaming)
# - chat-service.asyncapi.yaml (WebSocket bidirectional)
# - events-stream.asyncapi.yaml (Server-Sent Events)
# - auth-service.openapi.yaml (JWT, API keys, OAuth 2.0)
```

### Run an Example

**PHP** (includes working implementation):
```bash
cd php
composer install
composer start
```

**Other Languages** (directory structure available for implementation):
```bash
cd node-simple  # Node.js example
cd rust-lifecycle-hooks  # Rust lifecycle hooks demo
```

## Schemas Overview

| Schema | Type | Features |
|--------|------|----------|
| **todo-api** | REST | CRUD, pagination, filtering, sorting, auth |
| **file-service** | REST | Multipart upload, streaming, ranges, verification |
| **chat-service** | WebSocket | Bidirectional messaging, presence, typing indicators |
| **events-stream** | SSE | Server-to-client streaming, filtering, heartbeat |
| **auth-service** | REST | JWT, API keys, OAuth 2.0, token refresh |

## Directory Structure

```
examples/
├── README.md                       # This file
├── schemas/                        # OpenAPI & AsyncAPI specifications
│   ├── README.md                   # Schema suite guide
│   ├── todo-api.openapi.yaml
│   ├── file-service.openapi.yaml
│   ├── chat-service.asyncapi.yaml
│   ├── events-stream.asyncapi.yaml
│   └── auth-service.openapi.yaml
├── php/                            # PHP implementation (PSR-4)
├── node-simple/                    # Node.js example
├── rust-lifecycle-hooks/           # Rust lifecycle demo
├── asyncapi/                       # AsyncAPI examples
└── di/                             # Dependency injection examples
```

## Features Demonstrated

**HTTP Patterns:**
- Path/query parameters with validation
- Request/response validation (JSON Schema)
- Multiple status codes and error handling (RFC 9457)
- File upload (multipart/form-data) and streaming
- Conditional requests (ETag, If-Modified-Since)
- Range requests (206 Partial Content)

**Async Patterns:**
- WebSocket bidirectional messaging
- Server-Sent Events (SSE) with filtering
- Heartbeat/keep-alive messages
- Reconnection with catch-up

**Security:**
- Bearer token authentication
- API key management and rotation
- OAuth 2.0 authorization code flow
- Scope-based authorization
- Rate limiting

**Middleware:**
- Request ID generation
- Compression (gzip/brotli)
- Timeouts
- Body size limits
- Error handling

**Lifecycle Hooks:**
- onRequest - Connection/authentication
- preValidation - Before schema validation
- preHandler - Before business logic
- onResponse - Before response sent
- onError - On error or exception

## Language Support

Spikard provides bindings for:
- **Python** (PyO3 - msgspec models, asyncio)
- **Node.js** (napi-rs - TypeScript, strict mode)
- **Ruby** (magnus - RBS type definitions)
- **PHP** (ext-php-rs - PSR-4 autoloading)
- **WebAssembly** (wasm-bindgen)

## Testing

All examples validate against shared fixtures:

```bash
task test              # Run all tests
task test:python       # Language-specific tests
task test:node
task test:ruby
task test:php
```

**Fixture Location:** `testing_data/`
- `headers/` - Header validation scenarios
- `cookies/` - Cookie handling
- `json_bodies/` - Request body examples
- `validation_errors/` - RFC 9457 error format
- `status_codes/` - HTTP status scenarios
- `rate_limit/` - Rate limiting behavior

## Code Generation

Generate handler stubs from schemas:

```bash
# OpenAPI to Python
openapi-generator-cli generate \
  -i schemas/todo-api.openapi.yaml \
  -g python-flask -o generated/

# OpenAPI to TypeScript
openapi-generator-cli generate \
  -i schemas/todo-api.openapi.yaml \
  -g typescript-axios -o generated/

# AsyncAPI to models
asyncapi generate fromTemplate \
  schemas/chat-service.asyncapi.yaml \
  @asyncapi/python-pydantic-schema
```

## Key Schema Files

- **schemas/README.md** - Detailed schema descriptions and feature matrix
- **schemas/todo-api.openapi.yaml** - Full OpenAPI 3.1 example with 6 endpoints
- **schemas/file-service.openapi.yaml** - File operations with 7 endpoints
- **schemas/chat-service.asyncapi.yaml** - WebSocket with 10 message types
- **schemas/events-stream.asyncapi.yaml** - SSE with 6 event types
- **schemas/auth-service.openapi.yaml** - Authentication with 9 endpoints

## Validation & Linting

```bash
# Validate schemas
swagger-cli validate schemas/todo-api.openapi.yaml
asyncapi validate schemas/chat-service.asyncapi.yaml

# Lint all code
task lint

# Format code
task format
```

## Common Patterns

**Request Validation:**
```yaml
# Schema validates: type, minLength, maxLength, pattern, format
# Enum constraints, required fields, additional properties
# Complex: discriminator, oneOf, anyOf, allOf
```

**Error Responses:**
```json
{
  "error": "string",
  "code": "string",
  "details": {}
}
```

**Rate Limiting Headers:**
```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 45
X-RateLimit-Reset: 1234567890
```

## References

- [OpenAPI 3.1.0 Spec](https://spec.openapis.org/oas/v3.1.0)
- [AsyncAPI 3.0.0 Spec](https://www.asyncapi.com/docs/specifications/latest)
- [RFC 9457 - Problem Details](https://tools.ietf.org/html/rfc9457)
- [Tower-HTTP Middleware](https://github.com/tower-rs/tower-http)

## Next Steps

1. **Review schemas** - Start with `schemas/README.md`
2. **Choose implementation** - Pick a language from `php/`, `node-simple/`, etc.
3. **Run tests** - Validate against fixtures with `task test`
4. **Generate code** - Use OpenAPI/AsyncAPI generators
5. **Implement handlers** - Follow existing patterns in your language

---

**For detailed architecture and ADRs, see:** `docs/adr/`

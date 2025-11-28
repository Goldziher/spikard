# Spikard Examples Suite

Comprehensive, production-ready examples demonstrating all Spikard capabilities across multiple languages and patterns.

## Overview

This suite includes 5 focused API specifications and multi-language implementations showcasing:

- **REST APIs** with CRUD operations, validation, and error handling
- **File Operations** with streaming uploads/downloads and multipart forms
- **WebSocket** bidirectional messaging with presence tracking
- **Server-Sent Events** (SSE) streaming with reconnection handling
- **Authentication** with JWT, API keys, and OAuth 2.0

All examples are:
- ✓ Fully typed with strict type checking
- ✓ Thoroughly documented with inline comments
- ✓ Fixture-driven tested against shared test data
- ✓ Production-ready and security-hardened
- ✓ Demonstrating Spikard's full feature set

## Getting Started

### 1. Explore the Schemas

Start by reading the API specifications to understand what we're building:

```bash
# Overview of all schemas
cat schemas/README.md

# Individual schemas
cat schemas/todo-api.openapi.yaml
cat schemas/file-service.openapi.yaml
cat schemas/chat-service.asyncapi.yaml
cat schemas/events-stream.asyncapi.yaml
cat schemas/auth-service.openapi.yaml
```

### 2. Choose Your Language

Pick a language and explore the implementation:

**Python** (fully async with msgspec):
```bash
cd python/todo-api
cat README.md
python -m venv venv
source venv/bin/activate
pip install -e .
python src/app.py
```

**Node.js** (TypeScript with strict typing):
```bash
cd node/todo-api
cat README.md
pnpm install
pnpm dev
```

**Ruby** (with RBS type definitions):
```bash
cd ruby/todo-api
cat README.md
bundle install
bundle exec ruby lib/app.rb
```

**PHP** (PSR compliant):
```bash
cd php/todo-api
cat README.md
composer install
composer start
```

**WebAssembly** (Rust):
```bash
cd wasm/todo-api
cat README.md
wasm-pack build --target nodejs
npm start
```

### 3. Run Tests

All examples are tested against shared fixtures:

```bash
# Run all tests
task test

# Run specific language tests
task test:python
task test:node
task test:ruby
task test:php

# Validate schemas
task lint
```

### 4. Generate Code

Use the schemas as a starting point for code generation:

```bash
# Generate from OpenAPI
openapi-generator-cli generate \
  -i schemas/todo-api.openapi.yaml \
  -g python-flask \
  -o generated/

# Generate from AsyncAPI
asyncapi generate fromTemplate \
  schemas/chat-service.asyncapi.yaml \
  @asyncapi/python-pydantic-schema
```

## Schema Suite

### 1. Todo API (REST CRUD)

**File:** `schemas/todo-api.openapi.yaml`

A complete CRUD REST API demonstrating:

- Path parameters (UUID extraction)
- Query parameters (filtering, pagination, sorting)
- Request/response validation
- Multiple HTTP methods (GET, POST, PUT, DELETE)
- Multiple status codes with different response schemas
- RFC 9457 error responses
- Bearer token authentication
- Rate limiting headers
- Lifecycle hooks (onRequest, preValidation, preHandler, onResponse, onError)

**Key Endpoints:**
```
GET    /health                    Health check
GET    /todos                     List todos (paginated)
POST   /todos                     Create todo
GET    /todos/{id}                Get todo by ID
PUT    /todos/{id}                Update todo
DELETE /todos/{id}                Delete todo
```

**Features:**
- 🔍 Filtering (status, search text)
- 📊 Sorting (field, order)
- 📄 Pagination (page, limit)
- ✅ Full validation
- 🔐 Authentication required
- ⏱️ Rate limiting (10 req/min)

---

### 2. File Service (Upload & Streaming)

**File:** `schemas/file-service.openapi.yaml`

Advanced file handling demonstrating:

- Multipart form-data uploads
- File size/MIME type validation
- Binary streaming responses (downloads)
- Conditional requests (ETag, If-Modified-Since)
- Range requests (206 Partial Content)
- Large payload handling (413 Payload Too Large)
- File integrity verification (SHA-256 hashing)
- Compression (gzip/brotli)
- Caching strategies

**Key Endpoints:**
```
POST   /files/upload              Upload file
GET    /files                     List uploaded files
GET    /files/{fileId}            Download file (streaming)
DELETE /files/{fileId}            Delete file
GET    /files/{fileId}/info       Get file metadata
POST   /files/{fileId}/verify     Verify integrity
GET    /storage/quota             Get storage quota
```

**Features:**
- 📦 Multipart form parsing
- 🔒 MIME type validation
- 📏 Size limiting (max 50MB)
- 📥 Stream downloads
- 📍 Range requests
- 🔐 Hash verification
- ⏱️ Rate limiting (10 uploads/min)

---

### 3. Chat Service (WebSocket)

**File:** `schemas/chat-service.asyncapi.yaml`

Real-time bidirectional messaging demonstrating:

- WebSocket protocol (wss/ws)
- Multiple message types on single channel
- User presence tracking (join/leave)
- Message acknowledgments
- Typing indicators
- Error handling with structured payloads
- Connection state management
- Lifecycle hooks for auth and cleanup

**Message Types:**

Client → Server:
```
chatMessage         Send message to room
typingIndicator     Indicate user is typing
presenceRequest     Request online users
```

Server → Client:
```
chatMessageBroadcast  Message broadcast to all
chatAck               Acknowledge delivery
userJoined            User joined room
userLeft              User left room
userTyping            Another user typing
presenceUpdate        Current online users
chatError             Error response
connectionClosed      Connection closing
```

**Features:**
- 💬 Bidirectional messaging
- 👥 User presence tracking
- ✍️ Typing indicators
- 📬 Message acknowledgments
- 🔐 Authentication required
- ⏱️ Rate limiting (20 msg/min)
- ❌ Structured error handling

---

### 4. Events Stream (SSE)

**File:** `schemas/events-stream.asyncapi.yaml`

Server-Sent Events streaming demonstrating:

- Server-to-client streaming (HTTP)
- Multiple event types multiplexing
- Event filtering (type, severity, user)
- Heartbeat/keep-alive messages (every 30s)
- Reconnection handling with batch catch-up
- Memory-efficient buffering
- Lifecycle hooks for connection validation

**Event Types:**
```
systemAlert        Critical system alerts (info/warning/error/critical)
userNotification   User-specific notifications
statusUpdate       Service status changes
heartbeat          Keep-alive signal
notificationBatch  Batch for reconnected clients
streamError        Error messages
```

**Features:**
- 🌊 Server-to-client streaming
- 📡 Multiple event types
- 🔍 Event filtering
- 💓 Automatic heartbeat (30s)
- 🔄 Reconnection support
- 📦 Batch catch-up
- ⏱️ Rate limiting
- 🔐 Authentication required

---

### 5. Auth Service (Security)

**File:** `schemas/auth-service.openapi.yaml`

Comprehensive authentication demonstrating:

- JWT bearer tokens
- API key management
- OAuth 2.0 authorization code flow
- Token refresh
- API key rotation
- Scope-based authorization
- Rate limiting per API key
- Audit logging patterns

**Key Endpoints:**
```
POST   /auth/api-keys               Generate API key
GET    /auth/api-keys               List API keys
DELETE /auth/api-keys/{keyId}       Revoke API key
POST   /auth/token                  Issue JWT token
POST   /auth/refresh                Refresh token
POST   /auth/logout                 Logout (revoke token)
POST   /auth/verify                 Verify token
GET    /auth/oauth/authorize        OAuth authorize endpoint
POST   /auth/oauth/token            OAuth token exchange
```

**Features:**
- 🔑 API key management
- 🎫 JWT tokens
- 🔄 Token refresh
- 🔐 OAuth 2.0 flow
- 🔍 Token verification
- 📝 Audit logging
- ⏱️ Rate limiting

---

## Language Implementations

### Python

**Path:** `python/*/`

Stack:
- Spikard (PyO3 bindings)
- msgspec (JSON serialization)
- asyncio (async runtime)
- pytest (testing)
- mypy --strict (type checking)

Features:
- Fully async handlers
- msgspec.Struct models (not pydantic)
- Type hints on all functions
- Fixture-driven tests
- Strict type checking

**Example:**
```bash
cd python/todo-api
uv pip install -e .
python src/app.py
pytest tests/
```

---

### Node.js/TypeScript

**Path:** `node/*/`

Stack:
- Spikard (napi-rs bindings)
- TypeScript 5.x (strict mode)
- Biome (linting/formatting)
- Vitest (testing)
- pnpm (package manager)

Features:
- Strict TypeScript configuration
- Biome for formatting
- Vitest for testing
- Function-based handlers
- Type-safe dependency injection

**Example:**
```bash
cd node/todo-api
pnpm install
pnpm dev
pnpm test
```

---

### Ruby

**Path:** `ruby/*/`

Stack:
- Spikard (magnus bindings)
- RBS type definitions
- Steep (type checker)
- RSpec (testing)
- Rubocop (linting)

Features:
- RBS type definitions
- Steep type checking
- Idiomatic Ruby code
- RSpec BDD tests
- Rubocop linting

**Example:**
```bash
cd ruby/todo-api
bundle install
bundle exec ruby lib/app.rb
bundle exec rspec
```

---

### PHP

**Path:** `php/*/`

Stack:
- Spikard (ext-php-rs bindings)
- PHP 8.2+ (strict types)
- PSR-4 autoloading
- PHPStan (static analysis)
- PHPUnit (testing)

Features:
- PSR-4 autoloading
- PSR-12 coding style
- Strict types on all files
- PHPStan level max
- Type hints required

**Example:**
```bash
cd php/todo-api
composer install
composer start
composer test
```

---

### WebAssembly

**Path:** `wasm/*/`

Stack:
- Rust (WASM runtime)
- wasm-bindgen (FFI)
- wasm-pack (bundling)
- JavaScript interop

Features:
- Minimal binary size
- Zero-copy operations
- Async via promises
- No blocking I/O
- Direct JS interop

**Example:**
```bash
cd wasm/todo-api
wasm-pack build --target nodejs
npm start
```

---

## Features Demonstrated

### By Example

| Feature | todo-api | file-service | chat | events | auth |
|---------|----------|--------------|------|--------|------|
| **REST CRUD** | ✓ | ✓ | | | ✓ |
| **Path Params** | ✓ | ✓ | | | ✓ |
| **Query Params** | ✓ | ✓ | | | |
| **Validation** | ✓ | ✓ | ✓ | ✓ | ✓ |
| **File Upload** | | ✓ | | | |
| **Streaming** | | ✓ | | ✓ | |
| **WebSocket** | | | ✓ | | |
| **SSE** | | | | ✓ | |
| **Auth** | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Rate Limit** | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Error Handling** | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Lifecycle Hooks** | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Pagination** | ✓ | ✓ | | | |
| **Filtering** | ✓ | | | ✓ | |
| **Sorting** | ✓ | ✓ | | | |
| **Caching** | ✓ | ✓ | | | |

### By Spikard Feature

**Core Features:**
- ✓ HTTP request/response handling
- ✓ WebSocket bidirectional messaging
- ✓ Server-Sent Events streaming
- ✓ Request validation (headers, body, params)
- ✓ Error handling with RFC 9457 format
- ✓ Rate limiting with headers
- ✓ Authentication (Bearer, API Key, OAuth 2.0)
- ✓ Lifecycle hooks (onRequest, preValidation, preHandler, onResponse, onError)

**Middleware:**
- ✓ Request ID generation
- ✓ Timeouts
- ✓ Compression (gzip/brotli)
- ✓ CORS
- ✓ Static files
- ✓ Body size limits

**Language Bindings:**
- ✓ Python (PyO3)
- ✓ Node.js (napi-rs)
- ✓ Ruby (magnus)
- ✓ PHP (ext-php-rs)
- ✓ WebAssembly (wasm-bindgen)

---

## Testing

All examples are tested using fixture-driven approach:

```bash
# Run all tests
task test

# Run specific language
task test:python
task test:node
task test:ruby
task test:php

# Run specific test
pytest python/todo-api/tests/test_handlers.py
pnpm -C node/todo-api test
bundle exec rspec ruby/todo-api/spec/handlers_spec.rb
composer -C php/todo-api test
```

**Fixtures location:**
```
testing_data/
├── headers/                  # Header validation
├── cookies/                  # Cookie scenarios
├── json_bodies/              # Request body examples
├── validation_errors/        # Error response format
├── status_codes/             # HTTP status scenarios
├── rate_limit/               # Rate limiting behavior
└── cors/                     # CORS configurations
```

---

## Development Workflow

### 1. Schema Design

Start with the OpenAPI/AsyncAPI schema:

```bash
# View the spec
cat schemas/todo-api.openapi.yaml

# Validate the spec
swagger-cli validate schemas/todo-api.openapi.yaml

# Generate documentation
swagger-ui schema-file=schemas/todo-api.openapi.yaml
```

### 2. Code Generation

Generate handler stubs from schemas:

```bash
# OpenAPI to Python
openapi-generator-cli generate \
  -i schemas/todo-api.openapi.yaml \
  -g python-flask \
  -o generated/

# OpenAPI to TypeScript
openapi-generator-cli generate \
  -i schemas/todo-api.openapi.yaml \
  -g typescript-axios \
  -o generated/

# AsyncAPI to Python
asyncapi generate fromTemplate \
  schemas/chat-service.asyncapi.yaml \
  @asyncapi/python-pydantic-schema \
  -o generated/chat/
```

### 3. Implementation

Implement handlers following the pattern:

```python
# python/todo-api/src/handlers.py
async def list_todos(request: Request) -> TodoListResponse:
    """List all todos with pagination and filtering."""
    # Extract and validate query parameters
    # Fetch from database
    # Return paginated response
    pass
```

### 4. Testing

Write fixture-driven tests:

```python
# python/todo-api/tests/test_handlers.py
@pytest.mark.asyncio
async def test_list_todos(client: TestClient):
    """Test listing todos from fixture."""
    response = await client.get("/todos?page=1&limit=20")
    assert response.status_code == 200
    # Validate against schema
```

### 5. Deployment

Deploy using language-specific tools:

```bash
# Python: pip, uv
# Node: npm, pnpm, yarn
# Ruby: gem, bundler
# PHP: composer
# WASM: npm, wasm-pack
```

---

## Documentation

- **Schema Guide:** `schemas/README.md` - Overview of all specifications
- **Structure Guide:** `STRUCTURE.md` - Directory organization and layout
- **OpenAPI 3.1:** https://spec.openapis.org/oas/v3.1.0
- **AsyncAPI 3.0:** https://www.asyncapi.com/docs/specifications/latest
- **RFC 9457:** https://tools.ietf.org/html/rfc9457

---

## Contributing Examples

To add a new example:

1. **Create schema** in `schemas/` (OpenAPI or AsyncAPI)
2. **Create language folders** in each language directory
3. **Implement handlers** following existing patterns
4. **Add tests** using fixtures
5. **Document** with README in each folder
6. **Update** schemas/README.md and STRUCTURE.md

---

## Common Tasks

### Run all examples
```bash
task setup        # Install all dependencies
task build        # Build all bindings
task test         # Run all tests
task lint         # Lint all code
```

### Test a specific language
```bash
task test:python
task test:node
task test:ruby
task test:php
```

### Generate API documentation
```bash
# Swagger UI
docker run -p 80:8080 -e SWAGGER_JSON=/schemas/todo-api.openapi.yaml \
  -v $(pwd)/schemas:/schemas swaggerapi/swagger-ui

# ReDoc
docker run -p 8080:80 -e SPEC_URL=/schemas/todo-api.openapi.yaml \
  -v $(pwd)/schemas:/schemas redocly/redoc
```

### Validate schemas
```bash
# OpenAPI
swagger-cli validate schemas/todo-api.openapi.yaml

# AsyncAPI
asyncapi validate schemas/chat-service.asyncapi.yaml
```

---

## Resources

- **Spikard Documentation:** https://github.com/spikard/spikard
- **OpenAPI Specification:** https://spec.openapis.org/oas/v3.1.0
- **AsyncAPI Specification:** https://www.asyncapi.com/
- **RFC 9457 Problem Details:** https://tools.ietf.org/html/rfc9457
- **Tower-HTTP Middleware:** https://github.com/tower-rs/tower-http

---

## License

MIT - See LICENSE file in repository root

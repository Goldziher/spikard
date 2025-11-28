# Examples Structure & Organization

Comprehensive guide to the Spikard examples suite demonstrating all features across multiple languages and patterns.

## Directory Layout

```
examples/
├── STRUCTURE.md                    # This file - organization guide
├── schemas/                        # API specifications
│   ├── README.md                   # Schema suite overview & features
│   ├── todo-api.openapi.yaml       # REST CRUD with validation
│   ├── file-service.openapi.yaml   # File uploads/downloads
│   ├── chat-service.asyncapi.yaml  # WebSocket bidirectional
│   ├── events-stream.asyncapi.yaml # Server-Sent Events
│   └── auth-service.openapi.yaml   # Authentication & security
│
├── python/                         # Python implementation examples
│   ├── todo-api/                   # CRUD API implementation
│   │   ├── src/
│   │   │   ├── app.py              # Main Spikard app
│   │   │   ├── handlers.py         # Route handlers
│   │   │   ├── models.py           # Data models (msgspec)
│   │   │   └── lifecycle.py        # Lifecycle hook implementations
│   │   ├── tests/
│   │   │   └── test_handlers.py    # Integration tests
│   │   ├── pyproject.toml          # Project config
│   │   └── README.md               # Example documentation
│   │
│   ├── file-service/               # File upload/download
│   │   ├── src/
│   │   │   ├── app.py
│   │   │   ├── handlers.py
│   │   │   ├── storage.py          # File storage layer
│   │   │   └── validation.py
│   │   ├── tests/
│   │   └── README.md
│   │
│   ├── chat-service/               # WebSocket chat
│   │   ├── src/
│   │   │   ├── app.py              # WebSocket server setup
│   │   │   ├── handlers.py         # Message handlers
│   │   │   ├── rooms.py            # Room management
│   │   │   └── lifecycle.py
│   │   ├── tests/
│   │   └── README.md
│   │
│   ├── events-stream/              # SSE event streaming
│   │   ├── src/
│   │   │   ├── app.py              # SSE server setup
│   │   │   ├── handlers.py
│   │   │   ├── event_queue.py      # Event queue/buffer
│   │   │   └── subscribers.py      # Subscriber management
│   │   ├── tests/
│   │   └── README.md
│   │
│   └── auth-service/               # Authentication
│       ├── src/
│       │   ├── app.py
│       │   ├── handlers.py
│       │   ├── tokens.py           # JWT token management
│       │   ├── keys.py             # API key management
│       │   └── oauth.py            # OAuth 2.0 flow
│       ├── tests/
│       └── README.md
│
├── node/                           # Node.js/TypeScript examples
│   ├── todo-api/
│   │   ├── src/
│   │   │   ├── index.ts            # Main server
│   │   │   ├── handlers.ts         # Route handlers
│   │   │   ├── models.ts           # TypeScript interfaces
│   │   │   └── lifecycle.ts        # Lifecycle hooks
│   │   ├── tests/
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   └── README.md
│   │
│   ├── file-service/
│   ├── chat-service/
│   ├── events-stream/
│   └── auth-service/
│
├── ruby/                           # Ruby examples
│   ├── todo-api/
│   │   ├── lib/
│   │   │   ├── app.rb              # Main Spikard app
│   │   │   ├── handlers.rb
│   │   │   └── models.rb           # RBS definitions
│   │   ├── sig/
│   │   │   └── app.rbs             # Type signatures
│   │   ├── spec/
│   │   ├── Gemfile
│   │   └── README.md
│   │
│   ├── file-service/
│   ├── chat-service/
│   ├── events-stream/
│   └── auth-service/
│
├── php/                            # PHP examples
│   ├── todo-api/
│   │   ├── src/
│   │   │   ├── App.php             # Main server
│   │   │   ├── Handlers/
│   │   │   └── Models/
│   │   ├── tests/
│   │   ├── composer.json
│   │   └── README.md
│   │
│   ├── file-service/
│   ├── chat-service/
│   ├── events-stream/
│   └── auth-service/
│
├── wasm/                           # WebAssembly examples
│   ├── todo-api/
│   │   ├── src/
│   │   │   ├── lib.rs              # WASM entry point
│   │   │   └── handlers.rs
│   │   ├── Cargo.toml
│   │   ├── wasm-pack.toml
│   │   └── README.md
│   │
│   ├── file-service/
│   ├── chat-service/
│   └── events-stream/
│
└── README.md                       # Main examples guide
```

## Schema Files Overview

### 1. `schemas/todo-api.openapi.yaml`
**REST CRUD Operations**

Demonstrates:
- Path parameters (UUID extraction)
- Query parameters (filtering, pagination, sorting)
- Request body validation (JSON Schema)
- Multiple HTTP methods (GET, POST, PUT, DELETE)
- Multiple status codes (200, 201, 204, 400, 404, 409)
- RFC 9457 error responses
- Bearer token authentication
- Rate limiting headers
- Lifecycle hooks

**Endpoints:**
- `GET /health` - Health check
- `GET /todos` - List with pagination
- `POST /todos` - Create
- `GET /todos/{id}` - Get by ID
- `PUT /todos/{id}` - Update
- `DELETE /todos/{id}` - Delete

**Features:**
- Full CRUD operations
- Pagination (page, limit)
- Filtering (status, search)
- Sorting (field, order)
- Field validation
- Structured error responses

---

### 2. `schemas/file-service.openapi.yaml`
**File Upload & Streaming**

Demonstrates:
- Multipart form-data file uploads
- Binary streaming responses (downloads)
- File size/MIME type validation
- Request body limits (413 Payload Too Large)
- Conditional requests (ETag, If-Modified-Since)
- Range requests (206 Partial Content)
- Large payload handling
- File integrity verification (SHA-256)
- Rate limiting per user
- Compression (gzip/brotli)

**Endpoints:**
- `POST /files/upload` - Upload with validation
- `GET /files` - List uploaded files
- `GET /files/{fileId}` - Download file (streaming)
- `DELETE /files/{fileId}` - Delete file
- `GET /files/{fileId}/info` - Get metadata
- `POST /files/{fileId}/verify` - Verify hash
- `GET /storage/quota` - Get storage quota

**Features:**
- Multipart form parsing
- File type validation
- Size limiting
- Progress tracking
- Cache control (ETag, Last-Modified)
- Hash-based integrity verification
- Storage quota management

---

### 3. `schemas/chat-service.asyncapi.yaml`
**WebSocket Bidirectional Messaging**

Demonstrates:
- WebSocket bidirectional communication
- Multiple message types on single channel
- User presence tracking (join/leave)
- Message acknowledgments
- Typing indicators
- Presence updates
- Error handling with structured payloads
- Connection state management
- Lifecycle hooks (auth, validation, cleanup)

**Messages:**

Client → Server:
- `chatMessage` - Send message
- `typingIndicator` - User is typing
- `presenceRequest` - Request online users

Server → Client:
- `chatMessageBroadcast` - Message broadcast
- `chatAck` - Acknowledge delivery
- `userJoined` - User joined room
- `userLeft` - User left room
- `userTyping` - Another user typing
- `presenceUpdate` - Current online users
- `chatError` - Error response
- `connectionClosed` - Connection closing

**Features:**
- Bidirectional messaging
- User presence
- Message acknowledgments
- Typing indicators
- Error handling
- Connection state
- Rate limiting (20 msg/min)

---

### 4. `schemas/events-stream.asyncapi.yaml`
**Server-Sent Events Streaming**

Demonstrates:
- Server-Sent Events (SSE) protocol
- One-way server-to-client streaming
- Multiple event types (multiplexing)
- Heartbeat/keep-alive messages
- Reconnection handling
- Batch events for catch-up
- Event filtering by type/severity
- Connection pooling
- Memory-efficient buffering
- Lifecycle hooks

**Event Types:**
- `systemAlert` - Critical alerts (severity: info/warning/error/critical)
- `userNotification` - User-specific notifications
- `statusUpdate` - Service status changes
- `heartbeat` - Keep-alive signal (every 30s)
- `notificationBatch` - Batch for reconnection
- `streamError` - Error responses

**Features:**
- Server-to-client streaming
- Multiple event types
- Event filtering (type, severity, user)
- Automatic heartbeat
- Batch catch-up
- RFC 9457 errors
- Connection recovery

---

### 5. `schemas/auth-service.openapi.yaml`
**Authentication & Authorization**

Demonstrates:
- Multiple auth schemes (Bearer, API Key)
- JWT token management
- API key generation/rotation
- OAuth 2.0 authorization code flow
- Token refresh
- Scope-based authorization
- Rate limiting per API key
- Audit logging patterns
- Security best practices

**Endpoints:**
- `POST /auth/api-keys` - Generate API key
- `GET /auth/api-keys` - List API keys
- `DELETE /auth/api-keys/{keyId}` - Revoke API key
- `POST /auth/token` - Issue JWT token
- `POST /auth/refresh` - Refresh token
- `POST /auth/logout` - Revoke token
- `POST /auth/verify` - Verify token
- `GET /auth/oauth/authorize` - OAuth authorize
- `POST /auth/oauth/token` - OAuth token exchange

**Features:**
- Multiple auth methods
- Token generation/refresh
- API key management
- OAuth 2.0 flow
- Scope-based access
- Rate limiting
- Audit logging
- Token verification

---

## Language-Specific Examples

### Python (`examples/python/*/`)

**Stack:**
- Spikard (PyO3 bindings)
- msgspec (JSON serialization)
- asyncio (async runtime)
- httpx (HTTP client in tests)
- asyncpg (database access)
- pytest (testing)

**Structure per example:**
```
python/{service}/
├── src/
│   ├── app.py                 # Spikard app setup
│   ├── handlers.py            # Request handlers
│   ├── models.py              # msgspec.Struct models
│   ├── lifecycle.py           # Hook implementations
│   └── [service-specific].py  # Domain logic
├── tests/
│   ├── test_handlers.py       # Integration tests
│   └── conftest.py            # Test fixtures
├── pyproject.toml
└── README.md
```

**Key patterns:**
- Fully async handlers using `async def`
- msgspec for serialization (not pydantic)
- Type hints on all functions
- mypy --strict type checking
- Lifecycle hooks with async support
- Fixture-driven tests

---

### Node.js/TypeScript (`examples/node/*/`)

**Stack:**
- Spikard (napi-rs bindings)
- TypeScript 5.x with strict mode
- Biome (linting/formatting)
- Vitest (testing)
- pnpm (package manager)

**Structure per example:**
```
node/{service}/
├── src/
│   ├── index.ts               # Server setup
│   ├── handlers.ts            # Route handlers
│   ├── models.ts              # TypeScript types
│   ├── lifecycle.ts           # Hook implementations
│   └── [service].ts           # Domain logic
├── tests/
│   └── handlers.spec.ts       # Tests
├── package.json
├── tsconfig.json
├── biome.json
└── README.md
```

**Key patterns:**
- Strict TypeScript configuration
- Function-based handlers
- Zod for validation (or generated schemas)
- Biome for formatting
- pnpm workspaces
- Vitest for testing

---

### Ruby (`examples/ruby/*/`)

**Stack:**
- Spikard (magnus bindings)
- RBS for type definitions
- Steep for type checking
- RSpec for testing
- Rubocop for linting

**Structure per example:**
```
ruby/{service}/
├── lib/
│   ├── app.rb                 # Server setup
│   ├── handlers.rb            # Route handlers
│   └── [service]/             # Service modules
├── sig/
│   ├── app.rbs                # Type definitions
│   └── [service]/
├── spec/
│   └── handlers_spec.rb       # Tests
├── Gemfile
├── .ruby-version
├── steep.yaml
└── README.md
```

**Key patterns:**
- RBS type definitions
- Steep for type checking
- Clean Ruby idiomatic code
- Guard clauses
- Rubocop linting
- RSpec function-like tests

---

### PHP (`examples/php/*/`)

**Stack:**
- Spikard (ext-php-rs bindings)
- PHP 8.2+ with strict_types
- PSR-4 autoloading
- PHPStan for static analysis
- PHPUnit for testing
- Composer for dependencies

**Structure per example:**
```
php/{service}/
├── src/
│   ├── App.php                # Server setup
│   ├── Handlers/              # Route handlers
│   ├── Models/                # Domain models
│   └── Services/              # Business logic
├── tests/
│   └── HandlersTest.php       # Tests
├── composer.json
├── .php-version
├── phpstan.neon
└── README.md
```

**Key patterns:**
- PSR-4 autoloading
- PSR-12 coding style
- Strict types on all files
- Type hints required
- PHPStan level max
- Data providers for tests

---

### WebAssembly (`examples/wasm/*/`)

**Stack:**
- Rust (WASM runtime)
- wasm-bindgen for FFI
- wasm-pack for bundling
- JavaScript interop

**Structure per example:**
```
wasm/{service}/
├── src/
│   ├── lib.rs                 # WASM entry point
│   ├── handlers.rs            # Message handlers
│   └── [service].rs           # Domain logic
├── tests/
│   └── integration.rs         # Rust tests
├── Cargo.toml
├── wasm-pack.toml
└── README.md
```

**Key patterns:**
- Minimal binary size
- Zero-copy where possible
- Async via promises
- No blocking operations
- Direct JavaScript interop

---

## How to Use These Examples

### 1. Running an Example

**Python:**
```bash
cd examples/python/todo-api
python -m venv venv
source venv/bin/activate
uv pip install -e .
python src/app.py
# Server runs on http://localhost:8000
```

**Node.js:**
```bash
cd examples/node/todo-api
pnpm install
pnpm dev
# Server runs on http://localhost:8000
```

**Ruby:**
```bash
cd examples/ruby/todo-api
rbenv local 3.2.0
bundle install
bundle exec ruby lib/app.rb
# Server runs on http://localhost:8000
```

**PHP:**
```bash
cd examples/php/todo-api
composer install
composer start
# Server runs on http://localhost:8000
```

**WASM:**
```bash
cd examples/wasm/todo-api
wasm-pack build --target nodejs
npm start
```

### 2. Testing an Example

**Python:**
```bash
cd examples/python/todo-api
pytest tests/
```

**Node.js:**
```bash
cd examples/node/todo-api
pnpm test
```

**Ruby:**
```bash
cd examples/ruby/todo-api
bundle exec rspec
```

**PHP:**
```bash
cd examples/php/todo-api
composer test
```

### 3. Generating Code from Schemas

**OpenAPI code generation:**
```bash
# Generate Python server stub
openapi-generator-cli generate \
  -i examples/schemas/todo-api.openapi.yaml \
  -g python-flask \
  -o generated/todo-api/

# Generate TypeScript types
openapi-generator-cli generate \
  -i examples/schemas/todo-api.openapi.yaml \
  -g typescript-axios \
  -o generated/todo-api/
```

**AsyncAPI code generation:**
```bash
# Generate message schemas
asyncapi generate fromTemplate \
  examples/schemas/chat-service.asyncapi.yaml \
  @asyncapi/python-pydantic-schema \
  -o generated/chat-schemas/
```

---

## Features Demonstrated by Example

| Feature | todo-api | file-service | chat-service | events-stream | auth-service |
|---------|----------|--------------|--------------|---------------|--------------|
| **REST Operations** | ✓ | ✓ | | | ✓ |
| **Path Parameters** | ✓ | ✓ | | | ✓ |
| **Query Parameters** | ✓ | ✓ | | | |
| **Request Validation** | ✓ | ✓ | ✓ | ✓ | ✓ |
| **File Upload** | | ✓ | | | |
| **Streaming** | | ✓ | | ✓ | |
| **WebSocket** | | | ✓ | | |
| **SSE** | | | | ✓ | |
| **Authentication** | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Rate Limiting** | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Error Handling** | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Lifecycle Hooks** | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Pagination** | ✓ | ✓ | | | |
| **Filtering** | ✓ | | | ✓ | |
| **Sorting** | ✓ | ✓ | | | |
| **Caching** | ✓ | ✓ | | | |

---

## Testing with Fixtures

All examples are tested against `testing_data/` fixtures:

```bash
# Run all tests locally
task test

# Run specific test suite
task test:python
task test:node
task test:ruby
task test:php

# Validate schemas
task lint
```

**Fixture locations:**
- `testing_data/headers/*.json` - Header validation
- `testing_data/cookies/*.json` - Cookie scenarios
- `testing_data/json_bodies/*.json` - Request body examples
- `testing_data/validation_errors/` - Error response format
- `testing_data/status_codes/` - HTTP status scenarios
- `testing_data/rate_limit/` - Rate limiting behavior

---

## Next Steps

1. **Generate code stubs** from schemas using OpenAPI/AsyncAPI generators
2. **Implement handlers** in each language following the patterns
3. **Write tests** using fixture-driven approach
4. **Build and deploy** using language-specific CI/CD
5. **Document API** using generated Swagger/Redoc

---

## Reference Documentation

- [Schema Suite Guide](./schemas/README.md)
- [OpenAPI 3.1.0 Spec](https://spec.openapis.org/oas/v3.1.0)
- [AsyncAPI 3.0.0 Spec](https://www.asyncapi.com/docs/specifications/latest)
- [RFC 9457 - Problem Details](https://tools.ietf.org/html/rfc9457)
- Spikard ADRs: `docs/adr/`

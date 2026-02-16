# Spikard Schema Suite

Comprehensive OpenAPI 3.1 and AsyncAPI 3.0 schemas demonstrating all of Spikard's capabilities across REST, WebSocket, and Server-Sent Events patterns.

## Schema Overview

This suite includes 5 focused, production-ready schemas that collectively showcase Spikard's features:

### 1. **todo-api.openapi.yaml** - REST CRUD with Validation
Core REST API demonstrating:
- CRUD operations (GET, POST, PUT, DELETE)
- Request/response validation with JSON Schema
- Path parameters (UUID-based IDs)
- Query parameters (filtering, pagination, sorting)
- Request body validation
- Multiple status codes (200, 201, 204, 400, 404, 409, 422, 500)
- RFC 9457 problem details error responses
- Bearer token authentication
- Lifecycle hooks: `onRequest`, `preValidation`, `preHandler`, `onResponse`, `onError`

**Showcased Spikard Features:**
- Header validation (Authorization)
- Request body schema validation
- Response validation
- Error handling with structured payloads
- Path parameter extraction
- Query parameter parsing
- Lifecycle hooks for logging/monitoring

**Example Endpoints:**
- `GET /todos` - List todos with filters
- `POST /todos` - Create new todo
- `GET /todos/{id}` - Get todo by ID
- `PUT /todos/{id}` - Update todo
- DELETE /todos/{id}` - Delete todo

---

### 2. **file-service.openapi.yaml** - File Operations
REST API for file handling demonstrating:
- Multipart form-data uploads
- File size validation
- MIME type restrictions
- Binary response streams (file downloads)
- Progress tracking headers
- Rate limiting headers (X-RateLimit-*)
- Conditional requests (If-Modified-Since, ETag)
- Multiple status codes for upload scenarios (202 Accepted, 413 Payload Too Large)

**Showcased Spikard Features:**
- Multipart/form-data parsing
- Streaming responses
- Large payload handling
- Rate limiting middleware
- Compression (gzip/brotli)
- Conditional headers (caching)
- Request size limits
- File type validation

**Example Endpoints:**
- `POST /files/upload` - Upload file with multipart
- `GET /files/{fileId}` - Download file
- `GET /files/{fileId}/info` - Get file metadata
- `DELETE /files/{fileId}` - Delete file
- `POST /files/{fileId}/verify` - Verify upload integrity

---

### 3. **chat-service.asyncapi.yaml** - WebSocket Bidirectional
Async messaging API demonstrating:
- WebSocket protocol (wss/ws)
- Bidirectional message exchange
- Channel subscription
- User presence (join/leave events)
- Message acknowledgments
- Error messages
- Typing indicators
- Connection state management

**Showcased Spikard Features:**
- WebSocket handler implementation
- Event-driven patterns
- Lifecycle hooks: `onRequest` (connection validation), `onResponse` (disconnect handling)
- Message validation
- Connection timeout handling
- Graceful shutdown
- Multiple message types on single channel

**Example Messages:**
- `chatMessage` - User sends message
- `userJoined` - User joins chat
- `userLeft` - User leaves chat
- `typingIndicator` - User is typing
- `chatAck` - Server acknowledges receipt
- `chatError` - Server error response

---

### 4. **events-stream.asyncapi.yaml** - Server-Sent Events
Async API demonstrating:
- Server-Sent Events (SSE) protocol (https with sse)
- One-way server-to-client streaming
- Event type multiplexing
- Reconnection handling
- Batch events for catch-up
- Heartbeat/keep-alive events
- Event filtering
- Automatic subscription management

**Showcased Spikard Features:**
- Server-Sent Events handler implementation
- Streaming responses
- Connection pooling
- Memory-efficient message queueing
- Reconnection logic
- Heartbeat/keep-alive mechanism
- Lifecycle hooks: `onRequest` (filter validation), `onResponse` (cleanup)

**Example Events:**
- `systemAlert` - Critical system events
- `userNotification` - User-specific notifications
- `statusUpdate` - Service status changes
- `heartbeat` - Keep-alive message
- `notificationBatch` - Catch-up batch for reconnects

---

### 5. **auth-service.openapi.yaml** - Authentication & Security
REST API demonstrating advanced security patterns:
- API key authentication (header, query)
- Bearer token (JWT) authentication
- OAuth 2.0 code flow
- API key rotation
- Token refresh endpoints
- Rate limiting per API key
- Scope-based authorization
- Security scheme composition
- Audit logging patterns

**Showcased Spikard Features:**
- Multiple auth schemes on same endpoint
- Header validation (Authorization)
- Query parameter validation
- Structured error responses for auth failures
- Security scheme composition
- Lifecycle hooks: `preValidation` (auth check), `onResponse` (audit log)
- Rate limiting per authenticated user

**Example Endpoints:**
- `POST /auth/api-keys` - Generate API key
- `GET /auth/api-keys` - List API keys
- `DELETE /auth/api-keys/{keyId}` - Revoke API key
- `POST /auth/token` - Get bearer token
- `POST /auth/refresh` - Refresh token
- `POST /auth/logout` - Revoke token

---

## Key Design Patterns

### Error Handling (RFC 9457)
All schemas use consistent error response format:
```json
{
  "type": "about:blank",
  "title": "Validation Error",
  "status": 422,
  "detail": "Request validation failed",
  "instance": "/todos/invalid-id",
  "errors": [
    {
      "path": "/title",
      "message": "Title is required",
      "code": "REQUIRED"
    }
  ]
}
```

### Authentication Headers
- Bearer tokens: `Authorization: Bearer {token}`
- API keys: `X-API-Key: {key}` or query param `?api_key={key}`
- Session cookies: `Cookie: session_id={value}`

### Pagination
Common query parameters for list endpoints:
- `?page=1&limit=20` - Pagination
- `?sort=created_at&order=desc` - Sorting
- `?filter[status]=active` - Filtering

### Rate Limiting Headers
Response headers:
- `X-RateLimit-Limit: 100`
- `X-RateLimit-Remaining: 45`
- `X-RateLimit-Reset: 1702588800`

### Request Validation
- Path parameters: UUID, numeric IDs, slugs
- Query parameters: typed, constrained with min/max/pattern
- Request bodies: JSON Schema with required fields, constraints
- Headers: Content-Type, Authorization, custom headers

### Response Schemas
- Success: 2xx with appropriate data
- Validation errors: 400/422 with field-level errors
- Auth errors: 401/403 with auth-specific messages
- Not found: 404 with resource type
- Server errors: 500 with request ID for debugging

### Lifecycle Hook Integration
All schemas document where lifecycle hooks apply:
- `onRequest`: Connection validation, feature flags, custom headers
- `preValidation`: Early auth checks, schema selection
- `preHandler`: Data enrichment, dependency injection
- `onResponse`: Audit logging, custom headers, cleanup
- `onError`: Error transformation, logging, cleanup

---

## Schema Features Checklist

### REST (OpenAPI 3.1)

#### Request Features
- [x] Path parameters (typed: UUID, int, string)
- [x] Query parameters (typed, constrained, optional)
- [x] Headers (Authorization, Content-Type, custom)
- [x] Request body (JSON, multipart/form-data)
- [x] Form-encoded data
- [x] File uploads with MIME type constraints
- [x] Request body size limits
- [x] Conditional requests (If-Modified-Since, ETag)

#### Response Features
- [x] Multiple status codes per endpoint
- [x] Different response schemas per status
- [x] Headers (X-RateLimit-*, ETag, Cache-Control)
- [x] Streaming responses (file downloads)
- [x] Binary content (application/octet-stream)

#### Validation
- [x] Required fields
- [x] Type constraints (string, number, boolean)
- [x] String constraints (minLength, maxLength, pattern)
- [x] Numeric constraints (minimum, maximum, exclusiveMinimum)
- [x] Enum values
- [x] Format validation (email, uuid, date-time, uri)
- [x] Complex object validation
- [x] Array validation (minItems, maxItems)
- [x] Discriminator for polymorphic types

#### Security
- [x] Bearer token (Authorization header)
- [x] API key (header and query)
- [x] Multiple auth schemes
- [x] Security scope composition
- [x] Auth on specific endpoints only

#### Error Handling
- [x] RFC 9457 problem details format
- [x] Field-level validation errors
- [x] Custom error codes
- [x] Contextual error details
- [x] Request ID for tracing

#### Middleware Features
- [x] Rate limiting (response headers)
- [x] Compression (gzip, brotli)
- [x] Request ID generation
- [x] Timeout headers
- [x] CORS documentation
- [x] Static files (if applicable)

### Async (AsyncAPI 3.0)

#### WebSocket Features
- [x] Bidirectional messaging
- [x] User presence (join/leave)
- [x] Message acknowledgments
- [x] Multiple message types per channel
- [x] Error handling
- [x] Connection state
- [x] Typing indicators
- [x] Batch operations

#### SSE Features
- [x] Server-to-client streaming
- [x] Multiple event types
- [x] Event filtering
- [x] Reconnection handling
- [x] Batch events for catch-up
- [x] Heartbeat/keep-alive
- [x] Automatic subscription

#### Message Validation
- [x] Required fields
- [x] Type constraints
- [x] Enum values
- [x] Format validation
- [x] Payload examples

#### Lifecycle Integration
- [x] Connection validation (onRequest)
- [x] Disconnect cleanup (onResponse)
- [x] Message validation (preValidation)
- [x] Message transformation (preHandler)
- [x] Error handling (onError)

---

## Generation Commands

These schemas are designed for code generation using tools like:

```bash
# OpenAPI code generation (client/server)
openapi-generator-cli generate -i examples/schemas/todo-api.openapi.yaml -g python-flask -o generated/python/
openapi-generator-cli generate -i examples/schemas/file-service.openapi.yaml -g nodejs-express -o generated/node/

# AsyncAPI code generation (handler templates)
asyncapi generate fromTemplate examples/schemas/chat-service.asyncapi.yaml @asyncapi/python-pydantic-schema -o generated/schemas/chat/

# Spikard-specific code generation
spikard-codegen --spec examples/schemas/todo-api.openapi.yaml --lang python --output examples/python/todo-api/
spikard-codegen --spec examples/schemas/chat-service.asyncapi.yaml --lang python --output examples/python/chat-service/
```

---

## Integration with Spikard

Each schema serves as:

1. **Specification Reference**: Clear contract for API behavior
2. **Validation Template**: Defines schemas used by Spikard validators
3. **Code Generation Input**: Bootstrap for handler implementations
4. **Documentation**: API reference for developers
5. **Test Fixture Reference**: Validates against testing_data JSON files

The schemas align with:
- `testing_data/headers/*.json` - Request header validation
- `testing_data/cookies/*.json` - Cookie validation
- `testing_data/json_bodies/*.json` - Request body validation
- `testing_data/validation_errors/schema.json` - Error response format
- `testing_data/status_codes/*.json` - HTTP status code scenarios
- `testing_data/rate_limit/*.json` - Rate limiting behavior
- `testing_data/cors/*.json` - CORS scenarios

---

## Next Steps

1. **Code Generation**: Use these schemas to generate handler stubs
2. **Fixture Creation**: Create JSON test fixtures from schema examples
3. **Implementation**: Implement handlers in Python, Node, Ruby, PHP, Elixir
4. **Testing**: Validate implementations against fixture-driven tests
5. **Documentation**: Generate API docs from schemas with Swagger UI / Redoc

---

## Schema Versions

- **OpenAPI**: 3.1.0 (JSON Schema support, no YAML refs)
- **AsyncAPI**: 3.0.0 (latest, better streaming support)
- **JSON Schema**: 2020-12 (embedded in AsyncAPI 3.0)

---

## References

- [OpenAPI 3.1.0 Specification](https://spec.openapis.org/oas/v3.1.0)
- [AsyncAPI 3.0.0 Specification](https://www.asyncapi.com/en/docs/specifications/latest)
- [RFC 9457 - Problem Details](https://www.rfc-editor.org/rfc/rfc9457.html)
- [File Uploads Best Practices](https://www.speakeasy.com/openapi/content/file-uploads)
- [AsyncAPI WebSocket Guide](https://www.asyncapi.com/blog/websocket-part1)
- [AsyncAPI Pub/Sub Semantics](https://www.asyncapi.com/blog/publish-subscribe-semantics)

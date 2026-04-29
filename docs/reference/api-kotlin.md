---
title: "Kotlin API Reference"
---

## Kotlin API Reference <span class="version-badge">v0.14.0</span>

### Functions

#### schemaQueryOnly()

Create a simple schema configuration with only Query type.

This is a convenience function for schemas that only have queries.

**Returns:**

A `QueryOnlyConfig` with default settings

**Signature:**

```kotlin
// Phase 1: kotlin backend signature generation
```

**Returns:** `QueryOnlyConfig`

---

#### schemaQueryMutation()

Create a schema configuration with Query and Mutation types.

This is a convenience function for schemas with queries and mutations but no subscriptions.

**Returns:**

A `QueryMutationConfig` with default settings

**Signature:**

```kotlin
// Phase 1: kotlin backend signature generation
```

**Returns:** `QueryMutationConfig`

---

#### schemaFull()

Create a schema configuration with all three root types.

This is a convenience function for fully-featured schemas.

**Returns:**

A `FullSchemaConfig` with default settings

**Signature:**

```kotlin
// Phase 1: kotlin backend signature generation
```

**Returns:** `FullSchemaConfig`

---

### Types

#### ApiKeyConfig

API Key authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `keys` | `List<String>` | — | Valid API keys |
| `headerName` | `String` | — | Header name to check (e.g., "X-API-Key") |

---

#### BackgroundJobMetadata

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `requestId` | `String?` | `null` | Request id |

##### Methods

###### default()

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

---

#### BackgroundTaskConfig

Configuration for in-process background task execution.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `maxQueueSize` | `Long` | `1024` | Maximum queue size |
| `maxConcurrentTasks` | `Long` | `128` | Maximum concurrent tasks |
| `drainTimeoutSecs` | `Long` | `30` | Drain timeout secs |

##### Methods

###### default()

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

---

#### CompressionConfig

Compression configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `gzip` | `Boolean` | `true` | Enable gzip compression |
| `brotli` | `Boolean` | `true` | Enable brotli compression |
| `minSize` | `Long` | — | Minimum response size to compress (bytes) |
| `quality` | `Int` | — | Compression quality (0-11 for brotli, 0-9 for gzip) |

##### Methods

###### default()

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

---

#### ContactInfo

Contact information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String?` | `null` | The name |
| `email` | `String?` | `null` | Email |
| `url` | `String?` | `null` | Url |

---

#### CorsConfig

CORS configuration for a route

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `allowedOrigins` | `List<String>` | `[]` | Allowed origins |
| `allowedMethods` | `List<String>` | `[]` | Allowed methods |
| `allowedHeaders` | `List<String>` | `[]` | Allowed headers |
| `exposeHeaders` | `List<String>?` | `null` | Expose headers |
| `maxAge` | `Int?` | `null` | Maximum age |
| `allowCredentials` | `Boolean?` | `null` | Allow credentials |
| `methodsJoinedCache` | `String` | — | Methods joined cache |
| `headersJoinedCache` | `String` | — | Headers joined cache |

##### Methods

###### allowedMethodsJoined()

Get the cached joined methods string for preflight responses

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

###### allowedHeadersJoined()

Get the cached joined headers string for preflight responses

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

###### isOriginAllowed()

Check if an origin is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

###### isMethodAllowed()

Check if a method is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

###### areHeadersAllowed()

Check if all requested headers are allowed (O(n) where n = num requested headers)

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

###### default()

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

---

#### FullSchemaConfig

Configuration for fully-featured schemas with Query, Mutation, and Subscription types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `Boolean` | `true` | Enable introspection queries |
| `complexityLimit` | `Long?` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `Long?` | `null` | Maximum query depth (None = unlimited) |

##### Methods

###### default()

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

---

#### GraphQlError

##### Methods

###### statusCode()

Convert error to HTTP status code

Maps GraphQL error types to appropriate HTTP status codes:

- 400: Bad Request for parse/request-handling errors
- 401: Unauthorized for authentication errors
- 403: Forbidden for authorization errors
- 404: Not Found for resource not found
- 422: Unprocessable Entity for validation failures
- 429: Too Many Requests for rate limit errors
- 500: Internal Server Error for schema/serialization/internal errors
- 200: OK for GraphQL execution errors returned in GraphQL response body

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

---

#### GraphQlRouteConfig

Configuration for GraphQL routes

Provides a builder pattern for configuring GraphQL route parameters
for the Spikard HTTP server's routing system.

##### Methods

###### path()

Set the HTTP path for the GraphQL endpoint

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

###### method()

Set the HTTP method for the GraphQL endpoint

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

###### enablePlayground()

Enable or disable the GraphQL Playground UI

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

###### description()

Set a custom description for documentation

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

###### getPath()

Get the configured path

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

###### getMethod()

Get the configured method

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

###### isPlaygroundEnabled()

Check if playground is enabled

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

###### getDescription()

Get the description if set

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

###### default()

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

---

#### GrpcConfig

Configuration for gRPC support

Controls how the server handles gRPC requests, including compression,
timeouts, and protocol settings.

## Stream Limits

This configuration enforces message-level size limits but delegates
concurrent stream limiting to the HTTP/2 transport layer:

- **Message Size Limits**: The `max_message_size` field is enforced per
  individual message (request or response) in both unary and streaming RPCs.
  When a single message exceeds this limit, the request is rejected with
  `PAYLOAD_TOO_LARGE` (HTTP 413).

- **Concurrent Stream Limits**: The `max_concurrent_streams` is an advisory
  configuration passed to the HTTP/2 layer for connection-level stream
  negotiation. The HTTP/2 transport automatically enforces this limit and
  returns GOAWAY frames when exceeded. Applications should not rely on
  custom enforcement of this limit.

- **Stream Length Limits**: There is currently no built-in limit on the
  total number of messages in a stream. Handlers should implement their own
  message counting if needed. Future versions may add a `max_stream_response_bytes`
  field to limit total response size per stream.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `Boolean` | `true` | Enable gRPC support |
| `maxMessageSize` | `Long` | — | Maximum message size in bytes (for both sending and receiving) This limit applies to individual messages in both unary and streaming RPCs. When a single message exceeds this size, the request is rejected with HTTP 413 (Payload Too Large). Default: 4MB (4194304 bytes) **Note:** This limit does NOT apply to the total response size in streaming RPCs. For multi-message streams, the total response can exceed this limit as long as each individual message stays within the limit. |
| `enableCompression` | `Boolean` | `true` | Enable gzip compression for gRPC messages |
| `requestTimeout` | `Long?` | `null` | Timeout for gRPC requests in seconds (None = no timeout) |
| `maxConcurrentStreams` | `Int` | — | Maximum number of concurrent streams per connection (HTTP/2 advisory) This value is communicated to HTTP/2 clients as the server's flow control limit. The HTTP/2 transport layer enforces this limit automatically via SETTINGS frames and GOAWAY responses. Applications should NOT implement custom enforcement. Default: 100 streams per connection # Stream Limiting Strategy - **Per Connection**: This limit applies per HTTP/2 connection, not globally - **Transport Enforcement**: HTTP/2 handles all stream limiting; applications need not implement custom checks - **Streaming Requests**: In server streaming or bidi streaming, each logical RPC consumes one stream slot. Message ordering within a stream follows HTTP/2 frame ordering. # Future Enhancement A future `max_stream_response_bytes` field may be added to limit the total response size in streaming RPCs (separate from per-message limits). |
| `enableKeepalive` | `Boolean` | `true` | Enable HTTP/2 keepalive |
| `keepaliveInterval` | `Long` | — | HTTP/2 keepalive interval in seconds |
| `keepaliveTimeout` | `Long` | — | HTTP/2 keepalive timeout in seconds |

### Methods

#### default()

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

---

##### JsonRpcConfig

JSON-RPC server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `Boolean` | `true` | Enable JSON-RPC endpoint |
| `endpointPath` | `String` | — | HTTP endpoint path for JSON-RPC requests (default: "/rpc") |
| `enableBatch` | `Boolean` | — | Enable batch request processing (default: true) |
| `maxBatchSize` | `Long` | — | Maximum number of requests in a batch (default: 100) |

###### Methods

###### default()

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

---

##### JsonRpcMethodInfo

JSON-RPC method metadata for routes that support JSON-RPC

This struct captures the metadata needed to expose HTTP routes as JSON-RPC methods,
enabling discovery and documentation of RPC-compatible endpoints.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `methodName` | `String` | — | The JSON-RPC method name (e.g., "user.create") |
| `description` | `String?` | `null` | Optional description of what the method does |
| `paramsSchema` | `String?` | `null` | Optional JSON Schema for method parameters |
| `resultSchema` | `String?` | `null` | Optional JSON Schema for the result |
| `deprecated` | `Boolean` | — | Whether this method is deprecated |
| `tags` | `List<String>` | — | Tags for categorizing and grouping methods |

---

##### JwtConfig

JWT authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `secret` | `String` | — | Secret key for JWT verification |
| `algorithm` | `String` | — | Required algorithm (HS256, HS384, HS512, RS256, etc.) |
| `audience` | `List<String>?` | `null` | Required audience claim |
| `issuer` | `String?` | `null` | Required issuer claim |
| `leeway` | `Long` | — | Leeway for expiration checks (seconds) |

---

##### LicenseInfo

License information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `url` | `String?` | `null` | Url |

---

##### OpenApiConfig

OpenAPI configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `Boolean` | `false` | Enable OpenAPI generation (default: false for zero overhead) |
| `title` | `String` | `"API"` | API title |
| `version` | `String` | `"1.0.0"` | API version |
| `description` | `String?` | `null` | API description (supports markdown) |
| `swaggerUiPath` | `String` | — | Path to serve Swagger UI (default: "/docs") |
| `redocPath` | `String` | — | Path to serve Redoc (default: "/redoc") |
| `openapiJsonPath` | `String` | — | Path to serve OpenAPI JSON spec (default: "/openapi.json") |
| `contact` | `ContactInfo?` | `null` | Contact information |
| `license` | `LicenseInfo?` | `null` | License information |
| `servers` | `List<ServerInfo>` | `[]` | Server definitions |
| `securitySchemes` | `Map<String, SecuritySchemeInfo>` | `{}` | Security schemes (auto-detected from middleware if not provided) |

###### Methods

###### default()

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

---

##### ProblemDetails

RFC 9457 Problem Details for HTTP APIs

A machine-readable format for specifying errors in HTTP API responses.
Per RFC 9457, all fields are optional. The `type` field defaults to "about:blank"
if not specified.

## Content-Type

Responses using this struct should set:

```text
Content-Type: application/problem+json
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `typeUri` | `String` | — | A URI reference that identifies the problem type. Defaults to "about:blank" when absent. Should be a stable, human-readable identifier for the problem type. |
| `title` | `String` | — | A short, human-readable summary of the problem type. Should not change from occurrence to occurrence of the problem. |
| `status` | `Short` | — | The HTTP status code generated by the origin server. This is advisory; the actual HTTP status code takes precedence. |
| `detail` | `String?` | `null` | A human-readable explanation specific to this occurrence of the problem. |
| `instance` | `String?` | `null` | A URI reference that identifies the specific occurrence of the problem. It may or may not yield further information if dereferenced. |
| `extensions` | `Map<String, String>` | — | Extension members - problem-type-specific data. For validation errors, this typically contains an "errors" array. |

### Methods

#### withDetail()

Set the detail field

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

##### withInstance()

Set the instance field

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

###### notFound()

Create a not found error

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

###### methodNotAllowed()

Create a method not allowed error

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

###### internalServerError()

Create an internal server error

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

###### badRequest()

Create a bad request error

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

###### toJson()

Serialize to JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

###### toJsonPretty()

Serialize to pretty JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

---

##### QueryMutationConfig

Configuration for schemas with Query and Mutation types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `Boolean` | `true` | Enable introspection queries |
| `complexityLimit` | `Long?` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `Long?` | `null` | Maximum query depth (None = unlimited) |

###### Methods

###### default()

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

---

##### QueryOnlyConfig

Configuration for schemas with only Query type

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `Boolean` | `true` | Enable introspection queries |
| `complexityLimit` | `Long?` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `Long?` | `null` | Maximum query depth (None = unlimited) |

###### Methods

###### default()

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

---

##### RateLimitConfig

Rate limiting configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `perSecond` | `Long` | `100` | Requests per second |
| `burst` | `Int` | `200` | Burst allowance |
| `ipBased` | `Boolean` | `true` | Use IP-based rate limiting |

###### Methods

###### default()

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

---

##### Response

HTTP Response with custom status code, headers, and content

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String?` | `null` | Response body content |
| `statusCode` | `Short` | — | HTTP status code (defaults to 200) |
| `headers` | `Map<String, String>` | `{}` | Response headers |

###### Methods

###### setHeader()

Set a header

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

###### setCookie()

Set a cookie in the response

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

###### default()

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

---

##### SchemaConfig

Configuration for GraphQL schema building.

Encapsulates all schema-level configuration options including
introspection control, complexity limits, and depth limits.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `Boolean` | `true` | Enable introspection queries |
| `complexityLimit` | `Long?` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `Long?` | `null` | Maximum query depth (None = unlimited) |

###### Methods

###### default()

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

---

##### ServerConfig

Server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `host` | `String` | `"127.0.0.1"` | Host to bind to |
| `port` | `Short` | `8000` | Port to bind to |
| `workers` | `Long` | `1` | Number of Tokio runtime worker threads used by binding-managed server runtimes |
| `enableRequestId` | `Boolean` | `false` | Enable request ID generation and propagation |
| `maxBodySize` | `Long?` | `null` | Maximum request body size in bytes (None = unlimited, not recommended) |
| `requestTimeout` | `Long?` | `null` | Request timeout in seconds (None = no timeout) |
| `compression` | `CompressionConfig?` | `null` | Enable compression middleware |
| `rateLimit` | `RateLimitConfig?` | `null` | Enable rate limiting |
| `jwtAuth` | `JwtConfig?` | `null` | JWT authentication configuration |
| `apiKeyAuth` | `ApiKeyConfig?` | `null` | API Key authentication configuration |
| `staticFiles` | `List<StaticFilesConfig>` | `[]` | Static file serving configuration |
| `gracefulShutdown` | `Boolean` | `true` | Enable graceful shutdown on SIGTERM/SIGINT |
| `shutdownTimeout` | `Long` | `30` | Graceful shutdown timeout (seconds) |
| `openapi` | `OpenApiConfig?` | `null` | OpenAPI documentation configuration |
| `jsonrpc` | `JsonRpcConfig?` | `null` | JSON-RPC configuration |
| `grpc` | `GrpcConfig?` | `null` | gRPC configuration |
| `lifecycleHooks` | `String?` | `null` | Lifecycle hooks for request/response processing |
| `backgroundTasks` | `BackgroundTaskConfig` | — | Background task executor configuration |
| `enableHttpTrace` | `Boolean` | `false` | Enable per-request HTTP tracing (tower-http `TraceLayer`) |
| `diContainer` | `String?` | `null` | Dependency injection container (requires 'di' feature) |

###### Methods

###### default()

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

---

##### ServerInfo

Server information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | Url |
| `description` | `String?` | `null` | Human-readable description |

---

##### SseEvent

An individual SSE event

Represents a single Server-Sent Event to be sent to a connected client.
Events can have an optional type, ID, and retry timeout for advanced scenarios.

## SSE Format

Events are serialized to the following text format:

```text
event: event_type
data: {"json":"value"}
id: event-123
retry: 3000
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `eventType` | `String?` | `null` | Event type (optional) |
| `data` | `String` | — | Event data (JSON value) |
| `id` | `String?` | `null` | Event ID (optional, for client-side reconnection) |
| `retry` | `Long?` | `null` | Retry timeout in milliseconds (optional) |

### Methods

#### withId()

Set the event ID for client-side reconnection support

Sets an ID that clients can use to resume from this point if they disconnect.
The client sends this ID back in the `Last-Event-ID` header when reconnecting.

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

##### withRetry()

Set the retry timeout for client reconnection

Sets the time in milliseconds clients should wait before attempting to reconnect
if the connection is lost. The client browser will automatically handle reconnection.

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

---

##### StaticFilesConfig

Static file serving configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `directory` | `String` | — | Directory path to serve |
| `routePrefix` | `String` | — | URL path prefix (e.g., "/static") |
| `indexFile` | `Boolean` | — | Fallback to index.html for directories |
| `cacheControl` | `String?` | `null` | Cache-Control header value |

---

##### UploadFile

Represents an uploaded file from multipart/form-data requests.

This struct provides efficient access to file content with automatic
base64 decoding and implements standard I/O traits for compatibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `filename` | `String` | — | Original filename from the client |
| `contentType` | `String?` | `null` | MIME type of the uploaded file |
| `size` | `Long?` | `null` | Size of the file in bytes |
| `content` | `ByteArray` | — | File content (may be base64 encoded) |
| `contentEncoding` | `String?` | `null` | Content encoding type |
| `cursor` | `String` | — | Internal cursor for Read/Seek operations |

###### Methods

###### asBytes()

Get the raw file content as bytes.

This provides zero-copy access to the underlying buffer.

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

###### readToString()

Read the file content as a UTF-8 string.

**Errors:**

Returns an error if the content is not valid UTF-8.

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

###### contentTypeOrDefault()

Get the content type, defaulting to "application/octet-stream".

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

---

#### Enums

##### Method

HTTP method

| Value | Description |
|-------|-------------|
| `Get` | Get |
| `Post` | Post |
| `Put` | Put |
| `Patch` | Patch |
| `Delete` | Delete |
| `Head` | Head |
| `Options` | Options |
| `Trace` | Trace |

---

##### SecuritySchemeInfo

Security scheme types

| Value | Description |
|-------|-------------|
| `Http` | Http — Fields: `scheme`: `String`, `bearerFormat`: `String` |
| `ApiKey` | Api key — Fields: `location`: `String`, `name`: `String` |

---

#### Errors

##### GraphQlError

Errors that can occur during GraphQL operations

These errors are compatible with async-graphql error handling and can be
converted to structured HTTP responses matching the project's error fixtures.

| Variant | Description |
|---------|-------------|
| `ExecutionError` | Error during schema execution Occurs when the GraphQL executor encounters a runtime error during query execution. |
| `SchemaBuildError` | Error during schema building Occurs when schema construction fails due to invalid definitions or conflicts. |
| `RequestHandlingError` | Error during request handling Occurs when the HTTP request cannot be properly handled or parsed. |
| `SerializationError` | Serialization error Occurs during JSON serialization/deserialization of GraphQL values. |
| `JsonError` | JSON parsing error Occurs when JSON input cannot be parsed. |
| `ValidationError` | GraphQL validation error Occurs when a GraphQL query fails schema validation. |
| `ParseError` | GraphQL parse error Occurs when the GraphQL query string cannot be parsed. |
| `AuthenticationError` | Authentication error Occurs when request authentication fails. |
| `AuthorizationError` | Authorization error Occurs when user lacks required permissions. |
| `NotFound` | Not found error Occurs when a requested resource is not found. |
| `RateLimitExceeded` | Rate limit error Occurs when rate limit is exceeded. |
| `InvalidInput` | Invalid input error with validation details Occurs during input validation with detailed error information. |
| `ComplexityLimitExceeded` | Query complexity limit exceeded Occurs when a GraphQL query exceeds the configured complexity limit. |
| `DepthLimitExceeded` | Query depth limit exceeded Occurs when a GraphQL query exceeds the configured depth limit. |
| `InternalError` | Internal server error Occurs when an unexpected internal error happens. |

---

##### SchemaError

Error type for schema building operations

| Variant | Description |
|---------|-------------|
| `BuildingFailed` | Generic schema building error |
| `ValidationError` | Configuration validation error |
| `ComplexityLimitExceeded` | Complexity limit exceeded |
| `DepthLimitExceeded` | Depth limit exceeded |

---

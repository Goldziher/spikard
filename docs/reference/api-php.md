---
title: "PHP API Reference"
---

## PHP API Reference <span class="version-badge">v0.14.0</span>

### Functions

#### schemaQueryOnly()

Create a simple schema configuration with only Query type.

This is a convenience function for schemas that only have queries.

**Returns:**

A `QueryOnlyConfig` with default settings

**Signature:**

```php
public static function schemaQueryOnly(): QueryOnlyConfig
```

**Returns:** `QueryOnlyConfig`

---

#### schemaQueryMutation()

Create a schema configuration with Query and Mutation types.

This is a convenience function for schemas with queries and mutations but no subscriptions.

**Returns:**

A `QueryMutationConfig` with default settings

**Signature:**

```php
public static function schemaQueryMutation(): QueryMutationConfig
```

**Returns:** `QueryMutationConfig`

---

#### schemaFull()

Create a schema configuration with all three root types.

This is a convenience function for fully-featured schemas.

**Returns:**

A `FullSchemaConfig` with default settings

**Signature:**

```php
public static function schemaFull(): FullSchemaConfig
```

**Returns:** `FullSchemaConfig`

---

### Types

#### ApiKeyConfig

API Key authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `keys` | `array<string>` | — | Valid API keys |
| `headerName` | `string` | — | Header name to check (e.g., "X-API-Key") |

---

#### BackgroundJobMetadata

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | The name |
| `requestId` | `?string` | `null` | Request id |

##### Methods

###### default()

**Signature:**

```php
public static function default(): BackgroundJobMetadata
```

---

#### BackgroundTaskConfig

Configuration for in-process background task execution.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `maxQueueSize` | `int` | `1024` | Maximum queue size |
| `maxConcurrentTasks` | `int` | `128` | Maximum concurrent tasks |
| `drainTimeoutSecs` | `int` | `30` | Drain timeout secs |

##### Methods

###### default()

**Signature:**

```php
public static function default(): BackgroundTaskConfig
```

---

#### CompressionConfig

Compression configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `gzip` | `bool` | `true` | Enable gzip compression |
| `brotli` | `bool` | `true` | Enable brotli compression |
| `minSize` | `int` | — | Minimum response size to compress (bytes) |
| `quality` | `int` | — | Compression quality (0-11 for brotli, 0-9 for gzip) |

##### Methods

###### default()

**Signature:**

```php
public static function default(): CompressionConfig
```

---

#### ContactInfo

Contact information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `?string` | `null` | The name |
| `email` | `?string` | `null` | Email |
| `url` | `?string` | `null` | Url |

---

#### CorsConfig

CORS configuration for a route

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `allowedOrigins` | `array<string>` | `[]` | Allowed origins |
| `allowedMethods` | `array<string>` | `[]` | Allowed methods |
| `allowedHeaders` | `array<string>` | `[]` | Allowed headers |
| `exposeHeaders` | `?array<string>` | `null` | Expose headers |
| `maxAge` | `?int` | `null` | Maximum age |
| `allowCredentials` | `?bool` | `null` | Allow credentials |
| `methodsJoinedCache` | `string` | — | Methods joined cache |
| `headersJoinedCache` | `string` | — | Headers joined cache |

##### Methods

###### allowedMethodsJoined()

Get the cached joined methods string for preflight responses

**Signature:**

```php
public function allowedMethodsJoined(): string
```

###### allowedHeadersJoined()

Get the cached joined headers string for preflight responses

**Signature:**

```php
public function allowedHeadersJoined(): string
```

###### isOriginAllowed()

Check if an origin is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```php
public function isOriginAllowed(string $origin): bool
```

###### isMethodAllowed()

Check if a method is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```php
public function isMethodAllowed(string $method): bool
```

###### areHeadersAllowed()

Check if all requested headers are allowed (O(n) where n = num requested headers)

**Signature:**

```php
public function areHeadersAllowed(array<string> $requested): bool
```

###### default()

**Signature:**

```php
public static function default(): CorsConfig
```

---

#### FullSchemaConfig

Configuration for fully-featured schemas with Query, Mutation, and Subscription types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `bool` | `true` | Enable introspection queries |
| `complexityLimit` | `?int` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `?int` | `null` | Maximum query depth (None = unlimited) |

##### Methods

###### default()

**Signature:**

```php
public static function default(): FullSchemaConfig
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

```php
public function statusCode(): int
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

```php
public function path(string $path): GraphQlRouteConfig
```

###### method()

Set the HTTP method for the GraphQL endpoint

**Signature:**

```php
public function method(string $method): GraphQlRouteConfig
```

###### enablePlayground()

Enable or disable the GraphQL Playground UI

**Signature:**

```php
public function enablePlayground(bool $enable): GraphQlRouteConfig
```

###### description()

Set a custom description for documentation

**Signature:**

```php
public function description(string $description): GraphQlRouteConfig
```

###### getPath()

Get the configured path

**Signature:**

```php
public function getPath(): string
```

###### getMethod()

Get the configured method

**Signature:**

```php
public function getMethod(): string
```

###### isPlaygroundEnabled()

Check if playground is enabled

**Signature:**

```php
public function isPlaygroundEnabled(): bool
```

###### getDescription()

Get the description if set

**Signature:**

```php
public function getDescription(): ?string
```

###### default()

**Signature:**

```php
public static function default(): GraphQlRouteConfig
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
| `enabled` | `bool` | `true` | Enable gRPC support |
| `maxMessageSize` | `int` | — | Maximum message size in bytes (for both sending and receiving) This limit applies to individual messages in both unary and streaming RPCs. When a single message exceeds this size, the request is rejected with HTTP 413 (Payload Too Large). Default: 4MB (4194304 bytes) **Note:** This limit does NOT apply to the total response size in streaming RPCs. For multi-message streams, the total response can exceed this limit as long as each individual message stays within the limit. |
| `enableCompression` | `bool` | `true` | Enable gzip compression for gRPC messages |
| `requestTimeout` | `?int` | `null` | Timeout for gRPC requests in seconds (None = no timeout) |
| `maxConcurrentStreams` | `int` | — | Maximum number of concurrent streams per connection (HTTP/2 advisory) This value is communicated to HTTP/2 clients as the server's flow control limit. The HTTP/2 transport layer enforces this limit automatically via SETTINGS frames and GOAWAY responses. Applications should NOT implement custom enforcement. Default: 100 streams per connection # Stream Limiting Strategy - **Per Connection**: This limit applies per HTTP/2 connection, not globally - **Transport Enforcement**: HTTP/2 handles all stream limiting; applications need not implement custom checks - **Streaming Requests**: In server streaming or bidi streaming, each logical RPC consumes one stream slot. Message ordering within a stream follows HTTP/2 frame ordering. # Future Enhancement A future `max_stream_response_bytes` field may be added to limit the total response size in streaming RPCs (separate from per-message limits). |
| `enableKeepalive` | `bool` | `true` | Enable HTTP/2 keepalive |
| `keepaliveInterval` | `int` | — | HTTP/2 keepalive interval in seconds |
| `keepaliveTimeout` | `int` | — | HTTP/2 keepalive timeout in seconds |

### Methods

#### default()

**Signature:**

```php
public static function default(): GrpcConfig
```

---

##### JsonRpcConfig

JSON-RPC server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `true` | Enable JSON-RPC endpoint |
| `endpointPath` | `string` | — | HTTP endpoint path for JSON-RPC requests (default: "/rpc") |
| `enableBatch` | `bool` | — | Enable batch request processing (default: true) |
| `maxBatchSize` | `int` | — | Maximum number of requests in a batch (default: 100) |

###### Methods

###### default()

**Signature:**

```php
public static function default(): JsonRpcConfig
```

---

##### JsonRpcMethodInfo

JSON-RPC method metadata for routes that support JSON-RPC

This struct captures the metadata needed to expose HTTP routes as JSON-RPC methods,
enabling discovery and documentation of RPC-compatible endpoints.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `methodName` | `string` | — | The JSON-RPC method name (e.g., "user.create") |
| `description` | `?string` | `null` | Optional description of what the method does |
| `paramsSchema` | `?string` | `null` | Optional JSON Schema for method parameters |
| `resultSchema` | `?string` | `null` | Optional JSON Schema for the result |
| `deprecated` | `bool` | — | Whether this method is deprecated |
| `tags` | `array<string>` | — | Tags for categorizing and grouping methods |

---

##### JwtConfig

JWT authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `secret` | `string` | — | Secret key for JWT verification |
| `algorithm` | `string` | — | Required algorithm (HS256, HS384, HS512, RS256, etc.) |
| `audience` | `?array<string>` | `null` | Required audience claim |
| `issuer` | `?string` | `null` | Required issuer claim |
| `leeway` | `int` | — | Leeway for expiration checks (seconds) |

---

##### LicenseInfo

License information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | The name |
| `url` | `?string` | `null` | Url |

---

##### OpenApiConfig

OpenAPI configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `false` | Enable OpenAPI generation (default: false for zero overhead) |
| `title` | `string` | `"API"` | API title |
| `version` | `string` | `"1.0.0"` | API version |
| `description` | `?string` | `null` | API description (supports markdown) |
| `swaggerUiPath` | `string` | — | Path to serve Swagger UI (default: "/docs") |
| `redocPath` | `string` | — | Path to serve Redoc (default: "/redoc") |
| `openapiJsonPath` | `string` | — | Path to serve OpenAPI JSON spec (default: "/openapi.json") |
| `contact` | `?ContactInfo` | `null` | Contact information |
| `license` | `?LicenseInfo` | `null` | License information |
| `servers` | `array<ServerInfo>` | `[]` | Server definitions |
| `securitySchemes` | `array<string, SecuritySchemeInfo>` | `{}` | Security schemes (auto-detected from middleware if not provided) |

###### Methods

###### default()

**Signature:**

```php
public static function default(): OpenApiConfig
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
| `typeUri` | `string` | — | A URI reference that identifies the problem type. Defaults to "about:blank" when absent. Should be a stable, human-readable identifier for the problem type. |
| `title` | `string` | — | A short, human-readable summary of the problem type. Should not change from occurrence to occurrence of the problem. |
| `status` | `int` | — | The HTTP status code generated by the origin server. This is advisory; the actual HTTP status code takes precedence. |
| `detail` | `?string` | `null` | A human-readable explanation specific to this occurrence of the problem. |
| `instance` | `?string` | `null` | A URI reference that identifies the specific occurrence of the problem. It may or may not yield further information if dereferenced. |
| `extensions` | `array<string, string>` | — | Extension members - problem-type-specific data. For validation errors, this typically contains an "errors" array. |

### Methods

#### withDetail()

Set the detail field

**Signature:**

```php
public function withDetail(string $detail): ProblemDetails
```

##### withInstance()

Set the instance field

**Signature:**

```php
public function withInstance(string $instance): ProblemDetails
```

###### notFound()

Create a not found error

**Signature:**

```php
public static function notFound(string $detail): ProblemDetails
```

###### methodNotAllowed()

Create a method not allowed error

**Signature:**

```php
public static function methodNotAllowed(string $detail): ProblemDetails
```

###### internalServerError()

Create an internal server error

**Signature:**

```php
public static function internalServerError(string $detail): ProblemDetails
```

###### badRequest()

Create a bad request error

**Signature:**

```php
public static function badRequest(string $detail): ProblemDetails
```

###### toJson()

Serialize to JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```php
public function toJson(): string
```

###### toJsonPretty()

Serialize to pretty JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```php
public function toJsonPretty(): string
```

---

##### QueryMutationConfig

Configuration for schemas with Query and Mutation types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `bool` | `true` | Enable introspection queries |
| `complexityLimit` | `?int` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `?int` | `null` | Maximum query depth (None = unlimited) |

###### Methods

###### default()

**Signature:**

```php
public static function default(): QueryMutationConfig
```

---

##### QueryOnlyConfig

Configuration for schemas with only Query type

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `bool` | `true` | Enable introspection queries |
| `complexityLimit` | `?int` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `?int` | `null` | Maximum query depth (None = unlimited) |

###### Methods

###### default()

**Signature:**

```php
public static function default(): QueryOnlyConfig
```

---

##### RateLimitConfig

Rate limiting configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `perSecond` | `int` | `100` | Requests per second |
| `burst` | `int` | `200` | Burst allowance |
| `ipBased` | `bool` | `true` | Use IP-based rate limiting |

###### Methods

###### default()

**Signature:**

```php
public static function default(): RateLimitConfig
```

---

##### Response

HTTP Response with custom status code, headers, and content

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `?string` | `null` | Response body content |
| `statusCode` | `int` | — | HTTP status code (defaults to 200) |
| `headers` | `array<string, string>` | `{}` | Response headers |

###### Methods

###### setHeader()

Set a header

**Signature:**

```php
public function setHeader(string $key, string $value): void
```

###### setCookie()

Set a cookie in the response

**Signature:**

```php
public function setCookie(string $key, string $value, int $maxAge, string $domain, string $path, bool $secure, bool $httpOnly, string $sameSite): void
```

###### default()

**Signature:**

```php
public static function default(): Response
```

---

##### SchemaConfig

Configuration for GraphQL schema building.

Encapsulates all schema-level configuration options including
introspection control, complexity limits, and depth limits.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `bool` | `true` | Enable introspection queries |
| `complexityLimit` | `?int` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `?int` | `null` | Maximum query depth (None = unlimited) |

###### Methods

###### default()

**Signature:**

```php
public static function default(): SchemaConfig
```

---

##### ServerConfig

Server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `host` | `string` | `"127.0.0.1"` | Host to bind to |
| `port` | `int` | `8000` | Port to bind to |
| `workers` | `int` | `1` | Number of Tokio runtime worker threads used by binding-managed server runtimes |
| `enableRequestId` | `bool` | `false` | Enable request ID generation and propagation |
| `maxBodySize` | `?int` | `null` | Maximum request body size in bytes (None = unlimited, not recommended) |
| `requestTimeout` | `?int` | `null` | Request timeout in seconds (None = no timeout) |
| `compression` | `?CompressionConfig` | `null` | Enable compression middleware |
| `rateLimit` | `?RateLimitConfig` | `null` | Enable rate limiting |
| `jwtAuth` | `?JwtConfig` | `null` | JWT authentication configuration |
| `apiKeyAuth` | `?ApiKeyConfig` | `null` | API Key authentication configuration |
| `staticFiles` | `array<StaticFilesConfig>` | `[]` | Static file serving configuration |
| `gracefulShutdown` | `bool` | `true` | Enable graceful shutdown on SIGTERM/SIGINT |
| `shutdownTimeout` | `int` | `30` | Graceful shutdown timeout (seconds) |
| `openapi` | `?OpenApiConfig` | `null` | OpenAPI documentation configuration |
| `jsonrpc` | `?JsonRpcConfig` | `null` | JSON-RPC configuration |
| `grpc` | `?GrpcConfig` | `null` | gRPC configuration |
| `lifecycleHooks` | `?string` | `null` | Lifecycle hooks for request/response processing |
| `backgroundTasks` | `BackgroundTaskConfig` | — | Background task executor configuration |
| `enableHttpTrace` | `bool` | `false` | Enable per-request HTTP tracing (tower-http `TraceLayer`) |
| `diContainer` | `?string` | `null` | Dependency injection container (requires 'di' feature) |

###### Methods

###### default()

**Signature:**

```php
public static function default(): ServerConfig
```

---

##### ServerInfo

Server information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `string` | — | Url |
| `description` | `?string` | `null` | Human-readable description |

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
| `eventType` | `?string` | `null` | Event type (optional) |
| `data` | `string` | — | Event data (JSON value) |
| `id` | `?string` | `null` | Event ID (optional, for client-side reconnection) |
| `retry` | `?int` | `null` | Retry timeout in milliseconds (optional) |

### Methods

#### withId()

Set the event ID for client-side reconnection support

Sets an ID that clients can use to resume from this point if they disconnect.
The client sends this ID back in the `Last-Event-ID` header when reconnecting.

**Signature:**

```php
public function withId(string $id): SseEvent
```

##### withRetry()

Set the retry timeout for client reconnection

Sets the time in milliseconds clients should wait before attempting to reconnect
if the connection is lost. The client browser will automatically handle reconnection.

**Signature:**

```php
public function withRetry(int $retryMs): SseEvent
```

---

##### StaticFilesConfig

Static file serving configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `directory` | `string` | — | Directory path to serve |
| `routePrefix` | `string` | — | URL path prefix (e.g., "/static") |
| `indexFile` | `bool` | — | Fallback to index.html for directories |
| `cacheControl` | `?string` | `null` | Cache-Control header value |

---

##### UploadFile

Represents an uploaded file from multipart/form-data requests.

This struct provides efficient access to file content with automatic
base64 decoding and implements standard I/O traits for compatibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `filename` | `string` | — | Original filename from the client |
| `contentType` | `?string` | `null` | MIME type of the uploaded file |
| `size` | `?int` | `null` | Size of the file in bytes |
| `content` | `string` | — | File content (may be base64 encoded) |
| `contentEncoding` | `?string` | `null` | Content encoding type |
| `cursor` | `string` | — | Internal cursor for Read/Seek operations |

###### Methods

###### asBytes()

Get the raw file content as bytes.

This provides zero-copy access to the underlying buffer.

**Signature:**

```php
public function asBytes(): string
```

###### readToString()

Read the file content as a UTF-8 string.

**Errors:**

Returns an error if the content is not valid UTF-8.

**Signature:**

```php
public function readToString(): string
```

###### contentTypeOrDefault()

Get the content type, defaulting to "application/octet-stream".

**Signature:**

```php
public function contentTypeOrDefault(): string
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
| `Http` | Http — Fields: `scheme`: `string`, `bearerFormat`: `string` |
| `ApiKey` | Api key — Fields: `location`: `string`, `name`: `string` |

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

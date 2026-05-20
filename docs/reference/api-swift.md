---
title: "Swift API Reference"
---

## Swift API Reference <span class="version-badge">v0.14.0</span>

### Functions

#### schemaQueryOnly()

Create a simple schema configuration with only Query type.

This is a convenience function for schemas that only have queries.

**Returns:**

A `QueryOnlyConfig` with default settings

**Signature:**

```swift
public static func schemaQueryOnly() -> QueryOnlyConfig
```

**Returns:** `QueryOnlyConfig`

---

#### schemaQueryMutation()

Create a schema configuration with Query and Mutation types.

This is a convenience function for schemas with queries and mutations but no subscriptions.

**Returns:**

A `QueryMutationConfig` with default settings

**Signature:**

```swift
public static func schemaQueryMutation() -> QueryMutationConfig
```

**Returns:** `QueryMutationConfig`

---

#### schemaFull()

Create a schema configuration with all three root types.

This is a convenience function for fully-featured schemas.

**Returns:**

A `FullSchemaConfig` with default settings

**Signature:**

```swift
public static func schemaFull() -> FullSchemaConfig
```

**Returns:** `FullSchemaConfig`

---

### Types

#### ApiKeyConfig

API Key authentication configuration

| Field        | Type       | Default | Description                              |
| ------------ | ---------- | ------- | ---------------------------------------- |
| `keys`       | `[String]` | —       | Valid API keys                           |
| `headerName` | `String`   | —       | Header name to check (e.g., "X-API-Key") |

---

#### AsyncApiConfig

AsyncAPI HTTP endpoint configuration

| Field     | Type      | Default | Description                                                   |
| --------- | --------- | ------- | ------------------------------------------------------------- |
| `enabled` | `Bool`    | —       | Enable AsyncAPI endpoints (default: false)                    |
| `spec`    | `String?` | `null`  | Pre-registered AsyncAPI spec to serve from GET /asyncapi.json |

---

#### BackgroundJobMetadata

| Field       | Type      | Default | Description |
| ----------- | --------- | ------- | ----------- |
| `name`      | `String`  | —       | The name    |
| `requestId` | `String?` | `null`  | Request id  |

##### Methods

###### default()

**Signature:**

```swift
public static func default() -> BackgroundJobMetadata
```

---

#### BackgroundTaskConfig

Configuration for in-process background task execution.

| Field                | Type     | Default | Description              |
| -------------------- | -------- | ------- | ------------------------ |
| `maxQueueSize`       | `UInt64` | `1024`  | Maximum queue size       |
| `maxConcurrentTasks` | `UInt64` | `128`   | Maximum concurrent tasks |
| `drainTimeoutSecs`   | `UInt64` | `30`    | Drain timeout secs       |

##### Methods

###### default()

**Signature:**

```swift
public static func default() -> BackgroundTaskConfig
```

---

#### CompressionConfig

Compression configuration shared across runtimes

| Field     | Type     | Default | Description                                         |
| --------- | -------- | ------- | --------------------------------------------------- |
| `gzip`    | `Bool`   | `true`  | Enable gzip compression                             |
| `brotli`  | `Bool`   | `true`  | Enable brotli compression                           |
| `minSize` | `UInt64` | —       | Minimum response size to compress (bytes)           |
| `quality` | `UInt32` | —       | Compression quality (0-11 for brotli, 0-9 for gzip) |

##### Methods

###### default()

**Signature:**

```swift
public static func default() -> CompressionConfig
```

---

#### ContactInfo

Contact information

| Field   | Type      | Default | Description                                   |
| ------- | --------- | ------- | --------------------------------------------- |
| `name`  | `String?` | `null`  | Name of the contact person or organisation.   |
| `email` | `String?` | `null`  | Contact email address.                        |
| `url`   | `String?` | `null`  | URL pointing to the contact information page. |

---

#### CorsConfig

CORS configuration for a route

| Field                | Type        | Default | Description          |
| -------------------- | ----------- | ------- | -------------------- |
| `allowedOrigins`     | `[String]`  | `[]`    | Allowed origins      |
| `allowedMethods`     | `[String]`  | `[]`    | Allowed methods      |
| `allowedHeaders`     | `[String]`  | `[]`    | Allowed headers      |
| `exposeHeaders`      | `[String]?` | `null`  | Expose headers       |
| `maxAge`             | `UInt32?`   | `null`  | Maximum age          |
| `allowCredentials`   | `Bool?`     | `null`  | Allow credentials    |
| `methodsJoinedCache` | `String`    | —       | Methods joined cache |
| `headersJoinedCache` | `String`    | —       | Headers joined cache |

##### Methods

###### allowedMethodsJoined()

Get the cached joined methods string for preflight responses

**Signature:**

```swift
public func allowedMethodsJoined() -> String
```

###### allowedHeadersJoined()

Get the cached joined headers string for preflight responses

**Signature:**

```swift
public func allowedHeadersJoined() -> String
```

###### isOriginAllowed()

Check if an origin is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```swift
public func isOriginAllowed(origin: String) -> Bool
```

###### isMethodAllowed()

Check if a method is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```swift
public func isMethodAllowed(method: String) -> Bool
```

###### areHeadersAllowed()

Check if all requested headers are allowed (O(n) where n = num requested headers)

**Signature:**

```swift
public func areHeadersAllowed(requested: [String]) -> Bool
```

###### default()

**Signature:**

```swift
public static func default() -> CorsConfig
```

---

#### FullSchemaConfig

Configuration for fully-featured schemas with Query, Mutation, and Subscription types

| Field                  | Type      | Default | Description                                 |
| ---------------------- | --------- | ------- | ------------------------------------------- |
| `introspectionEnabled` | `Bool`    | `true`  | Enable introspection queries                |
| `complexityLimit`      | `UInt64?` | `null`  | Maximum query complexity (None = unlimited) |
| `depthLimit`           | `UInt64?` | `null`  | Maximum query depth (None = unlimited)      |

##### Methods

###### default()

**Signature:**

```swift
public static func default() -> FullSchemaConfig
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

```swift
public func path(path: String) -> GraphQlRouteConfig
```

###### method()

Set the HTTP method for the GraphQL endpoint

**Signature:**

```swift
public func method(method: String) -> GraphQlRouteConfig
```

###### enablePlayground()

Enable or disable the GraphQL Playground UI

**Signature:**

```swift
public func enablePlayground(enable: Bool) -> GraphQlRouteConfig
```

###### description()

Set a custom description for documentation

**Signature:**

```swift
public func description(description: String) -> GraphQlRouteConfig
```

###### getPath()

Get the configured path

**Signature:**

```swift
public func getPath() -> String
```

###### getMethod()

Get the configured method

**Signature:**

```swift
public func getMethod() -> String
```

###### isPlaygroundEnabled()

Check if playground is enabled

**Signature:**

```swift
public func isPlaygroundEnabled() -> Bool
```

###### getDescription()

Get the description if set

**Signature:**

```swift
public func getDescription() -> String?
```

###### default()

**Signature:**

```swift
public static func default() -> GraphQlRouteConfig
```

---

#### GraphQlSubscriptionSnapshot

Snapshot of a GraphQL subscription exchange over WebSocket.

| Field              | Type       | Default | Description                                                       |
| ------------------ | ---------- | ------- | ----------------------------------------------------------------- |
| `operationId`      | `String`   | —       | Operation id used for the subscription request.                   |
| `acknowledged`     | `Bool`     | —       | Whether the server acknowledged the GraphQL WebSocket connection. |
| `event`            | `String?`  | `null`  | First `next.payload` received for this subscription, if any.      |
| `errors`           | `[String]` | —       | GraphQL protocol errors emitted by the server.                    |
| `completeReceived` | `Bool`     | —       | Whether a `complete` frame was observed for this operation.       |

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

- **Stream Response Size Limits**: The `max_stream_response_bytes` field caps the
  total encoded bytes emitted across a server-streaming or bidi-streaming response.
  When the cumulative size exceeds the limit, the stream is terminated with
  `tonic.Status.resource_exhausted`. Defaults to `null` (unbounded).

| Field                    | Type      | Default | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| ------------------------ | --------- | ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                | `Bool`    | `true`  | Enable gRPC support                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `maxMessageSize`         | `UInt64`  | —       | Maximum message size in bytes (for both sending and receiving) This limit applies to individual messages in both unary and streaming RPCs. When a single message exceeds this size, the request is rejected with HTTP 413 (Payload Too Large). Default: 4MB (4194304 bytes) **Note:** This limit does NOT apply to the total response size in streaming RPCs. For multi-message streams, the total response can exceed this limit as long as each individual message stays within the limit.                                                                                                                                                                                                                                                               |
| `enableCompression`      | `Bool`    | `true`  | Enable gzip compression for gRPC messages                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `requestTimeout`         | `UInt64?` | `null`  | Timeout for gRPC requests in seconds (None = no timeout)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `maxConcurrentStreams`   | `UInt32`  | —       | Maximum number of concurrent streams per connection (HTTP/2 advisory) This value is communicated to HTTP/2 clients as the server's flow control limit. The HTTP/2 transport layer enforces this limit automatically via SETTINGS frames and GOAWAY responses. Applications should NOT implement custom enforcement. Default: 100 streams per connection # Stream Limiting Strategy - **Per Connection**: This limit applies per HTTP/2 connection, not globally - **Transport Enforcement**: HTTP/2 handles all stream limiting; applications need not implement custom checks - **Streaming Requests**: In server streaming or bidi streaming, each logical RPC consumes one stream slot. Message ordering within a stream follows HTTP/2 frame ordering. |
| `enableKeepalive`        | `Bool`    | `true`  | Enable HTTP/2 keepalive                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `keepaliveInterval`      | `UInt64`  | —       | HTTP/2 keepalive interval in seconds                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `keepaliveTimeout`       | `UInt64`  | —       | HTTP/2 keepalive timeout in seconds                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `maxStreamResponseBytes` | `UInt64?` | `null`  | Total byte cap across an entire streaming response. When `Some(n)`, the streaming adapter aborts the stream with `tonic.Status.resource_exhausted` once the cumulative encoded message bytes exceed `n`. The stream yields the error item and then terminates. Per-message cap remains `max_message_size`. This limit applies to server-streaming and bidirectional-streaming RPCs only; unary RPCs are governed solely by `max_message_size`. Default: `null` (unbounded total response size).                                                                                                                                                                                                                                                            |

### Methods

#### default()

**Signature:**

```swift
public static func default() -> GrpcConfig
```

---

##### JsonRpcConfig

JSON-RPC server configuration

| Field          | Type     | Default | Description                                                |
| -------------- | -------- | ------- | ---------------------------------------------------------- |
| `enabled`      | `Bool`   | `true`  | Enable JSON-RPC endpoint                                   |
| `endpointPath` | `String` | —       | HTTP endpoint path for JSON-RPC requests (default: "/rpc") |
| `enableBatch`  | `Bool`   | —       | Enable batch request processing (default: true)            |
| `maxBatchSize` | `UInt64` | —       | Maximum number of requests in a batch (default: 100)       |

###### Methods

###### default()

**Signature:**

```swift
public static func default() -> JsonRpcConfig
```

---

##### JsonRpcMethodInfo

JSON-RPC method metadata for routes that support JSON-RPC

This struct captures the metadata needed to expose HTTP routes as JSON-RPC methods,
enabling discovery and documentation of RPC-compatible endpoints.

| Field          | Type       | Default | Description                                    |
| -------------- | ---------- | ------- | ---------------------------------------------- |
| `methodName`   | `String`   | —       | The JSON-RPC method name (e.g., "user.create") |
| `description`  | `String?`  | `null`  | Optional description of what the method does   |
| `paramsSchema` | `String?`  | `null`  | Optional JSON Schema for method parameters     |
| `resultSchema` | `String?`  | `null`  | Optional JSON Schema for the result            |
| `deprecated`   | `Bool`     | —       | Whether this method is deprecated              |
| `tags`         | `[String]` | —       | Tags for categorizing and grouping methods     |

---

##### JwtConfig

JWT authentication configuration

| Field       | Type        | Default | Description                                           |
| ----------- | ----------- | ------- | ----------------------------------------------------- |
| `secret`    | `String`    | —       | Secret key for JWT verification                       |
| `algorithm` | `String`    | —       | Required algorithm (HS256, HS384, HS512, RS256, etc.) |
| `audience`  | `[String]?` | `null`  | Required audience claim                               |
| `issuer`    | `String?`   | `null`  | Required issuer claim                                 |
| `leeway`    | `UInt64`    | —       | Leeway for expiration checks (seconds)                |

---

##### LicenseInfo

License information

| Field  | Type      | Default | Description                                             |
| ------ | --------- | ------- | ------------------------------------------------------- |
| `name` | `String`  | —       | SPDX license identifier or display name (e.g. `"MIT"`). |
| `url`  | `String?` | `null`  | URL to the full license text.                           |

---

##### OpenApiConfig

OpenAPI configuration

| Field             | Type                           | Default   | Description                                                      |
| ----------------- | ------------------------------ | --------- | ---------------------------------------------------------------- |
| `enabled`         | `Bool`                         | `false`   | Enable OpenAPI generation (default: false for zero overhead)     |
| `title`           | `String`                       | `"API"`   | API title                                                        |
| `version`         | `String`                       | `"1.0.0"` | API version                                                      |
| `description`     | `String?`                      | `null`    | API description (supports markdown)                              |
| `swaggerUiPath`   | `String`                       | —         | Path to serve Swagger UI (default: "/docs")                      |
| `redocPath`       | `String`                       | —         | Path to serve Redoc (default: "/redoc")                          |
| `openapiJsonPath` | `String`                       | —         | Path to serve OpenAPI JSON spec (default: "/openapi.json")       |
| `contact`         | `ContactInfo?`                 | `null`    | Contact information                                              |
| `license`         | `LicenseInfo?`                 | `null`    | License information                                              |
| `servers`         | `[ServerInfo]`                 | `[]`      | Server definitions                                               |
| `securitySchemes` | `[String: SecuritySchemeInfo]` | `{}`      | Security schemes (auto-detected from middleware if not provided) |

###### Methods

###### default()

**Signature:**

```swift
public static func default() -> OpenApiConfig
```

---

##### ParseRequest

Request body for `POST /asyncapi/parse`

| Field  | Type     | Default | Description |
| ------ | -------- | ------- | ----------- |
| `spec` | `String` | —       | Spec        |

---

##### ParseResult

Full parse result returned by `POST /asyncapi/parse`

| Field         | Type                | Default | Description  |
| ------------- | ------------------- | ------- | ------------ |
| `specVersion` | `String`            | —       | Spec version |
| `title`       | `String`            | —       | Title        |
| `apiVersion`  | `String`            | —       | Api version  |
| `channels`    | `[ParsedChannel]`   | —       | Channels     |
| `operations`  | `[ParsedOperation]` | —       | Operations   |
| `messages`    | `[ParsedMessage]`   | —       | Messages     |

---

##### ParsedChannel

A single channel extracted from an AsyncAPI spec

| Field      | Type       | Default | Description                                                           |
| ---------- | ---------- | ------- | --------------------------------------------------------------------- |
| `name`     | `String`   | —       | Channel key from the spec (e.g. "chat/messages")                      |
| `address`  | `String`   | —       | Channel address / path                                                |
| `messages` | `[String]` | —       | Message names declared on this channel                                |
| `bindings` | `String?`  | `null`  | Bindings (ws / http / amqp / …) as raw JSON for forward-compatibility |

---

##### ParsedMessage

A resolved message (name + JSON Schema)

| Field    | Type      | Default | Description                                                |
| -------- | --------- | ------- | ---------------------------------------------------------- |
| `name`   | `String`  | —       | Message name                                               |
| `schema` | `String?` | `null`  | Resolved JSON Schema for the message payload, if available |

---

##### ParsedOperation

A single operation extracted from an AsyncAPI spec

| Field     | Type     | Default | Description                                      |
| --------- | -------- | ------- | ------------------------------------------------ |
| `name`    | `String` | —       | Operation name                                   |
| `action`  | `String` | —       | Operation action: "send" or "receive"            |
| `channel` | `String` | —       | Channel reference (resolved to the channel name) |

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

```json
{
  "type": "<https://spikard.dev/errors/validation-error",>
  "title": "Request Validation Failed",
  "status": 422,
  "detail": "2 validation errors in request body",
  "errors": [...]
}
```

| Field        | Type               | Default | Description                                                                                                                                                  |
| ------------ | ------------------ | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `typeUri`    | `String`           | —       | A URI reference that identifies the problem type. Defaults to "about:blank" when absent. Should be a stable, human-readable identifier for the problem type. |
| `title`      | `String`           | —       | A short, human-readable summary of the problem type. Should not change from occurrence to occurrence of the problem.                                         |
| `status`     | `UInt16`           | —       | The HTTP status code generated by the origin server. This is advisory; the actual HTTP status code takes precedence.                                         |
| `detail`     | `String?`          | `null`  | A human-readable explanation specific to this occurrence of the problem.                                                                                     |
| `instance`   | `String?`          | `null`  | A URI reference that identifies the specific occurrence of the problem. It may or may not yield further information if dereferenced.                         |
| `extensions` | `[String: String]` | —       | Extension members - problem-type-specific data. For validation errors, this typically contains an "errors" array.                                            |

### Methods

#### withDetail()

Set the detail field

**Signature:**

```swift
public func withDetail(detail: String) -> ProblemDetails
```

##### withInstance()

Set the instance field

**Signature:**

```swift
public func withInstance(instance: String) -> ProblemDetails
```

###### notFound()

Create a not found error

**Signature:**

```swift
public static func notFound(detail: String) -> ProblemDetails
```

###### methodNotAllowed()

Create a method not allowed error

**Signature:**

```swift
public static func methodNotAllowed(detail: String) -> ProblemDetails
```

###### internalServerError()

Create an internal server error

**Signature:**

```swift
public static func internalServerError(detail: String) -> ProblemDetails
```

###### badRequest()

Create a bad request error

**Signature:**

```swift
public static func badRequest(detail: String) -> ProblemDetails
```

###### toJson()

Serialize to JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```swift
public func toJson() throws -> String
```

###### toJsonPretty()

Serialize to pretty JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```swift
public func toJsonPretty() throws -> String
```

---

##### QueryMutationConfig

Configuration for schemas with Query and Mutation types

| Field                  | Type      | Default | Description                                 |
| ---------------------- | --------- | ------- | ------------------------------------------- |
| `introspectionEnabled` | `Bool`    | `true`  | Enable introspection queries                |
| `complexityLimit`      | `UInt64?` | `null`  | Maximum query complexity (None = unlimited) |
| `depthLimit`           | `UInt64?` | `null`  | Maximum query depth (None = unlimited)      |

###### Methods

###### default()

**Signature:**

```swift
public static func default() -> QueryMutationConfig
```

---

##### QueryOnlyConfig

Configuration for schemas with only Query type

| Field                  | Type      | Default | Description                                 |
| ---------------------- | --------- | ------- | ------------------------------------------- |
| `introspectionEnabled` | `Bool`    | `true`  | Enable introspection queries                |
| `complexityLimit`      | `UInt64?` | `null`  | Maximum query complexity (None = unlimited) |
| `depthLimit`           | `UInt64?` | `null`  | Maximum query depth (None = unlimited)      |

###### Methods

###### default()

**Signature:**

```swift
public static func default() -> QueryOnlyConfig
```

---

##### RateLimitConfig

Rate limiting configuration shared across runtimes

| Field       | Type     | Default | Description                |
| ----------- | -------- | ------- | -------------------------- |
| `perSecond` | `UInt64` | `100`   | Requests per second        |
| `burst`     | `UInt32` | `200`   | Burst allowance            |
| `ipBased`   | `Bool`   | `true`  | Use IP-based rate limiting |

###### Methods

###### default()

**Signature:**

```swift
public static func default() -> RateLimitConfig
```

---

##### Response

HTTP Response with custom status code, headers, and content

| Field        | Type               | Default | Description                        |
| ------------ | ------------------ | ------- | ---------------------------------- |
| `content`    | `String?`          | `null`  | Response body content              |
| `statusCode` | `UInt16`           | —       | HTTP status code (defaults to 200) |
| `headers`    | `[String: String]` | `{}`    | Response headers                   |

###### Methods

###### setHeader()

Set a header

**Signature:**

```swift
public func setHeader(key: String, value: String)
```

###### setCookie()

Set a cookie in the response

**Signature:**

```swift
public func setCookie(key: String, value: String, secure: Bool, httpOnly: Bool, maxAge: Int64? = nil, domain: String? = nil, path: String? = nil, sameSite: String? = nil)
```

###### default()

**Signature:**

```swift
public static func default() -> Response
```

---

##### ResponseSnapshot

Snapshot of an Axum response used by higher-level language bindings.

| Field     | Type               | Default | Description                                                |
| --------- | ------------------ | ------- | ---------------------------------------------------------- |
| `status`  | `UInt16`           | —       | HTTP status code.                                          |
| `headers` | `[String: String]` | —       | Response headers (lowercase keys for predictable lookups). |
| `body`    | `Data`             | —       | Response body bytes (decoded for supported encodings).     |

###### Methods

###### text()

Return response body as UTF-8 string.

**Signature:**

```swift
public func text() throws -> String
```

###### header()

Lookup header by case-insensitive name.

**Signature:**

```swift
public func header(name: String) -> String?
```

---

##### SchemaConfig

Configuration for GraphQL schema building.

Encapsulates all schema-level configuration options including
introspection control, complexity limits, and depth limits.

| Field                  | Type      | Default | Description                                 |
| ---------------------- | --------- | ------- | ------------------------------------------- |
| `introspectionEnabled` | `Bool`    | `true`  | Enable introspection queries                |
| `complexityLimit`      | `UInt64?` | `null`  | Maximum query complexity (None = unlimited) |
| `depthLimit`           | `UInt64?` | `null`  | Maximum query depth (None = unlimited)      |

###### Methods

###### default()

**Signature:**

```swift
public static func default() -> SchemaConfig
```

---

##### ServerConfig

Server configuration

| Field              | Type                   | Default       | Description                                                                    |
| ------------------ | ---------------------- | ------------- | ------------------------------------------------------------------------------ |
| `host`             | `String`               | `"127.0.0.1"` | Host to bind to                                                                |
| `port`             | `UInt16`               | `8000`        | Port to bind to                                                                |
| `workers`          | `UInt64`               | `1`           | Number of Tokio runtime worker threads used by binding-managed server runtimes |
| `enableRequestId`  | `Bool`                 | `false`       | Enable request ID generation and propagation                                   |
| `maxBodySize`      | `UInt64?`              | `null`        | Maximum request body size in bytes (None = unlimited, not recommended)         |
| `requestTimeout`   | `UInt64?`              | `null`        | Request timeout in seconds (None = no timeout)                                 |
| `compression`      | `CompressionConfig?`   | `null`        | Enable compression middleware                                                  |
| `rateLimit`        | `RateLimitConfig?`     | `null`        | Enable rate limiting                                                           |
| `jwtAuth`          | `JwtConfig?`           | `null`        | JWT authentication configuration                                               |
| `apiKeyAuth`       | `ApiKeyConfig?`        | `null`        | API Key authentication configuration                                           |
| `staticFiles`      | `[StaticFilesConfig]`  | `[]`          | Static file serving configuration                                              |
| `gracefulShutdown` | `Bool`                 | `true`        | Enable graceful shutdown on SIGTERM/SIGINT                                     |
| `shutdownTimeout`  | `UInt64`               | `30`          | Graceful shutdown timeout (seconds)                                            |
| `asyncapi`         | `AsyncApiConfig?`      | `null`        | AsyncAPI HTTP endpoint configuration                                           |
| `openapi`          | `OpenApiConfig?`       | `null`        | OpenAPI documentation configuration                                            |
| `jsonrpc`          | `JsonRpcConfig?`       | `null`        | JSON-RPC configuration                                                         |
| `grpc`             | `GrpcConfig?`          | `null`        | gRPC configuration                                                             |
| `lifecycleHooks`   | `String?`              | `null`        | Lifecycle hooks for request/response processing                                |
| `backgroundTasks`  | `BackgroundTaskConfig` | —             | Background task executor configuration                                         |
| `enableHttpTrace`  | `Bool`                 | `false`       | Enable per-request HTTP tracing (tower-http `TraceLayer`)                      |
| `diContainer`      | `String?`              | `null`        | Dependency injection container (requires 'di' feature)                         |

###### Methods

###### default()

**Signature:**

```swift
public static func default() -> ServerConfig
```

---

##### ServerInfo

Server information

| Field         | Type      | Default | Description                                                     |
| ------------- | --------- | ------- | --------------------------------------------------------------- |
| `url`         | `String`  | —       | Base URL of the server (e.g. `"<https://api.example.com/v1"`>). |
| `description` | `String?` | `null`  | Optional human-readable description of the server environment.  |

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

| Field       | Type      | Default | Description                                       |
| ----------- | --------- | ------- | ------------------------------------------------- |
| `eventType` | `String?` | `null`  | Event type (optional)                             |
| `data`      | `String`  | —       | Event data (JSON value)                           |
| `id`        | `String?` | `null`  | Event ID (optional, for client-side reconnection) |
| `retry`     | `UInt64?` | `null`  | Retry timeout in milliseconds (optional)          |

### Methods

#### withId()

Set the event ID for client-side reconnection support

Sets an ID that clients can use to resume from this point if they disconnect.
The client sends this ID back in the `Last-Event-ID` header when reconnecting.

**Signature:**

```swift
public func withId(id: String) -> SseEvent
```

##### withRetry()

Set the retry timeout for client reconnection

Sets the time in milliseconds clients should wait before attempting to reconnect
if the connection is lost. The client browser will automatically handle reconnection.

**Signature:**

```swift
public func withRetry(retryMs: UInt64) -> SseEvent
```

---

##### StaticFilesConfig

Static file serving configuration

| Field          | Type      | Default | Description                            |
| -------------- | --------- | ------- | -------------------------------------- |
| `directory`    | `String`  | —       | Directory path to serve                |
| `routePrefix`  | `String`  | —       | URL path prefix (e.g., "/static")      |
| `indexFile`    | `Bool`    | —       | Fallback to index.html for directories |
| `cacheControl` | `String?` | `null`  | Cache-Control header value             |

---

##### TestClient

Core test client for making HTTP requests to a Spikard application.

This struct wraps axum-test's TestServer and provides a language-agnostic
interface for making HTTP requests, sending WebSocket connections, and
handling Server-Sent Events. Language bindings wrap this to provide
native API surfaces.

###### Methods

###### get()

Make a GET request

**Signature:**

```swift
public func get(path: String, queryParams: [String]? = nil, headers: [String]? = nil) throws -> ResponseSnapshot
```

###### post()

Make a POST request

**Signature:**

```swift
public func post(path: String, json: String? = nil, formData: [String]? = nil, multipart: String, queryParams: [String]? = nil, headers: [String]? = nil) throws -> ResponseSnapshot
```

###### requestRaw()

Make a request with a raw body payload.

**Signature:**

```swift
public func requestRaw(method: Method, path: String, body: Data, queryParams: [String]? = nil, headers: [String]? = nil) throws -> ResponseSnapshot
```

###### put()

Make a PUT request

**Signature:**

```swift
public func put(path: String, json: String? = nil, queryParams: [String]? = nil, headers: [String]? = nil) throws -> ResponseSnapshot
```

###### patch()

Make a PATCH request

**Signature:**

```swift
public func patch(path: String, json: String? = nil, queryParams: [String]? = nil, headers: [String]? = nil) throws -> ResponseSnapshot
```

###### delete()

Make a DELETE request

**Signature:**

```swift
public func delete(path: String, queryParams: [String]? = nil, headers: [String]? = nil) throws -> ResponseSnapshot
```

###### options()

Make an OPTIONS request

**Signature:**

```swift
public func options(path: String, queryParams: [String]? = nil, headers: [String]? = nil) throws -> ResponseSnapshot
```

###### head()

Make a HEAD request

**Signature:**

```swift
public func head(path: String, queryParams: [String]? = nil, headers: [String]? = nil) throws -> ResponseSnapshot
```

###### trace()

Make a TRACE request

**Signature:**

```swift
public func trace(path: String, queryParams: [String]? = nil, headers: [String]? = nil) throws -> ResponseSnapshot
```

###### graphqlAt()

Send a GraphQL query/mutation to a custom endpoint

**Signature:**

```swift
public func graphqlAt(endpoint: String, query: String, variables: String? = nil, operationName: String? = nil) throws -> ResponseSnapshot
```

###### graphql()

Send a GraphQL query/mutation

**Signature:**

```swift
public func graphql(query: String, variables: String? = nil, operationName: String? = nil) throws -> ResponseSnapshot
```

###### graphqlWithStatus()

Send a GraphQL query and return HTTP status code separately

This method allows tests to distinguish between:

- HTTP-level errors (400/422 for invalid requests)
- GraphQL-level errors (200 with errors in response body)

**Signature:**

```swift
public func graphqlWithStatus(query: String, variables: String? = nil, operationName: String? = nil) throws -> String
```

###### graphqlSubscriptionAt()

Send a GraphQL subscription (WebSocket) to a custom endpoint.

Uses the `graphql-transport-ws` protocol and captures the first `next` payload.
After the first payload is received, this client sends `complete` to unsubscribe.

**Signature:**

```swift
public func graphqlSubscriptionAt(endpoint: String, query: String, variables: String? = nil, operationName: String? = nil) throws -> GraphQlSubscriptionSnapshot
```

###### graphqlSubscription()

Send a GraphQL subscription (WebSocket).

Uses `/graphql` as the default subscription endpoint.

**Signature:**

```swift
public func graphqlSubscription(query: String, variables: String? = nil, operationName: String? = nil) throws -> GraphQlSubscriptionSnapshot
```

---

##### TestingSseEvent

A single Server-Sent Event.

| Field  | Type     | Default | Description                  |
| ------ | -------- | ------- | ---------------------------- |
| `data` | `String` | —       | The data field of the event. |

---

##### UploadFile

Represents an uploaded file from multipart/form-data requests.

This struct provides efficient access to file content with automatic
base64 decoding and implements standard I/O traits for compatibility.

| Field             | Type      | Default | Description                              |
| ----------------- | --------- | ------- | ---------------------------------------- |
| `filename`        | `String`  | —       | Original filename from the client        |
| `contentType`     | `String?` | `null`  | MIME type of the uploaded file           |
| `size`            | `UInt64?` | `null`  | Size of the file in bytes                |
| `content`         | `Data`    | —       | File content (may be base64 encoded)     |
| `contentEncoding` | `String?` | `null`  | Content encoding type                    |
| `cursor`          | `String`  | —       | Internal cursor for Read/Seek operations |

###### Methods

###### asBytes()

Get the raw file content as bytes.

This provides zero-copy access to the underlying buffer.

**Signature:**

```swift
public func asBytes() -> Data
```

###### readToString()

Read the file content as a UTF-8 string.

**Errors:**

Returns an error if the content is not valid UTF-8.

**Signature:**

```swift
public func readToString() throws -> String
```

###### contentTypeOrDefault()

Get the content type, defaulting to "application/octet-stream".

**Signature:**

```swift
public func contentTypeOrDefault() -> String
```

---

##### ValidateRequest

Request body for `POST /asyncapi/validate`

| Field     | Type     | Default | Description |
| --------- | -------- | ------- | ----------- |
| `spec`    | `String` | —       | Spec        |
| `channel` | `String` | —       | Channel     |
| `message` | `String` | —       | Message     |
| `payload` | `String` | —       | Payload     |

---

##### ValidationResponse

Response body for `POST /asyncapi/validate`

| Field    | Type       | Default | Description |
| -------- | ---------- | ------- | ----------- |
| `valid`  | `Bool`     | —       | Valid       |
| `errors` | `[String]` | —       | Errors      |

---

#### Enums

##### SnapshotError

Possible errors while converting an Axum response into a snapshot.

| Value           | Description                                                            |
| --------------- | ---------------------------------------------------------------------- |
| `InvalidHeader` | Response header could not be decoded to UTF-8. — Fields: `0`: `String` |
| `Decompression` | Body decompression failed. — Fields: `0`: `String`                     |

---

##### WebSocketMessage

A WebSocket message that can be text or binary.

| Value    | Description                                                                                                                                                                                                                       |
| -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Text`   | A text message. — Fields: `0`: `String`                                                                                                                                                                                           |
| `Binary` | A binary message. — Fields: `0`: `Data`                                                                                                                                                                                           |
| `Close`  | A close message with a numeric close code (RFC 6455) and optional reason text. Common codes: 1000 Normal Closure, 1001 Going Away, 1005 No Status Received, 1006 Abnormal Closure. — Fields: `code`: `UInt16`, `reason`: `String` |
| `Ping`   | A ping message. — Fields: `0`: `Data`                                                                                                                                                                                             |
| `Pong`   | A pong message. — Fields: `0`: `Data`                                                                                                                                                                                             |

---

##### Method

HTTP method

| Value     | Description |
| --------- | ----------- |
| `Get`     | Get         |
| `Post`    | Post        |
| `Put`     | Put         |
| `Patch`   | Patch       |
| `Delete`  | Delete      |
| `Head`    | Head        |
| `Options` | Options     |
| `Trace`   | Trace       |

---

##### SecuritySchemeInfo

Security scheme types

| Value    | Description                                                 |
| -------- | ----------------------------------------------------------- |
| `Http`   | Http — Fields: `scheme`: `String`, `bearerFormat`: `String` |
| `ApiKey` | Api key — Fields: `location`: `String`, `name`: `String`    |

---

#### Errors

##### GraphQlError

Errors that can occur during GraphQL operations

These errors are compatible with async-graphql error handling and can be
converted to structured HTTP responses matching the project's error fixtures.

| Variant                   | Description                                                                                                       |
| ------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| `ExecutionError`          | Error during schema execution Occurs when the GraphQL executor encounters a runtime error during query execution. |
| `SchemaBuildError`        | Error during schema building Occurs when schema construction fails due to invalid definitions or conflicts.       |
| `RequestHandlingError`    | Error during request handling Occurs when the HTTP request cannot be properly handled or parsed.                  |
| `SerializationError`      | Serialization error Occurs during JSON serialization/deserialization of GraphQL values.                           |
| `JsonError`               | JSON parsing error Occurs when JSON input cannot be parsed.                                                       |
| `ValidationError`         | GraphQL validation error Occurs when a GraphQL query fails schema validation.                                     |
| `ParseError`              | GraphQL parse error Occurs when the GraphQL query string cannot be parsed.                                        |
| `AuthenticationError`     | Authentication error Occurs when request authentication fails.                                                    |
| `AuthorizationError`      | Authorization error Occurs when user lacks required permissions.                                                  |
| `NotFound`                | Not found error Occurs when a requested resource is not found.                                                    |
| `RateLimitExceeded`       | Rate limit error Occurs when rate limit is exceeded.                                                              |
| `InvalidInput`            | Invalid input error with validation details Occurs during input validation with detailed error information.       |
| `ComplexityLimitExceeded` | Query complexity limit exceeded Occurs when a GraphQL query exceeds the configured complexity limit.              |
| `DepthLimitExceeded`      | Query depth limit exceeded Occurs when a GraphQL query exceeds the configured depth limit.                        |
| `InternalError`           | Internal server error Occurs when an unexpected internal error happens.                                           |

---

##### SchemaError

Error type for schema building operations

| Variant                   | Description                    |
| ------------------------- | ------------------------------ |
| `BuildingFailed`          | Generic schema building error  |
| `ValidationError`         | Configuration validation error |
| `ComplexityLimitExceeded` | Complexity limit exceeded      |
| `DepthLimitExceeded`      | Depth limit exceeded           |

---

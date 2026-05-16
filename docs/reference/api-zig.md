---
title: "Zig API Reference"
---

## Zig API Reference <span class="version-badge">v0.14.0</span>

### Functions

#### schemaQueryOnly()

Create a simple schema configuration with only Query type.

This is a convenience function for schemas that only have queries.

**Returns:**

A `QueryOnlyConfig` with default settings

**Signature:**

```zig
// Phase 1: zig backend signature generation
```

**Returns:** `QueryOnlyConfig`

---

#### schemaQueryMutation()

Create a schema configuration with Query and Mutation types.

This is a convenience function for schemas with queries and mutations but no subscriptions.

**Returns:**

A `QueryMutationConfig` with default settings

**Signature:**

```zig
// Phase 1: zig backend signature generation
```

**Returns:** `QueryMutationConfig`

---

#### schemaFull()

Create a schema configuration with all three root types.

This is a convenience function for fully-featured schemas.

**Returns:**

A `FullSchemaConfig` with default settings

**Signature:**

```zig
// Phase 1: zig backend signature generation
```

**Returns:** `FullSchemaConfig`

---

### Types

#### ApiKeyConfig

API Key authentication configuration

| Field        | Type                   | Default | Description                              |
| ------------ | ---------------------- | ------- | ---------------------------------------- |
| `keys`       | `[]const [:0]const u8` | —       | Valid API keys                           |
| `headerName` | `[:0]const u8`         | —       | Header name to check (e.g., "X-API-Key") |

---

#### AsyncApiConfig

AsyncAPI HTTP endpoint configuration

| Field     | Type            | Default | Description                                                   |
| --------- | --------------- | ------- | ------------------------------------------------------------- |
| `enabled` | `bool`          | —       | Enable AsyncAPI endpoints (default: false)                    |
| `spec`    | `[:0]const u8?` | `null`  | Pre-registered AsyncAPI spec to serve from GET /asyncapi.json |

---

#### BackgroundJobMetadata

| Field       | Type            | Default | Description |
| ----------- | --------------- | ------- | ----------- |
| `name`      | `[:0]const u8`  | —       | The name    |
| `requestId` | `[:0]const u8?` | `null`  | Request id  |

##### Methods

###### default()

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

---

#### BackgroundTaskConfig

Configuration for in-process background task execution.

| Field                | Type  | Default | Description              |
| -------------------- | ----- | ------- | ------------------------ |
| `maxQueueSize`       | `u64` | `1024`  | Maximum queue size       |
| `maxConcurrentTasks` | `u64` | `128`   | Maximum concurrent tasks |
| `drainTimeoutSecs`   | `u64` | `30`    | Drain timeout secs       |

##### Methods

###### default()

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

---

#### CompressionConfig

Compression configuration shared across runtimes

| Field     | Type   | Default | Description                                         |
| --------- | ------ | ------- | --------------------------------------------------- |
| `gzip`    | `bool` | `true`  | Enable gzip compression                             |
| `brotli`  | `bool` | `true`  | Enable brotli compression                           |
| `minSize` | `u64`  | —       | Minimum response size to compress (bytes)           |
| `quality` | `u32`  | —       | Compression quality (0-11 for brotli, 0-9 for gzip) |

##### Methods

###### default()

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

---

#### ContactInfo

Contact information

| Field   | Type            | Default | Description |
| ------- | --------------- | ------- | ----------- |
| `name`  | `[:0]const u8?` | `null`  | The name    |
| `email` | `[:0]const u8?` | `null`  | Email       |
| `url`   | `[:0]const u8?` | `null`  | Url         |

---

#### CorsConfig

CORS configuration for a route

| Field              | Type                    | Default | Description       |
| ------------------ | ----------------------- | ------- | ----------------- |
| `allowedOrigins`   | `[]const [:0]const u8`  | `[]`    | Allowed origins   |
| `allowedMethods`   | `[]const [:0]const u8`  | `[]`    | Allowed methods   |
| `allowedHeaders`   | `[]const [:0]const u8`  | `[]`    | Allowed headers   |
| `exposeHeaders`    | `[]const [:0]const u8?` | `null`  | Expose headers    |
| `maxAge`           | `u32?`                  | `null`  | Maximum age       |
| `allowCredentials` | `bool?`                 | `null`  | Allow credentials |

##### Methods

###### allowedMethodsJoined()

Get the cached joined methods string for preflight responses

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### allowedHeadersJoined()

Get the cached joined headers string for preflight responses

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### isOriginAllowed()

Check if an origin is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### isMethodAllowed()

Check if a method is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### areHeadersAllowed()

Check if all requested headers are allowed (O(n) where n = num requested headers)

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### default()

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

---

#### FullSchemaConfig

Configuration for fully-featured schemas with Query, Mutation, and Subscription types

| Field                  | Type   | Default | Description                                 |
| ---------------------- | ------ | ------- | ------------------------------------------- |
| `introspectionEnabled` | `bool` | `true`  | Enable introspection queries                |
| `complexityLimit`      | `u64?` | `null`  | Maximum query complexity (None = unlimited) |
| `depthLimit`           | `u64?` | `null`  | Maximum query depth (None = unlimited)      |

##### Methods

###### default()

**Signature:**

```zig
// Phase 1: zig backend method signature generation
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

```zig
// Phase 1: zig backend method signature generation
```

###### method()

Set the HTTP method for the GraphQL endpoint

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### enablePlayground()

Enable or disable the GraphQL Playground UI

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### description()

Set a custom description for documentation

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### getPath()

Get the configured path

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### getMethod()

Get the configured method

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### isPlaygroundEnabled()

Check if playground is enabled

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### getDescription()

Get the description if set

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### default()

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

---

#### GraphQlSubscriptionSnapshot

Snapshot of a GraphQL subscription exchange over WebSocket.

| Field              | Type                   | Default | Description                                                       |
| ------------------ | ---------------------- | ------- | ----------------------------------------------------------------- |
| `operationId`      | `[:0]const u8`         | —       | Operation id used for the subscription request.                   |
| `acknowledged`     | `bool`                 | —       | Whether the server acknowledged the GraphQL WebSocket connection. |
| `event`            | `[:0]const u8?`        | `null`  | First `next.payload` received for this subscription, if any.      |
| `errors`           | `[]const [:0]const u8` | —       | GraphQL protocol errors emitted by the server.                    |
| `completeReceived` | `bool`                 | —       | Whether a `complete` frame was observed for this operation.       |

---

#### GrpcConfig

Configuration for gRPC support

Controls how the server handles gRPC requests, including compression,
timeouts, and protocol settings.

# Stream Limits

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

| Field                    | Type   | Default | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| ------------------------ | ------ | ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                | `bool` | `true`  | Enable gRPC support                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `maxMessageSize`         | `u64`  | —       | Maximum message size in bytes (for both sending and receiving) This limit applies to individual messages in both unary and streaming RPCs. When a single message exceeds this size, the request is rejected with HTTP 413 (Payload Too Large). Default: 4MB (4194304 bytes) **Note:** This limit does NOT apply to the total response size in streaming RPCs. For multi-message streams, the total response can exceed this limit as long as each individual message stays within the limit.                                                                                                                                                                                                                                                               |
| `enableCompression`      | `bool` | `true`  | Enable gzip compression for gRPC messages                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `requestTimeout`         | `u64?` | `null`  | Timeout for gRPC requests in seconds (None = no timeout)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `maxConcurrentStreams`   | `u32`  | —       | Maximum number of concurrent streams per connection (HTTP/2 advisory) This value is communicated to HTTP/2 clients as the server's flow control limit. The HTTP/2 transport layer enforces this limit automatically via SETTINGS frames and GOAWAY responses. Applications should NOT implement custom enforcement. Default: 100 streams per connection # Stream Limiting Strategy - **Per Connection**: This limit applies per HTTP/2 connection, not globally - **Transport Enforcement**: HTTP/2 handles all stream limiting; applications need not implement custom checks - **Streaming Requests**: In server streaming or bidi streaming, each logical RPC consumes one stream slot. Message ordering within a stream follows HTTP/2 frame ordering. |
| `enableKeepalive`        | `bool` | `true`  | Enable HTTP/2 keepalive                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `keepaliveInterval`      | `u64`  | —       | HTTP/2 keepalive interval in seconds                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `keepaliveTimeout`       | `u64`  | —       | HTTP/2 keepalive timeout in seconds                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `maxStreamResponseBytes` | `u64?` | `null`  | Total byte cap across an entire streaming response. When `Some(n)`, the streaming adapter aborts the stream with `tonic.Status.resource_exhausted` once the cumulative encoded message bytes exceed `n`. The stream yields the error item and then terminates. Per-message cap remains `max_message_size`. This limit applies to server-streaming and bidirectional-streaming RPCs only; unary RPCs are governed solely by `max_message_size`. Default: `null` (unbounded total response size).                                                                                                                                                                                                                                                            |

##### Methods

###### default()

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

---

#### JsonRpcConfig

JSON-RPC server configuration

| Field          | Type           | Default | Description                                                |
| -------------- | -------------- | ------- | ---------------------------------------------------------- |
| `enabled`      | `bool`         | `true`  | Enable JSON-RPC endpoint                                   |
| `endpointPath` | `[:0]const u8` | —       | HTTP endpoint path for JSON-RPC requests (default: "/rpc") |
| `enableBatch`  | `bool`         | —       | Enable batch request processing (default: true)            |
| `maxBatchSize` | `u64`          | —       | Maximum number of requests in a batch (default: 100)       |

##### Methods

###### default()

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

---

#### JsonRpcMethodInfo

JSON-RPC method metadata for routes that support JSON-RPC

This struct captures the metadata needed to expose HTTP routes as JSON-RPC methods,
enabling discovery and documentation of RPC-compatible endpoints.

| Field          | Type                   | Default | Description                                    |
| -------------- | ---------------------- | ------- | ---------------------------------------------- |
| `methodName`   | `[:0]const u8`         | —       | The JSON-RPC method name (e.g., "user.create") |
| `description`  | `[:0]const u8?`        | `null`  | Optional description of what the method does   |
| `paramsSchema` | `[:0]const u8?`        | `null`  | Optional JSON Schema for method parameters     |
| `resultSchema` | `[:0]const u8?`        | `null`  | Optional JSON Schema for the result            |
| `deprecated`   | `bool`                 | —       | Whether this method is deprecated              |
| `tags`         | `[]const [:0]const u8` | —       | Tags for categorizing and grouping methods     |

---

#### JwtConfig

JWT authentication configuration

| Field       | Type                    | Default | Description                                           |
| ----------- | ----------------------- | ------- | ----------------------------------------------------- |
| `secret`    | `[:0]const u8`          | —       | Secret key for JWT verification                       |
| `algorithm` | `[:0]const u8`          | —       | Required algorithm (HS256, HS384, HS512, RS256, etc.) |
| `audience`  | `[]const [:0]const u8?` | `null`  | Required audience claim                               |
| `issuer`    | `[:0]const u8?`         | `null`  | Required issuer claim                                 |
| `leeway`    | `u64`                   | —       | Leeway for expiration checks (seconds)                |

---

#### LicenseInfo

License information

| Field  | Type            | Default | Description |
| ------ | --------------- | ------- | ----------- |
| `name` | `[:0]const u8`  | —       | The name    |
| `url`  | `[:0]const u8?` | `null`  | Url         |

---

#### OpenApiConfig

OpenAPI configuration

| Field             | Type                                    | Default   | Description                                                      |
| ----------------- | --------------------------------------- | --------- | ---------------------------------------------------------------- |
| `enabled`         | `bool`                                  | `false`   | Enable OpenAPI generation (default: false for zero overhead)     |
| `title`           | `[:0]const u8`                          | `"API"`   | API title                                                        |
| `version`         | `[:0]const u8`                          | `"1.0.0"` | API version                                                      |
| `description`     | `[:0]const u8?`                         | `null`    | API description (supports markdown)                              |
| `swaggerUiPath`   | `[:0]const u8`                          | —         | Path to serve Swagger UI (default: "/docs")                      |
| `redocPath`       | `[:0]const u8`                          | —         | Path to serve Redoc (default: "/redoc")                          |
| `openapiJsonPath` | `[:0]const u8`                          | —         | Path to serve OpenAPI JSON spec (default: "/openapi.json")       |
| `contact`         | `ContactInfo?`                          | `null`    | Contact information                                              |
| `license`         | `LicenseInfo?`                          | `null`    | License information                                              |
| `servers`         | `[]const ServerInfo`                    | `[]`      | Server definitions                                               |
| `securitySchemes` | `std.StringHashMap(SecuritySchemeInfo)` | `{}`      | Security schemes (auto-detected from middleware if not provided) |

##### Methods

###### default()

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

---

#### ParseRequest

Request body for `POST /asyncapi/parse`

| Field  | Type           | Default | Description |
| ------ | -------------- | ------- | ----------- |
| `spec` | `[:0]const u8` | —       | Spec        |

---

#### ParseResult

Full parse result returned by `POST /asyncapi/parse`

| Field         | Type                      | Default | Description  |
| ------------- | ------------------------- | ------- | ------------ |
| `specVersion` | `[:0]const u8`            | —       | Spec version |
| `title`       | `[:0]const u8`            | —       | Title        |
| `apiVersion`  | `[:0]const u8`            | —       | Api version  |
| `channels`    | `[]const ParsedChannel`   | —       | Channels     |
| `operations`  | `[]const ParsedOperation` | —       | Operations   |
| `messages`    | `[]const ParsedMessage`   | —       | Messages     |

---

#### ParsedChannel

A single channel extracted from an AsyncAPI spec

| Field      | Type                   | Default | Description                                                           |
| ---------- | ---------------------- | ------- | --------------------------------------------------------------------- |
| `name`     | `[:0]const u8`         | —       | Channel key from the spec (e.g. "chat/messages")                      |
| `address`  | `[:0]const u8`         | —       | Channel address / path                                                |
| `messages` | `[]const [:0]const u8` | —       | Message names declared on this channel                                |
| `bindings` | `[:0]const u8?`        | `null`  | Bindings (ws / http / amqp / …) as raw JSON for forward-compatibility |

---

#### ParsedMessage

A resolved message (name + JSON Schema)

| Field    | Type            | Default | Description                                                |
| -------- | --------------- | ------- | ---------------------------------------------------------- |
| `name`   | `[:0]const u8`  | —       | Message name                                               |
| `schema` | `[:0]const u8?` | `null`  | Resolved JSON Schema for the message payload, if available |

---

#### ParsedOperation

A single operation extracted from an AsyncAPI spec

| Field     | Type           | Default | Description                                      |
| --------- | -------------- | ------- | ------------------------------------------------ |
| `name`    | `[:0]const u8` | —       | Operation name                                   |
| `action`  | `[:0]const u8` | —       | Operation action: "send" or "receive"            |
| `channel` | `[:0]const u8` | —       | Channel reference (resolved to the channel name) |

---

#### ProblemDetails

RFC 9457 Problem Details for HTTP APIs

A machine-readable format for specifying errors in HTTP API responses.
Per RFC 9457, all fields are optional. The `type` field defaults to "about:blank"
if not specified.

# Content-Type

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

| Field        | Type                              | Default | Description                                                                                                                                                  |
| ------------ | --------------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `typeUri`    | `[:0]const u8`                    | —       | A URI reference that identifies the problem type. Defaults to "about:blank" when absent. Should be a stable, human-readable identifier for the problem type. |
| `title`      | `[:0]const u8`                    | —       | A short, human-readable summary of the problem type. Should not change from occurrence to occurrence of the problem.                                         |
| `status`     | `u16`                             | —       | The HTTP status code generated by the origin server. This is advisory; the actual HTTP status code takes precedence.                                         |
| `detail`     | `[:0]const u8?`                   | `null`  | A human-readable explanation specific to this occurrence of the problem.                                                                                     |
| `instance`   | `[:0]const u8?`                   | `null`  | A URI reference that identifies the specific occurrence of the problem. It may or may not yield further information if dereferenced.                         |
| `extensions` | `std.StringHashMap([:0]const u8)` | —       | Extension members - problem-type-specific data. For validation errors, this typically contains an "errors" array.                                            |

##### Methods

###### withDetail()

Set the detail field

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### withInstance()

Set the instance field

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### notFound()

Create a not found error

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### methodNotAllowed()

Create a method not allowed error

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### internalServerError()

Create an internal server error

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### badRequest()

Create a bad request error

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### toJson()

Serialize to JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### toJsonPretty()

Serialize to pretty JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

---

#### QueryMutationConfig

Configuration for schemas with Query and Mutation types

| Field                  | Type   | Default | Description                                 |
| ---------------------- | ------ | ------- | ------------------------------------------- |
| `introspectionEnabled` | `bool` | `true`  | Enable introspection queries                |
| `complexityLimit`      | `u64?` | `null`  | Maximum query complexity (None = unlimited) |
| `depthLimit`           | `u64?` | `null`  | Maximum query depth (None = unlimited)      |

##### Methods

###### default()

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

---

#### QueryOnlyConfig

Configuration for schemas with only Query type

| Field                  | Type   | Default | Description                                 |
| ---------------------- | ------ | ------- | ------------------------------------------- |
| `introspectionEnabled` | `bool` | `true`  | Enable introspection queries                |
| `complexityLimit`      | `u64?` | `null`  | Maximum query complexity (None = unlimited) |
| `depthLimit`           | `u64?` | `null`  | Maximum query depth (None = unlimited)      |

##### Methods

###### default()

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

---

#### RateLimitConfig

Rate limiting configuration shared across runtimes

| Field       | Type   | Default | Description                |
| ----------- | ------ | ------- | -------------------------- |
| `perSecond` | `u64`  | `100`   | Requests per second        |
| `burst`     | `u32`  | `200`   | Burst allowance            |
| `ipBased`   | `bool` | `true`  | Use IP-based rate limiting |

##### Methods

###### default()

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

---

#### Response

HTTP Response with custom status code, headers, and content

| Field        | Type                              | Default | Description                        |
| ------------ | --------------------------------- | ------- | ---------------------------------- |
| `content`    | `[:0]const u8?`                   | `null`  | Response body content              |
| `statusCode` | `u16`                             | —       | HTTP status code (defaults to 200) |
| `headers`    | `std.StringHashMap([:0]const u8)` | `{}`    | Response headers                   |

##### Methods

###### setHeader()

Set a header

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### setCookie()

Set a cookie in the response

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### default()

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

---

#### ResponseSnapshot

Snapshot of an Axum response used by higher-level language bindings.

| Field     | Type                              | Default | Description                                                |
| --------- | --------------------------------- | ------- | ---------------------------------------------------------- |
| `status`  | `u16`                             | —       | HTTP status code.                                          |
| `headers` | `std.StringHashMap([:0]const u8)` | —       | Response headers (lowercase keys for predictable lookups). |
| `body`    | `[]const u8`                      | —       | Response body bytes (decoded for supported encodings).     |

##### Methods

###### text()

Return response body as UTF-8 string.

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### json()

Parse response body as JSON.

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### header()

Lookup header by case-insensitive name.

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### graphqlData()

Extract GraphQL data from response

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### graphqlErrors()

Extract GraphQL errors from response

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

---

#### SchemaConfig

Configuration for GraphQL schema building.

Encapsulates all schema-level configuration options including
introspection control, complexity limits, and depth limits.

| Field                  | Type   | Default | Description                                 |
| ---------------------- | ------ | ------- | ------------------------------------------- |
| `introspectionEnabled` | `bool` | `true`  | Enable introspection queries                |
| `complexityLimit`      | `u64?` | `null`  | Maximum query complexity (None = unlimited) |
| `depthLimit`           | `u64?` | `null`  | Maximum query depth (None = unlimited)      |

##### Methods

###### default()

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

---

#### ServerConfig

Server configuration

| Field              | Type                        | Default       | Description                                                                    |
| ------------------ | --------------------------- | ------------- | ------------------------------------------------------------------------------ |
| `host`             | `[:0]const u8`              | `"127.0.0.1"` | Host to bind to                                                                |
| `port`             | `u16`                       | `8000`        | Port to bind to                                                                |
| `workers`          | `u64`                       | `1`           | Number of Tokio runtime worker threads used by binding-managed server runtimes |
| `enableRequestId`  | `bool`                      | `false`       | Enable request ID generation and propagation                                   |
| `maxBodySize`      | `u64?`                      | `null`        | Maximum request body size in bytes (None = unlimited, not recommended)         |
| `requestTimeout`   | `u64?`                      | `null`        | Request timeout in seconds (None = no timeout)                                 |
| `compression`      | `CompressionConfig?`        | `null`        | Enable compression middleware                                                  |
| `rateLimit`        | `RateLimitConfig?`          | `null`        | Enable rate limiting                                                           |
| `jwtAuth`          | `JwtConfig?`                | `null`        | JWT authentication configuration                                               |
| `apiKeyAuth`       | `ApiKeyConfig?`             | `null`        | API Key authentication configuration                                           |
| `staticFiles`      | `[]const StaticFilesConfig` | `[]`          | Static file serving configuration                                              |
| `gracefulShutdown` | `bool`                      | `true`        | Enable graceful shutdown on SIGTERM/SIGINT                                     |
| `shutdownTimeout`  | `u64`                       | `30`          | Graceful shutdown timeout (seconds)                                            |
| `asyncapi`         | `AsyncApiConfig?`           | `null`        | AsyncAPI HTTP endpoint configuration                                           |
| `openapi`          | `OpenApiConfig?`            | `null`        | OpenAPI documentation configuration                                            |
| `jsonrpc`          | `JsonRpcConfig?`            | `null`        | JSON-RPC configuration                                                         |
| `grpc`             | `GrpcConfig?`               | `null`        | gRPC configuration                                                             |
| `backgroundTasks`  | `BackgroundTaskConfig`      | —             | Background task executor configuration                                         |
| `enableHttpTrace`  | `bool`                      | `false`       | Enable per-request HTTP tracing (tower-http `TraceLayer`)                      |

##### Methods

###### default()

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

---

#### ServerInfo

Server information

| Field         | Type            | Default | Description                |
| ------------- | --------------- | ------- | -------------------------- |
| `url`         | `[:0]const u8`  | —       | Url                        |
| `description` | `[:0]const u8?` | `null`  | Human-readable description |

---

#### SseEvent

An individual SSE event

Represents a single Server-Sent Event to be sent to a connected client.
Events can have an optional type, ID, and retry timeout for advanced scenarios.

# SSE Format

Events are serialized to the following text format:

```text
event: event_type
data: {"json":"value"}
id: event-123
retry: 3000
```

| Field       | Type            | Default | Description                                       |
| ----------- | --------------- | ------- | ------------------------------------------------- |
| `eventType` | `[:0]const u8?` | `null`  | Event type (optional)                             |
| `data`      | `[:0]const u8`  | —       | Event data (JSON value)                           |
| `id`        | `[:0]const u8?` | `null`  | Event ID (optional, for client-side reconnection) |
| `retry`     | `u64?`          | `null`  | Retry timeout in milliseconds (optional)          |

##### Methods

###### withId()

Set the event ID for client-side reconnection support

Sets an ID that clients can use to resume from this point if they disconnect.
The client sends this ID back in the `Last-Event-ID` header when reconnecting.

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### withRetry()

Set the retry timeout for client reconnection

Sets the time in milliseconds clients should wait before attempting to reconnect
if the connection is lost. The client browser will automatically handle reconnection.

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

---

#### StaticFilesConfig

Static file serving configuration

| Field          | Type            | Default | Description                            |
| -------------- | --------------- | ------- | -------------------------------------- |
| `directory`    | `[:0]const u8`  | —       | Directory path to serve                |
| `routePrefix`  | `[:0]const u8`  | —       | URL path prefix (e.g., "/static")      |
| `indexFile`    | `bool`          | —       | Fallback to index.html for directories |
| `cacheControl` | `[:0]const u8?` | `null`  | Cache-Control header value             |

---

#### TestClient

Core test client for making HTTP requests to a Spikard application.

This struct wraps axum-test's TestServer and provides a language-agnostic
interface for making HTTP requests, sending WebSocket connections, and
handling Server-Sent Events. Language bindings wrap this to provide
native API surfaces.

##### Methods

###### get()

Make a GET request

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### post()

Make a POST request

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### requestRaw()

Make a request with a raw body payload.

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### put()

Make a PUT request

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### patch()

Make a PATCH request

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### delete()

Make a DELETE request

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### options()

Make an OPTIONS request

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### head()

Make a HEAD request

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### trace()

Make a TRACE request

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### graphqlAt()

Send a GraphQL query/mutation to a custom endpoint

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### graphql()

Send a GraphQL query/mutation

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### graphqlWithStatus()

Send a GraphQL query and return HTTP status code separately

This method allows tests to distinguish between:

- HTTP-level errors (400/422 for invalid requests)
- GraphQL-level errors (200 with errors in response body)

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### graphqlSubscriptionAt()

Send a GraphQL subscription (WebSocket) to a custom endpoint.

Uses the `graphql-transport-ws` protocol and captures the first `next` payload.
After the first payload is received, this client sends `complete` to unsubscribe.

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### graphqlSubscription()

Send a GraphQL subscription (WebSocket).

Uses `/graphql` as the default subscription endpoint.

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

---

#### UploadFile

Represents an uploaded file from multipart/form-data requests.

This struct provides efficient access to file content with automatic
base64 decoding and implements standard I/O traits for compatibility.

| Field             | Type            | Default | Description                              |
| ----------------- | --------------- | ------- | ---------------------------------------- |
| `filename`        | `[:0]const u8`  | —       | Original filename from the client        |
| `contentType`     | `[:0]const u8?` | `null`  | MIME type of the uploaded file           |
| `size`            | `u64?`          | `null`  | Size of the file in bytes                |
| `content`         | `[]const u8`    | —       | File content (may be base64 encoded)     |
| `contentEncoding` | `[:0]const u8?` | `null`  | Content encoding type                    |
| `cursor`          | `[:0]const u8`  | —       | Internal cursor for Read/Seek operations |

##### Methods

###### asBytes()

Get the raw file content as bytes.

This provides zero-copy access to the underlying buffer.

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### readToString()

Read the file content as a UTF-8 string.

**Errors:**

Returns an error if the content is not valid UTF-8.

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### contentTypeOrDefault()

Get the content type, defaulting to "application/octet-stream".

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

---

#### ValidateRequest

Request body for `POST /asyncapi/validate`

| Field     | Type           | Default | Description |
| --------- | -------------- | ------- | ----------- |
| `spec`    | `[:0]const u8` | —       | Spec        |
| `channel` | `[:0]const u8` | —       | Channel     |
| `message` | `[:0]const u8` | —       | Message     |
| `payload` | `[:0]const u8` | —       | Payload     |

---

#### ValidationResponse

Response body for `POST /asyncapi/validate`

| Field    | Type                   | Default | Description |
| -------- | ---------------------- | ------- | ----------- |
| `valid`  | `bool`                 | —       | Valid       |
| `errors` | `[]const [:0]const u8` | —       | Errors      |

---

### Enums

#### SnapshotError

Possible errors while converting an Axum response into a snapshot.

| Value           | Description                                                                  |
| --------------- | ---------------------------------------------------------------------------- |
| `InvalidHeader` | Response header could not be decoded to UTF-8. — Fields: `0`: `[:0]const u8` |
| `Decompression` | Body decompression failed. — Fields: `0`: `[:0]const u8`                     |

---

#### WebSocketMessage

A WebSocket message that can be text or binary.

| Value    | Description                                                                                                                                                                                                                          |
| -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Text`   | A text message. — Fields: `0`: `[:0]const u8`                                                                                                                                                                                        |
| `Binary` | A binary message. — Fields: `0`: `[]const u8`                                                                                                                                                                                        |
| `Close`  | A close message with a numeric close code (RFC 6455) and optional reason text. Common codes: 1000 Normal Closure, 1001 Going Away, 1005 No Status Received, 1006 Abnormal Closure. — Fields: `code`: `u16`, `reason`: `[:0]const u8` |
| `Ping`   | A ping message. — Fields: `0`: `[]const u8`                                                                                                                                                                                          |
| `Pong`   | A pong message. — Fields: `0`: `[]const u8`                                                                                                                                                                                          |

---

#### Method

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

#### SecuritySchemeInfo

Security scheme types

| Value    | Description                                                             |
| -------- | ----------------------------------------------------------------------- |
| `Http`   | Http — Fields: `scheme`: `[:0]const u8`, `bearerFormat`: `[:0]const u8` |
| `ApiKey` | Api key — Fields: `location`: `[:0]const u8`, `name`: `[:0]const u8`    |

---

### Errors

#### GraphQlError

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

#### SchemaError

Error type for schema building operations

| Variant                   | Description                    |
| ------------------------- | ------------------------------ |
| `BuildingFailed`          | Generic schema building error  |
| `ValidationError`         | Configuration validation error |
| `ComplexityLimitExceeded` | Complexity limit exceeded      |
| `DepthLimitExceeded`      | Depth limit exceeded           |

---

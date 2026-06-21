---
title: "TypeScript API Reference"
---

## TypeScript API Reference <span class="version-badge">v0.16.0-rc.3</span>

### Functions

#### schemaQueryOnly()

Create a simple schema configuration with only Query type.

This is a convenience function for schemas that only have queries.

**Returns:**

A `QueryOnlyConfig` with default settings

**Signature:**

```typescript
function schemaQueryOnly(): QueryOnlyConfig
```

**Example:**

```typescript
const result = schemaQueryOnly();
```

**Returns:** `QueryOnlyConfig`

---

#### schemaQueryMutation()

Create a schema configuration with Query and Mutation types.

This is a convenience function for schemas with queries and mutations but no subscriptions.

**Returns:**

A `QueryMutationConfig` with default settings

**Signature:**

```typescript
function schemaQueryMutation(): QueryMutationConfig
```

**Example:**

```typescript
const result = schemaQueryMutation();
```

**Returns:** `QueryMutationConfig`

---

#### schemaFull()

Create a schema configuration with all three root types.

This is a convenience function for fully-featured schemas.

**Returns:**

A `FullSchemaConfig` with default settings

**Signature:**

```typescript
function schemaFull(): FullSchemaConfig
```

**Example:**

```typescript
const result = schemaFull();
```

**Returns:** `FullSchemaConfig`

---

### Types

#### ApiKeyConfig

API Key authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `keys` | `Array<string>` | — | Valid API keys |
| `headerName` | `string` | `serde(default = "default_api_key_header")` | Header name to check (e.g., "X-API-Key") |

---

#### AsyncApiConfig

AsyncAPI HTTP endpoint configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `boolean` | — | Enable AsyncAPI endpoints (default: false) |
| `spec` | `unknown \| null` | `null` | Pre-registered AsyncAPI spec to serve from GET /asyncapi.json |

---

#### BackgroundJobMetadata

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | The name |
| `requestId` | `string \| null` | `null` | Request id |

##### Methods

###### default()

**Signature:**

```typescript
static default(): BackgroundJobMetadata
```

**Example:**

```typescript
const result = BackgroundJobMetadata.default();
```

**Returns:** `BackgroundJobMetadata`

---

#### BackgroundTaskConfig

Configuration for in-process background task execution.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `maxQueueSize` | `number` | `1024` | Maximum queue size |
| `maxConcurrentTasks` | `number` | `128` | Maximum concurrent tasks |
| `drainTimeoutSecs` | `number` | `30` | Drain timeout secs |

##### Methods

###### default()

**Signature:**

```typescript
static default(): BackgroundTaskConfig
```

**Example:**

```typescript
const result = BackgroundTaskConfig.default();
```

**Returns:** `BackgroundTaskConfig`

---

#### CompressionConfig

Compression configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `gzip` | `boolean` | `true` | Enable gzip compression |
| `brotli` | `boolean` | `true` | Enable brotli compression |
| `minSize` | `number` | — | Minimum response size to compress (bytes) |
| `quality` | `number` | — | Compression quality (0-11 for brotli, 0-9 for gzip) |

##### Methods

###### default()

**Signature:**

```typescript
static default(): CompressionConfig
```

**Example:**

```typescript
const result = CompressionConfig.default();
```

**Returns:** `CompressionConfig`

---

#### ContactInfo

Contact information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string \| null` | `null` | Name of the contact person or organisation. |
| `email` | `string \| null` | `null` | Contact email address. |
| `url` | `string \| null` | `null` | URL pointing to the contact information page. |

---

#### CorsConfig

CORS configuration for a route

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `allowedOrigins` | `Array<string>` | `\[\]` | Allowed origins |
| `allowedMethods` | `Array<string>` | `\[\]` | Allowed methods |
| `allowedHeaders` | `Array<string>` | `\[\]` | Allowed headers |
| `exposeHeaders` | `Array<string> \| null` | `null` | Expose headers |
| `maxAge` | `number \| null` | `null` | Maximum age |
| `allowCredentials` | `boolean \| null` | `null` | Allow credentials |

##### Methods

###### allowedMethodsJoined()

Get the cached joined methods string for preflight responses

**Signature:**

```typescript
allowedMethodsJoined(): string
```

**Example:**

```typescript
const result = instance.allowedMethodsJoined();
```

**Returns:** `string`

###### allowedHeadersJoined()

Get the cached joined headers string for preflight responses

**Signature:**

```typescript
allowedHeadersJoined(): string
```

**Example:**

```typescript
const result = instance.allowedHeadersJoined();
```

**Returns:** `string`

###### isOriginAllowed()

Check if an origin is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```typescript
isOriginAllowed(origin: string): boolean
```

**Example:**

```typescript
const result = instance.isOriginAllowed("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `origin` | `string` | Yes | The origin |

**Returns:** `boolean`

###### isMethodAllowed()

Check if a method is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```typescript
isMethodAllowed(method: string): boolean
```

**Example:**

```typescript
const result = instance.isMethodAllowed("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `method` | `string` | Yes | The method |

**Returns:** `boolean`

###### default()

**Signature:**

```typescript
static default(): CorsConfig
```

**Example:**

```typescript
const result = CorsConfig.default();
```

**Returns:** `CorsConfig`

---

#### FullSchemaConfig

Configuration for fully-featured schemas with Query, Mutation, and Subscription types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `boolean` | `true` | Enable introspection queries |
| `complexityLimit` | `number \| null` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `number \| null` | `null` | Maximum query depth (None = unlimited) |

##### Methods

###### default()

**Signature:**

```typescript
static default(): FullSchemaConfig
```

**Example:**

```typescript
const result = FullSchemaConfig.default();
```

**Returns:** `FullSchemaConfig`

---

#### GraphQlRouteConfig

Configuration for GraphQL routes

Provides a builder pattern for configuring GraphQL route parameters
for the Spikard HTTP server's routing system.

##### Methods

###### new()

Create a new GraphQL route configuration with defaults

Default values:

- path: "/graphql"
- method: "POST"
- `enable_playground`: false

**Signature:**

```typescript
static new(): GraphQlRouteConfig
```

**Example:**

```typescript
const result = GraphQlRouteConfig.new();
```

**Returns:** `GraphQlRouteConfig`

###### path()

Set the HTTP path for the GraphQL endpoint

**Signature:**

```typescript
path(path: string): GraphQlRouteConfig
```

**Example:**

```typescript
const result = instance.path("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `string` | Yes | The URL path (e.g., "/graphql", "/api/graphql") |

**Returns:** `GraphQlRouteConfig`

###### method()

Set the HTTP method for the GraphQL endpoint

**Signature:**

```typescript
method(method: string): GraphQlRouteConfig
```

**Example:**

```typescript
const result = instance.method("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `method` | `string` | Yes | The HTTP method (typically "POST") |

**Returns:** `GraphQlRouteConfig`

###### enablePlayground()

Enable or disable the GraphQL Playground UI

**Signature:**

```typescript
enablePlayground(enable: boolean): GraphQlRouteConfig
```

**Example:**

```typescript
const result = instance.enablePlayground(true);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `enable` | `boolean` | Yes | Whether to enable playground |

**Returns:** `GraphQlRouteConfig`

###### description()

Set a custom description for documentation

**Signature:**

```typescript
description(description: string): GraphQlRouteConfig
```

**Example:**

```typescript
const result = instance.description("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `description` | `string` | Yes | Documentation string |

**Returns:** `GraphQlRouteConfig`

###### getPath()

Get the configured path

**Signature:**

```typescript
getPath(): string
```

**Example:**

```typescript
const result = instance.getPath();
```

**Returns:** `string`

###### getMethod()

Get the configured method

**Signature:**

```typescript
getMethod(): string
```

**Example:**

```typescript
const result = instance.getMethod();
```

**Returns:** `string`

###### isPlaygroundEnabled()

Check if playground is enabled

**Signature:**

```typescript
isPlaygroundEnabled(): boolean
```

**Example:**

```typescript
const result = instance.isPlaygroundEnabled();
```

**Returns:** `boolean`

###### getDescription()

Get the description if set

**Signature:**

```typescript
getDescription(): string | null
```

**Example:**

```typescript
const result = instance.getDescription();
```

**Returns:** `string | null`

###### default()

**Signature:**

```typescript
static default(): GraphQlRouteConfig
```

**Example:**

```typescript
const result = GraphQlRouteConfig.default();
```

**Returns:** `GraphQlRouteConfig`

---

#### GraphQlSubscriptionSnapshot

Snapshot of a GraphQL subscription exchange over WebSocket.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `operationId` | `string` | — | Operation id used for the subscription request. |
| `acknowledged` | `boolean` | — | Whether the server acknowledged the GraphQL WebSocket connection. |
| `event` | `unknown \| null` | `null` | First `next.payload` received for this subscription, if any. |
| `errors` | `Array<unknown>` | — | GraphQL protocol errors emitted by the server. |
| `completeReceived` | `boolean` | — | Whether a `complete` frame was observed for this operation. |

---

#### GrpcConfig

Configuration for gRPC support

Controls how the server handles gRPC requests, including compression,
timeouts, and protocol settings.

##### Stream Limits

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

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `boolean` | `true` | Enable gRPC support |
| `maxMessageSize` | `number` | — | Maximum message size in bytes (for both sending and receiving) This limit applies to individual messages in both unary and streaming RPCs. When a single message exceeds this size, the request is rejected with HTTP 413 (Payload Too Large). Default: 4MB (4194304 bytes) **Note:** This limit does NOT apply to the total response size in streaming RPCs. For multi-message streams, the total response can exceed this limit as long as each individual message stays within the limit. |
| `enableCompression` | `boolean` | `true` | Enable gzip compression for gRPC messages |
| `requestTimeout` | `number \| null` | `null` | Timeout for gRPC requests in seconds (None = no timeout) |
| `maxConcurrentStreams` | `number` | — | Maximum number of concurrent streams per connection (HTTP/2 advisory) This value is communicated to HTTP/2 clients as the server's flow control limit. The HTTP/2 transport layer enforces this limit automatically via SETTINGS frames and GOAWAY responses. Applications should NOT implement custom enforcement. Default: 100 streams per connection # Stream Limiting Strategy - **Per Connection**: This limit applies per HTTP/2 connection, not globally - **Transport Enforcement**: HTTP/2 handles all stream limiting; applications need not implement custom checks - **Streaming Requests**: In server streaming or bidi streaming, each logical RPC consumes one stream slot. Message ordering within a stream follows HTTP/2 frame ordering. |
| `enableKeepalive` | `boolean` | `true` | Enable HTTP/2 keepalive |
| `keepaliveInterval` | `number` | — | HTTP/2 keepalive interval in seconds |
| `keepaliveTimeout` | `number` | — | HTTP/2 keepalive timeout in seconds |
| `maxStreamResponseBytes` | `number \| null` | `null` | Total byte cap across an entire streaming response. When `Some(n)`, the streaming adapter aborts the stream with `tonic.Status.resource_exhausted` once the cumulative encoded message bytes exceed `n`. The stream yields the error item and then terminates. Per-message cap remains `max_message_size`. This limit applies to server-streaming and bidirectional-streaming RPCs only; unary RPCs are governed solely by `max_message_size`. Default: `null` (unbounded total response size). |

##### Methods

###### default()

**Signature:**

```typescript
static default(): GrpcConfig
```

**Example:**

```typescript
const result = GrpcConfig.default();
```

**Returns:** `GrpcConfig`

---

#### IntoHandler

Convert user-facing handler functions into the low-level `Handler` trait.

##### Methods

###### intoHandler()

Convert this value into a shared request handler.

**Signature:**

```typescript
intoHandler(): Handler
```

**Example:**

```typescript
const result = instance.intoHandler();
```

**Returns:** `Handler`

---

#### JsonRpcConfig

JSON-RPC server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `boolean` | `true` | Enable JSON-RPC endpoint |
| `endpointPath` | `string` | — | HTTP endpoint path for JSON-RPC requests (default: "/rpc") |
| `enableBatch` | `boolean` | — | Enable batch request processing (default: true) |
| `maxBatchSize` | `number` | — | Maximum number of requests in a batch (default: 100) |

##### Methods

###### default()

**Signature:**

```typescript
static default(): JsonRpcConfig
```

**Example:**

```typescript
const result = JsonRpcConfig.default();
```

**Returns:** `JsonRpcConfig`

---

#### JsonRpcMethodInfo

JSON-RPC method metadata for routes that support JSON-RPC

This struct captures the metadata needed to expose HTTP routes as JSON-RPC methods,
enabling discovery and documentation of RPC-compatible endpoints.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `methodName` | `string` | — | The JSON-RPC method name (e.g., "user.create") |
| `description` | `string \| null` | `null` | Optional description of what the method does |
| `paramsSchema` | `unknown \| null` | `null` | Optional JSON Schema for method parameters |
| `resultSchema` | `unknown \| null` | `null` | Optional JSON Schema for the result |
| `deprecated` | `boolean` | `/* serde(default) */` | Whether this method is deprecated |
| `tags` | `Array<string>` | `/* serde(default) */` | Tags for categorizing and grouping methods |

---

#### JwtConfig

JWT authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `secret` | `string` | — | Secret key for JWT verification |
| `algorithm` | `string` | `serde(default = "default_jwt_algorithm")` | Required algorithm (HS256, HS384, HS512, RS256, etc.) |
| `audience` | `Array<string> \| null` | `null` | Required audience claim |
| `issuer` | `string \| null` | `null` | Required issuer claim |
| `leeway` | `number` | `/* serde(default) */` | Leeway for expiration checks (seconds) |

---

#### LicenseInfo

License information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | SPDX license identifier or display name (e.g. `"MIT"`). |
| `url` | `string \| null` | `null` | URL to the full license text. |

---

#### OpenApiConfig

OpenAPI configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `boolean` | `false` | Enable OpenAPI generation (default: false for zero overhead) |
| `title` | `string` | `"API"` | API title |
| `version` | `string` | `"1.0.0"` | API version |
| `description` | `string \| null` | `null` | API description (supports markdown) |
| `swaggerUiPath` | `string` | — | Path to serve Swagger UI (default: "/docs") |
| `redocPath` | `string` | — | Path to serve Redoc (default: "/redoc") |
| `openapiJsonPath` | `string` | — | Path to serve OpenAPI JSON spec (default: "/openapi.json") |
| `contact` | `ContactInfo \| null` | `null` | Contact information |
| `license` | `LicenseInfo \| null` | `null` | License information |
| `servers` | `Array<ServerInfo>` | `\[\]` | Server definitions |
| `securitySchemes` | `Record<string, SecuritySchemeInfo>` | `{}` | Security schemes (auto-detected from middleware if not provided) |

##### Methods

###### default()

**Signature:**

```typescript
static default(): OpenApiConfig
```

**Example:**

```typescript
const result = OpenApiConfig.default();
```

**Returns:** `OpenApiConfig`

---

#### ParseRequest

Request body for `POST /asyncapi/parse`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `spec` | `unknown` | — | Spec |

---

#### ParseResult

Full parse result returned by `POST /asyncapi/parse`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `specVersion` | `string` | — | Spec version |
| `title` | `string` | — | Title |
| `apiVersion` | `string` | — | Api version |
| `channels` | `Array<ParsedChannel>` | — | Channels |
| `operations` | `Array<ParsedOperation>` | — | Operations |
| `messages` | `Array<ParsedMessage>` | — | Messages |

---

#### ParsedChannel

A single channel extracted from an AsyncAPI spec

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | Channel key from the spec (e.g. "chat/messages") |
| `address` | `string` | — | Channel address / path |
| `messages` | `Array<string>` | — | Message names declared on this channel |
| `bindings` | `unknown \| null` | `null` | Bindings (ws / http / amqp / …) as raw JSON for forward-compatibility |

---

#### ParsedMessage

A resolved message (name + JSON Schema)

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | Message name |
| `schema` | `unknown \| null` | `null` | Resolved JSON Schema for the message payload, if available |

---

#### ParsedOperation

A single operation extracted from an AsyncAPI spec

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | Operation name |
| `action` | `string` | — | Operation action: "send" or "receive" |
| `channel` | `string` | — | Channel reference (resolved to the channel name) |

---

#### ProblemDetails

RFC 9457 Problem Details for HTTP APIs

A machine-readable format for specifying errors in HTTP API responses.
Per RFC 9457, all fields are optional. The `type` field defaults to "about:blank"
if not specified.

##### Content-Type

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

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `typeUri` | `string` | — | A URI reference that identifies the problem type. Defaults to "about:blank" when absent. Should be a stable, human-readable identifier for the problem type. |
| `title` | `string` | — | A short, human-readable summary of the problem type. Should not change from occurrence to occurrence of the problem. |
| `status` | `number` | — | The HTTP status code generated by the origin server. This is advisory; the actual HTTP status code takes precedence. |
| `detail` | `string \| null` | `null` | A human-readable explanation specific to this occurrence of the problem. |
| `instance` | `string \| null` | `null` | A URI reference that identifies the specific occurrence of the problem. It may or may not yield further information if dereferenced. |
| `extensions` | `Record<string, unknown>` | — | Extension members - problem-type-specific data. For validation errors, this typically contains an "errors" array. |

##### Methods

###### withDetail()

Set the detail field

**Signature:**

```typescript
withDetail(detail: string): ProblemDetails
```

**Example:**

```typescript
const result = instance.withDetail("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `detail` | `string` | Yes | The detail |

**Returns:** `ProblemDetails`

###### withInstance()

Set the instance field

**Signature:**

```typescript
withInstance(instance: string): ProblemDetails
```

**Example:**

```typescript
const result = instance.withInstance("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `instance` | `string` | Yes | The instance |

**Returns:** `ProblemDetails`

###### notFound()

Create a not found error

**Signature:**

```typescript
static notFound(detail: string): ProblemDetails
```

**Example:**

```typescript
const result = ProblemDetails.notFound("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `detail` | `string` | Yes | The detail |

**Returns:** `ProblemDetails`

###### methodNotAllowed()

Create a method not allowed error

**Signature:**

```typescript
static methodNotAllowed(detail: string): ProblemDetails
```

**Example:**

```typescript
const result = ProblemDetails.methodNotAllowed("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `detail` | `string` | Yes | The detail |

**Returns:** `ProblemDetails`

###### internalServerError()

Create an internal server error

**Signature:**

```typescript
static internalServerError(detail: string): ProblemDetails
```

**Example:**

```typescript
const result = ProblemDetails.internalServerError("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `detail` | `string` | Yes | The detail |

**Returns:** `ProblemDetails`

###### badRequest()

Create a bad request error

**Signature:**

```typescript
static badRequest(detail: string): ProblemDetails
```

**Example:**

```typescript
const result = ProblemDetails.badRequest("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `detail` | `string` | Yes | The detail |

**Returns:** `ProblemDetails`

###### toJson()

Serialize to JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```typescript
toJson(): string
```

**Example:**

```typescript
const result = instance.toJson();
```

**Returns:** `string`

**Errors:** Throws `Error` with a descriptive message.

###### toJsonPretty()

Serialize to pretty JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```typescript
toJsonPretty(): string
```

**Example:**

```typescript
const result = instance.toJsonPretty();
```

**Returns:** `string`

**Errors:** Throws `Error` with a descriptive message.

---

#### QueryMutationConfig

Configuration for schemas with Query and Mutation types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `boolean` | `true` | Enable introspection queries |
| `complexityLimit` | `number \| null` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `number \| null` | `null` | Maximum query depth (None = unlimited) |

##### Methods

###### default()

**Signature:**

```typescript
static default(): QueryMutationConfig
```

**Example:**

```typescript
const result = QueryMutationConfig.default();
```

**Returns:** `QueryMutationConfig`

---

#### QueryOnlyConfig

Configuration for schemas with only Query type

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `boolean` | `true` | Enable introspection queries |
| `complexityLimit` | `number \| null` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `number \| null` | `null` | Maximum query depth (None = unlimited) |

##### Methods

###### default()

**Signature:**

```typescript
static default(): QueryOnlyConfig
```

**Example:**

```typescript
const result = QueryOnlyConfig.default();
```

**Returns:** `QueryOnlyConfig`

---

#### RateLimitConfig

Rate limiting configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `perSecond` | `number` | `100` | Requests per second |
| `burst` | `number` | `200` | Burst allowance |
| `ipBased` | `boolean` | `true` | Use IP-based rate limiting |

##### Methods

###### default()

**Signature:**

```typescript
static default(): RateLimitConfig
```

**Example:**

```typescript
const result = RateLimitConfig.default();
```

**Returns:** `RateLimitConfig`

---

#### Request

---

#### Response

HTTP Response with custom status code, headers, and content

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `unknown \| null` | `null` | Response body content |
| `statusCode` | `number` | — | HTTP status code (defaults to 200) |
| `headers` | `Record<string, string>` | `{}` | Response headers |

##### Methods

###### setHeader()

Set a header

**Signature:**

```typescript
setHeader(key: string, value: string): void
```

**Example:**

```typescript
instance.setHeader("value", "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `key` | `string` | Yes | The key |
| `value` | `string` | Yes | The value |

**Returns:** No return value.

###### setCookie()

Set a cookie in the response

**Signature:**

```typescript
setCookie(key: string, value: string, secure: boolean, httpOnly: boolean, maxAge: number, domain: string, path: string, sameSite: string): void
```

**Example:**

```typescript
instance.setCookie("value", "value", true, true, 42, "value", "value", "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `key` | `string` | Yes | The key |
| `value` | `string` | Yes | The value |
| `secure` | `boolean` | Yes | The secure |
| `httpOnly` | `boolean` | Yes | The http only |
| `maxAge` | `number \| null` | No | The max age |
| `domain` | `string \| null` | No | The domain |
| `path` | `string \| null` | No | Path to the file |
| `sameSite` | `string \| null` | No | The same site |

**Returns:** No return value.

###### default()

**Signature:**

```typescript
static default(): Response
```

**Example:**

```typescript
const result = Response.default();
```

**Returns:** `Response`

---

#### ResponseSnapshot

Snapshot of an Axum response used by higher-level language bindings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `status` | `number` | — | HTTP status code. |
| `headers` | `Record<string, string>` | — | Response headers (lowercase keys for predictable lookups). |
| `body` | `Buffer` | — | Response body bytes (decoded for supported encodings). |

##### Methods

###### text()

Return response body as UTF-8 string.

**Signature:**

```typescript
text(): string
```

**Example:**

```typescript
const result = instance.text();
```

**Returns:** `string`

**Errors:** Throws `Error` with a descriptive message.

###### header()

Lookup header by case-insensitive name.

**Signature:**

```typescript
header(name: string): string | null
```

**Example:**

```typescript
const result = instance.header("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `string` | Yes | The name |

**Returns:** `string | null`

---

#### RouteBuilder

Builder for defining a route.

##### Methods

###### new()

Create a new builder for the provided HTTP method and path.

**Signature:**

```typescript
static new(method: Method, path: string): RouteBuilder
```

**Example:**

```typescript
const result = RouteBuilder.new(new Method(), "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `method` | `Method` | Yes | The method |
| `path` | `string` | Yes | Path to the file |

**Returns:** `RouteBuilder`

###### handlerName()

Assign an explicit handler name.

**Signature:**

```typescript
handlerName(name: string): RouteBuilder
```

**Example:**

```typescript
const result = instance.handlerName("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `string` | Yes | The name |

**Returns:** `RouteBuilder`

###### requestSchemaJson()

Provide a raw JSON schema for the request body.

**Signature:**

```typescript
requestSchemaJson(schema: unknown): RouteBuilder
```

**Example:**

```typescript
const result = instance.requestSchemaJson({});
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `schema` | `unknown` | Yes | The schema |

**Returns:** `RouteBuilder`

###### responseSchemaJson()

Provide a raw JSON schema for the response body.

**Signature:**

```typescript
responseSchemaJson(schema: unknown): RouteBuilder
```

**Example:**

```typescript
const result = instance.responseSchemaJson({});
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `schema` | `unknown` | Yes | The schema |

**Returns:** `RouteBuilder`

###### paramsSchemaJson()

Provide a raw JSON schema for request parameters.

**Signature:**

```typescript
paramsSchemaJson(schema: unknown): RouteBuilder
```

**Example:**

```typescript
const result = instance.paramsSchemaJson({});
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `schema` | `unknown` | Yes | The schema |

**Returns:** `RouteBuilder`

###### fileParamsJson()

Provide multipart file parameter configuration.

**Signature:**

```typescript
fileParamsJson(schema: unknown): RouteBuilder
```

**Example:**

```typescript
const result = instance.fileParamsJson({});
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `schema` | `unknown` | Yes | The schema |

**Returns:** `RouteBuilder`

###### cors()

Attach a CORS configuration for this route.

**Signature:**

```typescript
cors(cors: CorsConfig): RouteBuilder
```

**Example:**

```typescript
const result = instance.cors(new CorsConfig());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `cors` | `CorsConfig` | Yes | The cors config |

**Returns:** `RouteBuilder`

###### compression()

Attach a compression configuration for this route.

**Signature:**

```typescript
compression(compression: CompressionConfig): RouteBuilder
```

**Example:**

```typescript
const result = instance.compression(new CompressionConfig());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `compression` | `CompressionConfig` | Yes | The compression config |

**Returns:** `RouteBuilder`

###### sync()

Mark the route as synchronous.

**Signature:**

```typescript
sync(): RouteBuilder
```

**Example:**

```typescript
const result = instance.sync();
```

**Returns:** `RouteBuilder`

###### handlerDependencies()

Declare the dependency keys that must be resolved before this handler runs.

**Signature:**

```typescript
handlerDependencies(dependencies: Array<string>): RouteBuilder
```

**Example:**

```typescript
const result = instance.handlerDependencies([]);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `dependencies` | `Array<string>` | Yes | The dependencies |

**Returns:** `RouteBuilder`

---

#### SchemaConfig

Configuration for GraphQL schema building.

Encapsulates all schema-level configuration options including
introspection control, complexity limits, and depth limits.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `boolean` | `true` | Enable introspection queries |
| `complexityLimit` | `number \| null` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `number \| null` | `null` | Maximum query depth (None = unlimited) |

##### Methods

###### default()

**Signature:**

```typescript
static default(): SchemaConfig
```

**Example:**

```typescript
const result = SchemaConfig.default();
```

**Returns:** `SchemaConfig`

---

#### ServerConfig

Server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `host` | `string` | `"127.0.0.1"` | Host to bind to |
| `port` | `number` | `8000` | Port to bind to |
| `workers` | `number` | `1` | Number of Tokio runtime worker threads used by binding-managed server runtimes |
| `enableRequestId` | `boolean` | `false` | Enable request ID generation and propagation |
| `maxBodySize` | `number \| null` | `null` | Maximum request body size in bytes (None = unlimited, not recommended) |
| `requestTimeout` | `number \| null` | `null` | Request timeout in seconds (None = no timeout) |
| `compression` | `CompressionConfig \| null` | `null` | Enable compression middleware |
| `rateLimit` | `RateLimitConfig \| null` | `null` | Enable rate limiting |
| `jwtAuth` | `JwtConfig \| null` | `null` | JWT authentication configuration |
| `apiKeyAuth` | `ApiKeyConfig \| null` | `null` | API Key authentication configuration |
| `staticFiles` | `Array<StaticFilesConfig>` | `\[\]` | Static file serving configuration |
| `gracefulShutdown` | `boolean` | `true` | Enable graceful shutdown on SIGTERM/SIGINT |
| `shutdownTimeout` | `number` | `30` | Graceful shutdown timeout (seconds) |
| `asyncapi` | `AsyncApiConfig \| null` | `null` | AsyncAPI HTTP endpoint configuration |
| `openapi` | `OpenApiConfig \| null` | `null` | OpenAPI documentation configuration |
| `jsonrpc` | `JsonRpcConfig \| null` | `null` | JSON-RPC configuration |
| `grpc` | `GrpcConfig \| null` | `null` | gRPC configuration |
| `backgroundTasks` | `BackgroundTaskConfig` | — | Background task executor configuration |
| `enableHttpTrace` | `boolean` | `false` | Enable per-request HTTP tracing (tower-http `TraceLayer`) |

##### Methods

###### default()

**Signature:**

```typescript
static default(): ServerConfig
```

**Example:**

```typescript
const result = ServerConfig.default();
```

**Returns:** `ServerConfig`

---

#### ServerInfo

Server information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `string` | — | Base URL of the server (e.g. `"<https://api.example.com/v1"`>). |
| `description` | `string \| null` | `null` | Optional human-readable description of the server environment. |

---

#### SseEvent

An individual SSE event

Represents a single Server-Sent Event to be sent to a connected client.
Events can have an optional type, ID, and retry timeout for advanced scenarios.

##### SSE Format

Events are serialized to the following text format:

```text
event: event_type
data: {"json":"value"}
id: event-123
retry: 3000
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `eventType` | `string \| null` | `null` | Event type (optional) |
| `data` | `unknown` | — | Event data (JSON value) |
| `id` | `string \| null` | `null` | Event ID (optional, for client-side reconnection) |
| `retry` | `number \| null` | `null` | Retry timeout in milliseconds (optional) |

##### Methods

###### withId()

Set the event ID for client-side reconnection support

Sets an ID that clients can use to resume from this point if they disconnect.
The client sends this ID back in the `Last-Event-ID` header when reconnecting.

**Signature:**

```typescript
withId(id: string): SseEvent
```

**Example:**

```typescript
const result = instance.withId("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | Yes | Unique identifier for this event |

**Returns:** `SseEvent`

###### withRetry()

Set the retry timeout for client reconnection

Sets the time in milliseconds clients should wait before attempting to reconnect
if the connection is lost. The client browser will automatically handle reconnection.

**Signature:**

```typescript
withRetry(retryMs: number): SseEvent
```

**Example:**

```typescript
const result = instance.withRetry(42);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `retryMs` | `number` | Yes | Retry timeout in milliseconds |

**Returns:** `SseEvent`

---

#### StaticFilesConfig

Static file serving configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `directory` | `string` | — | Directory path to serve |
| `routePrefix` | `string` | — | URL path prefix (e.g., "/static") |
| `indexFile` | `boolean` | `serde(default = "default_true")` | Fallback to index.html for directories |
| `cacheControl` | `string \| null` | `null` | Cache-Control header value |

---

#### TestClient

Core test client for making HTTP requests to a Spikard application.

This struct wraps axum-test's TestServer and provides a language-agnostic
interface for making HTTP requests, sending WebSocket connections, and
handling Server-Sent Events. Language bindings wrap this to provide
native API surfaces.

##### Methods

###### graphqlAt()

Send a GraphQL query/mutation to a custom endpoint

**Signature:**

```typescript
graphqlAt(endpoint: string, query: string, variables: unknown, operationName: string): Promise<ResponseSnapshot>
```

**Example:**

```typescript
const result = await instance.graphqlAt("value", "value", {}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `endpoint` | `string` | Yes | The endpoint |
| `query` | `string` | Yes | The query |
| `variables` | `unknown \| null` | No | The variables |
| `operationName` | `string \| null` | No | The operation name |

**Returns:** `ResponseSnapshot`

**Errors:** Throws `Error` with a descriptive message.

###### graphql()

Send a GraphQL query/mutation

**Signature:**

```typescript
graphql(query: string, variables: unknown, operationName: string): Promise<ResponseSnapshot>
```

**Example:**

```typescript
const result = await instance.graphql("value", {}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `query` | `string` | Yes | The query |
| `variables` | `unknown \| null` | No | The variables |
| `operationName` | `string \| null` | No | The operation name |

**Returns:** `ResponseSnapshot`

**Errors:** Throws `Error` with a descriptive message.

###### graphqlSubscriptionAt()

Send a GraphQL subscription (WebSocket) to a custom endpoint.

Uses the `graphql-transport-ws` protocol and captures the first `next` payload.
After the first payload is received, this client sends `complete` to unsubscribe.

**Signature:**

```typescript
graphqlSubscriptionAt(endpoint: string, query: string, variables: unknown, operationName: string): Promise<GraphQlSubscriptionSnapshot>
```

**Example:**

```typescript
const result = await instance.graphqlSubscriptionAt("value", "value", {}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `endpoint` | `string` | Yes | The endpoint |
| `query` | `string` | Yes | The query |
| `variables` | `unknown \| null` | No | The variables |
| `operationName` | `string \| null` | No | The operation name |

**Returns:** `GraphQlSubscriptionSnapshot`

**Errors:** Throws `Error` with a descriptive message.

###### graphqlSubscription()

Send a GraphQL subscription (WebSocket).

Uses `/graphql` as the default subscription endpoint.

**Signature:**

```typescript
graphqlSubscription(query: string, variables: unknown, operationName: string): Promise<GraphQlSubscriptionSnapshot>
```

**Example:**

```typescript
const result = await instance.graphqlSubscription("value", {}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `query` | `string` | Yes | The query |
| `variables` | `unknown \| null` | No | The variables |
| `operationName` | `string \| null` | No | The operation name |

**Returns:** `GraphQlSubscriptionSnapshot`

**Errors:** Throws `Error` with a descriptive message.

---

#### TestingSseEvent

A single Server-Sent Event.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data` | `string` | — | The data field of the event. |

---

#### UploadFile

Represents an uploaded file from multipart/form-data requests.

This struct provides efficient access to file content with automatic
base64 decoding and implements standard I/O traits for compatibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `filename` | `string` | — | Original filename from the client |
| `contentType` | `string \| null` | `null` | MIME type of the uploaded file |
| `size` | `number \| null` | `null` | Size of the file in bytes |
| `content` | `Buffer` | — | File content (may be base64 encoded) |
| `contentEncoding` | `string \| null` | `null` | Content encoding type |

##### Methods

###### asBytes()

Get the raw file content as bytes.

This provides zero-copy access to the underlying buffer.

**Signature:**

```typescript
asBytes(): Buffer
```

**Example:**

```typescript
const result = instance.asBytes();
```

**Returns:** `Buffer`

###### readToString()

Read the file content as a UTF-8 string.

**Errors:**

Returns an error if the content is not valid UTF-8.

**Signature:**

```typescript
readToString(): string
```

**Example:**

```typescript
const result = instance.readToString();
```

**Returns:** `string`

**Errors:** Throws `Error` with a descriptive message.

###### contentTypeOrDefault()

Get the content type, defaulting to "application/octet-stream".

**Signature:**

```typescript
contentTypeOrDefault(): string
```

**Example:**

```typescript
const result = instance.contentTypeOrDefault();
```

**Returns:** `string`

---

#### ValidateRequest

Request body for `POST /asyncapi/validate`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `spec` | `unknown` | — | Spec |
| `channel` | `string` | — | Channel |
| `message` | `string` | — | Message |
| `payload` | `unknown` | — | Payload |

---

#### ValidationResponse

Response body for `POST /asyncapi/validate`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `valid` | `boolean` | — | Valid |
| `errors` | `Array<string>` | — | Errors |

---

### Enums

#### Method

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
| `Connect` | Connect |
| `Trace` | Trace |

---

#### SecuritySchemeInfo

Security scheme types

| Value | Description |
|-------|-------------|
| `Http` | Http — Fields: `scheme`: `string`, `bearerFormat`: `string` |
| `ApiKey` | Api key — Fields: `location`: `string`, `name`: `string` |

---

#### SnapshotError

Possible errors while converting an Axum response into a snapshot.

| Value | Description |
|-------|-------------|
| `InvalidHeader` | Response header could not be decoded to UTF-8. — Fields: `0`: `string` |
| `Decompression` | Body decompression failed. — Fields: `0`: `string` |

---

#### WebSocketMessage

A WebSocket message that can be text or binary.

| Value | Description |
|-------|-------------|
| `Text` | A text message. — Fields: `0`: `string` |
| `Binary` | A binary message. — Fields: `0`: `Buffer` |
| `Close` | A close message with a numeric close code (RFC 6455) and optional reason text. Common codes: 1000 Normal Closure, 1001 Going Away, 1005 No Status Received, 1006 Abnormal Closure. — Fields: `code`: `number`, `reason`: `string` |
| `Ping` | A ping message. — Fields: `0`: `Buffer` |
| `Pong` | A pong message. — Fields: `0`: `Buffer` |

---

### Errors

#### AppError

Error type for application builder operations.

Errors are thrown as plain `Error` objects with descriptive messages.

| Variant | Description |
|---------|-------------|
| `Route` | Route registration failed. |
| `Server` | Server/router construction failed. |
| `Decode` | Failed to extract DTO from the request context. |

---

#### GraphQlError

Errors that can occur during GraphQL operations

These errors are compatible with async-graphql error handling and can be
converted to structured HTTP responses matching the project's error fixtures.

Errors are thrown as plain `Error` objects with descriptive messages.

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

#### SchemaError

Error type for schema building operations

Errors are thrown as plain `Error` objects with descriptive messages.

| Variant | Description |
|---------|-------------|
| `BuildingFailed` | Generic schema building error |
| `ValidationError` | Configuration validation error |
| `ComplexityLimitExceeded` | Complexity limit exceeded |
| `DepthLimitExceeded` | Depth limit exceeded |

---

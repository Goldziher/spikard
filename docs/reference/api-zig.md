---
title: "Zig API Reference"
---

## Zig API Reference <span class="version-badge">v0.15.6-rc.23</span>

### Functions

#### schemaQueryOnly()

Create a simple schema configuration with only Query type.

This is a convenience function for schemas that only have queries.

**Returns:**

A `QueryOnlyConfig` with default settings

**Signature:**

```zig
pub fn schema_query_only() QueryOnlyConfig
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
pub fn schema_query_mutation() QueryMutationConfig
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
pub fn schema_full() FullSchemaConfig
```

**Returns:** `FullSchemaConfig`

---

### Types

#### ApiKeyConfig

API Key authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `keys` | `[]const [:0]const u8` | — | Valid API keys |
| `headerName` | `[:0]const u8` | `/* serde(default) */` | Header name to check (e.g., "X-API-Key") |

---

#### App

Spikard application builder.

### Methods

#### new()

Create a new application with the default server configuration.

**Signature:**

```zig
pub fn new() App
```

#### onRequest()

Register an `on_request` lifecycle hook (runs before validation and handler dispatch).

**Signature:**

```zig
pub fn onRequest(self: *const App, hook: [:0]const u8) App
```

#### preValidation()

Register a `pre_validation` lifecycle hook (runs after `on_request`, before validation).

**Signature:**

```zig
pub fn preValidation(self: *const App, hook: [:0]const u8) App
```

#### preHandler()

Register a `pre_handler` lifecycle hook (runs after validation, before the handler).

**Signature:**

```zig
pub fn preHandler(self: *const App, hook: [:0]const u8) App
```

#### onResponse()

Register an `on_response` lifecycle hook (runs after a successful handler response).

**Signature:**

```zig
pub fn onResponse(self: *const App, hook: [:0]const u8) App
```

#### onError()

Register an `on_error` lifecycle hook (runs when the handler returns an error).

**Signature:**

```zig
pub fn onError(self: *const App, hook: [:0]const u8) App
```

#### mergeAxumRouter()

Attach an existing Axum router to this application, returning ownership.

**Signature:**

```zig
pub fn mergeAxumRouter(self: *const App, router: [:0]const u8) App
```

#### attachAxumRouter()

Attach an Axum router using a mutable reference for incremental configuration.

**Signature:**

```zig
pub fn attachAxumRouter(self: *const App, router: [:0]const u8) App
```

#### intoRouter()

Build the underlying Axum router.

**Errors:**

Returns an error if server or router construction fails.

**Signature:**

```zig
pub fn intoRouter(self: *const App) AppError![:0]const u8
```

#### intoRouterAndConfig()

Decompose the application into its Axum router and server configuration.

This is the low-level escape hatch used by the C FFI layer to start the
server on a background thread while retaining the bind address from the
caller-supplied `ServerConfig`.  Prefer `App.run` for normal use.

**Errors:**

Returns an error if router construction fails.

**Signature:**

```zig
pub fn intoRouterAndConfig(self: *const App) AppError![:0]const u8
```

#### default()

**Signature:**

```zig
pub fn default() App
```

---

#### AsyncApiConfig

AsyncAPI HTTP endpoint configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | — | Enable AsyncAPI endpoints (default: false) |
| `spec` | `[:0]const u8?` | `null` | Pre-registered AsyncAPI spec to serve from GET /asyncapi.json |

---

#### BackgroundJobMetadata

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `[:0]const u8` | — | The name |
| `requestId` | `[:0]const u8?` | `null` | Request id |

### Methods

#### default()

**Signature:**

```zig
pub fn default() BackgroundJobMetadata
```

---

#### BackgroundTaskConfig

Configuration for in-process background task execution.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `maxQueueSize` | `u64` | `1024` | Maximum queue size |
| `maxConcurrentTasks` | `u64` | `128` | Maximum concurrent tasks |
| `drainTimeoutSecs` | `u64` | `30` | Drain timeout secs |

### Methods

#### default()

**Signature:**

```zig
pub fn default() BackgroundTaskConfig
```

---

#### CompressionConfig

Compression configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `gzip` | `bool` | `true` | Enable gzip compression |
| `brotli` | `bool` | `true` | Enable brotli compression |
| `minSize` | `u64` | — | Minimum response size to compress (bytes) |
| `quality` | `u32` | — | Compression quality (0-11 for brotli, 0-9 for gzip) |

### Methods

#### default()

**Signature:**

```zig
pub fn default() CompressionConfig
```

---

#### ContactInfo

Contact information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `[:0]const u8?` | `null` | Name of the contact person or organisation. |
| `email` | `[:0]const u8?` | `null` | Contact email address. |
| `url` | `[:0]const u8?` | `null` | URL pointing to the contact information page. |

---

#### CorsConfig

CORS configuration for a route

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `allowedOrigins` | `[]const [:0]const u8` | `[]` | Allowed origins |
| `allowedMethods` | `[]const [:0]const u8` | `[]` | Allowed methods |
| `allowedHeaders` | `[]const [:0]const u8` | `[]` | Allowed headers |
| `exposeHeaders` | `[]const [:0]const u8?` | `null` | Expose headers |
| `maxAge` | `u32?` | `null` | Maximum age |
| `allowCredentials` | `bool?` | `null` | Allow credentials |
| `methodsJoinedCache` | `[:0]const u8` | — | Methods joined cache |
| `headersJoinedCache` | `[:0]const u8` | — | Headers joined cache |

### Methods

#### allowedMethodsJoined()

Get the cached joined methods string for preflight responses

**Signature:**

```zig
pub fn allowedMethodsJoined(self: *const CorsConfig) [:0]const u8
```

#### allowedHeadersJoined()

Get the cached joined headers string for preflight responses

**Signature:**

```zig
pub fn allowedHeadersJoined(self: *const CorsConfig) [:0]const u8
```

#### isOriginAllowed()

Check if an origin is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```zig
pub fn isOriginAllowed(self: *const CorsConfig, origin: [:0]const u8) bool
```

#### isMethodAllowed()

Check if a method is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```zig
pub fn isMethodAllowed(self: *const CorsConfig, method: [:0]const u8) bool
```

#### default()

**Signature:**

```zig
pub fn default() CorsConfig
```

---

#### FullSchemaConfig

Configuration for fully-featured schemas with Query, Mutation, and Subscription types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `bool` | `true` | Enable introspection queries |
| `complexityLimit` | `u64?` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `u64?` | `null` | Maximum query depth (None = unlimited) |

### Methods

#### default()

**Signature:**

```zig
pub fn default() FullSchemaConfig
```

---

#### GraphQlRouteConfig

Configuration for GraphQL routes

Provides a builder pattern for configuring GraphQL route parameters
for the Spikard HTTP server's routing system.

### Methods

#### new()

Create a new GraphQL route configuration with defaults

Default values:

- path: "/graphql"
- method: "POST"
- `enable_playground`: false

**Signature:**

```zig
pub fn new() GraphQlRouteConfig
```

#### path()

Set the HTTP path for the GraphQL endpoint

**Signature:**

```zig
pub fn path(self: *const GraphQlRouteConfig, path: [:0]const u8) GraphQlRouteConfig
```

#### method()

Set the HTTP method for the GraphQL endpoint

**Signature:**

```zig
pub fn method(self: *const GraphQlRouteConfig, method: [:0]const u8) GraphQlRouteConfig
```

#### enablePlayground()

Enable or disable the GraphQL Playground UI

**Signature:**

```zig
pub fn enablePlayground(self: *const GraphQlRouteConfig, enable: bool) GraphQlRouteConfig
```

#### description()

Set a custom description for documentation

**Signature:**

```zig
pub fn description(self: *const GraphQlRouteConfig, description: [:0]const u8) GraphQlRouteConfig
```

#### getPath()

Get the configured path

**Signature:**

```zig
pub fn getPath(self: *const GraphQlRouteConfig) [:0]const u8
```

#### getMethod()

Get the configured method

**Signature:**

```zig
pub fn getMethod(self: *const GraphQlRouteConfig) [:0]const u8
```

#### isPlaygroundEnabled()

Check if playground is enabled

**Signature:**

```zig
pub fn isPlaygroundEnabled(self: *const GraphQlRouteConfig) bool
```

#### getDescription()

Get the description if set

**Signature:**

```zig
pub fn getDescription(self: *const GraphQlRouteConfig) ?[:0]const u8
```

#### default()

**Signature:**

```zig
pub fn default() GraphQlRouteConfig
```

---

#### GraphQlSubscriptionSnapshot

Snapshot of a GraphQL subscription exchange over WebSocket.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `operationId` | `[:0]const u8` | — | Operation id used for the subscription request. |
| `acknowledged` | `bool` | — | Whether the server acknowledged the GraphQL WebSocket connection. |
| `event` | `[:0]const u8?` | `null` | First `next.payload` received for this subscription, if any. |
| `errors` | `[]const [:0]const u8` | — | GraphQL protocol errors emitted by the server. |
| `completeReceived` | `bool` | — | Whether a `complete` frame was observed for this operation. |

---

#### GrpcConfig

Configuration for gRPC support

Controls how the server handles gRPC requests, including compression,
timeouts, and protocol settings.

### Stream Limits

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
| `enabled` | `bool` | `true` | Enable gRPC support |
| `maxMessageSize` | `u64` | — | Maximum message size in bytes (for both sending and receiving) This limit applies to individual messages in both unary and streaming RPCs. When a single message exceeds this size, the request is rejected with HTTP 413 (Payload Too Large). Default: 4MB (4194304 bytes) **Note:** This limit does NOT apply to the total response size in streaming RPCs. For multi-message streams, the total response can exceed this limit as long as each individual message stays within the limit. |
| `enableCompression` | `bool` | `true` | Enable gzip compression for gRPC messages |
| `requestTimeout` | `u64?` | `null` | Timeout for gRPC requests in seconds (None = no timeout) |
| `maxConcurrentStreams` | `u32` | — | Maximum number of concurrent streams per connection (HTTP/2 advisory) This value is communicated to HTTP/2 clients as the server's flow control limit. The HTTP/2 transport layer enforces this limit automatically via SETTINGS frames and GOAWAY responses. Applications should NOT implement custom enforcement. Default: 100 streams per connection # Stream Limiting Strategy - **Per Connection**: This limit applies per HTTP/2 connection, not globally - **Transport Enforcement**: HTTP/2 handles all stream limiting; applications need not implement custom checks - **Streaming Requests**: In server streaming or bidi streaming, each logical RPC consumes one stream slot. Message ordering within a stream follows HTTP/2 frame ordering. |
| `enableKeepalive` | `bool` | `true` | Enable HTTP/2 keepalive |
| `keepaliveInterval` | `u64` | — | HTTP/2 keepalive interval in seconds |
| `keepaliveTimeout` | `u64` | — | HTTP/2 keepalive timeout in seconds |
| `maxStreamResponseBytes` | `u64?` | `null` | Total byte cap across an entire streaming response. When `Some(n)`, the streaming adapter aborts the stream with `tonic.Status.resource_exhausted` once the cumulative encoded message bytes exceed `n`. The stream yields the error item and then terminates. Per-message cap remains `max_message_size`. This limit applies to server-streaming and bidirectional-streaming RPCs only; unary RPCs are governed solely by `max_message_size`. Default: `null` (unbounded total response size). |

### Methods

#### default()

**Signature:**

```zig
pub fn default() GrpcConfig
```

---

#### Handler

Handler trait that all language bindings must implement

This trait is completely language-agnostic. Each binding (Python, Node, WASM)
implements this trait to bridge their runtime to our HTTP server.

### Methods

#### call()

Handle an HTTP request

Takes the extracted request data and returns a future that resolves to either:

- Ok(Response): A successful HTTP response
- Err((StatusCode, String)): An error with status code and message

**Signature:**

```zig
pub fn call(self: *const Handler, request: Request, request_data: RequestData) HandlerResult
```

#### prefersRawJsonBody()

Whether this handler prefers consuming `RequestData.raw_body` over the parsed
`RequestData.body` for JSON requests.

When `true`, the server may skip eager JSON parsing when there is no request-body
schema validator attached to the route.

**Signature:**

```zig
pub fn prefersRawJsonBody(self: *const Handler) bool
```

#### prefersParameterExtraction()

Whether this handler wants to perform its own parameter validation/extraction (path/query/header/cookie).

When `true`, the server will skip `ParameterValidator.validate_and_extract` in `ValidatingHandler`.
This is useful for language bindings which need to transform validated parameters into
language-specific values (e.g., Python kwargs) without duplicating work. When `false`,
the server stores validated output in `RequestData.validated_params`.

**Signature:**

```zig
pub fn prefersParameterExtraction(self: *const Handler) bool
```

#### wantsHeaders()

Whether this handler needs the parsed headers map in `RequestData`.

When `false`, the server may skip building `RequestData.headers` for requests without a body.
(Requests with bodies still typically need `Content-Type` decisions.)

**Signature:**

```zig
pub fn wantsHeaders(self: *const Handler) bool
```

#### wantsCookies()

Whether this handler needs the parsed cookies map in `RequestData`.

When `false`, the server may skip parsing cookies for requests without a body.

**Signature:**

```zig
pub fn wantsCookies(self: *const Handler) bool
```

#### wantsRequestExtensions()

Whether this handler needs `RequestData` stored in request extensions.

When `false`, the server avoids inserting `RequestData` into extensions to
skip cloning in hot paths.

**Signature:**

```zig
pub fn wantsRequestExtensions(self: *const Handler) bool
```

#### staticResponse()

Return a pre-built static response if this handler always produces the
same output. When `Some`, the server bypasses the full middleware
pipeline and serves the pre-built response directly.

**Signature:**

```zig
pub fn staticResponse(self: *const Handler) ?StaticResponse
```

---

#### IntoHandler

Convert user-facing handler functions into the low-level `Handler` trait.

### Methods

#### intoHandler()

Convert this value into a shared request handler.

**Signature:**

```zig
pub fn intoHandler(self: *const IntoHandler) Handler
```

---

#### JsonRpcConfig

JSON-RPC server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `true` | Enable JSON-RPC endpoint |
| `endpointPath` | `[:0]const u8` | — | HTTP endpoint path for JSON-RPC requests (default: "/rpc") |
| `enableBatch` | `bool` | — | Enable batch request processing (default: true) |
| `maxBatchSize` | `u64` | — | Maximum number of requests in a batch (default: 100) |

### Methods

#### default()

**Signature:**

```zig
pub fn default() JsonRpcConfig
```

---

#### JsonRpcMethodInfo

JSON-RPC method metadata for routes that support JSON-RPC

This struct captures the metadata needed to expose HTTP routes as JSON-RPC methods,
enabling discovery and documentation of RPC-compatible endpoints.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `methodName` | `[:0]const u8` | — | The JSON-RPC method name (e.g., "user.create") |
| `description` | `[:0]const u8?` | `null` | Optional description of what the method does |
| `paramsSchema` | `[:0]const u8?` | `null` | Optional JSON Schema for method parameters |
| `resultSchema` | `[:0]const u8?` | `null` | Optional JSON Schema for the result |
| `deprecated` | `bool` | `/* serde(default) */` | Whether this method is deprecated |
| `tags` | `[]const [:0]const u8` | `/* serde(default) */` | Tags for categorizing and grouping methods |

---

#### JwtConfig

JWT authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `secret` | `[:0]const u8` | — | Secret key for JWT verification |
| `algorithm` | `[:0]const u8` | `/* serde(default) */` | Required algorithm (HS256, HS384, HS512, RS256, etc.) |
| `audience` | `[]const [:0]const u8?` | `null` | Required audience claim |
| `issuer` | `[:0]const u8?` | `null` | Required issuer claim |
| `leeway` | `u64` | `/* serde(default) */` | Leeway for expiration checks (seconds) |

---

#### LicenseInfo

License information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `[:0]const u8` | — | SPDX license identifier or display name (e.g. `"MIT"`). |
| `url` | `[:0]const u8?` | `null` | URL to the full license text. |

---

#### OpenApiConfig

OpenAPI configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `false` | Enable OpenAPI generation (default: false for zero overhead) |
| `title` | `[:0]const u8` | `"API"` | API title |
| `version` | `[:0]const u8` | `"1.0.0"` | API version |
| `description` | `[:0]const u8?` | `null` | API description (supports markdown) |
| `swaggerUiPath` | `[:0]const u8` | — | Path to serve Swagger UI (default: "/docs") |
| `redocPath` | `[:0]const u8` | — | Path to serve Redoc (default: "/redoc") |
| `openapiJsonPath` | `[:0]const u8` | — | Path to serve OpenAPI JSON spec (default: "/openapi.json") |
| `contact` | `ContactInfo?` | `null` | Contact information |
| `license` | `LicenseInfo?` | `null` | License information |
| `servers` | `[]const ServerInfo` | `[]` | Server definitions |
| `securitySchemes` | `std.StringHashMap(SecuritySchemeInfo)` | `{}` | Security schemes (auto-detected from middleware if not provided) |

### Methods

#### default()

**Signature:**

```zig
pub fn default() OpenApiConfig
```

---

#### ParseRequest

Request body for `POST /asyncapi/parse`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `spec` | `[:0]const u8` | — | Spec |

---

#### ParseResult

Full parse result returned by `POST /asyncapi/parse`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `specVersion` | `[:0]const u8` | — | Spec version |
| `title` | `[:0]const u8` | — | Title |
| `apiVersion` | `[:0]const u8` | — | Api version |
| `channels` | `[]const ParsedChannel` | — | Channels |
| `operations` | `[]const ParsedOperation` | — | Operations |
| `messages` | `[]const ParsedMessage` | — | Messages |

---

#### ParsedChannel

A single channel extracted from an AsyncAPI spec

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `[:0]const u8` | — | Channel key from the spec (e.g. "chat/messages") |
| `address` | `[:0]const u8` | — | Channel address / path |
| `messages` | `[]const [:0]const u8` | — | Message names declared on this channel |
| `bindings` | `[:0]const u8?` | `null` | Bindings (ws / http / amqp / …) as raw JSON for forward-compatibility |

---

#### ParsedMessage

A resolved message (name + JSON Schema)

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `[:0]const u8` | — | Message name |
| `schema` | `[:0]const u8?` | `null` | Resolved JSON Schema for the message payload, if available |

---

#### ParsedOperation

A single operation extracted from an AsyncAPI spec

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `[:0]const u8` | — | Operation name |
| `action` | `[:0]const u8` | — | Operation action: "send" or "receive" |
| `channel` | `[:0]const u8` | — | Channel reference (resolved to the channel name) |

---

#### ProblemDetails

RFC 9457 Problem Details for HTTP APIs

A machine-readable format for specifying errors in HTTP API responses.
Per RFC 9457, all fields are optional. The `type` field defaults to "about:blank"
if not specified.

### Content-Type
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
| `typeUri` | `[:0]const u8` | — | A URI reference that identifies the problem type. Defaults to "about:blank" when absent. Should be a stable, human-readable identifier for the problem type. |
| `title` | `[:0]const u8` | — | A short, human-readable summary of the problem type. Should not change from occurrence to occurrence of the problem. |
| `status` | `u16` | — | The HTTP status code generated by the origin server. This is advisory; the actual HTTP status code takes precedence. |
| `detail` | `[:0]const u8?` | `null` | A human-readable explanation specific to this occurrence of the problem. |
| `instance` | `[:0]const u8?` | `null` | A URI reference that identifies the specific occurrence of the problem. It may or may not yield further information if dereferenced. |
| `extensions` | `std.StringHashMap([:0]const u8)` | — | Extension members - problem-type-specific data. For validation errors, this typically contains an "errors" array. |

### Methods

#### withDetail()

Set the detail field

**Signature:**

```zig
pub fn withDetail(self: *const ProblemDetails, detail: [:0]const u8) ProblemDetails
```

#### withInstance()

Set the instance field

**Signature:**

```zig
pub fn withInstance(self: *const ProblemDetails, instance: [:0]const u8) ProblemDetails
```

#### notFound()

Create a not found error

**Signature:**

```zig
pub fn notFound(detail: [:0]const u8) ProblemDetails
```

#### methodNotAllowed()

Create a method not allowed error

**Signature:**

```zig
pub fn methodNotAllowed(detail: [:0]const u8) ProblemDetails
```

#### internalServerError()

Create an internal server error

**Signature:**

```zig
pub fn internalServerError(detail: [:0]const u8) ProblemDetails
```

#### badRequest()

Create a bad request error

**Signature:**

```zig
pub fn badRequest(detail: [:0]const u8) ProblemDetails
```

#### toJson()

Serialize to JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```zig
pub fn toJson(self: *const ProblemDetails) Error![:0]const u8
```

#### toJsonPretty()

Serialize to pretty JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```zig
pub fn toJsonPretty(self: *const ProblemDetails) Error![:0]const u8
```

---

#### QueryMutationConfig

Configuration for schemas with Query and Mutation types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `bool` | `true` | Enable introspection queries |
| `complexityLimit` | `u64?` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `u64?` | `null` | Maximum query depth (None = unlimited) |

### Methods

#### default()

**Signature:**

```zig
pub fn default() QueryMutationConfig
```

---

#### QueryOnlyConfig

Configuration for schemas with only Query type

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `bool` | `true` | Enable introspection queries |
| `complexityLimit` | `u64?` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `u64?` | `null` | Maximum query depth (None = unlimited) |

### Methods

#### default()

**Signature:**

```zig
pub fn default() QueryOnlyConfig
```

---

#### RateLimitConfig

Rate limiting configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `perSecond` | `u64` | `100` | Requests per second |
| `burst` | `u32` | `200` | Burst allowance |
| `ipBased` | `bool` | `true` | Use IP-based rate limiting |

### Methods

#### default()

**Signature:**

```zig
pub fn default() RateLimitConfig
```

---

#### Request

---

#### Response

HTTP Response with custom status code, headers, and content

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `[:0]const u8?` | `null` | Response body content |
| `statusCode` | `u16` | — | HTTP status code (defaults to 200) |
| `headers` | `std.StringHashMap([:0]const u8)` | `{}` | Response headers |

### Methods

#### setHeader()

Set a header

**Signature:**

```zig
pub fn setHeader(self: *const Response, key: [:0]const u8, value: [:0]const u8) void
```

#### setCookie()

Set a cookie in the response

**Signature:**

```zig
pub fn setCookie(self: *const Response, key: [:0]const u8, value: [:0]const u8, secure: bool, http_only: bool, max_age: ?i64, domain: ?[:0]const u8, path: ?[:0]const u8, same_site: ?[:0]const u8) void
```

#### default()

**Signature:**

```zig
pub fn default() Response
```

---

#### ResponseSnapshot

Snapshot of an Axum response used by higher-level language bindings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `status` | `u16` | — | HTTP status code. |
| `headers` | `std.StringHashMap([:0]const u8)` | — | Response headers (lowercase keys for predictable lookups). |
| `body` | `[]const u8` | — | Response body bytes (decoded for supported encodings). |

### Methods

#### text()

Return response body as UTF-8 string.

**Signature:**

```zig
pub fn text(self: *const ResponseSnapshot) FromUtf8Error![:0]const u8
```

#### header()

Lookup header by case-insensitive name.

**Signature:**

```zig
pub fn header(self: *const ResponseSnapshot, name: [:0]const u8) ?[:0]const u8
```

---

#### RouteBuilder

Builder for defining a route.

### Methods

#### new()

Create a new builder for the provided HTTP method and path.

**Signature:**

```zig
pub fn new(method: Method, path: [:0]const u8) RouteBuilder
```

#### handlerName()

Assign an explicit handler name.

**Signature:**

```zig
pub fn handlerName(self: *const RouteBuilder, name: [:0]const u8) RouteBuilder
```

#### requestSchemaJson()

Provide a raw JSON schema for the request body.

**Signature:**

```zig
pub fn requestSchemaJson(self: *const RouteBuilder, schema: [:0]const u8) RouteBuilder
```

#### responseSchemaJson()

Provide a raw JSON schema for the response body.

**Signature:**

```zig
pub fn responseSchemaJson(self: *const RouteBuilder, schema: [:0]const u8) RouteBuilder
```

#### paramsSchemaJson()

Provide a raw JSON schema for request parameters.

**Signature:**

```zig
pub fn paramsSchemaJson(self: *const RouteBuilder, schema: [:0]const u8) RouteBuilder
```

#### fileParamsJson()

Provide multipart file parameter configuration.

**Signature:**

```zig
pub fn fileParamsJson(self: *const RouteBuilder, schema: [:0]const u8) RouteBuilder
```

#### cors()

Attach a CORS configuration for this route.

**Signature:**

```zig
pub fn cors(self: *const RouteBuilder, cors: CorsConfig) RouteBuilder
```

#### compression()

Attach a compression configuration for this route.

**Signature:**

```zig
pub fn compression(self: *const RouteBuilder, compression: CompressionConfig) RouteBuilder
```

#### sync()

Mark the route as synchronous.

**Signature:**

```zig
pub fn sync(self: *const RouteBuilder) RouteBuilder
```

#### handlerDependencies()

Declare the dependency keys that must be resolved before this handler runs.

**Signature:**

```zig
pub fn handlerDependencies(self: *const RouteBuilder, dependencies: []const [:0]const u8) RouteBuilder
```

---

#### SchemaConfig

Configuration for GraphQL schema building.

Encapsulates all schema-level configuration options including
introspection control, complexity limits, and depth limits.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `bool` | `true` | Enable introspection queries |
| `complexityLimit` | `u64?` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `u64?` | `null` | Maximum query depth (None = unlimited) |

### Methods

#### default()

**Signature:**

```zig
pub fn default() SchemaConfig
```

---

#### ServerConfig

Server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `host` | `[:0]const u8` | `"127.0.0.1"` | Host to bind to |
| `port` | `u16` | `8000` | Port to bind to |
| `workers` | `u64` | `1` | Number of Tokio runtime worker threads used by binding-managed server runtimes |
| `enableRequestId` | `bool` | `false` | Enable request ID generation and propagation |
| `maxBodySize` | `u64?` | `null` | Maximum request body size in bytes (None = unlimited, not recommended) |
| `requestTimeout` | `u64?` | `null` | Request timeout in seconds (None = no timeout) |
| `compression` | `CompressionConfig?` | `null` | Enable compression middleware |
| `rateLimit` | `RateLimitConfig?` | `null` | Enable rate limiting |
| `jwtAuth` | `JwtConfig?` | `null` | JWT authentication configuration |
| `apiKeyAuth` | `ApiKeyConfig?` | `null` | API Key authentication configuration |
| `staticFiles` | `[]const StaticFilesConfig` | `[]` | Static file serving configuration |
| `gracefulShutdown` | `bool` | `true` | Enable graceful shutdown on SIGTERM/SIGINT |
| `shutdownTimeout` | `u64` | `30` | Graceful shutdown timeout (seconds) |
| `asyncapi` | `AsyncApiConfig?` | `null` | AsyncAPI HTTP endpoint configuration |
| `openapi` | `OpenApiConfig?` | `null` | OpenAPI documentation configuration |
| `jsonrpc` | `JsonRpcConfig?` | `null` | JSON-RPC configuration |
| `grpc` | `GrpcConfig?` | `null` | gRPC configuration |
| `lifecycleHooks` | `[:0]const u8?` | `null` | Lifecycle hooks for request/response processing |
| `backgroundTasks` | `BackgroundTaskConfig` | — | Background task executor configuration |
| `enableHttpTrace` | `bool` | `false` | Enable per-request HTTP tracing (tower-http `TraceLayer`) |
| `diContainer` | `[:0]const u8?` | `null` | Dependency injection container (requires 'di' feature) |

### Methods

#### default()

**Signature:**

```zig
pub fn default() ServerConfig
```

---

#### ServerInfo

Server information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `[:0]const u8` | — | Base URL of the server (e.g. `"<https://api.example.com/v1"`>). |
| `description` | `[:0]const u8?` | `null` | Optional human-readable description of the server environment. |

---

#### SseEvent

An individual SSE event

Represents a single Server-Sent Event to be sent to a connected client.
Events can have an optional type, ID, and retry timeout for advanced scenarios.

### SSE Format

Events are serialized to the following text format:
```text
event: event_type
data: {"json":"value"}
id: event-123
retry: 3000
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `eventType` | `[:0]const u8?` | `null` | Event type (optional) |
| `data` | `[:0]const u8` | — | Event data (JSON value) |
| `id` | `[:0]const u8?` | `null` | Event ID (optional, for client-side reconnection) |
| `retry` | `u64?` | `null` | Retry timeout in milliseconds (optional) |

### Methods

#### withId()

Set the event ID for client-side reconnection support

Sets an ID that clients can use to resume from this point if they disconnect.
The client sends this ID back in the `Last-Event-ID` header when reconnecting.

**Signature:**

```zig
pub fn withId(self: *const SseEvent, id: [:0]const u8) SseEvent
```

#### withRetry()

Set the retry timeout for client reconnection

Sets the time in milliseconds clients should wait before attempting to reconnect
if the connection is lost. The client browser will automatically handle reconnection.

**Signature:**

```zig
pub fn withRetry(self: *const SseEvent, retry_ms: u64) SseEvent
```

---

#### StaticFilesConfig

Static file serving configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `directory` | `[:0]const u8` | — | Directory path to serve |
| `routePrefix` | `[:0]const u8` | — | URL path prefix (e.g., "/static") |
| `indexFile` | `bool` | `/* serde(default) */` | Fallback to index.html for directories |
| `cacheControl` | `[:0]const u8?` | `null` | Cache-Control header value |

---

#### TestClient

Core test client for making HTTP requests to a Spikard application.

This struct wraps axum-test's TestServer and provides a language-agnostic
interface for making HTTP requests, sending WebSocket connections, and
handling Server-Sent Events. Language bindings wrap this to provide
native API surfaces.

### Methods

#### graphqlAt()

Send a GraphQL query/mutation to a custom endpoint

**Signature:**

```zig
pub fn graphqlAt(self: *const TestClient, endpoint: [:0]const u8, query: [:0]const u8, variables: ?[:0]const u8, operation_name: ?[:0]const u8) SnapshotError!ResponseSnapshot
```

#### graphql()

Send a GraphQL query/mutation

**Signature:**

```zig
pub fn graphql(self: *const TestClient, query: [:0]const u8, variables: ?[:0]const u8, operation_name: ?[:0]const u8) SnapshotError!ResponseSnapshot
```

#### graphqlSubscriptionAt()

Send a GraphQL subscription (WebSocket) to a custom endpoint.

Uses the `graphql-transport-ws` protocol and captures the first `next` payload.
After the first payload is received, this client sends `complete` to unsubscribe.

**Signature:**

```zig
pub fn graphqlSubscriptionAt(self: *const TestClient, endpoint: [:0]const u8, query: [:0]const u8, variables: ?[:0]const u8, operation_name: ?[:0]const u8) SnapshotError!GraphQlSubscriptionSnapshot
```

#### graphqlSubscription()

Send a GraphQL subscription (WebSocket).

Uses `/graphql` as the default subscription endpoint.

**Signature:**

```zig
pub fn graphqlSubscription(self: *const TestClient, query: [:0]const u8, variables: ?[:0]const u8, operation_name: ?[:0]const u8) SnapshotError!GraphQlSubscriptionSnapshot
```

---

#### TestingSseEvent

A single Server-Sent Event.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data` | `[:0]const u8` | — | The data field of the event. |

---

#### UploadFile

Represents an uploaded file from multipart/form-data requests.

This struct provides efficient access to file content with automatic
base64 decoding and implements standard I/O traits for compatibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `filename` | `[:0]const u8` | — | Original filename from the client |
| `contentType` | `[:0]const u8?` | `null` | MIME type of the uploaded file |
| `size` | `u64?` | `null` | Size of the file in bytes |
| `content` | `[]const u8` | — | File content (may be base64 encoded) |
| `contentEncoding` | `[:0]const u8?` | `null` | Content encoding type |
| `cursor` | `[:0]const u8` | — | Internal cursor for Read/Seek operations |

### Methods

#### asBytes()

Get the raw file content as bytes.

This provides zero-copy access to the underlying buffer.

**Signature:**

```zig
pub fn asBytes(self: *const UploadFile) []const u8
```

#### readToString()

Read the file content as a UTF-8 string.

**Errors:**

Returns an error if the content is not valid UTF-8.

**Signature:**

```zig
pub fn readToString(self: *const UploadFile) Error![:0]const u8
```

#### contentTypeOrDefault()

Get the content type, defaulting to "application/octet-stream".

**Signature:**

```zig
pub fn contentTypeOrDefault(self: *const UploadFile) [:0]const u8
```

---

#### ValidateRequest

Request body for `POST /asyncapi/validate`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `spec` | `[:0]const u8` | — | Spec |
| `channel` | `[:0]const u8` | — | Channel |
| `message` | `[:0]const u8` | — | Message |
| `payload` | `[:0]const u8` | — | Payload |

---

#### ValidationResponse

Response body for `POST /asyncapi/validate`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `valid` | `bool` | — | Valid |
| `errors` | `[]const [:0]const u8` | — | Errors |

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
| `Http` | Http — Fields: `scheme`: `[:0]const u8`, `bearerFormat`: `[:0]const u8` |
| `ApiKey` | Api key — Fields: `location`: `[:0]const u8`, `name`: `[:0]const u8` |

---

#### SnapshotError

Possible errors while converting an Axum response into a snapshot.

| Value | Description |
|-------|-------------|
| `InvalidHeader` | Response header could not be decoded to UTF-8. — Fields: `0`: `[:0]const u8` |
| `Decompression` | Body decompression failed. — Fields: `0`: `[:0]const u8` |

---

#### WebSocketMessage

A WebSocket message that can be text or binary.

| Value | Description |
|-------|-------------|
| `Text` | A text message. — Fields: `0`: `[:0]const u8` |
| `Binary` | A binary message. — Fields: `0`: `[]const u8` |
| `Close` | A close message with a numeric close code (RFC 6455) and optional reason text. Common codes: 1000 Normal Closure, 1001 Going Away, 1005 No Status Received, 1006 Abnormal Closure. — Fields: `code`: `u16`, `reason`: `[:0]const u8` |
| `Ping` | A ping message. — Fields: `0`: `[]const u8` |
| `Pong` | A pong message. — Fields: `0`: `[]const u8` |

---

### Errors

#### AppError

Error type for application builder operations.

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

| Variant | Description |
|---------|-------------|
| `BuildingFailed` | Generic schema building error |
| `ValidationError` | Configuration validation error |
| `ComplexityLimitExceeded` | Complexity limit exceeded |
| `DepthLimitExceeded` | Depth limit exceeded |

---

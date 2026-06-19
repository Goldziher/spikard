---
title: "Dart API Reference"
---

## Dart API Reference <span class="version-badge">v0.16.0-rc.1</span>

### Functions

#### schemaQueryOnly()

Create a simple schema configuration with only Query type.

This is a convenience function for schemas that only have queries.

**Returns:**

A `QueryOnlyConfig` with default settings

**Signature:**

```dart
QueryOnlyConfig schemaQueryOnly()
```

**Example:**

```dart
final result = schemaQueryOnly();
```

**Returns:** `QueryOnlyConfig`

---

#### schemaQueryMutation()

Create a schema configuration with Query and Mutation types.

This is a convenience function for schemas with queries and mutations but no subscriptions.

**Returns:**

A `QueryMutationConfig` with default settings

**Signature:**

```dart
QueryMutationConfig schemaQueryMutation()
```

**Example:**

```dart
final result = schemaQueryMutation();
```

**Returns:** `QueryMutationConfig`

---

#### schemaFull()

Create a schema configuration with all three root types.

This is a convenience function for fully-featured schemas.

**Returns:**

A `FullSchemaConfig` with default settings

**Signature:**

```dart
FullSchemaConfig schemaFull()
```

**Example:**

```dart
final result = schemaFull();
```

**Returns:** `FullSchemaConfig`

---

### Types

#### ApiKeyConfig

API Key authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `keys` | `List<String>` | — | Valid API keys |
| `headerName` | `String` | `serde(default = "default_api_key_header")` | Header name to check (e.g., "X-API-Key") |

---

#### AsyncApiConfig

AsyncAPI HTTP endpoint configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | — | Enable AsyncAPI endpoints (default: false) |
| `spec` | `String?` | `null` | Pre-registered AsyncAPI spec to serve from GET /asyncapi.json |

---

#### BackgroundJobMetadata

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `requestId` | `String?` | `null` | Request id |

##### Methods

###### default()

**Signature:**

```dart
static BackgroundJobMetadata default()
```

**Example:**

```dart
final result = BackgroundJobMetadata.default();
```

**Returns:** `BackgroundJobMetadata`

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

```dart
static BackgroundTaskConfig default()
```

**Example:**

```dart
final result = BackgroundTaskConfig.default();
```

**Returns:** `BackgroundTaskConfig`

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

```dart
static CompressionConfig default()
```

**Example:**

```dart
final result = CompressionConfig.default();
```

**Returns:** `CompressionConfig`

---

#### ContactInfo

Contact information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String?` | `null` | Name of the contact person or organisation. |
| `email` | `String?` | `null` | Contact email address. |
| `url` | `String?` | `null` | URL pointing to the contact information page. |

---

#### CorsConfig

CORS configuration for a route

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `allowedOrigins` | `List<String>` | `\[\]` | Allowed origins |
| `allowedMethods` | `List<String>` | `\[\]` | Allowed methods |
| `allowedHeaders` | `List<String>` | `\[\]` | Allowed headers |
| `exposeHeaders` | `List<String>?` | `null` | Expose headers |
| `maxAge` | `int?` | `null` | Maximum age |
| `allowCredentials` | `bool?` | `null` | Allow credentials |

##### Methods

###### allowedMethodsJoined()

Get the cached joined methods string for preflight responses

**Signature:**

```dart
String allowedMethodsJoined()
```

**Example:**

```dart
final result = instance.allowedMethodsJoined();
```

**Returns:** `String`

###### allowedHeadersJoined()

Get the cached joined headers string for preflight responses

**Signature:**

```dart
String allowedHeadersJoined()
```

**Example:**

```dart
final result = instance.allowedHeadersJoined();
```

**Returns:** `String`

###### isOriginAllowed()

Check if an origin is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```dart
bool isOriginAllowed(String origin)
```

**Example:**

```dart
final result = instance.isOriginAllowed("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `origin` | `String` | Yes | The origin |

**Returns:** `bool`

###### isMethodAllowed()

Check if a method is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```dart
bool isMethodAllowed(String method)
```

**Example:**

```dart
final result = instance.isMethodAllowed("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `method` | `String` | Yes | The method |

**Returns:** `bool`

###### default()

**Signature:**

```dart
static CorsConfig default()
```

**Example:**

```dart
final result = CorsConfig.default();
```

**Returns:** `CorsConfig`

---

#### FullSchemaConfig

Configuration for fully-featured schemas with Query, Mutation, and Subscription types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `bool` | `true` | Enable introspection queries |
| `complexityLimit` | `int?` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `int?` | `null` | Maximum query depth (None = unlimited) |

##### Methods

###### default()

**Signature:**

```dart
static FullSchemaConfig default()
```

**Example:**

```dart
final result = FullSchemaConfig.default();
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

```dart
static GraphQlRouteConfig new()
```

**Example:**

```dart
final result = GraphQlRouteConfig.new();
```

**Returns:** `GraphQlRouteConfig`

###### path()

Set the HTTP path for the GraphQL endpoint

**Signature:**

```dart
GraphQlRouteConfig path(String path)
```

**Example:**

```dart
final result = instance.path("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `String` | Yes | The URL path (e.g., "/graphql", "/api/graphql") |

**Returns:** `GraphQlRouteConfig`

###### method()

Set the HTTP method for the GraphQL endpoint

**Signature:**

```dart
GraphQlRouteConfig method(String method)
```

**Example:**

```dart
final result = instance.method("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `method` | `String` | Yes | The HTTP method (typically "POST") |

**Returns:** `GraphQlRouteConfig`

###### enablePlayground()

Enable or disable the GraphQL Playground UI

**Signature:**

```dart
GraphQlRouteConfig enablePlayground(bool enable)
```

**Example:**

```dart
final result = instance.enablePlayground(true);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `enable` | `bool` | Yes | Whether to enable playground |

**Returns:** `GraphQlRouteConfig`

###### description()

Set a custom description for documentation

**Signature:**

```dart
GraphQlRouteConfig description(String description)
```

**Example:**

```dart
final result = instance.description("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `description` | `String` | Yes | Documentation string |

**Returns:** `GraphQlRouteConfig`

###### getPath()

Get the configured path

**Signature:**

```dart
String getPath()
```

**Example:**

```dart
final result = instance.getPath();
```

**Returns:** `String`

###### getMethod()

Get the configured method

**Signature:**

```dart
String getMethod()
```

**Example:**

```dart
final result = instance.getMethod();
```

**Returns:** `String`

###### isPlaygroundEnabled()

Check if playground is enabled

**Signature:**

```dart
bool isPlaygroundEnabled()
```

**Example:**

```dart
final result = instance.isPlaygroundEnabled();
```

**Returns:** `bool`

###### getDescription()

Get the description if set

**Signature:**

```dart
String? getDescription()
```

**Example:**

```dart
final result = instance.getDescription();
```

**Returns:** `String?`

###### default()

**Signature:**

```dart
static GraphQlRouteConfig default()
```

**Example:**

```dart
final result = GraphQlRouteConfig.default();
```

**Returns:** `GraphQlRouteConfig`

---

#### GraphQlSubscriptionSnapshot

Snapshot of a GraphQL subscription exchange over WebSocket.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `operationId` | `String` | — | Operation id used for the subscription request. |
| `acknowledged` | `bool` | — | Whether the server acknowledged the GraphQL WebSocket connection. |
| `event` | `String?` | `null` | First `next.payload` received for this subscription, if any. |
| `errors` | `List<String>` | — | GraphQL protocol errors emitted by the server. |
| `completeReceived` | `bool` | — | Whether a `complete` frame was observed for this operation. |

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
| `enabled` | `bool` | `true` | Enable gRPC support |
| `maxMessageSize` | `int` | — | Maximum message size in bytes (for both sending and receiving) This limit applies to individual messages in both unary and streaming RPCs. When a single message exceeds this size, the request is rejected with HTTP 413 (Payload Too Large). Default: 4MB (4194304 bytes) **Note:** This limit does NOT apply to the total response size in streaming RPCs. For multi-message streams, the total response can exceed this limit as long as each individual message stays within the limit. |
| `enableCompression` | `bool` | `true` | Enable gzip compression for gRPC messages |
| `requestTimeout` | `int?` | `null` | Timeout for gRPC requests in seconds (None = no timeout) |
| `maxConcurrentStreams` | `int` | — | Maximum number of concurrent streams per connection (HTTP/2 advisory) This value is communicated to HTTP/2 clients as the server's flow control limit. The HTTP/2 transport layer enforces this limit automatically via SETTINGS frames and GOAWAY responses. Applications should NOT implement custom enforcement. Default: 100 streams per connection # Stream Limiting Strategy - **Per Connection**: This limit applies per HTTP/2 connection, not globally - **Transport Enforcement**: HTTP/2 handles all stream limiting; applications need not implement custom checks - **Streaming Requests**: In server streaming or bidi streaming, each logical RPC consumes one stream slot. Message ordering within a stream follows HTTP/2 frame ordering. |
| `enableKeepalive` | `bool` | `true` | Enable HTTP/2 keepalive |
| `keepaliveInterval` | `int` | — | HTTP/2 keepalive interval in seconds |
| `keepaliveTimeout` | `int` | — | HTTP/2 keepalive timeout in seconds |
| `maxStreamResponseBytes` | `int?` | `null` | Total byte cap across an entire streaming response. When `Some(n)`, the streaming adapter aborts the stream with `tonic.Status.resource_exhausted` once the cumulative encoded message bytes exceed `n`. The stream yields the error item and then terminates. Per-message cap remains `max_message_size`. This limit applies to server-streaming and bidirectional-streaming RPCs only; unary RPCs are governed solely by `max_message_size`. Default: `null` (unbounded total response size). |

##### Methods

###### default()

**Signature:**

```dart
static GrpcConfig default()
```

**Example:**

```dart
final result = GrpcConfig.default();
```

**Returns:** `GrpcConfig`

---

#### IntoHandler

Convert user-facing handler functions into the low-level `Handler` trait.

##### Methods

###### intoHandler()

Convert this value into a shared request handler.

**Signature:**

```dart
Handler intoHandler()
```

**Example:**

```dart
final result = instance.intoHandler();
```

**Returns:** `Handler`

---

#### JsonRpcConfig

JSON-RPC server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `true` | Enable JSON-RPC endpoint |
| `endpointPath` | `String` | — | HTTP endpoint path for JSON-RPC requests (default: "/rpc") |
| `enableBatch` | `bool` | — | Enable batch request processing (default: true) |
| `maxBatchSize` | `int` | — | Maximum number of requests in a batch (default: 100) |

##### Methods

###### default()

**Signature:**

```dart
static JsonRpcConfig default()
```

**Example:**

```dart
final result = JsonRpcConfig.default();
```

**Returns:** `JsonRpcConfig`

---

#### JsonRpcMethodInfo

JSON-RPC method metadata for routes that support JSON-RPC

This struct captures the metadata needed to expose HTTP routes as JSON-RPC methods,
enabling discovery and documentation of RPC-compatible endpoints.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `methodName` | `String` | — | The JSON-RPC method name (e.g., "user.create") |
| `description` | `String?` | `null` | Optional description of what the method does |
| `paramsSchema` | `String?` | `null` | Optional JSON Schema for method parameters |
| `resultSchema` | `String?` | `null` | Optional JSON Schema for the result |
| `deprecated` | `bool` | `/* serde(default) */` | Whether this method is deprecated |
| `tags` | `List<String>` | `/* serde(default) */` | Tags for categorizing and grouping methods |

---

#### JwtConfig

JWT authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `secret` | `String` | — | Secret key for JWT verification |
| `algorithm` | `String` | `serde(default = "default_jwt_algorithm")` | Required algorithm (HS256, HS384, HS512, RS256, etc.) |
| `audience` | `List<String>?` | `null` | Required audience claim |
| `issuer` | `String?` | `null` | Required issuer claim |
| `leeway` | `int` | `/* serde(default) */` | Leeway for expiration checks (seconds) |

---

#### LicenseInfo

License information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | SPDX license identifier or display name (e.g. `"MIT"`). |
| `url` | `String?` | `null` | URL to the full license text. |

---

#### OpenApiConfig

OpenAPI configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `false` | Enable OpenAPI generation (default: false for zero overhead) |
| `title` | `String` | `"API"` | API title |
| `version` | `String` | `"1.0.0"` | API version |
| `description` | `String?` | `null` | API description (supports markdown) |
| `swaggerUiPath` | `String` | — | Path to serve Swagger UI (default: "/docs") |
| `redocPath` | `String` | — | Path to serve Redoc (default: "/redoc") |
| `openapiJsonPath` | `String` | — | Path to serve OpenAPI JSON spec (default: "/openapi.json") |
| `contact` | `ContactInfo?` | `null` | Contact information |
| `license` | `LicenseInfo?` | `null` | License information |
| `servers` | `List<ServerInfo>` | `\[\]` | Server definitions |
| `securitySchemes` | `Map<String, SecuritySchemeInfo>` | `{}` | Security schemes (auto-detected from middleware if not provided) |

##### Methods

###### default()

**Signature:**

```dart
static OpenApiConfig default()
```

**Example:**

```dart
final result = OpenApiConfig.default();
```

**Returns:** `OpenApiConfig`

---

#### ParseRequest

Request body for `POST /asyncapi/parse`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `spec` | `String` | — | Spec |

---

#### ParseResult

Full parse result returned by `POST /asyncapi/parse`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `specVersion` | `String` | — | Spec version |
| `title` | `String` | — | Title |
| `apiVersion` | `String` | — | Api version |
| `channels` | `List<ParsedChannel>` | — | Channels |
| `operations` | `List<ParsedOperation>` | — | Operations |
| `messages` | `List<ParsedMessage>` | — | Messages |

---

#### ParsedChannel

A single channel extracted from an AsyncAPI spec

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | Channel key from the spec (e.g. "chat/messages") |
| `address` | `String` | — | Channel address / path |
| `messages` | `List<String>` | — | Message names declared on this channel |
| `bindings` | `String?` | `null` | Bindings (ws / http / amqp / …) as raw JSON for forward-compatibility |

---

#### ParsedMessage

A resolved message (name + JSON Schema)

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | Message name |
| `schema` | `String?` | `null` | Resolved JSON Schema for the message payload, if available |

---

#### ParsedOperation

A single operation extracted from an AsyncAPI spec

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | Operation name |
| `action` | `String` | — | Operation action: "send" or "receive" |
| `channel` | `String` | — | Channel reference (resolved to the channel name) |

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
| `typeUri` | `String` | — | A URI reference that identifies the problem type. Defaults to "about:blank" when absent. Should be a stable, human-readable identifier for the problem type. |
| `title` | `String` | — | A short, human-readable summary of the problem type. Should not change from occurrence to occurrence of the problem. |
| `status` | `int` | — | The HTTP status code generated by the origin server. This is advisory; the actual HTTP status code takes precedence. |
| `detail` | `String?` | `null` | A human-readable explanation specific to this occurrence of the problem. |
| `instance` | `String?` | `null` | A URI reference that identifies the specific occurrence of the problem. It may or may not yield further information if dereferenced. |
| `extensions` | `Map<String, String>` | — | Extension members - problem-type-specific data. For validation errors, this typically contains an "errors" array. |

##### Methods

###### withDetail()

Set the detail field

**Signature:**

```dart
ProblemDetails withDetail(String detail)
```

**Example:**

```dart
final result = instance.withDetail("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `detail` | `String` | Yes | The detail |

**Returns:** `ProblemDetails`

###### withInstance()

Set the instance field

**Signature:**

```dart
ProblemDetails withInstance(String instance)
```

**Example:**

```dart
final result = instance.withInstance("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `instance` | `String` | Yes | The instance |

**Returns:** `ProblemDetails`

###### notFound()

Create a not found error

**Signature:**

```dart
static ProblemDetails notFound(String detail)
```

**Example:**

```dart
final result = ProblemDetails.notFound("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `detail` | `String` | Yes | The detail |

**Returns:** `ProblemDetails`

###### methodNotAllowed()

Create a method not allowed error

**Signature:**

```dart
static ProblemDetails methodNotAllowed(String detail)
```

**Example:**

```dart
final result = ProblemDetails.methodNotAllowed("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `detail` | `String` | Yes | The detail |

**Returns:** `ProblemDetails`

###### internalServerError()

Create an internal server error

**Signature:**

```dart
static ProblemDetails internalServerError(String detail)
```

**Example:**

```dart
final result = ProblemDetails.internalServerError("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `detail` | `String` | Yes | The detail |

**Returns:** `ProblemDetails`

###### badRequest()

Create a bad request error

**Signature:**

```dart
static ProblemDetails badRequest(String detail)
```

**Example:**

```dart
final result = ProblemDetails.badRequest("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `detail` | `String` | Yes | The detail |

**Returns:** `ProblemDetails`

###### toJson()

Serialize to JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```dart
String toJson()
```

**Example:**

```dart
final result = instance.toJson();
```

**Returns:** `String`

**Errors:** Throws `Error`.

###### toJsonPretty()

Serialize to pretty JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```dart
String toJsonPretty()
```

**Example:**

```dart
final result = instance.toJsonPretty();
```

**Returns:** `String`

**Errors:** Throws `Error`.

---

#### QueryMutationConfig

Configuration for schemas with Query and Mutation types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `bool` | `true` | Enable introspection queries |
| `complexityLimit` | `int?` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `int?` | `null` | Maximum query depth (None = unlimited) |

##### Methods

###### default()

**Signature:**

```dart
static QueryMutationConfig default()
```

**Example:**

```dart
final result = QueryMutationConfig.default();
```

**Returns:** `QueryMutationConfig`

---

#### QueryOnlyConfig

Configuration for schemas with only Query type

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `bool` | `true` | Enable introspection queries |
| `complexityLimit` | `int?` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `int?` | `null` | Maximum query depth (None = unlimited) |

##### Methods

###### default()

**Signature:**

```dart
static QueryOnlyConfig default()
```

**Example:**

```dart
final result = QueryOnlyConfig.default();
```

**Returns:** `QueryOnlyConfig`

---

#### RateLimitConfig

Rate limiting configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `perSecond` | `int` | `100` | Requests per second |
| `burst` | `int` | `200` | Burst allowance |
| `ipBased` | `bool` | `true` | Use IP-based rate limiting |

##### Methods

###### default()

**Signature:**

```dart
static RateLimitConfig default()
```

**Example:**

```dart
final result = RateLimitConfig.default();
```

**Returns:** `RateLimitConfig`

---

#### Request

---

#### Response

HTTP Response with custom status code, headers, and content

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String?` | `null` | Response body content |
| `statusCode` | `int` | — | HTTP status code (defaults to 200) |
| `headers` | `Map<String, String>` | `{}` | Response headers |

##### Methods

###### setHeader()

Set a header

**Signature:**

```dart
void setHeader(String key, String value)
```

**Example:**

```dart
instance.setHeader("value", "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `key` | `String` | Yes | The key |
| `value` | `String` | Yes | The value |

**Returns:** No return value.

###### setCookie()

Set a cookie in the response

**Signature:**

```dart
void setCookie(String key, String value, bool secure, bool httpOnly, [int? maxAge, String? domain, String? path, String? sameSite])
```

**Example:**

```dart
instance.setCookie("value", "value", true, true, 42, "value", "value", "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `key` | `String` | Yes | The key |
| `value` | `String` | Yes | The value |
| `secure` | `bool` | Yes | The secure |
| `httpOnly` | `bool` | Yes | The http only |
| `maxAge` | `int?` | No | The max age |
| `domain` | `String?` | No | The domain |
| `path` | `String?` | No | Path to the file |
| `sameSite` | `String?` | No | The same site |

**Returns:** No return value.

###### default()

**Signature:**

```dart
static Response default()
```

**Example:**

```dart
final result = Response.default();
```

**Returns:** `Response`

---

#### ResponseSnapshot

Snapshot of an Axum response used by higher-level language bindings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `status` | `int` | — | HTTP status code. |
| `headers` | `Map<String, String>` | — | Response headers (lowercase keys for predictable lookups). |
| `body` | `Uint8List` | — | Response body bytes (decoded for supported encodings). |

##### Methods

###### text()

Return response body as UTF-8 string.

**Signature:**

```dart
String text()
```

**Example:**

```dart
final result = instance.text();
```

**Returns:** `String`

**Errors:** Throws `FromUtf8Error`.

###### header()

Lookup header by case-insensitive name.

**Signature:**

```dart
String? header(String name)
```

**Example:**

```dart
final result = instance.header("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `String?`

---

#### RouteBuilder

Builder for defining a route.

##### Methods

###### new()

Create a new builder for the provided HTTP method and path.

**Signature:**

```dart
static RouteBuilder new(Method method, String path)
```

**Example:**

```dart
final result = RouteBuilder.new(Method(), "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `method` | `Method` | Yes | The method |
| `path` | `String` | Yes | Path to the file |

**Returns:** `RouteBuilder`

###### handlerName()

Assign an explicit handler name.

**Signature:**

```dart
RouteBuilder handlerName(String name)
```

**Example:**

```dart
final result = instance.handlerName("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `RouteBuilder`

###### requestSchemaJson()

Provide a raw JSON schema for the request body.

**Signature:**

```dart
RouteBuilder requestSchemaJson(String schema)
```

**Example:**

```dart
final result = instance.requestSchemaJson({});
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `schema` | `String` | Yes | The schema |

**Returns:** `RouteBuilder`

###### responseSchemaJson()

Provide a raw JSON schema for the response body.

**Signature:**

```dart
RouteBuilder responseSchemaJson(String schema)
```

**Example:**

```dart
final result = instance.responseSchemaJson({});
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `schema` | `String` | Yes | The schema |

**Returns:** `RouteBuilder`

###### paramsSchemaJson()

Provide a raw JSON schema for request parameters.

**Signature:**

```dart
RouteBuilder paramsSchemaJson(String schema)
```

**Example:**

```dart
final result = instance.paramsSchemaJson({});
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `schema` | `String` | Yes | The schema |

**Returns:** `RouteBuilder`

###### fileParamsJson()

Provide multipart file parameter configuration.

**Signature:**

```dart
RouteBuilder fileParamsJson(String schema)
```

**Example:**

```dart
final result = instance.fileParamsJson({});
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `schema` | `String` | Yes | The schema |

**Returns:** `RouteBuilder`

###### cors()

Attach a CORS configuration for this route.

**Signature:**

```dart
RouteBuilder cors(CorsConfig cors)
```

**Example:**

```dart
final result = instance.cors(CorsConfig());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `cors` | `CorsConfig` | Yes | The cors config |

**Returns:** `RouteBuilder`

###### compression()

Attach a compression configuration for this route.

**Signature:**

```dart
RouteBuilder compression(CompressionConfig compression)
```

**Example:**

```dart
final result = instance.compression(CompressionConfig());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `compression` | `CompressionConfig` | Yes | The compression config |

**Returns:** `RouteBuilder`

###### sync()

Mark the route as synchronous.

**Signature:**

```dart
RouteBuilder sync()
```

**Example:**

```dart
final result = instance.sync();
```

**Returns:** `RouteBuilder`

###### handlerDependencies()

Declare the dependency keys that must be resolved before this handler runs.

**Signature:**

```dart
RouteBuilder handlerDependencies(List<String> dependencies)
```

**Example:**

```dart
final result = instance.handlerDependencies([]);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `dependencies` | `List<String>` | Yes | The dependencies |

**Returns:** `RouteBuilder`

---

#### SchemaConfig

Configuration for GraphQL schema building.

Encapsulates all schema-level configuration options including
introspection control, complexity limits, and depth limits.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `bool` | `true` | Enable introspection queries |
| `complexityLimit` | `int?` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `int?` | `null` | Maximum query depth (None = unlimited) |

##### Methods

###### default()

**Signature:**

```dart
static SchemaConfig default()
```

**Example:**

```dart
final result = SchemaConfig.default();
```

**Returns:** `SchemaConfig`

---

#### ServerConfig

Server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `host` | `String` | `"127.0.0.1"` | Host to bind to |
| `port` | `int` | `8000` | Port to bind to |
| `workers` | `int` | `1` | Number of Tokio runtime worker threads used by binding-managed server runtimes |
| `enableRequestId` | `bool` | `false` | Enable request ID generation and propagation |
| `maxBodySize` | `int?` | `null` | Maximum request body size in bytes (None = unlimited, not recommended) |
| `requestTimeout` | `int?` | `null` | Request timeout in seconds (None = no timeout) |
| `compression` | `CompressionConfig?` | `null` | Enable compression middleware |
| `rateLimit` | `RateLimitConfig?` | `null` | Enable rate limiting |
| `jwtAuth` | `JwtConfig?` | `null` | JWT authentication configuration |
| `apiKeyAuth` | `ApiKeyConfig?` | `null` | API Key authentication configuration |
| `staticFiles` | `List<StaticFilesConfig>` | `\[\]` | Static file serving configuration |
| `gracefulShutdown` | `bool` | `true` | Enable graceful shutdown on SIGTERM/SIGINT |
| `shutdownTimeout` | `int` | `30` | Graceful shutdown timeout (seconds) |
| `asyncapi` | `AsyncApiConfig?` | `null` | AsyncAPI HTTP endpoint configuration |
| `openapi` | `OpenApiConfig?` | `null` | OpenAPI documentation configuration |
| `jsonrpc` | `JsonRpcConfig?` | `null` | JSON-RPC configuration |
| `grpc` | `GrpcConfig?` | `null` | gRPC configuration |
| `backgroundTasks` | `BackgroundTaskConfig` | — | Background task executor configuration |
| `enableHttpTrace` | `bool` | `false` | Enable per-request HTTP tracing (tower-http `TraceLayer`) |

##### Methods

###### default()

**Signature:**

```dart
static ServerConfig default()
```

**Example:**

```dart
final result = ServerConfig.default();
```

**Returns:** `ServerConfig`

---

#### ServerInfo

Server information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | Base URL of the server (e.g. `"<https://api.example.com/v1"`>). |
| `description` | `String?` | `null` | Optional human-readable description of the server environment. |

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
| `eventType` | `String?` | `null` | Event type (optional) |
| `data` | `String` | — | Event data (JSON value) |
| `id` | `String?` | `null` | Event ID (optional, for client-side reconnection) |
| `retry` | `int?` | `null` | Retry timeout in milliseconds (optional) |

##### Methods

###### withId()

Set the event ID for client-side reconnection support

Sets an ID that clients can use to resume from this point if they disconnect.
The client sends this ID back in the `Last-Event-ID` header when reconnecting.

**Signature:**

```dart
SseEvent withId(String id)
```

**Example:**

```dart
final result = instance.withId("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `String` | Yes | Unique identifier for this event |

**Returns:** `SseEvent`

###### withRetry()

Set the retry timeout for client reconnection

Sets the time in milliseconds clients should wait before attempting to reconnect
if the connection is lost. The client browser will automatically handle reconnection.

**Signature:**

```dart
SseEvent withRetry(int retryMs)
```

**Example:**

```dart
final result = instance.withRetry(42);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `retryMs` | `int` | Yes | Retry timeout in milliseconds |

**Returns:** `SseEvent`

---

#### StaticFilesConfig

Static file serving configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `directory` | `String` | — | Directory path to serve |
| `routePrefix` | `String` | — | URL path prefix (e.g., "/static") |
| `indexFile` | `bool` | `serde(default = "default_true")` | Fallback to index.html for directories |
| `cacheControl` | `String?` | `null` | Cache-Control header value |

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

```dart
ResponseSnapshot graphqlAt(String endpoint, String query, [String? variables, String? operationName])
```

**Example:**

```dart
final result = await instance.graphqlAt("value", "value", {}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `endpoint` | `String` | Yes | The endpoint |
| `query` | `String` | Yes | The query |
| `variables` | `String?` | No | The variables |
| `operationName` | `String?` | No | The operation name |

**Returns:** `ResponseSnapshot`

**Errors:** Throws `SnapshotError`.

###### graphql()

Send a GraphQL query/mutation

**Signature:**

```dart
ResponseSnapshot graphql(String query, [String? variables, String? operationName])
```

**Example:**

```dart
final result = await instance.graphql("value", {}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `query` | `String` | Yes | The query |
| `variables` | `String?` | No | The variables |
| `operationName` | `String?` | No | The operation name |

**Returns:** `ResponseSnapshot`

**Errors:** Throws `SnapshotError`.

###### graphqlSubscriptionAt()

Send a GraphQL subscription (WebSocket) to a custom endpoint.

Uses the `graphql-transport-ws` protocol and captures the first `next` payload.
After the first payload is received, this client sends `complete` to unsubscribe.

**Signature:**

```dart
GraphQlSubscriptionSnapshot graphqlSubscriptionAt(String endpoint, String query, [String? variables, String? operationName])
```

**Example:**

```dart
final result = await instance.graphqlSubscriptionAt("value", "value", {}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `endpoint` | `String` | Yes | The endpoint |
| `query` | `String` | Yes | The query |
| `variables` | `String?` | No | The variables |
| `operationName` | `String?` | No | The operation name |

**Returns:** `GraphQlSubscriptionSnapshot`

**Errors:** Throws `SnapshotError`.

###### graphqlSubscription()

Send a GraphQL subscription (WebSocket).

Uses `/graphql` as the default subscription endpoint.

**Signature:**

```dart
GraphQlSubscriptionSnapshot graphqlSubscription(String query, [String? variables, String? operationName])
```

**Example:**

```dart
final result = await instance.graphqlSubscription("value", {}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `query` | `String` | Yes | The query |
| `variables` | `String?` | No | The variables |
| `operationName` | `String?` | No | The operation name |

**Returns:** `GraphQlSubscriptionSnapshot`

**Errors:** Throws `SnapshotError`.

---

#### TestingSseEvent

A single Server-Sent Event.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data` | `String` | — | The data field of the event. |

---

#### UploadFile

Represents an uploaded file from multipart/form-data requests.

This struct provides efficient access to file content with automatic
base64 decoding and implements standard I/O traits for compatibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `filename` | `String` | — | Original filename from the client |
| `contentType` | `String?` | `null` | MIME type of the uploaded file |
| `size` | `int?` | `null` | Size of the file in bytes |
| `content` | `Uint8List` | — | File content (may be base64 encoded) |
| `contentEncoding` | `String?` | `null` | Content encoding type |

##### Methods

###### asBytes()

Get the raw file content as bytes.

This provides zero-copy access to the underlying buffer.

**Signature:**

```dart
Uint8List asBytes()
```

**Example:**

```dart
final result = instance.asBytes();
```

**Returns:** `Uint8List`

###### readToString()

Read the file content as a UTF-8 string.

**Errors:**

Returns an error if the content is not valid UTF-8.

**Signature:**

```dart
String readToString()
```

**Example:**

```dart
final result = instance.readToString();
```

**Returns:** `String`

**Errors:** Throws `Error`.

###### contentTypeOrDefault()

Get the content type, defaulting to "application/octet-stream".

**Signature:**

```dart
String contentTypeOrDefault()
```

**Example:**

```dart
final result = instance.contentTypeOrDefault();
```

**Returns:** `String`

---

#### ValidateRequest

Request body for `POST /asyncapi/validate`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `spec` | `String` | — | Spec |
| `channel` | `String` | — | Channel |
| `message` | `String` | — | Message |
| `payload` | `String` | — | Payload |

---

#### ValidationResponse

Response body for `POST /asyncapi/validate`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `valid` | `bool` | — | Valid |
| `errors` | `List<String>` | — | Errors |

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
| `Http` | Http — Fields: `scheme`: `String`, `bearerFormat`: `String` |
| `ApiKey` | Api key — Fields: `location`: `String`, `name`: `String` |

---

#### SnapshotError

Possible errors while converting an Axum response into a snapshot.

| Value | Description |
|-------|-------------|
| `InvalidHeader` | Response header could not be decoded to UTF-8. — Fields: `0`: `String` |
| `Decompression` | Body decompression failed. — Fields: `0`: `String` |

---

#### WebSocketMessage

A WebSocket message that can be text or binary.

| Value | Description |
|-------|-------------|
| `Text` | A text message. — Fields: `0`: `String` |
| `Binary` | A binary message. — Fields: `0`: `Uint8List` |
| `Close` | A close message with a numeric close code (RFC 6455) and optional reason text. Common codes: 1000 Normal Closure, 1001 Going Away, 1005 No Status Received, 1006 Abnormal Closure. — Fields: `code`: `int`, `reason`: `String` |
| `Ping` | A ping message. — Fields: `0`: `Uint8List` |
| `Pong` | A pong message. — Fields: `0`: `Uint8List` |

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

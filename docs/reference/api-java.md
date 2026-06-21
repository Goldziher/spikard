---
title: "Java API Reference"
---

## Java API Reference <span class="version-badge">v0.16.0-rc.3</span>

### Functions

#### schemaQueryOnly()

Create a simple schema configuration with only Query type.

This is a convenience function for schemas that only have queries.

**Returns:**

A `QueryOnlyConfig` with default settings

**Signature:**

```java
public static QueryOnlyConfig schemaQueryOnly()
```

**Example:**

```java
var result = schemaQueryOnly();
```

**Returns:** `QueryOnlyConfig`

---

#### schemaQueryMutation()

Create a schema configuration with Query and Mutation types.

This is a convenience function for schemas with queries and mutations but no subscriptions.

**Returns:**

A `QueryMutationConfig` with default settings

**Signature:**

```java
public static QueryMutationConfig schemaQueryMutation()
```

**Example:**

```java
var result = schemaQueryMutation();
```

**Returns:** `QueryMutationConfig`

---

#### schemaFull()

Create a schema configuration with all three root types.

This is a convenience function for fully-featured schemas.

**Returns:**

A `FullSchemaConfig` with default settings

**Signature:**

```java
public static FullSchemaConfig schemaFull()
```

**Example:**

```java
var result = schemaFull();
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
| `enabled` | `boolean` | — | Enable AsyncAPI endpoints (default: false) |
| `spec` | `Optional<Object>` | `null` | Pre-registered AsyncAPI spec to serve from GET /asyncapi.json |

---

#### BackgroundJobMetadata

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `requestId` | `Optional<String>` | `null` | Request id |

##### Methods

###### defaultOptions()

**Signature:**

```java
public static BackgroundJobMetadata defaultOptions()
```

**Example:**

```java
var result = BackgroundJobMetadata.defaultOptions();
```

**Returns:** `BackgroundJobMetadata`

---

#### BackgroundTaskConfig

Configuration for in-process background task execution.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `maxQueueSize` | `long` | `1024` | Maximum queue size |
| `maxConcurrentTasks` | `long` | `128` | Maximum concurrent tasks |
| `drainTimeoutSecs` | `long` | `30` | Drain timeout secs |

##### Methods

###### defaultOptions()

**Signature:**

```java
public static BackgroundTaskConfig defaultOptions()
```

**Example:**

```java
var result = BackgroundTaskConfig.defaultOptions();
```

**Returns:** `BackgroundTaskConfig`

---

#### CompressionConfig

Compression configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `gzip` | `boolean` | `true` | Enable gzip compression |
| `brotli` | `boolean` | `true` | Enable brotli compression |
| `minSize` | `long` | — | Minimum response size to compress (bytes) |
| `quality` | `int` | — | Compression quality (0-11 for brotli, 0-9 for gzip) |

##### Methods

###### defaultOptions()

**Signature:**

```java
public static CompressionConfig defaultOptions()
```

**Example:**

```java
var result = CompressionConfig.defaultOptions();
```

**Returns:** `CompressionConfig`

---

#### ContactInfo

Contact information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `Optional<String>` | `null` | Name of the contact person or organisation. |
| `email` | `Optional<String>` | `null` | Contact email address. |
| `url` | `Optional<String>` | `null` | URL pointing to the contact information page. |

---

#### CorsConfig

CORS configuration for a route

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `allowedOrigins` | `List<String>` | `Collections.emptyList()` | Allowed origins |
| `allowedMethods` | `List<String>` | `Collections.emptyList()` | Allowed methods |
| `allowedHeaders` | `List<String>` | `Collections.emptyList()` | Allowed headers |
| `exposeHeaders` | `Optional<List<String>>` | `null` | Expose headers |
| `maxAge` | `Optional<Integer>` | `null` | Maximum age |
| `allowCredentials` | `Optional<Boolean>` | `null` | Allow credentials |

##### Methods

###### allowedMethodsJoined()

Get the cached joined methods string for preflight responses

**Signature:**

```java
public String allowedMethodsJoined()
```

**Example:**

```java
var result = instance.allowedMethodsJoined();
```

**Returns:** `String`

###### allowedHeadersJoined()

Get the cached joined headers string for preflight responses

**Signature:**

```java
public String allowedHeadersJoined()
```

**Example:**

```java
var result = instance.allowedHeadersJoined();
```

**Returns:** `String`

###### isOriginAllowed()

Check if an origin is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```java
public boolean isOriginAllowed(String origin)
```

**Example:**

```java
var result = instance.isOriginAllowed("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `origin` | `String` | Yes | The origin |

**Returns:** `boolean`

###### isMethodAllowed()

Check if a method is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```java
public boolean isMethodAllowed(String method)
```

**Example:**

```java
var result = instance.isMethodAllowed("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `method` | `String` | Yes | The method |

**Returns:** `boolean`

###### defaultOptions()

**Signature:**

```java
public static CorsConfig defaultOptions()
```

**Example:**

```java
var result = CorsConfig.defaultOptions();
```

**Returns:** `CorsConfig`

---

#### FullSchemaConfig

Configuration for fully-featured schemas with Query, Mutation, and Subscription types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `boolean` | `true` | Enable introspection queries |
| `complexityLimit` | `Optional<Long>` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `Optional<Long>` | `null` | Maximum query depth (None = unlimited) |

##### Methods

###### defaultOptions()

**Signature:**

```java
public static FullSchemaConfig defaultOptions()
```

**Example:**

```java
var result = FullSchemaConfig.defaultOptions();
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

```java
public static GraphQlRouteConfig new()
```

**Example:**

```java
var result = GraphQlRouteConfig.new();
```

**Returns:** `GraphQlRouteConfig`

###### path()

Set the HTTP path for the GraphQL endpoint

**Signature:**

```java
public GraphQlRouteConfig path(String path)
```

**Example:**

```java
var result = instance.path("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `String` | Yes | The URL path (e.g., "/graphql", "/api/graphql") |

**Returns:** `GraphQlRouteConfig`

###### method()

Set the HTTP method for the GraphQL endpoint

**Signature:**

```java
public GraphQlRouteConfig method(String method)
```

**Example:**

```java
var result = instance.method("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `method` | `String` | Yes | The HTTP method (typically "POST") |

**Returns:** `GraphQlRouteConfig`

###### enablePlayground()

Enable or disable the GraphQL Playground UI

**Signature:**

```java
public GraphQlRouteConfig enablePlayground(boolean enable)
```

**Example:**

```java
var result = instance.enablePlayground(true);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `enable` | `boolean` | Yes | Whether to enable playground |

**Returns:** `GraphQlRouteConfig`

###### description()

Set a custom description for documentation

**Signature:**

```java
public GraphQlRouteConfig description(String description)
```

**Example:**

```java
var result = instance.description("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `description` | `String` | Yes | Documentation string |

**Returns:** `GraphQlRouteConfig`

###### getPath()

Get the configured path

**Signature:**

```java
public String getPath()
```

**Example:**

```java
var result = instance.getPath();
```

**Returns:** `String`

###### getMethod()

Get the configured method

**Signature:**

```java
public String getMethod()
```

**Example:**

```java
var result = instance.getMethod();
```

**Returns:** `String`

###### isPlaygroundEnabled()

Check if playground is enabled

**Signature:**

```java
public boolean isPlaygroundEnabled()
```

**Example:**

```java
var result = instance.isPlaygroundEnabled();
```

**Returns:** `boolean`

###### getDescription()

Get the description if set

**Signature:**

```java
public Optional<String> getDescription()
```

**Example:**

```java
var result = instance.getDescription();
```

**Returns:** `Optional<String>`

###### defaultOptions()

**Signature:**

```java
public static GraphQlRouteConfig defaultOptions()
```

**Example:**

```java
var result = GraphQlRouteConfig.defaultOptions();
```

**Returns:** `GraphQlRouteConfig`

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
| `maxMessageSize` | `long` | — | Maximum message size in bytes (for both sending and receiving) This limit applies to individual messages in both unary and streaming RPCs. When a single message exceeds this size, the request is rejected with HTTP 413 (Payload Too Large). Default: 4MB (4194304 bytes) **Note:** This limit does NOT apply to the total response size in streaming RPCs. For multi-message streams, the total response can exceed this limit as long as each individual message stays within the limit. |
| `enableCompression` | `boolean` | `true` | Enable gzip compression for gRPC messages |
| `requestTimeout` | `Optional<Long>` | `null` | Timeout for gRPC requests in seconds (None = no timeout) |
| `maxConcurrentStreams` | `int` | — | Maximum number of concurrent streams per connection (HTTP/2 advisory) This value is communicated to HTTP/2 clients as the server's flow control limit. The HTTP/2 transport layer enforces this limit automatically via SETTINGS frames and GOAWAY responses. Applications should NOT implement custom enforcement. Default: 100 streams per connection # Stream Limiting Strategy - **Per Connection**: This limit applies per HTTP/2 connection, not globally - **Transport Enforcement**: HTTP/2 handles all stream limiting; applications need not implement custom checks - **Streaming Requests**: In server streaming or bidi streaming, each logical RPC consumes one stream slot. Message ordering within a stream follows HTTP/2 frame ordering. |
| `enableKeepalive` | `boolean` | `true` | Enable HTTP/2 keepalive |
| `keepaliveInterval` | `long` | — | HTTP/2 keepalive interval in seconds |
| `keepaliveTimeout` | `long` | — | HTTP/2 keepalive timeout in seconds |
| `maxStreamResponseBytes` | `Optional<Long>` | `null` | Total byte cap across an entire streaming response. When `Some(n)`, the streaming adapter aborts the stream with `tonic.Status.resource_exhausted` once the cumulative encoded message bytes exceed `n`. The stream yields the error item and then terminates. Per-message cap remains `max_message_size`. This limit applies to server-streaming and bidirectional-streaming RPCs only; unary RPCs are governed solely by `max_message_size`. Default: `null` (unbounded total response size). |

##### Methods

###### defaultOptions()

**Signature:**

```java
public static GrpcConfig defaultOptions()
```

**Example:**

```java
var result = GrpcConfig.defaultOptions();
```

**Returns:** `GrpcConfig`

---

#### IntoHandler

Convert user-facing handler functions into the low-level `Handler` trait.

##### Methods

###### intoHandler()

Convert this value into a shared request handler.

**Signature:**

```java
public Handler intoHandler()
```

**Example:**

```java
var result = instance.intoHandler();
```

**Returns:** `Handler`

---

#### JsonRpcConfig

JSON-RPC server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `boolean` | `true` | Enable JSON-RPC endpoint |
| `endpointPath` | `String` | — | HTTP endpoint path for JSON-RPC requests (default: "/rpc") |
| `enableBatch` | `boolean` | — | Enable batch request processing (default: true) |
| `maxBatchSize` | `long` | — | Maximum number of requests in a batch (default: 100) |

##### Methods

###### defaultOptions()

**Signature:**

```java
public static JsonRpcConfig defaultOptions()
```

**Example:**

```java
var result = JsonRpcConfig.defaultOptions();
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
| `description` | `Optional<String>` | `null` | Optional description of what the method does |
| `paramsSchema` | `Optional<Object>` | `null` | Optional JSON Schema for method parameters |
| `resultSchema` | `Optional<Object>` | `null` | Optional JSON Schema for the result |
| `deprecated` | `boolean` | `/* serde(default) */` | Whether this method is deprecated |
| `tags` | `List<String>` | `/* serde(default) */` | Tags for categorizing and grouping methods |

---

#### JwtConfig

JWT authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `secret` | `String` | — | Secret key for JWT verification |
| `algorithm` | `String` | `serde(default = "default_jwt_algorithm")` | Required algorithm (HS256, HS384, HS512, RS256, etc.) |
| `audience` | `Optional<List<String>>` | `null` | Required audience claim |
| `issuer` | `Optional<String>` | `null` | Required issuer claim |
| `leeway` | `long` | `/* serde(default) */` | Leeway for expiration checks (seconds) |

---

#### LicenseInfo

License information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | SPDX license identifier or display name (e.g. `"MIT"`). |
| `url` | `Optional<String>` | `null` | URL to the full license text. |

---

#### OpenApiConfig

OpenAPI configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `boolean` | `false` | Enable OpenAPI generation (default: false for zero overhead) |
| `title` | `String` | `"API"` | API title |
| `version` | `String` | `"1.0.0"` | API version |
| `description` | `Optional<String>` | `null` | API description (supports markdown) |
| `swaggerUiPath` | `String` | — | Path to serve Swagger UI (default: "/docs") |
| `redocPath` | `String` | — | Path to serve Redoc (default: "/redoc") |
| `openapiJsonPath` | `String` | — | Path to serve OpenAPI JSON spec (default: "/openapi.json") |
| `contact` | `Optional<ContactInfo>` | `null` | Contact information |
| `license` | `Optional<LicenseInfo>` | `null` | License information |
| `servers` | `List<ServerInfo>` | `Collections.emptyList()` | Server definitions |
| `securitySchemes` | `Map<String, SecuritySchemeInfo>` | `Collections.emptyMap()` | Security schemes (auto-detected from middleware if not provided) |

##### Methods

###### defaultOptions()

**Signature:**

```java
public static OpenApiConfig defaultOptions()
```

**Example:**

```java
var result = OpenApiConfig.defaultOptions();
```

**Returns:** `OpenApiConfig`

---

#### ParseRequest

Request body for `POST /asyncapi/parse`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `spec` | `Object` | — | Spec |

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
| `bindings` | `Optional<Object>` | `null` | Bindings (ws / http / amqp / …) as raw JSON for forward-compatibility |

---

#### ParsedMessage

A resolved message (name + JSON Schema)

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | Message name |
| `schema` | `Optional<Object>` | `null` | Resolved JSON Schema for the message payload, if available |

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
| `status` | `short` | — | The HTTP status code generated by the origin server. This is advisory; the actual HTTP status code takes precedence. |
| `detail` | `Optional<String>` | `null` | A human-readable explanation specific to this occurrence of the problem. |
| `instance` | `Optional<String>` | `null` | A URI reference that identifies the specific occurrence of the problem. It may or may not yield further information if dereferenced. |
| `extensions` | `Map<String, Object>` | — | Extension members - problem-type-specific data. For validation errors, this typically contains an "errors" array. |

##### Methods

###### withDetail()

Set the detail field

**Signature:**

```java
public ProblemDetails withDetail(String detail)
```

**Example:**

```java
var result = instance.withDetail("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `detail` | `String` | Yes | The detail |

**Returns:** `ProblemDetails`

###### withInstance()

Set the instance field

**Signature:**

```java
public ProblemDetails withInstance(String instance)
```

**Example:**

```java
var result = instance.withInstance("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `instance` | `String` | Yes | The instance |

**Returns:** `ProblemDetails`

###### notFound()

Create a not found error

**Signature:**

```java
public static ProblemDetails notFound(String detail)
```

**Example:**

```java
var result = ProblemDetails.notFound("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `detail` | `String` | Yes | The detail |

**Returns:** `ProblemDetails`

###### methodNotAllowed()

Create a method not allowed error

**Signature:**

```java
public static ProblemDetails methodNotAllowed(String detail)
```

**Example:**

```java
var result = ProblemDetails.methodNotAllowed("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `detail` | `String` | Yes | The detail |

**Returns:** `ProblemDetails`

###### internalServerError()

Create an internal server error

**Signature:**

```java
public static ProblemDetails internalServerError(String detail)
```

**Example:**

```java
var result = ProblemDetails.internalServerError("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `detail` | `String` | Yes | The detail |

**Returns:** `ProblemDetails`

###### badRequest()

Create a bad request error

**Signature:**

```java
public static ProblemDetails badRequest(String detail)
```

**Example:**

```java
var result = ProblemDetails.badRequest("value");
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

```java
public String toJson() throws Error
```

**Example:**

```java
var result = instance.toJson();
```

**Returns:** `String`

**Errors:** Throws `ErrorException`.

###### toJsonPretty()

Serialize to pretty JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```java
public String toJsonPretty() throws Error
```

**Example:**

```java
var result = instance.toJsonPretty();
```

**Returns:** `String`

**Errors:** Throws `ErrorException`.

---

#### QueryMutationConfig

Configuration for schemas with Query and Mutation types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `boolean` | `true` | Enable introspection queries |
| `complexityLimit` | `Optional<Long>` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `Optional<Long>` | `null` | Maximum query depth (None = unlimited) |

##### Methods

###### defaultOptions()

**Signature:**

```java
public static QueryMutationConfig defaultOptions()
```

**Example:**

```java
var result = QueryMutationConfig.defaultOptions();
```

**Returns:** `QueryMutationConfig`

---

#### QueryOnlyConfig

Configuration for schemas with only Query type

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `boolean` | `true` | Enable introspection queries |
| `complexityLimit` | `Optional<Long>` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `Optional<Long>` | `null` | Maximum query depth (None = unlimited) |

##### Methods

###### defaultOptions()

**Signature:**

```java
public static QueryOnlyConfig defaultOptions()
```

**Example:**

```java
var result = QueryOnlyConfig.defaultOptions();
```

**Returns:** `QueryOnlyConfig`

---

#### RateLimitConfig

Rate limiting configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `perSecond` | `long` | `100` | Requests per second |
| `burst` | `int` | `200` | Burst allowance |
| `ipBased` | `boolean` | `true` | Use IP-based rate limiting |

##### Methods

###### defaultOptions()

**Signature:**

```java
public static RateLimitConfig defaultOptions()
```

**Example:**

```java
var result = RateLimitConfig.defaultOptions();
```

**Returns:** `RateLimitConfig`

---

#### Request

---

#### Response

HTTP Response with custom status code, headers, and content

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `Optional<Object>` | `null` | Response body content |
| `statusCode` | `short` | — | HTTP status code (defaults to 200) |
| `headers` | `Map<String, String>` | `Collections.emptyMap()` | Response headers |

##### Methods

###### setHeader()

Set a header

**Signature:**

```java
public void setHeader(String key, String value)
```

**Example:**

```java
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

```java
public void setCookie(String key, String value, boolean secure, boolean httpOnly, long maxAge, String domain, String path, String sameSite)
```

**Example:**

```java
instance.setCookie("value", "value", true, true, 42, "value", "value", "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `key` | `String` | Yes | The key |
| `value` | `String` | Yes | The value |
| `secure` | `boolean` | Yes | The secure |
| `httpOnly` | `boolean` | Yes | The http only |
| `maxAge` | `Optional<Long>` | No | The max age |
| `domain` | `Optional<String>` | No | The domain |
| `path` | `Optional<String>` | No | Path to the file |
| `sameSite` | `Optional<String>` | No | The same site |

**Returns:** No return value.

###### defaultOptions()

**Signature:**

```java
public static Response defaultOptions()
```

**Example:**

```java
var result = Response.defaultOptions();
```

**Returns:** `Response`

---

#### RouteBuilder

Builder for defining a route.

##### Methods

###### new()

Create a new builder for the provided HTTP method and path.

**Signature:**

```java
public static RouteBuilder new(Method method, String path)
```

**Example:**

```java
var result = RouteBuilder.new(new Method(), "value");
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

```java
public RouteBuilder handlerName(String name)
```

**Example:**

```java
var result = instance.handlerName("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `RouteBuilder`

###### requestSchemaJson()

Provide a raw JSON schema for the request body.

**Signature:**

```java
public RouteBuilder requestSchemaJson(Object schema)
```

**Example:**

```java
var result = instance.requestSchemaJson(Map.of());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `schema` | `Object` | Yes | The schema |

**Returns:** `RouteBuilder`

###### responseSchemaJson()

Provide a raw JSON schema for the response body.

**Signature:**

```java
public RouteBuilder responseSchemaJson(Object schema)
```

**Example:**

```java
var result = instance.responseSchemaJson(Map.of());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `schema` | `Object` | Yes | The schema |

**Returns:** `RouteBuilder`

###### paramsSchemaJson()

Provide a raw JSON schema for request parameters.

**Signature:**

```java
public RouteBuilder paramsSchemaJson(Object schema)
```

**Example:**

```java
var result = instance.paramsSchemaJson(Map.of());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `schema` | `Object` | Yes | The schema |

**Returns:** `RouteBuilder`

###### fileParamsJson()

Provide multipart file parameter configuration.

**Signature:**

```java
public RouteBuilder fileParamsJson(Object schema)
```

**Example:**

```java
var result = instance.fileParamsJson(Map.of());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `schema` | `Object` | Yes | The schema |

**Returns:** `RouteBuilder`

###### cors()

Attach a CORS configuration for this route.

**Signature:**

```java
public RouteBuilder cors(CorsConfig cors)
```

**Example:**

```java
var result = instance.cors(new CorsConfig());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `cors` | `CorsConfig` | Yes | The cors config |

**Returns:** `RouteBuilder`

###### compression()

Attach a compression configuration for this route.

**Signature:**

```java
public RouteBuilder compression(CompressionConfig compression)
```

**Example:**

```java
var result = instance.compression(new CompressionConfig());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `compression` | `CompressionConfig` | Yes | The compression config |

**Returns:** `RouteBuilder`

###### sync()

Mark the route as synchronous.

**Signature:**

```java
public RouteBuilder sync()
```

**Example:**

```java
var result = instance.sync();
```

**Returns:** `RouteBuilder`

###### handlerDependencies()

Declare the dependency keys that must be resolved before this handler runs.

**Signature:**

```java
public RouteBuilder handlerDependencies(List<String> dependencies)
```

**Example:**

```java
var result = instance.handlerDependencies(List.of());
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
| `introspectionEnabled` | `boolean` | `true` | Enable introspection queries |
| `complexityLimit` | `Optional<Long>` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `Optional<Long>` | `null` | Maximum query depth (None = unlimited) |

##### Methods

###### defaultOptions()

**Signature:**

```java
public static SchemaConfig defaultOptions()
```

**Example:**

```java
var result = SchemaConfig.defaultOptions();
```

**Returns:** `SchemaConfig`

---

#### ServerConfig

Server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `host` | `String` | `"127.0.0.1"` | Host to bind to |
| `port` | `short` | `8000` | Port to bind to |
| `workers` | `long` | `1` | Number of Tokio runtime worker threads used by binding-managed server runtimes |
| `enableRequestId` | `boolean` | `false` | Enable request ID generation and propagation |
| `maxBodySize` | `Optional<Long>` | `null` | Maximum request body size in bytes (None = unlimited, not recommended) |
| `requestTimeout` | `Optional<Long>` | `null` | Request timeout in seconds (None = no timeout) |
| `compression` | `Optional<CompressionConfig>` | `null` | Enable compression middleware |
| `rateLimit` | `Optional<RateLimitConfig>` | `null` | Enable rate limiting |
| `jwtAuth` | `Optional<JwtConfig>` | `null` | JWT authentication configuration |
| `apiKeyAuth` | `Optional<ApiKeyConfig>` | `null` | API Key authentication configuration |
| `staticFiles` | `List<StaticFilesConfig>` | `Collections.emptyList()` | Static file serving configuration |
| `gracefulShutdown` | `boolean` | `true` | Enable graceful shutdown on SIGTERM/SIGINT |
| `shutdownTimeout` | `long` | `30` | Graceful shutdown timeout (seconds) |
| `asyncapi` | `Optional<AsyncApiConfig>` | `null` | AsyncAPI HTTP endpoint configuration |
| `openapi` | `Optional<OpenApiConfig>` | `null` | OpenAPI documentation configuration |
| `jsonrpc` | `Optional<JsonRpcConfig>` | `null` | JSON-RPC configuration |
| `grpc` | `Optional<GrpcConfig>` | `null` | gRPC configuration |
| `backgroundTasks` | `BackgroundTaskConfig` | — | Background task executor configuration |
| `enableHttpTrace` | `boolean` | `false` | Enable per-request HTTP tracing (tower-http `TraceLayer`) |

##### Methods

###### defaultOptions()

**Signature:**

```java
public static ServerConfig defaultOptions()
```

**Example:**

```java
var result = ServerConfig.defaultOptions();
```

**Returns:** `ServerConfig`

---

#### ServerInfo

Server information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | Base URL of the server (e.g. `"<https://api.example.com/v1"`>). |
| `description` | `Optional<String>` | `null` | Optional human-readable description of the server environment. |

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
| `eventType` | `Optional<String>` | `null` | Event type (optional) |
| `data` | `Object` | — | Event data (JSON value) |
| `id` | `Optional<String>` | `null` | Event ID (optional, for client-side reconnection) |
| `retry` | `Optional<Long>` | `null` | Retry timeout in milliseconds (optional) |

##### Methods

###### withId()

Set the event ID for client-side reconnection support

Sets an ID that clients can use to resume from this point if they disconnect.
The client sends this ID back in the `Last-Event-ID` header when reconnecting.

**Signature:**

```java
public SseEvent withId(String id)
```

**Example:**

```java
var result = instance.withId("value");
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

```java
public SseEvent withRetry(long retryMs)
```

**Example:**

```java
var result = instance.withRetry(42);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `retryMs` | `long` | Yes | Retry timeout in milliseconds |

**Returns:** `SseEvent`

---

#### StaticFilesConfig

Static file serving configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `directory` | `String` | — | Directory path to serve |
| `routePrefix` | `String` | — | URL path prefix (e.g., "/static") |
| `indexFile` | `boolean` | `serde(default = "default_true")` | Fallback to index.html for directories |
| `cacheControl` | `Optional<String>` | `null` | Cache-Control header value |

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
| `contentType` | `Optional<String>` | `null` | MIME type of the uploaded file |
| `size` | `Optional<Long>` | `null` | Size of the file in bytes |
| `content` | `byte\[\]` | — | File content (may be base64 encoded) |
| `contentEncoding` | `Optional<String>` | `null` | Content encoding type |

##### Methods

###### asBytes()

Get the raw file content as bytes.

This provides zero-copy access to the underlying buffer.

**Signature:**

```java
public byte[] asBytes()
```

**Example:**

```java
var result = instance.asBytes();
```

**Returns:** `byte[]`

###### readToString()

Read the file content as a UTF-8 string.

**Errors:**

Returns an error if the content is not valid UTF-8.

**Signature:**

```java
public String readToString() throws Error
```

**Example:**

```java
var result = instance.readToString();
```

**Returns:** `String`

**Errors:** Throws `ErrorException`.

###### contentTypeOrDefault()

Get the content type, defaulting to "application/octet-stream".

**Signature:**

```java
public String contentTypeOrDefault()
```

**Example:**

```java
var result = instance.contentTypeOrDefault();
```

**Returns:** `String`

---

#### ValidateRequest

Request body for `POST /asyncapi/validate`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `spec` | `Object` | — | Spec |
| `channel` | `String` | — | Channel |
| `message` | `String` | — | Message |
| `payload` | `Object` | — | Payload |

---

#### ValidationResponse

Response body for `POST /asyncapi/validate`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `valid` | `boolean` | — | Valid |
| `errors` | `List<String>` | — | Errors |

---

### Enums

#### Method

HTTP method

| Value | Description |
|-------|-------------|
| `GET` | Get |
| `POST` | Post |
| `PUT` | Put |
| `PATCH` | Patch |
| `DELETE` | Delete |
| `HEAD` | Head |
| `OPTIONS` | Options |
| `CONNECT` | Connect |
| `TRACE` | Trace |

---

#### SecuritySchemeInfo

Security scheme types

| Value | Description |
|-------|-------------|
| `HTTP` | Http — Fields: `scheme`: `String`, `bearerFormat`: `String` |
| `API_KEY` | Api key — Fields: `location`: `String`, `name`: `String` |

---

### Errors

#### AppError

Error type for application builder operations.

| Variant | Description |
|---------|-------------|
| `ROUTE` | Route registration failed. |
| `SERVER` | Server/router construction failed. |
| `DECODE` | Failed to extract DTO from the request context. |

---

#### GraphQlError

Errors that can occur during GraphQL operations

These errors are compatible with async-graphql error handling and can be
converted to structured HTTP responses matching the project's error fixtures.

| Variant | Description |
|---------|-------------|
| `EXECUTION_ERROR` | Error during schema execution Occurs when the GraphQL executor encounters a runtime error during query execution. |
| `SCHEMA_BUILD_ERROR` | Error during schema building Occurs when schema construction fails due to invalid definitions or conflicts. |
| `REQUEST_HANDLING_ERROR` | Error during request handling Occurs when the HTTP request cannot be properly handled or parsed. |
| `SERIALIZATION_ERROR` | Serialization error Occurs during JSON serialization/deserialization of GraphQL values. |
| `JSON_ERROR` | JSON parsing error Occurs when JSON input cannot be parsed. |
| `VALIDATION_ERROR` | GraphQL validation error Occurs when a GraphQL query fails schema validation. |
| `PARSE_ERROR` | GraphQL parse error Occurs when the GraphQL query string cannot be parsed. |
| `AUTHENTICATION_ERROR` | Authentication error Occurs when request authentication fails. |
| `AUTHORIZATION_ERROR` | Authorization error Occurs when user lacks required permissions. |
| `NOT_FOUND` | Not found error Occurs when a requested resource is not found. |
| `RATE_LIMIT_EXCEEDED` | Rate limit error Occurs when rate limit is exceeded. |
| `INVALID_INPUT` | Invalid input error with validation details Occurs during input validation with detailed error information. |
| `COMPLEXITY_LIMIT_EXCEEDED` | Query complexity limit exceeded Occurs when a GraphQL query exceeds the configured complexity limit. |
| `DEPTH_LIMIT_EXCEEDED` | Query depth limit exceeded Occurs when a GraphQL query exceeds the configured depth limit. |
| `INTERNAL_ERROR` | Internal server error Occurs when an unexpected internal error happens. |

---

#### SchemaError

Error type for schema building operations

| Variant | Description |
|---------|-------------|
| `BUILDING_FAILED` | Generic schema building error |
| `VALIDATION_ERROR` | Configuration validation error |
| `COMPLEXITY_LIMIT_EXCEEDED` | Complexity limit exceeded |
| `DEPTH_LIMIT_EXCEEDED` | Depth limit exceeded |

---

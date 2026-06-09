---
title: "C# API Reference"
---

## C# API Reference <span class="version-badge">v0.15.6-rc.16</span>

### Functions

#### SchemaQueryOnly()

Create a simple schema configuration with only Query type.

This is a convenience function for schemas that only have queries.

**Returns:**

A `QueryOnlyConfig` with default settings

**Signature:**

```csharp
public static QueryOnlyConfig SchemaQueryOnly()
```

**Returns:** `QueryOnlyConfig`

---

#### SchemaQueryMutation()

Create a schema configuration with Query and Mutation types.

This is a convenience function for schemas with queries and mutations but no subscriptions.

**Returns:**

A `QueryMutationConfig` with default settings

**Signature:**

```csharp
public static QueryMutationConfig SchemaQueryMutation()
```

**Returns:** `QueryMutationConfig`

---

#### SchemaFull()

Create a schema configuration with all three root types.

This is a convenience function for fully-featured schemas.

**Returns:**

A `FullSchemaConfig` with default settings

**Signature:**

```csharp
public static FullSchemaConfig SchemaFull()
```

**Returns:** `FullSchemaConfig`

---

### Types

#### ApiKeyConfig

API Key authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Keys` | `List<string>` | — | Valid API keys |
| `HeaderName` | `string` | `/* serde(default) */` | Header name to check (e.g., "X-API-Key") |

---

#### App

Spikard application builder.

### Methods

#### New()

Create a new application with the default server configuration.

**Signature:**

```csharp
public App New()
```

#### MergeAxumRouter()

Attach an existing Axum router to this application, returning ownership.

**Signature:**

```csharp
public App MergeAxumRouter(string router)
```

#### AttachAxumRouter()

Attach an Axum router using a mutable reference for incremental configuration.

**Signature:**

```csharp
public App AttachAxumRouter(string router)
```

#### IntoRouter()

Build the underlying Axum router.

**Errors:**

Returns an error if server or router construction fails.

**Signature:**

```csharp
public string IntoRouter()
```

#### IntoRouterAndConfig()

Decompose the application into its Axum router and server configuration.

This is the low-level escape hatch used by the C FFI layer to start the
server on a background thread while retaining the bind address from the
caller-supplied `ServerConfig`. Prefer `App.run` for normal use.

**Errors:**

Returns an error if router construction fails.

**Signature:**

```csharp
public string IntoRouterAndConfig()
```

#### CreateDefault()

**Signature:**

```csharp
public App CreateDefault()
```

---

#### AsyncApiConfig

AsyncAPI HTTP endpoint configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Enabled` | `bool` | — | Enable AsyncAPI endpoints (default: false) |
| `Spec` | `object?` | `null` | Pre-registered AsyncAPI spec to serve from GET /asyncapi.json |

---

#### BackgroundJobMetadata

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | The name |
| `RequestId` | `string?` | `null` | Request id |

### Methods

#### CreateDefault()

**Signature:**

```csharp
public BackgroundJobMetadata CreateDefault()
```

---

#### BackgroundTaskConfig

Configuration for in-process background task execution.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `MaxQueueSize` | `nuint` | `1024` | Maximum queue size |
| `MaxConcurrentTasks` | `nuint` | `128` | Maximum concurrent tasks |
| `DrainTimeoutSecs` | `ulong` | `30` | Drain timeout secs |

### Methods

#### CreateDefault()

**Signature:**

```csharp
public BackgroundTaskConfig CreateDefault()
```

---

#### CompressionConfig

Compression configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Gzip` | `bool` | `true` | Enable gzip compression |
| `Brotli` | `bool` | `true` | Enable brotli compression |
| `MinSize` | `nuint` | — | Minimum response size to compress (bytes) |
| `Quality` | `uint` | — | Compression quality (0-11 for brotli, 0-9 for gzip) |

### Methods

#### CreateDefault()

**Signature:**

```csharp
public CompressionConfig CreateDefault()
```

---

#### ContactInfo

Contact information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string?` | `null` | Name of the contact person or organisation. |
| `Email` | `string?` | `null` | Contact email address. |
| `Url` | `string?` | `null` | URL pointing to the contact information page. |

---

#### CorsConfig

CORS configuration for a route

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `AllowedOrigins` | `List<string>` | `new List<string>()` | Allowed origins |
| `AllowedMethods` | `List<string>` | `new List<string>()` | Allowed methods |
| `AllowedHeaders` | `List<string>` | `new List<string>()` | Allowed headers |
| `ExposeHeaders` | `List<string>?` | `null` | Expose headers |
| `MaxAge` | `uint?` | `null` | Maximum age |
| `AllowCredentials` | `bool?` | `null` | Allow credentials |
| `MethodsJoinedCache` | `string` | — | Methods joined cache |
| `HeadersJoinedCache` | `string` | — | Headers joined cache |

### Methods

#### AllowedMethodsJoined()

Get the cached joined methods string for preflight responses

**Signature:**

```csharp
public string AllowedMethodsJoined()
```

#### AllowedHeadersJoined()

Get the cached joined headers string for preflight responses

**Signature:**

```csharp
public string AllowedHeadersJoined()
```

#### IsOriginAllowed()

Check if an origin is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```csharp
public bool IsOriginAllowed(string origin)
```

#### IsMethodAllowed()

Check if a method is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```csharp
public bool IsMethodAllowed(string method)
```

#### CreateDefault()

**Signature:**

```csharp
public CorsConfig CreateDefault()
```

---

#### FullSchemaConfig

Configuration for fully-featured schemas with Query, Mutation, and Subscription types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `IntrospectionEnabled` | `bool` | `true` | Enable introspection queries |
| `ComplexityLimit` | `nuint?` | `null` | Maximum query complexity (None = unlimited) |
| `DepthLimit` | `nuint?` | `null` | Maximum query depth (None = unlimited) |

### Methods

#### CreateDefault()

**Signature:**

```csharp
public FullSchemaConfig CreateDefault()
```

---

#### GraphQlRouteConfig

Configuration for GraphQL routes

Provides a builder pattern for configuring GraphQL route parameters
for the Spikard HTTP server's routing system.

### Methods

#### New()

Create a new GraphQL route configuration with defaults

Default values:

- path: "/graphql"
- method: "POST"
- `enable_playground`: false

**Signature:**

```csharp
public GraphQlRouteConfig New()
```

#### Path()

Set the HTTP path for the GraphQL endpoint

**Signature:**

```csharp
public GraphQlRouteConfig Path(string path)
```

#### Method()

Set the HTTP method for the GraphQL endpoint

**Signature:**

```csharp
public GraphQlRouteConfig Method(string method)
```

#### EnablePlayground()

Enable or disable the GraphQL Playground UI

**Signature:**

```csharp
public GraphQlRouteConfig EnablePlayground(bool enable)
```

#### Description()

Set a custom description for documentation

**Signature:**

```csharp
public GraphQlRouteConfig Description(string description)
```

#### GetPath()

Get the configured path

**Signature:**

```csharp
public string GetPath()
```

#### GetMethod()

Get the configured method

**Signature:**

```csharp
public string GetMethod()
```

#### IsPlaygroundEnabled()

Check if playground is enabled

**Signature:**

```csharp
public bool IsPlaygroundEnabled()
```

#### GetDescription()

Get the description if set

**Signature:**

```csharp
public string? GetDescription()
```

#### CreateDefault()

**Signature:**

```csharp
public GraphQlRouteConfig CreateDefault()
```

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
| `Enabled` | `bool` | `true` | Enable gRPC support |
| `MaxMessageSize` | `nuint` | — | Maximum message size in bytes (for both sending and receiving) This limit applies to individual messages in both unary and streaming RPCs. When a single message exceeds this size, the request is rejected with HTTP 413 (Payload Too Large). Default: 4MB (4194304 bytes) **Note:** This limit does NOT apply to the total response size in streaming RPCs. For multi-message streams, the total response can exceed this limit as long as each individual message stays within the limit. |
| `EnableCompression` | `bool` | `true` | Enable gzip compression for gRPC messages |
| `RequestTimeout` | `ulong?` | `null` | Timeout for gRPC requests in seconds (None = no timeout) |
| `MaxConcurrentStreams` | `uint` | — | Maximum number of concurrent streams per connection (HTTP/2 advisory) This value is communicated to HTTP/2 clients as the server's flow control limit. The HTTP/2 transport layer enforces this limit automatically via SETTINGS frames and GOAWAY responses. Applications should NOT implement custom enforcement. Default: 100 streams per connection # Stream Limiting Strategy - **Per Connection**: This limit applies per HTTP/2 connection, not globally - **Transport Enforcement**: HTTP/2 handles all stream limiting; applications need not implement custom checks - **Streaming Requests**: In server streaming or bidi streaming, each logical RPC consumes one stream slot. Message ordering within a stream follows HTTP/2 frame ordering. |
| `EnableKeepalive` | `bool` | `true` | Enable HTTP/2 keepalive |
| `KeepaliveInterval` | `ulong` | — | HTTP/2 keepalive interval in seconds |
| `KeepaliveTimeout` | `ulong` | — | HTTP/2 keepalive timeout in seconds |
| `MaxStreamResponseBytes` | `nuint?` | `null` | Total byte cap across an entire streaming response. When `Some(n)`, the streaming adapter aborts the stream with `tonic.Status.resource_exhausted` once the cumulative encoded message bytes exceed `n`. The stream yields the error item and then terminates. Per-message cap remains `max_message_size`. This limit applies to server-streaming and bidirectional-streaming RPCs only; unary RPCs are governed solely by `max_message_size`. Default: `null` (unbounded total response size). |

### Methods

#### CreateDefault()

**Signature:**

```csharp
public GrpcConfig CreateDefault()
```

---

#### Handler

Handler trait that all language bindings must implement

This trait is completely language-agnostic. Each binding (Python, Node, WASM)
implements this trait to bridge their runtime to our HTTP server.

### Methods

#### Call()

Handle an HTTP request

Takes the extracted request data and returns a future that resolves to either:

- Ok(Response): A successful HTTP response
- Err((StatusCode, String)): An error with status code and message

**Signature:**

```csharp
public async Task<HandlerResult> CallAsync(Request request, RequestData requestData)
```

#### PrefersRawJsonBody()

Whether this handler prefers consuming `RequestData.raw_body` over the parsed
`RequestData.body` for JSON requests.

When `true`, the server may skip eager JSON parsing when there is no request-body
schema validator attached to the route.

**Signature:**

```csharp
public bool PrefersRawJsonBody()
```

#### PrefersParameterExtraction()

Whether this handler wants to perform its own parameter validation/extraction (path/query/header/cookie).

When `true`, the server will skip `ParameterValidator.validate_and_extract` in `ValidatingHandler`.
This is useful for language bindings which need to transform validated parameters into
language-specific values (e.g., Python kwargs) without duplicating work. When `false`,
the server stores validated output in `RequestData.validated_params`.

**Signature:**

```csharp
public bool PrefersParameterExtraction()
```

#### WantsHeaders()

Whether this handler needs the parsed headers map in `RequestData`.

When `false`, the server may skip building `RequestData.headers` for requests without a body.
(Requests with bodies still typically need `Content-Type` decisions.)

**Signature:**

```csharp
public bool WantsHeaders()
```

#### WantsCookies()

Whether this handler needs the parsed cookies map in `RequestData`.

When `false`, the server may skip parsing cookies for requests without a body.

**Signature:**

```csharp
public bool WantsCookies()
```

#### WantsRequestExtensions()

Whether this handler needs `RequestData` stored in request extensions.

When `false`, the server avoids inserting `RequestData` into extensions to
skip cloning in hot paths.

**Signature:**

```csharp
public bool WantsRequestExtensions()
```

#### StaticResponse()

Return a pre-built static response if this handler always produces the
same output. When `Some`, the server bypasses the full middleware
pipeline and serves the pre-built response directly.

**Signature:**

```csharp
public StaticResponse? StaticResponse()
```

---

#### IntoHandler

Convert user-facing handler functions into the low-level `Handler` trait.

### Methods

#### IntoHandler()

Convert this value into a shared request handler.

**Signature:**

```csharp
public Handler IntoHandler()
```

---

#### JsonRpcConfig

JSON-RPC server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Enabled` | `bool` | `true` | Enable JSON-RPC endpoint |
| `EndpointPath` | `string` | — | HTTP endpoint path for JSON-RPC requests (default: "/rpc") |
| `EnableBatch` | `bool` | — | Enable batch request processing (default: true) |
| `MaxBatchSize` | `nuint` | — | Maximum number of requests in a batch (default: 100) |

### Methods

#### CreateDefault()

**Signature:**

```csharp
public JsonRpcConfig CreateDefault()
```

---

#### JsonRpcMethodInfo

JSON-RPC method metadata for routes that support JSON-RPC

This struct captures the metadata needed to expose HTTP routes as JSON-RPC methods,
enabling discovery and documentation of RPC-compatible endpoints.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `MethodName` | `string` | — | The JSON-RPC method name (e.g., "user.create") |
| `Description` | `string?` | `null` | Optional description of what the method does |
| `ParamsSchema` | `object?` | `null` | Optional JSON Schema for method parameters |
| `ResultSchema` | `object?` | `null` | Optional JSON Schema for the result |
| `Deprecated` | `bool` | `/* serde(default) */` | Whether this method is deprecated |
| `Tags` | `List<string>` | `/* serde(default) */` | Tags for categorizing and grouping methods |

---

#### JwtConfig

JWT authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Secret` | `string` | — | Secret key for JWT verification |
| `Algorithm` | `string` | `/* serde(default) */` | Required algorithm (HS256, HS384, HS512, RS256, etc.) |
| `Audience` | `List<string>?` | `null` | Required audience claim |
| `Issuer` | `string?` | `null` | Required issuer claim |
| `Leeway` | `ulong` | `/* serde(default) */` | Leeway for expiration checks (seconds) |

---

#### LicenseInfo

License information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | SPDX license identifier or display name (e.g. `"MIT"`). |
| `Url` | `string?` | `null` | URL to the full license text. |

---

#### OpenApiConfig

OpenAPI configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Enabled` | `bool` | `false` | Enable OpenAPI generation (default: false for zero overhead) |
| `Title` | `string` | `"API"` | API title |
| `Version` | `string` | `"1.0.0"` | API version |
| `Description` | `string?` | `null` | API description (supports markdown) |
| `SwaggerUiPath` | `string` | — | Path to serve Swagger UI (default: "/docs") |
| `RedocPath` | `string` | — | Path to serve Redoc (default: "/redoc") |
| `OpenapiJsonPath` | `string` | — | Path to serve OpenAPI JSON spec (default: "/openapi.json") |
| `Contact` | `ContactInfo?` | `null` | Contact information |
| `License` | `LicenseInfo?` | `null` | License information |
| `Servers` | `List<ServerInfo>` | `new List<ServerInfo>()` | Server definitions |
| `SecuritySchemes` | `Dictionary<string, SecuritySchemeInfo>` | `new Dictionary<string, SecuritySchemeInfo>()` | Security schemes (auto-detected from middleware if not provided) |

### Methods

#### CreateDefault()

**Signature:**

```csharp
public OpenApiConfig CreateDefault()
```

---

#### ParseRequest

Request body for `POST /asyncapi/parse`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Spec` | `object` | — | Spec |

---

#### ParseResult

Full parse result returned by `POST /asyncapi/parse`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `SpecVersion` | `string` | — | Spec version |
| `Title` | `string` | — | Title |
| `ApiVersion` | `string` | — | Api version |
| `Channels` | `List<ParsedChannel>` | — | Channels |
| `Operations` | `List<ParsedOperation>` | — | Operations |
| `Messages` | `List<ParsedMessage>` | — | Messages |

---

#### ParsedChannel

A single channel extracted from an AsyncAPI spec

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | Channel key from the spec (e.g. "chat/messages") |
| `Address` | `string` | — | Channel address / path |
| `Messages` | `List<string>` | — | Message names declared on this channel |
| `Bindings` | `object?` | `null` | Bindings (ws / http / amqp / …) as raw JSON for forward-compatibility |

---

#### ParsedMessage

A resolved message (name + JSON Schema)

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | Message name |
| `Schema` | `object?` | `null` | Resolved JSON Schema for the message payload, if available |

---

#### ParsedOperation

A single operation extracted from an AsyncAPI spec

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | Operation name |
| `Action` | `string` | — | Operation action: "send" or "receive" |
| `Channel` | `string` | — | Channel reference (resolved to the channel name) |

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
| `TypeUri` | `string` | — | A URI reference that identifies the problem type. Defaults to "about:blank" when absent. Should be a stable, human-readable identifier for the problem type. |
| `Title` | `string` | — | A short, human-readable summary of the problem type. Should not change from occurrence to occurrence of the problem. |
| `Status` | `ushort` | — | The HTTP status code generated by the origin server. This is advisory; the actual HTTP status code takes precedence. |
| `Detail` | `string?` | `null` | A human-readable explanation specific to this occurrence of the problem. |
| `Instance` | `string?` | `null` | A URI reference that identifies the specific occurrence of the problem. It may or may not yield further information if dereferenced. |
| `Extensions` | `Dictionary<string, object>` | — | Extension members - problem-type-specific data. For validation errors, this typically contains an "errors" array. |

### Methods

#### WithDetail()

Set the detail field

**Signature:**

```csharp
public ProblemDetails WithDetail(string detail)
```

#### WithInstance()

Set the instance field

**Signature:**

```csharp
public ProblemDetails WithInstance(string instance)
```

#### NotFound()

Create a not found error

**Signature:**

```csharp
public ProblemDetails NotFound(string detail)
```

#### MethodNotAllowed()

Create a method not allowed error

**Signature:**

```csharp
public ProblemDetails MethodNotAllowed(string detail)
```

#### InternalServerError()

Create an internal server error

**Signature:**

```csharp
public ProblemDetails InternalServerError(string detail)
```

#### BadRequest()

Create a bad request error

**Signature:**

```csharp
public ProblemDetails BadRequest(string detail)
```

#### ToJson()

Serialize to JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```csharp
public string ToJson()
```

#### ToJsonPretty()

Serialize to pretty JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```csharp
public string ToJsonPretty()
```

---

#### QueryMutationConfig

Configuration for schemas with Query and Mutation types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `IntrospectionEnabled` | `bool` | `true` | Enable introspection queries |
| `ComplexityLimit` | `nuint?` | `null` | Maximum query complexity (None = unlimited) |
| `DepthLimit` | `nuint?` | `null` | Maximum query depth (None = unlimited) |

### Methods

#### CreateDefault()

**Signature:**

```csharp
public QueryMutationConfig CreateDefault()
```

---

#### QueryOnlyConfig

Configuration for schemas with only Query type

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `IntrospectionEnabled` | `bool` | `true` | Enable introspection queries |
| `ComplexityLimit` | `nuint?` | `null` | Maximum query complexity (None = unlimited) |
| `DepthLimit` | `nuint?` | `null` | Maximum query depth (None = unlimited) |

### Methods

#### CreateDefault()

**Signature:**

```csharp
public QueryOnlyConfig CreateDefault()
```

---

#### RateLimitConfig

Rate limiting configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `PerSecond` | `ulong` | `100` | Requests per second |
| `Burst` | `uint` | `200` | Burst allowance |
| `IpBased` | `bool` | `true` | Use IP-based rate limiting |

### Methods

#### CreateDefault()

**Signature:**

```csharp
public RateLimitConfig CreateDefault()
```

---

#### Request

---

#### Response

HTTP Response with custom status code, headers, and content

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `object?` | `null` | Response body content |
| `StatusCode` | `ushort` | — | HTTP status code (defaults to 200) |
| `Headers` | `Dictionary<string, string>` | `new Dictionary<string, string>()` | Response headers |

### Methods

#### SetHeader()

Set a header

**Signature:**

```csharp
public void SetHeader(string key, string value)
```

#### SetCookie()

Set a cookie in the response

**Signature:**

```csharp
public void SetCookie(string key, string value, bool secure, bool httpOnly, long maxAge, string domain, string path, string sameSite)
```

#### CreateDefault()

**Signature:**

```csharp
public Response CreateDefault()
```

---

#### RouteBuilder

Builder for defining a route.

### Methods

#### New()

Create a new builder for the provided HTTP method and path.

**Signature:**

```csharp
public RouteBuilder New(Method method, string path)
```

#### HandlerName()

Assign an explicit handler name.

**Signature:**

```csharp
public RouteBuilder HandlerName(string name)
```

#### RequestSchemaJson()

Provide a raw JSON schema for the request body.

**Signature:**

```csharp
public RouteBuilder RequestSchemaJson(object schema)
```

#### ResponseSchemaJson()

Provide a raw JSON schema for the response body.

**Signature:**

```csharp
public RouteBuilder ResponseSchemaJson(object schema)
```

#### ParamsSchemaJson()

Provide a raw JSON schema for request parameters.

**Signature:**

```csharp
public RouteBuilder ParamsSchemaJson(object schema)
```

#### FileParamsJson()

Provide multipart file parameter configuration.

**Signature:**

```csharp
public RouteBuilder FileParamsJson(object schema)
```

#### Cors()

Attach a CORS configuration for this route.

**Signature:**

```csharp
public RouteBuilder Cors(CorsConfig cors)
```

#### Compression()

Attach a compression configuration for this route.

**Signature:**

```csharp
public RouteBuilder Compression(CompressionConfig compression)
```

#### Sync()

Mark the route as synchronous.

**Signature:**

```csharp
public RouteBuilder Sync()
```

#### HandlerDependencies()

Declare the dependency keys that must be resolved before this handler runs.

**Signature:**

```csharp
public RouteBuilder HandlerDependencies(List<string> dependencies)
```

---

#### SchemaConfig

Configuration for GraphQL schema building.

Encapsulates all schema-level configuration options including
introspection control, complexity limits, and depth limits.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `IntrospectionEnabled` | `bool` | `true` | Enable introspection queries |
| `ComplexityLimit` | `nuint?` | `null` | Maximum query complexity (None = unlimited) |
| `DepthLimit` | `nuint?` | `null` | Maximum query depth (None = unlimited) |

### Methods

#### CreateDefault()

**Signature:**

```csharp
public SchemaConfig CreateDefault()
```

---

#### ServerConfig

Server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Host` | `string` | `"127.0.0.1"` | Host to bind to |
| `Port` | `ushort` | `8000` | Port to bind to |
| `Workers` | `nuint` | `1` | Number of Tokio runtime worker threads used by binding-managed server runtimes |
| `EnableRequestId` | `bool` | `false` | Enable request ID generation and propagation |
| `MaxBodySize` | `nuint?` | `null` | Maximum request body size in bytes (None = unlimited, not recommended) |
| `RequestTimeout` | `ulong?` | `null` | Request timeout in seconds (None = no timeout) |
| `Compression` | `CompressionConfig?` | `null` | Enable compression middleware |
| `RateLimit` | `RateLimitConfig?` | `null` | Enable rate limiting |
| `JwtAuth` | `JwtConfig?` | `null` | JWT authentication configuration |
| `ApiKeyAuth` | `ApiKeyConfig?` | `null` | API Key authentication configuration |
| `StaticFiles` | `List<StaticFilesConfig>` | `new List<StaticFilesConfig>()` | Static file serving configuration |
| `GracefulShutdown` | `bool` | `true` | Enable graceful shutdown on SIGTERM/SIGINT |
| `ShutdownTimeout` | `ulong` | `30` | Graceful shutdown timeout (seconds) |
| `Asyncapi` | `AsyncApiConfig?` | `null` | AsyncAPI HTTP endpoint configuration |
| `Openapi` | `OpenApiConfig?` | `null` | OpenAPI documentation configuration |
| `Jsonrpc` | `JsonRpcConfig?` | `null` | JSON-RPC configuration |
| `Grpc` | `GrpcConfig?` | `null` | gRPC configuration |
| `LifecycleHooks` | `string?` | `null` | Lifecycle hooks for request/response processing |
| `BackgroundTasks` | `BackgroundTaskConfig` | — | Background task executor configuration |
| `EnableHttpTrace` | `bool` | `false` | Enable per-request HTTP tracing (tower-http `TraceLayer`) |
| `DiContainer` | `string?` | `null` | Dependency injection container (requires 'di' feature) |

### Methods

#### CreateDefault()

**Signature:**

```csharp
public ServerConfig CreateDefault()
```

---

#### ServerInfo

Server information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string` | — | Base URL of the server (e.g. `"<https://api.example.com/v1"`>). |
| `Description` | `string?` | `null` | Optional human-readable description of the server environment. |

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
| `EventType` | `string?` | `null` | Event type (optional) |
| `Data` | `object` | — | Event data (JSON value) |
| `Id` | `string?` | `null` | Event ID (optional, for client-side reconnection) |
| `Retry` | `ulong?` | `null` | Retry timeout in milliseconds (optional) |

### Methods

#### WithId()

Set the event ID for client-side reconnection support

Sets an ID that clients can use to resume from this point if they disconnect.
The client sends this ID back in the `Last-Event-ID` header when reconnecting.

**Signature:**

```csharp
public SseEvent WithId(string id)
```

#### WithRetry()

Set the retry timeout for client reconnection

Sets the time in milliseconds clients should wait before attempting to reconnect
if the connection is lost. The client browser will automatically handle reconnection.

**Signature:**

```csharp
public SseEvent WithRetry(ulong retryMs)
```

---

#### StaticFilesConfig

Static file serving configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Directory` | `string` | — | Directory path to serve |
| `RoutePrefix` | `string` | — | URL path prefix (e.g., "/static") |
| `IndexFile` | `bool` | `/* serde(default) */` | Fallback to index.html for directories |
| `CacheControl` | `string?` | `null` | Cache-Control header value |

---

#### TestingSseEvent

A single Server-Sent Event.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Data` | `string` | — | The data field of the event. |

---

#### UploadFile

Represents an uploaded file from multipart/form-data requests.

This struct provides efficient access to file content with automatic
base64 decoding and implements standard I/O traits for compatibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Filename` | `string` | — | Original filename from the client |
| `ContentType` | `string?` | `null` | MIME type of the uploaded file |
| `Size` | `nuint?` | `null` | Size of the file in bytes |
| `Content` | `byte[]` | — | File content (may be base64 encoded) |
| `ContentEncoding` | `string?` | `null` | Content encoding type |
| `Cursor` | `string` | — | Internal cursor for Read/Seek operations |

### Methods

#### AsBytes()

Get the raw file content as bytes.

This provides zero-copy access to the underlying buffer.

**Signature:**

```csharp
public byte[] AsBytes()
```

#### ReadToString()

Read the file content as a UTF-8 string.

**Errors:**

Returns an error if the content is not valid UTF-8.

**Signature:**

```csharp
public string ReadToString()
```

#### ContentTypeOrDefault()

Get the content type, defaulting to "application/octet-stream".

**Signature:**

```csharp
public string ContentTypeOrDefault()
```

---

#### ValidateRequest

Request body for `POST /asyncapi/validate`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Spec` | `object` | — | Spec |
| `Channel` | `string` | — | Channel |
| `Message` | `string` | — | Message |
| `Payload` | `object` | — | Payload |

---

#### ValidationResponse

Response body for `POST /asyncapi/validate`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Valid` | `bool` | — | Valid |
| `Errors` | `List<string>` | — | Errors |

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
| `Http` | Http — Fields: `Scheme`: `string`, `BearerFormat`: `string` |
| `ApiKey` | Api key — Fields: `Location`: `string`, `Name`: `string` |

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

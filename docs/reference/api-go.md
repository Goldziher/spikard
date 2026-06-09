---
title: "Go API Reference"
---

## Go API Reference <span class="version-badge">v0.15.6-rc.15</span>

### Functions

#### SchemaQueryOnly()

Create a simple schema configuration with only Query type.

This is a convenience function for schemas that only have queries.

**Returns:**

A `QueryOnlyConfig` with default settings

**Signature:**

```go
func SchemaQueryOnly() QueryOnlyConfig
```

**Returns:** `QueryOnlyConfig`

---

#### SchemaQueryMutation()

Create a schema configuration with Query and Mutation types.

This is a convenience function for schemas with queries and mutations but no subscriptions.

**Returns:**

A `QueryMutationConfig` with default settings

**Signature:**

```go
func SchemaQueryMutation() QueryMutationConfig
```

**Returns:** `QueryMutationConfig`

---

#### SchemaFull()

Create a schema configuration with all three root types.

This is a convenience function for fully-featured schemas.

**Returns:**

A `FullSchemaConfig` with default settings

**Signature:**

```go
func SchemaFull() FullSchemaConfig
```

**Returns:** `FullSchemaConfig`

---

### Types

#### ApiKeyConfig

API Key authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Keys` | `[]string` | — | Valid API keys |
| `HeaderName` | `string` | `/* serde(default) */` | Header name to check (e.g., "X-API-Key") |

---

#### App

Spikard application builder.

### Methods

#### New()

Create a new application with the default server configuration.

**Signature:**

```go
func (o *App) New() App
```

#### MergeAxumRouter()

Attach an existing Axum router to this application, returning ownership.

**Signature:**

```go
func (o *App) MergeAxumRouter(router string) App
```

#### AttachAxumRouter()

Attach an Axum router using a mutable reference for incremental configuration.

**Signature:**

```go
func (o *App) AttachAxumRouter(router string) App
```

#### IntoRouter()

Build the underlying Axum router.

**Errors:**

Returns an error if server or router construction fails.

**Signature:**

```go
func (o *App) IntoRouter() (string, error)
```

#### IntoRouterAndConfig()

Decompose the application into its Axum router and server configuration.

This is the low-level escape hatch used by the C FFI layer to start the
server on a background thread while retaining the bind address from the
caller-supplied `ServerConfig`. Prefer `App.run` for normal use.

**Errors:**

Returns an error if router construction fails.

**Signature:**

```go
func (o *App) IntoRouterAndConfig() (string, error)
```

#### Default()

**Signature:**

```go
func (o *App) Default() App
```

---

#### AsyncApiConfig

AsyncAPI HTTP endpoint configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Enabled` | `bool` | — | Enable AsyncAPI endpoints (default: false) |
| `Spec` | `*interface{}` | `nil` | Pre-registered AsyncAPI spec to serve from GET /asyncapi.json |

---

#### BackgroundJobMetadata

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | The name |
| `RequestId` | `*string` | `nil` | Request id |

### Methods

#### Default()

**Signature:**

```go
func (o *BackgroundJobMetadata) Default() BackgroundJobMetadata
```

---

#### BackgroundTaskConfig

Configuration for in-process background task execution.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `MaxQueueSize` | `int` | `1024` | Maximum queue size |
| `MaxConcurrentTasks` | `int` | `128` | Maximum concurrent tasks |
| `DrainTimeoutSecs` | `uint64` | `30` | Drain timeout secs |

### Methods

#### Default()

**Signature:**

```go
func (o *BackgroundTaskConfig) Default() BackgroundTaskConfig
```

---

#### CompressionConfig

Compression configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Gzip` | `bool` | `true` | Enable gzip compression |
| `Brotli` | `bool` | `true` | Enable brotli compression |
| `MinSize` | `int` | — | Minimum response size to compress (bytes) |
| `Quality` | `uint32` | — | Compression quality (0-11 for brotli, 0-9 for gzip) |

### Methods

#### Default()

**Signature:**

```go
func (o *CompressionConfig) Default() CompressionConfig
```

---

#### ContactInfo

Contact information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `*string` | `nil` | Name of the contact person or organisation. |
| `Email` | `*string` | `nil` | Contact email address. |
| `Url` | `*string` | `nil` | URL pointing to the contact information page. |

---

#### CorsConfig

CORS configuration for a route

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `AllowedOrigins` | `[]string` | `nil` | Allowed origins |
| `AllowedMethods` | `[]string` | `nil` | Allowed methods |
| `AllowedHeaders` | `[]string` | `nil` | Allowed headers |
| `ExposeHeaders` | `*[]string` | `nil` | Expose headers |
| `MaxAge` | `*uint32` | `nil` | Maximum age |
| `AllowCredentials` | `*bool` | `nil` | Allow credentials |
| `MethodsJoinedCache` | `string` | — | Methods joined cache |
| `HeadersJoinedCache` | `string` | — | Headers joined cache |

### Methods

#### AllowedMethodsJoined()

Get the cached joined methods string for preflight responses

**Signature:**

```go
func (o *CorsConfig) AllowedMethodsJoined() string
```

#### AllowedHeadersJoined()

Get the cached joined headers string for preflight responses

**Signature:**

```go
func (o *CorsConfig) AllowedHeadersJoined() string
```

#### IsOriginAllowed()

Check if an origin is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```go
func (o *CorsConfig) IsOriginAllowed(origin string) bool
```

#### IsMethodAllowed()

Check if a method is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```go
func (o *CorsConfig) IsMethodAllowed(method string) bool
```

#### Default()

**Signature:**

```go
func (o *CorsConfig) Default() CorsConfig
```

---

#### FullSchemaConfig

Configuration for fully-featured schemas with Query, Mutation, and Subscription types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `IntrospectionEnabled` | `bool` | `true` | Enable introspection queries |
| `ComplexityLimit` | `*int` | `nil` | Maximum query complexity (None = unlimited) |
| `DepthLimit` | `*int` | `nil` | Maximum query depth (None = unlimited) |

### Methods

#### Default()

**Signature:**

```go
func (o *FullSchemaConfig) Default() FullSchemaConfig
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

```go
func (o *GraphQlRouteConfig) New() GraphQlRouteConfig
```

#### Path()

Set the HTTP path for the GraphQL endpoint

**Signature:**

```go
func (o *GraphQlRouteConfig) Path(path string) GraphQlRouteConfig
```

#### Method()

Set the HTTP method for the GraphQL endpoint

**Signature:**

```go
func (o *GraphQlRouteConfig) Method(method string) GraphQlRouteConfig
```

#### EnablePlayground()

Enable or disable the GraphQL Playground UI

**Signature:**

```go
func (o *GraphQlRouteConfig) EnablePlayground(enable bool) GraphQlRouteConfig
```

#### Description()

Set a custom description for documentation

**Signature:**

```go
func (o *GraphQlRouteConfig) Description(description string) GraphQlRouteConfig
```

#### GetPath()

Get the configured path

**Signature:**

```go
func (o *GraphQlRouteConfig) GetPath() string
```

#### GetMethod()

Get the configured method

**Signature:**

```go
func (o *GraphQlRouteConfig) GetMethod() string
```

#### IsPlaygroundEnabled()

Check if playground is enabled

**Signature:**

```go
func (o *GraphQlRouteConfig) IsPlaygroundEnabled() bool
```

#### GetDescription()

Get the description if set

**Signature:**

```go
func (o *GraphQlRouteConfig) GetDescription() *string
```

#### Default()

**Signature:**

```go
func (o *GraphQlRouteConfig) Default() GraphQlRouteConfig
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
  `tonic.Status.resource_exhausted`. Defaults to `nil` (unbounded).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Enabled` | `bool` | `true` | Enable gRPC support |
| `MaxMessageSize` | `int` | — | Maximum message size in bytes (for both sending and receiving) This limit applies to individual messages in both unary and streaming RPCs. When a single message exceeds this size, the request is rejected with HTTP 413 (Payload Too Large). Default: 4MB (4194304 bytes) **Note:** This limit does NOT apply to the total response size in streaming RPCs. For multi-message streams, the total response can exceed this limit as long as each individual message stays within the limit. |
| `EnableCompression` | `bool` | `true` | Enable gzip compression for gRPC messages |
| `RequestTimeout` | `*uint64` | `nil` | Timeout for gRPC requests in seconds (None = no timeout) |
| `MaxConcurrentStreams` | `uint32` | — | Maximum number of concurrent streams per connection (HTTP/2 advisory) This value is communicated to HTTP/2 clients as the server's flow control limit. The HTTP/2 transport layer enforces this limit automatically via SETTINGS frames and GOAWAY responses. Applications should NOT implement custom enforcement. Default: 100 streams per connection # Stream Limiting Strategy - **Per Connection**: This limit applies per HTTP/2 connection, not globally - **Transport Enforcement**: HTTP/2 handles all stream limiting; applications need not implement custom checks - **Streaming Requests**: In server streaming or bidi streaming, each logical RPC consumes one stream slot. Message ordering within a stream follows HTTP/2 frame ordering. |
| `EnableKeepalive` | `bool` | `true` | Enable HTTP/2 keepalive |
| `KeepaliveInterval` | `uint64` | — | HTTP/2 keepalive interval in seconds |
| `KeepaliveTimeout` | `uint64` | — | HTTP/2 keepalive timeout in seconds |
| `MaxStreamResponseBytes` | `*int` | `nil` | Total byte cap across an entire streaming response. When `Some(n)`, the streaming adapter aborts the stream with `tonic.Status.resource_exhausted` once the cumulative encoded message bytes exceed `n`. The stream yields the error item and then terminates. Per-message cap remains `max_message_size`. This limit applies to server-streaming and bidirectional-streaming RPCs only; unary RPCs are governed solely by `max_message_size`. Default: `nil` (unbounded total response size). |

### Methods

#### Default()

**Signature:**

```go
func (o *GrpcConfig) Default() GrpcConfig
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

```go
func (o *Handler) Call(request Request, requestData RequestData) HandlerResult
```

#### PrefersRawJsonBody()

Whether this handler prefers consuming `RequestData.raw_body` over the parsed
`RequestData.body` for JSON requests.

When `true`, the server may skip eager JSON parsing when there is no request-body
schema validator attached to the route.

**Signature:**

```go
func (o *Handler) PrefersRawJsonBody() bool
```

#### PrefersParameterExtraction()

Whether this handler wants to perform its own parameter validation/extraction (path/query/header/cookie).

When `true`, the server will skip `ParameterValidator.validate_and_extract` in `ValidatingHandler`.
This is useful for language bindings which need to transform validated parameters into
language-specific values (e.g., Python kwargs) without duplicating work. When `false`,
the server stores validated output in `RequestData.validated_params`.

**Signature:**

```go
func (o *Handler) PrefersParameterExtraction() bool
```

#### WantsHeaders()

Whether this handler needs the parsed headers map in `RequestData`.

When `false`, the server may skip building `RequestData.headers` for requests without a body.
(Requests with bodies still typically need `Content-Type` decisions.)

**Signature:**

```go
func (o *Handler) WantsHeaders() bool
```

#### WantsCookies()

Whether this handler needs the parsed cookies map in `RequestData`.

When `false`, the server may skip parsing cookies for requests without a body.

**Signature:**

```go
func (o *Handler) WantsCookies() bool
```

#### WantsRequestExtensions()

Whether this handler needs `RequestData` stored in request extensions.

When `false`, the server avoids inserting `RequestData` into extensions to
skip cloning in hot paths.

**Signature:**

```go
func (o *Handler) WantsRequestExtensions() bool
```

#### StaticResponse()

Return a pre-built static response if this handler always produces the
same output. When `Some`, the server bypasses the full middleware
pipeline and serves the pre-built response directly.

**Signature:**

```go
func (o *Handler) StaticResponse() *StaticResponse
```

---

#### IntoHandler

Convert user-facing handler functions into the low-level `Handler` trait.

### Methods

#### IntoHandler()

Convert this value into a shared request handler.

**Signature:**

```go
func (o *IntoHandler) IntoHandler() Handler
```

---

#### JsonRpcConfig

JSON-RPC server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Enabled` | `bool` | `true` | Enable JSON-RPC endpoint |
| `EndpointPath` | `string` | — | HTTP endpoint path for JSON-RPC requests (default: "/rpc") |
| `EnableBatch` | `bool` | — | Enable batch request processing (default: true) |
| `MaxBatchSize` | `int` | — | Maximum number of requests in a batch (default: 100) |

### Methods

#### Default()

**Signature:**

```go
func (o *JsonRpcConfig) Default() JsonRpcConfig
```

---

#### JsonRpcMethodInfo

JSON-RPC method metadata for routes that support JSON-RPC

This struct captures the metadata needed to expose HTTP routes as JSON-RPC methods,
enabling discovery and documentation of RPC-compatible endpoints.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `MethodName` | `string` | — | The JSON-RPC method name (e.g., "user.create") |
| `Description` | `*string` | `nil` | Optional description of what the method does |
| `ParamsSchema` | `*interface{}` | `nil` | Optional JSON Schema for method parameters |
| `ResultSchema` | `*interface{}` | `nil` | Optional JSON Schema for the result |
| `Deprecated` | `bool` | `/* serde(default) */` | Whether this method is deprecated |
| `Tags` | `[]string` | `/* serde(default) */` | Tags for categorizing and grouping methods |

---

#### JwtConfig

JWT authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Secret` | `string` | — | Secret key for JWT verification |
| `Algorithm` | `string` | `/* serde(default) */` | Required algorithm (HS256, HS384, HS512, RS256, etc.) |
| `Audience` | `*[]string` | `nil` | Required audience claim |
| `Issuer` | `*string` | `nil` | Required issuer claim |
| `Leeway` | `uint64` | `/* serde(default) */` | Leeway for expiration checks (seconds) |

---

#### LicenseInfo

License information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | SPDX license identifier or display name (e.g. `"MIT"`). |
| `Url` | `*string` | `nil` | URL to the full license text. |

---

#### OpenApiConfig

OpenAPI configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Enabled` | `bool` | `false` | Enable OpenAPI generation (default: false for zero overhead) |
| `Title` | `string` | `"API"` | API title |
| `Version` | `string` | `"1.0.0"` | API version |
| `Description` | `*string` | `nil` | API description (supports markdown) |
| `SwaggerUiPath` | `string` | — | Path to serve Swagger UI (default: "/docs") |
| `RedocPath` | `string` | — | Path to serve Redoc (default: "/redoc") |
| `OpenapiJsonPath` | `string` | — | Path to serve OpenAPI JSON spec (default: "/openapi.json") |
| `Contact` | `*ContactInfo` | `nil` | Contact information |
| `License` | `*LicenseInfo` | `nil` | License information |
| `Servers` | `[]ServerInfo` | `nil` | Server definitions |
| `SecuritySchemes` | `map[string]SecuritySchemeInfo` | `nil` | Security schemes (auto-detected from middleware if not provided) |

### Methods

#### Default()

**Signature:**

```go
func (o *OpenApiConfig) Default() OpenApiConfig
```

---

#### ParseRequest

Request body for `POST /asyncapi/parse`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Spec` | `interface{}` | — | Spec |

---

#### ParseResult

Full parse result returned by `POST /asyncapi/parse`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `SpecVersion` | `string` | — | Spec version |
| `Title` | `string` | — | Title |
| `ApiVersion` | `string` | — | Api version |
| `Channels` | `[]ParsedChannel` | — | Channels |
| `Operations` | `[]ParsedOperation` | — | Operations |
| `Messages` | `[]ParsedMessage` | — | Messages |

---

#### ParsedChannel

A single channel extracted from an AsyncAPI spec

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | Channel key from the spec (e.g. "chat/messages") |
| `Address` | `string` | — | Channel address / path |
| `Messages` | `[]string` | — | Message names declared on this channel |
| `Bindings` | `*interface{}` | `nil` | Bindings (ws / http / amqp / …) as raw JSON for forward-compatibility |

---

#### ParsedMessage

A resolved message (name + JSON Schema)

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | Message name |
| `Schema` | `*interface{}` | `nil` | Resolved JSON Schema for the message payload, if available |

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
| `Status` | `uint16` | — | The HTTP status code generated by the origin server. This is advisory; the actual HTTP status code takes precedence. |
| `Detail` | `*string` | `nil` | A human-readable explanation specific to this occurrence of the problem. |
| `Instance` | `*string` | `nil` | A URI reference that identifies the specific occurrence of the problem. It may or may not yield further information if dereferenced. |
| `Extensions` | `map[string]interface{}` | — | Extension members - problem-type-specific data. For validation errors, this typically contains an "errors" array. |

### Methods

#### WithDetail()

Set the detail field

**Signature:**

```go
func (o *ProblemDetails) WithDetail(detail string) ProblemDetails
```

#### WithInstance()

Set the instance field

**Signature:**

```go
func (o *ProblemDetails) WithInstance(instance string) ProblemDetails
```

#### NotFound()

Create a not found error

**Signature:**

```go
func (o *ProblemDetails) NotFound(detail string) ProblemDetails
```

#### MethodNotAllowed()

Create a method not allowed error

**Signature:**

```go
func (o *ProblemDetails) MethodNotAllowed(detail string) ProblemDetails
```

#### InternalServerError()

Create an internal server error

**Signature:**

```go
func (o *ProblemDetails) InternalServerError(detail string) ProblemDetails
```

#### BadRequest()

Create a bad request error

**Signature:**

```go
func (o *ProblemDetails) BadRequest(detail string) ProblemDetails
```

#### ToJson()

Serialize to JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```go
func (o *ProblemDetails) ToJson() (string, error)
```

#### ToJsonPretty()

Serialize to pretty JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```go
func (o *ProblemDetails) ToJsonPretty() (string, error)
```

---

#### QueryMutationConfig

Configuration for schemas with Query and Mutation types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `IntrospectionEnabled` | `bool` | `true` | Enable introspection queries |
| `ComplexityLimit` | `*int` | `nil` | Maximum query complexity (None = unlimited) |
| `DepthLimit` | `*int` | `nil` | Maximum query depth (None = unlimited) |

### Methods

#### Default()

**Signature:**

```go
func (o *QueryMutationConfig) Default() QueryMutationConfig
```

---

#### QueryOnlyConfig

Configuration for schemas with only Query type

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `IntrospectionEnabled` | `bool` | `true` | Enable introspection queries |
| `ComplexityLimit` | `*int` | `nil` | Maximum query complexity (None = unlimited) |
| `DepthLimit` | `*int` | `nil` | Maximum query depth (None = unlimited) |

### Methods

#### Default()

**Signature:**

```go
func (o *QueryOnlyConfig) Default() QueryOnlyConfig
```

---

#### RateLimitConfig

Rate limiting configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `PerSecond` | `uint64` | `100` | Requests per second |
| `Burst` | `uint32` | `200` | Burst allowance |
| `IpBased` | `bool` | `true` | Use IP-based rate limiting |

### Methods

#### Default()

**Signature:**

```go
func (o *RateLimitConfig) Default() RateLimitConfig
```

---

#### Request

---

#### Response

HTTP Response with custom status code, headers, and content

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `*interface{}` | `nil` | Response body content |
| `StatusCode` | `uint16` | — | HTTP status code (defaults to 200) |
| `Headers` | `map[string]string` | `nil` | Response headers |

### Methods

#### SetHeader()

Set a header

**Signature:**

```go
func (o *Response) SetHeader(key string, value string)
```

#### SetCookie()

Set a cookie in the response

**Signature:**

```go
func (o *Response) SetCookie(key string, value string, secure bool, httpOnly bool, maxAge int64, domain string, path string, sameSite string)
```

#### Default()

**Signature:**

```go
func (o *Response) Default() Response
```

---

#### RouteBuilder

Builder for defining a route.

### Methods

#### New()

Create a new builder for the provided HTTP method and path.

**Signature:**

```go
func (o *RouteBuilder) New(method Method, path string) RouteBuilder
```

#### HandlerName()

Assign an explicit handler name.

**Signature:**

```go
func (o *RouteBuilder) HandlerName(name string) RouteBuilder
```

#### RequestSchemaJson()

Provide a raw JSON schema for the request body.

**Signature:**

```go
func (o *RouteBuilder) RequestSchemaJson(schema interface{}) RouteBuilder
```

#### ResponseSchemaJson()

Provide a raw JSON schema for the response body.

**Signature:**

```go
func (o *RouteBuilder) ResponseSchemaJson(schema interface{}) RouteBuilder
```

#### ParamsSchemaJson()

Provide a raw JSON schema for request parameters.

**Signature:**

```go
func (o *RouteBuilder) ParamsSchemaJson(schema interface{}) RouteBuilder
```

#### FileParamsJson()

Provide multipart file parameter configuration.

**Signature:**

```go
func (o *RouteBuilder) FileParamsJson(schema interface{}) RouteBuilder
```

#### Cors()

Attach a CORS configuration for this route.

**Signature:**

```go
func (o *RouteBuilder) Cors(cors CorsConfig) RouteBuilder
```

#### Compression()

Attach a compression configuration for this route.

**Signature:**

```go
func (o *RouteBuilder) Compression(compression CompressionConfig) RouteBuilder
```

#### Sync()

Mark the route as synchronous.

**Signature:**

```go
func (o *RouteBuilder) Sync() RouteBuilder
```

#### HandlerDependencies()

Declare the dependency keys that must be resolved before this handler runs.

**Signature:**

```go
func (o *RouteBuilder) HandlerDependencies(dependencies []string) RouteBuilder
```

---

#### SchemaConfig

Configuration for GraphQL schema building.

Encapsulates all schema-level configuration options including
introspection control, complexity limits, and depth limits.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `IntrospectionEnabled` | `bool` | `true` | Enable introspection queries |
| `ComplexityLimit` | `*int` | `nil` | Maximum query complexity (None = unlimited) |
| `DepthLimit` | `*int` | `nil` | Maximum query depth (None = unlimited) |

### Methods

#### Default()

**Signature:**

```go
func (o *SchemaConfig) Default() SchemaConfig
```

---

#### ServerConfig

Server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Host` | `string` | `"127.0.0.1"` | Host to bind to |
| `Port` | `uint16` | `8000` | Port to bind to |
| `Workers` | `int` | `1` | Number of Tokio runtime worker threads used by binding-managed server runtimes |
| `EnableRequestId` | `bool` | `false` | Enable request ID generation and propagation |
| `MaxBodySize` | `*int` | `nil` | Maximum request body size in bytes (None = unlimited, not recommended) |
| `RequestTimeout` | `*uint64` | `nil` | Request timeout in seconds (None = no timeout) |
| `Compression` | `*CompressionConfig` | `nil` | Enable compression middleware |
| `RateLimit` | `*RateLimitConfig` | `nil` | Enable rate limiting |
| `JwtAuth` | `*JwtConfig` | `nil` | JWT authentication configuration |
| `ApiKeyAuth` | `*ApiKeyConfig` | `nil` | API Key authentication configuration |
| `StaticFiles` | `[]StaticFilesConfig` | `nil` | Static file serving configuration |
| `GracefulShutdown` | `bool` | `true` | Enable graceful shutdown on SIGTERM/SIGINT |
| `ShutdownTimeout` | `uint64` | `30` | Graceful shutdown timeout (seconds) |
| `Asyncapi` | `*AsyncApiConfig` | `nil` | AsyncAPI HTTP endpoint configuration |
| `Openapi` | `*OpenApiConfig` | `nil` | OpenAPI documentation configuration |
| `Jsonrpc` | `*JsonRpcConfig` | `nil` | JSON-RPC configuration |
| `Grpc` | `*GrpcConfig` | `nil` | gRPC configuration |
| `LifecycleHooks` | `*string` | `nil` | Lifecycle hooks for request/response processing |
| `BackgroundTasks` | `BackgroundTaskConfig` | — | Background task executor configuration |
| `EnableHttpTrace` | `bool` | `false` | Enable per-request HTTP tracing (tower-http `TraceLayer`) |
| `DiContainer` | `*string` | `nil` | Dependency injection container (requires 'di' feature) |

### Methods

#### Default()

**Signature:**

```go
func (o *ServerConfig) Default() ServerConfig
```

---

#### ServerInfo

Server information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string` | — | Base URL of the server (e.g. `"<https://api.example.com/v1"`>). |
| `Description` | `*string` | `nil` | Optional human-readable description of the server environment. |

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
| `EventType` | `*string` | `nil` | Event type (optional) |
| `Data` | `interface{}` | — | Event data (JSON value) |
| `Id` | `*string` | `nil` | Event ID (optional, for client-side reconnection) |
| `Retry` | `*uint64` | `nil` | Retry timeout in milliseconds (optional) |

### Methods

#### WithId()

Set the event ID for client-side reconnection support

Sets an ID that clients can use to resume from this point if they disconnect.
The client sends this ID back in the `Last-Event-ID` header when reconnecting.

**Signature:**

```go
func (o *SseEvent) WithId(id string) SseEvent
```

#### WithRetry()

Set the retry timeout for client reconnection

Sets the time in milliseconds clients should wait before attempting to reconnect
if the connection is lost. The client browser will automatically handle reconnection.

**Signature:**

```go
func (o *SseEvent) WithRetry(retryMs uint64) SseEvent
```

---

#### StaticFilesConfig

Static file serving configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Directory` | `string` | — | Directory path to serve |
| `RoutePrefix` | `string` | — | URL path prefix (e.g., "/static") |
| `IndexFile` | `bool` | `/* serde(default) */` | Fallback to index.html for directories |
| `CacheControl` | `*string` | `nil` | Cache-Control header value |

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
| `ContentType` | `*string` | `nil` | MIME type of the uploaded file |
| `Size` | `*int` | `nil` | Size of the file in bytes |
| `Content` | `[]byte` | — | File content (may be base64 encoded) |
| `ContentEncoding` | `*string` | `nil` | Content encoding type |
| `Cursor` | `string` | — | Internal cursor for Read/Seek operations |

### Methods

#### AsBytes()

Get the raw file content as bytes.

This provides zero-copy access to the underlying buffer.

**Signature:**

```go
func (o *UploadFile) AsBytes() []byte
```

#### ReadToString()

Read the file content as a UTF-8 string.

**Errors:**

Returns an error if the content is not valid UTF-8.

**Signature:**

```go
func (o *UploadFile) ReadToString() (string, error)
```

#### ContentTypeOrDefault()

Get the content type, defaulting to "application/octet-stream".

**Signature:**

```go
func (o *UploadFile) ContentTypeOrDefault() string
```

---

#### ValidateRequest

Request body for `POST /asyncapi/validate`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Spec` | `interface{}` | — | Spec |
| `Channel` | `string` | — | Channel |
| `Message` | `string` | — | Message |
| `Payload` | `interface{}` | — | Payload |

---

#### ValidationResponse

Response body for `POST /asyncapi/validate`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Valid` | `bool` | — | Valid |
| `Errors` | `[]string` | — | Errors |

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

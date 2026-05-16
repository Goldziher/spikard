---
title: "Go API Reference"
---

## Go API Reference <span class="version-badge">v0.14.0</span>

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

| Field        | Type       | Default | Description                              |
| ------------ | ---------- | ------- | ---------------------------------------- |
| `Keys`       | `[]string` | —       | Valid API keys                           |
| `HeaderName` | `string`   | —       | Header name to check (e.g., "X-API-Key") |

---

#### AsyncApiConfig

AsyncAPI HTTP endpoint configuration

| Field     | Type           | Default | Description                                                   |
| --------- | -------------- | ------- | ------------------------------------------------------------- |
| `Enabled` | `bool`         | —       | Enable AsyncAPI endpoints (default: false)                    |
| `Spec`    | `*interface{}` | `nil`   | Pre-registered AsyncAPI spec to serve from GET /asyncapi.json |

---

#### BackgroundJobMetadata

| Field       | Type      | Default | Description |
| ----------- | --------- | ------- | ----------- |
| `Name`      | `string`  | —       | The name    |
| `RequestId` | `*string` | `nil`   | Request id  |

##### Methods

###### Default()

**Signature:**

```go
func (o *BackgroundJobMetadata) Default() BackgroundJobMetadata
```

---

#### BackgroundTaskConfig

Configuration for in-process background task execution.

| Field                | Type     | Default | Description              |
| -------------------- | -------- | ------- | ------------------------ |
| `MaxQueueSize`       | `int`    | `1024`  | Maximum queue size       |
| `MaxConcurrentTasks` | `int`    | `128`   | Maximum concurrent tasks |
| `DrainTimeoutSecs`   | `uint64` | `30`    | Drain timeout secs       |

##### Methods

###### Default()

**Signature:**

```go
func (o *BackgroundTaskConfig) Default() BackgroundTaskConfig
```

---

#### CompressionConfig

Compression configuration shared across runtimes

| Field     | Type     | Default | Description                                         |
| --------- | -------- | ------- | --------------------------------------------------- |
| `Gzip`    | `bool`   | `true`  | Enable gzip compression                             |
| `Brotli`  | `bool`   | `true`  | Enable brotli compression                           |
| `MinSize` | `int`    | —       | Minimum response size to compress (bytes)           |
| `Quality` | `uint32` | —       | Compression quality (0-11 for brotli, 0-9 for gzip) |

##### Methods

###### Default()

**Signature:**

```go
func (o *CompressionConfig) Default() CompressionConfig
```

---

#### ContactInfo

Contact information

| Field   | Type      | Default | Description |
| ------- | --------- | ------- | ----------- |
| `Name`  | `*string` | `nil`   | The name    |
| `Email` | `*string` | `nil`   | Email       |
| `Url`   | `*string` | `nil`   | Url         |

---

#### CorsConfig

CORS configuration for a route

| Field              | Type        | Default | Description       |
| ------------------ | ----------- | ------- | ----------------- |
| `AllowedOrigins`   | `[]string`  | `nil`   | Allowed origins   |
| `AllowedMethods`   | `[]string`  | `nil`   | Allowed methods   |
| `AllowedHeaders`   | `[]string`  | `nil`   | Allowed headers   |
| `ExposeHeaders`    | `*[]string` | `nil`   | Expose headers    |
| `MaxAge`           | `*uint32`   | `nil`   | Maximum age       |
| `AllowCredentials` | `*bool`     | `nil`   | Allow credentials |

##### Methods

###### AllowedMethodsJoined()

Get the cached joined methods string for preflight responses

**Signature:**

```go
func (o *CorsConfig) AllowedMethodsJoined() string
```

###### AllowedHeadersJoined()

Get the cached joined headers string for preflight responses

**Signature:**

```go
func (o *CorsConfig) AllowedHeadersJoined() string
```

###### IsOriginAllowed()

Check if an origin is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```go
func (o *CorsConfig) IsOriginAllowed(origin string) bool
```

###### IsMethodAllowed()

Check if a method is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```go
func (o *CorsConfig) IsMethodAllowed(method string) bool
```

###### AreHeadersAllowed()

Check if all requested headers are allowed (O(n) where n = num requested headers)

**Signature:**

```go
func (o *CorsConfig) AreHeadersAllowed(requested []string) bool
```

###### Default()

**Signature:**

```go
func (o *CorsConfig) Default() CorsConfig
```

---

#### FullSchemaConfig

Configuration for fully-featured schemas with Query, Mutation, and Subscription types

| Field                  | Type   | Default | Description                                 |
| ---------------------- | ------ | ------- | ------------------------------------------- |
| `IntrospectionEnabled` | `bool` | `true`  | Enable introspection queries                |
| `ComplexityLimit`      | `*int` | `nil`   | Maximum query complexity (None = unlimited) |
| `DepthLimit`           | `*int` | `nil`   | Maximum query depth (None = unlimited)      |

##### Methods

###### Default()

**Signature:**

```go
func (o *FullSchemaConfig) Default() FullSchemaConfig
```

---

#### GraphQlRouteConfig

Configuration for GraphQL routes

Provides a builder pattern for configuring GraphQL route parameters
for the Spikard HTTP server's routing system.

##### Methods

###### Path()

Set the HTTP path for the GraphQL endpoint

**Signature:**

```go
func (o *GraphQlRouteConfig) Path(path string) GraphQlRouteConfig
```

###### Method()

Set the HTTP method for the GraphQL endpoint

**Signature:**

```go
func (o *GraphQlRouteConfig) Method(method string) GraphQlRouteConfig
```

###### EnablePlayground()

Enable or disable the GraphQL Playground UI

**Signature:**

```go
func (o *GraphQlRouteConfig) EnablePlayground(enable bool) GraphQlRouteConfig
```

###### Description()

Set a custom description for documentation

**Signature:**

```go
func (o *GraphQlRouteConfig) Description(description string) GraphQlRouteConfig
```

###### GetPath()

Get the configured path

**Signature:**

```go
func (o *GraphQlRouteConfig) GetPath() string
```

###### GetMethod()

Get the configured method

**Signature:**

```go
func (o *GraphQlRouteConfig) GetMethod() string
```

###### IsPlaygroundEnabled()

Check if playground is enabled

**Signature:**

```go
func (o *GraphQlRouteConfig) IsPlaygroundEnabled() bool
```

###### GetDescription()

Get the description if set

**Signature:**

```go
func (o *GraphQlRouteConfig) GetDescription() *string
```

###### Default()

**Signature:**

```go
func (o *GraphQlRouteConfig) Default() GraphQlRouteConfig
```

---

#### GraphQlSubscriptionSnapshot

Snapshot of a GraphQL subscription exchange over WebSocket.

| Field              | Type            | Default | Description                                                       |
| ------------------ | --------------- | ------- | ----------------------------------------------------------------- |
| `OperationId`      | `string`        | —       | Operation id used for the subscription request.                   |
| `Acknowledged`     | `bool`          | —       | Whether the server acknowledged the GraphQL WebSocket connection. |
| `Event`            | `*interface{}`  | `nil`   | First `next.payload` received for this subscription, if any.      |
| `Errors`           | `[]interface{}` | —       | GraphQL protocol errors emitted by the server.                    |
| `CompleteReceived` | `bool`          | —       | Whether a `complete` frame was observed for this operation.       |

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
  `tonic.Status.resource_exhausted`. Defaults to `nil` (unbounded).

| Field                    | Type      | Default | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| ------------------------ | --------- | ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Enabled`                | `bool`    | `true`  | Enable gRPC support                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `MaxMessageSize`         | `int`     | —       | Maximum message size in bytes (for both sending and receiving) This limit applies to individual messages in both unary and streaming RPCs. When a single message exceeds this size, the request is rejected with HTTP 413 (Payload Too Large). Default: 4MB (4194304 bytes) **Note:** This limit does NOT apply to the total response size in streaming RPCs. For multi-message streams, the total response can exceed this limit as long as each individual message stays within the limit.                                                                                                                                                                                                                                                               |
| `EnableCompression`      | `bool`    | `true`  | Enable gzip compression for gRPC messages                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `RequestTimeout`         | `*uint64` | `nil`   | Timeout for gRPC requests in seconds (None = no timeout)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `MaxConcurrentStreams`   | `uint32`  | —       | Maximum number of concurrent streams per connection (HTTP/2 advisory) This value is communicated to HTTP/2 clients as the server's flow control limit. The HTTP/2 transport layer enforces this limit automatically via SETTINGS frames and GOAWAY responses. Applications should NOT implement custom enforcement. Default: 100 streams per connection # Stream Limiting Strategy - **Per Connection**: This limit applies per HTTP/2 connection, not globally - **Transport Enforcement**: HTTP/2 handles all stream limiting; applications need not implement custom checks - **Streaming Requests**: In server streaming or bidi streaming, each logical RPC consumes one stream slot. Message ordering within a stream follows HTTP/2 frame ordering. |
| `EnableKeepalive`        | `bool`    | `true`  | Enable HTTP/2 keepalive                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `KeepaliveInterval`      | `uint64`  | —       | HTTP/2 keepalive interval in seconds                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `KeepaliveTimeout`       | `uint64`  | —       | HTTP/2 keepalive timeout in seconds                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `MaxStreamResponseBytes` | `*int`    | `nil`   | Total byte cap across an entire streaming response. When `Some(n)`, the streaming adapter aborts the stream with `tonic.Status.resource_exhausted` once the cumulative encoded message bytes exceed `n`. The stream yields the error item and then terminates. Per-message cap remains `max_message_size`. This limit applies to server-streaming and bidirectional-streaming RPCs only; unary RPCs are governed solely by `max_message_size`. Default: `nil` (unbounded total response size).                                                                                                                                                                                                                                                             |

##### Methods

###### Default()

**Signature:**

```go
func (o *GrpcConfig) Default() GrpcConfig
```

---

#### JsonRpcConfig

JSON-RPC server configuration

| Field          | Type     | Default | Description                                                |
| -------------- | -------- | ------- | ---------------------------------------------------------- |
| `Enabled`      | `bool`   | `true`  | Enable JSON-RPC endpoint                                   |
| `EndpointPath` | `string` | —       | HTTP endpoint path for JSON-RPC requests (default: "/rpc") |
| `EnableBatch`  | `bool`   | —       | Enable batch request processing (default: true)            |
| `MaxBatchSize` | `int`    | —       | Maximum number of requests in a batch (default: 100)       |

##### Methods

###### Default()

**Signature:**

```go
func (o *JsonRpcConfig) Default() JsonRpcConfig
```

---

#### JsonRpcMethodInfo

JSON-RPC method metadata for routes that support JSON-RPC

This struct captures the metadata needed to expose HTTP routes as JSON-RPC methods,
enabling discovery and documentation of RPC-compatible endpoints.

| Field          | Type           | Default | Description                                    |
| -------------- | -------------- | ------- | ---------------------------------------------- |
| `MethodName`   | `string`       | —       | The JSON-RPC method name (e.g., "user.create") |
| `Description`  | `*string`      | `nil`   | Optional description of what the method does   |
| `ParamsSchema` | `*interface{}` | `nil`   | Optional JSON Schema for method parameters     |
| `ResultSchema` | `*interface{}` | `nil`   | Optional JSON Schema for the result            |
| `Deprecated`   | `bool`         | —       | Whether this method is deprecated              |
| `Tags`         | `[]string`     | —       | Tags for categorizing and grouping methods     |

---

#### JwtConfig

JWT authentication configuration

| Field       | Type        | Default | Description                                           |
| ----------- | ----------- | ------- | ----------------------------------------------------- |
| `Secret`    | `string`    | —       | Secret key for JWT verification                       |
| `Algorithm` | `string`    | —       | Required algorithm (HS256, HS384, HS512, RS256, etc.) |
| `Audience`  | `*[]string` | `nil`   | Required audience claim                               |
| `Issuer`    | `*string`   | `nil`   | Required issuer claim                                 |
| `Leeway`    | `uint64`    | —       | Leeway for expiration checks (seconds)                |

---

#### LicenseInfo

License information

| Field  | Type      | Default | Description |
| ------ | --------- | ------- | ----------- |
| `Name` | `string`  | —       | The name    |
| `Url`  | `*string` | `nil`   | Url         |

---

#### OpenApiConfig

OpenAPI configuration

| Field             | Type                            | Default   | Description                                                      |
| ----------------- | ------------------------------- | --------- | ---------------------------------------------------------------- |
| `Enabled`         | `bool`                          | `false`   | Enable OpenAPI generation (default: false for zero overhead)     |
| `Title`           | `string`                        | `"API"`   | API title                                                        |
| `Version`         | `string`                        | `"1.0.0"` | API version                                                      |
| `Description`     | `*string`                       | `nil`     | API description (supports markdown)                              |
| `SwaggerUiPath`   | `string`                        | —         | Path to serve Swagger UI (default: "/docs")                      |
| `RedocPath`       | `string`                        | —         | Path to serve Redoc (default: "/redoc")                          |
| `OpenapiJsonPath` | `string`                        | —         | Path to serve OpenAPI JSON spec (default: "/openapi.json")       |
| `Contact`         | `*ContactInfo`                  | `nil`     | Contact information                                              |
| `License`         | `*LicenseInfo`                  | `nil`     | License information                                              |
| `Servers`         | `[]ServerInfo`                  | `nil`     | Server definitions                                               |
| `SecuritySchemes` | `map[string]SecuritySchemeInfo` | `nil`     | Security schemes (auto-detected from middleware if not provided) |

##### Methods

###### Default()

**Signature:**

```go
func (o *OpenApiConfig) Default() OpenApiConfig
```

---

#### ParseRequest

Request body for `POST /asyncapi/parse`

| Field  | Type          | Default | Description |
| ------ | ------------- | ------- | ----------- |
| `Spec` | `interface{}` | —       | Spec        |

---

#### ParseResult

Full parse result returned by `POST /asyncapi/parse`

| Field         | Type                | Default | Description  |
| ------------- | ------------------- | ------- | ------------ |
| `SpecVersion` | `string`            | —       | Spec version |
| `Title`       | `string`            | —       | Title        |
| `ApiVersion`  | `string`            | —       | Api version  |
| `Channels`    | `[]ParsedChannel`   | —       | Channels     |
| `Operations`  | `[]ParsedOperation` | —       | Operations   |
| `Messages`    | `[]ParsedMessage`   | —       | Messages     |

---

#### ParsedChannel

A single channel extracted from an AsyncAPI spec

| Field      | Type           | Default | Description                                                           |
| ---------- | -------------- | ------- | --------------------------------------------------------------------- |
| `Name`     | `string`       | —       | Channel key from the spec (e.g. "chat/messages")                      |
| `Address`  | `string`       | —       | Channel address / path                                                |
| `Messages` | `[]string`     | —       | Message names declared on this channel                                |
| `Bindings` | `*interface{}` | `nil`   | Bindings (ws / http / amqp / …) as raw JSON for forward-compatibility |

---

#### ParsedMessage

A resolved message (name + JSON Schema)

| Field    | Type           | Default | Description                                                |
| -------- | -------------- | ------- | ---------------------------------------------------------- |
| `Name`   | `string`       | —       | Message name                                               |
| `Schema` | `*interface{}` | `nil`   | Resolved JSON Schema for the message payload, if available |

---

#### ParsedOperation

A single operation extracted from an AsyncAPI spec

| Field     | Type     | Default | Description                                      |
| --------- | -------- | ------- | ------------------------------------------------ |
| `Name`    | `string` | —       | Operation name                                   |
| `Action`  | `string` | —       | Operation action: "send" or "receive"            |
| `Channel` | `string` | —       | Channel reference (resolved to the channel name) |

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

| Field        | Type                     | Default | Description                                                                                                                                                  |
| ------------ | ------------------------ | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `TypeUri`    | `string`                 | —       | A URI reference that identifies the problem type. Defaults to "about:blank" when absent. Should be a stable, human-readable identifier for the problem type. |
| `Title`      | `string`                 | —       | A short, human-readable summary of the problem type. Should not change from occurrence to occurrence of the problem.                                         |
| `Status`     | `uint16`                 | —       | The HTTP status code generated by the origin server. This is advisory; the actual HTTP status code takes precedence.                                         |
| `Detail`     | `*string`                | `nil`   | A human-readable explanation specific to this occurrence of the problem.                                                                                     |
| `Instance`   | `*string`                | `nil`   | A URI reference that identifies the specific occurrence of the problem. It may or may not yield further information if dereferenced.                         |
| `Extensions` | `map[string]interface{}` | —       | Extension members - problem-type-specific data. For validation errors, this typically contains an "errors" array.                                            |

##### Methods

###### WithDetail()

Set the detail field

**Signature:**

```go
func (o *ProblemDetails) WithDetail(detail string) ProblemDetails
```

###### WithInstance()

Set the instance field

**Signature:**

```go
func (o *ProblemDetails) WithInstance(instance string) ProblemDetails
```

###### NotFound()

Create a not found error

**Signature:**

```go
func (o *ProblemDetails) NotFound(detail string) ProblemDetails
```

###### MethodNotAllowed()

Create a method not allowed error

**Signature:**

```go
func (o *ProblemDetails) MethodNotAllowed(detail string) ProblemDetails
```

###### InternalServerError()

Create an internal server error

**Signature:**

```go
func (o *ProblemDetails) InternalServerError(detail string) ProblemDetails
```

###### BadRequest()

Create a bad request error

**Signature:**

```go
func (o *ProblemDetails) BadRequest(detail string) ProblemDetails
```

###### ToJson()

Serialize to JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```go
func (o *ProblemDetails) ToJson() (string, error)
```

###### ToJsonPretty()

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

| Field                  | Type   | Default | Description                                 |
| ---------------------- | ------ | ------- | ------------------------------------------- |
| `IntrospectionEnabled` | `bool` | `true`  | Enable introspection queries                |
| `ComplexityLimit`      | `*int` | `nil`   | Maximum query complexity (None = unlimited) |
| `DepthLimit`           | `*int` | `nil`   | Maximum query depth (None = unlimited)      |

##### Methods

###### Default()

**Signature:**

```go
func (o *QueryMutationConfig) Default() QueryMutationConfig
```

---

#### QueryOnlyConfig

Configuration for schemas with only Query type

| Field                  | Type   | Default | Description                                 |
| ---------------------- | ------ | ------- | ------------------------------------------- |
| `IntrospectionEnabled` | `bool` | `true`  | Enable introspection queries                |
| `ComplexityLimit`      | `*int` | `nil`   | Maximum query complexity (None = unlimited) |
| `DepthLimit`           | `*int` | `nil`   | Maximum query depth (None = unlimited)      |

##### Methods

###### Default()

**Signature:**

```go
func (o *QueryOnlyConfig) Default() QueryOnlyConfig
```

---

#### RateLimitConfig

Rate limiting configuration shared across runtimes

| Field       | Type     | Default | Description                |
| ----------- | -------- | ------- | -------------------------- |
| `PerSecond` | `uint64` | `100`   | Requests per second        |
| `Burst`     | `uint32` | `200`   | Burst allowance            |
| `IpBased`   | `bool`   | `true`  | Use IP-based rate limiting |

##### Methods

###### Default()

**Signature:**

```go
func (o *RateLimitConfig) Default() RateLimitConfig
```

---

#### Response

HTTP Response with custom status code, headers, and content

| Field        | Type                | Default | Description                        |
| ------------ | ------------------- | ------- | ---------------------------------- |
| `Content`    | `*interface{}`      | `nil`   | Response body content              |
| `StatusCode` | `uint16`            | —       | HTTP status code (defaults to 200) |
| `Headers`    | `map[string]string` | `nil`   | Response headers                   |

##### Methods

###### SetHeader()

Set a header

**Signature:**

```go
func (o *Response) SetHeader(key string, value string)
```

###### SetCookie()

Set a cookie in the response

**Signature:**

```go
func (o *Response) SetCookie(key string, value string, secure bool, httpOnly bool, maxAge int64, domain string, path string, sameSite string)
```

###### Default()

**Signature:**

```go
func (o *Response) Default() Response
```

---

#### ResponseSnapshot

Snapshot of an Axum response used by higher-level language bindings.

| Field     | Type                | Default | Description                                                |
| --------- | ------------------- | ------- | ---------------------------------------------------------- |
| `Status`  | `uint16`            | —       | HTTP status code.                                          |
| `Headers` | `map[string]string` | —       | Response headers (lowercase keys for predictable lookups). |
| `Body`    | `[]byte`            | —       | Response body bytes (decoded for supported encodings).     |

##### Methods

###### Text()

Return response body as UTF-8 string.

**Signature:**

```go
func (o *ResponseSnapshot) Text() (string, error)
```

###### Json()

Parse response body as JSON.

**Signature:**

```go
func (o *ResponseSnapshot) Json() (interface{}, error)
```

###### Header()

Lookup header by case-insensitive name.

**Signature:**

```go
func (o *ResponseSnapshot) Header(name string) *string
```

###### GraphqlData()

Extract GraphQL data from response

**Signature:**

```go
func (o *ResponseSnapshot) GraphqlData() (interface{}, error)
```

###### GraphqlErrors()

Extract GraphQL errors from response

**Signature:**

```go
func (o *ResponseSnapshot) GraphqlErrors() ([]interface{}, error)
```

---

#### SchemaConfig

Configuration for GraphQL schema building.

Encapsulates all schema-level configuration options including
introspection control, complexity limits, and depth limits.

| Field                  | Type   | Default | Description                                 |
| ---------------------- | ------ | ------- | ------------------------------------------- |
| `IntrospectionEnabled` | `bool` | `true`  | Enable introspection queries                |
| `ComplexityLimit`      | `*int` | `nil`   | Maximum query complexity (None = unlimited) |
| `DepthLimit`           | `*int` | `nil`   | Maximum query depth (None = unlimited)      |

##### Methods

###### Default()

**Signature:**

```go
func (o *SchemaConfig) Default() SchemaConfig
```

---

#### ServerConfig

Server configuration

| Field              | Type                   | Default       | Description                                                                    |
| ------------------ | ---------------------- | ------------- | ------------------------------------------------------------------------------ |
| `Host`             | `string`               | `"127.0.0.1"` | Host to bind to                                                                |
| `Port`             | `uint16`               | `8000`        | Port to bind to                                                                |
| `Workers`          | `int`                  | `1`           | Number of Tokio runtime worker threads used by binding-managed server runtimes |
| `EnableRequestId`  | `bool`                 | `false`       | Enable request ID generation and propagation                                   |
| `MaxBodySize`      | `*int`                 | `nil`         | Maximum request body size in bytes (None = unlimited, not recommended)         |
| `RequestTimeout`   | `*uint64`              | `nil`         | Request timeout in seconds (None = no timeout)                                 |
| `Compression`      | `*CompressionConfig`   | `nil`         | Enable compression middleware                                                  |
| `RateLimit`        | `*RateLimitConfig`     | `nil`         | Enable rate limiting                                                           |
| `JwtAuth`          | `*JwtConfig`           | `nil`         | JWT authentication configuration                                               |
| `ApiKeyAuth`       | `*ApiKeyConfig`        | `nil`         | API Key authentication configuration                                           |
| `StaticFiles`      | `[]StaticFilesConfig`  | `nil`         | Static file serving configuration                                              |
| `GracefulShutdown` | `bool`                 | `true`        | Enable graceful shutdown on SIGTERM/SIGINT                                     |
| `ShutdownTimeout`  | `uint64`               | `30`          | Graceful shutdown timeout (seconds)                                            |
| `Asyncapi`         | `*AsyncApiConfig`      | `nil`         | AsyncAPI HTTP endpoint configuration                                           |
| `Openapi`          | `*OpenApiConfig`       | `nil`         | OpenAPI documentation configuration                                            |
| `Jsonrpc`          | `*JsonRpcConfig`       | `nil`         | JSON-RPC configuration                                                         |
| `Grpc`             | `*GrpcConfig`          | `nil`         | gRPC configuration                                                             |
| `BackgroundTasks`  | `BackgroundTaskConfig` | —             | Background task executor configuration                                         |
| `EnableHttpTrace`  | `bool`                 | `false`       | Enable per-request HTTP tracing (tower-http `TraceLayer`)                      |

##### Methods

###### Default()

**Signature:**

```go
func (o *ServerConfig) Default() ServerConfig
```

---

#### ServerInfo

Server information

| Field         | Type      | Default | Description                |
| ------------- | --------- | ------- | -------------------------- |
| `Url`         | `string`  | —       | Url                        |
| `Description` | `*string` | `nil`   | Human-readable description |

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

| Field       | Type          | Default | Description                                       |
| ----------- | ------------- | ------- | ------------------------------------------------- |
| `EventType` | `*string`     | `nil`   | Event type (optional)                             |
| `Data`      | `interface{}` | —       | Event data (JSON value)                           |
| `Id`        | `*string`     | `nil`   | Event ID (optional, for client-side reconnection) |
| `Retry`     | `*uint64`     | `nil`   | Retry timeout in milliseconds (optional)          |

##### Methods

###### WithId()

Set the event ID for client-side reconnection support

Sets an ID that clients can use to resume from this point if they disconnect.
The client sends this ID back in the `Last-Event-ID` header when reconnecting.

**Signature:**

```go
func (o *SseEvent) WithId(id string) SseEvent
```

###### WithRetry()

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

| Field          | Type      | Default | Description                            |
| -------------- | --------- | ------- | -------------------------------------- |
| `Directory`    | `string`  | —       | Directory path to serve                |
| `RoutePrefix`  | `string`  | —       | URL path prefix (e.g., "/static")      |
| `IndexFile`    | `bool`    | —       | Fallback to index.html for directories |
| `CacheControl` | `*string` | `nil`   | Cache-Control header value             |

---

#### TestClient

Core test client for making HTTP requests to a Spikard application.

This struct wraps axum-test's TestServer and provides a language-agnostic
interface for making HTTP requests, sending WebSocket connections, and
handling Server-Sent Events. Language bindings wrap this to provide
native API surfaces.

##### Methods

###### Get()

Make a GET request

**Signature:**

```go
func (o *TestClient) Get(path string, queryParams []string, headers []string) (ResponseSnapshot, error)
```

###### Post()

Make a POST request

**Signature:**

```go
func (o *TestClient) Post(path string, json interface{}, formData []string, multipart string, queryParams []string, headers []string) (ResponseSnapshot, error)
```

###### RequestRaw()

Make a request with a raw body payload.

**Signature:**

```go
func (o *TestClient) RequestRaw(method Method, path string, body []byte, queryParams []string, headers []string) (ResponseSnapshot, error)
```

###### Put()

Make a PUT request

**Signature:**

```go
func (o *TestClient) Put(path string, json interface{}, queryParams []string, headers []string) (ResponseSnapshot, error)
```

###### Patch()

Make a PATCH request

**Signature:**

```go
func (o *TestClient) Patch(path string, json interface{}, queryParams []string, headers []string) (ResponseSnapshot, error)
```

###### Delete()

Make a DELETE request

**Signature:**

```go
func (o *TestClient) Delete(path string, queryParams []string, headers []string) (ResponseSnapshot, error)
```

###### Options()

Make an OPTIONS request

**Signature:**

```go
func (o *TestClient) Options(path string, queryParams []string, headers []string) (ResponseSnapshot, error)
```

###### Head()

Make a HEAD request

**Signature:**

```go
func (o *TestClient) Head(path string, queryParams []string, headers []string) (ResponseSnapshot, error)
```

###### Trace()

Make a TRACE request

**Signature:**

```go
func (o *TestClient) Trace(path string, queryParams []string, headers []string) (ResponseSnapshot, error)
```

###### GraphqlAt()

Send a GraphQL query/mutation to a custom endpoint

**Signature:**

```go
func (o *TestClient) GraphqlAt(endpoint string, query string, variables interface{}, operationName string) (ResponseSnapshot, error)
```

###### Graphql()

Send a GraphQL query/mutation

**Signature:**

```go
func (o *TestClient) Graphql(query string, variables interface{}, operationName string) (ResponseSnapshot, error)
```

###### GraphqlWithStatus()

Send a GraphQL query and return HTTP status code separately

This method allows tests to distinguish between:

- HTTP-level errors (400/422 for invalid requests)
- GraphQL-level errors (200 with errors in response body)

**Signature:**

```go
func (o *TestClient) GraphqlWithStatus(query string, variables interface{}, operationName string) (string, error)
```

###### GraphqlSubscriptionAt()

Send a GraphQL subscription (WebSocket) to a custom endpoint.

Uses the `graphql-transport-ws` protocol and captures the first `next` payload.
After the first payload is received, this client sends `complete` to unsubscribe.

**Signature:**

```go
func (o *TestClient) GraphqlSubscriptionAt(endpoint string, query string, variables interface{}, operationName string) (GraphQlSubscriptionSnapshot, error)
```

###### GraphqlSubscription()

Send a GraphQL subscription (WebSocket).

Uses `/graphql` as the default subscription endpoint.

**Signature:**

```go
func (o *TestClient) GraphqlSubscription(query string, variables interface{}, operationName string) (GraphQlSubscriptionSnapshot, error)
```

---

#### UploadFile

Represents an uploaded file from multipart/form-data requests.

This struct provides efficient access to file content with automatic
base64 decoding and implements standard I/O traits for compatibility.

| Field             | Type      | Default | Description                              |
| ----------------- | --------- | ------- | ---------------------------------------- |
| `Filename`        | `string`  | —       | Original filename from the client        |
| `ContentType`     | `*string` | `nil`   | MIME type of the uploaded file           |
| `Size`            | `*int`    | `nil`   | Size of the file in bytes                |
| `Content`         | `[]byte`  | —       | File content (may be base64 encoded)     |
| `ContentEncoding` | `*string` | `nil`   | Content encoding type                    |
| `Cursor`          | `string`  | —       | Internal cursor for Read/Seek operations |

##### Methods

###### AsBytes()

Get the raw file content as bytes.

This provides zero-copy access to the underlying buffer.

**Signature:**

```go
func (o *UploadFile) AsBytes() []byte
```

###### ReadToString()

Read the file content as a UTF-8 string.

**Errors:**

Returns an error if the content is not valid UTF-8.

**Signature:**

```go
func (o *UploadFile) ReadToString() (string, error)
```

###### ContentTypeOrDefault()

Get the content type, defaulting to "application/octet-stream".

**Signature:**

```go
func (o *UploadFile) ContentTypeOrDefault() string
```

---

#### ValidateRequest

Request body for `POST /asyncapi/validate`

| Field     | Type          | Default | Description |
| --------- | ------------- | ------- | ----------- |
| `Spec`    | `interface{}` | —       | Spec        |
| `Channel` | `string`      | —       | Channel     |
| `Message` | `string`      | —       | Message     |
| `Payload` | `interface{}` | —       | Payload     |

---

#### ValidationResponse

Response body for `POST /asyncapi/validate`

| Field    | Type       | Default | Description |
| -------- | ---------- | ------- | ----------- |
| `Valid`  | `bool`     | —       | Valid       |
| `Errors` | `[]string` | —       | Errors      |

---

### Enums

#### SnapshotError

Possible errors while converting an Axum response into a snapshot.

| Value           | Description                                                            |
| --------------- | ---------------------------------------------------------------------- |
| `InvalidHeader` | Response header could not be decoded to UTF-8. — Fields: `0`: `string` |
| `Decompression` | Body decompression failed. — Fields: `0`: `string`                     |

---

#### WebSocketMessage

A WebSocket message that can be text or binary.

| Value    | Description                                                                                                                                                                                                                       |
| -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Text`   | A text message. — Fields: `0`: `string`                                                                                                                                                                                           |
| `Binary` | A binary message. — Fields: `0`: `[]byte`                                                                                                                                                                                         |
| `Close`  | A close message with a numeric close code (RFC 6455) and optional reason text. Common codes: 1000 Normal Closure, 1001 Going Away, 1005 No Status Received, 1006 Abnormal Closure. — Fields: `Code`: `uint16`, `Reason`: `string` |
| `Ping`   | A ping message. — Fields: `0`: `[]byte`                                                                                                                                                                                           |
| `Pong`   | A pong message. — Fields: `0`: `[]byte`                                                                                                                                                                                           |

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

| Value    | Description                                                 |
| -------- | ----------------------------------------------------------- |
| `Http`   | Http — Fields: `Scheme`: `string`, `BearerFormat`: `string` |
| `ApiKey` | Api key — Fields: `Location`: `string`, `Name`: `string`    |

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

---
title: "Rust API Reference"
---

## Rust API Reference <span class="version-badge">v0.15.6-rc.14</span>

### Functions

#### schema_query_only()

Create a simple schema configuration with only Query type.

This is a convenience function for schemas that only have queries.

**Returns:**

A `QueryOnlyConfig` with default settings

**Signature:**

```rust
pub fn schema_query_only() -> QueryOnlyConfig
```

**Returns:** `QueryOnlyConfig`

---

#### schema_query_mutation()

Create a schema configuration with Query and Mutation types.

This is a convenience function for schemas with queries and mutations but no subscriptions.

**Returns:**

A `QueryMutationConfig` with default settings

**Signature:**

```rust
pub fn schema_query_mutation() -> QueryMutationConfig
```

**Returns:** `QueryMutationConfig`

---

#### schema_full()

Create a schema configuration with all three root types.

This is a convenience function for fully-featured schemas.

**Returns:**

A `FullSchemaConfig` with default settings

**Signature:**

```rust
pub fn schema_full() -> FullSchemaConfig
```

**Returns:** `FullSchemaConfig`

---

### Types

#### ApiKeyConfig

API Key authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `keys` | `Vec<String>` | — | Valid API keys |
| `header_name` | `String` | `/* serde(default) */` | Header name to check (e.g., "X-API-Key") |

---

#### App

Spikard application builder.

### Methods

#### new()

Create a new application with the default server configuration.

**Signature:**

```rust
pub fn new() -> App
```

#### merge_axum_router()

Attach an existing Axum router to this application, returning ownership.

**Signature:**

```rust
pub fn merge_axum_router(&self, router: &str) -> App
```

#### attach_axum_router()

Attach an Axum router using a mutable reference for incremental configuration.

**Signature:**

```rust
pub fn attach_axum_router(&self, router: &str) -> App
```

#### into_router()

Build the underlying Axum router.

**Errors:**

Returns an error if server or router construction fails.

**Signature:**

```rust
pub fn into_router(&self) -> String
```

#### into_router_and_config()

Decompose the application into its Axum router and server configuration.

This is the low-level escape hatch used by the C FFI layer to start the
server on a background thread while retaining the bind address from the
caller-supplied `ServerConfig`. Prefer `App.run` for normal use.

**Errors:**

Returns an error if router construction fails.

**Signature:**

```rust
pub fn into_router_and_config(&self) -> String
```

#### default()

**Signature:**

```rust
pub fn default() -> App
```

---

#### AsyncApiConfig

AsyncAPI HTTP endpoint configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | — | Enable AsyncAPI endpoints (default: false) |
| `spec` | `Option<serde_json::Value>` | `Default::default()` | Pre-registered AsyncAPI spec to serve from GET /asyncapi.json |

---

#### BackgroundJobMetadata

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `request_id` | `Option<String>` | `None` | Request id |

### Methods

#### default()

**Signature:**

```rust
pub fn default() -> BackgroundJobMetadata
```

---

#### BackgroundTaskConfig

Configuration for in-process background task execution.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_queue_size` | `usize` | `1024` | Maximum queue size |
| `max_concurrent_tasks` | `usize` | `128` | Maximum concurrent tasks |
| `drain_timeout_secs` | `u64` | `30` | Drain timeout secs |

### Methods

#### default()

**Signature:**

```rust
pub fn default() -> BackgroundTaskConfig
```

---

#### CompressionConfig

Compression configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `gzip` | `bool` | `true` | Enable gzip compression |
| `brotli` | `bool` | `true` | Enable brotli compression |
| `min_size` | `usize` | — | Minimum response size to compress (bytes) |
| `quality` | `u32` | — | Compression quality (0-11 for brotli, 0-9 for gzip) |

### Methods

#### default()

**Signature:**

```rust
pub fn default() -> CompressionConfig
```

---

#### ContactInfo

Contact information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `Option<String>` | `None` | Name of the contact person or organisation. |
| `email` | `Option<String>` | `None` | Contact email address. |
| `url` | `Option<String>` | `None` | URL pointing to the contact information page. |

---

#### CorsConfig

CORS configuration for a route

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `allowed_origins` | `Vec<String>` | `vec![]` | Allowed origins |
| `allowed_methods` | `Vec<String>` | `vec![]` | Allowed methods |
| `allowed_headers` | `Vec<String>` | `vec![]` | Allowed headers |
| `expose_headers` | `Option<Vec<String>>` | `None` | Expose headers |
| `max_age` | `Option<u32>` | `None` | Maximum age |
| `allow_credentials` | `Option<bool>` | `None` | Allow credentials |
| `methods_joined_cache` | `String` | — | Methods joined cache |
| `headers_joined_cache` | `String` | — | Headers joined cache |

### Methods

#### allowed_methods_joined()

Get the cached joined methods string for preflight responses

**Signature:**

```rust
pub fn allowed_methods_joined(&self) -> String
```

#### allowed_headers_joined()

Get the cached joined headers string for preflight responses

**Signature:**

```rust
pub fn allowed_headers_joined(&self) -> String
```

#### is_origin_allowed()

Check if an origin is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```rust
pub fn is_origin_allowed(&self, origin: &str) -> bool
```

#### is_method_allowed()

Check if a method is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```rust
pub fn is_method_allowed(&self, method: &str) -> bool
```

#### default()

**Signature:**

```rust
pub fn default() -> CorsConfig
```

---

#### FullSchemaConfig

Configuration for fully-featured schemas with Query, Mutation, and Subscription types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `true` | Enable introspection queries |
| `complexity_limit` | `Option<usize>` | `None` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `Option<usize>` | `None` | Maximum query depth (None = unlimited) |

### Methods

#### default()

**Signature:**

```rust
pub fn default() -> FullSchemaConfig
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

```rust
pub fn new() -> GraphQlRouteConfig
```

#### path()

Set the HTTP path for the GraphQL endpoint

**Signature:**

```rust
pub fn path(&self, path: &str) -> GraphQlRouteConfig
```

#### method()

Set the HTTP method for the GraphQL endpoint

**Signature:**

```rust
pub fn method(&self, method: &str) -> GraphQlRouteConfig
```

#### enable_playground()

Enable or disable the GraphQL Playground UI

**Signature:**

```rust
pub fn enable_playground(&self, enable: bool) -> GraphQlRouteConfig
```

#### description()

Set a custom description for documentation

**Signature:**

```rust
pub fn description(&self, description: &str) -> GraphQlRouteConfig
```

#### get_path()

Get the configured path

**Signature:**

```rust
pub fn get_path(&self) -> String
```

#### get_method()

Get the configured method

**Signature:**

```rust
pub fn get_method(&self) -> String
```

#### is_playground_enabled()

Check if playground is enabled

**Signature:**

```rust
pub fn is_playground_enabled(&self) -> bool
```

#### get_description()

Get the description if set

**Signature:**

```rust
pub fn get_description(&self) -> Option<String>
```

#### default()

**Signature:**

```rust
pub fn default() -> GraphQlRouteConfig
```

---

#### GraphQlSubscriptionSnapshot

Snapshot of a GraphQL subscription exchange over WebSocket.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `operation_id` | `String` | — | Operation id used for the subscription request. |
| `acknowledged` | `bool` | — | Whether the server acknowledged the GraphQL WebSocket connection. |
| `event` | `Option<serde_json::Value>` | `None` | First `next.payload` received for this subscription, if any. |
| `errors` | `Vec<serde_json::Value>` | — | GraphQL protocol errors emitted by the server. |
| `complete_received` | `bool` | — | Whether a `complete` frame was observed for this operation. |

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
  `tonic.Status.resource_exhausted`. Defaults to `None` (unbounded).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `true` | Enable gRPC support |
| `max_message_size` | `usize` | — | Maximum message size in bytes (for both sending and receiving) This limit applies to individual messages in both unary and streaming RPCs. When a single message exceeds this size, the request is rejected with HTTP 413 (Payload Too Large). Default: 4MB (4194304 bytes) **Note:** This limit does NOT apply to the total response size in streaming RPCs. For multi-message streams, the total response can exceed this limit as long as each individual message stays within the limit. |
| `enable_compression` | `bool` | `true` | Enable gzip compression for gRPC messages |
| `request_timeout` | `Option<u64>` | `None` | Timeout for gRPC requests in seconds (None = no timeout) |
| `max_concurrent_streams` | `u32` | — | Maximum number of concurrent streams per connection (HTTP/2 advisory) This value is communicated to HTTP/2 clients as the server's flow control limit. The HTTP/2 transport layer enforces this limit automatically via SETTINGS frames and GOAWAY responses. Applications should NOT implement custom enforcement. Default: 100 streams per connection # Stream Limiting Strategy - **Per Connection**: This limit applies per HTTP/2 connection, not globally - **Transport Enforcement**: HTTP/2 handles all stream limiting; applications need not implement custom checks - **Streaming Requests**: In server streaming or bidi streaming, each logical RPC consumes one stream slot. Message ordering within a stream follows HTTP/2 frame ordering. |
| `enable_keepalive` | `bool` | `true` | Enable HTTP/2 keepalive |
| `keepalive_interval` | `u64` | — | HTTP/2 keepalive interval in seconds |
| `keepalive_timeout` | `u64` | — | HTTP/2 keepalive timeout in seconds |
| `max_stream_response_bytes` | `Option<usize>` | `None` | Total byte cap across an entire streaming response. When `Some(n)`, the streaming adapter aborts the stream with `tonic.Status.resource_exhausted` once the cumulative encoded message bytes exceed `n`. The stream yields the error item and then terminates. Per-message cap remains `max_message_size`. This limit applies to server-streaming and bidirectional-streaming RPCs only; unary RPCs are governed solely by `max_message_size`. Default: `None` (unbounded total response size). |

### Methods

#### default()

**Signature:**

```rust
pub fn default() -> GrpcConfig
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

```rust
pub fn call(&self, request: Request, request_data: RequestData) -> HandlerResult
```

#### prefers_raw_json_body()

Whether this handler prefers consuming `RequestData.raw_body` over the parsed
`RequestData.body` for JSON requests.

When `true`, the server may skip eager JSON parsing when there is no request-body
schema validator attached to the route.

**Signature:**

```rust
pub fn prefers_raw_json_body(&self) -> bool
```

#### prefers_parameter_extraction()

Whether this handler wants to perform its own parameter validation/extraction (path/query/header/cookie).

When `true`, the server will skip `ParameterValidator.validate_and_extract` in `ValidatingHandler`.
This is useful for language bindings which need to transform validated parameters into
language-specific values (e.g., Python kwargs) without duplicating work. When `false`,
the server stores validated output in `RequestData.validated_params`.

**Signature:**

```rust
pub fn prefers_parameter_extraction(&self) -> bool
```

#### wants_headers()

Whether this handler needs the parsed headers map in `RequestData`.

When `false`, the server may skip building `RequestData.headers` for requests without a body.
(Requests with bodies still typically need `Content-Type` decisions.)

**Signature:**

```rust
pub fn wants_headers(&self) -> bool
```

#### wants_cookies()

Whether this handler needs the parsed cookies map in `RequestData`.

When `false`, the server may skip parsing cookies for requests without a body.

**Signature:**

```rust
pub fn wants_cookies(&self) -> bool
```

#### wants_request_extensions()

Whether this handler needs `RequestData` stored in request extensions.

When `false`, the server avoids inserting `RequestData` into extensions to
skip cloning in hot paths.

**Signature:**

```rust
pub fn wants_request_extensions(&self) -> bool
```

#### static_response()

Return a pre-built static response if this handler always produces the
same output. When `Some`, the server bypasses the full middleware
pipeline and serves the pre-built response directly.

**Signature:**

```rust
pub fn static_response(&self) -> Option<StaticResponse>
```

---

#### IntoHandler

Convert user-facing handler functions into the low-level `Handler` trait.

### Methods

#### into_handler()

Convert this value into a shared request handler.

**Signature:**

```rust
pub fn into_handler(&self) -> Handler
```

---

#### JsonRpcConfig

JSON-RPC server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `true` | Enable JSON-RPC endpoint |
| `endpoint_path` | `String` | — | HTTP endpoint path for JSON-RPC requests (default: "/rpc") |
| `enable_batch` | `bool` | — | Enable batch request processing (default: true) |
| `max_batch_size` | `usize` | — | Maximum number of requests in a batch (default: 100) |

### Methods

#### default()

**Signature:**

```rust
pub fn default() -> JsonRpcConfig
```

---

#### JsonRpcMethodInfo

JSON-RPC method metadata for routes that support JSON-RPC

This struct captures the metadata needed to expose HTTP routes as JSON-RPC methods,
enabling discovery and documentation of RPC-compatible endpoints.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `method_name` | `String` | — | The JSON-RPC method name (e.g., "user.create") |
| `description` | `Option<String>` | `None` | Optional description of what the method does |
| `params_schema` | `Option<serde_json::Value>` | `None` | Optional JSON Schema for method parameters |
| `result_schema` | `Option<serde_json::Value>` | `None` | Optional JSON Schema for the result |
| `deprecated` | `bool` | `/* serde(default) */` | Whether this method is deprecated |
| `tags` | `Vec<String>` | `/* serde(default) */` | Tags for categorizing and grouping methods |

---

#### JwtConfig

JWT authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `secret` | `String` | — | Secret key for JWT verification |
| `algorithm` | `String` | `/* serde(default) */` | Required algorithm (HS256, HS384, HS512, RS256, etc.) |
| `audience` | `Option<Vec<String>>` | `None` | Required audience claim |
| `issuer` | `Option<String>` | `None` | Required issuer claim |
| `leeway` | `u64` | `/* serde(default) */` | Leeway for expiration checks (seconds) |

---

#### LicenseInfo

License information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | SPDX license identifier or display name (e.g. `"MIT"`). |
| `url` | `Option<String>` | `None` | URL to the full license text. |

---

#### OpenApiConfig

OpenAPI configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `false` | Enable OpenAPI generation (default: false for zero overhead) |
| `title` | `String` | `"API"` | API title |
| `version` | `String` | `"1.0.0"` | API version |
| `description` | `Option<String>` | `None` | API description (supports markdown) |
| `swagger_ui_path` | `String` | — | Path to serve Swagger UI (default: "/docs") |
| `redoc_path` | `String` | — | Path to serve Redoc (default: "/redoc") |
| `openapi_json_path` | `String` | — | Path to serve OpenAPI JSON spec (default: "/openapi.json") |
| `contact` | `Option<ContactInfo>` | `None` | Contact information |
| `license` | `Option<LicenseInfo>` | `None` | License information |
| `servers` | `Vec<ServerInfo>` | `vec![]` | Server definitions |
| `security_schemes` | `HashMap<String, SecuritySchemeInfo>` | `HashMap::new()` | Security schemes (auto-detected from middleware if not provided) |

### Methods

#### default()

**Signature:**

```rust
pub fn default() -> OpenApiConfig
```

---

#### ParseRequest

Request body for `POST /asyncapi/parse`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `spec` | `serde_json::Value` | — | Spec |

---

#### ParseResult

Full parse result returned by `POST /asyncapi/parse`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `spec_version` | `String` | — | Spec version |
| `title` | `String` | — | Title |
| `api_version` | `String` | — | Api version |
| `channels` | `Vec<ParsedChannel>` | — | Channels |
| `operations` | `Vec<ParsedOperation>` | — | Operations |
| `messages` | `Vec<ParsedMessage>` | — | Messages |

---

#### ParsedChannel

A single channel extracted from an AsyncAPI spec

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | Channel key from the spec (e.g. "chat/messages") |
| `address` | `String` | — | Channel address / path |
| `messages` | `Vec<String>` | — | Message names declared on this channel |
| `bindings` | `Option<serde_json::Value>` | `None` | Bindings (ws / http / amqp / …) as raw JSON for forward-compatibility |

---

#### ParsedMessage

A resolved message (name + JSON Schema)

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | Message name |
| `schema` | `Option<serde_json::Value>` | `None` | Resolved JSON Schema for the message payload, if available |

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
| `type_uri` | `String` | — | A URI reference that identifies the problem type. Defaults to "about:blank" when absent. Should be a stable, human-readable identifier for the problem type. |
| `title` | `String` | — | A short, human-readable summary of the problem type. Should not change from occurrence to occurrence of the problem. |
| `status` | `u16` | — | The HTTP status code generated by the origin server. This is advisory; the actual HTTP status code takes precedence. |
| `detail` | `Option<String>` | `None` | A human-readable explanation specific to this occurrence of the problem. |
| `instance` | `Option<String>` | `None` | A URI reference that identifies the specific occurrence of the problem. It may or may not yield further information if dereferenced. |
| `extensions` | `HashMap<String, serde_json::Value>` | — | Extension members - problem-type-specific data. For validation errors, this typically contains an "errors" array. |

### Methods

#### with_detail()

Set the detail field

**Signature:**

```rust
pub fn with_detail(&self, detail: &str) -> ProblemDetails
```

#### with_instance()

Set the instance field

**Signature:**

```rust
pub fn with_instance(&self, instance: &str) -> ProblemDetails
```

#### not_found()

Create a not found error

**Signature:**

```rust
pub fn not_found(detail: &str) -> ProblemDetails
```

#### method_not_allowed()

Create a method not allowed error

**Signature:**

```rust
pub fn method_not_allowed(detail: &str) -> ProblemDetails
```

#### internal_server_error()

Create an internal server error

**Signature:**

```rust
pub fn internal_server_error(detail: &str) -> ProblemDetails
```

#### bad_request()

Create a bad request error

**Signature:**

```rust
pub fn bad_request(detail: &str) -> ProblemDetails
```

#### to_json()

Serialize to JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```rust
pub fn to_json(&self) -> String
```

#### to_json_pretty()

Serialize to pretty JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```rust
pub fn to_json_pretty(&self) -> String
```

---

#### QueryMutationConfig

Configuration for schemas with Query and Mutation types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `true` | Enable introspection queries |
| `complexity_limit` | `Option<usize>` | `None` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `Option<usize>` | `None` | Maximum query depth (None = unlimited) |

### Methods

#### default()

**Signature:**

```rust
pub fn default() -> QueryMutationConfig
```

---

#### QueryOnlyConfig

Configuration for schemas with only Query type

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `true` | Enable introspection queries |
| `complexity_limit` | `Option<usize>` | `None` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `Option<usize>` | `None` | Maximum query depth (None = unlimited) |

### Methods

#### default()

**Signature:**

```rust
pub fn default() -> QueryOnlyConfig
```

---

#### RateLimitConfig

Rate limiting configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `per_second` | `u64` | `100` | Requests per second |
| `burst` | `u32` | `200` | Burst allowance |
| `ip_based` | `bool` | `true` | Use IP-based rate limiting |

### Methods

#### default()

**Signature:**

```rust
pub fn default() -> RateLimitConfig
```

---

#### Request

---

#### Response

HTTP Response with custom status code, headers, and content

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `Option<serde_json::Value>` | `Default::default()` | Response body content |
| `status_code` | `u16` | — | HTTP status code (defaults to 200) |
| `headers` | `HashMap<String, String>` | `HashMap::new()` | Response headers |

### Methods

#### set_header()

Set a header

**Signature:**

```rust
pub fn set_header(&self, key: &str, value: &str)
```

#### set_cookie()

Set a cookie in the response

**Signature:**

```rust
pub fn set_cookie(&self, key: &str, value: &str, secure: bool, http_only: bool, max_age: Option<i64>, domain: Option<String>, path: Option<String>, same_site: Option<String>)
```

#### default()

**Signature:**

```rust
pub fn default() -> Response
```

---

#### ResponseSnapshot

Snapshot of an Axum response used by higher-level language bindings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `status` | `u16` | — | HTTP status code. |
| `headers` | `HashMap<String, String>` | — | Response headers (lowercase keys for predictable lookups). |
| `body` | `Vec<u8>` | — | Response body bytes (decoded for supported encodings). |

### Methods

#### text()

Return response body as UTF-8 string.

**Signature:**

```rust
pub fn text(&self) -> String
```

#### header()

Lookup header by case-insensitive name.

**Signature:**

```rust
pub fn header(&self, name: &str) -> Option<String>
```

---

#### RouteBuilder

Builder for defining a route.

### Methods

#### new()

Create a new builder for the provided HTTP method and path.

**Signature:**

```rust
pub fn new(method: Method, path: &str) -> RouteBuilder
```

#### handler_name()

Assign an explicit handler name.

**Signature:**

```rust
pub fn handler_name(&self, name: &str) -> RouteBuilder
```

#### request_schema_json()

Provide a raw JSON schema for the request body.

**Signature:**

```rust
pub fn request_schema_json(&self, schema: serde_json::Value) -> RouteBuilder
```

#### response_schema_json()

Provide a raw JSON schema for the response body.

**Signature:**

```rust
pub fn response_schema_json(&self, schema: serde_json::Value) -> RouteBuilder
```

#### params_schema_json()

Provide a raw JSON schema for request parameters.

**Signature:**

```rust
pub fn params_schema_json(&self, schema: serde_json::Value) -> RouteBuilder
```

#### file_params_json()

Provide multipart file parameter configuration.

**Signature:**

```rust
pub fn file_params_json(&self, schema: serde_json::Value) -> RouteBuilder
```

#### cors()

Attach a CORS configuration for this route.

**Signature:**

```rust
pub fn cors(&self, cors: CorsConfig) -> RouteBuilder
```

#### compression()

Attach a compression configuration for this route.

**Signature:**

```rust
pub fn compression(&self, compression: CompressionConfig) -> RouteBuilder
```

#### sync()

Mark the route as synchronous.

**Signature:**

```rust
pub fn sync(&self) -> RouteBuilder
```

#### handler_dependencies()

Declare the dependency keys that must be resolved before this handler runs.

**Signature:**

```rust
pub fn handler_dependencies(&self, dependencies: Vec<String>) -> RouteBuilder
```

---

#### SchemaConfig

Configuration for GraphQL schema building.

Encapsulates all schema-level configuration options including
introspection control, complexity limits, and depth limits.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `true` | Enable introspection queries |
| `complexity_limit` | `Option<usize>` | `None` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `Option<usize>` | `None` | Maximum query depth (None = unlimited) |

### Methods

#### default()

**Signature:**

```rust
pub fn default() -> SchemaConfig
```

---

#### ServerConfig

Server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `host` | `String` | `"127.0.0.1"` | Host to bind to |
| `port` | `u16` | `8000` | Port to bind to |
| `workers` | `usize` | `1` | Number of Tokio runtime worker threads used by binding-managed server runtimes |
| `enable_request_id` | `bool` | `false` | Enable request ID generation and propagation |
| `max_body_size` | `Option<usize>` | `Default::default()` | Maximum request body size in bytes (None = unlimited, not recommended) |
| `request_timeout` | `Option<u64>` | `None` | Request timeout in seconds (None = no timeout) |
| `compression` | `Option<CompressionConfig>` | `None` | Enable compression middleware |
| `rate_limit` | `Option<RateLimitConfig>` | `None` | Enable rate limiting |
| `jwt_auth` | `Option<JwtConfig>` | `None` | JWT authentication configuration |
| `api_key_auth` | `Option<ApiKeyConfig>` | `None` | API Key authentication configuration |
| `static_files` | `Vec<StaticFilesConfig>` | `vec![]` | Static file serving configuration |
| `graceful_shutdown` | `bool` | `true` | Enable graceful shutdown on SIGTERM/SIGINT |
| `shutdown_timeout` | `u64` | `30` | Graceful shutdown timeout (seconds) |
| `asyncapi` | `Option<AsyncApiConfig>` | `None` | AsyncAPI HTTP endpoint configuration |
| `openapi` | `Option<OpenApiConfig>` | `None` | OpenAPI documentation configuration |
| `jsonrpc` | `Option<JsonRpcConfig>` | `None` | JSON-RPC configuration |
| `grpc` | `Option<GrpcConfig>` | `None` | gRPC configuration |
| `lifecycle_hooks` | `Option<String>` | `None` | Lifecycle hooks for request/response processing |
| `background_tasks` | `BackgroundTaskConfig` | — | Background task executor configuration |
| `enable_http_trace` | `bool` | `false` | Enable per-request HTTP tracing (tower-http `TraceLayer`) |
| `di_container` | `Option<String>` | `None` | Dependency injection container (requires 'di' feature) |

### Methods

#### default()

**Signature:**

```rust
pub fn default() -> ServerConfig
```

---

#### ServerInfo

Server information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | Base URL of the server (e.g. `"<https://api.example.com/v1"`>). |
| `description` | `Option<String>` | `None` | Optional human-readable description of the server environment. |

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
| `event_type` | `Option<String>` | `None` | Event type (optional) |
| `data` | `serde_json::Value` | — | Event data (JSON value) |
| `id` | `Option<String>` | `None` | Event ID (optional, for client-side reconnection) |
| `retry` | `Option<u64>` | `None` | Retry timeout in milliseconds (optional) |

### Methods

#### with_id()

Set the event ID for client-side reconnection support

Sets an ID that clients can use to resume from this point if they disconnect.
The client sends this ID back in the `Last-Event-ID` header when reconnecting.

**Signature:**

```rust
pub fn with_id(&self, id: &str) -> SseEvent
```

#### with_retry()

Set the retry timeout for client reconnection

Sets the time in milliseconds clients should wait before attempting to reconnect
if the connection is lost. The client browser will automatically handle reconnection.

**Signature:**

```rust
pub fn with_retry(&self, retry_ms: u64) -> SseEvent
```

---

#### StaticFilesConfig

Static file serving configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `directory` | `String` | — | Directory path to serve |
| `route_prefix` | `String` | — | URL path prefix (e.g., "/static") |
| `index_file` | `bool` | `/* serde(default) */` | Fallback to index.html for directories |
| `cache_control` | `Option<String>` | `None` | Cache-Control header value |

---

#### TestClient

Core test client for making HTTP requests to a Spikard application.

This struct wraps axum-test's TestServer and provides a language-agnostic
interface for making HTTP requests, sending WebSocket connections, and
handling Server-Sent Events. Language bindings wrap this to provide
native API surfaces.

### Methods

#### graphql_at()

Send a GraphQL query/mutation to a custom endpoint

**Signature:**

```rust
pub fn graphql_at(&self, endpoint: &str, query: &str, variables: Option<serde_json::Value>, operation_name: Option<String>) -> ResponseSnapshot
```

#### graphql()

Send a GraphQL query/mutation

**Signature:**

```rust
pub fn graphql(&self, query: &str, variables: Option<serde_json::Value>, operation_name: Option<String>) -> ResponseSnapshot
```

#### graphql_subscription_at()

Send a GraphQL subscription (WebSocket) to a custom endpoint.

Uses the `graphql-transport-ws` protocol and captures the first `next` payload.
After the first payload is received, this client sends `complete` to unsubscribe.

**Signature:**

```rust
pub fn graphql_subscription_at(&self, endpoint: &str, query: &str, variables: Option<serde_json::Value>, operation_name: Option<String>) -> GraphQlSubscriptionSnapshot
```

#### graphql_subscription()

Send a GraphQL subscription (WebSocket).

Uses `/graphql` as the default subscription endpoint.

**Signature:**

```rust
pub fn graphql_subscription(&self, query: &str, variables: Option<serde_json::Value>, operation_name: Option<String>) -> GraphQlSubscriptionSnapshot
```

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
| `content_type` | `Option<String>` | `None` | MIME type of the uploaded file |
| `size` | `Option<usize>` | `None` | Size of the file in bytes |
| `content` | `Vec<u8>` | — | File content (may be base64 encoded) |
| `content_encoding` | `Option<String>` | `None` | Content encoding type |
| `cursor` | `String` | — | Internal cursor for Read/Seek operations |

### Methods

#### as_bytes()

Get the raw file content as bytes.

This provides zero-copy access to the underlying buffer.

**Signature:**

```rust
pub fn as_bytes(&self) -> Vec<u8>
```

#### read_to_string()

Read the file content as a UTF-8 string.

**Errors:**

Returns an error if the content is not valid UTF-8.

**Signature:**

```rust
pub fn read_to_string(&self) -> String
```

#### content_type_or_default()

Get the content type, defaulting to "application/octet-stream".

**Signature:**

```rust
pub fn content_type_or_default(&self) -> String
```

---

#### ValidateRequest

Request body for `POST /asyncapi/validate`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `spec` | `serde_json::Value` | — | Spec |
| `channel` | `String` | — | Channel |
| `message` | `String` | — | Message |
| `payload` | `serde_json::Value` | — | Payload |

---

#### ValidationResponse

Response body for `POST /asyncapi/validate`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `valid` | `bool` | — | Valid |
| `errors` | `Vec<String>` | — | Errors |

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
| `Http` | Http — Fields: `scheme`: `String`, `bearer_format`: `String` |
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
| `Binary` | A binary message. — Fields: `0`: `Vec<u8>` |
| `Close` | A close message with a numeric close code (RFC 6455) and optional reason text. Common codes: 1000 Normal Closure, 1001 Going Away, 1005 No Status Received, 1006 Abnormal Closure. — Fields: `code`: `u16`, `reason`: `String` |
| `Ping` | A ping message. — Fields: `0`: `Vec<u8>` |
| `Pong` | A pong message. — Fields: `0`: `Vec<u8>` |

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

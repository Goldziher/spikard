---
title: "Elixir API Reference"
---

## Elixir API Reference <span class="version-badge">v0.15.6-rc.17</span>

### Functions

#### schema_query_only()

Create a simple schema configuration with only Query type.

This is a convenience function for schemas that only have queries.

**Returns:**

A `QueryOnlyConfig` with default settings

**Signature:**

```elixir
@spec schema_query_only() :: {:ok, term()} | {:error, term()}
def schema_query_only()
```

**Returns:** `QueryOnlyConfig`

---

#### schema_query_mutation()

Create a schema configuration with Query and Mutation types.

This is a convenience function for schemas with queries and mutations but no subscriptions.

**Returns:**

A `QueryMutationConfig` with default settings

**Signature:**

```elixir
@spec schema_query_mutation() :: {:ok, term()} | {:error, term()}
def schema_query_mutation()
```

**Returns:** `QueryMutationConfig`

---

#### schema_full()

Create a schema configuration with all three root types.

This is a convenience function for fully-featured schemas.

**Returns:**

A `FullSchemaConfig` with default settings

**Signature:**

```elixir
@spec schema_full() :: {:ok, term()} | {:error, term()}
def schema_full()
```

**Returns:** `FullSchemaConfig`

---

### Types

#### ApiKeyConfig

API Key authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `keys` | `list(String.t())` | — | Valid API keys |
| `header_name` | `String.t()` | `/* serde(default) */` | Header name to check (e.g., "X-API-Key") |

---

#### App

Spikard application builder.

### Functions

#### new()

Create a new application with the default server configuration.

**Signature:**

```elixir
def new()
```

#### merge_axum_router()

Attach an existing Axum router to this application, returning ownership.

**Signature:**

```elixir
def merge_axum_router(router)
```

#### attach_axum_router()

Attach an Axum router using a mutable reference for incremental configuration.

**Signature:**

```elixir
def attach_axum_router(router)
```

#### into_router()

Build the underlying Axum router.

**Errors:**

Returns an error if server or router construction fails.

**Signature:**

```elixir
def into_router()
```

#### into_router_and_config()

Decompose the application into its Axum router and server configuration.

This is the low-level escape hatch used by the C FFI layer to start the
server on a background thread while retaining the bind address from the
caller-supplied `ServerConfig`. Prefer `App.run` for normal use.

**Errors:**

Returns an error if router construction fails.

**Signature:**

```elixir
def into_router_and_config()
```

#### default()

**Signature:**

```elixir
def default()
```

---

#### AsyncApiConfig

AsyncAPI HTTP endpoint configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `boolean()` | — | Enable AsyncAPI endpoints (default: false) |
| `spec` | `term() \| nil` | `nil` | Pre-registered AsyncAPI spec to serve from GET /asyncapi.json |

---

#### BackgroundJobMetadata

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String.t()` | — | The name |
| `request_id` | `String.t() \| nil` | `nil` | Request id |

### Functions

#### default()

**Signature:**

```elixir
def default()
```

---

#### BackgroundTaskConfig

Configuration for in-process background task execution.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_queue_size` | `integer()` | `1024` | Maximum queue size |
| `max_concurrent_tasks` | `integer()` | `128` | Maximum concurrent tasks |
| `drain_timeout_secs` | `integer()` | `30` | Drain timeout secs |

### Functions

#### default()

**Signature:**

```elixir
def default()
```

---

#### CompressionConfig

Compression configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `gzip` | `boolean()` | `true` | Enable gzip compression |
| `brotli` | `boolean()` | `true` | Enable brotli compression |
| `min_size` | `integer()` | — | Minimum response size to compress (bytes) |
| `quality` | `integer()` | — | Compression quality (0-11 for brotli, 0-9 for gzip) |

### Functions

#### default()

**Signature:**

```elixir
def default()
```

---

#### ContactInfo

Contact information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String.t() \| nil` | `nil` | Name of the contact person or organisation. |
| `email` | `String.t() \| nil` | `nil` | Contact email address. |
| `url` | `String.t() \| nil` | `nil` | URL pointing to the contact information page. |

---

#### CorsConfig

CORS configuration for a route

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `allowed_origins` | `list(String.t())` | `[]` | Allowed origins |
| `allowed_methods` | `list(String.t())` | `[]` | Allowed methods |
| `allowed_headers` | `list(String.t())` | `[]` | Allowed headers |
| `expose_headers` | `list(String.t()) \| nil` | `nil` | Expose headers |
| `max_age` | `integer() \| nil` | `nil` | Maximum age |
| `allow_credentials` | `boolean() \| nil` | `nil` | Allow credentials |
| `methods_joined_cache` | `String.t()` | — | Methods joined cache |
| `headers_joined_cache` | `String.t()` | — | Headers joined cache |

### Functions

#### allowed_methods_joined()

Get the cached joined methods string for preflight responses

**Signature:**

```elixir
def allowed_methods_joined()
```

#### allowed_headers_joined()

Get the cached joined headers string for preflight responses

**Signature:**

```elixir
def allowed_headers_joined()
```

#### is_origin_allowed()

Check if an origin is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```elixir
def is_origin_allowed(origin)
```

#### is_method_allowed()

Check if a method is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```elixir
def is_method_allowed(method)
```

#### default()

**Signature:**

```elixir
def default()
```

---

#### FullSchemaConfig

Configuration for fully-featured schemas with Query, Mutation, and Subscription types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `boolean()` | `true` | Enable introspection queries |
| `complexity_limit` | `integer() \| nil` | `nil` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `integer() \| nil` | `nil` | Maximum query depth (None = unlimited) |

### Functions

#### default()

**Signature:**

```elixir
def default()
```

---

#### GraphQlRouteConfig

Configuration for GraphQL routes

Provides a builder pattern for configuring GraphQL route parameters
for the Spikard HTTP server's routing system.

### Functions

#### new()

Create a new GraphQL route configuration with defaults

Default values:

- path: "/graphql"
- method: "POST"
- `enable_playground`: false

**Signature:**

```elixir
def new()
```

#### path()

Set the HTTP path for the GraphQL endpoint

**Signature:**

```elixir
def path(path)
```

#### method()

Set the HTTP method for the GraphQL endpoint

**Signature:**

```elixir
def method(method)
```

#### enable_playground()

Enable or disable the GraphQL Playground UI

**Signature:**

```elixir
def enable_playground(enable)
```

#### description()

Set a custom description for documentation

**Signature:**

```elixir
def description(description)
```

#### get_path()

Get the configured path

**Signature:**

```elixir
def get_path()
```

#### get_method()

Get the configured method

**Signature:**

```elixir
def get_method()
```

#### is_playground_enabled()

Check if playground is enabled

**Signature:**

```elixir
def is_playground_enabled()
```

#### get_description()

Get the description if set

**Signature:**

```elixir
def get_description()
```

#### default()

**Signature:**

```elixir
def default()
```

---

#### GraphQlSubscriptionSnapshot

Snapshot of a GraphQL subscription exchange over WebSocket.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `operation_id` | `String.t()` | — | Operation id used for the subscription request. |
| `acknowledged` | `boolean()` | — | Whether the server acknowledged the GraphQL WebSocket connection. |
| `event` | `term() \| nil` | `nil` | First `next.payload` received for this subscription, if any. |
| `errors` | `list(term())` | — | GraphQL protocol errors emitted by the server. |
| `complete_received` | `boolean()` | — | Whether a `complete` frame was observed for this operation. |

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
| `enabled` | `boolean()` | `true` | Enable gRPC support |
| `max_message_size` | `integer()` | — | Maximum message size in bytes (for both sending and receiving) This limit applies to individual messages in both unary and streaming RPCs. When a single message exceeds this size, the request is rejected with HTTP 413 (Payload Too Large). Default: 4MB (4194304 bytes) **Note:** This limit does NOT apply to the total response size in streaming RPCs. For multi-message streams, the total response can exceed this limit as long as each individual message stays within the limit. |
| `enable_compression` | `boolean()` | `true` | Enable gzip compression for gRPC messages |
| `request_timeout` | `integer() \| nil` | `nil` | Timeout for gRPC requests in seconds (None = no timeout) |
| `max_concurrent_streams` | `integer()` | — | Maximum number of concurrent streams per connection (HTTP/2 advisory) This value is communicated to HTTP/2 clients as the server's flow control limit. The HTTP/2 transport layer enforces this limit automatically via SETTINGS frames and GOAWAY responses. Applications should NOT implement custom enforcement. Default: 100 streams per connection # Stream Limiting Strategy - **Per Connection**: This limit applies per HTTP/2 connection, not globally - **Transport Enforcement**: HTTP/2 handles all stream limiting; applications need not implement custom checks - **Streaming Requests**: In server streaming or bidi streaming, each logical RPC consumes one stream slot. Message ordering within a stream follows HTTP/2 frame ordering. |
| `enable_keepalive` | `boolean()` | `true` | Enable HTTP/2 keepalive |
| `keepalive_interval` | `integer()` | — | HTTP/2 keepalive interval in seconds |
| `keepalive_timeout` | `integer()` | — | HTTP/2 keepalive timeout in seconds |
| `max_stream_response_bytes` | `integer() \| nil` | `nil` | Total byte cap across an entire streaming response. When `Some(n)`, the streaming adapter aborts the stream with `tonic.Status.resource_exhausted` once the cumulative encoded message bytes exceed `n`. The stream yields the error item and then terminates. Per-message cap remains `max_message_size`. This limit applies to server-streaming and bidirectional-streaming RPCs only; unary RPCs are governed solely by `max_message_size`. Default: `nil` (unbounded total response size). |

### Functions

#### default()

**Signature:**

```elixir
def default()
```

---

#### Handler

Handler trait that all language bindings must implement

This trait is completely language-agnostic. Each binding (Python, Node, WASM)
implements this trait to bridge their runtime to our HTTP server.

### Functions

#### call()

Handle an HTTP request

Takes the extracted request data and returns a future that resolves to either:

- Ok(Response): A successful HTTP response
- Err((StatusCode, String)): An error with status code and message

**Signature:**

```elixir
def call(request, request_data)
```

#### prefers_raw_json_body()

Whether this handler prefers consuming `RequestData.raw_body` over the parsed
`RequestData.body` for JSON requests.

When `true`, the server may skip eager JSON parsing when there is no request-body
schema validator attached to the route.

**Signature:**

```elixir
def prefers_raw_json_body()
```

#### prefers_parameter_extraction()

Whether this handler wants to perform its own parameter validation/extraction (path/query/header/cookie).

When `true`, the server will skip `ParameterValidator.validate_and_extract` in `ValidatingHandler`.
This is useful for language bindings which need to transform validated parameters into
language-specific values (e.g., Python kwargs) without duplicating work. When `false`,
the server stores validated output in `RequestData.validated_params`.

**Signature:**

```elixir
def prefers_parameter_extraction()
```

#### wants_headers()

Whether this handler needs the parsed headers map in `RequestData`.

When `false`, the server may skip building `RequestData.headers` for requests without a body.
(Requests with bodies still typically need `Content-Type` decisions.)

**Signature:**

```elixir
def wants_headers()
```

#### wants_cookies()

Whether this handler needs the parsed cookies map in `RequestData`.

When `false`, the server may skip parsing cookies for requests without a body.

**Signature:**

```elixir
def wants_cookies()
```

#### wants_request_extensions()

Whether this handler needs `RequestData` stored in request extensions.

When `false`, the server avoids inserting `RequestData` into extensions to
skip cloning in hot paths.

**Signature:**

```elixir
def wants_request_extensions()
```

#### static_response()

Return a pre-built static response if this handler always produces the
same output. When `Some`, the server bypasses the full middleware
pipeline and serves the pre-built response directly.

**Signature:**

```elixir
def static_response()
```

---

#### IntoHandler

Convert user-facing handler functions into the low-level `Handler` trait.

### Functions

#### into_handler()

Convert this value into a shared request handler.

**Signature:**

```elixir
def into_handler()
```

---

#### JsonRpcConfig

JSON-RPC server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `boolean()` | `true` | Enable JSON-RPC endpoint |
| `endpoint_path` | `String.t()` | — | HTTP endpoint path for JSON-RPC requests (default: "/rpc") |
| `enable_batch` | `boolean()` | — | Enable batch request processing (default: true) |
| `max_batch_size` | `integer()` | — | Maximum number of requests in a batch (default: 100) |

### Functions

#### default()

**Signature:**

```elixir
def default()
```

---

#### JsonRpcMethodInfo

JSON-RPC method metadata for routes that support JSON-RPC

This struct captures the metadata needed to expose HTTP routes as JSON-RPC methods,
enabling discovery and documentation of RPC-compatible endpoints.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `method_name` | `String.t()` | — | The JSON-RPC method name (e.g., "user.create") |
| `description` | `String.t() \| nil` | `nil` | Optional description of what the method does |
| `params_schema` | `term() \| nil` | `nil` | Optional JSON Schema for method parameters |
| `result_schema` | `term() \| nil` | `nil` | Optional JSON Schema for the result |
| `deprecated` | `boolean()` | `/* serde(default) */` | Whether this method is deprecated |
| `tags` | `list(String.t())` | `/* serde(default) */` | Tags for categorizing and grouping methods |

---

#### JwtConfig

JWT authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `secret` | `String.t()` | — | Secret key for JWT verification |
| `algorithm` | `String.t()` | `/* serde(default) */` | Required algorithm (HS256, HS384, HS512, RS256, etc.) |
| `audience` | `list(String.t()) \| nil` | `nil` | Required audience claim |
| `issuer` | `String.t() \| nil` | `nil` | Required issuer claim |
| `leeway` | `integer()` | `/* serde(default) */` | Leeway for expiration checks (seconds) |

---

#### LicenseInfo

License information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String.t()` | — | SPDX license identifier or display name (e.g. `"MIT"`). |
| `url` | `String.t() \| nil` | `nil` | URL to the full license text. |

---

#### OpenApiConfig

OpenAPI configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `boolean()` | `false` | Enable OpenAPI generation (default: false for zero overhead) |
| `title` | `String.t()` | `"API"` | API title |
| `version` | `String.t()` | `"1.0.0"` | API version |
| `description` | `String.t() \| nil` | `nil` | API description (supports markdown) |
| `swagger_ui_path` | `String.t()` | — | Path to serve Swagger UI (default: "/docs") |
| `redoc_path` | `String.t()` | — | Path to serve Redoc (default: "/redoc") |
| `openapi_json_path` | `String.t()` | — | Path to serve OpenAPI JSON spec (default: "/openapi.json") |
| `contact` | `ContactInfo \| nil` | `nil` | Contact information |
| `license` | `LicenseInfo \| nil` | `nil` | License information |
| `servers` | `list(ServerInfo)` | `[]` | Server definitions |
| `security_schemes` | `map()` | `%{}` | Security schemes (auto-detected from middleware if not provided) |

### Functions

#### default()

**Signature:**

```elixir
def default()
```

---

#### ParseRequest

Request body for `POST /asyncapi/parse`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `spec` | `term()` | — | Spec |

---

#### ParseResult

Full parse result returned by `POST /asyncapi/parse`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `spec_version` | `String.t()` | — | Spec version |
| `title` | `String.t()` | — | Title |
| `api_version` | `String.t()` | — | Api version |
| `channels` | `list(ParsedChannel)` | — | Channels |
| `operations` | `list(ParsedOperation)` | — | Operations |
| `messages` | `list(ParsedMessage)` | — | Messages |

---

#### ParsedChannel

A single channel extracted from an AsyncAPI spec

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String.t()` | — | Channel key from the spec (e.g. "chat/messages") |
| `address` | `String.t()` | — | Channel address / path |
| `messages` | `list(String.t())` | — | Message names declared on this channel |
| `bindings` | `term() \| nil` | `nil` | Bindings (ws / http / amqp / …) as raw JSON for forward-compatibility |

---

#### ParsedMessage

A resolved message (name + JSON Schema)

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String.t()` | — | Message name |
| `schema` | `term() \| nil` | `nil` | Resolved JSON Schema for the message payload, if available |

---

#### ParsedOperation

A single operation extracted from an AsyncAPI spec

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String.t()` | — | Operation name |
| `action` | `String.t()` | — | Operation action: "send" or "receive" |
| `channel` | `String.t()` | — | Channel reference (resolved to the channel name) |

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
| `type_uri` | `String.t()` | — | A URI reference that identifies the problem type. Defaults to "about:blank" when absent. Should be a stable, human-readable identifier for the problem type. |
| `title` | `String.t()` | — | A short, human-readable summary of the problem type. Should not change from occurrence to occurrence of the problem. |
| `status` | `integer()` | — | The HTTP status code generated by the origin server. This is advisory; the actual HTTP status code takes precedence. |
| `detail` | `String.t() \| nil` | `nil` | A human-readable explanation specific to this occurrence of the problem. |
| `instance` | `String.t() \| nil` | `nil` | A URI reference that identifies the specific occurrence of the problem. It may or may not yield further information if dereferenced. |
| `extensions` | `map()` | — | Extension members - problem-type-specific data. For validation errors, this typically contains an "errors" array. |

### Functions

#### with_detail()

Set the detail field

**Signature:**

```elixir
def with_detail(detail)
```

#### with_instance()

Set the instance field

**Signature:**

```elixir
def with_instance(instance)
```

#### not_found()

Create a not found error

**Signature:**

```elixir
def not_found(detail)
```

#### method_not_allowed()

Create a method not allowed error

**Signature:**

```elixir
def method_not_allowed(detail)
```

#### internal_server_error()

Create an internal server error

**Signature:**

```elixir
def internal_server_error(detail)
```

#### bad_request()

Create a bad request error

**Signature:**

```elixir
def bad_request(detail)
```

#### to_json()

Serialize to JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```elixir
def to_json()
```

#### to_json_pretty()

Serialize to pretty JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```elixir
def to_json_pretty()
```

---

#### QueryMutationConfig

Configuration for schemas with Query and Mutation types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `boolean()` | `true` | Enable introspection queries |
| `complexity_limit` | `integer() \| nil` | `nil` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `integer() \| nil` | `nil` | Maximum query depth (None = unlimited) |

### Functions

#### default()

**Signature:**

```elixir
def default()
```

---

#### QueryOnlyConfig

Configuration for schemas with only Query type

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `boolean()` | `true` | Enable introspection queries |
| `complexity_limit` | `integer() \| nil` | `nil` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `integer() \| nil` | `nil` | Maximum query depth (None = unlimited) |

### Functions

#### default()

**Signature:**

```elixir
def default()
```

---

#### RateLimitConfig

Rate limiting configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `per_second` | `integer()` | `100` | Requests per second |
| `burst` | `integer()` | `200` | Burst allowance |
| `ip_based` | `boolean()` | `true` | Use IP-based rate limiting |

### Functions

#### default()

**Signature:**

```elixir
def default()
```

---

#### Response

HTTP Response with custom status code, headers, and content

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `term() \| nil` | `nil` | Response body content |
| `status_code` | `integer()` | — | HTTP status code (defaults to 200) |
| `headers` | `map()` | `%{}` | Response headers |

### Functions

#### set_header()

Set a header

**Signature:**

```elixir
def set_header(key, value)
```

#### set_cookie()

Set a cookie in the response

**Signature:**

```elixir
def set_cookie(key, value, secure, http_only, max_age, domain, path, same_site)
```

#### default()

**Signature:**

```elixir
def default()
```

---

#### ResponseSnapshot

Snapshot of an Axum response used by higher-level language bindings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `status` | `integer()` | — | HTTP status code. |
| `headers` | `map()` | — | Response headers (lowercase keys for predictable lookups). |
| `body` | `binary()` | — | Response body bytes (decoded for supported encodings). |

### Functions

#### text()

Return response body as UTF-8 string.

**Signature:**

```elixir
def text()
```

#### header()

Lookup header by case-insensitive name.

**Signature:**

```elixir
def header(name)
```

---

#### RouteBuilder

Builder for defining a route.

### Functions

#### new()

Create a new builder for the provided HTTP method and path.

**Signature:**

```elixir
def new(method, path)
```

#### handler_name()

Assign an explicit handler name.

**Signature:**

```elixir
def handler_name(name)
```

#### request_schema_json()

Provide a raw JSON schema for the request body.

**Signature:**

```elixir
def request_schema_json(schema)
```

#### response_schema_json()

Provide a raw JSON schema for the response body.

**Signature:**

```elixir
def response_schema_json(schema)
```

#### params_schema_json()

Provide a raw JSON schema for request parameters.

**Signature:**

```elixir
def params_schema_json(schema)
```

#### file_params_json()

Provide multipart file parameter configuration.

**Signature:**

```elixir
def file_params_json(schema)
```

#### cors()

Attach a CORS configuration for this route.

**Signature:**

```elixir
def cors(cors)
```

#### compression()

Attach a compression configuration for this route.

**Signature:**

```elixir
def compression(compression)
```

#### sync()

Mark the route as synchronous.

**Signature:**

```elixir
def sync()
```

#### handler_dependencies()

Declare the dependency keys that must be resolved before this handler runs.

**Signature:**

```elixir
def handler_dependencies(dependencies)
```

---

#### SchemaConfig

Configuration for GraphQL schema building.

Encapsulates all schema-level configuration options including
introspection control, complexity limits, and depth limits.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `boolean()` | `true` | Enable introspection queries |
| `complexity_limit` | `integer() \| nil` | `nil` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `integer() \| nil` | `nil` | Maximum query depth (None = unlimited) |

### Functions

#### default()

**Signature:**

```elixir
def default()
```

---

#### ServerConfig

Server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `host` | `String.t()` | `"127.0.0.1"` | Host to bind to |
| `port` | `integer()` | `8000` | Port to bind to |
| `workers` | `integer()` | `1` | Number of Tokio runtime worker threads used by binding-managed server runtimes |
| `enable_request_id` | `boolean()` | `false` | Enable request ID generation and propagation |
| `max_body_size` | `integer() \| nil` | `nil` | Maximum request body size in bytes (None = unlimited, not recommended) |
| `request_timeout` | `integer() \| nil` | `nil` | Request timeout in seconds (None = no timeout) |
| `compression` | `CompressionConfig \| nil` | `nil` | Enable compression middleware |
| `rate_limit` | `RateLimitConfig \| nil` | `nil` | Enable rate limiting |
| `jwt_auth` | `JwtConfig \| nil` | `nil` | JWT authentication configuration |
| `api_key_auth` | `ApiKeyConfig \| nil` | `nil` | API Key authentication configuration |
| `static_files` | `list(StaticFilesConfig)` | `[]` | Static file serving configuration |
| `graceful_shutdown` | `boolean()` | `true` | Enable graceful shutdown on SIGTERM/SIGINT |
| `shutdown_timeout` | `integer()` | `30` | Graceful shutdown timeout (seconds) |
| `asyncapi` | `AsyncApiConfig \| nil` | `nil` | AsyncAPI HTTP endpoint configuration |
| `openapi` | `OpenApiConfig \| nil` | `nil` | OpenAPI documentation configuration |
| `jsonrpc` | `JsonRpcConfig \| nil` | `nil` | JSON-RPC configuration |
| `grpc` | `GrpcConfig \| nil` | `nil` | gRPC configuration |
| `lifecycle_hooks` | `String.t() \| nil` | `nil` | Lifecycle hooks for request/response processing |
| `background_tasks` | `BackgroundTaskConfig` | — | Background task executor configuration |
| `enable_http_trace` | `boolean()` | `false` | Enable per-request HTTP tracing (tower-http `TraceLayer`) |
| `di_container` | `String.t() \| nil` | `nil` | Dependency injection container (requires 'di' feature) |

### Functions

#### default()

**Signature:**

```elixir
def default()
```

---

#### ServerInfo

Server information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String.t()` | — | Base URL of the server (e.g. `"<https://api.example.com/v1"`>). |
| `description` | `String.t() \| nil` | `nil` | Optional human-readable description of the server environment. |

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
| `event_type` | `String.t() \| nil` | `nil` | Event type (optional) |
| `data` | `term()` | — | Event data (JSON value) |
| `id` | `String.t() \| nil` | `nil` | Event ID (optional, for client-side reconnection) |
| `retry` | `integer() \| nil` | `nil` | Retry timeout in milliseconds (optional) |

### Functions

#### with_id()

Set the event ID for client-side reconnection support

Sets an ID that clients can use to resume from this point if they disconnect.
The client sends this ID back in the `Last-Event-ID` header when reconnecting.

**Signature:**

```elixir
def with_id(id)
```

#### with_retry()

Set the retry timeout for client reconnection

Sets the time in milliseconds clients should wait before attempting to reconnect
if the connection is lost. The client browser will automatically handle reconnection.

**Signature:**

```elixir
def with_retry(retry_ms)
```

---

#### StaticFilesConfig

Static file serving configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `directory` | `String.t()` | — | Directory path to serve |
| `route_prefix` | `String.t()` | — | URL path prefix (e.g., "/static") |
| `index_file` | `boolean()` | `/* serde(default) */` | Fallback to index.html for directories |
| `cache_control` | `String.t() \| nil` | `nil` | Cache-Control header value |

---

#### TestingSseEvent

A single Server-Sent Event.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data` | `String.t()` | — | The data field of the event. |

---

#### ValidateRequest

Request body for `POST /asyncapi/validate`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `spec` | `term()` | — | Spec |
| `channel` | `String.t()` | — | Channel |
| `message` | `String.t()` | — | Message |
| `payload` | `term()` | — | Payload |

---

#### ValidationResponse

Response body for `POST /asyncapi/validate`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `valid` | `boolean()` | — | Valid |
| `errors` | `list(String.t())` | — | Errors |

---

### Enums

#### Method

HTTP method

| Value | Description |
|-------|-------------|
| `get` | Get |
| `post` | Post |
| `put` | Put |
| `patch` | Patch |
| `delete` | Delete |
| `head` | Head |
| `options` | Options |
| `connect` | Connect |
| `trace` | Trace |

---

#### SecuritySchemeInfo

Security scheme types

| Value | Description |
|-------|-------------|
| `http` | Http — Fields: `scheme`: `String.t()`, `bearer_format`: `String.t()` |
| `api_key` | Api key — Fields: `location`: `String.t()`, `name`: `String.t()` |

---

#### SnapshotError

Possible errors while converting an Axum response into a snapshot.

| Value | Description |
|-------|-------------|
| `invalid_header` | Response header could not be decoded to UTF-8. — Fields: `0`: `String.t()` |
| `decompression` | Body decompression failed. — Fields: `0`: `String.t()` |

---

#### WebSocketMessage

A WebSocket message that can be text or binary.

| Value | Description |
|-------|-------------|
| `text` | A text message. — Fields: `0`: `String.t()` |
| `binary` | A binary message. — Fields: `0`: `binary()` |
| `close` | A close message with a numeric close code (RFC 6455) and optional reason text. Common codes: 1000 Normal Closure, 1001 Going Away, 1005 No Status Received, 1006 Abnormal Closure. — Fields: `code`: `integer()`, `reason`: `String.t()` |
| `ping` | A ping message. — Fields: `0`: `binary()` |
| `pong` | A pong message. — Fields: `0`: `binary()` |

---

### Errors

#### AppError

Error type for application builder operations.

| Variant | Description |
|---------|-------------|
| `route` | Route registration failed. |
| `server` | Server/router construction failed. |
| `decode` | Failed to extract DTO from the request context. |

---

#### GraphQlError

Errors that can occur during GraphQL operations

These errors are compatible with async-graphql error handling and can be
converted to structured HTTP responses matching the project's error fixtures.

| Variant | Description |
|---------|-------------|
| `execution_error` | Error during schema execution Occurs when the GraphQL executor encounters a runtime error during query execution. |
| `schema_build_error` | Error during schema building Occurs when schema construction fails due to invalid definitions or conflicts. |
| `request_handling_error` | Error during request handling Occurs when the HTTP request cannot be properly handled or parsed. |
| `serialization_error` | Serialization error Occurs during JSON serialization/deserialization of GraphQL values. |
| `json_error` | JSON parsing error Occurs when JSON input cannot be parsed. |
| `validation_error` | GraphQL validation error Occurs when a GraphQL query fails schema validation. |
| `parse_error` | GraphQL parse error Occurs when the GraphQL query string cannot be parsed. |
| `authentication_error` | Authentication error Occurs when request authentication fails. |
| `authorization_error` | Authorization error Occurs when user lacks required permissions. |
| `not_found` | Not found error Occurs when a requested resource is not found. |
| `rate_limit_exceeded` | Rate limit error Occurs when rate limit is exceeded. |
| `invalid_input` | Invalid input error with validation details Occurs during input validation with detailed error information. |
| `complexity_limit_exceeded` | Query complexity limit exceeded Occurs when a GraphQL query exceeds the configured complexity limit. |
| `depth_limit_exceeded` | Query depth limit exceeded Occurs when a GraphQL query exceeds the configured depth limit. |
| `internal_error` | Internal server error Occurs when an unexpected internal error happens. |

---

#### SchemaError

Error type for schema building operations

| Variant | Description |
|---------|-------------|
| `building_failed` | Generic schema building error |
| `validation_error` | Configuration validation error |
| `complexity_limit_exceeded` | Complexity limit exceeded |
| `depth_limit_exceeded` | Depth limit exceeded |

---

---
title: "Python API Reference"
---

## Python API Reference <span class="version-badge">v0.15.6-rc.9</span>

### Functions

#### schema_query_only()

Create a simple schema configuration with only Query type.

This is a convenience function for schemas that only have queries.

**Returns:**

A `QueryOnlyConfig` with default settings

**Signature:**

```python
def schema_query_only() -> QueryOnlyConfig
```

**Returns:** `QueryOnlyConfig`

---

#### schema_query_mutation()

Create a schema configuration with Query and Mutation types.

This is a convenience function for schemas with queries and mutations but no subscriptions.

**Returns:**

A `QueryMutationConfig` with default settings

**Signature:**

```python
def schema_query_mutation() -> QueryMutationConfig
```

**Returns:** `QueryMutationConfig`

---

#### schema_full()

Create a schema configuration with all three root types.

This is a convenience function for fully-featured schemas.

**Returns:**

A `FullSchemaConfig` with default settings

**Signature:**

```python
def schema_full() -> FullSchemaConfig
```

**Returns:** `FullSchemaConfig`

---

### Types

#### ApiKeyConfig

API Key authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `keys` | `list[str]` | — | Valid API keys |
| `header_name` | `str` | `/* serde(default) */` | Header name to check (e.g., "X-API-Key") |

---

#### App

Spikard application builder.

### Methods

#### new()

Create a new application with the default server configuration.

**Signature:**

```python
@staticmethod
def new() -> App
```

#### config()

Set the server configuration.

**Signature:**

```python
def config(self, config: ServerConfig) -> App
```

#### merge_axum_router()

Attach an existing Axum router to this application, returning ownership.

**Signature:**

```python
def merge_axum_router(self, router: str) -> App
```

#### attach_axum_router()

Attach an Axum router using a mutable reference for incremental configuration.

**Signature:**

```python
def attach_axum_router(self, router: str) -> App
```

#### into_router()

Build the underlying Axum router.

**Errors:**

Returns an error if server or router construction fails.

**Signature:**

```python
def into_router(self) -> str
```

#### run()

Run the HTTP server using the configured routes.

**Errors:**

Returns an error if server construction or execution fails.

**Signature:**

```python
def run(self) -> None
```

#### default()

**Signature:**

```python
@staticmethod
def default() -> App
```

#### route()

Register a route using the provided builder and handler function.

**Errors:**

Returns an error if route construction fails or if the handler registration fails.

**Signature:**

```python
def route(self, builder: RouteBuilder, handler: H) -> App
```

---

#### AsyncApiConfig

AsyncAPI HTTP endpoint configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | — | Enable AsyncAPI endpoints (default: false) |
| `spec` | `dict[str, Any] \| None` | `None` | Pre-registered AsyncAPI spec to serve from GET /asyncapi.json |

---

#### BackgroundJobMetadata

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | The name |
| `request_id` | `str \| None` | `None` | Request id |

### Methods

#### default()

**Signature:**

```python
@staticmethod
def default() -> BackgroundJobMetadata
```

---

#### BackgroundTaskConfig

Configuration for in-process background task execution.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_queue_size` | `int` | `1024` | Maximum queue size |
| `max_concurrent_tasks` | `int` | `128` | Maximum concurrent tasks |
| `drain_timeout_secs` | `int` | `30` | Drain timeout secs |

### Methods

#### default()

**Signature:**

```python
@staticmethod
def default() -> BackgroundTaskConfig
```

---

#### CompressionConfig

Compression configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `gzip` | `bool` | `True` | Enable gzip compression |
| `brotli` | `bool` | `True` | Enable brotli compression |
| `min_size` | `int` | — | Minimum response size to compress (bytes) |
| `quality` | `int` | — | Compression quality (0-11 for brotli, 0-9 for gzip) |

### Methods

#### default()

**Signature:**

```python
@staticmethod
def default() -> CompressionConfig
```

---

#### ContactInfo

Contact information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str \| None` | `None` | Name of the contact person or organisation. |
| `email` | `str \| None` | `None` | Contact email address. |
| `url` | `str \| None` | `None` | URL pointing to the contact information page. |

---

#### CorsConfig

CORS configuration for a route

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `allowed_origins` | `list[str]` | `[]` | Allowed origins |
| `allowed_methods` | `list[str]` | `[]` | Allowed methods |
| `allowed_headers` | `list[str]` | `[]` | Allowed headers |
| `expose_headers` | `list[str] \| None` | `None` | Expose headers |
| `max_age` | `int \| None` | `None` | Maximum age |
| `allow_credentials` | `bool \| None` | `None` | Allow credentials |
| `methods_joined_cache` | `str` | — | Methods joined cache |
| `headers_joined_cache` | `str` | — | Headers joined cache |

### Methods

#### allowed_methods_joined()

Get the cached joined methods string for preflight responses

**Signature:**

```python
def allowed_methods_joined(self) -> str
```

#### allowed_headers_joined()

Get the cached joined headers string for preflight responses

**Signature:**

```python
def allowed_headers_joined(self) -> str
```

#### is_origin_allowed()

Check if an origin is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```python
def is_origin_allowed(self, origin: str) -> bool
```

#### is_method_allowed()

Check if a method is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```python
def is_method_allowed(self, method: str) -> bool
```

#### are_headers_allowed()

Check if all requested headers are allowed (O(n) where n = num requested headers)

**Signature:**

```python
def are_headers_allowed(self, requested: list[str]) -> bool
```

#### default()

**Signature:**

```python
@staticmethod
def default() -> CorsConfig
```

---

#### FullSchemaConfig

Configuration for fully-featured schemas with Query, Mutation, and Subscription types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `True` | Enable introspection queries |
| `complexity_limit` | `int \| None` | `None` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `int \| None` | `None` | Maximum query depth (None = unlimited) |

### Methods

#### default()

**Signature:**

```python
@staticmethod
def default() -> FullSchemaConfig
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

```python
@staticmethod
def new() -> GraphQlRouteConfig
```

#### path()

Set the HTTP path for the GraphQL endpoint

**Signature:**

```python
def path(self, path: str) -> GraphQlRouteConfig
```

#### method()

Set the HTTP method for the GraphQL endpoint

**Signature:**

```python
def method(self, method: str) -> GraphQlRouteConfig
```

#### enable_playground()

Enable or disable the GraphQL Playground UI

**Signature:**

```python
def enable_playground(self, enable: bool) -> GraphQlRouteConfig
```

#### description()

Set a custom description for documentation

**Signature:**

```python
def description(self, description: str) -> GraphQlRouteConfig
```

#### get_path()

Get the configured path

**Signature:**

```python
def get_path(self) -> str
```

#### get_method()

Get the configured method

**Signature:**

```python
def get_method(self) -> str
```

#### is_playground_enabled()

Check if playground is enabled

**Signature:**

```python
def is_playground_enabled(self) -> bool
```

#### get_description()

Get the description if set

**Signature:**

```python
def get_description(self) -> str | None
```

#### default()

**Signature:**

```python
@staticmethod
def default() -> GraphQlRouteConfig
```

---

#### GraphQlSubscriptionSnapshot

Snapshot of a GraphQL subscription exchange over WebSocket.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `operation_id` | `str` | — | Operation id used for the subscription request. |
| `acknowledged` | `bool` | — | Whether the server acknowledged the GraphQL WebSocket connection. |
| `event` | `dict[str, Any] \| None` | `None` | First `next.payload` received for this subscription, if any. |
| `errors` | `list[dict[str, Any]]` | — | GraphQL protocol errors emitted by the server. |
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
| `enabled` | `bool` | `True` | Enable gRPC support |
| `max_message_size` | `int` | — | Maximum message size in bytes (for both sending and receiving) This limit applies to individual messages in both unary and streaming RPCs. When a single message exceeds this size, the request is rejected with HTTP 413 (Payload Too Large). Default: 4MB (4194304 bytes) **Note:** This limit does NOT apply to the total response size in streaming RPCs. For multi-message streams, the total response can exceed this limit as long as each individual message stays within the limit. |
| `enable_compression` | `bool` | `True` | Enable gzip compression for gRPC messages |
| `request_timeout` | `int \| None` | `None` | Timeout for gRPC requests in seconds (None = no timeout) |
| `max_concurrent_streams` | `int` | — | Maximum number of concurrent streams per connection (HTTP/2 advisory) This value is communicated to HTTP/2 clients as the server's flow control limit. The HTTP/2 transport layer enforces this limit automatically via SETTINGS frames and GOAWAY responses. Applications should NOT implement custom enforcement. Default: 100 streams per connection # Stream Limiting Strategy - **Per Connection**: This limit applies per HTTP/2 connection, not globally - **Transport Enforcement**: HTTP/2 handles all stream limiting; applications need not implement custom checks - **Streaming Requests**: In server streaming or bidi streaming, each logical RPC consumes one stream slot. Message ordering within a stream follows HTTP/2 frame ordering. |
| `enable_keepalive` | `bool` | `True` | Enable HTTP/2 keepalive |
| `keepalive_interval` | `int` | — | HTTP/2 keepalive interval in seconds |
| `keepalive_timeout` | `int` | — | HTTP/2 keepalive timeout in seconds |
| `max_stream_response_bytes` | `int \| None` | `None` | Total byte cap across an entire streaming response. When `Some(n)`, the streaming adapter aborts the stream with `tonic.Status.resource_exhausted` once the cumulative encoded message bytes exceed `n`. The stream yields the error item and then terminates. Per-message cap remains `max_message_size`. This limit applies to server-streaming and bidirectional-streaming RPCs only; unary RPCs are governed solely by `max_message_size`. Default: `None` (unbounded total response size). |

### Methods

#### default()

**Signature:**

```python
@staticmethod
def default() -> GrpcConfig
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

```python
def call(self, request: Request, request_data: RequestData) -> HandlerResult
```

#### prefers_raw_json_body()

Whether this handler prefers consuming `RequestData.raw_body` over the parsed
`RequestData.body` for JSON requests.

When `True`, the server may skip eager JSON parsing when there is no request-body
schema validator attached to the route.

**Signature:**

```python
def prefers_raw_json_body(self) -> bool
```

#### prefers_parameter_extraction()

Whether this handler wants to perform its own parameter validation/extraction (path/query/header/cookie).

When `True`, the server will skip `ParameterValidator.validate_and_extract` in `ValidatingHandler`.
This is useful for language bindings which need to transform validated parameters into
language-specific values (e.g., Python kwargs) without duplicating work. When `False`,
the server stores validated output in `RequestData.validated_params`.

**Signature:**

```python
def prefers_parameter_extraction(self) -> bool
```

#### wants_headers()

Whether this handler needs the parsed headers map in `RequestData`.

When `False`, the server may skip building `RequestData.headers` for requests without a body.
(Requests with bodies still typically need `Content-Type` decisions.)

**Signature:**

```python
def wants_headers(self) -> bool
```

#### wants_cookies()

Whether this handler needs the parsed cookies map in `RequestData`.

When `False`, the server may skip parsing cookies for requests without a body.

**Signature:**

```python
def wants_cookies(self) -> bool
```

#### wants_request_extensions()

Whether this handler needs `RequestData` stored in request extensions.

When `False`, the server avoids inserting `RequestData` into extensions to
skip cloning in hot paths.

**Signature:**

```python
def wants_request_extensions(self) -> bool
```

#### static_response()

Return a pre-built static response if this handler always produces the
same output. When `Some`, the server bypasses the full middleware
pipeline and serves the pre-built response directly.

**Signature:**

```python
def static_response(self) -> StaticResponse | None
```

---

#### IntoHandler

Convert user-facing handler functions into the low-level `Handler` trait.

### Methods

#### into_handler()

Convert this value into a shared request handler.

**Signature:**

```python
def into_handler(self) -> Handler
```

---

#### JsonRpcConfig

JSON-RPC server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `True` | Enable JSON-RPC endpoint |
| `endpoint_path` | `str` | — | HTTP endpoint path for JSON-RPC requests (default: "/rpc") |
| `enable_batch` | `bool` | — | Enable batch request processing (default: true) |
| `max_batch_size` | `int` | — | Maximum number of requests in a batch (default: 100) |

### Methods

#### default()

**Signature:**

```python
@staticmethod
def default() -> JsonRpcConfig
```

---

#### JsonRpcMethodInfo

JSON-RPC method metadata for routes that support JSON-RPC

This struct captures the metadata needed to expose HTTP routes as JSON-RPC methods,
enabling discovery and documentation of RPC-compatible endpoints.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `method_name` | `str` | — | The JSON-RPC method name (e.g., "user.create") |
| `description` | `str \| None` | `None` | Optional description of what the method does |
| `params_schema` | `dict[str, Any] \| None` | `None` | Optional JSON Schema for method parameters |
| `result_schema` | `dict[str, Any] \| None` | `None` | Optional JSON Schema for the result |
| `deprecated` | `bool` | `/* serde(default) */` | Whether this method is deprecated |
| `tags` | `list[str]` | `/* serde(default) */` | Tags for categorizing and grouping methods |

---

#### JwtConfig

JWT authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `secret` | `str` | — | Secret key for JWT verification |
| `algorithm` | `str` | `/* serde(default) */` | Required algorithm (HS256, HS384, HS512, RS256, etc.) |
| `audience` | `list[str] \| None` | `None` | Required audience claim |
| `issuer` | `str \| None` | `None` | Required issuer claim |
| `leeway` | `int` | `/* serde(default) */` | Leeway for expiration checks (seconds) |

---

#### LicenseInfo

License information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | SPDX license identifier or display name (e.g. `"MIT"`). |
| `url` | `str \| None` | `None` | URL to the full license text. |

---

#### OpenApiConfig

OpenAPI configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `False` | Enable OpenAPI generation (default: false for zero overhead) |
| `title` | `str` | `"API"` | API title |
| `version` | `str` | `"1.0.0"` | API version |
| `description` | `str \| None` | `None` | API description (supports markdown) |
| `swagger_ui_path` | `str` | — | Path to serve Swagger UI (default: "/docs") |
| `redoc_path` | `str` | — | Path to serve Redoc (default: "/redoc") |
| `openapi_json_path` | `str` | — | Path to serve OpenAPI JSON spec (default: "/openapi.json") |
| `contact` | `ContactInfo \| None` | `None` | Contact information |
| `license` | `LicenseInfo \| None` | `None` | License information |
| `servers` | `list[ServerInfo]` | `[]` | Server definitions |
| `security_schemes` | `dict[str, SecuritySchemeInfo]` | `{}` | Security schemes (auto-detected from middleware if not provided) |

### Methods

#### default()

**Signature:**

```python
@staticmethod
def default() -> OpenApiConfig
```

---

#### ParseRequest

Request body for `POST /asyncapi/parse`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `spec` | `dict[str, Any]` | — | Spec |

---

#### ParseResult

Full parse result returned by `POST /asyncapi/parse`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `spec_version` | `str` | — | Spec version |
| `title` | `str` | — | Title |
| `api_version` | `str` | — | Api version |
| `channels` | `list[ParsedChannel]` | — | Channels |
| `operations` | `list[ParsedOperation]` | — | Operations |
| `messages` | `list[ParsedMessage]` | — | Messages |

---

#### ParsedChannel

A single channel extracted from an AsyncAPI spec

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | Channel key from the spec (e.g. "chat/messages") |
| `address` | `str` | — | Channel address / path |
| `messages` | `list[str]` | — | Message names declared on this channel |
| `bindings` | `dict[str, Any] \| None` | `None` | Bindings (ws / http / amqp / …) as raw JSON for forward-compatibility |

---

#### ParsedMessage

A resolved message (name + JSON Schema)

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | Message name |
| `schema` | `dict[str, Any] \| None` | `None` | Resolved JSON Schema for the message payload, if available |

---

#### ParsedOperation

A single operation extracted from an AsyncAPI spec

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | Operation name |
| `action` | `str` | — | Operation action: "send" or "receive" |
| `channel` | `str` | — | Channel reference (resolved to the channel name) |

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
| `type_uri` | `str` | — | A URI reference that identifies the problem type. Defaults to "about:blank" when absent. Should be a stable, human-readable identifier for the problem type. |
| `title` | `str` | — | A short, human-readable summary of the problem type. Should not change from occurrence to occurrence of the problem. |
| `status` | `int` | — | The HTTP status code generated by the origin server. This is advisory; the actual HTTP status code takes precedence. |
| `detail` | `str \| None` | `None` | A human-readable explanation specific to this occurrence of the problem. |
| `instance` | `str \| None` | `None` | A URI reference that identifies the specific occurrence of the problem. It may or may not yield further information if dereferenced. |
| `extensions` | `dict[str, dict[str, Any]]` | — | Extension members - problem-type-specific data. For validation errors, this typically contains an "errors" array. |

### Methods

#### with_detail()

Set the detail field

**Signature:**

```python
def with_detail(self, detail: str) -> ProblemDetails
```

#### with_instance()

Set the instance field

**Signature:**

```python
def with_instance(self, instance: str) -> ProblemDetails
```

#### not_found()

Create a not found error

**Signature:**

```python
@staticmethod
def not_found(detail: str) -> ProblemDetails
```

#### method_not_allowed()

Create a method not allowed error

**Signature:**

```python
@staticmethod
def method_not_allowed(detail: str) -> ProblemDetails
```

#### internal_server_error()

Create an internal server error

**Signature:**

```python
@staticmethod
def internal_server_error(detail: str) -> ProblemDetails
```

#### bad_request()

Create a bad request error

**Signature:**

```python
@staticmethod
def bad_request(detail: str) -> ProblemDetails
```

#### to_json()

Serialize to JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```python
def to_json(self) -> str
```

#### to_json_pretty()

Serialize to pretty JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```python
def to_json_pretty(self) -> str
```

---

#### QueryMutationConfig

Configuration for schemas with Query and Mutation types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `True` | Enable introspection queries |
| `complexity_limit` | `int \| None` | `None` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `int \| None` | `None` | Maximum query depth (None = unlimited) |

### Methods

#### default()

**Signature:**

```python
@staticmethod
def default() -> QueryMutationConfig
```

---

#### QueryOnlyConfig

Configuration for schemas with only Query type

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `True` | Enable introspection queries |
| `complexity_limit` | `int \| None` | `None` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `int \| None` | `None` | Maximum query depth (None = unlimited) |

### Methods

#### default()

**Signature:**

```python
@staticmethod
def default() -> QueryOnlyConfig
```

---

#### RateLimitConfig

Rate limiting configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `per_second` | `int` | `100` | Requests per second |
| `burst` | `int` | `200` | Burst allowance |
| `ip_based` | `bool` | `True` | Use IP-based rate limiting |

### Methods

#### default()

**Signature:**

```python
@staticmethod
def default() -> RateLimitConfig
```

---

#### Response

HTTP Response with custom status code, headers, and content

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `dict[str, Any] \| None` | `None` | Response body content |
| `status_code` | `int` | — | HTTP status code (defaults to 200) |
| `headers` | `dict[str, str]` | `{}` | Response headers |

### Methods

#### set_header()

Set a header

**Signature:**

```python
def set_header(self, key: str, value: str) -> None
```

#### set_cookie()

Set a cookie in the response

**Signature:**

```python
def set_cookie(self, key: str, value: str, secure: bool, http_only: bool, max_age: int, domain: str, path: str, same_site: str) -> None
```

#### default()

**Signature:**

```python
@staticmethod
def default() -> Response
```

---

#### ResponseSnapshot

Snapshot of an Axum response used by higher-level language bindings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `status` | `int` | — | HTTP status code. |
| `headers` | `dict[str, str]` | — | Response headers (lowercase keys for predictable lookups). |
| `body` | `bytes` | — | Response body bytes (decoded for supported encodings). |

### Methods

#### text()

Return response body as UTF-8 string.

**Signature:**

```python
def text(self) -> str
```

#### header()

Lookup header by case-insensitive name.

**Signature:**

```python
def header(self, name: str) -> str | None
```

---

#### RouteBuilder

Builder for defining a route.

### Methods

#### new()

Create a new builder for the provided HTTP method and path.

**Signature:**

```python
@staticmethod
def new(method: Method, path: str) -> RouteBuilder
```

#### handler_name()

Assign an explicit handler name.

**Signature:**

```python
def handler_name(self, name: str) -> RouteBuilder
```

#### request_schema_json()

Provide a raw JSON schema for the request body.

**Signature:**

```python
def request_schema_json(self, schema: dict[str, Any]) -> RouteBuilder
```

#### response_schema_json()

Provide a raw JSON schema for the response body.

**Signature:**

```python
def response_schema_json(self, schema: dict[str, Any]) -> RouteBuilder
```

#### params_schema_json()

Provide a raw JSON schema for request parameters.

**Signature:**

```python
def params_schema_json(self, schema: dict[str, Any]) -> RouteBuilder
```

#### file_params_json()

Provide multipart file parameter configuration.

**Signature:**

```python
def file_params_json(self, schema: dict[str, Any]) -> RouteBuilder
```

#### cors()

Attach a CORS configuration for this route.

**Signature:**

```python
def cors(self, cors: CorsConfig) -> RouteBuilder
```

#### sync()

Mark the route as synchronous.

**Signature:**

```python
def sync(self) -> RouteBuilder
```

#### handler_dependencies()

Declare the dependency keys that must be resolved before this handler runs.

**Signature:**

```python
def handler_dependencies(self, dependencies: list[str]) -> RouteBuilder
```

---

#### SchemaConfig

Configuration for GraphQL schema building.

Encapsulates all schema-level configuration options including
introspection control, complexity limits, and depth limits.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `True` | Enable introspection queries |
| `complexity_limit` | `int \| None` | `None` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `int \| None` | `None` | Maximum query depth (None = unlimited) |

### Methods

#### default()

**Signature:**

```python
@staticmethod
def default() -> SchemaConfig
```

---

#### ServerConfig

Server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `host` | `str` | `"127.0.0.1"` | Host to bind to |
| `port` | `int` | `8000` | Port to bind to |
| `workers` | `int` | `1` | Number of Tokio runtime worker threads used by binding-managed server runtimes |
| `enable_request_id` | `bool` | `False` | Enable request ID generation and propagation |
| `max_body_size` | `int \| None` | `None` | Maximum request body size in bytes (None = unlimited, not recommended) |
| `request_timeout` | `int \| None` | `None` | Request timeout in seconds (None = no timeout) |
| `compression` | `CompressionConfig \| None` | `None` | Enable compression middleware |
| `rate_limit` | `RateLimitConfig \| None` | `None` | Enable rate limiting |
| `jwt_auth` | `JwtConfig \| None` | `None` | JWT authentication configuration |
| `api_key_auth` | `ApiKeyConfig \| None` | `None` | API Key authentication configuration |
| `static_files` | `list[StaticFilesConfig]` | `[]` | Static file serving configuration |
| `graceful_shutdown` | `bool` | `True` | Enable graceful shutdown on SIGTERM/SIGINT |
| `shutdown_timeout` | `int` | `30` | Graceful shutdown timeout (seconds) |
| `asyncapi` | `AsyncApiConfig \| None` | `None` | AsyncAPI HTTP endpoint configuration |
| `openapi` | `OpenApiConfig \| None` | `None` | OpenAPI documentation configuration |
| `jsonrpc` | `JsonRpcConfig \| None` | `None` | JSON-RPC configuration |
| `grpc` | `GrpcConfig \| None` | `None` | gRPC configuration |
| `lifecycle_hooks` | `str \| None` | `None` | Lifecycle hooks for request/response processing |
| `background_tasks` | `BackgroundTaskConfig` | — | Background task executor configuration |
| `enable_http_trace` | `bool` | `False` | Enable per-request HTTP tracing (tower-http `TraceLayer`) |
| `di_container` | `str \| None` | `None` | Dependency injection container (requires 'di' feature) |

### Methods

#### default()

**Signature:**

```python
@staticmethod
def default() -> ServerConfig
```

---

#### ServerInfo

Server information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `str` | — | Base URL of the server (e.g. `"<https://api.example.com/v1"`>). |
| `description` | `str \| None` | `None` | Optional human-readable description of the server environment. |

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
| `event_type` | `str \| None` | `None` | Event type (optional) |
| `data` | `dict[str, Any]` | — | Event data (JSON value) |
| `id` | `str \| None` | `None` | Event ID (optional, for client-side reconnection) |
| `retry` | `int \| None` | `None` | Retry timeout in milliseconds (optional) |

### Methods

#### with_id()

Set the event ID for client-side reconnection support

Sets an ID that clients can use to resume from this point if they disconnect.
The client sends this ID back in the `Last-Event-ID` header when reconnecting.

**Signature:**

```python
def with_id(self, id: str) -> SseEvent
```

#### with_retry()

Set the retry timeout for client reconnection

Sets the time in milliseconds clients should wait before attempting to reconnect
if the connection is lost. The client browser will automatically handle reconnection.

**Signature:**

```python
def with_retry(self, retry_ms: int) -> SseEvent
```

---

#### SseEventProducer

SSE event producer trait

Implement this trait to create custom Server-Sent Event (SSE) producers for your application.
The producer generates events that are streamed to connected clients.

### Understanding SSE

Server-Sent Events (SSE) provide one-way communication from server to client over HTTP.
Unlike WebSocket, SSE uses standard HTTP and automatically handles reconnection.
Use SSE when you need to push data to clients without bidirectional communication.

### Implementing the Trait

You must implement the `next_event` method to generate events. The `on_connect` and
`on_disconnect` methods are optional lifecycle hooks.

### Methods

#### next_event()

Generate the next event

Called repeatedly to produce the event stream. Should return `Some(event)` when
an event is ready to send, or `None` when the stream should end.

**Returns:**

- `Some(event)` - Event to send to the client
- `None` - Stream complete, connection will close

**Signature:**

```python
def next_event(self) -> Future
```

#### on_connect()

Called when a client connects to the SSE endpoint

Optional lifecycle hook invoked when a new SSE connection is established.
Default implementation does nothing.

**Signature:**

```python
def on_connect(self) -> Future
```

#### on_disconnect()

Called when a client disconnects from the SSE endpoint

Optional lifecycle hook invoked when an SSE connection is closed (either by the
client or the stream ending). Default implementation does nothing.

**Signature:**

```python
def on_disconnect(self) -> Future
```

---

#### StaticFilesConfig

Static file serving configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `directory` | `str` | — | Directory path to serve |
| `route_prefix` | `str` | — | URL path prefix (e.g., "/static") |
| `index_file` | `bool` | `/* serde(default) */` | Fallback to index.html for directories |
| `cache_control` | `str \| None` | `None` | Cache-Control header value |

---

#### TestClient

Core test client for making HTTP requests to a Spikard application.

This struct wraps axum-test's TestServer and provides a language-agnostic
interface for making HTTP requests, sending WebSocket connections, and
handling Server-Sent Events. Language bindings wrap this to provide
native API surfaces.

### Methods

#### get()

Make a GET request

**Signature:**

```python
def get(self, path: str, query_params: list[list[str]], headers: list[list[str]]) -> ResponseSnapshot
```

#### post()

Make a POST request

**Signature:**

```python
def post(self, path: str, json: dict[str, Any], form_data: list[list[str]], multipart: str, query_params: list[list[str]], headers: list[list[str]]) -> ResponseSnapshot
```

#### request_raw()

Make a request with a raw body payload.

**Signature:**

```python
def request_raw(self, method: Method, path: str, body: bytes, query_params: list[list[str]], headers: list[list[str]]) -> ResponseSnapshot
```

#### put()

Make a PUT request

**Signature:**

```python
def put(self, path: str, json: dict[str, Any], query_params: list[list[str]], headers: list[list[str]]) -> ResponseSnapshot
```

#### patch()

Make a PATCH request

**Signature:**

```python
def patch(self, path: str, json: dict[str, Any], query_params: list[list[str]], headers: list[list[str]]) -> ResponseSnapshot
```

#### delete()

Make a DELETE request

**Signature:**

```python
def delete(self, path: str, query_params: list[list[str]], headers: list[list[str]]) -> ResponseSnapshot
```

#### options()

Make an OPTIONS request

**Signature:**

```python
def options(self, path: str, query_params: list[list[str]], headers: list[list[str]]) -> ResponseSnapshot
```

#### head()

Make a HEAD request

**Signature:**

```python
def head(self, path: str, query_params: list[list[str]], headers: list[list[str]]) -> ResponseSnapshot
```

#### trace()

Make a TRACE request

**Signature:**

```python
def trace(self, path: str, query_params: list[list[str]], headers: list[list[str]]) -> ResponseSnapshot
```

#### graphql_at()

Send a GraphQL query/mutation to a custom endpoint

**Signature:**

```python
def graphql_at(self, endpoint: str, query: str, variables: dict[str, Any], operation_name: str) -> ResponseSnapshot
```

#### graphql()

Send a GraphQL query/mutation

**Signature:**

```python
def graphql(self, query: str, variables: dict[str, Any], operation_name: str) -> ResponseSnapshot
```

#### graphql_with_status()

Send a GraphQL query and return HTTP status code separately

This method allows tests to distinguish between:

- HTTP-level errors (400/422 for invalid requests)
- GraphQL-level errors (200 with errors in response body)

**Signature:**

```python
def graphql_with_status(self, query: str, variables: dict[str, Any], operation_name: str) -> str
```

#### graphql_subscription_at()

Send a GraphQL subscription (WebSocket) to a custom endpoint.

Uses the `graphql-transport-ws` protocol and captures the first `next` payload.
After the first payload is received, this client sends `complete` to unsubscribe.

**Signature:**

```python
def graphql_subscription_at(self, endpoint: str, query: str, variables: dict[str, Any], operation_name: str) -> GraphQlSubscriptionSnapshot
```

#### graphql_subscription()

Send a GraphQL subscription (WebSocket).

Uses `/graphql` as the default subscription endpoint.

**Signature:**

```python
def graphql_subscription(self, query: str, variables: dict[str, Any], operation_name: str) -> GraphQlSubscriptionSnapshot
```

---

#### TestingSseEvent

A single Server-Sent Event.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data` | `str` | — | The data field of the event. |

---

#### UploadFile

Represents an uploaded file from multipart/form-data requests.

This struct provides efficient access to file content with automatic
base64 decoding and implements standard I/O traits for compatibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `filename` | `str` | — | Original filename from the client |
| `content_type` | `str \| None` | `None` | MIME type of the uploaded file |
| `size` | `int \| None` | `None` | Size of the file in bytes |
| `content` | `bytes` | — | File content (may be base64 encoded) |
| `content_encoding` | `str \| None` | `None` | Content encoding type |
| `cursor` | `str` | — | Internal cursor for Read/Seek operations |

### Methods

#### as_bytes()

Get the raw file content as bytes.

This provides zero-copy access to the underlying buffer.

**Signature:**

```python
def as_bytes(self) -> bytes
```

#### read_to_string()

Read the file content as a UTF-8 string.

**Errors:**

Returns an error if the content is not valid UTF-8.

**Signature:**

```python
def read_to_string(self) -> str
```

#### content_type_or_default()

Get the content type, defaulting to "application/octet-stream".

**Signature:**

```python
def content_type_or_default(self) -> str
```

---

#### ValidateRequest

Request body for `POST /asyncapi/validate`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `spec` | `dict[str, Any]` | — | Spec |
| `channel` | `str` | — | Channel |
| `message` | `str` | — | Message |
| `payload` | `dict[str, Any]` | — | Payload |

---

#### ValidationResponse

Response body for `POST /asyncapi/validate`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `valid` | `bool` | — | Valid |
| `errors` | `list[str]` | — | Errors |

---

#### WebSocketHandler

WebSocket message handler trait

Implement this trait to create custom WebSocket message handlers for your application.
The handler processes JSON messages received from WebSocket clients and can optionally
send responses back.

### Implementing the Trait

You must implement the `handle_message` method. The `on_connect` and `on_disconnect`
methods are optional and provide lifecycle hooks.

### Methods

#### handle_message()

Handle incoming WebSocket message

Called whenever a text message is received from a WebSocket client.
Messages are automatically parsed as JSON.

**Returns:**

- `Some(value)` - JSON value to send back to the client
- `None` - No response to send

**Signature:**

```python
def handle_message(self, message: dict[str, Any]) -> Future
```

#### on_connect()

Called when a client connects to the WebSocket

Optional lifecycle hook invoked when a new WebSocket connection is established.
Default implementation does nothing.

**Signature:**

```python
def on_connect(self) -> Future
```

#### on_disconnect()

Called when a client disconnects from the WebSocket

Optional lifecycle hook invoked when a WebSocket connection is closed
(either by the client or due to an error). Default implementation does nothing.

**Signature:**

```python
def on_disconnect(self) -> Future
```

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
| `HTTP` | Http — Fields: `scheme`: `str`, `bearer_format`: `str` |
| `API_KEY` | Api key — Fields: `location`: `str`, `name`: `str` |

---

#### SnapshotError

Possible errors while converting an Axum response into a snapshot.

| Value | Description |
|-------|-------------|
| `INVALID_HEADER` | Response header could not be decoded to UTF-8. — Fields: `0`: `str` |
| `DECOMPRESSION` | Body decompression failed. — Fields: `0`: `str` |

---

#### WebSocketMessage

A WebSocket message that can be text or binary.

| Value | Description |
|-------|-------------|
| `TEXT` | A text message. — Fields: `0`: `str` |
| `BINARY` | A binary message. — Fields: `0`: `bytes` |
| `CLOSE` | A close message with a numeric close code (RFC 6455) and optional reason text. Common codes: 1000 Normal Closure, 1001 Going Away, 1005 No Status Received, 1006 Abnormal Closure. — Fields: `code`: `int`, `reason`: `str` |
| `PING` | A ping message. — Fields: `0`: `bytes` |
| `PONG` | A pong message. — Fields: `0`: `bytes` |

---

### Errors

#### AppError

Error type for application builder operations.

**Base class:** `AppError(Exception)`

| Exception | Description |
|-----------|-------------|
| `Route(AppError)` | Route registration failed. |
| `Server(AppError)` | Server/router construction failed. |
| `Decode(AppError)` | Failed to extract DTO from the request context. |

---

#### GraphQlError

Errors that can occur during GraphQL operations

These errors are compatible with async-graphql error handling and can be
converted to structured HTTP responses matching the project's error fixtures.

**Base class:** `GraphQlError(Exception)`

| Exception | Description |
|-----------|-------------|
| `ExecutionError(GraphQlError)` | Error during schema execution Occurs when the GraphQL executor encounters a runtime error during query execution. |
| `SchemaBuildError(GraphQlError)` | Error during schema building Occurs when schema construction fails due to invalid definitions or conflicts. |
| `RequestHandlingError(GraphQlError)` | Error during request handling Occurs when the HTTP request cannot be properly handled or parsed. |
| `SerializationError(GraphQlError)` | Serialization error Occurs during JSON serialization/deserialization of GraphQL values. |
| `JsonError(GraphQlError)` | JSON parsing error Occurs when JSON input cannot be parsed. |
| `ValidationError(GraphQlError)` | GraphQL validation error Occurs when a GraphQL query fails schema validation. |
| `ParseError(GraphQlError)` | GraphQL parse error Occurs when the GraphQL query string cannot be parsed. |
| `AuthenticationError(GraphQlError)` | Authentication error Occurs when request authentication fails. |
| `AuthorizationError(GraphQlError)` | Authorization error Occurs when user lacks required permissions. |
| `NotFound(GraphQlError)` | Not found error Occurs when a requested resource is not found. |
| `RateLimitExceeded(GraphQlError)` | Rate limit error Occurs when rate limit is exceeded. |
| `InvalidInput(GraphQlError)` | Invalid input error with validation details Occurs during input validation with detailed error information. |
| `ComplexityLimitExceeded(GraphQlError)` | Query complexity limit exceeded Occurs when a GraphQL query exceeds the configured complexity limit. |
| `DepthLimitExceeded(GraphQlError)` | Query depth limit exceeded Occurs when a GraphQL query exceeds the configured depth limit. |
| `InternalError(GraphQlError)` | Internal server error Occurs when an unexpected internal error happens. |

---

#### SchemaError

Error type for schema building operations

**Base class:** `SchemaError(Exception)`

| Exception | Description |
|-----------|-------------|
| `BuildingFailed(SchemaError)` | Generic schema building error |
| `ValidationError(SchemaError)` | Configuration validation error |
| `ComplexityLimitExceeded(SchemaError)` | Complexity limit exceeded |
| `DepthLimitExceeded(SchemaError)` | Depth limit exceeded |

---

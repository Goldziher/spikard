---
title: "Python API Reference"
---

## Python API Reference <span class="version-badge">v0.14.0</span>

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

#### add_cors_headers()

Add CORS headers to a successful response

Adds appropriate CORS headers to the response based on the configuration.
This function should be called for successful (non-error) responses to
cross-origin requests.

## Headers Added

- `Access-Control-Allow-Origin` - The origin that is allowed (if valid)
- `Access-Control-Expose-Headers` - Headers that are safe to expose to the client
- `Access-Control-Allow-Credentials` - "true" if credentials are allowed

**Signature:**

```python
def add_cors_headers(response: Response, origin: str, cors_config: CorsConfig) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `response` | `Response` | Yes | Mutable reference to the response to modify |
| `origin` | `str` | Yes | The origin from the request (e.g., `<https://example.com>`) |
| `cors_config` | `CorsConfig` | Yes | CORS configuration to apply |

**Returns:** `None`

---

### Types

#### ApiKeyConfig

API Key authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `keys` | `list[str]` | — | Valid API keys |
| `header_name` | `str` | — | Header name to check (e.g., "X-API-Key") |

---

##### BackgroundHandle

---

##### BackgroundJobError

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `str` | — | Message |

###### Methods

###### from()

**Signature:**

```python
@staticmethod
def from(message: str) -> BackgroundJobError
```

---

##### BackgroundJobMetadata

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | The name |
| `request_id` | `str | None` | `None` | Request id |

###### Methods

###### default()

**Signature:**

```python
@staticmethod
def default() -> BackgroundJobMetadata
```

---

##### BackgroundTaskConfig

Configuration for in-process background task execution.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_queue_size` | `int` | `1024` | Maximum queue size |
| `max_concurrent_tasks` | `int` | `128` | Maximum concurrent tasks |
| `drain_timeout_secs` | `int` | `30` | Drain timeout secs |

###### Methods

###### default()

**Signature:**

```python
@staticmethod
def default() -> BackgroundTaskConfig
```

---

##### Claims

JWT claims structure - can be extended based on needs

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `sub` | `str` | — | Sub |
| `exp` | `int` | — | Exp |
| `iat` | `int | None` | `None` | Iat |
| `nbf` | `int | None` | `None` | Nbf |
| `aud` | `list[str] | None` | `None` | Aud |
| `iss` | `str | None` | `None` | Iss |

---

##### CompressionConfig

Compression configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `gzip` | `bool` | `True` | Enable gzip compression |
| `brotli` | `bool` | `True` | Enable brotli compression |
| `min_size` | `int` | — | Minimum response size to compress (bytes) |
| `quality` | `int` | — | Compression quality (0-11 for brotli, 0-9 for gzip) |

###### Methods

###### default()

**Signature:**

```python
@staticmethod
def default() -> CompressionConfig
```

---

##### ContactInfo

Contact information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str | None` | `None` | The name |
| `email` | `str | None` | `None` | Email |
| `url` | `str | None` | `None` | Url |

---

##### CorsConfig

CORS configuration for a route

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `allowed_origins` | `list[str]` | `[]` | Allowed origins |
| `allowed_methods` | `list[str]` | `[]` | Allowed methods |
| `allowed_headers` | `list[str]` | `[]` | Allowed headers |
| `expose_headers` | `list[str] | None` | `None` | Expose headers |
| `max_age` | `int | None` | `None` | Maximum age |
| `allow_credentials` | `bool | None` | `None` | Allow credentials |
| `methods_joined_cache` | `str` | — | Methods joined cache |
| `headers_joined_cache` | `str` | — | Headers joined cache |

###### Methods

###### allowed_methods_joined()

Get the cached joined methods string for preflight responses

**Signature:**

```python
def allowed_methods_joined(self) -> str
```

###### allowed_headers_joined()

Get the cached joined headers string for preflight responses

**Signature:**

```python
def allowed_headers_joined(self) -> str
```

###### is_origin_allowed()

Check if an origin is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```python
def is_origin_allowed(self, origin: str) -> bool
```

###### is_method_allowed()

Check if a method is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```python
def is_method_allowed(self, method: str) -> bool
```

###### are_headers_allowed()

Check if all requested headers are allowed (O(n) where n = num requested headers)

**Signature:**

```python
def are_headers_allowed(self, requested: list[str]) -> bool
```

###### default()

**Signature:**

```python
@staticmethod
def default() -> CorsConfig
```

---

##### FullSchemaConfig

Configuration for fully-featured schemas with Query, Mutation, and Subscription types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `True` | Enable introspection queries |
| `complexity_limit` | `int | None` | `None` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `int | None` | `None` | Maximum query depth (None = unlimited) |

###### Methods

###### default()

**Signature:**

```python
@staticmethod
def default() -> FullSchemaConfig
```

---

##### GraphQlError

###### Methods

###### status_code()

Convert error to HTTP status code

Maps GraphQL error types to appropriate HTTP status codes:

- 400: Bad Request for parse/request-handling errors
- 401: Unauthorized for authentication errors
- 403: Forbidden for authorization errors
- 404: Not Found for resource not found
- 422: Unprocessable Entity for validation failures
- 429: Too Many Requests for rate limit errors
- 500: Internal Server Error for schema/serialization/internal errors
- 200: OK for GraphQL execution errors returned in GraphQL response body

**Signature:**

```python
def status_code(self) -> int
```

---

##### GraphQlRouteConfig

Configuration for GraphQL routes

Provides a builder pattern for configuring GraphQL route parameters
for the Spikard HTTP server's routing system.

###### Methods

###### path()

Set the HTTP path for the GraphQL endpoint

**Signature:**

```python
def path(self, path: str) -> GraphQlRouteConfig
```

###### method()

Set the HTTP method for the GraphQL endpoint

**Signature:**

```python
def method(self, method: str) -> GraphQlRouteConfig
```

###### enable_playground()

Enable or disable the GraphQL Playground UI

**Signature:**

```python
def enable_playground(self, enable: bool) -> GraphQlRouteConfig
```

###### description()

Set a custom description for documentation

**Signature:**

```python
def description(self, description: str) -> GraphQlRouteConfig
```

###### get_path()

Get the configured path

**Signature:**

```python
def get_path(self) -> str
```

###### get_method()

Get the configured method

**Signature:**

```python
def get_method(self) -> str
```

###### is_playground_enabled()

Check if playground is enabled

**Signature:**

```python
def is_playground_enabled(self) -> bool
```

###### get_description()

Get the description if set

**Signature:**

```python
def get_description(self) -> str | None
```

###### default()

**Signature:**

```python
@staticmethod
def default() -> GraphQlRouteConfig
```

---

##### GrpcConfig

Configuration for gRPC support

Controls how the server handles gRPC requests, including compression,
timeouts, and protocol settings.

## Stream Limits

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

- **Stream Length Limits**: There is currently no built-in limit on the
  total number of messages in a stream. Handlers should implement their own
  message counting if needed. Future versions may add a `max_stream_response_bytes`
  field to limit total response size per stream.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `True` | Enable gRPC support |
| `max_message_size` | `int` | — | Maximum message size in bytes (for both sending and receiving) This limit applies to individual messages in both unary and streaming RPCs. When a single message exceeds this size, the request is rejected with HTTP 413 (Payload Too Large). Default: 4MB (4194304 bytes) **Note:** This limit does NOT apply to the total response size in streaming RPCs. For multi-message streams, the total response can exceed this limit as long as each individual message stays within the limit. |
| `enable_compression` | `bool` | `True` | Enable gzip compression for gRPC messages |
| `request_timeout` | `int | None` | `None` | Timeout for gRPC requests in seconds (None = no timeout) |
| `max_concurrent_streams` | `int` | — | Maximum number of concurrent streams per connection (HTTP/2 advisory) This value is communicated to HTTP/2 clients as the server's flow control limit. The HTTP/2 transport layer enforces this limit automatically via SETTINGS frames and GOAWAY responses. Applications should NOT implement custom enforcement. Default: 100 streams per connection # Stream Limiting Strategy - **Per Connection**: This limit applies per HTTP/2 connection, not globally - **Transport Enforcement**: HTTP/2 handles all stream limiting; applications need not implement custom checks - **Streaming Requests**: In server streaming or bidi streaming, each logical RPC consumes one stream slot. Message ordering within a stream follows HTTP/2 frame ordering. # Future Enhancement A future `max_stream_response_bytes` field may be added to limit the total response size in streaming RPCs (separate from per-message limits). |
| `enable_keepalive` | `bool` | `True` | Enable HTTP/2 keepalive |
| `keepalive_interval` | `int` | — | HTTP/2 keepalive interval in seconds |
| `keepalive_timeout` | `int` | — | HTTP/2 keepalive timeout in seconds |

### Methods

#### default()

**Signature:**

```python
@staticmethod
def default() -> GrpcConfig
```

---

##### GrpcRequestData

gRPC request data passed to handlers

Contains the parsed components of a gRPC request:

- Service and method names from the request path
- Serialized protobuf payload as bytes
- Request metadata (headers)

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `service_name` | `str` | — | Fully qualified service name (e.g., "mypackage.MyService") |
| `method_name` | `str` | — | Method name (e.g., "GetUser") |
| `payload` | `bytes` | — | Serialized protobuf message bytes |
| `metadata` | `str` | — | gRPC metadata (similar to HTTP headers) |

---

##### GrpcResponseData

gRPC response data returned by handlers

Contains the serialized protobuf response and any metadata to include
in the response headers.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `payload` | `bytes` | — | Serialized protobuf message bytes |
| `metadata` | `str` | — | gRPC metadata to include in response (similar to HTTP headers) |

---

##### JsonRpcConfig

JSON-RPC server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `True` | Enable JSON-RPC endpoint |
| `endpoint_path` | `str` | — | HTTP endpoint path for JSON-RPC requests (default: "/rpc") |
| `enable_batch` | `bool` | — | Enable batch request processing (default: true) |
| `max_batch_size` | `int` | — | Maximum number of requests in a batch (default: 100) |

###### Methods

###### default()

**Signature:**

```python
@staticmethod
def default() -> JsonRpcConfig
```

---

##### JsonRpcMethodInfo

JSON-RPC method metadata for routes that support JSON-RPC

This struct captures the metadata needed to expose HTTP routes as JSON-RPC methods,
enabling discovery and documentation of RPC-compatible endpoints.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `method_name` | `str` | — | The JSON-RPC method name (e.g., "user.create") |
| `description` | `str | None` | `None` | Optional description of what the method does |
| `params_schema` | `str | None` | `None` | Optional JSON Schema for method parameters |
| `result_schema` | `str | None` | `None` | Optional JSON Schema for the result |
| `deprecated` | `bool` | — | Whether this method is deprecated |
| `tags` | `list[str]` | — | Tags for categorizing and grouping methods |

---

##### JwtConfig

JWT authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `secret` | `str` | — | Secret key for JWT verification |
| `algorithm` | `str` | — | Required algorithm (HS256, HS384, HS512, RS256, etc.) |
| `audience` | `list[str] | None` | `None` | Required audience claim |
| `issuer` | `str | None` | `None` | Required issuer claim |
| `leeway` | `int` | — | Leeway for expiration checks (seconds) |

---

##### LicenseInfo

License information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | The name |
| `url` | `str | None` | `None` | Url |

---

##### OpenApiConfig

OpenAPI configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `False` | Enable OpenAPI generation (default: false for zero overhead) |
| `title` | `str` | `"API"` | API title |
| `version` | `str` | `"1.0.0"` | API version |
| `description` | `str | None` | `None` | API description (supports markdown) |
| `swagger_ui_path` | `str` | — | Path to serve Swagger UI (default: "/docs") |
| `redoc_path` | `str` | — | Path to serve Redoc (default: "/redoc") |
| `openapi_json_path` | `str` | — | Path to serve OpenAPI JSON spec (default: "/openapi.json") |
| `contact` | `ContactInfo | None` | `None` | Contact information |
| `license` | `LicenseInfo | None` | `None` | License information |
| `servers` | `list[ServerInfo]` | `[]` | Server definitions |
| `security_schemes` | `dict[str, SecuritySchemeInfo]` | `{}` | Security schemes (auto-detected from middleware if not provided) |

###### Methods

###### default()

**Signature:**

```python
@staticmethod
def default() -> OpenApiConfig
```

---

##### ProblemDetails

RFC 9457 Problem Details for HTTP APIs

A machine-readable format for specifying errors in HTTP API responses.
Per RFC 9457, all fields are optional. The `type` field defaults to "about:blank"
if not specified.

## Content-Type

Responses using this struct should set:

```text
Content-Type: application/problem+json
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `type_uri` | `str` | — | A URI reference that identifies the problem type. Defaults to "about:blank" when absent. Should be a stable, human-readable identifier for the problem type. |
| `title` | `str` | — | A short, human-readable summary of the problem type. Should not change from occurrence to occurrence of the problem. |
| `status` | `int` | — | The HTTP status code generated by the origin server. This is advisory; the actual HTTP status code takes precedence. |
| `detail` | `str | None` | `None` | A human-readable explanation specific to this occurrence of the problem. |
| `instance` | `str | None` | `None` | A URI reference that identifies the specific occurrence of the problem. It may or may not yield further information if dereferenced. |
| `extensions` | `dict[str, str]` | — | Extension members - problem-type-specific data. For validation errors, this typically contains an "errors" array. |

### Methods

#### with_detail()

Set the detail field

**Signature:**

```python
def with_detail(self, detail: str) -> ProblemDetails
```

##### with_instance()

Set the instance field

**Signature:**

```python
def with_instance(self, instance: str) -> ProblemDetails
```

###### not_found()

Create a not found error

**Signature:**

```python
@staticmethod
def not_found(detail: str) -> ProblemDetails
```

###### method_not_allowed()

Create a method not allowed error

**Signature:**

```python
@staticmethod
def method_not_allowed(detail: str) -> ProblemDetails
```

###### internal_server_error()

Create an internal server error

**Signature:**

```python
@staticmethod
def internal_server_error(detail: str) -> ProblemDetails
```

###### bad_request()

Create a bad request error

**Signature:**

```python
@staticmethod
def bad_request(detail: str) -> ProblemDetails
```

###### to_json()

Serialize to JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```python
def to_json(self) -> str
```

###### to_json_pretty()

Serialize to pretty JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```python
def to_json_pretty(self) -> str
```

---

##### QueryMutationConfig

Configuration for schemas with Query and Mutation types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `True` | Enable introspection queries |
| `complexity_limit` | `int | None` | `None` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `int | None` | `None` | Maximum query depth (None = unlimited) |

###### Methods

###### default()

**Signature:**

```python
@staticmethod
def default() -> QueryMutationConfig
```

---

##### QueryOnlyConfig

Configuration for schemas with only Query type

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `True` | Enable introspection queries |
| `complexity_limit` | `int | None` | `None` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `int | None` | `None` | Maximum query depth (None = unlimited) |

###### Methods

###### default()

**Signature:**

```python
@staticmethod
def default() -> QueryOnlyConfig
```

---

##### RateLimitConfig

Rate limiting configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `per_second` | `int` | `100` | Requests per second |
| `burst` | `int` | `200` | Burst allowance |
| `ip_based` | `bool` | `True` | Use IP-based rate limiting |

###### Methods

###### default()

**Signature:**

```python
@staticmethod
def default() -> RateLimitConfig
```

---

##### Response

HTTP Response with custom status code, headers, and content

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `str | None` | `None` | Response body content |
| `status_code` | `int` | — | HTTP status code (defaults to 200) |
| `headers` | `dict[str, str]` | `{}` | Response headers |

###### Methods

###### set_header()

Set a header

**Signature:**

```python
def set_header(self, key: str, value: str) -> None
```

###### set_cookie()

Set a cookie in the response

**Signature:**

```python
def set_cookie(self, key: str, value: str, max_age: int, domain: str, path: str, secure: bool, http_only: bool, same_site: str) -> None
```

###### default()

**Signature:**

```python
@staticmethod
def default() -> Response
```

---

##### Route

Route definition with compiled validators

Validators are `Arc`-wrapped to enable cheap cloning across route instances
and to support schema deduplication via `SchemaRegistry`.

The `jsonrpc_method` field is optional and has zero overhead when None,
enabling routes to optionally expose themselves as JSON-RPC methods.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `method` | `Method` | `Method.GET` | Method (method) |
| `path` | `str` | `"/"` | File path |
| `handler_name` | `str` | `""` | Handler name |
| `request_validator` | `str | None` | `None` | Request validator |
| `response_validator` | `str | None` | `None` | Response validator |
| `parameter_validator` | `str | None` | `None` | Parameter validator |
| `file_params` | `str | None` | `None` | File params |
| `is_async` | `bool` | `True` | Whether async |
| `cors` | `CorsConfig | None` | `None` | Cors (cors config) |
| `expects_json_body` | `bool` | `False` | Precomputed flag: true if this route expects a JSON request body Used by middleware to validate Content-Type headers |
| `handler_dependencies` | `list[str]` | `[]` | List of dependency keys this handler requires (for DI) |
| `jsonrpc_method` | `JsonRpcMethodInfo | None` | `None` | Optional JSON-RPC method information When present, this route can be exposed as a JSON-RPC method |

###### Methods

###### default()

**Signature:**

```python
@staticmethod
def default() -> Route
```

###### is_jsonrpc_method()

Check if this route has JSON-RPC metadata

**Signature:**

```python
def is_jsonrpc_method(self) -> bool
```

###### jsonrpc_method_name()

Get the JSON-RPC method name if present

**Signature:**

```python
def jsonrpc_method_name(self) -> str | None
```

---

##### RouteMetadata

Route metadata extracted from bindings

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `method` | `str` | `"GET"` | Method |
| `path` | `str` | `"/"` | File path |
| `handler_name` | `str` | `""` | Handler name |
| `request_schema` | `str | None` | `None` | Request schema |
| `response_schema` | `str | None` | `None` | Response schema |
| `parameter_schema` | `str | None` | `None` | Parameter schema |
| `file_params` | `str | None` | `None` | File params |
| `is_async` | `bool` | `True` | Whether async |
| `cors` | `CorsConfig | None` | `None` | Cors (cors config) |
| `body_param_name` | `str | None` | `None` | Name of the body parameter (defaults to "body" if not specified) |
| `handler_dependencies` | `list[str] | None` | `None` | List of dependency keys this handler requires (for DI) |
| `jsonrpc_method` | `str | None` | `None` | JSON-RPC method metadata (if this route is exposed as a JSON-RPC method) |
| `static_response` | `str | None` | `None` | Optional static response configuration: `{"status": 200, "body": "OK", "content_type": "text/plain"}` When present, the handler is replaced by a `StaticResponseHandler` that bypasses the full middleware pipeline for maximum throughput. |

###### Methods

###### default()

**Signature:**

```python
@staticmethod
def default() -> RouteMetadata
```

---

##### SchemaConfig

Configuration for GraphQL schema building.

Encapsulates all schema-level configuration options including
introspection control, complexity limits, and depth limits.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `True` | Enable introspection queries |
| `complexity_limit` | `int | None` | `None` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `int | None` | `None` | Maximum query depth (None = unlimited) |

###### Methods

###### default()

**Signature:**

```python
@staticmethod
def default() -> SchemaConfig
```

---

##### ServerConfig

Server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `host` | `str` | `"127.0.0.1"` | Host to bind to |
| `port` | `int` | `8000` | Port to bind to |
| `workers` | `int` | `1` | Number of Tokio runtime worker threads used by binding-managed server runtimes |
| `enable_request_id` | `bool` | `False` | Enable request ID generation and propagation |
| `max_body_size` | `int | None` | `None` | Maximum request body size in bytes (None = unlimited, not recommended) |
| `request_timeout` | `int | None` | `None` | Request timeout in seconds (None = no timeout) |
| `compression` | `CompressionConfig | None` | `None` | Enable compression middleware |
| `rate_limit` | `RateLimitConfig | None` | `None` | Enable rate limiting |
| `jwt_auth` | `JwtConfig | None` | `None` | JWT authentication configuration |
| `api_key_auth` | `ApiKeyConfig | None` | `None` | API Key authentication configuration |
| `static_files` | `list[StaticFilesConfig]` | `[]` | Static file serving configuration |
| `graceful_shutdown` | `bool` | `True` | Enable graceful shutdown on SIGTERM/SIGINT |
| `shutdown_timeout` | `int` | `30` | Graceful shutdown timeout (seconds) |
| `openapi` | `OpenApiConfig | None` | `None` | OpenAPI documentation configuration |
| `jsonrpc` | `JsonRpcConfig | None` | `None` | JSON-RPC configuration |
| `grpc` | `GrpcConfig | None` | `None` | gRPC configuration |
| `lifecycle_hooks` | `str | None` | `None` | Lifecycle hooks for request/response processing |
| `background_tasks` | `BackgroundTaskConfig` | — | Background task executor configuration |
| `enable_http_trace` | `bool` | `False` | Enable per-request HTTP tracing (tower-http `TraceLayer`) |
| `di_container` | `str | None` | `None` | Dependency injection container (requires 'di' feature) |

###### Methods

###### default()

**Signature:**

```python
@staticmethod
def default() -> ServerConfig
```

---

##### ServerInfo

Server information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `str` | — | Url |
| `description` | `str | None` | `None` | Human-readable description |

---

##### SseEvent

An individual SSE event

Represents a single Server-Sent Event to be sent to a connected client.
Events can have an optional type, ID, and retry timeout for advanced scenarios.

## SSE Format

Events are serialized to the following text format:

```text
event: event_type
data: {"json":"value"}
id: event-123
retry: 3000
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `event_type` | `str | None` | `None` | Event type (optional) |
| `data` | `str` | — | Event data (JSON value) |
| `id` | `str | None` | `None` | Event ID (optional, for client-side reconnection) |
| `retry` | `int | None` | `None` | Retry timeout in milliseconds (optional) |

### Methods

#### with_id()

Set the event ID for client-side reconnection support

Sets an ID that clients can use to resume from this point if they disconnect.
The client sends this ID back in the `Last-Event-ID` header when reconnecting.

**Signature:**

```python
def with_id(self, id: str) -> SseEvent
```

##### with_retry()

Set the retry timeout for client reconnection

Sets the time in milliseconds clients should wait before attempting to reconnect
if the connection is lost. The client browser will automatically handle reconnection.

**Signature:**

```python
def with_retry(self, retry_ms: int) -> SseEvent
```

---

##### StaticFilesConfig

Static file serving configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `directory` | `str` | — | Directory path to serve |
| `route_prefix` | `str` | — | URL path prefix (e.g., "/static") |
| `index_file` | `bool` | — | Fallback to index.html for directories |
| `cache_control` | `str | None` | `None` | Cache-Control header value |

---

##### UploadFile

Represents an uploaded file from multipart/form-data requests.

This struct provides efficient access to file content with automatic
base64 decoding and implements standard I/O traits for compatibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `filename` | `str` | — | Original filename from the client |
| `content_type` | `str | None` | `None` | MIME type of the uploaded file |
| `size` | `int | None` | `None` | Size of the file in bytes |
| `content` | `bytes` | — | File content (may be base64 encoded) |
| `content_encoding` | `str | None` | `None` | Content encoding type |
| `cursor` | `str` | — | Internal cursor for Read/Seek operations |

###### Methods

###### as_bytes()

Get the raw file content as bytes.

This provides zero-copy access to the underlying buffer.

**Signature:**

```python
def as_bytes(self) -> bytes
```

###### read_to_string()

Read the file content as a UTF-8 string.

**Errors:**

Returns an error if the content is not valid UTF-8.

**Signature:**

```python
def read_to_string(self) -> str
```

###### content_type_or_default()

Get the content type, defaulting to "application/octet-stream".

**Signature:**

```python
def content_type_or_default(self) -> str
```

---

##### ValidatedParams

Validated parameters from request (path, query, headers, cookies)

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `params` | `dict[str, str]` | — | Params |

---

#### Enums

##### Method

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
| `TRACE` | Trace |

---

##### JsonRpcResponseType

JSON-RPC 2.0 Response Type

An enum that represents either a successful response or an error response.
This is useful for untagged deserialization and handling both response types uniformly.

## Variants

- `Success(JsonRpcResponse)` - A successful response with a result
- `Error(JsonRpcErrorResponse)` - An error response with error details

| Value | Description |
|-------|-------------|
| `SUCCESS` | Successful response containing a result — Fields: `0`: `str` |
| `ERROR` | Error response containing error details — Fields: `0`: `str` |

---

### JsonRpcRequestOrBatch

Represents either a single JSON-RPC request or a batch of requests

Used to distinguish between single and batch requests after parsing,
allowing different routing logic for each case.

| Value | Description |
|-------|-------------|
| `SINGLE` | A single JSON-RPC request — Fields: `0`: `str` |
| `BATCH` | A batch (array) of JSON-RPC requests — Fields: `0`: `list[str]` |

---

#### SecuritySchemeInfo

Security scheme types

| Value | Description |
|-------|-------------|
| `HTTP` | Http — Fields: `scheme`: `str`, `bearer_format`: `str` |
| `API_KEY` | Api key — Fields: `location`: `str`, `name`: `str` |

---

#### Errors

##### GraphQlError

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

##### SchemaError

Error type for schema building operations

**Base class:** `SchemaError(Exception)`

| Exception | Description |
|-----------|-------------|
| `BuildingFailed(SchemaError)` | Generic schema building error |
| `ValidationError(SchemaError)` | Configuration validation error |
| `ComplexityLimitExceeded(SchemaError)` | Complexity limit exceeded |
| `DepthLimitExceeded(SchemaError)` | Depth limit exceeded |

---

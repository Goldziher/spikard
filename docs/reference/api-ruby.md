---
title: "Ruby API Reference"
---

## Ruby API Reference <span class="version-badge">v0.14.0</span>

### Functions

#### schema_query_only()

Create a simple schema configuration with only Query type.

This is a convenience function for schemas that only have queries.

**Returns:**

A `QueryOnlyConfig` with default settings

**Signature:**

```ruby
def self.schema_query_only()
```

**Returns:** `QueryOnlyConfig`

---

#### schema_query_mutation()

Create a schema configuration with Query and Mutation types.

This is a convenience function for schemas with queries and mutations but no subscriptions.

**Returns:**

A `QueryMutationConfig` with default settings

**Signature:**

```ruby
def self.schema_query_mutation()
```

**Returns:** `QueryMutationConfig`

---

#### schema_full()

Create a schema configuration with all three root types.

This is a convenience function for fully-featured schemas.

**Returns:**

A `FullSchemaConfig` with default settings

**Signature:**

```ruby
def self.schema_full()
```

**Returns:** `FullSchemaConfig`

---

### Types

#### ApiKeyConfig

API Key authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `keys` | `Array<String>` | — | Valid API keys |
| `header_name` | `String` | — | Header name to check (e.g., "X-API-Key") |

---

#### BackgroundJobMetadata

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `request_id` | `String?` | `nil` | Request id |

##### Methods

###### default()

**Signature:**

```ruby
def self.default()
```

---

#### BackgroundTaskConfig

Configuration for in-process background task execution.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_queue_size` | `Integer` | `1024` | Maximum queue size |
| `max_concurrent_tasks` | `Integer` | `128` | Maximum concurrent tasks |
| `drain_timeout_secs` | `Integer` | `30` | Drain timeout secs |

##### Methods

###### default()

**Signature:**

```ruby
def self.default()
```

---

#### CompressionConfig

Compression configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `gzip` | `Boolean` | `true` | Enable gzip compression |
| `brotli` | `Boolean` | `true` | Enable brotli compression |
| `min_size` | `Integer` | — | Minimum response size to compress (bytes) |
| `quality` | `Integer` | — | Compression quality (0-11 for brotli, 0-9 for gzip) |

##### Methods

###### default()

**Signature:**

```ruby
def self.default()
```

---

#### ContactInfo

Contact information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String?` | `nil` | The name |
| `email` | `String?` | `nil` | Email |
| `url` | `String?` | `nil` | Url |

---

#### CorsConfig

CORS configuration for a route

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `allowed_origins` | `Array<String>` | `[]` | Allowed origins |
| `allowed_methods` | `Array<String>` | `[]` | Allowed methods |
| `allowed_headers` | `Array<String>` | `[]` | Allowed headers |
| `expose_headers` | `Array<String>?` | `nil` | Expose headers |
| `max_age` | `Integer?` | `nil` | Maximum age |
| `allow_credentials` | `Boolean?` | `nil` | Allow credentials |
| `methods_joined_cache` | `String` | — | Methods joined cache |
| `headers_joined_cache` | `String` | — | Headers joined cache |

##### Methods

###### allowed_methods_joined()

Get the cached joined methods string for preflight responses

**Signature:**

```ruby
def allowed_methods_joined()
```

###### allowed_headers_joined()

Get the cached joined headers string for preflight responses

**Signature:**

```ruby
def allowed_headers_joined()
```

###### is_origin_allowed()

Check if an origin is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```ruby
def is_origin_allowed(origin)
```

###### is_method_allowed()

Check if a method is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```ruby
def is_method_allowed(method)
```

###### are_headers_allowed()

Check if all requested headers are allowed (O(n) where n = num requested headers)

**Signature:**

```ruby
def are_headers_allowed(requested)
```

###### default()

**Signature:**

```ruby
def self.default()
```

---

#### FullSchemaConfig

Configuration for fully-featured schemas with Query, Mutation, and Subscription types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `Boolean` | `true` | Enable introspection queries |
| `complexity_limit` | `Integer?` | `nil` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `Integer?` | `nil` | Maximum query depth (None = unlimited) |

##### Methods

###### default()

**Signature:**

```ruby
def self.default()
```

---

#### GraphQlError

##### Methods

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

```ruby
def status_code()
```

---

#### GraphQlRouteConfig

Configuration for GraphQL routes

Provides a builder pattern for configuring GraphQL route parameters
for the Spikard HTTP server's routing system.

##### Methods

###### path()

Set the HTTP path for the GraphQL endpoint

**Signature:**

```ruby
def path(path)
```

###### method()

Set the HTTP method for the GraphQL endpoint

**Signature:**

```ruby
def method(method)
```

###### enable_playground()

Enable or disable the GraphQL Playground UI

**Signature:**

```ruby
def enable_playground(enable)
```

###### description()

Set a custom description for documentation

**Signature:**

```ruby
def description(description)
```

###### get_path()

Get the configured path

**Signature:**

```ruby
def get_path()
```

###### get_method()

Get the configured method

**Signature:**

```ruby
def get_method()
```

###### is_playground_enabled()

Check if playground is enabled

**Signature:**

```ruby
def is_playground_enabled()
```

###### get_description()

Get the description if set

**Signature:**

```ruby
def get_description()
```

###### default()

**Signature:**

```ruby
def self.default()
```

---

#### GrpcConfig

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
| `enabled` | `Boolean` | `true` | Enable gRPC support |
| `max_message_size` | `Integer` | — | Maximum message size in bytes (for both sending and receiving) This limit applies to individual messages in both unary and streaming RPCs. When a single message exceeds this size, the request is rejected with HTTP 413 (Payload Too Large). Default: 4MB (4194304 bytes) **Note:** This limit does NOT apply to the total response size in streaming RPCs. For multi-message streams, the total response can exceed this limit as long as each individual message stays within the limit. |
| `enable_compression` | `Boolean` | `true` | Enable gzip compression for gRPC messages |
| `request_timeout` | `Integer?` | `nil` | Timeout for gRPC requests in seconds (None = no timeout) |
| `max_concurrent_streams` | `Integer` | — | Maximum number of concurrent streams per connection (HTTP/2 advisory) This value is communicated to HTTP/2 clients as the server's flow control limit. The HTTP/2 transport layer enforces this limit automatically via SETTINGS frames and GOAWAY responses. Applications should NOT implement custom enforcement. Default: 100 streams per connection # Stream Limiting Strategy - **Per Connection**: This limit applies per HTTP/2 connection, not globally - **Transport Enforcement**: HTTP/2 handles all stream limiting; applications need not implement custom checks - **Streaming Requests**: In server streaming or bidi streaming, each logical RPC consumes one stream slot. Message ordering within a stream follows HTTP/2 frame ordering. # Future Enhancement A future `max_stream_response_bytes` field may be added to limit the total response size in streaming RPCs (separate from per-message limits). |
| `enable_keepalive` | `Boolean` | `true` | Enable HTTP/2 keepalive |
| `keepalive_interval` | `Integer` | — | HTTP/2 keepalive interval in seconds |
| `keepalive_timeout` | `Integer` | — | HTTP/2 keepalive timeout in seconds |

### Methods

#### default()

**Signature:**

```ruby
def self.default()
```

---

##### JsonRpcConfig

JSON-RPC server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `Boolean` | `true` | Enable JSON-RPC endpoint |
| `endpoint_path` | `String` | — | HTTP endpoint path for JSON-RPC requests (default: "/rpc") |
| `enable_batch` | `Boolean` | — | Enable batch request processing (default: true) |
| `max_batch_size` | `Integer` | — | Maximum number of requests in a batch (default: 100) |

###### Methods

###### default()

**Signature:**

```ruby
def self.default()
```

---

##### JsonRpcMethodInfo

JSON-RPC method metadata for routes that support JSON-RPC

This struct captures the metadata needed to expose HTTP routes as JSON-RPC methods,
enabling discovery and documentation of RPC-compatible endpoints.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `method_name` | `String` | — | The JSON-RPC method name (e.g., "user.create") |
| `description` | `String?` | `nil` | Optional description of what the method does |
| `params_schema` | `String?` | `nil` | Optional JSON Schema for method parameters |
| `result_schema` | `String?` | `nil` | Optional JSON Schema for the result |
| `deprecated` | `Boolean` | — | Whether this method is deprecated |
| `tags` | `Array<String>` | — | Tags for categorizing and grouping methods |

---

##### JwtConfig

JWT authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `secret` | `String` | — | Secret key for JWT verification |
| `algorithm` | `String` | — | Required algorithm (HS256, HS384, HS512, RS256, etc.) |
| `audience` | `Array<String>?` | `nil` | Required audience claim |
| `issuer` | `String?` | `nil` | Required issuer claim |
| `leeway` | `Integer` | — | Leeway for expiration checks (seconds) |

---

##### LicenseInfo

License information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `url` | `String?` | `nil` | Url |

---

##### OpenApiConfig

OpenAPI configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `Boolean` | `false` | Enable OpenAPI generation (default: false for zero overhead) |
| `title` | `String` | `"API"` | API title |
| `version` | `String` | `"1.0.0"` | API version |
| `description` | `String?` | `nil` | API description (supports markdown) |
| `swagger_ui_path` | `String` | — | Path to serve Swagger UI (default: "/docs") |
| `redoc_path` | `String` | — | Path to serve Redoc (default: "/redoc") |
| `openapi_json_path` | `String` | — | Path to serve OpenAPI JSON spec (default: "/openapi.json") |
| `contact` | `ContactInfo?` | `nil` | Contact information |
| `license` | `LicenseInfo?` | `nil` | License information |
| `servers` | `Array<ServerInfo>` | `[]` | Server definitions |
| `security_schemes` | `Hash{String=>SecuritySchemeInfo}` | `{}` | Security schemes (auto-detected from middleware if not provided) |

###### Methods

###### default()

**Signature:**

```ruby
def self.default()
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
| `type_uri` | `String` | — | A URI reference that identifies the problem type. Defaults to "about:blank" when absent. Should be a stable, human-readable identifier for the problem type. |
| `title` | `String` | — | A short, human-readable summary of the problem type. Should not change from occurrence to occurrence of the problem. |
| `status` | `Integer` | — | The HTTP status code generated by the origin server. This is advisory; the actual HTTP status code takes precedence. |
| `detail` | `String?` | `nil` | A human-readable explanation specific to this occurrence of the problem. |
| `instance` | `String?` | `nil` | A URI reference that identifies the specific occurrence of the problem. It may or may not yield further information if dereferenced. |
| `extensions` | `Hash{String=>String}` | — | Extension members - problem-type-specific data. For validation errors, this typically contains an "errors" array. |

### Methods

#### with_detail()

Set the detail field

**Signature:**

```ruby
def with_detail(detail)
```

##### with_instance()

Set the instance field

**Signature:**

```ruby
def with_instance(instance)
```

###### not_found()

Create a not found error

**Signature:**

```ruby
def self.not_found(detail)
```

###### method_not_allowed()

Create a method not allowed error

**Signature:**

```ruby
def self.method_not_allowed(detail)
```

###### internal_server_error()

Create an internal server error

**Signature:**

```ruby
def self.internal_server_error(detail)
```

###### bad_request()

Create a bad request error

**Signature:**

```ruby
def self.bad_request(detail)
```

###### to_json()

Serialize to JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```ruby
def to_json()
```

###### to_json_pretty()

Serialize to pretty JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```ruby
def to_json_pretty()
```

---

##### QueryMutationConfig

Configuration for schemas with Query and Mutation types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `Boolean` | `true` | Enable introspection queries |
| `complexity_limit` | `Integer?` | `nil` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `Integer?` | `nil` | Maximum query depth (None = unlimited) |

###### Methods

###### default()

**Signature:**

```ruby
def self.default()
```

---

##### QueryOnlyConfig

Configuration for schemas with only Query type

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `Boolean` | `true` | Enable introspection queries |
| `complexity_limit` | `Integer?` | `nil` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `Integer?` | `nil` | Maximum query depth (None = unlimited) |

###### Methods

###### default()

**Signature:**

```ruby
def self.default()
```

---

##### RateLimitConfig

Rate limiting configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `per_second` | `Integer` | `100` | Requests per second |
| `burst` | `Integer` | `200` | Burst allowance |
| `ip_based` | `Boolean` | `true` | Use IP-based rate limiting |

###### Methods

###### default()

**Signature:**

```ruby
def self.default()
```

---

##### Response

HTTP Response with custom status code, headers, and content

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String?` | `nil` | Response body content |
| `status_code` | `Integer` | — | HTTP status code (defaults to 200) |
| `headers` | `Hash{String=>String}` | `{}` | Response headers |

###### Methods

###### set_header()

Set a header

**Signature:**

```ruby
def set_header(key, value)
```

###### set_cookie()

Set a cookie in the response

**Signature:**

```ruby
def set_cookie(key, value, max_age, domain, path, secure, http_only, same_site)
```

###### default()

**Signature:**

```ruby
def self.default()
```

---

##### SchemaConfig

Configuration for GraphQL schema building.

Encapsulates all schema-level configuration options including
introspection control, complexity limits, and depth limits.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `Boolean` | `true` | Enable introspection queries |
| `complexity_limit` | `Integer?` | `nil` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `Integer?` | `nil` | Maximum query depth (None = unlimited) |

###### Methods

###### default()

**Signature:**

```ruby
def self.default()
```

---

##### ServerConfig

Server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `host` | `String` | `"127.0.0.1"` | Host to bind to |
| `port` | `Integer` | `8000` | Port to bind to |
| `workers` | `Integer` | `1` | Number of Tokio runtime worker threads used by binding-managed server runtimes |
| `enable_request_id` | `Boolean` | `false` | Enable request ID generation and propagation |
| `max_body_size` | `Integer?` | `nil` | Maximum request body size in bytes (None = unlimited, not recommended) |
| `request_timeout` | `Integer?` | `nil` | Request timeout in seconds (None = no timeout) |
| `compression` | `CompressionConfig?` | `nil` | Enable compression middleware |
| `rate_limit` | `RateLimitConfig?` | `nil` | Enable rate limiting |
| `jwt_auth` | `JwtConfig?` | `nil` | JWT authentication configuration |
| `api_key_auth` | `ApiKeyConfig?` | `nil` | API Key authentication configuration |
| `static_files` | `Array<StaticFilesConfig>` | `[]` | Static file serving configuration |
| `graceful_shutdown` | `Boolean` | `true` | Enable graceful shutdown on SIGTERM/SIGINT |
| `shutdown_timeout` | `Integer` | `30` | Graceful shutdown timeout (seconds) |
| `openapi` | `OpenApiConfig?` | `nil` | OpenAPI documentation configuration |
| `jsonrpc` | `JsonRpcConfig?` | `nil` | JSON-RPC configuration |
| `grpc` | `GrpcConfig?` | `nil` | gRPC configuration |
| `lifecycle_hooks` | `String?` | `nil` | Lifecycle hooks for request/response processing |
| `background_tasks` | `BackgroundTaskConfig` | — | Background task executor configuration |
| `enable_http_trace` | `Boolean` | `false` | Enable per-request HTTP tracing (tower-http `TraceLayer`) |
| `di_container` | `String?` | `nil` | Dependency injection container (requires 'di' feature) |

###### Methods

###### default()

**Signature:**

```ruby
def self.default()
```

---

##### ServerInfo

Server information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | Url |
| `description` | `String?` | `nil` | Human-readable description |

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
| `event_type` | `String?` | `nil` | Event type (optional) |
| `data` | `String` | — | Event data (JSON value) |
| `id` | `String?` | `nil` | Event ID (optional, for client-side reconnection) |
| `retry` | `Integer?` | `nil` | Retry timeout in milliseconds (optional) |

### Methods

#### with_id()

Set the event ID for client-side reconnection support

Sets an ID that clients can use to resume from this point if they disconnect.
The client sends this ID back in the `Last-Event-ID` header when reconnecting.

**Signature:**

```ruby
def with_id(id)
```

##### with_retry()

Set the retry timeout for client reconnection

Sets the time in milliseconds clients should wait before attempting to reconnect
if the connection is lost. The client browser will automatically handle reconnection.

**Signature:**

```ruby
def with_retry(retry_ms)
```

---

##### StaticFilesConfig

Static file serving configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `directory` | `String` | — | Directory path to serve |
| `route_prefix` | `String` | — | URL path prefix (e.g., "/static") |
| `index_file` | `Boolean` | — | Fallback to index.html for directories |
| `cache_control` | `String?` | `nil` | Cache-Control header value |

---

##### UploadFile

Represents an uploaded file from multipart/form-data requests.

This struct provides efficient access to file content with automatic
base64 decoding and implements standard I/O traits for compatibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `filename` | `String` | — | Original filename from the client |
| `content_type` | `String?` | `nil` | MIME type of the uploaded file |
| `size` | `Integer?` | `nil` | Size of the file in bytes |
| `content` | `String` | — | File content (may be base64 encoded) |
| `content_encoding` | `String?` | `nil` | Content encoding type |
| `cursor` | `String` | — | Internal cursor for Read/Seek operations |

###### Methods

###### as_bytes()

Get the raw file content as bytes.

This provides zero-copy access to the underlying buffer.

**Signature:**

```ruby
def as_bytes()
```

###### read_to_string()

Read the file content as a UTF-8 string.

**Errors:**

Returns an error if the content is not valid UTF-8.

**Signature:**

```ruby
def read_to_string()
```

###### content_type_or_default()

Get the content type, defaulting to "application/octet-stream".

**Signature:**

```ruby
def content_type_or_default()
```

---

#### Enums

##### Method

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
| `trace` | Trace |

---

##### SecuritySchemeInfo

Security scheme types

| Value | Description |
|-------|-------------|
| `http` | Http — Fields: `scheme`: `String`, `bearer_format`: `String` |
| `api_key` | Api key — Fields: `location`: `String`, `name`: `String` |

---

#### Errors

##### GraphQlError

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

##### SchemaError

Error type for schema building operations

| Variant | Description |
|---------|-------------|
| `building_failed` | Generic schema building error |
| `validation_error` | Configuration validation error |
| `complexity_limit_exceeded` | Complexity limit exceeded |
| `depth_limit_exceeded` | Depth limit exceeded |

---

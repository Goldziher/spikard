---
title: "C API Reference"
---

## C API Reference <span class="version-badge">v0.14.0</span>

### Functions

#### spikard_schema_query_only()

Create a simple schema configuration with only Query type.

This is a convenience function for schemas that only have queries.

**Returns:**

A `QueryOnlyConfig` with default settings

**Signature:**

```c
SpikardQueryOnlyConfig* spikard_schema_query_only();
```

**Returns:** `SpikardQueryOnlyConfig`

---

#### spikard_schema_query_mutation()

Create a schema configuration with Query and Mutation types.

This is a convenience function for schemas with queries and mutations but no subscriptions.

**Returns:**

A `QueryMutationConfig` with default settings

**Signature:**

```c
SpikardQueryMutationConfig* spikard_schema_query_mutation();
```

**Returns:** `SpikardQueryMutationConfig`

---

#### spikard_schema_full()

Create a schema configuration with all three root types.

This is a convenience function for fully-featured schemas.

**Returns:**

A `FullSchemaConfig` with default settings

**Signature:**

```c
SpikardFullSchemaConfig* spikard_schema_full();
```

**Returns:** `SpikardFullSchemaConfig`

---

### Types

#### SpikardApiKeyConfig

API Key authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `keys` | `const char**` | — | Valid API keys |
| `header_name` | `const char*` | — | Header name to check (e.g., "X-API-Key") |

---

#### SpikardBackgroundJobMetadata

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `const char*` | — | The name |
| `request_id` | `const char**` | `NULL` | Request id |

##### Methods

###### spikard_default()

**Signature:**

```c
SpikardBackgroundJobMetadata spikard_default();
```

---

#### SpikardBackgroundTaskConfig

Configuration for in-process background task execution.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_queue_size` | `uintptr_t` | `1024` | Maximum queue size |
| `max_concurrent_tasks` | `uintptr_t` | `128` | Maximum concurrent tasks |
| `drain_timeout_secs` | `uint64_t` | `30` | Drain timeout secs |

##### Methods

###### spikard_default()

**Signature:**

```c
SpikardBackgroundTaskConfig spikard_default();
```

---

#### SpikardCompressionConfig

Compression configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `gzip` | `bool` | `true` | Enable gzip compression |
| `brotli` | `bool` | `true` | Enable brotli compression |
| `min_size` | `uintptr_t` | — | Minimum response size to compress (bytes) |
| `quality` | `uint32_t` | — | Compression quality (0-11 for brotli, 0-9 for gzip) |

##### Methods

###### spikard_default()

**Signature:**

```c
SpikardCompressionConfig spikard_default();
```

---

#### SpikardContactInfo

Contact information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `const char**` | `NULL` | The name |
| `email` | `const char**` | `NULL` | Email |
| `url` | `const char**` | `NULL` | Url |

---

#### SpikardCorsConfig

CORS configuration for a route

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `allowed_origins` | `const char**` | `NULL` | Allowed origins |
| `allowed_methods` | `const char**` | `NULL` | Allowed methods |
| `allowed_headers` | `const char**` | `NULL` | Allowed headers |
| `expose_headers` | `const char***` | `NULL` | Expose headers |
| `max_age` | `uint32_t*` | `NULL` | Maximum age |
| `allow_credentials` | `bool*` | `NULL` | Allow credentials |
| `methods_joined_cache` | `const char*` | — | Methods joined cache |
| `headers_joined_cache` | `const char*` | — | Headers joined cache |

##### Methods

###### spikard_allowed_methods_joined()

Get the cached joined methods string for preflight responses

**Signature:**

```c
const char* spikard_allowed_methods_joined();
```

###### spikard_allowed_headers_joined()

Get the cached joined headers string for preflight responses

**Signature:**

```c
const char* spikard_allowed_headers_joined();
```

###### spikard_is_origin_allowed()

Check if an origin is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```c
bool spikard_is_origin_allowed(const char* origin);
```

###### spikard_is_method_allowed()

Check if a method is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```c
bool spikard_is_method_allowed(const char* method);
```

###### spikard_are_headers_allowed()

Check if all requested headers are allowed (O(n) where n = num requested headers)

**Signature:**

```c
bool spikard_are_headers_allowed(const char** requested);
```

###### spikard_default()

**Signature:**

```c
SpikardCorsConfig spikard_default();
```

---

#### SpikardFullSchemaConfig

Configuration for fully-featured schemas with Query, Mutation, and Subscription types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `true` | Enable introspection queries |
| `complexity_limit` | `uintptr_t*` | `NULL` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `uintptr_t*` | `NULL` | Maximum query depth (None = unlimited) |

##### Methods

###### spikard_default()

**Signature:**

```c
SpikardFullSchemaConfig spikard_default();
```

---

#### SpikardGraphQlError

##### Methods

###### spikard_status_code()

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

```c
uint16_t spikard_status_code();
```

---

#### SpikardGraphQlRouteConfig

Configuration for GraphQL routes

Provides a builder pattern for configuring GraphQL route parameters
for the Spikard HTTP server's routing system.

##### Methods

###### spikard_path()

Set the HTTP path for the GraphQL endpoint

**Signature:**

```c
SpikardGraphQlRouteConfig spikard_path(const char* path);
```

###### spikard_method()

Set the HTTP method for the GraphQL endpoint

**Signature:**

```c
SpikardGraphQlRouteConfig spikard_method(const char* method);
```

###### spikard_enable_playground()

Enable or disable the GraphQL Playground UI

**Signature:**

```c
SpikardGraphQlRouteConfig spikard_enable_playground(bool enable);
```

###### spikard_description()

Set a custom description for documentation

**Signature:**

```c
SpikardGraphQlRouteConfig spikard_description(const char* description);
```

###### spikard_get_path()

Get the configured path

**Signature:**

```c
const char* spikard_get_path();
```

###### spikard_get_method()

Get the configured method

**Signature:**

```c
const char* spikard_get_method();
```

###### spikard_is_playground_enabled()

Check if playground is enabled

**Signature:**

```c
bool spikard_is_playground_enabled();
```

###### spikard_get_description()

Get the description if set

**Signature:**

```c
const char** spikard_get_description();
```

###### spikard_default()

**Signature:**

```c
SpikardGraphQlRouteConfig spikard_default();
```

---

#### SpikardGrpcConfig

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
| `enabled` | `bool` | `true` | Enable gRPC support |
| `max_message_size` | `uintptr_t` | — | Maximum message size in bytes (for both sending and receiving) This limit applies to individual messages in both unary and streaming RPCs. When a single message exceeds this size, the request is rejected with HTTP 413 (Payload Too Large). Default: 4MB (4194304 bytes) **Note:** This limit does NOT apply to the total response size in streaming RPCs. For multi-message streams, the total response can exceed this limit as long as each individual message stays within the limit. |
| `enable_compression` | `bool` | `true` | Enable gzip compression for gRPC messages |
| `request_timeout` | `uint64_t*` | `NULL` | Timeout for gRPC requests in seconds (None = no timeout) |
| `max_concurrent_streams` | `uint32_t` | — | Maximum number of concurrent streams per connection (HTTP/2 advisory) This value is communicated to HTTP/2 clients as the server's flow control limit. The HTTP/2 transport layer enforces this limit automatically via SETTINGS frames and GOAWAY responses. Applications should NOT implement custom enforcement. Default: 100 streams per connection # Stream Limiting Strategy - **Per Connection**: This limit applies per HTTP/2 connection, not globally - **Transport Enforcement**: HTTP/2 handles all stream limiting; applications need not implement custom checks - **Streaming Requests**: In server streaming or bidi streaming, each logical RPC consumes one stream slot. Message ordering within a stream follows HTTP/2 frame ordering. # Future Enhancement A future `max_stream_response_bytes` field may be added to limit the total response size in streaming RPCs (separate from per-message limits). |
| `enable_keepalive` | `bool` | `true` | Enable HTTP/2 keepalive |
| `keepalive_interval` | `uint64_t` | — | HTTP/2 keepalive interval in seconds |
| `keepalive_timeout` | `uint64_t` | — | HTTP/2 keepalive timeout in seconds |

### Methods

#### spikard_default()

**Signature:**

```c
SpikardGrpcConfig spikard_default();
```

---

##### SpikardJsonRpcConfig

JSON-RPC server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `true` | Enable JSON-RPC endpoint |
| `endpoint_path` | `const char*` | — | HTTP endpoint path for JSON-RPC requests (default: "/rpc") |
| `enable_batch` | `bool` | — | Enable batch request processing (default: true) |
| `max_batch_size` | `uintptr_t` | — | Maximum number of requests in a batch (default: 100) |

###### Methods

###### spikard_default()

**Signature:**

```c
SpikardJsonRpcConfig spikard_default();
```

---

##### SpikardJsonRpcMethodInfo

JSON-RPC method metadata for routes that support JSON-RPC

This struct captures the metadata needed to expose HTTP routes as JSON-RPC methods,
enabling discovery and documentation of RPC-compatible endpoints.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `method_name` | `const char*` | — | The JSON-RPC method name (e.g., "user.create") |
| `description` | `const char**` | `NULL` | Optional description of what the method does |
| `params_schema` | `const char**` | `NULL` | Optional JSON Schema for method parameters |
| `result_schema` | `const char**` | `NULL` | Optional JSON Schema for the result |
| `deprecated` | `bool` | — | Whether this method is deprecated |
| `tags` | `const char**` | — | Tags for categorizing and grouping methods |

---

##### SpikardJwtConfig

JWT authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `secret` | `const char*` | — | Secret key for JWT verification |
| `algorithm` | `const char*` | — | Required algorithm (HS256, HS384, HS512, RS256, etc.) |
| `audience` | `const char***` | `NULL` | Required audience claim |
| `issuer` | `const char**` | `NULL` | Required issuer claim |
| `leeway` | `uint64_t` | — | Leeway for expiration checks (seconds) |

---

##### SpikardLicenseInfo

License information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `const char*` | — | The name |
| `url` | `const char**` | `NULL` | Url |

---

##### SpikardOpenApiConfig

OpenAPI configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `false` | Enable OpenAPI generation (default: false for zero overhead) |
| `title` | `const char*` | `"API"` | API title |
| `version` | `const char*` | `"1.0.0"` | API version |
| `description` | `const char**` | `NULL` | API description (supports markdown) |
| `swagger_ui_path` | `const char*` | — | Path to serve Swagger UI (default: "/docs") |
| `redoc_path` | `const char*` | — | Path to serve Redoc (default: "/redoc") |
| `openapi_json_path` | `const char*` | — | Path to serve OpenAPI JSON spec (default: "/openapi.json") |
| `contact` | `SpikardContactInfo*` | `NULL` | Contact information |
| `license` | `SpikardLicenseInfo*` | `NULL` | License information |
| `servers` | `SpikardServerInfo*` | `NULL` | Server definitions |
| `security_schemes` | `void*` | `NULL` | Security schemes (auto-detected from middleware if not provided) |

###### Methods

###### spikard_default()

**Signature:**

```c
SpikardOpenApiConfig spikard_default();
```

---

##### SpikardProblemDetails

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
| `type_uri` | `const char*` | — | A URI reference that identifies the problem type. Defaults to "about:blank" when absent. Should be a stable, human-readable identifier for the problem type. |
| `title` | `const char*` | — | A short, human-readable summary of the problem type. Should not change from occurrence to occurrence of the problem. |
| `status` | `uint16_t` | — | The HTTP status code generated by the origin server. This is advisory; the actual HTTP status code takes precedence. |
| `detail` | `const char**` | `NULL` | A human-readable explanation specific to this occurrence of the problem. |
| `instance` | `const char**` | `NULL` | A URI reference that identifies the specific occurrence of the problem. It may or may not yield further information if dereferenced. |
| `extensions` | `void*` | — | Extension members - problem-type-specific data. For validation errors, this typically contains an "errors" array. |

### Methods

#### spikard_with_detail()

Set the detail field

**Signature:**

```c
SpikardProblemDetails spikard_with_detail(const char* detail);
```

##### spikard_with_instance()

Set the instance field

**Signature:**

```c
SpikardProblemDetails spikard_with_instance(const char* instance);
```

###### spikard_not_found()

Create a not found error

**Signature:**

```c
SpikardProblemDetails spikard_not_found(const char* detail);
```

###### spikard_method_not_allowed()

Create a method not allowed error

**Signature:**

```c
SpikardProblemDetails spikard_method_not_allowed(const char* detail);
```

###### spikard_internal_server_error()

Create an internal server error

**Signature:**

```c
SpikardProblemDetails spikard_internal_server_error(const char* detail);
```

###### spikard_bad_request()

Create a bad request error

**Signature:**

```c
SpikardProblemDetails spikard_bad_request(const char* detail);
```

###### spikard_to_json()

Serialize to JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```c
const char* spikard_to_json();
```

###### spikard_to_json_pretty()

Serialize to pretty JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```c
const char* spikard_to_json_pretty();
```

---

##### SpikardQueryMutationConfig

Configuration for schemas with Query and Mutation types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `true` | Enable introspection queries |
| `complexity_limit` | `uintptr_t*` | `NULL` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `uintptr_t*` | `NULL` | Maximum query depth (None = unlimited) |

###### Methods

###### spikard_default()

**Signature:**

```c
SpikardQueryMutationConfig spikard_default();
```

---

##### SpikardQueryOnlyConfig

Configuration for schemas with only Query type

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `true` | Enable introspection queries |
| `complexity_limit` | `uintptr_t*` | `NULL` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `uintptr_t*` | `NULL` | Maximum query depth (None = unlimited) |

###### Methods

###### spikard_default()

**Signature:**

```c
SpikardQueryOnlyConfig spikard_default();
```

---

##### SpikardRateLimitConfig

Rate limiting configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `per_second` | `uint64_t` | `100` | Requests per second |
| `burst` | `uint32_t` | `200` | Burst allowance |
| `ip_based` | `bool` | `true` | Use IP-based rate limiting |

###### Methods

###### spikard_default()

**Signature:**

```c
SpikardRateLimitConfig spikard_default();
```

---

##### SpikardResponse

HTTP Response with custom status code, headers, and content

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `const char**` | `NULL` | Response body content |
| `status_code` | `uint16_t` | — | HTTP status code (defaults to 200) |
| `headers` | `void*` | `NULL` | Response headers |

###### Methods

###### spikard_set_header()

Set a header

**Signature:**

```c
void spikard_set_header(const char* key, const char* value);
```

###### spikard_set_cookie()

Set a cookie in the response

**Signature:**

```c
void spikard_set_cookie(const char* key, const char* value, bool secure, bool http_only, int64_t max_age, const char* domain, const char* path, const char* same_site);
```

###### spikard_default()

**Signature:**

```c
SpikardResponse spikard_default();
```

---

##### SpikardSchemaConfig

Configuration for GraphQL schema building.

Encapsulates all schema-level configuration options including
introspection control, complexity limits, and depth limits.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `true` | Enable introspection queries |
| `complexity_limit` | `uintptr_t*` | `NULL` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `uintptr_t*` | `NULL` | Maximum query depth (None = unlimited) |

###### Methods

###### spikard_default()

**Signature:**

```c
SpikardSchemaConfig spikard_default();
```

---

##### SpikardServerConfig

Server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `host` | `const char*` | `"127.0.0.1"` | Host to bind to |
| `port` | `uint16_t` | `8000` | Port to bind to |
| `workers` | `uintptr_t` | `1` | Number of Tokio runtime worker threads used by binding-managed server runtimes |
| `enable_request_id` | `bool` | `false` | Enable request ID generation and propagation |
| `max_body_size` | `uintptr_t*` | `NULL` | Maximum request body size in bytes (None = unlimited, not recommended) |
| `request_timeout` | `uint64_t*` | `NULL` | Request timeout in seconds (None = no timeout) |
| `compression` | `SpikardCompressionConfig*` | `NULL` | Enable compression middleware |
| `rate_limit` | `SpikardRateLimitConfig*` | `NULL` | Enable rate limiting |
| `jwt_auth` | `SpikardJwtConfig*` | `NULL` | JWT authentication configuration |
| `api_key_auth` | `SpikardApiKeyConfig*` | `NULL` | API Key authentication configuration |
| `static_files` | `SpikardStaticFilesConfig*` | `NULL` | Static file serving configuration |
| `graceful_shutdown` | `bool` | `true` | Enable graceful shutdown on SIGTERM/SIGINT |
| `shutdown_timeout` | `uint64_t` | `30` | Graceful shutdown timeout (seconds) |
| `openapi` | `SpikardOpenApiConfig*` | `NULL` | OpenAPI documentation configuration |
| `jsonrpc` | `SpikardJsonRpcConfig*` | `NULL` | JSON-RPC configuration |
| `grpc` | `SpikardGrpcConfig*` | `NULL` | gRPC configuration |
| `lifecycle_hooks` | `const char**` | `NULL` | Lifecycle hooks for request/response processing |
| `background_tasks` | `SpikardBackgroundTaskConfig` | — | Background task executor configuration |
| `enable_http_trace` | `bool` | `false` | Enable per-request HTTP tracing (tower-http `TraceLayer`) |
| `di_container` | `const char**` | `NULL` | Dependency injection container (requires 'di' feature) |

###### Methods

###### spikard_default()

**Signature:**

```c
SpikardServerConfig spikard_default();
```

---

##### SpikardServerInfo

Server information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `const char*` | — | Url |
| `description` | `const char**` | `NULL` | Human-readable description |

---

##### SpikardSseEvent

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
| `event_type` | `const char**` | `NULL` | Event type (optional) |
| `data` | `const char*` | — | Event data (JSON value) |
| `id` | `const char**` | `NULL` | Event ID (optional, for client-side reconnection) |
| `retry` | `uint64_t*` | `NULL` | Retry timeout in milliseconds (optional) |

### Methods

#### spikard_with_id()

Set the event ID for client-side reconnection support

Sets an ID that clients can use to resume from this point if they disconnect.
The client sends this ID back in the `Last-Event-ID` header when reconnecting.

**Signature:**

```c
SpikardSseEvent spikard_with_id(const char* id);
```

##### spikard_with_retry()

Set the retry timeout for client reconnection

Sets the time in milliseconds clients should wait before attempting to reconnect
if the connection is lost. The client browser will automatically handle reconnection.

**Signature:**

```c
SpikardSseEvent spikard_with_retry(uint64_t retry_ms);
```

---

##### SpikardStaticFilesConfig

Static file serving configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `directory` | `const char*` | — | Directory path to serve |
| `route_prefix` | `const char*` | — | URL path prefix (e.g., "/static") |
| `index_file` | `bool` | — | Fallback to index.html for directories |
| `cache_control` | `const char**` | `NULL` | Cache-Control header value |

---

##### SpikardUploadFile

Represents an uploaded file from multipart/form-data requests.

This struct provides efficient access to file content with automatic
base64 decoding and implements standard I/O traits for compatibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `filename` | `const char*` | — | Original filename from the client |
| `content_type` | `const char**` | `NULL` | MIME type of the uploaded file |
| `size` | `uintptr_t*` | `NULL` | Size of the file in bytes |
| `content` | `const uint8_t*` | — | File content (may be base64 encoded) |
| `content_encoding` | `const char**` | `NULL` | Content encoding type |
| `cursor` | `const char*` | — | Internal cursor for Read/Seek operations |

###### Methods

###### spikard_as_bytes()

Get the raw file content as bytes.

This provides zero-copy access to the underlying buffer.

**Signature:**

```c
const uint8_t* spikard_as_bytes();
```

###### spikard_read_to_string()

Read the file content as a UTF-8 string.

**Errors:**

Returns an error if the content is not valid UTF-8.

**Signature:**

```c
const char* spikard_read_to_string();
```

###### spikard_content_type_or_default()

Get the content type, defaulting to "application/octet-stream".

**Signature:**

```c
const char* spikard_content_type_or_default();
```

---

#### Enums

##### SpikardMethod

HTTP method

| Value | Description |
|-------|-------------|
| `SPIKARD_GET` | Get |
| `SPIKARD_POST` | Post |
| `SPIKARD_PUT` | Put |
| `SPIKARD_PATCH` | Patch |
| `SPIKARD_DELETE` | Delete |
| `SPIKARD_HEAD` | Head |
| `SPIKARD_OPTIONS` | Options |
| `SPIKARD_TRACE` | Trace |

---

##### SpikardSecuritySchemeInfo

Security scheme types

| Value | Description |
|-------|-------------|
| `SPIKARD_HTTP` | Http — Fields: `scheme`: `const char*`, `bearer_format`: `const char*` |
| `SPIKARD_API_KEY` | Api key — Fields: `location`: `const char*`, `name`: `const char*` |

---

#### Errors

##### SpikardGraphQlError

Errors that can occur during GraphQL operations

These errors are compatible with async-graphql error handling and can be
converted to structured HTTP responses matching the project's error fixtures.

| Variant | Description |
|---------|-------------|
| `SPIKARD_EXECUTION_ERROR` | Error during schema execution Occurs when the GraphQL executor encounters a runtime error during query execution. |
| `SPIKARD_SCHEMA_BUILD_ERROR` | Error during schema building Occurs when schema construction fails due to invalid definitions or conflicts. |
| `SPIKARD_REQUEST_HANDLING_ERROR` | Error during request handling Occurs when the HTTP request cannot be properly handled or parsed. |
| `SPIKARD_SERIALIZATION_ERROR` | Serialization error Occurs during JSON serialization/deserialization of GraphQL values. |
| `SPIKARD_JSON_ERROR` | JSON parsing error Occurs when JSON input cannot be parsed. |
| `SPIKARD_VALIDATION_ERROR` | GraphQL validation error Occurs when a GraphQL query fails schema validation. |
| `SPIKARD_PARSE_ERROR` | GraphQL parse error Occurs when the GraphQL query string cannot be parsed. |
| `SPIKARD_AUTHENTICATION_ERROR` | Authentication error Occurs when request authentication fails. |
| `SPIKARD_AUTHORIZATION_ERROR` | Authorization error Occurs when user lacks required permissions. |
| `SPIKARD_NOT_FOUND` | Not found error Occurs when a requested resource is not found. |
| `SPIKARD_RATE_LIMIT_EXCEEDED` | Rate limit error Occurs when rate limit is exceeded. |
| `SPIKARD_INVALID_INPUT` | Invalid input error with validation details Occurs during input validation with detailed error information. |
| `SPIKARD_COMPLEXITY_LIMIT_EXCEEDED` | Query complexity limit exceeded Occurs when a GraphQL query exceeds the configured complexity limit. |
| `SPIKARD_DEPTH_LIMIT_EXCEEDED` | Query depth limit exceeded Occurs when a GraphQL query exceeds the configured depth limit. |
| `SPIKARD_INTERNAL_ERROR` | Internal server error Occurs when an unexpected internal error happens. |

---

##### SpikardSchemaError

Error type for schema building operations

| Variant | Description |
|---------|-------------|
| `SPIKARD_BUILDING_FAILED` | Generic schema building error |
| `SPIKARD_VALIDATION_ERROR` | Configuration validation error |
| `SPIKARD_COMPLEXITY_LIMIT_EXCEEDED` | Complexity limit exceeded |
| `SPIKARD_DEPTH_LIMIT_EXCEEDED` | Depth limit exceeded |

---

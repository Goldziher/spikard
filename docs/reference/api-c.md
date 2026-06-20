---
title: "C API Reference"
---

## C API Reference <span class="version-badge">v0.16.0-rc.2</span>

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

**Example:**

```c
SpikardQueryOnlyConfig *result = spikard_schema_query_only();
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

**Example:**

```c
SpikardQueryMutationConfig *result = spikard_schema_query_mutation();
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

**Example:**

```c
SpikardFullSchemaConfig *result = spikard_schema_full();
```

**Returns:** `SpikardFullSchemaConfig`

---

### Types

#### SpikardApiKeyConfig

API Key authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `keys` | `const char**` | — | Valid API keys |
| `header_name` | `const char*` | `serde(default = "default_api_key_header")` | Header name to check (e.g., "X-API-Key") |

---

#### SpikardAsyncApiConfig

AsyncAPI HTTP endpoint configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | — | Enable AsyncAPI endpoints (default: false) |
| `spec` | `void**` | `NULL` | Pre-registered AsyncAPI spec to serve from GET /asyncapi.json |

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

**Example:**

```c
SpikardBackgroundJobMetadata *result = spikard_default();
```

**Returns:** `SpikardBackgroundJobMetadata`

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

**Example:**

```c
SpikardBackgroundTaskConfig *result = spikard_default();
```

**Returns:** `SpikardBackgroundTaskConfig`

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

**Example:**

```c
SpikardCompressionConfig *result = spikard_default();
```

**Returns:** `SpikardCompressionConfig`

---

#### SpikardContactInfo

Contact information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `const char**` | `NULL` | Name of the contact person or organisation. |
| `email` | `const char**` | `NULL` | Contact email address. |
| `url` | `const char**` | `NULL` | URL pointing to the contact information page. |

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

##### Methods

###### spikard_allowed_methods_joined()

Get the cached joined methods string for preflight responses

**Signature:**

```c
const char* spikard_allowed_methods_joined();
```

**Example:**

```c
const char *result = spikard_allowed_methods_joined(instance);
```

**Returns:** `const char*`

###### spikard_allowed_headers_joined()

Get the cached joined headers string for preflight responses

**Signature:**

```c
const char* spikard_allowed_headers_joined();
```

**Example:**

```c
const char *result = spikard_allowed_headers_joined(instance);
```

**Returns:** `const char*`

###### spikard_is_origin_allowed()

Check if an origin is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```c
bool spikard_is_origin_allowed(const char* origin);
```

**Example:**

```c
bool result = spikard_is_origin_allowed(instance, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `origin` | `const char*` | Yes | The origin |

**Returns:** `bool`

###### spikard_is_method_allowed()

Check if a method is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```c
bool spikard_is_method_allowed(const char* method);
```

**Example:**

```c
bool result = spikard_is_method_allowed(instance, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `method` | `const char*` | Yes | The method |

**Returns:** `bool`

###### spikard_default()

**Signature:**

```c
SpikardCorsConfig spikard_default();
```

**Example:**

```c
SpikardCorsConfig *result = spikard_default();
```

**Returns:** `SpikardCorsConfig`

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

**Example:**

```c
SpikardFullSchemaConfig *result = spikard_default();
```

**Returns:** `SpikardFullSchemaConfig`

---

#### SpikardGraphQlRouteConfig

Configuration for GraphQL routes

Provides a builder pattern for configuring GraphQL route parameters
for the Spikard HTTP server's routing system.

##### Methods

###### spikard_new()

Create a new GraphQL route configuration with defaults

Default values:

- path: "/graphql"
- method: "POST"
- `enable_playground`: false

**Signature:**

```c
SpikardGraphQlRouteConfig spikard_new();
```

**Example:**

```c
SpikardGraphQlRouteConfig *result = spikard_new();
```

**Returns:** `SpikardGraphQlRouteConfig`

###### spikard_path()

Set the HTTP path for the GraphQL endpoint

**Signature:**

```c
SpikardGraphQlRouteConfig spikard_path(const char* path);
```

**Example:**

```c
SpikardGraphQlRouteConfig *result = spikard_path(instance, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `path` | `const char*` | Yes | The URL path (e.g., "/graphql", "/api/graphql") |

**Returns:** `SpikardGraphQlRouteConfig`

###### spikard_method()

Set the HTTP method for the GraphQL endpoint

**Signature:**

```c
SpikardGraphQlRouteConfig spikard_method(const char* method);
```

**Example:**

```c
SpikardGraphQlRouteConfig *result = spikard_method(instance, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `method` | `const char*` | Yes | The HTTP method (typically "POST") |

**Returns:** `SpikardGraphQlRouteConfig`

###### spikard_enable_playground()

Enable or disable the GraphQL Playground UI

**Signature:**

```c
SpikardGraphQlRouteConfig spikard_enable_playground(bool enable);
```

**Example:**

```c
SpikardGraphQlRouteConfig *result = spikard_enable_playground(instance, true);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `enable` | `bool` | Yes | Whether to enable playground |

**Returns:** `SpikardGraphQlRouteConfig`

###### spikard_description()

Set a custom description for documentation

**Signature:**

```c
SpikardGraphQlRouteConfig spikard_description(const char* description);
```

**Example:**

```c
SpikardGraphQlRouteConfig *result = spikard_description(instance, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `description` | `const char*` | Yes | Documentation string |

**Returns:** `SpikardGraphQlRouteConfig`

###### spikard_get_path()

Get the configured path

**Signature:**

```c
const char* spikard_get_path();
```

**Example:**

```c
const char *result = spikard_get_path(instance);
```

**Returns:** `const char*`

###### spikard_get_method()

Get the configured method

**Signature:**

```c
const char* spikard_get_method();
```

**Example:**

```c
const char *result = spikard_get_method(instance);
```

**Returns:** `const char*`

###### spikard_is_playground_enabled()

Check if playground is enabled

**Signature:**

```c
bool spikard_is_playground_enabled();
```

**Example:**

```c
bool result = spikard_is_playground_enabled(instance);
```

**Returns:** `bool`

###### spikard_get_description()

Get the description if set

**Signature:**

```c
const char** spikard_get_description();
```

**Example:**

```c
const char** result = spikard_get_description(instance);
```

**Returns:** `const char**`

###### spikard_default()

**Signature:**

```c
SpikardGraphQlRouteConfig spikard_default();
```

**Example:**

```c
SpikardGraphQlRouteConfig *result = spikard_default();
```

**Returns:** `SpikardGraphQlRouteConfig`

---

#### SpikardGrpcConfig

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
  `tonic.Status.resource_exhausted`. Defaults to `NULL` (unbounded).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `true` | Enable gRPC support |
| `max_message_size` | `uintptr_t` | — | Maximum message size in bytes (for both sending and receiving) This limit applies to individual messages in both unary and streaming RPCs. When a single message exceeds this size, the request is rejected with HTTP 413 (Payload Too Large). Default: 4MB (4194304 bytes) **Note:** This limit does NOT apply to the total response size in streaming RPCs. For multi-message streams, the total response can exceed this limit as long as each individual message stays within the limit. |
| `enable_compression` | `bool` | `true` | Enable gzip compression for gRPC messages |
| `request_timeout` | `uint64_t*` | `NULL` | Timeout for gRPC requests in seconds (None = no timeout) |
| `max_concurrent_streams` | `uint32_t` | — | Maximum number of concurrent streams per connection (HTTP/2 advisory) This value is communicated to HTTP/2 clients as the server's flow control limit. The HTTP/2 transport layer enforces this limit automatically via SETTINGS frames and GOAWAY responses. Applications should NOT implement custom enforcement. Default: 100 streams per connection # Stream Limiting Strategy - **Per Connection**: This limit applies per HTTP/2 connection, not globally - **Transport Enforcement**: HTTP/2 handles all stream limiting; applications need not implement custom checks - **Streaming Requests**: In server streaming or bidi streaming, each logical RPC consumes one stream slot. Message ordering within a stream follows HTTP/2 frame ordering. |
| `enable_keepalive` | `bool` | `true` | Enable HTTP/2 keepalive |
| `keepalive_interval` | `uint64_t` | — | HTTP/2 keepalive interval in seconds |
| `keepalive_timeout` | `uint64_t` | — | HTTP/2 keepalive timeout in seconds |
| `max_stream_response_bytes` | `uintptr_t*` | `NULL` | Total byte cap across an entire streaming response. When `Some(n)`, the streaming adapter aborts the stream with `tonic.Status.resource_exhausted` once the cumulative encoded message bytes exceed `n`. The stream yields the error item and then terminates. Per-message cap remains `max_message_size`. This limit applies to server-streaming and bidirectional-streaming RPCs only; unary RPCs are governed solely by `max_message_size`. Default: `NULL` (unbounded total response size). |

##### Methods

###### spikard_default()

**Signature:**

```c
SpikardGrpcConfig spikard_default();
```

**Example:**

```c
SpikardGrpcConfig *result = spikard_default();
```

**Returns:** `SpikardGrpcConfig`

---

#### SpikardIntoHandler

Convert user-facing handler functions into the low-level `Handler` trait.

##### Methods

###### spikard_into_handler()

Convert this value into a shared request handler.

**Signature:**

```c
SpikardHandler spikard_into_handler();
```

**Example:**

```c
SpikardHandler *result = spikard_into_handler(instance);
```

**Returns:** `SpikardHandler`

---

#### SpikardJsonRpcConfig

JSON-RPC server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `true` | Enable JSON-RPC endpoint |
| `endpoint_path` | `const char*` | — | HTTP endpoint path for JSON-RPC requests (default: "/rpc") |
| `enable_batch` | `bool` | — | Enable batch request processing (default: true) |
| `max_batch_size` | `uintptr_t` | — | Maximum number of requests in a batch (default: 100) |

##### Methods

###### spikard_default()

**Signature:**

```c
SpikardJsonRpcConfig spikard_default();
```

**Example:**

```c
SpikardJsonRpcConfig *result = spikard_default();
```

**Returns:** `SpikardJsonRpcConfig`

---

#### SpikardJsonRpcMethodInfo

JSON-RPC method metadata for routes that support JSON-RPC

This struct captures the metadata needed to expose HTTP routes as JSON-RPC methods,
enabling discovery and documentation of RPC-compatible endpoints.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `method_name` | `const char*` | — | The JSON-RPC method name (e.g., "user.create") |
| `description` | `const char**` | `NULL` | Optional description of what the method does |
| `params_schema` | `void**` | `NULL` | Optional JSON Schema for method parameters |
| `result_schema` | `void**` | `NULL` | Optional JSON Schema for the result |
| `deprecated` | `bool` | `/* serde(default) */` | Whether this method is deprecated |
| `tags` | `const char**` | `/* serde(default) */` | Tags for categorizing and grouping methods |

---

#### SpikardJwtConfig

JWT authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `secret` | `const char*` | — | Secret key for JWT verification |
| `algorithm` | `const char*` | `serde(default = "default_jwt_algorithm")` | Required algorithm (HS256, HS384, HS512, RS256, etc.) |
| `audience` | `const char***` | `NULL` | Required audience claim |
| `issuer` | `const char**` | `NULL` | Required issuer claim |
| `leeway` | `uint64_t` | `/* serde(default) */` | Leeway for expiration checks (seconds) |

---

#### SpikardLicenseInfo

License information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `const char*` | — | SPDX license identifier or display name (e.g. `"MIT"`). |
| `url` | `const char**` | `NULL` | URL to the full license text. |

---

#### SpikardOpenApiConfig

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

##### Methods

###### spikard_default()

**Signature:**

```c
SpikardOpenApiConfig spikard_default();
```

**Example:**

```c
SpikardOpenApiConfig *result = spikard_default();
```

**Returns:** `SpikardOpenApiConfig`

---

#### SpikardParseRequest

Request body for `POST /asyncapi/parse`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `spec` | `void*` | — | Spec |

---

#### SpikardParseResult

Full parse result returned by `POST /asyncapi/parse`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `spec_version` | `const char*` | — | Spec version |
| `title` | `const char*` | — | Title |
| `api_version` | `const char*` | — | Api version |
| `channels` | `SpikardParsedChannel*` | — | Channels |
| `operations` | `SpikardParsedOperation*` | — | Operations |
| `messages` | `SpikardParsedMessage*` | — | Messages |

---

#### SpikardParsedChannel

A single channel extracted from an AsyncAPI spec

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `const char*` | — | Channel key from the spec (e.g. "chat/messages") |
| `address` | `const char*` | — | Channel address / path |
| `messages` | `const char**` | — | Message names declared on this channel |
| `bindings` | `void**` | `NULL` | Bindings (ws / http / amqp / …) as raw JSON for forward-compatibility |

---

#### SpikardParsedMessage

A resolved message (name + JSON Schema)

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `const char*` | — | Message name |
| `schema` | `void**` | `NULL` | Resolved JSON Schema for the message payload, if available |

---

#### SpikardParsedOperation

A single operation extracted from an AsyncAPI spec

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `const char*` | — | Operation name |
| `action` | `const char*` | — | Operation action: "send" or "receive" |
| `channel` | `const char*` | — | Channel reference (resolved to the channel name) |

---

#### SpikardProblemDetails

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
| `type_uri` | `const char*` | — | A URI reference that identifies the problem type. Defaults to "about:blank" when absent. Should be a stable, human-readable identifier for the problem type. |
| `title` | `const char*` | — | A short, human-readable summary of the problem type. Should not change from occurrence to occurrence of the problem. |
| `status` | `uint16_t` | — | The HTTP status code generated by the origin server. This is advisory; the actual HTTP status code takes precedence. |
| `detail` | `const char**` | `NULL` | A human-readable explanation specific to this occurrence of the problem. |
| `instance` | `const char**` | `NULL` | A URI reference that identifies the specific occurrence of the problem. It may or may not yield further information if dereferenced. |
| `extensions` | `void*` | — | Extension members - problem-type-specific data. For validation errors, this typically contains an "errors" array. |

##### Methods

###### spikard_with_detail()

Set the detail field

**Signature:**

```c
SpikardProblemDetails spikard_with_detail(const char* detail);
```

**Example:**

```c
SpikardProblemDetails *result = spikard_with_detail(instance, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `detail` | `const char*` | Yes | The detail |

**Returns:** `SpikardProblemDetails`

###### spikard_with_instance()

Set the instance field

**Signature:**

```c
SpikardProblemDetails spikard_with_instance(const char* instance);
```

**Example:**

```c
SpikardProblemDetails *result = spikard_with_instance(instance, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `instance` | `const char*` | Yes | The instance |

**Returns:** `SpikardProblemDetails`

###### spikard_not_found()

Create a not found error

**Signature:**

```c
SpikardProblemDetails spikard_not_found(const char* detail);
```

**Example:**

```c
SpikardProblemDetails *result = spikard_not_found("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `detail` | `const char*` | Yes | The detail |

**Returns:** `SpikardProblemDetails`

###### spikard_method_not_allowed()

Create a method not allowed error

**Signature:**

```c
SpikardProblemDetails spikard_method_not_allowed(const char* detail);
```

**Example:**

```c
SpikardProblemDetails *result = spikard_method_not_allowed("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `detail` | `const char*` | Yes | The detail |

**Returns:** `SpikardProblemDetails`

###### spikard_internal_server_error()

Create an internal server error

**Signature:**

```c
SpikardProblemDetails spikard_internal_server_error(const char* detail);
```

**Example:**

```c
SpikardProblemDetails *result = spikard_internal_server_error("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `detail` | `const char*` | Yes | The detail |

**Returns:** `SpikardProblemDetails`

###### spikard_bad_request()

Create a bad request error

**Signature:**

```c
SpikardProblemDetails spikard_bad_request(const char* detail);
```

**Example:**

```c
SpikardProblemDetails *result = spikard_bad_request("value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `detail` | `const char*` | Yes | The detail |

**Returns:** `SpikardProblemDetails`

###### spikard_to_json()

Serialize to JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```c
const char* spikard_to_json();
```

**Example:**

```c
const char *result = spikard_to_json(instance);
```

**Returns:** `const char*`

**Errors:** Returns `NULL` on error.

###### spikard_to_json_pretty()

Serialize to pretty JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```c
const char* spikard_to_json_pretty();
```

**Example:**

```c
const char *result = spikard_to_json_pretty(instance);
```

**Returns:** `const char*`

**Errors:** Returns `NULL` on error.

---

#### SpikardQueryMutationConfig

Configuration for schemas with Query and Mutation types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `true` | Enable introspection queries |
| `complexity_limit` | `uintptr_t*` | `NULL` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `uintptr_t*` | `NULL` | Maximum query depth (None = unlimited) |

##### Methods

###### spikard_default()

**Signature:**

```c
SpikardQueryMutationConfig spikard_default();
```

**Example:**

```c
SpikardQueryMutationConfig *result = spikard_default();
```

**Returns:** `SpikardQueryMutationConfig`

---

#### SpikardQueryOnlyConfig

Configuration for schemas with only Query type

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `true` | Enable introspection queries |
| `complexity_limit` | `uintptr_t*` | `NULL` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `uintptr_t*` | `NULL` | Maximum query depth (None = unlimited) |

##### Methods

###### spikard_default()

**Signature:**

```c
SpikardQueryOnlyConfig spikard_default();
```

**Example:**

```c
SpikardQueryOnlyConfig *result = spikard_default();
```

**Returns:** `SpikardQueryOnlyConfig`

---

#### SpikardRateLimitConfig

Rate limiting configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `per_second` | `uint64_t` | `100` | Requests per second |
| `burst` | `uint32_t` | `200` | Burst allowance |
| `ip_based` | `bool` | `true` | Use IP-based rate limiting |

##### Methods

###### spikard_default()

**Signature:**

```c
SpikardRateLimitConfig spikard_default();
```

**Example:**

```c
SpikardRateLimitConfig *result = spikard_default();
```

**Returns:** `SpikardRateLimitConfig`

---

#### SpikardRequest

---

#### SpikardResponse

HTTP Response with custom status code, headers, and content

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `void**` | `NULL` | Response body content |
| `status_code` | `uint16_t` | — | HTTP status code (defaults to 200) |
| `headers` | `void*` | `NULL` | Response headers |

##### Methods

###### spikard_set_header()

Set a header

**Signature:**

```c
void spikard_set_header(const char* key, const char* value);
```

**Example:**

```c
spikard_set_header(instance, "value", "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `key` | `const char*` | Yes | The key |
| `value` | `const char*` | Yes | The value |

**Returns:** No return value.

###### spikard_set_cookie()

Set a cookie in the response

**Signature:**

```c
void spikard_set_cookie(const char* key, const char* value, bool secure, bool http_only, int64_t max_age, const char* domain, const char* path, const char* same_site);
```

**Example:**

```c
spikard_set_cookie(instance, "value", "value", true, true, 42, "value", "value", "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `key` | `const char*` | Yes | The key |
| `value` | `const char*` | Yes | The value |
| `secure` | `bool` | Yes | The secure |
| `http_only` | `bool` | Yes | The http only |
| `max_age` | `int64_t*` | No | The max age |
| `domain` | `const char**` | No | The domain |
| `path` | `const char**` | No | Path to the file |
| `same_site` | `const char**` | No | The same site |

**Returns:** No return value.

###### spikard_default()

**Signature:**

```c
SpikardResponse spikard_default();
```

**Example:**

```c
SpikardResponse *result = spikard_default();
```

**Returns:** `SpikardResponse`

---

#### SpikardRouteBuilder

Builder for defining a route.

##### Methods

###### spikard_new()

Create a new builder for the provided HTTP method and path.

**Signature:**

```c
SpikardRouteBuilder spikard_new(SpikardMethod method, const char* path);
```

**Example:**

```c
SpikardRouteBuilder *result = spikard_new((SpikardMethod){0}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `method` | `SpikardMethod` | Yes | The method |
| `path` | `const char*` | Yes | Path to the file |

**Returns:** `SpikardRouteBuilder`

###### spikard_handler_name()

Assign an explicit handler name.

**Signature:**

```c
SpikardRouteBuilder spikard_handler_name(const char* name);
```

**Example:**

```c
SpikardRouteBuilder *result = spikard_handler_name(instance, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `const char*` | Yes | The name |

**Returns:** `SpikardRouteBuilder`

###### spikard_request_schema_json()

Provide a raw JSON schema for the request body.

**Signature:**

```c
SpikardRouteBuilder spikard_request_schema_json(void* schema);
```

**Example:**

```c
SpikardRouteBuilder *result = spikard_request_schema_json(instance, NULL);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `schema` | `void*` | Yes | The schema |

**Returns:** `SpikardRouteBuilder`

###### spikard_response_schema_json()

Provide a raw JSON schema for the response body.

**Signature:**

```c
SpikardRouteBuilder spikard_response_schema_json(void* schema);
```

**Example:**

```c
SpikardRouteBuilder *result = spikard_response_schema_json(instance, NULL);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `schema` | `void*` | Yes | The schema |

**Returns:** `SpikardRouteBuilder`

###### spikard_params_schema_json()

Provide a raw JSON schema for request parameters.

**Signature:**

```c
SpikardRouteBuilder spikard_params_schema_json(void* schema);
```

**Example:**

```c
SpikardRouteBuilder *result = spikard_params_schema_json(instance, NULL);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `schema` | `void*` | Yes | The schema |

**Returns:** `SpikardRouteBuilder`

###### spikard_file_params_json()

Provide multipart file parameter configuration.

**Signature:**

```c
SpikardRouteBuilder spikard_file_params_json(void* schema);
```

**Example:**

```c
SpikardRouteBuilder *result = spikard_file_params_json(instance, NULL);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `schema` | `void*` | Yes | The schema |

**Returns:** `SpikardRouteBuilder`

###### spikard_cors()

Attach a CORS configuration for this route.

**Signature:**

```c
SpikardRouteBuilder spikard_cors(SpikardCorsConfig cors);
```

**Example:**

```c
SpikardRouteBuilder *result = spikard_cors(instance, (SpikardCorsConfig){0});
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `cors` | `SpikardCorsConfig` | Yes | The cors config |

**Returns:** `SpikardRouteBuilder`

###### spikard_compression()

Attach a compression configuration for this route.

**Signature:**

```c
SpikardRouteBuilder spikard_compression(SpikardCompressionConfig compression);
```

**Example:**

```c
SpikardRouteBuilder *result = spikard_compression(instance, (SpikardCompressionConfig){0});
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `compression` | `SpikardCompressionConfig` | Yes | The compression config |

**Returns:** `SpikardRouteBuilder`

###### spikard_sync()

Mark the route as synchronous.

**Signature:**

```c
SpikardRouteBuilder spikard_sync();
```

**Example:**

```c
SpikardRouteBuilder *result = spikard_sync(instance);
```

**Returns:** `SpikardRouteBuilder`

###### spikard_handler_dependencies()

Declare the dependency keys that must be resolved before this handler runs.

**Signature:**

```c
SpikardRouteBuilder spikard_handler_dependencies(const char** dependencies);
```

**Example:**

```c
SpikardRouteBuilder *result = spikard_handler_dependencies(instance, NULL);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `dependencies` | `const char**` | Yes | The dependencies |

**Returns:** `SpikardRouteBuilder`

---

#### SpikardSchemaConfig

Configuration for GraphQL schema building.

Encapsulates all schema-level configuration options including
introspection control, complexity limits, and depth limits.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `true` | Enable introspection queries |
| `complexity_limit` | `uintptr_t*` | `NULL` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `uintptr_t*` | `NULL` | Maximum query depth (None = unlimited) |

##### Methods

###### spikard_default()

**Signature:**

```c
SpikardSchemaConfig spikard_default();
```

**Example:**

```c
SpikardSchemaConfig *result = spikard_default();
```

**Returns:** `SpikardSchemaConfig`

---

#### SpikardServerConfig

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
| `asyncapi` | `SpikardAsyncApiConfig*` | `NULL` | AsyncAPI HTTP endpoint configuration |
| `openapi` | `SpikardOpenApiConfig*` | `NULL` | OpenAPI documentation configuration |
| `jsonrpc` | `SpikardJsonRpcConfig*` | `NULL` | JSON-RPC configuration |
| `grpc` | `SpikardGrpcConfig*` | `NULL` | gRPC configuration |
| `background_tasks` | `SpikardBackgroundTaskConfig` | — | Background task executor configuration |
| `enable_http_trace` | `bool` | `false` | Enable per-request HTTP tracing (tower-http `TraceLayer`) |

##### Methods

###### spikard_default()

**Signature:**

```c
SpikardServerConfig spikard_default();
```

**Example:**

```c
SpikardServerConfig *result = spikard_default();
```

**Returns:** `SpikardServerConfig`

---

#### SpikardServerInfo

Server information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `const char*` | — | Base URL of the server (e.g. `"<https://api.example.com/v1"`>). |
| `description` | `const char**` | `NULL` | Optional human-readable description of the server environment. |

---

#### SpikardSseEvent

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
| `event_type` | `const char**` | `NULL` | Event type (optional) |
| `data` | `void*` | — | Event data (JSON value) |
| `id` | `const char**` | `NULL` | Event ID (optional, for client-side reconnection) |
| `retry` | `uint64_t*` | `NULL` | Retry timeout in milliseconds (optional) |

##### Methods

###### spikard_with_id()

Set the event ID for client-side reconnection support

Sets an ID that clients can use to resume from this point if they disconnect.
The client sends this ID back in the `Last-Event-ID` header when reconnecting.

**Signature:**

```c
SpikardSseEvent spikard_with_id(const char* id);
```

**Example:**

```c
SpikardSseEvent *result = spikard_with_id(instance, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `const char*` | Yes | Unique identifier for this event |

**Returns:** `SpikardSseEvent`

###### spikard_with_retry()

Set the retry timeout for client reconnection

Sets the time in milliseconds clients should wait before attempting to reconnect
if the connection is lost. The client browser will automatically handle reconnection.

**Signature:**

```c
SpikardSseEvent spikard_with_retry(uint64_t retry_ms);
```

**Example:**

```c
SpikardSseEvent *result = spikard_with_retry(instance, 42);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `retry_ms` | `uint64_t` | Yes | Retry timeout in milliseconds |

**Returns:** `SpikardSseEvent`

---

#### SpikardStaticFilesConfig

Static file serving configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `directory` | `const char*` | — | Directory path to serve |
| `route_prefix` | `const char*` | — | URL path prefix (e.g., "/static") |
| `index_file` | `bool` | `serde(default = "default_true")` | Fallback to index.html for directories |
| `cache_control` | `const char**` | `NULL` | Cache-Control header value |

---

#### SpikardTestingSseEvent

A single Server-Sent Event.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data` | `const char*` | — | The data field of the event. |

---

#### SpikardUploadFile

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

##### Methods

###### spikard_as_bytes()

Get the raw file content as bytes.

This provides zero-copy access to the underlying buffer.

**Signature:**

```c
const uint8_t* spikard_as_bytes();
```

**Example:**

```c
const uint8_t *result = spikard_as_bytes(instance);
```

**Returns:** `const uint8_t*`

###### spikard_read_to_string()

Read the file content as a UTF-8 string.

**Errors:**

Returns an error if the content is not valid UTF-8.

**Signature:**

```c
const char* spikard_read_to_string();
```

**Example:**

```c
const char *result = spikard_read_to_string(instance);
```

**Returns:** `const char*`

**Errors:** Returns `NULL` on error.

###### spikard_content_type_or_default()

Get the content type, defaulting to "application/octet-stream".

**Signature:**

```c
const char* spikard_content_type_or_default();
```

**Example:**

```c
const char *result = spikard_content_type_or_default(instance);
```

**Returns:** `const char*`

---

#### SpikardValidateRequest

Request body for `POST /asyncapi/validate`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `spec` | `void*` | — | Spec |
| `channel` | `const char*` | — | Channel |
| `message` | `const char*` | — | Message |
| `payload` | `void*` | — | Payload |

---

#### SpikardValidationResponse

Response body for `POST /asyncapi/validate`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `valid` | `bool` | — | Valid |
| `errors` | `const char**` | — | Errors |

---

### Enums

#### SpikardMethod

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
| `SPIKARD_CONNECT` | Connect |
| `SPIKARD_TRACE` | Trace |

---

#### SpikardSecuritySchemeInfo

Security scheme types

| Value | Description |
|-------|-------------|
| `SPIKARD_HTTP` | Http — Fields: `scheme`: `const char*`, `bearer_format`: `const char*` |
| `SPIKARD_API_KEY` | Api key — Fields: `location`: `const char*`, `name`: `const char*` |

---

### Errors

#### SpikardAppError

Error type for application builder operations.

| Variant | Description |
|---------|-------------|
| `SPIKARD_ROUTE` | Route registration failed. |
| `SPIKARD_SERVER` | Server/router construction failed. |
| `SPIKARD_DECODE` | Failed to extract DTO from the request context. |

---

#### SpikardGraphQlError

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

#### SpikardSchemaError

Error type for schema building operations

| Variant | Description |
|---------|-------------|
| `SPIKARD_BUILDING_FAILED` | Generic schema building error |
| `SPIKARD_VALIDATION_ERROR` | Configuration validation error |
| `SPIKARD_COMPLEXITY_LIMIT_EXCEEDED` | Complexity limit exceeded |
| `SPIKARD_DEPTH_LIMIT_EXCEEDED` | Depth limit exceeded |

---

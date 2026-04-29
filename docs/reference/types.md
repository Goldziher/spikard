---
title: "Types Reference"
---

## Types Reference

All types defined by the library, grouped by category. Types are shown using Rust as the canonical representation.

### Configuration Types

See [Configuration Reference](configuration.md) for detailed defaults and language-specific representations.

#### CorsConfig

CORS configuration for a route

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `allowed_origins` | `Vec<String>` | `vec![]` | Allowed origins |
| `allowed_methods` | `Vec<String>` | `vec![]` | Allowed methods |
| `allowed_headers` | `Vec<String>` | `vec![]` | Allowed headers |
| `expose_headers` | `Vec<String>` | `None` | Expose headers |
| `max_age` | `Option<u32>` | `None` | Maximum age |
| `allow_credentials` | `Option<bool>` | `None` | Allow credentials |
| `methods_joined_cache` | `String` | — | Methods joined cache |
| `headers_joined_cache` | `String` | — | Headers joined cache |

---

#### CompressionConfig

Compression configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `gzip` | `bool` | `true` | Enable gzip compression |
| `brotli` | `bool` | `true` | Enable brotli compression |
| `min_size` | `usize` | — | Minimum response size to compress (bytes) |
| `quality` | `u32` | — | Compression quality (0-11 for brotli, 0-9 for gzip) |

---

#### RateLimitConfig

Rate limiting configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `per_second` | `u64` | `100` | Requests per second |
| `burst` | `u32` | `200` | Burst allowance |
| `ip_based` | `bool` | `true` | Use IP-based rate limiting |

---

#### GraphQLRouteConfig

Configuration for GraphQL routes

Provides a builder pattern for configuring GraphQL route parameters
for the Spikard HTTP server's routing system.

*Opaque type — fields are not directly accessible.*

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

---

#### QueryOnlyConfig

Configuration for schemas with only Query type

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `true` | Enable introspection queries |
| `complexity_limit` | `Option<usize>` | `None` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `Option<usize>` | `None` | Maximum query depth (None = unlimited) |

---

#### QueryMutationConfig

Configuration for schemas with Query and Mutation types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `true` | Enable introspection queries |
| `complexity_limit` | `Option<usize>` | `None` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `Option<usize>` | `None` | Maximum query depth (None = unlimited) |

---

#### FullSchemaConfig

Configuration for fully-featured schemas with Query, Mutation, and Subscription types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `true` | Enable introspection queries |
| `complexity_limit` | `Option<usize>` | `None` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `Option<usize>` | `None` | Maximum query depth (None = unlimited) |

---

#### BackgroundTaskConfig

Configuration for in-process background task execution.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_queue_size` | `usize` | `1024` | Maximum queue size |
| `max_concurrent_tasks` | `usize` | `128` | Maximum concurrent tasks |
| `drain_timeout_secs` | `u64` | `30` | Drain timeout secs |

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
| `enabled` | `bool` | `true` | Enable gRPC support |
| `max_message_size` | `usize` | — | Maximum message size in bytes (for both sending and receiving) This limit applies to individual messages in both unary and streaming RPCs. When a single message exceeds this size, the request is rejected with HTTP 413 (Payload Too Large). Default: 4MB (4194304 bytes) **Note:** This limit does NOT apply to the total response size in streaming RPCs. For multi-message streams, the total response can exceed this limit as long as each individual message stays within the limit. |
| `enable_compression` | `bool` | `true` | Enable gzip compression for gRPC messages |
| `request_timeout` | `Option<u64>` | `None` | Timeout for gRPC requests in seconds (None = no timeout) |
| `max_concurrent_streams` | `u32` | — | Maximum number of concurrent streams per connection (HTTP/2 advisory) This value is communicated to HTTP/2 clients as the server's flow control limit. The HTTP/2 transport layer enforces this limit automatically via SETTINGS frames and GOAWAY responses. Applications should NOT implement custom enforcement. Default: 100 streams per connection # Stream Limiting Strategy - **Per Connection**: This limit applies per HTTP/2 connection, not globally - **Transport Enforcement**: HTTP/2 handles all stream limiting; applications need not implement custom checks - **Streaming Requests**: In server streaming or bidi streaming, each logical RPC consumes one stream slot. Message ordering within a stream follows HTTP/2 frame ordering. # Future Enhancement A future `max_stream_response_bytes` field may be added to limit the total response size in streaming RPCs (separate from per-message limits). |
| `enable_keepalive` | `bool` | `true` | Enable HTTP/2 keepalive |
| `keepalive_interval` | `u64` | — | HTTP/2 keepalive interval in seconds |
| `keepalive_timeout` | `u64` | — | HTTP/2 keepalive timeout in seconds |

---

### JsonRpcConfig

JSON-RPC server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `true` | Enable JSON-RPC endpoint |
| `endpoint_path` | `String` | — | HTTP endpoint path for JSON-RPC requests (default: "/rpc") |
| `enable_batch` | `bool` | — | Enable batch request processing (default: true) |
| `max_batch_size` | `usize` | — | Maximum number of requests in a batch (default: 100) |

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

---

##### Response

HTTP Response with custom status code, headers, and content

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `Option<String>` | `Default::default()` | Response body content |
| `status_code` | `u16` | — | HTTP status code (defaults to 200) |
| `headers` | `HashMap<String, String>` | `HashMap::new()` | Response headers |

---

##### JwtConfig

JWT authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `secret` | `String` | — | Secret key for JWT verification |
| `algorithm` | `String` | — | Required algorithm (HS256, HS384, HS512, RS256, etc.) |
| `audience` | `Vec<String>` | `None` | Required audience claim |
| `issuer` | `Option<String>` | `None` | Required issuer claim |
| `leeway` | `u64` | — | Leeway for expiration checks (seconds) |

---

##### ApiKeyConfig

API Key authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `keys` | `Vec<String>` | — | Valid API keys |
| `header_name` | `String` | — | Header name to check (e.g., "X-API-Key") |

---

##### StaticFilesConfig

Static file serving configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `directory` | `String` | — | Directory path to serve |
| `route_prefix` | `String` | — | URL path prefix (e.g., "/static") |
| `index_file` | `bool` | — | Fallback to index.html for directories |
| `cache_control` | `Option<String>` | `None` | Cache-Control header value |

---

##### ServerConfig

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
| `openapi` | `Option<OpenApiConfig>` | `None` | OpenAPI documentation configuration |
| `jsonrpc` | `Option<JsonRpcConfig>` | `None` | JSON-RPC configuration |
| `grpc` | `Option<GrpcConfig>` | `None` | gRPC configuration |
| `lifecycle_hooks` | `Option<String>` | `None` | Lifecycle hooks for request/response processing |
| `background_tasks` | `BackgroundTaskConfig` | — | Background task executor configuration |
| `enable_http_trace` | `bool` | `false` | Enable per-request HTTP tracing (tower-http `TraceLayer`) |
| `di_container` | `Option<String>` | `None` | Dependency injection container (requires 'di' feature) |

---

#### Metadata Types

##### BackgroundJobMetadata

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `request_id` | `Option<String>` | `None` | Request id |

---

#### Other Types

##### UploadFile

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

---

##### JsonRpcMethodInfo

JSON-RPC method metadata for routes that support JSON-RPC

This struct captures the metadata needed to expose HTTP routes as JSON-RPC methods,
enabling discovery and documentation of RPC-compatible endpoints.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `method_name` | `String` | — | The JSON-RPC method name (e.g., "user.create") |
| `description` | `Option<String>` | `None` | Optional description of what the method does |
| `params_schema` | `Option<String>` | `None` | Optional JSON Schema for method parameters |
| `result_schema` | `Option<String>` | `None` | Optional JSON Schema for the result |
| `deprecated` | `bool` | — | Whether this method is deprecated |
| `tags` | `Vec<String>` | — | Tags for categorizing and grouping methods |

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
| `status` | `u16` | — | The HTTP status code generated by the origin server. This is advisory; the actual HTTP status code takes precedence. |
| `detail` | `Option<String>` | `None` | A human-readable explanation specific to this occurrence of the problem. |
| `instance` | `Option<String>` | `None` | A URI reference that identifies the specific occurrence of the problem. It may or may not yield further information if dereferenced. |
| `extensions` | `HashMap<String, String>` | — | Extension members - problem-type-specific data. For validation errors, this typically contains an "errors" array. |

---

### GraphQLError

*Opaque type — fields are not directly accessible.*

---

#### ContactInfo

Contact information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `Option<String>` | `None` | The name |
| `email` | `Option<String>` | `None` | Email |
| `url` | `Option<String>` | `None` | Url |

---

##### LicenseInfo

License information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `url` | `Option<String>` | `None` | Url |

---

##### ServerInfo

Server information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | Url |
| `description` | `Option<String>` | `None` | Human-readable description |

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
| `event_type` | `Option<String>` | `None` | Event type (optional) |
| `data` | `String` | — | Event data (JSON value) |
| `id` | `Option<String>` | `None` | Event ID (optional, for client-side reconnection) |
| `retry` | `Option<u64>` | `None` | Retry timeout in milliseconds (optional) |

---

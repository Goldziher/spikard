---
title: "Configuration Reference"
---

## Configuration Reference

This page documents all configuration types and their defaults across all languages.

### BackgroundTaskConfig

Configuration for in-process background task execution.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_queue_size` | `int` | `1024` | Maximum queue size |
| `max_concurrent_tasks` | `int` | `128` | Maximum concurrent tasks |
| `drain_timeout_secs` | `int` | `30` | Drain timeout secs |

---

### BackgroundJobMetadata

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | The name |
| `request_id` | `str | None` | `None` | Request id |

---

### CorsConfig

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

---

### RouteMetadata

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

---

### CompressionConfig

Compression configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `gzip` | `bool` | `True` | Enable gzip compression |
| `brotli` | `bool` | `True` | Enable brotli compression |
| `min_size` | `int` | — | Minimum response size to compress (bytes) |
| `quality` | `int` | — | Compression quality (0-11 for brotli, 0-9 for gzip) |

---

### RateLimitConfig

Rate limiting configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `per_second` | `int` | `100` | Requests per second |
| `burst` | `int` | `200` | Burst allowance |
| `ip_based` | `bool` | `True` | Use IP-based rate limiting |

---

### Route

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

---

### GrpcConfig

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

---

### JsonRpcConfig

JSON-RPC server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `True` | Enable JSON-RPC endpoint |
| `endpoint_path` | `str` | — | HTTP endpoint path for JSON-RPC requests (default: "/rpc") |
| `enable_batch` | `bool` | — | Enable batch request processing (default: true) |
| `max_batch_size` | `int` | — | Maximum number of requests in a batch (default: 100) |

---

### OpenApiConfig

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

---

### Response

HTTP Response with custom status code, headers, and content

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `str | None` | `None` | Response body content |
| `status_code` | `int` | — | HTTP status code (defaults to 200) |
| `headers` | `dict[str, str]` | `{}` | Response headers |

---

### JwtConfig

JWT authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `secret` | `str` | — | Secret key for JWT verification |
| `algorithm` | `str` | — | Required algorithm (HS256, HS384, HS512, RS256, etc.) |
| `audience` | `list[str] | None` | `None` | Required audience claim |
| `issuer` | `str | None` | `None` | Required issuer claim |
| `leeway` | `int` | — | Leeway for expiration checks (seconds) |

---

### ApiKeyConfig

API Key authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `keys` | `list[str]` | — | Valid API keys |
| `header_name` | `str` | — | Header name to check (e.g., "X-API-Key") |

---

### StaticFilesConfig

Static file serving configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `directory` | `str` | — | Directory path to serve |
| `route_prefix` | `str` | — | URL path prefix (e.g., "/static") |
| `index_file` | `bool` | — | Fallback to index.html for directories |
| `cache_control` | `str | None` | `None` | Cache-Control header value |

---

### ServerConfig

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

---

### SchemaConfig

Configuration for GraphQL schema building.

Encapsulates all schema-level configuration options including
introspection control, complexity limits, and depth limits.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `True` | Enable introspection queries |
| `complexity_limit` | `int | None` | `None` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `int | None` | `None` | Maximum query depth (None = unlimited) |

---

### QueryOnlyConfig

Configuration for schemas with only Query type

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `True` | Enable introspection queries |
| `complexity_limit` | `int | None` | `None` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `int | None` | `None` | Maximum query depth (None = unlimited) |

---

### QueryMutationConfig

Configuration for schemas with Query and Mutation types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `True` | Enable introspection queries |
| `complexity_limit` | `int | None` | `None` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `int | None` | `None` | Maximum query depth (None = unlimited) |

---

### FullSchemaConfig

Configuration for fully-featured schemas with Query, Mutation, and Subscription types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `True` | Enable introspection queries |
| `complexity_limit` | `int | None` | `None` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `int | None` | `None` | Maximum query depth (None = unlimited) |

---


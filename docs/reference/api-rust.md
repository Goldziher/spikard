---
title: "Rust API Reference"
---

## Rust API Reference <span class="version-badge">v0.13.0</span>

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

```rust
pub fn add_cors_headers(response: Response, origin: &str, cors_config: CorsConfig)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `response` | `Response` | Yes | Mutable reference to the response to modify |
| `origin` | `String` | Yes | The origin from the request (e.g., `<https://example.com>`) |
| `cors_config` | `CorsConfig` | Yes | CORS configuration to apply |

**Returns:** `()`

---

### Types

#### ApiKeyConfig

API Key authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `keys` | `Vec<String>` | — | Valid API keys |
| `header_name` | `String` | — | Header name to check (e.g., "X-API-Key") |

---

##### BackgroundHandle

---

##### BackgroundJobError

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `String` | — | Message |

###### Methods

###### from()

**Signature:**

```rust
pub fn from(message: &str) -> BackgroundJobError
```

---

##### BackgroundJobMetadata

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `request_id` | `Option<String>` | `None` | Request id |

###### Methods

###### default()

**Signature:**

```rust
pub fn default() -> BackgroundJobMetadata
```

---

##### BackgroundTaskConfig

Configuration for in-process background task execution.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_queue_size` | `usize` | `1024` | Maximum queue size |
| `max_concurrent_tasks` | `usize` | `128` | Maximum concurrent tasks |
| `drain_timeout_secs` | `u64` | `30` | Drain timeout secs |

###### Methods

###### default()

**Signature:**

```rust
pub fn default() -> BackgroundTaskConfig
```

---

##### Claims

JWT claims structure - can be extended based on needs

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `sub` | `String` | — | Sub |
| `exp` | `usize` | — | Exp |
| `iat` | `Option<usize>` | `None` | Iat |
| `nbf` | `Option<usize>` | `None` | Nbf |
| `aud` | `Option<Vec<String>>` | `None` | Aud |
| `iss` | `Option<String>` | `None` | Iss |

---

##### CompressionConfig

Compression configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `gzip` | `bool` | `true` | Enable gzip compression |
| `brotli` | `bool` | `true` | Enable brotli compression |
| `min_size` | `usize` | — | Minimum response size to compress (bytes) |
| `quality` | `u32` | — | Compression quality (0-11 for brotli, 0-9 for gzip) |

###### Methods

###### default()

**Signature:**

```rust
pub fn default() -> CompressionConfig
```

---

##### ContactInfo

Contact information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `Option<String>` | `None` | The name |
| `email` | `Option<String>` | `None` | Email |
| `url` | `Option<String>` | `None` | Url |

---

##### CorsConfig

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

###### Methods

###### allowed_methods_joined()

Get the cached joined methods string for preflight responses

**Signature:**

```rust
pub fn allowed_methods_joined(&self) -> String
```

###### allowed_headers_joined()

Get the cached joined headers string for preflight responses

**Signature:**

```rust
pub fn allowed_headers_joined(&self) -> String
```

###### is_origin_allowed()

Check if an origin is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```rust
pub fn is_origin_allowed(&self, origin: &str) -> bool
```

###### is_method_allowed()

Check if a method is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```rust
pub fn is_method_allowed(&self, method: &str) -> bool
```

###### are_headers_allowed()

Check if all requested headers are allowed (O(n) where n = num requested headers)

**Signature:**

```rust
pub fn are_headers_allowed(&self, requested: Vec<String>) -> bool
```

###### default()

**Signature:**

```rust
pub fn default() -> CorsConfig
```

---

##### FullSchemaConfig

Configuration for fully-featured schemas with Query, Mutation, and Subscription types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `true` | Enable introspection queries |
| `complexity_limit` | `Option<usize>` | `None` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `Option<usize>` | `None` | Maximum query depth (None = unlimited) |

###### Methods

###### default()

**Signature:**

```rust
pub fn default() -> FullSchemaConfig
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

```rust
pub fn status_code(&self) -> u16
```

###### to_graphql_response()

Convert error to GraphQL error response JSON

Returns a JSON object matching the GraphQL spec error format with
structured extensions for HTTP integration.

## Format

```json
{
  "errors": [
    {
      "message": "error message",
      "extensions": {
        "code": "ERROR_CODE",
        "status": 400,
        "type": "<https://spikard.dev/errors/...">
      }
    }
  ]
}
```

**Signature:**

```rust
pub fn to_graphql_response(&self) -> String
```

### to_http_response()

Convert error to structured HTTP error response

Returns a JSON object matching the project's error fixture format,
suitable for direct HTTP response conversion.

## Format

```json
{
  "type": "<https://spikard.dev/errors/...",>
  "title": "Error Title",
  "status": 422,
  "detail": "error message",
  "errors": [
    {
      "type": "error_code",
      "message": "error message"
    }
  ]
}
```

**Signature:**

```rust
pub fn to_http_response(&self) -> String
```

---

### GraphQlRouteConfig

Configuration for GraphQL routes

Provides a builder pattern for configuring GraphQL route parameters
for the Spikard HTTP server's routing system.

#### Methods

##### path()

Set the HTTP path for the GraphQL endpoint

**Signature:**

```rust
pub fn path(&self, path: &str) -> GraphQlRouteConfig
```

###### method()

Set the HTTP method for the GraphQL endpoint

**Signature:**

```rust
pub fn method(&self, method: &str) -> GraphQlRouteConfig
```

###### enable_playground()

Enable or disable the GraphQL Playground UI

**Signature:**

```rust
pub fn enable_playground(&self, enable: bool) -> GraphQlRouteConfig
```

###### description()

Set a custom description for documentation

**Signature:**

```rust
pub fn description(&self, description: &str) -> GraphQlRouteConfig
```

###### get_path()

Get the configured path

**Signature:**

```rust
pub fn get_path(&self) -> String
```

###### get_method()

Get the configured method

**Signature:**

```rust
pub fn get_method(&self) -> String
```

###### is_playground_enabled()

Check if playground is enabled

**Signature:**

```rust
pub fn is_playground_enabled(&self) -> bool
```

###### get_description()

Get the description if set

**Signature:**

```rust
pub fn get_description(&self) -> Option<String>
```

###### default()

**Signature:**

```rust
pub fn default() -> GraphQlRouteConfig
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
| `enabled` | `bool` | `true` | Enable gRPC support |
| `max_message_size` | `usize` | — | Maximum message size in bytes (for both sending and receiving) This limit applies to individual messages in both unary and streaming RPCs. When a single message exceeds this size, the request is rejected with HTTP 413 (Payload Too Large). Default: 4MB (4194304 bytes) **Note:** This limit does NOT apply to the total response size in streaming RPCs. For multi-message streams, the total response can exceed this limit as long as each individual message stays within the limit. |
| `enable_compression` | `bool` | `true` | Enable gzip compression for gRPC messages |
| `request_timeout` | `Option<u64>` | `None` | Timeout for gRPC requests in seconds (None = no timeout) |
| `max_concurrent_streams` | `u32` | — | Maximum number of concurrent streams per connection (HTTP/2 advisory) This value is communicated to HTTP/2 clients as the server's flow control limit. The HTTP/2 transport layer enforces this limit automatically via SETTINGS frames and GOAWAY responses. Applications should NOT implement custom enforcement. Default: 100 streams per connection # Stream Limiting Strategy - **Per Connection**: This limit applies per HTTP/2 connection, not globally - **Transport Enforcement**: HTTP/2 handles all stream limiting; applications need not implement custom checks - **Streaming Requests**: In server streaming or bidi streaming, each logical RPC consumes one stream slot. Message ordering within a stream follows HTTP/2 frame ordering. # Future Enhancement A future `max_stream_response_bytes` field may be added to limit the total response size in streaming RPCs (separate from per-message limits). |
| `enable_keepalive` | `bool` | `true` | Enable HTTP/2 keepalive |
| `keepalive_interval` | `u64` | — | HTTP/2 keepalive interval in seconds |
| `keepalive_timeout` | `u64` | — | HTTP/2 keepalive timeout in seconds |

### Methods

#### default()

**Signature:**

```rust
pub fn default() -> GrpcConfig
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
| `service_name` | `String` | — | Fully qualified service name (e.g., "mypackage.MyService") |
| `method_name` | `String` | — | Method name (e.g., "GetUser") |
| `payload` | `Vec<u8>` | — | Serialized protobuf message bytes |
| `metadata` | `String` | — | gRPC metadata (similar to HTTP headers) |

---

##### GrpcResponseData

gRPC response data returned by handlers

Contains the serialized protobuf response and any metadata to include
in the response headers.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `payload` | `Vec<u8>` | — | Serialized protobuf message bytes |
| `metadata` | `String` | — | gRPC metadata to include in response (similar to HTTP headers) |

---

##### JsonRpcConfig

JSON-RPC server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `true` | Enable JSON-RPC endpoint |
| `endpoint_path` | `String` | — | HTTP endpoint path for JSON-RPC requests (default: "/rpc") |
| `enable_batch` | `bool` | — | Enable batch request processing (default: true) |
| `max_batch_size` | `usize` | — | Maximum number of requests in a batch (default: 100) |

###### Methods

###### default()

**Signature:**

```rust
pub fn default() -> JsonRpcConfig
```

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

##### JwtConfig

JWT authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `secret` | `String` | — | Secret key for JWT verification |
| `algorithm` | `String` | — | Required algorithm (HS256, HS384, HS512, RS256, etc.) |
| `audience` | `Option<Vec<String>>` | `None` | Required audience claim |
| `issuer` | `Option<String>` | `None` | Required issuer claim |
| `leeway` | `u64` | — | Leeway for expiration checks (seconds) |

---

##### LicenseInfo

License information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `url` | `Option<String>` | `None` | Url |

---

##### OpenApiConfig

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

###### Methods

###### default()

**Signature:**

```rust
pub fn default() -> OpenApiConfig
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
| `status` | `u16` | — | The HTTP status code generated by the origin server. This is advisory; the actual HTTP status code takes precedence. |
| `detail` | `Option<String>` | `None` | A human-readable explanation specific to this occurrence of the problem. |
| `instance` | `Option<String>` | `None` | A URI reference that identifies the specific occurrence of the problem. It may or may not yield further information if dereferenced. |
| `extensions` | `HashMap<String, String>` | — | Extension members - problem-type-specific data. For validation errors, this typically contains an "errors" array. |

### Methods

#### with_detail()

Set the detail field

**Signature:**

```rust
pub fn with_detail(&self, detail: &str) -> ProblemDetails
```

##### with_instance()

Set the instance field

**Signature:**

```rust
pub fn with_instance(&self, instance: &str) -> ProblemDetails
```

###### with_extension()

Add an extension field

**Signature:**

```rust
pub fn with_extension(&self, key: &str, value: &str) -> ProblemDetails
```

###### with_extensions()

Add all extensions from a JSON object

**Signature:**

```rust
pub fn with_extensions(&self, extensions: &str) -> ProblemDetails
```

###### from_validation_error()

Create a validation error Problem Details from `ValidationError`

This converts the FastAPI-style validation errors to RFC 9457 format:

- `type`: <https://spikard.dev/errors/validation-error>
- `title`: "Request Validation Failed"
- `status`: 422
- `detail`: Summary of error count
- `errors`: Array of validation error details (as extension field)

**Signature:**

```rust
pub fn from_validation_error(error: &str) -> ProblemDetails
```

###### not_found()

Create a not found error

**Signature:**

```rust
pub fn not_found(detail: &str) -> ProblemDetails
```

###### method_not_allowed()

Create a method not allowed error

**Signature:**

```rust
pub fn method_not_allowed(detail: &str) -> ProblemDetails
```

###### internal_server_error()

Create an internal server error

**Signature:**

```rust
pub fn internal_server_error(detail: &str) -> ProblemDetails
```

###### internal_server_error_debug()

Create an internal server error with debug information

Includes exception details, traceback, and request data for debugging.
Only use in development/debug mode.

**Signature:**

```rust
pub fn internal_server_error_debug(detail: &str, exception: &str, traceback: &str, request_data: &str) -> ProblemDetails
```

###### bad_request()

Create a bad request error

**Signature:**

```rust
pub fn bad_request(detail: &str) -> ProblemDetails
```

###### status_code()

Get the HTTP status code

**Signature:**

```rust
pub fn status_code(&self) -> String
```

###### to_json()

Serialize to JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```rust
pub fn to_json(&self) -> String
```

###### to_json_pretty()

Serialize to pretty JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```rust
pub fn to_json_pretty(&self) -> String
```

---

##### QueryMutationConfig

Configuration for schemas with Query and Mutation types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `true` | Enable introspection queries |
| `complexity_limit` | `Option<usize>` | `None` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `Option<usize>` | `None` | Maximum query depth (None = unlimited) |

###### Methods

###### default()

**Signature:**

```rust
pub fn default() -> QueryMutationConfig
```

---

##### QueryOnlyConfig

Configuration for schemas with only Query type

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `true` | Enable introspection queries |
| `complexity_limit` | `Option<usize>` | `None` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `Option<usize>` | `None` | Maximum query depth (None = unlimited) |

###### Methods

###### default()

**Signature:**

```rust
pub fn default() -> QueryOnlyConfig
```

---

##### RateLimitConfig

Rate limiting configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `per_second` | `u64` | `100` | Requests per second |
| `burst` | `u32` | `200` | Burst allowance |
| `ip_based` | `bool` | `true` | Use IP-based rate limiting |

###### Methods

###### default()

**Signature:**

```rust
pub fn default() -> RateLimitConfig
```

---

##### Response

HTTP Response with custom status code, headers, and content

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `Option<String>` | `Default::default()` | Response body content |
| `status_code` | `u16` | — | HTTP status code (defaults to 200) |
| `headers` | `HashMap<String, String>` | `HashMap::new()` | Response headers |

###### Methods

###### with_status()

Create a response with a specific status code

**Signature:**

```rust
pub fn with_status(content: Option<String>, status_code: u16) -> Response
```

###### set_header()

Set a header

**Signature:**

```rust
pub fn set_header(&self, key: &str, value: &str)
```

###### set_cookie()

Set a cookie in the response

**Signature:**

```rust
pub fn set_cookie(&self, key: &str, value: &str, max_age: Option<i64>, domain: Option<String>, path: Option<String>, secure: bool, http_only: bool, same_site: Option<String>)
```

###### default()

**Signature:**

```rust
pub fn default() -> Response
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
| `method` | `Method` | `Method::Get` | Method (method) |
| `path` | `String` | `"/"` | File path |
| `handler_name` | `String` | `""` | Handler name |
| `request_validator` | `Option<String>` | `None` | Request validator |
| `response_validator` | `Option<String>` | `None` | Response validator |
| `parameter_validator` | `Option<String>` | `None` | Parameter validator |
| `file_params` | `Option<String>` | `None` | File params |
| `is_async` | `bool` | `true` | Whether async |
| `cors` | `Option<CorsConfig>` | `None` | Cors (cors config) |
| `expects_json_body` | `bool` | `false` | Precomputed flag: true if this route expects a JSON request body Used by middleware to validate Content-Type headers |
| `handler_dependencies` | `Vec<String>` | `vec![]` | List of dependency keys this handler requires (for DI) |
| `jsonrpc_method` | `Option<JsonRpcMethodInfo>` | `None` | Optional JSON-RPC method information When present, this route can be exposed as a JSON-RPC method |

###### Methods

###### default()

**Signature:**

```rust
pub fn default() -> Route
```

###### from_metadata()

Create a route from metadata, using schema registry for deduplication

Auto-generates parameter schema from type hints in the path if no explicit schema provided.
Type hints like `/items/{id:uuid}` generate appropriate JSON Schema validation.
Explicit `parameter_schema` overrides auto-generated schemas.

**Errors:**
Returns an error if the schema compilation fails or metadata is invalid.

The schema registry ensures each unique schema is compiled only once, improving
startup performance and memory usage for applications with many routes.

**Signature:**

```rust
pub fn from_metadata(metadata: RouteMetadata, registry: &str) -> Route
```

###### with_jsonrpc_method()

Builder method to attach JSON-RPC method info to a route

This is a convenient way to add JSON-RPC metadata after route creation.
It consumes the route and returns a new route with the metadata attached.

**Signature:**

```rust
pub fn with_jsonrpc_method(&self, info: JsonRpcMethodInfo) -> Route
```

###### is_jsonrpc_method()

Check if this route has JSON-RPC metadata

**Signature:**

```rust
pub fn is_jsonrpc_method(&self) -> bool
```

###### jsonrpc_method_name()

Get the JSON-RPC method name if present

**Signature:**

```rust
pub fn jsonrpc_method_name(&self) -> Option<String>
```

---

##### RouteMetadata

Route metadata extracted from bindings

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `method` | `String` | `"GET"` | Method |
| `path` | `String` | `"/"` | File path |
| `handler_name` | `String` | `""` | Handler name |
| `request_schema` | `Option<String>` | `None` | Request schema |
| `response_schema` | `Option<String>` | `None` | Response schema |
| `parameter_schema` | `Option<String>` | `None` | Parameter schema |
| `file_params` | `Option<String>` | `None` | File params |
| `is_async` | `bool` | `true` | Whether async |
| `cors` | `Option<CorsConfig>` | `None` | Cors (cors config) |
| `body_param_name` | `Option<String>` | `None` | Name of the body parameter (defaults to "body" if not specified) |
| `handler_dependencies` | `Option<Vec<String>>` | `None` | List of dependency keys this handler requires (for DI) |
| `jsonrpc_method` | `Option<String>` | `None` | JSON-RPC method metadata (if this route is exposed as a JSON-RPC method) |
| `static_response` | `Option<String>` | `None` | Optional static response configuration: `{"status": 200, "body": "OK", "content_type": "text/plain"}` When present, the handler is replaced by a `StaticResponseHandler` that bypasses the full middleware pipeline for maximum throughput. |

###### Methods

###### default()

**Signature:**

```rust
pub fn default() -> RouteMetadata
```

---

##### SchemaConfig

Configuration for GraphQL schema building.

Encapsulates all schema-level configuration options including
introspection control, complexity limits, and depth limits.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspection_enabled` | `bool` | `true` | Enable introspection queries |
| `complexity_limit` | `Option<usize>` | `None` | Maximum query complexity (None = unlimited) |
| `depth_limit` | `Option<usize>` | `None` | Maximum query depth (None = unlimited) |

###### Methods

###### default()

**Signature:**

```rust
pub fn default() -> SchemaConfig
```

###### set_introspection_enabled()

Enable or disable introspection

**Signature:**

```rust
pub fn set_introspection_enabled(&self, enabled: bool) -> SchemaConfig
```

###### set_complexity_limit()

Set the complexity limit (0 means unlimited)

**Signature:**

```rust
pub fn set_complexity_limit(&self, limit: usize) -> SchemaConfig
```

###### set_depth_limit()

Set the depth limit (0 means unlimited)

**Signature:**

```rust
pub fn set_depth_limit(&self, limit: usize) -> SchemaConfig
```

###### validate()

Validate the configuration

**Errors:**

Returns an error if the configuration is invalid (currently all configurations are valid)

**Signature:**

```rust
pub fn validate(&self) -> String
```

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

###### Methods

###### default()

**Signature:**

```rust
pub fn default() -> ServerConfig
```

###### builder()

Create a new builder for ServerConfig

**Signature:**

```rust
pub fn builder() -> String
```

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

### Methods

#### with_type()

Create a new SSE event with an event type and data

Creates an event with a type field. Clients can filter events by type
in their event listener.

**Signature:**

```rust
pub fn with_type(event_type: &str, data: &str) -> SseEvent
```

##### with_id()

Set the event ID for client-side reconnection support

Sets an ID that clients can use to resume from this point if they disconnect.
The client sends this ID back in the `Last-Event-ID` header when reconnecting.

**Signature:**

```rust
pub fn with_id(&self, id: &str) -> SseEvent
```

###### with_retry()

Set the retry timeout for client reconnection

Sets the time in milliseconds clients should wait before attempting to reconnect
if the connection is lost. The client browser will automatically handle reconnection.

**Signature:**

```rust
pub fn with_retry(&self, retry_ms: u64) -> SseEvent
```

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

###### Methods

###### as_bytes()

Get the raw file content as bytes.

This provides zero-copy access to the underlying buffer.

**Signature:**

```rust
pub fn as_bytes(&self) -> Vec<u8>
```

###### read_to_string()

Read the file content as a UTF-8 string.

**Errors:**

Returns an error if the content is not valid UTF-8.

**Signature:**

```rust
pub fn read_to_string(&self) -> String
```

###### content_type_or_default()

Get the content type, defaulting to "application/octet-stream".

**Signature:**

```rust
pub fn content_type_or_default(&self) -> String
```

---

##### ValidatedParams

Validated parameters from request (path, query, headers, cookies)

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `params` | `HashMap<String, String>` | — | Params |

---

#### Enums

##### Method

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
| `Trace` | Trace |

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
| `Success` | Successful response containing a result — Fields: `0`: `String` |
| `Error` | Error response containing error details — Fields: `0`: `String` |

---

### JsonRpcRequestOrBatch

Represents either a single JSON-RPC request or a batch of requests

Used to distinguish between single and batch requests after parsing,
allowing different routing logic for each case.

| Value | Description |
|-------|-------------|
| `Single` | A single JSON-RPC request — Fields: `0`: `String` |
| `Batch` | A batch (array) of JSON-RPC requests — Fields: `0`: `Vec<String>` |

---

#### SecuritySchemeInfo

Security scheme types

| Value | Description |
|-------|-------------|
| `Http` | Http — Fields: `scheme`: `String`, `bearer_format`: `String` |
| `ApiKey` | Api key — Fields: `location`: `String`, `name`: `String` |

---

#### Errors

##### GraphQlError

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

##### SchemaError

Error type for schema building operations

| Variant | Description |
|---------|-------------|
| `BuildingFailed` | Generic schema building error |
| `ValidationError` | Configuration validation error |
| `ComplexityLimitExceeded` | Complexity limit exceeded |
| `DepthLimitExceeded` | Depth limit exceeded |

---

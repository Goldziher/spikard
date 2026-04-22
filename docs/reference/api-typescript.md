---
title: "TypeScript API Reference"
---

## TypeScript API Reference <span class="version-badge">v0.13.0</span>

### Functions

#### schemaQueryOnly()

Create a simple schema configuration with only Query type.

This is a convenience function for schemas that only have queries.

**Returns:**

A `QueryOnlyConfig` with default settings

**Signature:**

```typescript
function schemaQueryOnly(): QueryOnlyConfig
```

**Returns:** `QueryOnlyConfig`

---

#### schemaQueryMutation()

Create a schema configuration with Query and Mutation types.

This is a convenience function for schemas with queries and mutations but no subscriptions.

**Returns:**

A `QueryMutationConfig` with default settings

**Signature:**

```typescript
function schemaQueryMutation(): QueryMutationConfig
```

**Returns:** `QueryMutationConfig`

---

#### schemaFull()

Create a schema configuration with all three root types.

This is a convenience function for fully-featured schemas.

**Returns:**

A `FullSchemaConfig` with default settings

**Signature:**

```typescript
function schemaFull(): FullSchemaConfig
```

**Returns:** `FullSchemaConfig`

---

#### addCorsHeaders()

Add CORS headers to a successful response

Adds appropriate CORS headers to the response based on the configuration.
This function should be called for successful (non-error) responses to
cross-origin requests.

## Headers Added

- `Access-Control-Allow-Origin` - The origin that is allowed (if valid)
- `Access-Control-Expose-Headers` - Headers that are safe to expose to the client
- `Access-Control-Allow-Credentials` - "true" if credentials are allowed

**Signature:**

```typescript
function addCorsHeaders(response: Response, origin: string, corsConfig: CorsConfig): void
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `response` | `Response` | Yes | Mutable reference to the response to modify |
| `origin` | `string` | Yes | The origin from the request (e.g., `<https://example.com>`) |
| `corsConfig` | `CorsConfig` | Yes | CORS configuration to apply |

**Returns:** `void`

---

### Types

#### ApiKeyConfig

API Key authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `keys` | `Array<string>` | — | Valid API keys |
| `headerName` | `string` | — | Header name to check (e.g., "X-API-Key") |

---

##### BackgroundHandle

---

##### BackgroundJobError

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `string` | — | Message |

###### Methods

###### from()

**Signature:**

```typescript
static from(message: string): BackgroundJobError
```

---

##### BackgroundJobMetadata

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | The name |
| `requestId` | `string | null` | `null` | Request id |

###### Methods

###### default()

**Signature:**

```typescript
static default(): BackgroundJobMetadata
```

---

##### BackgroundTaskConfig

Configuration for in-process background task execution.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `maxQueueSize` | `number` | `1024` | Maximum queue size |
| `maxConcurrentTasks` | `number` | `128` | Maximum concurrent tasks |
| `drainTimeoutSecs` | `number` | `30` | Drain timeout secs |

###### Methods

###### default()

**Signature:**

```typescript
static default(): BackgroundTaskConfig
```

---

##### Claims

JWT claims structure - can be extended based on needs

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `sub` | `string` | — | Sub |
| `exp` | `number` | — | Exp |
| `iat` | `number | null` | `null` | Iat |
| `nbf` | `number | null` | `null` | Nbf |
| `aud` | `Array<string> | null` | `null` | Aud |
| `iss` | `string | null` | `null` | Iss |

---

##### CompressionConfig

Compression configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `gzip` | `boolean` | `true` | Enable gzip compression |
| `brotli` | `boolean` | `true` | Enable brotli compression |
| `minSize` | `number` | — | Minimum response size to compress (bytes) |
| `quality` | `number` | — | Compression quality (0-11 for brotli, 0-9 for gzip) |

###### Methods

###### default()

**Signature:**

```typescript
static default(): CompressionConfig
```

---

##### ContactInfo

Contact information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string | null` | `null` | The name |
| `email` | `string | null` | `null` | Email |
| `url` | `string | null` | `null` | Url |

---

##### CorsConfig

CORS configuration for a route

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `allowedOrigins` | `Array<string>` | `[]` | Allowed origins |
| `allowedMethods` | `Array<string>` | `[]` | Allowed methods |
| `allowedHeaders` | `Array<string>` | `[]` | Allowed headers |
| `exposeHeaders` | `Array<string> | null` | `null` | Expose headers |
| `maxAge` | `number | null` | `null` | Maximum age |
| `allowCredentials` | `boolean | null` | `null` | Allow credentials |
| `methodsJoinedCache` | `string` | — | Methods joined cache |
| `headersJoinedCache` | `string` | — | Headers joined cache |

###### Methods

###### allowedMethodsJoined()

Get the cached joined methods string for preflight responses

**Signature:**

```typescript
allowedMethodsJoined(): string
```

###### allowedHeadersJoined()

Get the cached joined headers string for preflight responses

**Signature:**

```typescript
allowedHeadersJoined(): string
```

###### isOriginAllowed()

Check if an origin is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```typescript
isOriginAllowed(origin: string): boolean
```

###### isMethodAllowed()

Check if a method is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```typescript
isMethodAllowed(method: string): boolean
```

###### areHeadersAllowed()

Check if all requested headers are allowed (O(n) where n = num requested headers)

**Signature:**

```typescript
areHeadersAllowed(requested: Array<string>): boolean
```

###### default()

**Signature:**

```typescript
static default(): CorsConfig
```

---

##### FullSchemaConfig

Configuration for fully-featured schemas with Query, Mutation, and Subscription types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `boolean` | `true` | Enable introspection queries |
| `complexityLimit` | `number | null` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `number | null` | `null` | Maximum query depth (None = unlimited) |

###### Methods

###### default()

**Signature:**

```typescript
static default(): FullSchemaConfig
```

---

##### GraphQlError

###### Methods

###### statusCode()

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

```typescript
statusCode(): number
```

###### toGraphqlResponse()

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

```typescript
toGraphqlResponse(): string
```

### toHttpResponse()

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

```typescript
toHttpResponse(): string
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

```typescript
path(path: string): GraphQlRouteConfig
```

###### method()

Set the HTTP method for the GraphQL endpoint

**Signature:**

```typescript
method(method: string): GraphQlRouteConfig
```

###### enablePlayground()

Enable or disable the GraphQL Playground UI

**Signature:**

```typescript
enablePlayground(enable: boolean): GraphQlRouteConfig
```

###### description()

Set a custom description for documentation

**Signature:**

```typescript
description(description: string): GraphQlRouteConfig
```

###### getPath()

Get the configured path

**Signature:**

```typescript
getPath(): string
```

###### getMethod()

Get the configured method

**Signature:**

```typescript
getMethod(): string
```

###### isPlaygroundEnabled()

Check if playground is enabled

**Signature:**

```typescript
isPlaygroundEnabled(): boolean
```

###### getDescription()

Get the description if set

**Signature:**

```typescript
getDescription(): string | null
```

###### default()

**Signature:**

```typescript
static default(): GraphQlRouteConfig
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
| `enabled` | `boolean` | `true` | Enable gRPC support |
| `maxMessageSize` | `number` | — | Maximum message size in bytes (for both sending and receiving) This limit applies to individual messages in both unary and streaming RPCs. When a single message exceeds this size, the request is rejected with HTTP 413 (Payload Too Large). Default: 4MB (4194304 bytes) **Note:** This limit does NOT apply to the total response size in streaming RPCs. For multi-message streams, the total response can exceed this limit as long as each individual message stays within the limit. |
| `enableCompression` | `boolean` | `true` | Enable gzip compression for gRPC messages |
| `requestTimeout` | `number | null` | `null` | Timeout for gRPC requests in seconds (None = no timeout) |
| `maxConcurrentStreams` | `number` | — | Maximum number of concurrent streams per connection (HTTP/2 advisory) This value is communicated to HTTP/2 clients as the server's flow control limit. The HTTP/2 transport layer enforces this limit automatically via SETTINGS frames and GOAWAY responses. Applications should NOT implement custom enforcement. Default: 100 streams per connection # Stream Limiting Strategy - **Per Connection**: This limit applies per HTTP/2 connection, not globally - **Transport Enforcement**: HTTP/2 handles all stream limiting; applications need not implement custom checks - **Streaming Requests**: In server streaming or bidi streaming, each logical RPC consumes one stream slot. Message ordering within a stream follows HTTP/2 frame ordering. # Future Enhancement A future `max_stream_response_bytes` field may be added to limit the total response size in streaming RPCs (separate from per-message limits). |
| `enableKeepalive` | `boolean` | `true` | Enable HTTP/2 keepalive |
| `keepaliveInterval` | `number` | — | HTTP/2 keepalive interval in seconds |
| `keepaliveTimeout` | `number` | — | HTTP/2 keepalive timeout in seconds |

### Methods

#### default()

**Signature:**

```typescript
static default(): GrpcConfig
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
| `serviceName` | `string` | — | Fully qualified service name (e.g., "mypackage.MyService") |
| `methodName` | `string` | — | Method name (e.g., "GetUser") |
| `payload` | `Buffer` | — | Serialized protobuf message bytes |
| `metadata` | `string` | — | gRPC metadata (similar to HTTP headers) |

---

##### GrpcResponseData

gRPC response data returned by handlers

Contains the serialized protobuf response and any metadata to include
in the response headers.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `payload` | `Buffer` | — | Serialized protobuf message bytes |
| `metadata` | `string` | — | gRPC metadata to include in response (similar to HTTP headers) |

---

##### JsonRpcConfig

JSON-RPC server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `boolean` | `true` | Enable JSON-RPC endpoint |
| `endpointPath` | `string` | — | HTTP endpoint path for JSON-RPC requests (default: "/rpc") |
| `enableBatch` | `boolean` | — | Enable batch request processing (default: true) |
| `maxBatchSize` | `number` | — | Maximum number of requests in a batch (default: 100) |

###### Methods

###### default()

**Signature:**

```typescript
static default(): JsonRpcConfig
```

---

##### JsonRpcMethodInfo

JSON-RPC method metadata for routes that support JSON-RPC

This struct captures the metadata needed to expose HTTP routes as JSON-RPC methods,
enabling discovery and documentation of RPC-compatible endpoints.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `methodName` | `string` | — | The JSON-RPC method name (e.g., "user.create") |
| `description` | `string | null` | `null` | Optional description of what the method does |
| `paramsSchema` | `string | null` | `null` | Optional JSON Schema for method parameters |
| `resultSchema` | `string | null` | `null` | Optional JSON Schema for the result |
| `deprecated` | `boolean` | — | Whether this method is deprecated |
| `tags` | `Array<string>` | — | Tags for categorizing and grouping methods |

---

##### JwtConfig

JWT authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `secret` | `string` | — | Secret key for JWT verification |
| `algorithm` | `string` | — | Required algorithm (HS256, HS384, HS512, RS256, etc.) |
| `audience` | `Array<string> | null` | `null` | Required audience claim |
| `issuer` | `string | null` | `null` | Required issuer claim |
| `leeway` | `number` | — | Leeway for expiration checks (seconds) |

---

##### LicenseInfo

License information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | The name |
| `url` | `string | null` | `null` | Url |

---

##### OpenApiConfig

OpenAPI configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `boolean` | `false` | Enable OpenAPI generation (default: false for zero overhead) |
| `title` | `string` | `"API"` | API title |
| `version` | `string` | `"1.0.0"` | API version |
| `description` | `string | null` | `null` | API description (supports markdown) |
| `swaggerUiPath` | `string` | — | Path to serve Swagger UI (default: "/docs") |
| `redocPath` | `string` | — | Path to serve Redoc (default: "/redoc") |
| `openapiJsonPath` | `string` | — | Path to serve OpenAPI JSON spec (default: "/openapi.json") |
| `contact` | `ContactInfo | null` | `null` | Contact information |
| `license` | `LicenseInfo | null` | `null` | License information |
| `servers` | `Array<ServerInfo>` | `[]` | Server definitions |
| `securitySchemes` | `Record<string, SecuritySchemeInfo>` | `{}` | Security schemes (auto-detected from middleware if not provided) |

###### Methods

###### default()

**Signature:**

```typescript
static default(): OpenApiConfig
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
| `typeUri` | `string` | — | A URI reference that identifies the problem type. Defaults to "about:blank" when absent. Should be a stable, human-readable identifier for the problem type. |
| `title` | `string` | — | A short, human-readable summary of the problem type. Should not change from occurrence to occurrence of the problem. |
| `status` | `number` | — | The HTTP status code generated by the origin server. This is advisory; the actual HTTP status code takes precedence. |
| `detail` | `string | null` | `null` | A human-readable explanation specific to this occurrence of the problem. |
| `instance` | `string | null` | `null` | A URI reference that identifies the specific occurrence of the problem. It may or may not yield further information if dereferenced. |
| `extensions` | `Record<string, string>` | — | Extension members - problem-type-specific data. For validation errors, this typically contains an "errors" array. |

### Methods

#### withDetail()

Set the detail field

**Signature:**

```typescript
withDetail(detail: string): ProblemDetails
```

##### withInstance()

Set the instance field

**Signature:**

```typescript
withInstance(instance: string): ProblemDetails
```

###### withExtension()

Add an extension field

**Signature:**

```typescript
withExtension(key: string, value: string): ProblemDetails
```

###### withExtensions()

Add all extensions from a JSON object

**Signature:**

```typescript
withExtensions(extensions: string): ProblemDetails
```

###### fromValidationError()

Create a validation error Problem Details from `ValidationError`

This converts the FastAPI-style validation errors to RFC 9457 format:

- `type`: <https://spikard.dev/errors/validation-error>
- `title`: "Request Validation Failed"
- `status`: 422
- `detail`: Summary of error count
- `errors`: Array of validation error details (as extension field)

**Signature:**

```typescript
static fromValidationError(error: string): ProblemDetails
```

###### notFound()

Create a not found error

**Signature:**

```typescript
static notFound(detail: string): ProblemDetails
```

###### methodNotAllowed()

Create a method not allowed error

**Signature:**

```typescript
static methodNotAllowed(detail: string): ProblemDetails
```

###### internalServerError()

Create an internal server error

**Signature:**

```typescript
static internalServerError(detail: string): ProblemDetails
```

###### internalServerErrorDebug()

Create an internal server error with debug information

Includes exception details, traceback, and request data for debugging.
Only use in development/debug mode.

**Signature:**

```typescript
static internalServerErrorDebug(detail: string, exception: string, traceback: string, requestData: string): ProblemDetails
```

###### badRequest()

Create a bad request error

**Signature:**

```typescript
static badRequest(detail: string): ProblemDetails
```

###### statusCode()

Get the HTTP status code

**Signature:**

```typescript
statusCode(): string
```

###### toJson()

Serialize to JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```typescript
toJson(): string
```

###### toJsonPretty()

Serialize to pretty JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```typescript
toJsonPretty(): string
```

---

##### QueryMutationConfig

Configuration for schemas with Query and Mutation types

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `boolean` | `true` | Enable introspection queries |
| `complexityLimit` | `number | null` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `number | null` | `null` | Maximum query depth (None = unlimited) |

###### Methods

###### default()

**Signature:**

```typescript
static default(): QueryMutationConfig
```

---

##### QueryOnlyConfig

Configuration for schemas with only Query type

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `boolean` | `true` | Enable introspection queries |
| `complexityLimit` | `number | null` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `number | null` | `null` | Maximum query depth (None = unlimited) |

###### Methods

###### default()

**Signature:**

```typescript
static default(): QueryOnlyConfig
```

---

##### RateLimitConfig

Rate limiting configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `perSecond` | `number` | `100` | Requests per second |
| `burst` | `number` | `200` | Burst allowance |
| `ipBased` | `boolean` | `true` | Use IP-based rate limiting |

###### Methods

###### default()

**Signature:**

```typescript
static default(): RateLimitConfig
```

---

##### Response

HTTP Response with custom status code, headers, and content

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `string | null` | `null` | Response body content |
| `statusCode` | `number` | — | HTTP status code (defaults to 200) |
| `headers` | `Record<string, string>` | `{}` | Response headers |

###### Methods

###### withStatus()

Create a response with a specific status code

**Signature:**

```typescript
static withStatus(content: string, statusCode: number): Response
```

###### setHeader()

Set a header

**Signature:**

```typescript
setHeader(key: string, value: string): void
```

###### setCookie()

Set a cookie in the response

**Signature:**

```typescript
setCookie(key: string, value: string, maxAge: number, domain: string, path: string, secure: boolean, httpOnly: boolean, sameSite: string): void
```

###### default()

**Signature:**

```typescript
static default(): Response
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
| `method` | `Method` | `Method.Get` | Method (method) |
| `path` | `string` | `"/"` | File path |
| `handlerName` | `string` | `""` | Handler name |
| `requestValidator` | `string | null` | `null` | Request validator |
| `responseValidator` | `string | null` | `null` | Response validator |
| `parameterValidator` | `string | null` | `null` | Parameter validator |
| `fileParams` | `string | null` | `null` | File params |
| `isAsync` | `boolean` | `true` | Whether async |
| `cors` | `CorsConfig | null` | `null` | Cors (cors config) |
| `expectsJsonBody` | `boolean` | `false` | Precomputed flag: true if this route expects a JSON request body Used by middleware to validate Content-Type headers |
| `handlerDependencies` | `Array<string>` | `[]` | List of dependency keys this handler requires (for DI) |
| `jsonrpcMethod` | `JsonRpcMethodInfo | null` | `null` | Optional JSON-RPC method information When present, this route can be exposed as a JSON-RPC method |

###### Methods

###### default()

**Signature:**

```typescript
static default(): Route
```

###### fromMetadata()

Create a route from metadata, using schema registry for deduplication

Auto-generates parameter schema from type hints in the path if no explicit schema provided.
Type hints like `/items/{id:uuid}` generate appropriate JSON Schema validation.
Explicit `parameter_schema` overrides auto-generated schemas.

**Errors:**
Returns an error if the schema compilation fails or metadata is invalid.

The schema registry ensures each unique schema is compiled only once, improving
startup performance and memory usage for applications with many routes.

**Signature:**

```typescript
static fromMetadata(metadata: RouteMetadata, registry: string): Route
```

###### withJsonrpcMethod()

Builder method to attach JSON-RPC method info to a route

This is a convenient way to add JSON-RPC metadata after route creation.
It consumes the route and returns a new route with the metadata attached.

**Signature:**

```typescript
withJsonrpcMethod(info: JsonRpcMethodInfo): Route
```

###### isJsonrpcMethod()

Check if this route has JSON-RPC metadata

**Signature:**

```typescript
isJsonrpcMethod(): boolean
```

###### jsonrpcMethodName()

Get the JSON-RPC method name if present

**Signature:**

```typescript
jsonrpcMethodName(): string | null
```

---

##### RouteMetadata

Route metadata extracted from bindings

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `method` | `string` | `"GET"` | Method |
| `path` | `string` | `"/"` | File path |
| `handlerName` | `string` | `""` | Handler name |
| `requestSchema` | `string | null` | `null` | Request schema |
| `responseSchema` | `string | null` | `null` | Response schema |
| `parameterSchema` | `string | null` | `null` | Parameter schema |
| `fileParams` | `string | null` | `null` | File params |
| `isAsync` | `boolean` | `true` | Whether async |
| `cors` | `CorsConfig | null` | `null` | Cors (cors config) |
| `bodyParamName` | `string | null` | `null` | Name of the body parameter (defaults to "body" if not specified) |
| `handlerDependencies` | `Array<string> | null` | `null` | List of dependency keys this handler requires (for DI) |
| `jsonrpcMethod` | `string | null` | `null` | JSON-RPC method metadata (if this route is exposed as a JSON-RPC method) |
| `staticResponse` | `string | null` | `null` | Optional static response configuration: `{"status": 200, "body": "OK", "content_type": "text/plain"}` When present, the handler is replaced by a `StaticResponseHandler` that bypasses the full middleware pipeline for maximum throughput. |

###### Methods

###### default()

**Signature:**

```typescript
static default(): RouteMetadata
```

---

##### SchemaConfig

Configuration for GraphQL schema building.

Encapsulates all schema-level configuration options including
introspection control, complexity limits, and depth limits.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `introspectionEnabled` | `boolean` | `true` | Enable introspection queries |
| `complexityLimit` | `number | null` | `null` | Maximum query complexity (None = unlimited) |
| `depthLimit` | `number | null` | `null` | Maximum query depth (None = unlimited) |

###### Methods

###### default()

**Signature:**

```typescript
static default(): SchemaConfig
```

###### setIntrospectionEnabled()

Enable or disable introspection

**Signature:**

```typescript
setIntrospectionEnabled(enabled: boolean): SchemaConfig
```

###### setComplexityLimit()

Set the complexity limit (0 means unlimited)

**Signature:**

```typescript
setComplexityLimit(limit: number): SchemaConfig
```

###### setDepthLimit()

Set the depth limit (0 means unlimited)

**Signature:**

```typescript
setDepthLimit(limit: number): SchemaConfig
```

###### validate()

Validate the configuration

**Errors:**

Returns an error if the configuration is invalid (currently all configurations are valid)

**Signature:**

```typescript
validate(): string
```

---

##### ServerConfig

Server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `host` | `string` | `"127.0.0.1"` | Host to bind to |
| `port` | `number` | `8000` | Port to bind to |
| `workers` | `number` | `1` | Number of Tokio runtime worker threads used by binding-managed server runtimes |
| `enableRequestId` | `boolean` | `false` | Enable request ID generation and propagation |
| `maxBodySize` | `number | null` | `null` | Maximum request body size in bytes (None = unlimited, not recommended) |
| `requestTimeout` | `number | null` | `null` | Request timeout in seconds (None = no timeout) |
| `compression` | `CompressionConfig | null` | `null` | Enable compression middleware |
| `rateLimit` | `RateLimitConfig | null` | `null` | Enable rate limiting |
| `jwtAuth` | `JwtConfig | null` | `null` | JWT authentication configuration |
| `apiKeyAuth` | `ApiKeyConfig | null` | `null` | API Key authentication configuration |
| `staticFiles` | `Array<StaticFilesConfig>` | `[]` | Static file serving configuration |
| `gracefulShutdown` | `boolean` | `true` | Enable graceful shutdown on SIGTERM/SIGINT |
| `shutdownTimeout` | `number` | `30` | Graceful shutdown timeout (seconds) |
| `openapi` | `OpenApiConfig | null` | `null` | OpenAPI documentation configuration |
| `jsonrpc` | `JsonRpcConfig | null` | `null` | JSON-RPC configuration |
| `grpc` | `GrpcConfig | null` | `null` | gRPC configuration |
| `lifecycleHooks` | `string | null` | `null` | Lifecycle hooks for request/response processing |
| `backgroundTasks` | `BackgroundTaskConfig` | — | Background task executor configuration |
| `enableHttpTrace` | `boolean` | `false` | Enable per-request HTTP tracing (tower-http `TraceLayer`) |
| `diContainer` | `string | null` | `null` | Dependency injection container (requires 'di' feature) |

###### Methods

###### default()

**Signature:**

```typescript
static default(): ServerConfig
```

###### builder()

Create a new builder for ServerConfig

**Signature:**

```typescript
static builder(): string
```

---

##### ServerInfo

Server information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `string` | — | Url |
| `description` | `string | null` | `null` | Human-readable description |

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
| `eventType` | `string | null` | `null` | Event type (optional) |
| `data` | `string` | — | Event data (JSON value) |
| `id` | `string | null` | `null` | Event ID (optional, for client-side reconnection) |
| `retry` | `number | null` | `null` | Retry timeout in milliseconds (optional) |

### Methods

#### withType()

Create a new SSE event with an event type and data

Creates an event with a type field. Clients can filter events by type
in their event listener.

**Signature:**

```typescript
static withType(eventType: string, data: string): SseEvent
```

##### withId()

Set the event ID for client-side reconnection support

Sets an ID that clients can use to resume from this point if they disconnect.
The client sends this ID back in the `Last-Event-ID` header when reconnecting.

**Signature:**

```typescript
withId(id: string): SseEvent
```

###### withRetry()

Set the retry timeout for client reconnection

Sets the time in milliseconds clients should wait before attempting to reconnect
if the connection is lost. The client browser will automatically handle reconnection.

**Signature:**

```typescript
withRetry(retryMs: number): SseEvent
```

---

##### StaticFilesConfig

Static file serving configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `directory` | `string` | — | Directory path to serve |
| `routePrefix` | `string` | — | URL path prefix (e.g., "/static") |
| `indexFile` | `boolean` | — | Fallback to index.html for directories |
| `cacheControl` | `string | null` | `null` | Cache-Control header value |

---

##### UploadFile

Represents an uploaded file from multipart/form-data requests.

This struct provides efficient access to file content with automatic
base64 decoding and implements standard I/O traits for compatibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `filename` | `string` | — | Original filename from the client |
| `contentType` | `string | null` | `null` | MIME type of the uploaded file |
| `size` | `number | null` | `null` | Size of the file in bytes |
| `content` | `Buffer` | — | File content (may be base64 encoded) |
| `contentEncoding` | `string | null` | `null` | Content encoding type |
| `cursor` | `string` | — | Internal cursor for Read/Seek operations |

###### Methods

###### asBytes()

Get the raw file content as bytes.

This provides zero-copy access to the underlying buffer.

**Signature:**

```typescript
asBytes(): Buffer
```

###### readToString()

Read the file content as a UTF-8 string.

**Errors:**

Returns an error if the content is not valid UTF-8.

**Signature:**

```typescript
readToString(): string
```

###### contentTypeOrDefault()

Get the content type, defaulting to "application/octet-stream".

**Signature:**

```typescript
contentTypeOrDefault(): string
```

---

##### ValidatedParams

Validated parameters from request (path, query, headers, cookies)

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `params` | `Record<string, string>` | — | Params |

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
| `Success` | Successful response containing a result — Fields: `0`: `string` |
| `Error` | Error response containing error details — Fields: `0`: `string` |

---

### JsonRpcRequestOrBatch

Represents either a single JSON-RPC request or a batch of requests

Used to distinguish between single and batch requests after parsing,
allowing different routing logic for each case.

| Value | Description |
|-------|-------------|
| `Single` | A single JSON-RPC request — Fields: `0`: `string` |
| `Batch` | A batch (array) of JSON-RPC requests — Fields: `0`: `Array<string>` |

---

#### SecuritySchemeInfo

Security scheme types

| Value | Description |
|-------|-------------|
| `Http` | Http — Fields: `scheme`: `string`, `bearerFormat`: `string` |
| `ApiKey` | Api key — Fields: `location`: `string`, `name`: `string` |

---

#### Errors

##### GraphQlError

Errors that can occur during GraphQL operations

These errors are compatible with async-graphql error handling and can be
converted to structured HTTP responses matching the project's error fixtures.

Errors are thrown as plain `Error` objects with descriptive messages.

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

Errors are thrown as plain `Error` objects with descriptive messages.

| Variant | Description |
|---------|-------------|
| `BuildingFailed` | Generic schema building error |
| `ValidationError` | Configuration validation error |
| `ComplexityLimitExceeded` | Complexity limit exceeded |
| `DepthLimitExceeded` | Depth limit exceeded |

---

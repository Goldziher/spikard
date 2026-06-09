---
title: "WebAssembly API Reference"
---

## WebAssembly API Reference <span class="version-badge">v0.15.6-rc.20</span>

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

### Types

#### ApiKeyConfig

API Key authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `keys` | `Array<string>` | — | Valid API keys |
| `headerName` | `string` | `/* serde(default) */` | Header name to check (e.g., "X-API-Key") |

---

#### AsyncApiConfig

AsyncAPI HTTP endpoint configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `boolean` | — | Enable AsyncAPI endpoints (default: false) |
| `spec` | `unknown \| null` | `null` | Pre-registered AsyncAPI spec to serve from GET /asyncapi.json |

---

#### CompressionConfig

Compression configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `gzip` | `boolean` | `true` | Enable gzip compression |
| `brotli` | `boolean` | `true` | Enable brotli compression |
| `minSize` | `number` | — | Minimum response size to compress (bytes) |
| `quality` | `number` | — | Compression quality (0-11 for brotli, 0-9 for gzip) |

### Methods

#### default()

**Signature:**

```typescript
static default(): CompressionConfig
```

---

#### ContactInfo

Contact information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string \| null` | `null` | Name of the contact person or organisation. |
| `email` | `string \| null` | `null` | Contact email address. |
| `url` | `string \| null` | `null` | URL pointing to the contact information page. |

---

#### CorsConfig

CORS configuration for a route

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `allowedOrigins` | `Array<string>` | `[]` | Allowed origins |
| `allowedMethods` | `Array<string>` | `[]` | Allowed methods |
| `allowedHeaders` | `Array<string>` | `[]` | Allowed headers |
| `exposeHeaders` | `Array<string> \| null` | `null` | Expose headers |
| `maxAge` | `number \| null` | `null` | Maximum age |
| `allowCredentials` | `boolean \| null` | `null` | Allow credentials |
| `methodsJoinedCache` | `string` | — | Methods joined cache |
| `headersJoinedCache` | `string` | — | Headers joined cache |

### Methods

#### allowedMethodsJoined()

Get the cached joined methods string for preflight responses

**Signature:**

```typescript
allowedMethodsJoined(): string
```

#### allowedHeadersJoined()

Get the cached joined headers string for preflight responses

**Signature:**

```typescript
allowedHeadersJoined(): string
```

#### isOriginAllowed()

Check if an origin is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```typescript
isOriginAllowed(origin: string): boolean
```

#### isMethodAllowed()

Check if a method is allowed (O(1) with wildcard, O(n) for exact match)

**Signature:**

```typescript
isMethodAllowed(method: string): boolean
```

#### default()

**Signature:**

```typescript
static default(): CorsConfig
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

```typescript
call(request: Request, requestData: RequestData): HandlerResult
```

#### prefersRawJsonBody()

Whether this handler prefers consuming `RequestData.raw_body` over the parsed
`RequestData.body` for JSON requests.

When `true`, the server may skip eager JSON parsing when there is no request-body
schema validator attached to the route.

**Signature:**

```typescript
prefersRawJsonBody(): boolean
```

#### prefersParameterExtraction()

Whether this handler wants to perform its own parameter validation/extraction (path/query/header/cookie).

When `true`, the server will skip `ParameterValidator.validate_and_extract` in `ValidatingHandler`.
This is useful for language bindings which need to transform validated parameters into
language-specific values (e.g., Python kwargs) without duplicating work. When `false`,
the server stores validated output in `RequestData.validated_params`.

**Signature:**

```typescript
prefersParameterExtraction(): boolean
```

#### wantsHeaders()

Whether this handler needs the parsed headers map in `RequestData`.

When `false`, the server may skip building `RequestData.headers` for requests without a body.
(Requests with bodies still typically need `Content-Type` decisions.)

**Signature:**

```typescript
wantsHeaders(): boolean
```

#### wantsCookies()

Whether this handler needs the parsed cookies map in `RequestData`.

When `false`, the server may skip parsing cookies for requests without a body.

**Signature:**

```typescript
wantsCookies(): boolean
```

#### wantsRequestExtensions()

Whether this handler needs `RequestData` stored in request extensions.

When `false`, the server avoids inserting `RequestData` into extensions to
skip cloning in hot paths.

**Signature:**

```typescript
wantsRequestExtensions(): boolean
```

#### staticResponse()

Return a pre-built static response if this handler always produces the
same output. When `Some`, the server bypasses the full middleware
pipeline and serves the pre-built response directly.

**Signature:**

```typescript
staticResponse(): StaticResponse | null
```

---

#### IntoHandler

Convert user-facing handler functions into the low-level `Handler` trait.

### Methods

#### intoHandler()

Convert this value into a shared request handler.

**Signature:**

```typescript
intoHandler(): Handler
```

---

#### JsonRpcConfig

JSON-RPC server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `boolean` | `true` | Enable JSON-RPC endpoint |
| `endpointPath` | `string` | — | HTTP endpoint path for JSON-RPC requests (default: "/rpc") |
| `enableBatch` | `boolean` | — | Enable batch request processing (default: true) |
| `maxBatchSize` | `number` | — | Maximum number of requests in a batch (default: 100) |

### Methods

#### default()

**Signature:**

```typescript
static default(): JsonRpcConfig
```

---

#### JsonRpcMethodInfo

JSON-RPC method metadata for routes that support JSON-RPC

This struct captures the metadata needed to expose HTTP routes as JSON-RPC methods,
enabling discovery and documentation of RPC-compatible endpoints.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `methodName` | `string` | — | The JSON-RPC method name (e.g., "user.create") |
| `description` | `string \| null` | `null` | Optional description of what the method does |
| `paramsSchema` | `unknown \| null` | `null` | Optional JSON Schema for method parameters |
| `resultSchema` | `unknown \| null` | `null` | Optional JSON Schema for the result |
| `deprecated` | `boolean` | `/* serde(default) */` | Whether this method is deprecated |
| `tags` | `Array<string>` | `/* serde(default) */` | Tags for categorizing and grouping methods |

---

#### JwtConfig

JWT authentication configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `secret` | `string` | — | Secret key for JWT verification |
| `algorithm` | `string` | `/* serde(default) */` | Required algorithm (HS256, HS384, HS512, RS256, etc.) |
| `audience` | `Array<string> \| null` | `null` | Required audience claim |
| `issuer` | `string \| null` | `null` | Required issuer claim |
| `leeway` | `number` | `/* serde(default) */` | Leeway for expiration checks (seconds) |

---

#### LicenseInfo

License information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | SPDX license identifier or display name (e.g. `"MIT"`). |
| `url` | `string \| null` | `null` | URL to the full license text. |

---

#### OpenApiConfig

OpenAPI configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `boolean` | `false` | Enable OpenAPI generation (default: false for zero overhead) |
| `title` | `string` | `"API"` | API title |
| `version` | `string` | `"1.0.0"` | API version |
| `description` | `string \| null` | `null` | API description (supports markdown) |
| `swaggerUiPath` | `string` | — | Path to serve Swagger UI (default: "/docs") |
| `redocPath` | `string` | — | Path to serve Redoc (default: "/redoc") |
| `openapiJsonPath` | `string` | — | Path to serve OpenAPI JSON spec (default: "/openapi.json") |
| `contact` | `ContactInfo \| null` | `null` | Contact information |
| `license` | `LicenseInfo \| null` | `null` | License information |
| `servers` | `Array<ServerInfo>` | `[]` | Server definitions |
| `securitySchemes` | `Record<string, SecuritySchemeInfo>` | `{}` | Security schemes (auto-detected from middleware if not provided) |

### Methods

#### default()

**Signature:**

```typescript
static default(): OpenApiConfig
```

---

#### ParseRequest

Request body for `POST /asyncapi/parse`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `spec` | `unknown` | — | Spec |

---

#### ParseResult

Full parse result returned by `POST /asyncapi/parse`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `specVersion` | `string` | — | Spec version |
| `title` | `string` | — | Title |
| `apiVersion` | `string` | — | Api version |
| `channels` | `Array<ParsedChannel>` | — | Channels |
| `operations` | `Array<ParsedOperation>` | — | Operations |
| `messages` | `Array<ParsedMessage>` | — | Messages |

---

#### ParsedChannel

A single channel extracted from an AsyncAPI spec

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | Channel key from the spec (e.g. "chat/messages") |
| `address` | `string` | — | Channel address / path |
| `messages` | `Array<string>` | — | Message names declared on this channel |
| `bindings` | `unknown \| null` | `null` | Bindings (ws / http / amqp / …) as raw JSON for forward-compatibility |

---

#### ParsedMessage

A resolved message (name + JSON Schema)

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | Message name |
| `schema` | `unknown \| null` | `null` | Resolved JSON Schema for the message payload, if available |

---

#### ParsedOperation

A single operation extracted from an AsyncAPI spec

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | Operation name |
| `action` | `string` | — | Operation action: "send" or "receive" |
| `channel` | `string` | — | Channel reference (resolved to the channel name) |

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
| `typeUri` | `string` | — | A URI reference that identifies the problem type. Defaults to "about:blank" when absent. Should be a stable, human-readable identifier for the problem type. |
| `title` | `string` | — | A short, human-readable summary of the problem type. Should not change from occurrence to occurrence of the problem. |
| `status` | `number` | — | The HTTP status code generated by the origin server. This is advisory; the actual HTTP status code takes precedence. |
| `detail` | `string \| null` | `null` | A human-readable explanation specific to this occurrence of the problem. |
| `instance` | `string \| null` | `null` | A URI reference that identifies the specific occurrence of the problem. It may or may not yield further information if dereferenced. |
| `extensions` | `Record<string, unknown>` | — | Extension members - problem-type-specific data. For validation errors, this typically contains an "errors" array. |

### Methods

#### withDetail()

Set the detail field

**Signature:**

```typescript
withDetail(detail: string): ProblemDetails
```

#### withInstance()

Set the instance field

**Signature:**

```typescript
withInstance(instance: string): ProblemDetails
```

#### notFound()

Create a not found error

**Signature:**

```typescript
static notFound(detail: string): ProblemDetails
```

#### methodNotAllowed()

Create a method not allowed error

**Signature:**

```typescript
static methodNotAllowed(detail: string): ProblemDetails
```

#### internalServerError()

Create an internal server error

**Signature:**

```typescript
static internalServerError(detail: string): ProblemDetails
```

#### badRequest()

Create a bad request error

**Signature:**

```typescript
static badRequest(detail: string): ProblemDetails
```

#### toJson()

Serialize to JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```typescript
toJson(): string
```

#### toJsonPretty()

Serialize to pretty JSON string

**Errors:**
Returns an error if the serialization fails.

**Signature:**

```typescript
toJsonPretty(): string
```

---

#### RateLimitConfig

Rate limiting configuration shared across runtimes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `perSecond` | `number` | `100` | Requests per second |
| `burst` | `number` | `200` | Burst allowance |
| `ipBased` | `boolean` | `true` | Use IP-based rate limiting |

### Methods

#### default()

**Signature:**

```typescript
static default(): RateLimitConfig
```

---

#### Request

---

#### Response

HTTP Response with custom status code, headers, and content

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `unknown \| null` | `null` | Response body content |
| `statusCode` | `number` | — | HTTP status code (defaults to 200) |
| `headers` | `Record<string, string>` | `{}` | Response headers |

### Methods

#### setHeader()

Set a header

**Signature:**

```typescript
setHeader(key: string, value: string): void
```

#### setCookie()

Set a cookie in the response

**Signature:**

```typescript
setCookie(key: string, value: string, secure: boolean, httpOnly: boolean, maxAge: number, domain: string, path: string, sameSite: string): void
```

#### default()

**Signature:**

```typescript
static default(): Response
```

---

#### ServerConfig

Server configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `host` | `string` | `"127.0.0.1"` | Host to bind to |
| `port` | `number` | `8000` | Port to bind to |
| `workers` | `number` | `1` | Number of Tokio runtime worker threads used by binding-managed server runtimes |
| `enableRequestId` | `boolean` | `false` | Enable request ID generation and propagation |
| `maxBodySize` | `number \| null` | `null` | Maximum request body size in bytes (None = unlimited, not recommended) |
| `requestTimeout` | `number \| null` | `null` | Request timeout in seconds (None = no timeout) |
| `compression` | `CompressionConfig \| null` | `null` | Enable compression middleware |
| `rateLimit` | `RateLimitConfig \| null` | `null` | Enable rate limiting |
| `jwtAuth` | `JwtConfig \| null` | `null` | JWT authentication configuration |
| `apiKeyAuth` | `ApiKeyConfig \| null` | `null` | API Key authentication configuration |
| `staticFiles` | `Array<StaticFilesConfig>` | `[]` | Static file serving configuration |
| `gracefulShutdown` | `boolean` | `true` | Enable graceful shutdown on SIGTERM/SIGINT |
| `shutdownTimeout` | `number` | `30` | Graceful shutdown timeout (seconds) |
| `asyncapi` | `AsyncApiConfig \| null` | `null` | AsyncAPI HTTP endpoint configuration |
| `openapi` | `OpenApiConfig \| null` | `null` | OpenAPI documentation configuration |
| `jsonrpc` | `JsonRpcConfig \| null` | `null` | JSON-RPC configuration |
| `grpc` | `GrpcConfig \| null` | `null` | gRPC configuration |
| `lifecycleHooks` | `string \| null` | `null` | Lifecycle hooks for request/response processing |
| `backgroundTasks` | `BackgroundTaskConfig` | — | Background task executor configuration |
| `enableHttpTrace` | `boolean` | `false` | Enable per-request HTTP tracing (tower-http `TraceLayer`) |
| `diContainer` | `string \| null` | `null` | Dependency injection container (requires 'di' feature) |

### Methods

#### default()

**Signature:**

```typescript
static default(): ServerConfig
```

---

#### ServerInfo

Server information

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `string` | — | Base URL of the server (e.g. `"<https://api.example.com/v1"`>). |
| `description` | `string \| null` | `null` | Optional human-readable description of the server environment. |

---

#### TestClient

Core test client for making HTTP requests to a Spikard application.

This struct wraps axum-test's TestServer and provides a language-agnostic
interface for making HTTP requests, sending WebSocket connections, and
handling Server-Sent Events. Language bindings wrap this to provide
native API surfaces.

### Methods

#### graphqlAt()

Send a GraphQL query/mutation to a custom endpoint

**Signature:**

```typescript
graphqlAt(endpoint: string, query: string, variables: unknown, operationName: string): ResponseSnapshot
```

#### graphql()

Send a GraphQL query/mutation

**Signature:**

```typescript
graphql(query: string, variables: unknown, operationName: string): ResponseSnapshot
```

#### graphqlSubscriptionAt()

Send a GraphQL subscription (WebSocket) to a custom endpoint.

Uses the `graphql-transport-ws` protocol and captures the first `next` payload.
After the first payload is received, this client sends `complete` to unsubscribe.

**Signature:**

```typescript
graphqlSubscriptionAt(endpoint: string, query: string, variables: unknown, operationName: string): GraphQlSubscriptionSnapshot
```

#### graphqlSubscription()

Send a GraphQL subscription (WebSocket).

Uses `/graphql` as the default subscription endpoint.

**Signature:**

```typescript
graphqlSubscription(query: string, variables: unknown, operationName: string): GraphQlSubscriptionSnapshot
```

---

#### TestingSseEvent

A single Server-Sent Event.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data` | `string` | — | The data field of the event. |

---

#### ValidateRequest

Request body for `POST /asyncapi/validate`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `spec` | `unknown` | — | Spec |
| `channel` | `string` | — | Channel |
| `message` | `string` | — | Message |
| `payload` | `unknown` | — | Payload |

---

#### ValidationResponse

Response body for `POST /asyncapi/validate`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `valid` | `boolean` | — | Valid |
| `errors` | `Array<string>` | — | Errors |

---

### Enums

#### Method

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
| `Connect` | Connect |
| `Trace` | Trace |

---

#### SecuritySchemeInfo

Security scheme types

| Value | Description |
|-------|-------------|
| `Http` | Http — Fields: `scheme`: `string`, `bearerFormat`: `string` |
| `ApiKey` | Api key — Fields: `location`: `string`, `name`: `string` |

---

### Errors

#### AppError

Error type for application builder operations.

Errors are thrown as plain `Error` objects with descriptive messages.

| Variant | Description |
|---------|-------------|
| `Route` | Route registration failed. |
| `Server` | Server/router construction failed. |
| `Decode` | Failed to extract DTO from the request context. |

---

#### GraphQlError

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

#### SchemaError

Error type for schema building operations

Errors are thrown as plain `Error` objects with descriptive messages.

| Variant | Description |
|---------|-------------|
| `BuildingFailed` | Generic schema building error |
| `ValidationError` | Configuration validation error |
| `ComplexityLimitExceeded` | Complexity limit exceeded |
| `DepthLimitExceeded` | Depth limit exceeded |

---

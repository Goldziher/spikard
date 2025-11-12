# Spikard

A multi-language package built with Rust, targeting Python, Node.js, Ruby, and WebAssembly.

## Features

### Core HTTP Framework
- [x] Request/Response handling (path, query, headers, cookies, body)
- [x] JSON Schema validation (Draft 2020-12)
- [x] Format validation (UUID, date, datetime, email, URI, IPv4/IPv6)
- [x] CORS support (preflight, origin/method/header validation)
- [x] Multipart/form-data handling
- [x] URL-encoded form handling
- [x] RFC 9457 Problem Details error responses
- [x] Type hints in routes (`/items/{id:uuid}`, `/users/{id:int}`)

### Middleware & Performance
- [x] Request ID generation (UUID-based, X-Request-ID)
- [x] Response compression (gzip, brotli) with `CompressionConfig`
- [x] Request timeouts (configurable)
- [x] Body size limits (configurable max size)
- [x] Rate limiting (IP-based, configurable) with `RateLimitConfig`
- [x] Graceful shutdown (SIGTERM/SIGINT)
- [x] Static file serving (with cache-control) with `StaticFilesConfig`
- [x] Sensitive header hiding (Authorization, Cookie)
- [x] Comprehensive `ServerConfig` with all middleware settings
- [x] JWT authentication middleware with `JwtConfig` (HS/RS/ES/PS algorithms)
- [x] API Key authentication middleware with `ApiKeyConfig`

### Advanced Features
- [x] OpenAPI 3.1.0 generation (auto-detects security schemes)
- [x] Swagger UI integration
- [x] Redoc integration
- [x] Test client (Python, Node.js, Ruby)
- [x] Lifecycle hooks (onRequest, preValidation, preHandler, onResponse, onError)
- [x] WebSocket support
- [x] Server-Sent Events (SSE)
- [x] Streaming responses (Rust, Python, Node.js, Ruby)
- [ ] Background tasks

### Language Bindings
- [x] Python (PyO3) - Full support with `ServerConfig`
- [x] Node.js (napi-rs) - Full support with `ServerConfig`
- [x] Ruby (Magnus) - Full support with `ServerConfig`
- [x] WebAssembly (wasm-bindgen) - Basic support
- [x] Python: Typed config forwarding with dataclasses + msgspec
- [x] Node.js: Typed config forwarding with TypeScript
- [x] Ruby: Typed config forwarding with Magnus FFI

### CLI & Code Generation
- [x] OpenAPI to handler generation
- [x] Multi-language code generation (Python, Node, Ruby, Rust)
- [x] Fixture-based testing
- [ ] AsyncAPI support (WebSocket generation)

### Testing & Benchmarking
- [x] Fixture-driven integration tests (408 fixtures + streaming/SSE/websocket coverage)
- [x] Python e2e tests (408/408 passing - 100%)
- [x] Node.js e2e tests (408/408 passing - 100%)
- [x] Ruby e2e tests (411/411 passing - 100%)
- [x] AsyncAPI streaming fixtures (Ruby, Python, Node.js runners)
- [x] Benchmark harness
- [x] Performance benchmarks (Python, Node, Ruby, Rust)
- [ ] WebSocket benchmarks
- [ ] SSE benchmarks

## Structure

- `crates/spikard` - Core Rust library
- `crates/spikard-cli` - Command-line interface
- `crates/spikard-http` - HTTP server
- `crates/spikard-py` - Python bindings (PyO3)
- `crates/spikard-node` - Node.js bindings (napi-rs)
- `crates/spikard-wasm` - WebAssembly bindings (wasm-bindgen)
- `packages/python/tests` - Fixture-driven integration tests backed by `testing_data/`

## Quick Start

### Python

```python
from spikard import Spikard, get
from spikard.config import ServerConfig, CompressionConfig, RateLimitConfig

# Create a simple API
app = Spikard()

@get("/hello")
def hello_world():
    return {"message": "Hello, World!"}

@get("/users/{user_id:int}")
def get_user(user_id: int):
    return {"id": user_id, "name": f"User {user_id}"}

# Configure server with middleware
config = ServerConfig(
    host="0.0.0.0",
    port=8080,
    workers=4,
    compression=CompressionConfig(
        gzip=True,
        brotli=True,
        quality=9
    ),
    rate_limit=RateLimitConfig(
        per_second=100,
        burst=200,
        ip_based=True
    ),
    enable_request_id=True,
    graceful_shutdown=True
)

# Run the server
app.run(config=config)
```

### Configuration Options

The `ServerConfig` class provides comprehensive server configuration:

```python
from spikard.config import (
    ServerConfig,
    CompressionConfig,
    RateLimitConfig,
    JwtConfig,
    ApiKeyConfig,
    StaticFilesConfig,
)

config = ServerConfig(
    # Network settings
    host="0.0.0.0",
    port=8080,
    workers=4,

    # Request handling
    enable_request_id=True,
    max_body_size=10 * 1024 * 1024,  # 10MB
    request_timeout=30,

    # Compression middleware
    compression=CompressionConfig(
        gzip=True,
        brotli=True,
        min_size=1024,
        quality=6
    ),

    # Rate limiting (IP-based)
    rate_limit=RateLimitConfig(
        per_second=100,
        burst=200,
        ip_based=True
    ),

    # JWT authentication
    jwt_auth=JwtConfig(
        secret="your-secret-key",
        algorithm="HS256",  # Supports HS256/384/512, RS256/384/512, ES256/384/512, PS256/384/512
        audience=["https://api.example.com"],
        issuer="https://auth.example.com"
    ),

    # API key authentication
    api_key_auth=ApiKeyConfig(
        keys=["secret-key-1", "secret-key-2"],
        header_name="X-API-Key"
    ),

    # Static file serving
    static_files=[
        StaticFilesConfig(
            directory="./public",
            route_prefix="/static",
            cache_control="public, max-age=3600"
        )
    ],

    # Graceful shutdown
    graceful_shutdown=True,
    shutdown_timeout=30
)
```

### Lifecycle Hooks

Spikard supports Fastify-inspired lifecycle hooks for executing custom logic at specific points in the request/response lifecycle:

```python
from spikard import Spikard, get, Request, Response

app = Spikard()

# onRequest: Runs before routing
@app.on_request
async def add_request_id(request: Request) -> Request:
    request.headers["x-request-id"] = str(uuid.uuid4())
    return request

# preValidation: Runs after routing, before validation
@app.pre_validation
async def log_request(request: Request) -> Request:
    logger.info(f"{request.method} {request.path}")
    return request

# preHandler: Runs after validation, before handler (ideal for auth)
@app.pre_handler
async def check_auth(request: Request) -> Request | Response:
    token = request.headers.get("authorization")
    if not token:
        return Response(status_code=401, content={"error": "Unauthorized"})

    user = await validate_token(token)
    request.state["user"] = user
    return request

# onResponse: Runs after handler (can modify response)
@app.on_response
async def add_security_headers(response: Response) -> Response:
    response.headers["x-content-type-options"] = "nosniff"
    response.headers["x-frame-options"] = "DENY"
    return response

# onError: Runs when an error occurs
@app.on_error
async def log_errors(response: Response) -> Response:
    if response.status_code >= 500:
        logger.error(f"Server error: {response.status_code}")
    return response

@get("/protected")
async def protected_route(request: Request):
    user = request.state["user"]
    return {"message": f"Hello, {user.name}!"}
```

**Available hooks:**
- `onRequest` - Before routing, can modify request or short-circuit with response
- `preValidation` - After routing, before validation
- `preHandler` - After validation, before handler (ideal for authentication)
- `onResponse` - After handler, can modify response
- `onError` - When an error occurs, can customize error response

**Cross-language support:**
- **Python:** Async/await with PyO3 and pyo3-async-runtimes
- **Node.js:** Promise-based with napi-rs ThreadsafeFunction
- **Ruby:** Proc-based with magnus Opaque wrapper for thread safety

### Streaming Responses

Download-style endpoints (CSV exports, NDJSON feeds, binary logs) can now stream bytes without buffering the entire payload. The fixture set under `testing_data/streaming/` is generated for every language and covered by the e2e suites.

```python
# Python
from spikard import StreamingResponse

@get("/stream/csv")
async def csv_report():
    async def rows():
        yield "id,name,value\n"
        yield "1,Alice,42\n"
        yield "2,Bob,7\n"

    return StreamingResponse(
        rows(),
        status_code=200,
        headers={"content-type": "text/csv"},
    )
```

- **Rust:** return `HandlerResponse::stream(stream)` (any `Stream<Item = Result<Bytes, E>>`) and optionally chain `.with_status(..)` / `.with_header(..)`.
- **Node.js:** `new StreamingResponse(async function* () { /* yield strings, Buffers, Uint8Arrays */ }, { statusCode, headers })`.
- **Ruby:** `Spikard::StreamingResponse.new(Enumerator.new { |y| y << chunk }, status_code: 200, headers: { "content-type" => "application/octet-stream" })`.

The test clients expose `response.body_bytes`, `response.text`, and `response.json` so you can assert on the accumulated stream just like in the fixture-driven suites.

### OpenAPI Documentation

Spikard automatically generates OpenAPI 3.1.0 specifications with security scheme auto-detection:

```python
from spikard.config import OpenApiConfig

config = ServerConfig(
    openapi=OpenApiConfig(
        enabled=True,
        title="My API",
        version="1.0.0",
        description="API documentation",
        swagger_ui_path="/docs",      # Swagger UI at /docs
        redoc_path="/redoc",           # Redoc at /redoc
        openapi_json_path="/openapi.json",
        # Optional metadata
        contact={"name": "API Team", "email": "api@example.com"},
        license={"name": "MIT"},
        servers=[
            {"url": "https://api.example.com", "description": "Production"},
            {"url": "http://localhost:8000", "description": "Development"}
        ]
    ),
    # Security schemes are auto-detected from middleware configuration
    jwt_auth=JwtConfig(...),    # Automatically adds bearerAuth scheme
    api_key_auth=ApiKeyConfig(...)  # Automatically adds apiKeyAuth scheme
)
```

Features:
- Auto-generates OpenAPI spec from route definitions
- Auto-detects JWT and API key security schemes
- Swagger UI and Redoc integration
- JSON Schema validation for all routes
- Parameter and response documentation

### Backwards Compatibility

The old API still works for simple use cases:

```python
app = Spikard()
app.run(host="0.0.0.0", port=8080, workers=4)
```

## Development

### Prerequisites

- Rust (2024 edition)
- Python 3.10+
- Node.js 18+
- pnpm
- uv (Python package manager)
- Task (task runner)

### Setup

```bash
task setup
```

### Building

Common build targets are exposed via the Taskfile:

```bash
task build:rust   # Build the Rust workspace
task build:py     # Build the PyO3 bindings
task build:node   # Build the Node.js bindings
task build:wasm   # Build the WASM bindings
task build:js     # Build all JavaScript/TypeScript packages
```

### Testing

```bash
task test        # Run Rust and Python suites with CI parity
task test:rust   # Rust-only checks
task test:python # Pytest suite (uses PYTHONPATH=. under the hood)
```

### Running

```bash
# Run CLI
task run:cli

# Run HTTP server
task run:http
```

## Taskfile quick reference

All automation lives in the root `Taskfile.yaml` and runs from the repository root:

- `task setup` – install toolchains and build the Python bindings once.
- `task build` – composite build for Rust, Python, and JavaScript targets.
- `task test` – execute Rust and Python tests just like CI.
- `task lint` – format and lint across languages (`cargo fmt`, `clippy`, `ruff`, etc.).

Custom commands automatically set `PYTHONPATH=.`, so the tasks can be copied directly into your shell without extra environment setup.

## License

MIT

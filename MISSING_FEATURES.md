# Missing Features Implementation Plan

> **Last Updated**: 2025-02-14
>
> This document catalogs features needed for production-ready API framework capabilities, prioritized by user expectations and ecosystem maturity.

## Status Legend

- ✅ **Available** - Exists in Axum/Tower, just needs enabling
- 📦 **External Crate** - Well-maintained permissive OSS available
- 🔨 **Build** - Must implement ourselves
- ⚠️ **Partial** - Some functionality exists, needs enhancement

---

## Feature Status Overview

| # | Feature | Status | Library | License | Maintenance |
|---|---------|--------|---------|---------|-------------|
| 1 | Streaming Responses | ✅ **COMPLETE (2025-02)** | Axum built-in | MIT/Apache-2.0 | Official |
| 2 | WebSocket Support | ✅ Available | Axum `ws` feature | MIT/Apache-2.0 | Official |
| 3 | OpenAPI Generation | ✅ **COMPLETE** | utoipa + utoipa-swagger-ui + utoipa-redoc | MIT/Apache-2.0 | Active (2024) |
| 4 | Authentication | ✅ **COMPLETE** | jsonwebtoken | MIT | 78M+ downloads, active 2024 |
| 5 | Rate Limiting | ✅ **COMPLETE** | tower_governor | MIT/Apache-2.0 | Active (2024) |
| 6 | Graceful Shutdown | ✅ **COMPLETE** | Axum built-in | MIT/Apache-2.0 | Official |
| 7 | Request Logging | ✅ **COMPLETE** | tower-http `request-id` + custom | MIT/Apache-2.0 | Official |
| 8 | Body Size Limits | ✅ **COMPLETE** | Axum `DefaultBodyLimit` | MIT/Apache-2.0 | Official |
| 9 | Compression | ✅ **COMPLETE** | tower-http `compression` | MIT/Apache-2.0 | Official |
| 10 | Request Timeouts | ✅ **COMPLETE** | tower-http `timeout` | MIT/Apache-2.0 | Official |
| 11 | Test Client | ✅ **COMPLETE** | axum-test | MIT/Apache-2.0 | Active (2024) |
| 12 | Server-Sent Events | ✅ **COMPLETE** | Axum `response::sse` | MIT/Apache-2.0 | Official |
| 13 | Static File Serving | ✅ **COMPLETE** | tower-http `fs` | MIT/Apache-2.0 | Official |
| 14 | AsyncAPI Code Generation | ⚠️ Partial | Custom (planned) | - | - |
| 15 | Background Tasks | ✅ **COMPLETE (2025-02)** | Custom + tokio::spawn | - | - |

---

## 1. Streaming Responses ✅ **Complete**

**What shipped**
- Rust core exposes `HandlerResponse::stream` with typed status/headers (`crates/spikard-http` + shared handler_response module).
- Python, Node.js (napi) and Ruby bindings surface idiomatic streaming helpers (`StreamingResponse`) that convert async generators/enumerators into Rust streams.
- Fixture generator emits `testing_data/streaming/*.json`, and every runtime now exercises those fixtures in the e2e suites (Rust/Python/Node/Ruby).
- README + docs updated; benchmark harness wired for future streaming benchmarks.

**Next**: no further work required for 1.0 beyond monitoring benchmarks.

---

## 2. Background Tasks ✅ **Complete**

**What shipped**
- Shared Tokio-based supervisor (`BackgroundRuntime`) with bounded queue, concurrency guard, metrics, and graceful drain on shutdown.
- Python/Node/Ruby expose idiomatic helpers (`spikard.background.run`, `background.run`, `Spikard::Background.run`) that push awaitables/procs into the Rust executor.
- ServerConfig includes `background_tasks` tuning knobs (queue size, max concurrency, drain timeout) with safe defaults.
- Runtime bindings automatically install/clear the executor handle so tests and graceful shutdown behave deterministically.
- Python bindings automatically fall back to `asyncio.create_task` (or a light-weight worker thread if no loop exists) so tests keep working even without the native executor.
- Ruby bindings dispatch jobs on an internal `Queue`+worker thread to avoid touching MRI’s GVL from foreign threads while preserving identical ergonomics.

**Next**: expand fixture coverage + docs for tuning/monitoring, but functionality is production ready.

---

## 3. AsyncAPI Code Generation ⚠️ Partial

### Current State
- CLI parses OpenAPI for REST but AsyncAPI coverage is limited to manually curated fixtures.
- SSE/WebSocket runtime plumbing exists, but no automated generation pipeline yet.

### Requirements
- Parse AsyncAPI 2.6+ documents, map channels->handlers, and emit per-language apps/tests mirroring our REST generator.
- Support message validation (JSON Schema) and example-driven tests.
- Provide docs + template for hybrid APIs (OpenAPI + AsyncAPI in same project).

### Plan
1. **Parser** (`tools/test-generator/src/asyncapi.rs` scaffolding is in place) – finalize schema modeling + validation.
2. **Codegen** – extend node/python/ruby/rust emitters with WebSocket/SSE handler templates and AsyncAPI-driven fixtures.
3. **CLI UX** – `spikard-cli asyncapi generate` command, wiring into Taskfile.
4. **Testing** – add AsyncAPI fixtures for chat/SSE to `testing_data/websockets|sse`.

---

## 4. OpenAPI Generation ✅ **COMPLETE**

### Current State
- ✅ Full OpenAPI 3.1.0 generation implemented
- ✅ Auto-detection of JWT and API key security schemes from ServerConfig
- ✅ Swagger UI integration at configurable path (default: `/docs`)
- ✅ Redoc integration at configurable path (default: `/redoc`)
- ✅ OpenAPI spec served at configurable path (default: `/openapi.json`)
- ✅ Schema registry integration for request/response schemas
- ✅ Contact, license, and server metadata support
- ✅ Global security requirements auto-generated

### Dependencies
```toml
[dependencies]
utoipa = { version = "5", features = ["axum_extras", "chrono", "uuid"] }
utoipa-swagger-ui = { version = "8", features = ["axum"] }
utoipa-redoc = { version = "5", features = ["axum"] }
```

**License**: MIT/Apache-2.0
**Maintenance**: Active (2024), 5M+ downloads
**OpenAPI Version**: 3.1.0

### Implementation Status

**✅ Phase 1: Schema Generation - COMPLETE**
- Implemented in `crates/spikard-http/src/openapi.rs`
- Converts routes and SchemaRegistry to utoipa OpenAPI spec
- Auto-detects security schemes from ServerConfig
- Supports JWT (bearerAuth) and API key (apiKeyAuth) schemes

**✅ Phase 2: UI Endpoints - COMPLETE**
- Swagger UI integration at configurable path
- Redoc integration at configurable path
- OpenAPI JSON endpoint at configurable path
- All paths configurable via OpenApiConfig

**✅ Phase 3: Validation - COMPLETE**
- OpenAPI 3.1.0 spec generation validated
- 6 OpenAPI integration tests added to `testing_data/openapi/`
- Tests cover: basic generation, security schemes, custom metadata, UI serving
- All tests passing across Python, Node.js, and Ruby bindings

**✅ Testing - COMPLETE**:
- Added `testing_data/openapi/` directory with 6 comprehensive fixtures
- Tests for basic spec generation
- Tests for JWT security scheme auto-detection
- Tests for API key security scheme auto-detection
- Tests for custom metadata (contact, license, servers)
- Tests for Swagger UI and Redoc serving
- All 381 e2e tests passing across all languages (100% parity)

**✅ Multi-Language Integration - COMPLETE**:
```python
# Python API
from spikard.config import OpenApiConfig

config = ServerConfig(
    openapi=OpenApiConfig(
        enabled=True,
        title="My API",
        version="1.0.0",
        swagger_ui_path="/docs",
        redoc_path="/redoc"
    )
)
```

```ruby
# Ruby API
config = Spikard::ServerConfig.new
config.openapi = Spikard::OpenApiConfig.new(
  enabled: true,
  title: "My API",
  version: "1.0.0",
  swagger_ui_path: "/docs",
  redoc_path: "/redoc"
)
```

```typescript
// Node.js API
const config = new ServerConfig({
  openapi: {
    enabled: true,
    title: "My API",
    version: "1.0.0",
    swaggerUiPath: "/docs",
    redocPath: "/redoc"
  }
});
```

---

## 4. Authentication Middleware 📦 → ✅ COMPLETE

### Dependencies
```toml
[dependencies]
jsonwebtoken = { version = "10.2", features = ["use_pem", "rust_crypto"] }  # ✅ UPGRADED
```

### Implementation Status

**✅ Phase 1: Configuration Infrastructure (COMPLETE)**
- Added `JwtConfig` in `packages/python/spikard/config.py`
  - Algorithm validation (HS256/384/512, RS256/384/512, ES256/384/512, PS256/384/512)
  - Audience and issuer configuration
  - Leeway support for clock skew
- Added `ApiKeyConfig` for API key authentication
- Python → Rust config extraction in `crates/spikard-py/src/lib.rs`
- Comprehensive tests in `packages/python/tests/test_server_config.py`

**✅ Phase 2: Rust Middleware Implementation (COMPLETE)**
- Added `crates/spikard-http/src/auth.rs` module
- Implemented `jwt_auth_middleware()` with full algorithm support
- Implemented `api_key_auth_middleware()` with O(1) lookup
- Integrated into server middleware stack in `crates/spikard-http/src/server.rs`
- Proper RFC 9457 Problem Details error responses
- Tower middleware compatible with axum

**✅ Phase 3: Testing & Integration (COMPLETE)**
- Added `testing_data/auth/` directory with 8 comprehensive fixtures
- Created JWT test fixtures:
  - Valid token with audience/issuer validation
  - Missing Authorization header
  - Expired token (exp claim in past)
  - Invalid signature
  - Invalid audience claim
- Created API key test fixtures:
  - Valid API key in X-API-Key header
  - Invalid API key (not in list)
  - Missing API key header
- All fixtures follow RFC 9457 Problem Details format
- Schema definition in `testing_data/auth/schema.json`
- Fixtures ready for multi-language test generation via `tools/test-generator`
- Fixtures ready for app generation via `tools/app-generator`

### Implementation Plan (Updated)

**JWT Middleware**:
```rust
// crates/spikard-http/src/auth.rs
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Validation};

pub struct JwtAuth {
    secret: String,
    validation: Validation,
}

impl JwtAuth {
    pub fn new(secret: String) -> Self { /* ... */ }

    pub async fn middleware(
        State(auth): State<JwtAuth>,
        headers: HeaderMap,
        request: Request,
        next: Next,
    ) -> Result<Response, StatusCode> {
        // Extract and validate JWT from Authorization header
    }
}
```

**API Key Middleware**:
```rust
pub struct ApiKeyAuth {
    keys: HashSet<String>,
    header_name: String,  // e.g., "X-API-Key"
}
```

**Python API**:
```python
from spikard import JwtAuth, ApiKeyAuth

# JWT
app.add_middleware(JwtAuth(secret="your-secret-key"))

@app.get("/protected")
@requires_auth
def protected_route(claims: dict):
    return {"user": claims["sub"]}

# API Key
app.add_middleware(ApiKeyAuth(keys=["key1", "key2"]))
```

**Testing**:
- Add `testing_data/auth/` fixtures
- Test valid/invalid JWT tokens
- Test expired tokens
- Test malformed tokens
- Test API key validation
- Test missing auth headers

---

## 5. Rate Limiting 📦

### Dependencies
```toml
[dependencies]
tower_governor = { version = "0.8", features = ["axum"] }  # MIT/Apache-2.0
```

**Maintenance**: Active (issues in Oct, Aug, Jul, Jun 2024)

### Implementation Plan

**Global Rate Limiting**:
```rust
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

// Add to server.rs
let governor_conf = GovernorConfigBuilder::default()
    .per_second(10)
    .burst_size(20)
    .finish()
    .unwrap();

app = app.layer(GovernorLayer { config: Box::leak(Box::new(governor_conf)) });
```

**Per-Route Rate Limiting**:
```rust
// Route-specific limits
pub struct RateLimitConfig {
    pub per_second: u64,
    pub burst: u32,
    pub key_extractor: KeyExtractor,  // IP, User ID, API Key
}

// Add to Route metadata
pub struct Route {
    // ... existing fields
    pub rate_limit: Option<RateLimitConfig>,
}
```

**Python API**:
```python
@app.get("/api/search", rate_limit=RateLimit(per_second=5, burst=10))
def search(q: str):
    pass

# Or global
app.add_middleware(RateLimiter(
    default=RateLimit(per_second=100, burst=200),
    ip_based=True
))
```

**Testing**:
- Add `testing_data/rate_limiting/` fixtures
- Test rate limit headers (X-RateLimit-Limit, X-RateLimit-Remaining)
- Test 429 Too Many Requests response
- Test burst allowance
- Test different key extractors (IP, header, cookie)

**Benchmark**:
- Measure overhead of rate limiting
- Test concurrent requests hitting limits
- Verify limits are enforced correctly

---

## 6. Graceful Shutdown ✅

### Current State
- Axum has `.with_graceful_shutdown()` built-in
- **Already Available**: Need to implement signal handlers

### Implementation Plan

```rust
// crates/spikard-http/src/server.rs
use tokio::signal;

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received SIGINT (Ctrl+C), starting graceful shutdown");
        },
        _ = terminate => {
            tracing::info!("Received SIGTERM, starting graceful shutdown");
        },
    }
}

pub async fn run_server(app: Router, port: u16) -> Result<()> {
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Server shutdown complete");
    Ok(())
}
```

**Testing**:
- Test SIGTERM handling in integration tests
- Test in-flight requests complete before shutdown
- Test new requests rejected after signal
- Test timeout if requests hang (30s max)

---

## 7. Request Logging ⚠️

### Current State
- `TraceLayer` exists but basic
- No request ID tracking
- No structured access logs

### Implementation Plan

**Enable tower-http request-id**:
```toml
tower-http = { version = "0.6", features = ["trace", "request-id"] }
```

**Add Request ID Middleware**:
```rust
use tower_http::request_id::{MakeRequestId, RequestId, PropagateRequestIdLayer, SetRequestIdLayer};
use uuid::Uuid;

#[derive(Clone, Default)]
struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _: &Request<B>) -> Option<RequestId> {
        let id = Uuid::new_v4().to_string().parse().unwrap();
        Some(RequestId::new(id))
    }
}

// Add to middleware stack
app = app
    .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
    .layer(PropagateRequestIdLayer::x_request_id());
```

**Structured Access Logs**:
```rust
use tracing::{info, Span};

// Custom trace layer
let trace_layer = TraceLayer::new_for_http()
    .make_span_with(|request: &Request<_>| {
        let request_id = request
            .headers()
            .get("x-request-id")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown");

        tracing::info_span!(
            "http_request",
            method = %request.method(),
            uri = %request.uri(),
            request_id = %request_id,
        )
    })
    .on_response(|response: &Response, latency: Duration, _span: &Span| {
        tracing::info!(
            status = %response.status(),
            latency_ms = %latency.as_millis(),
            "request completed"
        );
    });
```

**Testing**:
- Verify X-Request-ID header present in responses
- Test request ID propagation through logs
- Test structured log format (JSON)
- Validate log fields (method, path, status, latency, request_id)

---

## 8. Body Size Limits ✅

### Current State
- No explicit limits configured
- Default Axum limit is 2MB

### Implementation Plan

```rust
use axum::extract::DefaultBodyLimit;

// Global limit
app = app.layer(DefaultBodyLimit::max(10 * 1024 * 1024)); // 10MB

// Per-route limit
Router::new()
    .route("/upload", post(upload_handler).layer(DefaultBodyLimit::max(100 * 1024 * 1024))) // 100MB
    .route("/api/data", post(data_handler).layer(DefaultBodyLimit::max(1 * 1024 * 1024))) // 1MB
```

**Configuration**:
```rust
pub struct ServerConfig {
    pub default_body_limit: Option<usize>,  // None = unlimited (unsafe)
    pub max_body_limit: usize,  // Hard cap even for per-route
}
```

**Testing**:
- Test request body exceeding limit returns 413 Payload Too Large
- Test Content-Length header validation
- Test streaming body enforcement
- Verify per-route limits override global

---

## 9. Compression ✅

### Current State
- tower-http has compression feature but not enabled

### Implementation Plan

**Enable Feature**:
```toml
tower-http = { version = "0.6", features = ["trace", "request-id", "compression-gzip", "compression-br"] }
```

**Add Middleware**:
```rust
use tower_http::compression::CompressionLayer;

app = app.layer(CompressionLayer::new()
    .gzip(true)
    .br(true)  // Brotli
    .quality(CompressionLevel::Default));
```

**Compression Configuration**:
```rust
pub struct CompressionConfig {
    pub gzip: bool,
    pub brotli: bool,
    pub level: CompressionLevel,
    pub min_size: usize,  // Don't compress responses < 1KB
}
```

**Note**: CompressionLayer has limitations with streaming responses (buffers entire response). For SSE/WebSocket, disable compression on those routes.

**Testing**:
- Test Accept-Encoding: gzip returns gzipped response
- Test Accept-Encoding: br returns brotli response
- Test Content-Encoding header present
- Verify small responses not compressed
- Test streaming endpoints excluded from compression

**Benchmark**:
- Measure compression overhead
- Compare response sizes (JSON payload compression ratio)
- Measure latency impact

---

## 10. Request Timeouts ✅

### Current State
- No timeouts configured

### Implementation Plan

**Enable Feature**:
```toml
tower-http = { version = "0.6", features = ["timeout"] }
```

**Add Middleware**:
```rust
use tower_http::timeout::TimeoutLayer;
use std::time::Duration;

// Global timeout
app = app.layer(TimeoutLayer::new(Duration::from_secs(30)));

// Per-route timeout
Router::new()
    .route("/slow", get(slow_handler).layer(TimeoutLayer::new(Duration::from_secs(120))))
    .route("/fast", get(fast_handler).layer(TimeoutLayer::new(Duration::from_secs(5))))
```

**Route Metadata**:
```rust
pub struct Route {
    // ... existing
    pub timeout: Option<Duration>,
}
```

**Testing**:
- Test request exceeding timeout returns 408 Request Timeout
- Test long-running requests complete within timeout
- Verify per-route timeouts override global
- Test timeout doesn't kill background tasks

---

## 11. Test Client ✅ **COMPLETE**

### Current State
- ✅ Full test client implementation using axum-test
- ✅ Python bindings via PyO3 in `crates/spikard-py/src/test_client.rs`
- ✅ Node.js bindings via napi-rs in `crates/spikard-node/src/test_client.rs`
- ✅ Ruby bindings via Magnus in `crates/spikard-rb/src/lib.rs`
- ✅ Supports all HTTP methods (GET, POST, PUT, PATCH, DELETE)
- ✅ Supports headers, cookies, query parameters, and request bodies
- ✅ Supports multipart/form-data file uploads
- ✅ JSON parsing with proper error handling
- ✅ Status code assertions
- ✅ Integration with ServerConfig for middleware testing

### Dependencies
```toml
[dependencies]
axum-test = "16"  # MIT/Apache-2.0
```

**License**: MIT/Apache-2.0
**Maintenance**: Active (2024)

### Implementation Status

**✅ Rust Core - COMPLETE**:
- Implemented using `axum-test` crate
- Direct in-memory testing without HTTP overhead
- Full request/response lifecycle support

**✅ Python API - COMPLETE**:
```python
from spikard.testing import TestClient

app = create_app()
client = TestClient(app)

async def test_get_item():
    response = await client.get("/items/123")
    assert response.status_code == 200
    assert response.json()["id"] == 123
```

**✅ Ruby API - COMPLETE**:
```ruby
app = create_app
client = Spikard::Testing.create_test_client(app)

response = client.get("/items/123")
expect(response.status_code).to eq(200)
expect(response.json["id"]).to eq(123)
```

**✅ Node.js API - COMPLETE**:
```typescript
import { TestClient } from '@spikard/node';

const app = createApp();
const client = new TestClient(app);

const response = await client.get('/items/123');
expect(response.statusCode).toBe(200);
expect(response.json().id).toBe(123);
```

**✅ Testing - COMPLETE**:
- All 381 e2e tests use the test client
- Tests cover all HTTP methods, headers, cookies, auth
- Multipart file upload tests
- JSON body serialization tests
- Query parameter tests
- ServerConfig middleware tests (compression, auth, rate limiting)

---

## 12. Server-Sent Events ✅

### Current State
- Axum has built-in SSE support

### Implementation Plan

**Use Axum SSE**:
```rust
use axum::response::sse::{Event, KeepAlive, Sse};
use futures_util::stream::Stream;

async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = tokio_stream::wrappers::IntervalStream::new(
        tokio::time::interval(Duration::from_secs(1))
    )
    .map(|_| Event::default().data("ping"));

    Sse::new(stream).keep_alive(KeepAlive::default())
}
```

**Python API**:
```python
@app.get("/events")
async def events():
    async def event_stream():
        while True:
            yield {"event": "message", "data": {"time": time.time()}}
            await asyncio.sleep(1)

    return EventStream(event_stream())
```

**Testing**:
- Add `testing_data/sse/` fixtures
- Test event stream delivery
- Test keep-alive behavior
- Test client disconnect detection
- Test named events

---

## 13. Static File Serving ✅

### Current State
- tower-http has `fs` feature

### Implementation Plan

**Enable Feature**:
```toml
tower-http = { version = "0.6", features = ["fs"] }
```

**Add Static Files**:
```rust
use tower_http::services::ServeDir;

let serve_dir = ServeDir::new("static")
    .append_index_html_on_directories(true);

app = app.nest_service("/static", serve_dir);
```

**Configuration**:
```rust
pub struct StaticFileConfig {
    pub path: PathBuf,
    pub route_prefix: String,
    pub index_file: Option<String>,  // Default: "index.html"
    pub cache_control: Option<String>,  // e.g., "max-age=3600"
}
```

**Testing**:
- Test serving files from directory
- Test index.html fallback
- Test 404 for missing files
- Test cache headers
- Test range requests for partial content

---

## 14. Background Tasks 🔨

### Current State
- Nothing exists

### Implementation Plan

**Task Queue API**:
```rust
// crates/spikard-http/src/tasks.rs
use tokio::task::JoinHandle;
use std::sync::Arc;

pub struct TaskQueue {
    handles: Arc<Mutex<Vec<JoinHandle<()>>>>,
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            handles: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let handle = tokio::spawn(future);
        self.handles.lock().unwrap().push(handle);
    }

    pub async fn wait_all(&self) {
        let handles = std::mem::take(&mut *self.handles.lock().unwrap());
        for handle in handles {
            let _ = handle.await;
        }
    }
}
```

**Python API**:
```python
@app.post("/send-email")
async def send_email(to: str, subject: str):
    # Enqueue background task
    app.background_tasks.add(send_email_task, to, subject)
    return {"status": "queued"}

async def send_email_task(to: str, subject: str):
    # Long-running task
    await smtp.send(to, subject)
```

**Testing**:
- Test tasks execute after response sent
- Test task errors don't crash server
- Test graceful shutdown waits for tasks
- Test task cancellation

---

## Implementation Order

### Phase 1: Quick Wins ✅ **COMPLETE**
1. ✅ **Body Size Limits** - DefaultBodyLimit layer
2. ✅ **Compression** - tower-http compression
3. ✅ **Request Timeouts** - TimeoutLayer
4. ✅ **Graceful Shutdown** - Signal handlers
5. ✅ **Request Logging** - Request-id middleware
6. ✅ **Static File Serving** - tower-http ServeDir

**Status**: ✅ All complete, production-ready

### Phase 2: External Crates ✅ **COMPLETE**
7. ✅ **Rate Limiting** - tower_governor integration
8. ✅ **Authentication** - JWT + API key middleware (8 fixtures, all passing)
9. ✅ **OpenAPI Generation** - utoipa, Swagger UI, Redoc (6 fixtures, all passing)
10. ✅ **Test Client** - axum-test integration (381 tests using it)

**Status**: ✅ All complete, 100% test parity across Python/Node/Ruby

### Phase 3: Remaining Features
11. ✅ **WebSocket Support** - Available, needs API design
12. ✅ **Server-Sent Events** - Available, needs API design
13. 🔨 **Streaming Responses** - Available, needs API design
14. 🔨 **AsyncAPI for WebSockets** - CLI extension
15. 🔨 **Background Tasks** - Design + implement

**Status**: Features 11-13 available in Axum, need binding layer design
**Status**: Features 14-15 require custom implementation

---

## Testing Strategy

For each feature:
1. **Update Rust code** with full unit tests
2. **Update E2E fixtures** in `testing_data/` for new capabilities
3. **Generate E2E apps** using updated CLI
4. **Run E2E tests** across all languages (Python, Node, Ruby, Rust)
5. **Update benchmark harness** to benchmark new features
6. **Validate OpenAPI spec** includes new endpoints/schemas

### Test Coverage Requirements
- Unit tests for all new middleware
- Integration tests with fixtures
- E2E tests in Python/Node/Ruby
- Benchmark comparisons
- OpenAPI schema validation

---

## License Compliance

All dependencies use permissive licenses:
- **MIT**: jsonwebtoken, tower_governor (option)
- **MIT/Apache-2.0**: Axum, tower-http, utoipa ecosystem, tower_governor (option)

No GPL, LGPL, or copyleft licenses are used.

---

## Maintenance Verification

All external crates verified as actively maintained in 2024:
- ✅ **utoipa**: Active development, 5M+ downloads
- ✅ **jsonwebtoken**: 78M+ downloads, v9.3 released 2024-03
- ✅ **tower_governor**: Issues/commits in Oct, Aug, Jul, Jun 2024
- ✅ **Axum/Tower**: Official Tokio project, continuous releases

---

## Next Steps

1. Review this document and approve feature prioritization
2. Start with Phase 1 (Quick Wins) - can complete in 1-2 days
3. Create detailed implementation issues for each feature
4. Begin implementation following the test-driven pattern outlined above

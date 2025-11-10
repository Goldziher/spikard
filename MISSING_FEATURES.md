# Missing Features Implementation Plan

> **Last Updated**: 2025-01-10
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
| 1 | Streaming Responses | ✅ Available | Axum built-in | MIT/Apache-2.0 | Official |
| 2 | WebSocket Support | ✅ Available | Axum `ws` feature | MIT/Apache-2.0 | Official |
| 3 | OpenAPI Generation | 📦 External | utoipa + utoipa-swagger-ui + utoipa-redoc | MIT/Apache-2.0 | Active (2024) |
| 4 | Authentication | 📦 External | jsonwebtoken | MIT | 78M+ downloads, active 2024 |
| 5 | Rate Limiting | 📦 External | tower_governor | MIT/Apache-2.0 | Active (2024) |
| 6 | Graceful Shutdown | ✅ Available | Axum built-in | MIT/Apache-2.0 | Official |
| 7 | Request Logging | ⚠️ Partial | tower-http `request-id` + custom | MIT/Apache-2.0 | Official |
| 8 | Body Size Limits | ✅ Available | Axum `DefaultBodyLimit` | MIT/Apache-2.0 | Official |
| 9 | Compression | ✅ Available | tower-http `compression` | MIT/Apache-2.0 | Official |
| 10 | Request Timeouts | ✅ Available | tower-http `timeout` | MIT/Apache-2.0 | Official |
| 11 | Test Client | 🔨 Build | Custom implementation | - | - |
| 12 | Server-Sent Events | ✅ Available | Axum `response::sse` | MIT/Apache-2.0 | Official |
| 13 | Static File Serving | ✅ Available | tower-http `fs` | MIT/Apache-2.0 | Official |
| 14 | Background Tasks | 🔨 Build | Custom + tokio::spawn | - | - |

---

## 1. Streaming Responses ✅

### Current State
- Axum supports streaming via `Body::from_stream()`
- **Already Available**: Just needs to be exposed in our API

### Implementation Plan

**Rust Core** (`crates/spikard-http`):
```rust
// Add to handler_trait.rs
pub enum ResponseBody {
    Json(Value),
    Stream(Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>),
}

// Add to Response type
impl Response {
    pub fn stream<S>(stream: S) -> Self
    where
        S: Stream<Item = Result<Bytes, std::io::Error>> + Send + 'static
    {
        // Convert stream to Body::from_stream
    }
}
```

**Python API** (`packages/python`):
```python
@app.get("/download/{file_id}")
async def download_file(file_id: str):
    async def generate():
        with open(f"files/{file_id}", "rb") as f:
            while chunk := f.read(8192):
                yield chunk

    return Response.stream(generate(), content_type="application/octet-stream")
```

**Testing**:
- Add `testing_data/streaming/` with fixtures for large file downloads
- Test chunked transfer encoding
- Verify memory usage stays constant for large files
- Benchmark against buffered responses

**Benchmark**:
- Add streaming endpoint to benchmark-harness apps
- Measure throughput for 100MB+ files
- Compare memory usage vs buffered approach

---

## 2. WebSocket Support ✅

### Current State
- Axum has built-in WebSocket support via `ws` feature
- **Already Available**: Need to add feature flag and expose API

### Implementation Plan

**Rust Core** (`crates/spikard-http/Cargo.toml`):
```toml
[dependencies]
axum = { workspace = true, features = ["multipart", "ws"] }
```

**Handler API**:
```rust
// Add WebSocket handler type
pub trait WebSocketHandler: Send + Sync {
    fn on_connect(&self, socket: WebSocket) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;
}

// Router integration
impl Router {
    pub fn ws(&mut self, path: &str, handler: Arc<dyn WebSocketHandler>) {
        // Register WebSocket upgrade route
    }
}
```

**Python API**:
```python
@app.websocket("/ws")
async def websocket_endpoint(websocket: WebSocket):
    await websocket.accept()
    while True:
        data = await websocket.receive_text()
        await websocket.send_text(f"Echo: {data}")
```

**AsyncAPI Integration** 🔨:
- Extend CLI to parse AsyncAPI 2.6+ spec (https://github.com/asyncapi/spec)
- Generate WebSocket handlers for Python/Node/Ruby/Rust
- Support pub/sub patterns, message validation
- Generate client SDKs

**Testing**:
- Add `testing_data/websockets/` with AsyncAPI fixtures
- Test connection lifecycle (connect, message, disconnect)
- Test broadcast patterns
- Test binary vs text frames
- Validate message schemas

**Benchmark**:
- Add WebSocket echo server to benchmark-harness
- Measure concurrent connections (target: 10K+)
- Measure message throughput
- Test backpressure handling

---

## 3. OpenAPI Generation 📦

### Current State
- No OpenAPI generation exists
- `SchemaRegistry` has foundation (`all_schemas()`)

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

### Implementation Plan

**Phase 1: Schema Generation**
```rust
// Add to crates/spikard-http/src/openapi.rs
use utoipa::{OpenApi, ToSchema};
use std::sync::OnceLock;

#[derive(OpenApi)]
#[openapi(
    paths(/* auto-registered from routes */),
    components(schemas(/* from SchemaRegistry */))
)]
pub struct ApiDoc;

static CACHED_SPEC: OnceLock<String> = OnceLock::new();

pub fn generate_openapi_spec(routes: &[Route]) -> utoipa::openapi::OpenApi {
    // Convert our Route + SchemaRegistry to utoipa OpenApi
    // Cache on first generation
}
```

**Phase 2: UI Endpoints**
```rust
// Add UI routes
pub fn openapi_routes() -> Router {
    Router::new()
        .route("/api/openapi.json", get(serve_openapi_spec))
        .merge(SwaggerUi::new("/swagger-ui").url("/api/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::new("/api/openapi.json").url("/redoc"))
}
```

**Phase 3: Validation**
- Validate generated spec against OpenAPI 3.1.0 schema in tests
- Use `openapi-schema-validator` to ensure compliance
- Test that all routes appear in spec
- Verify JSON Schema compatibility

**Testing**:
- Add `testing_data/openapi/expected_spec.json`
- Generate spec from e2e app
- Validate spec structure
- Test Swagger UI serves correctly
- Test Redoc serves correctly
- Verify spec includes all routes, schemas, parameters

**Python/Node/Ruby Integration**:
```python
# Auto-register routes in OpenAPI
@app.get("/items/{id}")  # Automatically added to OpenAPI spec
def get_item(id: int) -> Item:
    pass

# Access spec
spec = app.openapi_spec()  # Returns dict
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

## 11. Test Client 🔨

### Current State
- No test client exists

### Implementation Plan

**Create Test Client**:
```rust
// crates/spikard-testing/src/lib.rs
pub struct TestClient {
    app: Router,
    base_url: String,
}

impl TestClient {
    pub fn new(app: Router) -> Self {
        Self {
            app,
            base_url: "http://localhost".to_string(),
        }
    }

    pub async fn get(&self, path: &str) -> TestResponse {
        // Use tower::ServiceExt::oneshot to call app directly
    }

    pub async fn post(&self, path: &str, body: Value) -> TestResponse { /* ... */ }
    // ... put, patch, delete, etc.
}

pub struct TestResponse {
    status: StatusCode,
    headers: HeaderMap,
    body: Bytes,
}

impl TestResponse {
    pub fn json<T: DeserializeOwned>(&self) -> T { /* ... */ }
    pub fn status(&self) -> StatusCode { /* ... */ }
    pub fn assert_status(&self, expected: StatusCode) { /* ... */ }
}
```

**Python API**:
```python
from spikard.testing import TestClient

app = create_app()
client = TestClient(app)

def test_get_item():
    response = client.get("/items/123")
    assert response.status_code == 200
    assert response.json()["id"] == 123
```

**Testing**:
- Test client can call all HTTP methods
- Test request/response body serialization
- Test headers, cookies, auth
- Test assertions helper methods

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

### Phase 1: Quick Wins (Already Available)
1. ✅ **Body Size Limits** - Add `DefaultBodyLimit` layer (1 hour)
2. ✅ **Compression** - Enable tower-http compression (2 hours)
3. ✅ **Request Timeouts** - Add `TimeoutLayer` (2 hours)
4. ✅ **Graceful Shutdown** - Add signal handlers (2 hours)
5. ✅ **Request Logging** - Add request-id middleware (3 hours)

**Total**: ~10 hours, significant production readiness improvement

### Phase 2: External Crates
6. 📦 **Rate Limiting** - Integrate tower_governor (4 hours)
7. 📦 **Authentication** - JWT + API key middleware (6 hours)
8. 📦 **OpenAPI Generation** - Integrate utoipa, Swagger UI, Redoc (12 hours)

**Total**: ~22 hours

### Phase 3: Core Features
9. ✅ **Streaming Responses** - Expose Axum streaming (4 hours)
10. ✅ **Server-Sent Events** - Expose Axum SSE (3 hours)
11. ✅ **Static File Serving** - Add tower-http ServeDir (2 hours)

**Total**: ~9 hours

### Phase 4: Advanced Features
12. ✅ **WebSocket Support** - Enable ws feature + API design (8 hours)
13. 🔨 **AsyncAPI for WebSockets** - CLI extension (20 hours)
14. 🔨 **Test Client** - Build custom test harness (8 hours)
15. 🔨 **Background Tasks** - Design + implement (6 hours)

**Total**: ~42 hours

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

# Middleware, Lifecycle Hooks, and Performance Optimization

**Date:** January 2025 (Updated November 2025)
**Status:** Phase 1 (Middleware) ✅ Complete | Phase 2 (Lifecycle Hooks) 🟡 Pending
**Research-driven design document based on 2024-2025 ecosystem analysis**

## Executive Summary

Based on comprehensive research of the Axum, Fastify, Litestar, PyO3, and napi-rs ecosystems in 2024-2025, this document outlines Spikard's approach to:

1. **Rust-native middleware** using tower-http ✅ **IMPLEMENTED** (battle-tested, production-ready)
2. **Zero-overhead lifecycle hooks** for Python/TypeScript/Ruby plugins 🟡 **PENDING**
3. **Pre-computation and caching** ✅ **IMPLEMENTED** (SchemaRegistry with deduplication)
4. **Conditional feature compilation** ✅ **IMPLEMENTED** (feature flags in Cargo.toml)

**Current Status (November 2025):**
- All core middleware integrated: compression, rate limiting, timeouts, graceful shutdown, static files, request IDs
- Configuration exposed via typed `ServerConfig` struct to Python/Node/Ruby bindings
- Lifecycle hooks designed but not yet implemented - this is the next major feature

## 1. Middleware Architecture (All in Rust)

### Core Principle
**ALL standard middleware lives in Rust** using the tower/tower-http ecosystem.

### Rationale (2024-2025 Research)
- Axum handles 500,000 req/sec on a single core with middleware (2025 benchmarks)
- tower-http is battle-tested and production-ready
- Zero overhead - compiled away when not used
- Rich ecosystem of permissive OSS middleware

### Middleware Stack

```toml
# Cargo.toml dependencies
[dependencies]
tower = "0.5"
tower-http = { version = "0.6", features = ["full"] }
tower-governor = "0.4"  # Rate limiting
```

#### Built-in Middleware (All Rust)

```rust
use tower_http::{
    cors::CorsLayer,
    compression::CompressionLayer,
    trace::TraceLayer,
    timeout::TimeoutLayer,
    limit::RequestBodyLimitLayer,
};
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};

// All middleware pre-configured in Rust
pub struct MiddlewareBuilder {
    cors: Option<CorsLayer>,
    compression: Option<CompressionLayer<Predicate>>,
    trace: Option<TraceLayer>,
    rate_limit: Option<GovernorLayer>,
    timeout: Option<TimeoutLayer>,
    body_limit: Option<RequestBodyLimitLayer>,
}

impl MiddlewareBuilder {
    pub fn with_cors(mut self, config: CorsConfig) -> Self {
        self.cors = Some(CorsLayer::new()
            .allow_origin(config.origins)
            .allow_methods(config.methods)
            .allow_headers(config.headers));
        self
    }

    pub fn with_compression(mut self) -> Self {
        self.compression = Some(CompressionLayer::new());
        self
    }

    pub fn with_rate_limit(mut self, max_burst: u32, per_seconds: u64) -> Self {
        let governor_conf = Box::new(
            GovernorConfigBuilder::default()
                .per_second(per_seconds)
                .burst_size(max_burst)
                .finish()
                .unwrap(),
        );
        self.rate_limit = Some(GovernorLayer { config: Arc::new(governor_conf) });
        self
    }
}
```

#### Available Middleware (tower-http)

All permissive licenses, production-ready as of 2024-2025:

| Middleware | Purpose | Performance Impact |
|------------|---------|-------------------|
| `CorsLayer` | CORS headers | Negligible |
| `CompressionLayer` | Response compression (gzip, brotli, zstd) | Variable (saves bandwidth) |
| `TraceLayer` | Request/response logging | Minimal (~5%) |
| `TimeoutLayer` | Request timeouts | Zero (compile-time) |
| `RequestBodyLimitLayer` | Body size limits | Zero (compile-time) |
| `tower-governor` | Rate limiting | Minimal (~2%) |
| `NormalizePath` | Trailing slash normalization | Zero |
| `PropagateHeader` | Header propagation | Negligible |
| `SetSensitiveHeaders` | Header redaction in logs | Negligible |

### Python API

```python
from spikard import Spikard
from spikard.middleware import Cors, Compression, RateLimit, Tracing

app = Spikard(
    middleware=[
        Cors(
            allow_origins=["https://example.com"],
            allow_methods=["GET", "POST"],
            allow_headers=["Content-Type"],
        ),
        Compression(algorithms=["gzip", "brotli"]),
        RateLimit(max_requests=100, per_seconds=60),
        Tracing(level="info"),
    ]
)
```

### TypeScript API

```typescript
import { Spikard, Cors, Compression, RateLimit } from 'spikard';

const app = new Spikard({
  middleware: [
    new Cors({
      allowOrigins: ['https://example.com'],
      allowMethods: ['GET', 'POST'],
    }),
    new Compression({ algorithms: ['gzip', 'brotli'] }),
    new RateLimit({ maxRequests: 100, perSeconds: 60 }),
  ]
});
```

## 2. Lifecycle Hooks (Fastify-inspired)

### Research Insights (Fastify 2024-2025)

Fastify's lifecycle hooks execute in this order:
```
Incoming Request
  → onRequest
  → preParsing
  → Parsing
  → preValidation
  → Validation
  → preHandler
  → Handler
  → preSerialization
  → onSend
  → onResponse
```

### Spikard Lifecycle Hooks

We adapt this for Spikard with **conditional execution** to avoid overhead:

```
Incoming Request
  ↓
┌─────────────────────────┐
│  Rust Middleware        │  Always executes
│  (CORS, compression)    │
└────────┬────────────────┘
         ↓
┌─────────────────────────┐
│  onRequest Hook         │  ← Conditional: Skip if no Python/TS hooks registered
└────────┬────────────────┘
         ↓
┌─────────────────────────┐
│  Routing (Rust)         │  Always executes
└────────┬────────────────┘
         ↓
┌─────────────────────────┐
│  preValidation Hook     │  ← Conditional
└────────┬────────────────┘
         ↓
┌─────────────────────────┐
│  Validation (Rust)      │  Always executes
└────────┬────────────────┘
         ↓
┌─────────────────────────┐
│  preHandler Hook        │  ← Conditional
└────────┬────────────────┘
         ↓
┌─────────────────────────┐
│  DI Resolution          │  ← Conditional: Skip if no dependencies
└────────┬────────────────┘
         ↓
┌─────────────────────────┐
│  Handler                │  Always executes
└────────┬────────────────┘
         ↓
┌─────────────────────────┐
│  onResponse Hook        │  ← Conditional
└─────────────────────────┘
```

### Zero-Cost Conditional Execution

**Key Insight:** Use Rust feature flags and runtime checks to skip expensive FFI calls.

```rust
pub struct LifecycleHooks {
    // Option<T> compiles to zero size if None
    on_request: Option<Arc<dyn Fn(Request) -> Result<Request>>>,
    pre_validation: Option<Arc<dyn Fn(Request) -> Result<Request>>>,
    pre_handler: Option<Arc<dyn Fn(Request) -> Result<Request>>>,
    on_response: Option<Arc<dyn Fn(Response) -> Result<Response>>>,
}

impl LifecycleHooks {
    #[inline(always)]
    async fn call_on_request(&self, req: Request) -> Result<Request> {
        // Zero-cost check: compiles to a simple null pointer check
        match &self.on_request {
            Some(hook) => hook(req),  // Only executes FFI if hook exists
            None => Ok(req),          // Fast path: no-op
        }
    }
}
```

**Performance:** Null pointer check is ~0.5ns (negligible).

### Python Lifecycle Hooks API

```python
from spikard import Spikard, Request, Response

async def on_request_hook(request: Request) -> Request:
    """Called before routing."""
    print(f"Incoming: {request.method} {request.url}")
    return request

async def pre_handler_hook(request: Request) -> Request:
    """Called after validation, before handler."""
    # Add request context, transform data, etc.
    request.state["user_id"] = extract_user_id(request)
    return request

async def on_response_hook(response: Response) -> Response:
    """Called before sending response."""
    response.headers["X-Process-Time"] = str(response.process_time)
    return response

app = Spikard(
    lifecycle_hooks={
        "on_request": [on_request_hook],
        "pre_handler": [pre_handler_hook],
        "on_response": [on_response_hook],
    }
)
```

### TypeScript Lifecycle Hooks API

```typescript
import { Spikard, Request, Response } from 'spikard';

const onRequestHook = async (request: Request): Promise<Request> => {
  console.log(`Incoming: ${request.method} ${request.url}`);
  return request;
};

const preHandlerHook = async (request: Request): Promise<Request> => {
  request.state.userId = extractUserId(request);
  return request;
};

const app = new Spikard({
  lifecycleHooks: {
    onRequest: [onRequestHook],
    preHandler: [preHandlerHook],
  }
});
```

## 3. Performance Optimization Strategies

### 3.1 Schema Pre-Computation (Startup Only)

**Research Insight (2024):** Schema compilation provides 10x+ performance improvement. Load once, reuse forever.

```rust
pub struct SchemaCache {
    // Compiled validators - computed once at startup
    validators: HashMap<String, Arc<jsonschema::Validator>>,
}

impl SchemaCache {
    pub fn new(routes: &[Route]) -> Self {
        let mut validators = HashMap::new();

        for route in routes {
            // Pre-compile all JSON schemas at startup
            if let Some(schema) = &route.parameter_schema {
                let validator = jsonschema::Validator::new(schema)
                    .expect("Invalid schema");
                validators.insert(
                    route.handler_name.clone(),
                    Arc::new(validator)
                );
            }
        }

        Self { validators }
    }
}
```

**Performance:**
- Startup: +50-100ms (one-time)
- Runtime: 10x faster validation (no schema parsing)

### 3.2 Dependency Injection Pre-Computation

**See:** [07-dependency-injection.md](./07-dependency-injection.md) for full DI architecture.

**Performance:**
- Startup: +10-50ms (DI graph building in Rust)
- Runtime: O(1) dependency resolution (pre-computed graph)
- Singleton cache: ~5ns lookup
- Request scope: ~5ns (cached) or ~20-40ns (factory call)

### 3.3 Conditional Compilation (Feature Flags)

**Research Insight (Rust 2024):** Feature flags have zero runtime cost.

```toml
[features]
default = ["cors", "compression", "tracing"]
cors = ["tower-http/cors"]
compression = ["tower-http/compression-full"]
tracing = ["tower-http/trace"]
rate-limit = ["tower-governor"]
python-hooks = []  # Enable lifecycle hooks for Python
typescript-hooks = []  # Enable lifecycle hooks for TypeScript

# Users can disable features they don't need:
# cargo build --no-default-features --features="cors"
```

```rust
#[cfg(feature = "python-hooks")]
pub struct PythonHooks {
    // Only compiled if python-hooks feature is enabled
    on_request: Option<Py<PyAny>>,
}

#[cfg(not(feature = "python-hooks"))]
pub struct PythonHooks;  // Zero-size type

impl PythonHooks {
    #[cfg(feature = "python-hooks")]
    #[inline(always)]
    pub async fn call_on_request(&self, req: Request) -> Result<Request> {
        match &self.on_request {
            Some(hook) => {
                // Only executed if hook is registered AND feature enabled
                Python::with_gil(|py| {
                    let result = hook.call1(py, (req,))?;
                    result.extract(py)
                })
            },
            None => Ok(req),
        }
    }

    #[cfg(not(feature = "python-hooks"))]
    #[inline(always)]
    pub async fn call_on_request(&self, req: Request) -> Result<Request> {
        Ok(req)  // No-op, compiles to nothing
    }
}
```

**Performance:** Feature-gated code compiles to zero bytes when disabled.

### 3.3 PyO3/napi-rs Overhead Mitigation

**Research Insights (2024):**
- PyO3: 20-40ns call overhead (acceptable for non-hot-path)
- napi-rs: Lower overhead, 2x faster than Neon

**Strategy:** Minimize FFI boundary crossings by batching operations when possible.

For DI, the Rust engine resolves all dependencies in one pass, minimizing FFI calls. See [07-dependency-injection.md](./07-dependency-injection.md) for details.

## 4. Implementation Strategy

### Phase 1: Rust Middleware ✅ COMPLETE (November 2025)

Implemented in `crates/spikard-http/src/server.rs`:

- ✅ Request ID generation (UUID-based, X-Request-ID with PropagateRequestIdLayer)
- ✅ Response compression (gzip, brotli via CompressionLayer)
- ✅ Request timeouts (configurable via TimeoutLayer)
- ✅ Body size limits (configurable via DefaultBodyLimit)
- ✅ Rate limiting (IP-based via tower_governor GovernorLayer)
- ✅ Graceful shutdown (SIGTERM/SIGINT signal handlers)
- ✅ Static file serving (ServeDir with cache-control headers)
- ✅ Sensitive header hiding (Authorization, Cookie via SetSensitiveRequestHeadersLayer)

**Configuration:** All middleware exposed via `ServerConfig` struct with typed options:
- `CompressionConfig` - gzip/brotli settings
- `RateLimitConfig` - per_second, burst, ip_based
- `StaticFilesConfig` - directory, route_prefix, cache_control
- Request/response limits, timeouts, graceful shutdown timeout

**Middleware Stack Order (reverse order - last added = first executed):**
1. Request ID (outermost)
2. Timeout
3. Rate Limit
4. Compression
5. Sensitive Headers
6. Body Limit

**Bindings:** Python/Node/Ruby use `ServerConfig::default()` with field overrides.

**Status:** Production-ready. All tower-http middleware integrated and tested.

### Phase 2: Lifecycle Hooks (Pending)
- [ ] Implement hook registration system
- [ ] Add conditional execution (skip if no hooks registered)
- [ ] Python API with async support (onRequest, preValidation, preHandler, onResponse)
- [ ] TypeScript API with async support
- [ ] Ruby API with fiber support

**Design:** Hooks will be optional Arc<dyn Fn> closures checked at runtime with ~0.5ns overhead when not registered.

### Phase 3: Pre-Computation ✅ COMPLETE
- ✅ Schema caching at startup (SchemaRegistry with Arc-wrapped validators)
- ✅ Route pre-compilation (Route::from_metadata with validator deduplication)
- [ ] DI graph pre-computation (see dependency-injection.md)
- [ ] Benchmark suite improvements

### Phase 4: Authentication Middleware (Pending)
- [ ] JWT authentication (jsonwebtoken crate integrated)
- [ ] API Key authentication (config struct ready)
- [ ] Hook into preHandler phase
- [ ] Expose via Python/TypeScript/Ruby APIs

### Phase 5: Optimization (Ongoing)
- [ ] Profile PyO3/napi-rs overhead
- [ ] Minimize FFI boundary crossings
- [ ] Add comprehensive benchmarking suite

## 5. Performance Targets (Based on 2024-2025 Research)

| Metric | Target | Source |
|--------|--------|--------|
| Requests/sec (with middleware) | 500k+ on single core | Axum 2025 benchmarks |
| Middleware overhead | <5% | tower-http production data |
| Schema validation | 10x faster than runtime | JSON Schema research 2024 |
| DI resolution | O(1) per request | Litestar architecture |
| Feature flag overhead | 0ns (compile-time) | Rust conditional compilation |
| Null hook check | <1ns | Rust pointer check |
| PyO3 call overhead | 20-40ns | PyO3 2024 benchmarks |
| napi-rs call overhead | <20ns | napi-rs 2024 benchmarks |

## 6. Code Vendoring Strategy

### From Litestar (Python DI)
We can vendor under MIT license:
- `litestar.di` - Dependency injection core
- `litestar._signature` - Signature parsing
- `litestar.params` - Parameter definitions

### Alternative: Implement in Rust
For better performance, implement DI graph resolution in Rust:
```rust
// Rust-based DI - better performance than Python
pub struct DependencyResolver {
    graph: DiGraph<DepNode, ()>,
    cache: HashMap<TypeId, Arc<dyn Any>>,
}
```

**Recommendation:** Start with Litestar vendoring for speed, optimize to Rust later if needed.

## 7. Next Steps (November 2025)

### Immediate Priorities

1. **Implement Lifecycle Hooks System** (3-5 days)
   - [ ] Design hook registration API for Python/TypeScript/Ruby
   - [ ] Implement conditional execution in Rust (skip if no hooks)
   - [ ] Add Python async hook support via pyo3_async_runtimes
   - [ ] Add TypeScript async hook support via ThreadsafeFunction
   - [ ] Add Ruby fiber-based hook support
   - [ ] Update e2e tests with lifecycle hook examples

2. **Implement Authentication Middleware** (2-3 days)
   - [ ] JWT authentication using jsonwebtoken crate
   - [ ] API Key authentication using existing config structs
   - [ ] Integrate with preHandler lifecycle hook
   - [ ] Add Python/TypeScript/Ruby configuration APIs
   - [ ] Add comprehensive tests for auth flows

3. **Implement OpenAPI Generation** (3-5 days)
   - [ ] Integrate utoipa for OpenAPI 3.1.0 generation
   - [ ] Add Swagger UI endpoint
   - [ ] Add Redoc endpoint
   - [ ] Cache schema on first use
   - [ ] Validate generated schema in tests

4. **Forward Typed Configs to Bindings** (2-3 days)
   - [ ] Python: Add ServerConfig pydantic model
   - [ ] TypeScript: Add ServerConfig interface with Zod validation
   - [ ] Ruby: Add ServerConfig struct with type checking
   - [ ] Document all config options in each language

### Long-term

5. **Test Client for Integration Testing** (2-3 days)
6. **WebSocket Support with AsyncAPI** (5-7 days)
7. **Server-Sent Events (SSE)** (2-3 days)
8. **Streaming Responses** (2-3 days)
9. **Background Tasks** (3-5 days)

---

**Key Takeaway:** By keeping all standard middleware in Rust (using battle-tested tower-http) ✅ DONE, conditionally invoking lifecycle hooks 🟡 NEXT, and pre-computing schemas at startup ✅ DONE, we achieve near-zero overhead while maintaining full flexibility for user plugins.

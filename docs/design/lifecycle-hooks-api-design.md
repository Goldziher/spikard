# Lifecycle Hooks API Design

## Overview

This document defines the lifecycle hooks API for Spikard, inspired by Fastify's hook system.

**Architecture**: Rust-first design where all language bindings (Python, TypeScript, Ruby, PHP) are thin wrappers over the Rust core.

The design prioritizes:

1. **Rust-first API** - Core implementation in Rust with zero-cost abstractions
2. **Full type safety** - Compile-time guarantees in Rust, runtime checks in bindings
3. **Idiomatic patterns** - Each language binding follows native conventions
4. **Consistent semantics** - Same behavior across all bindings
5. **Zero-cost when unused** - ~0.5ns overhead when no hooks registered

## Hook Points

### Request Phase Hooks
These hooks receive `Request` and can return `Request | Response`:

1. **`onRequest`** - Runs immediately after receiving request, before routing
   - Use case: Early auth checks, request logging, adding request IDs
   - Can inspect: method, path, headers, query params (unparsed)
   - Can short-circuit: Yes, return Response

2. **`preValidation`** - Runs after routing but before schema validation
   - Use case: Transform request data before validation, route-specific auth
   - Can inspect: matched route, all request data
   - Can short-circuit: Yes, return Response

3. **`preHandler`** - Runs after validation, immediately before handler
   - Use case: Load user context, set up request state, final auth checks
   - Can inspect: validated request data, matched route
   - Can short-circuit: Yes, return Response

### Response Phase Hooks
These hooks receive `Response` and must return `Response`:

4. **`onResponse`** - Runs after handler, before sending response
   - Use case: Add security headers, response logging, metrics
   - Can modify: headers, cookies, response body
   - Can short-circuit: No (response already determined)

5. **`onError`** - Runs when errors occur in any phase
   - Use case: Custom error formatting, error logging, alerting
   - Can modify: error response entirely
   - Can short-circuit: No (already in error state)

## Hook Execution Order

```
┌─────────────────────────────────────────────────────────────┐
│ HTTP Request Received                                        │
└───────────────────┬──────────────────────────────────────────┘
                    │
                    ▼
            ┌───────────────┐
            │  onRequest    │ ◄─── Can short-circuit
            │  hooks (N)    │      Return Response to skip routing
            └───────┬───────┘
                    │
                    ▼
            ┌───────────────┐
            │  Route Match  │
            └───────┬───────┘
                    │
                    ▼
            ┌───────────────┐
            │ preValidation │ ◄─── Can short-circuit
            │  hooks (N)    │      Return Response to skip validation
            └───────┬───────┘
                    │
                    ▼
            ┌───────────────┐
            │  Validation   │ ───► onError hooks (if validation fails)
            └───────┬───────┘
                    │
                    ▼
            ┌───────────────┐
            │  preHandler   │ ◄─── Can short-circuit
            │  hooks (N)    │      Return Response to skip handler
            └───────┬───────┘
                    │
                    ▼
            ┌───────────────┐
            │    Handler    │ ───► onError hooks (if handler fails)
            │   Execution   │
            └───────┬───────┘
                    │
                    ▼
            ┌───────────────┐
            │  onResponse   │
            │  hooks (N)    │
            └───────┬───────┘
                    │
                    ▼
┌───────────────────────────────────────────────────────────────┐
│ HTTP Response Sent                                            │
└───────────────────────────────────────────────────────────────┘

Note: (N) means multiple hooks can be registered and run in order
```

## Hook Semantics

### Multiple Hooks
- Multiple hooks can be registered for the same hook point
- Hooks execute in registration order
- Each hook receives the output of the previous hook
- First short-circuit stops execution

### Short-Circuiting
Request phase hooks can short-circuit by returning a `Response`:
```python
async def auth_hook(request: Request) -> Request | Response:
    if not authorized(request):
        return Response({"error": "Unauthorized"}, status_code=401)
    return request  # Continue to next hook/phase
```

### Error Handling
- If a hook raises an exception, `onError` hooks are invoked
- `onError` hooks receive the error response
- `onError` hooks cannot throw (errors are logged)

### State Passing
State can be passed between hooks and handlers using extensible context mechanisms (implementation varies by language).

## Rust API Design (Core)

**The Rust API is the foundation for all language bindings.** All other languages wrap this core implementation.

### Core Types (Already Implemented)

Located in `crates/spikard-http/src/lifecycle.rs`:

```rust
use axum::{body::Body, http::{Request, Response}};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Result of a lifecycle hook execution
#[derive(Debug)]
pub enum HookResult<T> {
    /// Continue to the next phase with the (possibly modified) value
    Continue(T),
    /// Short-circuit the request pipeline and return this response immediately
    ShortCircuit(Response<Body>),
}

/// Trait for lifecycle hooks
///
/// Language bindings (Python, TypeScript, Ruby) implement this trait to wrap
/// their functions and make them callable from Rust.
pub trait LifecycleHook: Send + Sync {
    /// Hook name for debugging and error messages
    fn name(&self) -> &str;

    /// Execute hook with a request (for onRequest, preValidation, preHandler)
    fn execute_request<'a>(
        &'a self,
        req: Request<Body>,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>>, String>> + Send + 'a>>;

    /// Execute hook with a response (for onResponse, onError)
    fn execute_response<'a>(
        &'a self,
        resp: Response<Body>,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Response<Body>>, String>> + Send + 'a>>;
}

/// Container for all lifecycle hooks
#[derive(Default, Clone)]
pub struct LifecycleHooks {
    /// Hooks that run before routing
    on_request: Vec<Arc<dyn LifecycleHook>>,

    /// Hooks that run after routing, before validation
    pre_validation: Vec<Arc<dyn LifecycleHook>>,

    /// Hooks that run after validation, before handler
    pre_handler: Vec<Arc<dyn LifecycleHook>>,

    /// Hooks that run after handler execution
    on_response: Vec<Arc<dyn LifecycleHook>>,

    /// Hooks that run when errors occur
    on_error: Vec<Arc<dyn LifecycleHook>>,
}
```

### Native Rust Hooks (Pure Rust API)

For pure Rust applications, hooks can be simple functions or closures:

```rust
use spikard_http::{LifecycleHook, LifecycleHooks, HookResult};
use axum::{body::Body, http::{Request, Response, StatusCode}};
use std::sync::Arc;

// Define a hook as a struct implementing LifecycleHook
struct RequestLogger {
    name: String,
}

impl LifecycleHook for RequestLogger {
    fn name(&self) -> &str {
        &self.name
    }

    fn execute_request<'a>(
        &'a self,
        req: Request<Body>,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>>, String>> + Send + 'a>> {
        Box::pin(async move {
            println!("{} {}", req.method(), req.uri().path());
            Ok(HookResult::Continue(req))
        })
    }

    fn execute_response<'a>(
        &'a self,
        resp: Response<Body>,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Response<Body>>, String>> + Send + 'a>> {
        Box::pin(async move {
            Ok(HookResult::Continue(resp))
        })
    }
}

// Build lifecycle hooks
let mut hooks = LifecycleHooks::new();
hooks.add_on_request(Arc::new(RequestLogger {
    name: "request_logger".to_string(),
}));
```

### Functional Hook Builder (Ergonomic Rust API)

For a more ergonomic Rust API, we can provide builder functions:

```rust
// In crates/spikard-http/src/lifecycle.rs

use std::future::Future;

/// Create a request hook from an async function
pub fn request_hook<F, Fut>(name: impl Into<String>, f: F) -> Arc<dyn LifecycleHook>
where
    F: Fn(Request<Body>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<HookResult<Request<Body>>, String>> + Send + 'static,
{
    struct FnHook<F> {
        name: String,
        func: F,
    }

    impl<F, Fut> LifecycleHook for FnHook<F>
    where
        F: Fn(Request<Body>) -> Fut + Send + Sync,
        Fut: Future<Output = Result<HookResult<Request<Body>>, String>> + Send + 'static,
    {
        fn name(&self) -> &str {
            &self.name
        }

        fn execute_request<'a>(
            &'a self,
            req: Request<Body>,
        ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>>, String>> + Send + 'a>> {
            Box::pin((self.func)(req))
        }

        fn execute_response<'a>(
            &'a self,
            _resp: Response<Body>,
        ) -> Pin<Box<dyn Future<Output = Result<HookResult<Response<Body>>, String>> + Send + 'a>> {
            Box::pin(async move {
                Err("Request hook called with response".to_string())
            })
        }
    }

    Arc::new(FnHook {
        name: name.into(),
        func: f,
    })
}

/// Create a response hook from an async function
pub fn response_hook<F, Fut>(name: impl Into<String>, f: F) -> Arc<dyn LifecycleHook>
where
    F: Fn(Response<Body>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<HookResult<Response<Body>>, String>> + Send + 'static,
{
    struct FnHook<F> {
        name: String,
        func: F,
    }

    impl<F, Fut> LifecycleHook for FnHook<F>
    where
        F: Fn(Response<Body>) -> Fut + Send + Sync,
        Fut: Future<Output = Result<HookResult<Response<Body>>, String>> + Send + 'static,
    {
        fn name(&self) -> &str {
            &self.name
        }

        fn execute_request<'a>(
            &'a self,
            _req: Request<Body>,
        ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>>, String>> + Send + 'a>> {
            Box::pin(async move {
                Err("Response hook called with request".to_string())
            })
        }

        fn execute_response<'a>(
            &'a self,
            resp: Response<Body>,
        ) -> Pin<Box<dyn Future<Output = Result<HookResult<Response<Body>>, String>> + Send + 'a>> {
            Box::pin((self.func)(resp))
        }
    }

    Arc::new(FnHook {
        name: name.into(),
        func: f,
    })
}
```

### Usage Example (Pure Rust)

```rust
use spikard_http::{
    LifecycleHooks, HookResult, request_hook, response_hook,
    ServerConfig,
};
use axum::{body::Body, http::{Request, Response, StatusCode, HeaderValue}};

#[tokio::main]
async fn main() {
    let mut hooks = LifecycleHooks::new();

    // Add request logger
    hooks.add_on_request(request_hook("request_logger", |req| async move {
        println!("[{}] {} {}",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
            req.method(),
            req.uri().path()
        );
        Ok(HookResult::Continue(req))
    }));

    // Add authentication hook
    hooks.add_pre_handler(request_hook("auth", |req| async move {
        // Skip public routes
        if req.uri().path().starts_with("/public") {
            return Ok(HookResult::Continue(req));
        }

        // Check authorization header
        match req.headers().get("authorization") {
            Some(auth_header) if auth_header.to_str().ok()?.starts_with("Bearer ") => {
                let token = auth_header.to_str().ok()?.trim_start_matches("Bearer ");

                // Validate token (example)
                if validate_token(token).await.is_ok() {
                    Ok(HookResult::Continue(req))
                } else {
                    let response = Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(Body::from(r#"{"error":"Invalid token"}"#))
                        .unwrap();
                    Ok(HookResult::ShortCircuit(response))
                }
            }
            _ => {
                let response = Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::from(r#"{"error":"Missing authorization"}"#))
                    .unwrap();
                Ok(HookResult::ShortCircuit(response))
            }
        }
    }));

    // Add security headers to responses
    hooks.add_on_response(response_hook("security_headers", |mut resp| async move {
        let headers = resp.headers_mut();
        headers.insert(
            "X-Content-Type-Options",
            HeaderValue::from_static("nosniff")
        );
        headers.insert(
            "X-Frame-Options",
            HeaderValue::from_static("DENY")
        );
        headers.insert(
            "X-XSS-Protection",
            HeaderValue::from_static("1; mode=block")
        );
        Ok(HookResult::Continue(resp))
    }));

    // Add error logger
    hooks.add_on_error(response_hook("error_logger", |resp| async move {
        if resp.status().as_u16() >= 500 {
            eprintln!("Server error: {}", resp.status());
            // Send to monitoring service
            // sentry::capture_error(&resp).await;
        }
        Ok(HookResult::Continue(resp))
    }));

    // Create server config with hooks
    let config = ServerConfig {
        host: "0.0.0.0".to_string(),
        port: 3000,
        lifecycle_hooks: Some(hooks),
        ..Default::default()
    };

    // Start server
    spikard_http::serve(config).await.unwrap();
}

async fn validate_token(token: &str) -> Result<(), ()> {
    // Token validation logic
    Ok(())
}
```

### Builder Pattern for Complex Hooks

```rust
use spikard_http::{LifecycleHooks, HookBuilder};

let hooks = LifecycleHooks::builder()
    // Add multiple onRequest hooks
    .on_request(request_hook("request_id", |mut req| async move {
        req.headers_mut().insert(
            "X-Request-ID",
            HeaderValue::from_str(&uuid::Uuid::new_v4().to_string()).unwrap()
        );
        Ok(HookResult::Continue(req))
    }))
    .on_request(request_hook("cors", |req| async move {
        // CORS handling
        Ok(HookResult::Continue(req))
    }))

    // Add preValidation hook
    .pre_validation(request_hook("rate_limit", |req| async move {
        if is_rate_limited(&req).await {
            let response = Response::builder()
                .status(StatusCode::TOO_MANY_REQUESTS)
                .header("Retry-After", "60")
                .body(Body::from(r#"{"error":"Rate limit exceeded"}"#))
                .unwrap();
            return Ok(HookResult::ShortCircuit(response));
        }
        Ok(HookResult::Continue(req))
    }))

    // Add preHandler hook
    .pre_handler(request_hook("auth", |req| async move {
        // Authentication logic
        Ok(HookResult::Continue(req))
    }))

    // Add onResponse hooks
    .on_response(response_hook("timing", |mut resp| async move {
        // Add timing header
        resp.headers_mut().insert(
            "X-Response-Time",
            HeaderValue::from_str("123ms").unwrap()
        );
        Ok(HookResult::Continue(resp))
    }))
    .on_response(response_hook("security", |mut resp| async move {
        // Add security headers
        Ok(HookResult::Continue(resp))
    }))

    // Add error hook
    .on_error(response_hook("error_handler", |resp| async move {
        // Error handling
        Ok(HookResult::Continue(resp))
    }))

    .build();
```

### State Extensions (Tower Layer Pattern)

For passing data between hooks, use Axum's extension system:

```rust
use axum::{Extension, extract::State};
use std::sync::Arc;

#[derive(Clone)]
struct User {
    id: u64,
    name: String,
    role: String,
}

// In pre_handler hook, add user to extensions
hooks.add_pre_handler(request_hook("load_user", |mut req| async move {
    let token = req.headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "));

    if let Some(token) = token {
        if let Ok(user) = authenticate_token(token).await {
            // Add user to request extensions
            req.extensions_mut().insert(user);
        }
    }

    Ok(HookResult::Continue(req))
}));

// In handler, extract user from extensions
async fn profile_handler(
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    Json(json!({
        "id": user.id,
        "name": user.name,
        "role": user.role,
    }))
}
```

### Type-Safe Hook Context

For more complex state passing, define a context type:

```rust
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct HookContext {
    pub user: Option<User>,
    pub request_id: Option<String>,
    pub start_time: Option<std::time::Instant>,
    pub custom: HashMap<String, String>,
}

// Store in request extensions
req.extensions_mut().insert(HookContext {
    request_id: Some(uuid::Uuid::new_v4().to_string()),
    start_time: Some(std::time::Instant::now()),
    ..Default::default()
});

// Access in later hooks
if let Some(ctx) = req.extensions().get::<HookContext>() {
    println!("Request ID: {:?}", ctx.request_id);
}
```

### Integration with ServerConfig

```rust
// In crates/spikard-http/src/lib.rs

pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    // ... other config fields ...

    /// Lifecycle hooks for request/response processing
    pub lifecycle_hooks: Option<LifecycleHooks>,
}

impl ServerConfig {
    /// Builder for adding hooks
    pub fn with_hooks(mut self, hooks: LifecycleHooks) -> Self {
        self.lifecycle_hooks = Some(hooks);
        self
    }
}
```

### Language Binding Integration

Each language binding implements `LifecycleHook` for their functions:

**Python** (in `crates/spikard-py/src/lifecycle.rs`):
```rust
pub struct PythonHook {
    name: String,
    func: Py<PyAny>,  // Python function
}

impl LifecycleHook for PythonHook {
    // Convert Python async function to Rust Future
    // Handle Request/Response conversion
}
```

**TypeScript** (in `crates/spikard-node/src/lifecycle.rs`):
```rust
pub struct TypeScriptHook {
    name: String,
    func: napi::threadsafe_function::ThreadsafeFunction<...>,
}

impl LifecycleHook for TypeScriptHook {
    // Call JS function via ThreadsafeFunction
    // Handle Request/Response conversion
}
```

**Ruby** (in `crates/spikard-rb/src/lifecycle.rs`):
```rust
pub struct RubyHook {
    name: String,
    func: magnus::Value,  // Ruby proc/lambda
}

impl LifecycleHook for RubyHook {
    // Call Ruby block
    // Handle Request/Response conversion
}
```

### Performance Characteristics

- **No hooks**: ~0.5ns overhead (single `is_empty()` check)
- **With hooks**: O(n) where n = number of hooks
- **Async hooks**: ~1-5μs per hook (tokio spawn overhead)
- **Short-circuit**: Stops immediately, no further hooks execute

### Testing Hooks

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_request_hook_continues() {
        let mut hooks = LifecycleHooks::new();

        hooks.add_on_request(request_hook("test", |req| async move {
            Ok(HookResult::Continue(req))
        }));

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();

        assert!(matches!(result, HookResult::Continue(_)));
    }

    #[tokio::test]
    async fn test_request_hook_short_circuits() {
        let mut hooks = LifecycleHooks::new();

        hooks.add_on_request(request_hook("auth", |_req| async move {
            let response = Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::empty())
                .unwrap();
            Ok(HookResult::ShortCircuit(response))
        }));

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();

        match result {
            HookResult::ShortCircuit(resp) => {
                assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
            }
            _ => panic!("Expected ShortCircuit"),
        }
    }
}
```

## Python API Design (Binding)

### Type Definitions

```python
from typing import Protocol, TypeAlias, Awaitable, Union
from spikard import Request, Response

# Hook types
RequestHook: TypeAlias = Callable[[Request], Union[Request, Response, Awaitable[Union[Request, Response]]]]
ResponseHook: TypeAlias = Callable[[Response], Union[Response, Awaitable[Response]]]

class LifecycleHooks(TypedDict, total=False):
    """Hook configuration for routes."""
    on_request: list[RequestHook]
    pre_validation: list[RequestHook]
    pre_handler: list[RequestHook]
    on_response: list[ResponseHook]
    on_error: list[ResponseHook]
```

### Global Hooks Registration

```python
from spikard import Spikard, Request, Response

app = Spikard()

# Method 1: Using decorators (recommended)
@app.on_request
async def log_request(request: Request) -> Request:
    """Log all incoming requests."""
    print(f"{request.method} {request.path}")
    return request

@app.pre_handler
async def check_auth(request: Request) -> Request | Response:
    """Verify authentication before handlers."""
    if not request.headers.get("Authorization"):
        return Response(
            content={"error": "Missing authorization"},
            status_code=401
        )

    # Validate token and load user
    user = await authenticate(request.headers["Authorization"])
    if not user:
        return Response(
            content={"error": "Invalid token"},
            status_code=401
        )

    # Add user to request state for handlers
    request.state["user"] = user
    return request

@app.on_response
async def add_security_headers(response: Response) -> Response:
    """Add security headers to all responses."""
    response.headers["X-Content-Type-Options"] = "nosniff"
    response.headers["X-Frame-Options"] = "DENY"
    response.headers["X-XSS-Protection"] = "1; mode=block"
    return response

@app.on_error
async def log_errors(response: Response) -> Response:
    """Log all errors."""
    if response.status_code >= 500:
        await log_to_monitoring(response)
    return response

# Method 2: Using add_hook (functional style)
async def cors_handler(request: Request) -> Request:
    request.state["cors_origin"] = request.headers.get("Origin", "*")
    return request

app.add_hook("on_request", cors_handler)

# Method 3: Sync hooks also supported
@app.on_request
def add_request_id(request: Request) -> Request:
    """Add unique request ID (sync function)."""
    request.state["request_id"] = generate_uuid()
    return request
```

### Per-Route Hooks

```python
from spikard import get, post, Request, Response

# Hooks specific to this route
@get("/api/admin/users", hooks={
    "pre_handler": [require_admin_role, rate_limit_admin],
})
async def list_admin_users(request: Request):
    """List all users (admin only)."""
    return {"users": await db.get_all_users()}

# Multiple hooks per hook point
@post("/api/payments", hooks={
    "pre_validation": [validate_payment_method, check_fraud],
    "pre_handler": [verify_payment_credentials, calculate_fees],
    "on_response": [log_payment_attempt, send_receipt],
})
async def process_payment(amount: float, method: str):
    """Process payment."""
    result = await payment_processor.charge(amount, method)
    return {"transaction_id": result.id}

# Combine global + route hooks
@app.on_request  # Runs for ALL routes
async def global_auth(request: Request) -> Request | Response:
    # Global auth logic
    return request

@get("/protected", hooks={
    "pre_handler": [additional_check]  # Runs AFTER global_auth
})
async def protected_endpoint():
    return {"data": "secret"}
```

### Hook Configuration in App

```python
# Pass hooks during app initialization
app = Spikard(
    config=ServerConfig(host="0.0.0.0", port=8000),
    hooks={
        "on_request": [request_id_middleware, cors_middleware],
        "on_response": [security_headers, compress_response],
        "on_error": [error_logger, sentry_reporter],
    }
)

# Or add hooks after initialization
app.add_hook("pre_handler", auth_middleware)
app.add_hook("on_response", metrics_collector)
```

### Full Example

```python
from spikard import Spikard, Request, Response, get, post
from spikard.config import ServerConfig
from typing import Any

app = Spikard(config=ServerConfig(port=3000))

# ============================================================================
# Global Hooks
# ============================================================================

@app.on_request
async def request_logger(request: Request) -> Request:
    """Log every request."""
    print(f"[{datetime.now()}] {request.method} {request.path}")
    request.state["start_time"] = time.time()
    return request

@app.on_request
def add_request_id(request: Request) -> Request:
    """Add unique request ID."""
    request.state["request_id"] = str(uuid.uuid4())
    request.headers["X-Request-ID"] = request.state["request_id"]
    return request

@app.pre_handler
async def authenticate(request: Request) -> Request | Response:
    """Check authentication for protected routes."""
    # Skip auth for public routes
    if request.path.startswith("/public"):
        return request

    auth_header = request.headers.get("Authorization")
    if not auth_header or not auth_header.startswith("Bearer "):
        return Response(
            content={"error": "Missing or invalid authorization"},
            status_code=401
        )

    token = auth_header.replace("Bearer ", "")
    user = await validate_token(token)

    if not user:
        return Response(
            content={"error": "Invalid token"},
            status_code=401
        )

    request.state["user"] = user
    return request

@app.on_response
async def add_timing_header(response: Response) -> Response:
    """Add response time header."""
    # Access request state from response context
    if "start_time" in response.state:
        duration = time.time() - response.state["start_time"]
        response.headers["X-Response-Time"] = f"{duration*1000:.2f}ms"
    return response

@app.on_error
async def error_handler(response: Response) -> Response:
    """Format and log errors."""
    if response.status_code >= 500:
        # Log to monitoring
        await sentry.capture_error(response)

    # Ensure consistent error format
    if not response.content or not isinstance(response.content, dict):
        response.content = {
            "error": "Internal server error",
            "request_id": response.state.get("request_id")
        }

    return response

# ============================================================================
# Route-Specific Hooks
# ============================================================================

async def require_admin(request: Request) -> Request | Response:
    """Require admin role."""
    user = request.state.get("user")
    if not user or user.role != "admin":
        return Response(
            content={"error": "Admin access required"},
            status_code=403
        )
    return request

async def rate_limit(request: Request) -> Request | Response:
    """Rate limit expensive operations."""
    user_id = request.state.get("user", {}).get("id")
    if await rate_limiter.is_limited(user_id):
        return Response(
            content={"error": "Rate limit exceeded"},
            status_code=429,
            headers={"Retry-After": "60"}
        )
    return request

# ============================================================================
# Routes
# ============================================================================

@get("/public/health")
async def health_check():
    """Public health check endpoint."""
    return {"status": "ok"}

@get("/api/profile")
async def get_profile(request: Request):
    """Get user profile (requires auth via global hook)."""
    user = request.state["user"]
    return {"user": user.to_dict()}

@post("/api/admin/users", hooks={
    "pre_handler": [require_admin, rate_limit],
})
async def create_user(request: Request, name: str, email: str):
    """Create user (admin only, rate limited)."""
    admin = request.state["user"]
    user = await db.create_user(name=name, email=email, created_by=admin.id)
    return {"user": user.to_dict()}, 201

@get("/api/data/export", hooks={
    "pre_handler": [require_admin, rate_limit],
    "on_response": [compress_large_response, log_export],
})
async def export_data(request: Request):
    """Export all data (admin only, rate limited, compressed)."""
    data = await db.export_all()
    return {"data": data, "count": len(data)}

if __name__ == "__main__":
    app.run()
```

### Type Safety

Full type checking with mypy/pyright:

```python
from typing import reveal_type

@app.on_request
async def my_hook(request: Request) -> Request | Response:
    reveal_type(request)  # Request
    reveal_type(request.method)  # str
    reveal_type(request.headers)  # dict[str, str]
    reveal_type(request.state)  # dict[str, Any]

    # Type error: must return Request or Response
    return "invalid"  # ❌ Type error

    # Correct
    return request  # ✅
    return Response({"error": "denied"}, status_code=403)  # ✅

@app.on_response
async def response_hook(response: Response) -> Response:
    reveal_type(response)  # Response
    reveal_type(response.status_code)  # int
    reveal_type(response.headers)  # dict[str, str]

    # Type error: must return Response only
    return request  # ❌ Type error

    # Correct
    return response  # ✅
```

## TypeScript API Design

### Type Definitions

```typescript
import type { Request, Response } from '@spikard/node';

// Hook types
type RequestHook = (request: Request) => Request | Response | Promise<Request | Response>;
type ResponseHook = (response: Response) => Response | Promise<Response>;

interface LifecycleHooks {
  onRequest?: RequestHook[];
  preValidation?: RequestHook[];
  preHandler?: RequestHook[];
  onResponse?: ResponseHook[];
  onError?: ResponseHook[];
}
```

### Global Hooks Registration

```typescript
import { Spikard, Request, Response } from '@spikard/node';

const app = new Spikard();

// Method 1: Using hook methods (recommended)
app.onRequest(async (request: Request): Promise<Request> => {
  console.log(`${request.method} ${request.path}`);
  return request;
});

app.preHandler(async (request: Request): Promise<Request | Response> => {
  const authHeader = request.headers['authorization'];

  if (!authHeader) {
    return new Response(
      { error: 'Missing authorization' },
      { status: 401 }
    );
  }

  const user = await authenticate(authHeader);
  if (!user) {
    return new Response(
      { error: 'Invalid token' },
      { status: 401 }
    );
  }

  request.state.user = user;
  return request;
});

app.onResponse(async (response: Response): Promise<Response> => {
  response.headers['X-Content-Type-Options'] = 'nosniff';
  response.headers['X-Frame-Options'] = 'DENY';
  return response;
});

app.onError(async (response: Response): Promise<Response> => {
  if (response.status >= 500) {
    await logToMonitoring(response);
  }
  return response;
});

// Method 2: Using addHook (functional style)
app.addHook('onRequest', async (request) => {
  request.state.requestId = generateUUID();
  return request;
});

// Sync hooks also supported
app.onRequest((request: Request): Request => {
  request.headers['X-Request-ID'] = generateUUID();
  return request;
});
```

### Per-Route Hooks

```typescript
import { get, post, Request, Response } from '@spikard/node';

// Hooks specific to this route
app.get('/api/admin/users', {
  hooks: {
    preHandler: [requireAdminRole, rateLimitAdmin],
  },
  handler: async (request: Request) => {
    return { users: await db.getAllUsers() };
  }
});

// Multiple hooks per hook point
app.post('/api/payments', {
  hooks: {
    preValidation: [validatePaymentMethod, checkFraud],
    preHandler: [verifyPaymentCredentials, calculateFees],
    onResponse: [logPaymentAttempt, sendReceipt],
  },
  handler: async (amount: number, method: string) => {
    const result = await paymentProcessor.charge(amount, method);
    return { transactionId: result.id };
  }
});

// Combine global + route hooks
app.onRequest(async (request) => {
  // Global auth logic
  return request;
});

app.get('/protected', {
  hooks: {
    preHandler: [additionalCheck]  // Runs AFTER global onRequest
  },
  handler: async () => {
    return { data: 'secret' };
  }
});
```

### Full Example

```typescript
import {
  Spikard,
  Request,
  Response,
  ServerConfig
} from '@spikard/node';

const app = new Spikard({
  config: { port: 3000 }
});

// ============================================================================
// Global Hooks
// ============================================================================

app.onRequest(async (request: Request): Promise<Request> => {
  console.log(`[${new Date().toISOString()}] ${request.method} ${request.path}`);
  request.state.startTime = Date.now();
  return request;
});

app.onRequest((request: Request): Request => {
  request.state.requestId = crypto.randomUUID();
  request.headers['X-Request-ID'] = request.state.requestId;
  return request;
});

app.preHandler(async (request: Request): Promise<Request | Response> => {
  // Skip auth for public routes
  if (request.path.startsWith('/public')) {
    return request;
  }

  const authHeader = request.headers['authorization'];
  if (!authHeader || !authHeader.startsWith('Bearer ')) {
    return new Response(
      { error: 'Missing or invalid authorization' },
      { status: 401 }
    );
  }

  const token = authHeader.replace('Bearer ', '');
  const user = await validateToken(token);

  if (!user) {
    return new Response(
      { error: 'Invalid token' },
      { status: 401 }
    );
  }

  request.state.user = user;
  return request;
});

app.onResponse(async (response: Response): Promise<Response> => {
  if (response.state.startTime) {
    const duration = Date.now() - response.state.startTime;
    response.headers['X-Response-Time'] = `${duration}ms`;
  }
  return response;
});

app.onError(async (response: Response): Promise<Response> => {
  if (response.status >= 500) {
    await sentry.captureError(response);
  }

  if (!response.body || typeof response.body !== 'object') {
    response.body = {
      error: 'Internal server error',
      requestId: response.state.requestId
    };
  }

  return response;
});

// ============================================================================
// Route-Specific Hooks
// ============================================================================

const requireAdmin = async (request: Request): Promise<Request | Response> => {
  const user = request.state.user;
  if (!user || user.role !== 'admin') {
    return new Response(
      { error: 'Admin access required' },
      { status: 403 }
    );
  }
  return request;
};

const rateLimit = async (request: Request): Promise<Request | Response> => {
  const userId = request.state.user?.id;
  if (await rateLimiter.isLimited(userId)) {
    return new Response(
      { error: 'Rate limit exceeded' },
      {
        status: 429,
        headers: { 'Retry-After': '60' }
      }
    );
  }
  return request;
};

// ============================================================================
// Routes
// ============================================================================

app.get('/public/health', async () => {
  return { status: 'ok' };
});

app.get('/api/profile', async (request: Request) => {
  const user = request.state.user;
  return { user: user.toJSON() };
});

app.post('/api/admin/users', {
  hooks: {
    preHandler: [requireAdmin, rateLimit],
  },
  handler: async (request: Request, name: string, email: string) => {
    const admin = request.state.user;
    const user = await db.createUser({ name, email, createdBy: admin.id });
    return { user: user.toJSON() };
  }
});

app.get('/api/data/export', {
  hooks: {
    preHandler: [requireAdmin, rateLimit],
    onResponse: [compressLargeResponse, logExport],
  },
  handler: async (request: Request) => {
    const data = await db.exportAll();
    return { data, count: data.length };
  }
});

app.listen();
```

### Type Safety

Full TypeScript type checking:

```typescript
// ✅ Correct: Request hook returns Request | Response
app.onRequest(async (request: Request): Promise<Request | Response> => {
  return request;  // ✅
  return new Response({ error: 'denied' }, { status: 403 });  // ✅
});

// ❌ Type error: must return Request or Response
app.onRequest(async (request: Request): Promise<Request> => {
  return "invalid";  // ❌ Type error
});

// ✅ Correct: Response hook returns Response only
app.onResponse(async (response: Response): Promise<Response> => {
  return response;  // ✅
});

// ❌ Type error: cannot return Request
app.onResponse(async (response: Response): Promise<Response> => {
  return request;  // ❌ Type error
});

// ✅ Request state is properly typed
app.preHandler(async (request: Request): Promise<Request> => {
  request.state.user = { id: 1, name: 'Alice' };  // ✅
  request.state.customData = [1, 2, 3];  // ✅
  return request;
});

app.get('/profile', async (request: Request) => {
  const user = request.state.user;  // Type: any (can be narrowed with type guards)
  return { user };
});
```

## Ruby API Design

### Type Definitions (using Sorbet/RBS)

```ruby
# sig/spikard.rbs
module Spikard
  class Request
    attr_accessor method: String
    attr_accessor path: String
    attr_accessor headers: Hash[String, String]
    attr_accessor state: Hash[Symbol, untyped]
  end

  class Response
    attr_accessor status: Integer
    attr_accessor headers: Hash[String, String]
    attr_accessor body: untyped
    attr_accessor state: Hash[Symbol, untyped]
  end

  type request_hook = ^(Request) -> (Request | Response)
  type response_hook = ^(Response) -> Response

  class App
    def on_request: (?request_hook) ?{ (Request) -> (Request | Response) } -> void
    def pre_validation: (?request_hook) ?{ (Request) -> (Request | Response) } -> void
    def pre_handler: (?request_hook) ?{ (Request) -> (Request | Response) } -> void
    def on_response: (?response_hook) ?{ (Response) -> Response } -> void
    def on_error: (?response_hook) ?{ (Response) -> Response } -> void
  end
end
```

### Global Hooks Registration

```ruby
require 'spikard'

app = Spikard::App.new

# Method 1: Using block syntax (idiomatic Ruby)
app.on_request do |request|
  puts "#{request.method} #{request.path}"
  request
end

app.pre_handler do |request|
  auth_header = request.headers['Authorization']

  unless auth_header
    return Spikard::Response.new(
      { error: 'Missing authorization' },
      status: 401
    )
  end

  user = authenticate(auth_header)
  unless user
    return Spikard::Response.new(
      { error: 'Invalid token' },
      status: 401
    )
  end

  request.state[:user] = user
  request
end

app.on_response do |response|
  response.headers['X-Content-Type-Options'] = 'nosniff'
  response.headers['X-Frame-Options'] = 'DENY'
  response
end

app.on_error do |response|
  log_to_monitoring(response) if response.status >= 500
  response
end

# Method 2: Using method references
def request_logger(request)
  puts "[#{Time.now}] #{request.method} #{request.path}"
  request
end

app.add_hook(:on_request, method(:request_logger))

# Method 3: Using lambda/proc
cors_handler = ->(request) {
  request.state[:cors_origin] = request.headers['Origin'] || '*'
  request
}

app.on_request(cors_handler)
```

### Per-Route Hooks

```ruby
# Hooks specific to this route
app.get '/api/admin/users',
  hooks: {
    pre_handler: [method(:require_admin_role), method(:rate_limit_admin)]
  } do |request|
  { users: DB.get_all_users }
end

# Multiple hooks per hook point
app.post '/api/payments',
  hooks: {
    pre_validation: [
      method(:validate_payment_method),
      method(:check_fraud)
    ],
    pre_handler: [
      method(:verify_payment_credentials),
      method(:calculate_fees)
    ],
    on_response: [
      method(:log_payment_attempt),
      method(:send_receipt)
    ]
  } do |amount:, method:|
  result = PaymentProcessor.charge(amount, method)
  { transaction_id: result.id }
end

# Combine global + route hooks
app.on_request do |request|
  # Global auth logic
  request
end

app.get '/protected',
  hooks: {
    pre_handler: [method(:additional_check)]  # Runs AFTER global on_request
  } do
  { data: 'secret' }
end
```

### Full Example

```ruby
require 'spikard'
require 'securerandom'

app = Spikard::App.new(port: 3000)

# ============================================================================
# Global Hooks
# ============================================================================

app.on_request do |request|
  puts "[#{Time.now}] #{request.method} #{request.path}"
  request.state[:start_time] = Time.now
  request
end

app.on_request do |request|
  request.state[:request_id] = SecureRandom.uuid
  request.headers['X-Request-ID'] = request.state[:request_id]
  request
end

app.pre_handler do |request|
  # Skip auth for public routes
  return request if request.path.start_with?('/public')

  auth_header = request.headers['Authorization']
  unless auth_header&.start_with?('Bearer ')
    return Spikard::Response.new(
      { error: 'Missing or invalid authorization' },
      status: 401
    )
  end

  token = auth_header.delete_prefix('Bearer ')
  user = validate_token(token)

  unless user
    return Spikard::Response.new(
      { error: 'Invalid token' },
      status: 401
    )
  end

  request.state[:user] = user
  request
end

app.on_response do |response|
  if response.state[:start_time]
    duration = ((Time.now - response.state[:start_time]) * 1000).round(2)
    response.headers['X-Response-Time'] = "#{duration}ms"
  end
  response
end

app.on_error do |response|
  Sentry.capture_error(response) if response.status >= 500

  unless response.body.is_a?(Hash)
    response.body = {
      error: 'Internal server error',
      request_id: response.state[:request_id]
    }
  end

  response
end

# ============================================================================
# Route-Specific Hooks
# ============================================================================

def require_admin(request)
  user = request.state[:user]
  unless user&.admin?
    return Spikard::Response.new(
      { error: 'Admin access required' },
      status: 403
    )
  end
  request
end

def rate_limit(request)
  user_id = request.state[:user]&.id
  if RateLimiter.limited?(user_id)
    return Spikard::Response.new(
      { error: 'Rate limit exceeded' },
      status: 429,
      headers: { 'Retry-After' => '60' }
    )
  end
  request
end

# ============================================================================
# Routes
# ============================================================================

app.get '/public/health' do
  { status: 'ok' }
end

app.get '/api/profile' do |request|
  user = request.state[:user]
  { user: user.to_h }
end

app.post '/api/admin/users',
  hooks: {
    pre_handler: [method(:require_admin), method(:rate_limit)]
  } do |request, name:, email:|
  admin = request.state[:user]
  user = DB.create_user(name: name, email: email, created_by: admin.id)
  [{ user: user.to_h }, 201]
end

app.get '/api/data/export',
  hooks: {
    pre_handler: [method(:require_admin), method(:rate_limit)],
    on_response: [method(:compress_large_response), method(:log_export)]
  } do |request|
  data = DB.export_all
  { data: data, count: data.length }
end

app.run
```

## PHP API Design (Future)

### Type Definitions

```php
<?php

namespace Spikard;

use Closure;

/**
 * @template TRequest of Request
 * @template TResponse of Response
 */
interface RequestHook
{
    /**
     * @param Request $request
     * @return Request|Response
     */
    public function __invoke(Request $request): Request|Response;
}

/**
 * @template TResponse of Response
 */
interface ResponseHook
{
    /**
     * @param Response $response
     * @return Response
     */
    public function __invoke(Response $response): Response;
}

class LifecycleHooks
{
    /** @var array<RequestHook> */
    public array $onRequest = [];

    /** @var array<RequestHook> */
    public array $preValidation = [];

    /** @var array<RequestHook> */
    public array $preHandler = [];

    /** @var array<ResponseHook> */
    public array $onResponse = [];

    /** @var array<ResponseHook> */
    public array $onError = [];
}
```

### Global Hooks Registration

```php
<?php

use Spikard\App;
use Spikard\Request;
use Spikard\Response;

$app = new App();

// Method 1: Using closures
$app->onRequest(function (Request $request): Request {
    echo "{$request->method} {$request->path}\n";
    return $request;
});

$app->preHandler(function (Request $request): Request|Response {
    $authHeader = $request->headers['Authorization'] ?? null;

    if (!$authHeader) {
        return new Response(
            ['error' => 'Missing authorization'],
            status: 401
        );
    }

    $user = authenticate($authHeader);
    if (!$user) {
        return new Response(
            ['error' => 'Invalid token'],
            status: 401
        );
    }

    $request->state['user'] = $user;
    return $request;
});

$app->onResponse(function (Response $response): Response {
    $response->headers['X-Content-Type-Options'] = 'nosniff';
    $response->headers['X-Frame-Options'] = 'DENY';
    return $response;
});

// Method 2: Using callable arrays
class AuthMiddleware
{
    public function __invoke(Request $request): Request|Response
    {
        // Auth logic
        return $request;
    }
}

$app->addHook('pre_handler', new AuthMiddleware());
```

### Per-Route Hooks

```php
<?php

use Spikard\Attributes\Get;
use Spikard\Attributes\Post;
use Spikard\Attributes\Hooks;

#[Get('/api/admin/users')]
#[Hooks(
    preHandler: [RequireAdminRole::class, RateLimitAdmin::class]
)]
function listAdminUsers(Request $request): array
{
    return ['users' => DB::getAllUsers()];
}

#[Post('/api/payments')]
#[Hooks(
    preValidation: [ValidatePaymentMethod::class, CheckFraud::class],
    preHandler: [VerifyPaymentCredentials::class, CalculateFees::class],
    onResponse: [LogPaymentAttempt::class, SendReceipt::class]
)]
function processPayment(float $amount, string $method): array
{
    $result = PaymentProcessor::charge($amount, $method);
    return ['transaction_id' => $result->id];
}
```

## Implementation Plan

### Phase 0: Rust Core (COMPLETED ✅)
- [x] Implement `LifecycleHook` trait in `crates/spikard-http/src/lifecycle.rs`
- [x] Implement `HookResult` enum with Continue/ShortCircuit
- [x] Implement `LifecycleHooks` container with zero-cost design
- [x] Add execution methods (execute_on_request, execute_pre_validation, etc.)
- [x] Add unit tests for core hook execution
- [x] Integrate with `ServerConfig`

### Phase 1: Rust Ergonomic API (Current Priority)
- [ ] Add `request_hook()` builder function in `crates/spikard-http/src/lifecycle.rs`
- [ ] Add `response_hook()` builder function
- [ ] Implement builder pattern for `LifecycleHooks`
- [ ] Add examples in `examples/rust-lifecycle-hooks/`
- [ ] Write comprehensive integration tests with Axum
- [ ] Document Axum extensions for state passing
- [ ] Add benchmarks for hook performance

### Phase 2: Python Binding
- [x] Implement `PythonHook` in `crates/spikard-py/src/lifecycle.rs`
- [x] Implement `PyRequest` wrapper for lifecycle hooks
- [x] Add PyO3 async support with `asyncio.run()` pattern
- [ ] Create Python API in `packages/python/spikard/lifecycle.py`
- [ ] Add `@app.on_request` decorator pattern to `Spikard` class
- [ ] Support per-route hooks in routing decorators
- [ ] Add comprehensive type hints
- [ ] Write integration tests

### Phase 3: TypeScript Binding
- [ ] Implement `TypeScriptHook` in `crates/spikard-node/src/lifecycle.rs`
- [ ] Use napi-rs ThreadsafeFunction for JS function calls
- [ ] Create TypeScript API with method chaining
- [ ] Add `.d.ts` type definitions with full generics
- [ ] Support per-route hooks
- [ ] Write integration tests

### Phase 4: Ruby Binding
- [ ] Implement `RubyHook` in `crates/spikard-rb/src/lifecycle.rs`
- [ ] Use magnus for Ruby proc/lambda calls
- [ ] Create idiomatic Ruby block-based API
- [ ] Add Sorbet/RBS type definitions
- [ ] Support per-route hooks
- [ ] Write integration tests

### Phase 5: PHP Binding (Future)
- [ ] Design ext-php-rs integration
- [ ] Implement attribute-based hooks
- [ ] Add PHPStan type definitions
- [ ] Support per-route hooks
- [ ] Write integration tests

## Testing Strategy

Each language binding should have:

1. **Unit Tests**: Test individual hook execution
2. **Integration Tests**: Test hooks in real HTTP requests
3. **Type Tests**: Verify compile-time type safety
4. **Performance Tests**: Ensure zero-cost when not used

Example test scenarios:
- Multiple hooks execute in order
- Short-circuit stops execution
- Request state passes through hooks
- Error hooks catch exceptions
- Async and sync hooks both work
- Per-route hooks override global hooks

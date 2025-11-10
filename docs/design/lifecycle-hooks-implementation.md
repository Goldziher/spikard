# Lifecycle Hooks Implementation Guide

**Date:** November 2025
**Status:** 🟡 Design Complete, Implementation Pending
**Related Docs:** [middleware-lifecycle-optimization.md](./middleware-lifecycle-optimization.md)

## Executive Summary

This document provides a concrete implementation plan for Fastify-inspired lifecycle hooks in Spikard, enabling Python, TypeScript, and Ruby users to execute custom logic at specific points in the request/response lifecycle without sacrificing performance.

**Key Design Principles:**
- ✅ Zero-cost when hooks are not registered (null pointer check)
- ✅ Minimal FFI overhead when hooks are registered (conditional execution)
- ✅ Type-safe APIs in all language bindings
- ✅ Async-first with proper event loop integration

## Overview

### Goals

1. Enable user-defined lifecycle hooks at key request processing points
2. Maintain near-zero overhead when hooks are not used
3. Provide idiomatic APIs for Python, TypeScript, and Ruby
4. Support both sync and async hooks in all languages
5. Allow hooks to modify requests/responses or short-circuit execution

### Non-Goals

- Custom middleware system (use tower-http for standard middleware)
- Complex hook ordering/priority system (hooks run in registration order)
- Hook dependency resolution (use DI for dependencies)

## Lifecycle Hook Points

Based on Fastify and Axum patterns, Spikard supports these lifecycle hook points:

```
Incoming Request
  ↓
┌─────────────────────────┐
│  onRequest              │  ← Hook: Inspect/modify request before routing
└────────┬────────────────┘
         ↓
┌─────────────────────────┐
│  Routing (Rust)         │  Always executes
└────────┬────────────────┘
         ↓
┌─────────────────────────┐
│  preValidation          │  ← Hook: Transform data before validation
└────────┬────────────────┘
         ↓
┌─────────────────────────┐
│  Validation (Rust)      │  Always executes
└────────┬────────────────┘
         ↓
┌─────────────────────────┐
│  preHandler             │  ← Hook: Add request context, auth checks
└────────┬────────────────┘
         ↓
┌─────────────────────────┐
│  Handler                │  Always executes
└────────┬────────────────┘
         ↓
┌─────────────────────────┐
│  onResponse             │  ← Hook: Transform response, add headers
└────────┬────────────────┘
         ↓
┌─────────────────────────┐
│  onError                │  ← Hook: Custom error handling
└─────────────────────────┘
```

### Hook Capabilities

| Hook | Can Modify Request | Can Modify Response | Can Short-Circuit | Typical Use Cases |
|------|-------------------|---------------------|-------------------|-------------------|
| `onRequest` | ✅ Yes | ❌ No | ✅ Yes (return response) | Logging, early auth checks, request ID injection |
| `preValidation` | ✅ Yes | ❌ No | ✅ Yes (return response) | Data transformation before validation |
| `preHandler` | ✅ Yes (add context) | ❌ No | ✅ Yes (return response) | Auth, request context, dependency injection |
| `onResponse` | ❌ No | ✅ Yes | ❌ No | Response headers, logging, metrics |
| `onError` | ❌ No | ✅ Yes | ✅ Yes (return response) | Custom error formatting, error logging |

## Implementation Strategy

### Phase 1: Rust Core (1-2 days)

**File:** `crates/spikard-http/src/lifecycle.rs` (new)

```rust
use std::sync::Arc;
use axum::http::{Request, Response};
use futures::future::BoxFuture;

/// Lifecycle hook result - either continue or short-circuit with response
pub enum HookResult<T> {
    /// Continue to next phase
    Continue(T),
    /// Short-circuit and return this response immediately
    ShortCircuit(Response<axum::body::Body>),
}

/// Trait for lifecycle hooks - implemented by language bindings
pub trait LifecycleHook: Send + Sync {
    /// Hook name for debugging
    fn name(&self) -> &str;

    /// Execute hook with request
    fn execute_request<'a>(
        &'a self,
        req: Request<axum::body::Body>,
    ) -> BoxFuture<'a, Result<HookResult<Request<axum::body::Body>>, String>>;

    /// Execute hook with response
    fn execute_response<'a>(
        &'a self,
        resp: Response<axum::body::Body>,
    ) -> BoxFuture<'a, Result<Response<axum::body::Body>, String>>;
}

/// Container for all lifecycle hooks
#[derive(Default, Clone)]
pub struct LifecycleHooks {
    /// onRequest hooks - run before routing
    on_request: Vec<Arc<dyn LifecycleHook>>,

    /// preValidation hooks - run after routing, before validation
    pre_validation: Vec<Arc<dyn LifecycleHook>>,

    /// preHandler hooks - run after validation, before handler
    pre_handler: Vec<Arc<dyn LifecycleHook>>,

    /// onResponse hooks - run after handler
    on_response: Vec<Arc<dyn LifecycleHook>>,

    /// onError hooks - run on errors
    on_error: Vec<Arc<dyn LifecycleHook>>,
}

impl LifecycleHooks {
    /// Create new empty hooks container
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if any hooks are registered
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.on_request.is_empty()
            && self.pre_validation.is_empty()
            && self.pre_handler.is_empty()
            && self.on_response.is_empty()
            && self.on_error.is_empty()
    }

    /// Add onRequest hook
    pub fn add_on_request(&mut self, hook: Arc<dyn LifecycleHook>) {
        self.on_request.push(hook);
    }

    /// Execute onRequest hooks - return None to continue, Some(response) to short-circuit
    pub async fn execute_on_request(
        &self,
        mut req: Request<axum::body::Body>,
    ) -> Result<HookResult<Request<axum::body::Body>>, String> {
        // Fast path: no hooks registered
        if self.on_request.is_empty() {
            return Ok(HookResult::Continue(req));
        }

        // Execute each hook in order
        for hook in &self.on_request {
            match hook.execute_request(req).await? {
                HookResult::Continue(r) => req = r,
                HookResult::ShortCircuit(response) => {
                    return Ok(HookResult::ShortCircuit(response));
                }
            }
        }

        Ok(HookResult::Continue(req))
    }

    // Similar methods for other hook points...
}
```

**File:** `crates/spikard-http/src/server.rs` (update)

Integrate hooks into the request pipeline:

```rust
// In Server::with_handlers_and_config()
async fn handle_request_with_hooks(
    hooks: Arc<LifecycleHooks>,
    route: Route,
    handler: Arc<dyn Handler>,
    req: Request<axum::body::Body>,
) -> Response<axum::body::Body> {
    // 1. onRequest hooks
    let req = match hooks.execute_on_request(req).await {
        Ok(HookResult::Continue(r)) => r,
        Ok(HookResult::ShortCircuit(response)) => return response,
        Err(e) => return error_response(e),
    };

    // 2. Routing happens in Axum layer (already done)

    // 3. preValidation hooks
    let req = match hooks.execute_pre_validation(req).await {
        Ok(HookResult::Continue(r)) => r,
        Ok(HookResult::ShortCircuit(response)) => return response,
        Err(e) => return error_response(e),
    };

    // 4. Validation (existing code)
    let validated_req = match route.validate_request(req).await {
        Ok(r) => r,
        Err(e) => {
            // Execute onError hooks
            let error_response = create_validation_error_response(e);
            return hooks.execute_on_error(error_response).await.unwrap_or(error_response);
        }
    };

    // 5. preHandler hooks
    let validated_req = match hooks.execute_pre_handler(validated_req).await {
        Ok(HookResult::Continue(r)) => r,
        Ok(HookResult::ShortCircuit(response)) => return response,
        Err(e) => return error_response(e),
    };

    // 6. Execute handler (existing code)
    let response = match handler.handle(validated_req).await {
        Ok(r) => r,
        Err(e) => {
            let error_response = create_handler_error_response(e);
            return hooks.execute_on_error(error_response).await.unwrap_or(error_response);
        }
    };

    // 7. onResponse hooks
    hooks.execute_on_response(response).await.unwrap_or_else(|e| error_response(e))
}
```

### Phase 2: Python Bindings (1-2 days)

**File:** `crates/spikard-py/src/lifecycle.rs` (new)

```rust
use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::into_future;
use spikard_http::lifecycle::{LifecycleHook, HookResult};

/// Python lifecycle hook wrapper
pub struct PythonHook {
    name: String,
    /// Python async function: async def hook(request) -> Request | Response
    func: Py<PyAny>,
}

impl PythonHook {
    pub fn new(name: String, func: Py<PyAny>) -> Self {
        Self { name, func }
    }
}

#[async_trait::async_trait]
impl LifecycleHook for PythonHook {
    fn name(&self) -> &str {
        &self.name
    }

    fn execute_request<'a>(
        &'a self,
        req: Request<axum::body::Body>,
    ) -> BoxFuture<'a, Result<HookResult<Request<axum::body::Body>>, String>> {
        Box::pin(async move {
            // Convert Rust request to Python Request object
            let py_request = Python::with_gil(|py| {
                convert_request_to_python(py, req)
            })?;

            // Call Python async function
            let result = Python::with_gil(|py| -> PyResult<PyObject> {
                let coroutine = self.func.call1(py, (py_request,))?;
                Ok(coroutine.into())
            }).map_err(|e| format!("Hook '{}' failed: {}", self.name, e))?;

            // Await the coroutine
            let py_result = into_future(result).await
                .map_err(|e| format!("Hook '{}' async error: {}", self.name, e))?;

            // Check if result is Request or Response
            Python::with_gil(|py| {
                let result_obj = py_result.as_ref(py);

                // If it's a Response, short-circuit
                if result_obj.is_instance_of::<crate::response::Response>() {
                    let response = convert_python_to_response(py, result_obj)?;
                    return Ok(HookResult::ShortCircuit(response));
                }

                // Otherwise it should be a Request
                let request = convert_python_to_request(py, result_obj)?;
                Ok(HookResult::Continue(request))
            })
        })
    }

    // Similar for execute_response...
}
```

**File:** `packages/python/spikard/lifecycle.py` (new)

```python
from typing import Callable, Awaitable, Union
from spikard import Request, Response

# Type aliases for lifecycle hooks
OnRequestHook = Callable[[Request], Awaitable[Union[Request, Response]]]
PreValidationHook = Callable[[Request], Awaitable[Union[Request, Response]]]
PreHandlerHook = Callable[[Request], Awaitable[Union[Request, Response]]]
OnResponseHook = Callable[[Response], Awaitable[Response]]
OnErrorHook = Callable[[Response], Awaitable[Response]]

class LifecycleHooks:
    """Configuration for lifecycle hooks"""

    def __init__(
        self,
        on_request: list[OnRequestHook] | None = None,
        pre_validation: list[PreValidationHook] | None = None,
        pre_handler: list[PreHandlerHook] | None = None,
        on_response: list[OnResponseHook] | None = None,
        on_error: list[OnErrorHook] | None = None,
    ):
        self.on_request = on_request or []
        self.pre_validation = pre_validation or []
        self.pre_handler = pre_handler or []
        self.on_response = on_response or []
        self.on_error = on_error or []
```

**Usage Example:**

```python
from spikard import Spikard, Request, Response
from spikard.lifecycle import LifecycleHooks

async def add_request_id(request: Request) -> Request:
    """Add a unique request ID to all requests"""
    if "x-request-id" not in request.headers:
        request.headers["x-request-id"] = str(uuid.uuid4())
    return request

async def log_response(response: Response) -> Response:
    """Log all responses"""
    logger.info(f"Response: {response.status_code}")
    return response

async def auth_check(request: Request) -> Request | Response:
    """Check authentication - short-circuit if unauthorized"""
    token = request.headers.get("authorization")
    if not token:
        return Response(
            status_code=401,
            body={"error": "Unauthorized"},
        )

    # Validate token and add user to request state
    user = await validate_token(token)
    request.state["user"] = user
    return request

app = Spikard(
    lifecycle_hooks=LifecycleHooks(
        on_request=[add_request_id],
        pre_handler=[auth_check],
        on_response=[log_response],
    )
)
```

### Phase 3: TypeScript Bindings (1-2 days)

**File:** `crates/spikard-node/src/lifecycle.rs` (new)

```rust
use napi::bindgen_prelude::*;
use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use spikard_http::lifecycle::{LifecycleHook, HookResult};

/// Node.js lifecycle hook wrapper
pub struct NodeHook {
    name: String,
    /// ThreadsafeFunction for calling JS from Rust async
    tsfn: ThreadsafeFunction<Request, Promise<Response>>,
}

#[async_trait::async_trait]
impl LifecycleHook for NodeHook {
    fn name(&self) -> &str {
        &self.name
    }

    fn execute_request<'a>(
        &'a self,
        req: Request<axum::body::Body>,
    ) -> BoxFuture<'a, Result<HookResult<Request<axum::body::Body>>, String>> {
        Box::pin(async move {
            // Convert request to JS-compatible format
            let js_request = convert_request_to_js(req)?;

            // Call JS function via ThreadsafeFunction
            let result = self.tsfn.call_async(js_request).await
                .map_err(|e| format!("Hook '{}' failed: {}", self.name, e))?;

            // Check if result is Request or Response
            if result.is_response() {
                let response = convert_js_to_response(result)?;
                return Ok(HookResult::ShortCircuit(response));
            }

            let request = convert_js_to_request(result)?;
            Ok(HookResult::Continue(request))
        })
    }
}
```

**File:** `packages/node/src/lifecycle.ts` (new)

```typescript
import { Request, Response } from './types';

export type OnRequestHook = (request: Request) => Promise<Request | Response>;
export type PreValidationHook = (request: Request) => Promise<Request | Response>;
export type PreHandlerHook = (request: Request) => Promise<Request | Response>;
export type OnResponseHook = (response: Response) => Promise<Response>;
export type OnErrorHook = (response: Response) => Promise<Response>;

export interface LifecycleHooks {
  onRequest?: OnRequestHook[];
  preValidation?: PreValidationHook[];
  preHandler?: PreHandlerHook[];
  onResponse?: OnResponseHook[];
  onError?: OnErrorHook[];
}
```

**Usage Example:**

```typescript
import { Spikard, Request, Response } from '@spikard/node';

const app = new Spikard({
  lifecycleHooks: {
    onRequest: [
      async (request: Request): Promise<Request> => {
        console.log(`${request.method} ${request.url}`);
        return request;
      }
    ],
    preHandler: [
      async (request: Request): Promise<Request | Response> => {
        const token = request.headers.authorization;
        if (!token) {
          return new Response(401, { error: 'Unauthorized' });
        }

        const user = await validateToken(token);
        request.state.user = user;
        return request;
      }
    ],
    onResponse: [
      async (response: Response): Promise<Response> => {
        response.headers['x-powered-by'] = 'Spikard';
        return response;
      }
    ]
  }
});
```

### Phase 4: Ruby Bindings (1-2 days)

Similar to Python but using Magnus and Ruby fibers for async.

**File:** `crates/spikard-rb/src/lifecycle.rs` (new)

```rust
// Similar to Python implementation but using Magnus
```

## Testing Strategy

### Unit Tests (Rust)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_empty_hooks_fast_path() {
        let hooks = LifecycleHooks::new();
        assert!(hooks.is_empty());

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();
        assert!(matches!(result, HookResult::Continue(_)));
    }

    #[tokio::test]
    async fn test_hook_short_circuit() {
        let mut hooks = LifecycleHooks::new();

        // Add hook that returns early response
        hooks.add_on_request(Arc::new(TestHook::new(|_req| {
            let response = Response::builder()
                .status(401)
                .body(Body::from("Unauthorized"))
                .unwrap();
            Ok(HookResult::ShortCircuit(response))
        })));

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();

        assert!(matches!(result, HookResult::ShortCircuit(_)));
    }
}
```

### Integration Tests (Python)

**File:** `packages/python/tests/test_lifecycle_hooks.py`

```python
import pytest
from spikard import Spikard, Request, Response, get
from spikard.lifecycle import LifecycleHooks

@pytest.mark.asyncio
async def test_on_request_hook():
    """Test onRequest hook can modify request"""
    request_modified = False

    async def mark_request(request: Request) -> Request:
        nonlocal request_modified
        request_modified = True
        request.state["marked"] = True
        return request

    app = Spikard(lifecycle_hooks=LifecycleHooks(on_request=[mark_request]))

    @get("/")
    async def root(request: Request):
        return {"marked": request.state.get("marked", False)}

    client = app.test_client()
    response = await client.get("/")

    assert request_modified
    assert response.json() == {"marked": True}

@pytest.mark.asyncio
async def test_pre_handler_short_circuit():
    """Test preHandler can return early response"""
    async def auth_check(request: Request) -> Request | Response:
        if "authorization" not in request.headers:
            return Response(401, body={"error": "Unauthorized"})
        return request

    app = Spikard(lifecycle_hooks=LifecycleHooks(pre_handler=[auth_check]))

    @get("/protected")
    async def protected():
        return {"data": "secret"}

    client = app.test_client()

    # Without auth header - should return 401
    response = await client.get("/protected")
    assert response.status_code == 401
    assert response.json() == {"error": "Unauthorized"}

    # With auth header - should reach handler
    response = await client.get("/protected", headers={"authorization": "Bearer token"})
    assert response.status_code == 200
    assert response.json() == {"data": "secret"}

@pytest.mark.asyncio
async def test_on_response_hook():
    """Test onResponse hook can modify response"""
    async def add_header(response: Response) -> Response:
        response.headers["x-custom"] = "value"
        return response

    app = Spikard(lifecycle_hooks=LifecycleHooks(on_response=[add_header]))

    @get("/")
    async def root():
        return {"hello": "world"}

    client = app.test_client()
    response = await client.get("/")

    assert response.headers["x-custom"] == "value"
    assert response.json() == {"hello": "world"}
```

## Performance Benchmarks

Target performance characteristics:

| Scenario | Overhead | Benchmark |
|----------|----------|-----------|
| No hooks registered | <1ns | Null pointer check |
| 1 Python async hook | ~50ns | PyO3 FFI + async overhead |
| 3 Python async hooks | ~150ns | 3 × FFI calls |
| 1 TypeScript async hook | ~30ns | napi-rs FFI + async overhead |

Benchmark file: `crates/spikard-http/benches/lifecycle_hooks.rs`

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_empty_hooks(c: &mut Criterion) {
    c.bench_function("empty lifecycle hooks", |b| {
        let hooks = LifecycleHooks::new();
        let rt = tokio::runtime::Runtime::new().unwrap();

        b.iter(|| {
            rt.block_on(async {
                let req = Request::builder().body(Body::empty()).unwrap();
                black_box(hooks.execute_on_request(req).await.unwrap());
            })
        });
    });
}

criterion_group!(benches, benchmark_empty_hooks);
criterion_main!(benches);
```

## Implementation Checklist

### Rust Core
- [ ] Create `crates/spikard-http/src/lifecycle.rs`
- [ ] Define `LifecycleHook` trait
- [ ] Implement `LifecycleHooks` container
- [ ] Add hook execution to request pipeline in `server.rs`
- [ ] Write unit tests for hook execution
- [ ] Add benchmarks for hook overhead
- [ ] Update `ServerConfig` to accept hooks

### Python Bindings
- [ ] Create `crates/spikard-py/src/lifecycle.rs`
- [ ] Implement `PythonHook` wrapper with async support
- [ ] Create `packages/python/spikard/lifecycle.py` API
- [ ] Update `Spikard` constructor to accept `lifecycle_hooks`
- [ ] Add type hints and docstrings
- [ ] Write integration tests in `packages/python/tests/test_lifecycle_hooks.py`
- [ ] Add example in `examples/python/lifecycle_hooks.py`

### TypeScript Bindings
- [ ] Create `crates/spikard-node/src/lifecycle.rs`
- [ ] Implement `NodeHook` with ThreadsafeFunction
- [ ] Create `packages/node/src/lifecycle.ts` API
- [ ] Update Spikard constructor to accept lifecycle hooks
- [ ] Add TypeScript type definitions
- [ ] Write integration tests
- [ ] Add example in `examples/typescript/lifecycle-hooks.ts`

### Ruby Bindings
- [ ] Create `crates/spikard-rb/src/lifecycle.rs`
- [ ] Implement `RubyHook` with fiber support
- [ ] Update Ruby API
- [ ] Write tests
- [ ] Add example

### Documentation
- [ ] Update middleware-lifecycle-optimization.md with implementation status
- [ ] Add lifecycle hooks section to main README
- [ ] Create user guide with examples
- [ ] Document performance characteristics
- [ ] Add migration guide for users of other frameworks

## References

- **Fastify Lifecycle:** https://fastify.dev/docs/latest/Reference/Lifecycle/
- **Axum Middleware:** https://docs.rs/axum/latest/axum/middleware/
- **tower-http layers:** https://docs.rs/tower-http/latest/tower_http/
- **PyO3 async:** https://pyo3.rs/v0.21.0/ecosystem/async-await
- **napi-rs ThreadsafeFunction:** https://napi.rs/docs/concepts/threadsafe-function

---

**Key Takeaway:** Lifecycle hooks provide controlled extensibility at key request processing points while maintaining Spikard's performance-first design through conditional execution and minimal FFI overhead.

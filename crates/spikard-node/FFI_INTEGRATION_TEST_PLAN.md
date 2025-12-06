# Node.js FFI Integration Test Plan

## Overview

The Node.js FFI layer (`crates/spikard-node`) bridges Rust's async HTTP server with JavaScript handlers via napi-rs `ThreadsafeFunction` (TSFN). This design eliminates JSON serialization overhead (6x speedup) by passing structured objects directly. **Critical constraint: TSFN cannot be tested in unit tests**—it requires a running Node.js V8 runtime with an event loop. Proper FFI coverage requires integration tests that start an actual server and make HTTP requests.

## Why Unit Tests Cannot Test FFI Boundaries

### napi::Env Requirement
- `ThreadsafeFunction` is bound to a specific napi environment (`napi::Env`)
- `napi::Env` only exists during native module execution (inside Node.js)
- Unit tests in `crates/spikard-node/tests/` run as standalone Rust binaries, not within Node.js
- Attempting to create or invoke TSFN outside V8 causes panic or hangs

### JavaScript Event Loop Dependency
- TSFN's `call_async()` schedules work on the Node.js event loop
- Unit tests have no event loop; async Rust tasks run on tokio, not Node's libuv
- Promises returned from TSFN resolve on the JavaScript side; without a running loop, they never complete

### Promise Handling
- TSFN returns `Promise<T>` from JavaScript
- Promises must be awaited from Rust via the TSFN callback mechanism
- This two-way bridge (Rust → JS → Promise → Rust) requires live JavaScript execution

**Result**: Rust-level unit tests can only verify that TSFN types compile and basic conversions work. Actual FFI behavior (calling JS, promise resolution, error propagation) must be tested via HTTP requests to a live server.

## Proper Integration Test Approach

### Location
```
packages/node/tests/integration/
├── handler-invocation.spec.ts         # RequestData → HandlerInput → JS → HandlerOutput
├── di-factory-resolution.spec.ts      # DI container factory calls via TSFN
├── lifecycle-hooks.spec.ts            # onRequest, preValidation, preHandler, onResponse, onError
├── sse-websocket-streaming.spec.ts    # Callback chains, event loops
├── error-handling.spec.ts             # Promise rejection, panic conversion, error structure
└── background-tasks.spec.ts           # Task spawning from handlers
```

### Technology Stack
- **Framework**: Vitest (already configured in `packages/node/vitest.config.ts`)
- **Language**: TypeScript with strict types
- **HTTP Client**: node-fetch or axios for making requests
- **Server**: Start actual Spikard server via npm or Node.js process
- **Lifecycle**: beforeAll/afterAll for server startup/shutdown

### Test Structure Template

```typescript
import { describe, it, expect, beforeAll, afterAll } from "vitest";
import type { Spikard } from "../index";

describe("FFI: Handler Invocation", () => {
  let app: Spikard;
  let server: Awaited<ReturnType<typeof app.listen>>;

  beforeAll(async () => {
    app = new Spikard();

    // Register handler with metadata + JavaScript function
    app.addRoute(
      {
        method: "POST",
        path: "/echo",
        handler_name: "echoHandler",
        is_async: true,
      },
      async (input) => ({
        status: 200,
        body: { received: input.body }, // Verify input structure
      })
    );

    // Start HTTP server (port picked by OS)
    server = await app.listen({ port: 0 });
    const { port } = server.address() as { port: number };
    this.baseUrl = `http://localhost:${port}`;
  });

  afterAll(async () => {
    if (server) {
      await server.close();
    }
  });

  it("should marshal RequestData to JavaScript handler", async () => {
    // Make HTTP POST with JSON body
    const response = await fetch(`${this.baseUrl}/echo`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ test: "data" }),
    });

    const body = await response.json();

    // Verify handler received correct input structure (HandlerInput)
    expect(body.received).toEqual({ test: "data" });
    expect(response.status).toBe(200);
  });

  it("should convert HandlerOutput to HTTP response", async () => {
    app.addRoute(
      {
        method: "GET",
        path: "/custom-response",
        handler_name: "customHandler",
        is_async: true,
      },
      async () => ({
        status: 201,
        headers: { "X-Custom": "header-value" },
        body: { created: true },
      })
    );

    // Restart server with new route
    if (server) await server.close();
    server = await app.listen({ port: 0 });
    const { port } = (server.address() as { port: number });

    const response = await fetch(`http://localhost:${port}/custom-response`, {
      method: "GET",
    });

    expect(response.status).toBe(201);
    expect(response.headers.get("X-Custom")).toBe("header-value");
    expect(await response.json()).toEqual({ created: true });
  });
});
```

## Critical FFI Paths Requiring Integration Testing

### 1. Handler Invocation Chain
**Path**: HTTP Request → `Request<Body>` → `RequestData` → `HandlerInput` → JavaScript handler → `HandlerOutput` → Response

**Tests**:
- Request method, path, headers, body are correctly converted to `HandlerInput`
- Path parameters, query params extracted and passed to handler
- Handler output structure (status, headers, body) correctly converts to HTTP response
- Status codes (200, 201, 404, 500) passed through correctly
- Headers preserved (custom headers, content-type)
- JSON body serialization round-trips without corruption
- Binary/buffer bodies handled (if applicable)

**Integration Test Example**:
```typescript
it("should extract path parameters in HandlerInput", async () => {
  app.addRoute(
    {
      method: "GET",
      path: "/users/:id",
      handler_name: "getUserHandler",
      is_async: true,
    },
    async (input) => ({
      status: 200,
      body: { userId: input.path_params.id },
    })
  );

  const response = await fetch(`${baseUrl}/users/42`);
  const body = await response.json();
  expect(body.userId).toBe("42");
});
```

### 2. Promise Rejection Error Handling
**Path**: JavaScript Promise rejection → `.await` in Rust → `Err` → HTTP 500 error response

**Tests**:
- Handler that rejects with Error object produces 500 response with error details
- Handler that rejects with string is wrapped in error structure
- Handler that throws exception is caught and converted to error response
- Error structure matches JSON schema: `{ error: string, code: string, details: object }`
- Stack traces preserved in error responses
- Multiple concurrent rejections don't corrupt app state

**Integration Test Example**:
```typescript
it("should handle handler rejection with Error object", async () => {
  app.addRoute(
    {
      method: "GET",
      path: "/error",
      handler_name: "errorHandler",
      is_async: true,
    },
    async () => {
      throw new Error("Handler failed");
    }
  );

  const response = await fetch(`${baseUrl}/error`);
  const body = await response.json();

  expect(response.status).toBe(500);
  expect(body.error).toBeDefined();
  expect(body.code).toBeDefined();
  expect(body.details).toBeDefined();
});
```

### 3. DI Factory Resolution via TSFN
**Path**: Request → DI container → factory function (TSFN) → Promise → dependency resolved → handler receives dependency

**Tests**:
- Factory function called once per request (non-singleton)
- Factory function called once, result cached (singleton)
- Factory promise rejection produces error response
- Factory timeout detected and handled
- Singleton cache not poisoned after factory failure
- Multiple concurrent factory resolutions isolated
- Async generator factories properly cleaned up on error

**Integration Test Example**:
```typescript
it("should resolve DI factory via ThreadsafeFunction", async () => {
  const callCount = { value: 0 };

  const container = {
    dbConnection: {
      isFactory: true,
      factory: async () => {
        callCount.value++;
        return { connected: true };
      },
      dependsOn: [],
      singleton: false,
    },
  };

  app.addRoute(
    {
      method: "GET",
      path: "/db-test",
      handler_name: "dbHandler",
      is_async: true,
    },
    async (input, deps) => ({
      status: 200,
      body: { callCount: callCount.value, hasDb: !!deps.dbConnection },
    })
  );

  app.di(container);

  // Two requests should call factory twice (non-singleton)
  await fetch(`${baseUrl}/db-test`);
  await fetch(`${baseUrl}/db-test`);

  expect(callCount.value).toBe(2);
});
```

### 4. Lifecycle Hooks Execution
**Path**: HTTP Request → onRequest hook (TSFN) → Promise → preValidation hook (TSFN) → ... → handler → onResponse hook (TSFN) → Response

**Tests**:
- All five hooks (onRequest, preValidation, preHandler, onResponse, onError) execute in order
- Hook can short-circuit request processing (onRequest rejection → 400 response)
- Hook can modify request/response objects
- Hook timeout detected and handled
- Hook rejection error structure preserved
- Multiple hooks can execute on same request
- Hook failure doesn't prevent onError hook execution

**Integration Test Example**:
```typescript
it("should execute lifecycle hooks in order", async () => {
  const executionOrder: string[] = [];

  app.hooks({
    onRequest: async (req) => {
      executionOrder.push("onRequest");
      return req;
    },
    preValidation: async (req) => {
      executionOrder.push("preValidation");
      return req;
    },
    preHandler: async (req) => {
      executionOrder.push("preHandler");
      return req;
    },
    onResponse: async (res) => {
      executionOrder.push("onResponse");
      return res;
    },
  });

  app.addRoute(
    {
      method: "GET",
      path: "/hooked",
      handler_name: "hookedHandler",
      is_async: true,
    },
    async () => {
      executionOrder.push("handler");
      return { status: 200, body: { order: executionOrder } };
    }
  );

  const response = await fetch(`${baseUrl}/hooked`);
  const body = await response.json();

  expect(body.order).toEqual([
    "onRequest",
    "preValidation",
    "preHandler",
    "handler",
    "onResponse",
  ]);
});
```

### 5. SSE/WebSocket Callback Chains
**Path**: Handler initiates SSE/WS → next_event callback (TSFN) → JavaScript generator → Promise → Rust → send event → repeat

**Tests**:
- SSE client receives events from JavaScript handler
- WebSocket message handler called for each incoming message
- Client disconnect triggers cleanup (stream closed, no hung promises)
- Multiple concurrent SSE/WS connections isolated
- Callback error doesn't close stream unexpectedly
- ThreadsafeFunction safe to call after stream closes
- Resource cleanup after stream termination (no memory leak)

**Integration Test Example**:
```typescript
it("should stream SSE events via callback chain", async () => {
  const eventCount = { value: 0 };

  app.addRoute(
    {
      method: "GET",
      path: "/events",
      handler_name: "sseHandler",
      is_async: true,
    },
    async (input) => {
      return {
        status: 200,
        headers: { "Content-Type": "text/event-stream" },
        stream: async function* () {
          for (let i = 0; i < 3; i++) {
            eventCount.value++;
            yield { data: { index: i } };
            await new Promise((r) => setTimeout(r, 10));
          }
        },
      };
    }
  );

  const response = await fetch(`${baseUrl}/events`);
  const reader = response.body?.getReader();
  const decoder = new TextDecoder();

  let text = "";
  while (true) {
    const { done, value } = await reader!.read();
    if (done) break;
    text += decoder.decode(value);
  }

  expect(eventCount.value).toBe(3);
  expect(text).toContain("index: 0");
  expect(text).toContain("index: 2");
});
```

### 6. Background Task Spawning from Handlers
**Path**: Handler calls `spawn_background(callback)` → Rust spawns tokio task → Task calls callback (TSFN) → JavaScript function executes asynchronously

**Tests**:
- Background task spawned and executed
- Background task can spawn additional tasks
- Task cancellation works (early return)
- Task failure isolated (doesn't affect handler response)
- Multiple concurrent tasks don't starve event loop
- Memory cleanup after task completion (no leak)
- Task state not corrupted by exceptions

**Integration Test Example**:
```typescript
it("should spawn background tasks from handler", async () => {
  const taskLog: string[] = [];

  app.addRoute(
    {
      method: "POST",
      path: "/bg-task",
      handler_name: "bgHandler",
      is_async: true,
    },
    async (input) => {
      // Task executes after response sent
      app.spawnBackground(async () => {
        taskLog.push("task_start");
        await new Promise((r) => setTimeout(r, 50));
        taskLog.push("task_complete");
      });

      return { status: 200, body: { queued: true } };
    }
  );

  const response = await fetch(`${baseUrl}/bg-task`, { method: "POST" });
  expect(response.status).toBe(200);

  // Wait for background task to complete
  await new Promise((r) => setTimeout(r, 100));
  expect(taskLog).toContain("task_complete");
});
```

## Acceptance Criteria: Proper FFI Coverage

A test suite achieves **proper FFI coverage** when:

1. **Handler Invocation (3+ tests)**
   - Input marshaling verified (method, path, headers, body, params)
   - Output conversion verified (status, headers, body)
   - At least 2 status codes tested (200, 5xx)

2. **Promise Handling (5+ tests)**
   - Successful promise resolution
   - Promise rejection with Error object
   - Promise rejection with string/non-Error value
   - Promise timeout
   - Concurrent promises (10+) isolated

3. **Error Structure (3+ tests)**
   - All errors return `{ error: string, code: string, details: object }`
   - Stack traces preserved
   - Multiple error sources handled

4. **DI Factory (4+ tests)**
   - Factory resolution called via TSFN
   - Singleton vs. non-singleton behavior
   - Factory promise rejection handling
   - Cache not poisoned after failure

5. **Lifecycle Hooks (4+ tests)**
   - All five hooks execute in correct order
   - Hook rejection short-circuits request
   - Hook timeout detected
   - Multiple hooks on same request

6. **Streaming (SSE/WebSocket) (3+ tests)**
   - Events/messages delivered to client
   - Client disconnect cleanup
   - Concurrent connections isolated
   - Resource cleanup (memory not leaked)

7. **Background Tasks (3+ tests)**
   - Task spawning and execution
   - Task failure isolation
   - Memory cleanup after completion

8. **Concurrency & Isolation (5+ tests)**
   - 10+ concurrent requests without state corruption
   - Partial failures don't poison app state
   - Mixed success/failure requests tracked accurately

9. **Configuration (3+ tests)**
   - Compression config applied
   - Rate limiting enforced
   - Timeout detection working

**Total Target**: 35+ integration tests covering all critical FFI paths.

## Running Integration Tests

### Local Development
```bash
# Start test server and run tests
cd packages/node
npm run test:integration

# Or with Vitest directly
npx vitest run packages/node/tests/integration/
```

### CI/CD
```bash
# CI task should build binding, start server, run tests
task test:node:integration

# Verify coverage meets threshold
npx vitest run --coverage packages/node/tests/integration/
```

### Test Isolation & Cleanup
- Each test file has `beforeAll/afterAll` for server lifecycle
- Port 0 lets OS assign free port (no conflicts)
- Graceful server shutdown before process exit
- Cleanup handlers for streams, connections, background tasks

## Documentation & Examples

Each integration test directory should have a `README.md`:
- **What**: Specific FFI path being tested
- **Why**: Which Rust code path depends on this
- **How**: Test setup, assertions, cleanup

Example test names (self-documenting):
- `handler-invocation.spec.ts`
- `di-factory-resolution.spec.ts`
- `lifecycle-hooks-order.spec.ts`
- `sse-event-streaming.spec.ts`
- `promise-rejection-handling.spec.ts`
- `concurrent-requests-isolation.spec.ts`

## Common Pitfalls & Solutions

### Pitfall 1: Server Port Conflicts
**Problem**: Tests fail with "EADDRINUSE" if port hardcoded.
**Solution**: Use `port: 0` to let OS assign free port; read actual port from `server.address()`.

### Pitfall 2: Promises Hang
**Problem**: `await fetch(...)` never completes if server crashes.
**Solution**: Add timeout to fetch: `fetch(url, { signal: AbortSignal.timeout(5000) })`.

### Pitfall 3: Resource Leaks
**Problem**: Streams/connections not cleaned up between tests.
**Solution**: Always use `afterAll/afterEach` to close server, abort fetch signals, disconnect clients.

### Pitfall 4: Race Conditions
**Problem**: Background task hasn't completed by assertion time.
**Solution**: Add deterministic wait: `await new Promise(r => setTimeout(r, 100))` or use completion flag.

### Pitfall 5: Flaky Timing Tests
**Problem**: Concurrency tests fail intermittently due to timing.
**Solution**: Use tokio::time::sleep in handlers to control timing; verify order via event logs, not wall clock.

## Migration Path

1. **Phase 1** (Done): Unit tests in `crates/spikard-node/tests/` verify Rust-level logic
2. **Phase 2** (This Plan): Integration tests in `packages/node/tests/integration/` verify FFI
3. **Phase 3**: Fixture-driven parity tests verify Node.js behaves like Python/Ruby/PHP

Each phase is independent; Phase 2 doesn't require Phase 1 or 3 to succeed.

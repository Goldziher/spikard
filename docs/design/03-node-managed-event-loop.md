# Architecture: Node-Managed Event Loop

**Status**: In Progress
**Decision Date**: 2025-11-08
**Impact**: New feature - applies Python-managed pattern to Node.js

## Problem

Following the same issues identified in Python (see `02-python-managed-event-loop.md`):
- Need natural async/await support in Node.js handlers
- Rust should not manage Node's event loop
- Want consistent cross-language architecture pattern

## Solution

Apply the same "language-managed" pattern to Node.js:
- **Node runs Spikard** (not Rust embeds Node)
- Node manages its own event loop
- Rust extension provides HTTP server (Axum)
- Natural async/await support

## Architecture

### Node-Managed Pattern
```
┌─────────────────────────────────────┐
│ Node Process (node server.js)      │
│ ┌─────────────────────────────────┐│
│ │ Node Event Loop (libuv)         ││  ✅ Node controls!
│ │  - Manages async/await          ││  ✅ Async works!
│ └─────────────────────────────────┘│
│              ↓                      │
│ ┌─────────────────────────────────┐│
│ │ spikard-node.node (Rust)        ││
│ │  └─ Axum HTTP Server (Tokio)   ││  ✅ Fast HTTP!
│ └─────────────────────────────────┘│
└─────────────────────────────────────┘

Usage: node server.js (or bun server.js)
```

## Implementation Strategy

### 1. Rust Extension API (napi-rs)
**File**: `crates/spikard-node/src/lib.rs`

```rust
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Run Spikard server from Node.js
#[napi]
pub fn run_server(
    app: Object,
    host: Option<String>,
    port: Option<u32>,
) -> Result<()> {
    // Extract routes from Node app object
    // Build Axum server with Node handlers
    // Start Tokio runtime
    // Run server until Ctrl+C
}
```

### 2. Node Handler Implementation
**File**: `crates/spikard-node/src/handler.rs`

Key considerations:
- Use `napi::threadsafe_function` for calling JS from Rust
- Async handlers return Promises
- Convert between Rust and JS types efficiently

```rust
use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use spikard_http::{Handler, HandlerResult, RequestData};

pub struct NodeHandler {
    handler: ThreadsafeFunction<RequestData, Result<ResponseResult>>,
    is_async: bool,
}

impl Handler for NodeHandler {
    fn handle(&self, req: Request<Body>, data: RequestData)
        -> Pin<Box<dyn Future<Output = HandlerResult> + Send>>
    {
        // Call Node.js handler via threadsafe function
        // Handle both sync and async JS functions
        // Convert JS response to Rust types
    }
}
```

### 3. TypeScript/JavaScript API
**File**: `packages/node/src/server.ts`

```typescript
import { runServer as nativeRunServer } from '../spikard-node.node';
import type { SpikardApp } from './index';

export interface ServerOptions {
  host?: string;
  port?: number;
}

export function runServer(app: SpikardApp, options: ServerOptions = {}): void {
  const { host = '127.0.0.1', port = 8000 } = options;

  // Call native Rust function
  nativeRunServer(app, host, port);
}
```

### 4. Application Class
**File**: `packages/node/src/app.ts`

```typescript
export class Spikard {
  routes: RouteMetadata[] = [];
  handlers: Map<string, Function> = new Map();

  run(options: ServerOptions = {}): void {
    runServer(this, options);
  }
}
```

## Async Handler Execution

### Strategy
Unlike Python which uses `asyncio.run()`, Node.js handlers:
1. **Async handlers**: Return Promises - napi-rs handles this natively
2. **Sync handlers**: Return values directly
3. **Event loop**: Managed entirely by Node.js (libuv)

### Implementation Pattern
```rust
// In NodeHandler::handle()
let call_result = self.handler.call_async(request_data).await?;

// napi-rs automatically handles:
// - Promise unwrapping for async functions
// - Direct return for sync functions
// - Event loop coordination
```

## TypeScript Decorators

Support familiar decorator patterns:

```typescript
import { Spikard, get, post } from '@spikard/node';

const app = new Spikard();

@get('/')
async function root() {
  return { message: 'Hello' };
}

@post('/users')
async function createUser(body: Body<UserSchema>) {
  return { id: 1, ...body };
}

if (require.main === module) {
  app.run({ host: '0.0.0.0', port: 8000 });
}
```

## Benefits

1. ✅ **Natural async/await** - Node manages event loop
2. ✅ **Familiar DX** - matches Express/Fastify patterns
3. ✅ **Bun compatible** - same API works with Bun runtime
4. ✅ **Cross-language consistency** - same pattern as Python
5. ✅ **Type safety** - Full TypeScript support
6. ✅ **Performance** - Rust HTTP server + Node handlers

## Bun Support

Same API works with Bun:
```bash
bun server.js
```

Bun's faster JavaScript runtime should provide:
- Faster startup times
- Lower memory usage
- Better async performance

## Comparison with Other Frameworks

### vs Express
```typescript
// Express
app.get('/', (req, res) => {
  res.json({ message: 'Hello' });
});

// Spikard
@get('/')
async function root() {
  return { message: 'Hello' };
}
```

### vs Fastify
```typescript
// Fastify
fastify.get('/', async (request, reply) => {
  return { message: 'Hello' };
});

// Spikard
@get('/')
async function root() {
  return { message: 'Hello' };
}
```

## Performance Expectations

Based on Rust HTTP server (Axum) + Node handlers:
- **Sync handlers**: <1ms latency (similar to Python sync)
- **Async handlers**: 1-2ms latency (Node event loop overhead)
- **Throughput**: Higher than pure Node frameworks due to Rust HTTP layer
- **Bun**: Expected 20-30% improvement over Node

## Implementation Phases

### Phase 1: Core Handler (Current)
- [ ] Implement `NodeHandler` with `ThreadsafeFunction`
- [ ] Support sync and async handlers
- [ ] Request/response conversion

### Phase 2: Application API
- [ ] Create `Spikard` class
- [ ] Implement route decorators
- [ ] Add TypeScript types

### Phase 3: Benchmark Infrastructure
- [ ] Generate benchmark server app
- [ ] Add Node to benchmark harness
- [ ] Run comparative benchmarks

### Phase 4: Advanced Features
- [ ] Schema validation (Zod integration)
- [ ] Error handling
- [ ] Middleware support

## References

- napi-rs guide: https://napi.rs/
- napi-rs async: https://napi.rs/docs/concepts/async-task
- Node.js async patterns: https://nodejs.org/api/async_hooks.html
- Bun compatibility: https://bun.sh/docs/runtime/nodejs-apis

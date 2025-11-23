# Dependency Injection Implementation Plan

**Version:** 1.0
**Date:** 2025-11-23
**Status:** In Progress
**Branch:** `feature/dependency-injection`
**Worktree:** `~/workspace/worktrees/spikard-di`

---

## Executive Summary

This document provides a comprehensive implementation plan for adding dependency injection (DI) to Spikard. The DI system will be built on Axum's State pattern, combining Fastify's simplicity with Litestar's power, implemented in Rust core with bindings for Python, Node.js, Ruby, and WASM.

**Timeline:** 10 weeks
**Effort:** ~400 hours
**Risk Level:** Medium (custom implementation, cross-language complexity)

---

## Table of Contents

1. [Background & Motivation](#background--motivation)
2. [Architecture Overview](#architecture-overview)
3. [Design Principles](#design-principles)
4. [Implementation Phases](#implementation-phases)
5. [Code Structure](#code-structure)
6. [Testing Strategy](#testing-strategy)
7. [Performance Targets](#performance-targets)
8. [Migration Guide](#migration-guide)
9. [Risk Assessment](#risk-assessment)
10. [Success Criteria](#success-criteria)

---

## Background & Motivation

### Current State

Spikard currently lacks a dependency injection system. Developers must:
- Manually create services in every handler
- Use Axum's `State<T>` for shared state (limited to one type)
- Implement their own patterns for resource lifecycle
- Duplicate service setup across bindings

### Problems This Solves

1. **Boilerplate Reduction** - No more manual service instantiation
2. **Loose Coupling** - Handlers depend on abstractions, not concrete types
3. **Testability** - Easy to mock dependencies for testing
4. **Resource Management** - First-class cleanup support (generators, Drop)
5. **Cross-Language Consistency** - Same DI patterns across all bindings

### Goals

- ✅ **Simple API** - Learn in <5 minutes (like Fastify)
- ✅ **Type-Safe** - Compile-time guarantees (Rust's type system)
- ✅ **Powerful** - Nested dependencies, async, cleanup (like Litestar)
- ✅ **Zero-Cost** - No overhead when not used
- ✅ **Cross-Language** - Python, Node, Ruby, WASM support

---

## Architecture Overview

### High-Level Design

```
┌─────────────────────────────────────────────────────────────┐
│                      Language Bindings                       │
│  Python (PyO3)  │  Node (napi-rs)  │  Ruby (magnus)  │ WASM │
└───────────────┬─────────────────────┬───────────────────────┘
                │                     │
                ▼                     ▼
        ┌──────────────────────────────────────┐
        │      spikard-http (HTTP Runtime)     │
        │  • DependencyInjectingHandler        │
        │  • Route registration                │
        └──────────────┬───────────────────────┘
                       │
                       ▼
        ┌──────────────────────────────────────┐
        │    spikard-core (Core DI System)     │
        │  • DependencyContainer               │
        │  • DependencyGraph                   │
        │  • Dependency trait                  │
        │  • ValueDependency                   │
        │  • FactoryDependency                 │
        └──────────────────────────────────────┘
```

### Core Components

#### 1. Dependency Trait

**Location:** `crates/spikard-core/src/di.rs`

The foundation for all injectable dependencies.

```rust
pub trait Dependency: Send + Sync {
    fn resolve(
        &self,
        request: &Request<Body>,
        request_data: &RequestData,
        resolved: &ResolvedDependencies,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send>>;

    fn key(&self) -> &str;
    fn depends_on(&self) -> Vec<String>;
    fn cacheable(&self) -> bool { false }
    fn singleton(&self) -> bool { false }
}
```

**Key Design Decisions:**
- Returns `Arc<dyn Any>` for type erasure (heterogeneous container)
- Async by default (`Pin<Box<dyn Future>>`)
- Takes `resolved` for nested dependency access
- Separate caching strategies (singleton vs per-request)

#### 2. DependencyContainer

**Location:** `crates/spikard-core/src/di.rs`

Stores and resolves dependencies.

```rust
pub struct DependencyContainer {
    dependencies: HashMap<String, Arc<dyn Dependency>>,
    dependency_graph: DependencyGraph,
    singleton_cache: Arc<RwLock<HashMap<String, Arc<dyn Any + Send + Sync>>>>,
}

impl DependencyContainer {
    pub fn new() -> Self;
    pub fn register(&mut self, key: String, dep: Arc<dyn Dependency>) -> Result<&mut Self>;
    pub async fn resolve_for_handler(&self, deps: &[String], req: &Request, data: &RequestData)
        -> Result<ResolvedDependencies>;
}
```

**Key Features:**
- Registration with cycle detection
- Batched parallel resolution
- Singleton caching (Arc<RwLock<>>)
- Type-safe retrieval with downcast

#### 3. DependencyGraph

**Location:** `crates/spikard-core/src/di.rs`

Manages dependency relationships and enables parallel resolution.

```rust
struct DependencyGraph {
    graph: HashMap<String, Vec<String>>,
}

impl DependencyGraph {
    fn add_dependency(&mut self, key: &str, depends_on: Vec<String>) -> Result<()>;
    fn calculate_batches(&self, keys: &[String]) -> Result<Vec<HashSet<String>>>;
    fn has_cycle_with(&self, new_key: &str, new_deps: &[String]) -> bool;
}
```

**Algorithm:**
- **Topological Sort** - Dependencies with no sub-deps in first batch
- **Batched Execution** - Each batch can run in parallel
- **Cycle Detection** - DFS at registration time

**Example:**
```
Dependencies: db -> config, cache -> config, auth -> [db, cache]

Batch 1 (parallel): config
Batch 2 (parallel): db, cache
Batch 3 (sequential): auth
```

#### 4. Built-in Dependency Types

**ValueDependency** - Static values (config, constants)
```rust
pub struct ValueDependency<T: Clone + Send + Sync + 'static> {
    key: String,
    value: Arc<T>,
}
```

**FactoryDependency** - Dynamic creation (DB pools, sessions)
```rust
pub struct FactoryDependency {
    key: String,
    factory: Arc<dyn Fn(...) -> BoxFuture<...>>,
    dependencies: Vec<String>,
    cacheable: bool,
    singleton: bool,
}
```

#### 5. Handler Integration

**DependencyInjectingHandler** - Wraps handlers with DI
```rust
pub struct DependencyInjectingHandler {
    inner: Arc<dyn Handler>,
    container: Arc<DependencyContainer>,
    required_dependencies: Vec<String>,
}
```

**Execution Flow:**
1. Resolve dependencies (batched, parallel)
2. Attach to `request_data.dependencies`
3. Call inner handler
4. Cleanup (async Drop pattern)

#### 6. ServerConfig Integration

```rust
impl ServerConfigBuilder {
    pub fn provide_value<T>(&mut self, key: impl Into<String>, value: T) -> &mut Self;
    pub fn provide_factory<F, Fut, T>(&mut self, key: impl Into<String>, factory: F) -> &mut Self;
}
```

---

## Design Principles

### 1. Zero-Cost Abstraction

**When DI Not Used:**
```rust
let config = ServerConfig::builder().build();
// dependency_container = None
// Zero overhead - no allocation, no checks
```

**When DI Used:**
```rust
let config = ServerConfig::builder()
    .provide_value("config", AppConfig::load())
    .build();
// dependency_container = Some(Arc<Container>)
// Minimal overhead - Arc clone per request
```

### 2. Type Safety

**Compile-Time Checks:**
- Dependencies must be `Send + Sync` (enforced by trait bounds)
- Factory signatures validated by type system
- Invalid casts caught by `downcast` at runtime with clear errors

**Runtime Type Safety:**
```rust
// Type-safe retrieval
let db: Arc<DatabasePool> = resolved.get_arc("db").ok_or(...)?;

// Type mismatch caught immediately
let wrong: Arc<Cache> = resolved.get_arc("db").ok_or(...)?;  // Returns None
```

### 3. Simplicity

**API Surface:**
1. `provide_value(key, value)` - Register static value
2. `provide_factory(key, factory)` - Register factory
3. That's it! (optional: scopes, cleanup)

**Example:**
```rust
ServerConfig::builder()
    .provide_value("port", 3000)
    .provide_factory("db", |req, data, resolved| async {
        Ok(DatabasePool::new().await?)
    })
    .build()
```

### 4. Cross-Language Parity

**Same Semantics Across Bindings:**

**Python:**
```python
app.provide("db", Provide(create_db_pool, depends_on=["config"]))
```

**TypeScript:**
```typescript
app.provide('db', Provide(createDbPool, { dependsOn: ['config'] }))
```

**Ruby:**
```ruby
app.provide(:db, Spikard::Provide.new(create_db_pool, depends_on: [:config]))
```

**Core Principle:** Rust does the work, bindings provide idiomatic APIs

---

## Implementation Phases

### Phase 1: Core Foundation (Week 1-2)

**Deliverables:**
- `crates/spikard-core/src/di.rs` with all core types
- `Dependency` trait implementation
- `DependencyContainer` with registration and resolution
- `DependencyGraph` with cycle detection and batching
- `ValueDependency` and `FactoryDependency`
- Unit tests for all components

**Key Files:**
```
crates/spikard-core/src/
├── di.rs                    # Main DI module (NEW)
├── di/
│   ├── mod.rs
│   ├── container.rs        # DependencyContainer
│   ├── dependency.rs       # Dependency trait
│   ├── graph.rs            # DependencyGraph
│   ├── value.rs            # ValueDependency
│   ├── factory.rs          # FactoryDependency
│   └── error.rs            # DependencyError
└── lib.rs                  # Export di module
```

**Test Coverage:**
- DependencyGraph batching algorithm
- Cycle detection (various graph shapes)
- ValueDependency resolution
- FactoryDependency async resolution
- Singleton caching
- Per-request caching

**Acceptance Criteria:**
- [ ] All unit tests pass
- [ ] Cargo clippy clean
- [ ] Rustdoc complete
- [ ] Coverage >= 95%

---

### Phase 2: Handler Integration (Week 2-3)

**Deliverables:**
- `DependencyInjectingHandler` wrapper
- `RequestData` update to carry dependencies
- Router integration
- Lifecycle hooks integration
- Observability (tracing spans)

**Key Files:**
```
crates/spikard-http/src/
├── handler_trait.rs         # Add DependencyInjectingHandler
├── server.rs                # Update build_router_with_handlers
└── request_data.rs          # Add dependencies field (if separate)
```

**Integration Points:**
1. **ValidatingHandler** - DI runs after validation
2. **LifecycleHooks** - DI dependencies available in hooks
3. **Tracing** - Spans for resolution time

**Acceptance Criteria:**
- [ ] Handler wrapper tests pass
- [ ] Integration with existing handlers works
- [ ] Cleanup executes after handler
- [ ] Tracing spans visible in logs

---

### Phase 3: ServerConfig Integration (Week 3-4)

**Deliverables:**
- `ServerConfig::dependency_container` field
- `ServerConfigBuilder` helper methods
- Route-level dependency registration (optional)
- Examples in `examples/di/rust_basic.rs`

**Key Files:**
```
crates/spikard-core/src/
├── config.rs                # Add dependency_container field
└── config/builder.rs        # Add provide_* methods

examples/di/
└── rust_basic.rs            # Basic DI example
```

**API Design:**
```rust
let config = ServerConfig::builder()
    .port(3000)
    .provide_value("app_name", "MyApp".to_string())
    .provide_factory("db", |req, data, resolved| async {
        let config = resolved.get::<AppConfig>("config")?;
        Ok(DatabasePool::connect(&config.db_url).await?)
    })
    .build();
```

**Acceptance Criteria:**
- [ ] Builder tests pass
- [ ] Examples run successfully
- [ ] Documentation updated
- [ ] CLAUDE.md updated with DI rules

---

### Phase 4: Fixtures & Testing (Week 4-5)

**Deliverables:**
- `testing_data/di/` fixtures
- Python integration tests
- Fixture validation in CI
- Coverage reports

**Fixture Structure:**
```
testing_data/di/
├── basic_value/
│   ├── schema.json
│   ├── input.json
│   └── expected.json
├── factory/
│   └── ...
├── singleton/
│   └── ...
├── nested_dependencies/
│   └── ...
├── circular_dependency_error/
│   └── ...
└── cleanup/
    └── ...
```

**Python Tests:**
```python
# packages/python/tests/test_di.py

def test_value_injection(fixture_app):
    """Test simple value injection"""
    app = create_app_with_fixture("testing_data/di/basic_value")
    response = app.test_client().get("/")
    assert response.json() == load_expected("testing_data/di/basic_value")

async def test_async_factory(fixture_app):
    """Test async factory dependency"""
    # ...

def test_cleanup_executed():
    """Test cleanup runs after request"""
    # ...
```

**Acceptance Criteria:**
- [ ] All fixtures created
- [ ] Python tests >= 80% coverage
- [ ] Fixtures pass in CI
- [ ] `task test:python` passes

---

### Phase 5: Python Binding (Week 5-6)

**Deliverables:**
- `packages/python/spikard/di.py` module
- `Provide` class
- `app.provide()` method
- Parameter introspection for auto-injection
- Generator pattern support
- Examples and documentation

**Key Files:**
```
packages/python/spikard/
├── di.py                    # DI module (NEW)
├── _internal/
│   └── di_converter.py      # Rust -> Python conversion
└── __init__.py              # Export Provide

examples/di/
├── python_basic.py
├── python_database.py       # With cleanup
└── python_nested.py         # Nested dependencies
```

**Python API:**
```python
from spikard import Server, Provide

app = Server()

# Value
app.provide("config", AppConfig(port=3000))

# Factory (sync)
def create_db(config: AppConfig) -> Database:
    return Database(config.db_url)

app.provide("db", Provide(create_db, depends_on=["config"]))

# Factory (async with cleanup)
async def create_session(db: Database):
    session = await db.create_session()
    yield session
    await session.close()

app.provide("session", Provide(create_session, depends_on=["db"]))

# Handler with auto-injection
@app.get("/users")
async def get_users(session: Session) -> list[User]:
    return await session.query(User).all()
```

**Implementation Details:**

1. **Provide Class:**
```python
class Provide:
    def __init__(
        self,
        dependency: Callable,
        *,
        depends_on: list[str] | None = None,
        use_cache: bool = False,
        singleton: bool = False,
    ):
        self.dependency = dependency
        self.depends_on = depends_on or []
        self.use_cache = use_cache
        self.singleton = singleton
        self.is_async = asyncio.iscoroutinefunction(dependency)
        self.is_generator = inspect.isgeneratorfunction(dependency)
```

2. **Parameter Introspection:**
```python
def inject_dependencies(
    handler: Callable,
    resolved: dict[str, Any],
) -> dict[str, Any]:
    """Match dependencies to handler parameters by name and type"""
    sig = inspect.signature(handler)
    kwargs = {}

    for param_name, param in sig.parameters.items():
        # Try name match first
        if param_name in resolved:
            kwargs[param_name] = resolved[param_name]
        # Try type match
        elif param.annotation in resolved.values():
            kwargs[param_name] = next(
                v for v in resolved.values() if isinstance(v, param.annotation)
            )

    return kwargs
```

**Acceptance Criteria:**
- [ ] All Python examples run
- [ ] Type hints complete
- [ ] Docstrings (NumPy style)
- [ ] Tests >= 80% coverage
- [ ] Python README updated

---

### Phase 6: Node/TypeScript Binding (Week 6-7)

**Deliverables:**
- `crates/spikard-node/src/di.rs` module
- TypeScript `Provide` class
- `.d.ts` type definitions
- Examples and documentation

**Key Files:**
```
crates/spikard-node/src/
├── di.rs                    # DI FFI bindings

packages/node/src/
├── di.ts                    # Provide class
└── index.ts                 # Export

examples/di/
├── node_basic.ts
└── node_database.ts
```

**TypeScript API:**
```typescript
import { Server, Provide } from '@spikard/node';

const app = new Server();

// Value
app.provide('config', { port: 3000 });

// Factory
app.provide('db', Provide(
  async (config: AppConfig) => {
    return await createConnection(config.dbUrl);
  },
  { dependsOn: ['config'], singleton: true }
));

// Handler with destructuring
app.get('/users', async (request, { db }: { db: Database }) => {
  return await db.query('SELECT * FROM users');
});
```

**Type Definitions:**
```typescript
export class Provide<T> {
  constructor(
    factory: (...deps: any[]) => Promise<T> | T,
    options?: {
      dependsOn?: string[];
      singleton?: boolean;
      useCache?: boolean;
    }
  );
}

export interface Server {
  provide<T>(key: string, value: T | Provide<T>): this;
}
```

**Acceptance Criteria:**
- [ ] All Node examples run
- [ ] `.d.ts` types complete
- [ ] JSDoc comments
- [ ] Tests >= 80% coverage
- [ ] Node README updated

---

### Phase 7: Ruby Binding (Week 7-8)

**Deliverables:**
- `crates/spikard-rb/src/di.rs` module
- `Spikard::Provide` class
- RBS type definitions
- Examples and documentation

**Key Files:**
```
crates/spikard-rb/src/
├── di.rs                    # DI FFI bindings

sig/spikard/
└── di.rbs                   # RBS types

examples/di/
├── ruby_basic.rb
└── ruby_database.rb
```

**Ruby API:**
```ruby
require 'spikard'

app = Spikard::Server.new

# Value
app.provide(:config, { port: 3000 })

# Factory with lambda
app.provide(:db, Spikard::Provide.new(
  -> (config) { Database.connect(config[:db_url]) },
  depends_on: [:config],
  singleton: true
))

# Handler with keyword args
app.get('/users') do |request, db:|
  { users: db.query('SELECT * FROM users') }
end
```

**RBS Types:**
```ruby
module Spikard
  class Provide
    def initialize: (
      ^(untyped) -> untyped factory,
      ?depends_on: Array[Symbol],
      ?singleton: bool,
      ?use_cache: bool
    ) -> void
  end

  class Server
    def provide: (Symbol key, untyped value) -> self
  end
end
```

**Acceptance Criteria:**
- [ ] All Ruby examples run
- [ ] RBS types complete
- [ ] YARD documentation
- [ ] Tests >= 80% coverage
- [ ] Ruby README updated

---

### Phase 8: WASM Binding (Week 8)

**Deliverables:**
- `crates/spikard-wasm/src/di.rs` module
- JavaScript `Provide` wrapper
- Browser + Node/Deno compatibility
- Examples and documentation

**Key Files:**
```
crates/spikard-wasm/src/
├── di.rs                    # DI WASM bindings

examples/di/
└── wasm_basic.js
```

**WASM API:**
```javascript
import { Server, Provide } from '@spikard/wasm';

const app = new Server();

// Value
app.provide('config', { port: 3000 });

// Factory
app.provide('db', Provide(
  async (config) => {
    return await Database.connect(config.dbUrl);
  },
  { dependsOn: ['config'] }
));
```

**Acceptance Criteria:**
- [ ] WASM examples run in browser
- [ ] WASM examples run in Node
- [ ] TypeScript types generated
- [ ] WASM README updated

---

### Phase 9: Documentation & Polish (Week 9)

**Deliverables:**
- Finalize ADR-0008
- Update all related ADRs
- Create migration guide
- Write comparison docs
- Review and polish all examples
- Update main README

**Documentation Checklist:**
- [ ] ADR-0008 complete
- [ ] Migration guide written
- [ ] Comparison doc (vs Litestar, Fastify, Shaku)
- [ ] Troubleshooting guide
- [ ] All examples reviewed
- [ ] Main README updated
- [ ] CHANGELOG.md entry

**Code Quality:**
- [ ] `task lint` clean
- [ ] `task format` applied
- [ ] All clippy warnings addressed
- [ ] All tests passing
- [ ] Coverage targets met

---

### Phase 10: Review & Merge (Week 10)

**Deliverables:**
- Self-review complete
- Performance benchmarks run
- Final testing
- PR created

**Review Checklist:**
- [ ] Code review (self)
- [ ] Security review
- [ ] Performance benchmarks
- [ ] All tests passing
- [ ] Documentation complete
- [ ] CHANGELOG updated
- [ ] Rebase on main
- [ ] PR created

**Benchmark Results:**
```
Baseline (no DI):            1000 req/s
DI with 0 dependencies:      995 req/s  (-0.5%)
DI with 1 dependency:        980 req/s  (-2.0%)
DI with 5 nested deps:       950 req/s  (-5.0%)
DI with singleton cache:     990 req/s  (-1.0%)
```

**Acceptance Criteria:**
- [ ] All phase deliverables complete
- [ ] Performance within targets (<5% overhead)
- [ ] Coverage targets met
- [ ] Documentation complete
- [ ] Ready for merge

---

## Code Structure

### New Files

```
crates/spikard-core/src/
├── di.rs                          # Main DI module
├── di/
│   ├── mod.rs                     # Module exports
│   ├── container.rs               # DependencyContainer
│   ├── dependency.rs              # Dependency trait
│   ├── graph.rs                   # DependencyGraph
│   ├── value.rs                   # ValueDependency
│   ├── factory.rs                 # FactoryDependency
│   ├── resolved.rs                # ResolvedDependencies
│   └── error.rs                   # DependencyError

crates/spikard-http/src/
├── di_handler.rs                  # DependencyInjectingHandler

packages/python/spikard/
├── di.py                          # Provide class
└── _internal/di_converter.py      # Conversion helpers

crates/spikard-node/src/
└── di.rs                          # Node DI bindings

crates/spikard-rb/src/
└── di.rs                          # Ruby DI bindings

crates/spikard-wasm/src/
└── di.rs                          # WASM DI bindings

testing_data/di/
├── basic_value/
├── factory/
├── singleton/
├── nested_dependencies/
├── circular_dependency_error/
└── cleanup/

examples/di/
├── rust_basic.rs
├── python_basic.py
├── python_database.py
├── python_nested.py
├── node_basic.ts
├── node_database.ts
├── ruby_basic.rb
├── ruby_database.rb
└── wasm_basic.js

docs/adr/
└── 0008-dependency-injection.md
```

### Modified Files

```
crates/spikard-core/src/
├── lib.rs                         # Export di module
└── config.rs                      # Add dependency_container

crates/spikard-http/src/
├── handler_trait.rs               # Export DependencyInjectingHandler
├── server.rs                      # Integrate DI wrapper
└── request_data.rs                # Add dependencies field

packages/python/spikard/
├── __init__.py                    # Export Provide
└── server.py                      # Add provide() method

CLAUDE.md                          # Add DI rules
README.md                          # Add DI section
CHANGELOG.md                       # Document DI addition
```

---

## Testing Strategy

### Unit Tests (Rust)

**Location:** `crates/spikard-core/src/di/tests.rs`

**Coverage:**
- DependencyGraph topological sorting
- Cycle detection (linear, branching, self-loops)
- ValueDependency resolution
- FactoryDependency async resolution
- Singleton caching behavior
- Per-request caching behavior
- Cleanup task execution
- Error handling (missing deps, type mismatch)

**Test Count:** ~50 unit tests

### Integration Tests (Python)

**Location:** `packages/python/tests/test_di.py`

**Coverage:**
- Value injection in handlers
- Factory injection (sync and async)
- Nested dependency resolution (3+ levels)
- Error handling (missing dependency)
- Error handling (circular dependency)
- Cleanup after request
- Type-based injection
- Name-based injection
- Generator pattern cleanup

**Test Count:** ~30 integration tests

### Fixture Tests

**Location:** `packages/python/tests/test_all_fixtures.py`

Parametrized tests loading all fixtures from `testing_data/di/`.

**Fixtures:**
1. basic_value - Simple value injection
2. factory - Factory dependency
3. singleton - Singleton caching
4. nested_dependencies - Multi-level deps
5. circular_dependency_error - Cycle detection
6. cleanup - Generator cleanup

### End-to-End Tests

**Location:** `packages/python/tests/test_di_e2e.py`

Full request flow tests:
- HTTP request → DI resolution → handler → response
- Tracing span verification
- Performance under load

### Coverage Targets

| Component | Target | Measurement |
|-----------|--------|-------------|
| Rust core | 95% | tarpaulin |
| Python | 80% | pytest-cov |
| Node | 80% | vitest |
| Ruby | 80% | simplecov |

---

## Performance Targets

### Overhead Benchmarks

| Scenario | Baseline | With DI | Overhead | Target |
|----------|----------|---------|----------|--------|
| No dependencies | 1000 req/s | 995 req/s | -0.5% | <1% |
| 1 dependency | 1000 req/s | 980 req/s | -2.0% | <2% |
| 5 nested deps | 1000 req/s | 950 req/s | -5.0% | <5% |
| Singleton cached | 1000 req/s | 990 req/s | -1.0% | <1% |

### Memory Usage

- **Container:** O(n) where n = number of registered dependencies
- **Per-request:** O(m) where m = dependencies used by handler
- **Singleton cache:** O(k) where k = number of singletons
- **Target:** <1 MB for 100 dependencies

### Resolution Time

- **0 dependencies:** <1μs (Option::None check)
- **1 dependency:** <10μs (Arc clone + downcast)
- **5 nested deps:** <100μs (parallel batches)
- **Singleton cache hit:** <1μs (HashMap lookup + Arc clone)

---

## Migration Guide

### For Existing Spikard Apps

#### Before (Axum State)

```rust
#[derive(Clone)]
struct AppState {
    db: Arc<DatabasePool>,
    cache: Arc<Cache>,
}

async fn handler(State(state): State<AppState>) -> String {
    let users = state.db.query("SELECT * FROM users").await?;
    format!("{:?}", users)
}

let app = Router::new()
    .route("/", get(handler))
    .with_state(AppState {
        db: Arc::new(DatabasePool::new()),
        cache: Arc::new(Cache::new()),
    });
```

#### After (With DI)

```rust
let config = ServerConfig::builder()
    .provide_value("db", Arc::new(DatabasePool::new()))
    .provide_value("cache", Arc::new(Cache::new()))
    .build();

async fn handler(db: Arc<DatabasePool>, cache: Arc<Cache>) -> String {
    let users = db.query("SELECT * FROM users").await?;
    format!("{:?}", users)
}

// Handler registration unchanged
```

**Benefits:**
- No AppState struct needed
- Dependencies extracted individually
- Easy to mock in tests
- Cleaner handler signatures

### Backward Compatibility

✅ **Existing State pattern continues to work**
✅ **DI is opt-in** via `ServerConfig::provide_*()`
✅ **Can mix State and DI** in same app
✅ **No breaking changes** to existing APIs

---

## Risk Assessment

### High Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Cross-language complexity | High | High | Thorough testing, clear FFI boundaries |
| Performance regression | Medium | High | Benchmarking, profiling, optimization |
| API design mistakes | Medium | High | User feedback, iterate on API |

### Medium Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Async cleanup complexity | Medium | Medium | Generator pattern, extensive testing |
| Type erasure issues | Low | Medium | Comprehensive downcast tests |
| Circular dependency bugs | Low | Medium | Robust cycle detection algorithm |

### Low Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Documentation gaps | Medium | Low | Review checklist, examples |
| Test coverage gaps | Low | Low | Coverage tracking, CI enforcement |

---

## Success Criteria

### Functional Requirements

- [ ] Value dependencies register and resolve
- [ ] Factory dependencies create instances on-demand
- [ ] Nested dependencies resolve in correct order
- [ ] Circular dependencies detected and rejected
- [ ] Singleton caching works across requests
- [ ] Per-request caching avoids duplicate work
- [ ] Cleanup executes after handler completes
- [ ] Python binding matches Litestar API style
- [ ] Node binding supports TypeScript types
- [ ] Ruby binding uses idiomatic keyword args
- [ ] WASM binding works in browser and Node

### Non-Functional Requirements

- [ ] Performance overhead <5% for complex cases
- [ ] Memory usage <1 MB for 100 dependencies
- [ ] Test coverage >= 95% (Rust), >= 80% (bindings)
- [ ] Documentation complete (ADR, examples, guides)
- [ ] Zero clippy warnings
- [ ] All CI checks pass

### User Experience

- [ ] API learnable in <5 minutes
- [ ] Examples run without modification
- [ ] Error messages are clear and actionable
- [ ] IDE autocomplete works (TypeScript, Python)
- [ ] Migration from State pattern is straightforward

---

## Appendix: Example Usage

### Rust

```rust
use spikard::{Server, ServerConfig};
use std::sync::Arc;

let config = ServerConfig::builder()
    .provide_value("app_name", "MyApp".to_string())
    .provide_factory("db", |req, data, resolved| async {
        let config = resolved.get::<AppConfig>("config")?;
        Ok(DatabasePool::connect(&config.db_url).await?)
    })
    .build();

async fn handler(app_name: String, db: Arc<DatabasePool>) -> String {
    format!("Welcome to {}!", app_name)
}

let server = Server::new(config);
```

### Python

```python
from spikard import Server, Provide

app = Server()

app.provide("config", AppConfig(db_url="postgresql://localhost/mydb"))

async def create_db_pool(config: AppConfig):
    pool = await create_pool(config.db_url)
    yield pool
    await pool.close()

app.provide("db", Provide(create_db_pool, depends_on=["config"]))

@app.get("/users")
async def get_users(db: DatabasePool) -> list[User]:
    return await db.fetch_all("SELECT * FROM users")
```

### TypeScript

```typescript
import { Server, Provide } from '@spikard/node';

const app = new Server();

app.provide('config', { dbUrl: 'postgresql://localhost/mydb' });

app.provide('db', Provide(
  async (config: AppConfig) => {
    return await createPool(config.dbUrl);
  },
  { dependsOn: ['config'], singleton: true }
));

app.get('/users', async (request, { db }: { db: DatabasePool }) => {
  return await db.query('SELECT * FROM users');
});
```

### Ruby

```ruby
app = Spikard::Server.new

app.provide(:config, { db_url: 'postgresql://localhost/mydb' })

app.provide(:db, Spikard::Provide.new(
  -> (config) { DatabasePool.connect(config[:db_url]) },
  depends_on: [:config],
  singleton: true
))

app.get('/users') do |request, db:|
  { users: db.query('SELECT * FROM users') }
end
```

---

**End of Implementation Plan**

For questions or clarifications, refer to:
- [ADR-0008: Dependency Injection](./docs/adr/0008-dependency-injection.md)
- [TODO.md](./TODO.md)
- [Spikard GitHub Issues](https://github.com/yourusername/spikard/issues)

# ADR 0008: Dependency Injection System

**Status:** Proposed
**Date:** 2025-11-23
**Authors:** Spikard Team
**Deciders:** Core maintainers

## Context and Problem Statement

Spikard currently lacks a dependency injection (DI) system, requiring developers to manually manage service instantiation, configuration, and lifecycle. This leads to:

1. **Boilerplate code** - Manual service creation in every handler
2. **Tight coupling** - Handlers directly depend on concrete implementations
3. **Testing difficulty** - Hard to mock dependencies without extensive setup
4. **Resource management** - No standard pattern for cleanup (database connections, file handles, etc.)
5. **Cross-language inconsistency** - Each binding (Python, Node, Ruby, PHP) implements its own patterns

We need a DI system that is:
- **Simple** - Minimal API surface (like Fastify's decoration pattern)
- **Powerful** - Type-driven resolution with nested dependencies (like Litestar)
- **Cross-language** - Same semantics across Python, TypeScript, Ruby, PHP, WASM
- **Zero-cost** - No overhead when not used
- **Rust-first** - Resolution logic in Rust core, not bindings

## Decision Drivers

1. **Simplicity over features** - Prefer minimal API over comprehensive DI framework
2. **Performance** - Zero-cost abstraction when DI not used, minimal overhead when used
3. **Type safety** - Leverage Rust's type system for compile-time guarantees
4. **Cross-language parity** - Same DI semantics across all bindings
5. **Existing ecosystem** - Build on Axum State pattern (already in our stack)
6. **Testability** - Easy to mock dependencies in tests
7. **Resource management** - First-class support for cleanup (generators, Drop)

## Considered Options

### Option 1: External DI Crate (Shaku)

**Description:** Use [shaku](https://github.com/AzureMarker/shaku), a compile-time DI library with Axum integration.

**Pros:**
- ✅ Battle-tested (559 GitHub stars, 123K downloads)
- ✅ Compile-time resolution (zero runtime overhead)
- ✅ Direct Axum integration via `shaku_axum`
- ✅ Module system for organization
- ✅ MIT/Apache-2.0 dual license

**Cons:**
- ❌ Macro-heavy API doesn't translate well to other languages
- ❌ Module-based registration differs from Fastify/Litestar patterns
- ❌ Limited async support (no async factories)
- ❌ External dependency to maintain
- ❌ Opinionated design (components vs providers)

**Verdict:** ❌ Rejected - Doesn't align with cross-language goals

### Option 2: Custom DI on Axum State (CHOSEN)

**Description:** Build lightweight DI system on Axum's `State<T>` pattern, inspired by Fastify (simplicity) + Litestar (power).

**Pros:**
- ✅ No external dependencies
- ✅ Full control over API design
- ✅ Can match Fastify/Litestar patterns exactly
- ✅ Zero-cost when not used (`Option<Arc<Container>>`)
- ✅ Already using Axum State
- ✅ Cross-language bindings can share Container type
- ✅ Incremental complexity (start simple, add features)

**Cons:**
- ⚠️ Must implement ourselves (initial development time)
- ⚠️ Need to maintain (but we control API)

**Verdict:** ✅ **CHOSEN** - Best fit for requirements

### Option 3: Axum State + Shaku Hybrid

**Description:** Use Axum State for simple dependencies, Shaku for complex DI scenarios.

**Pros:**
- ✅ Best of both worlds
- ✅ Proven framework for advanced use cases

**Cons:**
- ❌ Two different DI patterns to learn
- ❌ Confusing for users ("when to use which?")
- ❌ External dependency still required

**Verdict:** ❌ Rejected - Adds complexity without sufficient benefit

## Decision Outcome

**Chosen option:** **Option 2 - Custom DI on Axum State**

We will implement a custom dependency injection system built on Axum's `State<T>` pattern, combining:
- **Fastify's simplicity** - Minimal API (`provide_value`, `provide_factory`)
- **Litestar's power** - Type-driven resolution, dependency graphs, batched parallel execution
- **Rust's type safety** - Compile-time checks, zero-cost abstractions
- **Axum's patterns** - State, FromRef, extractors

## Architecture Design

### Core Abstractions

#### 1. Dependency Trait

```rust
/// Core trait for injectable dependencies
pub trait Dependency: Send + Sync {
    /// Resolve the dependency asynchronously
    fn resolve(
        &self,
        request: &Request<Body>,
        request_data: &RequestData,
        resolved: &ResolvedDependencies,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send>>;

    /// Unique key for this dependency
    fn key(&self) -> &str;

    /// Dependencies that must be resolved before this one
    fn depends_on(&self) -> Vec<String>;

    /// Should this dependency be cached per-request?
    fn cacheable(&self) -> bool { false }

    /// Should this dependency be cached globally (singleton)?
    fn singleton(&self) -> bool { false }
}
```

**Design rationale:**
- `Send + Sync` required for async/multi-threaded handlers
- Returns `Arc<dyn Any>` for type erasure (containers store heterogeneous types)
- Takes `resolved` dependencies for nested resolution
- Separate `cacheable` and `singleton` for flexibility

#### 2. DependencyContainer

```rust
pub struct DependencyContainer {
    dependencies: HashMap<String, Arc<dyn Dependency>>,
    dependency_graph: DependencyGraph,
    singleton_cache: Arc<RwLock<HashMap<String, Arc<dyn Any + Send + Sync>>>>,
}
```

**Key methods:**
- `register(key, dependency)` - Add dependency, detect cycles
- `resolve_for_handler(deps, request, data)` - Resolve batches in parallel
- `get<T>(key)` - Type-safe retrieval with downcast

**Design rationale:**
- `DependencyGraph` enables topological sorting for parallel resolution
- Singleton cache shared across requests (Arc<RwLock<>>)
- Cycle detection at registration time (fail fast)

#### 3. DependencyGraph

```rust
struct DependencyGraph {
    graph: HashMap<String, Vec<String>>,
}
```

**Key methods:**
- `add_dependency(key, depends_on)` - Add edge, check for cycles
- `calculate_batches(keys)` - Topological sort into parallel batches
- `has_cycle_with(new_key, new_deps)` - DFS cycle detection

**Design rationale:**
- Batched resolution enables parallelism (like Litestar)
- Dependencies with no sub-deps resolve in first batch
- Each batch can execute concurrently (tokio::spawn tasks)

#### 4. Built-in Dependency Types

```rust
/// Simple value dependency (like Fastify's decorate)
pub struct ValueDependency<T: Clone + Send + Sync + 'static> {
    key: String,
    value: Arc<T>,
}

/// Factory dependency (like Litestar's Provide)
pub struct FactoryDependency {
    key: String,
    factory: Arc<dyn Fn(&Request, &RequestData, &ResolvedDependencies) -> BoxFuture<...>>,
    dependencies: Vec<String>,
    cacheable: bool,
    singleton: bool,
}
```

**Design rationale:**
- `ValueDependency` for static values (config, constants)
- `FactoryDependency` for dynamic creation (DB connections, sessions)
- Factory can be async and depend on other dependencies
- Configurable caching strategy

### Handler Integration

#### DependencyInjectingHandler Wrapper

```rust
pub struct DependencyInjectingHandler {
    inner: Arc<dyn Handler>,
    container: Arc<DependencyContainer>,
    required_dependencies: Vec<String>,
}

impl Handler for DependencyInjectingHandler {
    fn call(&self, request: Request<Body>, mut request_data: RequestData)
        -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>>
    {
        Box::pin(async move {
            // 1. Resolve dependencies in parallel batches
            let resolved = self.container
                .resolve_for_handler(&self.required_dependencies, &request, &request_data)
                .await?;

            // 2. Attach to request_data
            request_data.dependencies = Some(Arc::new(resolved));

            // 3. Call inner handler
            let result = self.inner.call(request, request_data).await;

            // 4. Cleanup (async Drop pattern)
            if let Some(deps) = request_data.dependencies.take() {
                if let Ok(deps) = Arc::try_unwrap(deps) {
                    deps.cleanup().await;
                }
            }

            result
        })
    }
}
```

**Design rationale:**
- Wraps existing handler (composition over inheritance)
- Follows `ValidatingHandler` pattern already in codebase
- Cleanup happens after handler completes (generator pattern)
- Integrates with existing `RequestData` struct

### ServerConfig Integration

```rust
pub struct ServerConfig {
    // ... existing fields ...
    pub dependency_container: Option<Arc<DependencyContainer>>,
}

impl ServerConfigBuilder {
    /// Register a value dependency (like Fastify decorate)
    pub fn provide_value<T: Clone + Send + Sync + 'static>(
        mut self,
        key: impl Into<String>,
        value: T,
    ) -> Self {
        let dep = ValueDependency::new(key, value);
        self.dependency(dep)
    }

    /// Register a factory dependency (like Litestar Provide)
    pub fn provide_factory<F, Fut, T>(
        mut self,
        key: impl Into<String>,
        factory: F,
    ) -> Self
    where
        F: Fn(&Request<Body>, &RequestData, &ResolvedDependencies) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<T, String>> + Send + 'static,
        T: Send + Sync + 'static,
    {
        let dep = FactoryDependency::new(key, factory);
        self.dependency(dep)
    }
}
```

**Design rationale:**
- Builder pattern for ergonomic registration
- Two simple methods match Fastify/Litestar patterns
- Type inference reduces boilerplate

### Language Binding APIs

#### Python

```python
from spikard import Server, Provide

app = Server()

# Simple value
app.provide("db_url", "postgresql://localhost/mydb")

# Factory (sync or async)
async def get_db_session(db_url: str):
    async with sessionmaker(db_url) as session:
        yield session  # Cleanup after handler

app.provide("db", Provide(get_db_session, depends_on=["db_url"]))

# Handler with auto-injection
@app.get("/users")
async def get_users(db: AsyncSession) -> list[User]:
    # db is injected by matching type or name
    return await db.query(User).all()
```

**Design rationale:**
- `Provide` wrapper matches Litestar API
- Generator pattern for cleanup (Pythonic)
- Type hints enable auto-injection
- Parameter name or type annotation for matching

#### TypeScript/Node

```typescript
import { Server, Provide } from 'spikard';

const app = new Server();

// Simple value
app.provide('dbUrl', 'postgresql://localhost/mydb');

// Factory
app.provide('db', Provide(async (dbUrl: string) => {
  return await createConnection(dbUrl);
}, { dependsOn: ['dbUrl'], singleton: true }));

// Handler with destructuring
app.get('/users', async (request, { db }: { db: Database }) => {
  return await db.query('SELECT * FROM users');
});
```

**Design rationale:**
- Object destructuring for dependency access
- TypeScript types for safety
- Optional decorator support (future)

#### Ruby

```ruby
app = Spikard::Server.new

# Simple value
app.provide(:db_url, 'postgresql://localhost/mydb')

# Factory with block
app.provide(:db, Spikard::Provide.new(
  -> (db_url) { DBConnection.new(db_url) },
  depends_on: [:db_url]
))

# Handler with keyword args
app.get('/users') do |request, db:|
  { users: db.query('SELECT * FROM users') }
end
```

**Design rationale:**
- Keyword arguments (idiomatic Ruby)
- Blocks and procs supported
- Symbol keys (Ruby convention)

#### PHP

```php
<?php
use Spikard\App;
use Spikard\DI\Provide;
use Spikard\Attributes\Get;
use Spikard\Http\Request;
use Spikard\Http\Response;

$app = new App();

// Simple value
$app->provide('dbUrl', 'postgresql://localhost/mydb');

// Factory with closure
$app->provide('db', new Provide(
    fn(string $dbUrl) => new PDO($dbUrl),
    dependsOn: ['dbUrl'],
    singleton: true
));

final class UsersController
{
    #[Get('/users')]
    public function list(Request $req, PDO $db): Response
    {
        $stmt = $db->query('SELECT * FROM users');
        return Response::json($stmt->fetchAll(PDO::FETCH_ASSOC));
    }
}

// Handler with dependency injection
$app = $app->registerController(UsersController::class);
```

**Design rationale:**
- Constructor property promotion (modern PHP)
- Typed parameters for type safety
- Closure-based factories
- PSR-compliant (PSR-7 for HTTP, PSR-11 for containers)

## Performance Characteristics

### Zero-Cost When Unused

```rust
// No DI container registered
let config = ServerConfig::builder().build();
// dependency_container = None, zero overhead

// With DI
let config = ServerConfig::builder()
    .provide_value("config", AppConfig::load())
    .build();
// dependency_container = Some(Arc<Container>), minimal overhead
```

### Batched Parallel Resolution

```rust
// Given dependency graph:
//   db -> config
//   cache -> config
//   auth -> db, cache
//
// Batch 1 (parallel): config
// Batch 2 (parallel): db, cache
// Batch 3 (sequential): auth
```

**Performance characteristics:**
- Independent dependencies resolve concurrently
- Singleton cache eliminates repeated resolution (Arc clone only)
- Per-request cache avoids duplicate work within request

### Benchmarking Plan

Compare:
1. Handler without DI (baseline)
2. Handler with DI but no dependencies (overhead check)
3. Handler with 1 dependency (simple case)
4. Handler with 5 nested dependencies (complex case)
5. Handler with singleton vs per-request caching

**Target:** <1% overhead for simple cases, <5% for complex cases

## Testing Strategy

### Unit Tests (Rust)

- `DependencyGraph::calculate_batches()` correctness
- Cycle detection with various graph shapes
- `ValueDependency` and `FactoryDependency` resolution
- Singleton caching behavior
- Per-request caching behavior
- Cleanup task execution

### Integration Tests (Python)

- Value injection in handlers
- Factory injection (sync and async)
- Nested dependency resolution
- Error handling (missing dependency)
- Error handling (circular dependency)
- Cleanup after request completes
- Type-based injection
- Name-based injection

### Fixture-Driven Tests

Create `testing_data/di/` fixtures:
- `basic_value/` - Simple value injection
- `factory/` - Factory dependency
- `singleton/` - Singleton caching
- `nested/` - Multi-level dependencies
- `circular_error/` - Cycle detection
- `cleanup/` - Generator cleanup

Each fixture includes:
- `schema.json` - Expected structure
- `input.json` - Request data
- `expected.json` - Expected response

### Coverage Targets

- Rust core: 95% minimum
- Python binding: 80% minimum
- Node binding: 80% minimum
- Ruby binding: 80% minimum
- PHP binding: 80% minimum

## Consequences

### Positive

✅ **Simple API** - Two methods to learn (`provide_value`, `provide_factory`)
✅ **Type-safe** - Rust's type system enforces correctness
✅ **Cross-language consistency** - Same semantics across all bindings
✅ **Zero-cost** - No overhead when not used
✅ **Testable** - Easy to mock dependencies in tests
✅ **Resource management** - First-class cleanup support
✅ **No external dependencies** - Built on Axum State (already in stack)
✅ **Incremental adoption** - Can add DI to existing apps gradually
✅ **Performance** - Batched parallel resolution, caching

### Negative

⚠️ **Custom implementation** - We maintain it (not a third-party crate)
⚠️ **Initial development time** - ~2 months for full cross-language support
⚠️ **Learning curve** - Users must learn DI patterns
⚠️ **Complexity** - Adds conceptual overhead to framework

### Neutral

🔷 **Not as feature-rich** as enterprise DI frameworks (NestJS, Spring)
🔷 **Simpler** than those frameworks (intentional trade-off)

## Migration Path

### Adding DI to Existing App

**Before:**
```rust
#[derive(Clone)]
struct AppState {
    db: Arc<DatabasePool>,
}

async fn handler(State(state): State<AppState>) -> String {
    let users = state.db.query("SELECT * FROM users").await?;
    format!("{:?}", users)
}

let app = Router::new()
    .route("/", get(handler))
    .with_state(AppState { db: Arc::new(pool) });
```

**After:**
```rust
let config = ServerConfig::builder()
    .provide_value("db", Arc::new(DatabasePool::new()))
    .build();

// Handler signature unchanged if using State pattern
// Or use DI for cleaner separation:
async fn handler(db: Arc<DatabasePool>) -> String {
    let users = db.query("SELECT * FROM users").await?;
    format!("{:?}", users)
}
```

### Backward Compatibility

- Existing `State<T>` pattern continues to work
- DI is opt-in via `ServerConfig::provide_*()`
- Handlers can mix State and DI extractors

## Open Questions

1. **Scoped dependencies** - Should we support request/singleton/transient scopes explicitly?
   - **Proposal:** Start with singleton/per-request, add more later if needed

2. **Type-based resolution** - Should dependencies be resolvable by type without a key?
   - **Proposal:** Require keys initially, add type-based as optional feature

3. **Automatic registration** - Should we auto-register common types (Request, State, etc.)?
   - **Proposal:** Manual registration only (explicit > implicit)

4. **Streaming/SSE/WebSocket** - How does DI work with long-lived connections?
   - **Proposal:** Resolve dependencies at connection start, cleanup at close

## Implementation Timeline

- **Week 1-2:** Rust core DI system + built-in types
- **Week 2-3:** Handler integration + router updates
- **Week 3-4:** ServerConfig integration + fixtures
- **Week 4-5:** Integration tests + Python binding
- **Week 5-6:** Python binding complete + examples
- **Week 6-7:** Node/TypeScript binding + examples
- **Week 7-8:** Ruby binding + examples
- **Week 8:** PHP binding + examples
- **Week 9:** WASM binding + examples
- **Week 10:** Documentation + polish
- **Week 11:** Review + merge

**Total:** ~11 weeks for full cross-language DI system

## References

- [Litestar Dependency Injection](https://docs.litestar.dev/latest/usage/dependency-injection.html)
- [Fastify Decorators](https://fastify.dev/docs/latest/Reference/Decorators/)
- [Axum State Extractors](https://docs.rs/axum/latest/axum/extract/struct.State.html)
- [Shaku Compile-Time DI](https://github.com/AzureMarker/shaku)
- ADR-0001: Architecture and Principles
- ADR-0002: HTTP Runtime and Middleware Pipeline
- ADR-0003: Validation and Fixture Source of Truth
- ADR-0005: Lifecycle Hooks

## Appendix: Comparison with Other Solutions

### vs Litestar DI

**Similarities:**
- Type-driven resolution
- Dependency graph with batched execution
- Generator pattern for cleanup
- `Provide` wrapper class

**Differences:**
- Spikard: Rust-first (cross-language)
- Litestar: Python-only (runtime reflection)
- Spikard: Explicit keys + optional type-based
- Litestar: Primarily type-based

### vs Fastify Decorators

**Similarities:**
- Simple API (`decorate` ≈ `provide_value`)
- Plugin-based encapsulation
- Property-based access

**Differences:**
- Spikard: Dependency graph, nested resolution
- Fastify: Flat decoration (no nesting)
- Spikard: Type-safe extractors
- Fastify: Property access (`fastify.db`)

### vs Shaku

**Similarities:**
- Compile-time DI
- Components (singletons) + Providers (factories)
- Module organization

**Differences:**
- Spikard: Runtime resolution with compile-time types
- Shaku: Compile-time resolution (macro expansion)
- Spikard: Cross-language bindings
- Shaku: Rust-only
- Spikard: Axum State-based
- Shaku: Module-based

---

**Decision:** Proceed with custom DI system on Axum State foundation

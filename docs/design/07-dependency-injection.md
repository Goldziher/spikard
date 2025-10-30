# Dependency Injection

**Date:** January 2025
**Status:** Active
**Related:** [05-middleware-lifecycle-optimization.md](./05-middleware-lifecycle-optimization.md), [03-api-design.md](./03-api-design.md)

## Executive Summary

Spikard implements a **Rust-powered DI engine** with language-specific APIs similar to Litestar. The DI graph computation, caching, and lifecycle management happen in Rust for maximum performance, while Python, TypeScript, and other bindings provide idiomatic APIs for their respective ecosystems.

**Key Decision:** DI engine in Rust + language-native APIs (vendoring Litestar for Python, implementing equivalents for TypeScript/others)

## 1. Overview

### Goals
- Provide Litestar-quality DI experience across all language bindings
- Pre-compute dependency graphs at startup in Rust
- O(1) dependency resolution at request time
- Support singleton, request, and transient scopes
- Enable dependency chains (dependencies that depend on other dependencies)
- Type-safe within each language

### Non-Goals
- Language-agnostic DI API (each binding has its own idioms)
- Runtime dependency graph changes (graph is computed at startup)
- Circular dependency support (detected and rejected at startup)

## 2. Architecture

### 2.1 Core Components

```
┌─────────────────────────────────────────┐
│  Python/TypeScript/Other Bindings       │
│  • Provide() / @Injectable() decorators│
│  • Type extraction from signatures      │
│  • Factory function registration        │
└──────────────┬──────────────────────────┘
               │ FFI (Register providers)
┌──────────────▼──────────────────────────┐
│  Rust DI Engine (crates/spikard/di)    │
│  • Dependency graph (DiGraph)           │
│  • Topological sort for resolution      │
│  • Scope management (singleton/request) │
│  • Circular dependency detection        │
└──────────────┬──────────────────────────┘
               │ At request time
┌──────────────▼──────────────────────────┐
│  Request Context                        │
│  • Request-scoped cache                 │
│  • Singleton cache (Arc<T>)             │
│  • Dependency resolution state          │
└─────────────────────────────────────────┘
```

### 2.2 Rust DI Engine

```rust
// crates/spikard/src/di/mod.rs
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;
use petgraph::graph::DiGraph;
use parking_lot::RwLock;

/// Dependency provider scope
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    /// Created once, shared across all requests
    Singleton,
    /// Created per request, cached for that request
    Request,
    /// Created every time it's needed (no caching)
    Transient,
}

/// A dependency provider
pub struct Provider {
    /// Unique identifier for this dependency type
    pub type_id: TypeId,
    /// Human-readable type name
    pub type_name: String,
    /// Scope of this dependency
    pub scope: Scope,
    /// Factory function (opaque - language-specific)
    pub factory: Arc<dyn Fn(&RequestContext) -> Result<Box<dyn Any>, Error> + Send + Sync>,
    /// Other dependencies this provider needs
    pub dependencies: Vec<TypeId>,
}

/// The dependency injection container
pub struct DIContainer {
    /// Dependency graph (nodes = providers, edges = dependencies)
    graph: DiGraph<Provider, ()>,
    /// Mapping from TypeId to graph node index
    type_to_node: HashMap<TypeId, petgraph::graph::NodeIndex>,
    /// Pre-computed resolution order (topological sort)
    resolution_order: HashMap<TypeId, Vec<petgraph::graph::NodeIndex>>,
    /// Singleton cache (thread-safe)
    singleton_cache: Arc<RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>>,
}

impl DIContainer {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            type_to_node: HashMap::new(),
            resolution_order: HashMap::new(),
            singleton_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a provider
    pub fn register(&mut self, provider: Provider) -> Result<(), Error> {
        let type_id = provider.type_id;
        let node_idx = self.graph.add_node(provider);
        self.type_to_node.insert(type_id, node_idx);
        Ok(())
    }

    /// Build the dependency graph (called after all providers registered)
    pub fn build(&mut self) -> Result<(), Error> {
        // Add edges based on dependencies
        for node_idx in self.graph.node_indices() {
            let provider = &self.graph[node_idx];
            for dep_type_id in &provider.dependencies {
                if let Some(&dep_node_idx) = self.type_to_node.get(dep_type_id) {
                    self.graph.add_edge(node_idx, dep_node_idx, ());
                } else {
                    return Err(Error::MissingDependency(
                        provider.type_name.clone(),
                        format!("{:?}", dep_type_id)
                    ));
                }
            }
        }

        // Detect circular dependencies
        if petgraph::algo::is_cyclic_directed(&self.graph) {
            return Err(Error::CircularDependency);
        }

        // Pre-compute resolution order (topological sort)
        let topo_order = petgraph::algo::toposort(&self.graph, None)
            .map_err(|_| Error::CircularDependency)?;

        for (&type_id, &node_idx) in &self.type_to_node {
            // For each type, compute the order in which its dependencies must be resolved
            let mut order = Vec::new();
            for &idx in &topo_order {
                if idx == node_idx || petgraph::algo::has_path_connecting(
                    &self.graph,
                    node_idx,
                    idx,
                    None
                ) {
                    order.push(idx);
                }
            }
            self.resolution_order.insert(type_id, order);
        }

        Ok(())
    }

    /// Resolve a dependency for a request
    pub fn resolve(
        &self,
        type_id: TypeId,
        ctx: &mut RequestContext,
    ) -> Result<Arc<dyn Any + Send + Sync>, Error> {
        let node_idx = self.type_to_node.get(&type_id)
            .ok_or(Error::TypeNotRegistered(format!("{:?}", type_id)))?;

        let provider = &self.graph[*node_idx];

        match provider.scope {
            Scope::Singleton => {
                // Check if already cached
                {
                    let cache = self.singleton_cache.read();
                    if let Some(instance) = cache.get(&type_id) {
                        return Ok(instance.clone());
                    }
                }

                // Not cached, create instance
                let instance = (provider.factory)(ctx)?;
                let arc_instance = Arc::new(instance);

                // Cache it
                {
                    let mut cache = self.singleton_cache.write();
                    cache.insert(type_id, arc_instance.clone());
                }

                Ok(arc_instance)
            }
            Scope::Request => {
                // Check request-scoped cache
                if let Some(instance) = ctx.di_cache.get(&type_id) {
                    return Ok(instance.clone());
                }

                // Create instance
                let instance = (provider.factory)(ctx)?;
                let arc_instance = Arc::new(instance);

                // Cache in request context
                ctx.di_cache.insert(type_id, arc_instance.clone());

                Ok(arc_instance)
            }
            Scope::Transient => {
                // Always create new instance
                let instance = (provider.factory)(ctx)?;
                Ok(Arc::new(instance))
            }
        }
    }
}

/// Request context for dependency resolution
pub struct RequestContext {
    /// Request-scoped dependency cache
    pub di_cache: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
    /// Request data (headers, query params, etc.)
    pub request_data: Arc<RequestData>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Circular dependency detected")]
    CircularDependency,
    #[error("Missing dependency: {0} requires {1}")]
    MissingDependency(String, String),
    #[error("Type not registered: {0}")]
    TypeNotRegistered(String),
    #[error("Factory error: {0}")]
    FactoryError(String),
}
```

## 3. Python API (Litestar-Inspired)

### 3.1 Basic Usage

```python
from spikard import get, Provide

# Simple factory function
async def get_db() -> Database:
    return Database(connection_string="postgresql://...")

# Handler with DI
@get("/users/{user_id}")
async def get_user(
    user_id: int,
    db: Database = Provide(get_db),  # Injected dependency
) -> dict:
    return await db.get_user(user_id)
```

### 3.2 Dependency Chains

```python
# Database dependency
async def get_db() -> Database:
    return Database("postgresql://...")

# Service depends on Database
async def get_user_service(db: Database = Provide(get_db)) -> UserService:
    return UserService(db)

# Handler uses service (automatic chain resolution)
@get("/users/{user_id}")
async def get_user(
    user_id: int,
    service: UserService = Provide(get_user_service),
) -> User:
    return await service.get(user_id)
```

### 3.3 Scopes

```python
from spikard import Provide, Scope

# Singleton - created once, shared across all requests
def get_config() -> Config:
    return Config.load_from_env()

# Request - created per request, cached for that request
async def get_db_session() -> Session:
    return await create_session()

# Transient - created every time (no caching)
def get_logger() -> Logger:
    return Logger()

@get("/data")
async def get_data(
    config: Config = Provide(get_config, scope=Scope.SINGLETON),
    session: Session = Provide(get_db_session, scope=Scope.REQUEST),
    logger: Logger = Provide(get_logger, scope=Scope.TRANSIENT),
) -> dict:
    logger.info("Fetching data")
    return await session.query(config.data_table).all()
```

### 3.4 Implementation (Python Binding)

```python
# packages/python/spikard/di.py
import inspect
from typing import Callable, Any, TypeVar, get_type_hints
from enum import Enum

T = TypeVar('T')

class Scope(Enum):
    SINGLETON = "singleton"
    REQUEST = "request"
    TRANSIENT = "transient"

class Provide:
    """Marks a parameter as a dependency to be injected."""

    def __init__(self, factory: Callable[..., T], scope: Scope = Scope.REQUEST):
        self.factory = factory
        self.scope = scope
        self.dependencies = self._extract_dependencies(factory)

    def _extract_dependencies(self, factory: Callable) -> list[type]:
        """Extract dependency types from factory signature."""
        sig = inspect.signature(factory)
        deps = []
        for param_name, param in sig.parameters.items():
            if isinstance(param.default, Provide):
                # This parameter is itself a dependency
                annotation = param.annotation
                if annotation != inspect.Parameter.empty:
                    deps.append(annotation)
        return deps

def extract_di_metadata(handler: Callable) -> list['DIParameter']:
    """Extract DI metadata from handler signature at route registration."""
    sig = inspect.signature(handler)
    type_hints = get_type_hints(handler)

    di_params = []
    for param_name, param in sig.parameters.items():
        if isinstance(param.default, Provide):
            di_params.append(DIParameter(
                name=param_name,
                type_annotation=type_hints.get(param_name),
                factory=param.default.factory,
                scope=param.default.scope,
                dependencies=param.default.dependencies,
            ))

    return di_params

class DIParameter:
    def __init__(
        self,
        name: str,
        type_annotation: type,
        factory: Callable,
        scope: Scope,
        dependencies: list[type],
    ):
        self.name = name
        self.type_annotation = type_annotation
        self.factory = factory
        self.scope = scope
        self.dependencies = dependencies

# Register with Rust DI engine
def register_di_providers(di_params: list[DIParameter], rust_container):
    """Register Python providers with Rust DI container."""
    for param in di_params:
        # Convert Python factory to Rust-compatible callback
        def rust_factory(ctx):
            # Call Python factory
            return param.factory()

        rust_container.register_provider(
            type_id=id(param.type_annotation),
            type_name=param.type_annotation.__name__,
            scope=param.scope.value,
            factory=rust_factory,
            dependencies=[id(dep) for dep in param.dependencies],
        )
```

## 4. TypeScript API

### 4.1 Basic Usage (Functional)

```typescript
import { Spikard, provide, Scope } from 'spikard';

// Factory function
const getDb = () => new Database('postgresql://...');

const app = new Spikard();

// Register providers
app.provide(Database, { useFactory: getDb, scope: Scope.Singleton });

// Handler with DI
app.get('/users/:userId', {
  params: z.object({ userId: z.number() }),
  handler: async ({ params, inject }) => {
    const db = inject(Database);
    return db.getUser(params.userId);
  }
});
```

### 4.2 Class-Based (Optional)

```typescript
import { Injectable, Inject } from 'spikard';

@Injectable()
class Database {
  constructor(private connectionString: string) {}
  async getUser(userId: number) { ... }
}

@Injectable()
class UserService {
  constructor(@Inject() private db: Database) {}
  async getUser(userId: number) {
    return this.db.getUser(userId);
  }
}

const app = new Spikard({
  providers: [
    { provide: Database, useFactory: () => new Database('postgresql://...') },
    { provide: UserService, useClass: UserService },
  ]
});

app.get('/users/:userId', {
  handler: async ({ params, inject }) => {
    const service = inject(UserService);
    return service.getUser(params.userId);
  }
});
```

## 5. Performance Characteristics

### 5.1 Startup Cost

| Operation | Time | Notes |
|-----------|------|-------|
| Graph building | 10-50ms | Depends on number of providers |
| Topological sort | <1ms | One-time cost |
| Circular detection | <1ms | One-time cost |
| **Total startup** | **<100ms** | For ~100 providers |

### 5.2 Runtime Cost

| Operation | Time | Notes |
|-----------|------|-------|
| Singleton resolution (cached) | ~5ns | HashMap lookup |
| Request resolution (cached) | ~5ns | HashMap lookup |
| Request resolution (uncached) | 20-40ns | Factory call + cache insert |
| Transient resolution | 20-40ns | Factory call only |

### 5.3 Memory Usage

| Scope | Memory | Notes |
|-------|--------|-------|
| Singleton | 1x instance | Shared across all requests |
| Request | N instances | One per concurrent request |
| Transient | Temporary | Dropped after use |

## 6. Implementation Strategy

### Phase 1: Rust DI Engine (Week 1-2)
- [ ] Implement `DIContainer` with `DiGraph` in `crates/spikard/src/di/mod.rs`
- [ ] Provider registration API
- [ ] Dependency graph building with topological sort
- [ ] Circular dependency detection
- [ ] Scope management (singleton/request/transient)
- [ ] Resolution algorithm
- [ ] Unit tests for graph building and resolution

### Phase 2: Python Binding (Week 3-4)
- [ ] Vendor Litestar's DI modules (`litestar.di`, `litestar._signature`)
- [ ] Implement `Provide()` marker
- [ ] Extract DI metadata from handler signatures
- [ ] Register Python factories with Rust engine via PyO3
- [ ] Implement factory callbacks (Rust → Python)
- [ ] Request context integration
- [ ] Integration tests with fixtures

### Phase 3: TypeScript Binding (Week 5-6)
- [ ] Design TypeScript DI API (functional + class-based)
- [ ] Implement provider registration
- [ ] Extract DI metadata from decorators/signatures
- [ ] Register TypeScript factories with Rust engine via napi-rs
- [ ] Implement factory callbacks (Rust → TypeScript)
- [ ] Integration tests

### Phase 4: Optimization (Week 7)
- [ ] Benchmark startup cost
- [ ] Benchmark resolution performance
- [ ] Optimize factory callbacks (minimize FFI overhead)
- [ ] Profile memory usage
- [ ] Document best practices

## 7. Testing Strategy

### 7.1 Rust Engine Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_resolution() {
        let mut container = DIContainer::new();

        // Register a simple provider
        container.register(Provider {
            type_id: TypeId::of::<String>(),
            type_name: "String".into(),
            scope: Scope::Singleton,
            factory: Arc::new(|_ctx| Ok(Box::new("Hello".to_string()))),
            dependencies: vec![],
        }).unwrap();

        container.build().unwrap();

        // Resolve
        let mut ctx = RequestContext::default();
        let result = container.resolve(TypeId::of::<String>(), &mut ctx).unwrap();

        let s = result.downcast_ref::<String>().unwrap();
        assert_eq!(s, "Hello");
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut container = DIContainer::new();

        let type_a = TypeId::of::<i32>();
        let type_b = TypeId::of::<f64>();

        // A depends on B
        container.register(Provider {
            type_id: type_a,
            type_name: "A".into(),
            scope: Scope::Singleton,
            factory: Arc::new(|_| Ok(Box::new(42i32))),
            dependencies: vec![type_b],
        }).unwrap();

        // B depends on A (circular!)
        container.register(Provider {
            type_id: type_b,
            type_name: "B".into(),
            scope: Scope::Singleton,
            factory: Arc::new(|_| Ok(Box::new(3.14f64))),
            dependencies: vec![type_a],
        }).unwrap();

        // Should detect cycle
        let result = container.build();
        assert!(matches!(result, Err(Error::CircularDependency)));
    }
}
```

### 7.2 Integration Tests

```python
# packages/python/tests/test_di.py
import pytest
from spikard import get, Provide, Scope

class Database:
    def __init__(self, connection_string: str):
        self.connection_string = connection_string

    async def get_user(self, user_id: int):
        return {"id": user_id, "name": "Alice"}

class UserService:
    def __init__(self, db: Database):
        self.db = db

    async def get(self, user_id: int):
        return await self.db.get_user(user_id)

async def get_db() -> Database:
    return Database("postgresql://...")

async def get_service(db: Database = Provide(get_db)) -> UserService:
    return UserService(db)

@get("/users/{user_id}")
async def get_user_handler(
    user_id: int,
    service: UserService = Provide(get_service),
) -> dict:
    return await service.get(user_id)

def test_dependency_injection(test_client):
    response = test_client.get("/users/123")
    assert response.status_code == 200
    assert response.json() == {"id": 123, "name": "Alice"}

def test_singleton_scope():
    """Verify singleton instances are shared."""
    call_count = 0

    def get_config() -> Config:
        nonlocal call_count
        call_count += 1
        return Config()

    @get("/a")
    def handler_a(config: Config = Provide(get_config, scope=Scope.SINGLETON)):
        return {"count": call_count}

    @get("/b")
    def handler_b(config: Config = Provide(get_config, scope=Scope.SINGLETON)):
        return {"count": call_count}

    # Both handlers should get same instance
    test_client.get("/a")
    response_b = test_client.get("/b")

    # Factory should only be called once
    assert response_b.json() == {"count": 1}
```

## 8. Migration from Other Frameworks

### 8.1 From FastAPI

```python
# FastAPI
from fastapi import FastAPI, Depends

def get_db():
    return Database()

@app.get("/users/{user_id}")
def get_user(user_id: int, db: Database = Depends(get_db)):
    return db.get_user(user_id)

# Spikard (nearly identical)
from spikard import get, Provide

def get_db():
    return Database()

@get("/users/{user_id}")
def get_user(user_id: int, db: Database = Provide(get_db)):
    return db.get_user(user_id)
```

**Migration effort:** Minimal - replace `Depends` with `Provide`

### 8.2 From Litestar

```python
# Litestar
from litestar import get, Provide

# Spikard (identical API)
from spikard import get, Provide
```

**Migration effort:** Zero - API is identical

## 9. Open Questions

- [x] Should DI engine be in Rust or bindings? **Decision: Rust engine**
- [x] Should we vendor Litestar or reimplement? **Decision: Vendor for Python**
- [ ] How to handle async factories in Rust? (PyO3 async, napi-rs async)
- [ ] Should we support context-aware factories (factories that receive Request)?
- [ ] How to expose DI container for testing (mock providers)?

## 10. References

### Codebases Analyzed
- [Robyn](https://github.com/sparckles/robyn) - Simple dict-based DI
- [Litestar](https://github.com/litestar-org/litestar) - Mature Python DI with dependency graphs
- [NestJS](https://github.com/nestjs/nest) - TypeScript DI with decorators
- [TSyringe](https://github.com/microsoft/tsyringe) - Lightweight TypeScript DI

### Crates
- [petgraph](https://github.com/petgraph/petgraph) - Graph data structures for dependency graph
- [parking_lot](https://github.com/Amanieu/parking_lot) - Fast RwLock for singleton cache

### Related Documents
- [03-api-design.md](./03-api-design.md) - API design patterns
- [05-middleware-lifecycle-optimization.md](./05-middleware-lifecycle-optimization.md) - Performance considerations

---

**Key Takeaway:** Spikard implements a Rust-powered DI engine for maximum performance (O(1) resolution, pre-computed graphs) while providing Litestar-quality APIs in Python and equivalent patterns in TypeScript. The engine handles graph computation, caching, and lifecycle management, while bindings provide language-native factory registration and type extraction.

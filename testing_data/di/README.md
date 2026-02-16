# Dependency Injection (DI) Fixtures

This directory contains comprehensive E2E test fixtures for Spikard's Dependency Injection system. These fixtures follow a **Test-Driven Development (TDD)** approach — they define the expected behavior BEFORE implementation.

## Overview

The DI system enables handlers to declare dependencies that are automatically resolved and injected at request time. It supports:

- **Value Dependencies** - Static values (config, constants)
- **Factory Dependencies** - Dynamic creation (sync)
- **Async Factory Dependencies** - Async creation (database pools, HTTP clients)
- **Nested Dependencies** - Dependencies that depend on other dependencies
- **Caching Strategies** - Singleton (global) and per-request caching
- **Resource Cleanup** - Generator pattern for cleanup (Python yield, etc.)
- **Cross-Language Support** - Python, Node.js, TypeScript, Ruby, Elixir

## Fixture Categories

### 1. Core DI Scenarios (01-06)

Basic dependency injection patterns that form the foundation of the DI system.

#### 01_value_dependency_success.json
**Purpose:** Tests simple value injection (config, constants)

**Dependencies:**
- `app_name`: String value "SpikardApp"
- `version`: String value "1.0.0"
- `max_connections`: Integer value 100

**Expected:** Handler receives all three values and returns them in response.

**Test Pattern:** Basic value dependency resolution

---

#### 02_factory_dependency_success.json
**Purpose:** Tests factory dependency that creates instances on-demand

**Dependencies:**
- `timestamp_generator`: Factory that creates timestamps

**Expected:** Handler receives timestamp from factory. Each request gets a new timestamp (not cached).

**Test Pattern:** Factory dependency with no caching

---

#### 03_async_factory_success.json
**Purpose:** Tests async factory that creates resources asynchronously

**Dependencies:**
- `db_pool`: Async factory that creates database pool

**Expected:** Handler receives connected database pool. Pool is cached within request (cacheable=true).

**Test Pattern:** Async factory with per-request caching

---

#### 04_nested_dependencies_success.json
**Purpose:** Tests 3-level dependency resolution with parallel batching

**Dependencies:**
- `config`: Value (no dependencies)
- `db_pool`: Async factory (depends on `config`)
- `cache`: Async factory (depends on `config`)
- `auth_service`: Factory (depends on `db_pool` and `cache`)

**Resolution Order:**
1. Batch 1: `config` (no dependencies, resolved first)
2. Batch 2: `db_pool`, `cache` (both depend on config, **resolved in parallel**)
3. Batch 3: `auth_service` (depends on batch 2, resolved last)

**Expected:** Handler receives `auth_service`. Response includes `dependency_resolution_order` showing batched execution.

**Test Pattern:** Nested dependencies with topological sorting and parallel batching

---

#### 05_singleton_caching_success.json
**Purpose:** Tests singleton dependency shared across all requests

**Dependencies:**
- `app_counter`: Factory with `singleton=true`

**Expected:** First request creates counter with UUID. **Subsequent requests receive SAME instance** (same UUID, incremented count).

**Test Pattern:** Singleton caching (global shared state)

**Multi-Request Test:** Make 3 requests, verify:
- All have same `counter_id` (UUID)
- `count` increments: 1, 2, 3

---

#### 06_per_request_caching_success.json
**Purpose:** Tests per-request caching where dependency is reused within a single request

**Dependencies:**
- `request_id_generator`: Factory with `cacheable=true, singleton=false`

**Handler Uses:** Dependency twice in same request

**Expected:** Both usages receive **same instance** (same UUID) within the request. Different requests get different instances.

**Test Pattern:** Per-request caching (within request scope only)

---

### 2. Error Scenarios (07-09)

Error handling and validation for dependency injection system.

#### 07_circular_dependency_error.json
**Purpose:** Tests circular dependency detection (A → B, B → A)

**Dependencies:**
- `service_a`: Factory depends on `service_b`
- `service_b`: Factory depends on `service_a`

**Expected:** 500 error with structured error response:
```json
{
  "type": "https://spikard.dev/errors/dependency-error",
  "title": "Dependency Resolution Failed",
  "status": 500,
  "detail": "Circular dependency detected",
  "errors": [{
    "type": "circular_dependency",
    "msg": "Circular dependency detected in dependency graph",
    "cycle": ["service_a", "service_b", "service_a"]
  }]
}
```

**Test Pattern:** Cycle detection at registration time (DFS algorithm)

---

#### 08_missing_dependency_error.json
**Purpose:** Tests error when handler requires unregistered dependency

**Dependencies:** (empty)

**Handler Requires:** `non_existent_service`

**Expected:** 500 error with:
```json
{
  "type": "https://spikard.dev/errors/dependency-error",
  "errors": [{
    "type": "missing_dependency",
    "msg": "Dependency 'non_existent_service' is not registered",
    "dependency_key": "non_existent_service"
  }]
}
```

**Test Pattern:** Missing dependency validation

---

#### 09_type_mismatch_error.json
**Purpose:** Tests error when handler expects different type than provided

**Dependencies:**
- `config`: String value "string_config"

**Handler Expects:** Object type for `config`

**Expected:** 500 error with:
```json
{
  "errors": [{
    "type": "type_mismatch",
    "msg": "Dependency 'config' type mismatch: expected object, got string",
    "expected_type": "object",
    "actual_type": "string"
  }]
}
```

**Test Pattern:** Runtime type safety validation

---

### 3. Resource Management (10-11)

Cleanup and resource lifecycle management.

#### 10_cleanup_after_request.json
**Purpose:** Tests generator pattern cleanup (Python yield, etc.)

**Dependencies:**
- `db_session`: Async factory with `cleanup=true`

**Background State Tracking:**
- State endpoint: `/api/cleanup-state`
- Expected events: `["session_opened", "session_closed"]`

**Flow:**
1. Request arrives
2. Factory creates session → records "session_opened"
3. Handler executes
4. Response sent
5. Cleanup runs → records "session_closed"

**Expected:** Response succeeds, background state shows both events in order.

**Test Pattern:** Generator cleanup with async/await

---

#### 11_multiple_dependencies_cleanup.json
**Purpose:** Tests cleanup order for multiple dependencies with cleanup

**Dependencies:**
- `db_connection`: Async factory with cleanup
- `cache_connection`: Async factory with cleanup
- `session`: Factory with cleanup (depends on db and cache)

**Background State Tracking:**
- Expected events: `["db_opened", "cache_opened", "session_opened", "session_closed", "cache_closed", "db_closed"]`

**Cleanup Order:** **Reverse of resolution order** (like destructors)
1. Resolution: config → db + cache → session
2. Cleanup: session → cache → db

**Test Pattern:** Multiple cleanup handlers in reverse dependency order

---

### 4. Cross-Language Patterns (12-15)

Language-specific dependency injection patterns.

#### 12_python_type_injection.json
**Purpose:** Tests Python type annotation-based injection

**Dependencies:**
- `database_pool`: Object with `python_type="DatabasePool"`
- `cache_client`: Object with `python_type="CacheClient"`

**Python Handler:**
```python
async def handler(db: DatabasePool, cache: CacheClient):
    # Dependencies injected by matching type hints
```

**Injection Strategy:** `type` (match by type annotation)

**Test Pattern:** Python type-based dependency resolution

---

#### 13_python_name_injection.json
**Purpose:** Tests Python parameter name-based injection

**Dependencies:**
- `db_pool`: Object
- `cache`: Object

**Python Handler:**
```python
def handler(db_pool, cache):
    # Dependencies injected by matching parameter names
```

**Injection Strategy:** `name` (match by parameter name)

**Test Pattern:** Python name-based dependency resolution (no type hints required)

---

#### 14_node_destructuring.json
**Purpose:** Tests Node.js/TypeScript object destructuring injection

**Dependencies:**
- `db`: Database object
- `logger`: Logger object

**TypeScript Handler:**
```typescript
async (request, { db, logger }: { db: Database, logger: Logger }) => {
  // Dependencies injected via object destructuring
}
```

**Injection Strategy:** `destructure`

**Test Pattern:** TypeScript destructuring pattern

---

#### 15_ruby_keyword_args.json
**Purpose:** Tests Ruby keyword argument injection

**Dependencies:**
- `db_pool`: Database pool object
- `session`: Session object

**Ruby Handler:**
```ruby
def handler(request, db_pool:, session:)
  # Dependencies injected as keyword arguments
end
```

**Injection Strategy:** `keyword_args`

**Test Pattern:** Ruby keyword argument pattern

---

### 5. Advanced Scenarios (16-18)

Complex DI patterns and integrations.

#### 16_dependency_override.json
**Purpose:** Tests route-level dependency override for testing/mocking

**Dependencies:**
- `api_key_validator`: App-level with `mode="production"`

**Route Overrides:**
- `api_key_validator`: Route-level with `mode="test"`

**Expected:** Handler receives route-level override (mode="test"), not app-level.

**Use Case:** Override production services with test mocks for specific routes.

**Test Pattern:** Route-level dependency override

---

#### 17_mixed_singleton_and_request.json
**Purpose:** Tests mixing singleton and per-request caching strategies

**Dependencies:**
- `app_config`: Value, `singleton=true` (shared across all requests)
- `db_pool`: Factory, `singleton=true` (shared globally)
- `request_context`: Factory, `singleton=false, cacheable=true` (per-request)

**Multi-Request Test:**
- Request 1: Creates all dependencies, records UUIDs
- Request 2: Same `app_config` and `db_pool` UUIDs, **different** `request_context` UUID
- Request 3: Same pattern

**Test Pattern:** Mixed caching strategies in same handler

---

#### 18_dependency_in_lifecycle_hook.json
**Purpose:** Tests accessing dependencies in lifecycle hooks (onRequest, preHandler)

**Dependencies:**
- `auth_service`: Authentication service
- `logger`: Logger service

**Lifecycle Hooks:**
- `onRequest`: Uses `logger` dependency
- `preHandler`: Uses `auth_service` dependency

**Flow:**
1. Request arrives
2. Resolve dependencies
3. `onRequest` hook executes (with logger)
4. `preHandler` hook executes (with auth_service)
5. Handler executes
6. Response

**Expected:** Hooks can access dependencies. Response headers show hook execution (`X-Log-Level`, `X-Auth-Mode`).

**Test Pattern:** DI integration with lifecycle hooks

---

## Fixture Schema Extensions

The master schema at `testing_data/00-FIXTURE-SCHEMA.json` has been extended with DI fields:

### handler.dependencies

Map of dependency key → dependency definition:

```json
{
  "dependency_key": {
    "type": "value" | "factory" | "async_factory",
    "value": <any>,                    // For type="value"
    "value_type": "string" | "number" | "object" | ...,
    "factory": "function_name",        // For type="factory"
    "depends_on": ["dep1", "dep2"],    // Dependencies this depends on
    "singleton": true | false,         // Global caching
    "cacheable": true | false,         // Per-request caching
    "cleanup": true | false,           // Has cleanup handler
    "scope": "app" | "route",          // Dependency scope
    "python_type": "ClassName"         // For Python type-based injection
  }
}
```

### handler.handler_dependencies

Array of dependency keys required by the handler:

```json
["config", "db_pool", "auth_service"]
```

### handler.route_overrides

Route-level dependency overrides:

```json
{
  "dependency_key": {
    // Same structure as dependencies
    // Overrides app-level dependency with same key
  }
}
```

### handler.injection_strategy

Language-specific injection pattern:
- `"name"` - Python parameter name matching
- `"type"` - Python type annotation matching
- `"destructure"` - Node.js/TypeScript object destructuring
- `"keyword_args"` - Ruby keyword arguments

### expected_response.dependency_resolution_order

Expected batched resolution order:

```json
[
  ["config"],                    // Batch 1 (parallel)
  ["db_pool", "cache"],         // Batch 2 (parallel)
  ["auth_service"]              // Batch 3 (sequential after batch 2)
]
```

## Testing Strategy

### Fixture-Driven Tests

All fixtures are loaded by `packages/python/tests/test_all_fixtures.py`:

```python
@pytest.mark.parametrize("fixture_file", glob("testing_data/di/*.json"))
def test_di_fixture(fixture_file, fixture_app):
    """Test DI fixture with auto-generated handler"""
    fixture = load_fixture(fixture_file)

    # Create app with dependencies from fixture
    app = create_app_with_dependencies(fixture)

    # Make request
    response = app.test_client().request(
        method=fixture["request"]["method"],
        path=fixture["request"]["path"],
        # ...
    )

    # Assert response
    assert response.status_code == fixture["expected_response"]["status_code"]
    assert response.json() == fixture["expected_response"]["body"]

    # If background cleanup, poll state endpoint
    if "background" in fixture:
        state = poll_state(fixture["background"]["state_path"])
        assert state == fixture["background"]["expected_state"]
```

### Multi-Request Tests

Some fixtures require multiple requests to verify behavior:

- **05_singleton_caching_success.json** - Make 3+ requests, verify same instance
- **17_mixed_singleton_and_request.json** - Make 3+ requests, verify caching strategies

### Background State Validation

Fixtures with cleanup use background state endpoints:

```python
# Handler records events
events = []

# Fixture background config
"background": {
  "state_path": "/api/cleanup-state",
  "state_key": "cleanup_events",
  "expected_state": ["session_opened", "session_closed"]
}

# Test polls state endpoint
state = client.get("/api/cleanup-state").json()
assert state["cleanup_events"] == ["session_opened", "session_closed"]
```

## Implementation Checklist

When implementing DI, fixtures should drive development:

- [ ] **01-06**: Core DI works (value, factory, async, nested, caching)
- [ ] **07-09**: Error handling (circular, missing, type mismatch)
- [ ] **10-11**: Cleanup/resource management
- [ ] **12-15**: Cross-language injection patterns
- [ ] **16-18**: Advanced features (overrides, mixed caching, hooks)

### TDD Workflow

1. **Red**: Run tests → all fixtures fail (not implemented)
2. **Green**: Implement DI system → fixtures pass one by one
3. **Refactor**: Optimize, add observability, polish

## Fixture Naming Convention

Pattern: `{ordinal}_{descriptive_name}.json`

- `01-06`: Core scenarios
- `07-09`: Error scenarios
- `10-11`: Resource management
- `12-15`: Language-specific patterns
- `16-18`: Advanced scenarios

## Cross-Language Compatibility

All fixtures are designed to work across:

- **Python** - Type hints, generator cleanup, async/await
- **Node.js/TypeScript** - Object destructuring, TypeScript types, async/await
- **Ruby** - Keyword arguments, blocks, async
- **Elixir** - Rustler NIF integration, BEAM process model

The test generator will create language-specific implementations that satisfy the same fixtures.

## Related Documentation

- **Implementation Plan**: `/IMPLEMENTATION_PLAN.md` - Full DI roadmap
- **ADR-0008**: `/docs/adr/0008-dependency-injection.md` - Design decisions
- **Master Schema**: `/testing_data/00-FIXTURE-SCHEMA.json` - Fixture schema
- **Python Tests**: `/packages/python/tests/test_di.py` - Integration tests

## Contributing

When adding new DI features:

1. Create fixture(s) first (TDD approach)
2. Update this README with fixture documentation
3. Run `task test` to verify fixtures fail (red)
4. Implement feature
5. Run `task test` to verify fixtures pass (green)
6. Document in ADR-0008 and IMPLEMENTATION_PLAN.md

## Questions & Support

For questions about DI fixtures:
- See `IMPLEMENTATION_PLAN.md` for architecture details
- See `docs/adr/0008-dependency-injection.md` for design rationale
- Check existing fixtures for patterns
- Open GitHub issue with `fixture:di` label

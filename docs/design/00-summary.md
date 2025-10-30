# Spikard Design Summary

## What is Spikard?

Spikard is a **Rust-first, multi-language web framework** designed for maximum performance and developer experience. All critical operations (routing, parameter extraction, validation) happen in Rust, while bindings provide idiomatic APIs for Python, TypeScript, and other languages.

## Core Architecture

```
┌─────────────────────────────────────┐
│   Application Layer                 │
│   (Python, TypeScript, Rust, ...)  │
│   • Route definitions               │
│   • Dependency injection            │
│   • Application middleware          │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│   Language Bindings                 │
│   (PyO3, napi-rs, wasm-bindgen)    │
│   • Schema conversion               │
│   • Handler invocation              │
│   • Response serialization          │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│   Spikard Rust Core                 │
│   • Routing (Axum/matchit)         │
│   • Parameter extraction (all types)│
│   • JSON Schema validation          │
│   • Performance middleware          │
└─────────────────────────────────────┘
```

## Design Documents

1. **[01-validation-strategy.md](./01-validation-strategy.md)**: Parameter and body validation approach
2. **[02-testing-strategy.md](./02-testing-strategy.md)**: Fixture-driven testing methodology
3. **[03-api-design.md](./03-api-design.md)**: Language-specific API designs
4. **[04-dependency-injection-middleware.md](./04-dependency-injection-middleware.md)**: DI and middleware patterns
5. **[05-middleware-lifecycle-optimization.md](./05-middleware-lifecycle-optimization.md)**: Middleware, lifecycle hooks, and performance optimization
6. **[06-observability-openapi.md](./06-observability-openapi.md)**: OpenTelemetry instrumentation and OpenAPI generation
7. **[axum-routing.md](./axum-routing.md)**: Routing implementation details

## Key Design Decisions

### 1. Route Decorators (Litestar-style)

**Python:**
```python
from spikard import get, post, put, delete, Spikard

@get("/users/{user_id:int}")
async def get_user(user_id: int) -> User:
    return User(...)

app = Spikard(route_handlers=[get_user])
```

**TypeScript:**
```typescript
import { Spikard } from 'spikard';

const app = new Spikard();

app.get('/users/:userId', {
  params: { userId: z.number() },
  handler: async ({ params }) => { ... }
});
```

### 2. Universal Schema Support

**Python supports:**
- dataclasses, TypedDict, NamedTuple (stdlib)
- msgspec.Struct
- attrs
- Pydantic v1/v2
- Raw JSON Schema dicts

**TypeScript supports:**
- Zod (primary)
- TypeBox
- io-ts
- Yup
- Raw JSON Schema objects

**All schemas convert to JSON Schema → validated in Rust**

### 3. Parameter Sources

Explicit via `Parameter()` function:

```python
from spikard import Parameter

@get("/search")
async def search(
    q: str = Parameter(query="q", min_length=3),
    api_key: str = Parameter(header="X-API-Key"),
    session: str = Parameter(cookie="session_id"),
) -> list[Result]:
    ...
```

### 4. Dependency Injection

**Handled in bindings, not Rust:**

```python
from spikard import get, Provide

async def get_db() -> Database:
    return Database(...)

@get("/users/{user_id}")
async def get_user(
    user_id: int,
    db: Database = Provide(get_db),
) -> User:
    return await db.get_user(user_id)
```

### 5. Middleware Stack

```
Request → Rust Middleware (CORS, compression, rate limit)
       → Language Middleware (auth, logging, custom)
       → Routing (Rust)
       → Validation (Rust)
       → DI Resolution (Python/TS)
       → Handler
       → Response
```

## Performance Characteristics

| Operation | Handled By | Why |
|-----------|-----------|-----|
| Routing | Rust (Axum) | O(log n) trie lookup |
| Parameter extraction | Rust | Zero-copy, no GIL |
| Validation | Rust (jsonschema) | No runtime overhead |
| Schema conversion | Bindings | One-time at startup |
| DI resolution | Bindings | Flexibility, language-native |
| Application middleware | Bindings | Flexibility, custom logic |

## Error Format

**Consistent across all languages:**

```json
{
  "detail": [
    {
      "type": "string_too_short",
      "loc": ["body", "name"],
      "msg": "String should have at least 3 characters",
      "input": "ab",
      "ctx": {
        "min_length": 3
      }
    }
  ]
}
```

## Implementation Phases

### Phase 1: Core Foundation ✅
- [x] Axum routing with path parameters
- [x] Query parameter extraction and validation
- [x] Path parameter extraction and validation
- [x] Header and cookie extraction
- [x] JSON Schema validation in Rust
- [x] Error format standardization

### Phase 2: Python API (In Progress)
- [ ] Route decorator implementation (`@get`, `@post`, etc.)
- [ ] Schema extraction from dataclasses, Pydantic, msgspec
- [ ] Parameter() function for explicit sources
- [ ] Request/Response objects
- [ ] Dependency injection (Provide mechanism)

### Phase 3: Request Bodies
- [ ] JSON body parsing and validation
- [ ] Multipart/form-data support
- [ ] URL-encoded form support
- [ ] File uploads

### Phase 4: TypeScript Bindings
- [ ] napi-rs binding layer
- [ ] Zod schema conversion
- [ ] TypeScript route API
- [ ] Dependency injection

### Phase 5: Advanced Features
- [ ] WebSocket support
- [ ] Server-Sent Events (SSE)
- [ ] Streaming responses
- [ ] GraphQL integration (maybe)

### Phase 6: Production Ready
- [ ] OpenAPI/Swagger generation
- [ ] Comprehensive documentation
- [ ] Performance benchmarks
- [ ] Migration guides (FastAPI, NestJS, Express)

## Success Criteria

1. **Performance**: 10x faster than FastAPI for parameter-heavy routes
2. **Type Safety**: Full type inference in Python and TypeScript
3. **DX**: Better or equal to FastAPI/NestJS developer experience
4. **Compatibility**: Support all major schema libraries
5. **Stability**: Comprehensive test coverage (>90%)

## Next Steps

1. Implement Python route decorators (`@get`, `@post`, etc.)
2. Schema extraction from Python types → JSON Schema
3. Create canonical test fixtures based on our API
4. Implement request body handling
5. Build out dependency injection system

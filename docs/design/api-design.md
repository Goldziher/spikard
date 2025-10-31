# Spikard API Design

## Philosophy

Spikard is a **Rust-first, multi-language web framework** that provides:

1. **Zero-overhead validation**: All parameter/body validation happens in Rust
2. **Type-safe by default**: Leverage native type systems (Python type hints, TypeScript types, Rust types)
3. **Consistent semantics**: Same behavior across all language bindings
4. **Explicit over magic**: Clear, predictable behavior
5. **Performance-first**: Fast path is the default path
6. **Schema-driven**: JSON Schema as the universal contract between languages

## Core Design Decisions

### Language-First Design
- Rust-first API that can be idiomatically reflected in other languages
- Each binding respects the host language's conventions and patterns

### Inspiration Sources
- **Fastify** (TypeScript): Effective schema-first design and plugin architecture
- **Axum** (Rust): High-performance, composable HTTP with type-safe extractors
- **Litestar** (Python): Clean decorator API and dependency injection patterns
- **Future languages**: Go (fiber/echo), Ruby (rails/sinatra), PHP (laravel/symfony)

### Code Generation
**See**: [codegen-strategy.md](./codegen-strategy.md) for complete design

Code generation is a **test infrastructure tool** in this repository:
- Generates complete test suites from fixtures (Rust, Python/pytest, TypeScript/Vitest)
- Generates minimal test apps to validate fixture scenarios
- Lives in `e2e/<lang>/` and can be regenerated at will
- **NOT a published feature** - internal tooling for maintaining test coverage
- Full codegen feature (OpenAPI → production apps) is future work

### 1. Route Decorators (Python: Litestar-style)

Following Litestar's excellent design pattern:

```python
from spikard import get, post, put, patch, delete, head, options, route
```

**NOT** `app.get()` - decorators are imported directly and used on handler functions.

### 2. Schema Libraries Support

#### Rust
- ? TBD

#### Python
Support ALL major Python schema libraries via `msgspec` conversion to JSON Schema:

- ✅ **dataclasses** (stdlib)
- ✅ **TypedDict** (stdlib)
- ✅ **NamedTuple** (stdlib)
- ✅ **attrs**
- ✅ **msgspec.Struct**
- ✅ **Pydantic v1** (via protocol)
- ✅ **Pydantic v2** (via protocol)
- ✅ **Raw dict/Mapping** with JSON Schema

All of these convert to JSON Schema → sent to Rust for validation.

#### TypeScript
Support major TypeScript schema libraries via JSON Schema extraction:

- ✅ **Zod** (most popular, ~38k stars)
- ✅ **TypeBox** (~4.6k stars, JSON Schema native)
- ✅ **io-ts** (~6.7k stars)
- ✅ **Yup** (~22k stars, if it supports JSON Schema)
- ✅ **ArkType** (newer, growing)
- ✅ **Raw object literal** with JSON Schema

#### Ruby

TBD

#### PHP

TBD

**Requirement**: Must provide `.toJSONSchema()` method or similar API. We can target a set of conventional names - in different langueges - interface driven design.

### 3. No Decorators in TypeScript

We will **NOT** use the NestJS decorator approach. Instead:

```typescript
// Functional API like Fastify
app.get('/users/:userId', {
  params: UserParamsSchema,
  query: UserQuerySchema,
  handler: async ({ params, query }) => { ... }
});
```

## Design Principles

### 1. Rust Core, Language Bindings

```
┌─────────────────────────────────────────┐
│         Application Code                │
│    (Python, TypeScript, Rust, etc.)     │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│      Language-Specific Bindings         │
│   (PyO3, napi-rs, wasm-bindgen, etc.)  │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│         Spikard Rust Core               │
│  • Routing (Axum)                       │
│  • Validation (jsonschema + custom)     │
│  • Parameter extraction                 │
│  • Serialization (serde_json)           │
└─────────────────────────────────────────┘
```

### 2. Schema-Driven Validation

All validation is schema-driven:
- **Python**: Extract JSON Schema from Pydantic/msgspec/dataclasses
- **TypeScript**: Extract JSON Schema from Zod/TypeBox/io-ts
- **Rust**: Direct struct definition or JSON Schema

This means:
- ✅ Single source of truth (the schema)
- ✅ Validation happens once in Rust
- ✅ No runtime overhead in bindings
- ✅ OpenAPI generation for free

### 3. Explicit Parameter Sources

We distinguish between parameter sources explicitly:

```
Request
  ├── path: /users/{user_id}          → Path parameters
  ├── query: ?page=1&limit=10         → Query parameters
  ├── headers: Authorization: ...     → Headers
  ├── cookies: session=abc123         → Cookies
  └── body: {"name": "Alice"}         → Request body
```

## Python API Design (Litestar-inspired)

### Core Approach

```python
from spikard import Spikard, get, post, put, delete, Parameter
from dataclasses import dataclass

app = Spikard()

@dataclass
class User:
    name: str
    email: str
    age: int

# Simple handler - automatic parameter inference
@get("/users/{user_id:int}")
async def get_user(user_id: int) -> User:
    """Path params inferred from route and type hints."""
    return User(name="Alice", email="alice@example.com", age=30)

# Query parameters with defaults
@get("/users")
async def list_users(
    page: int = 1,
    limit: int = 10,
    search: str | None = None,
) -> list[User]:
    """Query params inferred from default values."""
    return []

# Request body (POST/PUT/PATCH) - implicit from object type
@post("/users")
async def create_user(user: User) -> User:
    """Body parameter inferred for POST with object type."""
    return user

# Explicit parameter sources with Parameter()
@get("/items")
async def search_items(
    query: str = Parameter(query="q", min_length=3),
    api_key: str = Parameter(header="X-API-Key"),
    session: str | None = Parameter(cookie="session_id", default=None),
) -> list[dict]:
    """Explicit sources via Parameter() function."""
    return []

# Multiple HTTP methods via route()
from spikard import route

@route("/resource/{id:int}", http_method=["GET", "HEAD"])
async def get_resource(id: int) -> dict:
    return {"id": id}
```

### Schema Library Support

#### 1. Dataclasses (stdlib)

```python
from dataclasses import dataclass
from spikard import post

@dataclass
class CreateUser:
    name: str
    email: str
    age: int

@post("/users")
async def create_user(user: CreateUser) -> CreateUser:
    return user
```

#### 2. TypedDict (stdlib)

```python
from typing import TypedDict
from spikard import post

class CreateUser(TypedDict):
    name: str
    email: str
    age: int

@post("/users")
async def create_user(user: CreateUser) -> CreateUser:
    return user
```

#### 3. msgspec.Struct

```python
import msgspec
from spikard import post

class CreateUser(msgspec.Struct):
    name: str
    email: str
    age: int

@post("/users")
async def create_user(user: CreateUser) -> CreateUser:
    return user
```

#### 4. Pydantic (v1/v2)

```python
from pydantic import BaseModel, Field
from spikard import post

class CreateUser(BaseModel):
    name: str = Field(min_length=1, max_length=50)
    email: str = Field(pattern=r"^[\w\.-]+@[\w\.-]+\.\w+$")
    age: int = Field(ge=0, le=150)

@post("/users")
async def create_user(user: CreateUser) -> CreateUser:
    return user
```

#### 5. attrs

```python
from attrs import define
from spikard import post

@define
class CreateUser:
    name: str
    email: str
    age: int

@post("/users")
async def create_user(user: CreateUser) -> CreateUser:
    return user
```

#### 6. Raw JSON Schema

```python
from spikard import post

user_schema = {
    "type": "object",
    "properties": {
        "name": {"type": "string", "minLength": 1},
        "email": {"type": "string", "format": "email"},
        "age": {"type": "integer", "minimum": 0}
    },
    "required": ["name", "email", "age"]
}

@post("/users", body_schema=user_schema, response_schema=user_schema)
async def create_user(user: dict) -> dict:
    return user
```

### Parameter() Function API

Following Litestar's design:

```python
def Parameter(
    # Parameter source (ONE of these should be set)
    query: str | None = None,      # Query parameter name
    header: str | None = None,      # Header name
    cookie: str | None = None,      # Cookie name
    # Defaults and requirements
    default: Any = Empty,            # Default value
    required: bool | None = None,    # Override required inference
    # Validation constraints (OpenAPI/JSON Schema)
    gt: float | None = None,         # exclusiveMinimum
    ge: float | None = None,         # minimum
    lt: float | None = None,         # exclusiveMaximum
    le: float | None = None,         # maximum
    min_length: int | None = None,   # minLength
    max_length: int | None = None,   # maxLength
    pattern: str | None = None,      # regex pattern
    min_items: int | None = None,    # minItems (arrays)
    max_items: int | None = None,    # maxItems (arrays)
    multiple_of: float | None = None, # multipleOf
    # OpenAPI documentation
    title: str | None = None,
    description: str | None = None,
    examples: list | None = None,
) -> Any:
    """Create a parameter with explicit source and constraints."""
    ...
```

## TypeScript API Design

### Core Approach - Functional API (No Decorators)

```typescript
import { Spikard } from 'spikard';
import { z } from 'zod';

const app = new Spikard();

// Simple route with inline schemas
app.get('/users/:userId', {
  params: z.object({
    userId: z.number().int().positive()
  }),
  query: z.object({
    page: z.number().int().default(1),
    limit: z.number().int().min(1).max(100).default(10)
  }),
  handler: async ({ params, query }) => {
    return {
      userId: params.userId,
      page: query.page,
      limit: query.limit
    };
  }
});

// POST with body validation
const CreateUser = z.object({
  name: z.string().min(1).max(50),
  email: z.string().email(),
  age: z.number().int().min(0).max(150)
});

app.post('/users', {
  body: CreateUser,
  headers: z.object({
    'x-api-key': z.string()
  }),
  handler: async ({ body, headers }) => {
    return body;
  }
});
```

### Schema Library Support

#### 1. Zod (Primary, Built-in)

```typescript
import { z } from 'zod'; // NOT: never re-export third party code.,

const User = z.object({
  name: z.string(),
  email: z.string().email(),
  age: z.number().int().min(0)
});

app.post('/users', {
  body: User,
  handler: async ({ body }) => body
});
```

#### 2. TypeBox

```typescript
import { Type } from '@sinclair/typebox';
import { Spikard } from 'spikard';

const User = Type.Object({
  name: Type.String(),
  email: Type.String({ format: 'email' }),
  age: Type.Integer({ minimum: 0 })
});

app.post('/users', {
  body: User, // TypeBox has .toJSONSchema()
  handler: async ({ body }) => body
});
```

#### 3. io-ts

```typescript
import * as t from 'io-ts';
import { Spikard } from 'spikard';

const User = t.type({
  name: t.string,
  email: t.string,
  age: t.number
});

app.post('/users', {
  body: User, // io-ts codec
  handler: async ({ body }) => body
});
```

#### 4. Raw JSON Schema

```typescript
const userSchema = {
  type: 'object',
  properties: {
    name: { type: 'string', minLength: 1 },
    email: { type: 'string', format: 'email' },
    age: { type: 'integer', minimum: 0 }
  },
  required: ['name', 'email', 'age']
};

app.post('/users', {
  body: userSchema,
  handler: async ({ body }) => body as User
});
```

### Interface-Based Schema Protocol

Spikard uses an interface-based approach to support any schema library:

```typescript
interface SpikardSchema {
  // Option 1: JSON Schema property
  jsonSchema?: object;

  // Option 2: Method to generate JSON Schema
  toJSONSchema?(): object;

  // Option 3: OpenAPI schema property
  openapi?: object;

  // Option 4: Method to generate OpenAPI schema
  toOpenAPI?(): object;
}
```

Any object implementing this interface can be used as a schema.

## Rust API Design

```rust
use spikard::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Schema)]
struct User {
    name: String,
    email: String,
    age: u32,
}

#[derive(Debug, Deserialize, Schema)]
struct GetUserParams {
    #[schema(minimum = 1)]
    user_id: i32,
}

#[derive(Debug, Deserialize, Schema)]
struct GetUserQuery {
    #[schema(default = false)]
    include_profile: bool,
}

async fn get_user(
    Path(params): Path<GetUserParams>,
    Query(query): Query<GetUserQuery>,
) -> Json<User> {
    Json(User {
        name: "Alice".into(),
        email: "alice@example.com".into(),
        age: 30,
    })
}

#[tokio::main]
async fn main() {
    let app = Spikard::new()
        .route("/users/:user_id", get(get_user))
        .run("0.0.0.0:3000")
        .await;
}
```

## Recommended Approach

### Python: Option C (Hybrid)
- Smart defaults for simplicity
- Explicit markers when needed
- Familiar to FastAPI users but cleaner

### TypeScript: Option C (Hybrid with Zod)
- Zod for schema definition (popular, well-typed)
- Destructured handler params
- Optional decorator support for those who want it

### Rust: Native Axum-style
- Leverage Rust's type system fully
- Axum extractors pattern (familiar to Rust devs)
- Derive macros for schemas

## Key Decisions

### 1. Parameter Source Inference

**Rule**: Infer parameter source from HTTP method and type:

| Method      | Parameter Type | Default Source |
|-------------|---------------|----------------|
| GET/DELETE  | Scalar        | Query          |
| GET/DELETE  | Object        | Query (flat)   |
| POST/PUT/PATCH | Object    | Body           |
| POST/PUT/PATCH | Scalar    | Query          |
| Any         | `Path(...)`   | Path           |
| Any         | `Header(...)` | Header         |
| Any         | `Cookie(...)` | Cookie         |

### 2. Validation Strategy

- **Schema-first**: All validation via JSON Schema
- **Language-native constraints**: Pydantic/Zod constraints → JSON Schema
- **Custom validators**: Future extension point

### 3. Error Format

**Consistent across all languages**:

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

### 4. Response Handling

```python
# Automatic serialization
@app.get("/user")
async def get_user() -> User:
    return User(...)  # Serialized to JSON

# Custom response
from spikard import Response

@app.get("/data")
async def get_data() -> Response:
    return Response(
        content={"data": "value"},
        status_code=201,
        headers={"X-Custom": "header"}
    )

# Stream response
@app.get("/stream")
async def stream_data():
    async def generate():
        for i in range(10):
            yield f"data: {i}\n\n"
    return StreamingResponse(generate(), media_type="text/event-stream")
```

## Next Steps

1. **Finalize Python API**: Choose variant and implement
2. **Finalize TypeScript API**: Choose variant and implement
3. **Create canonical test fixtures**: Based on our API design
4. **Analyze other frameworks**: Extract valuable test cases
5. **Implement missing categories**: Headers, cookies, bodies

## Questions to Resolve

1. **Python**: Option B (clean) vs Option C (hybrid)?
2. **TypeScript**: Decorators optional or mandatory?
3. **Dependency injection**: Do we want it?
4. **Middleware API**: How should it look?
5. **WebSocket support**: Now or later?

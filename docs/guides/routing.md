# Routing Basics

Routing is uniform across bindings: define an `App`, register routes with typed parameters, and return typed responses.

## Patterns
- **Path params**: `/users/{id:int}` (Python) or `/users/:id` (TypeScript/Ruby/Rust)
- **Query params**: automatically parsed into dictionaries/objects; add validators to enforce shapes.
- **HTTP verbs**: `get`, `post`, `put`, `patch`, `delete`, and `options` share the same naming across bindings.

## Python Example
```python
from spikard import App
from msgspec import Struct

class User(Struct):
    id: int
    name: str

app = App()

@app.get("/users/{id:int}")
async def get_user(id: int) -> User:
    return User(id=id, name="Alice")
```

## TypeScript Example
```typescript
import { App } from "spikard";
import { z } from "zod";

const app = new App();
const User = z.object({ id: z.number(), name: z.string() });

app.get("/users/:id", ({ params }) => ({
  id: Number(params.id),
  name: "Alice",
}));

app.post("/users", ({ body }) => User.parse(body));
```

## Middleware per Route
Attach middleware at the app level or per route to handle auth/logging selectively. See [Middleware guide](middleware.md) for patterns.

## Best Practices
- Keep handlers pure and push IO to services; the runtime will handle serialization and validation.
- Prefer DTO structs/classes for shared schemas so codegen can derive OpenAPI.
- Group related routes into modules or routers to keep startup organized.

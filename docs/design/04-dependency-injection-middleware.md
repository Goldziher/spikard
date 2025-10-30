# Dependency Injection and Middleware

## Philosophy

- **DI handled in bindings**: Python and TypeScript implement their own DI systems
- **Rust stays simple**: Pure functional handlers, no magic DI
- **Middleware in Rust**: Performance-critical middleware lives in Rust core
- **Application middleware**: Can be implemented in bindings

## Dependency Injection (Python)

Following Litestar's excellent DI pattern:

### Basic Dependency

```python
from spikard import get, Provide
from dataclasses import dataclass

@dataclass
class Database:
    connection_string: str

    async def get_user(self, user_id: int):
        # Database logic
        return {"id": user_id, "name": "Alice"}

async def get_db() -> Database:
    """Dependency provider function."""
    return Database(connection_string="postgresql://...")

@get("/users/{user_id}")
async def get_user(
    user_id: int,
    db: Database = Provide(get_db),  # Inject dependency
) -> dict:
    return await db.get_user(user_id)
```

### Request-Scoped Dependencies

```python
from spikard import get, Provide, Request

async def get_current_user(request: Request) -> User:
    """Extract user from request (auth token, etc.)."""
    token = request.headers.get("authorization")
    # Validate token and return user
    return User(id=123, name="Alice")

@get("/profile")
async def get_profile(
    current_user: User = Provide(get_current_user),
) -> User:
    return current_user
```

### Dependency Chains

```python
async def get_db() -> Database:
    return Database(...)

async def get_user_service(db: Database = Provide(get_db)) -> UserService:
    """Dependency can depend on other dependencies."""
    return UserService(db)

@get("/users/{user_id}")
async def get_user(
    user_id: int,
    service: UserService = Provide(get_user_service),
) -> User:
    return await service.get(user_id)
```

### Application-Level Dependencies

```python
from spikard import Spikard

def get_config() -> Config:
    return Config.load()

app = Spikard(
    dependencies={
        Config: Provide(get_config),  # Available to all handlers
    }
)
```

## Dependency Injection (TypeScript)

Similar pattern using functional providers:

### Basic Dependency

```typescript
import { Spikard, provide } from 'spikard';

class Database {
  constructor(private connectionString: string) {}

  async getUser(userId: number) {
    return { id: userId, name: 'Alice' };
  }
}

const getDb = () => {
  return new Database('postgresql://...');
};

const app = new Spikard();

app.get('/users/:userId', {
  params: z.object({ userId: z.number() }),
  dependencies: {
    db: provide(getDb)
  },
  handler: async ({ params, db }) => {
    return await db.getUser(params.userId);
  }
});
```

### Request-Scoped Dependencies

```typescript
import { Request } from 'spikard';

const getCurrentUser = async (request: Request): Promise<User> => {
  const token = request.headers['authorization'];
  // Validate and return user
  return { id: 123, name: 'Alice' };
};

app.get('/profile', {
  dependencies: {
    currentUser: provide(getCurrentUser, { scope: 'request' })
  },
  handler: async ({ currentUser }) => {
    return currentUser;
  }
});
```

### Dependency Injection Design

```typescript
interface DependencyProvider<T> {
  factory: (...deps: any[]) => T | Promise<T>;
  scope?: 'singleton' | 'request' | 'transient';
  dependencies?: string[]; // Names of other dependencies needed
}

function provide<T>(
  factory: (...deps: any[]) => T | Promise<T>,
  options?: { scope?: 'singleton' | 'request' | 'transient' }
): DependencyProvider<T>;
```

## Middleware (Rust Core)

Performance-critical middleware lives in Rust:

### Built-in Middleware

```rust
use spikard::middleware::{Cors, Compression, RateLimit, Logging};

let app = Spikard::new()
    .middleware(Cors::permissive())
    .middleware(Compression::gzip())
    .middleware(RateLimit::new(100, Duration::from_secs(60)))
    .middleware(Logging::default())
    .routes(routes);
```

### Custom Rust Middleware

```rust
use spikard::middleware::{Middleware, Next};
use axum::http::Request;

struct AuthMiddleware {
    secret_key: String,
}

#[async_trait]
impl Middleware for AuthMiddleware {
    async fn handle(
        &self,
        req: Request<Body>,
        next: Next,
    ) -> Result<Response<Body>, Error> {
        // Validate auth header
        if let Some(token) = req.headers().get("authorization") {
            if self.validate_token(token) {
                return next.run(req).await;
            }
        }

        Err(Error::Unauthorized)
    }
}
```

## Middleware (Python Application Layer)

Application-specific middleware in Python:

### Function-Based Middleware

```python
from spikard import Spikard, Request, Response
from typing import Awaitable, Callable

async def timing_middleware(
    request: Request,
    handler: Callable[[Request], Awaitable[Response]],
) -> Response:
    """Measure request duration."""
    start = time.time()
    response = await handler(request)
    duration = time.time() - start
    response.headers["X-Duration"] = str(duration)
    return response

app = Spikard(middleware=[timing_middleware])
```

### Class-Based Middleware

```python
from spikard.middleware import Middleware

class RateLimitMiddleware(Middleware):
    def __init__(self, max_requests: int, window_seconds: int):
        self.max_requests = max_requests
        self.window = window_seconds
        self.requests = {}

    async def process(
        self,
        request: Request,
        handler: Callable,
    ) -> Response:
        client_ip = request.client.host

        # Rate limit logic
        if self.is_rate_limited(client_ip):
            return Response(
                content={"error": "Rate limit exceeded"},
                status_code=429
            )

        return await handler(request)

app = Spikard(
    middleware=[
        RateLimitMiddleware(max_requests=100, window_seconds=60)
    ]
)
```

## Middleware (TypeScript Application Layer)

```typescript
import { Spikard, Request, Response, NextFunction } from 'spikard';

const timingMiddleware = async (
  req: Request,
  next: NextFunction
): Promise<Response> => {
  const start = Date.now();
  const response = await next();
  const duration = Date.now() - start;
  response.headers['X-Duration'] = duration.toString();
  return response;
};

const app = new Spikard({
  middleware: [timingMiddleware]
});
```

## Middleware Execution Order

```
Incoming Request
     ↓
┌────────────────────┐
│  Rust Middleware   │  ← CORS, Compression, Rate Limiting
│  (High Performance)│
└────────┬───────────┘
         ↓
┌────────────────────┐
│ Language Middleware│  ← Logging, Auth, Custom Logic
│  (Python/TS)       │
└────────┬───────────┘
         ↓
┌────────────────────┐
│  Routing (Rust)    │  ← Path matching, parameter extraction
└────────┬───────────┘
         ↓
┌────────────────────┐
│ Validation (Rust)  │  ← JSON Schema validation
└────────┬───────────┘
         ↓
┌────────────────────┐
│ DI Resolution      │  ← Dependency injection (Python/TS)
│  (Python/TS)       │
└────────┬───────────┘
         ↓
┌────────────────────┐
│  Handler Function  │  ← User code
└────────┬───────────┘
         ↓
    Response
```

## Implementation Strategy

### Phase 1: Rust Core (Now)
- ✅ Routing
- ✅ Parameter extraction
- ✅ Validation
- 🔨 Built-in middleware (CORS, compression, rate limiting)

### Phase 2: Python DI (Next)
- Implement `Provide` mechanism
- Dependency graph resolution
- Scope management (singleton, request, transient)
- Circular dependency detection

### Phase 3: TypeScript DI (Parallel)
- Similar to Python but TypeScript-native
- Integration with decorators (optional)
- Type inference for dependencies

### Phase 4: Application Middleware
- Python middleware hooks
- TypeScript middleware hooks
- Integration with Rust middleware stack

## Design Principles

1. **Performance**: Rust handles hot path (routing, validation)
2. **Flexibility**: Bindings handle DI and app-specific middleware
3. **Consistency**: Same DI/middleware patterns across Python and TypeScript
4. **Simplicity**: Rust stays simple and fast, complexity in bindings
5. **Type Safety**: Full type inference in both Python and TypeScript

## Next Steps

1. Finalize DI API for Python
2. Finalize DI API for TypeScript
3. Implement Rust middleware infrastructure
4. Create comprehensive examples
5. Document migration paths from FastAPI/NestJS

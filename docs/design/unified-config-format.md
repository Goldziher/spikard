# Unified Configuration Format & Code Generation

**Date**: 2025-01-30
**Status**: 🟡 Draft
**Related**: [08-metaprogramming-test-system.md](./08-metaprogramming-test-system.md), [00-architecture.md](./00-architecture.md)

## Executive Summary

A YAML/JSON configuration format that defines entire servers declaratively, enabling code generation for HTTP, queues, cloud events, and gRPC across Python, TypeScript, and Rust. Spikard becomes a toolbox where each protocol module plugs into a unified configuration schema.

## Goals

1. **Declarative Servers**: Define complete applications in YAML/JSON
2. **Multi-Protocol**: HTTP, gRPC, queues (SQS/Kafka), CloudEvents in one config
3. **Code Generation**: Generate idiomatic code for Python/TypeScript/Rust
4. **OpenAPI/Protobuf**: Integrate with existing ecosystem standards
5. **CLI Integration**: `spikard generate` command for codegen
6. **Type Safety**: Generated code is fully typed and validated

## Non-Goals

- Supporting every possible framework feature (focus on common patterns)
- Runtime interpretation (prefer build-time generation)
- Replacing hand-written code entirely (augment, don't replace)

## Architecture

```
┌────────────────────────────────────────────────────┐
│          spikard.yaml (Configuration)              │
│  - HTTP routes with validation                     │
│  - gRPC services                                   │
│  - Queue consumers/producers                       │
│  - CloudEvents handlers                            │
└──────────────────┬─────────────────────────────────┘
                   │
         ┌─────────▼─────────┐
         │  spikard-codegen  │  (separate crate/package)
         │  - Parser         │
         │  - Validator      │
         │  - Generators     │
         └─────────┬─────────┘
                   │
        ┌──────────┼──────────┐
        │          │          │
   ┌────▼───┐ ┌───▼───┐ ┌───▼────┐
   │Python  │ │TypeScr│ │  Rust  │
   │ .py    │ │ipt.ts │ │  .rs   │
   └────────┘ └───────┘ └────────┘
```

## Configuration Schema

### Top-Level Structure

```yaml
# spikard.yaml
version: "1.0"
name: "my-service"
description: "Multi-protocol microservice"

# Runtime configuration
runtime:
  host: "0.0.0.0"
  port: 8000
  workers: 4
  log_level: "info"

# HTTP routes
http:
  routes:
    - path: "/users"
      method: POST
      handler: "handlers.create_user"
      request:
        body:
          type: object
          required: [name, email]
          properties:
            name: { type: string, minLength: 1 }
            email: { type: string, format: email }
      response:
        status: 201
        body:
          type: object
          properties:
            id: { type: string, format: uuid }
            name: { type: string }
            email: { type: string }

    - path: "/users/{user_id}"
      method: GET
      handler: "handlers.get_user"
      parameters:
        path:
          user_id: { type: string, format: uuid }
      response:
        status: 200
        body: { $ref: "#/schemas/User" }

  middleware:
    - type: cors
      allow_origins: ["https://example.com"]
    - type: auth
      scheme: bearer
      jwks_url: "https://auth.example.com/.well-known/jwks.json"

# gRPC services
grpc:
  services:
    - name: UserService
      proto: "protos/user.proto"
      handlers:
        GetUser: "handlers.get_user_grpc"
        CreateUser: "handlers.create_user_grpc"

# Queue consumers
queues:
  consumers:
    - name: user-events
      type: sqs
      queue_url: "${SQS_QUEUE_URL}"
      handler: "handlers.process_user_event"
      batch_size: 10
      visibility_timeout: 30

  producers:
    - name: notifications
      type: kafka
      topic: "user-notifications"
      brokers: ["kafka:9092"]

# CloudEvents handlers
cloudevents:
  subscriptions:
    - type: "com.example.user.created"
      version: "1.0"
      handler: "handlers.on_user_created"
    - type: "com.example.user.updated"
      version: "1.0"
      handler: "handlers.on_user_updated"

# Reusable schemas
schemas:
  User:
    type: object
    required: [id, name, email]
    properties:
      id: { type: string, format: uuid }
      name: { type: string }
      email: { type: string, format: email }
      created_at: { type: string, format: date-time }

# OpenAPI integration
openapi:
  info:
    title: "My Service API"
    version: "1.0.0"
  servers:
    - url: "https://api.example.com"
```

### HTTP Route Configuration

```yaml
http:
  routes:
    - path: "/items/{item_id}"
      method: GET
      handler: "handlers.get_item"

      # Path parameters (from URL)
      parameters:
        path:
          item_id:
            type: integer
            minimum: 1

        # Query parameters
        query:
          limit:
            type: integer
            default: 10
            minimum: 1
            maximum: 100
          offset:
            type: integer
            default: 0
            minimum: 0

        # Headers
        headers:
          X-API-Key:
            type: string
            required: true
            pattern: "^[a-f0-9]{32}$"

        # Cookies
        cookies:
          session_id:
            type: string
            required: true

      # Request body (for POST/PUT/PATCH)
      request:
        content_type: application/json
        body:
          type: object
          required: [name]
          properties:
            name: { type: string, minLength: 3 }
            tags:
              type: array
              items: { type: string }
              uniqueItems: true

      # Response specification
      response:
        status: 200
        headers:
          X-Total-Count: { type: integer }
        body:
          type: object
          properties:
            data: { $ref: "#/schemas/Item" }
            meta:
              type: object
              properties:
                total: { type: integer }

      # Validation errors
      errors:
        - status: 400
          body: { $ref: "#/schemas/ValidationError" }
        - status: 401
          body: { $ref: "#/schemas/UnauthorizedError" }
        - status: 404
          body: { $ref: "#/schemas/NotFoundError" }

      # Middleware overrides
      middleware:
        - type: rate_limit
          requests_per_minute: 100
        - type: cache
          ttl: 300

      # Observability
      tracing:
        enabled: true
        sample_rate: 0.1
      metrics:
        enabled: true
        labels:
          service: "items-api"
```

### Queue Configuration

```yaml
queues:
  consumers:
    - name: order-processor
      type: sqs
      queue_url: "${SQS_QUEUE_URL}"
      handler: "handlers.process_order"

      # Consumer configuration
      batch_size: 10
      visibility_timeout: 30
      max_retries: 3
      dead_letter_queue: "orders-dlq"

      # Message schema validation
      message_schema:
        type: object
        required: [order_id, items]
        properties:
          order_id: { type: string, format: uuid }
          items:
            type: array
            items:
              type: object
              required: [product_id, quantity]
              properties:
                product_id: { type: string }
                quantity: { type: integer, minimum: 1 }

    - name: user-events
      type: kafka
      topic: "user-events"
      group_id: "user-service"
      handler: "handlers.process_user_event"

      # Kafka-specific config
      offset_reset: earliest
      enable_auto_commit: true
      message_schema:
        $ref: "#/schemas/UserEvent"

  producers:
    - name: notifications
      type: kafka
      topic: "notifications"
      brokers: ["kafka:9092"]

      # Producer config
      compression: gzip
      acks: all
      retries: 3
```

### gRPC Configuration

```yaml
grpc:
  services:
    - name: UserService
      proto: "protos/user.proto"

      # Method handlers
      handlers:
        GetUser: "handlers.grpc.get_user"
        CreateUser: "handlers.grpc.create_user"
        UpdateUser: "handlers.grpc.update_user"
        DeleteUser: "handlers.grpc.delete_user"

      # Interceptors (middleware)
      interceptors:
        - type: auth
          header: "authorization"
        - type: logging
        - type: metrics

      # Streaming support
      streaming:
        ListUsers:
          type: server_streaming
          handler: "handlers.grpc.list_users_stream"
        UploadData:
          type: client_streaming
          handler: "handlers.grpc.upload_data_stream"
        Chat:
          type: bidirectional_streaming
          handler: "handlers.grpc.chat_stream"
```

### CloudEvents Configuration

```yaml
cloudevents:
  # Event subscriptions
  subscriptions:
    - type: "com.example.user.created"
      version: "1.0"
      handler: "handlers.events.on_user_created"

      # Schema validation
      data_schema:
        type: object
        required: [user_id, email]
        properties:
          user_id: { type: string, format: uuid }
          email: { type: string, format: email }

      # Filtering
      filters:
        source: "users-service"
        subject: "^user/"

    - type: "com.example.order.*"  # Wildcard support
      version: "1.0"
      handler: "handlers.events.on_order_event"

  # Event publishing
  publishers:
    - name: user_events
      type: http
      endpoint: "${CLOUDEVENTS_ENDPOINT}"

      # Event types this service publishes
      event_types:
        - type: "com.example.user.created"
          version: "1.0"
          schema: { $ref: "#/schemas/UserCreatedEvent" }
```

## Code Generation

### CLI Integration

```bash
# Generate code from config
spikard generate --config spikard.yaml --target python --out src/generated/

# Generate for multiple targets
spikard generate --config spikard.yaml --target python,typescript,rust

# Generate only specific protocols
spikard generate --config spikard.yaml --protocols http,grpc --target python

# Watch mode for development
spikard generate --config spikard.yaml --target python --watch

# Validate configuration
spikard validate --config spikard.yaml

# Generate OpenAPI spec
spikard openapi --config spikard.yaml --out openapi.yaml

# Generate Protobuf definitions
spikard proto --config spikard.yaml --out protos/
```

### Generated Python Code

```python
# generated/routes.py (auto-generated from spikard.yaml)
from spikard import Spikard, Request, Response, route
from spikard.validation import validate
from typing import Annotated
from pydantic import BaseModel, EmailStr
from uuid import UUID

# Generated schema models
class CreateUserRequest(BaseModel):
    name: str
    email: EmailStr

class User(BaseModel):
    id: UUID
    name: str
    email: EmailStr

# Generated route handlers (you implement the logic)
@route("/users", method="POST")
@validate(body=CreateUserRequest)
async def create_user(request: Request[CreateUserRequest]) -> Response[User]:
    # Your implementation here
    from .handlers import create_user as handler
    return await handler(request)

@route("/users/{user_id}", method="GET")
@validate(path={"user_id": UUID})
async def get_user(user_id: UUID) -> Response[User]:
    from .handlers import get_user as handler
    return await handler(user_id)

# Generated app factory
def create_app() -> Spikard:
    app = Spikard()

    # Register routes
    app.add_route(create_user)
    app.add_route(get_user)

    # Add middleware
    app.add_middleware(CORSMiddleware(
        allow_origins=["https://example.com"]
    ))

    return app
```

### Generated TypeScript Code

```typescript
// generated/routes.ts (auto-generated from spikard.yaml)
import { Spikard, Route, Validate } from '@spikard/node';
import { z } from 'zod';

// Generated schemas
const CreateUserRequestSchema = z.object({
  name: z.string().min(1),
  email: z.string().email()
});

const UserSchema = z.object({
  id: z.string().uuid(),
  name: z.string(),
  email: z.string().email()
});

type CreateUserRequest = z.infer<typeof CreateUserRequestSchema>;
type User = z.infer<typeof UserSchema>;

// Generated route definitions
export const routes = [
  {
    path: '/users',
    method: 'POST' as const,
    schema: {
      body: CreateUserRequestSchema,
      response: {
        201: UserSchema
      }
    },
    handler: async (request: { body: CreateUserRequest }) => {
      const { create_user } = await import('./handlers');
      return create_user(request);
    }
  },

  {
    path: '/users/:user_id',
    method: 'GET' as const,
    schema: {
      params: z.object({
        user_id: z.string().uuid()
      }),
      response: {
        200: UserSchema
      }
    },
    handler: async (request: { params: { user_id: string } }) => {
      const { get_user } = await import('./handlers');
      return get_user(request.params.user_id);
    }
  }
];

// Generated app factory
export function createApp(): Spikard {
  const app = new Spikard();

  for (const route of routes) {
    app.route(route.path, route);
  }

  return app;
}
```

### Generated Rust Code

```rust
// generated/routes.rs (auto-generated from spikard.yaml)
use spikard::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Generated schema structs
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

// Generated route handlers
pub async fn create_user(
    ValidatedJson(body): ValidatedJson<CreateUserRequest>
) -> Result<Json<User>, Error> {
    crate::handlers::create_user(body).await
}

pub async fn get_user(
    Path(user_id): Path<Uuid>
) -> Result<Json<User>, Error> {
    crate::handlers::get_user(user_id).await
}

// Generated router
pub fn create_router() -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users/:user_id", get(get_user))
}
```

## Package Structure: `spikard-codegen`

```
crates/spikard-codegen/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Public API
│   ├── parser/
│   │   ├── mod.rs
│   │   ├── config.rs       # Parse YAML/JSON config
│   │   ├── validator.rs    # Validate config schema
│   │   └── resolver.rs     # Resolve $ref, includes
│   ├── ir/                 # Intermediate representation
│   │   ├── mod.rs
│   │   ├── route.rs        # Route IR
│   │   ├── schema.rs       # Schema IR
│   │   ├── grpc.rs         # gRPC service IR
│   │   └── queue.rs        # Queue consumer/producer IR
│   ├── generators/
│   │   ├── mod.rs
│   │   ├── python.rs       # Python code generator
│   │   ├── typescript.rs   # TypeScript code generator
│   │   ├── rust.rs         # Rust code generator
│   │   └── openapi.rs      # OpenAPI spec generator
│   └── templates/          # Code templates
│       ├── python/
│       ├── typescript/
│       └── rust/
└── tests/
    ├── fixtures/           # Test configs
    └── snapshots/          # Generated code snapshots
```

### Core API

```rust
// crates/spikard-codegen/src/lib.rs
pub struct CodegenConfig {
    pub config_path: PathBuf,
    pub target: Target,
    pub output_dir: PathBuf,
    pub protocols: Vec<Protocol>,
}

pub enum Target {
    Python,
    TypeScript,
    Rust,
}

pub enum Protocol {
    Http,
    Grpc,
    Queue,
    CloudEvents,
}

pub struct Generator {
    config: SpikardConfig,
}

impl Generator {
    pub fn from_file(path: &Path) -> Result<Self>;
    pub fn validate(&self) -> Result<()>;
    pub fn generate(&self, target: Target, output: &Path) -> Result<()>;
    pub fn generate_openapi(&self) -> Result<OpenApiSpec>;
}
```

## Integration with Existing Ecosystem

### OpenAPI

```yaml
# spikard.yaml includes OpenAPI metadata
openapi:
  info:
    title: "My API"
    version: "1.0.0"
    contact:
      email: "api@example.com"

# Generate full OpenAPI spec
# $ spikard openapi --config spikard.yaml
```

Generated `openapi.yaml`:

```yaml
openapi: 3.1.0
info:
  title: My API
  version: 1.0.0
paths:
  /users:
    post:
      requestBody:
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/CreateUserRequest'
      responses:
        '201':
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/User'
```

### Protobuf

```yaml
# spikard.yaml references existing .proto files
grpc:
  services:
    - name: UserService
      proto: "protos/user.proto"  # Existing protobuf definition
      handlers: ...

# Or generate .proto from schema
# $ spikard proto --config spikard.yaml --out protos/
```

## Benefits

1. **Single Source of Truth**: Config defines API contract
2. **Type Safety**: Generated code is fully typed
3. **Multi-Protocol**: HTTP, gRPC, queues, events in one place
4. **Ecosystem Integration**: Works with OpenAPI, Protobuf
5. **DRY**: Define once, generate for all targets
6. **Validation**: Config validation catches errors before code generation

## Implementation Plan

### Phase 1: Core Parser & Validator (2 weeks)
- Parse YAML/JSON config
- Validate against schema
- Build intermediate representation (IR)

### Phase 2: Python Generator (2 weeks)
- Generate Python route handlers
- Generate Pydantic models
- Integrate with existing `spikard-py`

### Phase 3: TypeScript Generator (2 weeks)
- Generate TypeScript routes
- Generate Zod schemas
- Integrate with `spikard-node`

### Phase 4: Extended Protocols (4 weeks)
- gRPC service generation
- Queue consumer/producer generation
- CloudEvents handler generation

### Phase 5: CLI Integration (1 week)
- Add `generate` command to `spikard-cli`
- Watch mode for development
- IDE integration (LSP)

## References

- [OpenAPI 3.1 Specification](https://spec.openapis.org/oas/v3.1.0)
- [CloudEvents Specification](https://cloudevents.io/)
- [Protocol Buffers](https://protobuf.dev/)
- [AsyncAPI Specification](https://www.asyncapi.com/)

## Key Takeaway

A unified configuration format enables declarative definition of multi-protocol services, with code generation producing idiomatic, type-safe implementations across Python, TypeScript, and Rust while integrating seamlessly with OpenAPI and Protobuf ecosystems.

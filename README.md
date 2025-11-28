# Spikard

[![Documentation](https://img.shields.io/badge/docs-spikard.dev-58FBDA)](https://spikard.dev)
[![Discord](https://img.shields.io/badge/Discord-Join%20our%20community-7289da)](https://discord.gg/pXxagNK2zN)
[![PyPI](https://badge.fury.io/py/spikard.svg)](https://badge.fury.io/py/spikard)
[![npm](https://img.shields.io/npm/v/spikard)](https://www.npmjs.com/package/spikard)
[![npm (WASM)](https://img.shields.io/npm/v/spikard-wasm?label=npm%20%28wasm%29)](https://www.npmjs.com/package/spikard-wasm)
[![RubyGems](https://badge.fury.io/rb/spikard.svg)](https://rubygems.org/gems/spikard)
[![Packagist](https://img.shields.io/packagist/v/spikard/spikard)](https://packagist.org/packages/spikard/spikard)
[![Crates.io](https://img.shields.io/crates/v/spikard)](https://crates.io/crates/spikard)
[![CI Status](https://github.com/Goldziher/spikard/workflows/CI/badge.svg)](https://github.com/Goldziher/spikard/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**An experimental polyglot web development framework.**

Spikard is a high-performance API toolkit built in Rust with bindings for Python, TypeScript, Ruby, PHP, and WebAssembly. Write REST APIs, JSON-RPC services, or Protobuf-based applications in the language you prefer—all sharing the same runtime, middleware, and validation engine.

## Why?

**The need for a single, high-performance toolbox across languages.**

Working across multiple language ecosystems means learning different frameworks, different conventions, and different performance characteristics for each. Spikard provides one consistent API experience—whether you're writing Python for ML pipelines, TypeScript for frontends, Ruby for Rails integration, or Rust for maximum performance.

Same middleware. Same validation. Same correctness guarantees. Different languages.

## What Spikard Is (and Isn't)

**Spikard IS:**
- A batteries-included HTTP/API toolkit
- High-performance routing, middleware, and validation
- Protocol-agnostic (REST, JSON-RPC, Protobuf, GraphQL)
- Polyglot with consistent APIs across languages
- Built for microservices, APIs, and real-time services

**Spikard IS NOT:**
- A full-stack MVC framework (not Django, Rails, Laravel)
- A database ORM or query builder
- A template engine or view layer
- An admin interface or CMS

**You bring your own:**
- Database library (SQLAlchemy, Prisma, Sequel, Diesel, etc.)
- Template engine if needed (Jinja2, EJS, ERB)
- Frontend framework

Spikard focuses on being the best HTTP/API layer—everything from the socket to your handler. Database, templates, and full-stack concerns are intentionally out of scope.

## Language Support

### Current Bindings

- **Python** - Decorators, async/await, msgspec/Pydantic validation ([README](packages/python/README.md))
- **TypeScript** - Node.js/Bun (native) and Deno/Edge (WASM) ([README](packages/node/README.md))
- **Ruby** - Block-friendly routing, idiomatic patterns ([README](packages/ruby/README.md))
- **PHP** - PSR-compliant, background tasks, streaming, WebSocket/SSE support ([README](packages/php/README.md))
- **Rust** - Zero-cost native performance ([README](crates/spikard/README.md))

### Future Bindings

**Planned:**
- **Go** (exploring FFI options)
- **C#**, **Java/Kotlin**, **Elixir** (exploring)

We're open to additional language bindings—both FFI-based (like potential Go support) and native binding approaches. Community contributions welcome.

### Feature Parity Across Languages

All language bindings share the same core features through the Rust runtime:

| Feature | Python | TypeScript | Ruby | PHP | Rust |
|---------|--------|------------|------|-----|------|
| **HTTP/REST** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Path Parameters** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **JSON Validation** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Background Tasks** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Dependency Injection** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Streaming** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **WebSocket** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Server-Sent Events** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **AsyncAPI Support** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **OpenAPI Codegen** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Lifecycle Hooks** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Middleware Stack** | ✅ | ✅ | ✅ | ✅ | ✅ |

All features are implemented once in Rust and exposed through thin, idiomatic bindings for each language.

## Design Principles

**Correctness First**
- Follow web standards and IETF drafts (RFC 9457, JSON Schema Draft 2020-12, OpenAPI 3.1)
- Developed using TDD and Benchmark-Driven Development
- 400+ fixture-driven test scenarios ensure identical behavior across all languages

**Simplicity**
- Familiar APIs designed to feel natural in each language
- No reinventing concepts—leverage each ecosystem's best practices

**Strong Validation**
- JSON Schema validation for all requests and responses
- Strict typing enforced even in dynamic languages
- Native integration with language-specific validators

**Build on Open Source**
- Don't reinvent the wheel—use the best from each ecosystem
- Rust: axum, tower, tower-http, jsonschema
- Python: msgspec (required), PyO3 (with optional pydantic/attrs/dataclasses detection)
- TypeScript: Zod, napi-rs
- Ruby: Magnus FFI, RBS types (with optional dry-schema/dry-struct detection)

## Quick Examples

### Python
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

@app.post("/users")
async def create_user(user: User) -> User:
    # Automatic validation via msgspec
    return user

if __name__ == "__main__":
    app.run(port=8000)
```

### TypeScript (Node/Bun)
```typescript
import { Spikard, type Request } from "spikard";
import { z } from "zod";

const UserSchema = z.object({
  id: z.number(),
  name: z.string(),
});
type User = z.infer<typeof UserSchema>;

const app = new Spikard();

const getUser = async (req: Request): Promise<User> => {
  const id = Number(req.params["id"] ?? 0);
  return { id, name: "Alice" };
};

const createUser = async (req: Request): Promise<User> => {
  return UserSchema.parse(req.json());
};

app.addRoute(
  { method: "GET", path: "/users/:id", handler_name: "getUser", is_async: true },
  getUser,
);

app.addRoute(
  {
    method: "POST",
    path: "/users",
    handler_name: "createUser",
    request_schema: UserSchema,
    response_schema: UserSchema,
    is_async: true,
  },
  createUser,
);

app.run({ port: 8000 });
```

### Ruby
```ruby
require 'spikard'

app = Spikard::App.new

app.get('/users/:id') do |request|
  user_id = request[:path_params]["id"].to_i
  { id: user_id, name: 'Alice' }
end

app.post('/users') do |request|
  # request[:body] is validated when a schema is provided
  request[:body]
end

app.run(port: 8000)
```

### PHP
```php
<?php
use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Spikard\Background\BackgroundTask;

$app = new App(new ServerConfig(port: 8000));

// Route with path parameter
$app = $app->addRoute('GET', '/users/{id}', function (Request $req) {
    $id = (int)$req->pathParams['id'];
    return Response::json(['id' => $id, 'name' => 'Alice']);
});

// JSON body with validation
$app = $app->addRoute('POST', '/users', function (Request $req) {
    $user = $req->body;

    // Run background task (e.g., send email)
    BackgroundTask::run(function () use ($user) {
        sendWelcomeEmail($user);
    });

    return Response::json($user, 201);
});

// Streaming response (SSE)
$app = $app->addRoute('GET', '/events', function () {
    $generator = function (): Generator {
        for ($i = 0; $i < 10; $i++) {
            yield "data: " . json_encode(['count' => $i]) . "\n\n";
            sleep(1);
        }
    };
    return \Spikard\Http\StreamingResponse::sse($generator());
});

$app->run();
```

### Rust
```rust
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use spikard::prelude::*;

#[derive(Serialize, Deserialize, JsonSchema)]
struct User {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    app.route(get("/users/:id"), |ctx: Context| async move {
        let id: i32 = ctx
            .path_param("id")
            .unwrap_or("0")
            .parse()
            .unwrap_or_default();
        Ok(Json(User { id, name: "Alice".to_string() }))
    })?;

    app.route(
        post("/users").request_body::<User>().response_body::<User>(),
        |ctx: Context| async move {
            let user: User = ctx.json()?;
            Ok(Json(user))
        },
    )?;

    app.run().await?;
    Ok(())
}
```

## Key Features

### HTTP & Protocols

**Current:**
- REST with typed routing (`/users/{id:uuid}`)
- HTTP/1.1 and HTTP/2
- Request/response streaming
- Server-Sent Events (SSE)
- WebSockets
- Multipart file uploads
- URL-encoded and JSON bodies

**Coming Soon:**
- JSON-RPC
- Protobuf with protoc support
- GraphQL (queries, mutations, subscriptions)
- CloudEvents *(post v1.0)*
- Queue protocols (AMQP, Kafka, etc.) *(post v1.0)*
- HTTP/3 (QUIC) *(post v1.0)*

### Middleware Stack (Tower-HTTP)

All middleware is implemented in Rust and exposed through language bindings:

- **Compression** - gzip, brotli with configurable quality
- **Rate Limiting** - Token bucket, per-IP or global
- **Request Timeouts** - Configurable per-route or global
- **Body Size Limits** - Prevent abuse
- **Request ID** - UUID injection with custom headers
- **CORS** - RFC 6454 compliant
- **Authentication** - JWT and API Key support
- **Static Files** - ETag, Last-Modified, precompressed assets

### Validation & Type Safety

**JSON Schema:**
- Draft 2020-12 support
- Automatic type coercion
- Comprehensive error messages (RFC 9457 Problem Details)

**Language-Specific Power Tools:**

Spikard integrates with native validation libraries in each language:

- **Python**: msgspec (required - zero-copy, fastest), with automatic detection of Pydantic v2, attrs, and dataclasses
- **TypeScript**: Zod (type inference and runtime validation)
- **Ruby**: Automatic detection of dry-schema, dry-struct when present
- **PHP**: Native array/object validation with PSR-7 HTTP message interfaces
- **Rust**: serde with schemars for JSON Schema generation

All validation approaches compile to the same JSON Schema contracts, ensuring cross-language consistency.

### Lifecycle Hooks

Fine-grained control over request processing:

- `onRequest` - Before any processing
- `preValidation` - Before schema validation
- `preHandler` - After validation, before handler
- `onResponse` - Before sending response
- `onError` - Error handling

**Features:**
- Zero-cost when not used (`Option<Arc<dyn Fn>>`)
- Can short-circuit the pipeline
- Full async support

### Dependency Injection

- Container-based DI for services and dependencies
- Scoped injection (request cache, singleton, per-call)
- Language-idiomatic integration (Python `Provide`, TypeScript `provide`, Ruby `provide` with keyword deps, PHP `DependencyContainer`, Rust `ServerConfig::provide_*`)

## Code Generation & CLI

Code generation is a **first-class citizen** in Spikard.

### Supported Specs

**OpenAPI 3.1/3.0:**
```bash
spikard generate openapi --spec api.yaml --lang python --output ./app
```
Generates: REST routes, request/response types, validation, documentation

**AsyncAPI 2.x/3.x:**
```bash
spikard generate asyncapi --spec events.yaml --lang typescript --output ./app
```
Generates: SSE producers, WebSocket handlers, event schemas, documentation

**Future Protocols:**
- JSON-RPC spec → RPC handlers
- Protobuf (.proto files) → gRPC services
- GraphQL SDL → resolvers and types
- CloudEvents → event handlers *(post v1.0)*

### Documentation Generation

All generated code includes:
- OpenAPI specs with complete schemas
- Markdown documentation
- Interactive API docs (Swagger UI, ReDoc)
- Code examples in all supported languages

### AI-First Approach

**Planned:**
- MCP (Model Context Protocol) server for AI assistants
- Official Claude skills with up-to-date examples
- Context-aware code completion
- Automatic fixture generation from natural language

The CLI is designed to be AI-friendly: structured output, consistent patterns, and rich context for code generation tools.

## Performance

Benchmarks on Apple M4 Pro (14-core, 48GB RAM) with 100 concurrent connections:

### Rust vs Python Bindings
| Binding | Avg Throughput | Mean Latency | P99 Latency |
|---------|----------------|--------------|-------------|
| **Rust** | 165,454 req/s | 0.60ms | 1.53ms |
| **Python** | 35,779 req/s | 7.44ms | - |

**Key findings:**
- Rust ~4.6x faster than Python, but Python still delivers 35k+ req/s
- Performance stable across payload sizes (86 bytes to 150KB)
- Low memory usage (~27MB for both)
- Zero-copy serialization (serde for Rust, msgspec for Python)

Node.js, Ruby, and WASM benchmarks coming soon.

## Status: 0.1 - Experimental

**What works:**
- ✅ Core HTTP server with full middleware stack
- ✅ Python, TypeScript, Ruby, PHP, WASM, Rust bindings
- ✅ JSON validation with JSON Schema
- ✅ Code generation from OpenAPI and AsyncAPI
- ✅ Streaming, SSE, WebSockets
- ✅ Background tasks and dependency injection (all bindings)
- ✅ 400+ fixtures with comprehensive test coverage
- ✅ Benchmark harness for performance testing

**Expectations:**
- This is an **initial experimental release** (0.1)
- APIs will change as we gather feedback
- **Not production-ready**—use for evaluation only
- Breaking changes expected before v1.0

## Roadmap

### Core Features
- [x] HTTP/1.1 and HTTP/2 support
- [x] REST routing with typed parameters
- [x] JSON Schema validation
- [x] Middleware stack (compression, rate limiting, CORS, auth, etc.)
- [x] Streaming, SSE, WebSockets
- [x] Lifecycle hooks
- [x] OpenAPI 3.1 code generation
- [x] AsyncAPI code generation
- [x] Documentation generation
- [x] **Dependency Injection**
- [ ] JSON-RPC protocol support
- [ ] Protobuf with protoc integration
- [ ] GraphQL support (queries, mutations, subscriptions)
- [ ] HTTP/3 (QUIC) support *(post v1.0)*
- [ ] CloudEvents support *(post v1.0)*
- [ ] Queue protocols (AMQP, Kafka) *(post v1.0)*
- [ ] Plugin/extension system

### Language Bindings
- [x] Python (PyO3 + msgspec/Pydantic)
- [x] TypeScript/Node.js (napi-rs + Zod/ArkType/Valibot)
- [x] Ruby (Magnus + DrySchema)
- [x] PHP (ext-php-rs + PSR standards)
- [x] WebAssembly (wasm-bindgen)
- [x] Rust (native)
- [ ] Go (Maybe?)
- [ ] C# (Maybe?)
- [ ] Java/Kotlin (Maybe?)
- [ ] Elixir (Maybe?)

### Validation Libraries Integration
- [x] Python: msgspec (required), with auto-detection of Pydantic v2, attrs, dataclasses
- [x] TypeScript: Zod
- [x] Ruby: Auto-detection of dry-schema, dry-struct
- [ ] TypeScript: ArkType, Valibot, TypeBox, Effect
- [ ] Ruby: dry-validation, ActiveModel

### Developer Experience
- [x] Fixture-driven testing (400+ scenarios)
- [x] CLI for code generation
- [x] OpenAPI and AsyncAPI support
- [x] Documentation generation
- [ ] MCP server (as part of the CLI)
- [ ] Official Claude skills
- [ ] Comprehensive documentation site
- [ ] Example applications library

### Performance
- [x] Benchmark harness (profiling support builtin)
- [x] Zero-copy serialization paths

### Stability
- [ ] API stability guarantees

## Getting Started

### Installation

**Language-specific packages:**

#### Python
```bash
pip install spikard
```
See [Python README](packages/python/README.md) for more details.

#### Node.js / TypeScript
```bash
npm install spikard
# or with pnpm
pnpm add spikard
```
See [Node.js README](packages/node/README.md) for more details.

#### Ruby
```bash
gem install spikard
```
See [Ruby README](packages/ruby/README.md) for more details.

#### PHP
```bash
composer require spikard/spikard
```
See [PHP README](packages/php/README.md) for more details.

#### WebAssembly
```bash
npm install spikard-wasm
# or
pnpm add spikard-wasm
```
See [WASM README](crates/spikard-wasm/README.md) for more details.

#### Rust (native)
```bash
cargo add spikard
```
See [Rust README](crates/spikard/README.md) for more details.

### From Source

**Requirements:**
- Rust 1.80+ (`rustup`)
- Python 3.11+ with `uv` or `pip`
- Node.js 20+ with `pnpm`
- Ruby 3.2+ with `bundler`
- PHP 8.2+ with `composer`

**Quick Start:**
```bash
# Clone repository
git clone https://github.com/Goldziher/spikard.git
cd spikard

# Install dependencies
pnpm install
bundle install
uv sync
composer install

# Build bindings
task build:py      # Python
task build:node    # Node.js
task build:ruby    # Ruby
task build:php     # PHP
task build:wasm    # WebAssembly
task build:rust    # Rust

# Run tests
task test

# Try an example
cd examples/python
python server.py
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed development setup.

## Documentation

- **Language-Specific READMEs:**
  - [Python](packages/python/README.md)
  - [TypeScript/Node.js](packages/node/README.md)
  - [Ruby](packages/ruby/README.md)
  - [PHP](packages/php/README.md)
  - [Rust](crates/spikard/README.md)

- **Architecture Decision Records (ADRs):** [docs/adr/](docs/adr/)
  - Design decisions, middleware architecture, validation strategy

- **Contributing:** [CONTRIBUTING.md](CONTRIBUTING.md)
  - Development setup, testing, benchmarking

- **Comprehensive Documentation:** *(Coming soon)*

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for:
- Development workflow
- Testing and fixture-driven development
- Benchmarking methodology
- Code standards
- How to add language bindings

**Ways to help:**
- Try it out and report issues
- Share benchmark results
- Contribute language bindings
- Add example applications
- Improve documentation

## License

MIT License - see the [LICENSE](LICENSE) file for details.

Copyright (c) 2025 Na'aman Hirschfeld

## Acknowledgments

### Inspirations

**Rust:**
- **Axum** - Type-safe extractors, ergonomic routing, and Tower middleware integration
- **Tower** - Composable middleware and service abstractions

**Python:**
- **Litestar** - Declarative API design, lifecycle hooks, and dependency injection patterns
- **FastAPI** - Common conventions, OpenAPI integration, and developer experience
- **Robyn** - Rust-Python binding techniques and async bridge design

**TypeScript/Node:**
- **Fastify** - Opt-in performance philosophy and plugin architecture
- **Hono** - Clean routing API and edge runtime support

**Ruby:**
- **Roda** - Routing tree design and minimal overhead approach
- **Hanami** - Component architecture and clean abstractions
- **Rack** - Middleware pipeline and HTTP abstraction

---

**Spikard** - One toolbox. Many languages.

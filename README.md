# Spikard

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <!-- Language Bindings -->
  <a href="https://crates.io/crates/spikard">
    <img src="https://img.shields.io/crates/v/spikard?label=Rust&color=007ec6" alt="Rust">
  </a>
  <a href="https://pypi.org/project/spikard/">
    <img src="https://img.shields.io/pypi/v/spikard?label=Python&color=007ec6" alt="Python">
  </a>
  <a href="https://www.npmjs.com/package/@spikard/node">
    <img src="https://img.shields.io/npm/v/@spikard/node?label=Node.js&color=007ec6" alt="Node.js">
  </a>
  <a href="https://www.npmjs.com/package/@spikard/wasm">
    <img src="https://img.shields.io/npm/v/@spikard/wasm?label=WASM&color=007ec6" alt="WASM">
  </a>
  <a href="https://rubygems.org/gems/spikard">
    <img src="https://img.shields.io/gem/v/spikard?label=Ruby&color=007ec6" alt="Ruby">
  </a>
  <a href="https://packagist.org/packages/spikard/spikard">
    <img src="https://img.shields.io/packagist/v/spikard/spikard?label=PHP&color=007ec6" alt="PHP">
  </a>
  <a href="https://hex.pm/packages/spikard">
    <img src="https://img.shields.io/hexpm/v/spikard?label=Elixir&color=007ec6" alt="Elixir">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.spikard/spikard">
    <img src="https://img.shields.io/maven-central/v/dev.spikard/spikard?label=Java&color=007ec6" alt="Java">
  </a>
  <a href="https://github.com/Goldziher/spikard/releases">
    <img src="https://img.shields.io/github/v/tag/Goldziher/spikard?label=Go&color=007ec6" alt="Go">
  </a>
  <a href="https://www.nuget.org/packages/Spikard/">
    <img src="https://img.shields.io/nuget/v/Spikard?label=C%23&color=007ec6" alt="C#">
  </a>

  <!-- Project Info -->
  <a href="https://github.com/Goldziher/spikard/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-007ec6" alt="License">
  </a>
  <a href="https://github.com/Goldziher/spikard">
    <img src="https://img.shields.io/badge/docs-GitHub-007ec6" alt="Documentation">
  </a>
</div>

Rust-centric polyglot HTTP framework with type-safe routing, OpenAPI/AsyncAPI/GraphQL/JSON-RPC codegen, tower-http middleware, and fixture-driven cross-language testing. Single Rust core compiled to 15 languages via alef-generated bindings.

**Powered by Rust.** Native performance for HTTP routing, validation, and middleware. Write once, bind everywhere.

## Key Features

- **Type-safe across all bindings** – HTTP routing with path, query, body, and header validation. Errors convert losslessly between languages.
- **Polyglot bindings** – 15 languages: Python, TypeScript/Node, Ruby, PHP, Elixir, Go, Java, C#, Kotlin, Dart, Gleam, Swift, Zig, WASM, C FFI. All generated from Rust API surface via alef.
- **Fixture-driven testing** – Shared JSON fixtures drive tests across all language bindings for behavioral consistency.
- **Schema codegen** – Parse OpenAPI 3.0, AsyncAPI 3.0, GraphQL SDL, and JSON-RPC 2.0 specs. Generate type-safe handlers and validators per binding.
- **Tower middleware** – Compression (gzip/brotli), rate limiting, timeouts, request IDs, authentication (JWT/API key), static file serving.
- **Lifecycle hooks** – `onRequest`, `preValidation`, `preHandler`, `onResponse`, `onError`. Execute in order with zero overhead when unregistered.
- **WebSocket, SSE, background tasks** – Real-time bidirectional and server-sent event streams. Fire-and-forget background job support.
- **CLI and MCP server** – Initialize projects, generate code, validate schemas. MCP-compatible for IDE integration.

## Installation

Each language binding is available through its native package manager. Full installation guides with system requirements are provided per binding:

**Scripting Languages:**

- **[Python](packages/python/README.md)** – PyPI/spikard, async/await support, type stubs
- **[Ruby](packages/ruby/README.md)** – RubyGems/spikard, idiomatic Ruby API
- **[PHP](packages/php/README.md)** – Composer/spikard, PHP 8.2+ native extension
- **[Elixir](packages/elixir/README.md)** – Hex/spikard, OTP integration via NIFs

**JavaScript/TypeScript:**

- **[Node.js](packages/node/README.md)** – npm/@spikard/node, NAPI-RS native bindings
- **[WASM](packages/typescript/README.md)** – npm/@spikard/wasm, browser/Deno/Cloudflare Workers

**Compiled Languages:**

- **[Go](packages/go/README.md)** – go.mod github.com/Goldziher/spikard/packages/go, FFI bindings
- **[Java](packages/java/README.md)** – Maven Central/dev.spikard:spikard, Foreign Function & Memory API
- **[C#](packages/csharp/README.md)** – NuGet/Spikard, .NET 6.0+ with async/await

**Native:**

- **[Rust](crates/spikard/README.md)** – crates.io/spikard, core library, feature flags
- **[C (FFI)](crates/spikard-ffi/README.md)** – cbindgen headers + shared library, pkg-config/CMake support

**Additional Targets:**

- **[Kotlin](packages/kotlin/README.md)** – Maven Central/dev.spikard:spikard
- **[Dart](packages/dart/README.md)** – pub.dev/spikard
- **[Gleam](packages/gleam/README.md)** – Hex/spikard_gleam
- **[Swift](packages/swift/README.md)** – SwiftPM or Cocoapods
- **[Zig](packages/zig/README.md)** – build.zig.zon dependency

## Quick Start

**Python:**

```python
from spikard import Spikard
from spikard.config import ServerConfig
from msgspec import Struct

class User(Struct):
    id: int
    name: str

app = Spikard()

@app.get("/users/{id:int}")
async def get_user(id: int) -> User:
    return User(id=id, name="Alice")

@app.post("/users")
async def create_user(user: User) -> User:
    return user

if __name__ == "__main__":
    app.run(config=ServerConfig(port=8000))
```

**TypeScript/Node.js:**

```typescript
import { Spikard, ServerConfig } from '@spikard/node';

interface User {
  id: number;
  name: string;
}

const app = new Spikard();

app.get("/users/{id:int}", async (id: number): Promise<User> => {
  return { id, name: "Alice" };
});

app.post("/users", async (user: User): Promise<User> => {
  return user;
});

const config = new ServerConfig({ port: 8000 });
app.run(config);
```

## Architecture

All language bindings call through a thin FFI to the shared Rust core:

```text
┌─────────────────────────────────────────────────────┐
│ Language Bindings (Python, Ruby, Node, Go, Java ...) │
│ Type conversion, async runtime integration         │
└────────────────┬────────────────────────────────────┘
                 │ FFI / NAPI / PyO3 / Magnus
                 ↓
    ┌────────────────────────────────┐
    │ crates/spikard-http            │
    │ Router, middleware, auth       │
    └────────────────┬───────────────┘
                     │
    ┌────────────────↓───────────────┐
    │ crates/spikard-core            │
    │ HTTP types, validation, errors │
    └────────────────┬───────────────┘
                     │
    ┌────────────────↓───────────────┐
    │ crates/spikard-codegen         │
    │ OpenAPI, GraphQL, AsyncAPI     │
    └────────────────────────────────┘
```

Bindings are auto-generated from the Rust API surface via [alef](https://github.com/kreuzberg-dev/alef). No hand-written FFI glue — just type and error conversion at language boundaries.

## Development

Install Rust 1.x and Node 18+ (for running alef). Use the `task` command runner:

```bash
# Setup: install dependencies + alef + pre-commit hooks
task setup

# Build: compile Rust core + all bindings
task build

# Test: run all test suites (Rust core + language-specific)
task test

# E2E: generate + run cross-language tests from fixtures
task test:e2e

# Lint: run clippy, ruff, biome, rubocop, phpstan, etc. via pre-commit
task lint

# Format: run cargo fmt + alef format
task format
```

Full task list: `task --list`

## Project Status

**Phase 1-2 Complete:**

- Rust core (HTTP router, middleware, validation, auth)
- Polyglot bindings for 15 languages via alef code generation
- OpenAPI 3.0, AsyncAPI 3.0, GraphQL SDL, JSON-RPC 2.0 codegen

**E2E Testing In Progress:**

- Rust and Node.js: 410 fixtures, all green
- Python, Ruby, PHP, Elixir, Go, Java, C#: generators ready, tests being filled in
- All tests powered by shared JSON fixtures for consistent behavior across languages

**Stability:** Rust core is stable (0.14.0). Language bindings follow Rust versioning; pre-release versions are in active development pending full e2e coverage.

## Contributing

Contributions welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines — start with issues marked `good-first-issue` or `help-wanted`.

Code must pass `task lint` + `task test` before submission. No modifications to generated code under `e2e/`; instead, update fixtures or generator source and regenerate.

## License

MIT License — see [LICENSE](LICENSE) for details.

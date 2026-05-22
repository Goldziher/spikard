# Spikard

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <!-- Built with -->
  <a href="https://github.com/kreuzberg-dev/alef">
    <img src="https://img.shields.io/badge/Bindings-alef%20%D7%90-007ec6" alt="Bindings">
  </a>

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
  <a href="https://www.npmjs.com/package/@spikard/node-wasm">
    <img src="https://img.shields.io/npm/v/@spikard/node-wasm?label=WASM&color=007ec6" alt="WASM">
  </a>
  <a href="https://rubygems.org/gems/spikard">
    <img src="https://img.shields.io/gem/v/spikard?label=Ruby&color=007ec6" alt="Ruby">
  </a>
  <a href="https://packagist.org/packages/goldziher/spikard">
    <img src="https://img.shields.io/packagist/v/goldziher/spikard?label=PHP&color=007ec6" alt="PHP">
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
  <a href="https://central.sonatype.com/artifact/dev.spikard/spikard">
    <img src="https://img.shields.io/maven-central/v/dev.spikard/spikard?label=Kotlin&color=007ec6" alt="Kotlin">
  </a>
  <a href="https://pub.dev/packages/spikard">
    <img src="https://img.shields.io/pub/v/spikard?label=Dart&color=007ec6" alt="Dart">
  </a>
  <a href="https://github.com/Goldziher/spikard/tree/main/packages/swift">
    <img src="https://img.shields.io/badge/Swift-Spikard-007ec6" alt="Swift">
  </a>
  <a href="https://github.com/Goldziher/spikard/tree/main/packages/zig">
    <img src="https://img.shields.io/badge/Zig-spikard-007ec6" alt="Zig">
  </a>
  <a href="https://github.com/Goldziher/spikard/tree/main/crates/spikard-ffi">
    <img src="https://img.shields.io/badge/C-FFI-007ec6" alt="C FFI">
  </a>
  <a href="https://github.com/Goldziher/homebrew-tap">
    <img src="https://img.shields.io/badge/Homebrew-007ec6?logo=homebrew&logoColor=white" alt="Homebrew">
  </a>

  <!-- Project Info -->
  <a href="https://github.com/Goldziher/spikard/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-007ec6" alt="License">
  </a>
  <a href="https://github.com/Goldziher/spikard/tree/main/docs">
    <img src="https://img.shields.io/badge/Docs-spikard-007ec6" alt="Documentation">
  </a>
</div>

<div align="center" style="display: flex; flex-wrap: wrap; gap: 12px; justify-content: center; margin: 28px 0 24px;">
  <a href="https://discord.gg/pXxagNK2zN">
    <img height="22" src="https://img.shields.io/badge/Discord-Chat-007ec6?logo=discord&logoColor=white" alt="Join Discord">
  </a>
</div>

Rust-centric polyglot HTTP framework with type-safe routing, OpenAPI/AsyncAPI/GraphQL/JSON-RPC codegen, tower-http middleware, and fixture-driven cross-language testing. Single Rust core compiled to 15 languages through alef-generated bindings.

**Powered by Rust.** Native performance for HTTP routing, validation, and middleware. Write once, bind everywhere.

## Key Features

- **Type-safe across bindings** - HTTP routing with path, query, body, and header validation. Errors convert losslessly between languages.
- **Polyglot bindings** - Python, TypeScript/Node, Ruby, PHP, Elixir, Go, Java, C#, Kotlin, Dart, Swift, Zig, WASM, Rust, and C FFI.
- **Fixture-driven testing** - shared JSON fixtures drive tests across language bindings for behavioral consistency.
- **Schema codegen** - parse OpenAPI 3.0, AsyncAPI 3.0, GraphQL SDL, and JSON-RPC 2.0 specs. Generate handlers and validators per binding.
- **SQL to HTTP codegen** - annotate SQL queries with `@http GET /path`, `@http_auth bearer:jwt`, and emit route metadata, OpenAPI 3.1 specs, and sidecars.
- **Tower middleware** - compression, rate limiting, timeouts, request IDs, authentication, and static file serving.
- **Lifecycle hooks** - `onRequest`, `preValidation`, `preHandler`, `onResponse`, and `onError`.
- **WebSocket, SSE, background tasks** - bidirectional and server-sent streams plus fire-and-forget background jobs.
- **CLI and MCP server** - initialize projects, generate code, validate schemas, and integrate with MCP-compatible tools.

## Installation

Each binding ships through its native package manager. Use the package README for platform requirements and build details.

| Target | Package | README |
| ------ | ------- | ------ |
| Rust | `spikard` on crates.io | [crates/spikard](crates/spikard/README.md) |
| Python | `spikard` on PyPI | [packages/python](packages/python/README.md) |
| Node.js | `@spikard/node` on npm | [crates/spikard-node](crates/spikard-node/README.md) |
| WASM | `@spikard/node-wasm` on npm | [crates/spikard-wasm](crates/spikard-wasm/README.md) |
| Ruby | `spikard` on RubyGems | [packages/ruby](packages/ruby/README.md) |
| PHP | `goldziher/spikard` on Packagist | [packages/php](packages/php/README.md) |
| Elixir | `spikard` on Hex | [packages/elixir](packages/elixir/README.md) |
| Go | `github.com/Goldziher/spikard` | [packages/go](packages/go/README.md) |
| Java | `dev.spikard:spikard` on Maven Central | [packages/java](packages/java/README.md) |
| C# | `Spikard` on NuGet | [packages/csharp](packages/csharp/README.md) |
| Kotlin | `dev.spikard:spikard` on Maven Central | [packages/kotlin](packages/kotlin/README.md) |
| Dart | `spikard` on pub.dev | [packages/dart](packages/dart/README.md) |
| Swift | `Spikard` through SwiftPM | [packages/swift](packages/swift/README.md) |
| Zig | `spikard` through `build.zig.zon` | [packages/zig](packages/zig/README.md) |
| C FFI | `spikard-ffi` shared/static library | [crates/spikard-ffi](crates/spikard-ffi/README.md) |

## Quick Start

### Python

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

if __name__ == "__main__":
    app.run(config=ServerConfig(port=8000))
```

### TypeScript

```typescript
import { Spikard, ServerConfig } from "@spikard/node";

const app = new Spikard();

app.get("/users/{id:int}", async (id: number) => {
  return { id, name: "Alice" };
});

const config = new ServerConfig({ port: 8000 });
app.run(config);
```

## Architecture

All bindings call a shared Rust core through thin language-native layers:

```text
Language bindings (Python, Node, Ruby, Go, Java, C#...)
        |
        v
FFI / NAPI / PyO3 / Magnus / runtime bridge
        |
        v
crates/spikard-http      Router, middleware, auth
crates/spikard-core      HTTP types, validation, errors
crates/spikard-codegen   OpenAPI, GraphQL, AsyncAPI, JSON-RPC
```

Bindings are generated from the Rust API surface via [alef](https://github.com/kreuzberg-dev/alef). Binding code stays thin: type conversion, error conversion, and runtime integration.

## Development

```bash
task setup
task build
task test
task test:e2e
task lint
task format
```

Run `task --list` for the full task catalog.

## Project Status

- Rust core is stable.
- Binding packages follow the Rust crate version.
- E2E coverage is fixture-driven and shared across supported language targets.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Generated binding and e2e files should be changed through Rust source, fixtures, templates, or `alef.toml`.

## License

MIT License - see [LICENSE](LICENSE) for details.

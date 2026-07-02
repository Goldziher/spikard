<!-- markdownlint-disable MD033 MD041 -->
<div align="center">

<img src="https://raw.githubusercontent.com/Goldziher/spikard/main/docs/assets/spikard-banner.svg" alt="spikard - polyglot web toolkit" width="820">

**One web toolkit, every language.**

Rust-centric polyglot web toolkit. Single Rust core compiled to 15 languages through alef-generated bindings — type-safe routing, OpenAPI/AsyncAPI/GraphQL/JSON-RPC codegen, tower-http middleware, and fixture-driven cross-language testing.

**Rust core** · Python · TypeScript · Ruby · PHP · Elixir · Go · Java · C# · Kotlin · Dart · Swift · Zig · WASM · C FFI

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <!-- Built with alef -->
  <a href="https://github.com/xberg-io/alef">
    <img src="https://img.shields.io/badge/built%20with-alef%20%D7%90-007ec6?style=flat-square" alt="Built with alef">
  </a>

  <!-- Language bindings -->
  <a href="https://crates.io/crates/spikard">
    <img src="https://img.shields.io/crates/v/spikard?style=flat-square" alt="Rust">
  </a>
  <a href="https://pypi.org/project/spikard/">
    <img src="https://img.shields.io/pypi/v/spikard?style=flat-square" alt="Python">
  </a>
  <a href="https://www.npmjs.com/package/@spikard/node">
    <img src="https://img.shields.io/npm/v/@spikard/node?style=flat-square" alt="Node.js">
  </a>
  <a href="https://www.npmjs.com/package/@spikard/node-wasm">
    <img src="https://img.shields.io/npm/v/@spikard/node-wasm?style=flat-square" alt="WASM">
  </a>
  <a href="https://rubygems.org/gems/spikard">
    <img src="https://img.shields.io/gem/v/spikard?style=flat-square" alt="Ruby">
  </a>
  <a href="https://packagist.org/packages/goldziher/spikard">
    <img src="https://img.shields.io/packagist/v/goldziher/spikard?style=flat-square" alt="PHP">
  </a>
  <a href="https://hex.pm/packages/spikard">
    <img src="https://img.shields.io/hexpm/v/spikard?style=flat-square" alt="Elixir">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.spikard/spikard">
    <img src="https://img.shields.io/maven-central/v/dev.spikard/spikard?style=flat-square" alt="Java">
  </a>
  <a href="https://github.com/Goldziher/spikard/releases">
    <img src="https://img.shields.io/github/v/tag/Goldziher/spikard?label=Go&style=flat-square" alt="Go">
  </a>
  <a href="https://www.nuget.org/packages/Spikard/">
    <img src="https://img.shields.io/nuget/v/Spikard?style=flat-square" alt="C#">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.spikard/spikard">
    <img src="https://img.shields.io/maven-central/v/dev.spikard/spikard?label=Kotlin&style=flat-square" alt="Kotlin">
  </a>
  <a href="https://pub.dev/packages/spikard">
    <img src="https://img.shields.io/pub/v/spikard?style=flat-square" alt="Dart">
  </a>
  <a href="https://github.com/Goldziher/spikard/tree/main/packages/swift">
    <img src="https://img.shields.io/badge/Swift-Spikard-007ec6?style=flat-square" alt="Swift">
  </a>
  <a href="https://github.com/Goldziher/spikard/tree/main/packages/zig">
    <img src="https://img.shields.io/badge/Zig-spikard-007ec6?style=flat-square" alt="Zig">
  </a>
  <a href="https://github.com/Goldziher/spikard/tree/main/crates/spikard-ffi">
    <img src="https://img.shields.io/badge/C%20FFI-007ec6?style=flat-square" alt="C FFI">
  </a>
  <a href="https://github.com/Goldziher/homebrew-tap">
    <img src="https://img.shields.io/badge/Homebrew-007ec6?style=flat-square&logo=homebrew&logoColor=white" alt="Homebrew">
  </a>

  <!-- Project info -->
  <a href="https://github.com/Goldziher/spikard/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-007ec6?style=flat-square" alt="License">
  </a>
  <a href="https://github.com/Goldziher/spikard">
    <img src="https://img.shields.io/github/stars/Goldziher/spikard?style=flat-square" alt="GitHub Stars">
  </a>
</div>

[Install](#installation) · [Why spikard](#why-spikard) · [Quick example](#quick-example) · [Docs](https://github.com/Goldziher/spikard)

</div>

---

## Why spikard

| Capability | Details |
|---|---|
| **Type-safe across bindings** | HTTP routing with path, query, body, and header validation. Errors convert losslessly between languages. |
| **Polyglot bindings** | Python, TypeScript/Node, Ruby, PHP, Elixir, Go, Java, C#, Kotlin, Dart, Swift, Zig, WASM, Rust, and C FFI. |
| **Schema codegen** | Parse OpenAPI 3.0, AsyncAPI 3.0, GraphQL SDL, and JSON-RPC 2.0 specs. Generate handlers and validators per binding. |
| **SQL to HTTP codegen** | Annotate SQL queries with `@http GET /path`, `@http_auth bearer:jwt`, and emit route metadata and OpenAPI specs. |
| **Tower middleware** | Compression, rate limiting, timeouts, request IDs, JWT/API-key auth, and static file serving. |
| **Lifecycle hooks** | `onRequest`, `preValidation`, `preHandler`, `onResponse`, and `onError`. |
| **WebSocket & SSE** | Bidirectional streams and server-sent events. |
| **Fixture-driven testing** | Shared JSON fixtures drive tests across language bindings for behavioral consistency. |
| **CLI & MCP server** | Initialize projects, generate code, validate schemas, and integrate with MCP-compatible tools. |

## Installation

Each binding ships through its native package manager.

<!-- markdownlint-disable MD013 -->

| Target  | Package                                | Install                                              |
| ------- | -------------------------------------- | ---------------------------------------------------- |
| Rust    | `spikard` on crates.io                 | `cargo add spikard`                                  |
| Python  | `spikard` on PyPI                      | `pip install spikard`                                |
| Node.js | `@spikard/node` on npm                 | `npm install @spikard/node`                          |
| WASM    | `@spikard/node-wasm` on npm            | `npm install @spikard/node-wasm`                     |
| Ruby    | `spikard` on RubyGems                  | `gem install spikard`                                |
| PHP     | `goldziher/spikard` on Packagist       | `composer require goldziher/spikard`                 |
| Elixir  | `spikard` on Hex                       | Add `{:spikard, "~> 0.1"}` to `mix.exs`              |
| Go      | `github.com/Goldziher/spikard`         | `go get github.com/Goldziher/spikard`                |
| Java    | `dev.spikard:spikard` on Maven Central | Maven/Gradle — see [Java README](packages/java)      |
| C#      | `Spikard` on NuGet                     | `dotnet add package Spikard`                         |
| Kotlin  | `dev.spikard:spikard` on Maven Central | Maven/Gradle — see [Kotlin README](packages/kotlin)  |
| Dart    | `spikard` on pub.dev                   | `dart pub add spikard`                               |
| Swift   | `Spikard` via SwiftPM                  | Add to `Package.swift`                               |
| Zig     | `spikard` via `build.zig.zon`          | Add to build manifest                                |
| C FFI   | `spikard-ffi` shared/static library    | [GitHub Releases](https://github.com/Goldziher/spikard/releases) |

<!-- markdownlint-enable MD013 -->

## Quick example

### Python

```python
from spikard import Spikard
from msgspec import Struct

class User(Struct):
    id: int
    name: str

app = Spikard()

@app.get("/users/{id:int}")
async def get_user(id: int) -> User:
    return User(id=id, name="Alice")

if __name__ == "__main__":
    app.run(port=8000)
```

### TypeScript

```typescript
import { Spikard } from "@spikard/node";

const app = new Spikard();

app.get("/users/{id:int}", async (id: number) => {
  return { id, name: "Alice" };
});

app.run({ port: 8000 });
```

<details>
<summary><strong>More examples</strong> (Ruby · PHP · Elixir · Go · Java · C# · Kotlin · Dart · Swift)</summary>

See the [examples directory](https://github.com/Goldziher/spikard/tree/main/crates/spikard-http/examples) in the repository for working examples in every supported language.

</details>

## Architecture

All bindings call a shared Rust core through thin language-native layers:

<details>
<summary><strong>How bindings work</strong></summary>

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

Bindings are generated from the Rust API surface via [alef](https://github.com/xberg-io/alef). Binding code stays thin: type conversion, error conversion, and runtime integration. Business logic, validation, middleware, and codegen all live in Rust.

</details>

<details>
<summary><strong>Specification support</strong></summary>

- **OpenAPI 3.0** — Route definitions to specs, parameter validators, Swagger/ReDoc UI
- **GraphQL** — SDL schema parsing, query execution, introspection, Handler trait integration
- **AsyncAPI 3.0** — Channel/operation extraction, message validators, WebSocket integration
- **OpenRPC** — JSON-RPC 2.0 method handlers, parameter validation, batch requests

</details>

<details>
<summary><strong>Middleware stack</strong></summary>

Compression (gzip/brotli), rate limiting, timeouts, request IDs, authentication (JWT/API key), static files. Configured via ServerConfig structs. All middleware is implemented in Rust via tower-http.

</details>

## Development

```bash
task setup     # Install dependencies
task build     # Build Rust core (debug)
task test      # Run Rust tests
task test:all  # Run all tests (Rust + bindings)
task e2e:all   # Generate + build + run e2e tests
task format    # Format all code
```

Run `task --list` for the full task catalog.

<details>
<summary><strong>Project status</strong></summary>

- Rust core is stable and production-ready.
- Binding packages follow the Rust crate version.
- E2E coverage is fixture-driven and shared across supported language targets.
- See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on modifying generated bindings.

</details>

## License

MIT License — see [LICENSE](LICENSE) for details.

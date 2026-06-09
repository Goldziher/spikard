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

Rust-centric polyglot HTTP framework with OpenAPI/AsyncAPI/GraphQL/JSON-RPC codegen, tower-http middleware, and fixture-driven cross-language testing. Java bindings via JNI/Panama with type-safe API.

## What This Package Provides

- **HTTP application core** — typed routing, request data extraction, validation, lifecycle hooks, and Tower middleware from the Rust engine.
- **Spec-driven work** — OpenAPI, AsyncAPI, GraphQL SDL, JSON-RPC, and SQL-to-HTTP codegen are shared across bindings.
- **Cross-language parity** — generated bindings use the same DTOs, fixtures, and error model, so behavior does not drift between runtimes.
- **Native integration** — no sidecar server required; each package calls the Rust core through its language-native bridge.
- **Java package** — JVM binding for typed routes and generated schemas.

## Installation

**Maven:**

```xml
<dependency>
    <groupId>dev.spikard</groupId>
    <artifactId>spikard</artifactId>
    <version>0.15.6-rc.20</version>
</dependency>
```

**Gradle:**

```gradle
implementation 'dev.spikard:spikard:0.15.6-rc.20'
```

### System Requirements

- **Java 17+** required

## Quick Start

See the [spikard repository](https://github.com/Goldziher/spikard) for usage examples and guides.

## Features

- **HTTP routing** — type-safe route definitions with path, query, and body parameter validation
- **OpenAPI / AsyncAPI / GraphQL / JSON-RPC** — code generation and spec parsing built in
- **Tower middleware** — compression, rate limiting, timeouts, auth (JWT/API key), static files
- **Lifecycle hooks** — `onRequest`, `preValidation`, `preHandler`, `onResponse`, `onError`
- **Fixture-driven testing** — shared JSON fixtures drive tests across all language bindings
- **Polyglot** — single Rust core, thin bindings for Python, Node.js, Ruby, PHP, Elixir, Go, Java, C#, Kotlin, Dart, Gleam, WASM, Swift, Zig, and C FFI

## Documentation

- **[Repository](https://github.com/Goldziher/spikard)** — source code, examples, and contributing guide
- **[Examples](https://github.com/Goldziher/spikard/tree/main/crates/spikard-http/examples)** — working server examples
- **[Issues](https://github.com/Goldziher/spikard/issues)** — bug reports and feature requests

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](https://github.com/Goldziher/spikard/blob/main/CONTRIBUTING.md).

## License

MIT License — see [LICENSE](https://github.com/Goldziher/spikard/blob/main/LICENSE) for details.

<!-- markdownlint-disable MD033 MD041 -->
<div align="center">

<img src="https://raw.githubusercontent.com/Goldziher/spikard/main/docs/assets/spikard-banner.svg" alt="spikard - polyglot web toolkit" width="820">

**Spikard for Go** — Type-safe HTTP framework backed by Rust via static linking.

Go bindings for the spikard polyglot web toolkit. Backed by the Rust core via CGO static linking.

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <!-- Built with alef -->
  <a href="https://github.com/xberg-io/alef">
    <img src="https://img.shields.io/badge/built%20with-alef%20%D7%90-007ec6?style=flat-square" alt="Built with alef">
  </a>

  <!-- Language bindings -->
  <a href="https://crates.io/crates/spikard">
    <img src="https://img.shields.io/crates/v/spikard?color=007ec6&style=flat-square" alt="Rust">
  </a>
  <a href="https://pypi.org/project/spikard/">
    <img src="https://img.shields.io/pypi/v/spikard?color=007ec6&style=flat-square" alt="Python">
  </a>
  <a href="https://www.npmjs.com/package/@spikard/node">
    <img src="https://img.shields.io/npm/v/@spikard/node?color=007ec6&style=flat-square" alt="Node.js">
  </a>
  <a href="https://www.npmjs.com/package/@spikard/node-wasm">
    <img src="https://img.shields.io/npm/v/@spikard/node-wasm?color=007ec6&style=flat-square" alt="WASM">
  </a>
  <a href="https://rubygems.org/gems/spikard">
    <img src="https://img.shields.io/gem/v/spikard?color=007ec6&style=flat-square" alt="Ruby">
  </a>
  <a href="https://packagist.org/packages/goldziher/spikard">
    <img src="https://img.shields.io/packagist/v/goldziher/spikard?color=007ec6&style=flat-square" alt="PHP">
  </a>
  <a href="https://hex.pm/packages/spikard">
    <img src="https://img.shields.io/hexpm/v/spikard?color=007ec6&style=flat-square" alt="Elixir">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.spikard/spikard">
    <img src="https://img.shields.io/maven-central/v/dev.spikard/spikard?color=007ec6&style=flat-square" alt="Java">
  </a>
  <a href="https://github.com/Goldziher/spikard/releases">
    <img src="https://img.shields.io/github/v/tag/Goldziher/spikard?label=Go&color=007ec6&style=flat-square" alt="Go">
  </a>
  <a href="https://www.nuget.org/packages/Spikard/">
    <img src="https://img.shields.io/nuget/v/Spikard?color=007ec6&style=flat-square" alt="C#">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.spikard/spikard">
    <img src="https://img.shields.io/maven-central/v/dev.spikard/spikard?label=Kotlin&color=007ec6&style=flat-square" alt="Kotlin">
  </a>
  <a href="https://pub.dev/packages/spikard">
    <img src="https://img.shields.io/pub/v/spikard?color=007ec6&style=flat-square" alt="Dart">
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
</div>

[Install](#installation) · [Quick example](#quick-example) · [Static linking](#static-linking) · [Docs](https://github.com/Goldziher/spikard)

</div>

---

## What you get

- **Go module over Rust core** — Route definitions, request extraction, validation, and middleware through Go structs and errors
- **Spec-driven codegen** — OpenAPI 3.0, AsyncAPI 3.0, GraphQL SDL, JSON-RPC 2.0 — shared Rust implementation across all bindings
- **Static-link workflow** — Build against `spikard-ffi` and ship a self-contained binary, no runtime library needed
- **Context-aware** — Full `context.Context` support for cancellation and timeouts
- **Cross-binding parity** — Behavior matches Python, Node.js, Ruby, PHP, Elixir, Java, .NET, Kotlin, Dart, Swift, Zig, WASM, Rust, and C FFI packages

## Installation

```bash
go get github.com/Goldziher/spikard@latest
```

**System requirements:** Go 1.21+, CGO enabled (default). You need the `spikard-ffi` static library at build time — see [Static linking](#static-linking).

## Quick example

```go
package main

import (
    "fmt"
    "log"

    spikard "github.com/Goldziher/spikard"
)

func main() {
    router := spikard.NewRouter()

    router.Get("/hello", func(req *spikard.Request) (*spikard.Response, error) {
        return spikard.JSON(map[string]string{"message": "Hello, World!"}), nil
    })

    if err := router.Listen(":8080"); err != nil {
        log.Fatal(err)
    }
    fmt.Println("Server running on :8080")
}
```

Build:

```bash
CGO_LDFLAGS="-L/path/to/lib -lspikard_ffi" go build
```

## Static linking

The Go module requires the `spikard-ffi` static library (`libspikard_ffi.a`) at link time.

<details>
<summary><strong>Option 1: Download pre-built</strong></summary>

From [GitHub Releases](https://github.com/Goldziher/spikard/releases):

```bash
# Example: Linux x86_64
curl -LO https://github.com/Goldziher/spikard/releases/download/v0.17.0-rc.4/go-ffi-linux-x86_64.tar.gz
tar -xzf go-ffi-linux-x86_64.tar.gz

CGO_LDFLAGS="-L/path/to/lib -lspikard_ffi" go build
```

</details>

<details>
<summary><strong>Option 2: Build from source</strong></summary>

```bash
git clone https://github.com/Goldziher/spikard.git
cd spikard
cargo build -p spikard-ffi --release

CGO_LDFLAGS="-L$(pwd)/target/release -lspikard_ffi" go build
```

</details>

<details>
<summary><strong>Development (monorepo)</strong></summary>

```bash
# Build the static FFI library
cargo build -p spikard-ffi --release

cd packages/go
go build -v
```

</details>

## Features

| Feature | Details |
|---|---|
| **HTTP routing** | Type-safe route definitions with parameter validation |
| **Specs** | OpenAPI 3.0 · AsyncAPI 3.0 · GraphQL SDL · JSON-RPC 2.0 |
| **Middleware** | Via FFI bridge to Rust middleware stack (compression, rate limiting, auth) |
| **Static linking** | Resulting binary is self-contained |
| **Context support** | Full `context.Context` for cancellation and timeouts |
| **Error handling** | Consistent ProblemDetails JSON across all bindings |
| **Testing** | Shared fixture suite drives behavior across all language bindings |

## Troubleshooting

| Issue | Fix |
|---|---|
| `undefined reference to 'spikard_...'` | `CGO_LDFLAGS` not set. Use `CGO_LDFLAGS="-L/path/to/lib -lspikard_ffi"` |
| `cannot find -lspikard_ffi` | Static library missing. Build: `cargo build -p spikard-ffi --release` |

## Resources

- **[Repository](https://github.com/Goldziher/spikard)** — source code, examples, and contributing guide
- **[Go package](https://github.com/Goldziher/spikard/tree/main/packages/go)** — Go-specific examples and tests
- **[Examples](https://github.com/Goldziher/spikard/tree/main/crates/spikard-http/examples)** — working implementations in all supported languages
- **[Issues](https://github.com/Goldziher/spikard/issues)** — bug reports and feature requests

## License

MIT License — see [LICENSE](https://github.com/Goldziher/spikard/blob/main/LICENSE) for details.

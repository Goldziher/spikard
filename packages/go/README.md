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

Go bindings for the spikard polyglot HTTP framework. Backed by the Rust core via CGO static linking.

> **Version 0.14.0**
> Report issues at [github.com/Goldziher/spikard](https://github.com/Goldziher/spikard/issues).

## Install

### Development (monorepo)

```bash
# Build the static FFI library
cargo build -p spikard-ffi --release

cd packages/go
go build -v
```

### Go Modules

```bash
go get github.com/Goldziher/spikard@latest
```

You need the `spikard-ffi` static library at link time. See [Building with Static Libraries](#building-with-static-libraries).

### Building with Static Libraries

#### Option 1: Download Pre-built

Download from [GitHub Releases](https://github.com/Goldziher/spikard/releases):

```bash
# Example: Linux x86_64
curl -LO https://github.com/Goldziher/spikard/releases/download/v0.14.0/go-ffi-linux-x86_64.tar.gz
tar -xzf go-ffi-linux-x86_64.tar.gz

CGO_LDFLAGS="-L/path/to/lib -lspikard_ffi" go build
```

#### Option 2: Build from Source

```bash
git clone https://github.com/Goldziher/spikard.git
cd spikard
cargo build -p spikard-ffi --release

CGO_LDFLAGS="-L$(pwd)/target/release -lspikard_ffi" go build
```

### System Requirements

- **Go 1.21+** required
- CGO enabled (default)
- `spikard-ffi` static library (`libspikard_ffi.a`) at build time

## Quick Start

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
CGO_LDFLAGS="-L$HOME/spikard/lib -lspikard_ffi" go build
```

## Features

- **HTTP routing** — type-safe route definitions with parameter validation
- **OpenAPI / AsyncAPI / GraphQL / JSON-RPC** — code generation and spec parsing
- **Tower middleware** — via FFI bridge to Rust middleware stack
- **CGO static linking** — resulting binary is self-contained, no runtime library needed
- **Context-aware** — full `context.Context` support for cancellation and timeouts

## Troubleshooting

| Issue | Fix |
|-------|-----|
| `undefined reference to 'spikard_...'` | `CGO_LDFLAGS` not pointing at the static library. Set `CGO_LDFLAGS="-L/path/to/lib -lspikard_ffi"` |
| `cannot find -lspikard_ffi` | Static library missing. Build it: `cargo build -p spikard-ffi --release` |

## Documentation

- **[Repository](https://github.com/Goldziher/spikard)** — source code, examples, and contributing guide
- **[Examples](https://github.com/Goldziher/spikard/tree/main/packages/go)** — Go-specific examples
- **[Issues](https://github.com/Goldziher/spikard/issues)** — bug reports and feature requests

## License

MIT License — see [LICENSE](https://github.com/Goldziher/spikard/blob/main/LICENSE) for details.

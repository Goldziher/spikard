<!-- markdownlint-disable MD033 MD041 -->
<div align="center">

<img src="https://raw.githubusercontent.com/Goldziher/spikard/main/docs/assets/spikard-banner.svg" alt="spikard - polyglot web toolkit" width="820">

**Spikard for Go** — Type-safe HTTP framework backed by Rust via static linking.

Go bindings for the spikard polyglot web toolkit. Backed by the Rust core via CGO static linking.

{% include 'partials/badges.html.jinja' %}

[Install](#installation) · [Quick example](#quick-example) · [Static linking](#static-linking) · [Docs]({{ repository }})

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
go get {{ package_name }}@latest
```

**System requirements:** Go 1.21+, CGO enabled (default). You need the `spikard-ffi` static library at build time — see [Static linking](#static-linking).

## Quick example

```go
package main

import (
    "fmt"
    "log"

    spikard "{{ package_name }}"
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
curl -LO https://github.com/Goldziher/spikard/releases/download/v{{ version }}/go-ffi-linux-x86_64.tar.gz
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
| **Static linking** — Resulting binary is self-contained |
| **Context support** | Full `context.Context` for cancellation and timeouts |
| **Error handling** | Consistent ProblemDetails JSON across all bindings |
| **Testing** | Shared fixture suite drives behavior across all language bindings |

## Troubleshooting

| Issue | Fix |
|---|---|
| `undefined reference to 'spikard_...'` | `CGO_LDFLAGS` not set. Use `CGO_LDFLAGS="-L/path/to/lib -lspikard_ffi"` |
| `cannot find -lspikard_ffi` | Static library missing. Build: `cargo build -p spikard-ffi --release` |

## Resources

- **[Repository]({{ repository }})** — source code, examples, and contributing guide
- **[Go package]({{ repository }}/tree/main/packages/go)** — Go-specific examples and tests
- **[Examples]({{ repository }}/tree/main/crates/spikard-http/examples)** — working implementations in all supported languages
- **[Issues]({{ repository }}/issues)** — bug reports and feature requests

## License

{{ license }} License — see [LICENSE]({{ repository }}/blob/main/LICENSE) for details.

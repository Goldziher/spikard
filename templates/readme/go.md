# Spikard

{% include 'partials/badges.html.jinja' %}

Go bindings for the spikard polyglot HTTP framework. Backed by the Rust core via CGO static linking.

> **Version {{ version }}**
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
go get {{ package_name }}@latest
```

You need the `spikard-ffi` static library at link time. See [Building with Static Libraries](#building-with-static-libraries).

### Building with Static Libraries

#### Option 1: Download Pre-built

Download from [GitHub Releases](https://github.com/Goldziher/spikard/releases):

```bash
# Example: Linux x86_64
curl -LO https://github.com/Goldziher/spikard/releases/download/v{{ version }}/go-ffi-linux-x86_64.tar.gz
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

- **[Repository]({{ repository }})** — source code, examples, and contributing guide
- **[Examples]({{ repository }}/tree/main/packages/go)** — Go-specific examples
- **[Issues]({{ repository }}/issues)** — bug reports and feature requests

## License

{{ license }} License — see [LICENSE]({{ repository }}/blob/main/LICENSE) for details.

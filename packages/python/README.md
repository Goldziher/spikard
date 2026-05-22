# Spikard

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <!-- Built with -->
  <a href="https://github.com/kreuzberg-dev/alef">
    <img src="https://img.shields.io/badge/bindings%20by-alef%20%D7%90-007ec6" alt="Bindings by alef">
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
    <img src="https://img.shields.io/badge/docs-GitHub-007ec6" alt="Documentation">
  </a>
</div>

<div align="center" style="display: flex; flex-wrap: wrap; gap: 12px; justify-content: center; margin: 28px 0 24px;">
  <a href="https://discord.gg/pXxagNK2zN">
    <img height="32" src="https://img.shields.io/badge/Discord-Join%20our%20community-007ec6?logo=discord&logoColor=white" alt="Join Discord">
  </a>
</div>

Rust-centric polyglot HTTP framework with OpenAPI/AsyncAPI/GraphQL/JSON-RPC codegen, tower-http middleware, and fixture-driven cross-language testing. Native Python bindings with async/await support.

## What This Package Provides

- **Python-native HTTP app API** — async route handlers, typed config, request extraction, validation, and middleware hooks.
- **Spec-driven generation** — OpenAPI, AsyncAPI, GraphQL SDL, JSON-RPC, and SQL-to-HTTP codegen through the shared Rust core.
- **Fixture-backed parity** — Python behavior is tested against the same fixtures as the Node.js, Ruby, PHP, Elixir, Go, Java, .NET, Kotlin, Dart, Swift, Zig, WASM, Rust, and C FFI packages.
- **No sidecar server** — the package calls the Rust core directly.

## Installation

```bash
pip install spikard
```

### System Requirements

- **Python 3.10+** required
- Pre-built wheels for Linux (x86_64, aarch64), macOS (arm64, x86_64), Windows (x86_64)

## Quick Start

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

## Route Definition

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

## Request Handling

```python
from spikard import Spikard
from msgspec import Struct

app = Spikard()


class Order(Struct):
    id: int
    item: str
    quantity: int
    verbose: bool | None = None


@app.post("/orders/{order_id:int}")
async def update_order(order_id: int, order: Order, verbose: bool | None = False) -> Order:
    return Order(id=order_id, item=order.item, quantity=order.quantity, verbose=verbose or False)
```

## Validation

```python
from msgspec import Struct

class Payment(Struct):
    id: str
    amount: float

@app.post("/payments")
async def create_payment(payment: Payment) -> Payment:
    return payment
```

## Middleware

```python
from spikard import Spikard

app = Spikard()

@app.on_request
async def logging_hook(request):
    print(f"{request['method']} {request['path']}")
    return request
```

## Server Configuration

```python
from spikard import Spikard
from spikard.config import ServerConfig

config = ServerConfig(
    host="0.0.0.0",
    port=8080,
    workers=4,
    request_timeout=60,
    max_body_size=5 * 1024 * 1024,  # 5MB
)

app = Spikard(config=config)

@app.get("/health")
async def health():
    return {"status": "ok"}

if __name__ == "__main__":
    app.run()
```

## Async Support

Full async/await support — handlers are async functions, integrated with asyncio:

```bash
python app.py
```

## Features

- **HTTP routing** — type-safe route definitions with path, query, and body parameter validation
- **OpenAPI / AsyncAPI / GraphQL / JSON-RPC** — code generation and spec parsing
- **Tower middleware** — compression, rate limiting, timeouts, auth (JWT/API key)
- **Lifecycle hooks** — `onRequest`, `preValidation`, `preHandler`, `onResponse`, `onError`
- **Fixture-driven testing** — shared JSON fixtures drive tests across all language bindings
- **Polyglot** — single Rust core, thin bindings across 15+ languages

## Documentation

- **[Repository](https://github.com/Goldziher/spikard)** — source code and contributing guide
- **[Issues](https://github.com/Goldziher/spikard/issues)** — bug reports and feature requests

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](https://github.com/Goldziher/spikard/blob/main/CONTRIBUTING.md).

## License

MIT License — see [LICENSE](https://github.com/Goldziher/spikard/blob/main/LICENSE) for details.

<!-- markdownlint-disable MD033 MD041 -->
<div align="center">

<img src="https://raw.githubusercontent.com/Goldziher/spikard/main/docs/assets/spikard-banner.svg" alt="spikard - polyglot web toolkit" width="820">

**Spikard for Python** — Type-safe async HTTP framework backed by Rust.

Rust-centric polyglot HTTP framework with OpenAPI/AsyncAPI/GraphQL/JSON-RPC codegen, tower-http middleware, and fixture-driven cross-language testing. Native Python bindings with async/await support.

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

[Install](#installation) · [Quick example](#quick-example) · [Async/await](#asyncawait-support) · [Docs](https://github.com/Goldziher/spikard)

</div>

---

## What you get

- **Async/await routing** — Native Python async functions as route handlers, integrated with asyncio
- **Type-safe validation** — Request validation with typed structs and dataclasses, error handling as exceptions
- **Spec-driven codegen** — OpenAPI 3.0, AsyncAPI 3.0, GraphQL SDL, JSON-RPC 2.0 code generation
- **Tower middleware** — Compression, rate limiting, timeouts, JWT/API-key auth, static files via the Rust core
- **Fixture-backed testing** — Behavior tested against shared fixtures with Node.js, Ruby, PHP, Elixir, Go, Java, C#, Kotlin, Dart, Swift, Zig, WASM, Rust, and C FFI packages
- **No sidecar** — Calls the Rust core directly; no separate server process

## Installation

```bash
pip install spikard
```

**System requirements:** Python 3.10+. Pre-built wheels for Linux (x86_64, aarch64), macOS (arm64, x86_64), Windows (x86_64).

## Quick example

```python
from spikard import App, ServerConfig
from msgspec import Struct

class User(Struct):
    id: int
    name: str

app = App()

@app.get_decorator("/users/{id:int}")
async def get_user(id: int) -> User:
    return User(id=id, name="Alice")

@app.post_decorator("/users")
async def create_user(user: User) -> User:
    return user

if __name__ == "__main__":
    app.config(ServerConfig(port=8000))
    app.run()
```

## Async/await support

Handlers are native async functions integrated with asyncio — full concurrency with async libraries:

```bash
python app.py
```

<details>
<summary><strong>Route definition</strong></summary>

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

</details>

<details>
<summary><strong>Request handling</strong></summary>

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

</details>

<details>
<summary><strong>Validation</strong></summary>

```python
from msgspec import Struct

class Payment(Struct):
    id: str
    amount: float

@app.post("/payments")
async def create_payment(payment: Payment) -> Payment:
    return payment
```

</details>

<details>
<summary><strong>Middleware & configuration</strong></summary>

```python
from spikard import Spikard

app = Spikard()

@app.on_request
async def logging_hook(request):
    print(f"{request['method']} {request['path']}")
    return request
```

See [server configuration](https://github.com/Goldziher/spikard/tree/main/packages/python) for full options.

</details>

## Features

| Feature | Details |
|---|---|
| **Async routes** | All handlers are native `async def`, fully concurrent with asyncio |
| **Type safety** | Request validation with typed structs, automatic type conversion |
| **HTTP routing** | Path, query, body, and header parameter extraction |
| **Specs** | OpenAPI 3.0 · AsyncAPI 3.0 · GraphQL SDL · JSON-RPC 2.0 |
| **Middleware** | Compression · rate limiting · timeouts · auth · static files |
| **Lifecycle** | Hooks for request, pre-validation, pre-handler, response, error |
| **Error handling** | Consistent ProblemDetails JSON across all bindings |
| **Testing** | Shared fixture suite drives behavior across all language bindings |

## Resources

- **[Repository](https://github.com/Goldziher/spikard)** — source code, examples, and contributing guide
- **[Python package](https://github.com/Goldziher/spikard/tree/main/packages/python)** — Python-specific examples and tests
- **[Examples](https://github.com/Goldziher/spikard/tree/main/crates/spikard-http/examples)** — working implementations in all supported languages
- **[Issues](https://github.com/Goldziher/spikard/issues)** — bug reports and feature requests

## License

MIT License — see [LICENSE](https://github.com/Goldziher/spikard/blob/main/LICENSE) for details.

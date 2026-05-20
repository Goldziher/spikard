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

Rust-centric polyglot HTTP framework with OpenAPI/AsyncAPI/GraphQL/JSON-RPC codegen, tower-http middleware, and fixture-driven cross-language testing. Native Python bindings with async/await support.

**Powered by a Rust core** — native performance for HTTP routing, middleware, and schema validation.

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

- **[Repository](https://github.com/Goldziher/spikard)** — source code, examples, and contributing guide
- **[Examples](https://github.com/Goldziher/spikard/tree/main/examples/python)** — Python-specific examples
- **[Issues](https://github.com/Goldziher/spikard/issues)** — bug reports and feature requests

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](https://github.com/Goldziher/spikard/blob/main/CONTRIBUTING.md).

## License

MIT License — see [LICENSE](https://github.com/Goldziher/spikard/blob/main/LICENSE) for details.

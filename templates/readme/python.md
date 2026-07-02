<!-- markdownlint-disable MD033 MD041 -->
<div align="center">

<img src="https://raw.githubusercontent.com/Goldziher/spikard/main/docs/assets/spikard-banner.svg" alt="spikard - polyglot web toolkit" width="820">

**Spikard for Python** — Type-safe async HTTP framework backed by Rust.

{{ description }}

{% include 'partials/badges.html.jinja' %}

[Install](#installation) · [Quick example](#quick-example) · [Async/await](#asyncawait-support) · [Docs]({{ repository }})

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

{{ 'quickstart_routes.md' | include_snippet('python') }}

## Async/await support

Handlers are native async functions integrated with asyncio — full concurrency with async libraries:

{{ 'run_app.md' | include_snippet('python') }}

<details>
<summary><strong>Route definition</strong></summary>

{{ 'hello_route.md' | include_snippet('python') }}

</details>

<details>
<summary><strong>Request handling</strong></summary>

{{ 'request_data.md' | include_snippet('python') }}

</details>

<details>
<summary><strong>Validation</strong></summary>

{{ 'validation_basic.md' | include_snippet('python') }}

</details>

<details>
<summary><strong>Middleware & configuration</strong></summary>

{{ 'middleware_basic.md' | include_snippet('python') }}

See [server configuration]({{ repository }}/tree/main/packages/python) for full options.

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

- **[Repository]({{ repository }})** — source code, examples, and contributing guide
- **[Python package]({{ repository }}/tree/main/packages/python)** — Python-specific examples and tests
- **[Examples]({{ repository }}/tree/main/crates/spikard-http/examples)** — working implementations in all supported languages
- **[Issues]({{ repository }}/issues)** — bug reports and feature requests

## License

{{ license }} License — see [LICENSE]({{ repository }}/blob/main/LICENSE) for details.

# Spikard

{% include 'partials/badges.html.jinja' %}

{{ description }}

**Powered by a Rust core** — native performance for HTTP routing, middleware, and schema validation.

## Installation

```bash
pip install spikard
```

### System Requirements

- **Python 3.10+** required
- Pre-built wheels for Linux (x86_64, aarch64), macOS (arm64, x86_64), Windows (x86_64)

## Quick Start

{{ 'quickstart_routes.md' | include_snippet('python') }}

## Route Definition

{{ 'hello_route.md' | include_snippet('python') }}

## Request Handling

{{ 'request_data.md' | include_snippet('python') }}

## Validation

{{ 'validation_basic.md' | include_snippet('python') }}

## Middleware

{{ 'middleware_basic.md' | include_snippet('python') }}

## Server Configuration

{{ 'config_server.md' | include_snippet('python') }}

## Async Support

Full async/await support — handlers are async functions, integrated with asyncio:

{{ 'run_app.md' | include_snippet('python') }}

## Features

- **HTTP routing** — type-safe route definitions with path, query, and body parameter validation
- **OpenAPI / AsyncAPI / GraphQL / JSON-RPC** — code generation and spec parsing
- **Tower middleware** — compression, rate limiting, timeouts, auth (JWT/API key)
- **Lifecycle hooks** — `onRequest`, `preValidation`, `preHandler`, `onResponse`, `onError`
- **Fixture-driven testing** — shared JSON fixtures drive tests across all language bindings
- **Polyglot** — single Rust core, thin bindings across 15+ languages

## Documentation

- **[Repository]({{ repository }})** — source code, examples, and contributing guide
- **[Examples]({{ repository }}/tree/main/examples/python)** — Python-specific examples
- **[Issues]({{ repository }}/issues)** — bug reports and feature requests

## Contributing

Contributions are welcome. See [CONTRIBUTING.md]({{ repository }}/blob/main/CONTRIBUTING.md).

## License

{{ license }} License — see [LICENSE]({{ repository }}/blob/main/LICENSE) for details.

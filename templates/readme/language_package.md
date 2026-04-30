# {{ name }}

{% include 'partials/badges.html.jinja' %}

{{ description }}

## Installation

{% include 'partials/installation.md.jinja' %}

## Quick Start

{% if snippets.quickstart %}
{{ snippets.quickstart | include_snippet(language) }}

{% elif snippets.hello_route %}
{{ snippets.hello_route | include_snippet(language) }}

{% else %}
See the [spikard repository]({{ repository }}) for usage examples and guides.

{% endif %}

## Features

- **HTTP routing** — type-safe route definitions with path, query, and body parameter validation
- **OpenAPI / AsyncAPI / GraphQL / JSON-RPC** — code generation and spec parsing built in
- **Tower middleware** — compression, rate limiting, timeouts, auth (JWT/API key), static files
- **Lifecycle hooks** — `onRequest`, `preValidation`, `preHandler`, `onResponse`, `onError`
- **Fixture-driven testing** — shared JSON fixtures drive tests across all language bindings
- **Polyglot** — single Rust core, thin bindings for Python, Node.js, Ruby, PHP, Elixir, Go, Java, C#, Kotlin, Dart, Gleam, WASM, Swift, Zig, and C FFI

{% if snippets.routing_basic %}

## Routing

{{ snippets.routing_basic | include_snippet(language) }}

{% endif %}
{% if snippets.validation_basic %}

## Validation

{{ snippets.validation_basic | include_snippet(language) }}

{% endif %}
{% if snippets.middleware_basic %}

## Middleware

{{ snippets.middleware_basic | include_snippet(language) }}

{% endif %}

## Documentation

- **[Repository]({{ repository }})** — source code, examples, and contributing guide
- **[Examples]({{ repository }}/tree/main/examples)** — working examples per language
- **[Issues]({{ repository }}/issues)** — bug reports and feature requests

## Contributing

Contributions are welcome. See [CONTRIBUTING.md]({{ repository }}/blob/main/CONTRIBUTING.md).

## License

{{ license }} License — see [LICENSE]({{ repository }}/blob/main/LICENSE) for details.

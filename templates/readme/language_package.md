# {{ name }}

{% include 'partials/badges.html.jinja' %}

{{ description }}

## What This Package Provides

- **HTTP application core** — typed routing, request data extraction, validation, lifecycle hooks, and Tower middleware from the Rust engine.
- **Spec-driven work** — OpenAPI, AsyncAPI, GraphQL SDL, JSON-RPC, and SQL-to-HTTP codegen are shared across bindings.
- **Cross-language parity** — generated bindings use the same DTOs, fixtures, and error model, so behavior does not drift between runtimes.
- **Native integration** — no sidecar server required; each package calls the Rust core through its language-native bridge.
{% if language == "typescript" %}
- **Node-first TypeScript API** — NAPI-RS package with generated types and async/await routing.
{% elif language == "python" %}
- **Python package** — async route handlers, typed config, and validation-friendly request objects.
{% elif language == "rust" %}
- **Rust crate** — canonical Axum/Tower implementation used by every binding.
{% elif language == "ruby" %}
- **Ruby package** — Magnus-backed native extension with Ruby route objects.
{% elif language == "php" %}
- **PHP extension** — PHP 8.2+ API over the shared HTTP core.
{% elif language == "elixir" %}
- **BEAM package** — Rustler NIF binding for OTP applications.
{% elif language == "ffi" %}
- **C ABI** — stable shared library surface for custom hosts and secondary bindings.
{% elif language == "go" %}
- **Go module** — cgo-backed API with Go contexts and errors.
{% elif language == "java" %}
- **Java package** — JVM binding for typed routes and generated schemas.
{% elif language == "csharp" %}
- **.NET package** — async/await API with generated C# DTOs.
{% elif language == "kotlin" %}
- **Kotlin package** — coroutine-friendly JVM binding with data classes.
{% elif language == "dart" %}
- **Dart package** — Future/Stream API through flutter_rust_bridge.
{% elif language == "wasm" %}
- **WASM package** — browser, Deno, and edge-compatible binding.
{% elif language == "swift" %}
- **SwiftPM package** — swift-bridge API for Swift services and tools.
{% elif language == "zig" %}
- **Zig package** — wrapper over the C FFI with explicit ownership.
{% endif %}

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
- **[Examples]({{ repository }}/tree/main/crates/spikard-http/examples)** — working server examples
- **[Issues]({{ repository }}/issues)** — bug reports and feature requests

## Contributing

Contributions are welcome. See [CONTRIBUTING.md]({{ repository }}/blob/main/CONTRIBUTING.md).

## License

{{ license }} License — see [LICENSE]({{ repository }}/blob/main/LICENSE) for details.

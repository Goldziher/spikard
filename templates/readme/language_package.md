<!-- markdownlint-disable MD033 MD041 -->
<div align="center">

<img src="https://raw.githubusercontent.com/Goldziher/spikard/main/docs/assets/spikard-banner.svg" alt="spikard - polyglot web toolkit" width="820">

**{{ name }}** — Part of the spikard polyglot web toolkit.

{{ description }}

{% include 'partials/badges.html.jinja' %}

[Install](#installation) · [Quick example](#quick-example) · [Features](#features) · [Docs]({{ repository }})

</div>

---

## What this package provides

{% if language == "typescript" %}
- **Node.js-first TypeScript API** — NAPI-RS native bindings with generated types and full async/await support
{% elif language == "python" %}
- **Python-native async routing** — PyO3 bindings with async route handlers and typed request objects
{% elif language == "rust" %}
- **Canonical Rust implementation** — the core HTTP server used by all other bindings
{% elif language == "ruby" %}
- **Magnus-backed native extension** — Ruby 3.2+ with native performance for routing and validation
{% elif language == "php" %}
- **PHP 8.2+ extension** — native ext-php-rs bindings for the shared HTTP core
{% elif language == "elixir" %}
- **OTP-integrated Rustler bindings** — Elixir 1.14+ with Erlang 25+ for distributed systems
{% elif language == "ffi" %}
- **C ABI stable interface** — shared and static library surface for custom hosts
{% elif language == "go" %}
- **CGO-backed Go module** — statically-linked library for self-contained binaries
{% elif language == "java" %}
- **JVM binding** — Panama FFM / JNI for Java 17+ applications
{% elif language == "csharp" %}
- **.NET async API** — P/Invoke bindings with async/await for .NET 8.0+
{% elif language == "kotlin" %}
- **Kotlin coroutines** — JVM binding with Kotlin 1.9+ data classes and suspend functions
{% elif language == "dart" %}
- **Flutter-compatible FFI** — Dart 3.0+ with Future and Stream APIs
{% elif language == "wasm" %}
- **Browser and server WASM** — wasm-bindgen for browsers, Node.js 18+, and Deno 1.0+
{% elif language == "swift" %}
- **SwiftPM package** — Swift 5.9+ with swift-bridge for macOS 13+ and Linux
{% elif language == "zig" %}
- **Zig wrapper** — explicit memory management over the C FFI stable surface
{% else %}
- **{{ name }}** — compiled from the shared Rust HTTP core via alef-generated bindings
{% endif %}

- **Type-safe routing** — HTTP definitions with path, query, body, and header validation across all bindings
- **Spec-driven codegen** — OpenAPI 3.0, AsyncAPI 3.0, GraphQL SDL, and JSON-RPC 2.0 support
- **Cross-language parity** — same DTOs, fixtures, and error model prevent runtime drift
- **Tower middleware** — compression, rate limiting, timeouts, auth (JWT/API key), static files
- **Lifecycle hooks** — `onRequest`, `preValidation`, `preHandler`, `onResponse`, `onError`

## Installation

{% include 'partials/installation.md.jinja' %}

## Quick example

{% if snippets.quickstart %}
{{ snippets.quickstart | include_snippet(language) }}

{% elif snippets.hello_route %}
{{ snippets.hello_route | include_snippet(language) }}

{% else %}
See the [spikard repository]({{ repository }}) for usage examples and guides.

{% endif %}

## Features

| Feature | Support |
|---|---|
| **Type-safe routing** | Path, query, body, and header parameter validation |
| **Request extraction** | Typed structs for JSON, form data, multipart, and raw bodies |
| **Spec support** | OpenAPI 3.0 · AsyncAPI 3.0 · GraphQL SDL · JSON-RPC 2.0 |
| **Middleware** | Compression, rate limiting, timeouts, authentication, static files |
| **Lifecycle hooks** | Request, pre-validation, pre-handler, response, and error hooks |
| **WebSocket & SSE** | Bidirectional streams and server-sent events |
| **Error handling** | Consistent error responses across all bindings via ProblemDetails |
| **Fixture testing** | Shared JSON fixtures for behavioral consistency across languages |

<details>
<summary><strong>Routing</strong></summary>

{% if snippets.routing_basic %}
{{ snippets.routing_basic | include_snippet(language) }}
{% else %}
See [examples]({{ repository }}/tree/main/crates/spikard-http/examples) in the repository.
{% endif %}

</details>

<details>
<summary><strong>Validation</strong></summary>

{% if snippets.validation_basic %}
{{ snippets.validation_basic | include_snippet(language) }}
{% else %}
See [examples]({{ repository }}/tree/main/crates/spikard-http/examples) in the repository.
{% endif %}

</details>

<details>
<summary><strong>Middleware & configuration</strong></summary>

{% if snippets.middleware_basic %}
{{ snippets.middleware_basic | include_snippet(language) }}
{% else %}
See [examples]({{ repository }}/tree/main/crates/spikard-http/examples) in the repository.
{% endif %}

</details>

## Resources

- **[Repository]({{ repository }})** — source code, examples, and issues
- **[Examples]({{ repository }}/tree/main/crates/spikard-http/examples)** — working implementations in all supported languages
- **[Contributing]({{ repository }}/blob/main/CONTRIBUTING.md)** — how to contribute

## License

{{ license }} License — see [LICENSE]({{ repository }}/blob/main/LICENSE) for details.

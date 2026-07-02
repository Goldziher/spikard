<!-- markdownlint-disable MD033 MD041 -->
<div align="center">

<img src="https://raw.githubusercontent.com/Goldziher/spikard/main/docs/assets/spikard-banner.svg" alt="spikard — polyglot http framework" width="820">

**Spikard** — Part of the spikard polyglot HTTP framework.

Rust-centric polyglot HTTP framework with OpenAPI/AsyncAPI/GraphQL/JSON-RPC codegen, tower-http middleware, and fixture-driven cross-language testing. Ruby bindings via Magnus with idiomatic Ruby API.

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <!-- Built with alef -->
  <a href="https://github.com/xberg-io/alef">
    <img src="https://img.shields.io/badge/built%20with-alef%20%D7%90-007ec6?style=flat-square" alt="Built with alef">
  </a>

  <!-- Language bindings -->
  <a href="https://crates.io/crates/spikard">
    <img src="https://img.shields.io/crates/v/spikard?style=flat-square" alt="Rust">
  </a>
  <a href="https://pypi.org/project/spikard/">
    <img src="https://img.shields.io/pypi/v/spikard?style=flat-square" alt="Python">
  </a>
  <a href="https://www.npmjs.com/package/@spikard/node">
    <img src="https://img.shields.io/npm/v/@spikard/node?style=flat-square" alt="Node.js">
  </a>
  <a href="https://www.npmjs.com/package/@spikard/node-wasm">
    <img src="https://img.shields.io/npm/v/@spikard/node-wasm?style=flat-square" alt="WASM">
  </a>
  <a href="https://rubygems.org/gems/spikard">
    <img src="https://img.shields.io/gem/v/spikard?style=flat-square" alt="Ruby">
  </a>
  <a href="https://packagist.org/packages/goldziher/spikard">
    <img src="https://img.shields.io/packagist/v/goldziher/spikard?style=flat-square" alt="PHP">
  </a>
  <a href="https://hex.pm/packages/spikard">
    <img src="https://img.shields.io/hexpm/v/spikard?style=flat-square" alt="Elixir">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.spikard/spikard">
    <img src="https://img.shields.io/maven-central/v/dev.spikard/spikard?style=flat-square" alt="Java">
  </a>
  <a href="https://github.com/Goldziher/spikard/releases">
    <img src="https://img.shields.io/github/v/tag/Goldziher/spikard?label=Go&style=flat-square" alt="Go">
  </a>
  <a href="https://www.nuget.org/packages/Spikard/">
    <img src="https://img.shields.io/nuget/v/Spikard?style=flat-square" alt="C#">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.spikard/spikard">
    <img src="https://img.shields.io/maven-central/v/dev.spikard/spikard?label=Kotlin&style=flat-square" alt="Kotlin">
  </a>
  <a href="https://pub.dev/packages/spikard">
    <img src="https://img.shields.io/pub/v/spikard?style=flat-square" alt="Dart">
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
  <a href="https://github.com/Goldziher/spikard">
    <img src="https://img.shields.io/github/stars/Goldziher/spikard?style=flat-square" alt="GitHub Stars">
  </a>
</div>

[Install](#installation) · [Quick example](#quick-example) · [Features](#features) · [Docs](https://github.com/Goldziher/spikard)

</div>

---

## What this package provides

- **Magnus-backed native extension** — Ruby 3.2+ with native performance for routing and validation

- **Type-safe routing** — HTTP definitions with path, query, body, and header validation across all bindings
- **Spec-driven codegen** — OpenAPI 3.0, AsyncAPI 3.0, GraphQL SDL, and JSON-RPC 2.0 support
- **Cross-language parity** — same DTOs, fixtures, and error model prevent runtime drift
- **Tower middleware** — compression, rate limiting, timeouts, auth (JWT/API key), static files
- **Lifecycle hooks** — `onRequest`, `preValidation`, `preHandler`, `onResponse`, `onError`

## Installation

**gem:**

```bash
gem install spikard
```

**Bundler:**

```ruby
gem 'spikard'
```

### System Requirements

- **Ruby 3.2+** required

## Quick example

```ruby
require "spikard"

app = Spikard::App.new

app.get("/users/:id") do |params, _query, _body|
  { id: params[:id].to_i, name: "Alice" }
end

app.post("/users") do |_params, _query, body|
  user = body
  { id: user["id"], name: user["name"] }
end

app.run(config: { port: 8000 })
```

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

```ruby
require "spikard"

app = Spikard::App.new

app.get("/health") { |_params, _query, _body| { status: "ok" } }
app.post("/users") { |_params, _query, body| body }
```

</details>

<details>
<summary><strong>Validation</strong></summary>

```ruby
require "spikard"

PaymentSchema = Dry::Schema.Params do
  required(:id).filled(:string)
  required(:amount).filled(:float)
end

app = Spikard::App.new

app.post("/payments") do |_params, _query, body|
  PaymentSchema.call(body)
end
```

</details>

<details>
<summary><strong>Middleware & configuration</strong></summary>

```ruby
require "spikard"

app = Spikard::App.new

app.on_request do |request|
  puts "#{request[:method]} #{request[:path]}"
  request
end
```

</details>

## Resources

- **[Repository](https://github.com/Goldziher/spikard)** — source code, examples, and issues
- **[Examples](https://github.com/Goldziher/spikard/tree/main/crates/spikard-http/examples)** — working implementations in all supported languages
- **[Contributing](https://github.com/Goldziher/spikard/blob/main/CONTRIBUTING.md)** — how to contribute

## License

MIT License — see [LICENSE](https://github.com/Goldziher/spikard/blob/main/LICENSE) for details.

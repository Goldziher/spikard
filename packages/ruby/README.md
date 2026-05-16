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

Rust-centric polyglot HTTP framework with OpenAPI/AsyncAPI/GraphQL/JSON-RPC codegen, tower-http middleware, and fixture-driven cross-language testing. Ruby bindings via Magnus with idiomatic Ruby API.

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

## Quick Start

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

- **HTTP routing** — type-safe route definitions with path, query, and body parameter validation
- **OpenAPI / AsyncAPI / GraphQL / JSON-RPC** — code generation and spec parsing built in
- **Tower middleware** — compression, rate limiting, timeouts, auth (JWT/API key), static files
- **Lifecycle hooks** — `onRequest`, `preValidation`, `preHandler`, `onResponse`, `onError`
- **Fixture-driven testing** — shared JSON fixtures drive tests across all language bindings
- **Polyglot** — single Rust core, thin bindings for Python, Node.js, Ruby, PHP, Elixir, Go, Java, C#, Kotlin, Dart, Gleam, WASM, Swift, Zig, and C FFI

## Routing

```ruby
require "spikard"

app = Spikard::App.new

app.get("/health") { |_params, _query, _body| { status: "ok" } }
app.post("/users") { |_params, _query, body| body }
```

## Validation

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

## Middleware

```ruby
require "spikard"

app = Spikard::App.new

app.on_request do |request|
  puts "#{request[:method]} #{request[:path]}"
  request
end
```

## Documentation

- **[Repository](https://github.com/Goldziher/spikard)** — source code, examples, and contributing guide
- **[Examples](https://github.com/Goldziher/spikard/tree/main/examples)** — working examples per language
- **[Issues](https://github.com/Goldziher/spikard/issues)** — bug reports and feature requests

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](https://github.com/Goldziher/spikard/blob/main/CONTRIBUTING.md).

## License

MIT License — see [LICENSE](https://github.com/Goldziher/spikard/blob/main/LICENSE) for details.

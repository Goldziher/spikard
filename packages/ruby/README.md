<!-- GENERATED FILE — DO NOT EDIT DIRECTLY. Run: task readme:generate -->

# Spikard Ruby

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <a href="https://spikard.dev">
    <img src="https://img.shields.io/badge/docs-spikard.dev-007ec6" alt="Documentation">
  </a>
  <a href="https://crates.io/crates/spikard">
    <img src="https://img.shields.io/crates/v/spikard.svg?color=007ec6" alt="Crates.io">
  </a>
  <a href="https://pypi.org/project/spikard/">
    <img src="https://img.shields.io/pypi/v/spikard.svg?color=007ec6" alt="PyPI">
  </a>
  <a href="https://www.npmjs.com/package/@spikard/node">
    <img src="https://img.shields.io/npm/v/@spikard/node.svg?color=007ec6" alt="npm">
  </a>
  <a href="https://rubygems.org/gems/spikard">
    <img src="https://img.shields.io/gem/v/spikard.svg?color=007ec6" alt="RubyGems">
  </a>
  <a href="https://packagist.org/packages/spikard/spikard">
    <img src="https://img.shields.io/packagist/v/spikard/spikard.svg?color=007ec6" alt="Packagist">
  </a>
  <a href="https://hex.pm/packages/spikard">
    <img src="https://img.shields.io/hexpm/v/spikard.svg?color=007ec6" alt="Hex.pm">
  </a>
  <a href="https://github.com/Goldziher/spikard/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-007ec6" alt="License">
  </a>
</div>

Ruby bindings for Spikard: a Rust-centric web framework with type-safe code generation from OpenAPI, GraphQL, AsyncAPI, and OpenRPC specifications. Leverage Sinatra-style routing with zero-copy FFI performance.

## Key Features
- **Type-safe with RBS**: Full RBS type definitions for Steep type checking
- **Zero-copy FFI**: Magnus/rb-sys bindings eliminate serialization overhead
- **Sinatra-style routing**: Familiar `get`, `post`, `put`, `patch`, `delete` DSL
- **Code generation**: Generate type-safe handlers from OpenAPI, GraphQL, AsyncAPI, and OpenRPC specs
- **Full async support**: Non-blocking handlers with complete async/await integration
- **Tower-HTTP middleware**: Compression, rate limiting, authentication, CORS, request IDs
- **Real-time**: WebSockets and Server-Sent Events (SSE)
- **Production-ready**: Dependency injection, validation schemas, lifecycle hooks

## Installation

**Via RubyGems (recommended):**

```bash
gem install spikard
```

**From source:**

```bash
bundle install && bundle exec rake ext:build
```

**Requirements:** Ruby 3.2-4.x, Bundler, and Rust toolchain (for building from source)

## Quick Start

```ruby
require "spikard"

app = Spikard::App.new

app.get "/hello" do |request|
  { message: "Hello, World!" }
end

app.get "/users/:id" do |request|
  user_id = request[:path_params]["id"]
  { id: user_id, name: "Alice" }
end

app.post "/users" do |request|
  { id: 1, name: request[:body]["name"] }
end

app.run(config: { port: 8000 })
```

The `request` hash provides access to:
- `request[:method]` - HTTP method
- `request[:path]` - URL path
- `request[:path_params]` - Path parameters
- `request[:query]` - Query parameters
- `request[:headers]` - Request headers
- `request[:cookies]` - Request cookies
- `request[:body]` - Parsed request body

## Performance

Benchmarked across 34 workloads at 100 concurrency ([methodology](../../docs/benchmarks/methodology.md)):

| Framework | Avg RPS | P50 (ms) | P99 (ms) |
|-----------|--------:|----------:|----------:|
| **spikard** | 7,151 | 14.62 | 18.98 |
| roda | 5,038 | 26.89 | 35.61 |
| hanami-api | 5,032 | 76.1 | 414.35 |

Spikard is **1.4x faster than Roda with significantly lower tail latency**.

## Testing

Use the TestClient for integration tests:

```ruby
client = Spikard::TestClient.new(app)

# HTTP requests
response = client.get("/hello", query: { name: "Alice" })
puts response.status_code  # 200
puts response.json         # { "message" => "Hello, World!" }

# POST, WebSocket, SSE all supported
response = client.post("/users", json: { name: "Bob" })
ws = client.websocket("/chat")
sse = client.sse("/events")

client.close
```

## Examples

**Examples & Code Generation:**
- [Runnable Examples](../../examples/) - Ruby, Python, TypeScript, PHP, Elixir, and Rust
- [Code Generation Guide](../../examples/README.md) - Generate from OpenAPI, GraphQL, AsyncAPI, OpenRPC

## Documentation

Full documentation at [spikard.dev](https://spikard.dev). See also [CONTRIBUTING.md](../../CONTRIBUTING.md).

## Other Languages

- **Rust:** [Crates.io](https://crates.io/crates/spikard)
- **Python:** [PyPI](https://pypi.org/project/spikard/)
- **TypeScript:** [npm (@spikard/node)](https://www.npmjs.com/package/@spikard/node)
- **Ruby:** [RubyGems](https://rubygems.org/gems/spikard)
- **PHP:** [Packagist](https://packagist.org/packages/spikard/spikard)
- **Elixir:** [Hex.pm](https://hex.pm/packages/spikard)

## License

MIT - See [LICENSE](../../LICENSE) for details

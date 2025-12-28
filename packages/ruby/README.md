# Spikard Ruby

[![Documentation](https://img.shields.io/badge/docs-spikard.dev-58FBDA)](https://spikard.dev)
[![Gem Version](https://img.shields.io/gem/v/spikard.svg)](https://rubygems.org/gems/spikard)
[![Gem Downloads](https://img.shields.io/gem/dt/spikard.svg)](https://rubygems.org/gems/spikard)
[![Ruby Version](https://img.shields.io/badge/ruby-%3E%3D%203.2-red.svg)](https://www.ruby-lang.org/)
[![codecov](https://codecov.io/gh/Goldziher/spikard/graph/badge.svg?token=H4ZXDZ4A69)](https://codecov.io/gh/Goldziher/spikard)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

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
bundle install
bundle exec rake ext:build
```

**Requirements:** Ruby 3.2+, Bundler, and Rust toolchain (for building from source). On Windows, use RubyInstaller with MSYS2 DevKit and the GNU Rust toolchain.

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

app.run(port: 8000)
```

The `request` hash provides access to:
- `request[:method]` - HTTP method
- `request[:path]` - URL path
- `request[:path_params]` - Path parameters
- `request[:query]` - Query parameters
- `request[:headers]` - Request headers
- `request[:cookies]` - Request cookies
- `request[:body]` - Parsed request body

## Validation

Pass a `request_schema` to validate incoming JSON:

```ruby
require "dry-schema"

UserSchema = Dry::Schema.JSON do
  required(:name).filled(:str?)
  required(:email).filled(:str?)
end

app.post "/users", request_schema: UserSchema do |request|
  { id: 1, name: request[:body]["name"] }
end
```

Also supports raw JSON Schema objects and dry-struct schemas.

## Dependency Injection

Inject dependencies as keyword parameters:

```ruby
app.provide("config", { "db_url" => "postgresql://localhost" })
app.provide("db", depends_on: ["config"], singleton: true) { |config:| Pool.new(config) }

app.get "/data" do |request, config:, db:|
  { url: config["db_url"] }
end
```

## Responses

Return a Hash, String, or Response object:

```ruby
# Simple hash (auto-serialized to JSON)
app.get "/hello" do
  { message: "Hello, World!" }
end

# Custom status and headers
app.post "/users" do |request|
  Spikard::Response.new(
    content: { id: 1 },
    status_code: 201,
    headers: { "X-Custom" => "value" }
  )
end

# Streaming response
app.get "/stream" do
  stream = Enumerator.new { |y| y << "data" }
  Spikard::StreamingResponse.new(stream)
end

# File uploads
app.post "/upload", file_params: true do |request|
  file = request[:body]["file"]
  { filename: file.filename, size: file.size }
end
```

## Configuration

Configure the server with middleware options:

```ruby
config = Spikard::ServerConfig.new(
  host: "0.0.0.0",
  port: 8080,
  workers: 4,
  compression: Spikard::CompressionConfig.new(gzip: true, brotli: true),
  rate_limit: Spikard::RateLimitConfig.new(per_second: 100),
  jwt: Spikard::JwtConfig.new(secret: "key", algorithm: "HS256"),
  static_files: Spikard::StaticFilesConfig.new(directory: "./public"),
  max_body_size: 10 * 1024 * 1024,
  request_timeout: 30
)

app.run(config: config)
```

See [Configuration Reference](lib/spikard/config.rb) for full options.

## Lifecycle Hooks

Execute logic at key points in the request lifecycle:

```ruby
app.on_request { |req| puts "#{req[:method]} #{req[:path]}" }
app.pre_validation { |req| req }
app.pre_handler { |req| req }
app.on_response { |res| res }
app.on_error { |res| res }
```

Return a Request/Response object to continue, or a Response to short-circuit.

## Real-Time Communication

**WebSockets:**

```ruby
class ChatHandler < Spikard::WebSocketHandler
  def handle_message(message) = { echo: message }
end

app.websocket("/chat") { ChatHandler.new }
```

**Server-Sent Events:**

```ruby
class Events < Spikard::SseEventProducer
  def next_event = Spikard::SseEvent.new(data: { msg: "Hello" })
end

app.sse("/events") { Events.new }
```

## Background Tasks

Offload work after sending response:

```ruby
app.post "/process" do
  Spikard::Background.run { perform_long_task }
  { status: "processing" }
end
```

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

## Type Safety with RBS & Steep

Full RBS type definitions are included in `sig/spikard.rbs`. Use Steep for type checking:

```bash
bundle exec steep check
```

## Performance

Built with zero-overhead FFI via Magnus and rb-sys. Benchmark results: ~8,000 RPS, ~6.5ms latency at 50 concurrency. See [benchmarks](../../snapshots/benchmarks/) for full results.

## Learn More

**Examples & Code Generation:**
- [Runnable Examples](../../examples/) - Ruby, Python, TypeScript, PHP, and WASM
- [Code Generation Guide](../../examples/README.md) - Generate from OpenAPI, GraphQL, AsyncAPI, OpenRPC

**Documentation:**
- [Type Definitions (RBS)](sig/spikard.rbs) - Full Steep type signatures
- [Main README](../../README.md) - Multi-language ecosystem and feature overview
- [Architecture Decisions](../../docs/adr/) - Design choices and patterns

**Other Languages:**
- [Python (PyPI)](https://pypi.org/project/spikard/)
- [TypeScript (npm)](https://www.npmjs.com/package/spikard)
- [PHP (Packagist)](https://packagist.org/packages/spikard/spikard)
- [Rust (Crates.io)](https://crates.io/crates/spikard)

## License

MIT - See [LICENSE](../../LICENSE) for details

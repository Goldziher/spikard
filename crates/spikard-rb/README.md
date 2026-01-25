# spikard-rb

High-performance Ruby bindings for Spikard HTTP framework via Magnus.

## Status & Badges

[![RubyGems](https://img.shields.io/gem/v/spikard.svg)](https://rubygems.org/gems/spikard)
[![Downloads](https://img.shields.io/gem/dt/spikard.svg)](https://rubygems.org/gems/spikard)
[![Crates.io](https://img.shields.io/crates/v/spikard-rb.svg)](https://crates.io/crates/spikard-rb)
[![Documentation](https://docs.rs/spikard-rb/badge.svg)](https://docs.rs/spikard-rb)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Discord](https://img.shields.io/badge/Discord-Join%20our%20community-7289da)](https://discord.gg/pXxagNK2zN)

## Overview

High-performance Ruby web framework with a Rust core. Build REST APIs with Sinatra-style blocks backed by Axum and Tower-HTTP.

## Installation

**From source (currently):**

```bash
cd packages/ruby
bundle install
bundle exec rake ext:build
```

**Requirements:**
- Ruby 3.2-4.x
- Bundler
- Rust toolchain (for building native extension)

## Quick Start

```ruby
require "spikard"
require "dry-schema"

UserSchema = Dry::Schema.JSON do
  required(:name).filled(:str?)
  required(:email).filled(:str?)
end

app = Spikard::App.new

app.get "/users/:id" do |request|
  user_id = request[:path_params]["id"].to_i
  { id: user_id, name: "Alice" }
end

app.post "/users", request_schema: UserSchema do |request|
  body = request[:body]
  { id: 1, name: body["name"], email: body["email"] }
end

app.run(port: 8000)
```

## Request Hash Structure

Handlers receive a single `request` hash argument with the following keys:

- `:method` - HTTP method (String): `"GET"`, `"POST"`, etc.
- `:path` - URL path (String): `"/users/123"`
- `:path_params` - Path parameters (Hash): `{"id" => "123"}`
- `:query` - Query parameters (Hash): `{"search" => "ruby"}`
- `:raw_query` - Raw query multimap (Hash of Arrays)
- `:headers` - Request headers (Hash): `{"Authorization" => "Bearer..."}`
- `:cookies` - Request cookies (Hash): `{"session_id" => "..."}`
- `:body` - Parsed request body (Hash or nil)
- `:params` - Merged params from path, query, headers, and cookies

**Example:**

```ruby
app.get "/users/:id" do |request|
  user_id = request[:path_params]["id"]
  search = request[:query]["search"]
  auth = request[:headers]["Authorization"]

  { id: user_id, search: search }
end
```

## Route Registration

### HTTP Methods

```ruby
app.get "/path" do |request|
  # Handler code
  { method: request[:method] }
end

app.post "/path" do |request|
  { created: true }
end

app.put "/path" do |request|
  { updated: true }
end

app.patch "/path" do |request|
  { patched: true }
end

app.delete "/path" do |request|
  { deleted: true }
end

app.options "/path" do |request|
  { options: [] }
end

app.head "/path" do |request|
  # HEAD request
end

app.trace "/path" do |request|
  # TRACE request
end
```

### Path Parameters

```ruby
app.get "/users/:user_id" do |request|
  { user_id: request[:path_params]["user_id"].to_i }
end

app.get "/posts/:post_id/comments/:comment_id" do |request|
  {
    post_id: request[:path_params]["post_id"].to_i,
    comment_id: request[:path_params]["comment_id"].to_i
  }
end
```

### Query Parameters

```ruby
app.get "/search" do |request|
  q = request[:query]["q"]
  limit = (request[:query]["limit"] || "10").to_i
  { query: q, limit: limit }
end
```

## Validation

Spikard supports **dry-schema** and **raw JSON Schema objects**.

### With dry-schema

```ruby
require "dry-schema"
Dry::Schema.load_extensions(:json_schema)

UserSchema = Dry::Schema.JSON do
  required(:name).filled(:str?)
  required(:email).filled(:str?)
  required(:age).filled(:int?)
end

app.post "/users", request_schema: UserSchema do |request|
  # request[:body] is validated against schema
  { id: 1, name: request[:body]["name"] }
end
```

### With raw JSON Schema

```ruby
user_schema = {
  "type" => "object",
  "properties" => {
    "name" => { "type" => "string" },
    "email" => { "type" => "string", "format" => "email" }
  },
  "required" => ["name", "email"]
}

app.post "/users", request_schema: user_schema do |request|
  { id: 1, name: request[:body]["name"], email: request[:body]["email"] }
end
```

### With dry-struct

```ruby
require "dry-struct"
require "dry-types"

module Types
  include Dry.Types()
end

class User < Dry::Struct
  attribute :name, Types::String
  attribute :email, Types::String
  attribute :age, Types::Integer
end

app.post "/users", request_schema: User do |request|
  # request[:body] validated as User
  { id: 1, name: request[:body]["name"] }
end
```

## Response Types

### Simple Hash Response

```ruby
app.get "/hello" do
  { message: "Hello, World!" }
end
```

### String Response

```ruby
app.get "/text" do
  "Plain text response"
end
```

### Full Response Object

```ruby
app.post "/users" do |request|
  Spikard::Response.new(
    content: { id: 1, name: request[:body]["name"] },
    status_code: 201,
    headers: { "X-Custom" => "value" }
  )
end
```

### Streaming Response

```ruby
app.get "/stream" do
  stream = Enumerator.new do |yielder|
    10.times do |i|
      yielder << "Chunk #{i}\n"
      sleep 0.1
    end
  end

  Spikard::StreamingResponse.new(
    stream,
    status_code: 200,
    headers: { "Content-Type" => "text/plain" }
  )
end
```

## File Uploads

```ruby
app.post "/upload", file_params: true do |request|
  file = request[:body]["file"]  # UploadFile instance

  {
    filename: file.filename,
    size: file.size,
    content_type: file.content_type,
    content: file.read
  }
end
```

## Configuration

```ruby
config = Spikard::ServerConfig.new(
  host: "0.0.0.0",
  port: 8080,
  workers: 4,
  enable_request_id: true,
  max_body_size: 10 * 1024 * 1024,  # 10 MB
  request_timeout: 30,
  compression: Spikard::CompressionConfig.new(
    gzip: true,
    brotli: true,
    quality: 6
  ),
  rate_limit: Spikard::RateLimitConfig.new(
    per_second: 100,
    burst: 200
  )
)

app.run(config: config)
```

### Middleware Configuration

**Compression:**

```ruby
compression = Spikard::CompressionConfig.new(
  gzip: true,
  brotli: true,
  min_size: 1024,
  quality: 6
)
```

**Rate Limiting:**

```ruby
rate_limit = Spikard::RateLimitConfig.new(
  per_second: 100,
  burst: 200,
  ip_based: true
)
```

**JWT Authentication:**

```ruby
jwt = Spikard::JwtConfig.new(
  secret: "your-secret-key",
  algorithm: "HS256",
  audience: ["api.example.com"],
  issuer: "auth.example.com",
  leeway: 30
)
```

**Static Files:**

```ruby
static = Spikard::StaticFilesConfig.new(
  directory: "./public",
  route_prefix: "/static",
  index_file: true,
  cache_control: "public, max-age=3600"
)
```

**OpenAPI Documentation:**

```ruby
openapi = Spikard::OpenApiConfig.new(
  enabled: true,
  title: "My API",
  version: "1.0.0",
  description: "API docs",
  swagger_ui_path: "/docs",
  redoc_path: "/redoc"
)
```

## Lifecycle Hooks

```ruby
app.on_request do |request|
  puts "#{request[:method]} #{request[:path]}"
  request
end

app.pre_validation do |request|
  if too_many_requests?
    Spikard::Response.new(
      content: { error: "Rate limit exceeded" },
      status_code: 429
    )
  else
    request
  end
end

app.pre_handler do |request|
  if invalid_token?(request[:headers]["Authorization"])
    Spikard::Response.new(
      content: { error: "Unauthorized" },
      status_code: 401
    )
  else
    request
  end
end

app.on_response do |response|
  response.headers["X-Frame-Options"] = "DENY"
  response
end

app.on_error do |response|
  puts "Error: #{response.status_code}"
  response
end
```

## WebSockets

```ruby
class ChatHandler < Spikard::WebSocketHandler
  def on_connect
    puts "Client connected"
  end

  def handle_message(message)
    # message is a Hash (parsed JSON)
    { echo: message, timestamp: Time.now.to_i }
  end

  def on_disconnect
    puts "Client disconnected"
  end
end

app.websocket("/chat") { ChatHandler.new }
```

## Server-Sent Events (SSE)

```ruby
class NotificationProducer < Spikard::SseEventProducer
  def initialize
    @count = 0
  end

  def on_connect
    puts "Client connected to SSE stream"
  end

  def next_event
    sleep 1

    return nil if @count >= 10  # End stream

    event = Spikard::SseEvent.new(
      data: { message: "Notification #{@count}" },
      event_type: "notification",
      id: @count.to_s,
      retry_ms: 3000
    )
    @count += 1
    event
  end

  def on_disconnect
    puts "Client disconnected from SSE"
  end
end

app.sse("/notifications") { NotificationProducer.new }
```

## Background Tasks

```ruby
app.post "/process" do |request|
  Spikard::Background.run do
    # Heavy processing after response
    ProcessData.perform(request[:path_params]["id"])
  end

  { status: "processing" }
end
```

## Testing

```ruby
require "spikard"

app = Spikard::App.new
app.get "/hello" do
  { message: "Hello, World!" }
end

client = Spikard::TestClient.new(app)

# HTTP requests
response = client.get("/hello", query: { name: "Alice" })
puts response.status_code  # => 200
puts response.json         # => { "message" => "Hello, World!" }

# POST with JSON
response = client.post("/users", json: { name: "Bob" })

# File upload
response = client.post("/upload", files: {
  file: ["test.txt", "content", "text/plain"]
})

# WebSocket
ws = client.websocket("/chat")
ws.send_json({ message: "hello" })
message = ws.receive_json
ws.close

# SSE
sse = client.sse("/events")
events = sse.events_as_json
puts events.length

# Cleanup
client.close
```

## Running the Server

```ruby
# Development
app.run(port: 8000)

# Production
config = Spikard::ServerConfig.new(
  host: "0.0.0.0",
  port: 8080,
  workers: 4
)
app.run(config: config)
```

## Type Safety with RBS

RBS type signatures are provided in `sig/spikard.rbs`:

```ruby
module Spikard
  class App
    def initialize: () -> void
    def get: (String, ?handler_name: String?, **untyped) { (untyped) -> untyped } -> Proc
    def post: (String, ?handler_name: String?, **untyped) { (untyped) -> untyped } -> Proc
    def run: (?config: ServerConfig | Hash[Symbol, untyped]?) -> void
  end

  class ServerConfig
    def initialize: (?host: String, ?port: Integer, **untyped) -> void
  end
end
```

Use with Steep for type checking:

```bash
bundle exec steep check
```

## Performance

Ruby bindings use:
- **Magnus** for zero-overhead FFI
- **rb-sys** for modern Ruby 3.2-4.x integration
- Idiomatic Ruby blocks and procs
- GC-safe handler storage

## Examples

See `/examples/ruby/` for more examples.

## Documentation

- [Main Project README](../../README.md)
- [Contributing Guide](../../CONTRIBUTING.md)
- [RBS Type Signatures](sig/spikard.rbs)

## License

MIT

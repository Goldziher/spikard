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

## Type Safety with RBS & Steep

Full RBS type definitions are included in `sig/spikard.rbs`. Use Steep for type checking:

```bash
bundle exec steep check
```

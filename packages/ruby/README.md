# Spikard for Ruby

Ruby bindings for Spikard’s Rust HTTP runtime. Sinatra-like blocks, Rack-style responses, and Magnus-backed performance.

## Install from source
```bash
cd packages/ruby
bundle install
bundle exec rake ext:build   # compiles the native extension (spikard_rb)
```

## Quick start
```ruby
require "spikard"

app = Spikard::App.new

app.get "/hello" do |_request|
  { message: "Hello, World!" }
end

app.post "/users/{id:int}" do |_request|
  { status: "created" }
end

if $PROGRAM_NAME == __FILE__
  config = Spikard::Config::ServerConfig.new(host: "0.0.0.0", port: 8000)
  app.run(config: config)
end
```
- Route DSL mirrors the other bindings (`get`, `post`, `put`, `patch`, `delete`, etc.).
- Config objects enable compression, rate limits, timeouts, body limits, static files, and request IDs.
- WebSocket and SSE helpers are available via `app.websocket` and `app.sse`.
- Request objects (provided by the native runtime) expose headers, params, query, and parsed bodies when you need them.

## Testing
Exercise handlers without opening sockets using the bundled test client:
```ruby
client = Spikard::TestClient.new(app)
response = client.get("/hello?name=Ada")
puts response.status_code  # => 200
puts response.json         # => {"message"=>"Hello, Ada!"}
```
Run the suite (shared fixtures from `testing_data/`) with:
```bash
bundle exec rake spec
```

## Code generation
Generate Ruby-ready routes and tests from specs with the CLI:
```bash
spikard generate openapi --fixtures ../../testing_data --output ./generated
spikard generate asyncapi --fixtures ../../testing_data/sse --output ./generated
```

## Development notes
- Ruby-facing code lives under `lib/spikard/`; native bindings sit in `crates/spikard-rb`.
- Keep fixture updates synchronized with the shared e2e suite in `e2e/ruby`.
- The API favors Ruby idioms—blocks for handlers, symbols/strings for config—while matching behavior across languages.

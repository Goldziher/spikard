# Ruby API Reference

The Ruby binding uses magnus to expose the Rust runtime with a Ruby-friendly DSL.

## Gem
- Install: `gem install spikard`
- Require: `require "spikard"`

## Core Types
- `Spikard::App` – register routes and start the server
- Handler args – receive path params, query params, and body (or use handler wrappers)
- Lifecycle hooks (`on_request`, `pre_validation`, `pre_handler`, `on_response`, `on_error`)
- Dependency injection via `app.provide` and keyword parameters

## Routing
```ruby
require "spikard"

app = Spikard::App.new

app.get("/health") do |_params, _query, _body|
  { status: "ok" }
end

app.run(port: 8000)
```

## Middleware
```ruby
app = Spikard::App.new

app.on_request do |request|
  puts "#{request[:method]} #{request[:path]}"
  request
end
```

## Dependency Injection
```ruby
app.provide("config", { "db_url" => "postgresql://localhost/app" })
app.provide("db_pool", depends_on: ["config"], singleton: true) { |config:| config["db_url"] }
```

## Validation
Ruby handlers can rely on RBS signatures and runtime validation hooks. DTO generation from OpenAPI/AsyncAPI keeps Ruby types in sync with other bindings.

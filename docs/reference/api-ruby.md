# Ruby API Reference

The Ruby binding uses magnus to expose the Rust runtime with a Ruby-friendly DSL.

## Gem
- Install: `gem install spikard`
- Require: `require "spikard"`

## Core Types
- `Spikard::App` – register routes, middleware, and start the server
- `ctx` – provides `params`, `query`, `headers`, `cookies`, and parsed request bodies

## Routing
```ruby
require "spikard"

App = Spikard::App.new

App.get("/health") do |_ctx|
  { status: "ok" }
end

App.listen(port: 8000)
```

## Middleware
```ruby
App.use do |ctx, next_middleware|
  puts "#{ctx.method} #{ctx.path}"
  next_middleware.call
end
```

## Validation
Ruby handlers can rely on RBS signatures and runtime validation hooks. DTO generation from OpenAPI/AsyncAPI keeps Ruby types in sync with other bindings.

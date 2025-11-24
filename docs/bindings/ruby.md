# Ruby Binding

Ruby binding built on Magnus. Handlers receive a request hash; dry-schema provides validation, and responses are plain Ruby hashes/objects.

## Quickstart

```ruby
require "spikard"

app = Spikard::App.new

app.get "/health" do |request|
  { status: "ok", path: request[:path] }
end

app.run(port: 8000)
```

## Validation (dry-schema)

```ruby
require "dry-schema"
Dry::Schema.load_extensions(:json_schema)

UserSchema = Dry::Schema.JSON do
  required(:name).filled(:str?)
  required(:email).filled(:str?)
end

app.post "/users", request_schema: UserSchema do |request|
  body = request[:body]
  { id: 1, name: body["name"], email: body["email"] }
end
```

## Requests & Responses
- Request hash keys: `:method`, `:path`, `:path_params`, `:query`, `:raw_query`, `:headers`, `:cookies`, `:body`, `:params` (merged).
- Return Ruby hashes/arrays; the runtime serializes.

## Middleware

```ruby
app.use do |ctx, next_middleware|
  puts "#{ctx.method} #{ctx.path}"
  next_middleware.call
end
```

## Deployment
- Local: `ruby app.rb` or run via the Rust CLI.
- Native extension requires Ruby 3.2+ and a Rust toolchain; ensure `bundle exec rake ext:build` has been run.

## Troubleshooting
- If build fails, confirm Rust is on PATH and `bundle config set build.spikard --with-cflags="-std=c++17"` where needed.
- For schema errors, ensure `dry-schema` is installed and extension `:json_schema` loaded.

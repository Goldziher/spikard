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

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

app.run(port: 8000)
```

```ruby
require "spikard"

App = Spikard::App.new

App.get("/users/:id") do |ctx|
  { id: ctx.params[:id].to_i, name: "Alice" }
end

App.post("/users") do |ctx|
  user = ctx.json
  { id: user["id"], name: user["name"] }
end

App.run(port: 8000)
```

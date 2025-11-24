```ruby
require "spikard"

App = Spikard::App.new

App.get("/users/:id") do |ctx|
  { id: ctx.params[:id].to_i, name: "Alice" }
end

App.run(port: 8000)
```

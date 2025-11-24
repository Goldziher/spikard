```ruby
require "spikard"

app = Spikard::App.new

app.get("/health") { |_params, _query, _body| { status: "ok" } }
app.post("/users") { |_params, _query, body| body }
```

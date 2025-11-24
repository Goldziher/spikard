```ruby
require "spikard"

App = Spikard::App.new

App.get("/health") { { status: "ok" } }
App.post("/users") { |ctx| ctx.json }
```

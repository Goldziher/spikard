```elixir
require Logger

app = Spikard.App.new()

app =
  Spikard.App.get(app, "/hello", fn _conn ->
    %{"message" => "Hello, world!"}
  end)

Spikard.App.run(app)
```

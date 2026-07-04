```elixir
app = Spikard.App.new()

app =
  Spikard.App.get(app, "/", fn _conn ->
    %{"status" => "running"}
  end)

# Run with default ServerConfig (127.0.0.1:8000)
Spikard.App.run(app)
```

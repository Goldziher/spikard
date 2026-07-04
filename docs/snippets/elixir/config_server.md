```elixir
{:ok, config} = Spikard.ServerConfig.default()
config = Map.put(config, :host, "0.0.0.0")
config = Map.put(config, :port, 8080)
config = Map.put(config, :workers, 4)

app = Spikard.App.new()
app = Spikard.App.config(app, config)

app =
  Spikard.App.get(app, "/", fn _conn ->
    %{"status" => "ok"}
  end)

Spikard.App.run(app)
```

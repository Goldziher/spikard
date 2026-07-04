```elixir
host = System.get_env("HOST", "127.0.0.1")
port = System.get_env("PORT", "8000") |> String.to_integer()
workers = System.get_env("WORKERS", "1") |> String.to_integer()

{:ok, config} = Spikard.ServerConfig.default()
config = Map.put(config, :host, host)
config = Map.put(config, :port, port)
config = Map.put(config, :workers, workers)

app = Spikard.App.new()
app = Spikard.App.config(app, config)

app =
  Spikard.App.get(app, "/env", fn _conn ->
    %{"host" => host, "port" => port, "workers" => workers}
  end)

Spikard.App.run(app)
```

```elixir
{:ok, compression} = Spikard.CompressionConfig.default()
compression = Map.put(compression, :gzip, true)
compression = Map.put(compression, :min_size, 1024)

{:ok, rate_limit} = Spikard.RateLimitConfig.default()
rate_limit = Map.put(rate_limit, :per_second, 100)
rate_limit = Map.put(rate_limit, :burst, 200)

{:ok, config} = Spikard.ServerConfig.default()
config = Map.put(config, :host, "0.0.0.0")
config = Map.put(config, :port, 8080)
config = Map.put(config, :workers, 8)
config = Map.put(config, :request_timeout, 30)
config = Map.put(config, :compression, compression)
config = Map.put(config, :rate_limit, rate_limit)

app = Spikard.App.new()
app = Spikard.App.config(app, config)

app =
  Spikard.App.get(app, "/", fn _conn ->
    %{"status" => "production"}
  end)

Spikard.App.run(app)
```

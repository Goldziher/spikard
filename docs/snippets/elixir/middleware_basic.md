```elixir
app = Spikard.App.new()

logging_middleware = fn conn ->
  IO.inspect(%{
    "method" => conn.method,
    "path" => conn.path,
    "timestamp" => System.os_time(:second)
  })
  conn
end

app =
  Spikard.App.get(app, "/", fn conn ->
    conn = logging_middleware.(conn)
    %{"status" => "ok"}
  end)

Spikard.App.run(app)
```

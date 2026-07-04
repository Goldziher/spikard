```elixir
app = Spikard.App.new()

app =
  Spikard.App.get(app, "/ok", fn _conn ->
    %{"status" => "ok"}
  end)

app =
  Spikard.App.get(app, "/error", fn _conn ->
    %{"error" => "Something went wrong", "code" => "ERR_INTERNAL"}
  end)

app =
  Spikard.App.post(app, "/data", fn conn ->
    %{"received" => conn.body, "timestamp" => System.os_time(:second)}
  end)

Spikard.App.run(app)
```

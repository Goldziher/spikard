```elixir
app = Spikard.App.new()

app =
  Spikard.App.get(app, "/health", fn _conn ->
    %{"status" => "ok"}
  end)

app =
  Spikard.App.post(app, "/users", fn conn ->
    %{"id" => 1, "name" => conn.body["name"]}
  end)

Spikard.App.run(app)
```

```elixir
app = Spikard.App.new()

app =
  Spikard.App.get(app, "/stream", fn _conn ->
    body = Enum.map_join(0..2, fn tick -> Jason.encode!(%{"tick" => tick}) <> "\n" end)

    %Spikard.Response{
      status_code: 200,
      headers: %{"content-type" => "application/x-ndjson"},
      content: body
    }
  end)

Spikard.App.run(app)
```

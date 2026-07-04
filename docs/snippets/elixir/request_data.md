```elixir
app = Spikard.App.new()

app =
  Spikard.App.post(app, "/orders/:order_id", fn conn ->
    order_id = Map.get(conn.path_params, "order_id")
    verbose = Map.get(conn.query_params, "verbose", "false")
    auth_header = Map.get(conn.headers, "authorization")
    session_cookie = Map.get(conn.cookies, "session")
    body = conn.body || %{}

    %{
      "order_id" => order_id,
      "verbose" => verbose,
      "auth" => auth_header,
      "session" => session_cookie,
      "item" => body["item"],
      "quantity" => body["quantity"]
    }
  end)

Spikard.App.run(app)
```

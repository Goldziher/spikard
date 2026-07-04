```elixir
app = Spikard.App.new()

app =
  Spikard.App.get(app, "/items/:id", fn conn ->
    item_id = Map.get(conn.path_params, "id")
    
    case Integer.parse(item_id || "") do
      {id, ""} when id > 0 ->
        %{"item_id" => id, "name" => "Item #{id}"}
      _ ->
        %{"error" => "Invalid item ID", "code" => "INVALID_PATH_PARAM"}
    end
  end)

Spikard.App.run(app)
```

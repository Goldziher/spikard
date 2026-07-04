```elixir
app = Spikard.App.new()

check_jwt = fn conn ->
  case Map.get(conn.headers, "authorization") do
    "Bearer " <> token ->
      # In production, verify token signature and claims
      case String.length(token) do
        len when len > 10 -> {:ok, token}
        _ -> {:error, "Invalid token"}
      end
    _ ->
      {:error, "Missing authorization header"}
  end
end

app =
  Spikard.App.post(app, "/protected", fn conn ->
    case check_jwt.(conn) do
      {:ok, _token} ->
        %{"status" => "authenticated", "message" => "Access granted"}
      {:error, reason} ->
        %{"error" => reason, "status" => "unauthorized", "code" => 401}
    end
  end)

Spikard.App.run(app)
```

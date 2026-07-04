```elixir
{:ok, jwt_config} = Spikard.JwtConfig.default()
jwt_config = %Spikard.JwtConfig{
  secret: "your-secret-key",
  algorithm: "HS256",
  audience: ["api"],
  issuer: "your-issuer"
}

{:ok, config} = Spikard.ServerConfig.default()
config = Map.put(config, :jwt_auth, jwt_config)

app = Spikard.App.new()
app = Spikard.App.config(app, config)

verify_token = fn conn ->
  case Map.get(conn.headers, "authorization") do
    "Bearer " <> token ->
      # In production, verify against jwt_config
      {:ok, token}
    _ ->
      {:error, "Missing authorization"}
  end
end

app =
  Spikard.App.get(app, "/secure", fn conn ->
    case verify_token.(conn) do
      {:ok, _token} -> %{"access" => "granted"}
      {:error, reason} -> %{"error" => reason}
    end
  end)

Spikard.App.run(app)
```

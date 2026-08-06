```elixir
alias Spikard.WebSocketMessage

app = Spikard.App.new()

# The Elixir binding does not yet expose a dedicated WebSocket route
# registration (no App.websocket/3 or Method.ws/0); the upgrade handshake
# is negotiated by the Rust core. This handler models echo semantics using
# the message shapes documented on Spikard.WebSocketMessage.
app =
  Spikard.App.get(app, "/ws", fn conn ->
    case conn.body do
      %{"type" => "text", "value" => text} ->
        %{"type" => "text", "value" => text}

      _ ->
        %{"type" => "close", "code" => 1000, "reason" => "no message"}
    end
  end)

Spikard.App.run(app)
```

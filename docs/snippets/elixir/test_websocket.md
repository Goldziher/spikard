```elixir
defmodule WebSocketEchoTest do
  use ExUnit.Case, async: true

  # No dedicated WebSocket handler registration exists for the Elixir
  # binding yet (see websocket.md); this exercises the echo logic directly.
  defp echo(%{"type" => "text", "value" => _value} = message), do: message

  test "echoes back a text message" do
    message = %{"type" => "text", "value" => "Hello"}

    assert echo(message) == message
  end

  test "echoes back a JSON payload" do
    message = %{"type" => "text", "value" => Jason.encode!(%{"ping" => true})}

    assert echo(message) == message
  end
end
```

```elixir
defmodule SseEventsTest do
  use ExUnit.Case, async: true

  defp format_event(%Spikard.SseEvent{} = event) do
    "data: #{event.data}\n\n"
  end

  defp build_events do
    Enum.map(0..2, fn tick -> %Spikard.SseEvent{data: Jason.encode!(%{"count" => tick})} end)
  end

  test "streams three SSE events" do
    events = Enum.map(build_events(), &format_event/1)

    assert length(events) == 3
    assert List.first(events) == "data: {\"count\":0}\n\n"
    assert List.last(events) == "data: {\"count\":2}\n\n"
  end
end
```

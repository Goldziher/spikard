```elixir
app = Spikard.App.new()

format_event = fn %Spikard.SseEvent{} = event ->
  [
    if(event.event_type, do: "event: #{event.event_type}\n", else: ""),
    "data: #{event.data}\n",
    if(event.id, do: "id: #{event.id}\n", else: ""),
    if(event.retry, do: "retry: #{event.retry}\n", else: ""),
    "\n"
  ]
  |> IO.iodata_to_binary()
end

app =
  Spikard.App.get(app, "/events", fn _conn ->
    body =
      Enum.map_join(0..2, fn tick ->
        format_event.(%Spikard.SseEvent{data: Jason.encode!(%{"tick" => tick})})
      end)

    %Spikard.Response{
      status_code: 200,
      headers: %{"content-type" => "text/event-stream"},
      content: body
    }
  end)

Spikard.App.run(app)
```

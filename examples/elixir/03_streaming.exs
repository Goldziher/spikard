# Streaming Example
#
# Demonstrates streaming responses and Server-Sent Events.

defmodule StreamApp.Router do
  use Spikard.Router

  get "/stream", &stream_data/1
  sse "/events", StreamApp.TickProducer
  get "/", &index/1

  defp index(_request) do
    Spikard.Response.html("""
    <h1>Streaming Examples</h1>
    <ul>
      <li><a href="/stream">Streaming response</a></li>
      <li><a href="/events">Server-Sent Events</a></li>
    </ul>
    """)
  end

  defp stream_data(_request) do
    stream =
      Stream.interval(500)
      |> Stream.take(10)
      |> Stream.map(&"chunk #{&1}\n")

    Spikard.Response.stream(stream)
  end
end

defmodule StreamApp.TickProducer do
  use Spikard.Sse.Producer

  def init(_opts), do: {:ok, 0}

  def next_event(count) when count < 20 do
    event = %Spikard.Sse.Event{
      data: %{tick: count, timestamp: DateTime.utc_now() |> DateTime.to_iso8601()},
      event: "tick",
      id: "#{count}"
    }

    Process.sleep(1000)
    {:ok, event, count + 1}
  end

  def next_event(_count), do: :done
end

IO.puts("Starting streaming example on http://127.0.0.1:8002")
{:ok, _server} = Spikard.start(StreamApp.Router, port: 8002)
Process.sleep(:infinity)

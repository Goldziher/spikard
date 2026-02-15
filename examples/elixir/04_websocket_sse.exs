# WebSocket & SSE Example
#
# Demonstrates WebSocket bidirectional communication
# and Server-Sent Events for real-time features.

defmodule RealtimeApp.Router do
  use Spikard.Router

  get "/", &index/1
  websocket "/chat", RealtimeApp.ChatHandler
  sse "/notifications", RealtimeApp.NotificationProducer
end

defmodule RealtimeApp.ChatHandler do
  use Spikard.WebSocket

  def handle_connect(_ws, _opts) do
    IO.puts("Client connected to chat")
    {:ok, %{messages: [], connected_at: DateTime.utc_now()}}
  end

  def handle_message(message, state) do
    IO.puts("Received: #{inspect(message)}")
    new_state = Map.update(state, :messages, [message], &[message | &1])

    reply = %{
      type: "echo",
      data: message,
      count: length(new_state.messages)
    }

    {:reply, reply, new_state}
  end

  def handle_disconnect(_ws, state) do
    IO.puts("Client disconnected. Total messages: #{length(state.messages)}")
    :ok
  end
end

defmodule RealtimeApp.NotificationProducer do
  use Spikard.Sse.Producer

  def init(_opts), do: {:ok, 0}

  def next_event(count) when count < 50 do
    event = %Spikard.Sse.Event{
      data: %{
        id: count,
        message: "Notification ##{count}",
        timestamp: DateTime.utc_now() |> DateTime.to_iso8601()
      },
      event: "notification",
      id: "notif-#{count}"
    }

    Process.sleep(2000)
    {:ok, event, count + 1}
  end

  def next_event(_count), do: :done
end

defmodule RealtimeApp.Handlers do
  def index(_request) do
    Spikard.Response.html("""
    <h1>Real-time Examples</h1>
    <ul>
      <li>WebSocket: ws://localhost:8003/chat</li>
      <li>SSE: <a href="/notifications">http://localhost:8003/notifications</a></li>
    </ul>
    """)
  end
end

IO.puts("Starting real-time example on http://127.0.0.1:8003")
{:ok, _server} = Spikard.start(RealtimeApp.Router, port: 8003)
Process.sleep(:infinity)

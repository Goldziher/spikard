defmodule Spikard.Sse do
  @moduledoc """
  Server-Sent Events (SSE) support for Spikard.

  This module provides infrastructure for streaming events to clients using the
  Server-Sent Events protocol. SSE is ideal for real-time data feeds, notifications,
  and other server-push scenarios.

  ## Overview

  SSE endpoints are implemented using the `Spikard.Sse.Producer` behaviour. A producer
  generates a stream of events that are sent to connected clients.

  ## Quick Start

      defmodule MyApp.EventProducer do
        use Spikard.Sse.Producer

        def init(_opts) do
          {:ok, 0}
        end

        def next_event(count) when count < 10 do
          {:ok, %Spikard.Sse.Event{data: count}, count + 1}
        end

        def next_event(_) do
          :done
        end
      end

      # In your router
      sse "/events", MyApp.EventProducer

  ## Event Format

  SSE events follow the standard SSE format with optional fields:

      %Spikard.Sse.Event{
        data: "Hello",        # Required: event data
        event: "message",     # Optional: event type
        id: "123"            # Optional: event ID
      }

  ## Producer Callbacks

  Producers implement the `Spikard.Sse.Producer` behaviour:

  - `init/1` - Initialize producer state
  - `next_event/1` - Generate the next event
  - `on_connect/1` - Optional callback when client connects
  - `on_disconnect/1` - Optional callback when client disconnects
  """
end

defmodule Spikard.Sse.Event do
  @moduledoc """
  Represents a Server-Sent Event.

  ## Fields

  - `data` - The event data (required). Can be any term that can be serialized.
  - `event` - The event type (optional). Clients can filter on this.
  - `id` - The event ID (optional). Clients use this for reconnection.

  ## Examples

      # Simple event with just data
      %Event{data: "Hello World"}

      # Event with type
      %Event{data: %{user: "alice"}, event: "user_joined"}

      # Event with ID for reconnection support
      %Event{data: "message", event: "chat", id: "123"}
  """

  @type t :: %__MODULE__{
          data: term(),
          event: String.t() | nil,
          id: String.t() | nil
        }

  defstruct [:data, :event, :id]

  @doc """
  Convert an Event to SSE format string.

  ## Examples

      iex> event = %Spikard.Sse.Event{data: "hello", event: "msg", id: "1"}
      iex> Spikard.Sse.Event.to_sse(event)
      "event: msg\\nid: 1\\ndata: hello\\n\\n"
  """
  @spec to_sse(t()) :: String.t()
  def to_sse(%__MODULE__{} = event) do
    lines = []

    # Add event type if present
    lines =
      if event.event do
        lines ++ ["event: #{event.event}\n"]
      else
        lines
      end

    # Add ID if present
    lines =
      if event.id do
        lines ++ ["id: #{event.id}\n"]
      else
        lines
      end

    # Add data lines (split multiline data)
    data_str = to_string(event.data)

    data_lines =
      data_str
      |> String.split("\n")
      |> Enum.map(&"data: #{&1}\n")

    lines = lines ++ data_lines

    # End with blank line
    Enum.join(lines, "") <> "\n"
  end
end

defmodule Spikard.Sse.Producer do
  @moduledoc """
  Behaviour for SSE event producers.

  Implement this behaviour to create custom SSE event streams. The producer
  maintains state between events and generates events on demand.

  ## Callbacks

  - `init/1` - Initialize the producer with options. Return `{:ok, state}` or `{:error, reason}`.
  - `next_event/1` - Generate the next event. Return `{:ok, event, new_state}` or `:done`.
  - `on_connect/1` - Called when a client connects (optional).
  - `on_disconnect/1` - Called when a client disconnects (optional).

  ## Example

      defmodule MyProducer do
        use Spikard.Sse.Producer

        def init(_opts) do
          {:ok, 0}
        end

        def next_event(count) when count < 100 do
          event = %Spikard.Sse.Event{
            data: Jason.encode!(%{count: count}),
            event: "tick"
          }
          {:ok, event, count + 1}
        end

        def next_event(_count) do
          :done
        end

        def on_connect(_opts) do
          IO.puts("Client connected!")
          :ok
        end

        def on_disconnect(_opts) do
          IO.puts("Client disconnected!")
          :ok
        end
      end
  """

  alias Spikard.Sse.Event

  @doc """
  Initialize the producer with the given options.

  Return `{:ok, state}` to start the producer with initial state,
  or `{:error, reason}` to fail initialization.
  """
  @callback init(opts :: term()) :: {:ok, state :: term()} | {:error, reason :: term()}

  @doc """
  Generate the next event in the stream.

  Return `{:ok, event, new_state}` to send an event and continue,
  or `:done` to end the stream.
  """
  @callback next_event(state :: term()) ::
              {:ok, Event.t(), new_state :: term()} | :done | :error

  @doc """
  Called when a client connects to the SSE endpoint.

  This callback is optional. The default implementation does nothing.
  """
  @callback on_connect(opts :: term()) :: :ok | {:error, term()}

  @doc """
  Called when a client disconnects from the SSE endpoint.

  This callback is optional. The default implementation does nothing.
  """
  @callback on_disconnect(opts :: term()) :: :ok | {:error, term()}

  @optional_callbacks on_connect: 1, on_disconnect: 1

  @doc false
  defmacro __using__(_opts) do
    quote do
      @behaviour Spikard.Sse.Producer

      @doc false
      def on_connect(_opts), do: :ok

      @doc false
      def on_disconnect(_opts), do: :ok

      defoverridable on_connect: 1, on_disconnect: 1
    end
  end

  @doc """
  Placeholder for init/1 in behaviour modules.
  """
  def init(_opts), do: {:ok, nil}
end

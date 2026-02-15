defmodule Spikard.WebSocket do
  @moduledoc """
  WebSocket handler behaviour for Spikard.

  Defines the callback interface for implementing WebSocket handlers in Spikard.
  Handlers using this module will be called to process WebSocket connection lifecycle
  events and incoming messages.

  ## Example

  ```
  defmodule MyWebSocketHandler do
    use Spikard.WebSocket
    require Logger

    @doc "Called when a WebSocket connection is established"
    def handle_connect(_ws, _opts) do
      Logger.info("WebSocket connected")
      {:ok, %{count: 0}}
    end

    @doc "Called for each incoming message"
    def handle_message(_msg, state) do
      new_state = Map.update(state, :count, 1, &(&1 + 1))
      {:reply, _msg, new_state}
    end

    @doc "Called when the connection is closed"
    def handle_disconnect(_ws, _state) do
      :ok
    end
  end
  ```

  ## Callbacks

  - `handle_connect/2` - Called when a WebSocket connection is established
  - `handle_message/2` - Called for each incoming message
  - `handle_disconnect/2` - Called when the connection is closed

  ## Return Values

  ### handle_connect/2

  - `{:ok, state}` - Accept the connection and initialize state
  - `{:error, reason}` - Reject the connection

  ### handle_message/2

  - `{:reply, message, state}` - Send a message back to the client
  - `{:noreply, state}` - Don't send anything
  - `{:error, reason}` - Close the connection with an error

  ### handle_disconnect/2

  - `:ok` - Acknowledge disconnection
  - `{:error, reason}` - Log disconnection error (won't affect connection)
  """

  @callback handle_connect(ws :: term(), opts :: map() | list()) ::
              {:ok, state :: term()} | {:error, reason :: term()}

  @callback handle_message(message :: term(), state :: term()) ::
              {:reply, message :: term(), state :: term()}
              | {:noreply, state :: term()}
              | {:error, reason :: term()}

  @callback handle_disconnect(ws :: term(), state :: term()) ::
              :ok | {:error, reason :: term()}

  @doc """
  Send a message to a WebSocket client.

  Can be called from within a WebSocket handler to send messages back to the client.

  ## Arguments

    - `ws` - WebSocket reference (received in handle_connect/2 or handle_message/2)
    - `message` - Message to send (can be text, binary, or any term that can be serialized)

  ## Examples

      def handle_connect(ws, _opts) do
        :ok = Spikard.WebSocket.send(ws, "Welcome!")
        {:ok, []}
      end
  """
  @spec send(term(), term()) :: :ok | {:error, term()}
  def send(ws_ref, message) do
    Spikard.WebSocket.Client.send(ws_ref, message)
  end

  @doc """
  Use the WebSocket behaviour in a module.

  Provides default implementations for all callbacks.
  """
  defmacro __using__(_opts) do
    quote do
      @behaviour Spikard.WebSocket

      # Default implementations - handlers can override these

      @impl true
      def handle_connect(_ws, _opts), do: {:ok, []}

      @impl true
      def handle_message(msg, state), do: {:reply, msg, state}

      @impl true
      def handle_disconnect(_ws, _state), do: :ok

      defoverridable handle_connect: 2, handle_message: 2, handle_disconnect: 2
    end
  end
end

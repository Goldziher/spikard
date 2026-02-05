defmodule Spikard.WebSocket.Client do
  @moduledoc """
  WebSocket client API for sending messages to connected clients.

  Provides the main interface for WebSocket handlers to send messages back to clients.
  """

  require Logger

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

      def handle_message(msg, state) do
        # Echo the message back
        Spikard.WebSocket.send(ws, msg)
        {:noreply, state}
      end
  """
  @spec send(term(), term()) :: :ok | {:error, term()}
  def send(ws_ref, message) do
    Logger.debug("WebSocket.Client.send: message=#{inspect(message)}")
    Spikard.Native.websocket_send(ws_ref, message)
  end

  @doc """
  Close a WebSocket connection.

  ## Arguments

    - `ws` - WebSocket reference

  ## Examples

      def handle_message(_msg, state) do
        {:error, "Connection closing"}
      end

      # In the handler, the connection will be closed
  """
  @spec close(term()) :: :ok | {:error, term()}
  def close(ws_ref) do
    Logger.debug("WebSocket.Client.close")
    Spikard.Native.websocket_close(ws_ref)
  end
end

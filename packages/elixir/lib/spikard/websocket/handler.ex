defmodule Spikard.WebSocket.Handler do
  @moduledoc """
  GenServer for managing individual WebSocket connections.

  This module spawns a dedicated GenServer for each WebSocket connection,
  handling message processing and state management.
  """

  use GenServer
  require Logger

  @doc """
  Start a new WebSocket handler for the given connection.
  """
  @spec start_link(module(), term(), map() | list()) :: GenServer.on_start()
  def start_link(handler_module, ws_ref, opts) do
    GenServer.start_link(__MODULE__, {handler_module, ws_ref, opts})
  end

  @doc """
  Send a message to a WebSocket client.

  This is called from the user's WebSocket handler to send messages back to the client.
  """
  @spec send_message(pid(), term()) :: :ok | {:error, term()}
  def send_message(pid, message) do
    GenServer.call(pid, {:send_message, message}, :infinity)
  end

  @doc """
  Close the WebSocket connection.
  """
  @spec close(pid()) :: :ok
  def close(pid) do
    GenServer.call(pid, :close, :infinity)
  end

  # Server Callbacks

  @impl true
  def init({handler_module, ws_ref, opts}) do
    Logger.debug("WebSocket.Handler init: module=#{inspect(handler_module)}, opts=#{inspect(opts)}")

    # Call the handler's connect callback
    case call_handler_callback(handler_module, :handle_connect, [ws_ref, opts]) do
      {:ok, state} ->
        Logger.debug("WebSocket connection established")
        {:ok, %{
          handler_module: handler_module,
          ws_ref: ws_ref,
          opts: opts,
          state: state,
          connected: true
        }}

      {:error, reason} ->
        Logger.warning("WebSocket connection rejected: #{inspect(reason)}")
        {:stop, {:connection_rejected, reason}}
    end
  end

  @impl true
  def handle_call({:send_message, message}, _from, state) do
    Logger.debug("WebSocket.Handler send_message: #{inspect(message)}")

    # Call the native function to send the message
    case Spikard.Native.websocket_send(state.ws_ref, message) do
      :ok ->
        {:reply, :ok, state}

      error ->
        Logger.error("Failed to send WebSocket message: #{inspect(error)}")
        {:reply, error, state}
    end
  end

  def handle_call(:close, _from, state) do
    Logger.debug("WebSocket.Handler closing connection")

    # Call the handler's disconnect callback
    try do
      call_handler_callback(state.handler_module, :handle_disconnect, [state.ws_ref, state.state])
    rescue
      e ->
        Logger.error("Error in handle_disconnect: #{inspect(e)}")
    end

    # Call native function to close the connection
    Spikard.Native.websocket_close(state.ws_ref)

    {:reply, :ok, %{state | connected: false}}
  end

  # Handle incoming messages from the WebSocket
  @impl true
  def handle_info({:websocket_message, message}, state) do
    Logger.debug("WebSocket.Handler received message: #{inspect(message)}")

    case call_handler_callback(state.handler_module, :handle_message, [message, state.state]) do
      {:reply, response_message, new_state} ->
        Logger.debug("Handler replied with: #{inspect(response_message)}")
        # Send the response back to the client
        case Spikard.Native.websocket_send(state.ws_ref, response_message) do
          :ok ->
            {:noreply, %{state | state: new_state}}

          error ->
            Logger.error("Failed to send WebSocket response: #{inspect(error)}")
            {:noreply, %{state | state: new_state}}
        end

      {:noreply, new_state} ->
        {:noreply, %{state | state: new_state}}

      {:error, reason} ->
        Logger.error("Handler returned error: #{inspect(reason)}")
        # Close the connection on handler error
        Spikard.Native.websocket_close(state.ws_ref)
        {:noreply, %{state | connected: false}}
    end
  end

  def handle_info({:websocket_closed, _reason}, state) do
    Logger.debug("WebSocket connection closed")

    # Call the handler's disconnect callback
    try do
      call_handler_callback(state.handler_module, :handle_disconnect, [state.ws_ref, state.state])
    rescue
      e ->
        Logger.error("Error in handle_disconnect: #{inspect(e)}")
    end

    {:stop, :normal, %{state | connected: false}}
  end

  def handle_info(msg, state) do
    Logger.warning("WebSocket.Handler received unexpected message: #{inspect(msg)}")
    {:noreply, state}
  end

  # Private Helpers

  @spec call_handler_callback(module(), atom(), list()) :: term()
  defp call_handler_callback(module, callback, args) do
    apply(module, callback, args)
  rescue
    e ->
      Logger.error("Error calling #{inspect(module)}.#{callback}: #{inspect(e)}")
      {:error, {Exception.message(e), __STACKTRACE__}}
  end
end

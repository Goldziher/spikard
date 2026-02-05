defmodule Spikard.Sse.ProducerServer do
  @moduledoc """
  GenServer that manages SSE event production and streaming.

  This GenServer coordinates with the Rust NIF to stream events to clients.
  It maintains the producer state and calls next_event/1 repeatedly to generate
  events that are sent to the client.

  The server is started automatically when an SSE request arrives and runs
  until the producer returns `:done` or an error occurs.
  """

  use GenServer
  require Logger

  alias Spikard.Sse.Event

  @type state :: %{
          producer_module: module(),
          producer_state: term(),
          client_pid: pid() | nil,
          request_id: String.t() | nil
        }

  # Client API

  @doc """
  Start the SSE producer server.

  ## Arguments

    - `producer_module` - Module implementing Spikard.Sse.Producer behaviour
    - `opts` - Options passed to the producer's init/1 callback

  ## Returns

    - `{:ok, pid}` - Successfully started
    - `{:error, reason}` - Failed to start
  """
  @spec start_link(module(), term()) :: GenServer.on_start()
  def start_link(producer_module, opts \\ []) when is_atom(producer_module) do
    GenServer.start_link(__MODULE__, {producer_module, opts})
  end

  @doc """
  Connect a client to the producer and start streaming events.

  Called by the NIF when an SSE request arrives. The server will stream
  events to the client via the NIF.

  ## Arguments

    - `server` - PID of the ProducerServer
    - `client_pid` - PID to send events to
    - `request_id` - Unique request identifier

  ## Returns

    - `:ok` - Stream started successfully
  """
  @spec connect(pid(), pid(), String.t()) :: :ok
  def connect(server, client_pid, request_id) do
    GenServer.cast(server, {:connect, client_pid, request_id})
  end

  @doc """
  Trigger the next event in the stream.

  Called by the NIF after successfully sending the previous event.
  Returns the next event to send, or :done when the stream is complete.
  """
  @spec next(pid()) :: {:ok, Event.t(), boolean()} | :done
  def next(server) do
    GenServer.call(server, :next_event, :infinity)
  end

  @doc """
  Notify the server that the client has disconnected.

  Called by the NIF when the client closes the connection.
  """
  @spec disconnect(pid()) :: :ok
  def disconnect(server) do
    GenServer.cast(server, :disconnect)
  end

  # Server Callbacks

  @impl true
  def init({producer_module, opts}) do
    case producer_module.init(opts) do
      {:ok, producer_state} ->
        {:ok,
         %{
           producer_module: producer_module,
           producer_state: producer_state,
           client_pid: nil,
           request_id: nil,
           opts: opts
         }}

      {:error, reason} ->
        Logger.error("SSE producer init failed: #{inspect(reason)}")
        {:stop, reason}
    end
  end

  @impl true
  def handle_cast({:connect, client_pid, request_id}, state) do
    Logger.debug("SSE ProducerServer connected for request #{request_id}")

    # Call the on_connect callback if defined
    case state.producer_module.on_connect(state.opts) do
      :ok ->
        {:noreply, %{state | client_pid: client_pid, request_id: request_id}}

      {:error, reason} ->
        Logger.warning("SSE on_connect callback failed: #{inspect(reason)}")
        {:noreply, state}
    end
  end

  @impl true
  def handle_cast(:disconnect, state) do
    Logger.debug("SSE ProducerServer disconnected for request #{state.request_id}")

    # Call the on_disconnect callback if defined
    case state.producer_module.on_disconnect(state.opts) do
      :ok ->
        :ok

      {:error, reason} ->
        Logger.warning("SSE on_disconnect callback failed: #{inspect(reason)}")
    end

    {:noreply, state}
  end

  @impl true
  def handle_call(:next_event, _from, state) do
    case state.producer_module.next_event(state.producer_state) do
      {:ok, event, new_state} ->
        Logger.debug("SSE next_event returned: #{inspect(event)}")
        {:reply, {:ok, event, false}, %{state | producer_state: new_state}}

      :done ->
        Logger.debug("SSE stream completed (done)")
        {:reply, :done, state}

      :error ->
        Logger.warning("SSE next_event returned error")
        {:reply, :done, state}

      other ->
        Logger.warning("SSE next_event returned unexpected value: #{inspect(other)}")
        {:reply, :done, state}
    end
  end

  @impl true
  def handle_info(msg, state) do
    Logger.warning("ProducerServer received unexpected message: #{inspect(msg)}")
    {:noreply, state}
  end
end

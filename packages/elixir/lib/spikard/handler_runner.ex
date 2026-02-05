defmodule Spikard.HandlerRunner do
  @moduledoc """
  GenServer that manages handler invocations for the Spikard HTTP server.

  This module receives request messages from the Rust NIF, invokes the appropriate
  Elixir handler function, and sends the response back to the NIF.

  The HandlerRunner is started automatically when a server is started and stores
  a map of handler_name => handler_function for route dispatch.
  """

  use GenServer
  require Logger

  @type handler_map :: %{String.t() => (Spikard.Request.t() -> map())}
  @type state :: %{
          handlers: handler_map()
        }

  # Client API

  @doc """
  Start the HandlerRunner GenServer with the given handlers.

  ## Arguments

    - `handlers` - Map of handler_name => handler_function

  ## Returns

    - `{:ok, pid}` - Successfully started
    - `{:error, reason}` - Failed to start
  """
  @spec start_link(handler_map()) :: GenServer.on_start()
  def start_link(handlers) when is_map(handlers) do
    GenServer.start_link(__MODULE__, handlers)
  end

  @doc """
  Invoke a handler with the given request data.

  This is called from the NIF when a request comes in. It runs the handler
  synchronously and returns the response.

  ## Arguments

    - `runner` - PID of the HandlerRunner
    - `handler_name` - Name of the handler to invoke
    - `request_map` - Request data as a map

  ## Returns

    - Response map with status, headers, body
  """
  @spec invoke(pid(), String.t(), map()) :: map()
  def invoke(runner, handler_name, request_map) do
    GenServer.call(runner, {:invoke, handler_name, request_map}, :infinity)
  end

  # Server Callbacks

  @impl true
  def init(handlers) do
    {:ok, %{handlers: handlers}}
  end

  @impl true
  def handle_call({:invoke, handler_name, request_map}, _from, state) do
    response = execute_handler(state.handlers, handler_name, request_map)
    {:reply, response, state}
  end

  # Handle async request messages from Rust NIF
  # Format: {:handle_request, request_id, handler_name, request_map}
  @impl true
  def handle_info({:handle_request, request_id, handler_name, request_map}, state) do
    Logger.debug("HandlerRunner received request #{request_id} for handler #{handler_name}")
    Logger.debug("Request map: #{inspect(request_map)}")

    # Execute the handler
    response = execute_handler(state.handlers, handler_name, request_map)

    Logger.debug("Handler response: #{inspect(response)}")

    # Deliver the response back to the waiting Rust handler
    result = Spikard.Native.deliver_handler_response(request_id, response)
    Logger.debug("Deliver result: #{inspect(result)}")

    {:noreply, state}
  end

  def handle_info(msg, state) do
    Logger.warning("HandlerRunner received unexpected message: #{inspect(msg)}")
    {:noreply, state}
  end

  # Private Functions

  @spec execute_handler(handler_map(), String.t(), map()) :: map()
  defp execute_handler(handlers, handler_name, request_map) do
    case Map.get(handlers, handler_name) do
      nil ->
        Logger.warning("Handler not found: #{handler_name}")
        error_response(404, "Handler not found: #{handler_name}")

      handler when is_function(handler, 1) ->
        try do
          request = Spikard.Request.from_map(request_map)
          result = handler.(request)
          normalize_response(result)
        rescue
          e ->
            Logger.error("Handler #{handler_name} raised: #{inspect(e)}")
            error_response(500, "Handler error: #{Exception.message(e)}")
        catch
          kind, reason ->
            Logger.error("Handler #{handler_name} threw #{kind}: #{inspect(reason)}")
            error_response(500, "Handler error: #{inspect(reason)}")
        end

      other ->
        Logger.warning("Invalid handler type for #{handler_name}: #{inspect(other)}")
        error_response(500, "Invalid handler configuration")
    end
  end

  @spec normalize_response(term()) :: map()
  defp normalize_response(response) when is_map(response) do
    %{
      "status" => Map.get(response, :status, Map.get(response, "status", 200)),
      "headers" => normalize_headers(Map.get(response, :headers, Map.get(response, "headers", %{}))),
      "body" => Map.get(response, :body, Map.get(response, "body"))
    }
  end

  defp normalize_response(response) when is_binary(response) do
    %{"status" => 200, "headers" => %{}, "body" => response}
  end

  defp normalize_response(response) do
    %{"status" => 200, "headers" => %{}, "body" => response}
  end

  @spec normalize_headers(term()) :: map()
  defp normalize_headers(headers) when is_list(headers) do
    headers
    |> Enum.map(fn
      {k, v} -> {to_string(k), to_string(v)}
      other -> other
    end)
    |> Enum.into(%{})
  end

  defp normalize_headers(headers) when is_map(headers) do
    headers
    |> Enum.map(fn {k, v} -> {to_string(k), to_string(v)} end)
    |> Enum.into(%{})
  end

  defp normalize_headers(_), do: %{}

  @spec error_response(integer(), String.t()) :: map()
  defp error_response(status, message) do
    %{
      "status" => status,
      "headers" => %{"content-type" => "application/json"},
      "body" => Jason.encode!(%{error: message})
    }
  end
end

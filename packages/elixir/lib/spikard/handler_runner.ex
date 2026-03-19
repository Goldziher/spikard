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
  alias Spikard.Grpc

  @type handler_map :: %{String.t() => (Spikard.Request.t() -> map())}
  @type grpc_handler_map ::
          %{String.t() => %{String.t() => {Grpc.Service.rpc_mode(), function()}}}
  @type lifecycle_hooks :: %{
          String.t() => [function()]
        }
  @type dependencies :: [Spikard.DI.dependency()]
  @type singleton_cache :: %{String.t() => term()}
  @type state :: %{
          handlers: handler_map(),
          grpc_services: grpc_handler_map(),
          lifecycle_hooks: lifecycle_hooks(),
          dependencies: dependencies(),
          singleton_cache: singleton_cache()
        }

  # Client API

  @doc """
  Start the HandlerRunner GenServer with handlers, optional lifecycle hooks, and optional dependencies.

  ## Arguments

    - `handlers` - Map of handler_name => handler_function
    - `lifecycle_hooks` - Optional map of hook_type => [hook_functions]
    - `dependencies` - Optional list of Spikard.DI.dependency() structs for dependency injection
    - `grpc_services` - Optional map of service_name => method_name => {rpc_mode, handler}

  ## Returns

    - `{:ok, pid}` - Successfully started
    - `{:error, reason}` - Failed to start

  ## Examples

      iex> handlers = %{"get_users" => &MyApp.Handlers.get_users/1}
      iex> deps = [Spikard.DI.value("db", db_config)]
      iex> {:ok, pid} = Spikard.HandlerRunner.start_link(handlers, %{}, deps)
  """
  @spec start_link(handler_map(), lifecycle_hooks(), dependencies(), grpc_handler_map()) :: GenServer.on_start()
  def start_link(handlers, lifecycle_hooks \\ %{}, dependencies \\ [], grpc_services \\ %{})
      when is_map(handlers) and is_map(lifecycle_hooks) and is_list(dependencies) and is_map(grpc_services) do
    GenServer.start_link(__MODULE__, {handlers, lifecycle_hooks, dependencies, grpc_services})
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
  def init({handlers, lifecycle_hooks, dependencies, grpc_services}) do
    {:ok,
     %{
       handlers: handlers,
       grpc_services: grpc_services,
       lifecycle_hooks: lifecycle_hooks,
       dependencies: dependencies,
       singleton_cache: %{}
     }}
  end

  @impl true
  def handle_call({:invoke, handler_name, request_map}, _from, state) do
    {response, new_state} = execute_handler(state, handler_name, request_map)
    {:reply, response, new_state}
  end

  # Handle async request messages from Rust NIF
  # Format: {:handle_request, request_id, handler_name, request_map}
  @impl true
  def handle_info({:handle_request, request_id, handler_name, request_map}, state) do
    Logger.debug("HandlerRunner received request #{request_id} for handler #{handler_name}")
    Logger.debug("Request map: #{inspect(request_map)}")

    # Execute the handler and update state (for singleton cache)
    {response, new_state} = execute_handler(state, handler_name, request_map)

    Logger.debug("Handler response: #{inspect(response)}")

    # Deliver the response back to the waiting Rust handler
    result = Spikard.Native.deliver_handler_response(request_id, response)
    Logger.debug("Deliver result: #{inspect(result)}")

    {:noreply, new_state}
  end

  @impl true
  def handle_info({:grpc_execute, request_id, rpc_mode, service_name, method_name, payload, metadata}, state)
      when rpc_mode in [:unary, :server_stream] and is_binary(payload) and is_map(metadata) do
    Logger.debug("HandlerRunner executing gRPC #{rpc_mode} #{service_name}/#{method_name} request #{request_id}")

    response =
      execute_grpc_request(
        state.grpc_services,
        rpc_mode,
        service_name,
        method_name,
        payload,
        metadata
      )

    result = Spikard.Native.deliver_grpc_response(request_id, response)
    Logger.debug("Deliver gRPC result: #{inspect(result)}")

    {:noreply, state}
  end

  @impl true
  def handle_info({:grpc_execute, request_id, rpc_mode, service_name, method_name, payloads, metadata}, state)
      when rpc_mode in [:client_stream, :bidi_stream] and is_list(payloads) and is_map(metadata) do
    Logger.debug("HandlerRunner executing gRPC #{rpc_mode} #{service_name}/#{method_name} request #{request_id}")

    response =
      execute_grpc_stream_request(
        state.grpc_services,
        rpc_mode,
        service_name,
        method_name,
        payloads,
        metadata
      )

    result = Spikard.Native.deliver_grpc_response(request_id, response)
    Logger.debug("Deliver gRPC result: #{inspect(result)}")

    {:noreply, state}
  end

  # Handle lifecycle hook execution messages from Rust NIF
  # Format: {:hook_execute, hook_type, hook_index, request_id, context}
  @impl true
  def handle_info({:hook_execute, hook_type, hook_index, request_id, context}, state) do
    Logger.debug("HandlerRunner executing hook #{hook_type} at index #{hook_index} for request #{request_id}")

    # Convert context from string keys to atom keys for idiomatic Elixir access
    atomized_context = atomize_keys(context)

    # Get the hook function from state
    result = execute_hook(state.lifecycle_hooks, hook_type, hook_index, atomized_context)

    Logger.debug("Hook result: #{inspect(result)}")

    # Deliver the hook response back to the Rust handler
    # Convert atom keys back to string keys for Rust compatibility
    case result do
      {:continue, new_context} ->
        stringified = stringify_keys(new_context)
        delivery_result = Spikard.Native.deliver_hook_response(request_id, :continue, stringified)
        Logger.debug("Deliver hook continue result: #{inspect(delivery_result)}")

      {:short_circuit, response} ->
        stringified = stringify_keys(response)
        delivery_result = Spikard.Native.deliver_hook_response(request_id, :short_circuit, stringified)
        Logger.debug("Deliver hook short_circuit result: #{inspect(delivery_result)}")

      error ->
        Logger.error("Hook execution failed: #{inspect(error)}")

        delivery_result =
          Spikard.Native.deliver_hook_response(request_id, :error, %{"error" => "Hook execution failed"})

        Logger.debug("Deliver hook error result: #{inspect(delivery_result)}")
    end

    {:noreply, state}
  end

  def handle_info(msg, state) do
    Logger.warning("HandlerRunner received unexpected message: #{inspect(msg)}")
    {:noreply, state}
  end

  # Private Functions

  @spec execute_handler(state(), String.t(), map()) :: {map(), state()}
  defp execute_handler(state, handler_name, request_map) do
    case Map.get(state.handlers, handler_name) do
      nil ->
        Logger.warning("Handler not found: #{handler_name}")
        {error_response(404, "Handler not found: #{handler_name}"), state}

      handler when is_function(handler, 1) ->
        try do
          # Resolve dependencies and merge into request_map
          {resolved_deps, new_state} = resolve_dependencies(state, request_map)
          request_map_with_deps = Map.put(request_map, "dependencies", resolved_deps)

          request = Spikard.Request.from_map(request_map_with_deps)
          result = handler.(request)
          {normalize_response(result), new_state}
        rescue
          e ->
            Logger.error("Handler #{handler_name} raised: #{inspect(e)}")
            {error_response(500, "Handler error: #{Exception.message(e)}"), state}
        catch
          kind, reason ->
            Logger.error("Handler #{handler_name} threw #{kind}: #{inspect(reason)}")
            {error_response(500, "Handler error: #{inspect(reason)}"), state}
        end

      other ->
        Logger.warning("Invalid handler type for #{handler_name}: #{inspect(other)}")
        {error_response(500, "Invalid handler configuration"), state}
    end
  end

  @spec resolve_dependencies(state(), map()) :: {map(), state()}
  defp resolve_dependencies(state, _request_map) do
    {resolved_deps, new_cache} =
      Enum.reduce(state.dependencies, {%{}, state.singleton_cache}, fn dep, {deps_acc, cache_acc} ->
        resolve_dependency(dep, deps_acc, cache_acc)
      end)

    {resolved_deps, %{state | singleton_cache: new_cache}}
  end

  @spec resolve_dependency(map(), map(), map()) :: {map(), map()}
  defp resolve_dependency(%{type: :value, key: key, value: value}, deps_acc, cache_acc) do
    {Map.put(deps_acc, key, value), cache_acc}
  end

  defp resolve_dependency(%{type: :factory} = dep, deps_acc, cache_acc) do
    resolve_factory_dependency(dep, deps_acc, cache_acc)
  end

  @spec resolve_factory_dependency(map(), map(), map()) :: {map(), map()}
  defp resolve_factory_dependency(%{singleton: true, key: key} = dep, deps_acc, cache_acc) do
    case Map.fetch(cache_acc, key) do
      {:ok, cached_value} ->
        {Map.put(deps_acc, key, cached_value), cache_acc}

      :error ->
        value = dep.factory.()
        {Map.put(deps_acc, key, value), Map.put(cache_acc, key, value)}
    end
  end

  defp resolve_factory_dependency(%{singleton: false, key: key} = dep, deps_acc, cache_acc) do
    value = dep.factory.()
    {Map.put(deps_acc, key, value), cache_acc}
  end

  @spec normalize_response(term()) :: map()
  defp normalize_response(response) when is_map(response) do
    body = Map.get(response, :body, Map.get(response, "body"))

    %{
      "status" => Map.get(response, :status, Map.get(response, "status", 200)),
      "headers" => normalize_headers(Map.get(response, :headers, Map.get(response, "headers", %{}))),
      "body" => stringify_keys(body)
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

  @spec execute_grpc_request(grpc_handler_map(), Grpc.Service.rpc_mode(), String.t(), String.t(), binary(), map()) ::
          term()
  defp execute_grpc_request(grpc_services, rpc_mode, service_name, method_name, payload, metadata) do
    with {:ok, handler} <- fetch_grpc_handler(grpc_services, service_name, method_name, rpc_mode),
         request <- Grpc.Service.build_request(service_name, method_name, payload, metadata),
         {:ok, result} <- invoke_grpc_handler(handler, request) do
      case rpc_mode do
        :unary -> normalize_grpc_unary_result(result, rpc_mode)
        :server_stream -> normalize_grpc_stream_result(result)
      end
    else
      {:error, {:grpc, code, message, meta}} ->
        {:error, normalize_grpc_code(code), to_string(message), normalize_headers(meta)}

      {:error, message} ->
        {:error, "internal", to_string(message), %{}}
    end
  end

  @spec execute_grpc_stream_request(
          grpc_handler_map(),
          Grpc.Service.rpc_mode(),
          String.t(),
          String.t(),
          [binary()],
          map()
        ) :: term()
  defp execute_grpc_stream_request(grpc_services, rpc_mode, service_name, method_name, payloads, metadata) do
    with {:ok, handler} <- fetch_grpc_handler(grpc_services, service_name, method_name, rpc_mode),
         requests <- Enum.map(payloads, &Grpc.Service.build_request(service_name, method_name, &1, metadata)),
         {:ok, result} <- invoke_grpc_stream_handler(handler, requests, rpc_mode) do
      case rpc_mode do
        :client_stream -> normalize_grpc_unary_result(result, rpc_mode)
        :bidi_stream -> normalize_grpc_stream_result(result)
      end
    else
      {:error, {:grpc, code, message, meta}} ->
        {:error, normalize_grpc_code(code), to_string(message), normalize_headers(meta)}

      {:error, message} ->
        {:error, "internal", to_string(message), %{}}
    end
  end

  @spec fetch_grpc_handler(grpc_handler_map(), String.t(), String.t(), Grpc.Service.rpc_mode()) ::
          {:ok, function()} | {:error, String.t()}
  defp fetch_grpc_handler(grpc_services, service_name, method_name, rpc_mode) do
    with {:ok, methods} <- Map.fetch(grpc_services, service_name),
         {:ok, {registered_mode, handler}} <- Map.fetch(methods, method_name) do
      if registered_mode == rpc_mode do
        {:ok, handler}
      else
        {:error, "gRPC method #{service_name}/#{method_name} was registered as #{registered_mode}, not #{rpc_mode}"}
      end
    else
      :error -> {:error, "No gRPC handler registered for #{service_name}/#{method_name}"}
    end
  end

  @spec invoke_grpc_handler(function(), Grpc.Request.t()) :: {:ok, term()} | {:error, term()}
  defp invoke_grpc_handler(handler, request) do
    try do
      {:ok, handler.(request)}
    rescue
      e ->
        Logger.error("gRPC handler raised: #{inspect(e)}")
        {:error, Exception.message(e)}
    catch
      kind, reason ->
        Logger.error("gRPC handler threw #{kind}: #{inspect(reason)}")
        {:error, inspect(reason)}
    end
  end

  @spec invoke_grpc_stream_handler(function(), [Grpc.Request.t()], Grpc.Service.rpc_mode()) ::
          {:ok, term()} | {:error, term()}
  defp invoke_grpc_stream_handler(handler, requests, _rpc_mode) do
    try do
      {:ok, handler.(requests)}
    rescue
      e ->
        Logger.error("gRPC stream handler raised: #{inspect(e)}")
        {:error, Exception.message(e)}
    catch
      kind, reason ->
        Logger.error("gRPC stream handler threw #{kind}: #{inspect(reason)}")
        {:error, inspect(reason)}
    end
  end

  @spec normalize_grpc_unary_result(term(), Grpc.Service.rpc_mode()) :: term()
  defp normalize_grpc_unary_result(result, _rpc_mode) do
    case result do
      %Grpc.Response{} = response ->
        {:ok, response.payload, normalize_headers(response.metadata)}

      response when is_binary(response) ->
        {:ok, response, %{}}

      {:error, %Grpc.Error{} = error} ->
        {:error, normalize_grpc_code(error.code), error.message, normalize_headers(error.metadata)}

      {:error, code, message} ->
        {:error, normalize_grpc_code(code), to_string(message), %{}}

      {:error, code, message, metadata} ->
        {:error, normalize_grpc_code(code), to_string(message), normalize_headers(metadata)}

      other ->
        {:error, "internal", "Invalid gRPC handler result: #{inspect(other)}", %{}}
    end
  end

  @spec normalize_grpc_stream_result(term()) :: term()
  defp normalize_grpc_stream_result({:error, %Grpc.Error{} = error}) do
    {:error, normalize_grpc_code(error.code), error.message, normalize_headers(error.metadata)}
  end

  defp normalize_grpc_stream_result({:error, code, message}) do
    {:error, normalize_grpc_code(code), to_string(message), %{}}
  end

  defp normalize_grpc_stream_result({:error, code, message, metadata}) do
    {:error, normalize_grpc_code(code), to_string(message), normalize_headers(metadata)}
  end

  defp normalize_grpc_stream_result(responses) do
    with {:ok, list} <- coerce_stream_list(responses),
         {:ok, payloads} <- Enum.reduce_while(list, {:ok, []}, &collect_stream_payload/2) do
      {:stream, Enum.reverse(payloads)}
    else
      {:error, message} -> {:error, "internal", message, %{}}
    end
  end

  @spec coerce_stream_list(term()) :: {:ok, list()} | {:error, String.t()}
  defp coerce_stream_list(list) when is_list(list), do: {:ok, list}

  defp coerce_stream_list(other) do
    if Enumerable.impl_for(other) do
      {:ok, Enum.to_list(other)}
    else
      {:error, "Streaming gRPC handlers must return a list or enumerable of responses"}
    end
  end

  @spec collect_stream_payload(term(), {:ok, [binary()]}) :: {:cont, {:ok, [binary()]}} | {:halt, {:error, String.t()}}
  defp collect_stream_payload(item, {:ok, payloads}) when is_binary(item) do
    {:cont, {:ok, [item | payloads]}}
  end

  defp collect_stream_payload(%Grpc.Response{payload: payload, metadata: metadata}, {:ok, payloads})
       when is_binary(payload) do
    if metadata == %{} or metadata == nil do
      {:cont, {:ok, [payload | payloads]}}
    else
      {:halt, {:error, "Streaming gRPC responses do not support metadata"}}
    end
  end

  defp collect_stream_payload(other, _acc) do
    {:halt, {:error, "Invalid streaming gRPC response item: #{inspect(other)}"}}
  end

  defp normalize_grpc_code(code) when is_atom(code), do: Atom.to_string(code)
  defp normalize_grpc_code(code) when is_integer(code), do: Integer.to_string(code)
  defp normalize_grpc_code(code), do: to_string(code)

  # Convert string keys to atoms for top-level idiomatic Elixir access
  # but preserve nested maps like headers with string keys (HTTP headers can have hyphens)
  @spec atomize_keys(term()) :: term()
  defp atomize_keys(map) when is_map(map) do
    Map.new(map, fn {k, v} ->
      key = if is_binary(k), do: String.to_atom(k), else: k
      # Don't atomize keys inside the :headers map - keep them as strings
      value = if key == :headers, do: v, else: atomize_keys(v)
      {key, value}
    end)
  end

  defp atomize_keys(list) when is_list(list) do
    Enum.map(list, &atomize_keys/1)
  end

  defp atomize_keys(other), do: other

  # Convert atom keys to strings recursively for Rust compatibility
  @spec stringify_keys(term()) :: term()
  defp stringify_keys(map) when is_map(map) do
    Map.new(map, fn {k, v} ->
      key = if is_atom(k), do: Atom.to_string(k), else: k
      {key, stringify_keys(v)}
    end)
  end

  defp stringify_keys(list) when is_list(list) do
    Enum.map(list, &stringify_keys/1)
  end

  defp stringify_keys(other), do: other

  @spec execute_hook(lifecycle_hooks(), atom() | String.t(), non_neg_integer(), map()) ::
          {:continue, map()} | {:short_circuit, map()} | {:error, term()}
  defp execute_hook(lifecycle_hooks, hook_type, hook_index, context) do
    # Normalize hook_type to string since lifecycle_hooks uses string keys
    # but Rust sends atoms
    hook_type_str = to_string(hook_type)

    case Map.get(lifecycle_hooks, hook_type_str) do
      nil ->
        Logger.warning("No hooks registered for hook_type: #{hook_type_str}")
        {:continue, context}

      hooks when is_list(hooks) ->
        case Enum.at(hooks, hook_index) do
          nil ->
            Logger.warning("Hook index #{hook_index} not found for hook_type: #{hook_type_str}")
            {:continue, context}

          hook when is_function(hook, 1) ->
            try do
              hook.(context)
            rescue
              e ->
                Logger.error("Hook #{hook_type_str}[#{hook_index}] raised: #{inspect(e)}")
                {:error, Exception.message(e)}
            catch
              kind, reason ->
                Logger.error("Hook #{hook_type_str}[#{hook_index}] threw #{kind}: #{inspect(reason)}")
                {:error, inspect(reason)}
            end

          other ->
            Logger.warning("Invalid hook type at #{hook_type_str}[#{hook_index}]: #{inspect(other)}")
            {:error, "Invalid hook configuration"}
        end

      other ->
        Logger.warning("Invalid hooks for hook_type #{hook_type_str}: #{inspect(other)}")
        {:continue, context}
    end
  end
end

defmodule Spikard do
  @moduledoc """
  High-performance HTTP framework for Elixir powered by Rust.

  Spikard provides a thin Elixir binding over a Rust HTTP server implementation,
  offering excellent performance while maintaining an idiomatic Elixir API.

  ## Quick Start with Router Module

      defmodule MyApp.Router do
        use Spikard.Router

        get "/", &hello/1
        post "/users", &create_user/1

        defp hello(_request) do
          Spikard.Response.json(%{message: "Hello, World!"})
        end

        defp create_user(request) do
          Spikard.Response.json(%{id: 1, name: "New User"}, status: 201)
        end
      end

      # Start the server with router module
      {:ok, server} = Spikard.start(MyApp.Router, port: 4000, host: "0.0.0.0")

      # Stop the server
      :ok = Spikard.stop(server)

  ## Quick Start with Inline Routes

      {:ok, server} = Spikard.start(
        port: 4000,
        host: "127.0.0.1",
        routes: [
          {:get, "/", &MyApp.Handlers.index/1},
          {:post, "/users", &MyApp.Handlers.create_user/1}
        ]
      )

  ## Server Configuration

  The `:config` option allows you to set additional server options:

      {:ok, server} = Spikard.start(MyApp.Router,
        port: 4000,
        host: "0.0.0.0",
        config: [
          max_connections: 1000,
          request_timeout: 30000
        ]
      )
  """

  @type server_handle :: {String.t(), pos_integer()}

  @type route :: {atom(), String.t(), (term() -> term())}

  @doc """
  Starts a Spikard HTTP server with a Router module.

  The router module must use `Spikard.Router` and define routes using the
  route macros (`get`, `post`, `put`, `patch`, `delete`, `head`, `options`).

  ## Parameters

    * `router_module` - Module defining routes via `Spikard.Router`
    * `opts` - Keyword list of options

  ## Options

    * `:port` - Port number to listen on (required, 1-65535)
    * `:host` - Host address to bind to (default: "0.0.0.0")
    * `:config` - Additional server configuration map (default: %{})

  ## Returns

    * `{:ok, server_handle}` - Server started successfully; handle can be used with `stop/1`
    * `{:error, reason}` - Failed to start server with error reason

  ## Examples

      defmodule MyApp.Router do
        use Spikard.Router
        get "/", &MyApp.Handlers.index/1
      end

      {:ok, server} = Spikard.start(MyApp.Router, port: 4000)
      {:ok, server} = Spikard.start(MyApp.Router, port: 4000, host: "127.0.0.1")
  """
  @spec start(module(), keyword()) :: {:ok, server_handle()} | {:error, String.t()}
  def start(router_module, opts) when is_atom(router_module) and is_list(opts) do
    with {:ok, routes} <- get_routes_from_module(router_module) do
      start(opts ++ [routes: routes])
    end
  end

  @doc """
  Starts a Spikard HTTP server with inline routes.

  Accepts either a list of route tuples or keyword options containing routes.

  ## Parameters (arity 1)

    * `opts` - Keyword list containing:
      * `:port` - Port number to listen on (required, 1-65535)
      * `:host` - Host address to bind to (default: "0.0.0.0")
      * `:routes` - List of {method, path, handler} tuples (required if not called with start/2)
      * `:config` - Additional server configuration map (default: %{})

  ## Returns

    * `{:ok, server_handle}` - Server started successfully
    * `{:error, reason}` - Failed to start server with error reason

  ## Examples

      {:ok, server} = Spikard.start(
        port: 4000,
        routes: [
          {:get, "/", &MyApp.Handlers.index/1},
          {:post, "/users", &MyApp.Handlers.create_user/1}
        ]
      )

      {:ok, server} = Spikard.start(
        port: 8080,
        host: "127.0.0.1",
        routes: [{:get, "/health", &MyApp.Handlers.health/1}],
        config: [max_connections: 500]
      )
  """
  @spec start(keyword()) :: {:ok, server_handle()} | {:error, String.t()}
  def start(opts) when is_list(opts) do
    with :ok <- validate_start_opts(opts),
         {:ok, port, host, routes, config} <- extract_start_params(opts),
         {:ok, routes_json} <- serialize_routes(routes),
         handlers <- build_handlers_map(routes),
         {:ok, handler_runner_pid} <- Spikard.HandlerRunner.start_link(handlers),
         {:ok, server_handle} <-
           Spikard.Native.start_server(port, host, routes_json, handler_runner_pid, config) do
      {:ok, server_handle}
    else
      {:error, reason} -> {:error, reason}
      error -> {:error, inspect(error)}
    end
  end

  @doc """
  Stops a running Spikard server.

  Gracefully shuts down the server identified by the given handle.
  The handle is returned from `start/1` or `start/2`.

  ## Parameters

    * `server` - Server handle returned from `start/1` or `start/2`

  ## Returns

    * `:ok` - Server stopped successfully
    * `{:error, reason}` - Failed to stop server

  ## Examples

      {:ok, server} = Spikard.start(port: 4000, routes: [...])
      :ok = Spikard.stop(server)
  """
  @spec stop(server_handle()) :: :ok | {:error, String.t()}
  def stop({host, port} = _server) when is_binary(host) and is_integer(port) do
    case Spikard.Native.stop_server(host, port) do
      {:ok, :stopped} -> :ok
      :ok -> :ok
      {:error, reason} -> {:error, reason}
    end
  end

  @doc """
  Gets server information for a running Spikard server.

  Returns metadata about a server identified by its handle.

  ## Parameters

    * `server` - Server handle returned from `start/1` or `start/2`

  ## Returns

    * `{host, port}` - Server handle with host and port information

  ## Examples

      {:ok, server} = Spikard.start(port: 4000, routes: [...])
      {host, port} = Spikard.server_info(server)
  """
  @spec server_info(server_handle()) :: server_handle()
  def server_info({host, port} = _server) when is_binary(host) and is_integer(port) do
    Spikard.Native.server_info(host, port)
  end

  # Private helper functions

  @spec get_routes_from_module(module()) :: {:ok, [map()]} | {:error, String.t()}
  defp get_routes_from_module(module) do
    if function_exported?(module, :routes, 0) do
      {:ok, module.routes()}
    else
      {:error, "Router module #{inspect(module)} must define routes/0 using Spikard.Router"}
    end
  end

  @spec validate_start_opts(keyword()) :: :ok | {:error, String.t()}
  defp validate_start_opts(opts) do
    cond do
      !Keyword.has_key?(opts, :port) ->
        {:error, "Missing required option: :port"}

      !is_integer(Keyword.get(opts, :port)) ->
        {:error, "Option :port must be an integer"}

      !Keyword.has_key?(opts, :routes) ->
        {:error, "Missing required option: :routes"}

      !is_list(Keyword.get(opts, :routes)) ->
        {:error, "Option :routes must be a list"}

      true ->
        :ok
    end
  end

  @spec extract_start_params(keyword()) ::
          {:ok, pos_integer(), String.t(), [route() | map()], map()} | {:error, String.t()}
  defp extract_start_params(opts) do
    port = Keyword.get(opts, :port)
    host = Keyword.get(opts, :host, "0.0.0.0")
    routes = Keyword.get(opts, :routes, [])
    config = Keyword.get(opts, :config, %{})

    config_map =
      case config do
        map when is_map(map) -> map
        list when is_list(list) -> Enum.into(list, %{})
        _ -> %{}
      end

    # Validate port range
    if port >= 1 and port <= 65535 do
      {:ok, port, host, routes, config_map}
    else
      {:error, "Port must be between 1 and 65535"}
    end
  end

  @spec serialize_routes([route() | map()]) :: {:ok, String.t()} | {:error, String.t()}
  defp serialize_routes(routes) do
    try do
      routes_json =
        routes
        |> Enum.map(&normalize_route/1)
        |> Jason.encode!()

      {:ok, routes_json}
    rescue
      error -> {:error, "Failed to serialize routes: #{inspect(error)}"}
    end
  end

  @spec normalize_route(route() | map()) :: map()
  defp normalize_route({method, path, handler}) when is_atom(method) do
    %{
      "method" => method |> Atom.to_string() |> String.upcase(),
      "path" => path,
      "handler_name" => handler_name(handler),
      "pipes" => []
    }
  end

  defp normalize_route(route) when is_map(route) do
    # Already a map from Router - convert to JSON-friendly format
    method = Map.get(route, :method, "GET") |> to_string() |> String.upcase()
    %{
      "method" => method,
      "path" => Map.get(route, :path, "/"),
      "handler_name" => handler_name(Map.get(route, :handler, "unknown")),
      "pipes" => Map.get(route, :pipes, [])
    }
  end

  @spec handler_name(term()) :: String.t()
  defp handler_name({module, function}) when is_atom(module) and is_atom(function) do
    "#{inspect(module)}.#{function}"
  end

  defp handler_name(handler) do
    inspect(handler)
  end

  @spec build_handlers_map([route() | map()]) :: map()
  defp build_handlers_map(routes) do
    routes
    |> Enum.map(&extract_handler/1)
    |> Enum.into(%{})
  end

  @spec extract_handler(route() | map()) :: {String.t(), term()}
  defp extract_handler({_method, _path, handler}) do
    {handler_name(handler), handler}
  end

  defp extract_handler(route) when is_map(route) do
    handler = Map.get(route, :handler)
    {handler_name(handler), handler}
  end
end

defmodule Spikard.Router do
  @moduledoc """
  Phoenix-style router for Spikard HTTP applications.

  Provides macros for defining routes with compile-time route collection.
  Routes are stored as compile-time metadata and can be serialized to JSON
  for the Rust NIF.

  ## Usage

      defmodule MyApp.Router do
        use Spikard.Router

        get "/", &MyApp.Handler.index/1
        post "/users", &MyApp.Handler.create_user/1
        get "/users/:id", &MyApp.Handler.show_user/1

        scope "/api/v1" do
          pipe_through [:json, :auth]
          get "/items", &MyApp.Handler.list_items/1
          post "/items", &MyApp.Handler.create_item/1
        end
      end

  ## Route Parameters

  Routes can include path parameters using the `:param` syntax:

      get "/users/:id", &MyApp.Handler.show/1
      get "/posts/:id/comments/:comment_id", &MyApp.Handler.show_comment/1

  ## HTTP Methods

  All standard HTTP methods are supported:

      get, post, put, patch, delete, head, options, trace

  ## Middleware Chains

  Use `pipe_through/1` to apply middleware to routes:

      scope "/api" do
        pipe_through [:json, :auth]
        get "/admin", &MyApp.Handler.admin/1
      end

  ## Scopes

  Use `scope/2` to group routes with a common path prefix:

      scope "/api/v1" do
        get "/users", &handler/1
        post "/items", &handler/1
      end
  """

  @doc false
  defmacro __using__(_opts) do
    quote do
      import Spikard.Router
      Module.register_attribute(__MODULE__, :spikard_routes, accumulate: true)
      @spikard_scope_prefix ""
      @spikard_pipes []
      @before_compile Spikard.Router
    end
  end

  @doc false
  defmacro __before_compile__(env) do
    routes = Module.get_attribute(env.module, :spikard_routes, [])
    compiled_routes = Enum.reverse(routes)

    quote do
      @doc """
      Returns the list of compiled routes for this router.
      """
      @spec routes() :: [map()]
      def routes do
        unquote(Macro.escape(compiled_routes))
      end

      @doc """
      Returns routes as JSON string for passing to Rust NIF.
      """
      @spec routes_json() :: String.t()
      def routes_json do
        routes()
        |> Enum.map(&Spikard.Router.to_route_metadata/1)
        |> Jason.encode!()
      end

      @doc """
      Returns a map of handler names to handler functions.
      """
      @spec handlers() :: map()
      def handlers do
        routes()
        |> Enum.map(fn route ->
          {route.handler_name, route.handler}
        end)
        |> Map.new()
      end
    end
  end

  @type http_method :: :get | :post | :put | :patch | :delete | :head | :options | :trace

  @type route_t :: %{
          method: String.t(),
          path: String.t(),
          handler: (term() -> term()),
          handler_name: String.t(),
          pipes: [atom()],
          parameter_schema: map() | nil,
          request_schema: map() | nil,
          response_schema: map() | nil
        }

  @doc false
  @spec to_route_metadata(route_t()) :: map()
  def to_route_metadata(route) do
    %{
      "method" => route.method,
      "path" => route.path,
      "handler_name" => route.handler_name,
      "request_body_schema" => Map.get(route, :request_schema),
      "request_params_schema" => Map.get(route, :parameter_schema),
      "response_schema" => Map.get(route, :response_schema)
    }
  end

  @doc """
  Defines a GET route.

  ## Examples

      get "/", &MyApp.Handler.index/1
      get "/users/:id", &MyApp.Handler.show/1
      get "/items/:id", &MyApp.Handler.show/1, parameter_schema: %{...}
  """
  defmacro get(path, handler, opts \\ []) do
    add_route(:get, path, handler, opts)
  end

  @doc """
  Defines a POST route.

  ## Examples

      post "/users", &MyApp.Handler.create/1
      post "/items", &MyApp.Handler.create/1, request_schema: %{...}
  """
  defmacro post(path, handler, opts \\ []) do
    add_route(:post, path, handler, opts)
  end

  @doc """
  Defines a PUT route.

  ## Examples

      put "/users/:id", &MyApp.Handler.update/1
  """
  defmacro put(path, handler, opts \\ []) do
    add_route(:put, path, handler, opts)
  end

  @doc """
  Defines a PATCH route.

  ## Examples

      patch "/users/:id", &MyApp.Handler.patch_update/1
  """
  defmacro patch(path, handler, opts \\ []) do
    add_route(:patch, path, handler, opts)
  end

  @doc """
  Defines a DELETE route.

  ## Examples

      delete "/users/:id", &MyApp.Handler.delete/1
  """
  defmacro delete(path, handler, opts \\ []) do
    add_route(:delete, path, handler, opts)
  end

  @doc """
  Defines a HEAD route.

  ## Examples

      head "/health", &MyApp.Handler.health/1
  """
  defmacro head(path, handler, opts \\ []) do
    add_route(:head, path, handler, opts)
  end

  @doc """
  Defines an OPTIONS route.

  ## Examples

      options "/users", &MyApp.Handler.options/1
  """
  defmacro options(path, handler, opts \\ []) do
    add_route(:options, path, handler, opts)
  end

  @doc """
  Defines a TRACE route.

  ## Examples

      trace "/debug", &MyApp.Handler.trace/1
  """
  defmacro trace(path, handler, opts \\ []) do
    add_route(:trace, path, handler, opts)
  end

  @doc """
  Groups routes under a common path prefix.

  ## Examples

      scope "/api/v1" do
        get "/users", &handler/1
      end
  """
  defmacro scope(scope_path, do: block) do
    quote do
      previous_prefix = @spikard_scope_prefix
      previous_pipes = @spikard_pipes
      @spikard_scope_prefix previous_prefix <> unquote(scope_path)

      unquote(block)

      @spikard_scope_prefix previous_prefix
      @spikard_pipes previous_pipes
    end
  end

  @doc """
  Applies middleware pipes to routes in the current scope.

  ## Examples

      scope "/api" do
        pipe_through [:json]
        get "/users", &handler/1
      end
  """
  defmacro pipe_through(pipes) when is_list(pipes) do
    quote do
      @spikard_pipes @spikard_pipes ++ unquote(pipes)
    end
  end

  # Private helper to add a route
  defp add_route(method, path, handler, opts) do
    method_str = method |> Atom.to_string() |> String.upcase()

    # Generate a handler name from the handler AST
    handler_name =
      case handler do
        {:&, _, [{:/, _, [{{:., _, [mod, fun]}, _, _}, _arity]}]} ->
          # &Module.function/arity format
          mod_name = Macro.to_string(mod)
          "#{mod_name}.#{fun}"

        {:&, _, [{:/, _, [{fun, _, _}, _arity]}]} ->
          # &function/arity format (local function)
          "#{fun}"

        _ ->
          # Anonymous function or other - generate deterministic name from method and path
          # We use the AST path at compile time since the actual path value isn't available yet
          path_hash = path |> Macro.to_string() |> :erlang.phash2()
          "handler_#{method}_#{path_hash}"
      end

    quote do
      full_path = @spikard_scope_prefix <> unquote(path)
      current_pipes = @spikard_pipes
      route_opts = unquote(opts)

      @spikard_routes %{
        method: unquote(method_str),
        path: full_path,
        handler: unquote(handler),
        handler_name: unquote(handler_name),
        pipes: current_pipes,
        parameter_schema: Keyword.get(route_opts, :parameter_schema),
        request_schema: Keyword.get(route_opts, :request_schema),
        response_schema: Keyword.get(route_opts, :response_schema)
      }
    end
  end
end

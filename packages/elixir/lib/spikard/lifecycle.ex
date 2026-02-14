defmodule Spikard.Lifecycle do
  @moduledoc """
  Lifecycle hooks for Spikard HTTP request processing.

  Lifecycle hooks allow you to run code at specific points during request processing:

  ## Hook Phases

  - `:on_request` - Called immediately after a request is received
  - `:pre_validation` - Called before request validation
  - `:pre_handler` - Called after validation, before the handler executes
  - `:on_response` - Called after the handler returns a response
  - `:on_error` - Called when an error occurs during processing

  ## Hook Return Values

  Request-phase hooks (on_request, pre_validation, pre_handler) should return:
  - `{:continue, context}` - Continue to the next phase with potentially modified context
  - `{:short_circuit, response}` - Return early with the given response

  Response-phase hooks (on_response, on_error) should return:
  - `{:continue, response}` - Continue with potentially modified response

  ## Context Structure

  Request-phase hooks receive a context map with:
  - `:method` - HTTP method (e.g., "GET", "POST")
  - `:path` - Request path
  - `:query` - Query string
  - `:headers` - Map of request headers

  Response-phase hooks receive a response map with:
  - `:status` - HTTP status code
  - `:headers` - Map of response headers
  - `:body` - Response body (if available)

  ## Examples

      # Authentication hook
      auth_hook = fn ctx ->
        case Map.get(ctx.headers, "authorization") do
          "Bearer " <> _token -> {:continue, ctx}
          _ -> {:short_circuit, %{status: 401, body: %{error: "Unauthorized"}}}
        end
      end

      # Security headers hook
      security_hook = fn response ->
        headers = Map.merge(response.headers, %{
          "x-frame-options" => "DENY",
          "x-content-type-options" => "nosniff"
        })
        {:continue, %{response | headers: headers}}
      end

      {:ok, server} = Spikard.start(
        port: 4000,
        routes: [...],
        lifecycle: [
          pre_handler: [auth_hook],
          on_response: [security_hook]
        ]
      )
  """

  @type hook_context :: %{
          method: String.t(),
          path: String.t(),
          query: String.t(),
          headers: %{String.t() => String.t()}
        }

  @type response_context :: %{
          status: integer(),
          headers: %{String.t() => String.t()},
          body: term()
        }

  @type request_hook :: (hook_context() -> {:continue, hook_context()} | {:short_circuit, map()})
  @type response_hook :: (response_context() -> {:continue, response_context()})

  @type lifecycle_config :: [
          on_request: [request_hook()],
          pre_validation: [request_hook()],
          pre_handler: [request_hook()],
          on_response: [response_hook()],
          on_error: [response_hook()]
        ]

  @doc """
  Validates lifecycle configuration.

  Returns `:ok` if valid, `{:error, reason}` otherwise.
  """
  @spec validate(lifecycle_config()) :: :ok | {:error, String.t()}
  def validate(config) when is_list(config) do
    valid_keys = [:on_request, :pre_validation, :pre_handler, :on_response, :on_error]

    Enum.reduce_while(config, :ok, fn {key, hooks}, _acc ->
      cond do
        key not in valid_keys ->
          {:halt, {:error, "Invalid lifecycle hook type: #{inspect(key)}"}}

        not is_list(hooks) ->
          {:halt, {:error, "Hooks for #{key} must be a list"}}

        not Enum.all?(hooks, &is_function(&1, 1)) ->
          {:halt, {:error, "All hooks for #{key} must be functions with arity 1"}}

        true ->
          {:cont, :ok}
      end
    end)
  end

  def validate(_), do: {:error, "Lifecycle config must be a keyword list"}

  @doc """
  Counts the number of hooks for each lifecycle phase.

  Returns a map with counts for each hook type.
  """
  @spec count_hooks(lifecycle_config()) :: %{
          on_request: non_neg_integer(),
          pre_validation: non_neg_integer(),
          pre_handler: non_neg_integer(),
          on_response: non_neg_integer(),
          on_error: non_neg_integer()
        }
  def count_hooks(config) when is_list(config) do
    %{
      on_request: length(Keyword.get(config, :on_request, [])),
      pre_validation: length(Keyword.get(config, :pre_validation, [])),
      pre_handler: length(Keyword.get(config, :pre_handler, [])),
      on_response: length(Keyword.get(config, :on_response, [])),
      on_error: length(Keyword.get(config, :on_error, []))
    }
  end

  def count_hooks(_), do: %{on_request: 0, pre_validation: 0, pre_handler: 0, on_response: 0, on_error: 0}

  @doc """
  Gets hooks for a specific phase.
  """
  @spec get_hooks(lifecycle_config(), atom()) :: [function()]
  def get_hooks(config, phase) when is_list(config) and is_atom(phase) do
    Keyword.get(config, phase, [])
  end

  def get_hooks(_, _), do: []

  @doc """
  Executes a hook function with the given context.

  Returns `{:continue, context}` or `{:short_circuit, response}`.
  """
  @spec execute_hook(function(), term()) :: {:continue, term()} | {:short_circuit, term()}
  def execute_hook(hook, context) when is_function(hook, 1) do
    try do
      case hook.(context) do
        {:continue, ctx} -> {:continue, ctx}
        {:short_circuit, resp} -> {:short_circuit, resp}
        other -> {:continue, other}
      end
    rescue
      e ->
        {:short_circuit,
         %{
           status: 500,
           body: %{error: "Hook error", message: Exception.message(e)}
         }}
    catch
      kind, reason ->
        {:short_circuit,
         %{
           status: 500,
           body: %{error: "Hook error", message: "#{kind}: #{inspect(reason)}"}
         }}
    end
  end
end

# This file is generated. Do not edit.

defmodule App do
  @moduledoc """
  Spikard application builder.
  """

  defstruct [
    :registrations,
    :config,
  ]

  @doc """
  Create a new application with the default server configuration.
  """
  def new(options \\ []) do
    %__MODULE__{
      registrations: [],
    }
  end

  @doc """
  Set the server configuration.
  """
  def config(self, config) do
    self = %__MODULE__{self | config: config}
    self
  end

  @doc """
  Register a route using the provided builder and handler function.

  # Errors

  Returns an error if route construction fails or if the handler registration fails.
  """
  def route(self, builder, handler) do
    entry = {"route", {builder}, handler}
    %__MODULE__{self | registrations: [entry | self.registrations]}
  end

  @doc """
  Register a GET route at the given path.
  """
  def get(app, path, handler) do
    app
  end

  @doc """
  Register a POST route at the given path.
  """
  def post(app, path, handler) do
    app
  end

  @doc """
  Register a PUT route at the given path.
  """
  def put(app, path, handler) do
    app
  end

  @doc """
  Register a PATCH route at the given path.
  """
  def patch(app, path, handler) do
    app
  end

  @doc """
  Register a DELETE route at the given path.
  """
  def delete(app, path, handler) do
    app
  end

  @doc """
  Register a HEAD route at the given path.
  """
  def head(app, path, handler) do
    app
  end

  @doc """
  Register an OPTIONS route at the given path.
  """
  def options(app, path, handler) do
    app
  end

  @doc """
  Register a CONNECT route at the given path.
  """
  def connect(app, path, handler) do
    app
  end

  @doc """
  Register a TRACE route at the given path.
  """
  def trace(app, path, handler) do
    app
  end

  # GenServer for dispatching trait_call messages from Rust.
  defmodule App.Handler do
    use GenServer

    def start_link(state) do
      GenServer.start_link(__MODULE__, state)
    end

    def init(state) do
      {:ok, state}
    end

    def handle_cast({:trait_call, method, args_json, reply_id}, registrations) do
      # Decode JSON args and dispatch to registered handler
      case decode_args_and_dispatch(method, args_json, registrations) do
        {:ok, response} ->
          Native.complete_trait_call(reply_id, response)
        {:error, reason} ->
          error_response = %{"error" => reason}
          Native.complete_trait_call(reply_id, error_response)
      end
      {:noreply, registrations}
    end

    defp decode_args_and_dispatch(method, args_json, registrations) do
      # Find handler entry for the method
      case find_handler(method, registrations) do
        nil ->
          {:error, "Handler not registered for method: #{method}"}
        {^method, _metadata, handler} ->
          # Decode JSON args (assumes handler accepts a single arg)
          case Jason.decode(args_json) do
            {:ok, args} ->
              # Call the registered handler with decoded args
              try do
                response = handler.(args)
                # Encode response to JSON
                case Jason.encode(response) do
                  {:ok, response_json} -> {:ok, response_json}
                  {:error, reason} -> {:error, "Failed to encode response: #{reason}"}
                end
              rescue
                e ->
                  {:error, "Handler raised exception: #{inspect(e)}"}
              end
            {:error, reason} ->
              {:error, "Failed to decode args: #{reason}"}
          end
      end
    end

    defp find_handler(_method, []), do: nil
    defp find_handler(target, [{name, _metadata, _handler} = entry | _rest]) when name == target do
      entry
    end
    defp find_handler(target, [_head | rest]) do
      find_handler(target, rest)
    end

  end

  @doc """
  Run the HTTP server using the configured routes.

  # Errors

  Returns an error if server construction or execution fails.
  """
  def run(self) do
    Native.app_run(self.registrations)
  end

  @doc """
  Build the underlying Axum router.

  # Errors

  Returns an error if server or router construction fails.
  """
  def into_router(self) do
    Native.app_into_router(self.registrations)
  end

end

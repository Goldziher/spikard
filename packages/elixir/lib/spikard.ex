defmodule Spikard do
  @moduledoc """
  High-performance HTTP framework for Elixir powered by Rust.

  Spikard provides a thin Elixir binding over a Rust HTTP server implementation,
  offering excellent performance while maintaining an idiomatic Elixir API.

  ## Quick Start

      defmodule MyApp.Router do
        use Spikard.Router

        get "/", &hello/1

        defp hello(_request) do
          Spikard.Response.json(%{message: "Hello, World!"})
        end
      end

      # Start the server
      {:ok, _server} = Spikard.start(port: 4000, routes: MyApp.Router.routes())
  """

  @doc """
  Starts a Spikard HTTP server.

  ## Options

    * `:port` - The port to listen on (required)
    * `:routes` - List of route tuples (required)

  ## Examples

      {:ok, server} = Spikard.start(port: 4000, routes: [])
  """
  @spec start(keyword()) :: {:ok, term()} | {:error, term()}
  def start(_opts) do
    # TODO: Implement in Stage 1
    {:error, :not_implemented}
  end

  @doc """
  Stops a running Spikard server.
  """
  @spec stop(term()) :: :ok | {:error, term()}
  def stop(_server) do
    # TODO: Implement in Stage 1
    {:error, :not_implemented}
  end
end

defmodule PhoenixBench.Application do
  @moduledoc false
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      BenchWeb.Endpoint
    ]

    opts = [strategy: :one_for_one, name: PhoenixBench.Supervisor]
    Supervisor.start_link(children, opts)
  end
end

defmodule E2eElixir.MixProject do
  use Mix.Project

  def project do
    [
      app: :e2e_elixir,
      version: "0.1.0",
      elixir: "~> 1.14",
      deps: deps()
    ]
  end

  defp deps do
    [
      {:spikard, path: "../../packages/elixir"},
      {:rustler, "~> 0.37.0", optional: true, runtime: false},
      {:req, "~> 0.5"},
      {:jason, "~> 1.4"}
    ]
  end
end

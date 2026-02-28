defmodule SpikardElixirBench.MixProject do
  use Mix.Project

  def project do
    [
      app: :spikard_elixir_bench,
      version: "0.1.0",
      elixir: "~> 1.18",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:spikard, path: "../../../../packages/elixir"},
      {:jason, "~> 1.4"}
    ]
  end
end

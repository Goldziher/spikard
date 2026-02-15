defmodule E2EElixirApp.MixProject do
  use Mix.Project

  def project do
    [
      app: :e2e_elixir_app,
      version: "0.1.0",
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger, :inets, :ssl]
    ]
  end

  defp deps do
    [
      {:spikard, path: "../../packages/elixir"},
      {:jason, "~> 1.4"}
    ]
  end
end

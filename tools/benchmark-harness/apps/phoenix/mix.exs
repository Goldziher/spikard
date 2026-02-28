defmodule PhoenixBench.MixProject do
  use Mix.Project

  def project do
    [
      app: :phoenix_bench,
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
      {:phoenix, "~> 1.7"},
      {:bandit, "~> 1.8"},
      {:jason, "~> 1.4"}
    ]
  end
end

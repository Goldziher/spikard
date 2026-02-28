defmodule PlugBanditBench.MixProject do
  use Mix.Project

  def project do
    [
      app: :plug_bandit_bench,
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
      {:bandit, "~> 1.8"},
      {:plug, "~> 1.16"},
      {:jason, "~> 1.4"}
    ]
  end
end

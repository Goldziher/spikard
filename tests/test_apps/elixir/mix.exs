defmodule SpikardTestApp.MixProject do
  use Mix.Project

  @version "0.10.2"

  def project do
    [
      app: :spikard_test_app,
      version: @version,
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
      {:spikard, "== #{@version}"}
    ]
  end
end

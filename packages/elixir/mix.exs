defmodule Spikard.MixProject do
  use Mix.Project

  def project do
    [
      app: :spikard,
      version: "0.13.0",
      elixir: "~> 1.14",
      description: "Polyglot web framework powered by Rust",
      package: package(),
      deps: deps()
    ]
  end

  defp package do
    [
      licenses: ["MIT"],
      links: %{"GitHub" => "https://github.com/kreuzberg-dev/spikard"}
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.34"}
    ]
  end
end

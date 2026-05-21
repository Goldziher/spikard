defmodule Spikard.MixProject do
  use Mix.Project

  def project do
    [
      app: :spikard,
      version: "0.15.2",
      elixir: "~> 1.14",
      elixirc_paths: ["lib", Path.expand("../../crates/spikard-elixir/src", __DIR__)],
      rustler_crates: [spikard_nif: [mode: :release]],
      description: "Rust-centric multi-language HTTP framework with polyglot bindings",
      package: package(),
      deps: deps()
    ]
  end

  defp package do
    [
      licenses: ["MIT"],
      links: %{"GitHub" => "https://github.com/Goldziher/spikard"},
      files: ~w(lib native .formatter.exs mix.exs README* checksum-*.exs)
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.37.0", runtime: false},
      {:rustler_precompiled, "~> 0.9"},
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false},
      {:ex_doc, "~> 0.40", only: :dev, runtime: false}
    ]
  end
end

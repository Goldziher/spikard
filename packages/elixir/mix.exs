defmodule Spikard.MixProject do
  use Mix.Project

  def project do
    [
      app: :spikard,
      version: "0.15.6-rc.9",
      elixir: "~> 1.14",
      rustler_crates: [
        spikard_nif: [
          mode: :release,
          targets: ~w(aarch64-apple-darwin aarch64-unknown-linux-gnu x86_64-unknown-linux-gnu x86_64-pc-windows-gnu)
        ]
      ],
      description: "Rust-centric multi-language HTTP framework with polyglot bindings",
      package: package(),
      deps: deps()
    ]
  end

  defp package do
    [
      licenses: ["MIT"],
      links: %{"GitHub" => "https://github.com/Goldziher/spikard"},
      files:
        ~w(lib .formatter.exs mix.exs README* checksum-*.exs native/spikard_nif/Cargo.toml native/spikard_nif/Cargo.lock native/spikard_nif/src)
    ]
  end

  defp deps do
    [
      {:jason, "~> 1.4"},
      {:rustler, "~> 0.38.0", runtime: false},
      {:rustler_precompiled, "~> 0.9"},
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false},
      {:ex_doc, "~> 0.40", only: :dev, runtime: false}
    ]
  end
end

defmodule Spikard.MixProject do
  use Mix.Project

  @version "0.11.0"
  @source_url "https://github.com/Goldziher/spikard"

  def project do
    [
      app: :spikard,
      version: @version,
      elixir: "~> 1.18",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      description: "High-performance HTTP framework for Elixir powered by Rust",
      package: package(),
      docs: docs(),
      aliases: aliases(),
      test_coverage: [tool: ExCoveralls],
      dialyzer: [
        plt_file: {:no_warn, "priv/plts/dialyzer.plt"},
        plt_add_apps: [:mix]
      ],
      rustler_crates: [spikard_elixir: [mode: :release]]
    ]
  end

  def cli do
    [
      preferred_envs: [
        "test.watch": :test,
        coveralls: :test,
        "coveralls.html": :test,
        "coveralls.lcov": :test
      ]
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      # NIF compilation
      {:rustler, "~> 0.37", runtime: false},
      {:rustler_precompiled, "~> 0.8"},

      # Development and testing
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false},
      {:dialyxir, "~> 1.4", only: [:dev, :test], runtime: false},
      {:ex_doc, "~> 0.38", only: :dev, runtime: false},
      {:excoveralls, "~> 0.18", only: :test},
      {:jason, "~> 1.4"}
    ]
  end

  defp package do
    [
      name: "spikard",
      files: ~w(lib native .formatter.exs mix.exs README.md LICENSE),
      licenses: ["MIT"],
      links: %{
        "GitHub" => @source_url,
        "Changelog" => "#{@source_url}/blob/main/CHANGELOG.md"
      }
    ]
  end

  defp docs do
    [
      main: "Spikard",
      source_url: @source_url,
      source_ref: "v#{@version}",
      homepage_url: "https://spikard.dev",
      extras: ["README.md"],
      groups_for_modules: [
        Core: [Spikard, Spikard.Router, Spikard.Request, Spikard.Response],
        Authentication: [Spikard.Auth, Spikard.Auth.JWT, Spikard.Auth.ApiKey],
        "Real-Time": [Spikard.WebSocket, Spikard.Sse],
        Testing: [Spikard.TestClient, Spikard.TestClient.Response],
        "Dependency Injection": [Spikard.DI],
        Advanced: [Spikard.Lifecycle, Spikard.Background, Spikard.UploadFile, Spikard.OpenAPI]
      ]
    ]
  end

  defp aliases do
    [
      lint: ["credo suggest --all --strict"],
      "lint.fix": ["credo suggest --all --strict"]
    ]
  end
end

import Config

config :phoenix_bench, BenchWeb.Endpoint,
  url: [host: "0.0.0.0"],
  adapter: Bandit.PhoenixAdapter,
  render_errors: [formats: [json: BenchWeb.ErrorJSON], layout: false],
  server: true

config :phoenix, :json_library, Jason

config :logger, level: :warning

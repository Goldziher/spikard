# Phoenix API-only benchmark server startup script.
# Usage: mix run --no-halt run.exs [port]

port =
  case System.argv() do
    [port_str | _] -> String.to_integer(port_str)
    _ -> 8000
  end

Application.put_env(:phoenix_bench, BenchWeb.Endpoint,
  http: [ip: {0, 0, 0, 0}, port: port],
  url: [host: "0.0.0.0"],
  adapter: Bandit.PhoenixAdapter,
  render_errors: [formats: [json: BenchWeb.ErrorJSON], layout: false],
  server: true
)

Application.put_env(:phoenix, :json_library, Jason)

IO.puts(:stderr, "[phoenix] Starting benchmark server on port #{port}")

{:ok, _} = Supervisor.start_link([BenchWeb.Endpoint], strategy: :one_for_one)

Process.sleep(:infinity)

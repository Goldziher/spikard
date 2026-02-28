# Spikard Elixir binding benchmark server startup script.
# Usage: mix run --no-halt run.exs [port]

port =
  case System.argv() do
    [port_str | _] -> String.to_integer(port_str)
    _ -> 8000
  end

IO.puts(:stderr, "[spikard-elixir] Starting benchmark server on port #{port}")

# Ensure compiled modules are loaded before starting server
Code.ensure_loaded!(BenchServer.Router)
Code.ensure_loaded!(BenchServer.Handlers)

{:ok, _server} = Spikard.start(BenchServer.Router, port: port, host: "0.0.0.0")

Process.sleep(:infinity)

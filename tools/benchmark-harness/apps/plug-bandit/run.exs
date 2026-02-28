# Plug.Router + Bandit benchmark server startup script.
# Usage: mix run --no-halt run.exs [port]

port =
  case System.argv() do
    [port_str | _] -> String.to_integer(port_str)
    _ -> 8000
  end

IO.puts(:stderr, "[plug-bandit] Starting benchmark server on port #{port}")

{:ok, _pid} = Bandit.start_link(plug: BenchServer.Router, port: port, ip: {0, 0, 0, 0})

Process.sleep(:infinity)

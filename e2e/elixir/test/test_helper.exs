ExUnit.start()

# Spawn app_harness subprocess and set SUT_URL
# If SUT_URL is already set, a parent process started a shared harness.
# Use it as-is and do NOT spawn our own.

unless System.get_env("SUT_URL") do
  app_harness_bin = Path.expand("../app_harness.exs", __DIR__)

  port = Port.open({:spawn_executable, System.find_executable("elixir")}, [
    :binary,
    {:line, 65_536},
    args: [app_harness_bin]
  ])

  url = "http://127.0.0.1:8000"

  # Poll until the harness accepts TCP connections
  deadline = :erlang.monotonic_time(:millisecond) + 15_000
  ready = false

  {ready, url} =
    Enum.reduce_while(1..150, {false, url}, fn _, {_, url_acc} ->
      now = :erlang.monotonic_time(:millisecond)
      if now > deadline do
        {:halt, {false, url_acc}}
      else
        case :gen_tcp.connect(String.to_charlist("127.0.0.1"), 8000, [], 500) do
          {:ok, socket} ->
            :gen_tcp.close(socket)
            {:halt, {true, url_acc}}
          {:error, _} ->
            Process.sleep(100)
            {:cont, {false, url_acc}}
        end
      end
    end)

  unless ready do
    Port.close(port)
    raise "App harness did not become reachable on 127.0.0.1:8000 within 15s"
  end

  System.put_env("SUT_URL", url)
end

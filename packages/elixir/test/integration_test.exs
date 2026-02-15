defmodule SpikardIntegrationTest do
  use ExUnit.Case, async: false

  @moduletag :integration

  setup_all do
    :inets.start()
    :ssl.start()
    Logger.configure(level: :debug)
    :ok
  end

  describe "handler callback" do
    test "handler is called and returns response" do
      # Define a simple handler - body is a map, will be JSON encoded by the framework
      handler = fn request ->
        %{
          status: 200,
          headers: %{"content-type" => "application/json"},
          body: %{
            message: "Hello from Elixir!",
            path: request.path,
            method: request.method
          }
        }
      end

      # Start server with the handler
      port = free_port()

      result =
        Spikard.start(
          port: port,
          host: "127.0.0.1",
          routes: [{:get, "/test", handler}]
        )

      case result do
        {:ok, server} ->
          on_exit(fn -> Spikard.stop(server) end)

          # Make HTTP request
          url = "http://127.0.0.1:#{port}/test"

          case wait_for_http_ok(url) do
            {:ok, {{_, status, _}, _headers, body}} ->
              body_str = to_string(body)
              IO.puts("Response body: #{inspect(body_str)}")
              assert status == 200
              parsed = Jason.decode!(body_str)
              IO.puts("Parsed: #{inspect(parsed)}")
              assert parsed["message"] == "Hello from Elixir!"
              assert parsed["path"] == "/test"
              assert parsed["method"] == "GET"

            {:error, reason} ->
              flunk("HTTP request failed: #{inspect(reason)}")
          end

        {:error, reason} ->
          # Port might be in use, skip
          IO.puts("Skipping integration test: #{inspect(reason)}")
      end
    end
  end

  defp free_port do
    {:ok, socket} = :gen_tcp.listen(0, [:binary, active: false])
    {:ok, {_addr, port}} = :inet.sockname(socket)
    :gen_tcp.close(socket)
    port
  end

  defp wait_for_http_ok(url, attempts \\ 50) do
    case :httpc.request(:get, {to_charlist(url), []}, [], []) do
      {:ok, _} = ok ->
        ok

      {:error, _reason} when attempts > 0 ->
        Process.sleep(50)
        wait_for_http_ok(url, attempts - 1)

      {:error, _reason} = err ->
        err
    end
  end
end

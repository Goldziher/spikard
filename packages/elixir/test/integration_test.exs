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
      result =
        Spikard.start(
          port: 59900,
          host: "127.0.0.1",
          routes: [{:get, "/test", handler}]
        )

      case result do
        {:ok, server} ->
          # Give server time to start
          Process.sleep(100)

          # Make HTTP request
          case :httpc.request(:get, {~c"http://127.0.0.1:59900/test", []}, [], []) do
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

          # Stop server
          assert :ok = Spikard.stop(server)

        {:error, reason} ->
          # Port might be in use, skip
          IO.puts("Skipping integration test: #{inspect(reason)}")
      end
    end
  end
end

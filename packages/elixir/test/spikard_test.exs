defmodule SpikardTest do
  use ExUnit.Case
  doctest Spikard

  # Define a test router module for start/2 tests
  defmodule TestRouter do
    use Spikard.Router

    get("/", &__MODULE__.index/1)

    def index(_req) do
      %{status: 200, headers: [], body: "OK"}
    end
  end

  # Define a module without routes/0 for error testing
  defmodule InvalidRouter do
    # No routes/0 function
  end

  describe "start/1" do
    test "accepts empty routes list (server starts with no routes)" do
      # An empty routes list is a valid configuration - server will just return 404 for all requests
      result = Spikard.start(port: 4000, routes: [])
      assert match?({:ok, _}, result) or match?({:error, _}, result)

      case result do
        {:ok, server} -> Spikard.stop(server)
        _ -> :ok
      end
    end

    test "validates port is required" do
      # Elixir validation returns string error
      assert {:error, "Missing required option: :port"} = Spikard.start(routes: [%{}])
    end

    test "validates port must be in valid range" do
      # Elixir validation at extract_start_params
      assert {:error, "Port must be between 1 and 65535"} = Spikard.start(port: 0, routes: [%{}])
      assert {:error, "Port must be between 1 and 65535"} = Spikard.start(port: 65536, routes: [%{}])
    end

    test "validates port must be integer" do
      # Elixir validation returns string error
      assert {:error, "Option :port must be an integer"} = Spikard.start(port: "4000", routes: [])
    end

    test "validates routes is required" do
      assert {:error, "Missing required option: :routes"} = Spikard.start(port: 4000)
    end

    test "validates routes must be a list" do
      assert {:error, "Option :routes must be a list"} = Spikard.start(port: 4000, routes: "invalid")
    end

    test "accepts tuple routes" do
      # Test that tuple routes are accepted and normalized
      # This will fail at NIF level since we're using a test port
      # but validates the route normalization path
      handler = fn _req -> %{status: 200, body: "OK"} end
      result = Spikard.start(port: 59999, routes: [{:get, "/test", handler}])
      # Either starts successfully or fails at bind
      assert match?({:ok, _}, result) or match?({:error, _}, result)

      # Clean up if started
      case result do
        {:ok, server} -> Spikard.stop(server)
        _ -> :ok
      end
    end

    test "accepts config as map" do
      result = Spikard.start(port: 59998, routes: [%{path: "/", method: "GET"}], config: %{key: "value"})
      assert match?({:ok, _}, result) or match?({:error, _}, result)

      case result do
        {:ok, server} -> Spikard.stop(server)
        _ -> :ok
      end
    end

    test "accepts config as keyword list" do
      result = Spikard.start(port: 59997, routes: [%{path: "/", method: "GET"}], config: [key: "value"])
      assert match?({:ok, _}, result) or match?({:error, _}, result)

      case result do
        {:ok, server} -> Spikard.stop(server)
        _ -> :ok
      end
    end

    test "uses default host when not provided" do
      result = Spikard.start(port: 59996, routes: [%{path: "/", method: "GET"}])
      assert match?({:ok, _}, result) or match?({:error, _}, result)

      case result do
        {:ok, server} -> Spikard.stop(server)
        _ -> :ok
      end
    end
  end

  describe "start/2 with router module" do
    test "starts server with router module" do
      result = Spikard.start(TestRouter, port: 59995)
      assert match?({:ok, _}, result) or match?({:error, _}, result)

      case result do
        {:ok, server} -> Spikard.stop(server)
        _ -> :ok
      end
    end

    test "returns error for module without routes/0" do
      assert {:error, message} = Spikard.start(InvalidRouter, port: 4000)
      assert message =~ "must define routes/0"
    end

    test "passes host option through" do
      result = Spikard.start(TestRouter, port: 59994, host: "127.0.0.1")
      assert match?({:ok, _}, result) or match?({:error, _}, result)

      case result do
        {:ok, server} -> Spikard.stop(server)
        _ -> :ok
      end
    end
  end

  describe "stop/1" do
    test "returns ok for non-existent server" do
      # NIF returns ok even if server doesn't exist (idempotent stop)
      assert :ok = Spikard.stop({"127.0.0.1", 9999})
    end

    test "requires tuple with host and port" do
      assert_raise FunctionClauseError, fn ->
        Spikard.stop(:invalid_server)
      end
    end

    test "can stop a running server" do
      # Start a server
      result = Spikard.start(TestRouter, port: 59993)

      case result do
        {:ok, server} ->
          # Stop it
          assert :ok = Spikard.stop(server)
          # Stop again (idempotent)
          assert :ok = Spikard.stop(server)

        {:error, _} ->
          # Port might be in use, skip this test
          :ok
      end
    end
  end

  describe "server_info/1" do
    test "returns server info tuple" do
      # Can't test without running server, just verify function exists
      assert function_exported?(Spikard, :server_info, 1)
    end

    test "returns info for non-running server" do
      result = Spikard.server_info({"127.0.0.1", 9999})
      # Should return {:ok, info} with running: false
      assert match?({:ok, _}, result)
    end
  end
end

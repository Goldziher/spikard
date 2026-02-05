defmodule E2EElixirApp.RequestTimeoutTest do
  @moduledoc """
  Generated tests for request_timeout fixtures.

  Each test starts its own isolated server with only its route,
  avoiding conflicts when multiple fixtures share the same path.
  """
  use ExUnit.Case, async: false

  alias E2EElixirApp.AppFactories

  @base_url "http://127.0.0.1:59800"

  setup do
    :inets.start()
    :ssl.start()
    :ok
  end

  @tag :integration
  test "test request timeout Request completes before timeout" do
    {routes, config} =
      AppFactories.create_app_handle_request_timeout_request_completes_before_timeout()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/timeouts/fast"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["status"] == "ok"
      assert parsed_body["duration"] == "fast"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test request timeout Request exceeds timeout" do
    {routes, config} = AppFactories.create_app_handle_request_timeout_request_exceeds_timeout()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/timeouts/slow"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 408, "Expected status 408, got #{status}"
    after
      Spikard.stop(server)
    end
  end
end

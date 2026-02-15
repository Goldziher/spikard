defmodule E2EElixirApp.RequestIdTest do
  @moduledoc """
  Generated tests for request_id fixtures.

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
  test "test request id Request ID header is preserved" do
    {routes, config} = AppFactories.create_app_handle_request_id_request_id_header_is_preserved()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/request-id/preserved"
      headers = [{~c"X-Request-ID", ~c"trace-123"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["status"] == "preserved"
      assert parsed_body["echo"] == "trace-123"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test request id Request ID is generated when not provided" do
    {routes, config} =
      AppFactories.create_app_handle_request_id_request_id_is_generated_when_not_provided()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/request-id/generated"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["status"] == "generated"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test request id Request ID middleware can be disabled" do
    {routes, config} =
      AppFactories.create_app_handle_request_id_request_id_middleware_can_be_disabled()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/request-id/disabled"
      headers = [{~c"X-Request-ID", ~c"external-id"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["status"] == "no-request-id"
    after
      Spikard.stop(server)
    end
  end
end

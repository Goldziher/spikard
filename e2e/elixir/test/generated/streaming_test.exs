defmodule E2EElixirApp.StreamingTest do
  @moduledoc """
  Generated tests for streaming fixtures.

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
  test "test streaming Binary log download" do
    {routes, config} = AppFactories.create_app_handle_streaming_binary_log_download()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/stream/logfile"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      assert resp_body_str == "LOG:\\u0000\\u0001\\u0002\\u0003|TAIL|\\u0007\\n"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test streaming Chunked CSV export" do
    {routes, config} = AppFactories.create_app_handle_streaming_chunked_csv_export()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/stream/csv-report"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      assert resp_body_str == "id,name,value\\n1,Alice,42\\n2,Bob,7\\n"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test streaming Stream JSON lines" do
    {routes, config} = AppFactories.create_app_handle_streaming_stream_json_lines()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/stream/json-lines"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)

      assert resp_body_str ==
               "{\"index\":0,\"payload\":\"alpha\"}\\n{\"index\":1,\"payload\":\"beta\"}\\n{\"index\":2,\"payload\":\"gamma\"}\\n"
    after
      Spikard.stop(server)
    end
  end
end

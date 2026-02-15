defmodule E2EElixirApp.CompressionTest do
  @moduledoc """
  Generated tests for compression fixtures.

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
  test "test compression Compression - gzip applied" do
    {routes, config} = AppFactories.create_app_handle_compression_compression___gzip_applied()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/compression/gzip"
      headers = [{~c"Accept-Encoding", ~c"gzip"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["message"] == "Compressed payload"

      assert parsed_body["payload"] ==
               "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test compression Compression - payload below min size is not compressed" do
    {routes, config} =
      AppFactories.create_app_handle_compression_compression___payload_below_min_size_is_not_compressed()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/compression/skip"
      headers = [{~c"Accept-Encoding", ~c"gzip"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["message"] == "Small payload"
      assert parsed_body["payload"] == "tiny"
    after
      Spikard.stop(server)
    end
  end
end

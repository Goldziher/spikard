defmodule E2EElixirApp.StaticFilesTest do
  @moduledoc """
  Generated tests for static_files fixtures.

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
  test "test static files Static file server returns text file" do
    {routes, config} =
      AppFactories.create_app_handle_static_files_static_file_server_returns_text_file()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/public/hello.txt"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      assert resp_body_str == "Hello from static storage"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test static files Static server returns index.html for directory" do
    {routes, config} =
      AppFactories.create_app_handle_static_files_static_server_returns_index_html_for_directory()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/app/"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      assert resp_body_str == "<!doctype html><h1>Welcome</h1>"
    after
      Spikard.stop(server)
    end
  end
end

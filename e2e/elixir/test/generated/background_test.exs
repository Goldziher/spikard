defmodule E2EElixirApp.BackgroundTest do
  @moduledoc """
  Generated tests for background fixtures.

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
  test "test background Background event logging" do
    {routes, config} = AppFactories.create_app_handle_background_background_event_logging()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/background/events"
      headers = []
      req_body = Jason.encode!(%{"event" => "alpha"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 202, "Expected status 202, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test background Background event logging - second payload" do
    {routes, config} =
      AppFactories.create_app_handle_background_background_event_logging___second_payload()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/background/events"
      headers = []
      req_body = Jason.encode!(%{"event" => "beta"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 202, "Expected status 202, got #{status}"
    after
      Spikard.stop(server)
    end
  end
end

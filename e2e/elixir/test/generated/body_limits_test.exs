defmodule E2EElixirApp.BodyLimitsTest do
  @moduledoc """
  Generated tests for body_limits fixtures.

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
  test "test body limits Body over limit returns 413" do
    {routes, config} = AppFactories.create_app_handle_body_limits_body_over_limit_returns_413()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/body-limit/over"
      headers = []

      req_body =
        Jason.encode!(%{
          "note" =>
            "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
        })

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 413, "Expected status 413, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test body limits Body under limit succeeds" do
    {routes, config} = AppFactories.create_app_handle_body_limits_body_under_limit_succeeds()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/body-limit/under"
      headers = []
      req_body = Jason.encode!(%{"note" => "small"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["accepted"] == true
      assert parsed_body["note"] == "small"
    after
      Spikard.stop(server)
    end
  end
end

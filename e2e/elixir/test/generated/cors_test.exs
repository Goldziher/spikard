defmodule E2EElixirApp.CorsTest do
  @moduledoc """
  Generated tests for cors fixtures.

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
  test "test cors 06 cors preflight method not allowed" do
    {routes, config} = AppFactories.create_app_handle_cors_06_cors_preflight_method_not_allowed()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/data"

      headers = [
        {~c"Origin", ~c"https://example.com"},
        {~c"Access-Control-Request-Method", ~c"DELETE"},
        {~c"Access-Control-Request-Headers", ~c"Content-Type"}
      ]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:options, {String.to_charlist(url), headers}, [], [])

      assert status == 403, "Expected status 403, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cors 07 cors preflight header not allowed" do
    {routes, config} = AppFactories.create_app_handle_cors_07_cors_preflight_header_not_allowed()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/data"

      headers = [
        {~c"Access-Control-Request-Headers", ~c"X-Custom-Header"},
        {~c"Origin", ~c"https://example.com"},
        {~c"Access-Control-Request-Method", ~c"POST"}
      ]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:options, {String.to_charlist(url), headers}, [], [])

      assert status == 403, "Expected status 403, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cors 08 cors max age" do
    {routes, config} = AppFactories.create_app_handle_cors_08_cors_max_age()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/data"

      headers = [
        {~c"Access-Control-Request-Headers", ~c"Content-Type"},
        {~c"Origin", ~c"https://example.com"},
        {~c"Access-Control-Request-Method", ~c"POST"}
      ]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:options, {String.to_charlist(url), headers}, [], [])

      assert status == 204, "Expected status 204, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cors 09 cors expose headers" do
    {routes, config} = AppFactories.create_app_handle_cors_09_cors_expose_headers()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/data"
      headers = [{~c"Origin", ~c"https://example.com"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cors 10 cors origin null" do
    {routes, config} = AppFactories.create_app_handle_cors_10_cors_origin_null()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/data"
      headers = [{~c"Origin", ~c"null"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 403, "Expected status 403, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["error"] == "Origin 'null' is not allowed"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cors CORS Private Network Access" do
    {routes, config} = AppFactories.create_app_handle_cors_cors_private_network_access()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/local-resource"

      headers = [
        {~c"Origin", ~c"https://public.example.com"},
        {~c"Access-Control-Request-Method", ~c"GET"},
        {~c"Access-Control-Request-Private-Network", ~c"true"}
      ]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:options, {String.to_charlist(url), headers}, [], [])

      assert status == 204, "Expected status 204, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cors CORS Vary header for proper caching" do
    {routes, config} = AppFactories.create_app_handle_cors_cors_vary_header_for_proper_caching()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/cached-resource"
      headers = [{~c"Cache-Control", ~c"max-age=3600"}, {~c"Origin", ~c"https://app.example.com"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["data"] == "cacheable resource"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cors CORS multiple allowed origins" do
    {routes, config} = AppFactories.create_app_handle_cors_cors_multiple_allowed_origins()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/data"
      headers = [{~c"Origin", ~c"https://admin.example.com"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["data"] == "resource data"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cors CORS origin case sensitivity" do
    {routes, config} = AppFactories.create_app_handle_cors_cors_origin_case_sensitivity()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/data"
      headers = [{~c"Origin", ~c"https://EXAMPLE.COM"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cors CORS preflight for DELETE method" do
    {routes, config} = AppFactories.create_app_handle_cors_cors_preflight_for_delete_method()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/resource/456"

      headers = [
        {~c"Origin", ~c"https://app.example.com"},
        {~c"Access-Control-Request-Method", ~c"DELETE"}
      ]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:options, {String.to_charlist(url), headers}, [], [])

      assert status == 204, "Expected status 204, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cors CORS preflight for PUT method" do
    {routes, config} = AppFactories.create_app_handle_cors_cors_preflight_for_put_method()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/resource/123"

      headers = [
        {~c"Access-Control-Request-Headers", ~c"Content-Type, X-Custom-Header"},
        {~c"Origin", ~c"https://app.example.com"},
        {~c"Access-Control-Request-Method", ~c"PUT"}
      ]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:options, {String.to_charlist(url), headers}, [], [])

      assert status == 204, "Expected status 204, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cors CORS preflight request" do
    {routes, config} = AppFactories.create_app_handle_cors_cors_preflight_request()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"

      headers = [
        {~c"Access-Control-Request-Headers", ~c"Content-Type, X-Custom-Header"},
        {~c"Access-Control-Request-Method", ~c"POST"},
        {~c"Origin", ~c"https://example.com"}
      ]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:options, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cors CORS regex pattern matching for origins" do
    {routes, config} =
      AppFactories.create_app_handle_cors_cors_regex_pattern_matching_for_origins()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/data"
      headers = [{~c"Origin", ~c"https://subdomain.example.com"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["data"] == "resource data"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cors CORS request blocked" do
    {routes, config} = AppFactories.create_app_handle_cors_cors_request_blocked()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Origin", ~c"https://malicious-site.com"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 403, "Expected status 403, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)

      assert parsed_body["detail"] ==
               "CORS request from origin 'https://malicious-site.com' not allowed"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cors CORS safelisted headers without preflight" do
    {routes, config} =
      AppFactories.create_app_handle_cors_cors_safelisted_headers_without_preflight()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/form"

      headers = [
        {~c"Accept-Language", ~c"en-US"},
        {~c"Origin", ~c"https://app.example.com"},
        {~c"Content-Type", ~c"text/plain"},
        {~c"Accept", ~c"application/json"}
      ]

      req_body = ""

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"text/plain", req_body},
          [],
          []
        )

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["message"] == "Success"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cors CORS wildcard origin" do
    {routes, config} = AppFactories.create_app_handle_cors_cors_wildcard_origin()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/public/data"
      headers = [{~c"Origin", ~c"https://random-site.com"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["data"] == "public"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cors CORS with credentials" do
    {routes, config} = AppFactories.create_app_handle_cors_cors_with_credentials()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/user/profile"
      headers = [{~c"Origin", ~c"https://app.example.com"}, {~c"Cookie", ~c"session=abc123"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["username"] == "john"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cors Simple CORS request" do
    {routes, config} = AppFactories.create_app_handle_cors_simple_cors_request()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Origin", ~c"https://example.com"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert Map.has_key?(parsed_body, "items")
    after
      Spikard.stop(server)
    end
  end
end

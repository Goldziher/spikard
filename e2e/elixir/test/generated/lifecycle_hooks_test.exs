defmodule E2EElixirApp.LifecycleHooksTest do
  @moduledoc """
  Generated tests for lifecycle_hooks fixtures.

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
  test "test lifecycle hooks Hook Execution Order" do
    {routes, config} = AppFactories.create_app_handle_lifecycle_hooks_hook_execution_order()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/test-hook-order"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["message"] == "Hooks executed in order"
      assert Map.has_key?(parsed_body, "execution_order")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test lifecycle hooks Multiple Hooks - All Phases" do
    {routes, config} =
      AppFactories.create_app_handle_lifecycle_hooks_multiple_hooks___all_phases()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/full-lifecycle"

      headers = [
        {~c"Authorization", ~c"Bearer valid-token-12345"},
        {~c"Content-Type", ~c"application/json"}
      ]

      req_body = Jason.encode!(%{"user_id" => "user-123", "action" => "update_profile"})

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
      assert parsed_body["message"] == "Action completed successfully"
      assert parsed_body["user_id"] == "user-123"
      assert parsed_body["action"] == "update_profile"
      assert parsed_body["request_id"] == ".*"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test lifecycle hooks onError - Error Logging" do
    {routes, config} = AppFactories.create_app_handle_lifecycle_hooks_onerror___error_logging()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/test-error"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 500, "Expected status 500, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["error"] == "Internal Server Error"
      assert parsed_body["message"] == "An unexpected error occurred"
      assert parsed_body["error_id"] == ".*"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test lifecycle hooks onRequest - Request Logging" do
    {routes, config} =
      AppFactories.create_app_handle_lifecycle_hooks_onrequest___request_logging()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/test-on-request"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["message"] == "onRequest hooks executed"
      assert parsed_body["request_logged"] == true
      assert parsed_body["has_request_id"] == true
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test lifecycle hooks onResponse - Response Timing" do
    {routes, config} =
      AppFactories.create_app_handle_lifecycle_hooks_onresponse___response_timing()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/test-timing"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["message"] == "Response with timing info"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test lifecycle hooks onResponse - Security Headers" do
    {routes, config} =
      AppFactories.create_app_handle_lifecycle_hooks_onresponse___security_headers()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/test-security-headers"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["message"] == "Response with security headers"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test lifecycle hooks preHandler - Authentication Failed (Short Circuit)" do
    {routes, config} =
      AppFactories.create_app_handle_lifecycle_hooks_prehandler___authentication_failed__short_circuit_()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/protected-resource-fail"
      headers = [{~c"Authorization", ~c"Bearer invalid-token"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 401, "Expected status 401, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["error"] == "Unauthorized"
      assert parsed_body["message"] == "Invalid or expired authentication token"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test lifecycle hooks preHandler - Authentication Success" do
    {routes, config} =
      AppFactories.create_app_handle_lifecycle_hooks_prehandler___authentication_success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/protected-resource"
      headers = [{~c"Authorization", ~c"Bearer valid-token-12345"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["message"] == "Access granted"
      assert parsed_body["user_id"] == "user-123"
      assert parsed_body["authenticated"] == true
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test lifecycle hooks preHandler - Authorization Check" do
    {routes, config} =
      AppFactories.create_app_handle_lifecycle_hooks_prehandler___authorization_check()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/admin-only"
      headers = [{~c"Authorization", ~c"Bearer admin-token-67890"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["message"] == "Admin access granted"
      assert parsed_body["user_id"] == "admin-456"
      assert parsed_body["role"] == "admin"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test lifecycle hooks preHandler - Authorization Forbidden (Short Circuit)" do
    {routes, config} =
      AppFactories.create_app_handle_lifecycle_hooks_prehandler___authorization_forbidden__short_circuit_()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/admin-only-forbidden"
      headers = [{~c"Authorization", ~c"Bearer user-token-11111"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 403, "Expected status 403, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["error"] == "Forbidden"
      assert parsed_body["message"] == "Admin role required for this endpoint"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test lifecycle hooks preValidation - Rate Limit Exceeded (Short Circuit)" do
    {routes, config} =
      AppFactories.create_app_handle_lifecycle_hooks_prevalidation___rate_limit_exceeded__short_circuit_()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/test-rate-limit-exceeded"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"data" => "test"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 429, "Expected status 429, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["error"] == "Rate limit exceeded"
      assert parsed_body["message"] == "Too many requests, please try again later"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test lifecycle hooks preValidation - Rate Limiting" do
    {routes, config} =
      AppFactories.create_app_handle_lifecycle_hooks_prevalidation___rate_limiting()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/test-rate-limit"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"data" => "test"})

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
      assert parsed_body["message"] == "Request accepted"
      assert parsed_body["rate_limit_checked"] == true
    after
      Spikard.stop(server)
    end
  end
end

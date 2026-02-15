defmodule E2EElixirApp.DiTest do
  @moduledoc """
  Generated tests for di fixtures.

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
  test "test di Async factory dependency - success" do
    {routes, config} = AppFactories.create_app_handle_di_async_factory_dependency___success()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/db-status"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["pool_status"] == "connected"
      assert parsed_body["max_size"] == 10
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test di Circular dependency detection - error" do
    {routes, config} = AppFactories.create_app_handle_di_circular_dependency_detection___error()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/circular"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 500, "Expected status 500, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["type"] == "https://spikard.dev/errors/dependency-error"
      assert parsed_body["title"] == "Dependency Resolution Failed"
      assert parsed_body["status"] == 500
      assert parsed_body["detail"] == "Circular dependency detected"
      assert Map.has_key?(parsed_body, "errors")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test di Dependency injection in lifecycle hooks - success" do
    {routes, config} =
      AppFactories.create_app_handle_di_dependency_injection_in_lifecycle_hooks___success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/hook-di-test"
      headers = [{~c"authorization", ~c"Bearer valid_token"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["authenticated"] == true
      assert parsed_body["logged"] == true
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test di Factory dependency - success" do
    {routes, config} = AppFactories.create_app_handle_di_factory_dependency___success()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/timestamp"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["timestamp"] == "<<present>>"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test di Missing dependency - error" do
    {routes, config} = AppFactories.create_app_handle_di_missing_dependency___error()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/missing-dep"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 500, "Expected status 500, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["type"] == "https://spikard.dev/errors/dependency-error"
      assert parsed_body["title"] == "Dependency Resolution Failed"
      assert parsed_body["status"] == 500
      assert parsed_body["detail"] == "Required dependency not found"
      assert Map.has_key?(parsed_body, "errors")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test di Mixed singleton and per-request caching - success" do
    {routes, config} =
      AppFactories.create_app_handle_di_mixed_singleton_and_per_request_caching___success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/mixed-caching"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["app_name"] == "MyApp"
      assert parsed_body["pool_id"] == "<<uuid>>"
      assert parsed_body["context_id"] == "<<uuid>>"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test di Multiple dependencies with cleanup - success" do
    {routes, config} =
      AppFactories.create_app_handle_di_multiple_dependencies_with_cleanup___success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/multi-cleanup-test"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["session_active"] == true
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test di Nested dependencies (3 levels) - success" do
    {routes, config} =
      AppFactories.create_app_handle_di_nested_dependencies__3_levels____success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/auth-status"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["auth_enabled"] == true
      assert parsed_body["has_db"] == true
      assert parsed_body["has_cache"] == true
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test di Node.js object destructuring injection - success" do
    {routes, config} =
      AppFactories.create_app_handle_di_node_js_object_destructuring_injection___success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/node-destructure"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["db_name"] == "PostgreSQL"
      assert parsed_body["log_level"] == "info"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test di Per-request dependency caching - success" do
    {routes, config} =
      AppFactories.create_app_handle_di_per_request_dependency_caching___success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/request-id"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["first_id"] == "<<uuid>>"
      assert parsed_body["second_id"] == "<<same_as:first_id>>"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test di Python parameter name-based injection - success" do
    {routes, config} =
      AppFactories.create_app_handle_di_python_parameter_name_based_injection___success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/python-name-inject"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["db_status"] == "connected"
      assert parsed_body["cache_status"] == "ready"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test di Python type annotation-based injection - success" do
    {routes, config} =
      AppFactories.create_app_handle_di_python_type_annotation_based_injection___success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/python-type-inject"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["pool_type"] == "PostgreSQL"
      assert parsed_body["cache_type"] == "Redis"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test di Resource cleanup after request - success" do
    {routes, config} =
      AppFactories.create_app_handle_di_resource_cleanup_after_request___success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/cleanup-test"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["session_id"] == "<<uuid>>"
      assert parsed_body["status"] == "completed"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test di Route-level dependency override - success" do
    {routes, config} =
      AppFactories.create_app_handle_di_route_level_dependency_override___success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/override-test"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["mode"] == "test"
      assert parsed_body["strict"] == false
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test di Ruby keyword argument injection - success" do
    {routes, config} =
      AppFactories.create_app_handle_di_ruby_keyword_argument_injection___success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/ruby-kwargs"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["adapter"] == "postgresql"
      assert parsed_body["user_id"] == 42
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test di Singleton dependency caching - success" do
    {routes, config} = AppFactories.create_app_handle_di_singleton_dependency_caching___success()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/app-counter"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["counter_id"] == "<<uuid>>"
      assert parsed_body["count"] == 1
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test di Type mismatch in dependency resolution - error" do
    {routes, config} =
      AppFactories.create_app_handle_di_type_mismatch_in_dependency_resolution___error()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/type-mismatch"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 500, "Expected status 500, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["type"] == "https://spikard.dev/errors/dependency-error"
      assert parsed_body["title"] == "Dependency Resolution Failed"
      assert parsed_body["status"] == 500
      assert parsed_body["detail"] == "Dependency type mismatch"
      assert Map.has_key?(parsed_body, "errors")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test di Value dependency injection - success" do
    {routes, config} = AppFactories.create_app_handle_di_value_dependency_injection___success()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/config"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["app_name"] == "SpikardApp"
      assert parsed_body["version"] == "1.0.0"
      assert parsed_body["max_connections"] == 100
    after
      Spikard.stop(server)
    end
  end
end

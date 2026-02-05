defmodule E2EElixirApp.HttpMethodsTest do
  @moduledoc """
  Generated tests for http_methods fixtures.

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
  test "test http methods DELETE - Remove resource" do
    {routes, config} = AppFactories.create_app_handle_http_methods_delete___remove_resource()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/1"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:delete, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test http methods DELETE - Resource not found" do
    {routes, config} = AppFactories.create_app_handle_http_methods_delete___resource_not_found()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/999"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:delete, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test http methods DELETE - With response body" do
    {routes, config} = AppFactories.create_app_handle_http_methods_delete___with_response_body()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/1"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:delete, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["id"] == 1
      assert parsed_body["name"] == "Deleted Item"
      assert parsed_body["message"] == "Item deleted successfully"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test http methods HEAD - Get metadata without body" do
    {routes, config} =
      AppFactories.create_app_handle_http_methods_head___get_metadata_without_body()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/1"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, _resp_body}} =
        :httpc.request(:head, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test http methods OPTIONS - CORS preflight request" do
    {routes, config} =
      AppFactories.create_app_handle_http_methods_options___cors_preflight_request()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"

      headers = [
        {~c"Origin", ~c"https://example.com"},
        {~c"Access-Control-Request-Method", ~c"POST"},
        {~c"Access-Control-Request-Headers", ~c"Content-Type"}
      ]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:options, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test http methods PATCH - Partial update" do
    {routes, config} = AppFactories.create_app_handle_http_methods_patch___partial_update()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/1"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"price" => 79.99})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :patch,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["id"] == 1
      assert parsed_body["name"] == "Existing Item"
      assert parsed_body["price"] == 79.99
      assert parsed_body["in_stock"] == true
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test http methods PATCH - Update multiple fields" do
    {routes, config} =
      AppFactories.create_app_handle_http_methods_patch___update_multiple_fields()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/1"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"name" => "Updated Name", "price" => 89.99, "in_stock" => false})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :patch,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["id"] == 1
      assert parsed_body["name"] == "Updated Name"
      assert parsed_body["price"] == 89.99
      assert parsed_body["in_stock"] == false
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test http methods PUT - Complete resource replacement" do
    {routes, config} =
      AppFactories.create_app_handle_http_methods_put___complete_resource_replacement()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/1"
      headers = [{~c"Content-Type", ~c"application/json"}]

      req_body =
        Jason.encode!(%{
          "id" => 1,
          "name" => "Updated Item",
          "description" => "Completely replaced",
          "price" => 99.99,
          "in_stock" => true
        })

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :put,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["id"] == 1
      assert parsed_body["name"] == "Updated Item"
      assert parsed_body["description"] == "Completely replaced"
      assert parsed_body["price"] == 99.99
      assert parsed_body["in_stock"] == true
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test http methods PUT - Create resource if doesn't exist" do
    {routes, config} =
      AppFactories.create_app_handle_http_methods_put___create_resource_if_doesn_t_exist()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/999"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"id" => 999, "name" => "New Item", "price" => 49.99})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :put,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["id"] == 999
      assert parsed_body["name"] == "New Item"
      assert parsed_body["price"] == 49.99
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test http methods PUT - Idempotent operation" do
    {routes, config} = AppFactories.create_app_handle_http_methods_put___idempotent_operation()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/1"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"id" => 1, "name" => "Fixed Name", "price" => 50.0})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :put,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["id"] == 1
      assert parsed_body["name"] == "Fixed Name"
      assert parsed_body["price"] == 50.0
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test http methods PUT - Missing required field" do
    {routes, config} = AppFactories.create_app_handle_http_methods_put___missing_required_field()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/1"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"id" => 1, "name" => "Item Name"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :put,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 422, "Expected status 422, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["type"] == "https://spikard.dev/errors/validation-error"
      assert parsed_body["title"] == "Request Validation Failed"
      assert parsed_body["status"] == 422
      assert parsed_body["detail"] == "1 validation error in request"
      assert Map.has_key?(parsed_body, "errors")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test http methods PUT - Validation error" do
    {routes, config} = AppFactories.create_app_handle_http_methods_put___validation_error()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/1"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"id" => 1, "name" => "X", "price" => -10})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :put,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 422, "Expected status 422, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["type"] == "https://spikard.dev/errors/validation-error"
      assert parsed_body["title"] == "Request Validation Failed"
      assert parsed_body["status"] == 422
      assert parsed_body["detail"] == "2 validation errors in request"
      assert Map.has_key?(parsed_body, "errors")
    after
      Spikard.stop(server)
    end
  end
end

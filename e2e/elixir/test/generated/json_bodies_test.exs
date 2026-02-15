defmodule E2EElixirApp.JsonBodiesTest do
  @moduledoc """
  Generated tests for json_bodies fixtures.

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
  test "test json bodies 29 nested object validation success" do
    {routes, config} =
      AppFactories.create_app_handle_json_bodies_29_nested_object_validation_success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/users"
      headers = []

      req_body =
        Jason.encode!(%{"profile" => %{"name" => "John Doe", "email" => "john@example.com"}})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 201, "Expected status 201, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies 30 nested object missing field" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_30_nested_object_missing_field()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/users"
      headers = []
      req_body = Jason.encode!(%{"profile" => %{"name" => "John Doe"}})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
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
  test "test json bodies 31 nullable property null value" do
    {routes, config} =
      AppFactories.create_app_handle_json_bodies_31_nullable_property_null_value()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/users"
      headers = []
      req_body = Jason.encode!(%{"name" => "Test User", "description" => nil})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 201, "Expected status 201, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies 32 schema ref definitions" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_32_schema_ref_definitions()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/products"
      headers = []
      req_body = Jason.encode!(%{"product" => %{"name" => "Widget", "price" => 9.99}})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 201, "Expected status 201, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies 33 allof schema composition" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_33_allof_schema_composition()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items"
      headers = []
      req_body = Jason.encode!(%{"name" => "Product", "price" => 29.99})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 201, "Expected status 201, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies 34 additional properties false" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_34_additional_properties_false()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/users"
      headers = []

      req_body =
        Jason.encode!(%{
          "name" => "John",
          "email" => "john@example.com",
          "extra_field" => "should fail"
        })

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
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
  test "test json bodies 35 oneof schema success" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_35_oneof_schema_success()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/payment"
      headers = []
      req_body = Jason.encode!(%{"credit_card" => "1234567812345678"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 201, "Expected status 201, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies 36 oneof schema multiple match failure" do
    {routes, config} =
      AppFactories.create_app_handle_json_bodies_36_oneof_schema_multiple_match_failure()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/payment"
      headers = []

      req_body =
        Jason.encode!(%{
          "credit_card" => "1234567812345678",
          "paypal_email" => "user@example.com"
        })

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
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
  test "test json bodies 37 oneof schema no match failure" do
    {routes, config} =
      AppFactories.create_app_handle_json_bodies_37_oneof_schema_no_match_failure()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/payment"
      headers = []
      req_body = Jason.encode!(%{"bitcoin_address" => "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
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
  test "test json bodies 38 anyof schema success" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_38_anyof_schema_success()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/contact"
      headers = []
      req_body = Jason.encode!(%{"name" => "John Doe", "email" => "john@example.com"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 201, "Expected status 201, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies 39 anyof schema multiple match success" do
    {routes, config} =
      AppFactories.create_app_handle_json_bodies_39_anyof_schema_multiple_match_success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/contact"
      headers = []

      req_body =
        Jason.encode!(%{
          "name" => "John Doe",
          "email" => "john@example.com",
          "phone" => "+1-555-0100"
        })

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 201, "Expected status 201, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies 40 anyof schema failure" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_40_anyof_schema_failure()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/contact"
      headers = []
      req_body = Jason.encode!(%{"name" => "John Doe"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
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
  test "test json bodies 41 not schema success" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_41_not_schema_success()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/users"
      headers = []
      req_body = Jason.encode!(%{"username" => "john_doe"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 201, "Expected status 201, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies 42 not schema failure" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_42_not_schema_failure()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/users"
      headers = []
      req_body = Jason.encode!(%{"username" => "admin"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
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
  test "test json bodies 43 const validation success" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_43_const_validation_success()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/v1/data"
      headers = []
      req_body = Jason.encode!(%{"version" => "1.0", "data" => "test"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 201, "Expected status 201, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies 44 const validation failure" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_44_const_validation_failure()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/v1/data"
      headers = []
      req_body = Jason.encode!(%{"version" => "2.0", "data" => "test"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
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
  test "test json bodies 45 minproperties validation success" do
    {routes, config} =
      AppFactories.create_app_handle_json_bodies_45_minproperties_validation_success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/config"
      headers = []
      req_body = Jason.encode!(%{"host" => "localhost", "port" => 8080})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 201, "Expected status 201, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies 46 minproperties validation failure" do
    {routes, config} =
      AppFactories.create_app_handle_json_bodies_46_minproperties_validation_failure()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/config"
      headers = []
      req_body = Jason.encode!(%{"host" => "localhost"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
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
  test "test json bodies 47 maxproperties validation failure" do
    {routes, config} =
      AppFactories.create_app_handle_json_bodies_47_maxproperties_validation_failure()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/config"
      headers = []

      req_body =
        Jason.encode!(%{"host" => "localhost", "port" => 8080, "ssl" => true, "debug" => false})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
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
  test "test json bodies 48 dependencies validation success" do
    {routes, config} =
      AppFactories.create_app_handle_json_bodies_48_dependencies_validation_success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/billing"
      headers = []

      req_body =
        Jason.encode!(%{
          "name" => "John Doe",
          "credit_card" => "1234567812345678",
          "billing_address" => "123 Main St"
        })

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 201, "Expected status 201, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies 49 dependencies validation failure" do
    {routes, config} =
      AppFactories.create_app_handle_json_bodies_49_dependencies_validation_failure()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/billing"
      headers = []
      req_body = Jason.encode!(%{"name" => "John Doe", "credit_card" => "1234567812345678"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
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
  test "test json bodies 50 deep nesting 4 levels" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_50_deep_nesting_4_levels()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/data"
      headers = []

      req_body =
        Jason.encode!(%{
          "user" => %{"profile" => %{"contact" => %{"address" => %{"street" => "123 Main St"}}}}
        })

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 201, "Expected status 201, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies Array of objects - success" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_array_of_objects___success()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/list"
      headers = [{~c"Content-Type", ~c"application/json"}]

      req_body =
        Jason.encode!(%{
          "name" => "Product Bundle",
          "tags" => ["electronics", "gadget"],
          "images" => [
            %{"url" => "https://example.com/img1.jpg", "name" => "Front"},
            %{"url" => "https://example.com/img2.jpg", "name" => "Back"}
          ]
        })

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
      assert parsed_body["name"] == "Product Bundle"
      assert Map.has_key?(parsed_body, "tags")
      assert Map.has_key?(parsed_body, "images")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies Array of primitive values" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_array_of_primitive_values()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/json"}]

      req_body =
        Jason.encode!(%{
          "name" => "Product",
          "tags" => ["electronics", "gadget", "new"],
          "ratings" => [4.5, 4.8, 5.0, 4.2]
        })

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
      assert parsed_body["name"] == "Product"
      assert Map.has_key?(parsed_body, "tags")
      assert Map.has_key?(parsed_body, "ratings")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies Body with query parameters" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_body_with_query_parameters()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/?limit=10"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"name" => "Item", "price" => 42.0})

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
      assert Map.has_key?(parsed_body, "item")
      assert parsed_body["limit"] == 10
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies Boolean field - success" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_boolean_field___success()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"name" => "Item", "price" => 42.0, "in_stock" => true})

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
      assert parsed_body["name"] == "Item"
      assert parsed_body["price"] == 42.0
      assert parsed_body["in_stock"] == true
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies Date field - success" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_date_field___success()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/events/"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"name" => "Conference", "event_date" => "2024-03-15"})

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
      assert parsed_body["name"] == "Conference"
      assert parsed_body["event_date"] == "2024-03-15"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies Datetime field - success" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_datetime_field___success()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/events/"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"name" => "Meeting", "created_at" => "2024-03-15T10:30:00Z"})

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
      assert parsed_body["name"] == "Meeting"
      assert parsed_body["created_at"] == "2024-03-15T10:30:00Z"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies Deeply nested objects" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_deeply_nested_objects()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/nested"
      headers = [{~c"Content-Type", ~c"application/json"}]

      req_body =
        Jason.encode!(%{
          "name" => "Product",
          "price" => 100.0,
          "seller" => %{
            "name" => "John Doe",
            "address" => %{
              "street" => "123 Main St",
              "city" => "Springfield",
              "country" => %{"name" => "USA", "code" => "US"}
            }
          }
        })

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
      assert parsed_body["name"] == "Product"
      assert parsed_body["price"] == 100.0
      assert Map.has_key?(parsed_body, "seller")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies Empty JSON object" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_empty_json_object()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/optional-all"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{})

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
      assert Map.has_key?(parsed_body, "name")
      assert Map.has_key?(parsed_body, "description")
      assert Map.has_key?(parsed_body, "price")
      assert Map.has_key?(parsed_body, "tax")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies Empty array validation - fail" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_empty_array_validation___fail()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/list-validated"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"name" => "Product", "tags" => []})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
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
  test "test json bodies Enum field - invalid value" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_enum_field___invalid_value()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"name" => "Item", "category" => "furniture"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
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
  test "test json bodies Enum field - success" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_enum_field___success()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"name" => "Item", "category" => "electronics"})

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
      assert parsed_body["name"] == "Item"
      assert parsed_body["category"] == "electronics"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies Extra fields ignored (no additionalProperties)" do
    {routes, config} =
      AppFactories.create_app_handle_json_bodies_extra_fields_ignored__no_additionalproperties_()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/json"}]

      req_body =
        Jason.encode!(%{
          "name" => "Item",
          "price" => 42.0,
          "extra_field" => "this should be ignored",
          "another_extra" => 123
        })

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
      assert parsed_body["name"] == "Item"
      assert parsed_body["price"] == 42.0
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies Field type validation - invalid type" do
    {routes, config} =
      AppFactories.create_app_handle_json_bodies_field_type_validation___invalid_type()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/json"}]

      req_body =
        Jason.encode!(%{
          "name" => "Foo",
          "description" => "A very nice Item",
          "price" => "not a number",
          "tax" => 3.2
        })

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
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
  test "test json bodies Nested object - success" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_nested_object___success()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/nested"
      headers = [{~c"Content-Type", ~c"application/json"}]

      req_body =
        Jason.encode!(%{
          "name" => "Foo",
          "price" => 42.0,
          "image" => %{"url" => "https://example.com/image.jpg", "name" => "Product Image"}
        })

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
      assert parsed_body["name"] == "Foo"
      assert parsed_body["price"] == 42.0
      assert Map.has_key?(parsed_body, "image")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies Null value for optional field" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_null_value_for_optional_field()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/json"}]

      req_body =
        Jason.encode!(%{"name" => "Item", "price" => 42.0, "description" => nil, "tax" => nil})

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
      assert parsed_body["name"] == "Item"
      assert parsed_body["price"] == 42.0
      assert Map.has_key?(parsed_body, "description")
      assert Map.has_key?(parsed_body, "tax")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies Numeric ge validation - fail" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_numeric_ge_validation___fail()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/validated"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"name" => "Item", "price" => 0.5})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
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
  test "test json bodies Numeric le validation - success" do
    {routes, config} =
      AppFactories.create_app_handle_json_bodies_numeric_le_validation___success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/validated"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"name" => "Item", "price" => 100.0})

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
      assert parsed_body["name"] == "Item"
      assert parsed_body["price"] == 100.0
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies Optional fields - omitted" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_optional_fields___omitted()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"name" => "Foo", "price" => 35.4})

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
      assert parsed_body["name"] == "Foo"
      assert parsed_body["price"] == 35.4
      assert Map.has_key?(parsed_body, "description")
      assert Map.has_key?(parsed_body, "tax")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies PATCH partial update" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_patch_partial_update()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/1"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"price" => 45.0})

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
      assert parsed_body["name"] == "Original Item"
      assert parsed_body["price"] == 45.0
      assert parsed_body["description"] == "Original description"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies Required field missing - validation error" do
    {routes, config} =
      AppFactories.create_app_handle_json_bodies_required_field_missing___validation_error()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"description" => "A very nice Item", "price" => 35.4})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
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
  test "test json bodies Simple JSON object - success" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_simple_json_object___success()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/json"}]

      req_body =
        Jason.encode!(%{
          "name" => "Foo",
          "description" => "A very nice Item",
          "price" => 35.4,
          "tax" => 3.2
        })

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
      assert parsed_body["name"] == "Foo"
      assert parsed_body["description"] == "A very nice Item"
      assert parsed_body["price"] == 35.4
      assert parsed_body["tax"] == 3.2
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies String max length validation - fail" do
    {routes, config} =
      AppFactories.create_app_handle_json_bodies_string_max_length_validation___fail()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/validated"
      headers = [{~c"Content-Type", ~c"application/json"}]

      req_body =
        Jason.encode!(%{
          "name" => "This is a very long name that exceeds the maximum length",
          "price" => 35.4
        })

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
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
  test "test json bodies String min length validation - fail" do
    {routes, config} =
      AppFactories.create_app_handle_json_bodies_string_min_length_validation___fail()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/validated"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"name" => "ab", "price" => 35.4})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
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
  test "test json bodies String pattern validation - fail" do
    {routes, config} =
      AppFactories.create_app_handle_json_bodies_string_pattern_validation___fail()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/validated"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"name" => "Item", "sku" => "ABC-123"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
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
  test "test json bodies String pattern validation - success" do
    {routes, config} =
      AppFactories.create_app_handle_json_bodies_string_pattern_validation___success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/validated"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"name" => "Item", "sku" => "ABC1234"})

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
      assert parsed_body["name"] == "Item"
      assert parsed_body["sku"] == "ABC1234"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test json bodies UUID field - invalid format" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_uuid_field___invalid_format()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"name" => "Item", "item_id" => "not-a-valid-uuid"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
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
  test "test json bodies UUID field - success" do
    {routes, config} = AppFactories.create_app_handle_json_bodies_uuid_field___success()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/json"}]

      req_body =
        Jason.encode!(%{"name" => "Item", "item_id" => "c892496f-b1fd-4b91-bdb8-b46f92df1716"})

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
      assert parsed_body["name"] == "Item"
      assert parsed_body["item_id"] == "c892496f-b1fd-4b91-bdb8-b46f92df1716"
    after
      Spikard.stop(server)
    end
  end
end

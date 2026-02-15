defmodule E2EElixirApp.ValidationErrorsTest do
  @moduledoc """
  Generated tests for validation_errors fixtures.

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
  test "test validation errors 09 multiple validation errors" do
    {routes, config} =
      AppFactories.create_app_handle_validation_errors_09_multiple_validation_errors()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/users"
      headers = []
      req_body = Jason.encode!(%{"name" => "ab", "email" => "invalid-email", "age" => 15})

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
      assert parsed_body["detail"] == "3 validation errors in request"
      assert Map.has_key?(parsed_body, "errors")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test validation errors 10 nested error path" do
    {routes, config} = AppFactories.create_app_handle_validation_errors_10_nested_error_path()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/profiles"
      headers = []
      req_body = Jason.encode!(%{"profile" => %{"contact" => %{"email" => "invalid"}}})

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
  test "test validation errors Array item validation error" do
    {routes, config} =
      AppFactories.create_app_handle_validation_errors_array_item_validation_error()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/json"}]

      req_body =
        Jason.encode!(%{"name" => "Item", "price" => 10.0, "tags" => ["tag1", "tag2", 123]})

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
  test "test validation errors Array max items constraint violation" do
    {routes, config} =
      AppFactories.create_app_handle_validation_errors_array_max_items_constraint_violation()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/json"}]

      req_body =
        Jason.encode!(%{
          "name" => "Item",
          "price" => 10.0,
          "tags" => [
            "tag1",
            "tag2",
            "tag3",
            "tag4",
            "tag5",
            "tag6",
            "tag7",
            "tag8",
            "tag9",
            "tag10",
            "tag11"
          ]
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
  test "test validation errors Array min items constraint violation" do
    {routes, config} =
      AppFactories.create_app_handle_validation_errors_array_min_items_constraint_violation()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"name" => "Item", "price" => 10.0, "tags" => []})

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
  test "test validation errors Body field type error - string for float" do
    {routes, config} =
      AppFactories.create_app_handle_validation_errors_body_field_type_error___string_for_float()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"name" => "Item", "price" => "not_a_float"})

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
  test "test validation errors Header validation error" do
    {routes, config} = AppFactories.create_app_handle_validation_errors_header_validation_error()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/?q=test"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

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
  test "test validation errors Invalid UUID format" do
    {routes, config} = AppFactories.create_app_handle_validation_errors_invalid_uuid_format()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/not-a-uuid"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

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
  test "test validation errors Invalid boolean value" do
    {routes, config} = AppFactories.create_app_handle_validation_errors_invalid_boolean_value()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/?q=test&is_active=maybe"
      headers = [{~c"x-token", ~c"test-token"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

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
  test "test validation errors Invalid datetime format" do
    {routes, config} = AppFactories.create_app_handle_validation_errors_invalid_datetime_format()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/json"}]

      req_body =
        Jason.encode!(%{"name" => "Item", "price" => 10.0, "created_at" => "not-a-datetime"})

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
  test "test validation errors Invalid enum value" do
    {routes, config} = AppFactories.create_app_handle_validation_errors_invalid_enum_value()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/models/invalid_model"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

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
  test "test validation errors Malformed JSON body" do
    {routes, config} = AppFactories.create_app_handle_validation_errors_malformed_json_body()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!("{\"name\": \"Item\", \"price\": }")

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 400, "Expected status 400, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["detail"] == "Invalid request format"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test validation errors Missing required body field" do
    {routes, config} =
      AppFactories.create_app_handle_validation_errors_missing_required_body_field()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"name" => "Item"})

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
  test "test validation errors Missing required query parameter" do
    {routes, config} =
      AppFactories.create_app_handle_validation_errors_missing_required_query_parameter()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"x-token", ~c"test-token"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

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
  test "test validation errors Multiple validation errors" do
    {routes, config} =
      AppFactories.create_app_handle_validation_errors_multiple_validation_errors()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"name" => "X", "price" => -10, "quantity" => "not_a_number"})

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
      assert parsed_body["detail"] == "3 validation errors in request"
      assert Map.has_key?(parsed_body, "errors")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test validation errors Nested object validation error" do
    {routes, config} =
      AppFactories.create_app_handle_validation_errors_nested_object_validation_error()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/json"}]

      req_body =
        Jason.encode!(%{
          "name" => "Product",
          "price" => 10.0,
          "seller" => %{"name" => "Jo", "address" => %{"city" => "SF", "zip_code" => "123"}}
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
      assert parsed_body["detail"] == "3 validation errors in request"
      assert Map.has_key?(parsed_body, "errors")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test validation errors Numeric constraint violation - gt (greater than)" do
    {routes, config} =
      AppFactories.create_app_handle_validation_errors_numeric_constraint_violation___gt__greater_than_()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/?q=test&price=0"
      headers = [{~c"x-token", ~c"test-token"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

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
  test "test validation errors Numeric constraint violation - le (less than or equal)" do
    {routes, config} =
      AppFactories.create_app_handle_validation_errors_numeric_constraint_violation___le__less_than_or_equal_()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/?q=test&limit=101"
      headers = [{~c"x-token", ~c"test-token"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

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
  test "test validation errors Query param type error - string provided for int" do
    {routes, config} =
      AppFactories.create_app_handle_validation_errors_query_param_type_error___string_provided_for_int()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/?q=test&skip=not_a_number"
      headers = [{~c"x-token", ~c"test-token"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

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
  test "test validation errors String max length constraint violation" do
    {routes, config} =
      AppFactories.create_app_handle_validation_errors_string_max_length_constraint_violation()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url =
        @base_url <>
          "/items/?q=this_is_a_very_long_query_string_that_exceeds_maximum_length_limit_for_this_parameter"

      headers = [{~c"x-token", ~c"test-token"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

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
  test "test validation errors String min length constraint violation" do
    {routes, config} =
      AppFactories.create_app_handle_validation_errors_string_min_length_constraint_violation()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/?q=ab"
      headers = [{~c"x-token", ~c"test-token"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

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
  test "test validation errors String regex pattern mismatch" do
    {routes, config} =
      AppFactories.create_app_handle_validation_errors_string_regex_pattern_mismatch()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/?q=invalid!"
      headers = [{~c"x-token", ~c"test-token"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

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
end

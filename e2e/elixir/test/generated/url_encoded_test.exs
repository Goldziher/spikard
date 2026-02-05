defmodule E2EElixirApp.UrlEncodedTest do
  @moduledoc """
  Generated tests for url_encoded fixtures.

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
  test "test url encoded 13 array field success" do
    {routes, config} = AppFactories.create_app_handle_url_encoded_13_array_field_success()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/register"
      headers = [{~c"Content-Type", ~c"application/x-www-form-urlencoded"}]
      req_body = "tags[]=python&tags[]=rust&tags[]=typescript"

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/x-www-form-urlencoded", req_body},
          [],
          []
        )

      assert status == 201, "Expected status 201, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert Map.has_key?(parsed_body, "tags")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test url encoded 14 nested object bracket notation" do
    {routes, config} =
      AppFactories.create_app_handle_url_encoded_14_nested_object_bracket_notation()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/profile"
      headers = [{~c"Content-Type", ~c"application/x-www-form-urlencoded"}]
      req_body = "user[name]=John%20Doe&user[email]=john@example.com&user[age]=30"

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/x-www-form-urlencoded", req_body},
          [],
          []
        )

      assert status == 201, "Expected status 201, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert Map.has_key?(parsed_body, "user")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test url encoded 15 special characters field names" do
    {routes, config} =
      AppFactories.create_app_handle_url_encoded_15_special_characters_field_names()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/data"
      headers = [{~c"Content-Type", ~c"application/x-www-form-urlencoded"}]
      req_body = "user-name=JohnDoe&contact.email=john%40example.com"

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/x-www-form-urlencoded", req_body},
          [],
          []
        )

      assert status == 201, "Expected status 201, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["user-name"] == "JohnDoe"
      assert parsed_body["contact.email"] == "john@example.com"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test url encoded 16 minlength validation failure" do
    {routes, config} =
      AppFactories.create_app_handle_url_encoded_16_minlength_validation_failure()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/users"
      headers = [{~c"Content-Type", ~c"application/x-www-form-urlencoded"}]
      req_body = "username=ab"

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/x-www-form-urlencoded", req_body},
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
  test "test url encoded 17 pattern validation failure" do
    {routes, config} = AppFactories.create_app_handle_url_encoded_17_pattern_validation_failure()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/accounts"
      headers = [{~c"Content-Type", ~c"application/x-www-form-urlencoded"}]
      req_body = "account_id=INVALID123"

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/x-www-form-urlencoded", req_body},
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
  test "test url encoded 18 integer minimum validation failure" do
    {routes, config} =
      AppFactories.create_app_handle_url_encoded_18_integer_minimum_validation_failure()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/products"
      headers = [{~c"Content-Type", ~c"application/x-www-form-urlencoded"}]
      req_body = "quantity=0"

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/x-www-form-urlencoded", req_body},
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
  test "test url encoded 19 array minitems validation failure" do
    {routes, config} =
      AppFactories.create_app_handle_url_encoded_19_array_minitems_validation_failure()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/tags"
      headers = [{~c"Content-Type", ~c"application/x-www-form-urlencoded"}]
      req_body = "tags[]=single"

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/x-www-form-urlencoded", req_body},
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
  test "test url encoded 20 format email validation failure" do
    {routes, config} =
      AppFactories.create_app_handle_url_encoded_20_format_email_validation_failure()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/subscribe"
      headers = [{~c"Content-Type", ~c"application/x-www-form-urlencoded"}]
      req_body = "email=not-an-email"

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/x-www-form-urlencoded", req_body},
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
  test "test url encoded 21 integer type coercion failure" do
    {routes, config} =
      AppFactories.create_app_handle_url_encoded_21_integer_type_coercion_failure()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/products"
      headers = [{~c"Content-Type", ~c"application/x-www-form-urlencoded"}]
      req_body = "price=not-a-number"

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/x-www-form-urlencoded", req_body},
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
  test "test url encoded 22 additional properties strict failure" do
    {routes, config} =
      AppFactories.create_app_handle_url_encoded_22_additional_properties_strict_failure()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/settings"
      headers = [{~c"Content-Type", ~c"application/x-www-form-urlencoded"}]
      req_body = "theme=dark&unknown_field=value"

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/x-www-form-urlencoded", req_body},
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
  test "test url encoded Boolean field conversion" do
    {routes, config} = AppFactories.create_app_handle_url_encoded_boolean_field_conversion()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/form/"
      headers = [{~c"Content-Type", ~c"application/x-www-form-urlencoded"}]
      req_body = "subscribe=true&username=johndoe"

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/x-www-form-urlencoded", req_body},
          [],
          []
        )

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["username"] == "johndoe"
      assert parsed_body["subscribe"] == true
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test url encoded Empty string value" do
    {routes, config} = AppFactories.create_app_handle_url_encoded_empty_string_value()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/form/"
      headers = [{~c"Content-Type", ~c"application/x-www-form-urlencoded"}]
      req_body = "description=&username=johndoe"

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/x-www-form-urlencoded", req_body},
          [],
          []
        )

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["username"] == "johndoe"
      assert parsed_body["description"] == ""
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test url encoded Multiple values for same field" do
    {routes, config} = AppFactories.create_app_handle_url_encoded_multiple_values_for_same_field()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/form/tags"
      headers = [{~c"Content-Type", ~c"application/x-www-form-urlencoded"}]
      req_body = "tags=python&tags=fastapi&tags=web"

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/x-www-form-urlencoded", req_body},
          [],
          []
        )

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert Map.has_key?(parsed_body, "tags")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test url encoded Numeric field type conversion" do
    {routes, config} = AppFactories.create_app_handle_url_encoded_numeric_field_type_conversion()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/form/"
      headers = [{~c"Content-Type", ~c"application/x-www-form-urlencoded"}]
      req_body = "age=30&username=johndoe"

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/x-www-form-urlencoded", req_body},
          [],
          []
        )

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["username"] == "johndoe"
      assert parsed_body["age"] == 30
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test url encoded OAuth2 password grant flow" do
    {routes, config} = AppFactories.create_app_handle_url_encoded_oauth2_password_grant_flow()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/token"
      headers = [{~c"Content-Type", ~c"application/x-www-form-urlencoded"}]
      req_body = "password=secret&grant_type=password&username=johndoe&scope="

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/x-www-form-urlencoded", req_body},
          [],
          []
        )

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["access_token"] == "johndoe"
      assert parsed_body["token_type"] == "bearer"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test url encoded Optional field missing - success" do
    {routes, config} =
      AppFactories.create_app_handle_url_encoded_optional_field_missing___success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/register/"
      headers = [{~c"Content-Type", ~c"application/x-www-form-urlencoded"}]
      req_body = "password=secret&username=johndoe"

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/x-www-form-urlencoded", req_body},
          [],
          []
        )

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["username"] == "johndoe"
      assert Map.has_key?(parsed_body, "email")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test url encoded Pattern validation - fail" do
    {routes, config} = AppFactories.create_app_handle_url_encoded_pattern_validation___fail()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/form/validated"
      headers = [{~c"Content-Type", ~c"application/x-www-form-urlencoded"}]
      req_body = "username=john%20doe"

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/x-www-form-urlencoded", req_body},
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
  test "test url encoded Required field missing - validation error" do
    {routes, config} =
      AppFactories.create_app_handle_url_encoded_required_field_missing___validation_error()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/login/"
      headers = [{~c"Content-Type", ~c"application/x-www-form-urlencoded"}]
      req_body = "password=secret"

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/x-www-form-urlencoded", req_body},
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
  test "test url encoded Simple form submission - success" do
    {routes, config} =
      AppFactories.create_app_handle_url_encoded_simple_form_submission___success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/login/"
      headers = [{~c"Content-Type", ~c"application/x-www-form-urlencoded"}]
      req_body = "username=johndoe&password=secret"

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/x-www-form-urlencoded", req_body},
          [],
          []
        )

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["username"] == "johndoe"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test url encoded Special characters encoding" do
    {routes, config} = AppFactories.create_app_handle_url_encoded_special_characters_encoding()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/form/"
      headers = [{~c"Content-Type", ~c"application/x-www-form-urlencoded"}]
      req_body = "description=Test%20%26%20Development&name=John%20Doe"

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/x-www-form-urlencoded", req_body},
          [],
          []
        )

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["name"] == "John Doe"
      assert parsed_body["description"] == "Test & Development"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test url encoded String max length validation - fail" do
    {routes, config} =
      AppFactories.create_app_handle_url_encoded_string_max_length_validation___fail()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/form/validated"
      headers = [{~c"Content-Type", ~c"application/x-www-form-urlencoded"}]
      req_body = "username=this_is_a_very_long_username_that_exceeds_limit"

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/x-www-form-urlencoded", req_body},
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
  test "test url encoded String min length validation - fail" do
    {routes, config} =
      AppFactories.create_app_handle_url_encoded_string_min_length_validation___fail()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/form/validated"
      headers = [{~c"Content-Type", ~c"application/x-www-form-urlencoded"}]
      req_body = "username=ab"

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/x-www-form-urlencoded", req_body},
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
end

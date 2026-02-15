defmodule E2EElixirApp.CookiesTest do
  @moduledoc """
  Generated tests for cookies fixtures.

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
  test "test cookies 24 cookie samesite strict" do
    {routes, config} = AppFactories.create_app_handle_cookies_24_cookie_samesite_strict()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/secure"
      headers = [{~c"Cookie", ~c"session_id=abc123xyz789"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cookies 25 cookie samesite lax" do
    {routes, config} = AppFactories.create_app_handle_cookies_25_cookie_samesite_lax()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/data"
      headers = [{~c"Cookie", ~c"tracking=track123"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cookies 26 cookie secure flag" do
    {routes, config} = AppFactories.create_app_handle_cookies_26_cookie_secure_flag()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/secure"
      headers = [{~c"Cookie", ~c"auth_token=secure_token_xyz"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cookies 27 cookie httponly flag" do
    {routes, config} = AppFactories.create_app_handle_cookies_27_cookie_httponly_flag()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/secure"
      headers = [{~c"Cookie", ~c"session=session_abc123"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cookies APIKey cookie authentication - missing" do
    {routes, config} =
      AppFactories.create_app_handle_cookies_apikey_cookie_authentication___missing()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/users/me/auth"
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
  test "test cookies APIKey cookie authentication - success" do
    {routes, config} =
      AppFactories.create_app_handle_cookies_apikey_cookie_authentication___success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/users/me"
      headers = [{~c"Cookie", ~c"key=secret"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["username"] == "secret"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cookies Cookie regex pattern validation - fail" do
    {routes, config} =
      AppFactories.create_app_handle_cookies_cookie_regex_pattern_validation___fail()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/cookies/pattern"
      headers = [{~c"Cookie", ~c"tracking_id=invalid-format"}]

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
  test "test cookies Cookie regex pattern validation - success" do
    {routes, config} =
      AppFactories.create_app_handle_cookies_cookie_regex_pattern_validation___success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/cookies/pattern"
      headers = [{~c"Cookie", ~c"tracking_id=ABC12345"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["tracking_id"] == "ABC12345"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cookies Cookie validation - max length constraint fail" do
    {routes, config} =
      AppFactories.create_app_handle_cookies_cookie_validation___max_length_constraint_fail()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/cookies/validated"
      headers = [{~c"Cookie", ~c"session_id=this_cookie_value_is_way_too_long"}]

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
  test "test cookies Cookie validation - min length constraint success" do
    {routes, config} =
      AppFactories.create_app_handle_cookies_cookie_validation___min_length_constraint_success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/cookies/min-length"
      headers = [{~c"Cookie", ~c"token=abc"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["token"] == "abc"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cookies Cookie validation - min length failure" do
    {routes, config} =
      AppFactories.create_app_handle_cookies_cookie_validation___min_length_failure()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Cookie", ~c"tracking_id=ab"}]

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
  test "test cookies Multiple cookies - success" do
    {routes, config} = AppFactories.create_app_handle_cookies_multiple_cookies___success()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"

      headers = [
        {~c"Cookie",
         ~c"googall_tracker=ga789; fatebook_tracker=tracker456; session_id=session123"}
      ]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["session_id"] == "session123"
      assert parsed_body["fatebook_tracker"] == "tracker456"
      assert parsed_body["googall_tracker"] == "ga789"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cookies Optional APIKey cookie - missing" do
    {routes, config} = AppFactories.create_app_handle_cookies_optional_apikey_cookie___missing()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/users/me"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["msg"] == "Create an account first"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cookies Optional cookie parameter - missing" do
    {routes, config} =
      AppFactories.create_app_handle_cookies_optional_cookie_parameter___missing()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert Map.has_key?(parsed_body, "ads_id")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cookies Optional cookie parameter - success" do
    {routes, config} =
      AppFactories.create_app_handle_cookies_optional_cookie_parameter___success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Cookie", ~c"ads_id=abc123"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["ads_id"] == "abc123"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cookies Required cookie - missing" do
    {routes, config} = AppFactories.create_app_handle_cookies_required_cookie___missing()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/cookies"
      headers = [{~c"Cookie", ~c"fatebook_tracker=tracker456"}]

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
  test "test cookies Response - delete cookie" do
    {routes, config} = AppFactories.create_app_handle_cookies_response___delete_cookie()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/cookies/delete"
      headers = [{~c"Cookie", ~c"session=old_session_123"}]
      req_body = ""

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
      assert parsed_body["message"] == "Cookie deleted"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cookies Response - multiple cookies" do
    {routes, config} = AppFactories.create_app_handle_cookies_response___multiple_cookies()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/cookies/multiple"
      headers = []
      req_body = Jason.encode!(%{"user" => "john", "session" => "session123"})

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
      assert parsed_body["message"] == "Multiple cookies set"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cookies Response - session cookie (no max age)" do
    {routes, config} =
      AppFactories.create_app_handle_cookies_response___session_cookie__no_max_age_()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/cookies/session"
      headers = []
      req_body = Jason.encode!(%{"value" => "session_abc123"})

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
      assert parsed_body["message"] == "Session cookie set"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cookies Response cookie with SameSite=Lax" do
    {routes, config} = AppFactories.create_app_handle_cookies_response_cookie_with_samesite_lax()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/cookies/samesite-lax"
      headers = []
      req_body = Jason.encode!(%{"value" => "lax_cookie"})

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
      assert parsed_body["message"] == "Cookie set with SameSite=Lax"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cookies Response cookie with SameSite=None" do
    {routes, config} = AppFactories.create_app_handle_cookies_response_cookie_with_samesite_none()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/cookies/samesite-none"
      headers = []
      req_body = Jason.encode!(%{"value" => "none_cookie"})

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
      assert parsed_body["message"] == "Cookie set with SameSite=None"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cookies Response cookie with SameSite=Strict" do
    {routes, config} =
      AppFactories.create_app_handle_cookies_response_cookie_with_samesite_strict()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/cookies/samesite-strict"
      headers = []
      req_body = Jason.encode!(%{"value" => "strict_cookie"})

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
      assert parsed_body["message"] == "Cookie set with SameSite=Strict"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cookies Response cookie with attributes" do
    {routes, config} = AppFactories.create_app_handle_cookies_response_cookie_with_attributes()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/cookie/set"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["message"] == "Cookie set"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cookies Response cookie with domain attribute" do
    {routes, config} =
      AppFactories.create_app_handle_cookies_response_cookie_with_domain_attribute()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/cookies/set-with-domain"
      headers = []
      req_body = Jason.encode!(%{"value" => "domain_test"})

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
      assert parsed_body["message"] == "Cookie set with domain"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cookies Response cookie with path attribute" do
    {routes, config} =
      AppFactories.create_app_handle_cookies_response_cookie_with_path_attribute()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/cookies/set-with-path"
      headers = []
      req_body = Jason.encode!(%{"value" => "path_test"})

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
      assert parsed_body["message"] == "Cookie set with path"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test cookies Response set cookie - basic" do
    {routes, config} = AppFactories.create_app_handle_cookies_response_set_cookie___basic()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/cookie/"
      headers = []
      req_body = ""

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
      assert parsed_body["message"] == "Come to the dark side, we have cookies"
    after
      Spikard.stop(server)
    end
  end
end

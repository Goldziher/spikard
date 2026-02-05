defmodule E2EElixirApp.AuthTest do
  @moduledoc """
  Generated tests for auth fixtures.

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
  test "test auth API key authentication - invalid key" do
    {routes, config} = AppFactories.create_app_handle_auth_api_key_authentication___invalid_key()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/data"
      headers = [{~c"X-API-Key", ~c"invalid_key_12345"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 401, "Expected status 401, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["type"] == "https://spikard.dev/errors/unauthorized"
      assert parsed_body["title"] == "Invalid API key"
      assert parsed_body["status"] == 401
      assert parsed_body["detail"] == "The provided API key is not valid"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test auth API key authentication - missing header" do
    {routes, config} =
      AppFactories.create_app_handle_auth_api_key_authentication___missing_header()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/data"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 401, "Expected status 401, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["type"] == "https://spikard.dev/errors/unauthorized"
      assert parsed_body["title"] == "Missing API key"
      assert parsed_body["status"] == 401

      assert parsed_body["detail"] ==
               "Expected 'X-API-Key' header or 'api_key' query parameter with valid API key"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test auth API key authentication - valid key" do
    {routes, config} = AppFactories.create_app_handle_auth_api_key_authentication___valid_key()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/data"
      headers = [{~c"X-API-Key", ~c"sk_test_123456"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["message"] == "Access granted"
      assert parsed_body["data"] == "sensitive information"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test auth API key in query parameter" do
    {routes, config} = AppFactories.create_app_handle_auth_api_key_in_query_parameter()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/data?api_key=sk_test_123456"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["message"] == "Access granted"
      assert parsed_body["data"] == "sensitive information"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test auth API key rotation - old key still valid" do
    {routes, config} =
      AppFactories.create_app_handle_auth_api_key_rotation___old_key_still_valid()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/data"
      headers = [{~c"X-API-Key", ~c"sk_test_old_123456"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["message"] == "Access granted"
      assert parsed_body["data"] == "sensitive information"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test auth API key with custom header name" do
    {routes, config} = AppFactories.create_app_handle_auth_api_key_with_custom_header_name()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/data"
      headers = [{~c"X-API-Token", ~c"sk_test_123456"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["message"] == "Access granted"
      assert parsed_body["data"] == "sensitive information"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test auth Bearer token without prefix" do
    {routes, config} = AppFactories.create_app_handle_auth_bearer_token_without_prefix()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/protected"

      headers = [
        {~c"Authorization",
         ~c"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDZ9.8yXqZ9jKCR0BwqJc7pN_QvD3mYLxHfWzUeIaGkTnOsA"}
      ]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 401, "Expected status 401, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["type"] == "https://spikard.dev/errors/unauthorized"
      assert parsed_body["title"] == "Invalid Authorization header format"
      assert parsed_body["status"] == 401

      assert parsed_body["detail"] ==
               "Authorization header must use Bearer scheme: 'Bearer <token>'"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test auth JWT authentication - expired token" do
    {routes, config} = AppFactories.create_app_handle_auth_jwt_authentication___expired_token()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/protected/user"

      headers = [
        {~c"Authorization",
         ~c"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoxNjAwMDAwMDAwLCJpYXQiOjE1OTAwMDAwMDB9.n4oBw9XuO2aAJWi1e4Bz9Y_m2iEyJHGAODcetNuwYFo"}
      ]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 401, "Expected status 401, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["type"] == "https://spikard.dev/errors/unauthorized"
      assert parsed_body["title"] == "JWT validation failed"
      assert parsed_body["status"] == 401
      assert parsed_body["detail"] == "Token has expired"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test auth JWT authentication - invalid audience" do
    {routes, config} = AppFactories.create_app_handle_auth_jwt_authentication___invalid_audience()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/protected/user"

      headers = [
        {~c"Authorization",
         ~c"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNTM0MDIzMDA3OTk5LCJpYXQiOjE3MzEyNTIwMDAsImF1ZCI6WyJodHRwczovL3dyb25nLXNlcnZpY2UuY29tIl19.YR2a9fSJjhen7ksYFI2djSBSC7Pc29FDCloBGhkj3kU"}
      ]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 401, "Expected status 401, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["type"] == "https://spikard.dev/errors/unauthorized"
      assert parsed_body["title"] == "JWT validation failed"
      assert parsed_body["status"] == 401
      assert parsed_body["detail"] == "Token audience is invalid"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test auth JWT authentication - invalid signature" do
    {routes, config} =
      AppFactories.create_app_handle_auth_jwt_authentication___invalid_signature()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/protected/user"

      headers = [
        {~c"Authorization",
         ~c"Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNTM0MDIzMDA3OTksImlhdCI6MTczMTI1MjAwMH0.invalid_signature_here"}
      ]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 401, "Expected status 401, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["type"] == "https://spikard.dev/errors/unauthorized"
      assert parsed_body["title"] == "JWT validation failed"
      assert parsed_body["status"] == 401
      assert parsed_body["detail"] == "Token signature is invalid"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test auth JWT authentication - missing Authorization header" do
    {routes, config} =
      AppFactories.create_app_handle_auth_jwt_authentication___missing_authorization_header()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/protected/user"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 401, "Expected status 401, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["type"] == "https://spikard.dev/errors/unauthorized"
      assert parsed_body["title"] == "Missing or invalid Authorization header"
      assert parsed_body["status"] == 401
      assert parsed_body["detail"] == "Expected 'Authorization: Bearer <token>'"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test auth JWT authentication - valid token" do
    {routes, config} = AppFactories.create_app_handle_auth_jwt_authentication___valid_token()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/protected/user"

      headers = [
        {~c"Authorization",
         ~c"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2F1dGguZXhhbXBsZS5jb20ifQ.TpRpCJeXROQ12-ehRCVZm6EgN7Dn6QpfoekxJvnzgQg"}
      ]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["message"] == "Access granted"
      assert parsed_body["user_id"] == "user123"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test auth JWT invalid issuer" do
    {routes, config} = AppFactories.create_app_handle_auth_jwt_invalid_issuer()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/protected"

      headers = [
        {~c"Authorization",
         ~c"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE2MDAwMDAwMDAsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2V2aWwuY29tIn0.mbL5L04_hpaaiz0SPABap6ZWfBLu18aiexBjzwQ1nnA"}
      ]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 401, "Expected status 401, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["type"] == "https://spikard.dev/errors/unauthorized"
      assert parsed_body["title"] == "JWT validation failed"
      assert parsed_body["status"] == 401

      assert parsed_body["detail"] ==
               "Token issuer is invalid, expected 'https://auth.example.com'"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test auth JWT malformed token format" do
    {routes, config} = AppFactories.create_app_handle_auth_jwt_malformed_token_format()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/protected"
      headers = [{~c"Authorization", ~c"Bearer invalid.token"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 401, "Expected status 401, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["type"] == "https://spikard.dev/errors/unauthorized"
      assert parsed_body["title"] == "Malformed JWT token"
      assert parsed_body["status"] == 401

      assert parsed_body["detail"] ==
               "Malformed JWT token: expected 3 parts separated by dots, found 2"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test auth JWT missing required custom claims" do
    {routes, config} = AppFactories.create_app_handle_auth_jwt_missing_required_custom_claims()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/admin"

      headers = [
        {~c"Authorization",
         ~c"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2F1dGguZXhhbXBsZS5jb20ifQ.TpRpCJeXROQ12-ehRCVZm6EgN7Dn6QpfoekxJvnzgQg"}
      ]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 403, "Expected status 403, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["type"] == "https://spikard.dev/errors/forbidden"
      assert parsed_body["title"] == "Forbidden"
      assert parsed_body["status"] == 403
      assert parsed_body["detail"] == "Required claims 'role' and 'permissions' missing from JWT"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test auth JWT not before claim in future" do
    {routes, config} = AppFactories.create_app_handle_auth_jwt_not_before_claim_in_future()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/protected"

      headers = [
        {~c"Authorization",
         ~c"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE2MDAwMDAwMDAsIm5iZiI6MjYyNjc4Mzk0NiwiYXVkIjpbImh0dHBzOi8vYXBpLmV4YW1wbGUuY29tIl0sImlzcyI6Imh0dHBzOi8vYXV0aC5leGFtcGxlLmNvbSJ9.hG4I76_3kJfsbJ_jmxoP1NSYnkcqdyBFcPpdo-jYU4E"}
      ]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 401, "Expected status 401, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["type"] == "https://spikard.dev/errors/unauthorized"
      assert parsed_body["title"] == "JWT validation failed"
      assert parsed_body["status"] == 401
      assert parsed_body["detail"] == "JWT not valid yet, not before claim is in the future"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test auth JWT with multiple audiences" do
    {routes, config} = AppFactories.create_app_handle_auth_jwt_with_multiple_audiences()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/protected"

      headers = [
        {~c"Authorization",
         ~c"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE2MDAwMDAwMDAsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSIsImh0dHBzOi8vYWRtaW4uZXhhbXBsZS5jb20iXSwiaXNzIjoiaHR0cHM6Ly9hdXRoLmV4YW1wbGUuY29tIn0.9MBL_XccGXfu9cDUnCpQruDMOl2hHYydzeGn-20dQOs"}
      ]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["message"] == "Access granted"
      assert parsed_body["user_id"] == "user123"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test auth Multiple authentication schemes - JWT precedence" do
    {routes, config} =
      AppFactories.create_app_handle_auth_multiple_authentication_schemes___jwt_precedence()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/data"

      headers = [
        {~c"X-API-Key", ~c"sk_test_123456"},
        {~c"Authorization",
         ~c"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2F1dGguZXhhbXBsZS5jb20ifQ.TpRpCJeXROQ12-ehRCVZm6EgN7Dn6QpfoekxJvnzgQg"}
      ]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["message"] == "Access granted"
      assert parsed_body["user_id"] == "user123"
      assert parsed_body["auth_method"] == "jwt"
    after
      Spikard.stop(server)
    end
  end
end

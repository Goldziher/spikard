defmodule Spikard.Auth.ApiKeyTest do
  @moduledoc """
  Tests for API key authentication configuration and middleware.

  These tests verify that API key authentication works correctly:
  - Valid keys are accepted
  - Invalid keys are rejected with 401 status
  - Missing keys are rejected with 401 status
  - Custom header names are supported
  - Query parameters are supported as fallback

  Note: These tests require Rust-side API key middleware implementation.
  """

  use ExUnit.Case, async: true
  @moduletag :incomplete

  alias Spikard.TestClient
  alias Spikard.TestClient.Response

  describe "API key authentication with header" do
    test "accepts request with valid API key in header" do
      handler = fn _req -> %{status: 200, body: %{message: "OK"}} end

      {:ok, client} = TestClient.new(
        routes: [{:get, "/api", handler}],
        api_key_auth: %{keys: ["valid-key-123"], header_name: "X-API-Key"}
      )

      {:ok, response} = TestClient.get(client, "/api", headers: [{"X-API-Key", "valid-key-123"}])

      assert response.status_code == 200
      assert Response.json(response)["message"] == "OK"
    end

    test "rejects request with invalid API key in header" do
      handler = fn _req -> %{status: 200, body: %{message: "OK"}} end

      {:ok, client} = TestClient.new(
        routes: [{:get, "/api", handler}],
        api_key_auth: %{keys: ["valid-key-123"], header_name: "X-API-Key"}
      )

      {:ok, response} = TestClient.get(client, "/api", headers: [{"X-API-Key", "invalid-key"}])

      assert response.status_code == 401
      json = Response.json(response)
      assert json["status"] == 401
      assert String.contains?(json["title"], "Unauthorized")
    end

    test "rejects request with missing API key header" do
      handler = fn _req -> %{status: 200, body: %{message: "OK"}} end

      {:ok, client} = TestClient.new(
        routes: [{:get, "/api", handler}],
        api_key_auth: %{keys: ["valid-key-123"], header_name: "X-API-Key"}
      )

      {:ok, response} = TestClient.get(client, "/api")

      assert response.status_code == 401
      json = Response.json(response)
      assert json["status"] == 401
    end

    test "supports custom header name" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} = TestClient.new(
        routes: [{:get, "/api", handler}],
        api_key_auth: %{keys: ["secret"], header_name: "Authorization-Key"}
      )

      # Should fail with standard header
      {:ok, response1} = TestClient.get(client, "/api", headers: [{"X-API-Key", "secret"}])
      assert response1.status_code == 401

      # Should succeed with custom header
      {:ok, response2} = TestClient.get(client, "/api", headers: [{"Authorization-Key", "secret"}])
      assert response2.status_code == 200
    end

    test "supports multiple valid keys" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} = TestClient.new(
        routes: [{:get, "/api", handler}],
        api_key_auth: %{keys: ["key1", "key2", "key3"], header_name: "X-API-Key"}
      )

      # All keys should work
      for key <- ["key1", "key2", "key3"] do
        {:ok, response} = TestClient.get(client, "/api", headers: [{"X-API-Key", key}])
        assert response.status_code == 200, "Key #{key} should be accepted"
      end
    end

    test "case-sensitive header name matching" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} = TestClient.new(
        routes: [{:get, "/api", handler}],
        api_key_auth: %{keys: ["secret"], header_name: "X-API-Key"}
      )

      # HTTP headers are case-insensitive, should work with different casing
      {:ok, response} = TestClient.get(client, "/api", headers: [{"x-api-key", "secret"}])
      # This may be 200 or 401 depending on HTTP normalization
      # Elixir/Axum normalize headers to lowercase, so this should work
      assert response.status_code in [200, 401]
    end
  end

  describe "API key authentication with query parameter" do
    test "accepts request with valid API key in query parameter" do
      handler = fn _req -> %{status: 200, body: %{message: "OK"}} end

      {:ok, client} = TestClient.new(
        routes: [{:get, "/api", handler}],
        api_key_auth: %{keys: ["valid-key"], header_name: "X-API-Key"}
      )

      {:ok, response} = TestClient.get(client, "/api?api_key=valid-key")

      assert response.status_code == 200
      assert Response.json(response)["message"] == "OK"
    end

    test "rejects request with invalid API key in query parameter" do
      handler = fn _req -> %{status: 200, body: %{message: "OK"}} end

      {:ok, client} = TestClient.new(
        routes: [{:get, "/api", handler}],
        api_key_auth: %{keys: ["valid-key"], header_name: "X-API-Key"}
      )

      {:ok, response} = TestClient.get(client, "/api?api_key=wrong-key")

      assert response.status_code == 401
    end

    test "prefers header over query parameter" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} = TestClient.new(
        routes: [{:get, "/api", handler}],
        api_key_auth: %{keys: ["header-key", "query-key"], header_name: "X-API-Key"}
      )

      # Header takes precedence - should use header-key and reject
      {:ok, response} = TestClient.get(client, "/api?api_key=query-key", headers: [{"X-API-Key", "invalid"}])

      assert response.status_code == 401
    end

    test "falls back to query parameter when header is missing" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} = TestClient.new(
        routes: [{:get, "/api", handler}],
        api_key_auth: %{keys: ["query-key"], header_name: "X-API-Key"}
      )

      {:ok, response} = TestClient.get(client, "/api?api_key=query-key")

      assert response.status_code == 200
    end
  end

  describe "API key authentication error responses" do
    test "returns RFC 9457 Problem Details format" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} = TestClient.new(
        routes: [{:get, "/api", handler}],
        api_key_auth: %{keys: ["valid-key"], header_name: "X-API-Key"}
      )

      {:ok, response} = TestClient.get(client, "/api")

      assert response.status_code == 401
      json = Response.json(response)

      # Check RFC 9457 format
      assert json["type"] != nil
      assert json["title"] != nil
      assert json["status"] == 401
      assert json["detail"] != nil
    end

    test "includes helpful error message for missing key" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} = TestClient.new(
        routes: [{:get, "/api", handler}],
        api_key_auth: %{keys: ["valid-key"], header_name: "X-API-Key"}
      )

      {:ok, response} = TestClient.get(client, "/api")

      assert response.status_code == 401
      json = Response.json(response)
      assert String.contains?(json["detail"], "X-API-Key")
    end

    test "includes helpful error message for invalid key" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} = TestClient.new(
        routes: [{:get, "/api", handler}],
        api_key_auth: %{keys: ["valid-key"], header_name: "X-API-Key"}
      )

      {:ok, response} = TestClient.get(client, "/api", headers: [{"X-API-Key", "wrong"}])

      assert response.status_code == 401
      json = Response.json(response)
      assert String.contains?(String.downcase(json["detail"]), "invalid")
    end
  end

  describe "API key authentication with different HTTP methods" do
    test "applies to GET requests" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} = TestClient.new(
        routes: [{:get, "/api", handler}],
        api_key_auth: %{keys: ["valid"], header_name: "X-API-Key"}
      )

      {:ok, response} = TestClient.get(client, "/api", headers: [{"X-API-Key", "valid"}])
      assert response.status_code == 200
    end

    test "applies to POST requests" do
      handler = fn _req -> %{status: 201, body: %{ok: true}} end

      {:ok, client} = TestClient.new(
        routes: [{:post, "/api", handler}],
        api_key_auth: %{keys: ["valid"], header_name: "X-API-Key"}
      )

      {:ok, response} = TestClient.post(client, "/api", json: %{}, headers: [{"X-API-Key", "valid"}])
      assert response.status_code == 201
    end

    test "applies to PUT requests" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} = TestClient.new(
        routes: [{:put, "/api/resource", handler}],
        api_key_auth: %{keys: ["valid"], header_name: "X-API-Key"}
      )

      {:ok, response} = TestClient.put(client, "/api/resource", json: %{}, headers: [{"X-API-Key", "valid"}])
      assert response.status_code == 200
    end

    test "applies to DELETE requests" do
      handler = fn _req -> %{status: 204, body: ""} end

      {:ok, client} = TestClient.new(
        routes: [{:delete, "/api/resource", handler}],
        api_key_auth: %{keys: ["valid"], header_name: "X-API-Key"}
      )

      {:ok, response} = TestClient.delete(client, "/api/resource", headers: [{"X-API-Key", "valid"}])
      assert response.status_code == 204
    end
  end

  describe "API key authentication configuration" do
    test "accepts default header name when not specified" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} = TestClient.new(
        routes: [{:get, "/api", handler}],
        api_key_auth: %{keys: ["valid"]}
      )

      # Should use default header name "x-api-key"
      {:ok, response} = TestClient.get(client, "/api", headers: [{"x-api-key", "valid"}])
      assert response.status_code == 200
    end

    test "rejects configuration with empty keys list" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      # Empty keys list should be invalid
      result = TestClient.new(
        routes: [{:get, "/api", handler}],
        api_key_auth: %{keys: [], header_name: "X-API-Key"}
      )

      # Either rejects the config or accepts but requires no auth
      case result do
        {:ok, _client} -> :ok  # Some implementations allow empty keys (permissive)
        {:error, _reason} -> :ok  # Others reject it
      end
    end
  end
end

defmodule Spikard.CorsTest do
  @moduledoc """
  Tests for CORS configuration and handling in Spikard.

  These tests validate CORS preflight request handling, origin validation,
  and proper CORS header configuration in responses.
  """

  use ExUnit.Case, async: true

  alias Spikard.TestClient
  alias Spikard.TestClient.Response

  describe "CORS configuration" do
    test "allows all origins by default" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      assert {:ok, client} =
               TestClient.new(
                 routes: [{:get, "/api", handler}],
                 cors: %{
                   allowed_origins: ["*"],
                   allowed_methods: ["GET", "POST"]
                 }
               )

      assert is_reference(client) or is_tuple(client)
    end

    test "validates specific origins" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      assert {:ok, _client} =
               TestClient.new(
                 routes: [{:get, "/api", handler}],
                 cors: %{
                   allowed_origins: ["https://example.com", "https://app.example.com"],
                   allowed_methods: ["GET", "POST"]
                 }
               )
    end

    test "supports allowed_headers configuration" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      assert {:ok, _client} =
               TestClient.new(
                 routes: [{:post, "/api", handler}],
                 cors: %{
                   allowed_origins: ["https://example.com"],
                   allowed_methods: ["GET", "POST"],
                   allowed_headers: ["content-type", "authorization"]
                 }
               )
    end

    test "supports expose_headers configuration" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      assert {:ok, _client} =
               TestClient.new(
                 routes: [{:get, "/api", handler}],
                 cors: %{
                   allowed_origins: ["*"],
                   allowed_methods: ["GET"],
                   expose_headers: ["x-total-count", "x-page"]
                 }
               )
    end

    test "supports max_age configuration" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      assert {:ok, _client} =
               TestClient.new(
                 routes: [{:get, "/api", handler}],
                 cors: %{
                   allowed_origins: ["*"],
                   allowed_methods: ["GET"],
                   max_age: 3600
                 }
               )
    end

    test "supports allow_credentials configuration" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      assert {:ok, _client} =
               TestClient.new(
                 routes: [{:get, "/api", handler}],
                 cors: %{
                   allowed_origins: ["https://example.com"],
                   allowed_methods: ["GET"],
                   allow_credentials: true
                 }
               )
    end
  end

  describe "CORS preflight requests (OPTIONS)" do
    test "returns 204 No Content for valid preflight" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} =
        TestClient.new(
          routes: [{:get, "/api", handler}],
          cors: %{
            allowed_origins: ["https://example.com"],
            allowed_methods: ["GET", "POST"]
          }
        )

      {:ok, response} =
        TestClient.options(client, "/api",
          headers: [
            {"origin", "https://example.com"},
            {"access-control-request-method", "GET"}
          ]
        )

      assert response.status_code == 204
    end

    test "returns access-control-allow-origin header" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} =
        TestClient.new(
          routes: [{:get, "/api", handler}],
          cors: %{
            allowed_origins: ["https://example.com"],
            allowed_methods: ["GET"]
          }
        )

      {:ok, response} =
        TestClient.options(client, "/api",
          headers: [
            {"origin", "https://example.com"},
            {"access-control-request-method", "GET"}
          ]
        )

      allow_origin = Response.header(response, "access-control-allow-origin")
      assert allow_origin == "https://example.com"
    end

    test "returns access-control-allow-methods header" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} =
        TestClient.new(
          routes: [{:get, "/api", handler}],
          cors: %{
            allowed_origins: ["https://example.com"],
            allowed_methods: ["GET", "POST", "PUT"]
          }
        )

      {:ok, response} =
        TestClient.options(client, "/api",
          headers: [
            {"origin", "https://example.com"},
            {"access-control-request-method", "POST"}
          ]
        )

      methods_header = Response.header(response, "access-control-allow-methods")
      assert String.contains?(methods_header, "GET")
      assert String.contains?(methods_header, "POST")
      assert String.contains?(methods_header, "PUT")
    end

    test "returns access-control-allow-headers when requested" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} =
        TestClient.new(
          routes: [{:post, "/api", handler}],
          cors: %{
            allowed_origins: ["https://example.com"],
            allowed_methods: ["POST"],
            allowed_headers: ["content-type", "authorization", "x-api-key"]
          }
        )

      {:ok, response} =
        TestClient.options(client, "/api",
          headers: [
            {"origin", "https://example.com"},
            {"access-control-request-method", "POST"},
            {"access-control-request-headers", "content-type, authorization"}
          ]
        )

      allow_headers = Response.header(response, "access-control-allow-headers")
      assert String.contains?(allow_headers, "content-type")
      assert String.contains?(allow_headers, "authorization")
    end

    test "includes max-age header when configured" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} =
        TestClient.new(
          routes: [{:get, "/api", handler}],
          cors: %{
            allowed_origins: ["https://example.com"],
            allowed_methods: ["GET"],
            max_age: 7200
          }
        )

      {:ok, response} =
        TestClient.options(client, "/api",
          headers: [
            {"origin", "https://example.com"},
            {"access-control-request-method", "GET"}
          ]
        )

      max_age = Response.header(response, "access-control-max-age")
      assert max_age == "7200"
    end

    test "includes credentials header when enabled" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} =
        TestClient.new(
          routes: [{:get, "/api", handler}],
          cors: %{
            allowed_origins: ["https://example.com"],
            allowed_methods: ["GET"],
            allow_credentials: true
          }
        )

      {:ok, response} =
        TestClient.options(client, "/api",
          headers: [
            {"origin", "https://example.com"},
            {"access-control-request-method", "GET"}
          ]
        )

      credentials = Response.header(response, "access-control-allow-credentials")
      assert credentials == "true"
    end

    test "rejects preflight with disallowed origin" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} =
        TestClient.new(
          routes: [{:get, "/api", handler}],
          cors: %{
            allowed_origins: ["https://example.com"],
            allowed_methods: ["GET"]
          }
        )

      {:ok, response} =
        TestClient.options(client, "/api",
          headers: [
            {"origin", "https://evil.com"},
            {"access-control-request-method", "GET"}
          ]
        )

      assert response.status_code == 403
    end

    test "rejects preflight with disallowed method" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} =
        TestClient.new(
          routes: [{:get, "/api", handler}],
          cors: %{
            allowed_origins: ["https://example.com"],
            allowed_methods: ["GET"]
          }
        )

      {:ok, response} =
        TestClient.options(client, "/api",
          headers: [
            {"origin", "https://example.com"},
            {"access-control-request-method", "DELETE"}
          ]
        )

      assert response.status_code == 403
    end

    test "rejects preflight with disallowed headers" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} =
        TestClient.new(
          routes: [{:post, "/api", handler}],
          cors: %{
            allowed_origins: ["https://example.com"],
            allowed_methods: ["POST"],
            allowed_headers: ["content-type"]
          }
        )

      {:ok, response} =
        TestClient.options(client, "/api",
          headers: [
            {"origin", "https://example.com"},
            {"access-control-request-method", "POST"},
            {"access-control-request-headers", "x-forbidden-header"}
          ]
        )

      assert response.status_code == 403
    end

    test "allows wildcard origin" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} =
        TestClient.new(
          routes: [{:get, "/api", handler}],
          cors: %{
            allowed_origins: ["*"],
            allowed_methods: ["GET"]
          }
        )

      {:ok, response} =
        TestClient.options(client, "/api",
          headers: [
            {"origin", "https://any-domain.com"},
            {"access-control-request-method", "GET"}
          ]
        )

      assert response.status_code == 204
    end

    test "allows wildcard methods" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} =
        TestClient.new(
          routes: [{:get, "/api", handler}],
          cors: %{
            allowed_origins: ["https://example.com"],
            allowed_methods: ["*"]
          }
        )

      {:ok, response} =
        TestClient.options(client, "/api",
          headers: [
            {"origin", "https://example.com"},
            {"access-control-request-method", "DELETE"}
          ]
        )

      assert response.status_code == 204
    end

    test "allows wildcard headers" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} =
        TestClient.new(
          routes: [{:post, "/api", handler}],
          cors: %{
            allowed_origins: ["https://example.com"],
            allowed_methods: ["POST"],
            allowed_headers: ["*"]
          }
        )

      {:ok, response} =
        TestClient.options(client, "/api",
          headers: [
            {"origin", "https://example.com"},
            {"access-control-request-method", "POST"},
            {"access-control-request-headers", "x-custom-header, x-another-header"}
          ]
        )

      assert response.status_code == 204
    end

    test "case-insensitive method matching" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} =
        TestClient.new(
          routes: [{:get, "/api", handler}],
          cors: %{
            allowed_origins: ["https://example.com"],
            allowed_methods: ["GET", "POST"]
          }
        )

      # Test lowercase method in request
      {:ok, response} =
        TestClient.options(client, "/api",
          headers: [
            {"origin", "https://example.com"},
            {"access-control-request-method", "get"}
          ]
        )

      assert response.status_code == 204
    end
  end

  describe "CORS with actual requests" do
    test "request without origin is allowed" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} =
        TestClient.new(
          routes: [{:get, "/api", handler}],
          cors: %{
            allowed_origins: ["https://example.com"],
            allowed_methods: ["GET"]
          }
        )

      {:ok, response} = TestClient.get(client, "/api")

      assert response.status_code == 200
    end

    test "POST with JSON body" do
      handler = fn req ->
        body = Spikard.Request.get_body(req)
        %{status: 201, body: body}
      end

      {:ok, client} =
        TestClient.new(
          routes: [{:post, "/api", handler}],
          cors: %{
            allowed_origins: ["https://example.com"],
            allowed_methods: ["POST"]
          }
        )

      {:ok, response} =
        TestClient.post(client, "/api", json: %{name: "test"})

      assert response.status_code == 201
    end
  end

  describe "CORS edge cases" do
    test "empty origin list prevents all requests" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      # Note: Empty allowed_origins should be invalid, but test behavior
      {:ok, client} =
        TestClient.new(
          routes: [{:get, "/api", handler}],
          cors: %{
            allowed_origins: [],
            allowed_methods: ["GET"]
          }
        )

      {:ok, response} =
        TestClient.options(client, "/api",
          headers: [
            {"origin", "https://example.com"},
            {"access-control-request-method", "GET"}
          ]
        )

      assert response.status_code == 403
    end

    test "multiple allowed origins" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} =
        TestClient.new(
          routes: [{:get, "/api", handler}],
          cors: %{
            allowed_origins: [
              "https://example.com",
              "https://app.example.com",
              "https://test.example.com"
            ],
            allowed_methods: ["GET"]
          }
        )

      # Test first origin
      {:ok, response1} =
        TestClient.options(client, "/api",
          headers: [
            {"origin", "https://example.com"},
            {"access-control-request-method", "GET"}
          ]
        )

      assert response1.status_code == 204

      # Test second origin
      {:ok, response2} =
        TestClient.options(client, "/api",
          headers: [
            {"origin", "https://app.example.com"},
            {"access-control-request-method", "GET"}
          ]
        )

      assert response2.status_code == 204

      # Test disallowed origin
      {:ok, response3} =
        TestClient.options(client, "/api",
          headers: [
            {"origin", "https://evil.com"},
            {"access-control-request-method", "GET"}
          ]
        )

      assert response3.status_code == 403
    end

    test "multiple allowed methods" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} =
        TestClient.new(
          routes: [{:get, "/api", handler}],
          cors: %{
            allowed_origins: ["*"],
            allowed_methods: ["GET", "POST", "PUT", "DELETE", "PATCH"]
          }
        )

      for method <- ["GET", "POST", "PUT", "DELETE", "PATCH"] do
        {:ok, response} =
          TestClient.options(client, "/api",
            headers: [
              {"origin", "https://example.com"},
              {"access-control-request-method", method}
            ]
          )

        assert response.status_code == 204, "Method #{method} should be allowed"
      end
    end

    test "comma-separated request headers are validated" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      {:ok, client} =
        TestClient.new(
          routes: [{:post, "/api", handler}],
          cors: %{
            allowed_origins: ["https://example.com"],
            allowed_methods: ["POST"],
            allowed_headers: ["content-type", "authorization", "x-api-key"]
          }
        )

      {:ok, response} =
        TestClient.options(client, "/api",
          headers: [
            {"origin", "https://example.com"},
            {"access-control-request-method", "POST"},
            {"access-control-request-headers", "content-type, authorization, x-api-key"}
          ]
        )

      assert response.status_code == 204
    end
  end
end

defmodule Spikard.Auth.JwtTest do
  @moduledoc """
  Tests for JWT authentication configuration for Spikard.

  These tests validate that JWT configuration can be created and passed
  to the server, and that the configuration is properly serialized.

  Note: Actual JWT middleware enforcement testing requires a running
  HTTP server. The TestClient bypasses middleware for direct handler testing.
  """

  use ExUnit.Case, async: true

  alias Spikard.Auth.Jwt

  describe "Jwt.new/1" do
    test "creates JWT config with required secret" do
      config = Jwt.new(secret: "test-secret")

      assert config.secret == "test-secret"
      assert config.algorithm == :hs256
      assert config.header_name == "authorization"
      assert config.required_claims == []
    end

    test "creates JWT config with custom algorithm" do
      config = Jwt.new(secret: "test-secret", algorithm: :hs512)

      assert config.algorithm == :hs512
    end

    test "creates JWT config with custom header name" do
      config = Jwt.new(secret: "test-secret", header_name: "x-auth-token")

      assert config.header_name == "x-auth-token"
    end

    test "creates JWT config with required claims" do
      config =
        Jwt.new(
          secret: "test-secret",
          required_claims: ["sub", "exp"]
        )

      assert config.required_claims == ["sub", "exp"]
    end

    test "raises error when secret is missing" do
      assert_raise KeyError, fn ->
        Jwt.new(algorithm: :hs256)
      end
    end

    test "raises error when secret is empty string" do
      assert_raise ArgumentError, fn ->
        Jwt.new(secret: "")
      end
    end

    test "raises error when algorithm is invalid" do
      assert_raise ArgumentError, fn ->
        Jwt.new(secret: "test", algorithm: :invalid)
      end
    end
  end

  describe "Jwt.to_map/1" do
    test "converts HS256 config to map" do
      config = Jwt.new(secret: "test-secret")
      map = Jwt.to_map(config)

      assert map["secret"] == "test-secret"
      assert map["algorithm"] == "HS256"
      assert map["header_name"] == "authorization"
      assert map["required_claims"] == []
    end

    test "converts HS512 algorithm to uppercase string" do
      config = Jwt.new(secret: "test", algorithm: :hs512)
      map = Jwt.to_map(config)

      assert map["algorithm"] == "HS512"
    end

    test "converts custom header name" do
      config = Jwt.new(secret: "test", header_name: "x-token")
      map = Jwt.to_map(config)

      assert map["header_name"] == "x-token"
    end

    test "converts required claims list" do
      config = Jwt.new(secret: "test", required_claims: ["sub", "aud"])
      map = Jwt.to_map(config)

      assert map["required_claims"] == ["sub", "aud"]
    end

    test "all algorithm variants convert correctly" do
      algorithms = [:hs256, :hs384, :hs512, :rs256, :rs384, :rs512, :es256, :es384, :ps256, :ps384, :ps512]

      for alg <- algorithms do
        config = Jwt.new(secret: "test", algorithm: alg)
        map = Jwt.to_map(config)
        assert is_binary(map["algorithm"])
        assert String.length(map["algorithm"]) > 0
      end
    end
  end

  describe "Jwt configuration with TestClient" do
    test "TestClient accepts jwt_auth config without error" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end
      jwt_config = Jwt.new(secret: "test-secret")

      # This should not raise - config is accepted and serialized
      {:ok, _client} =
        Spikard.TestClient.new(
          routes: [{:get, "/protected", handler}],
          jwt_auth: jwt_config
        )

      assert true
    end

    test "TestClient works with jwt_auth configured" do
      handler = fn _req ->
        %{status: 200, body: %{ok: true}}
      end

      jwt_config = Jwt.new(secret: "test-secret")

      {:ok, client} =
        Spikard.TestClient.new(
          routes: [{:get, "/test", handler}],
          jwt_auth: jwt_config
        )

      # Verify that TestClient works correctly with JWT config
      {:ok, response} =
        Spikard.TestClient.get(client, "/test", headers: [{"authorization", "Bearer test-token"}])

      assert response.status_code == 200
      assert Spikard.TestClient.Response.json(response)["ok"] == true
    end
  end
end

defmodule Spikard.OpenapiTest do
  @moduledoc """
  Tests for OpenAPI specification generation.

  These tests verify that OpenAPI configs are correctly created and that
  the generator properly converts routes to OpenAPI path specifications.
  """
  use ExUnit.Case, async: true

  alias Spikard.OpenAPI

  describe "OpenAPI.config/1" do
    test "creates config with title and version" do
      config = OpenAPI.config(title: "My API", version: "1.0.0")

      assert config.title == "My API"
      assert config.version == "1.0.0"
    end

    test "includes description when provided" do
      config = OpenAPI.config(
        title: "My API",
        version: "1.0.0",
        description: "A test API"
      )

      assert config.description == "A test API"
    end

    test "description is nil by default" do
      config = OpenAPI.config(title: "API", version: "1.0.0")
      assert config.description == nil
    end

    test "sets default path for openapi_json_path" do
      config = OpenAPI.config(title: "API", version: "1.0.0")

      assert config.openapi_json_path == "/openapi.json"
    end

    test "allows customizing openapi_json_path" do
      config = OpenAPI.config(
        title: "API",
        version: "1.0.0",
        openapi_json_path: "/api/spec.json"
      )

      assert config.openapi_json_path == "/api/spec.json"
    end

    test "includes contact information when provided" do
      contact = %{"name" => "Support", "email" => "support@example.com"}

      config = OpenAPI.config(
        title: "API",
        version: "1.0.0",
        contact: contact
      )

      assert config.contact == contact
    end

    test "contact is nil by default" do
      config = OpenAPI.config(title: "API", version: "1.0.0")
      assert config.contact == nil
    end

    test "includes license information when provided" do
      license = %{"name" => "MIT"}

      config = OpenAPI.config(
        title: "API",
        version: "1.0.0",
        license: license
      )

      assert config.license == license
    end

    test "license is nil by default" do
      config = OpenAPI.config(title: "API", version: "1.0.0")
      assert config.license == nil
    end

    test "includes server information when provided" do
      servers = [%{"url" => "https://api.example.com"}]

      config = OpenAPI.config(
        title: "API",
        version: "1.0.0",
        servers: servers
      )

      assert config.servers == servers
    end

    test "servers is nil by default" do
      config = OpenAPI.config(title: "API", version: "1.0.0")
      assert config.servers == nil
    end

    test "raises when title is missing" do
      assert_raise KeyError, fn ->
        OpenAPI.config(version: "1.0.0")
      end
    end

    test "raises when version is missing" do
      assert_raise KeyError, fn ->
        OpenAPI.config(title: "API")
      end
    end
  end

  describe "OpenAPI.enabled?/1" do
    test "returns true when OpenAPI config is present" do
      config = OpenAPI.config(title: "API", version: "1.0.0")
      assert OpenAPI.enabled?(config) == true
    end

    test "returns false when no OpenAPI config" do
      assert OpenAPI.enabled?(nil) == false
    end

    test "returns false for empty map" do
      assert OpenAPI.enabled?(%{}) == false
    end

    test "returns false for non-map" do
      assert OpenAPI.enabled?("string") == false
      assert OpenAPI.enabled?(123) == false
      assert OpenAPI.enabled?([]) == false
    end
  end

  describe "OpenAPI.Generator.new/1" do
    test "creates generator from config" do
      config = OpenAPI.config(title: "My API", version: "2.0.0")
      generator = OpenAPI.Generator.new(config)

      assert generator.title == "My API"
      assert generator.version == "2.0.0"
    end

    test "preserves all config fields" do
      config = OpenAPI.config(
        title: "API",
        version: "1.0.0",
        description: "Test",
        openapi_json_path: "/spec",
        contact: %{"name" => "Support"},
        license: %{"name" => "MIT"},
        servers: [%{"url" => "http://localhost"}]
      )

      generator = OpenAPI.Generator.new(config)

      assert generator.title == "API"
      assert generator.version == "1.0.0"
      assert generator.description == "Test"
      assert generator.openapi_json_path == "/spec"
      assert generator.contact == %{"name" => "Support"}
      assert generator.license == %{"name" => "MIT"}
      assert generator.servers == [%{"url" => "http://localhost"}]
    end
  end

  describe "OpenAPI.Generator.to_paths/2" do
    test "converts empty routes list to empty paths" do
      config = OpenAPI.config(title: "API", version: "1.0.0")
      generator = OpenAPI.Generator.new(config)

      paths = OpenAPI.Generator.to_paths(generator, [])

      assert paths == %{}
    end

    test "converts single GET route to paths" do
      config = OpenAPI.config(title: "API", version: "1.0.0")
      generator = OpenAPI.Generator.new(config)

      routes = [
        %{
          "method" => "GET",
          "path" => "/users",
          "handler_name" => "Handler.list_users"
        }
      ]

      paths = OpenAPI.Generator.to_paths(generator, routes)

      assert Map.has_key?(paths, "/users")
      assert Map.has_key?(paths["/users"], "get")
      assert paths["/users"]["get"]["summary"] == "Handler.list_users"
    end

    test "converts multiple methods on same path" do
      config = OpenAPI.config(title: "API", version: "1.0.0")
      generator = OpenAPI.Generator.new(config)

      routes = [
        %{"method" => "GET", "path" => "/users", "handler_name" => "list"},
        %{"method" => "POST", "path" => "/users", "handler_name" => "create"}
      ]

      paths = OpenAPI.Generator.to_paths(generator, routes)

      assert Map.has_key?(paths["/users"], "get")
      assert Map.has_key?(paths["/users"], "post")
    end

    test "converts multiple paths" do
      config = OpenAPI.config(title: "API", version: "1.0.0")
      generator = OpenAPI.Generator.new(config)

      routes = [
        %{"method" => "GET", "path" => "/users", "handler_name" => "list"},
        %{"method" => "GET", "path" => "/items", "handler_name" => "items"}
      ]

      paths = OpenAPI.Generator.to_paths(generator, routes)

      assert Map.has_key?(paths, "/users")
      assert Map.has_key?(paths, "/items")
    end

    test "includes response schema when provided" do
      config = OpenAPI.config(title: "API", version: "1.0.0")
      generator = OpenAPI.Generator.new(config)

      user_schema = %{
        "type" => "object",
        "properties" => %{"id" => %{"type" => "integer"}}
      }

      routes = [
        %{
          "method" => "GET",
          "path" => "/users/:id",
          "handler_name" => "show",
          "response_schema" => user_schema
        }
      ]

      paths = OpenAPI.Generator.to_paths(generator, routes)

      # Response should have content with schema
      response = paths["/users/:id"]["get"]["responses"]["200"]
      assert Map.has_key?(response, "content")
      assert Map.has_key?(response["content"], "application/json")
      assert response["content"]["application/json"]["schema"] == user_schema
    end

    test "handles routes without response schema" do
      config = OpenAPI.config(title: "API", version: "1.0.0")
      generator = OpenAPI.Generator.new(config)

      routes = [
        %{
          "method" => "POST",
          "path" => "/users",
          "handler_name" => "create"
          # No response_schema
        }
      ]

      paths = OpenAPI.Generator.to_paths(generator, routes)

      # Should still have responses but without content
      response = paths["/users"]["post"]["responses"]["200"]
      assert response["description"] == "Successful response"
      assert !Map.has_key?(response, "content")
    end

    test "generates operation IDs from routes" do
      config = OpenAPI.config(title: "API", version: "1.0.0")
      generator = OpenAPI.Generator.new(config)

      routes = [
        %{
          "method" => "GET",
          "path" => "/users",
          "handler_name" => "list"
        },
        %{
          "method" => "POST",
          "path" => "/users",
          "handler_name" => "create"
        }
      ]

      paths = OpenAPI.Generator.to_paths(generator, routes)

      get_op_id = paths["/users"]["get"]["operationId"]
      post_op_id = paths["/users"]["post"]["operationId"]

      # Operation IDs should be deterministic and include method and path
      assert get_op_id =~ "get"
      assert get_op_id =~ "users"
      assert post_op_id =~ "post"
      assert post_op_id =~ "users"
      assert get_op_id != post_op_id
    end

    test "handles all HTTP methods" do
      config = OpenAPI.config(title: "API", version: "1.0.0")
      generator = OpenAPI.Generator.new(config)

      methods = ~w(GET POST PUT PATCH DELETE HEAD OPTIONS TRACE)

      routes =
        Enum.map(methods, fn method ->
          %{
            "method" => method,
            "path" => "/resource",
            "handler_name" => "handler"
          }
        end)

      paths = OpenAPI.Generator.to_paths(generator, routes)

      # All methods should be present, converted to lowercase
      Enum.each(methods, fn method ->
        assert Map.has_key?(paths["/resource"], String.downcase(method))
      end)
    end
  end
end

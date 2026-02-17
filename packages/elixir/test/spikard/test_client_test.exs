defmodule Spikard.TestClientTest do
  @moduledoc """
  Tests for the Spikard.TestClient module.

  These tests validate the TestClient's ability to make HTTP requests
  without actual network overhead, enabling fast integration testing.
  """

  use ExUnit.Case, async: true

  alias Spikard.TestClient
  alias Spikard.TestClient.Response

  describe "TestClient.new/1" do
    test "creates a test client with routes" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end

      assert {:ok, client} = TestClient.new(routes: [{:get, "/", handler}])
      assert is_reference(client) or is_tuple(client)
    end

    test "returns error for missing routes" do
      assert {:error, _reason} = TestClient.new([])
    end
  end

  describe "TestClient.get/3" do
    test "returns response with status code" do
      handler = fn _req -> %{status: 200, body: %{message: "Hello"}} end
      {:ok, client} = TestClient.new(routes: [{:get, "/", handler}])

      {:ok, response} = TestClient.get(client, "/")

      assert response.status_code == 200
    end

    test "returns response body as JSON" do
      handler = fn _req -> %{status: 200, body: %{name: "Alice", age: 30}} end
      {:ok, client} = TestClient.new(routes: [{:get, "/users", handler}])

      {:ok, response} = TestClient.get(client, "/users")

      assert response.status_code == 200
      json = Response.json(response)
      assert json["name"] == "Alice"
      assert json["age"] == 30
    end

    test "returns response body as text" do
      handler = fn _req -> %{status: 200, body: "Hello, World!", headers: %{"content-type" => "text/plain"}} end
      {:ok, client} = TestClient.new(routes: [{:get, "/hello", handler}])

      {:ok, response} = TestClient.get(client, "/hello")

      assert Response.text(response) == "Hello, World!"
    end

    test "handles path parameters" do
      handler = fn req ->
        id = Spikard.Request.get_path_param(req, "id")
        %{status: 200, body: %{id: id}}
      end

      {:ok, client} = TestClient.new(routes: [{:get, "/users/:id", handler}])

      {:ok, response} = TestClient.get(client, "/users/123")

      assert response.status_code == 200
      assert Response.json(response)["id"] == "123"
    end

    test "handles query parameters" do
      handler = fn req ->
        page = Spikard.Request.get_query_param(req, "page", "1")
        %{status: 200, body: %{page: page}}
      end

      {:ok, client} = TestClient.new(routes: [{:get, "/items", handler}])

      {:ok, response} = TestClient.get(client, "/items", query: [{"page", "2"}])

      # Query params may be auto-coerced to integers during JSON serialization
      page = Response.json(response)["page"]
      assert to_string(page) == "2"
    end

    test "handles custom headers" do
      handler = fn req ->
        auth = Spikard.Request.get_header(req, "authorization")
        %{status: 200, body: %{auth: auth}}
      end

      {:ok, client} = TestClient.new(routes: [{:get, "/protected", handler}])

      {:ok, response} = TestClient.get(client, "/protected", headers: [{"authorization", "Bearer token123"}])

      assert Response.json(response)["auth"] == "Bearer token123"
    end

    test "returns 404 for undefined routes" do
      handler = fn _req -> %{status: 200, body: %{ok: true}} end
      {:ok, client} = TestClient.new(routes: [{:get, "/", handler}])

      {:ok, response} = TestClient.get(client, "/undefined")

      assert response.status_code == 404
    end
  end

  describe "TestClient.post/3" do
    test "sends JSON body" do
      handler = fn req ->
        body = Spikard.Request.get_body(req)
        %{status: 201, body: body}
      end

      {:ok, client} = TestClient.new(routes: [{:post, "/users", handler}])

      {:ok, response} = TestClient.post(client, "/users", json: %{name: "Alice"})

      assert response.status_code == 201
      assert Response.json(response)["name"] == "Alice"
    end

    test "sends form data" do
      handler = fn req ->
        # Form data is sent as raw_body (URL-encoded string)
        # In a real server, the body parser middleware would parse this
        # For testing, we verify the raw_body contains the form data
        raw_body = Spikard.Request.get_raw_body(req)
        raw_str = if is_list(raw_body), do: List.to_string(raw_body), else: to_string(raw_body || "")

        # Parse the URL-encoded form data
        parsed = URI.decode_query(raw_str)
        %{status: 200, body: parsed}
      end

      {:ok, client} = TestClient.new(routes: [{:post, "/form", handler}])

      {:ok, response} = TestClient.post(client, "/form", form: [{"username", "alice"}, {"password", "secret"}])

      assert response.status_code == 200
      json = Response.json(response)
      assert json["username"] == "alice"
    end

    test "supports query params with POST" do
      handler = fn req ->
        action = Spikard.Request.get_query_param(req, "action")
        body = Spikard.Request.get_body(req)
        %{status: 200, body: Map.put(body, "action", action)}
      end

      {:ok, client} = TestClient.new(routes: [{:post, "/items", handler}])

      {:ok, response} = TestClient.post(client, "/items", json: %{name: "Item"}, query: [{"action", "create"}])

      json = Response.json(response)
      assert json["name"] == "Item"
      assert json["action"] == "create"
    end
  end

  describe "TestClient.put/3" do
    test "updates resource with JSON body" do
      handler = fn req ->
        id = Spikard.Request.get_path_param(req, "id")
        body = Spikard.Request.get_body(req)
        %{status: 200, body: Map.put(body, "id", id)}
      end

      {:ok, client} = TestClient.new(routes: [{:put, "/users/:id", handler}])

      {:ok, response} = TestClient.put(client, "/users/42", json: %{name: "Updated"})

      json = Response.json(response)
      assert json["id"] == "42"
      assert json["name"] == "Updated"
    end
  end

  describe "TestClient.patch/3" do
    test "partially updates resource" do
      handler = fn req ->
        body = Spikard.Request.get_body(req)
        %{status: 200, body: body}
      end

      {:ok, client} = TestClient.new(routes: [{:patch, "/users/:id", handler}])

      {:ok, response} = TestClient.patch(client, "/users/1", json: %{email: "new@example.com"})

      assert Response.json(response)["email"] == "new@example.com"
    end
  end

  describe "TestClient.delete/3" do
    test "deletes resource" do
      handler = fn _req -> %{status: 204} end
      {:ok, client} = TestClient.new(routes: [{:delete, "/users/:id", handler}])

      {:ok, response} = TestClient.delete(client, "/users/42")

      assert response.status_code == 204
    end
  end

  describe "TestClient.options/3" do
    test "returns allowed methods" do
      handler = fn _req ->
        %{
          status: 200,
          headers: %{"allow" => "GET, POST, OPTIONS"},
          body: nil
        }
      end

      {:ok, client} = TestClient.new(routes: [{:options, "/api", handler}])

      {:ok, response} = TestClient.options(client, "/api")

      assert response.status_code == 200
      assert Response.header(response, "allow") =~ "GET"
    end
  end

  describe "TestClient.head/3" do
    test "returns headers without body" do
      handler = fn _req ->
        %{
          status: 200,
          headers: %{"x-custom" => "value"},
          body: nil
        }
      end

      {:ok, client} = TestClient.new(routes: [{:head, "/health", handler}])

      {:ok, response} = TestClient.head(client, "/health")

      assert response.status_code == 200
      assert Response.header(response, "x-custom") == "value"
    end
  end

  describe "GraphQL helpers" do
    test "graphql/3 posts GraphQL payload" do
      handler = fn req ->
        body = Spikard.Request.get_body(req)
        %{status: 200, body: body}
      end

      {:ok, client} = TestClient.new(routes: [{:post, "/graphql", handler}])

      {:ok, response} =
        TestClient.graphql(client, "query Hello($id: ID!) { hello(id: $id) }",
          variables: %{"id" => "42"},
          operation_name: "Hello"
        )

      assert response.status_code == 200
      json = Response.json(response)
      assert json["query"] == "query Hello($id: ID!) { hello(id: $id) }"
      assert json["variables"]["id"] == "42"
      assert json["operationName"] == "Hello"
    end

    test "graphql_with_status/3 returns status and response" do
      handler = fn _req -> %{status: 201, body: %{"ok" => true}} end
      {:ok, client} = TestClient.new(routes: [{:post, "/graphql", handler}])

      assert {:ok, {status, response}} = TestClient.graphql_with_status(client, "query { ok }")
      assert status == 201
      assert response.status_code == 201
    end

    test "graphql_subscription/3 returns error when endpoint is not a websocket route" do
      handler = fn _req -> %{status: 200, body: %{"data" => %{"hello" => "world"}}} end
      {:ok, client} = TestClient.new(routes: [{:post, "/graphql", handler}])

      assert {:error, {_reason, message}} =
               TestClient.graphql_subscription(client, "subscription { ticker }")

      assert is_binary(message)
    end
  end

  describe "Response helpers" do
    test "Response.json/1 parses JSON body" do
      handler = fn _req -> %{status: 200, body: %{data: [1, 2, 3]}} end
      {:ok, client} = TestClient.new(routes: [{:get, "/", handler}])

      {:ok, response} = TestClient.get(client, "/")

      json = Response.json(response)
      assert json["data"] == [1, 2, 3]
    end

    test "Response.text/1 returns raw text" do
      handler = fn _req -> %{status: 200, body: "plain text", headers: %{"content-type" => "text/plain"}} end
      {:ok, client} = TestClient.new(routes: [{:get, "/", handler}])

      {:ok, response} = TestClient.get(client, "/")

      assert Response.text(response) == "plain text"
    end

    test "Response.header/2 retrieves header value" do
      handler = fn _req ->
        %{status: 200, headers: %{"x-request-id" => "abc123"}, body: nil}
      end

      {:ok, client} = TestClient.new(routes: [{:get, "/", handler}])

      {:ok, response} = TestClient.get(client, "/")

      assert Response.header(response, "x-request-id") == "abc123"
    end

    test "Response.header/2 is case-insensitive" do
      handler = fn _req ->
        %{status: 200, headers: %{"Content-Type" => "application/json"}, body: nil}
      end

      {:ok, client} = TestClient.new(routes: [{:get, "/", handler}])

      {:ok, response} = TestClient.get(client, "/")

      assert Response.header(response, "content-type") == "application/json"
      assert Response.header(response, "CONTENT-TYPE") == "application/json"
    end
  end

  describe "error handling" do
    test "handler exceptions return 500" do
      handler = fn _req -> raise "Unexpected error" end
      {:ok, client} = TestClient.new(routes: [{:get, "/error", handler}])

      {:ok, response} = TestClient.get(client, "/error")

      assert response.status_code == 500
    end

    test "handler throws return 500" do
      handler = fn _req -> throw(:boom) end
      {:ok, client} = TestClient.new(routes: [{:get, "/throw", handler}])

      {:ok, response} = TestClient.get(client, "/throw")

      assert response.status_code == 500
    end
  end

  describe "cookies" do
    test "sends cookies in request" do
      handler = fn req ->
        session = Spikard.Request.get_cookie(req, "session")
        %{status: 200, body: %{session: session}}
      end

      {:ok, client} = TestClient.new(routes: [{:get, "/", handler}])

      {:ok, response} = TestClient.get(client, "/", cookies: [{"session", "abc123"}])

      assert Response.json(response)["session"] == "abc123"
    end
  end
end

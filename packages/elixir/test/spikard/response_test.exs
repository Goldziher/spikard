defmodule Spikard.ResponseTest do
  use ExUnit.Case
  doctest Spikard.Response

  alias Spikard.Response

  describe "new/0" do
    test "creates empty response with status 200" do
      response = Response.new()

      assert response.status == 200
      assert response.headers == []
      assert response.body == nil
    end
  end

  describe "json/2" do
    test "creates JSON response with default status" do
      response = Response.json(%{hello: "world"})

      assert response.status == 200
      assert {"content-type", "application/json"} in response.headers
      assert response.body == ~s({"hello":"world"})
    end

    test "creates JSON response with custom status" do
      response = Response.json(%{error: "not found"}, status: 404)

      assert response.status == 404
    end
  end

  describe "text/2" do
    test "creates text response" do
      response = Response.text("Hello, World!")

      assert response.status == 200
      assert {"content-type", "text/plain; charset=utf-8"} in response.headers
      assert response.body == "Hello, World!"
    end
  end

  describe "html/2" do
    test "creates HTML response with default status" do
      response = Response.html("<h1>Hello</h1>")

      assert response.status == 200
      assert {"content-type", "text/html; charset=utf-8"} in response.headers
      assert response.body == "<h1>Hello</h1>"
    end

    test "creates HTML response with custom status" do
      response = Response.html("<h1>Error</h1>", status: 500)

      assert response.status == 500
    end
  end

  describe "status/1" do
    test "creates response with status only" do
      response = Response.status(204)

      assert response.status == 204
      assert response.body == nil
    end
  end

  describe "with_status/2" do
    test "sets status on response" do
      response =
        Response.new()
        |> Response.with_status(201)

      assert response.status == 201
    end
  end

  describe "with_header/3" do
    test "adds single header to response" do
      response =
        Response.json(%{})
        |> Response.with_header("X-Request-Id", "abc123")

      assert {"x-request-id", "abc123"} in response.headers
    end

    test "normalizes header names to lowercase" do
      response =
        Response.new()
        |> Response.with_header("X-CUSTOM-HEADER", "value")

      assert {"x-custom-header", "value"} in response.headers
    end
  end

  describe "with_headers/2" do
    test "adds multiple headers from list" do
      response =
        Response.json(%{})
        |> Response.with_headers([{"X-Request-Id", "abc"}, {"X-Trace-Id", "xyz"}])

      assert {"x-request-id", "abc"} in response.headers
      assert {"x-trace-id", "xyz"} in response.headers
    end

    test "adds multiple headers from map" do
      response =
        Response.json(%{})
        |> Response.with_headers(%{"X-Request-Id" => "abc", "X-Trace-Id" => "xyz"})

      assert {"x-request-id", "abc"} in response.headers
      assert {"x-trace-id", "xyz"} in response.headers
    end
  end

  describe "with_json/2" do
    test "sets JSON body on response" do
      response =
        Response.new()
        |> Response.with_status(201)
        |> Response.with_json(%{created: true})

      assert response.status == 201
      assert {"content-type", "application/json"} in response.headers
      assert response.body == ~s({"created":true})
    end
  end

  describe "with_text/2" do
    test "sets text body on response" do
      response =
        Response.new()
        |> Response.with_text("Hello")

      assert {"content-type", "text/plain; charset=utf-8"} in response.headers
      assert response.body == "Hello"
    end
  end

  describe "with_html/2" do
    test "sets HTML body on response" do
      response =
        Response.new()
        |> Response.with_html("<p>Hello</p>")

      assert {"content-type", "text/html; charset=utf-8"} in response.headers
      assert response.body == "<p>Hello</p>"
    end
  end

  describe "with_cookie/4" do
    test "adds basic cookie header" do
      response =
        Response.json(%{})
        |> Response.with_cookie("session", "abc123")

      assert {"set-cookie", "session=abc123"} in response.headers
    end

    test "adds cookie with max_age" do
      response =
        Response.json(%{})
        |> Response.with_cookie("session", "abc123", max_age: 3600)

      cookie_header = find_header(response.headers, "set-cookie")
      assert cookie_header =~ "session=abc123"
      assert cookie_header =~ "Max-Age=3600"
    end

    test "adds cookie with http_only and secure flags" do
      response =
        Response.json(%{})
        |> Response.with_cookie("token", "xyz", http_only: true, secure: true)

      cookie_header = find_header(response.headers, "set-cookie")
      assert cookie_header =~ "token=xyz"
      assert cookie_header =~ "HttpOnly"
      assert cookie_header =~ "Secure"
    end

    test "adds cookie with same_site policy" do
      response =
        Response.json(%{})
        |> Response.with_cookie("prefs", "dark", same_site: "Strict")

      cookie_header = find_header(response.headers, "set-cookie")
      assert cookie_header =~ "SameSite=Strict"
    end

    test "adds cookie with path and domain" do
      response =
        Response.json(%{})
        |> Response.with_cookie("session", "abc", path: "/admin", domain: "example.com")

      cookie_header = find_header(response.headers, "set-cookie")
      assert cookie_header =~ "Path=/admin"
      assert cookie_header =~ "Domain=example.com"
    end
  end

  describe "redirect/2" do
    test "creates temporary redirect by default" do
      response = Response.redirect("/login")

      assert response.status == 302
      assert {"location", "/login"} in response.headers
      assert response.body == nil
    end

    test "creates permanent redirect with custom status" do
      response = Response.redirect("/new-page", status: 301)

      assert response.status == 301
      assert {"location", "/new-page"} in response.headers
    end
  end

  # Helper to find a header value by name
  defp find_header(headers, name) do
    case Enum.find(headers, fn {n, _v} -> n == name end) do
      {_, value} -> value
      nil -> nil
    end
  end
end

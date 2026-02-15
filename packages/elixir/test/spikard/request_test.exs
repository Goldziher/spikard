defmodule Spikard.RequestTest do
  use ExUnit.Case, async: true

  alias Spikard.Request

  describe "from_map/1" do
    test "creates request from valid map" do
      map = %{
        "method" => "GET",
        "path" => "/users/123",
        "headers" => %{"content-type" => "application/json"},
        "query_params" => %{"page" => "1"},
        "path_params" => %{"id" => "123"},
        "body" => %{"name" => "test"},
        "raw_body" => "{\"name\":\"test\"}"
      }

      request = Request.from_map(map)

      assert request.method == "GET"
      assert request.path == "/users/123"
      assert request.headers == %{"content-type" => "application/json"}
      assert request.query_params == %{"page" => "1"}
      assert request.path_params == %{"id" => "123"}
      assert request.body == %{"name" => "test"}
      assert request.raw_body == "{\"name\":\"test\"}"
    end

    test "handles missing fields with defaults" do
      request = Request.from_map(%{})

      # Method and path default to empty strings
      assert request.method == ""
      assert request.path == ""
      # Maps default to empty maps
      assert request.headers == %{}
      assert request.query_params == %{}
      assert request.path_params == %{}
      # Body fields default to nil
      assert request.body == nil
      assert request.raw_body == nil
    end

    test "converts nil field values to empty maps" do
      map = %{
        "headers" => nil,
        "query_params" => nil,
        "path_params" => nil
      }

      request = Request.from_map(map)

      assert request.headers == %{}
      assert request.query_params == %{}
      assert request.path_params == %{}
    end
  end

  describe "get_path_param/2" do
    test "returns param when present" do
      request = %Request{path_params: %{"id" => "123", "slug" => "test"}}
      assert Request.get_path_param(request, "id") == "123"
      assert Request.get_path_param(request, "slug") == "test"
    end

    test "returns nil when param missing" do
      request = %Request{path_params: %{"id" => "123"}}
      assert Request.get_path_param(request, "missing") == nil
    end

    test "works with empty path_params map" do
      request = %Request{path_params: %{}}
      assert Request.get_path_param(request, "id") == nil
    end
  end

  describe "get_query_param/2,3" do
    test "returns param when present" do
      request = %Request{query_params: %{"page" => "1", "limit" => "10"}}
      assert Request.get_query_param(request, "page") == "1"
    end

    test "returns nil when param missing (arity 2)" do
      request = %Request{query_params: %{"page" => "1"}}
      assert Request.get_query_param(request, "missing") == nil
    end

    test "returns default when param missing (arity 3)" do
      request = %Request{query_params: %{"page" => "1"}}
      assert Request.get_query_param(request, "missing", "default") == "default"
    end

    test "works with empty query_params map" do
      request = %Request{query_params: %{}}
      assert Request.get_query_param(request, "page") == nil
      assert Request.get_query_param(request, "page", "1") == "1"
    end
  end

  describe "get_header/2" do
    test "returns header when present (lowercase key in map)" do
      # Headers are stored lowercase in the map
      request = %Request{headers: %{"content-type" => "application/json"}}
      assert Request.get_header(request, "content-type") == "application/json"
    end

    test "performs case-insensitive lookup" do
      # Headers stored lowercase, lookup downcases the key
      request = %Request{headers: %{"content-type" => "application/json"}}
      assert Request.get_header(request, "Content-Type") == "application/json"
      assert Request.get_header(request, "CONTENT-TYPE") == "application/json"
    end

    test "returns nil when header missing" do
      request = %Request{headers: %{"content-type" => "application/json"}}
      assert Request.get_header(request, "Authorization") == nil
    end

    test "works with empty headers map" do
      request = %Request{headers: %{}}
      assert Request.get_header(request, "Content-Type") == nil
    end
  end

  describe "get_cookie/2" do
    test "returns cookie when present" do
      request = %Request{cookies: %{"session" => "abc123"}}
      assert Request.get_cookie(request, "session") == "abc123"
    end

    test "returns nil when cookie missing" do
      request = %Request{cookies: %{"session" => "abc123"}}
      assert Request.get_cookie(request, "token") == nil
    end

    test "works with empty cookies map" do
      request = %Request{cookies: %{}}
      assert Request.get_cookie(request, "session") == nil
    end
  end

  describe "get_body/1" do
    test "returns parsed body" do
      request = %Request{body: %{"name" => "test"}}
      assert Request.get_body(request) == %{"name" => "test"}
    end

    test "returns nil when body is nil" do
      request = %Request{body: nil}
      assert Request.get_body(request) == nil
    end
  end

  describe "get_raw_body/1" do
    test "returns raw body string" do
      request = %Request{raw_body: "{\"name\":\"test\"}"}
      assert Request.get_raw_body(request) == "{\"name\":\"test\"}"
    end

    test "returns nil when raw_body is nil" do
      request = %Request{raw_body: nil}
      assert Request.get_raw_body(request) == nil
    end
  end
end

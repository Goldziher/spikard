defmodule Spikard.ResponseTest do
  use ExUnit.Case
  doctest Spikard.Response

  alias Spikard.Response

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

  describe "status/1" do
    test "creates response with status only" do
      response = Response.status(204)

      assert response.status == 204
      assert response.body == nil
    end
  end
end

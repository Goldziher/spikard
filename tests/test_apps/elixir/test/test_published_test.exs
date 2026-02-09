defmodule SpikardTestApp.PublishedTest do
  use ExUnit.Case

  test "spikard module is available" do
    assert Code.ensure_loaded?(Spikard)
  end

  test "router compiles routes" do
    routes = SpikardTestApp.Router.routes()
    assert is_list(routes)
    assert length(routes) > 0
  end

  test "test client works with basic routes" do
    {:ok, client} = Spikard.TestClient.new(routes: SpikardTestApp.Router.routes())
    {:ok, response} = Spikard.TestClient.get(client, "/")

    assert response.status_code == 200
    json = Spikard.TestClient.Response.json(response)
    assert json["app"] == "spikard-test-app"
    assert json["language"] == "elixir"
  end

  test "health endpoint returns ok" do
    {:ok, client} = Spikard.TestClient.new(routes: SpikardTestApp.Router.routes())
    {:ok, response} = Spikard.TestClient.get(client, "/health")

    assert response.status_code == 200
    assert Spikard.TestClient.Response.json(response)["status"] == "ok"
  end

  test "echo endpoint returns body" do
    {:ok, client} = Spikard.TestClient.new(routes: SpikardTestApp.Router.routes())
    {:ok, response} = Spikard.TestClient.post(client, "/echo", json: %{hello: "world"})

    assert response.status_code == 200
    json = Spikard.TestClient.Response.json(response)
    assert json["echo"]["hello"] == "world"
  end
end

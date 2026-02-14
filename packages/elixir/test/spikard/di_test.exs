defmodule Spikard.DITest do
  @moduledoc """
  Tests for Spikard dependency injection system.

  Covers value dependencies (simple storage) and factory dependencies
  (resolved via Elixir callback functions).

  Note: The "dependency injection in handlers" tests require Rust-side
  DI implementation to pass dependencies from the DI container to handlers.
  """

  use ExUnit.Case
  doctest Spikard.DI

  alias Spikard.TestClient.Response

  describe "Spikard.DI.value/2" do
    test "creates a value dependency" do
      db = %{connected: true}
      dep = Spikard.DI.value("db", db)

      assert dep.type == :value
      assert dep.key == "db"
      assert dep.value == db
    end

    test "stores any type of value" do
      dep1 = Spikard.DI.value("string_key", "value")
      assert dep1.value == "value"

      dep2 = Spikard.DI.value("list_key", [1, 2, 3])
      assert dep2.value == [1, 2, 3]

      dep3 = Spikard.DI.value("map_key", %{"nested" => "map"})
      assert dep3.value == %{"nested" => "map"}
    end
  end

  describe "Spikard.DI.factory/2" do
    test "creates a factory dependency" do
      factory = fn -> %{id: 1} end
      dep = Spikard.DI.factory("ctx", factory)

      assert dep.type == :factory
      assert dep.key == "ctx"
      assert is_function(dep.factory)
    end

    test "factory must be a function with arity 0" do
      assert_raise FunctionClauseError, fn ->
        Spikard.DI.factory("key", "not a function")
      end

      assert_raise FunctionClauseError, fn ->
        # Function with arity 1 should fail
        factory_with_arg = fn _arg -> %{} end
        Spikard.DI.factory("key", factory_with_arg)
      end
    end

    test "factory with dependencies option" do
      factory = fn -> %{id: 1} end
      dep = Spikard.DI.factory("ctx", factory, depends_on: ["db", "logger"])

      assert dep.type == :factory
      assert dep.key == "ctx"
      assert dep.depends_on == ["db", "logger"]
    end

    test "factory with singleton option (default true)" do
      factory = fn -> %{id: 1} end
      dep1 = Spikard.DI.factory("singleton", factory)
      assert dep1.singleton == true

      dep2 = Spikard.DI.factory("transient", factory, singleton: false)
      assert dep2.singleton == false
    end
  end

  describe "dependency injection in handlers" do
    test "handler receives value dependency from request" do
      db = %{connected: true}

      handler = fn req ->
        db = Spikard.Request.get_dependency(req, "db")
        %{status: 200, body: %{db_status: db.connected}}
      end

      {:ok, client} =
        Spikard.TestClient.new(
          routes: [{:get, "/", handler}],
          dependencies: [Spikard.DI.value("db", db)]
        )

      {:ok, response} = Spikard.TestClient.get(client, "/")
      assert response.status_code == 200
      assert Response.json(response)["db_status"] == true
    end

    test "handler receives factory dependency" do
      factory = fn -> %{request_id: System.unique_integer()} end

      handler = fn req ->
        ctx = Spikard.Request.get_dependency(req, "ctx")
        %{status: 200, body: %{request_id: ctx.request_id}}
      end

      # Use singleton: false to get different values per request
      {:ok, client} =
        Spikard.TestClient.new(
          routes: [{:get, "/", handler}],
          dependencies: [Spikard.DI.factory("ctx", factory, singleton: false)]
        )

      {:ok, r1} = Spikard.TestClient.get(client, "/")
      {:ok, r2} = Spikard.TestClient.get(client, "/")

      assert Response.json(r1)["request_id"] != Response.json(r2)["request_id"]
    end

    test "factory dependency is called per request (not singleton)" do
      call_count = :counters.new(1, [:atomics])

      factory = fn ->
        :counters.add(call_count, 1, 1)
        %{id: :counters.get(call_count, 1)}
      end

      handler = fn req ->
        ctx = Spikard.Request.get_dependency(req, "ctx")
        %{status: 200, body: %{id: ctx.id}}
      end

      {:ok, client} =
        Spikard.TestClient.new(
          routes: [{:get, "/", handler}],
          dependencies: [Spikard.DI.factory("ctx", factory, singleton: false)]
        )

      {:ok, r1} = Spikard.TestClient.get(client, "/")
      {:ok, r2} = Spikard.TestClient.get(client, "/")

      # Each request should have incremented call count
      assert Response.json(r1)["id"] == 1
      assert Response.json(r2)["id"] == 2
    end

    test "multiple dependencies in single request" do
      db = %{name: "postgres"}
      cache = %{ttl: 3600}

      handler = fn req ->
        db = Spikard.Request.get_dependency(req, "db")
        cache = Spikard.Request.get_dependency(req, "cache")

        %{
          status: 200,
          body: %{
            db_name: db.name,
            cache_ttl: cache.ttl
          }
        }
      end

      {:ok, client} =
        Spikard.TestClient.new(
          routes: [{:get, "/", handler}],
          dependencies: [
            Spikard.DI.value("db", db),
            Spikard.DI.value("cache", cache)
          ]
        )

      {:ok, response} = Spikard.TestClient.get(client, "/")
      assert Response.json(response)["db_name"] == "postgres"
      assert Response.json(response)["cache_ttl"] == 3600
    end

    test "missing dependency returns nil" do
      handler = fn req ->
        missing = Spikard.Request.get_dependency(req, "missing_key")
        %{status: 200, body: %{missing: missing}}
      end

      {:ok, client} =
        Spikard.TestClient.new(
          routes: [{:get, "/", handler}],
          dependencies: [Spikard.DI.value("db", %{connected: true})]
        )

      {:ok, response} = Spikard.TestClient.get(client, "/")
      assert Response.json(response)["missing"] == nil
    end

    test "dependency with complex object" do
      logger = %{
        "log" => fn msg -> IO.write(msg) end,
        "level" => "info"
      }

      handler = fn req ->
        logger = Spikard.Request.get_dependency(req, "logger")
        %{status: 200, body: %{level: logger["level"]}}
      end

      {:ok, client} =
        Spikard.TestClient.new(
          routes: [{:get, "/", handler}],
          dependencies: [Spikard.DI.value("logger", logger)]
        )

      {:ok, response} = Spikard.TestClient.get(client, "/")
      assert Response.json(response)["level"] == "info"
    end
  end

  describe "DI configuration validation" do
    test "validates dependency configuration" do
      db = %{connected: true}
      deps = [Spikard.DI.value("db", db)]

      # Should not raise
      :ok = Spikard.DI.validate(deps)
    end

    test "validates empty dependencies" do
      :ok = Spikard.DI.validate([])
    end

    test "dependency keys must be strings" do
      # Should work with string keys
      assert Spikard.DI.value("db", %{}) |> Map.fetch!(:key) == "db"
    end

    test "counts dependencies correctly" do
      db = Spikard.DI.value("db", %{})
      cache = Spikard.DI.value("cache", %{})
      factory = Spikard.DI.factory("ctx", fn -> %{} end)

      deps = [db, cache, factory]
      counts = Spikard.DI.count(deps)

      assert counts.value == 2
      assert counts.factory == 1
      assert counts.total == 3
    end
  end

  describe "Spikard.Request.get_dependency/2" do
    test "retrieves a dependency by key" do
      deps = %{"db" => %{connected: true}}

      req = %Spikard.Request{
        path_params: %{},
        query_params: %{},
        raw_query_params: %{},
        headers: %{},
        cookies: %{},
        body: nil,
        raw_body: nil,
        method: "GET",
        path: "/",
        validated_params: nil,
        dependencies: deps
      }

      assert Spikard.Request.get_dependency(req, "db") == %{connected: true}
    end

    test "returns nil for missing dependency" do
      req = %Spikard.Request{
        path_params: %{},
        query_params: %{},
        raw_query_params: %{},
        headers: %{},
        cookies: %{},
        body: nil,
        raw_body: nil,
        method: "GET",
        path: "/",
        validated_params: nil,
        dependencies: %{}
      }

      assert Spikard.Request.get_dependency(req, "missing") == nil
    end

    test "returns nil when dependencies is nil" do
      req = %Spikard.Request{
        path_params: %{},
        query_params: %{},
        raw_query_params: %{},
        headers: %{},
        cookies: %{},
        body: nil,
        raw_body: nil,
        method: "GET",
        path: "/",
        validated_params: nil,
        dependencies: nil
      }

      assert Spikard.Request.get_dependency(req, "db") == nil
    end
  end
end

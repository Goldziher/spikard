defmodule Spikard.DISimpleTest do
  @moduledoc """
  Simple unit tests for Spikard DI module without server integration.
  """

  use ExUnit.Case

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

    test "multiple value dependencies have unique keys" do
      dep1 = Spikard.DI.value("db", %{})
      dep2 = Spikard.DI.value("cache", %{})
      assert dep1.key != dep2.key
      assert dep1.key == "db"
      assert dep2.key == "cache"
    end
  end

  describe "Spikard.DI.factory/2 and factory/3" do
    test "creates a factory dependency with default options" do
      factory = fn -> %{id: 1} end
      dep = Spikard.DI.factory("ctx", factory)

      assert dep.type == :factory
      assert dep.key == "ctx"
      assert is_function(dep.factory)
      assert dep.singleton == true
      assert dep.depends_on == []
    end

    test "factory with custom singleton option" do
      factory = fn -> %{id: 1} end

      dep_singleton = Spikard.DI.factory("singleton", factory, singleton: true)
      assert dep_singleton.singleton == true

      dep_transient = Spikard.DI.factory("transient", factory, singleton: false)
      assert dep_transient.singleton == false
    end

    test "factory with depends_on option" do
      factory = fn -> %{id: 1} end
      dep = Spikard.DI.factory("user", factory, depends_on: ["db", "logger"])

      assert dep.depends_on == ["db", "logger"]
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
  end

  describe "dependency validation" do
    test "validates empty dependency list" do
      assert :ok == Spikard.DI.validate([])
    end

    test "validates value dependencies" do
      deps = [Spikard.DI.value("db", %{})]
      assert :ok == Spikard.DI.validate(deps)
    end

    test "validates factory dependencies" do
      deps = [Spikard.DI.factory("ctx", fn -> %{} end)]
      assert :ok == Spikard.DI.validate(deps)
    end

    test "validates mixed dependencies" do
      deps = [
        Spikard.DI.value("db", %{}),
        Spikard.DI.factory("ctx", fn -> %{} end),
        Spikard.DI.value("cache", %{})
      ]
      assert :ok == Spikard.DI.validate(deps)
    end
  end

  describe "dependency counting" do
    test "counts empty dependencies" do
      counts = Spikard.DI.count([])
      assert counts.value == 0
      assert counts.factory == 0
      assert counts.total == 0
    end

    test "counts only value dependencies" do
      deps = [
        Spikard.DI.value("db", %{}),
        Spikard.DI.value("cache", %{})
      ]
      counts = Spikard.DI.count(deps)
      assert counts.value == 2
      assert counts.factory == 0
      assert counts.total == 2
    end

    test "counts only factory dependencies" do
      deps = [
        Spikard.DI.factory("ctx1", fn -> %{} end),
        Spikard.DI.factory("ctx2", fn -> %{} end),
        Spikard.DI.factory("ctx3", fn -> %{} end)
      ]
      counts = Spikard.DI.count(deps)
      assert counts.value == 0
      assert counts.factory == 3
      assert counts.total == 3
    end

    test "counts mixed dependencies" do
      deps = [
        Spikard.DI.value("db", %{}),
        Spikard.DI.factory("ctx", fn -> %{} end),
        Spikard.DI.value("cache", %{})
      ]
      counts = Spikard.DI.count(deps)
      assert counts.value == 2
      assert counts.factory == 1
      assert counts.total == 3
    end
  end

  describe "dependency retrieval" do
    test "gets a dependency by key" do
      db_dep = Spikard.DI.value("db", %{host: "localhost"})
      cache_dep = Spikard.DI.value("cache", %{ttl: 3600})

      deps = [db_dep, cache_dep]

      assert Spikard.DI.get(deps, "db") == db_dep
      assert Spikard.DI.get(deps, "cache") == cache_dep
    end

    test "returns nil for missing dependency" do
      deps = [Spikard.DI.value("db", %{})]
      assert Spikard.DI.get(deps, "missing") == nil
    end

    test "returns nil from empty dependency list" do
      assert Spikard.DI.get([], "any") == nil
    end
  end

  describe "Spikard.Request.get_dependency/2" do
    test "retrieves a dependency from request" do
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
        dependencies: %{"db" => %{connected: true}}
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
        dependencies: %{"db" => %{}}
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

    test "returns nil when dependencies is empty map" do
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

      assert Spikard.Request.get_dependency(req, "db") == nil
    end

    test "multiple dependencies in request" do
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
        dependencies: %{
          "db" => %{host: "localhost"},
          "cache" => %{ttl: 3600},
          "logger" => %{level: "info"}
        }
      }

      assert Spikard.Request.get_dependency(req, "db") == %{host: "localhost"}
      assert Spikard.Request.get_dependency(req, "cache") == %{ttl: 3600}
      assert Spikard.Request.get_dependency(req, "logger") == %{level: "info"}
      assert Spikard.Request.get_dependency(req, "missing") == nil
    end
  end
end

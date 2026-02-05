defmodule Spikard.RouterTest do
  use ExUnit.Case

  defmodule SimpleRouter do
    use Spikard.Router

    get("/", &__MODULE__.index/1)
    post("/users", &__MODULE__.create_user/1)
    get("/users/:id", &__MODULE__.show_user/1)

    def index(_req) do
      %{status: 200, headers: [], body: "OK"}
    end

    def create_user(_req) do
      %{status: 201, headers: [], body: "Created"}
    end

    def show_user(_req) do
      %{status: 200, headers: [], body: "Show"}
    end
  end

  defmodule ScopedRouter do
    use Spikard.Router

    scope "/api/v1" do
      pipe_through([:json])
      get("/items", &__MODULE__.list_items/1)
      post("/items", &__MODULE__.create_item/1)
    end

    scope "/admin" do
      pipe_through([:auth, :admin])
      get("/dashboard", &__MODULE__.dashboard/1)
    end

    def list_items(_req) do
      %{status: 200, headers: [], body: "[]"}
    end

    def create_item(_req) do
      %{status: 201, headers: [], body: "{}"}
    end

    def dashboard(_req) do
      %{status: 200, headers: [], body: "Admin"}
    end
  end

  defmodule NestedScopeRouter do
    use Spikard.Router

    scope "/api" do
      scope "/v1" do
        get("/users", &__MODULE__.users/1)
      end

      scope "/v2" do
        get("/users", &__MODULE__.users_v2/1)
      end
    end

    def users(_req), do: %{status: 200, headers: [], body: ""}
    def users_v2(_req), do: %{status: 200, headers: [], body: ""}
  end

  test "simple routes are compiled" do
    routes = SimpleRouter.routes()
    assert length(routes) == 3

    # Check GET /
    assert Enum.any?(routes, fn r ->
             r.method == "GET" && r.path == "/" && r.pipes == []
           end)

    # Check POST /users
    assert Enum.any?(routes, fn r ->
             r.method == "POST" && r.path == "/users" && r.pipes == []
           end)

    # Check GET /users/:id
    assert Enum.any?(routes, fn r ->
             r.method == "GET" && r.path == "/users/:id" && r.pipes == []
           end)
  end

  test "scoped routes with pipes are compiled" do
    routes = ScopedRouter.routes()
    # 3 routes: GET /api/v1/items, POST /api/v1/items, GET /admin/dashboard
    assert length(routes) == 3

    # Check /api/v1/items routes
    assert Enum.any?(routes, fn r ->
             r.method == "GET" && r.path == "/api/v1/items" && r.pipes == [:json]
           end)

    assert Enum.any?(routes, fn r ->
             r.method == "POST" && r.path == "/api/v1/items" && r.pipes == [:json]
           end)

    # Check /admin/dashboard
    assert Enum.any?(routes, fn r ->
             r.method == "GET" && r.path == "/admin/dashboard" && r.pipes == [:auth, :admin]
           end)
  end

  test "nested scopes are compiled correctly" do
    routes = NestedScopeRouter.routes()
    assert length(routes) == 2

    # Check /api/v1/users
    assert Enum.any?(routes, fn r ->
             r.method == "GET" && r.path == "/api/v1/users"
           end)

    # Check /api/v2/users
    assert Enum.any?(routes, fn r ->
             r.method == "GET" && r.path == "/api/v2/users"
           end)
  end

  test "routes_json/0 returns JSON string" do
    json_string = SimpleRouter.routes_json()
    assert is_binary(json_string)

    # Decode and verify structure
    {:ok, json_routes} = Jason.decode(json_string)
    assert length(json_routes) == 3

    first = hd(json_routes)
    assert is_map(first)
    assert Map.has_key?(first, "method")
    assert Map.has_key?(first, "path")
    assert Map.has_key?(first, "handler_name")
  end

  defmodule AllMethodsRouter do
    use Spikard.Router

    get("/items", &__MODULE__.items_get/1)
    post("/items", &__MODULE__.items_post/1)
    put("/items/:id", &__MODULE__.items_put/1)
    patch("/items/:id", &__MODULE__.items_patch/1)
    delete("/items/:id", &__MODULE__.items_delete/1)
    head("/health", &__MODULE__.health/1)
    options("/items", &__MODULE__.items_options/1)

    def items_get(_req), do: nil
    def items_post(_req), do: nil
    def items_put(_req), do: nil
    def items_patch(_req), do: nil
    def items_delete(_req), do: nil
    def health(_req), do: nil
    def items_options(_req), do: nil
  end

  test "all HTTP methods are supported" do
    routes = AllMethodsRouter.routes()
    methods = routes |> Enum.map(& &1.method) |> Enum.sort()

    assert methods == ["DELETE", "GET", "HEAD", "OPTIONS", "PATCH", "POST", "PUT"]
  end

  defmodule PipeAccumulateRouter do
    use Spikard.Router

    scope "/api" do
      pipe_through([:json])
      get("/v1", &__MODULE__.v1/1)

      pipe_through([:auth])
      get("/v2", &__MODULE__.v2/1)
    end

    def v1(_req), do: nil
    def v2(_req), do: nil
  end

  test "pipe_through accumulates pipes in same scope" do
    routes = PipeAccumulateRouter.routes()

    v1_route = Enum.find(routes, &(&1.path == "/api/v1"))
    v2_route = Enum.find(routes, &(&1.path == "/api/v2"))

    # First call adds :json
    assert v1_route.pipes == [:json]
    # Second call adds :auth on top of :json
    assert v2_route.pipes == [:json, :auth]
  end
end

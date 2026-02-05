defmodule Spikard.DI do
  @moduledoc """
  Dependency Injection configuration and helpers for Spikard.

  This module provides utilities for configuring and managing dependencies
  that can be injected into handlers. Two types of dependencies are supported:

  ## Value Dependencies

  Simple storage of values that are immediately available to handlers:

      db = %{host: "localhost", port: 5432}
      Spikard.DI.value("db", db)

  ## Factory Dependencies

  Call an Elixir function to create a value per request:

      factory = fn -> %{request_id: System.unique_integer()} end
      Spikard.DI.factory("request_context", factory)

  Factory dependencies can be singleton (created once) or transient (per-request).

  ## Examples

      {:ok, server} = Spikard.start(
        port: 4000,
        routes: [...],
        dependencies: [
          Spikard.DI.value("database", db),
          Spikard.DI.factory("request_id", fn -> System.unique_integer() end)
        ]
      )

      # In a handler:
      defmodule MyHandler do
        def handle(request) do
          db = Spikard.Request.get_dependency(request, "database")
          req_id = Spikard.Request.get_dependency(request, "request_id")
          %{status: 200, body: %{db: db, id: req_id}}
        end
      end
  """

  @type value_dependency :: %{
          type: :value,
          key: String.t(),
          value: term()
        }

  @type factory_dependency :: %{
          type: :factory,
          key: String.t(),
          factory: function(),
          depends_on: [String.t()],
          singleton: boolean()
        }

  @type dependency :: value_dependency() | factory_dependency()

  @doc """
  Creates a value dependency with a simple stored value.

  Value dependencies are immediately available to handlers without any
  computation. They are ideal for configuration, database connections, etc.

  ## Parameters

    * `key` - Unique identifier for the dependency (string)
    * `value` - The value to store and inject

  ## Returns

    A value dependency struct that can be passed to `Spikard.start/2`

  ## Examples

      iex> db = %{host: "localhost"}
      iex> dep = Spikard.DI.value("db", db)
      iex> dep.type
      :value
      iex> dep.key
      "db"
      iex> dep.value == db
      true
  """
  @spec value(String.t(), term()) :: value_dependency()
  def value(key, value) when is_binary(key) do
    %{
      type: :value,
      key: key,
      value: value
    }
  end

  @doc """
  Creates a factory dependency with an Elixir callback function.

  Factory dependencies call an Elixir function to create a value. This allows
  for per-request context creation, lazy initialization, or dynamic values.

  ## Parameters

    * `key` - Unique identifier for the dependency (string)
    * `factory` - Function with arity 0 that returns the dependency value
    * `opts` - Optional keyword arguments:
      - `depends_on: [String.t()]` - Keys of other dependencies this one requires
      - `singleton: boolean()` - If true, factory is called once; if false, per-request

  ## Returns

    A factory dependency struct that can be passed to `Spikard.start/2`

  ## Examples

      iex> my_factory = fn -> %{id: 1} end
      iex> dep = Spikard.DI.factory("request_context", my_factory)
      iex> dep.type
      :factory
      iex> dep.singleton
      true

      iex> my_factory2 = fn -> %{id: 2} end
      iex> dep2 = Spikard.DI.factory("context", my_factory2, singleton: false)
      iex> dep2.singleton
      false

      iex> factory_with_deps = fn -> %{user_id: 1} end
      iex> dep3 = Spikard.DI.factory("user", factory_with_deps, depends_on: ["db"])
      iex> dep3.depends_on
      ["db"]
  """
  @spec factory(String.t(), (-> term()), Keyword.t()) :: factory_dependency()
  def factory(key, factory, opts \\ []) when is_binary(key) and is_function(factory, 0) do
    %{
      type: :factory,
      key: key,
      factory: factory,
      depends_on: Keyword.get(opts, :depends_on, []),
      singleton: Keyword.get(opts, :singleton, true)
    }
  end

  @doc """
  Validates a list of dependencies.

  Checks that all dependencies have valid structure and keys are unique.

  ## Parameters

    * `deps` - List of dependency structs

  ## Returns

    `:ok` if valid, `{:error, reason}` otherwise

  ## Examples

      iex> deps = [Spikard.DI.value("db", %{})]
      iex> Spikard.DI.validate(deps)
      :ok

      iex> Spikard.DI.validate([])
      :ok
  """
  @spec validate([dependency()]) :: :ok | {:error, String.t()}
  def validate(deps) when is_list(deps) do
    deps
    |> Enum.reduce_while(:ok, fn dep, _acc ->
      case validate_dependency(dep) do
        :ok -> {:cont, :ok}
        error -> {:halt, error}
      end
    end)
  end

  defp validate_dependency(dep) when is_map(dep) do
    case Map.get(dep, :type) do
      :value ->
        if is_binary(Map.get(dep, :key)), do: :ok, else: {:error, "Value dependency missing key"}

      :factory ->
        if is_binary(Map.get(dep, :key)) and is_function(Map.get(dep, :factory)),
          do: :ok,
          else: {:error, "Factory dependency invalid"}

      _ ->
        {:error, "Unknown dependency type"}
    end
  end

  defp validate_dependency(_), do: {:error, "Dependency must be a map"}

  @doc """
  Counts dependencies by type.

  ## Parameters

    * `deps` - List of dependency structs

  ## Returns

    A map with counts for `:value`, `:factory`, and `:total`

  ## Examples

      iex> deps = [
      ...>   Spikard.DI.value("db", %{}),
      ...>   Spikard.DI.value("cache", %{}),
      ...>   Spikard.DI.factory("ctx", fn -> %{} end)
      ...> ]
      iex> counts = Spikard.DI.count(deps)
      iex> counts.value
      2
      iex> counts.factory
      1
      iex> counts.total
      3
  """
  @spec count([dependency()]) :: %{value: non_neg_integer(), factory: non_neg_integer(), total: non_neg_integer()}
  def count(deps) when is_list(deps) do
    counts =
      Enum.reduce(deps, %{value: 0, factory: 0}, fn dep, acc ->
        case dep.type do
          :value -> %{acc | value: acc.value + 1}
          :factory -> %{acc | factory: acc.factory + 1}
          _ -> acc
        end
      end)

    %{
      value: counts.value,
      factory: counts.factory,
      total: counts.value + counts.factory
    }
  end

  @doc """
  Gets a dependency by key.

  ## Parameters

    * `deps` - List of dependency structs
    * `key` - The dependency key to find

  ## Returns

    The dependency struct, or `nil` if not found
  """
  @spec get([dependency()], String.t()) :: dependency() | nil
  def get(deps, key) when is_list(deps) and is_binary(key) do
    Enum.find(deps, &(&1.key == key))
  end
end

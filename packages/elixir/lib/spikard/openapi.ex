defmodule Spikard.OpenAPI do
  @moduledoc """
  OpenAPI specification configuration and generation for Spikard HTTP applications.

  This module provides configuration helpers for generating OpenAPI 3.0 specifications
  from route definitions. The OpenAPI spec is automatically generated and served
  at a configurable endpoint (default: /openapi.json).

  ## Configuration

  To enable OpenAPI generation, pass an OpenAPI config to TestClient or server start:

      {:ok, client} = Spikard.TestClient.new(
        routes: [{:get, "/users", &handler/1}],
        openapi: Spikard.OpenAPI.config(
          title: "My API",
          version: "1.0.0",
          description: "A comprehensive API"
        )
      )

  ## Route Schemas

  You can annotate routes with response schemas:

      get "/users", &list_users/1, response_schema: user_list_schema()

  ## Endpoints

  When OpenAPI is enabled:
  - `/openapi.json` - OpenAPI 3.0 specification in JSON format

  ## Examples

      # Basic configuration
      config = Spikard.OpenAPI.config(
        title: "Users API",
        version: "2.0.0"
      )

      # With all options
      config = Spikard.OpenAPI.config(
        title: "Users API",
        version: "2.0.0",
        description: "API for managing users",
        openapi_json_path: "/api/spec.json",
        contact: %{
          "name" => "API Support",
          "email" => "support@example.com"
        },
        license: %{
          "name" => "MIT"
        },
        servers: [
          %{"url" => "https://api.example.com", "description" => "Production"}
        ]
      )
  """

  @type config :: %{
          title: String.t(),
          version: String.t(),
          description: String.t() | nil,
          openapi_json_path: String.t(),
          contact: map() | nil,
          license: map() | nil,
          servers: [map()] | nil
        }

  @doc """
  Creates an OpenAPI configuration from the given options.

  ## Parameters

    * `:title` - (required) The title of the API
    * `:version` - (required) The version of the API
    * `:description` - (optional) A description of the API
    * `:openapi_json_path` - (optional) Path to serve the OpenAPI spec (default: "/openapi.json")
    * `:contact` - (optional) Contact information map with keys: "name", "email", "url"
    * `:license` - (optional) License information map with keys: "name", "url"
    * `:servers` - (optional) List of server maps with keys: "url", "description"

  ## Returns

    A config map that can be passed to TestClient.new/1 or Spikard.start/1

  ## Examples

      Spikard.OpenAPI.config(title: "My API", version: "1.0.0")

      Spikard.OpenAPI.config(
        title: "Users API",
        version: "2.0.0",
        description: "Manage user accounts",
        openapi_json_path: "/api/spec.json"
      )
  """
  @spec config(keyword()) :: config()
  def config(opts) when is_list(opts) do
    title = Keyword.fetch!(opts, :title)
    version = Keyword.fetch!(opts, :version)

    %{
      title: title,
      version: version,
      description: Keyword.get(opts, :description),
      openapi_json_path: Keyword.get(opts, :openapi_json_path, "/openapi.json"),
      contact: Keyword.get(opts, :contact),
      license: Keyword.get(opts, :license),
      servers: Keyword.get(opts, :servers)
    }
  end

  @doc """
  Checks if OpenAPI is enabled (config is present and valid).

  ## Parameters

    * `config` - OpenAPI config or nil

  ## Returns

    * `true` if config is a valid OpenAPI configuration
    * `false` otherwise

  ## Examples

      config = Spikard.OpenAPI.config(title: "API", version: "1.0.0")
      Spikard.OpenAPI.enabled?(config)  # => true

      Spikard.OpenAPI.enabled?(nil)  # => false
  """
  @spec enabled?(config() | nil) :: boolean()
  def enabled?(config) when is_map(config) do
    Map.has_key?(config, :title) and Map.has_key?(config, :version)
  end

  def enabled?(_), do: false

  defmodule Generator do
    @moduledoc """
    OpenAPI spec generator from route metadata.

    This module converts route definitions into OpenAPI path specifications.
    """

    @type generator :: %{
            title: String.t(),
            version: String.t(),
            description: String.t() | nil,
            openapi_json_path: String.t(),
            contact: map() | nil,
            license: map() | nil,
            servers: [map()] | nil
          }

    @doc """
    Creates a new OpenAPI generator from a configuration.

    ## Parameters

      * `config` - OpenAPI configuration from Spikard.OpenAPI.config/1

    ## Returns

      A generator struct ready to convert routes to OpenAPI spec
    """
    @spec new(Spikard.OpenAPI.config()) :: generator()
    def new(config) when is_map(config) do
      config
    end

    @doc """
    Converts routes to OpenAPI paths specification.

    Takes a list of route metadata and converts them to OpenAPI path objects.
    Routes are keyed by path with HTTP method sub-keys.

    ## Parameters

      * `generator` - Generator created by new/1
      * `routes` - List of route metadata maps

    ## Returns

      A map of paths following OpenAPI 3.0 structure

    ## Examples

        routes = [
          %{"method" => "GET", "path" => "/users", "handler_name" => "Handler.list"},
          %{"method" => "POST", "path" => "/users", "handler_name" => "Handler.create"}
        ]

        paths = Generator.to_paths(generator, routes)
        # => %{
        #   "/users" => %{
        #     "get" => %{...},
        #     "post" => %{...}
        #   }
        # }
    """
    @spec to_paths(generator(), [map()]) :: map()
    def to_paths(_generator, routes) when is_list(routes) do
      routes
      |> Enum.group_by(fn route -> route["path"] end)
      |> Enum.map(fn {path, path_routes} ->
        methods = build_methods(path_routes)
        {path, methods}
      end)
      |> Enum.into(%{})
    end

    # Build method entries for a path
    defp build_methods(routes) do
      routes
      |> Enum.map(fn route ->
        method = route["method"] |> String.downcase()
        operation = build_operation(route)
        {method, operation}
      end)
      |> Enum.into(%{})
    end

    # Build a single operation (method) for a route
    defp build_operation(route) do
      %{
        "summary" => route["handler_name"] || "Operation",
        "operationId" => operation_id(route),
        "responses" => build_responses(route)
      }
    end

    # Generate an operation ID from route info
    defp operation_id(route) do
      method = route["method"] |> String.downcase()
      path = route["path"] |> String.replace(~r/[{}:]/, "_")
      "#{method}_#{path}"
    end

    # Build responses section
    defp build_responses(route) do
      response_schema = route["response_schema"]

      base_response = %{
        "200" => %{
          "description" => "Successful response"
        }
      }

      if response_schema do
        Map.update!(base_response, "200", fn resp ->
          Map.put(resp, "content", %{
            "application/json" => %{
              "schema" => response_schema
            }
          })
        end)
      else
        base_response
      end
    end
  end
end

defmodule Spikard.TestClient do
  @moduledoc """
  Test client for making HTTP requests to Spikard handlers without network overhead.

  This module provides a convenient API for testing Spikard HTTP handlers in isolation.
  Handlers are invoked synchronously, making it ideal for unit and integration testing.

  ## Quick Start

      handler = fn _req -> %{status: 200, body: %{ok: true}} end
      {:ok, client} = TestClient.new(routes: [{:get, "/", handler}])
      {:ok, response} = TestClient.get(client, "/")
      assert response.status_code == 200

  ## Supported HTTP Methods

  The client supports all standard HTTP methods:
  - `get/3` - GET requests
  - `post/3` - POST requests
  - `put/3` - PUT requests
  - `patch/3` - PATCH requests
  - `delete/3` - DELETE requests
  - `options/3` - OPTIONS requests
  - `head/3` - HEAD requests

  ## Request Options

  All HTTP method functions accept options as the third parameter (default: []):

  - `:headers` - List of {name, value} tuples for custom headers
  - `:query` - List of {key, value} tuples for query parameters
  - `:json` - Map or term to encode as JSON body
  - `:form` - List of {key, value} tuples for form data
  - `:multipart` - List of {name, data} or {name, data, options} tuples for multipart
  - `:cookies` - List of {name, value} tuples for cookies

  ## Examples

      # Simple GET request
      {:ok, response} = TestClient.get(client, "/users")
      assert response.status_code == 200

      # GET with query parameters
      {:ok, response} = TestClient.get(client, "/users", query: [{"page", "2"}])

      # POST with JSON body
      {:ok, response} = TestClient.post(client, "/users", json: %{name: "Alice"})

      # PUT with headers
      {:ok, response} = TestClient.put(client, "/users/1",
        headers: [{"authorization", "Bearer token"}],
        json: %{email: "new@example.com"}
      )

      # POST with multipart
      {:ok, response} = TestClient.post(client, "/upload",
        multipart: [{"file", "content", filename: "test.txt"}]
      )

  ## Response Handling

  All request functions return `{:ok, response}` on success. The response map
  contains fields that can be accessed directly or via Response helper functions:

      {:ok, response} = TestClient.get(client, "/api/users")

      # Direct field access
      status = response.status_code
      headers = response.headers

      # Using Response helpers
      json = Response.json(response)
      text = Response.text(response)
      allow_header = Response.header(response, "allow")

  ## Handler Errors

  If a handler raises an exception or throws, the test client returns a 500
  status code with an error message in the body.
  """

  @type client :: reference() | tuple()
  @type response :: map()
  @type request_opts :: keyword()

  alias Spikard.Native
  alias Spikard.HandlerRunner
  alias __MODULE__.Response

  @doc """
  Creates a new test client with the given routes.

  Routes should be a list of {method, path, handler} tuples where:
  - `method` is an atom like `:get`, `:post`, etc.
  - `path` is a string like `"/"` or `"/users/:id"`
  - `handler` is a function that takes a Request and returns a response map

  ## Parameters

    * `opts` - Keyword list with the following keys:
      - `:routes` (required) - List of route tuples
      - `:lifecycle` (optional) - Lifecycle hooks configuration
      - `:jwt_auth` (optional) - JWT authentication configuration
      - `:api_key_auth` (optional) - API key authentication configuration
      - `:cors` (optional) - CORS configuration

  ## Returns

    * `{:ok, client}` - Test client created successfully
    * `{:error, reason}` - Failed to create client

  ## Examples

      handler = fn _req -> %{status: 200, body: %{ok: true}} end
      {:ok, client} = TestClient.new(routes: [{:get, "/", handler}])

      # With API key authentication
      {:ok, client} = TestClient.new(
        routes: [{:get, "/api", handler}],
        api_key_auth: %{keys: ["my-key"], header_name: "X-API-Key"}
      )

      # With CORS configuration
      {:ok, client} = TestClient.new(
        routes: [{:get, "/api", handler}],
        cors: %{
          allowed_origins: ["https://example.com"],
          allowed_methods: ["GET", "POST"],
          allowed_headers: ["content-type"],
          max_age: 3600
        }
      )
  """
  @spec new(keyword()) :: {:ok, client()} | {:error, String.t()}
  def new(opts) when is_list(opts) do
    with :ok <- validate_new_opts(opts),
         {:ok, routes_json} <- serialize_routes(Keyword.get(opts, :routes, []), Keyword.get(opts, :cors)),
         handlers <- build_handlers_map(Keyword.get(opts, :routes, [])),
         lifecycle <- Keyword.get(opts, :lifecycle, []),
         :ok <- validate_lifecycle(lifecycle),
         # Add API key auth hook if configured
         lifecycle_with_auth <- maybe_add_api_key_auth_hook(lifecycle, Keyword.get(opts, :api_key_auth)),
         lifecycle_hooks <- organize_lifecycle_hooks(lifecycle_with_auth),
         dependencies <- Keyword.get(opts, :dependencies, []),
         {:ok, handler_runner_pid} <- HandlerRunner.start_link(handlers, lifecycle_hooks, dependencies),
         config <- build_config(opts),
         {:ok, client} <- Native.test_client_new(routes_json, handler_runner_pid, config) do
      {:ok, client}
    else
      {:error, reason} -> {:error, reason}
      error -> {:error, inspect(error)}
    end
  end

  # Build complete config map including lifecycle, auth, cors, etc. for the NIF
  # Important: lifecycle must be a map (not keyword list) for Rust to parse it
  defp build_config(opts) do
    lifecycle = Keyword.get(opts, :lifecycle, [])
    api_key_auth = Keyword.get(opts, :api_key_auth)

    # Add API key auth hook if configured (same logic as in new/1)
    # This ensures the hook count is included in the config sent to Rust
    lifecycle_with_auth = maybe_add_api_key_auth_hook(lifecycle, api_key_auth)

    # Convert lifecycle keyword list to a map with string keys and list lengths
    # Rust side extracts counts (number of hooks per type) from this
    lifecycle_map = lifecycle_to_counts_map(lifecycle_with_auth)

    %{
      "lifecycle" => lifecycle_map,
      "jwt_auth" => Keyword.get(opts, :jwt_auth),
      "api_key_auth" => api_key_auth,
      "cors" => Keyword.get(opts, :cors)
    }
  end

  # Convert lifecycle keyword list to a map with hook counts for Rust
  # Input: [on_request: [hook1, hook2], pre_handler: [hook3], ...]
  # Output: %{"on_request" => [nil, nil], "pre_handler" => [nil], ...}
  # The Rust side counts list length to determine number of hooks
  defp lifecycle_to_counts_map(lifecycle) when is_list(lifecycle) do
    Enum.reduce(lifecycle, %{}, fn {hook_type, hooks}, acc ->
      hook_type_str = to_string(hook_type)
      # Create a list of nils with same length as hooks list
      # Rust counts the list length to know how many hooks exist
      hooks_list = if is_list(hooks), do: Enum.map(hooks, fn _ -> nil end), else: [nil]
      Map.put(acc, hook_type_str, hooks_list)
    end)
  end

  defp lifecycle_to_counts_map(_), do: %{}

  # Add API key authentication hook if api_key_auth is configured
  defp maybe_add_api_key_auth_hook(lifecycle, nil), do: lifecycle

  defp maybe_add_api_key_auth_hook(lifecycle, %{} = api_key_config) do
    valid_keys = Map.get(api_key_config, :keys, [])
    # Keep original header name for error messages, use lowercase for lookup
    original_header_name = Map.get(api_key_config, :header_name, "x-api-key")
    header_name_lower = String.downcase(original_header_name)

    # Create the API key validation hook
    api_key_hook = fn ctx ->
      # Check header first (headers are already lowercase from Rust)
      header_key = Map.get(ctx.headers, header_name_lower, "")

      # Check query parameter as fallback
      query_key = extract_api_key_from_query(ctx.query)

      # Use header if present, otherwise query param
      provided_key = if header_key != "", do: header_key, else: query_key

      cond do
        provided_key == "" ->
          # No API key provided - use original header name in error message
          {:short_circuit, build_api_key_error_response(401, "missing", original_header_name)}

        provided_key in valid_keys ->
          # Valid key
          {:continue, ctx}

        true ->
          # Invalid key
          {:short_circuit, build_api_key_error_response(401, "invalid", original_header_name)}
      end
    end

    # Prepend the API key hook to pre_handler hooks
    existing_pre_handler = Keyword.get(lifecycle, :pre_handler, [])
    Keyword.put(lifecycle, :pre_handler, [api_key_hook | existing_pre_handler])
  end

  defp maybe_add_api_key_auth_hook(lifecycle, _), do: lifecycle

  # Extract api_key from query string
  defp extract_api_key_from_query(query) when is_binary(query) do
    query
    |> URI.decode_query()
    |> Map.get("api_key", "")
  end

  defp extract_api_key_from_query(_), do: ""

  # Build RFC 9457 Problem Details error response
  defp build_api_key_error_response(status, error_type, header_name) do
    {title, detail} =
      case error_type do
        "missing" ->
          {"Unauthorized",
           "API key is required. Provide it via the '#{header_name}' header or 'api_key' query parameter."}

        "invalid" ->
          {"Unauthorized", "The provided API key is invalid."}

        _ ->
          {"Unauthorized", "Authentication failed."}
      end

    %{
      status: status,
      headers: %{"content-type" => "application/problem+json"},
      body: %{
        type: "about:blank",
        title: title,
        status: status,
        detail: detail
      }
    }
  end

  # Validate options to new/1
  defp validate_new_opts(opts) do
    case Keyword.get(opts, :routes) do
      nil -> {:error, "routes option is required"}
      [] -> {:error, "routes must not be empty"}
      _routes -> :ok
    end
  end

  # Serialize routes to JSON for the Rust layer
  defp serialize_routes(routes, cors) do
    try do
      json_routes =
        routes
        |> Enum.with_index()
        |> Enum.map(fn {{method, path, _handler}, idx} ->
          # Convert path params from :id to {id} style for Axum
          converted_path = convert_path_params(path)

          %{
            "index" => idx,
            "method" => atom_to_http_method(method),
            "path" => converted_path,
            "handler_name" => "handler_#{idx}",
            "is_async" => true,
            "cors" => cors
          }
        end)

      {:ok, Jason.encode!(json_routes)}
    rescue
      e -> {:error, "Failed to serialize routes: #{inspect(e)}"}
    end
  end

  # Convert Elixir-style path parameters (:id) to Axum-style ({id})
  defp convert_path_params(path) when is_binary(path) do
    Regex.replace(~r/:([a-zA-Z_][a-zA-Z0-9_]*)/, path, "{\\1}")
  end

  # Build a map of handlers keyed by handler name for the HandlerRunner
  defp build_handlers_map(routes) do
    routes
    |> Enum.with_index()
    |> Enum.reduce(%{}, fn {{_method, _path, handler}, idx}, acc ->
      # Use string key matching the handler_name generated in serialize_routes
      Map.put(acc, "handler_#{idx}", handler)
    end)
  end

  # Validate lifecycle hooks if provided
  defp validate_lifecycle(lifecycle) when is_list(lifecycle), do: :ok
  defp validate_lifecycle(nil), do: :ok
  defp validate_lifecycle(_), do: {:error, "lifecycle must be a list"}

  # Organize lifecycle hooks from keyword list to map format for HandlerRunner
  # Input: [on_request: [hook1, hook2], pre_handler: [hook3], ...]
  # Output: %{"on_request" => [hook1, hook2], "pre_handler" => [hook3], ...}
  defp organize_lifecycle_hooks(lifecycle) when is_list(lifecycle) do
    lifecycle
    |> Enum.reduce(%{}, fn {hook_type, hooks}, acc ->
      hook_type_str = to_string(hook_type)
      hooks_list = if is_list(hooks), do: hooks, else: [hooks]
      Map.put(acc, hook_type_str, hooks_list)
    end)
  end

  defp organize_lifecycle_hooks(_), do: %{}

  # Convert atom HTTP method to string
  defp atom_to_http_method(:get), do: "GET"
  defp atom_to_http_method(:post), do: "POST"
  defp atom_to_http_method(:put), do: "PUT"
  defp atom_to_http_method(:patch), do: "PATCH"
  defp atom_to_http_method(:delete), do: "DELETE"
  defp atom_to_http_method(:options), do: "OPTIONS"
  defp atom_to_http_method(:head), do: "HEAD"
  defp atom_to_http_method(other), do: String.upcase(to_string(other))

  @doc """
  Make a GET request to the test client.

  ## Parameters

    * `client` - Test client reference
    * `path` - Request path
    * `opts` - Request options (default: [])

  ## Returns

    * `{:ok, response}` - Request successful
    * `{:error, reason}` - Request failed
  """
  @spec get(client(), String.t(), request_opts()) :: {:ok, response()} | {:error, term()}
  def get(client, path, opts \\ []) when is_binary(path) and is_list(opts) do
    request(client, "GET", path, opts)
  end

  @doc """
  Make a POST request to the test client.

  ## Parameters

    * `client` - Test client reference
    * `path` - Request path
    * `opts` - Request options (default: [])

  ## Returns

    * `{:ok, response}` - Request successful
    * `{:error, reason}` - Request failed
  """
  @spec post(client(), String.t(), request_opts()) :: {:ok, response()} | {:error, term()}
  def post(client, path, opts \\ []) when is_binary(path) and is_list(opts) do
    request(client, "POST", path, opts)
  end

  @doc """
  Make a PUT request to the test client.

  ## Parameters

    * `client` - Test client reference
    * `path` - Request path
    * `opts` - Request options (default: [])

  ## Returns

    * `{:ok, response}` - Request successful
    * `{:error, reason}` - Request failed
  """
  @spec put(client(), String.t(), request_opts()) :: {:ok, response()} | {:error, term()}
  def put(client, path, opts \\ []) when is_binary(path) and is_list(opts) do
    request(client, "PUT", path, opts)
  end

  @doc """
  Make a PATCH request to the test client.

  ## Parameters

    * `client` - Test client reference
    * `path` - Request path
    * `opts` - Request options (default: [])

  ## Returns

    * `{:ok, response}` - Request successful
    * `{:error, reason}` - Request failed
  """
  @spec patch(client(), String.t(), request_opts()) :: {:ok, response()} | {:error, term()}
  def patch(client, path, opts \\ []) when is_binary(path) and is_list(opts) do
    request(client, "PATCH", path, opts)
  end

  @doc """
  Make a DELETE request to the test client.

  ## Parameters

    * `client` - Test client reference
    * `path` - Request path
    * `opts` - Request options (default: [])

  ## Returns

    * `{:ok, response}` - Request successful
    * `{:error, reason}` - Request failed
  """
  @spec delete(client(), String.t(), request_opts()) :: {:ok, response()} | {:error, term()}
  def delete(client, path, opts \\ []) when is_binary(path) and is_list(opts) do
    request(client, "DELETE", path, opts)
  end

  @doc """
  Make an OPTIONS request to the test client.

  ## Parameters

    * `client` - Test client reference
    * `path` - Request path
    * `opts` - Request options (default: [])

  ## Returns

    * `{:ok, response}` - Request successful
    * `{:error, reason}` - Request failed
  """
  @spec options(client(), String.t(), request_opts()) :: {:ok, response()} | {:error, term()}
  def options(client, path, opts \\ []) when is_binary(path) and is_list(opts) do
    request(client, "OPTIONS", path, opts)
  end

  @doc """
  Make a HEAD request to the test client.

  ## Parameters

    * `client` - Test client reference
    * `path` - Request path
    * `opts` - Request options (default: [])

  ## Returns

    * `{:ok, response}` - Request successful
    * `{:error, reason}` - Request failed
  """
  @spec head(client(), String.t(), request_opts()) :: {:ok, response()} | {:error, term()}
  def head(client, path, opts \\ []) when is_binary(path) and is_list(opts) do
    request(client, "HEAD", path, opts)
  end

  @doc """
  Make a request to the test client.

  This is the underlying function that all HTTP method functions use.

  ## Parameters

    * `client` - Test client reference
    * `method` - HTTP method as string (e.g., "GET", "POST")
    * `path` - Request path
    * `opts` - Request options

  ## Returns

    * `{:ok, response}` - Request successful
    * `{:error, reason}` - Request failed
  """
  @spec request(client(), String.t(), String.t(), request_opts()) :: {:ok, response()} | {:error, term()}
  def request(client, method, path, opts \\ []) when is_binary(method) and is_binary(path) and is_list(opts) do
    req_map = opts_to_map(opts)

    case Native.test_client_request(client, method, path, req_map) do
      {:ok, response_map} ->
        {:ok, Response.from_map(response_map)}

      error ->
        error
    end
  end

  # Convert request options to a map for the NIF
  # Converts headers from list of tuples to map for Rust compatibility
  # Also converts atom keys in json body to string keys
  defp opts_to_map(opts) do
    opts
    |> Keyword.new()
    |> Enum.reduce(%{}, fn {key, value}, acc ->
      key_str = to_string(key)

      converted_value =
        case key_str do
          "headers" -> convert_headers_to_map(value)
          "multipart" -> convert_multipart_parts(value)
          "json" -> stringify_keys(value)
          _ -> value
        end

      Map.put(acc, key_str, converted_value)
    end)
  end

  # Recursively convert atom keys to string keys for JSON serialization
  defp stringify_keys(map) when is_map(map) do
    Map.new(map, fn {k, v} ->
      key = if is_atom(k), do: Atom.to_string(k), else: k
      {key, stringify_keys(v)}
    end)
  end

  defp stringify_keys(list) when is_list(list) do
    Enum.map(list, &stringify_keys/1)
  end

  defp stringify_keys(other), do: other

  # Convert multipart parts from various formats to a consistent structure for Rust
  # Supports: {"name", "data"}, {"name", "data", filename: "name.txt", content_type: "text/plain"}
  defp convert_multipart_parts(parts) when is_list(parts) do
    Enum.map(parts, &convert_single_multipart_part/1)
  end

  defp convert_multipart_parts(_), do: []

  defp convert_single_multipart_part({name, data}) when is_binary(name) and is_binary(data) do
    %{
      "name" => name,
      "content" => data,
      "filename" => nil,
      "content_type" => "application/octet-stream"
    }
  end

  defp convert_single_multipart_part({name, data, opts}) when is_binary(name) and is_binary(data) and is_list(opts) do
    %{
      "name" => name,
      "content" => data,
      "filename" => Keyword.get(opts, :filename),
      "content_type" => Keyword.get(opts, :content_type, "application/octet-stream")
    }
  end

  defp convert_single_multipart_part(_),
    do: %{
      "name" => "",
      "content" => "",
      "filename" => nil,
      "content_type" => "application/octet-stream"
    }

  defp convert_headers_to_map(headers) when is_list(headers) do
    Enum.into(headers, %{}, fn {k, v} -> {to_string(k), to_string(v)} end)
  end

  defp convert_headers_to_map(headers) when is_map(headers), do: headers
  defp convert_headers_to_map(_), do: %{}
end

defmodule Spikard.TestClient.Response do
  @moduledoc """
  Helper functions for working with test client responses.

  This module provides convenient functions to extract and parse data from
  test client responses.

  ## Examples

      {:ok, response} = TestClient.get(client, "/api/users")

      # Get JSON-parsed body
      json = Response.json(response)

      # Get raw text body
      text = Response.text(response)

      # Get a specific header (case-insensitive)
      content_type = Response.header(response, "content-type")
  """

  @type client :: reference() | tuple()
  @type request_opts :: keyword()

  @doc """
  Extracts and parses the JSON body from a response.

  If the response body is already parsed (from the JSON request), it returns
  that directly. Otherwise, it attempts to decode the body as JSON.

  ## Parameters

    * `response` - Response map from a test client request

  ## Returns

    A parsed JSON structure (map, list, etc.), or nil if no body

  ## Examples

      {:ok, response} = TestClient.get(client, "/users")
      json = Response.json(response)
      IO.inspect(json)
      # %{"id" => 1, "name" => "Alice"}
  """
  @spec json(map()) :: term()
  def json(response) when is_map(response) do
    cond do
      # If response already has a json field, use it
      Map.has_key?(response, :json) and response.json != nil ->
        response.json

      # If body is already a map/list (parsed), return it
      is_map(response.body) or is_list(response.body) ->
        response.body

      # If body is a string, try to parse it
      is_binary(response.body) ->
        try do
          Jason.decode!(response.body)
        rescue
          _error -> nil
        end

      true ->
        nil
    end
  end

  @doc """
  Extracts the raw text body from a response.

  If the body is a binary string, returns it directly. If it's a parsed
  structure (map/list), encodes it to JSON first.

  ## Parameters

    * `response` - Response map from a test client request

  ## Returns

    The body as a string, or nil if no body

  ## Examples

      {:ok, response} = TestClient.get(client, "/hello")
      text = Response.text(response)
      # "Hello, World!"
  """
  @spec text(map()) :: String.t() | nil
  def text(response) when is_map(response) do
    cond do
      is_binary(response.body) ->
        response.body

      is_map(response.body) or is_list(response.body) ->
        Jason.encode!(response.body)

      true ->
        nil
    end
  end

  @doc """
  Retrieves a header value from the response (case-insensitive).

  Header names are normalized to lowercase for comparison, so lookups
  are case-insensitive.

  ## Parameters

    * `response` - Response map from a test client request
    * `header_name` - Name of the header to retrieve (case-insensitive)

  ## Returns

    The header value as a string, or nil if not found

  ## Examples

      {:ok, response} = TestClient.options(client, "/api")
      allow = Response.header(response, "allow")
      # "GET, POST, OPTIONS"

      content_type = Response.header(response, "Content-Type")
      # "application/json"
  """
  @spec header(map(), String.t()) :: String.t() | nil
  def header(response, header_name) when is_map(response) and is_binary(header_name) do
    headers = response.headers || %{}

    # Normalize header lookup to lowercase
    header_lower = String.downcase(header_name)

    # Try to find the header with case-insensitive matching
    Enum.find_value(headers, nil, fn {k, v} ->
      if String.downcase(to_string(k)) == header_lower do
        to_string(v)
      else
        nil
      end
    end)
  end

  @doc """
  Creates a Response struct from a map (typically from the Rust NIF).

  This function takes a map containing response data from the Rust layer
  and converts it into a typed Response struct.

  ## Parameters

    * `data` - A map with keys matching response fields

  ## Returns

    A Response struct

  ## Examples

      iex> Spikard.TestClient.Response.from_map(%{
      ...>   "status_code" => 200,
      ...>   "headers" => %{"content-type" => "application/json"},
      ...>   "body" => %{"ok" => true}
      ...> })
      %Spikard.TestClient.Response{
        status_code: 200,
        headers: %{"content-type" => "application/json"},
        body: %{"ok" => true}
      }
  """
  @spec from_map(map()) :: map()
  def from_map(data) when is_map(data) do
    %{
      status_code: Map.get(data, "status_code", 200),
      headers: Map.get(data, "headers", %{}),
      body: Map.get(data, "body")
    }
  end
end

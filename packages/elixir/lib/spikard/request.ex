defmodule Spikard.Request do
  @moduledoc """
  HTTP request representation for Spikard handlers.

  This module defines the Request struct that handlers receive and provides
  helper functions to extract data from requests.

  ## Struct Fields

    * `:path_params` - Map of path parameters extracted from route (e.g., %{"id" => "123"})
    * `:query_params` - Parsed query string parameters as a map
    * `:raw_query_params` - Raw query parameters as lists of strings
    * `:headers` - Map of HTTP headers (lowercase keys)
    * `:cookies` - Map of parsed cookies
    * `:body` - Parsed request body (JSON as map or other structured data)
    * `:raw_body` - Raw request body as binary
    * `:method` - HTTP method as string (e.g., "GET", "POST")
    * `:path` - Request path (e.g., "/api/users/123")
    * `:validated_params` - Optional validated parameters from schema validation

  ## Examples

      defmodule MyApp.Handler do
        def show_user(request) do
          user_id = Spikard.Request.get_path_param(request, "id")
          Spikard.Response.json(%{user_id: user_id})
        end

        def list_users(request) do
          page = Spikard.Request.get_query_param(request, "page", "1")
          auth = Spikard.Request.get_header(request, "authorization")
          Spikard.Response.json(%{page: page, auth: auth})
        end

        def create_user(request) do
          body = Spikard.Request.get_body(request)
          Spikard.Response.json(%{created: body})
        end
      end
  """

  @type t :: %__MODULE__{
          path_params: map(),
          query_params: map(),
          raw_query_params: map(),
          headers: map(),
          cookies: map(),
          body: term(),
          raw_body: binary() | nil,
          method: String.t(),
          path: String.t(),
          validated_params: term() | nil,
          dependencies: map() | nil,
          files: [Spikard.UploadFile.t()]
        }

  defstruct [
    :path_params,
    :query_params,
    :raw_query_params,
    :headers,
    :cookies,
    :body,
    :raw_body,
    :method,
    :path,
    :validated_params,
    :dependencies,
    files: []
  ]

  @doc """
  Creates a Request struct from a map (typically from the Rust NIF).

  This function takes a map containing request data from the Rust layer
  and converts it into a typed Request struct.

  ## Parameters

    * `data` - A map with keys matching request fields

  ## Returns

    A Request struct

  ## Examples

      iex> Spikard.Request.from_map(%{
      ...>   "path_params" => %{"id" => "123"},
      ...>   "query_params" => %{"page" => "1"},
      ...>   "headers" => %{"authorization" => "Bearer token"},
      ...>   "cookies" => %{"session" => "abc123"},
      ...>   "body" => %{"name" => "John"},
      ...>   "raw_body" => "{\"name\": \"John\"}",
      ...>   "method" => "POST",
      ...>   "path" => "/api/users",
      ...>   "raw_query_params" => %{"page" => ["1"]},
      ...>   "validated_params" => nil
      ...> })
      %Spikard.Request{
        path_params: %{"id" => "123"},
        query_params: %{"page" => "1"},
        headers: %{"authorization" => "Bearer token"},
        method: "POST"
      }
  """
  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      path_params: Map.get(data, "path_params", %{}) |> ensure_map(),
      query_params: Map.get(data, "query_params", %{}) |> ensure_map(),
      raw_query_params: Map.get(data, "raw_query_params", %{}) |> ensure_map(),
      headers: Map.get(data, "headers", %{}) |> ensure_map(),
      cookies: Map.get(data, "cookies", %{}) |> ensure_map(),
      body: Map.get(data, "body", nil),
      raw_body: Map.get(data, "raw_body", nil),
      method: Map.get(data, "method", ""),
      path: Map.get(data, "path", ""),
      validated_params: Map.get(data, "validated_params", nil),
      dependencies: Map.get(data, "dependencies", nil) |> ensure_map(),
      files: Map.get(data, "files", []) |> ensure_list() |> parse_files()
    }
  end

  @doc """
  Retrieves a path parameter value from the request.

  Path parameters are extracted from the URL path based on route definitions.
  For example, in a route "/users/:id", the `:id` parameter is stored in path_params.

  ## Parameters

    * `request` - The Request struct
    * `key` - The parameter key as a string (without the colon prefix)

  ## Returns

    The parameter value as a string, or `nil` if not found

  ## Examples

      iex> request = Spikard.Request.from_map(%{
      ...>   "path_params" => %{"id" => "123"}
      ...> })
      iex> Spikard.Request.get_path_param(request, "id")
      "123"

      iex> Spikard.Request.get_path_param(request, "missing")
      nil
  """
  @spec get_path_param(t(), String.t()) :: String.t() | nil
  def get_path_param(%__MODULE__{path_params: params}, key) when is_binary(key) do
    Map.get(params, key)
  end

  @doc """
  Retrieves a query parameter value from the request.

  Query parameters come from the URL query string (after the `?`).
  For example, in the URL "/users?page=2&limit=10", query_params would be
  %{"page" => "2", "limit" => "10"}.

  ## Parameters

    * `request` - The Request struct
    * `key` - The parameter key as a string
    * `default` - Default value if parameter is not found (default: `nil`)

  ## Returns

    The parameter value as a string, or the default value if not found

  ## Examples

      iex> request = Spikard.Request.from_map(%{
      ...>   "query_params" => %{"page" => "2", "limit" => "10"}
      ...> })
      iex> Spikard.Request.get_query_param(request, "page")
      "2"

      iex> Spikard.Request.get_query_param(request, "offset", "0")
      "0"

      iex> Spikard.Request.get_query_param(request, "missing")
      nil
  """
  @spec get_query_param(t(), String.t(), term()) :: term()
  def get_query_param(%__MODULE__{query_params: params}, key, default \\ nil) when is_binary(key) do
    Map.get(params, key, default)
  end

  @doc """
  Retrieves an HTTP header value from the request.

  Header lookups are case-insensitive. The underlying header map stores
  all keys in lowercase.

  ## Parameters

    * `request` - The Request struct
    * `key` - The header name (case-insensitive)

  ## Returns

    The header value as a string, or `nil` if not found

  ## Examples

      iex> request = Spikard.Request.from_map(%{
      ...>   "headers" => %{"authorization" => "Bearer xyz", "content-type" => "application/json"}
      ...> })
      iex> Spikard.Request.get_header(request, "Authorization")
      "Bearer xyz"

      iex> Spikard.Request.get_header(request, "CONTENT-TYPE")
      "application/json"

      iex> Spikard.Request.get_header(request, "X-Custom")
      nil
  """
  @spec get_header(t(), String.t()) :: String.t() | nil
  def get_header(%__MODULE__{headers: headers}, key) when is_binary(key) do
    # Normalize key to lowercase for case-insensitive lookup
    key_lower = String.downcase(key)
    Map.get(headers, key_lower)
  end

  @doc """
  Retrieves a cookie value from the request.

  Cookies are parsed from the Cookie header and stored as a map.

  ## Parameters

    * `request` - The Request struct
    * `key` - The cookie name

  ## Returns

    The cookie value as a string, or `nil` if not found

  ## Examples

      iex> request = Spikard.Request.from_map(%{
      ...>   "cookies" => %{"session" => "abc123", "preferences" => "dark-mode"}
      ...> })
      iex> Spikard.Request.get_cookie(request, "session")
      "abc123"

      iex> Spikard.Request.get_cookie(request, "missing")
      nil
  """
  @spec get_cookie(t(), String.t()) :: String.t() | nil
  def get_cookie(%__MODULE__{cookies: cookies}, key) when is_binary(key) do
    Map.get(cookies, key)
  end

  @doc """
  Retrieves the parsed request body.

  The body is typically parsed from JSON or form data, depending on
  the Content-Type header. The structure depends on what was sent.

  ## Parameters

    * `request` - The Request struct

  ## Returns

    The parsed body (usually a map for JSON), or `nil` if no body

  ## Examples

      iex> request = Spikard.Request.from_map(%{
      ...>   "body" => %{"name" => "John", "email" => "john@example.com"}
      ...> })
      iex> Spikard.Request.get_body(request)
      %{"name" => "John", "email" => "john@example.com"}

      iex> request = Spikard.Request.from_map(%{"body" => nil})
      iex> Spikard.Request.get_body(request)
      nil
  """
  @spec get_body(t()) :: term()
  def get_body(%__MODULE__{body: body}) do
    body
  end

  @doc """
  Retrieves the raw request body as binary.

  Returns the unparsed request body as a binary. Useful for streaming,
  binary data, or handling non-standard content types.

  ## Parameters

    * `request` - The Request struct

  ## Returns

    The raw body as binary, or `nil` if no body

  ## Examples

      iex> request = Spikard.Request.from_map(%{
      ...>   "raw_body" => "{\"name\": \"John\"}"
      ...> })
      iex> Spikard.Request.get_raw_body(request)
      "{\"name\": \"John\"}"

      iex> request = Spikard.Request.from_map(%{"raw_body" => nil})
      iex> Spikard.Request.get_raw_body(request)
      nil
  """
  @spec get_raw_body(t()) :: binary() | nil
  def get_raw_body(%__MODULE__{raw_body: raw_body}) do
    raw_body
  end

  @doc """
  Retrieves a dependency from the request by key.

  Dependencies are injected into requests at the server level and are available
  to all handlers. This function retrieves a dependency by its key.

  ## Parameters

    * `request` - The Request struct
    * `key` - The dependency key as a string

  ## Returns

    The dependency value, or `nil` if not found or dependencies are not configured

  ## Examples

      iex> request = %Spikard.Request{
      ...>   dependencies: %{"db" => %{host: "localhost"}},
      ...>   path_params: %{},
      ...>   query_params: %{},
      ...>   raw_query_params: %{},
      ...>   headers: %{},
      ...>   cookies: %{},
      ...>   body: nil,
      ...>   raw_body: nil,
      ...>   method: "GET",
      ...>   path: "/",
      ...>   validated_params: nil
      ...> }
      iex> Spikard.Request.get_dependency(request, "db")
      %{host: "localhost"}

      iex> Spikard.Request.get_dependency(request, "cache")
      nil
  """
  @spec get_dependency(t(), String.t()) :: term()
  def get_dependency(%__MODULE__{dependencies: deps}, key) when is_binary(key) do
    case deps do
      nil -> nil
      deps when is_map(deps) -> Map.get(deps, key)
      _ -> nil
    end
  end

  @doc """
  Retrieves all uploaded files from the request.

  Files are parsed from multipart/form-data requests. Each file contains
  metadata (filename, content type, size) and the file content as binary.

  ## Parameters

    * `request` - The Request struct

  ## Returns

    A list of Spikard.UploadFile structs. Returns an empty list if no files
    were uploaded.

  ## Examples

      iex> request = Spikard.Request.from_map(%{
      ...>   "files" => [
      ...>     %{"filename" => "test.txt", "content_type" => "text/plain", "size" => 5, "content" => "hello"}
      ...>   ]
      ...> })
      iex> files = Spikard.Request.files(request)
      iex> length(files)
      1
      iex> hd(files).filename
      "test.txt"

      iex> request = Spikard.Request.from_map(%{})
      iex> Spikard.Request.files(request)
      []
  """
  @spec files(t()) :: [Spikard.UploadFile.t()]
  def files(%__MODULE__{files: files}) do
    files
  end

  # Private helper to ensure maps are returned consistently
  @spec ensure_map(term()) :: map()
  defp ensure_map(value) when is_map(value), do: value
  defp ensure_map(nil), do: %{}
  defp ensure_map(_other), do: %{}

  # Private helper to ensure lists are returned consistently
  @spec ensure_list(term()) :: list()
  defp ensure_list(value) when is_list(value), do: value
  defp ensure_list(_), do: []

  # Private helper to parse file data from request
  @spec parse_files(list()) :: [Spikard.UploadFile.t()]
  defp parse_files(file_list) when is_list(file_list) do
    Enum.map(file_list, &parse_single_file/1)
  end

  # Parse a single file object from the request data
  @spec parse_single_file(map() | any()) :: Spikard.UploadFile.t()
  defp parse_single_file(file_data) when is_map(file_data) do
    filename = Map.get(file_data, "filename")
    content_type = Map.get(file_data, "content_type", "application/octet-stream")
    content = Map.get(file_data, "content", "")

    # Handle size - could be provided or calculated from content
    size =
      case Map.get(file_data, "size") do
        nil ->
          if is_binary(content), do: byte_size(content), else: 0
        s when is_integer(s) -> s
        _ -> 0
      end

    # Convert content to binary if needed
    data =
      case content do
        b when is_binary(b) -> b
        l when is_list(l) ->
          try do
            Enum.map(l, &Integer.to_string/1) |> Enum.join("") |> String.to_charlist() |> List.to_string()
          rescue
            _ -> ""
          end
        _ -> ""
      end

    Spikard.UploadFile.new(filename, content_type, size, data)
  end

  defp parse_single_file(_), do: Spikard.UploadFile.new(nil, "application/octet-stream", 0, "")
end

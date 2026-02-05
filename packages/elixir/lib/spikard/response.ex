defmodule Spikard.Response do
  @moduledoc """
  Response builders for Spikard handlers.

  This module provides a fluent API for building HTTP responses. Responses can be
  created with convenience functions like `json/2`, `text/2`, `html/2`, or built
  incrementally using the `new/0` and builder functions.

  ## Examples

  Simple JSON response:

      Response.json(%{hello: "world"})
      #=> %{status: 200, headers: [{"content-type", "application/json"}], body: "{\\"hello\\":\\"world\\"}"}

  Building a response with headers:

      Response.new()
      |> Response.with_status(201)
      |> Response.with_header("x-request-id", "abc123")
      |> Response.with_json(%{created: true})

  Setting cookies:

      Response.json(%{logged_in: true})
      |> Response.with_cookie("session_id", "xyz789", max_age: 3600, http_only: true)
  """

  @type header :: {String.t(), String.t()}
  @type headers :: [header()] | %{String.t() => String.t()}
  @type t :: %{
          status: non_neg_integer(),
          headers: [header()],
          body: binary() | nil
        }

  @doc """
  Creates a new empty response with status 200.

  ## Examples

      Response.new()
      #=> %{status: 200, headers: [], body: nil}
  """
  @spec new() :: t()
  def new do
    %{status: 200, headers: [], body: nil}
  end

  @doc """
  Creates a JSON response.

  ## Options

  - `:status` - HTTP status code (default: 200)

  ## Examples

      Response.json(%{hello: "world"})
      #=> %{status: 200, headers: [{"content-type", "application/json"}], body: "{\\"hello\\":\\"world\\"}"}

      Response.json(%{error: "not found"}, status: 404)
      #=> %{status: 404, headers: [{"content-type", "application/json"}], ...}
  """
  @spec json(term(), keyword()) :: t()
  def json(data, opts \\ []) do
    status = Keyword.get(opts, :status, 200)
    body = Jason.encode!(data)

    %{
      status: status,
      headers: [{"content-type", "application/json"}],
      body: body
    }
  end

  @doc """
  Creates a plain text response.

  ## Options

  - `:status` - HTTP status code (default: 200)

  ## Examples

      Response.text("Hello, World!")
      #=> %{status: 200, headers: [{"content-type", "text/plain; charset=utf-8"}], body: "Hello, World!"}
  """
  @spec text(String.t(), keyword()) :: t()
  def text(content, opts \\ []) do
    status = Keyword.get(opts, :status, 200)

    %{
      status: status,
      headers: [{"content-type", "text/plain; charset=utf-8"}],
      body: content
    }
  end

  @doc """
  Creates an HTML response.

  ## Options

  - `:status` - HTTP status code (default: 200)

  ## Examples

      Response.html("<h1>Hello</h1>")
      #=> %{status: 200, headers: [{"content-type", "text/html; charset=utf-8"}], body: "<h1>Hello</h1>"}
  """
  @spec html(String.t(), keyword()) :: t()
  def html(content, opts \\ []) do
    status = Keyword.get(opts, :status, 200)

    %{
      status: status,
      headers: [{"content-type", "text/html; charset=utf-8"}],
      body: content
    }
  end

  @doc """
  Creates a response with a specific status code and no body.

  ## Examples

      Response.status(204)
      #=> %{status: 204, headers: [], body: nil}
  """
  @spec status(non_neg_integer()) :: t()
  def status(code) do
    %{
      status: code,
      headers: [],
      body: nil
    }
  end

  @doc """
  Sets the status code on an existing response.

  ## Examples

      Response.new()
      |> Response.with_status(201)
      #=> %{status: 201, headers: [], body: nil}
  """
  @spec with_status(t(), non_neg_integer()) :: t()
  def with_status(response, status_code) do
    %{response | status: status_code}
  end

  @doc """
  Adds a single header to the response.

  Header names are normalized to lowercase.

  ## Examples

      Response.json(%{})
      |> Response.with_header("x-request-id", "abc123")
  """
  @spec with_header(t(), String.t(), String.t()) :: t()
  def with_header(response, name, value) do
    normalized_name = String.downcase(name)
    %{response | headers: [{normalized_name, value} | response.headers]}
  end

  @doc """
  Adds multiple headers to the response.

  Accepts either a list of tuples or a map. Header names are normalized to lowercase.

  ## Examples

      Response.json(%{})
      |> Response.with_headers([{"x-request-id", "abc"}, {"x-trace-id", "xyz"}])

      Response.json(%{})
      |> Response.with_headers(%{"X-Request-Id" => "abc", "X-Trace-Id" => "xyz"})
  """
  @spec with_headers(t(), headers()) :: t()
  def with_headers(response, headers) when is_list(headers) do
    normalized =
      Enum.map(headers, fn {name, value} ->
        {String.downcase(to_string(name)), to_string(value)}
      end)

    %{response | headers: normalized ++ response.headers}
  end

  def with_headers(response, headers) when is_map(headers) do
    with_headers(response, Map.to_list(headers))
  end

  @doc """
  Sets the response body as JSON.

  This is useful for building responses incrementally.

  ## Examples

      Response.new()
      |> Response.with_status(201)
      |> Response.with_header("x-custom", "value")
      |> Response.with_json(%{created: true})
  """
  @spec with_json(t(), term()) :: t()
  def with_json(response, data) do
    body = Jason.encode!(data)

    response
    |> with_header("content-type", "application/json")
    |> Map.put(:body, body)
  end

  @doc """
  Sets the response body as plain text.

  ## Examples

      Response.new()
      |> Response.with_status(200)
      |> Response.with_text("Hello, World!")
  """
  @spec with_text(t(), String.t()) :: t()
  def with_text(response, content) do
    response
    |> with_header("content-type", "text/plain; charset=utf-8")
    |> Map.put(:body, content)
  end

  @doc """
  Sets the response body as HTML.

  ## Examples

      Response.new()
      |> Response.with_html("<h1>Welcome</h1>")
  """
  @spec with_html(t(), String.t()) :: t()
  def with_html(response, content) do
    response
    |> with_header("content-type", "text/html; charset=utf-8")
    |> Map.put(:body, content)
  end

  @doc """
  Adds a Set-Cookie header to the response.

  ## Options

  - `:max_age` - Cookie max age in seconds
  - `:expires` - Expiration date (DateTime or string)
  - `:domain` - Cookie domain
  - `:path` - Cookie path (default: "/")
  - `:secure` - Secure flag (boolean)
  - `:http_only` - HttpOnly flag (boolean)
  - `:same_site` - SameSite policy ("Strict", "Lax", or "None")

  ## Examples

      Response.json(%{logged_in: true})
      |> Response.with_cookie("session", "abc123", max_age: 3600, http_only: true)

      Response.json(%{})
      |> Response.with_cookie("prefs", "dark", path: "/settings", same_site: "Strict")
  """
  @spec with_cookie(t(), String.t(), String.t(), keyword()) :: t()
  def with_cookie(response, name, value, opts \\ []) do
    cookie_value = build_cookie_string(name, value, opts)
    with_header(response, "set-cookie", cookie_value)
  end

  @doc """
  Creates a redirect response.

  ## Options

  - `:status` - HTTP status code (default: 302 for temporary redirect)

  ## Examples

      Response.redirect("/login")
      #=> %{status: 302, headers: [{"location", "/login"}], body: nil}

      Response.redirect("/new-page", status: 301)
      #=> %{status: 301, headers: [{"location", "/new-page"}], body: nil}
  """
  @spec redirect(String.t(), keyword()) :: t()
  def redirect(location, opts \\ []) do
    status_code = Keyword.get(opts, :status, 302)

    %{
      status: status_code,
      headers: [{"location", location}],
      body: nil
    }
  end

  # Private helpers

  defp build_cookie_string(name, value, opts) do
    base = "#{name}=#{value}"

    parts =
      [
        base,
        cookie_part(:max_age, opts),
        cookie_part(:expires, opts),
        cookie_part(:domain, opts),
        cookie_part(:path, opts),
        cookie_part(:secure, opts),
        cookie_part(:http_only, opts),
        cookie_part(:same_site, opts)
      ]
      |> Enum.reject(&is_nil/1)

    Enum.join(parts, "; ")
  end

  defp cookie_part(:max_age, opts) do
    case Keyword.get(opts, :max_age) do
      nil -> nil
      age -> "Max-Age=#{age}"
    end
  end

  defp cookie_part(:expires, opts) do
    case Keyword.get(opts, :expires) do
      nil -> nil
      %DateTime{} = dt -> "Expires=#{Calendar.strftime(dt, "%a, %d %b %Y %H:%M:%S GMT")}"
      str when is_binary(str) -> "Expires=#{str}"
    end
  end

  defp cookie_part(:domain, opts) do
    case Keyword.get(opts, :domain) do
      nil -> nil
      domain -> "Domain=#{domain}"
    end
  end

  defp cookie_part(:path, opts) do
    case Keyword.get(opts, :path) do
      nil -> nil
      path -> "Path=#{path}"
    end
  end

  defp cookie_part(:secure, opts) do
    if Keyword.get(opts, :secure), do: "Secure", else: nil
  end

  defp cookie_part(:http_only, opts) do
    if Keyword.get(opts, :http_only), do: "HttpOnly", else: nil
  end

  defp cookie_part(:same_site, opts) do
    case Keyword.get(opts, :same_site) do
      nil -> nil
      policy -> "SameSite=#{policy}"
    end
  end

  @doc """
  Creates a streaming response from an Enumerable/Stream.

  This is a convenience function that delegates to `Spikard.Stream.stream/2`.
  The stream should produce binary chunks that will be sent to the client.
  Each element yielded by the stream becomes a chunk in the response body.

  ## Options

  - `:status` - HTTP status code (default: 200)
  - `:content_type` - Content-Type header (default: "application/octet-stream")

  ## Examples

      stream = Stream.map(1..100, &Integer.to_string/1)
      Response.stream(stream, content_type: "text/plain")

      stream = Stream.map(data, &Jason.encode!/1)
      Response.stream(stream, content_type: "application/x-ndjson", status: 200)
  """
  @spec stream(Enumerable.t(), keyword()) :: t()
  def stream(enumerable, opts \\ []) do
    Spikard.Stream.stream(enumerable, opts)
  end
end

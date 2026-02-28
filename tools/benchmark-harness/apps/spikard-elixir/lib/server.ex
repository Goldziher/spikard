defmodule BenchServer.Handlers do
  @moduledoc false

  # ============================================================================
  # Health
  # ============================================================================

  def health(_request) do
    Spikard.Response.json(%{status: "ok"})
  end

  # ============================================================================
  # Raw JSON body endpoints
  # ============================================================================

  def json_small(request) do
    Spikard.Response.json(Spikard.Request.get_body(request))
  end

  def json_medium(request) do
    Spikard.Response.json(Spikard.Request.get_body(request))
  end

  def json_large(request) do
    Spikard.Response.json(Spikard.Request.get_body(request))
  end

  def json_very_large(request) do
    Spikard.Response.json(Spikard.Request.get_body(request))
  end

  # ============================================================================
  # Raw multipart endpoints
  # ============================================================================

  def multipart_small(request) do
    count_files(request)
  end

  def multipart_medium(request) do
    count_files(request)
  end

  def multipart_large(request) do
    count_files(request)
  end

  defp count_files(request) do
    files = Spikard.Request.files(request)

    {files_received, total_bytes} =
      Enum.reduce(files, {0, 0}, fn file, {count, bytes} ->
        size = Map.get(file, :size, 0)
        {count + 1, bytes + size}
      end)

    Spikard.Response.json(%{files_received: files_received, total_bytes: total_bytes})
  end

  # ============================================================================
  # Raw URL-encoded endpoints
  # ============================================================================

  def urlencoded_simple(request) do
    Spikard.Response.json(Spikard.Request.get_body(request))
  end

  def urlencoded_complex(request) do
    Spikard.Response.json(Spikard.Request.get_body(request))
  end

  # ============================================================================
  # Raw path parameter endpoints
  # ============================================================================

  def path_simple(request) do
    id = Spikard.Request.get_path_param(request, "id")
    Spikard.Response.json(%{id: id})
  end

  def path_multiple(request) do
    user_id = Spikard.Request.get_path_param(request, "user_id")
    post_id = Spikard.Request.get_path_param(request, "post_id")
    Spikard.Response.json(%{user_id: user_id, post_id: post_id})
  end

  def path_deep(request) do
    Spikard.Response.json(%{
      org: Spikard.Request.get_path_param(request, "org"),
      team: Spikard.Request.get_path_param(request, "team"),
      project: Spikard.Request.get_path_param(request, "project"),
      resource: Spikard.Request.get_path_param(request, "resource"),
      id: Spikard.Request.get_path_param(request, "id")
    })
  end

  def path_int(request) do
    id = Spikard.Request.get_path_param(request, "id")
    Spikard.Response.json(%{id: parse_int(id)})
  end

  def path_uuid(request) do
    uuid = Spikard.Request.get_path_param(request, "uuid")
    Spikard.Response.json(%{uuid: uuid})
  end

  def path_date(request) do
    date = Spikard.Request.get_path_param(request, "date")
    Spikard.Response.json(%{date: date})
  end

  # ============================================================================
  # Raw query parameter endpoints
  # ============================================================================

  def query_few(request) do
    params = request.query_params || %{}

    result =
      %{}
      |> maybe_put("q", params["q"])
      |> maybe_put_int("page", params["page"])
      |> maybe_put_int("limit", params["limit"])

    Spikard.Response.json(result)
  end

  def query_medium(request) do
    params = request.query_params || %{}

    result =
      %{}
      |> maybe_put("search", params["search"])
      |> maybe_put("category", params["category"])
      |> maybe_put("sort", params["sort"])
      |> maybe_put("order", params["order"])
      |> maybe_put_int("page", params["page"])
      |> maybe_put_int("limit", params["limit"])
      |> maybe_put("filter", params["filter"])

    Spikard.Response.json(result)
  end

  def query_many(request) do
    params = request.query_params || %{}

    result =
      %{}
      |> maybe_put("q", params["q"])
      |> maybe_put("category", params["category"])
      |> maybe_put("subcategory", params["subcategory"])
      |> maybe_put("brand", params["brand"])
      |> maybe_put_float("min_price", params["min_price"])
      |> maybe_put_float("max_price", params["max_price"])
      |> maybe_put("color", params["color"])
      |> maybe_put("size", params["size"])
      |> maybe_put("material", params["material"])
      |> maybe_put_int("rating", params["rating"])
      |> maybe_put("sort", params["sort"])
      |> maybe_put("order", params["order"])
      |> maybe_put_int("page", params["page"])
      |> maybe_put_int("limit", params["limit"])
      |> maybe_put_bool("in_stock", params["in_stock"])
      |> maybe_put_bool("on_sale", params["on_sale"])

    Spikard.Response.json(result)
  end

  # ============================================================================
  # Validated JSON body endpoints
  # ============================================================================

  def validated_json_small(request) do
    Spikard.Response.json(Spikard.Request.get_body(request))
  end

  def validated_json_medium(request) do
    Spikard.Response.json(Spikard.Request.get_body(request))
  end

  def validated_json_large(request) do
    Spikard.Response.json(Spikard.Request.get_body(request))
  end

  def validated_json_very_large(request) do
    Spikard.Response.json(Spikard.Request.get_body(request))
  end

  # ============================================================================
  # Validated multipart endpoints
  # ============================================================================

  def validated_multipart_small(request) do
    validated_count_files(request)
  end

  def validated_multipart_medium(request) do
    validated_count_files(request)
  end

  def validated_multipart_large(request) do
    validated_count_files(request)
  end

  defp validated_count_files(request) do
    files = Spikard.Request.files(request)

    {files_received, total_bytes} =
      Enum.reduce(files, {0, 0}, fn file, {count, bytes} ->
        size = Map.get(file, :size, 0)
        {count + 1, bytes + size}
      end)

    if files_received == 0 do
      Spikard.Response.json(%{error: "No files uploaded"}, status: 400)
    else
      Spikard.Response.json(%{files_received: files_received, total_bytes: total_bytes})
    end
  end

  # ============================================================================
  # Validated URL-encoded endpoints
  # ============================================================================

  def validated_urlencoded_simple(request) do
    body = Spikard.Request.get_body(request)
    Spikard.Response.json(coerce_urlencoded_simple(body))
  end

  def validated_urlencoded_complex(request) do
    body = Spikard.Request.get_body(request)
    Spikard.Response.json(coerce_urlencoded_complex(body))
  end

  defp coerce_urlencoded_simple(body) when is_map(body) do
    body
    |> coerce_key_int("age")
    |> coerce_key_bool("subscribe")
  end

  defp coerce_urlencoded_simple(body), do: body

  defp coerce_urlencoded_complex(body) when is_map(body) do
    body
    |> coerce_key_int("age")
    |> coerce_key_bool("subscribe")
    |> coerce_key_bool("newsletter")
    |> coerce_key_bool("terms_accepted")
    |> coerce_key_bool("privacy_accepted")
    |> coerce_key_bool("marketing_consent")
    |> coerce_key_bool("two_factor_enabled")
  end

  defp coerce_urlencoded_complex(body), do: body

  # ============================================================================
  # Validated path parameter endpoints
  # ============================================================================

  def validated_path_simple(request) do
    id = Spikard.Request.get_path_param(request, "id")

    if valid_path_param?(id) do
      Spikard.Response.json(%{id: id})
    else
      Spikard.Response.json(%{error: "Invalid path parameter format"}, status: 400)
    end
  end

  def validated_path_multiple(request) do
    user_id = Spikard.Request.get_path_param(request, "user_id")
    post_id = Spikard.Request.get_path_param(request, "post_id")

    cond do
      not valid_path_param?(user_id) ->
        Spikard.Response.json(%{error: "Invalid path parameter format"}, status: 400)

      not valid_path_param?(post_id) ->
        Spikard.Response.json(%{error: "Invalid path parameter format"}, status: 400)

      true ->
        Spikard.Response.json(%{user_id: user_id, post_id: post_id})
    end
  end

  def validated_path_deep(request) do
    params = %{
      org: Spikard.Request.get_path_param(request, "org"),
      team: Spikard.Request.get_path_param(request, "team"),
      project: Spikard.Request.get_path_param(request, "project"),
      resource: Spikard.Request.get_path_param(request, "resource"),
      id: Spikard.Request.get_path_param(request, "id")
    }

    invalid =
      Enum.find(params, fn {_key, val} -> not valid_path_param?(val) end)

    case invalid do
      nil ->
        Spikard.Response.json(params)

      {key, _val} ->
        Spikard.Response.json(%{error: "Invalid path parameter: #{key}"}, status: 400)
    end
  end

  def validated_path_int(request) do
    id = Spikard.Request.get_path_param(request, "id")
    Spikard.Response.json(%{id: parse_int(id)})
  end

  def validated_path_uuid(request) do
    uuid = Spikard.Request.get_path_param(request, "uuid")

    if Regex.match?(~r/\A[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\z/i, uuid || "") do
      Spikard.Response.json(%{uuid: uuid})
    else
      Spikard.Response.json(%{error: "Invalid UUID format"}, status: 400)
    end
  end

  def validated_path_date(request) do
    date_str = Spikard.Request.get_path_param(request, "date")

    case Date.from_iso8601(date_str || "") do
      {:ok, _date} ->
        Spikard.Response.json(%{date: date_str})

      {:error, _} ->
        Spikard.Response.json(%{error: "Invalid date format"}, status: 400)
    end
  end

  # ============================================================================
  # Validated query parameter endpoints
  # ============================================================================

  def validated_query_few(request) do
    params = request.query_params || %{}

    case params["q"] do
      nil ->
        Spikard.Response.json(%{errors: %{q: ["is missing"]}}, status: 400)

      _ ->
        result =
          %{q: params["q"]}
          |> maybe_put_int("page", params["page"])
          |> maybe_put_int("limit", params["limit"])

        Spikard.Response.json(result)
    end
  end

  def validated_query_medium(request) do
    params = request.query_params || %{}

    case params["search"] do
      nil ->
        Spikard.Response.json(%{errors: %{search: ["is missing"]}}, status: 400)

      _ ->
        result =
          %{search: params["search"]}
          |> maybe_put("category", params["category"])
          |> maybe_put("sort", params["sort"])
          |> maybe_put("order", params["order"])
          |> maybe_put_int("page", params["page"])
          |> maybe_put_int("limit", params["limit"])
          |> maybe_put("filter", params["filter"])

        Spikard.Response.json(result)
    end
  end

  def validated_query_many(request) do
    params = request.query_params || %{}

    case params["q"] do
      nil ->
        Spikard.Response.json(%{errors: %{q: ["is missing"]}}, status: 400)

      _ ->
        result =
          %{q: params["q"]}
          |> maybe_put("category", params["category"])
          |> maybe_put("subcategory", params["subcategory"])
          |> maybe_put("brand", params["brand"])
          |> maybe_put_float("min_price", params["min_price"])
          |> maybe_put_float("max_price", params["max_price"])
          |> maybe_put("color", params["color"])
          |> maybe_put("size", params["size"])
          |> maybe_put("material", params["material"])
          |> maybe_put_int("rating", params["rating"])
          |> maybe_put("sort", params["sort"])
          |> maybe_put("order", params["order"])
          |> maybe_put_int("page", params["page"])
          |> maybe_put_int("limit", params["limit"])
          |> maybe_put_bool("in_stock", params["in_stock"])
          |> maybe_put_bool("on_sale", params["on_sale"])

        Spikard.Response.json(result)
    end
  end

  # ============================================================================
  # Helpers
  # ============================================================================

  defp maybe_put(map, _key, nil), do: map
  defp maybe_put(map, key, val), do: Map.put(map, String.to_atom(key), val)

  defp maybe_put_int(map, _key, nil), do: map

  defp maybe_put_int(map, key, val) do
    Map.put(map, String.to_atom(key), parse_int(val))
  end

  defp maybe_put_float(map, _key, nil), do: map

  defp maybe_put_float(map, key, val) do
    Map.put(map, String.to_atom(key), parse_float(val))
  end

  defp maybe_put_bool(map, _key, nil), do: map

  defp maybe_put_bool(map, key, val) do
    Map.put(map, String.to_atom(key), parse_bool(val))
  end

  defp parse_int(val) when is_integer(val), do: val
  defp parse_int(val) when is_binary(val), do: String.to_integer(val)
  defp parse_int(val), do: val

  defp parse_float(val) when is_float(val), do: val
  defp parse_float(val) when is_integer(val), do: val / 1
  defp parse_float(val) when is_binary(val), do: String.to_float(val)
  defp parse_float(val), do: val

  defp parse_bool(val) when is_boolean(val), do: val
  defp parse_bool("true"), do: true
  defp parse_bool("1"), do: true
  defp parse_bool("yes"), do: true
  defp parse_bool("on"), do: true
  defp parse_bool("false"), do: false
  defp parse_bool("0"), do: false
  defp parse_bool("no"), do: false
  defp parse_bool("off"), do: false
  defp parse_bool(val), do: val

  defp valid_path_param?(nil), do: false
  defp valid_path_param?(""), do: false

  defp valid_path_param?(val) when is_binary(val) do
    String.length(val) <= 255 and Regex.match?(~r/\A[a-zA-Z0-9_-]+\z/, val)
  end

  defp valid_path_param?(_), do: false

  defp coerce_key_int(map, key) do
    case Map.get(map, key) do
      nil -> map
      val -> Map.put(map, key, parse_int(val))
    end
  end

  defp coerce_key_bool(map, key) do
    case Map.get(map, key) do
      nil -> map
      val -> Map.put(map, key, parse_bool(val))
    end
  end
end

defmodule BenchServer.Router do
  @moduledoc false
  use Spikard.Router

  get "/health", &BenchServer.Handlers.health/1

  # Raw JSON body endpoints
  post "/json/small", &BenchServer.Handlers.json_small/1
  post "/json/medium", &BenchServer.Handlers.json_medium/1
  post "/json/large", &BenchServer.Handlers.json_large/1
  post "/json/very-large", &BenchServer.Handlers.json_very_large/1

  # Raw multipart endpoints
  post "/multipart/small", &BenchServer.Handlers.multipart_small/1
  post "/multipart/medium", &BenchServer.Handlers.multipart_medium/1
  post "/multipart/large", &BenchServer.Handlers.multipart_large/1

  # Raw URL-encoded endpoints
  post "/urlencoded/simple", &BenchServer.Handlers.urlencoded_simple/1
  post "/urlencoded/complex", &BenchServer.Handlers.urlencoded_complex/1

  # Raw path parameter endpoints
  get "/path/simple/{id}", &BenchServer.Handlers.path_simple/1
  get "/path/multiple/{user_id}/{post_id}", &BenchServer.Handlers.path_multiple/1
  get "/path/deep/{org}/{team}/{project}/{resource}/{id}", &BenchServer.Handlers.path_deep/1
  get "/path/int/{id}", &BenchServer.Handlers.path_int/1
  get "/path/uuid/{uuid}", &BenchServer.Handlers.path_uuid/1
  get "/path/date/{date}", &BenchServer.Handlers.path_date/1

  # Raw query parameter endpoints
  get "/query/few", &BenchServer.Handlers.query_few/1
  get "/query/medium", &BenchServer.Handlers.query_medium/1
  get "/query/many", &BenchServer.Handlers.query_many/1

  # Validated JSON body endpoints
  post "/validated/json/small", &BenchServer.Handlers.validated_json_small/1
  post "/validated/json/medium", &BenchServer.Handlers.validated_json_medium/1
  post "/validated/json/large", &BenchServer.Handlers.validated_json_large/1
  post "/validated/json/very-large", &BenchServer.Handlers.validated_json_very_large/1

  # Validated multipart endpoints
  post "/validated/multipart/small", &BenchServer.Handlers.validated_multipart_small/1
  post "/validated/multipart/medium", &BenchServer.Handlers.validated_multipart_medium/1
  post "/validated/multipart/large", &BenchServer.Handlers.validated_multipart_large/1

  # Validated URL-encoded endpoints
  post "/validated/urlencoded/simple", &BenchServer.Handlers.validated_urlencoded_simple/1
  post "/validated/urlencoded/complex", &BenchServer.Handlers.validated_urlencoded_complex/1

  # Validated path parameter endpoints
  get "/validated/path/simple/{id}", &BenchServer.Handlers.validated_path_simple/1
  get "/validated/path/multiple/{user_id}/{post_id}", &BenchServer.Handlers.validated_path_multiple/1
  get "/validated/path/deep/{org}/{team}/{project}/{resource}/{id}", &BenchServer.Handlers.validated_path_deep/1
  get "/validated/path/int/{id}", &BenchServer.Handlers.validated_path_int/1
  get "/validated/path/uuid/{uuid}", &BenchServer.Handlers.validated_path_uuid/1
  get "/validated/path/date/{date}", &BenchServer.Handlers.validated_path_date/1

  # Validated query parameter endpoints
  get "/validated/query/few", &BenchServer.Handlers.validated_query_few/1
  get "/validated/query/medium", &BenchServer.Handlers.validated_query_medium/1
  get "/validated/query/many", &BenchServer.Handlers.validated_query_many/1
end

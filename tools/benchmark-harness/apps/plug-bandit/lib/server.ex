defmodule BenchServer.Router do
  @moduledoc false
  use Plug.Router

  plug Plug.Parsers,
    parsers: [:json, :urlencoded, :multipart],
    json_decoder: Jason,
    pass: ["*/*"],
    length: 10_000_000

  plug :match
  plug :dispatch

  # ============================================================================
  # Health
  # ============================================================================

  get "/health" do
    json_response(conn, 200, %{status: "ok"})
  end

  # ============================================================================
  # Raw JSON body endpoints
  # ============================================================================

  post "/json/small" do
    json_response(conn, 200, conn.body_params)
  end

  post "/json/medium" do
    json_response(conn, 200, conn.body_params)
  end

  post "/json/large" do
    json_response(conn, 200, conn.body_params)
  end

  post "/json/very-large" do
    json_response(conn, 200, conn.body_params)
  end

  # ============================================================================
  # Raw multipart endpoints
  # ============================================================================

  post "/multipart/small" do
    multipart_response(conn)
  end

  post "/multipart/medium" do
    multipart_response(conn)
  end

  post "/multipart/large" do
    multipart_response(conn)
  end

  # ============================================================================
  # Raw URL-encoded endpoints
  # ============================================================================

  post "/urlencoded/simple" do
    json_response(conn, 200, conn.body_params)
  end

  post "/urlencoded/complex" do
    json_response(conn, 200, conn.body_params)
  end

  # ============================================================================
  # Raw path parameter endpoints
  # ============================================================================

  get "/path/simple/:id" do
    json_response(conn, 200, %{id: conn.path_params["id"]})
  end

  get "/path/multiple/:user_id/:post_id" do
    json_response(conn, 200, %{
      user_id: conn.path_params["user_id"],
      post_id: conn.path_params["post_id"]
    })
  end

  get "/path/deep/:org/:team/:project/:resource/:id" do
    json_response(conn, 200, %{
      org: conn.path_params["org"],
      team: conn.path_params["team"],
      project: conn.path_params["project"],
      resource: conn.path_params["resource"],
      id: conn.path_params["id"]
    })
  end

  get "/path/int/:id" do
    json_response(conn, 200, %{id: String.to_integer(conn.path_params["id"])})
  end

  get "/path/uuid/:uuid" do
    json_response(conn, 200, %{uuid: conn.path_params["uuid"]})
  end

  get "/path/date/:date" do
    json_response(conn, 200, %{date: conn.path_params["date"]})
  end

  # ============================================================================
  # Raw query parameter endpoints
  # ============================================================================

  get "/query/few" do
    params = fetch_query_params(conn).query_params

    result =
      %{}
      |> maybe_put(:q, params["q"])
      |> maybe_put_int(:page, params["page"])
      |> maybe_put_int(:limit, params["limit"])

    json_response(conn, 200, result)
  end

  get "/query/medium" do
    params = fetch_query_params(conn).query_params

    result =
      %{}
      |> maybe_put(:search, params["search"])
      |> maybe_put(:category, params["category"])
      |> maybe_put(:sort, params["sort"])
      |> maybe_put(:order, params["order"])
      |> maybe_put_int(:page, params["page"])
      |> maybe_put_int(:limit, params["limit"])
      |> maybe_put(:filter, params["filter"])

    json_response(conn, 200, result)
  end

  get "/query/many" do
    params = fetch_query_params(conn).query_params

    result =
      %{}
      |> maybe_put(:q, params["q"])
      |> maybe_put(:category, params["category"])
      |> maybe_put(:subcategory, params["subcategory"])
      |> maybe_put(:brand, params["brand"])
      |> maybe_put_float(:min_price, params["min_price"])
      |> maybe_put_float(:max_price, params["max_price"])
      |> maybe_put(:color, params["color"])
      |> maybe_put(:size, params["size"])
      |> maybe_put(:material, params["material"])
      |> maybe_put_int(:rating, params["rating"])
      |> maybe_put(:sort, params["sort"])
      |> maybe_put(:order, params["order"])
      |> maybe_put_int(:page, params["page"])
      |> maybe_put_int(:limit, params["limit"])
      |> maybe_put_bool(:in_stock, params["in_stock"])
      |> maybe_put_bool(:on_sale, params["on_sale"])

    json_response(conn, 200, result)
  end

  # ============================================================================
  # Validated JSON body endpoints
  # ============================================================================

  post "/validated/json/small" do
    json_response(conn, 200, conn.body_params)
  end

  post "/validated/json/medium" do
    json_response(conn, 200, conn.body_params)
  end

  post "/validated/json/large" do
    json_response(conn, 200, conn.body_params)
  end

  post "/validated/json/very-large" do
    json_response(conn, 200, conn.body_params)
  end

  # ============================================================================
  # Validated multipart endpoints
  # ============================================================================

  post "/validated/multipart/small" do
    validated_multipart_response(conn)
  end

  post "/validated/multipart/medium" do
    validated_multipart_response(conn)
  end

  post "/validated/multipart/large" do
    validated_multipart_response(conn)
  end

  # ============================================================================
  # Validated URL-encoded endpoints
  # ============================================================================

  post "/validated/urlencoded/simple" do
    body = conn.body_params
    coerced = coerce_urlencoded_simple(body)
    json_response(conn, 200, coerced)
  end

  post "/validated/urlencoded/complex" do
    body = conn.body_params
    coerced = coerce_urlencoded_complex(body)
    json_response(conn, 200, coerced)
  end

  # ============================================================================
  # Validated path parameter endpoints
  # ============================================================================

  get "/validated/path/simple/:id" do
    id = conn.path_params["id"]

    if valid_path_param?(id) do
      json_response(conn, 200, %{id: id})
    else
      json_response(conn, 400, %{error: "Invalid path parameter format"})
    end
  end

  get "/validated/path/multiple/:user_id/:post_id" do
    user_id = conn.path_params["user_id"]
    post_id = conn.path_params["post_id"]

    cond do
      not valid_path_param?(user_id) ->
        json_response(conn, 400, %{error: "Invalid path parameter format"})

      not valid_path_param?(post_id) ->
        json_response(conn, 400, %{error: "Invalid path parameter format"})

      true ->
        json_response(conn, 200, %{user_id: user_id, post_id: post_id})
    end
  end

  get "/validated/path/deep/:org/:team/:project/:resource/:id" do
    params = %{
      org: conn.path_params["org"],
      team: conn.path_params["team"],
      project: conn.path_params["project"],
      resource: conn.path_params["resource"],
      id: conn.path_params["id"]
    }

    invalid = Enum.find(params, fn {_key, val} -> not valid_path_param?(val) end)

    case invalid do
      nil ->
        json_response(conn, 200, params)

      {key, _val} ->
        json_response(conn, 400, %{error: "Invalid path parameter: #{key}"})
    end
  end

  get "/validated/path/int/:id" do
    json_response(conn, 200, %{id: String.to_integer(conn.path_params["id"])})
  end

  get "/validated/path/uuid/:uuid" do
    uuid = conn.path_params["uuid"]

    if Regex.match?(~r/\A[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\z/i, uuid || "") do
      json_response(conn, 200, %{uuid: uuid})
    else
      json_response(conn, 400, %{error: "Invalid UUID format"})
    end
  end

  get "/validated/path/date/:date" do
    date_str = conn.path_params["date"]

    case Date.from_iso8601(date_str || "") do
      {:ok, _date} ->
        json_response(conn, 200, %{date: date_str})

      {:error, _} ->
        json_response(conn, 400, %{error: "Invalid date format"})
    end
  end

  # ============================================================================
  # Validated query parameter endpoints
  # ============================================================================

  get "/validated/query/few" do
    params = fetch_query_params(conn).query_params

    case params["q"] do
      nil ->
        json_response(conn, 400, %{errors: %{q: ["is missing"]}})

      _ ->
        result =
          %{q: params["q"]}
          |> maybe_put_int(:page, params["page"])
          |> maybe_put_int(:limit, params["limit"])

        json_response(conn, 200, result)
    end
  end

  get "/validated/query/medium" do
    params = fetch_query_params(conn).query_params

    case params["search"] do
      nil ->
        json_response(conn, 400, %{errors: %{search: ["is missing"]}})

      _ ->
        result =
          %{search: params["search"]}
          |> maybe_put(:category, params["category"])
          |> maybe_put(:sort, params["sort"])
          |> maybe_put(:order, params["order"])
          |> maybe_put_int(:page, params["page"])
          |> maybe_put_int(:limit, params["limit"])
          |> maybe_put(:filter, params["filter"])

        json_response(conn, 200, result)
    end
  end

  get "/validated/query/many" do
    params = fetch_query_params(conn).query_params

    case params["q"] do
      nil ->
        json_response(conn, 400, %{errors: %{q: ["is missing"]}})

      _ ->
        result =
          %{q: params["q"]}
          |> maybe_put(:category, params["category"])
          |> maybe_put(:subcategory, params["subcategory"])
          |> maybe_put(:brand, params["brand"])
          |> maybe_put_float(:min_price, params["min_price"])
          |> maybe_put_float(:max_price, params["max_price"])
          |> maybe_put(:color, params["color"])
          |> maybe_put(:size, params["size"])
          |> maybe_put(:material, params["material"])
          |> maybe_put_int(:rating, params["rating"])
          |> maybe_put(:sort, params["sort"])
          |> maybe_put(:order, params["order"])
          |> maybe_put_int(:page, params["page"])
          |> maybe_put_int(:limit, params["limit"])
          |> maybe_put_bool(:in_stock, params["in_stock"])
          |> maybe_put_bool(:on_sale, params["on_sale"])

        json_response(conn, 200, result)
    end
  end

  # ============================================================================
  # Catch-all
  # ============================================================================

  match _ do
    json_response(conn, 404, %{error: "not found"})
  end

  # ============================================================================
  # Helpers
  # ============================================================================

  defp json_response(conn, status, body) do
    conn
    |> put_resp_content_type("application/json")
    |> send_resp(status, Jason.encode!(body))
  end

  defp multipart_response(conn) do
    {files_received, total_bytes} = count_uploaded_files(conn)
    json_response(conn, 200, %{files_received: files_received, total_bytes: total_bytes})
  end

  defp validated_multipart_response(conn) do
    {files_received, total_bytes} = count_uploaded_files(conn)

    if files_received == 0 do
      json_response(conn, 400, %{error: "No files uploaded"})
    else
      json_response(conn, 200, %{files_received: files_received, total_bytes: total_bytes})
    end
  end

  defp count_uploaded_files(conn) do
    conn.body_params
    |> Enum.reduce({0, 0}, fn
      {_key, %Plug.Upload{path: path}}, {count, bytes} ->
        size = case File.stat(path) do
          {:ok, %{size: s}} -> s
          _ -> 0
        end
        {count + 1, bytes + size}

      _, acc ->
        acc
    end)
  end

  defp maybe_put(map, _key, nil), do: map
  defp maybe_put(map, key, val), do: Map.put(map, key, val)

  defp maybe_put_int(map, _key, nil), do: map
  defp maybe_put_int(map, key, val), do: Map.put(map, key, parse_int(val))

  defp maybe_put_float(map, _key, nil), do: map
  defp maybe_put_float(map, key, val), do: Map.put(map, key, parse_float(val))

  defp maybe_put_bool(map, _key, nil), do: map
  defp maybe_put_bool(map, key, val), do: Map.put(map, key, parse_bool(val))

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

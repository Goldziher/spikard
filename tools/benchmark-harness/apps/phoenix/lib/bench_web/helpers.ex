defmodule BenchWeb.Helpers do
  @moduledoc false

  def maybe_put(map, _key, nil), do: map
  def maybe_put(map, key, val), do: Map.put(map, key, val)

  def maybe_put_int(map, _key, nil), do: map
  def maybe_put_int(map, key, val), do: Map.put(map, key, parse_int(val))

  def maybe_put_float(map, _key, nil), do: map
  def maybe_put_float(map, key, val), do: Map.put(map, key, parse_float(val))

  def maybe_put_bool(map, _key, nil), do: map
  def maybe_put_bool(map, key, val), do: Map.put(map, key, parse_bool(val))

  def parse_int(val) when is_integer(val), do: val
  def parse_int(val) when is_binary(val), do: String.to_integer(val)
  def parse_int(val), do: val

  def parse_float(val) when is_float(val), do: val
  def parse_float(val) when is_integer(val), do: val / 1
  def parse_float(val) when is_binary(val), do: String.to_float(val)
  def parse_float(val), do: val

  def parse_bool(val) when is_boolean(val), do: val
  def parse_bool("true"), do: true
  def parse_bool("1"), do: true
  def parse_bool("yes"), do: true
  def parse_bool("on"), do: true
  def parse_bool("false"), do: false
  def parse_bool("0"), do: false
  def parse_bool("no"), do: false
  def parse_bool("off"), do: false
  def parse_bool(val), do: val

  def valid_path_param?(nil), do: false
  def valid_path_param?(""), do: false

  def valid_path_param?(val) when is_binary(val) do
    String.length(val) <= 255 and Regex.match?(~r/\A[a-zA-Z0-9_-]+\z/, val)
  end

  def valid_path_param?(_), do: false

  def coerce_urlencoded_simple(body) when is_map(body) do
    body
    |> coerce_key_int("age")
    |> coerce_key_bool("subscribe")
  end

  def coerce_urlencoded_simple(body), do: body

  def coerce_urlencoded_complex(body) when is_map(body) do
    body
    |> coerce_key_int("age")
    |> coerce_key_bool("subscribe")
    |> coerce_key_bool("newsletter")
    |> coerce_key_bool("terms_accepted")
    |> coerce_key_bool("privacy_accepted")
    |> coerce_key_bool("marketing_consent")
    |> coerce_key_bool("two_factor_enabled")
  end

  def coerce_urlencoded_complex(body), do: body

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

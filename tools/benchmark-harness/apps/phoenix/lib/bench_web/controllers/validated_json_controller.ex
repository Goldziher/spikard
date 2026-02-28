defmodule BenchWeb.ValidatedJsonController do
  use Phoenix.Controller, formats: [:json]

  def small(conn, params), do: json(conn, params)
  def medium(conn, params), do: json(conn, params)
  def large(conn, params), do: json(conn, params)
  def very_large(conn, params), do: json(conn, params)
end

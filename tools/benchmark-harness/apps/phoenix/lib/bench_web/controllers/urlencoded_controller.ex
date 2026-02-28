defmodule BenchWeb.UrlencodedController do
  use Phoenix.Controller, formats: [:json]

  def simple(conn, params), do: json(conn, params)
  def complex(conn, params), do: json(conn, params)
end

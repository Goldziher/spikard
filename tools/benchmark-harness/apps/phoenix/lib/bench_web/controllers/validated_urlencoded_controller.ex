defmodule BenchWeb.ValidatedUrlencodedController do
  use Phoenix.Controller, formats: [:json]
  import BenchWeb.Helpers

  def simple(conn, params) do
    json(conn, coerce_urlencoded_simple(params))
  end

  def complex(conn, params) do
    json(conn, coerce_urlencoded_complex(params))
  end
end

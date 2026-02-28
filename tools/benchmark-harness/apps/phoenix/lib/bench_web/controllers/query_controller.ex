defmodule BenchWeb.QueryController do
  use Phoenix.Controller, formats: [:json]
  import BenchWeb.Helpers

  def few(conn, params) do
    result =
      %{}
      |> maybe_put(:q, params["q"])
      |> maybe_put_int(:page, params["page"])
      |> maybe_put_int(:limit, params["limit"])

    json(conn, result)
  end

  def medium(conn, params) do
    result =
      %{}
      |> maybe_put(:search, params["search"])
      |> maybe_put(:category, params["category"])
      |> maybe_put(:sort, params["sort"])
      |> maybe_put(:order, params["order"])
      |> maybe_put_int(:page, params["page"])
      |> maybe_put_int(:limit, params["limit"])
      |> maybe_put(:filter, params["filter"])

    json(conn, result)
  end

  def many(conn, params) do
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

    json(conn, result)
  end
end

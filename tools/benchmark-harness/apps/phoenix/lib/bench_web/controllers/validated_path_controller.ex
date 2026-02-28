defmodule BenchWeb.ValidatedPathController do
  use Phoenix.Controller, formats: [:json]
  import BenchWeb.Helpers

  def simple(conn, %{"id" => id}) do
    if valid_path_param?(id) do
      json(conn, %{id: id})
    else
      conn |> put_status(400) |> json(%{error: "Invalid path parameter format"})
    end
  end

  def multiple(conn, %{"user_id" => user_id, "post_id" => post_id}) do
    cond do
      not valid_path_param?(user_id) ->
        conn |> put_status(400) |> json(%{error: "Invalid path parameter format"})

      not valid_path_param?(post_id) ->
        conn |> put_status(400) |> json(%{error: "Invalid path parameter format"})

      true ->
        json(conn, %{user_id: user_id, post_id: post_id})
    end
  end

  def deep(conn, %{"org" => org, "team" => team, "project" => project, "resource" => resource, "id" => id}) do
    params = %{org: org, team: team, project: project, resource: resource, id: id}

    invalid = Enum.find(params, fn {_key, val} -> not valid_path_param?(val) end)

    case invalid do
      nil ->
        json(conn, params)

      {key, _val} ->
        conn |> put_status(400) |> json(%{error: "Invalid path parameter: #{key}"})
    end
  end

  def int_param(conn, %{"id" => id}) do
    json(conn, %{id: String.to_integer(id)})
  end

  def uuid_param(conn, %{"uuid" => uuid}) do
    if Regex.match?(~r/\A[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\z/i, uuid || "") do
      json(conn, %{uuid: uuid})
    else
      conn |> put_status(400) |> json(%{error: "Invalid UUID format"})
    end
  end

  def date_param(conn, %{"date" => date_str}) do
    case Date.from_iso8601(date_str || "") do
      {:ok, _date} ->
        json(conn, %{date: date_str})

      {:error, _} ->
        conn |> put_status(400) |> json(%{error: "Invalid date format"})
    end
  end
end

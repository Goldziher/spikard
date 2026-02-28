defmodule BenchWeb.PathController do
  use Phoenix.Controller, formats: [:json]

  def simple(conn, %{"id" => id}) do
    json(conn, %{id: id})
  end

  def multiple(conn, %{"user_id" => user_id, "post_id" => post_id}) do
    json(conn, %{user_id: user_id, post_id: post_id})
  end

  def deep(conn, %{"org" => org, "team" => team, "project" => project, "resource" => resource, "id" => id}) do
    json(conn, %{org: org, team: team, project: project, resource: resource, id: id})
  end

  def int_param(conn, %{"id" => id}) do
    json(conn, %{id: String.to_integer(id)})
  end

  def uuid_param(conn, %{"uuid" => uuid}) do
    json(conn, %{uuid: uuid})
  end

  def date_param(conn, %{"date" => date}) do
    json(conn, %{date: date})
  end
end

defmodule SpikardTestApp.Router do
  @moduledoc "Minimal test app to verify published Hex package works."
  use Spikard.Router

  get "/", &index/1
  get "/health", &health/1
  post "/echo", &echo/1

  defp index(_request) do
    Spikard.Response.json(%{app: "spikard-test-app", language: "elixir"})
  end

  defp health(_request) do
    Spikard.Response.json(%{status: "ok"})
  end

  defp echo(request) do
    body = Spikard.Request.get_body(request)
    Spikard.Response.json(%{echo: body})
  end
end

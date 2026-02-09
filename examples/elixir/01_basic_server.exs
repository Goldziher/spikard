# Basic Server Example
#
# The simplest possible Spikard Elixir application.
# Starts a server on port 8000 with routes that return JSON.

defmodule BasicApp.Router do
  use Spikard.Router

  get "/", &hello/1
  get "/health", &health/1
  post "/echo", &echo/1

  defp hello(_request) do
    Spikard.Response.json(%{message: "Hello, World!"})
  end

  defp health(_request) do
    Spikard.Response.json(%{
      status: "ok",
      timestamp: DateTime.utc_now() |> DateTime.to_iso8601()
    })
  end

  defp echo(request) do
    body = Spikard.Request.get_body(request)

    case body do
      %{} ->
        Spikard.Response.json(%{
          echo: body,
          received_at: DateTime.utc_now() |> DateTime.to_iso8601()
        })

      _ ->
        Spikard.Response.json(%{error: "Request body must be JSON"}, status: 400)
    end
  end
end

IO.puts("Starting Spikard Elixir server on http://127.0.0.1:8000")
IO.puts("Press Ctrl+C to stop")

{:ok, _server} = Spikard.start(BasicApp.Router, port: 8000, host: "127.0.0.1")
Process.sleep(:infinity)

# Lifecycle Hooks Example
#
# Demonstrates request lifecycle hooks for logging,
# authentication, and response modification.

defmodule HooksApp.Router do
  use Spikard.Router

  get "/", &index/1
  get "/public", &public_page/1
  get "/protected", &protected_page/1
  post "/data", &create_data/1

  defp index(_request) do
    Spikard.Response.json(%{
      endpoints: ["/public", "/protected", "/data"],
      auth: "Use 'Authorization: Bearer valid-token' header for /protected"
    })
  end

  defp public_page(_request) do
    Spikard.Response.json(%{message: "This is public"})
  end

  defp protected_page(request) do
    Spikard.Response.json(%{
      message: "You are authenticated",
      method: request.method,
      path: request.path
    })
  end

  defp create_data(request) do
    body = Spikard.Request.get_body(request)
    Spikard.Response.json(%{created: body}, status: 201)
  end
end

# Logging hook - runs on every request
logging_hook = fn ctx ->
  IO.puts("[#{DateTime.utc_now() |> DateTime.to_iso8601()}] #{ctx.method} #{ctx.path}")
  {:continue, ctx}
end

# Auth hook - validates Bearer token on protected routes
auth_hook = fn ctx ->
  if String.starts_with?(ctx.path, "/protected") do
    case Map.get(ctx.headers, "authorization") do
      "Bearer valid-token" ->
        {:continue, ctx}

      _ ->
        {:short_circuit, %{
          status: 401,
          headers: %{"content-type" => "application/json"},
          body: %{error: "Unauthorized", message: "Valid Bearer token required"}
        }}
    end
  else
    {:continue, ctx}
  end
end

# Security headers hook - adds security headers to every response
security_hook = fn response ->
  headers = Map.merge(response.headers || %{}, %{
    "x-frame-options" => "DENY",
    "x-content-type-options" => "nosniff",
    "x-xss-protection" => "1; mode=block"
  })

  {:continue, %{response | headers: headers}}
end

IO.puts("Starting lifecycle hooks example on http://127.0.0.1:8004")

{:ok, _server} = Spikard.start(HooksApp.Router,
  port: 8004,
  lifecycle: [
    on_request: [logging_hook],
    pre_handler: [auth_hook],
    on_response: [security_hook]
  ]
)

Process.sleep(:infinity)

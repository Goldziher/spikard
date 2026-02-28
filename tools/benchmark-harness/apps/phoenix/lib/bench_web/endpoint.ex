defmodule BenchWeb.Endpoint do
  use Phoenix.Endpoint, otp_app: :phoenix_bench

  plug Plug.Parsers,
    parsers: [:json, :urlencoded, :multipart],
    json_decoder: Jason,
    pass: ["*/*"],
    length: 10_000_000

  plug BenchWeb.Router
end

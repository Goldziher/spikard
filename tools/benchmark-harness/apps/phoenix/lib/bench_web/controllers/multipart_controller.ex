defmodule BenchWeb.MultipartController do
  use Phoenix.Controller, formats: [:json]

  def small(conn, params), do: multipart_response(conn, params)
  def medium(conn, params), do: multipart_response(conn, params)
  def large(conn, params), do: multipart_response(conn, params)

  defp multipart_response(conn, params) do
    {files_received, total_bytes} = count_files(params)
    json(conn, %{files_received: files_received, total_bytes: total_bytes})
  end

  defp count_files(params) do
    Enum.reduce(params, {0, 0}, fn
      {_key, %Plug.Upload{path: path}}, {count, bytes} ->
        size =
          case File.stat(path) do
            {:ok, %{size: s}} -> s
            _ -> 0
          end

        {count + 1, bytes + size}

      _, acc ->
        acc
    end)
  end
end

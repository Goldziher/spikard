defmodule BenchWeb.ValidatedMultipartController do
  use Phoenix.Controller, formats: [:json]

  def small(conn, params), do: validated_multipart_response(conn, params)
  def medium(conn, params), do: validated_multipart_response(conn, params)
  def large(conn, params), do: validated_multipart_response(conn, params)

  defp validated_multipart_response(conn, params) do
    {files_received, total_bytes} = count_files(params)

    if files_received == 0 do
      conn
      |> put_status(400)
      |> json(%{error: "No files uploaded"})
    else
      json(conn, %{files_received: files_received, total_bytes: total_bytes})
    end
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

defmodule Spikard.Stream do
  @moduledoc """
  Streaming response support for Spikard handlers.

  This module provides helpers for creating HTTP streaming responses. Streaming
  is useful for sending large amounts of data without buffering everything in
  memory, sending server-sent events, or implementing chunked responses.

  ## Examples

  Basic streaming with text chunks:

      handler = fn _req ->
        stream = Stream.map(1..100, &Integer.to_string/1)
        Spikard.Stream.stream(stream, content_type: "text/plain")
      end

  Streaming JSON lines (NDJSON):

      handler = fn _req ->
        stream =
          [%{id: 1}, %{id: 2}, %{id: 3}]
          |> Stream.map(&Jason.encode!/1)

        Spikard.Stream.stream(stream, content_type: "application/x-ndjson")
      end

  Streaming CSV data:

      handler = fn _req ->
        stream =
          users
          |> Stream.map(&csv_row/1)

        Spikard.Stream.stream(stream, content_type: "text/csv")
      end

  Streaming with custom headers:

      handler = fn _req ->
        stream = Stream.map(1..10, &Integer.to_string/1)

        Spikard.Stream.stream(stream, content_type: "text/plain")
        |> Spikard.Response.with_header("x-stream-id", request_id)
        |> Spikard.Response.with_header("cache-control", "no-cache")
      end
  """

  @type t :: %{
          status: non_neg_integer(),
          headers: [{String.t(), String.t()}],
          body: Enumerable.t()
        }

  @doc """
  Creates a streaming response from an Enumerable/Stream.

  The stream should produce binary chunks that will be sent to the client.
  Each element yielded by the stream becomes a chunk in the response body.

  ## Options

  - `:status` - HTTP status code (default: 200)
  - `:content_type` - Content-Type header (default: "application/octet-stream")

  ## Parameters

    * `stream` - An Enumerable or Stream that produces binary chunks
    * `opts` - Options keyword list

  ## Returns

  A response map with the stream in the body that can be further configured
  with Response builder functions.

  ## Examples

      # Simple text stream
      iex> stream = Stream.map(["Hello", " ", "World"], & &1)
      iex> response = Spikard.Stream.stream(stream, content_type: "text/plain")
      iex> response.status
      200
      iex> response.body |> Enum.to_list() |> Enum.join("")
      "Hello World"

      # Custom status
      iex> stream = Stream.map([1, 2, 3], &Integer.to_string/1)
      iex> response = Spikard.Stream.stream(stream, status: 201, content_type: "text/plain")
      iex> response.status
      201

      # Empty stream
      iex> stream = Stream.map([], & &1)
      iex> response = Spikard.Stream.stream(stream)
      iex> response.body |> Enum.to_list()
      []
  """
  @spec stream(Enumerable.t(), keyword()) :: t()
  def stream(enumerable, opts \\ []) do
    status = Keyword.get(opts, :status, 200)
    content_type = Keyword.get(opts, :content_type, "application/octet-stream")

    %{
      status: status,
      headers: [{"content-type", content_type}],
      body: enumerable
    }
  end
end

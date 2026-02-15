defmodule Spikard.StreamTest do
  use ExUnit.Case
  doctest Spikard.Stream

  alias Spikard.{Response}
  alias Spikard.Stream, as: ResponseStream

  describe "stream/2" do
    test "creates streaming response with default content_type" do
      stream_data = Stream.map(1..3, &Integer.to_string/1)
      response = ResponseStream.stream(stream_data)

      assert response.status == 200
      assert {"content-type", "application/octet-stream"} in response.headers
      assert is_stream(response.body)
    end

    test "creates streaming response with custom content_type" do
      stream_data = Stream.map(1..3, &Integer.to_string/1)
      response = ResponseStream.stream(stream_data, content_type: "text/plain")

      assert response.status == 200
      assert {"content-type", "text/plain"} in response.headers
      assert is_stream(response.body)
    end

    test "creates streaming response with custom status" do
      stream_data = Stream.map(1..3, &Integer.to_string/1)
      response = ResponseStream.stream(stream_data, status: 201, content_type: "text/csv")

      assert response.status == 201
      assert {"content-type", "text/csv"} in response.headers
    end

    test "streaming response contains enumerable stream" do
      stream_data = Stream.map(1..3, &Integer.to_string/1)
      response = ResponseStream.stream(stream_data)

      # The body should be an Enumerable/Stream that we can consume
      chunks = response.body |> Enum.to_list()
      assert chunks == ["1", "2", "3"]
    end

    test "supports chunked values (binary strings)" do
      chunks = ["Hello, ", "World", "!"]
      stream_data = Stream.map(chunks, & &1)
      response = ResponseStream.stream(stream_data)

      result = response.body |> Enum.to_list() |> Enum.join("")
      assert result == "Hello, World!"
    end

    test "supports complex data transformation" do
      stream_data =
        1..5
        |> Stream.map(&Integer.to_string/1)
        |> Stream.map(&(String.pad_leading(&1, 2, "0") <> "\n"))

      response = ResponseStream.stream(stream_data, content_type: "text/plain")
      result = response.body |> Enum.to_list() |> Enum.join("")

      assert result == "01\n02\n03\n04\n05\n"
    end

    test "stream can be chained with Response builders" do
      stream_data = Stream.map(["a", "b", "c"], &String.upcase/1)

      response =
        stream_data
        |> ResponseStream.stream(content_type: "text/plain")
        |> Response.with_status(202)
        |> Response.with_header("x-custom", "value")

      assert response.status == 202
      assert {"x-custom", "value"} in response.headers
      assert {"content-type", "text/plain"} in response.headers
    end

    test "handles empty stream" do
      stream_data = Stream.map([], & &1)
      response = ResponseStream.stream(stream_data)

      chunks = response.body |> Enum.to_list()
      assert chunks == []
    end

    test "handles single chunk" do
      stream_data = Stream.map(["single"], & &1)
      response = ResponseStream.stream(stream_data)

      chunks = response.body |> Enum.to_list()
      assert chunks == ["single"]
    end
  end

  describe "Response.stream/2 convenience function" do
    test "delegates to Spikard.Stream.stream/2" do
      stream_data = Stream.map(["a", "b"], & &1)
      response = Response.stream(stream_data, content_type: "text/plain")

      assert response.status == 200
      assert {"content-type", "text/plain"} in response.headers
      assert response.body |> Enum.to_list() == ["a", "b"]
    end

    test "supports all options" do
      stream_data = Stream.map([1, 2], &Integer.to_string/1)
      response = Response.stream(stream_data, status: 202, content_type: "text/csv")

      assert response.status == 202
      assert {"content-type", "text/csv"} in response.headers
    end
  end

  # Helper to check if something is an Enumerable
  defp is_stream(enumerable) do
    try do
      # Try to take one element to verify it's enumerable
      _ = Enum.take(enumerable, 1)
      true
    rescue
      _ -> false
    end
  end
end

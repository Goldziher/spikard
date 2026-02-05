defmodule Spikard.SseTest do
  use ExUnit.Case
  doctest Spikard.Sse

  alias Spikard.Sse.Event

  describe "Event struct" do
    test "creates event with data only" do
      event = %Event{data: "hello"}
      assert event.data == "hello"
      assert event.event == nil
      assert event.id == nil
    end

    test "creates event with all fields" do
      event = %Event{data: "data", event: "message", id: "123"}
      assert event.data == "data"
      assert event.event == "message"
      assert event.id == "123"
    end

    test "event can be encoded to SSE format" do
      event = %Event{data: "hello", event: "msg", id: "1"}
      formatted = Event.to_sse(event)

      assert formatted =~ "event: msg\n"
      assert formatted =~ "data: hello\n"
      assert formatted =~ "id: 1\n"
    end

    test "event encodes multiline data with proper SSE format" do
      event = %Event{data: "line1\nline2\nline3"}
      formatted = Event.to_sse(event)

      assert formatted =~ "data: line1\n"
      assert formatted =~ "data: line2\n"
      assert formatted =~ "data: line3\n"
    end

    test "event with JSON data" do
      event = %Event{data: Jason.encode!(%{hello: "world"})}
      formatted = Event.to_sse(event)
      assert formatted =~ "data: "
    end
  end

  # Note: Full SSE streaming tests require WebSocket/SSE NIF implementation
  # which is marked for future implementation. Basic Event struct tests above
  # validate the core SSE data structures work correctly.
end

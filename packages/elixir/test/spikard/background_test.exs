defmodule Spikard.BackgroundTest do
  @moduledoc """
  Tests for the Spikard.Background module.

  These tests validate that background tasks are scheduled correctly and execute
  after the response is sent to the client.
  """

  use ExUnit.Case, async: true

  alias Spikard.Background
  alias Spikard.TestClient

  describe "Background.run/1" do
    test "schedules a background task" do
      test_pid = self()

      handler = fn _req ->
        Background.run(fn -> send(test_pid, :task_executed) end)
        %{status: 200}
      end

      {:ok, client} = TestClient.new(routes: [{:get, "/", handler}])
      {:ok, _response} = TestClient.get(client, "/")

      # Task should execute after response
      assert_receive :task_executed, 1000
    end

    test "multiple background tasks all execute" do
      test_pid = self()

      handler = fn _req ->
        Background.run(fn -> send(test_pid, :task_1) end)
        Background.run(fn -> send(test_pid, :task_2) end)
        Background.run(fn -> send(test_pid, :task_3) end)
        %{status: 200}
      end

      {:ok, client} = TestClient.new(routes: [{:get, "/", handler}])
      {:ok, _response} = TestClient.get(client, "/")

      # All tasks should execute
      assert_receive :task_1, 1000
      assert_receive :task_2, 1000
      assert_receive :task_3, 1000
    end

    test "background task can access process state" do
      test_pid = self()
      counter = :counters.new(1, [])

      handler = fn _req ->
        Background.run(fn ->
          :counters.add(counter, 1, 1)
          current = :counters.get(counter, 1)
          send(test_pid, {:counter_value, current})
        end)

        %{status: 200}
      end

      {:ok, client} = TestClient.new(routes: [{:get, "/", handler}])
      {:ok, _response} = TestClient.get(client, "/")

      assert_receive {:counter_value, 1}, 1000
    end

    test "background task runs after response is sent" do
      test_pid = self()

      handler = fn _req ->
        Background.run(fn ->
          # Add a small delay to ensure we can detect ordering
          Process.sleep(50)
          send(test_pid, :task_complete)
        end)

        %{status: 200, body: %{message: "Response sent"}}
      end

      {:ok, client} = TestClient.new(routes: [{:get, "/", handler}])

      start_time = System.monotonic_time(:millisecond)
      {:ok, response} = TestClient.get(client, "/")
      response_time = System.monotonic_time(:millisecond) - start_time

      # Response should be fast (not waiting for background task with delay)
      assert response_time < 500, "Response took too long: #{response_time}ms"
      assert response.status_code == 200

      # But task should still execute
      assert_receive :task_complete, 1000
    end

    test "background task error does not affect response" do
      test_pid = self()

      handler = fn _req ->
        Background.run(fn ->
          raise "Task error"
        end)

        send(test_pid, :response_sent)
        %{status: 200}
      end

      {:ok, client} = TestClient.new(routes: [{:get, "/", handler}])
      {:ok, response} = TestClient.get(client, "/")

      # Response should still succeed
      assert response.status_code == 200
      assert_receive :response_sent, 1000

      # Note: In a real implementation, we might want to capture and log the error
      # but the response should not be affected
    end

    test "background task can spawn other tasks" do
      test_pid = self()

      handler = fn _req ->
        Background.run(fn ->
          Background.run(fn ->
            send(test_pid, :nested_task)
          end)

          send(test_pid, :outer_task)
        end)

        %{status: 200}
      end

      {:ok, client} = TestClient.new(routes: [{:get, "/", handler}])
      {:ok, _response} = TestClient.get(client, "/")

      # Both nested and outer tasks should execute
      assert_receive :outer_task, 1000
      assert_receive :nested_task, 1000
    end

    test "background task receives correct arguments" do
      test_pid = self()

      handler = fn _req ->
        Background.run(fn ->
          send(test_pid, {:task_arg, 42})
        end)

        %{status: 200}
      end

      {:ok, client} = TestClient.new(routes: [{:get, "/", handler}])
      {:ok, _response} = TestClient.get(client, "/")

      assert_receive {:task_arg, 42}, 1000
    end

    test "multiple requests each run their own background tasks" do
      test_pid = self()
      counter = :counters.new(1, [])

      handler = fn _req ->
        Background.run(fn ->
          :counters.add(counter, 1, 1)
          current = :counters.get(counter, 1)
          send(test_pid, {:request_complete, current})
        end)

        %{status: 200}
      end

      {:ok, client} = TestClient.new(routes: [{:get, "/", handler}])

      # Make two requests
      {:ok, _} = TestClient.get(client, "/")
      {:ok, _} = TestClient.get(client, "/")

      # Both should execute their background tasks
      assert_receive {:request_complete, 1}, 1000
      assert_receive {:request_complete, 2}, 1000
    end

    test "background task with timeout completes within reasonable time" do
      test_pid = self()

      handler = fn _req ->
        Background.run(fn ->
          Process.sleep(100)
          send(test_pid, :slow_task)
        end)

        %{status: 200}
      end

      {:ok, client} = TestClient.new(routes: [{:get, "/", handler}])
      {:ok, _response} = TestClient.get(client, "/")

      # Task should complete within a reasonable time
      assert_receive :slow_task, 2000
    end
  end

  describe "Background.run/2 with options" do
    test "respects timeout option" do
      test_pid = self()

      handler = fn _req ->
        Background.run(
          fn ->
            Process.sleep(100)
            send(test_pid, :task_complete)
          end,
          timeout: 500
        )

        %{status: 200}
      end

      {:ok, client} = TestClient.new(routes: [{:get, "/", handler}])
      {:ok, _response} = TestClient.get(client, "/")

      # Task should complete within timeout
      assert_receive :task_complete, 1000
    end

    test "respects short timeout" do
      test_pid = self()

      handler = fn _req ->
        Background.run(
          fn ->
            Process.sleep(200)
            send(test_pid, :should_timeout)
          end,
          timeout: 50
        )

        send(test_pid, :handler_done)
        %{status: 200}
      end

      {:ok, client} = TestClient.new(routes: [{:get, "/", handler}])
      {:ok, _response} = TestClient.get(client, "/")

      # Handler should complete immediately
      assert_receive :handler_done, 1000

      # Task may or may not complete depending on timing
      # But we shouldn't receive it quickly due to timeout
      refute_receive :should_timeout, 100
    end
  end
end

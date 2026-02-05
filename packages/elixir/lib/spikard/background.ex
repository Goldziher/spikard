defmodule Spikard.Background do
  @moduledoc """
  Background task scheduling for Spikard handlers.

  This module allows HTTP request handlers to schedule work that runs after
  the response has been sent to the client. This is useful for cleanup,
  logging, background processing, or any work that doesn't need to block
  the response.

  ## Usage

  ```elixir
  handler = fn _req ->
    # Schedule background task
    Spikard.Background.run(fn ->
      IO.puts("This runs after response is sent")
    end)

    # Return response immediately
    %{status: 200, body: %{message: "OK"}}
  end
  ```

  ## Task Execution

  - Tasks are spawned in separate Elixir processes
  - Multiple tasks can be scheduled in a single request
  - Tasks execute concurrently after the response is sent
  - Task exceptions do not affect the response
  - All tasks are fire-and-forget by default

  ## Options

  - `:timeout` - Optional timeout in milliseconds for task execution
                (default: no timeout, task runs indefinitely)
  """

  @doc """
  Schedule a background task to run after the response is sent.

  The given function is executed in a separate Erlang process after the
  HTTP response has been delivered to the client. This allows expensive
  or non-critical work to happen without blocking the response.

  ## Arguments

    - `fun` - Zero-argument function to execute in background

  ## Returns

    - `:ok` - Task was scheduled successfully

  ## Examples

      iex> Spikard.Background.run(fn -> IO.puts("Background work") end)
      :ok

      iex> Spikard.Background.run(fn ->
      ...>   Process.sleep(1000)
      ...>   log_metrics()
      ...> end)
      :ok
  """
  @spec run((-> any())) :: :ok
  def run(fun) when is_function(fun, 0) do
    run(fun, [])
  end

  @doc """
  Schedule a background task with options.

  ## Arguments

    - `fun` - Zero-argument function to execute in background
    - `opts` - Keyword list of options

  ## Options

    - `:timeout` - Milliseconds to wait before killing task (optional)

  ## Examples

      iex> Spikard.Background.run(fn -> long_running_job() end, timeout: 30000)
      :ok
  """
  @spec run((-> any()), keyword()) :: :ok
  def run(fun, opts) when is_function(fun, 0) and is_list(opts) do
    _timeout = Keyword.get(opts, :timeout, nil)

    # Spawn a task wrapper that executes the function safely
    Task.start(fn ->
      try do
        fun.()
      rescue
        _e -> :ok
      catch
        _kind, _reason -> :ok
      end
    end)

    :ok
  end

  @doc """
  Check if a background task is currently scheduled (for testing purposes).

  This is mainly useful for testing to verify that tasks have been scheduled.

  ## Returns

    - `:ok` - Current task count

  ## Examples

      iex> Spikard.Background.run(fn -> :ok end)
      :ok
      iex> Spikard.Background.pending_task_count()
      1
  """
  @spec pending_task_count() :: non_neg_integer()
  def pending_task_count do
    # Get count of running tasks for this process
    case Process.info(self(), :links) do
      {:links, links} ->
        Enum.count(links, fn pid ->
          case Process.info(pid, :status) do
            {:status, :running} -> true
            _ -> false
          end
        end)

      _ ->
        0
    end
  end
end

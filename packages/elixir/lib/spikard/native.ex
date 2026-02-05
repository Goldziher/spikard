defmodule Spikard.Native do
  @moduledoc """
  NIF interface to the Rust spikard-elixir crate.

  This module provides low-level NIF stubs that call into the Rust implementation.
  For typical usage, prefer the high-level `Spikard` module instead.

  The NIF is loaded from precompiled binaries when available (production),
  or compiled from source when SPIKARD_BUILD=1 is set (CI/development).
  """

  @version Mix.Project.config()[:version]

  use RustlerPrecompiled,
    otp_app: :spikard,
    crate: "spikard_elixir",
    base_url: "https://github.com/Goldziher/spikard/releases/download/v#{@version}",
    version: @version,
    force_build: System.get_env("SPIKARD_BUILD") in ["1", "true"],
    targets: ~w(
      aarch64-apple-darwin
      aarch64-unknown-linux-gnu
      aarch64-unknown-linux-musl
      x86_64-apple-darwin
      x86_64-unknown-linux-gnu
      x86_64-unknown-linux-musl
    ),
    nif_versions: ["2.15", "2.16", "2.17"]

  @doc """
  Start the Spikard HTTP server.

  ## Arguments

    - `port` - Port number to bind to (integer, 1-65535)
    - `host` - Host address to bind to (string, e.g., "0.0.0.0" or "127.0.0.1")
    - `routes_json` - JSON string containing route metadata
    - `handler_runner_pid` - PID of the HandlerRunner GenServer
    - `config_map` - Server configuration options as a map

  ## Returns

    - `{:ok, server_ref}` - Server started; server_ref is `{host, port}`
    - `{:error, reason}` - Error with reason atom or string
  """
  @spec start_server(integer(), String.t(), String.t(), pid(), map()) ::
          {:ok, {String.t(), integer()}} | {:error, atom() | String.t()}
  def start_server(_port, _host, _routes_json, _handler_runner_pid, _config_map) do
    :erlang.nif_error(:nif_not_loaded)
  end

  @doc """
  Stop a running Spikard HTTP server.

  ## Arguments

    - `host` - Host address the server is bound to
    - `port` - Port number the server is listening on

  ## Returns

    - `{:ok, :stopped}` - Server stopped
    - `{:error, reason}` - Error with reason
  """
  @spec stop_server(String.t(), integer()) :: :ok | {:error, {atom(), String.t()}}
  def stop_server(_host, _port) do
    :erlang.nif_error(:nif_not_loaded)
  end

  @doc """
  Get information about a running server.

  ## Arguments

    - `host` - Host address the server is bound to
    - `port` - Port number the server is listening on

  ## Returns

    - A tuple `{host, port}` with server info
  """
  @spec server_info(String.t(), integer()) :: {String.t(), integer()}
  def server_info(_host, _port) do
    :erlang.nif_error(:nif_not_loaded)
  end

  @doc """
  Deliver a handler response from Elixir back to the waiting Rust handler.

  This is called by the HandlerRunner GenServer after processing a request.

  ## Arguments

    - `request_id` - Unique identifier for the request
    - `response_map` - Response map with status, headers, body

  ## Returns

    - `:ok` - Response delivered successfully
    - `{:error, reason}` - Failed to deliver response
  """
  @spec deliver_handler_response(non_neg_integer(), map()) :: :ok | {:error, atom()}
  def deliver_handler_response(_request_id, _response_map) do
    :erlang.nif_error(:nif_not_loaded)
  end
end

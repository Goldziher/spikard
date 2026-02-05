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

  # SSE NIFs

  @doc """
  Deliver an SSE event result from Elixir producer back to the waiting Rust code.

  This is called by the ProducerServer after getting the next event from the producer.

  ## Arguments

    - `request_id` - Unique identifier for the SSE request
    - `event_term` - Event term (:done atom or event map with data, event, id fields)

  ## Returns

    - `:ok` - Event delivered successfully
    - `{:error, reason}` - Failed to deliver event
  """
  @spec deliver_sse_event_result(non_neg_integer(), term()) :: :ok | {:error, atom()}
  def deliver_sse_event_result(_request_id, _event_term) do
    :erlang.nif_error(:nif_not_loaded)
  end

  # TestClient NIFs

  @doc """
  Create a new test client from routes configuration.

  ## Arguments

    - `routes_json` - JSON string containing route metadata
    - `handler_runner_pid` - PID of the HandlerRunner GenServer
    - `config_map` - Optional server configuration

  ## Returns

    - `{:ok, client_ref}` - Test client created successfully
    - `{:error, reason}` - Failed to create client
  """
  @spec test_client_new(String.t(), pid(), map()) :: {:ok, reference()} | {:error, term()}
  def test_client_new(_routes_json, _handler_runner_pid, _config_map) do
    :erlang.nif_error(:nif_not_loaded)
  end

  @doc """
  Make a request to the test client.

  ## Arguments

    - `client` - TestClient resource reference
    - `method` - HTTP method string (GET, POST, etc.)
    - `path` - Request path
    - `opts` - Request options map (headers, query, json, form, cookies)

  ## Returns

    - `{:ok, response_map}` - Request successful
    - `{:error, reason}` - Request failed
  """
  @spec test_client_request(reference(), String.t(), String.t(), map()) ::
          {:ok, map()} | {:error, term()}
  def test_client_request(_client, _method, _path, _opts) do
    :erlang.nif_error(:nif_not_loaded)
  end

  @doc """
  Close the test client and release resources.

  ## Arguments

    - `client` - TestClient resource reference

  ## Returns

    - `:ok` - Client closed
  """
  @spec test_client_close(reference()) :: :ok
  def test_client_close(_client) do
    :erlang.nif_error(:nif_not_loaded)
  end

  # Lifecycle hook NIFs

  @doc """
  Deliver a lifecycle hook response from Elixir back to the waiting Rust hook.

  This is called by the HandlerRunner GenServer after processing a hook.

  ## Arguments

    - `request_id` - The unique ID of the hook request
    - `result_type` - Atom :continue or :short_circuit
    - `payload` - The hook result payload (context or response map)

  ## Returns

    - `:ok` - Response delivered successfully
    - `{:error, reason}` - Failed to deliver response
  """
  @spec deliver_hook_response(non_neg_integer(), atom(), term()) :: :ok | {:error, atom()}
  def deliver_hook_response(_request_id, _result_type, _payload) do
    :erlang.nif_error(:nif_not_loaded)
  end

  # DI NIFs

  @doc """
  Deliver a factory dependency response from Elixir back to waiting Rust code.

  ## Arguments

    - `request_id` - The unique ID of the factory request
    - `result` - The factory result value

  ## Returns

    - `:ok` - Response delivered successfully
    - `{:error, reason}` - Failed to deliver response
  """
  @spec deliver_factory_response(non_neg_integer(), term()) :: :ok | {:error, atom()}
  def deliver_factory_response(_request_id, _result) do
    :erlang.nif_error(:nif_not_loaded)
  end
end

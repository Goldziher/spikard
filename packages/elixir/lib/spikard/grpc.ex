defmodule Spikard.Grpc do
  @moduledoc """
  gRPC support for Spikard.

  Spikard's Elixir gRPC surface is intentionally transport-backed by the Rust
  runtime. Elixir handlers receive raw protobuf payload bytes and return raw
  protobuf payload bytes, while Spikard handles the HTTP/2 and gRPC protocol
  details.

  ## Quick Start

      alias Spikard.Grpc

      grpc =
        Grpc.Service.new()
        |> Grpc.Service.register("example.EchoService", "Echo", :unary, fn request ->
          %Grpc.Response{payload: request.payload}
        end)

      {:ok, server} = Spikard.start(port: 50051, grpc: grpc)

  ## Streaming

  The registry supports all four RPC modes:

    * `:unary`
    * `:server_stream`
    * `:client_stream`
    * `:bidi_stream`
  """

  defmodule Request do
    @moduledoc """
    gRPC request passed to Elixir handlers.
    """

    @enforce_keys [:service_name, :method_name, :payload, :metadata]
    defstruct [:service_name, :method_name, :payload, :metadata]

    @type t :: %__MODULE__{
            service_name: String.t(),
            method_name: String.t(),
            payload: binary(),
            metadata: %{optional(String.t()) => String.t()}
          }
  end

  defmodule Error do
    @moduledoc """
    Structured gRPC error returned by Elixir handlers.
    """

    @enforce_keys [:code, :message]
    defstruct code: :internal, message: "Internal error", metadata: %{}

    @type code ::
            :ok
            | :cancelled
            | :unknown
            | :invalid_argument
            | :deadline_exceeded
            | :not_found
            | :already_exists
            | :permission_denied
            | :resource_exhausted
            | :failed_precondition
            | :aborted
            | :out_of_range
            | :unimplemented
            | :internal
            | :unavailable
            | :data_loss
            | :unauthenticated
            | non_neg_integer()
            | String.t()

    @type t :: %__MODULE__{
            code: code(),
            message: String.t(),
            metadata: %{optional(String.t()) => String.t()}
          }
  end

  defmodule Response do
    @moduledoc """
    gRPC response returned by Elixir handlers.
    """

    defstruct payload: <<>>, metadata: %{}

    @type t :: %__MODULE__{
            payload: binary(),
            metadata: %{optional(String.t()) => String.t()}
          }

    @spec error(String.t(), Spikard.Grpc.Error.code(), %{optional(String.t()) => String.t()}) ::
            {:error, Spikard.Grpc.Error.t()}
    def error(message, code \\ :internal, metadata \\ %{}) do
      {:error, %Spikard.Grpc.Error{code: code, message: message, metadata: metadata}}
    end
  end

  defmodule Service do
    @moduledoc """
    Registry of gRPC methods mounted on a Spikard server.

    Each registered method carries its RPC mode and an Elixir handler function.
    The registry is split into two views internally:

      * service definitions for the Rust gRPC router
      * handler functions retained in Elixir for method execution
    """

    alias Spikard.Grpc

    @type rpc_mode :: :unary | :server_stream | :client_stream | :bidi_stream
    @type method_handler :: function()
    @type method_entry :: %{mode: rpc_mode(), handler: method_handler()}
    @type service_entry :: %{optional(String.t()) => method_entry()}
    @type t :: %__MODULE__{services: %{optional(String.t()) => service_entry()}}

    defstruct services: %{}

    @spec new() :: t()
    def new do
      %__MODULE__{}
    end

    @spec register(t(), String.t(), String.t(), rpc_mode(), method_handler()) :: t()
    def register(%__MODULE__{} = service, service_name, method_name, rpc_mode, handler)
        when is_binary(service_name) and is_function(handler) do
      normalized_service = normalize_service_name(service_name)
      normalized_method = normalize_method_name(method_name)
      normalized_mode = normalize_rpc_mode(rpc_mode)

      methods =
        service.services
        |> Map.get(normalized_service, %{})
        |> Map.put(normalized_method, %{mode: normalized_mode, handler: handler})

      %{service | services: Map.put(service.services, normalized_service, methods)}
    end

    @spec register(t(), String.t(), %{required(String.t() | atom()) => {rpc_mode(), method_handler()}}) :: t()
    def register(%__MODULE__{} = service, service_name, methods)
        when is_binary(service_name) and is_map(methods) do
      Enum.reduce(methods, service, fn {method_name, {rpc_mode, handler}}, acc ->
        register(acc, service_name, to_string(method_name), rpc_mode, handler)
      end)
    end

    @spec service_definitions(t()) :: %{optional(String.t()) => %{optional(String.t()) => String.t()}}
    def service_definitions(%__MODULE__{services: services}) do
      Enum.into(services, %{}, fn {service_name, methods} ->
        method_defs =
          Enum.into(methods, %{}, fn {method_name, %{mode: mode}} ->
            {method_name, mode_to_string(mode)}
          end)

        {service_name, method_defs}
      end)
    end

    @spec handler_map(t()) :: %{optional(String.t()) => %{optional(String.t()) => {rpc_mode(), method_handler()}}}
    def handler_map(%__MODULE__{services: services}) do
      Enum.into(services, %{}, fn {service_name, methods} ->
        handler_methods =
          Enum.into(methods, %{}, fn {method_name, %{mode: mode, handler: handler}} ->
            {method_name, {mode, handler}}
          end)

        {service_name, handler_methods}
      end)
    end

    @spec registered?(t(), String.t(), String.t()) :: boolean()
    def registered?(%__MODULE__{services: services}, service_name, method_name) do
      normalized_service = normalize_service_name(service_name)
      normalized_method = normalize_method_name(method_name)

      services
      |> Map.get(normalized_service, %{})
      |> Map.has_key?(normalized_method)
    end

    @spec list_services(t()) :: [String.t()]
    def list_services(%__MODULE__{services: services}) do
      Map.keys(services)
    end

    @spec list_methods(t(), String.t()) :: [String.t()]
    def list_methods(%__MODULE__{services: services}, service_name) do
      service_name
      |> normalize_service_name()
      |> then(&Map.get(services, &1, %{}))
      |> Map.keys()
    end

    @spec build_request(String.t(), String.t(), binary(), map()) :: Grpc.Request.t()
    def build_request(service_name, method_name, payload, metadata) do
      %Grpc.Request{
        service_name: service_name,
        method_name: method_name,
        payload: payload,
        metadata: stringify_metadata(metadata)
      }
    end

    defp normalize_service_name(service_name) when is_binary(service_name) do
      service_name
      |> String.trim()
      |> case do
        "" -> raise ArgumentError, "service_name cannot be empty"
        value -> value
      end
    end

    defp normalize_method_name(method_name) do
      method_name
      |> to_string()
      |> String.trim()
      |> case do
        "" -> raise ArgumentError, "method_name cannot be empty"
        value -> value
      end
    end

    defp normalize_rpc_mode(rpc_mode) when rpc_mode in [:unary, :server_stream, :client_stream, :bidi_stream] do
      rpc_mode
    end

    defp normalize_rpc_mode(rpc_mode) do
      raise ArgumentError,
            "unsupported gRPC rpc_mode #{inspect(rpc_mode)}; expected :unary, :server_stream, :client_stream, or :bidi_stream"
    end

    defp mode_to_string(:unary), do: "unary"
    defp mode_to_string(:server_stream), do: "server_stream"
    defp mode_to_string(:client_stream), do: "client_stream"
    defp mode_to_string(:bidi_stream), do: "bidi_stream"

    defp stringify_metadata(metadata) when is_map(metadata) do
      Enum.into(metadata, %{}, fn {key, value} -> {to_string(key), to_string(value)} end)
    end
  end
end

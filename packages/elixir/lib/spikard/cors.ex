defmodule Spikard.Cors do
  @moduledoc """
  CORS (Cross-Origin Resource Sharing) configuration for Spikard.

  This module provides a struct and validation functions for configuring
  CORS behavior in Spikard applications.

  ## Configuration Options

  - `:allowed_origins` - List of allowed origins (required). Use `["*"]` to allow all origins.
  - `:allowed_methods` - List of allowed HTTP methods (required). Use `["*"]` to allow all methods.
  - `:allowed_headers` - List of allowed request headers (optional). Use `["*"]` to allow all headers.
  - `:expose_headers` - List of headers to expose to the client (optional).
  - `:max_age` - Preflight cache duration in seconds (optional).
  - `:allow_credentials` - Whether to include credentials in CORS requests (optional).

  ## Examples

      # Allow any origin with GET and POST
      cors: %{
        allowed_origins: ["*"],
        allowed_methods: ["GET", "POST"]
      }

      # Restrict to specific origins with custom headers
      cors: %{
        allowed_origins: ["https://example.com", "https://app.example.com"],
        allowed_methods: ["GET", "POST", "PUT"],
        allowed_headers: ["content-type", "authorization"],
        expose_headers: ["x-total-count", "x-page"],
        max_age: 3600,
        allow_credentials: true
      }

  ## CORS Preflight

  CORS preflight requests (OPTIONS requests) are automatically handled by the
  Spikard framework. Responses include appropriate headers:

  - `Access-Control-Allow-Origin` - Allowed origin
  - `Access-Control-Allow-Methods` - Allowed methods
  - `Access-Control-Allow-Headers` - Allowed headers
  - `Access-Control-Max-Age` - Preflight cache duration
  - `Access-Control-Allow-Credentials` - Whether credentials are allowed

  Regular (non-OPTIONS) requests also include CORS headers when applicable.
  """

  @typedoc "CORS configuration map"
  @type t :: map()

  @doc """
  Validates a CORS configuration map.

  Returns `{:ok, config}` if the configuration is valid, or
  `{:error, reason}` if there are validation errors.

  Required keys:
  - `:allowed_origins` - Non-empty list of strings
  - `:allowed_methods` - Non-empty list of strings

  Optional keys:
  - `:allowed_headers` - List of strings (default: [])
  - `:expose_headers` - List of strings or nil
  - `:max_age` - Positive integer or nil
  - `:allow_credentials` - Boolean or nil
  """
  @spec validate(map()) :: {:ok, t()} | {:error, String.t()}
  def validate(config) when is_map(config) do
    with :ok <- validate_allowed_origins(config),
         :ok <- validate_allowed_methods(config),
         :ok <- validate_optional_fields(config) do
      {:ok, config}
    end
  end

  def validate(_), do: {:error, "CORS configuration must be a map"}

  @doc """
  Validates a CORS configuration, raising an error if invalid.

  See `validate/1` for detailed validation rules.
  """
  @spec validate!(map()) :: t()
  def validate!(config) do
    case validate(config) do
      {:ok, validated} -> validated
      {:error, reason} -> raise ArgumentError, reason
    end
  end

  # Private helper functions

  defp validate_allowed_origins(config) do
    case Map.get(config, :allowed_origins) do
      nil ->
        {:error, "CORS configuration requires :allowed_origins"}

      origins when is_list(origins) ->
        if Enum.empty?(origins) do
          {:error, "CORS :allowed_origins cannot be empty"}
        else
          case Enum.all?(origins, &is_binary/1) do
            true -> :ok
            false -> {:error, "CORS :allowed_origins must be a list of strings"}
          end
        end

      _ ->
        {:error, "CORS :allowed_origins must be a list"}
    end
  end

  defp validate_allowed_methods(config) do
    case Map.get(config, :allowed_methods) do
      nil ->
        {:error, "CORS configuration requires :allowed_methods"}

      methods when is_list(methods) ->
        if Enum.empty?(methods) do
          {:error, "CORS :allowed_methods cannot be empty"}
        else
          case Enum.all?(methods, &is_binary/1) do
            true -> :ok
            false -> {:error, "CORS :allowed_methods must be a list of strings"}
          end
        end

      _ ->
        {:error, "CORS :allowed_methods must be a list"}
    end
  end

  defp validate_optional_fields(config) do
    with :ok <- validate_allowed_headers(config),
         :ok <- validate_expose_headers(config),
         :ok <- validate_max_age(config),
         :ok <- validate_allow_credentials(config) do
      :ok
    end
  end

  defp validate_allowed_headers(config) do
    case Map.get(config, :allowed_headers) do
      nil ->
        :ok

      headers when is_list(headers) ->
        if Enum.all?(headers, &is_binary/1) do
          :ok
        else
          {:error, "CORS :allowed_headers must be a list of strings"}
        end

      _ ->
        {:error, "CORS :allowed_headers must be a list or nil"}
    end
  end

  defp validate_expose_headers(config) do
    case Map.get(config, :expose_headers) do
      nil ->
        :ok

      headers when is_list(headers) ->
        if Enum.all?(headers, &is_binary/1) do
          :ok
        else
          {:error, "CORS :expose_headers must be a list of strings"}
        end

      _ ->
        {:error, "CORS :expose_headers must be a list or nil"}
    end
  end

  defp validate_max_age(config) do
    case Map.get(config, :max_age) do
      nil -> :ok
      age when is_integer(age) and age > 0 -> :ok
      _ -> {:error, "CORS :max_age must be a positive integer or nil"}
    end
  end

  defp validate_allow_credentials(config) do
    case Map.get(config, :allow_credentials) do
      nil -> :ok
      true -> :ok
      false -> :ok
      _ -> {:error, "CORS :allow_credentials must be a boolean or nil"}
    end
  end
end

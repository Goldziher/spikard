defmodule Spikard.Auth.ApiKey do
  @moduledoc """
  API Key authentication configuration for Spikard.

  This module defines the configuration structure for API key authentication
  middleware, which validates incoming requests contain a valid API key
  in either a specified header or query parameter.

  ## Configuration

  API key authentication can be configured with the following options:

  - `keys` (required list): List of valid API keys that are accepted.
                            Example: `["key1", "key2", "key3"]`
  - `header_name` (optional string): Header to check for the API key.
                                     Default: "x-api-key"
                                     Example: "Authorization-Key"

  ## Usage

  Use with `Spikard.TestClient.new/1`:

      {:ok, client} = Spikard.TestClient.new(
        routes: [{:get, "/api", handler}],
        api_key_auth: %{
          keys: ["valid-key-123", "valid-key-456"],
          header_name: "X-API-Key"
        }
      )

  Or with `Spikard.start/2`:

      {:ok, server} = Spikard.start(
        port: 8000,
        routes: routes,
        api_key_auth: %{keys: ["my-secret-key"]}
      )

  ## Authentication Flow

  When API key authentication is enabled:

  1. The middleware checks for an API key in the specified header (default: "x-api-key")
  2. If header is missing, checks for "api_key" query parameter
  3. If either contains a value in the `keys` list, the request proceeds
  4. Otherwise, returns 401 Unauthorized with RFC 9457 Problem Details

  ## Error Response

  When authentication fails, the server responds with:

      HTTP/1.1 401 Unauthorized
      Content-Type: application/problem+json

      {
        "type": "https://spikard.dev/errors/unauthorized",
        "title": "Unauthorized",
        "status": 401,
        "detail": "Missing or invalid API key"
      }

  ## Examples

  ### Basic usage with default header

      api_key_config = %{keys: ["my-api-key"]}

  ### Custom header name

      api_key_config = %{
        keys: ["prod-key-1", "prod-key-2"],
        header_name: "Authorization-Token"
      }

  ### Multiple keys for rotation

      api_key_config = %{
        keys: [
          "old-key-for-migration",
          "current-key",
          "staging-key"
        ],
        header_name: "X-API-Key"
      }
  """

  @typedoc """
  API key authentication configuration.

  - `keys`: List of valid API keys
  - `header_name`: Header to check (defaults to "x-api-key" if not specified)
  """
  @type t :: %{
    required(:keys) => [String.t()],
    optional(:header_name) => String.t()
  }

  @doc """
  Validates API key configuration structure.

  Returns `:ok` if valid, or `{:error, reason}` if configuration is invalid.

  ## Examples

      iex> Spikard.Auth.ApiKey.validate(%{keys: ["valid"]})
      :ok

      iex> Spikard.Auth.ApiKey.validate(%{keys: []})
      {:error, "keys list cannot be empty"}

      iex> Spikard.Auth.ApiKey.validate(%{})
      {:error, "keys field is required"}
  """
  @spec validate(term()) :: :ok | {:error, String.t()}
  def validate(config) when is_map(config) do
    cond do
      not Map.has_key?(config, :keys) ->
        {:error, "keys field is required"}

      not is_list(config.keys) ->
        {:error, "keys must be a list of strings"}

      Enum.empty?(config.keys) ->
        {:error, "keys list cannot be empty"}

      not Enum.all?(config.keys, &is_binary/1) ->
        {:error, "all keys must be strings"}

      true ->
        # Validate header_name if present
        case Map.get(config, :header_name) do
          nil -> :ok
          header_name when is_binary(header_name) -> :ok
          _ -> {:error, "header_name must be a string"}
        end
    end
  end

  def validate(_config) do
    {:error, "api_key_auth configuration must be a map"}
  end

  @doc """
  Normalizes API key configuration to ensure all fields are present with defaults.

  ## Examples

      iex> Spikard.Auth.ApiKey.normalize(%{keys: ["key1"]})
      %{keys: ["key1"], header_name: "x-api-key"}

      iex> Spikard.Auth.ApiKey.normalize(%{keys: ["key1"], header_name: "X-Custom"})
      %{keys: ["key1"], header_name: "X-Custom"}
  """
  @spec normalize(t()) :: t()
  def normalize(config) do
    Map.put_new(config, :header_name, "x-api-key")
  end
end

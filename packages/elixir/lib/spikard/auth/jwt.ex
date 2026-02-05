defmodule Spikard.Auth.Jwt do
  @moduledoc """
  JWT authentication configuration for Spikard.

  This module provides a configuration struct for JWT (JSON Web Token) authentication
  middleware. When enabled on a Spikard server or test client, all incoming requests
  must include a valid JWT token in the Authorization header (or custom header).

  ## Configuration

  Create a JWT config with required and optional parameters:

      config = Spikard.Auth.Jwt.new(
        secret: "my-secret",
        algorithm: :hs256,
        header_name: "authorization",
        required_claims: ["sub", "exp"]
      )

  ## Supported Algorithms

  The following JWT signing algorithms are supported:

  - `:hs256` - HMAC SHA-256 (default)
  - `:hs384` - HMAC SHA-384
  - `:hs512` - HMAC SHA-512
  - `:rs256` - RSA Signature with SHA-256
  - `:rs384` - RSA Signature with SHA-384
  - `:rs512` - RSA Signature with SHA-512
  - `:es256` - ECDSA with SHA-256
  - `:es384` - ECDSA with SHA-384
  - `:ps256` - RSA PSS Signature with SHA-256
  - `:ps384` - RSA PSS Signature with SHA-384
  - `:ps512` - RSA PSS Signature with SHA-512

  ## Token Format

  Tokens must be sent in the Authorization header using the Bearer scheme:

      Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...

  Or in a custom header if configured:

      X-Auth-Token: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...

  ## Error Responses

  Invalid or missing tokens return HTTP 401 Unauthorized with a JSON error response:

      {
        "type": "https://spikard.dev/errors/unauthorized",
        "title": "JWT validation failed",
        "status": 401,
        "detail": "Token has expired"
      }

  ## Examples

  ### Basic Usage

      jwt_config = Spikard.Auth.Jwt.new(secret: "my-secret")

      {:ok, client} = Spikard.TestClient.new(
        routes: [{:get, "/api", handler}],
        jwt_auth: jwt_config
      )

  ### With Custom Algorithm

      jwt_config = Spikard.Auth.Jwt.new(
        secret: "my-secret",
        algorithm: :hs512
      )

  ### With Custom Header

      jwt_config = Spikard.Auth.Jwt.new(
        secret: "my-secret",
        header_name: "x-api-token"
      )

  ### With Required Claims

      jwt_config = Spikard.Auth.Jwt.new(
        secret: "my-secret",
        required_claims: ["sub", "exp", "aud"]
      )
  """

  @typedoc """
  JWT authentication configuration.

  ## Fields

    - `:secret` - The secret key for HMAC algorithms or public key for RSA/ECDSA
    - `:algorithm` - The JWT signing algorithm (default: :hs256)
    - `:header_name` - The HTTP header name to check for the token (default: "authorization")
    - `:required_claims` - List of claim names that must be present in the token (default: [])
  """
  @type t :: %__MODULE__{
          secret: String.t(),
          algorithm: atom(),
          header_name: String.t(),
          required_claims: [String.t()]
        }

  defstruct [
    :secret,
    algorithm: :hs256,
    header_name: "authorization",
    required_claims: []
  ]

  @doc """
  Create a new JWT configuration.

  ## Arguments

    - `opts` - Keyword list with configuration options

  ## Options

    - `:secret` (required) - The secret key for verifying JWT signatures
    - `:algorithm` - JWT signing algorithm atom (default: :hs256)
    - `:header_name` - HTTP header to check for token (default: "authorization")
    - `:required_claims` - List of required claim names (default: [])

  ## Returns

    A `t:Spikard.Auth.Jwt.t/0` struct with the provided configuration.

  ## Raises

    - `ArgumentError` if `:secret` is not provided or is empty
    - `ArgumentError` if `:algorithm` is not a supported algorithm atom

  ## Examples

      iex> Spikard.Auth.Jwt.new(secret: "test-secret")
      %Spikard.Auth.Jwt{
        secret: "test-secret",
        algorithm: :hs256,
        header_name: "authorization",
        required_claims: []
      }

      iex> Spikard.Auth.Jwt.new(
      ...>   secret: "test-secret",
      ...>   algorithm: :hs512,
      ...>   header_name: "x-token",
      ...>   required_claims: ["sub"]
      ...> )
      %Spikard.Auth.Jwt{
        secret: "test-secret",
        algorithm: :hs512,
        header_name: "x-token",
        required_claims: ["sub"]
      }
  """
  @spec new(opts :: keyword()) :: t()
  def new(opts) do
    secret = Keyword.fetch!(opts, :secret)

    if not is_binary(secret) or String.length(secret) == 0 do
      raise ArgumentError, "secret must be a non-empty string"
    end

    algorithm = Keyword.get(opts, :algorithm, :hs256)
    validate_algorithm!(algorithm)

    header_name = Keyword.get(opts, :header_name, "authorization")
    required_claims = Keyword.get(opts, :required_claims, [])

    %__MODULE__{
      secret: secret,
      algorithm: algorithm,
      header_name: header_name,
      required_claims: required_claims
    }
  end

  @doc """
  Convert JWT config struct to a map for passing to Rust.

  This function is used internally to serialize the JWT configuration
  when passing it to the Rust implementation via NIFs.

  ## Examples

      iex> config = Spikard.Auth.Jwt.new(secret: "test-secret")
      iex> Spikard.Auth.Jwt.to_map(config)
      %{
        "secret" => "test-secret",
        "algorithm" => "HS256",
        "header_name" => "authorization",
        "required_claims" => []
      }
  """
  @spec to_map(t()) :: map()
  def to_map(config) do
    %{
      "secret" => config.secret,
      "algorithm" => algorithm_to_string(config.algorithm),
      "header_name" => config.header_name,
      "required_claims" => config.required_claims
    }
  end

  # Convert Elixir atom to Rust algorithm string
  @spec algorithm_to_string(atom()) :: String.t()
  defp algorithm_to_string(alg) do
    case alg do
      :hs256 -> "HS256"
      :hs384 -> "HS384"
      :hs512 -> "HS512"
      :rs256 -> "RS256"
      :rs384 -> "RS384"
      :rs512 -> "RS512"
      :es256 -> "ES256"
      :es384 -> "ES384"
      :ps256 -> "PS256"
      :ps384 -> "PS384"
      :ps512 -> "PS512"
      other -> raise ArgumentError, "unsupported algorithm: #{inspect(other)}"
    end
  end

  # Validate that algorithm is a supported atom
  @spec validate_algorithm!(atom()) :: :ok
  defp validate_algorithm!(alg) do
    case alg do
      :hs256 -> :ok
      :hs384 -> :ok
      :hs512 -> :ok
      :rs256 -> :ok
      :rs384 -> :ok
      :rs512 -> :ok
      :es256 -> :ok
      :es384 -> :ok
      :ps256 -> :ok
      :ps384 -> :ok
      :ps512 -> :ok
      other -> raise ArgumentError, "unsupported algorithm: #{inspect(other)}"
    end
  end
end

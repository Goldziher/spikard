defmodule Spikard.Auth do
  @moduledoc """
  Authentication configuration and utilities for Spikard.

  This module provides the main namespace for authentication-related functionality
  including JWT and API key authentication.

  ## Examples

      # Create JWT config
      jwt_config = Spikard.Auth.Jwt.new(secret: "my-secret")

      # Use with TestClient
      {:ok, client} = Spikard.TestClient.new(
        routes: routes,
        jwt_auth: jwt_config
      )

      # Or with Spikard.start
      {:ok, server} = Spikard.start(
        port: 8000,
        routes: routes,
        jwt_auth: jwt_config
      )
  """
end

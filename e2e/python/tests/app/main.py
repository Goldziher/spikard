"""Generated app creators for auth fixtures."""

from spikard import Spikard, get
from spikard.config import ServerConfig, JwtConfig, ApiKeyConfig


def create_app_auth_jwt_authentication_valid_token():
    """Create app for JWT authentication with valid token."""
    config = ServerConfig(
        jwt_auth=JwtConfig(
            secret="test-secret-key-do-not-use-in-production",
            algorithm="HS256",
            audience=["https://api.example.com"],
            issuer="https://auth.example.com",
        )
    )
    app = Spikard(config=config)

    @get("/protected/user")
    def protected_route():
        return {"message": "Access granted", "user_id": "user123"}

    return app


def create_app_auth_jwt_authentication_expired_token():
    """Create app for JWT authentication with expired token."""
    config = ServerConfig(jwt_auth=JwtConfig(secret="test-secret-key-do-not-use-in-production", algorithm="HS256"))
    app = Spikard(config=config)

    @get("/protected/user")
    def protected_route():
        return {"message": "Access granted", "user_id": "user123"}

    return app


def create_app_auth_jwt_authentication_invalid_signature():
    """Create app for JWT authentication with invalid signature."""
    config = ServerConfig(jwt_auth=JwtConfig(secret="test-secret-key-do-not-use-in-production", algorithm="HS256"))
    app = Spikard(config=config)

    @get("/protected/user")
    def protected_route():
        return {"message": "Access granted", "user_id": "user123"}

    return app


def create_app_auth_jwt_authentication_missing_authorization_header():
    """Create app for JWT authentication with missing authorization header."""
    config = ServerConfig(jwt_auth=JwtConfig(secret="test-secret-key-do-not-use-in-production", algorithm="HS256"))
    app = Spikard(config=config)

    @get("/protected/user")
    def protected_route():
        return {"message": "Access granted", "user_id": "user123"}

    return app


def create_app_auth_jwt_authentication_invalid_audience():
    """Create app for JWT authentication with invalid audience."""
    config = ServerConfig(
        jwt_auth=JwtConfig(
            secret="test-secret-key-do-not-use-in-production", algorithm="HS256", audience=["https://api.example.com"]
        )
    )
    app = Spikard(config=config)

    @get("/protected/user")
    def protected_route():
        return {"message": "Access granted", "user_id": "user123"}

    return app


def create_app_auth_api_key_authentication_valid_key():
    """Create app for API key authentication with valid key."""
    config = ServerConfig(api_key_auth=ApiKeyConfig(keys=["sk_test_123456", "sk_test_789012"], header_name="X-API-Key"))
    app = Spikard(config=config)

    @get("/api/data")
    def api_route():
        return {"message": "Access granted", "data": "sensitive information"}

    return app


def create_app_auth_api_key_authentication_invalid_key():
    """Create app for API key authentication with invalid key."""
    config = ServerConfig(api_key_auth=ApiKeyConfig(keys=["sk_test_123456"], header_name="X-API-Key"))
    app = Spikard(config=config)

    @get("/api/data")
    def api_route():
        return {"message": "Access granted", "data": "sensitive information"}

    return app


def create_app_auth_api_key_authentication_missing_header():
    """Create app for API key authentication with missing header."""
    config = ServerConfig(api_key_auth=ApiKeyConfig(keys=["sk_test_123456", "sk_test_789012"], header_name="X-API-Key"))
    app = Spikard(config=config)

    @get("/api/data")
    def api_route():
        return {"message": "Access granted", "data": "sensitive information"}

    return app

"""Manual test for authentication middleware.

This demonstrates that the auth middleware works correctly.
The test-generator needs to be updated to support middleware configuration from fixtures.
"""

import asyncio
import jwt
from datetime import datetime, timedelta, timezone

from spikard import Spikard, get
from spikard.config import ServerConfig, JwtConfig, ApiKeyConfig
from spikard.testing import TestClient


# Test JWT Authentication
def test_jwt_auth_valid_token():
    """Test JWT auth with a valid token."""
    secret = "test-secret-key"

    # Create a valid JWT token
    payload = {
        "sub": "user123",
        "exp": datetime.now(timezone.utc) + timedelta(hours=1),
        "iat": datetime.now(timezone.utc),
        "aud": ["https://api.example.com"],
        "iss": "https://auth.example.com",
    }
    token = jwt.encode(payload, secret, algorithm="HS256")

    # Create config with JWT middleware
    config = ServerConfig(
        jwt_auth=JwtConfig(
            secret=secret, algorithm="HS256", audience=["https://api.example.com"], issuer="https://auth.example.com"
        )
    )

    # Create app with config
    app = Spikard(config=config)

    @get("/protected")
    def protected_route():
        return {"message": "Access granted", "user_id": "user123"}

    # Test with valid token
    client = TestClient(app)
    response = client.get("/protected", headers={"Authorization": f"Bearer {token}"})

    print(f"✓ Valid JWT token: {response.status_code} - {response.json()}")
    assert response.status_code == 200
    assert response.json()["message"] == "Access granted"


def test_jwt_auth_missing_header():
    """Test JWT auth without Authorization header."""
    config = ServerConfig(jwt_auth=JwtConfig(secret="test-secret", algorithm="HS256"))

    app = Spikard(config=config)

    @get("/protected")
    def protected_route():
        return {"message": "Access granted"}

    client = TestClient(app)
    response = client.get("/protected")

    print(f"✓ Missing JWT token: {response.status_code} - {response.json()}")
    assert response.status_code == 401
    assert "Missing or invalid Authorization header" in response.json()["title"]


def test_jwt_auth_expired_token():
    """Test JWT auth with an expired token."""
    secret = "test-secret-key"

    # Create an expired JWT token
    payload = {
        "sub": "user123",
        "exp": datetime.now(timezone.utc) - timedelta(hours=1),  # Expired 1 hour ago
        "iat": datetime.now(timezone.utc) - timedelta(hours=2),
    }
    token = jwt.encode(payload, secret, algorithm="HS256")

    app = Spikard()

    @get("/protected")
    def protected_route():
        return {"message": "Access granted"}

    config = ServerConfig(jwt_auth=JwtConfig(secret=secret, algorithm="HS256"))

    client = TestClient(app, config=config)
    response = client.get("/protected", headers={"Authorization": f"Bearer {token}"})

    print(f"✓ Expired JWT token: {response.status_code} - {response.json()}")
    assert response.status_code == 401
    assert "expired" in response.json()["detail"].lower()


def test_api_key_auth_valid():
    """Test API key auth with a valid key."""
    app = Spikard()

    @get("/api/data")
    def api_route():
        return {"message": "Access granted", "data": "sensitive information"}

    config = ServerConfig(api_key_auth=ApiKeyConfig(keys=["sk_test_123456", "sk_test_789012"], header_name="X-API-Key"))

    client = TestClient(app, config=config)
    response = client.get("/api/data", headers={"X-API-Key": "sk_test_123456"})

    print(f"✓ Valid API key: {response.status_code} - {response.json()}")
    assert response.status_code == 200
    assert response.json()["message"] == "Access granted"


def test_api_key_auth_invalid():
    """Test API key auth with an invalid key."""
    app = Spikard()

    @get("/api/data")
    def api_route():
        return {"message": "Access granted"}

    config = ServerConfig(api_key_auth=ApiKeyConfig(keys=["sk_test_123456"], header_name="X-API-Key"))

    client = TestClient(app, config=config)
    response = client.get("/api/data", headers={"X-API-Key": "invalid_key"})

    print(f"✓ Invalid API key: {response.status_code} - {response.json()}")
    assert response.status_code == 401
    assert "Invalid API key" in response.json()["title"]


def test_api_key_auth_missing():
    """Test API key auth without the header."""
    app = Spikard()

    @get("/api/data")
    def api_route():
        return {"message": "Access granted"}

    config = ServerConfig(api_key_auth=ApiKeyConfig(keys=["sk_test_123456"], header_name="X-API-Key"))

    client = TestClient(app, config=config)
    response = client.get("/api/data")

    print(f"✓ Missing API key: {response.status_code} - {response.json()}")
    assert response.status_code == 401
    assert "Missing API key" in response.json()["title"]


if __name__ == "__main__":
    print("Testing JWT Authentication Middleware\n" + "=" * 50)
    test_jwt_auth_valid_token()
    test_jwt_auth_missing_header()
    test_jwt_auth_expired_token()

    print("\nTesting API Key Authentication Middleware\n" + "=" * 50)
    test_api_key_auth_valid()
    test_api_key_auth_invalid()
    test_api_key_auth_missing()

    print("\n" + "=" * 50)
    print("✅ All authentication tests passed!")

"""E2E tests for auth."""

from spikard.testing import TestClient
from app.main import (
    create_app_auth_api_key_authentication_invalid_key,
    create_app_auth_api_key_authentication_missing_header,
    create_app_auth_api_key_authentication_valid_key,
    create_app_auth_jwt_authentication_expired_token,
    create_app_auth_jwt_authentication_invalid_audience,
    create_app_auth_jwt_authentication_invalid_signature,
    create_app_auth_jwt_authentication_missing_authorization_header,
    create_app_auth_jwt_authentication_valid_token,
)


async def test_jwt_authentication_valid_token() -> None:
    """Tests JWT authentication with a valid token containing correct signature and claims."""

    app = create_app_auth_jwt_authentication_valid_token()
    client = TestClient(app)

    headers = {
        "Authorization": "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2F1dGguZXhhbXBsZS5jb20ifQ.TpRpCJeXROQ12-ehRCVZm6EgN7Dn6QpfoekxJvnzgQg",
    }
    response = await client.get("/protected/user", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Access granted"
    assert "user_id" in response_data
    assert response_data["user_id"] == "user123"


async def test_jwt_authentication_expired_token() -> None:
    """Tests JWT authentication failure when token has expired (exp claim in the past)."""

    app = create_app_auth_jwt_authentication_expired_token()
    client = TestClient(app)

    headers = {
        "Authorization": "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoxNjAwMDAwMDAwLCJpYXQiOjE1OTAwMDAwMDB9.n4oBw9XuO2aAJWi1e4Bz9Y_m2iEyJHGAODcetNuwYFo",
    }
    response = await client.get("/protected/user", headers=headers)

    assert response.status_code == 401
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Token has expired"
    assert "status" in response_data
    assert response_data["status"] == 401
    assert "title" in response_data
    assert response_data["title"] == "JWT validation failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/unauthorized"


async def test_api_key_authentication_invalid_key() -> None:
    """Tests API key authentication failure when provided key is not in the valid keys list."""

    app = create_app_auth_api_key_authentication_invalid_key()
    client = TestClient(app)

    headers = {
        "X-API-Key": "invalid_key_12345",
    }
    response = await client.get("/api/data", headers=headers)

    assert response.status_code == 401
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "The provided API key is not valid"
    assert "status" in response_data
    assert response_data["status"] == 401
    assert "title" in response_data
    assert response_data["title"] == "Invalid API key"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/unauthorized"


async def test_api_key_authentication_valid_key() -> None:
    """Tests API key authentication with a valid API key in custom header."""

    app = create_app_auth_api_key_authentication_valid_key()
    client = TestClient(app)

    headers = {
        "X-API-Key": "sk_test_123456",
    }
    response = await client.get("/api/data", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "data" in response_data
    assert response_data["data"] == "sensitive information"
    assert "message" in response_data
    assert response_data["message"] == "Access granted"


async def test_api_key_authentication_missing_header() -> None:
    """Tests API key authentication failure when required header is not provided."""

    app = create_app_auth_api_key_authentication_missing_header()
    client = TestClient(app)

    response = await client.get("/api/data")

    assert response.status_code == 401
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Expected 'X-API-Key' header with valid API key"
    assert "status" in response_data
    assert response_data["status"] == 401
    assert "title" in response_data
    assert response_data["title"] == "Missing API key"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/unauthorized"


async def test_jwt_authentication_invalid_signature() -> None:
    """Tests JWT authentication failure when token signature does not match the secret."""

    app = create_app_auth_jwt_authentication_invalid_signature()
    client = TestClient(app)

    headers = {
        "Authorization": "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNTM0MDIzMDA3OTksImlhdCI6MTczMTI1MjAwMH0.invalid_signature_here",
    }
    response = await client.get("/protected/user", headers=headers)

    assert response.status_code == 401
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Token signature is invalid"
    assert "status" in response_data
    assert response_data["status"] == 401
    assert "title" in response_data
    assert response_data["title"] == "JWT validation failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/unauthorized"


async def test_jwt_authentication_missing_authorization_header() -> None:
    """Tests JWT authentication failure when Authorization header is not provided."""

    app = create_app_auth_jwt_authentication_missing_authorization_header()
    client = TestClient(app)

    response = await client.get("/protected/user")

    assert response.status_code == 401
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Expected 'Authorization: Bearer <token>'"
    assert "status" in response_data
    assert response_data["status"] == 401
    assert "title" in response_data
    assert response_data["title"] == "Missing or invalid Authorization header"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/unauthorized"


async def test_jwt_authentication_invalid_audience() -> None:
    """Tests JWT authentication failure when token audience claim does not match required audience."""

    app = create_app_auth_jwt_authentication_invalid_audience()
    client = TestClient(app)

    headers = {
        "Authorization": "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNTM0MDIzMDA3OTk5LCJpYXQiOjE3MzEyNTIwMDAsImF1ZCI6WyJodHRwczovL3dyb25nLXNlcnZpY2UuY29tIl19.YR2a9fSJjhen7ksYFI2djSBSC7Pc29FDCloBGhkj3kU",
    }
    response = await client.get("/protected/user", headers=headers)

    assert response.status_code == 401
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Token audience is invalid"
    assert "status" in response_data
    assert response_data["status"] == 401
    assert "title" in response_data
    assert response_data["title"] == "JWT validation failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/unauthorized"

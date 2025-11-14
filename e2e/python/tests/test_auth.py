"""E2E tests for auth."""

from spikard.testing import TestClient
from app.main import (
    create_app_auth_api_key_authentication_invalid_key,
    create_app_auth_api_key_authentication_missing_header,
    create_app_auth_api_key_authentication_valid_key,
    create_app_auth_api_key_in_query_parameter,
    create_app_auth_api_key_rotation_old_key_still_valid,
    create_app_auth_api_key_with_custom_header_name,
    create_app_auth_bearer_token_without_prefix,
    create_app_auth_jwt_authentication_expired_token,
    create_app_auth_jwt_authentication_invalid_audience,
    create_app_auth_jwt_authentication_invalid_signature,
    create_app_auth_jwt_authentication_missing_authorization_header,
    create_app_auth_jwt_authentication_valid_token,
    create_app_auth_jwt_invalid_issuer,
    create_app_auth_jwt_malformed_token_format,
    create_app_auth_jwt_missing_required_custom_claims,
    create_app_auth_jwt_not_before_claim_in_future,
    create_app_auth_jwt_with_multiple_audiences,
    create_app_auth_multiple_authentication_schemes_jwt_precedence,
)


async def test_jwt_malformed_token_format() -> None:
    """Tests JWT rejection when token doesn't have the required 3-part structure (header.payload.signature)."""

    async with TestClient(create_app_auth_jwt_malformed_token_format()) as client:
        headers = {
            "Authorization": "Bearer invalid.token",
        }
        response = await client.get("/api/protected", headers=headers)

        assert response.status_code == 401
        response_data = response.json()
        assert "detail" in response_data
        assert response_data["detail"] == "Malformed JWT token: expected 3 parts separated by dots, found 2"
        assert "status" in response_data
        assert response_data["status"] == 401
        assert "title" in response_data
        assert response_data["title"] == "Malformed JWT token"
        assert "type" in response_data
        assert response_data["type"] == "https://spikard.dev/errors/unauthorized"


async def test_bearer_token_without_prefix() -> None:
    """Tests JWT rejection when token is provided without 'Bearer ' prefix in Authorization header."""

    async with TestClient(create_app_auth_bearer_token_without_prefix()) as client:
        headers = {
            "Authorization": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDZ9.8yXqZ9jKCR0BwqJc7pN_QvD3mYLxHfWzUeIaGkTnOsA",
        }
        response = await client.get("/api/protected", headers=headers)

        assert response.status_code == 401
        response_data = response.json()
        assert "detail" in response_data
        assert response_data["detail"] == "Authorization header must use Bearer scheme: 'Bearer <token>'"
        assert "status" in response_data
        assert response_data["status"] == 401
        assert "title" in response_data
        assert response_data["title"] == "Invalid Authorization header format"
        assert "type" in response_data
        assert response_data["type"] == "https://spikard.dev/errors/unauthorized"


async def test_jwt_authentication_valid_token() -> None:
    """Tests JWT authentication with a valid token containing correct signature and claims."""

    async with TestClient(create_app_auth_jwt_authentication_valid_token()) as client:
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


async def test_api_key_rotation_old_key_still_valid() -> None:
    """Tests API key authentication during rotation period when old key remains valid alongside new key."""

    async with TestClient(create_app_auth_api_key_rotation_old_key_still_valid()) as client:
        headers = {
            "X-API-Key": "sk_test_old_123456",
        }
        response = await client.get("/api/data", headers=headers)

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert response_data["data"] == "sensitive information"
        assert "message" in response_data
        assert response_data["message"] == "Access granted"
        response_headers = response.headers
        assert response_headers.get("x-api-key-deprecated") == "true"


async def test_jwt_invalid_issuer() -> None:
    """Tests JWT rejection when issuer claim doesn't match expected value."""

    async with TestClient(create_app_auth_jwt_invalid_issuer()) as client:
        headers = {
            "Authorization": "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE2MDAwMDAwMDAsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2V2aWwuY29tIn0.mbL5L04_hpaaiz0SPABap6ZWfBLu18aiexBjzwQ1nnA",
        }
        response = await client.get("/api/protected", headers=headers)

        assert response.status_code == 401
        response_data = response.json()
        assert "detail" in response_data
        assert response_data["detail"] == "Token issuer is invalid, expected 'https://auth.example.com'"
        assert "status" in response_data
        assert response_data["status"] == 401
        assert "title" in response_data
        assert response_data["title"] == "JWT validation failed"
        assert "type" in response_data
        assert response_data["type"] == "https://spikard.dev/errors/unauthorized"


async def test_jwt_with_multiple_audiences() -> None:
    """Tests JWT validation when token has multiple audiences and one must match."""

    async with TestClient(create_app_auth_jwt_with_multiple_audiences()) as client:
        headers = {
            "Authorization": "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE2MDAwMDAwMDAsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSIsImh0dHBzOi8vYWRtaW4uZXhhbXBsZS5jb20iXSwiaXNzIjoiaHR0cHM6Ly9hdXRoLmV4YW1wbGUuY29tIn0.9MBL_XccGXfu9cDUnCpQruDMOl2hHYydzeGn-20dQOs",
        }
        response = await client.get("/api/protected", headers=headers)

        assert response.status_code == 200
        response_data = response.json()
        assert "message" in response_data
        assert response_data["message"] == "Access granted"
        assert "user_id" in response_data
        assert response_data["user_id"] == "user123"


async def test_api_key_in_query_parameter() -> None:
    """Tests API key authentication when key is provided as query parameter instead of header."""

    async with TestClient(create_app_auth_api_key_in_query_parameter()) as client:
        response = await client.get("/api/data?api_key=sk_test_123456")

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert response_data["data"] == "sensitive information"
        assert "message" in response_data
        assert response_data["message"] == "Access granted"


async def test_jwt_authentication_expired_token() -> None:
    """Tests JWT authentication failure when token has expired (exp claim in the past)."""

    async with TestClient(create_app_auth_jwt_authentication_expired_token()) as client:
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

    async with TestClient(create_app_auth_api_key_authentication_invalid_key()) as client:
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


async def test_jwt_not_before_claim_in_future() -> None:
    """Tests JWT rejection when nbf (not before) claim is in the future."""

    async with TestClient(create_app_auth_jwt_not_before_claim_in_future()) as client:
        headers = {
            "Authorization": "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE2MDAwMDAwMDAsIm5iZiI6MjYyNjc4Mzk0NiwiYXVkIjpbImh0dHBzOi8vYXBpLmV4YW1wbGUuY29tIl0sImlzcyI6Imh0dHBzOi8vYXV0aC5leGFtcGxlLmNvbSJ9.hG4I76_3kJfsbJ_jmxoP1NSYnkcqdyBFcPpdo-jYU4E",
        }
        response = await client.get("/api/protected", headers=headers)

        assert response.status_code == 401
        response_data = response.json()
        assert "detail" in response_data
        assert response_data["detail"] == "JWT not valid yet, not before claim is in the future"
        assert "status" in response_data
        assert response_data["status"] == 401
        assert "title" in response_data
        assert response_data["title"] == "JWT validation failed"
        assert "type" in response_data
        assert response_data["type"] == "https://spikard.dev/errors/unauthorized"


async def test_multiple_authentication_schemes_jwt_precedence() -> None:
    """Tests authentication when both JWT and API key are provided, JWT takes precedence."""

    async with TestClient(create_app_auth_multiple_authentication_schemes_jwt_precedence()) as client:
        headers = {
            "Authorization": "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2F1dGguZXhhbXBsZS5jb20ifQ.TpRpCJeXROQ12-ehRCVZm6EgN7Dn6QpfoekxJvnzgQg",
            "X-API-Key": "sk_test_123456",
        }
        response = await client.get("/api/data", headers=headers)

        assert response.status_code == 200
        response_data = response.json()
        assert "auth_method" in response_data
        assert response_data["auth_method"] == "jwt"
        assert "message" in response_data
        assert response_data["message"] == "Access granted"
        assert "user_id" in response_data
        assert response_data["user_id"] == "user123"


async def test_jwt_missing_required_custom_claims() -> None:
    """Tests JWT rejection when required custom claims (role, permissions) are missing."""

    async with TestClient(create_app_auth_jwt_missing_required_custom_claims()) as client:
        headers = {
            "Authorization": "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2F1dGguZXhhbXBsZS5jb20ifQ.TpRpCJeXROQ12-ehRCVZm6EgN7Dn6QpfoekxJvnzgQg",
        }
        response = await client.get("/api/admin", headers=headers)

        assert response.status_code == 403
        response_data = response.json()
        assert "detail" in response_data
        assert response_data["detail"] == "Required claims 'role' and 'permissions' missing from JWT"
        assert "status" in response_data
        assert response_data["status"] == 403
        assert "title" in response_data
        assert response_data["title"] == "Forbidden"
        assert "type" in response_data
        assert response_data["type"] == "https://spikard.dev/errors/forbidden"


async def test_api_key_authentication_valid_key() -> None:
    """Tests API key authentication with a valid API key in custom header."""

    async with TestClient(create_app_auth_api_key_authentication_valid_key()) as client:
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


async def test_api_key_with_custom_header_name() -> None:
    """Tests API key authentication with a custom header name (X-API-Token instead of X-API-Key)."""

    async with TestClient(create_app_auth_api_key_with_custom_header_name()) as client:
        headers = {
            "X-API-Token": "sk_test_123456",
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

    async with TestClient(create_app_auth_api_key_authentication_missing_header()) as client:
        response = await client.get("/api/data")

        assert response.status_code == 401
        response_data = response.json()
        assert "detail" in response_data
        assert response_data["detail"] == "Expected 'X-API-Key' header or 'api_key' query parameter with valid API key"
        assert "status" in response_data
        assert response_data["status"] == 401
        assert "title" in response_data
        assert response_data["title"] == "Missing API key"
        assert "type" in response_data
        assert response_data["type"] == "https://spikard.dev/errors/unauthorized"


async def test_jwt_authentication_invalid_signature() -> None:
    """Tests JWT authentication failure when token signature does not match the secret."""

    async with TestClient(create_app_auth_jwt_authentication_invalid_signature()) as client:
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

    async with TestClient(create_app_auth_jwt_authentication_missing_authorization_header()) as client:
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

    async with TestClient(create_app_auth_jwt_authentication_invalid_audience()) as client:
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

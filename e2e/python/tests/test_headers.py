"""E2E tests for headers."""

import pytest
from typing import Any

async def test_header_regex_validation_success() -> None:
    """Tests header with regex pattern validation success."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_header_regex_validation_success

    app = create_app_headers_header_regex_validation_success()
    client = TestClient(app)

    headers = {
        "X-Request-Id": "12345",
    }
    response = await client.get("/headers/pattern", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_33_api_key_header_valid() -> None:
    """X-API-Key header with valid format should be accepted."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_33_api_key_header_valid

    app = create_app_headers_33_api_key_header_valid()
    client = TestClient(app)

    headers = {
        "X-API-Key": "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4",
    }
    response = await client.get("/api/data", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_content_type_header_application_json() -> None:
    """Tests Content-Type header with JSON media type."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_content_type_header_application_json

    app = create_app_headers_content_type_header_application_json()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    response = await client.get("/headers/content-type", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_accept_language_header() -> None:
    """Tests Accept-Language header for locale/i18n."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_accept_language_header

    app = create_app_headers_accept_language_header()
    client = TestClient(app)

    headers = {
        "Accept-Language": "en-US,en;q=0.9",
    }
    response = await client.get("/headers/accept-language", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_x_api_key_required_header_success() -> None:
    """Tests required X-API-Key header with valid value."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_x_api_key_required_header_success

    app = create_app_headers_x_api_key_required_header_success()
    client = TestClient(app)

    headers = {
        "key": "secret",
    }
    response = await client.get("/users/me", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_header_validation_max_length_constraint_fail() -> None:
    """Tests header validation with max_length constraint failure."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_header_validation_max_length_constraint_fail

    app = create_app_headers_header_validation_max_length_constraint_fail()
    client = TestClient(app)

    headers = {
        "X-Session-Id": "this_is_way_too_long_for_validation",
    }
    response = await client.get("/headers/max-length", headers=headers)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_x_api_key_required_header_missing() -> None:
    """Tests required X-API-Key header when not provided, returns 403."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_x_api_key_required_header_missing

    app = create_app_headers_x_api_key_required_header_missing()
    client = TestClient(app)

    response = await client.get("/users/me")

    assert response.status_code == 403
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Not authenticated"


async def test_origin_header() -> None:
    """Tests Origin header for CORS."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_origin_header

    app = create_app_headers_origin_header()
    client = TestClient(app)

    headers = {
        "Origin": "https://example.com",
    }
    response = await client.get("/headers/origin", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_user_agent_header_default_value() -> None:
    """Tests optional User-Agent header when not provided, uses testclient default."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_user_agent_header_default_value

    app = create_app_headers_user_agent_header_default_value()
    client = TestClient(app)

    response = await client.get("/items/")

    assert response.status_code == 200
    response_data = response.json()


async def test_32_bearer_token_missing_prefix() -> None:
    """Authorization header without Bearer prefix should fail validation."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_32_bearer_token_missing_prefix

    app = create_app_headers_32_bearer_token_missing_prefix()
    client = TestClient(app)

    headers = {
        "Authorization": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9",
    }
    response = await client.get("/protected", headers=headers)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_optional_header_with_none_default_missing() -> None:
    """Tests optional header parameter with None default when not provided."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_optional_header_with_none_default_missing

    app = create_app_headers_optional_header_with_none_default_missing()
    client = TestClient(app)

    response = await client.get("/items/")

    assert response.status_code == 200
    response_data = response.json()


async def test_header_regex_validation_fail() -> None:
    """Tests header with regex pattern validation failure."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_header_regex_validation_fail

    app = create_app_headers_header_regex_validation_fail()
    client = TestClient(app)

    headers = {
        "X-Request-Id": "invalid-format",
    }
    response = await client.get("/headers/pattern", headers=headers)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_31_bearer_token_format_invalid() -> None:
    """Authorization header with invalid Bearer token format should fail validation."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_31_bearer_token_format_invalid

    app = create_app_headers_31_bearer_token_format_invalid()
    client = TestClient(app)

    headers = {
        "Authorization": "Bearer invalid token with spaces",
    }
    response = await client.get("/protected", headers=headers)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_x_api_key_optional_header_success() -> None:
    """Tests optional X-API-Key header with valid value."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_x_api_key_optional_header_success

    app = create_app_headers_x_api_key_optional_header_success()
    client = TestClient(app)

    headers = {
        "key": "secret",
    }
    response = await client.get("/users/me", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_authorization_header_success() -> None:
    """Tests Authorization header with valid Digest scheme."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_authorization_header_success

    app = create_app_headers_authorization_header_success()
    client = TestClient(app)

    headers = {
        "Authorization": "Digest foobar",
    }
    response = await client.get("/users/me", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_30_bearer_token_format_valid() -> None:
    """Authorization header with valid Bearer token format should be accepted."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_30_bearer_token_format_valid

    app = create_app_headers_30_bearer_token_format_valid()
    client = TestClient(app)

    headers = {
        "Authorization": "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U",
    }
    response = await client.get("/protected", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_authorization_header_missing() -> None:
    """Tests missing Authorization header returns 403."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_authorization_header_missing

    app = create_app_headers_authorization_header_missing()
    client = TestClient(app)

    response = await client.get("/users/me")

    assert response.status_code == 403
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Not authenticated"


async def test_accept_header_json() -> None:
    """Tests Accept header for content negotiation."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_accept_header_json

    app = create_app_headers_accept_header_json()
    client = TestClient(app)

    headers = {
        "Accept": "application/json",
    }
    response = await client.get("/headers/accept", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_accept_encoding_header() -> None:
    """Tests Accept-Encoding header for compression negotiation."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_accept_encoding_header

    app = create_app_headers_accept_encoding_header()
    client = TestClient(app)

    headers = {
        "Accept-Encoding": "gzip, deflate, br",
    }
    response = await client.get("/headers/accept-encoding", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_authorization_header_wrong_scheme() -> None:
    """Tests Authorization header with incorrect scheme returns 403."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_authorization_header_wrong_scheme

    app = create_app_headers_authorization_header_wrong_scheme()
    client = TestClient(app)

    headers = {
        "Authorization": "Other invalidauthorization",
    }
    response = await client.get("/users/me", headers=headers)

    assert response.status_code == 403
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Invalid authentication credentials"


async def test_header_validation_min_length_constraint() -> None:
    """Tests header validation with min_length constraint."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_header_validation_min_length_constraint

    app = create_app_headers_header_validation_min_length_constraint()
    client = TestClient(app)

    headers = {
        "X-Token": "ab",
    }
    response = await client.get("/headers/validated", headers=headers)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_basic_authentication_success() -> None:
    """Tests Authorization header with Basic auth scheme."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_basic_authentication_success

    app = create_app_headers_basic_authentication_success()
    client = TestClient(app)

    headers = {
        "Authorization": "Basic dXNlcm5hbWU6cGFzc3dvcmQ=",
    }
    response = await client.get("/headers/basic-auth", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_bearer_token_authentication_missing() -> None:
    """Tests missing Bearer token returns 401 Unauthorized."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_bearer_token_authentication_missing

    app = create_app_headers_bearer_token_authentication_missing()
    client = TestClient(app)

    response = await client.get("/headers/bearer-auth")

    assert response.status_code == 401
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Not authenticated"


async def test_x_api_key_optional_header_missing() -> None:
    """Tests optional X-API-Key header when not provided, returns fallback message."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_x_api_key_optional_header_missing

    app = create_app_headers_x_api_key_optional_header_missing()
    client = TestClient(app)

    response = await client.get("/users/me")

    assert response.status_code == 200
    response_data = response.json()


async def test_multiple_custom_headers() -> None:
    """Tests multiple custom headers in single request."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_multiple_custom_headers

    app = create_app_headers_multiple_custom_headers()
    client = TestClient(app)

    headers = {
        "X-Request-Id": "req-12345",
        "X-Client-Version": "1.2.3",
        "X-Trace-Id": "trace-abc",
    }
    response = await client.get("/headers/multiple", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_34_api_key_header_invalid() -> None:
    """X-API-Key header with invalid format should fail validation."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_34_api_key_header_invalid

    app = create_app_headers_34_api_key_header_invalid()
    client = TestClient(app)

    headers = {
        "X-API-Key": "invalid-key",
    }
    response = await client.get("/api/data", headers=headers)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_bearer_token_authentication_success() -> None:
    """Tests Authorization header with Bearer token scheme."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_bearer_token_authentication_success

    app = create_app_headers_bearer_token_authentication_success()
    client = TestClient(app)

    headers = {
        "Authorization": "Bearer valid_token_123",
    }
    response = await client.get("/headers/bearer-auth", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_host_header() -> None:
    """Tests Host header (standard HTTP header)."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_host_header

    app = create_app_headers_host_header()
    client = TestClient(app)

    headers = {
        "Host": "example.com:8080",
    }
    response = await client.get("/headers/host", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_referer_header() -> None:
    """Tests Referer header (standard misspelling)."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_referer_header

    app = create_app_headers_referer_header()
    client = TestClient(app)

    headers = {
        "Referer": "https://example.com/page",
    }
    response = await client.get("/headers/referer", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_header_with_underscore_conversion_explicit() -> None:
    """Tests X-Token header converted to x_token parameter."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_header_with_underscore_conversion_explicit

    app = create_app_headers_header_with_underscore_conversion_explicit()
    client = TestClient(app)

    headers = {
        "X-Token": "secret123",
    }
    response = await client.get("/headers/underscore", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_header_case_insensitivity_access() -> None:
    """Tests case-insensitive header access (Content-Type vs content-type)."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_header_case_insensitivity_access

    app = create_app_headers_header_case_insensitivity_access()
    client = TestClient(app)

    headers = {
        "content-type": "application/json",
    }
    json_data = {"test": "data"}
    response = await client.post("/echo", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "test" in response_data
    assert response_data["test"] == "data"


async def test_user_agent_header_custom_value() -> None:
    """Tests User-Agent header with custom value."""
    from spikard.testing import TestClient
    from app.main import create_app_headers_user_agent_header_custom_value

    app = create_app_headers_user_agent_header_custom_value()
    client = TestClient(app)

    headers = {
        "User-Agent": "Mozilla/5.0 Custom Browser",
    }
    response = await client.get("/items/", headers=headers)

    assert response.status_code == 200
    response_data = response.json()



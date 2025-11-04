"""E2E tests for cors."""

from app.main import (
    create_app_cors_06_cors_preflight_method_not_allowed,
    create_app_cors_07_cors_preflight_header_not_allowed,
    create_app_cors_08_cors_max_age,
    create_app_cors_09_cors_expose_headers,
    create_app_cors_10_cors_origin_null,
    create_app_cors_cors_preflight_request,
    create_app_cors_cors_request_blocked,
    create_app_cors_cors_wildcard_origin,
    create_app_cors_cors_with_credentials,
    create_app_cors_simple_cors_request,
)

from spikard.testing import TestClient


async def test_07_cors_preflight_header_not_allowed() -> None:
    """CORS preflight request with non-allowed header should be rejected."""

    app = create_app_cors_07_cors_preflight_header_not_allowed()
    client = TestClient(app)

    headers = {
        "Access-Control-Request-Method": "POST",
        "Access-Control-Request-Headers": "X-Custom-Header",
        "Origin": "https://example.com",
    }
    response = await client.options("/api/data", headers=headers)

    assert response.status_code == 403


async def test_cors_preflight_request() -> None:
    """Tests OPTIONS preflight request for CORS."""

    app = create_app_cors_cors_preflight_request()
    client = TestClient(app)

    headers = {
        "Access-Control-Request-Method": "POST",
        "Access-Control-Request-Headers": "Content-Type, X-Custom-Header",
        "Origin": "https://example.com",
    }
    response = await client.options("/items/", headers=headers)

    assert response.status_code == 200
    response.json()


async def test_cors_with_credentials() -> None:
    """Tests CORS request with credentials (cookies, auth headers)."""

    app = create_app_cors_cors_with_credentials()
    client = TestClient(app)

    headers = {
        "Cookie": "session=abc123",
        "Origin": "https://app.example.com",
    }
    response = await client.get("/api/user/profile", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "username" in response_data
    assert response_data["username"] == "john"


async def test_08_cors_max_age() -> None:
    """CORS preflight response should include Access-Control-Max-Age."""

    app = create_app_cors_08_cors_max_age()
    client = TestClient(app)

    headers = {
        "Origin": "https://example.com",
        "Access-Control-Request-Method": "POST",
        "Access-Control-Request-Headers": "Content-Type",
    }
    response = await client.options("/api/data", headers=headers)

    assert response.status_code == 204


async def test_10_cors_origin_null() -> None:
    """CORS request with 'null' origin should be handled according to policy."""

    app = create_app_cors_10_cors_origin_null()
    client = TestClient(app)

    headers = {
        "Origin": "null",
    }
    response = await client.get("/api/data", headers=headers)

    assert response.status_code == 403
    response_data = response.json()
    assert "error" in response_data
    assert response_data["error"] == "Origin 'null' is not allowed"


async def test_cors_wildcard_origin() -> None:
    """Tests CORS with wildcard allowing all origins."""

    app = create_app_cors_cors_wildcard_origin()
    client = TestClient(app)

    headers = {
        "Origin": "https://random-site.com",
    }
    response = await client.get("/public/data", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "data" in response_data
    assert response_data["data"] == "public"


async def test_cors_request_blocked() -> None:
    """Tests CORS request from disallowed origin."""

    app = create_app_cors_cors_request_blocked()
    client = TestClient(app)

    headers = {
        "Origin": "https://malicious-site.com",
    }
    response = await client.get("/items/", headers=headers)

    assert response.status_code == 403
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "CORS request from origin 'https://malicious-site.com' not allowed"


async def test_simple_cors_request() -> None:
    """Tests simple CORS request with Origin header."""

    app = create_app_cors_simple_cors_request()
    client = TestClient(app)

    headers = {
        "Origin": "https://example.com",
    }
    response = await client.get("/items/", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "items" in response_data
    assert len(response_data["items"]) == 0


async def test_09_cors_expose_headers() -> None:
    """CORS response should include Access-Control-Expose-Headers for custom headers."""

    app = create_app_cors_09_cors_expose_headers()
    client = TestClient(app)

    headers = {
        "Origin": "https://example.com",
    }
    response = await client.get("/api/data", headers=headers)

    assert response.status_code == 200
    response.json()


async def test_06_cors_preflight_method_not_allowed() -> None:
    """CORS preflight request for non-allowed method should be rejected."""

    app = create_app_cors_06_cors_preflight_method_not_allowed()
    client = TestClient(app)

    headers = {
        "Origin": "https://example.com",
        "Access-Control-Request-Method": "DELETE",
        "Access-Control-Request-Headers": "Content-Type",
    }
    response = await client.options("/api/data", headers=headers)

    assert response.status_code == 403

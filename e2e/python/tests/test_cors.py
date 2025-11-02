"""E2E tests for cors."""

from typing import Any


async def test_07_cors_preflight_header_not_allowed(client: Any) -> None:
    """CORS preflight request with non-allowed header should be rejected."""
    headers = {
        "Origin": "https://example.com",
        "Access-Control-Request-Method": "POST",
        "Access-Control-Request-Headers": "X-Custom-Header",
    }
    response = await client.options("/api/data", headers=headers)

    assert response.status_code == 403


async def test_cors_preflight_request(client: Any) -> None:
    """Tests OPTIONS preflight request for CORS."""
    headers = {
        "Access-Control-Request-Method": "POST",
        "Origin": "https://example.com",
        "Access-Control-Request-Headers": "Content-Type, X-Custom-Header",
    }
    response = await client.options("/items/", headers=headers)

    assert response.status_code == 200


async def test_cors_with_credentials(client: Any) -> None:
    """Tests CORS request with credentials (cookies, auth headers)."""
    headers = {
        "Origin": "https://app.example.com",
        "Cookie": "session=abc123",
    }
    response = await client.get("/api/user/profile", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "username" in response_data
    assert response_data["username"] == "john"


async def test_08_cors_max_age(client: Any) -> None:
    """CORS preflight response should include Access-Control-Max-Age."""
    headers = {
        "Access-Control-Request-Method": "POST",
        "Access-Control-Request-Headers": "Content-Type",
        "Origin": "https://example.com",
    }
    response = await client.options("/api/data", headers=headers)

    assert response.status_code == 204


async def test_10_cors_origin_null(client: Any) -> None:
    """CORS request with 'null' origin should be handled according to policy."""
    headers = {
        "Origin": "null",
    }
    response = await client.get("/api/data", headers=headers)

    assert response.status_code == 403
    response_data = response.json()
    assert "error" in response_data
    assert response_data["error"] == "Origin 'null' is not allowed"


async def test_cors_wildcard_origin(client: Any) -> None:
    """Tests CORS with wildcard allowing all origins."""
    headers = {
        "Origin": "https://random-site.com",
    }
    response = await client.get("/public/data", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "data" in response_data
    assert response_data["data"] == "public"


async def test_cors_request_blocked(client: Any) -> None:
    """Tests CORS request from disallowed origin."""
    headers = {
        "Origin": "https://malicious-site.com",
    }
    response = await client.get("/items/", headers=headers)

    assert response.status_code == 403
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "CORS request from origin 'https://malicious-site.com' not allowed"


async def test_simple_cors_request(client: Any) -> None:
    """Tests simple CORS request with Origin header."""
    headers = {
        "Origin": "https://example.com",
    }
    response = await client.get("/items/", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "items" in response_data
    assert len(response_data["items"]) == 0


async def test_09_cors_expose_headers(client: Any) -> None:
    """CORS response should include Access-Control-Expose-Headers for custom headers."""
    headers = {
        "Origin": "https://example.com",
    }
    response = await client.get("/api/data", headers=headers)

    assert response.status_code == 200


async def test_06_cors_preflight_method_not_allowed(client: Any) -> None:
    """CORS preflight request for non-allowed method should be rejected."""
    headers = {
        "Origin": "https://example.com",
        "Access-Control-Request-Method": "DELETE",
        "Access-Control-Request-Headers": "Content-Type",
    }
    response = await client.options("/api/data", headers=headers)

    assert response.status_code == 403

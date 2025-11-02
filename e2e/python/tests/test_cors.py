"""E2E tests for cors."""

import pytest
from typing import Any

async def test_07_cors_preflight_header_not_allowed() -> None:
    """CORS preflight request with non-allowed header should be rejected."""
    from spikard.testing import TestClient
    from app.main import create_app_cors_07_cors_preflight_header_not_allowed

    app = create_app_cors_07_cors_preflight_header_not_allowed()
    client = TestClient(app)

    headers = {
        "Origin": "https://example.com",
        "Access-Control-Request-Headers": "X-Custom-Header",
        "Access-Control-Request-Method": "POST",
    }
    response = await client.options("/api/data", headers=headers)

    assert response.status_code == 403


async def test_cors_preflight_request() -> None:
    """Tests OPTIONS preflight request for CORS."""
    from spikard.testing import TestClient
    from app.main import create_app_cors_cors_preflight_request

    app = create_app_cors_cors_preflight_request()
    client = TestClient(app)

    headers = {
        "Origin": "https://example.com",
        "Access-Control-Request-Method": "POST",
        "Access-Control-Request-Headers": "Content-Type, X-Custom-Header",
    }
    response = await client.options("/items/", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_cors_with_credentials() -> None:
    """Tests CORS request with credentials (cookies, auth headers)."""
    from spikard.testing import TestClient
    from app.main import create_app_cors_cors_with_credentials

    app = create_app_cors_cors_with_credentials()
    client = TestClient(app)

    headers = {
        "Origin": "https://app.example.com",
        "Cookie": "session=abc123",
    }
    response = await client.get("/api/user/profile", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_08_cors_max_age() -> None:
    """CORS preflight response should include Access-Control-Max-Age."""
    from spikard.testing import TestClient
    from app.main import create_app_cors_08_cors_max_age

    app = create_app_cors_08_cors_max_age()
    client = TestClient(app)

    headers = {
        "Access-Control-Request-Method": "POST",
        "Origin": "https://example.com",
        "Access-Control-Request-Headers": "Content-Type",
    }
    response = await client.options("/api/data", headers=headers)

    assert response.status_code == 204


async def test_10_cors_origin_null() -> None:
    """CORS request with 'null' origin should be handled according to policy."""
    from spikard.testing import TestClient
    from app.main import create_app_cors_10_cors_origin_null

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
    from spikard.testing import TestClient
    from app.main import create_app_cors_cors_wildcard_origin

    app = create_app_cors_cors_wildcard_origin()
    client = TestClient(app)

    headers = {
        "Origin": "https://random-site.com",
    }
    response = await client.get("/public/data", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_cors_request_blocked() -> None:
    """Tests CORS request from disallowed origin."""
    from spikard.testing import TestClient
    from app.main import create_app_cors_cors_request_blocked

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
    from spikard.testing import TestClient
    from app.main import create_app_cors_simple_cors_request

    app = create_app_cors_simple_cors_request()
    client = TestClient(app)

    headers = {
        "Origin": "https://example.com",
    }
    response = await client.get("/items/", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_09_cors_expose_headers() -> None:
    """CORS response should include Access-Control-Expose-Headers for custom headers."""
    from spikard.testing import TestClient
    from app.main import create_app_cors_09_cors_expose_headers

    app = create_app_cors_09_cors_expose_headers()
    client = TestClient(app)

    headers = {
        "Origin": "https://example.com",
    }
    response = await client.get("/api/data", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_06_cors_preflight_method_not_allowed() -> None:
    """CORS preflight request for non-allowed method should be rejected."""
    from spikard.testing import TestClient
    from app.main import create_app_cors_06_cors_preflight_method_not_allowed

    app = create_app_cors_06_cors_preflight_method_not_allowed()
    client = TestClient(app)

    headers = {
        "Access-Control-Request-Headers": "Content-Type",
        "Origin": "https://example.com",
        "Access-Control-Request-Method": "DELETE",
    }
    response = await client.options("/api/data", headers=headers)

    assert response.status_code == 403



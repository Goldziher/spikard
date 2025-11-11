"""E2E tests for cors."""

from spikard.testing import TestClient
from app.main import (
    create_app_cors_06_cors_preflight_method_not_allowed,
    create_app_cors_07_cors_preflight_header_not_allowed,
    create_app_cors_08_cors_max_age,
    create_app_cors_09_cors_expose_headers,
    create_app_cors_10_cors_origin_null,
    create_app_cors_cors_multiple_allowed_origins,
    create_app_cors_cors_origin_case_sensitivity,
    create_app_cors_cors_preflight_for_delete_method,
    create_app_cors_cors_preflight_for_put_method,
    create_app_cors_cors_preflight_request,
    create_app_cors_cors_private_network_access,
    create_app_cors_cors_regex_pattern_matching_for_origins,
    create_app_cors_cors_request_blocked,
    create_app_cors_cors_safelisted_headers_without_preflight,
    create_app_cors_cors_vary_header_for_proper_caching,
    create_app_cors_cors_wildcard_origin,
    create_app_cors_cors_with_credentials,
    create_app_cors_simple_cors_request,
)


async def test_07_cors_preflight_header_not_allowed() -> None:
    """CORS preflight request with non-allowed header should be rejected."""

    app = create_app_cors_07_cors_preflight_header_not_allowed()
    client = TestClient(app)

    headers = {
        "Origin": "https://example.com",
        "Access-Control-Request-Method": "POST",
        "Access-Control-Request-Headers": "X-Custom-Header",
    }
    response = await client.options("/api/data", headers=headers)

    assert response.status_code == 403


async def test_cors_vary_header_for_proper_caching() -> None:
    """Tests that Vary: Origin header is present for correct cache behavior."""

    app = create_app_cors_cors_vary_header_for_proper_caching()
    client = TestClient(app)

    headers = {
        "Cache-Control": "max-age=3600",
        "Origin": "https://app.example.com",
    }
    response = await client.get("/api/cached-resource", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "data" in response_data
    assert response_data["data"] == "cacheable resource"


async def test_cors_preflight_for_put_method() -> None:
    """Tests OPTIONS preflight request for PUT method with custom headers."""

    app = create_app_cors_cors_preflight_for_put_method()
    client = TestClient(app)

    headers = {
        "Access-Control-Request-Method": "PUT",
        "Access-Control-Request-Headers": "Content-Type, X-Custom-Header",
        "Origin": "https://app.example.com",
    }
    response = await client.options("/api/resource/123", headers=headers)

    assert response.status_code == 204


async def test_cors_preflight_for_delete_method() -> None:
    """Tests OPTIONS preflight request for DELETE method."""

    app = create_app_cors_cors_preflight_for_delete_method()
    client = TestClient(app)

    headers = {
        "Origin": "https://app.example.com",
        "Access-Control-Request-Method": "DELETE",
    }
    response = await client.options("/api/resource/456", headers=headers)

    assert response.status_code == 204


async def test_cors_multiple_allowed_origins() -> None:
    """Tests CORS when multiple origins are allowed and request origin matches one."""

    app = create_app_cors_cors_multiple_allowed_origins()
    client = TestClient(app)

    headers = {
        "Origin": "https://admin.example.com",
    }
    response = await client.get("/api/data", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "data" in response_data
    assert response_data["data"] == "resource data"


async def test_cors_preflight_request() -> None:
    """Tests OPTIONS preflight request for CORS."""

    app = create_app_cors_cors_preflight_request()
    client = TestClient(app)

    headers = {
        "Access-Control-Request-Method": "POST",
        "Origin": "https://example.com",
        "Access-Control-Request-Headers": "Content-Type, X-Custom-Header",
    }
    response = await client.options("/items/", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


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


async def test_cors_regex_pattern_matching_for_origins() -> None:
    """Tests CORS with regex pattern matching for subdomain wildcards."""

    app = create_app_cors_cors_regex_pattern_matching_for_origins()
    client = TestClient(app)

    headers = {
        "Origin": "https://subdomain.example.com",
    }
    response = await client.get("/api/data", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "data" in response_data
    assert response_data["data"] == "resource data"


async def test_08_cors_max_age() -> None:
    """CORS preflight response should include Access-Control-Max-Age."""

    app = create_app_cors_08_cors_max_age()
    client = TestClient(app)

    headers = {
        "Origin": "https://example.com",
        "Access-Control-Request-Headers": "Content-Type",
        "Access-Control-Request-Method": "POST",
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


async def test_cors_safelisted_headers_without_preflight() -> None:
    """Tests that safelisted headers (Content-Type: text/plain, Accept, Accept-Language) don't require preflight."""

    app = create_app_cors_cors_safelisted_headers_without_preflight()
    client = TestClient(app)

    headers = {
        "Accept": "application/json",
        "Accept-Language": "en-US",
        "Content-Type": "text/plain",
        "Origin": "https://app.example.com",
    }
    response = await client.post("/api/form", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Success"


async def test_cors_private_network_access() -> None:
    """Tests Private Network Access (RFC 1918) preflight with Access-Control-Request-Private-Network."""

    app = create_app_cors_cors_private_network_access()
    client = TestClient(app)

    headers = {
        "Access-Control-Request-Private-Network": "true",
        "Origin": "https://public.example.com",
        "Access-Control-Request-Method": "GET",
    }
    response = await client.options("/api/local-resource", headers=headers)

    assert response.status_code == 204


async def test_cors_origin_case_sensitivity() -> None:
    """Tests that CORS origin matching is case-sensitive for the domain part."""

    app = create_app_cors_cors_origin_case_sensitivity()
    client = TestClient(app)

    headers = {
        "Origin": "https://EXAMPLE.COM",
    }
    response = await client.get("/api/data", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


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
    response_data = response.json()


async def test_06_cors_preflight_method_not_allowed() -> None:
    """CORS preflight request for non-allowed method should be rejected."""

    app = create_app_cors_06_cors_preflight_method_not_allowed()
    client = TestClient(app)

    headers = {
        "Access-Control-Request-Method": "DELETE",
        "Origin": "https://example.com",
        "Access-Control-Request-Headers": "Content-Type",
    }
    response = await client.options("/api/data", headers=headers)

    assert response.status_code == 403

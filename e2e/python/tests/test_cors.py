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

    async with TestClient(create_app_cors_07_cors_preflight_header_not_allowed()) as client:
        headers = {
            "Access-Control-Request-Method": "POST",
            "Origin": "https://example.com",
            "Access-Control-Request-Headers": "X-Custom-Header",
        }
        response = await client.options("/api/data", headers=headers)

        assert response.status_code == 403


async def test_cors_vary_header_for_proper_caching() -> None:
    """Tests that Vary: Origin header is present for correct cache behavior."""

    async with TestClient(create_app_cors_cors_vary_header_for_proper_caching()) as client:
        headers = {
            "Origin": "https://app.example.com",
            "Cache-Control": "max-age=3600",
        }
        response = await client.get("/api/cached-resource", headers=headers)

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert response_data["data"] == "cacheable resource"
        response_headers = response.headers
        assert response_headers.get("cache-control") == "public, max-age=3600"
        assert response_headers.get("access-control-allow-origin") == "https://app.example.com"
        assert response_headers.get("vary") == "Origin"


async def test_cors_preflight_for_put_method() -> None:
    """Tests OPTIONS preflight request for PUT method with custom headers."""

    async with TestClient(create_app_cors_cors_preflight_for_put_method()) as client:
        headers = {
            "Origin": "https://app.example.com",
            "Access-Control-Request-Method": "PUT",
            "Access-Control-Request-Headers": "Content-Type, X-Custom-Header",
        }
        response = await client.options("/api/resource/123", headers=headers)

        assert response.status_code == 204
        response_headers = response.headers
        assert response_headers.get("access-control-allow-headers") == "Content-Type, X-Custom-Header"
        assert response_headers.get("access-control-allow-methods") == "GET, POST, PUT, PATCH, DELETE"
        assert response_headers.get("access-control-allow-origin") == "https://app.example.com"
        assert response_headers.get("vary") == "Origin"
        assert response_headers.get("access-control-max-age") == "3600"


async def test_cors_preflight_for_delete_method() -> None:
    """Tests OPTIONS preflight request for DELETE method."""

    async with TestClient(create_app_cors_cors_preflight_for_delete_method()) as client:
        headers = {
            "Access-Control-Request-Method": "DELETE",
            "Origin": "https://app.example.com",
        }
        response = await client.options("/api/resource/456", headers=headers)

        assert response.status_code == 204
        response_headers = response.headers
        assert response_headers.get("access-control-allow-methods") == "GET, POST, PUT, PATCH, DELETE"
        assert response_headers.get("access-control-allow-origin") == "https://app.example.com"
        assert response_headers.get("vary") == "Origin"
        assert response_headers.get("access-control-max-age") == "3600"


async def test_cors_multiple_allowed_origins() -> None:
    """Tests CORS when multiple origins are allowed and request origin matches one."""

    async with TestClient(create_app_cors_cors_multiple_allowed_origins()) as client:
        headers = {
            "Origin": "https://admin.example.com",
        }
        response = await client.get("/api/data", headers=headers)

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert response_data["data"] == "resource data"
        response_headers = response.headers
        assert response_headers.get("access-control-allow-origin") == "https://admin.example.com"
        assert response_headers.get("vary") == "Origin"


async def test_cors_preflight_request() -> None:
    """Tests OPTIONS preflight request for CORS."""

    async with TestClient(create_app_cors_cors_preflight_request()) as client:
        headers = {
            "Origin": "https://example.com",
            "Access-Control-Request-Method": "POST",
            "Access-Control-Request-Headers": "Content-Type, X-Custom-Header",
        }
        response = await client.options("/items/", headers=headers)

        assert response.status_code == 200
        response_data = response.json()
        response_headers = response.headers
        assert response_headers.get("access-control-allow-methods") == "GET, POST, PUT, DELETE, OPTIONS"
        assert response_headers.get("access-control-allow-origin") == "https://example.com"
        assert response_headers.get("access-control-allow-headers") == "Content-Type, X-Custom-Header"
        assert response_headers.get("access-control-max-age") == "600"


async def test_cors_with_credentials() -> None:
    """Tests CORS request with credentials (cookies, auth headers)."""

    async with TestClient(create_app_cors_cors_with_credentials()) as client:
        headers = {
            "Origin": "https://app.example.com",
            "Cookie": "session=abc123",
        }
        response = await client.get("/api/user/profile", headers=headers)

        assert response.status_code == 200
        response_data = response.json()
        assert "username" in response_data
        assert response_data["username"] == "john"
        response_headers = response.headers
        assert response_headers.get("access-control-allow-origin") == "https://app.example.com"
        assert response_headers.get("vary") == "Origin"
        assert response_headers.get("access-control-allow-credentials") == "true"


async def test_cors_regex_pattern_matching_for_origins() -> None:
    """Tests CORS with regex pattern matching for subdomain wildcards."""

    async with TestClient(create_app_cors_cors_regex_pattern_matching_for_origins()) as client:
        headers = {
            "Origin": "https://subdomain.example.com",
        }
        response = await client.get("/api/data", headers=headers)

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert response_data["data"] == "resource data"
        response_headers = response.headers
        assert response_headers.get("vary") == "Origin"
        assert response_headers.get("access-control-allow-origin") == "https://subdomain.example.com"


async def test_08_cors_max_age() -> None:
    """CORS preflight response should include Access-Control-Max-Age."""

    async with TestClient(create_app_cors_08_cors_max_age()) as client:
        headers = {
            "Access-Control-Request-Headers": "Content-Type",
            "Origin": "https://example.com",
            "Access-Control-Request-Method": "POST",
        }
        response = await client.options("/api/data", headers=headers)

        assert response.status_code == 204
        response_headers = response.headers
        assert response_headers.get("access-control-allow-headers") == "Content-Type"
        assert response_headers.get("access-control-allow-origin") == "https://example.com"
        assert response_headers.get("access-control-allow-methods") == "POST"
        assert response_headers.get("access-control-max-age") == "3600"


async def test_10_cors_origin_null() -> None:
    """CORS request with 'null' origin should be handled according to policy."""

    async with TestClient(create_app_cors_10_cors_origin_null()) as client:
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

    async with TestClient(create_app_cors_cors_wildcard_origin()) as client:
        headers = {
            "Origin": "https://random-site.com",
        }
        response = await client.get("/public/data", headers=headers)

        assert response.status_code == 200
        response_data = response.json()
        assert "data" in response_data
        assert response_data["data"] == "public"
        response_headers = response.headers
        assert response_headers.get("access-control-allow-origin") == "*"


async def test_cors_safelisted_headers_without_preflight() -> None:
    """Tests that safelisted headers (Content-Type: text/plain, Accept, Accept-Language) don't require preflight."""

    async with TestClient(create_app_cors_cors_safelisted_headers_without_preflight()) as client:
        headers = {
            "Accept": "application/json",
            "Content-Type": "text/plain",
            "Accept-Language": "en-US",
            "Origin": "https://app.example.com",
        }
        response = await client.post("/api/form", headers=headers)

        assert response.status_code == 200
        response_data = response.json()
        assert "message" in response_data
        assert response_data["message"] == "Success"
        response_headers = response.headers
        assert response_headers.get("access-control-allow-origin") == "https://app.example.com"
        assert response_headers.get("vary") == "Origin"


async def test_cors_private_network_access() -> None:
    """Tests Private Network Access (RFC 1918) preflight with Access-Control-Request-Private-Network."""

    async with TestClient(create_app_cors_cors_private_network_access()) as client:
        headers = {
            "Access-Control-Request-Method": "GET",
            "Origin": "https://public.example.com",
            "Access-Control-Request-Private-Network": "true",
        }
        response = await client.options("/api/local-resource", headers=headers)

        assert response.status_code == 204
        response_headers = response.headers
        assert response_headers.get("access-control-allow-private-network") == "true"
        assert response_headers.get("access-control-allow-methods") == "GET, POST"
        assert response_headers.get("vary") == "Origin"
        assert response_headers.get("access-control-allow-origin") == "https://public.example.com"


async def test_cors_origin_case_sensitivity() -> None:
    """Tests that CORS origin matching is case-sensitive for the domain part."""

    async with TestClient(create_app_cors_cors_origin_case_sensitivity()) as client:
        headers = {
            "Origin": "https://EXAMPLE.COM",
        }
        response = await client.get("/api/data", headers=headers)

        assert response.status_code == 200
        response_data = response.json()
        response_headers = response.headers
        assert response_headers.get("vary") == "Origin"


async def test_cors_request_blocked() -> None:
    """Tests CORS request from disallowed origin."""

    async with TestClient(create_app_cors_cors_request_blocked()) as client:
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

    async with TestClient(create_app_cors_simple_cors_request()) as client:
        headers = {
            "Origin": "https://example.com",
        }
        response = await client.get("/items/", headers=headers)

        assert response.status_code == 200
        response_data = response.json()
        assert "items" in response_data
        assert len(response_data["items"]) == 0
        response_headers = response.headers
        assert response_headers.get("access-control-allow-origin") == "https://example.com"
        assert response_headers.get("vary") == "Origin"


async def test_09_cors_expose_headers() -> None:
    """CORS response should include Access-Control-Expose-Headers for custom headers."""

    async with TestClient(create_app_cors_09_cors_expose_headers()) as client:
        headers = {
            "Origin": "https://example.com",
        }
        response = await client.get("/api/data", headers=headers)

        assert response.status_code == 200
        response_data = response.json()
        response_headers = response.headers
        assert response_headers.get("x-total-count") == "42"
        assert response_headers.get("access-control-expose-headers") == "X-Total-Count, X-Request-Id"
        assert response_headers.get("access-control-allow-origin") == "https://example.com"
        assert response_headers.get("x-request-id") == "abc123"


async def test_06_cors_preflight_method_not_allowed() -> None:
    """CORS preflight request for non-allowed method should be rejected."""

    async with TestClient(create_app_cors_06_cors_preflight_method_not_allowed()) as client:
        headers = {
            "Access-Control-Request-Method": "DELETE",
            "Access-Control-Request-Headers": "Content-Type",
            "Origin": "https://example.com",
        }
        response = await client.options("/api/data", headers=headers)

        assert response.status_code == 403

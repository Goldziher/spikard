"""E2E tests for lifecycle_hooks."""

import re

from spikard.testing import TestClient
from app.main import (
    create_app_lifecycle_hooks_hook_execution_order,
    create_app_lifecycle_hooks_multiple_hooks_all_phases,
    create_app_lifecycle_hooks_onerror_error_logging,
    create_app_lifecycle_hooks_onrequest_request_logging,
    create_app_lifecycle_hooks_onresponse_response_timing,
    create_app_lifecycle_hooks_onresponse_security_headers,
    create_app_lifecycle_hooks_prehandler_authentication_failed_short_circuit,
    create_app_lifecycle_hooks_prehandler_authentication_success,
    create_app_lifecycle_hooks_prehandler_authorization_check,
    create_app_lifecycle_hooks_prehandler_authorization_forbidden_short_circuit,
    create_app_lifecycle_hooks_prevalidation_rate_limit_exceeded_short_circuit,
    create_app_lifecycle_hooks_prevalidation_rate_limiting,
)


async def test_onresponse_security_headers() -> None:
    """Test onResponse hook that adds security headers to all responses."""

    async with TestClient(create_app_lifecycle_hooks_onresponse_security_headers()) as client:
        response = await client.get("/api/test-security-headers")

        assert response.status_code == 200
        response_data = response.json()
        assert "message" in response_data
        assert response_data["message"] == "Response with security headers"
        response_headers = response.headers
        assert response_headers.get("strict-transport-security") == "max-age=31536000; includeSubDomains"
        assert response_headers.get("x-frame-options") == "DENY"
        assert response_headers.get("x-content-type-options") == "nosniff"
        assert response_headers.get("x-xss-protection") == "1; mode=block"


async def test_prehandler_authentication_failed_short_circuit() -> None:
    """Test preHandler hook that short-circuits on invalid authentication."""

    async with TestClient(create_app_lifecycle_hooks_prehandler_authentication_failed_short_circuit()) as client:
        headers = {
            "Authorization": "Bearer invalid-token",
        }
        response = await client.get("/api/protected-resource-fail", headers=headers)

        assert response.status_code == 401
        response_data = response.json()
        assert "error" in response_data
        assert response_data["error"] == "Unauthorized"
        assert "message" in response_data
        assert response_data["message"] == "Invalid or expired authentication token"


async def test_prehandler_authorization_check() -> None:
    """Test preHandler hook for role-based authorization after authentication."""

    async with TestClient(create_app_lifecycle_hooks_prehandler_authorization_check()) as client:
        headers = {
            "Authorization": "Bearer admin-token-67890",
        }
        response = await client.get("/api/admin-only", headers=headers)

        assert response.status_code == 200
        response_data = response.json()
        assert "message" in response_data
        assert response_data["message"] == "Admin access granted"
        assert "role" in response_data
        assert response_data["role"] == "admin"
        assert "user_id" in response_data
        assert response_data["user_id"] == "admin-456"


async def test_prehandler_authentication_success() -> None:
    """Test preHandler hook that validates authentication tokens."""

    async with TestClient(create_app_lifecycle_hooks_prehandler_authentication_success()) as client:
        headers = {
            "Authorization": "Bearer valid-token-12345",
        }
        response = await client.get("/api/protected-resource", headers=headers)

        assert response.status_code == 200
        response_data = response.json()
        assert "authenticated" in response_data
        assert response_data["authenticated"] == True
        assert "message" in response_data
        assert response_data["message"] == "Access granted"
        assert "user_id" in response_data
        assert response_data["user_id"] == "user-123"


async def test_prevalidation_rate_limit_exceeded_short_circuit() -> None:
    """Test preValidation hook that short-circuits when rate limit is exceeded."""

    async with TestClient(create_app_lifecycle_hooks_prevalidation_rate_limit_exceeded_short_circuit()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"data": "test"}
        response = await client.post("/api/test-rate-limit-exceeded", headers=headers, json=json_data)

        assert response.status_code == 429
        response_data = response.json()
        assert "error" in response_data
        assert response_data["error"] == "Rate limit exceeded"
        assert "message" in response_data
        assert response_data["message"] == "Too many requests, please try again later"
        response_headers = response.headers
        assert response_headers.get("retry-after") == "60"


async def test_onerror_error_logging() -> None:
    """Test onError hook that logs server errors and formats error responses."""

    async with TestClient(create_app_lifecycle_hooks_onerror_error_logging()) as client:
        response = await client.get("/api/test-error")

        assert response.status_code == 500
        response_data = response.json()
        assert "error" in response_data
        assert response_data["error"] == "Internal Server Error"
        assert "error_id" in response_data
        assert response_data["error_id"] == ".*"
        assert "message" in response_data
        assert response_data["message"] == "An unexpected error occurred"
        response_headers = response.headers
        assert response_headers.get("content-type") == "application/json"


async def test_multiple_hooks_all_phases() -> None:
    """Test multiple lifecycle hooks across all five phases for a complete request lifecycle."""

    async with TestClient(create_app_lifecycle_hooks_multiple_hooks_all_phases()) as client:
        headers = {
            "Content-Type": "application/json",
            "Authorization": "Bearer valid-token-12345",
        }
        json_data = {"action": "update_profile", "user_id": "user-123"}
        response = await client.post("/api/full-lifecycle", headers=headers, json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "action" in response_data
        assert response_data["action"] == "update_profile"
        assert "message" in response_data
        assert response_data["message"] == "Action completed successfully"
        assert "request_id" in response_data
        assert response_data["request_id"] == ".*"
        assert "user_id" in response_data
        assert response_data["user_id"] == "user-123"
        response_headers = response.headers
        assert response_headers.get("x-frame-options") == "DENY"
        header_value = response_headers.get("x-response-time")
        assert header_value is not None
        assert re.match(r".*ms", header_value)
        assert response_headers.get("x-content-type-options") == "nosniff"
        header_value = response_headers.get("x-request-id")
        assert header_value is not None
        assert re.match(r".*", header_value)


async def test_hook_execution_order() -> None:
    """Test that multiple hooks of the same type execute in registration order."""

    async with TestClient(create_app_lifecycle_hooks_hook_execution_order()) as client:
        response = await client.get("/api/test-hook-order")

        assert response.status_code == 200
        response_data = response.json()
        assert "execution_order" in response_data
        assert len(response_data["execution_order"]) == 3
        assert response_data["execution_order"][0] == "first_hook"
        assert response_data["execution_order"][1] == "second_hook"
        assert response_data["execution_order"][2] == "third_hook"
        assert "message" in response_data
        assert response_data["message"] == "Hooks executed in order"


async def test_onresponse_response_timing() -> None:
    """Test onResponse hook that adds timing information to response headers."""

    async with TestClient(create_app_lifecycle_hooks_onresponse_response_timing()) as client:
        response = await client.get("/api/test-timing")

        assert response.status_code == 200
        response_data = response.json()
        assert "message" in response_data
        assert response_data["message"] == "Response with timing info"
        response_headers = response.headers
        header_value = response_headers.get("x-response-time")
        assert header_value is not None
        assert re.match(r".*ms", header_value)


async def test_prehandler_authorization_forbidden_short_circuit() -> None:
    """Test preHandler hook that denies access for insufficient permissions."""

    async with TestClient(create_app_lifecycle_hooks_prehandler_authorization_forbidden_short_circuit()) as client:
        headers = {
            "Authorization": "Bearer user-token-11111",
        }
        response = await client.get("/api/admin-only-forbidden", headers=headers)

        assert response.status_code == 403
        response_data = response.json()
        assert "error" in response_data
        assert response_data["error"] == "Forbidden"
        assert "message" in response_data
        assert response_data["message"] == "Admin role required for this endpoint"


async def test_onrequest_request_logging() -> None:
    """Test onRequest hook that logs incoming requests and adds a request ID."""

    async with TestClient(create_app_lifecycle_hooks_onrequest_request_logging()) as client:
        response = await client.get("/api/test-on-request")

        assert response.status_code == 200
        response_data = response.json()
        assert "has_request_id" in response_data
        assert response_data["has_request_id"] == True
        assert "message" in response_data
        assert response_data["message"] == "onRequest hooks executed"
        assert "request_logged" in response_data
        assert response_data["request_logged"] == True
        response_headers = response.headers
        header_value = response_headers.get("x-request-id")
        assert header_value is not None
        assert re.match(r".*", header_value)


async def test_prevalidation_rate_limiting() -> None:
    """Test preValidation hook that implements rate limiting before validation."""

    async with TestClient(create_app_lifecycle_hooks_prevalidation_rate_limiting()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"data": "test"}
        response = await client.post("/api/test-rate-limit", headers=headers, json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "message" in response_data
        assert response_data["message"] == "Request accepted"
        assert "rate_limit_checked" in response_data
        assert response_data["rate_limit_checked"] == True
